//! [confirmed] A4F2 post-score strategy closure: archaeology T20/F20.
//! [designed] This is not the surrounding 52B6/6B2F/78B7 implementation.
use crate::{extra_event, placement, step_tail, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub age: u8,
    pub flags3: u8,
    pub level: u8,
    pub position: u8,
    pub flags10: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub roster: [u16; 20],
    pub byte51: u8,
    pub byte54: u8,
    pub strategy: u8,
    pub baseline_index: u8,
    pub baseline_value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub scores: [u8; 2],
    pub managed_clubs: [u8; 2],
    pub limits: [u8; 2],
    pub raised: [u8; 2],
    pub lowered: [u8; 2],
    pub clubs: [Club; 2],
    pub players: Vec<Player>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub state: State,
    pub argument: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Path {
    pub routine: String,
    pub side: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raise: Option<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Normalizer {
    pub player: u16,
    pub before: u8,
    pub after: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Getter {
    pub club: u16,
    pub value: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Setter {
    pub club: u16,
    pub value: u8,
    pub input: State,
    pub normalizer_start: usize,
    pub output: State,
    pub normalizer_end: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: State,
    pub draws: Vec<Draw>,
    pub paths: Vec<Path>,
    pub setters: Vec<Setter>,
    pub normalizers: Vec<Normalizer>,
    pub getters: Vec<Getter>,
}
const P8: usize = 0x30000;
const P15: usize = 0x20000;
const CLUB: usize = 0x40000;
fn p8(id: u16, f: usize) -> usize {
    P8 + usize::from(id) * 8 + f
}
fn flags(id: u16) -> usize {
    P15 + usize::from(id) * 15 + 10
}
fn club(id: u16, f: usize) -> usize {
    CLUB + (usize::from(id) - 1) * 62 + f
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes());
}
fn validate(s: &State) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    let mut clubs = BTreeMap::new();
    let mut cells = BTreeMap::new();
    for c in &s.clubs {
        if !(1..=176).contains(&c.id) || c.roster.iter().any(|&id| id > 3920) {
            return Err("unsupported club/roster domain".into());
        }
        if clubs.insert(c.id, c).is_some_and(|old| old != c) {
            return Err("conflicting aliased club".into());
        }
        if cells
            .insert(c.baseline_index, c.baseline_value)
            .is_some_and(|old| old != c.baseline_value)
        {
            return Err("conflicting baseline cell".into());
        }
        ids.extend(c.roster);
    }
    let mut players = BTreeSet::new();
    for p in &s.players {
        if p.id > 3920 || !players.insert(p.id) {
            return Err("ambiguous shared player".into());
        }
    }
    if ids != players {
        return Err("missing/unreferenced player input".into());
    }
    Ok(())
}
fn blank(s: &State) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed: s.seed,
        ratings: [0; 8],
        goals: [0; 2],
    })
}
fn load(e: &mut Engine, s: &State, preserve: bool) {
    if !preserve {
        e.put_word(0xA51C, s.seed);
        e.image[0xD0C4..0xD0C6].copy_from_slice(&s.scores);
        for (i, c) in s.clubs.iter().enumerate() {
            e.put_word(0xD6E4 + 2 * i as u16, c.id);
        }
    }
    e.image[0xDCBA..0xDCBC].copy_from_slice(&s.managed_clubs);
    e.image[0x0C66..0x0C68].copy_from_slice(&s.limits);
    e.image[0xD0CC..0xD0CE].copy_from_slice(&s.raised);
    e.image[0xD0CE..0xD0D0].copy_from_slice(&s.lowered);
    for c in &s.clubs {
        for (i, &id) in c.roster.iter().enumerate() {
            put(e, club(c.id, 2 * i), id);
        }
        for (f, v) in [(51, c.byte51), (54, c.byte54), (57, c.strategy)] {
            e.image[club(c.id, f)] = v;
        }
        e.image[0xD885 + usize::from(c.id)] = c.baseline_index;
        e.image[0x93F7 + usize::from(c.baseline_index)] = c.baseline_value;
    }
    for p in &s.players {
        for (f, v) in [(1, p.age), (3, p.flags3), (5, p.level), (6, p.position)] {
            e.image[p8(p.id, f)] = v;
        }
        e.image[flags(p.id)] = p.flags10;
    }
}
fn project(e: &Engine, template: &State) -> State {
    let mut s = template.clone();
    s.seed = e.word(0xA51C);
    s.scores.copy_from_slice(&e.image[0xD0C4..0xD0C6]);
    s.managed_clubs.copy_from_slice(&e.image[0xDCBA..0xDCBC]);
    s.limits.copy_from_slice(&e.image[0x0C66..0x0C68]);
    s.raised.copy_from_slice(&e.image[0xD0CC..0xD0CE]);
    s.lowered.copy_from_slice(&e.image[0xD0CE..0xD0D0]);
    for (i, c) in s.clubs.iter_mut().enumerate() {
        c.id = e.word(0xD6E4 + 2 * i as u16);
        for (j, id) in c.roster.iter_mut().enumerate() {
            *id = word(e, club(c.id, 2 * j));
        }
        c.byte51 = e.image[club(c.id, 51)];
        c.byte54 = e.image[club(c.id, 54)];
        c.strategy = e.image[club(c.id, 57)];
        c.baseline_index = e.image[0xD885 + usize::from(c.id)];
        c.baseline_value = e.image[0x93F7 + usize::from(c.baseline_index)];
    }
    for p in &mut s.players {
        p.age = e.image[p8(p.id, 1)];
        p.flags3 = e.image[p8(p.id, 3)];
        p.level = e.image[p8(p.id, 5)];
        p.position = e.image[p8(p.id, 6)];
        p.flags10 = e.image[flags(p.id)];
    }
    s
}
pub fn strategy(input: &Input) -> Result<Outcome, String> {
    validate(&input.state)?;
    let mut e = blank(&input.state);
    load(&mut e, &input.state, false);
    // [confirmed] A4F2 never reads the supplied argument word.
    Ok(e.post_score_loaded(&input.state))
}
#[derive(Debug, Serialize)]
pub struct ComposedOutcome {
    pub predecessor: step_tail::ComposedOutcome,
    pub strategy: Outcome,
}
pub fn composed(
    placement: &placement::Input,
    prefix: &step_tail::PrefixInput,
    input: &Input,
) -> Result<ComposedOutcome, String> {
    validate(&input.state)?;
    if prefix.mode == 0 || prefix.placement.goal_flag != 1 {
        return Err("strategy projection requires quick goal-reset path".into());
    }
    let actor = prefix.placement.actor;
    if !(1..=22).contains(&actor) || actor == 1 || actor == 12 {
        return Err("untranslated keeper/invalid reset prelude".into());
    }
    if input.state.clubs[0].roster != placement.rosters[1]
        || input.state.clubs[1].roster != placement.rosters[0]
    {
        return Err("strategy rosters conflict with placement input".into());
    }
    let (mut e, predecessor) = step_tail::composed_prefix_engine(placement, prefix)?;
    let strategy = e.apply_post_score(input)?;
    Ok(ComposedOutcome {
        predecessor,
        strategy,
    })
}
fn magnitude(scores: [u8; 2]) -> i16 {
    let byte = (i16::from(scores[0] as i8).wrapping_sub(i16::from(scores[1] as i8))) as i8;
    i16::from(byte).abs()
}
impl Engine {
    pub fn apply_post_score(&mut self, input: &Input) -> Result<Outcome, String> {
        validate(&input.state)?;
        let s = &input.state;
        if self.word(0xA51C) != s.seed
            || self.image[0xD0C4..0xD0C6] != s.scores
            || self.word(0xD6E4) != s.clubs[0].id
            || self.word(0xD6E6) != s.clubs[1].id
        {
            return Err("actual prefix seed/score/club strategy handoff differs".into());
        }
        let mut trial = Engine {
            image: self.image.clone(),
        };
        load(&mut trial, s, true);
        let result = trial.post_score_loaded(s);
        *self = trial;
        Ok(result)
    }
    fn post_score_loaded(&mut self, template: &State) -> Outcome {
        let mut out = Outcome {
            output: template.clone(),
            draws: vec![],
            paths: vec![],
            setters: vec![],
            normalizers: vec![],
            getters: vec![],
        };
        let scores = [self.image[0xD0C4], self.image[0xD0C5]];
        let gap = magnitude(scores);
        for side in 0..2 {
            let id = self.word(0xD6E4 + 2 * side as u16);
            if [u16::from(self.image[0xDCBA]), u16::from(self.image[0xDCBB])].contains(&id) {
                continue;
            }
            if gap < 2 {
                self.post_baseline(template, side, &mut out);
            } else if gap == 2 {
                self.post_three(template, side, scores[side] <= scores[side ^ 1], &mut out);
            } else {
                self.post_five(template, side, scores[side] <= scores[side ^ 1], &mut out);
            }
        }
        out.output = project(self, template);
        out
    }
    fn post_path(&self, routine: &str, side: usize, raise: Option<bool>, out: &mut Outcome) {
        out.paths.push(Path {
            routine: routine.into(),
            side: side as u8,
            raise: raise.map(u8::from),
        });
    }
    fn post_normalize(&mut self, id: u16, out: &mut Outcome) {
        let before = self.image[flags(id)];
        let mut value = before;
        if extra_event::threshold(self, id) < 4 {
            value = (value | 0x80) & !8;
        }
        match self.image[p8(id, 6)] {
            0 | 1 | 3 | 4 => value |= 0x11,
            2 => value |= 0x32,
            _ => {}
        }
        self.image[flags(id)] = value;
        out.normalizers.push(Normalizer {
            player: id,
            before,
            after: value,
        });
    }
    fn post_set(&mut self, template: &State, id: u16, value: u8, out: &mut Outcome) {
        let input = project(self, template);
        let start = out.normalizers.len();
        self.image[club(id, 57)] = value;
        for i in 0..20 {
            let player = word(self, club(id, 2 * i));
            if player != 0 {
                self.post_normalize(player, out);
            }
        }
        out.setters.push(Setter {
            club: id,
            value,
            input,
            normalizer_start: start,
            output: project(self, template),
            normalizer_end: out.normalizers.len(),
        });
    }
    fn post_rating(&self, id: u16, out: &mut Outcome) -> u16 {
        let captain = self.image[club(id, 54)];
        let selected = if captain == 0 {
            self.image[p8(word(self, club(id, 0)), 5)]
        } else {
            captain
        };
        let value = 193u16
            .wrapping_sub(u16::from(self.image[club(id, 51)]))
            .wrapping_sub(u16::from(selected));
        out.getters.push(Getter { club: id, value });
        value
    }
    fn post_baseline(&mut self, template: &State, side: usize, out: &mut Outcome) {
        self.post_path("0AB9:A23C", side, None, out);
        let id = self.word(0xD6E4 + 2 * side as u16);
        let index = self.image[0xD885 + usize::from(id)];
        let value = self.image[0x93F7 + usize::from(index)];
        self.post_set(template, id, value, out);
        if side == 1 {
            self.image[club(id, 57)] |= 1;
            self.post_set(template, id, self.image[club(id, 57)], out);
        }
        let other = self.word(0xD6E4 + 2 * (side ^ 1) as u16);
        let first = self.post_rating(other, out);
        let second = self.post_rating(other, out);
        if (second.wrapping_sub(first) as i16) > i16::from(self.image[0x0C66 + side]) {
            for _ in 0..2 {
                let bit = self.draw(6, "0AB9:A305", &mut out.draws);
                self.image[club(id, 57)] |= 1u8 << (bit - 1);
            }
            self.post_set(template, id, self.image[club(id, 57)], out);
        }
        self.image[0xD0CC + side] = 0;
        self.image[0xD0CE + side] = 0;
    }
    fn post_three(&mut self, template: &State, side: usize, raise: bool, out: &mut Outcome) {
        self.post_path("0AB9:A34D", side, Some(raise), out);
        self.post_baseline(template, side, out);
        let id = self.word(0xD6E4 + 2 * side as u16);
        for _ in 0..3 {
            let bit = self.draw(
                6,
                if raise { "0AB9:A38C" } else { "0AB9:A3A4" },
                &mut out.draws,
            );
            let mask = 1u8 << (bit - 1);
            if raise {
                self.image[club(id, 57)] |= mask;
            } else {
                self.image[club(id, 57)] &= !mask;
            }
        }
        self.post_set(template, id, self.image[club(id, 57)], out);
    }
    fn post_five(&mut self, template: &State, side: usize, raise: bool, out: &mut Outcome) {
        self.post_path(
            if raise { "0AB9:A3D6" } else { "0AB9:A464" },
            side,
            None,
            out,
        );
        let latch = if raise { 0xD0CC + side } else { 0xD0CE + side };
        if self.image[latch] != 0 {
            return;
        }
        let id = self.word(0xD6E4 + 2 * side as u16);
        for _ in 0..5 {
            let bit = self.draw(
                5,
                if raise { "0AB9:A417" } else { "0AB9:A4A5" },
                &mut out.draws,
            );
            let mask = 1u8 << (bit - 1);
            if raise {
                self.image[club(id, 57)] |= mask;
            } else {
                self.image[club(id, 57)] &= !mask;
            }
        }
        if raise {
            self.image[club(id, 57)] |= 0x20;
        } else {
            self.image[club(id, 57)] &= !0x20;
        }
        self.post_set(template, id, self.image[club(id, 57)], out);
        self.image[latch] = 1;
    }
}
