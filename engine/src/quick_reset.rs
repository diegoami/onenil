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
