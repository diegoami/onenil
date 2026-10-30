//! [confirmed] 69B8 quick logical effect closure: archaeology T19/F19 at 6804fbb.
//! [designed] AI roster-changing calls diagnose; render/timer effects stay external.
use crate::{
    extra_event,
    goal_effects::{self, Commentary, Queue, QueueRow},
    placement, step_tail, Draw, Engine, Input as ContestInput,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_0: u8,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_4: u8,
    pub p8_5: u8,
    pub p8_6: u8,
    pub p8_7: u8,
    pub p15_6: u8,
    pub p15_8: u8,
    pub p15_10: u8,
    pub p15_11: u8,
    pub p15_14: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Roster {
    pub club: u16,
    pub players: [u16; 20],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub index: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Selectors {
    pub head: u16,
    pub cells: [Cell; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneExtra {
    pub index: u8,
    pub word8: u16,
    pub motion: u8,
    pub motion_word: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Weight {
    pub code: u8,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub handoff: extra_event::State,
    pub players: Vec<Player>,
    pub rosters: [Roster; 2],
    pub selectors: Selectors,
    pub managed_clubs: [u8; 2],
    pub mode: u8,
    pub pending: u8,
    pub cf53: u8,
    pub sub_flags: [u8; 2],
    pub reserves: [u8; 2],
    pub added: u16,
    pub active: u8,
    pub control: u8,
    pub clear_word: u16,
    pub target_word: u16,
    pub scene_extra: Vec<SceneExtra>,
    pub counters: [[u8; 3]; 2],
    pub cache: [[u8; 20]; 2],
    pub event_totals: [[u16; 20]; 2],
    pub weights: [Weight; 3],
    pub event_counts: [u16; 2],
    pub queue: Queue,
    pub source_pair: [u16; 2],
    pub target_pair: [u16; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Selector {
    pub site: String,
    pub head_before: u16,
    pub head_after: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Action {
    pub routine: String,
    pub scene: u8,
    pub input: State,
    pub output: State,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: State,
    pub selectors: Vec<Selector>,
    pub draws: Vec<Draw>,
    pub actions: Vec<Action>,
    pub helpers: Vec<serde_json::Value>,
    pub events: Vec<Commentary>,
    pub external_calls: Vec<serde_json::Value>,
}
const P8: usize = 0x30000;
const P15: usize = 0x20000;
const CLUB: usize = 0x40000;
fn scene(i: u8) -> usize {
    0xCCEF + usize::from(i) * 25
}
fn p8(i: u16, f: usize) -> usize {
    P8 + usize::from(i) * 8 + f
}
fn p15(i: u16, f: usize) -> usize {
    P15 + usize::from(i) * 15 + f
}
fn roster(club: u16, slot: usize) -> usize {
    CLUB + (usize::from(club) - 1) * 62 + 2 * slot
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes());
}
fn row(i: u16) -> usize {
    0xA52A + 6 * usize::from(i)
}
fn next_queue(i: u16) -> u16 {
    if i == 1700 {
        1
    } else {
        i + 1
    }
}
fn next_selector(i: u16) -> u16 {
    if i == 255 {
        1
    } else {
        i + 1
    }
}
fn queue_row(e: &Engine, i: u16) -> QueueRow {
    let a = row(i);
    QueueRow {
        index: i,
        code: e.image[a],
        first_player: word(e, a + 1),
        second_player: word(e, a + 3),
        side: e.image[a + 5],
    }
}
fn validate(s: &State) -> Result<(), String> {
    extra_event::validate_state(&s.handoff)?;
    if s.mode == 0
        || !(1..=22).contains(&s.handoff.actor)
        || !(1..=22).contains(&s.handoff.secondary)
        || s.handoff.scenes.iter().any(|r| !(1..=22).contains(&r.slot))
        || s.handoff.side_offsets.iter().any(|&v| v > 11)
    {
        return Err("unsupported quick mode, actors, slots or offsets".into());
    }
    if !(1..=255).contains(&s.selectors.head)
        || s.selectors.cells[0].index != s.selectors.head
        || s.selectors.cells[1].index != next_selector(s.selectors.head)
    {
        return Err("complete selector entry context required".into());
    }
    let mut expected = [0u16; 4];
    expected[0] = s.queue.head;
    if s.queue.head > 1700 {
        return Err("unsupported queue head".into());
    }
    for i in 1..4 {
        expected[i] = next_queue(expected[i - 1]);
    }
    if s.queue.rows.len() != 4 || s.queue.rows.iter().zip(expected).any(|(r, i)| r.index != i) {
        return Err("complete four-row numeric context required".into());
    }
    if s.weights.iter().map(|w| w.code).collect::<Vec<_>>() != [50, 51, 52]
        || s.scene_extra.len() != 22
        || s.scene_extra
            .iter()
            .enumerate()
            .any(|(i, r)| usize::from(r.index) != i + 1)
    {
        return Err("complete weight/scene reset fields required".into());
    }
    let mut ids = s
        .handoff
        .scenes
        .iter()
        .map(|r| r.player)
        .collect::<BTreeSet<_>>();
    for (i, r) in s.rosters.iter().enumerate() {
        if r.club != s.handoff.clubs[i].id || r.players.iter().any(|&id| id > 3920) {
            return Err("roster/club identity differs".into());
        }
        ids.extend(r.players);
    }
    if s.rosters[0].club == s.rosters[1].club && s.rosters[0].players != s.rosters[1].players {
        return Err("conflicting aliased rosters".into());
    }
    let mut players = BTreeMap::new();
    for p in &s.players {
        if p.id > 3920 || players.insert(p.id, p).is_some() {
            return Err("ambiguous shared player definition".into());
        }
    }
    if ids != players.keys().copied().collect() {
        return Err("missing/unreferenced effect player".into());
    }
    for p in &s.handoff.players {
        let q = players[&p.id];
        if (
            p.p8_0, p.p8_1, p.p8_3, p.p8_4, p.p8_5, p.p8_7, p.p15_6, p.p15_8, p.p15_10, p.p15_11,
        ) != (
            q.p8_0, q.p8_1, q.p8_3, q.p8_4, q.p8_5, q.p8_7, q.p15_6, q.p15_8, q.p15_10, q.p15_11,
        ) {
            return Err("effect player conflicts with decision handoff".into());
        }
    }
    Ok(())
}
fn load(e: &mut Engine, s: &State, preserve: bool) -> Result<(), String> {
    if !preserve {
        extra_event::load_state(e, &s.handoff, false)?;
    }
    let known = s
        .handoff
        .players
        .iter()
        .map(|p| p.id)
        .collect::<BTreeSet<_>>();
    for p in &s.players {
        for (i, v) in [
            (0, p.p8_0),
            (1, p.p8_1),
            (3, p.p8_3),
            (4, p.p8_4),
            (5, p.p8_5),
            (6, p.p8_6),
            (7, p.p8_7),
        ] {
            if !preserve || !known.contains(&p.id) || i == 6 {
                e.image[p8(p.id, i)] = v;
            }
        }
        for (i, v) in [
            (6, p.p15_6),
            (8, p.p15_8),
            (10, p.p15_10),
            (11, p.p15_11),
            (14, p.p15_14),
        ] {
            if !preserve || !known.contains(&p.id) || i == 14 {
                e.image[p15(p.id, i)] = v;
            }
        }
    }
    for r in &s.rosters {
        for (i, &id) in r.players.iter().enumerate() {
            put(e, roster(r.club, i), id);
        }
    }
    put(e, 0xD072, s.selectors.head);
    for c in &s.selectors.cells {
        e.image[0xCF71 + usize::from(c.index)] = c.value;
    }
    e.image[0xDCBA..0xDCBC].copy_from_slice(&s.managed_clubs);
    for (a, v) in [
        (0xD074, s.mode),
        (0xD6E2, s.pending),
        (0xCF53, s.cf53),
        (0xD0B8, s.active),
        (0xD0BA, s.control),
    ] {
        e.image[a] = v;
    }
    e.image[0xD6DE..0xD6E0].copy_from_slice(&s.sub_flags);
    e.image[0xD6E0..0xD6E2].copy_from_slice(&s.reserves);
    for (a, v) in [
        (0xD0B4, s.added),
        (0x0D2E, s.clear_word),
        (0x0D30, s.target_word),
        (0x0380, s.queue.head),
    ] {
        put(e, a, v);
    }
    for r in &s.scene_extra {
        let a = scene(r.index);
        put(e, a + 8, r.word8);
        e.image[a + 19] = r.motion;
        put(e, a + 20, r.motion_word);
    }
    for side in 0..2 {
        for i in 0..3 {
            e.image[0xD0D9 + 10 * side + i] = s.counters[side][i];
        }
        for i in 0..20 {
            e.image[0xD198 + 20 * side + i] = s.cache[side][i];
            put(e, 0xD148 + 40 * side + 2 * i, s.event_totals[side][i]);
        }
        put(e, 0xD1C0 + 2 * side, s.event_counts[side]);
    }
    for w in &s.weights {
        e.image[0x097A + usize::from(w.code)] = w.value;
    }
    for r in &s.queue.rows {
        let a = row(r.index);
        e.image[a] = r.code;
        put(e, a + 1, r.first_player);
        put(e, a + 3, r.second_player);
        e.image[a + 5] = r.side;
    }
    for (a, v) in [
        (0xD08A, s.source_pair[0]),
        (0xD08C, s.source_pair[1]),
        (0xD09A, s.target_pair[0]),
        (0xD09C, s.target_pair[1]),
    ] {
        put(e, a, v);
    }
    Ok(())
}
fn project(e: &Engine, template: &State) -> State {
    let mut s = template.clone();
    s.handoff = extra_event::project(e, &template.handoff);
    for p in &mut s.players {
        p.p8_0 = e.image[p8(p.id, 0)];
        p.p8_1 = e.image[p8(p.id, 1)];
        p.p8_3 = e.image[p8(p.id, 3)];
        p.p8_4 = e.image[p8(p.id, 4)];
        p.p8_5 = e.image[p8(p.id, 5)];
        p.p8_6 = e.image[p8(p.id, 6)];
        p.p8_7 = e.image[p8(p.id, 7)];
        p.p15_6 = e.image[p15(p.id, 6)];
        p.p15_8 = e.image[p15(p.id, 8)];
        p.p15_10 = e.image[p15(p.id, 10)];
        p.p15_11 = e.image[p15(p.id, 11)];
        p.p15_14 = e.image[p15(p.id, 14)];
    }
    for r in &mut s.rosters {
        for (i, id) in r.players.iter_mut().enumerate() {
            *id = word(e, roster(r.club, i));
        }
    }
    s.selectors.head = word(e, 0xD072);
    for c in &mut s.selectors.cells {
        c.value = e.image[0xCF71 + usize::from(c.index)];
    }
    s.managed_clubs.copy_from_slice(&e.image[0xDCBA..0xDCBC]);
    s.mode = e.image[0xD074];
    s.pending = e.image[0xD6E2];
    s.cf53 = e.image[0xCF53];
    s.sub_flags.copy_from_slice(&e.image[0xD6DE..0xD6E0]);
    s.reserves.copy_from_slice(&e.image[0xD6E0..0xD6E2]);
    s.added = word(e, 0xD0B4);
    s.active = e.image[0xD0B8];
    s.control = e.image[0xD0BA];
    s.clear_word = word(e, 0x0D2E);
    s.target_word = word(e, 0x0D30);
    for r in &mut s.scene_extra {
        let a = scene(r.index);
        r.word8 = word(e, a + 8);
        r.motion = e.image[a + 19];
        r.motion_word = word(e, a + 20);
    }
    for side in 0..2 {
        for i in 0..3 {
            s.counters[side][i] = e.image[0xD0D9 + 10 * side + i];
        }
        for i in 0..20 {
            s.cache[side][i] = e.image[0xD198 + 20 * side + i];
            s.event_totals[side][i] = word(e, 0xD148 + 40 * side + 2 * i);
        }
        s.event_counts[side] = word(e, 0xD1C0 + 2 * side);
    }
    for w in &mut s.weights {
        w.value = e.image[0x097A + usize::from(w.code)];
    }
    s.queue.head = word(e, 0x0380);
    s.queue.rows = template
        .queue
        .rows
        .iter()
        .map(|r| queue_row(e, r.index))
        .collect();
    s.source_pair = [word(e, 0xD08A), word(e, 0xD08C)];
    s.target_pair = [word(e, 0xD09A), word(e, 0xD09C)];
    s
}
fn blank(s: &State) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed: s.handoff.seed,
        ratings: [0; 8],
        goals: [0; 2],
    })
}
pub fn handler(input: &State) -> Result<Outcome, String> {
    validate(input)?;
    let mut e = blank(input);
    load(&mut e, input, false)?;
    e.logical_event_loaded(input)
}
#[derive(Debug, Serialize)]
pub struct ComposedOutcome {
    pub predecessor: extra_event::ComposedOutcome,
    pub effects: Outcome,
}
pub fn composed(
    placement: &placement::Input,
    prefix: &step_tail::PrefixInput,
    decision: &extra_event::Input,
    input: &State,
) -> Result<ComposedOutcome, String> {
    validate(input)?;
    if input.rosters[0].players != placement.rosters[1]
        || input.rosters[1].players != placement.rosters[0]
    {
        return Err("effect rosters conflict with placement input".into());
    }
    let (mut e, predecessor) = extra_event::composed_engine(placement, prefix, decision)?;
    let request = predecessor
        .decision
        .dispatch
        .as_ref()
        .ok_or("actual DBC6 did not dispatch 69B8")?;
    let effects = e.apply_event_effects(input, request)?;
    Ok(ComposedOutcome {
        predecessor,
        effects,
    })
}

impl Engine {
    pub fn apply_event_effects(
        &mut self,
        input: &State,
        request: &extra_event::Request,
    ) -> Result<Outcome, String> {
        validate(input)?;
        if request.routine != "0AB9:69B8"
            || request.input != input.handoff
            || extra_event::project(self, &input.handoff) != request.input
        {
            return Err("actual DBC6 seed/actor/player/scene/club request differs".into());
        }
        let mut trial = Engine {
            image: self.image.clone(),
        };
        load(&mut trial, input, true)?;
        let out = trial.logical_event_loaded(input)?;
        *self = trial;
        Ok(out)
    }
    fn logical_event_loaded(&mut self, template: &State) -> Result<Outcome, String> {
        let mut out = Outcome {
            output: template.clone(),
            selectors: vec![],
            draws: vec![],
            actions: vec![],
            helpers: vec![],
            events: vec![],
            external_calls: vec![],
        };
        let first = self.effect_selector("0AB9:69D3", &mut out);
        let secondary = self.image[0xD0A0];
        if first < 4 {
            let id = word(self, scene(secondary) + 14);
            let routine = if self.image[p15(id, 11)] & 0x10 != 0 {
                "0AB9:6589"
            } else {
                "0AB9:63E3"
            };
            self.effect_action(template, routine, secondary, &mut out)?;
        } else if self.draw(32, "0AB9:6A15", &mut out.draws) == 1 {
            self.effect_action(template, "0AB9:6589", secondary, &mut out)?;
        }
        if self.effect_selector("0AB9:6A25", &mut out) == 1 {
            let secondary = self.image[0xD0A0];
            let actor = self.image[0xD09F];
            let other = self.effect_threshold(word(self, scene(secondary) + 14), &mut out);
            let own = self.effect_threshold(word(self, scene(actor) + 14), &mut out);
            self.effect_action(
                template,
                "0AB9:6849",
                if own > other { secondary } else { actor },
                &mut out,
            )?;
        }
        out.output = project(self, template);
        Ok(out)
    }
    fn effect_selector(&mut self, site: &str, out: &mut Outcome) -> u8 {
        let before = word(self, 0xD072);
        let value = self.image[0xCF71 + usize::from(before)];
        let after = next_selector(before);
        put(self, 0xD072, after);
        out.selectors.push(Selector {
            site: site.into(),
            head_before: before,
            head_after: after,
            value,
        });
        value
    }
    fn effect_managed(&self, club: u16) -> bool {
        u16::from(self.image[0xDCBA]) == club || u16::from(self.image[0xDCBB]) == club
    }
    fn effect_available(&self, id: u16) -> u8 {
        placement::unavailable(&placement::Player {
            id,
            p15_sbyte8: self.image[p15(id, 8)] as i8,
            p15_byte11: self.image[p15(id, 11)],
        })
    }
    fn effect_availability(&self, index: u8, out: &mut Outcome) -> u8 {
        let id = word(self, scene(index) + 14);
        let result = self.effect_available(id);
        out.helpers.push(json!({"kind":"availability","scene":index,"player":id,"byte8":self.image[p15(id,8)],"flags11":self.image[p15(id,11)],"result":result}));
        result
    }
    fn effect_threshold(&self, id: u16, out: &mut Outcome) -> u16 {
        let result = extra_event::threshold(self, id);
        out.helpers
            .push(json!({"kind":"threshold","player":id,"result":result}));
        result
    }
    fn effect_budget(&self, side: usize) -> bool {
        let a = 0xD0DA + 10 * side;
        let count = u16::from(self.image[a]) + u16::from(self.image[a + 1]);
        let club = self.word(0xD6E4 + 2 * side as u16);
        if self.effect_managed(club) {
            count < 3
        } else {
            (0..20)
                .filter(|&i| self.effect_available(word(self, roster(club, i))) == 0)
                .count()
                > 11
                && count < 3
        }
    }
    fn effect_byte6(&mut self, index: u8, delta: i16, flag: u8, out: &mut Outcome) {
        let a = scene(index);
        let id = word(self, a + 14);
        let slot = self.image[a + 7];
        let side = usize::from(slot < 12);
        let norm = usize::from(if slot > 11 { slot - 11 } else { slot });
        let cache = 0xD197 + 20 * side + norm;
        let input = goal_effects::Byte6Input {
            old_byte6: self.image[p15(id, 6)],
            delta,
            cache_flag: flag,
            cache_before: self.image[cache],
        };
        let output = goal_effects::byte6(&input);
        self.image[p15(id, 6)] = output.byte6;
        if flag != 0 {
            self.image[cache] = output.cache;
        }
        out.helpers.push(json!({"kind":"byte6","scene":index,"player":id,"cache_side":side,"cache_slot":norm,"input":input,"output":output}));
    }
    fn effect_added(&mut self) {
        let value = word(self, 0xD0B4);
        if value < 5 {
            put(self, 0xD0B4, value + 1);
        }
    }
    fn effect_clear(&mut self, index: u8) {
        let a = scene(index);
        if word(self, a) != word(self, 0x0D2E) {
            self.image[0xCF53] = 1;
            put(self, a + 8, word(self, 0x0D30));
            put(self, a + 20, 0);
            put(self, a, word(self, 0x0D2E));
            self.image[a + 19] = 0;
        }
    }
    fn effect_event(&mut self, code: u8, target: Option<u8>, out: &mut Outcome) {
        let normalized = |slot: u8| u16::from(if slot > 11 { slot - 11 } else { slot });
        let (side, actor, secondary) = if let Some(index) = target {
            let slot = self.image[scene(index) + 7];
            (u8::from(slot < 12), normalized(slot), 0)
        } else {
            let other = self.image[0xD0A0];
            let actor = self.image[0xD09F];
            (
                u8::from(other < 12),
                normalized(self.image[scene(other) + 7]),
                normalized(self.image[scene(actor) + 7]),
            )
        };
        out.events.push(goal_effects::numeric_commentary(
            self, code, actor, secondary, side,
        ));
    }
    fn effect_action(
        &mut self,
        template: &State,
        routine: &str,
        index: u8,
        out: &mut Outcome,
    ) -> Result<(), String> {
        let input = project(self, template);
        let a = scene(index);
        let id = word(self, a + 14);
        let side = usize::from(self.image[a + 7] < 12);
        if routine == "0AB9:63E3" {
            self.effect_event(50, None, out);
            self.image[p15(id, 11)] |= 0x10;
            let c = 0xD0D9 + 10 * side;
            self.image[c] = self.image[c].wrapping_add(1);
            self.effect_added();
            if !self.effect_managed(u16::from(self.image[p8(id, 0)])) {
                self.image[p15(id, 10)] = (self.image[p15(id, 10)] | 0x80) & !8;
            }
        } else if routine == "0AB9:6589" {
            if self.effect_budget(side) {
                if !self.effect_managed(u16::from(self.image[p8(id, 0)])) {
                    return Err("untranslated AI A837/A94B roster effects".into());
                }
                self.effect_event(51, None, out);
                self.image[p15(id, 11)] = (self.image[p15(id, 11)] | 0x20) & !0x10;
                let c = 0xD0DA + 10 * side;
                self.image[c] = self.image[c].wrapping_add(1);
                self.effect_clear(self.image[a + 7]);
                self.effect_added();
                self.effect_byte6(index, -2, 1, out);
                let offset = self.image[0x0D56 + (side ^ 1)];
                for n in 1..=11 {
                    let i = offset + n;
                    if self.effect_availability(i, out) == 0 {
                        self.effect_byte6(i, 2, 0, out);
                    }
                }
                if self.image[p8(id, 6)] == 0 {
                    self.image[0xD6E2] = 1;
                }
            }
        } else if routine == "0AB9:6849" {
            if self.effect_availability(index, out) == 0 && self.effect_budget(side) {
                if !self.effect_managed(u16::from(self.image[p8(id, 0)])) {
                    return Err("untranslated AI A94B injury roster effects".into());
                }
                self.effect_event(52, Some(index), out);
                self.image[p15(id, 8)] = 0;
                let threshold = self.effect_threshold(id, out);
                let value = self.draw(20u16.wrapping_sub(threshold), "1998:13EE", &mut out.draws);
                self.image[p15(id, 14)] = value as u8;
                self.effect_clear(self.image[a + 7]);
                self.effect_byte6(index, -2, 1, out);
                self.effect_added();
                let c = 0xD0DB + 10 * side;
                self.image[c] = self.image[c].wrapping_add(1);
                if self.image[p8(id, 6)] == 0 || self.image[0xD6E0 + side] == 0 {
                    self.image[0xD6E2] = 1;
                }
            }
        } else {
            return Err("unknown effect handler".into());
        }
        out.actions.push(Action {
            routine: routine.into(),
            scene: index,
            input,
            output: project(self, template),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn synchronize(s: &mut State) {
        let ids = s
            .handoff
            .scenes
            .iter()
            .map(|r| r.player)
            .collect::<BTreeSet<_>>();
        s.handoff.players = ids
            .into_iter()
            .map(|id| {
                let p = s.players.iter().find(|p| p.id == id).unwrap();
                extra_event::Player {
                    id,
                    p8_0: p.p8_0,
                    p8_1: p.p8_1,
                    p8_3: p.p8_3,
                    p8_4: p.p8_4,
                    p8_5: p.p8_5,
                    p8_7: p.p8_7,
                    p15_6: p.p15_6,
                    p15_8: p.p15_8,
                    p15_10: p.p15_10,
                    p15_11: p.p15_11,
                    skills: [
                        extra_event::Skill {
                            mode: 6,
                            style_cell: 6,
                            level_cell: 32,
                        },
                        extra_event::Skill {
                            mode: 14,
                            style_cell: 14,
                            level_cell: 32,
                        },
                    ],
                }
            })
            .collect();
    }
    fn state() -> State {
        let c: serde_json::Value = serde_json::from_str(
            include_str!("../tests/fixtures/event_effects/T19-r0002.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let mut s: State = serde_json::from_value(c["input"].clone()).unwrap();
        s.managed_clubs = [s.handoff.clubs[0].id as u8, s.handoff.clubs[1].id as u8];
        s.selectors.cells[0].value = 0;
        s.selectors.cells[1].value = 0;
        s.handoff.seed = 0;
        s.counters = [[0; 3]; 2];
        s.cache = [[17; 20]; 2];
        s.event_totals = [[0; 20]; 2];
        s.event_counts = [0, 0];
        s.source_pair = [3, 4];
        s.target_pair = [8, 9];
        s.added = 0;
        s.pending = 0;
        s.cf53 = 0;
        s.reserves = [1, 1];
        s.active = 7;
        s.control = 9;
        s.clear_word = 0x1234;
        s.target_word = 0x4567;
        for p in &mut s.players {
            p.p8_1 = 25;
            p.p8_3 = 0;
            p.p8_5 = 10;
            p.p8_6 = 3;
            p.p15_6 = 10;
            p.p15_8 = 10;
            p.p15_10 = 0;
            p.p15_11 = 0;
        }
        synchronize(&mut s);
        validate(&s).unwrap();
        s
    }
    fn id(s: &State, index: u8) -> u16 {
        s.handoff.scenes[usize::from(index) - 1].player
    }
    fn player_mut(s: &mut State, id: u16) -> &mut Player {
        s.players.iter_mut().find(|p| p.id == id).unwrap()
    }
    fn loaded(s: &State) -> Engine {
        validate(s).unwrap();
        let mut e = blank(s);
        load(&mut e, s, false).unwrap();
        e
    }
    fn target_side(s: &State) -> usize {
        usize::from(s.handoff.scenes[usize::from(s.handoff.secondary) - 1].slot < 12)
    }
    fn red_state() -> State {
        let mut s = state();
        let id = id(&s, s.handoff.secondary);
        player_mut(&mut s, id).p15_11 = 0x10;
        synchronize(&mut s);
        s
    }
    #[test]
    fn selector_wrap_is_independent_of_the_lcg() {
        let mut s = state();
        s.selectors = Selectors {
            head: 255,
            cells: [
                Cell {
                    index: 255,
                    value: 0,
                },
                Cell { index: 1, value: 0 },
            ],
        };
        let got = handler(&s).unwrap();
        assert_eq!(got.output.selectors.head, 2);
        assert_eq!(got.output.handoff.seed, s.handoff.seed);
        assert!(got.draws.is_empty());
        assert_eq!(got.selectors[0].head_after, 1);
        assert_eq!(got.selectors[1].head_before, 1);
    }
    #[test]
    fn chance_short_circuit_has_both_lcg_paths_and_seed_wrap() {
        let mut s = state();
        s.selectors.cells[0].value = 15;
        let mut seen = BTreeSet::new();
        for seed in 0..500 {
            s.handoff.seed = seed;
            let got = handler(&s).unwrap();
            assert_eq!(got.draws.len(), 1);
            assert_eq!(got.draws[0].site, "0AB9:6A15");
            assert_eq!(got.draws[0].n, 32);
            assert_eq!(got.selectors.len(), 2);
            seen.insert(!got.actions.is_empty());
            if got.draws[0].value == 1 {
                assert_eq!(got.actions[0].routine, "0AB9:6589");
            } else {
                assert!(got.actions.is_empty());
            }
            if seen.len() == 2 {
                break;
            }
        }
        assert_eq!(seen.len(), 2);
        s.handoff.seed = 65535;
        let got = handler(&s).unwrap();
        assert_eq!(got.draws[0].seed_after, 0xA052);
    }
    #[test]
    fn ai_caution_updates_raw_flags_while_managed_caution_preserves_them() {
        let mut s = state();
        s.managed_clubs = [0, 0];
        let target = id(&s, s.handoff.secondary);
        player_mut(&mut s, target).p15_10 = 8;
        synchronize(&mut s);
        let got = handler(&s).unwrap();
        let p = got.output.players.iter().find(|p| p.id == target).unwrap();
        assert_eq!(p.p15_10, 0x80);
        assert_eq!(p.p15_11, 0x10);
        s.managed_clubs = [s.handoff.clubs[0].id as u8, s.handoff.clubs[1].id as u8];
        let got = handler(&s).unwrap();
        assert_eq!(
            got.output
                .players
                .iter()
                .find(|p| p.id == target)
                .unwrap()
                .p15_10,
            8
        );
    }
    #[test]
    fn signed_event_weights_counter_wrap_queue_wrap_and_added_cap() {
        let mut s = state();
        s.queue.head = 1700;
        for (r, i) in s.queue.rows.iter_mut().zip([1700, 1, 2, 3]) {
            r.index = i;
        }
        let side = target_side(&s);
        s.counters[side][0] = 255;
        s.event_counts[side] = 65535;
        s.event_totals[side][5] = 10;
        s.added = 4;
        let got = handler(&s).unwrap();
        assert_eq!(got.output.queue.head, 1);
        assert_eq!(got.output.queue.rows[1].code, 50);
        assert_eq!(got.output.queue.rows[2].code, 255);
        assert_eq!(
            got.output.queue.rows[2].first_player,
            s.queue.rows[2].first_player
        );
        assert_eq!(got.output.counters[side][0], 0);
        assert_eq!(got.output.event_counts[side], 0);
        assert_eq!(got.output.event_totals[side][5], 65526);
        assert_eq!(got.output.target_pair, s.source_pair);
        assert_eq!(got.output.added, 5);
        for added in [5, 6, 65535] {
            s.added = added;
            assert_eq!(handler(&s).unwrap().output.added, added);
        }
    }
    #[test]
    fn duplicate_numeric_event_still_weights_and_applies_player_effects() {
        let mut s = state();
        let side = usize::from(s.handoff.secondary < 12);
        let target_slot = s.handoff.scenes[usize::from(s.handoff.secondary) - 1].slot;
        let target_slot = usize::from(if target_slot > 11 {
            target_slot - 11
        } else {
            target_slot
        });
        let actor_slot = s.handoff.scenes[usize::from(s.handoff.actor) - 1].slot;
        let actor_slot = usize::from(if actor_slot > 11 {
            actor_slot - 11
        } else {
            actor_slot
        });
        let r = &mut s.queue.rows[0];
        r.code = 50;
        r.side = side as u8;
        r.first_player = s.rosters[side].players[target_slot - 1];
        r.second_player = s.rosters[side].players[actor_slot - 1];
        s.event_totals[side][target_slot - 1] = 10;
        s.event_counts[side] = 9;
        let got = handler(&s).unwrap();
        assert_eq!(got.events[0].accepted, 0);
        assert_eq!(got.output.queue, s.queue);
        assert_eq!(got.output.event_totals[side][target_slot - 1], 65526);
        assert_eq!(got.output.event_counts[side], 9);
        assert_eq!(got.output.target_pair, s.target_pair);
        assert_eq!(got.output.counters[target_side(&s)][0], 1);
        assert_eq!(
            got.output
                .players
                .iter()
                .find(|p| p.id == id(&s, s.handoff.secondary))
                .unwrap()
                .p15_11,
            0x10
        );
    }
    #[test]
    fn physical_event_side_and_scene_slot_counter_side_are_distinct() {
        let mut s = state();
        let i = usize::from(s.handoff.secondary) - 1;
        s.handoff.scenes[i].slot = 2;
        let got = handler(&s).unwrap();
        assert_eq!(
            (
                got.events[0].event_side,
                got.events[0].actor_slot,
                got.events[0].secondary_slot
            ),
            (0, 2, 7)
        );
        assert_eq!(got.output.counters[1][0], 1);
        assert_eq!(got.output.counters[0][0], 0);
    }
    #[test]
    fn dismissal_updates_shared_ids_sequentially_and_skips_newly_unavailable_aliases() {
        for alias_target in [false, true] {
            let mut s = red_state();
            let shared = if alias_target {
                id(&s, s.handoff.secondary)
            } else {
                id(&s, s.handoff.actor)
            };
            for r in &mut s.handoff.scenes[..11] {
                r.player = shared;
            }
            player_mut(&mut s, shared).p15_6 = 127;
            synchronize(&mut s);
            let got = handler(&s).unwrap();
            let p = got.output.players.iter().find(|p| p.id == shared).unwrap();
            if alias_target {
                assert_eq!(p.p15_6, 125);
                assert_eq!(got.helpers.len(), 12);
                assert!(got.helpers.iter().skip(1).all(|h| h["result"] == 1));
            } else {
                assert_eq!(p.p15_6, 19);
                assert_eq!(got.helpers.len(), 23);
                let updates: Vec<_> = got
                    .helpers
                    .iter()
                    .filter(|h| h["kind"] == "byte6" && h["player"] == shared)
                    .map(|h| h["output"]["byte6"].as_u64().unwrap())
                    .collect();
                assert_eq!(updates, [129, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
            }
        }
    }
    #[test]
    fn scene_clear_uses_the_passed_slot_preserves_clock_fields_and_is_idempotent() {
        let mut s = red_state();
        let target = usize::from(s.handoff.secondary) - 1;
        s.handoff.scenes[target].slot = 18;
        s.handoff.scenes[17].word0 = s.clear_word ^ 1;
        s.scene_extra[17].word8 = 65535;
        s.scene_extra[17].motion = 9;
        s.scene_extra[17].motion_word = 65535;
        let old = s.handoff.scenes[target].word0;
        let got = handler(&s).unwrap();
        assert_eq!(got.output.handoff.scenes[target].word0, old);
        assert_eq!(got.output.handoff.scenes[17].word0, s.clear_word);
        assert_eq!(
            (
                got.output.scene_extra[17].word8,
                got.output.scene_extra[17].motion,
                got.output.scene_extra[17].motion_word
            ),
            (s.target_word, 0, 0)
        );
        assert_eq!(
            (got.output.active, got.output.control, got.output.cf53),
            (s.active, s.control, 1)
        );
        s.handoff.scenes[17].word0 = s.clear_word;
        s.cf53 = 77;
        let got = handler(&s).unwrap();
        assert_eq!(got.output.scene_extra[17], s.scene_extra[17]);
        assert_eq!(got.output.cf53, 77);
    }
    #[test]
    fn managed_budget_blocks_effects_and_uses_word_sum_not_byte_wrap() {
        for (dismissed, injured) in [(2, 1), (255, 1)] {
            let mut s = red_state();
            let side = target_side(&s);
            s.counters[side][1] = dismissed;
            s.counters[side][2] = injured;
            let got = handler(&s).unwrap();
            assert_eq!(got.actions.len(), 1);
            assert!(got.events.is_empty());
            assert!(got.helpers.is_empty());
            assert_eq!(got.actions[0].input, got.actions[0].output);
            assert_eq!(got.output.players, s.players);
        }
    }
    fn injury_state() -> State {
        let mut s = state();
        s.selectors.cells[0].value = 15;
        s.selectors.cells[1].value = 1;
        s
    }
    #[test]
    fn managed_injury_draws_updates_cache_and_preserves_tie_choice() {
        let s = injury_state();
        let target = id(&s, s.handoff.actor);
        let got = handler(&s).unwrap();
        assert_eq!(got.actions.len(), 1);
        assert_eq!(got.actions[0].routine, "0AB9:6849");
        assert_eq!(got.actions[0].scene, s.handoff.actor);
        assert_eq!(
            got.draws
                .iter()
                .map(|d| d.site.as_str())
                .collect::<Vec<_>>(),
            ["0AB9:6A15", "1998:13EE"]
        );
        assert_eq!(got.draws[1].n, 10);
        let p = got.output.players.iter().find(|p| p.id == target).unwrap();
        assert_eq!(
            (p.p15_8, p.p15_6, p.p15_14),
            (0, 8, got.draws[1].value as u8)
        );
        assert_eq!(
            (
                got.output.cache[1][6],
                got.output.counters[1][2],
                got.output.added,
                got.output.pending
            ),
            (8, 1, 1, 0)
        );
        assert_eq!(
            (
                got.events[0].code,
                got.events[0].actor_slot,
                got.events[0].secondary_slot
            ),
            (52, 7, 0)
        );
    }
    #[test]
    fn injury_bounds_zero_and_underflow_are_word_sized_and_store_the_low_byte() {
        for (level, flag, bound) in [(20, 0, 0), (0, 1, 65301)] {
            let mut s = injury_state();
            for p in &mut s.players {
                p.p8_5 = level;
                p.p8_3 = flag;
            }
            synchronize(&mut s);
            let got = handler(&s).unwrap();
            assert_eq!(got.draws[1].n, bound);
            let p = got
                .output
                .players
                .iter()
                .find(|p| p.id == id(&s, s.handoff.actor))
                .unwrap();
            assert_eq!(p.p15_14, got.draws[1].value as u8);
            if bound == 0 {
                assert_eq!(p.p15_14, 1);
            }
        }
    }
    #[test]
    fn injury_availability_budget_and_pending_flags_take_their_own_paths() {
        let mut s = injury_state();
        let id = id(&s, s.handoff.actor);
        player_mut(&mut s, id).p15_8 = 0;
        synchronize(&mut s);
        let got = handler(&s).unwrap();
        assert!(got.events.is_empty());
        assert_eq!(got.draws.len(), 1);
        s = injury_state();
        s.counters[1][1] = 3;
        assert!(handler(&s).unwrap().events.is_empty());
        for (position, reserve) in [(0, 1), (3, 0)] {
            s = injury_state();
            player_mut(&mut s, id).p8_6 = position;
            s.reserves[1] = reserve;
            assert_eq!(handler(&s).unwrap().output.pending, 1);
        }
    }
    #[test]
    fn caution_then_injury_can_accept_two_numeric_events_in_one_window() {
        let mut s = state();
        s.selectors.cells[1].value = 1;
        let got = handler(&s).unwrap();
        assert_eq!(
            got.events.iter().map(|e| e.code).collect::<Vec<_>>(),
            [50, 52]
        );
        assert_eq!(got.output.queue.head, s.queue.head + 2);
        assert_eq!(got.output.queue.rows[3].code, 255);
        assert_eq!(got.selectors.len(), 2);
        assert_eq!(got.draws.len(), 1);
    }
    #[test]
    fn reached_ai_roster_changes_and_wrong_requests_are_transactional_diagnostics() {
        for injury in [false, true] {
            let mut s = if injury { injury_state() } else { red_state() };
            s.managed_clubs = [0, 0];
            let mut e = loaded(&s);
            let before = e.image.clone();
            let request = extra_event::Request {
                routine: "0AB9:69B8".into(),
                input: s.handoff.clone(),
            };
            assert!(e
                .apply_event_effects(&s, &request)
                .unwrap_err()
                .contains("AI"));
            assert_eq!(e.image, before);
        }
        let s = state();
        let mut e = loaded(&s);
        e.image[0x11000] = 0xAA;
        let before = e.image.clone();
        let mut request = extra_event::Request {
            routine: "0AB9:69B8".into(),
            input: s.handoff.clone(),
        };
        request.input.seed ^= 1;
        assert!(e.apply_event_effects(&s, &request).is_err());
        assert_eq!(e.image, before);
        request.input = s.handoff.clone();
        e.apply_event_effects(&s, &request).unwrap();
        assert_eq!(e.image[0x11000], 0xAA);
    }
    #[test]
    fn malformed_context_aliases_and_expected_inputs_are_rejected() {
        for damage in 0..8 {
            let mut s = state();
            match damage {
                0 => s.mode = 0,
                1 => s.selectors.head = 0,
                2 => s.selectors.cells[1].index = s.selectors.head,
                3 => {
                    s.queue.rows.pop();
                }
                4 => s.queue.head = 1701,
                5 => {
                    s.scene_extra.pop();
                }
                6 => s.weights[0].code = 49,
                _ => {
                    s.players.push(s.players[0].clone());
                }
            }
            assert!(handler(&s).is_err(), "damage {damage}");
        }
        let mut raw = serde_json::to_value(state()).unwrap();
        raw["expected_events"] = json!([]);
        assert!(serde_json::from_value::<State>(raw).is_err());
    }
}
