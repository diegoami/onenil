//! [confirmed] Named quick52B6 reset closure: archaeology T21/F21.
//! [designed] Rendering/timers,045C buffers,6B2F/78B7 and full DD02 stay external.
use crate::{goal_effects, placement, post_score, step_tail, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub index: u8,
    pub player: u16,
    pub slot: u8,
    pub word0: u16,
    pub word8: u16,
    pub word10: u16,
    pub motion: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub byte6: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub strategy: post_score::State,
    pub mode: u8,
    pub actor: u8,
    pub secondary: u8,
    pub active: u8,
    pub zero: u8,
    pub alternate: u8,
    pub kind: u8,
    pub remaining: u16,
    pub side_offsets: [u8; 2],
    pub signs: [u8; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub cache: [[u8; 20]; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CopyVisit {
    pub index: u8,
    pub word0: u16,
    pub before: u16,
    pub after: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MotionVisit {
    pub index: u8,
    pub before: u8,
    pub after: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Keeper {
    pub input: State,
    pub draw_start: usize,
    pub output: State,
    pub draw_end: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: State,
    pub copies: Vec<CopyVisit>,
    pub motions: Vec<MotionVisit>,
    pub keeper: Option<Keeper>,
    pub strategy_input: Option<State>,
    pub strategy_output: Option<State>,
    pub strategy: Option<post_score::Outcome>,
    pub helpers: Vec<serde_json::Value>,
    pub draws: Vec<Draw>,
}
const P15: usize = 0x20000;
fn scene(i: u8) -> usize {
    0xCCEF + 25 * usize::from(i)
}
fn player(id: u16, f: usize) -> usize {
    P15 + 15 * usize::from(id) + f
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes());
}
fn validate(s: &State) -> Result<(), String> {
    post_score::validate(&s.strategy)?;
    if s.mode == 0
        || !(1..=22).contains(&s.actor)
        || s.side_offsets != [11, 0]
        || s.scenes.len() != 22
        || s.scenes.iter().enumerate().any(|(i, r)| {
            usize::from(r.index) != i + 1 || r.player > 3920 || !(1..=22).contains(&r.slot)
        })
    {
        return Err("unsupported quick actor/physical offset/scene domain".into());
    }
    let mut ids = s
        .strategy
        .players
        .iter()
        .map(|p| p.id)
        .collect::<BTreeSet<_>>();
    ids.extend(s.scenes.iter().map(|r| r.player));
    let mut players = BTreeSet::new();
    for p in &s.players {
        if p.id > 3920 || !players.insert(p.id) {
            return Err("ambiguous reset player".into());
        }
    }
    if ids != players {
        return Err("missing/unreferenced reset player".into());
    }
    Ok(())
}
fn blank(s: &State) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed: s.strategy.seed,
        ratings: [0; 8],
        goals: [0; 2],
    })
}
fn load(e: &mut Engine, s: &State, preserve: bool) {
    post_score::load(e, &s.strategy, preserve);
    if !preserve {
        e.image[0xD09F] = s.actor;
        e.put_word(0xD0AE, s.remaining);
    }
    for (a, v) in [
        (0xD074, s.mode),
        (0xD0A0, s.secondary),
        (0xD0B8, s.active),
        (0xD0B9, s.zero),
        (0xD1CA, s.alternate),
        (0xD0AC, s.kind),
    ] {
        e.image[a] = v;
    }
    e.image[0x0D56..0x0D58].copy_from_slice(&s.side_offsets);
    e.image[0xA338..0xA33A].copy_from_slice(&s.signs);
    let known = s.scenes.iter().map(|r| r.player).collect::<BTreeSet<_>>();
    for r in &s.scenes {
        let a = scene(r.index);
        if !preserve {
            put(e, a, r.word0);
            put(e, a + 14, r.player);
        }
        e.image[a + 7] = r.slot;
        put(e, a + 8, r.word8);
        put(e, a + 10, r.word10);
        e.image[a + 19] = r.motion;
    }
    for p in &s.players {
        e.image[player(p.id, 6)] = p.byte6;
        if !preserve || !known.contains(&p.id) {
            e.image[player(p.id, 8)] = p.byte8;
            e.image[player(p.id, 11)] = p.flags11;
        }
    }
    for side in 0..2 {
        e.image[0xD198 + 20 * side..0xD198 + 20 * (side + 1)].copy_from_slice(&s.cache[side]);
    }
}
fn project(e: &Engine, template: &State) -> State {
    let mut s = template.clone();
    s.strategy = post_score::project(e, &template.strategy);
    s.mode = e.image[0xD074];
    s.actor = e.image[0xD09F];
    s.secondary = e.image[0xD0A0];
    s.active = e.image[0xD0B8];
    s.zero = e.image[0xD0B9];
    s.alternate = e.image[0xD1CA];
    s.kind = e.image[0xD0AC];
    s.remaining = e.word(0xD0AE);
    s.side_offsets.copy_from_slice(&e.image[0x0D56..0x0D58]);
    s.signs.copy_from_slice(&e.image[0xA338..0xA33A]);
    for r in &mut s.scenes {
        let a = scene(r.index);
        r.player = word(e, a + 14);
        r.slot = e.image[a + 7];
        r.word0 = word(e, a);
        r.word8 = word(e, a + 8);
        r.word10 = word(e, a + 10);
        r.motion = e.image[a + 19];
    }
    for p in &mut s.players {
        p.byte6 = e.image[player(p.id, 6)];
        p.byte8 = e.image[player(p.id, 8)];
        p.flags11 = e.image[player(p.id, 11)];
    }
    for side in 0..2 {
        s.cache[side].copy_from_slice(&e.image[0xD198 + 20 * side..0xD198 + 20 * (side + 1)]);
    }
    s
}
pub fn reset(input: &State) -> Result<Outcome, String> {
    validate(input)?;
    let mut e = blank(input);
    load(&mut e, input, false);
    e.reset_loaded(input)
}
#[derive(Debug, Serialize)]
pub struct ComposedOutcome {
    pub predecessor: step_tail::ComposedOutcome,
    pub reset: Outcome,
}
pub fn composed(
    placement: &placement::Input,
    prefix: &step_tail::PrefixInput,
    input: &State,
) -> Result<ComposedOutcome, String> {
    validate(input)?;
    if prefix.placement.goal_flag != 1 || prefix.mode == 0 {
        return Err("reset composition requires quick goal path".into());
    }
    if input.strategy.clubs[0].roster != placement.rosters[1]
        || input.strategy.clubs[1].roster != placement.rosters[0]
    {
        return Err("reset roster/placement conflict".into());
    }
    let (mut e, predecessor) = step_tail::composed_prefix_engine(placement, prefix)?;
    let reset = e.apply_quick_reset(input)?;
    Ok(ComposedOutcome { predecessor, reset })
}
impl Engine {
    pub fn apply_quick_reset(&mut self, input: &State) -> Result<Outcome, String> {
        validate(input)?;
        let s = &input.strategy;
        let players = input
            .players
            .iter()
            .map(|p| (p.id, p))
            .collect::<BTreeMap<_, _>>();
        if self.word(0xA51C) != s.seed
            || self.image[0xD0C4..0xD0C6] != s.scores
            || self.word(0xD6E4) != s.clubs[0].id
            || self.word(0xD6E6) != s.clubs[1].id
            || self.image[0xD09F] != input.actor
            || self.word(0xD0AE) != input.remaining
            || input.scenes.iter().any(|r| {
                word(self, scene(r.index)) != r.word0 || word(self, scene(r.index) + 14) != r.player
            })
            || input.scenes.iter().any(|r| {
                self.image[player(r.player, 8)] != players[&r.player].byte8
                    || self.image[player(r.player, 11)] != players[&r.player].flags11
            })
        {
            return Err(
                "actual prefix reset seed/score/actor/club/scene/eligibility handoff differs"
                    .into(),
            );
        }
        let mut trial = Engine {
            image: self.image.clone(),
        };
        load(&mut trial, input, true);
        let result = trial.reset_loaded(input)?;
        *self = trial;
        Ok(result)
    }
    fn reset_loaded(&mut self, template: &State) -> Result<Outcome, String> {
        let mut out = Outcome {
            output: template.clone(),
            copies: vec![],
            motions: vec![],
            keeper: None,
            strategy_input: None,
            strategy_output: None,
            strategy: None,
            helpers: vec![],
            draws: vec![],
        };
        self.image[0xD0B8] = 1;
        for index in 1..=22 {
            let a = scene(index);
            let value = word(self, a);
            let before = word(self, a + 8);
            put(self, a + 8, value);
            out.copies.push(CopyVisit {
                index,
                word0: value,
                before,
                after: word(self, a + 8),
            });
        }
        if self.image[0xD0B9] == 0 {
            let actor_side = self.image[0xD09F] < 12;
            for index in 1..=22 {
                let a = scene(index);
                let before = self.image[a + 19];
                let after = if (self.image[a + 7] < 12) == actor_side && index != 1 && index != 12 {
                    6
                } else {
                    0
                };
                self.image[a + 19] = after;
                out.motions.push(MotionVisit {
                    index,
                    before,
                    after,
                });
            }
            let before = project(self, template);
            let start = out.draws.len();
            self.reset_keeper(&mut out)?;
            out.keeper = Some(Keeper {
                input: before,
                draw_start: start,
                output: project(self, template),
                draw_end: out.draws.len(),
            });
            out.strategy_input = Some(project(self, template));
            let strategy = self.post_score_loaded(&template.strategy);
            out.draws.extend(strategy.draws.iter().cloned());
            out.strategy = Some(strategy);
            out.strategy_output = Some(project(self, template));
            for index in 1..=22 {
                if self.reset_available(index, &mut out) == 0 {
                    let equal = (self.image[scene(index) + 7] < 12) == (self.image[0xD09F] < 12);
                    let selector = usize::from(equal) ^ usize::from(self.image[0xD1CA] != 0);
                    let delta = i16::from(self.image[0xA338 + selector] as i8);
                    self.reset_byte6(index, delta, &mut out);
                }
            }
        }
        out.output = project(self, template);
        Ok(out)
    }
    fn reset_available(&self, index: u8, out: &mut Outcome) -> u8 {
        let id = word(self, scene(index) + 14);
        let byte8 = self.image[player(id, 8)];
        let flags11 = self.image[player(id, 11)];
        let result = placement::unavailable(&placement::Player {
            id,
            p15_sbyte8: byte8 as i8,
            p15_byte11: flags11,
        });
        out.helpers.push(json!({"kind":"availability","scene":index,"player":id,"byte8":byte8,"flags11":flags11,"result":result}));
        result
    }
    fn reset_factor(&self, index: u8) -> i16 {
        // [confirmed] 1ECC compares as signed32, with remaining zero-extended.
        let later = i32::from(self.word(0xD0AE)) > i32::from(self.image[0xD0AC] as i8);
        let side = self.image[scene(index) + 7] < 12;
        let selector = usize::from(later) ^ usize::from(side);
        i16::from(self.image[0xA338 + selector] as i8)
    }
    fn reset_keeper(&mut self, out: &mut Outcome) -> Result<(), String> {
        let actor = self.image[0xD09F];
        if actor == 1 || actor == 12 {
            let a = scene(actor);
            let pair = word(self, a + 10).to_le_bytes();
            let transformed = [
                (i16::from(pair[0] as i8).wrapping_mul(self.reset_factor(actor))) as u8,
                (i16::from(pair[1] as i8).wrapping_mul(self.reset_factor(actor))) as u8,
            ];
            put(self, a + 8, u16::from_le_bytes(transformed));
            let offset = self.image[0x0D56 + usize::from(actor < 12)];
            let candidate = self.draw(10, "0AB9:4FDF", &mut out.draws) + 1 + u16::from(offset);
            self.image[0xD09F] = candidate as u8;
            let mut found = false;
            for _ in 0..10 {
                let i = self.image[0xD09F];
                if self.reset_available(i, out) == 0 {
                    found = true;
                    break;
                }
                let offset = self.image[0x0D56 + usize::from(i < 12)];
                let next = if i == offset + 11 { offset + 2 } else { i + 1 };
                self.image[0xD09F] = next;
            }
            if !found {
                return Err("no eligible keeper replacement in physical cycle".into());
            }
        }
        let a = scene(self.image[0xD09F]);
        put(self, a + 8, word(self, a));
        Ok(())
    }
    fn reset_byte6(&mut self, index: u8, delta: i16, out: &mut Outcome) {
        let a = scene(index);
        let id = word(self, a + 14);
        let slot = self.image[a + 7];
        let side = usize::from(slot < 12);
        let norm = usize::from(if slot > 11 { slot - 11 } else { slot });
        let address = 0xD197 + 20 * side + norm;
        let input = goal_effects::Byte6Input {
            old_byte6: self.image[player(id, 6)],
            delta,
            cache_flag: 0,
            cache_before: self.image[address],
        };
        let output = goal_effects::byte6(&input);
        self.image[player(id, 6)] = output.byte6;
        out.helpers.push(json!({"kind":"byte6","scene":index,"player":id,"cache_side":side,"cache_slot":norm,"input":input,"output":output}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> State {
        let c: serde_json::Value = serde_json::from_str(
            include_str!("../tests/fixtures/quick_reset/T21-r0001.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let mut s: State = serde_json::from_value(c["input"].clone()).unwrap();
        s.strategy.seed = 0;
        s.strategy.managed_clubs = [s.strategy.clubs[0].id as u8, s.strategy.clubs[1].id as u8];
        s.actor = 2;
        s.active = 0;
        s.zero = 0;
        s.alternate = 0;
        s.kind = 45;
        s.remaining = 44;
        s.signs = [255, 1];
        s.cache = [[17; 20]; 2];
        for r in &mut s.scenes {
            r.slot = r.index;
            r.word0 = 1000 + u16::from(r.index);
            r.word8 = 2000 + u16::from(r.index);
            r.word10 = 0x7F80;
            r.motion = 99;
        }
        for p in &mut s.players {
            p.byte6 = 10;
            p.byte8 = 10;
            p.flags11 = 0;
        }
        s
    }
    fn id(s: &State, index: u8) -> u16 {
        s.scenes[usize::from(index) - 1].player
    }
    fn p_mut(s: &mut State, id: u16) -> &mut Player {
        s.players.iter_mut().find(|p| p.id == id).unwrap()
    }
    fn p(s: &State, id: u16) -> &Player {
        s.players.iter().find(|p| p.id == id).unwrap()
    }
    fn loaded(s: &State) -> Engine {
        validate(s).unwrap();
        let mut e = blank(s);
        load(&mut e, s, false);
        e
    }
    #[test]
    fn zero_flag_sets_active_and_copies_every_position_without_other_work() {
        let mut s = input();
        s.actor = 1;
        s.zero = 2;
        for p in &mut s.players {
            p.byte8 = 0;
        }
        let got = reset(&s).unwrap();
        assert_eq!(got.output.active, 1);
        assert_eq!(got.copies.len(), 22);
        assert!(
            got.motions.is_empty()
                && got.keeper.is_none()
                && got.strategy.is_none()
                && got.helpers.is_empty()
                && got.draws.is_empty()
        );
        assert_eq!(got.output.players, s.players);
        assert_eq!(got.output.cache, s.cache);
        assert_eq!(got.output.actor, 1);
        for (a, b) in got.output.scenes.iter().zip(&s.scenes) {
            assert_eq!(a.word8, b.word0);
            assert_eq!(a.motion, b.motion);
        }
    }
    #[test]
    fn physical_keeper_exclusion_and_stored_slot_sides_are_independent() {
        let mut s = input();
        s.scenes[1].slot = 13;
        s.scenes[12].slot = 2;
        let got = reset(&s).unwrap();
        assert_eq!(got.output.scenes[0].motion, 0);
        assert_eq!(got.output.scenes[11].motion, 0);
        assert_eq!(got.output.scenes[1].motion, 0);
        assert_eq!(got.output.scenes[12].motion, 6);
        assert_eq!(p(&got.output, id(&s, 2)).byte6, 9);
        assert_eq!(p(&got.output, id(&s, 13)).byte6, 11);
        assert_eq!(got.output.cache, s.cache);
    }
    #[test]
    fn both_keeper_preludes_transform_the_pair_and_select_on_the_shared_seed() {
        for (actor, replacement, coordinate) in [(1, 3, 0x7F80), (12, 14, 0x8180)] {
            let mut s = input();
            s.actor = actor;
            let got = reset(&s).unwrap();
            assert_eq!(got.draws.len(), 1);
            assert_eq!(
                (
                    got.draws[0].site.as_str(),
                    got.draws[0].n,
                    got.draws[0].value,
                    got.draws[0].seed_after
                ),
                ("0AB9:4FDF", 10, 2, 0x1B0F)
            );
            let k = got.keeper.as_ref().unwrap();
            assert_eq!(
                (k.input.actor, k.output.actor, k.draw_start, k.draw_end),
                (actor, replacement, 0, 1)
            );
            assert_eq!(k.input.strategy.seed, 0);
            assert_eq!(k.output.strategy.seed, 0x1B0F);
            assert_eq!(got.strategy_input.as_ref().unwrap().strategy.seed, 0x1B0F);
            assert_eq!(got.output.scenes[usize::from(actor) - 1].word8, coordinate);
            assert_eq!(
                got.output.scenes[usize::from(replacement) - 1].word8,
                1000 + u16::from(replacement)
            );
            assert_eq!(got.output.actor, replacement);
        }
    }
    #[test]
    fn pair_factor_uses_signed_kind_signed_bytes_and_low_byte_products() {
        let mut s = input();
        s.actor = 1;
        s.kind = 128;
        s.remaining = 0;
        assert_eq!(reset(&s).unwrap().output.scenes[0].word8, 0x8180);
        s.signs = [128, 3];
        assert_eq!(reset(&s).unwrap().output.scenes[0].word8, 0x8000);
        s.signs = [255, 1];
        s.kind = 45;
        s.remaining = 45;
        assert_eq!(reset(&s).unwrap().output.scenes[0].word8, 0x7F80);
        s.remaining = 46;
        assert_eq!(reset(&s).unwrap().output.scenes[0].word8, 0x8180);
        s.remaining = 44;
        s.scenes[0].slot = 13;
        assert_eq!(reset(&s).unwrap().output.scenes[0].word8, 0x8180);
    }
    #[test]
    fn keeper_retry_observes_current_eligibility_without_redrawing() {
        let mut s = input();
        s.actor = 1;
        for (index, byte8, flags11) in [(3, 0, 0), (4, 10, 0x20), (5, 255, 0)] {
            let id = id(&s, index);
            let q = p_mut(&mut s, id);
            q.byte8 = byte8;
            q.flags11 = flags11;
        }
        let got = reset(&s).unwrap();
        assert_eq!(got.output.actor, 6);
        assert_eq!(got.draws.len(), 1);
        let scenes: Vec<_> = got
            .helpers
            .iter()
            .take(4)
            .map(|h| h["scene"].as_u64().unwrap())
            .collect();
        assert_eq!(scenes, [3, 4, 5, 6]);
        assert_eq!(got.keeper.unwrap().output.actor, 6);
    }
    #[test]
    fn keeper_cycle_wraps_from_eleven_back_to_two() {
        let mut s = input();
        s.actor = 1;
        for seed in 0..100 {
            s.strategy.seed = seed;
            let baseline = reset(&s).unwrap();
            if baseline.draws[0].value == 10 {
                for index in [11, 2] {
                    let id = id(&s, index);
                    p_mut(&mut s, id).byte8 = 0;
                }
                let got = reset(&s).unwrap();
                assert_eq!(got.output.actor, 3);
                assert_eq!(
                    got.helpers
                        .iter()
                        .take(3)
                        .map(|h| h["scene"].as_u64().unwrap())
                        .collect::<Vec<_>>(),
                    [11, 2, 3]
                );
                return;
            }
        }
        panic!("no wrap candidate exercised");
    }
    #[test]
    fn no_eligible_keeper_cycle_is_a_transactional_diagnostic() {
        for actor in [1, 12] {
            let mut s = input();
            s.actor = actor;
            let indices = if actor == 1 { 2..=11 } else { 13..=22 };
            for i in indices {
                let id = id(&s, i);
                p_mut(&mut s, id).byte8 = 0;
            }
            let mut e = loaded(&s);
            let before = e.image.clone();
            assert!(e.apply_quick_reset(&s).unwrap_err().contains("no eligible"));
            assert_eq!(e.image, before);
            s.zero = 1;
            assert!(reset(&s).is_ok());
        }
    }
    #[test]
    fn strategy_uses_actual_post_keeper_seed_without_resetting_it() {
        let mut s = input();
        s.actor = 1;
        s.strategy.managed_clubs = [0, 0];
        s.strategy.scores = [0, 2];
        s.strategy.limits = [255, 255];
        let got = reset(&s).unwrap();
        let strategy = got.strategy.as_ref().unwrap();
        assert_eq!(strategy.draws.len(), 6);
        assert_eq!(got.draws.len(), 7);
        assert_eq!(strategy.draws[0].seed_before, got.draws[0].seed_after);
        assert_eq!(
            got.output.strategy.seed,
            got.draws.last().unwrap().seed_after
        );
        assert_eq!(strategy.output.seed, got.output.strategy.seed);
        assert_ne!(
            got.strategy_input.as_ref().unwrap().strategy.seed,
            s.strategy.seed
        );
    }
    #[test]
    fn unavailable_and_zero_id_players_are_skipped_but_still_have_motion() {
        for (byte8, flags11) in [(0, 0), (128, 0), (10, 0x80), (10, 0x20), (10, 2), (10, 4)] {
            let mut s = input();
            let id = id(&s, 2);
            let q = p_mut(&mut s, id);
            q.byte8 = byte8;
            q.flags11 = flags11;
            let got = reset(&s).unwrap();
            assert_eq!(p(&got.output, id).byte6, 10);
            assert_eq!(got.output.scenes[1].motion, 6);
            assert!(!got
                .helpers
                .iter()
                .any(|h| h["kind"] == "byte6" && h["scene"] == 2));
        }
        let mut s = input();
        s.scenes[1].player = 0;
        if !s.players.iter().any(|p| p.id == 0) {
            s.players.push(Player {
                id: 0,
                byte6: 10,
                byte8: 10,
                flags11: 0,
            });
        }
        let got = reset(&s).unwrap();
        assert_eq!(p(&got.output, 0).byte6, 10);
    }
    #[test]
    fn repeated_ids_reload_signed_byte6_and_never_write_the_cache() {
        let mut s = input();
        let shared = id(&s, 2);
        for r in &mut s.scenes[1..11] {
            r.player = shared;
        }
        p_mut(&mut s, shared).byte6 = 127;
        let got = reset(&s).unwrap();
        assert_eq!(p(&got.output, shared).byte6, 9);
        assert_eq!(got.output.cache, s.cache);
        let updates: Vec<_> = got
            .helpers
            .iter()
            .filter(|h| h["kind"] == "byte6" && h["player"] == shared)
            .map(|h| h["output"]["byte6"].as_u64().unwrap())
            .collect();
        assert_eq!(updates, [128, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn alternate_inverts_live_table_delta_and_all_clamps_remain_signed() {
        let mut s = input();
        s.alternate = 1;
        let got = reset(&s).unwrap();
        assert_eq!(p(&got.output, id(&s, 2)).byte6, 9);
        assert_eq!(p(&got.output, id(&s, 13)).byte6, 11);
        let own = id(&s, 2);
        p_mut(&mut s, own).byte6 = 200;
        assert_eq!(p(&reset(&s).unwrap().output, own).byte6, 1);
        s.alternate = 0;
        p_mut(&mut s, own).byte6 = 0;
        assert_eq!(p(&reset(&s).unwrap().output, own).byte6, 1);
    }
    #[test]
    fn malformed_domains_and_expected_intermediate_fields_are_rejected() {
        for damage in 0..7 {
            let mut s = input();
            match damage {
                0 => s.mode = 0,
                1 => s.actor = 0,
                2 => s.side_offsets = [0, 11],
                3 => s.scenes[0].slot = 0,
                4 => {
                    s.scenes.pop();
                }
                5 => {
                    s.players.push(s.players[0].clone());
                }
                _ => s.scenes.swap(0, 1),
            }
            assert!(reset(&s).is_err(), "damage {damage}");
        }
        let mut value = serde_json::to_value(input()).unwrap();
        value["strategy_input"] = json!({});
        assert!(serde_json::from_value::<State>(value).is_err());
        let mut value = serde_json::to_value(input()).unwrap();
        value["players"][0]["byte8"] = json!(256);
        assert!(serde_json::from_value::<State>(value).is_err());
    }
    #[test]
    fn invalid_handoffs_preserve_the_entire_image_and_success_preserves_unowned_fields() {
        let s = input();
        let mut e = loaded(&s);
        e.image[0x11000] = 0xAA;
        put(&mut e, player(s.players[0].id, 2), 0xBEEF);
        let before = e.image.clone();
        let mut bad = s.clone();
        bad.strategy.seed ^= 1;
        assert!(e.apply_quick_reset(&bad).is_err());
        assert_eq!(e.image, before);
        e.apply_quick_reset(&s).unwrap();
        assert_eq!(e.image[0x11000], 0xAA);
        assert_eq!(word(&e, player(s.players[0].id, 2)), 0xBEEF);
    }
}
