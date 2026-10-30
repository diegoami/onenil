//! [confirmed] DBC6 decision/helpers: archaeology T17/F17 at 321b495;
//! reached raw D0B input: T18/F18 at 59eb0c4.
//! [designed] 69B8 is an emitted external request, not an executed handler.
use crate::{placement, step_tail, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub mode: u16,
    pub style_cell: u8,
    pub level_cell: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_0: u8,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_4: u8,
    pub p8_5: u8,
    pub p8_7: u8,
    pub p15_6: u8,
    pub p15_8: u8,
    pub p15_10: u8,
    pub p15_11: u8,
    pub skills: [Skill; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub index: u8,
    pub word0: u16,
    pub slot: u8,
    pub player: u16,
    pub rating_bonus: u8,
    pub half: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub byte57: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Flag {
    pub mode: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub actor: u8,
    pub secondary: u8,
    pub d6dd: u8,
    pub home_bonus: u8,
    pub side_offsets: [u8; 2],
    pub opponents: [u8; 10],
    pub clubs: [Club; 2],
    pub side_flags: [u8; 2],
    pub skill_bonus_flags: [Flag; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub state: State,
    pub opponent_d0b: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Helper {
    pub caller: String,
    pub kind: String,
    pub input: serde_json::Value,
    pub result: u16,
    pub draw_position: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Request {
    pub routine: String,
    pub input: State,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: State,
    pub draws: Vec<Draw>,
    pub helpers: Vec<Helper>,
    pub dispatch: Option<Request>,
}

const P8: usize = 0x30000;
const P15: usize = 0x20000;
const CLUB: usize = 0x40000;
fn scene(index: u8) -> usize {
    0xCCEF + usize::from(index) * 25
}
fn p8(id: u16, field: usize) -> usize {
    P8 + usize::from(id) * 8 + field
}
fn p15(id: u16, field: usize) -> usize {
    P15 + usize::from(id) * 15 + field
}
fn club(id: u16) -> usize {
    CLUB + (usize::from(id) - 1) * 62 + 57
}
fn word(e: &Engine, address: usize) -> u16 {
    u16::from_le_bytes([e.image[address], e.image[address + 1]])
}
fn put(e: &mut Engine, address: usize, value: u16) {
    e.image[address..address + 2].copy_from_slice(&value.to_le_bytes());
}

fn table_cells(s: &State) -> Result<BTreeMap<usize, u8>, String> {
    let mut cells = BTreeMap::new();
    for p in &s.players {
        // [designed] Bounded decoded tables keep physical far aliases outside this
        // named-pool projection. Original T17 cases are all in this domain.
        if p.p8_7 > 13 || p.p8_5 > 20 || p.skills[0].mode != 6 || p.skills[1].mode != 14 {
            return Err("unsupported style/level/mode lookup domain".into());
        }
        for k in &p.skills {
            for (a, value) in [
                (
                    0x957D + 22 * usize::from(p.p8_7) + usize::from(k.mode),
                    k.style_cell,
                ),
                (
                    0x96B1 + 22 * usize::from(p.p8_5) + usize::from(k.style_cell),
                    k.level_cell,
                ),
            ] {
                if cells.insert(a, value).is_some_and(|old| old != value) {
                    return Err("conflicting shared lookup cell".into());
                }
            }
        }
    }
    Ok(cells)
}
pub(crate) fn validate_state(s: &State) -> Result<(), String> {
    if s.scenes.len() != 22
        || s.scenes
            .iter()
            .enumerate()
            .any(|(i, r)| usize::from(r.index) != i + 1 || r.player > 3920)
    {
        return Err("complete ordered scenes required".into());
    }
    let mut ids = BTreeSet::new();
    for p in &s.players {
        if p.id > 3920 || !ids.insert(p.id) {
            return Err("unsupported or duplicate shared player".into());
        }
    }
    if ids != s.scenes.iter().map(|r| r.player).collect() {
        return Err("missing/unreferenced shared player".into());
    }
    if s.clubs.iter().any(|c| !(1..=176).contains(&c.id))
        || (s.clubs[0].id == s.clubs[1].id && s.clubs[0].byte57 != s.clubs[1].byte57)
        || s.skill_bonus_flags[0].mode != 6
        || s.skill_bonus_flags[1].mode != 14
    {
        return Err("unsupported club/bonus selectors or conflicting club alias".into());
    }
    table_cells(s)?;
    Ok(())
}
fn validate(input: &Input) -> Result<(), String> {
    validate_state(&input.state)?;
    let s = &input.state;
    if s.side_offsets[1] > 11
        || s.opponents[1..]
            .iter()
            .chain([&input.opponent_d0b])
            .any(|&v| {
                let index = u16::from(v) + u16::from(s.side_offsets[0]);
                !(1..=22).contains(&index)
            })
    {
        return Err("candidate outside physical scenes".into());
    }
    Ok(())
}

fn load(e: &mut Engine, input: &Input, preserve: bool) -> Result<(), String> {
    load_state(e, &input.state, preserve)?;
    e.image[0x0D0B] = input.opponent_d0b;
    Ok(())
}
pub(crate) fn load_state(e: &mut Engine, s: &State, preserve: bool) -> Result<(), String> {
    if !preserve {
        e.put_word(0xA51C, s.seed);
        e.image[0xD09F] = s.actor;
        e.image[0xD6DD] = s.d6dd;
        e.put_word(0xD6E4, s.clubs[0].id);
        e.put_word(0xD6E6, s.clubs[1].id);
    }
    e.image[0xD0A0] = s.secondary;
    e.image[0x0D29] = s.home_bonus;
    e.image[0x0D56..0x0D58].copy_from_slice(&s.side_offsets);
    e.image[0x0D01..0x0D0B].copy_from_slice(&s.opponents);
    e.image[0xD1CE..0xD1D0].copy_from_slice(&s.side_flags);
    for f in &s.skill_bonus_flags {
        e.image[0x09FB + usize::from(f.mode)] = f.value;
    }
    for c in &s.clubs {
        e.image[club(c.id)] = c.byte57;
    }
    for r in &s.scenes {
        let a = scene(r.index);
        if !preserve {
            put(e, a, r.word0);
            put(e, a + 14, r.player);
        }
        e.image[a + 7] = r.slot;
        e.image[a + 18] = r.rating_bonus;
        e.image[a + 22] = r.half;
    }
    for p in &s.players {
        for (i, v) in [
            (0, p.p8_0),
            (1, p.p8_1),
            (3, p.p8_3),
            (4, p.p8_4),
            (5, p.p8_5),
            (7, p.p8_7),
        ] {
            e.image[p8(p.id, i)] = v;
        }
        for (i, v) in [(6, p.p15_6), (8, p.p15_8), (10, p.p15_10), (11, p.p15_11)] {
            if !preserve || (i != 8 && i != 11) {
                e.image[p15(p.id, i)] = v;
            }
        }
    }
    for (address, value) in table_cells(s)? {
        e.image[address] = value;
    }
    Ok(())
}
pub(crate) fn project(e: &Engine, template: &State) -> State {
    let mut s = template.clone();
    s.seed = e.word(0xA51C);
    s.actor = e.image[0xD09F];
    s.secondary = e.image[0xD0A0];
    s.d6dd = e.image[0xD6DD];
    s.home_bonus = e.image[0x0D29];
    s.side_offsets.copy_from_slice(&e.image[0x0D56..0x0D58]);
    s.opponents.copy_from_slice(&e.image[0x0D01..0x0D0B]);
    s.side_flags.copy_from_slice(&e.image[0xD1CE..0xD1D0]);
    for (i, c) in s.clubs.iter_mut().enumerate() {
        c.id = e.word(0xD6E4 + 2 * i as u16);
        c.byte57 = e.image[club(c.id)];
    }
    for f in &mut s.skill_bonus_flags {
        f.value = e.image[0x09FB + usize::from(f.mode)];
    }
    for r in &mut s.scenes {
        let a = scene(r.index);
        r.word0 = word(e, a);
        r.player = word(e, a + 14);
        r.slot = e.image[a + 7];
        r.rating_bonus = e.image[a + 18];
        r.half = e.image[a + 22];
    }
    for p in &mut s.players {
        p.p8_0 = e.image[p8(p.id, 0)];
        p.p8_1 = e.image[p8(p.id, 1)];
        p.p8_3 = e.image[p8(p.id, 3)];
        p.p8_4 = e.image[p8(p.id, 4)];
        p.p8_5 = e.image[p8(p.id, 5)];
        p.p8_7 = e.image[p8(p.id, 7)];
        p.p15_6 = e.image[p15(p.id, 6)];
        p.p15_8 = e.image[p15(p.id, 8)];
        p.p15_10 = e.image[p15(p.id, 10)];
        p.p15_11 = e.image[p15(p.id, 11)];
        for k in &mut p.skills {
            k.style_cell = e.image[0x957D + 22 * usize::from(p.p8_7) + usize::from(k.mode)];
            k.level_cell = e.image[0x96B1 + 22 * usize::from(p.p8_5) + usize::from(k.style_cell)];
        }
    }
    s
}
fn unavailable(e: &Engine, index: u8) -> u16 {
    let id = word(e, scene(index) + 14);
    u16::from(placement::unavailable(&placement::Player {
        id,
        p15_sbyte8: e.image[p15(id, 8)] as i8,
        p15_byte11: e.image[p15(id, 11)],
    }))
}
fn predicate(e: &Engine, index: u8) -> u16 {
    // [confirmed] 1A4C(mode4), one-based bit membership. F17 at 321b495.
    if e.image[0xD6DD] == 0 {
        return 0;
    }
    let a = scene(index);
    let id = word(e, a + 14);
    let flags = e.image[p15(id, 10)];
    let source = if flags & 0x80 != 0 {
        flags
    } else {
        let side = usize::from(e.image[a + 7] < 12);
        e.image[club(e.word(0xD6E4 + 2 * side as u16))]
    };
    u16::from(source & 8 != 0)
}
pub(crate) fn threshold(e: &Engine, id: u16) -> u16 {
    let age = e.image[p8(id, 1)];
    let n = e.image[p8(id, 5)];
    if age < 18 {
        (i16::from(n) - 1 - 3 * (18 - i16::from(age))).max(0) as u8 as u16
    } else {
        u16::from(n.wrapping_sub(e.image[p8(id, 3)] & 1))
    }
}
fn rating(e: &Engine, index: u8, mode: u16, stack: Option<u16>) -> Result<u16, String> {
    let a = scene(index);
    let id = word(e, a + 14);
    let b6 = e.image[p15(id, 6)] as i8;
    let mut base = if b6 < 32 {
        (i16::from(b6) + 1) as u16
    } else {
        let side = usize::from(u16::from(e.image[p8(id, 0)]) == e.word(0xD6E6));
        if e.image[0xD1CE + side] != 0 {
            0
        } else {
            stack.ok_or("scene rating needs preexisting scratch local; composed propagation unsupported")?.wrapping_add(32)
        }
    };
    base = base.wrapping_add(e.image[p15(id, 8)] as i8 as i16 as u16);
    let style = e.image[0x957D + 22 * usize::from(e.image[p8(id, 7)]) + usize::from(mode)];
    let cell = e.image[0x96B1 + 22 * usize::from(e.image[p8(id, 5)]) + usize::from(style)];
    let mut skill = u16::from(cell).wrapping_mul(u16::from(e.image[p8(id, 4)]) + 1) >> 5;
    if e.image[a + 22] != 0 {
        skill >>= 1;
    }
    let mut value = (base.wrapping_mul(skill) >> 8).wrapping_add(skill >> 2);
    if e.image[0x09FB + usize::from(mode)] != 0 {
        value = value.wrapping_add(u16::from(e.image[a + 18]));
    }
    if e.image[a + 7] < 12 && e.image[0x0D29] != 0 {
        value = value.wrapping_add(value >> 4);
    }
    Ok(value.max(1))
}
fn blank(s: &State) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed: s.seed,
        ratings: [0; 8],
        goals: [0; 2],
    })
}

/// Pure helper call from legitimate raw state, without expected returns or draws.
pub fn helper(
    input: &Input,
    kind: &str,
    argument: u16,
    mode: u16,
    stack_word: Option<u16>,
) -> Result<u16, String> {
    validate(input)?;
    let mut e = blank(&input.state);
    load(&mut e, input, false)?;
    if kind == "threshold" {
        if mode != 0 {
            return Err("unsupported threshold mode".into());
        }
        if !input.state.players.iter().any(|p| p.id == argument) {
            return Err("missing threshold player".into());
        }
        return Ok(threshold(&e, argument));
    }
    if !(1..=22).contains(&argument) {
        return Err("helper scene outside physical pool".into());
    }
    match kind {
        "availability" if mode == 0 => Ok(unavailable(&e, argument as u8)),
        "predicate" if mode == 4 => Ok(predicate(&e, argument as u8)),
        "rating" if mode == 6 || mode == 14 => rating(&e, argument as u8, mode, stack_word),
        _ => Err("unsupported helper/mode".into()),
    }
}
pub fn decision(input: &Input) -> Result<Outcome, String> {
    validate(input)?;
    let mut e = blank(&input.state);
    load(&mut e, input, false)?;
    e.decide_loaded(&input.state)
}

#[derive(Debug, Serialize)]
pub struct ComposedOutcome {
    pub prefix: step_tail::ComposedOutcome,
    pub decision: Outcome,
}
pub fn composed(
    placement: &placement::Input,
    prefix: &step_tail::PrefixInput,
    input: &Input,
) -> Result<ComposedOutcome, String> {
    composed_engine(placement, prefix, input).map(|(_, outcome)| outcome)
}
pub(crate) fn composed_engine(
    placement: &placement::Input,
    prefix: &step_tail::PrefixInput,
    input: &Input,
) -> Result<(Engine, ComposedOutcome), String> {
    validate(input)?;
    if prefix.placement.goal_flag != 0 || prefix.d6dd == 0 {
        return Err("prefix does not request DBC6".into());
    }
    let (mut e, prefix) = step_tail::composed_prefix_engine(placement, prefix)?;
    let decision = e.apply_extra_event(input)?;
    Ok((e, ComposedOutcome { prefix, decision }))
}
impl Engine {
    /// Guard actual T05 fields before loading any additional raw inputs. Errors
    /// leave the caller's entire image unchanged, including reached scratch gaps.
    pub fn apply_extra_event(&mut self, input: &Input) -> Result<Outcome, String> {
        validate(input)?;
        let s = &input.state;
        if self.word(0xA51C) != s.seed
            || self.image[0xD09F] != s.actor
            || self.image[0xD6DD] != s.d6dd
            || self.word(0xD6E4) != s.clubs[0].id
            || self.word(0xD6E6) != s.clubs[1].id
            || s.scenes.iter().any(|r| {
                word(self, scene(r.index)) != r.word0 || word(self, scene(r.index) + 14) != r.player
            })
            || s.players.iter().any(|p| {
                self.image[p15(p.id, 8)] != p.p15_8 || self.image[p15(p.id, 11)] != p.p15_11
            })
        {
            return Err("actual prefix seed/actor/scene/player/club handoff differs".into());
        }
        let mut candidate = Engine {
            image: self.image.clone(),
        };
        load(&mut candidate, input, true)?;
        let out = candidate.decide_loaded(s)?;
        *self = candidate;
        Ok(out)
    }
    fn decide_loaded(&mut self, template: &State) -> Result<Outcome, String> {
        let mut draws = vec![];
        let mut helpers = vec![];
        let initial = self.draw(10, "0AB9:DBD1", &mut draws).wrapping_add(1);
        let candidates = [
            u16::from(self.image[0x0D00 + usize::from(initial)]) + u16::from(self.image[0x0D56]),
            initial + u16::from(self.image[0x0D57]),
        ];
        let mut add_helper = |e: &Engine,
                              caller: &str,
                              kind: &str,
                              arg: u8,
                              mode: u16,
                              result: u16,
                              position: usize| {
            let input = if kind == "threshold" {
                json!({"player":word(e,scene(arg)+14)})
            } else if mode == 0 {
                json!({"scene":arg})
            } else {
                json!({"scene":arg,"mode":mode})
            };
            helpers.push(Helper {
                caller: caller.into(),
                kind: kind.into(),
                input,
                result,
                draw_position: position,
            });
        };
        let first = candidates[1] as u8;
        let second = candidates[0] as u8;
        let available1 = unavailable(self, first);
        add_helper(
            self,
            "0AB9:DC17",
            "availability",
            first,
            0,
            available1,
            draws.len(),
        );
        let mut dispatch = false;
        if available1 == 0 {
            let available2 = unavailable(self, second);
            add_helper(
                self,
                "0AB9:DC2C",
                "availability",
                second,
                0,
                available2,
                draws.len(),
            );
            if available2 == 0 {
                let chosen = usize::from(self.draw(2, "0AB9:DC38", &mut draws) == 1);
                let own = candidates[chosen] as u8;
                let other = candidates[chosen ^ 1] as u8;
                let opposite = rating(self, other, 14, None)?;
                add_helper(
                    self,
                    "0AB9:DC71",
                    "rating",
                    other,
                    14,
                    opposite,
                    draws.len(),
                );
                let selected = rating(self, own, 6, None)?;
                add_helper(self, "0AB9:DC7D", "rating", own, 6, selected, draws.len());
                if rating_gate(selected, opposite) {
                    let flag = predicate(self, own);
                    add_helper(self, "0AB9:DC8D", "predicate", own, 4, flag, draws.len());
                    if flag != 0 || self.draw(20, "0AB9:DC98", &mut draws) == 1 {
                        let n = threshold(self, word(self, scene(other) + 14));
                        add_helper(self, "0AB9:DCB7", "threshold", other, 0, n, draws.len());
                        let opponent = self.draw(n, "0AB9:DCBF", &mut draws);
                        let n = threshold(self, word(self, scene(own) + 14));
                        add_helper(self, "0AB9:DCCC", "threshold", own, 0, n, draws.len());
                        let selected = self.draw(n, "0AB9:DCD4", &mut draws);
                        if signed_win(selected, opponent) {
                            self.image[0xD0A0] = own;
                            self.image[0xD09F] = other;
                            dispatch = true;
                        }
                    }
                }
            }
        }
        let output = project(self, template);
        let request = dispatch.then(|| Request {
            routine: "0AB9:69B8".into(),
            input: output.clone(),
        });
        Ok(Outcome {
            output,
            draws,
            helpers,
            dispatch: request,
        })
    }
}
fn signed_win(selected: u16, opposite: u16) -> bool {
    (selected as i16) > (opposite as i16)
}
fn rating_gate(selected: u16, opposite: u16) -> bool {
    selected < opposite
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Input {
        let players = (1..=2)
            .map(|id| Player {
                id,
                p8_0: id as u8,
                p8_1: 25,
                p8_3: 0,
                p8_4: 127,
                p8_5: id as u8,
                p8_7: id as u8,
                p15_6: 10,
                p15_8: 10,
                p15_10: 0,
                p15_11: 0,
                skills: [
                    Skill {
                        mode: 6,
                        style_cell: 6,
                        level_cell: 32,
                    },
                    Skill {
                        mode: 14,
                        style_cell: 14,
                        level_cell: 32,
                    },
                ],
            })
            .collect();
        Input {
            opponent_d0b: 2,
            state: State {
                seed: 0,
                actor: 9,
                secondary: 8,
                d6dd: 1,
                home_bonus: 0,
                side_offsets: [11, 0],
                opponents: [120, 11, 10, 9, 8, 7, 6, 5, 4, 3],
                clubs: [Club { id: 1, byte57: 0 }, Club { id: 2, byte57: 0 }],
                side_flags: [0, 0],
                skill_bonus_flags: [Flag { mode: 6, value: 0 }, Flag { mode: 14, value: 0 }],
                scenes: (1..=22)
                    .map(|index| Scene {
                        index,
                        word0: 17,
                        slot: index,
                        player: if index < 12 { 1 } else { 2 },
                        rating_bonus: 7,
                        half: 0,
                    })
                    .collect(),
                players,
            },
        }
    }
    fn loaded(input: &Input) -> Engine {
        validate(input).unwrap();
        let mut e = blank(&input.state);
        load(&mut e, input, false).unwrap();
        e
    }
    #[test]
    fn availability_exits_have_exact_draw_counts_and_do_not_mutate_records() {
        for (id, n) in [(1, 1), (2, 2)] {
            let mut x = input();
            x.state.players[id - 1].p15_8 = 0;
            let got = decision(&x).unwrap();
            assert_eq!(got.draws.len(), 1);
            assert_eq!(got.helpers.len(), n);
            assert!(got.dispatch.is_none());
            assert_eq!(got.output.players, x.state.players);
            assert_eq!(got.output.scenes, x.state.scenes);
        }
    }
    #[test]
    fn both_coin_selectors_and_equal_rating_exit_are_observed() {
        let mut sides = BTreeSet::new();
        for seed in 0..50 {
            let mut x = input();
            x.state.seed = seed;
            let got = decision(&x).unwrap();
            assert_eq!(got.draws.len(), 2);
            assert_eq!(got.helpers.len(), 4);
            assert!(got.dispatch.is_none());
            sides.insert(got.helpers[3].input["scene"].as_u64().unwrap() < 12);
        }
        assert_eq!(sides.len(), 2);
    }
    #[test]
    fn predicate_player_override_club_fallback_and_disabled_event_flag() {
        let mut x = input();
        x.state.clubs[1].byte57 = 8;
        assert_eq!(helper(&x, "predicate", 2, 4, None).unwrap(), 1);
        assert_eq!(helper(&x, "predicate", 13, 4, None).unwrap(), 0);
        x.state.players[0].p15_10 = 0x80;
        assert_eq!(helper(&x, "predicate", 2, 4, None).unwrap(), 0);
        x.state.players[0].p15_10 = 0x88;
        assert_eq!(helper(&x, "predicate", 2, 4, None).unwrap(), 1);
        x.state.d6dd = 0;
        assert_eq!(helper(&x, "predicate", 2, 4, None).unwrap(), 0);
    }
    #[test]
    fn rating_uses_signed_bytes_low_word_products_and_each_bonus_in_order() {
        let mut x = input();
        assert_eq!(helper(&x, "rating", 2, 6, None).unwrap(), 42);
        x.state.scenes[1].half = 1;
        assert_eq!(helper(&x, "rating", 2, 6, None).unwrap(), 21);
        x.state.skill_bonus_flags[0].value = 1;
        assert_eq!(helper(&x, "rating", 2, 6, None).unwrap(), 28);
        x.state.home_bonus = 1;
        assert_eq!(helper(&x, "rating", 2, 6, None).unwrap(), 29);
        x.state.home_bonus = 0;
        x.state.skill_bonus_flags[0].value = 0;
        x.state.scenes[1].half = 0;
        x.state.players[0].p15_6 = 255;
        x.state.players[0].p15_8 = 255;
        assert_eq!(helper(&x, "rating", 2, 6, None).unwrap(), 287);
        x.state.players[0].p8_4 = 0;
        x.state.players[0].skills[0].level_cell = 0;
        assert_eq!(helper(&x, "rating", 2, 6, None).unwrap(), 1);
    }
    #[test]
    fn scratch_local_is_raw_helper_input_and_composed_diagnostic_is_transactional() {
        let mut x = input();
        for p in &mut x.state.players {
            p.p15_6 = 32;
        }
        assert_eq!(helper(&x, "rating", 2, 6, Some(100)).unwrap(), 103);
        assert_eq!(helper(&x, "rating", 2, 6, Some(200)).unwrap(), 153);
        assert!(helper(&x, "rating", 2, 6, None)
            .unwrap_err()
            .contains("scratch"));
        let mut e = loaded(&x);
        let before = e.image.clone();
        assert!(e.apply_extra_event(&x).unwrap_err().contains("scratch"));
        assert_eq!(e.image, before);
        x.state.side_flags = [1, 1];
        assert_eq!(helper(&x, "rating", 2, 6, None).unwrap(), 37);
    }
    #[test]
    fn threshold_clamps_youth_branch_and_wraps_older_branch() {
        let mut e = blank(&input().state);
        for (age, n, flags, expected) in [
            (10, 20, 0, 0),
            (17, 5, 0, 1),
            (17, 4, 0, 0),
            (18, 0, 1, 255),
            (18, 5, 3, 4),
            (18, 5, 2, 5),
        ] {
            e.image[p8(1, 1)] = age;
            e.image[p8(1, 5)] = n;
            e.image[p8(1, 3)] = flags;
            assert_eq!(threshold(&e, 1), expected);
        }
    }
    fn contest_input() -> Input {
        let mut x = input();
        for p in &mut x.state.players {
            p.skills[0].level_cell = 16;
            p.skills[1].level_cell = 64;
        }
        x
    }
    #[test]
    fn contingent_chance_has_both_paths_and_predicate_short_circuits_it() {
        let mut chance_false = false;
        let mut chance_true = false;
        for seed in 0..1000 {
            let mut x = contest_input();
            x.state.seed = seed;
            let got = decision(&x).unwrap();
            assert_eq!(got.draws[2].site, "0AB9:DC98");
            if got.draws.len() == 3 {
                chance_false = true;
            } else {
                assert_eq!(got.draws.len(), 5);
                chance_true = true;
            }
            if chance_false && chance_true {
                break;
            }
        }
        assert!(chance_false && chance_true);
        let mut x = contest_input();
        for p in &mut x.state.players {
            p.p15_10 = 0x88;
        }
        let got = decision(&x).unwrap();
        assert_eq!(got.draws.len(), 4);
        assert!(!got.draws.iter().any(|d| d.site == "0AB9:DC98"));
    }
    #[test]
    fn zero_thresholds_still_draw_and_ties_do_not_dispatch() {
        let mut x = contest_input();
        for p in &mut x.state.players {
            p.p8_1 = 0;
            p.p8_5 = 0;
            p.p15_10 = 0x88;
        }
        let got = decision(&x).unwrap();
        assert_eq!(got.draws.len(), 4);
        assert!(got.dispatch.is_none());
        for d in &got.draws[2..] {
            assert_eq!((d.n, d.value), (0, 1));
            assert_ne!(d.seed_before, d.seed_after);
        }
    }
    #[test]
    fn signed_final_comparison_and_unsigned_rating_comparison_differ() {
        assert!(signed_win(1, 65535));
        assert!(!signed_win(0x8000, 32767));
        assert!(!signed_win(9, 9));
        assert!(!rating_gate(0x8000, 1));
        assert!(rating_gate(1, 0xFFFF));
        // A wrapped high rating remains a large unsigned rating, rather than
        // becoming negative and incorrectly reaching the event predicate.
        let mut x = input();
        x.state.players[0].p15_6 = 255;
        x.state.players[0].p15_8 = 255;
        let e = loaded(&x);
        assert!(rating(&e, 2, 6, None).unwrap() > rating(&e, 13, 14, None).unwrap());
    }
    #[test]
    fn canonical_aliases_are_shared_and_conflicts_are_rejected() {
        let mut x = input();
        for r in &mut x.state.scenes {
            r.player = 1;
        }
        x.state.players.pop();
        let got = decision(&x).unwrap();
        assert_eq!(got.output.players.len(), 1);
        assert_eq!(got.output.players, x.state.players);
        x.state.players.push(x.state.players[0].clone());
        assert!(decision(&x).is_err());
        let mut x = input();
        x.state.players[1].p8_7 = x.state.players[0].p8_7;
        x.state.players[1].skills[0].style_cell ^= 1;
        assert!(decision(&x).unwrap_err().contains("lookup"));
    }
    #[test]
    fn invalid_shapes_domains_and_unexpected_expected_fields_are_rejected() {
        for damage in 0..8 {
            let mut x = input();
            match damage {
                0 => {
                    x.state.scenes.pop();
                }
                1 => x.state.scenes.swap(0, 1),
                2 => {
                    x.state.players.pop();
                }
                3 => x.state.clubs[0].id = 0,
                4 => x.state.players[0].p8_5 = 255,
                5 => x.state.skill_bonus_flags[0].mode = 14,
                6 => x.opponent_d0b = 255,
                _ => x.state.side_offsets[1] = 255,
            }
            assert!(decision(&x).is_err(), "damage {damage}");
        }
        let mut raw = serde_json::to_value(input()).unwrap();
        raw["expected_helpers"] = json!([]);
        assert!(serde_json::from_value::<Input>(raw).is_err());
        assert!(helper(&input(), "rating", 2, 4, None).is_err());
        assert!(helper(&input(), "threshold", 3920, 0, None).is_err());
    }
    #[test]
    fn failed_handoff_preserves_entire_existing_image_and_unrelated_bytes() {
        let x = input();
        let mut e = loaded(&x);
        e.image[0x11000] = 0xAA;
        let before = e.image.clone();
        let mut bad = x.clone();
        bad.state.seed ^= 1;
        assert!(e.apply_extra_event(&bad).is_err());
        assert_eq!(e.image, before);
        let _ = e.apply_extra_event(&x).unwrap();
        assert_eq!(e.image[0x11000], 0xAA);
    }
    #[test]
    fn seed_wrap_and_the_reached_final_raw_lookup_cell_are_not_inferred() {
        let mut x = input();
        x.state.seed = 65535;
        let got = decision(&x).unwrap();
        assert_eq!(got.draws[0].seed_after, 0xA052);
        for seed in 0..100 {
            x.state.seed = seed;
            let a = decision(&x).unwrap();
            if a.draws[0].value == 10 {
                assert_eq!(a.helpers[1].input["scene"], 13);
                x.opponent_d0b = 3;
                let b = decision(&x).unwrap();
                assert_eq!(b.helpers[1].input["scene"], 14);
                return;
            }
        }
        panic!("no final table index exercised");
    }
}
