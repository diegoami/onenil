//! [confirmed] Logical 5083/67D0 effects: archaeology F12/F13 at a39ac70;
//! T14/F14 provides the complete original before/after projection.
//! [designed] Text and render buffers stay external; logical cache/totals and
//! numeric commentary bookkeeping are part of this projection. No RNG is drawn.

use crate::{placement::GoalEffectRequest, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Flags {
    pub d1ca: u8,
    pub d6dd: u8,
    pub d0b7: u8,
    pub d0b9: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Scene {
    pub index: u8,
    pub player: u16,
    pub slot: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Player {
    pub id: u16,
    pub p8_position: u8,
    pub p8_flag7: u8,
    pub p15_word2: u16,
    pub p15_byte5: u8,
    pub p15_byte6: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Log {
    pub player: u16,
    pub minute_byte: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QueueRow {
    pub index: u16,
    pub code: u8,
    pub first_player: u16,
    pub second_player: u16,
    pub side: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Queue {
    pub head: u16,
    pub rows: Vec<QueueRow>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct State {
    pub global_actor: u8,
    pub secondary_actor: u8,
    pub clubs: [u16; 2],
    pub side_offsets: [u8; 2],
    pub flags: Flags,
    pub position_deltas: [i8; 14],
    pub event_weights: [i8; 2],
    pub scenes: Vec<Scene>,
    pub rosters: [[u16; 20]; 2],
    pub players: Vec<Player>,
    pub event_totals: [[u16; 20]; 2],
    pub cache: [[u8; 20]; 2],
    pub event_counts: [u16; 2],
    pub side_counts: [u8; 2],
    pub log_counts: [u8; 2],
    pub logs: [[Log; 11]; 2],
    pub source_pair: [u16; 2],
    pub target_pair: [u16; 2],
    pub queue: Queue,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Commentary {
    pub code: u8,
    pub actor_slot: u16,
    pub secondary_slot: u16,
    pub event_side: u8,
    pub accepted: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub state: State,
    pub commentary: Commentary,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Byte6Input {
    pub old_byte6: u8,
    pub delta: i16,
    pub cache_flag: u8,
    pub cache_before: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Byte6Output {
    pub byte6: u8,
    pub cache: u8,
}
#[derive(Clone, Debug, Deserialize)]
pub struct HelperCall {
    pub input: Byte6Input,
    pub output: Byte6Output,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Case {
    pub schema: String,
    pub week: String,
    pub step: usize,
    pub request: GoalEffectRequest,
    pub before: State,
    pub after: State,
    pub commentary: Commentary,
    pub helper_calls: Vec<HelperCall>,
    pub source: serde_json::Value,
}
#[derive(Clone, Debug, Deserialize)]
pub struct HelperCase {
    pub schema: String,
    pub week: String,
    pub step: usize,
    pub input: Byte6Input,
    pub output: Byte6Output,
}

pub fn byte6(_input: &Byte6Input) -> Byte6Output {
    todo!("T03 original 67D0 replay must fail before implementation")
}

pub fn apply_goal_effects(state: &State, request: &GoalEffectRequest) -> Result<Outcome, String> {
    let draw = request.draws.first().ok_or("consumed coin draw missing")?;
    let mut engine = Engine::new(&ContestInput {
        k: 0,
        seed: draw.seed_after,
        ratings: [0; 8],
        goals: [0; 2],
    });
    engine.image[0xD09F] = state.global_actor;
    engine.apply_goal_effects(state, request)
}

impl Engine {
    pub fn apply_goal_effects(
        &mut self,
        _state: &State,
        _request: &GoalEffectRequest,
    ) -> Result<Outcome, String> {
        todo!("T03 original logical goal-effect replay must fail before implementation")
    }
    pub fn goal_effect_seed(&self) -> u16 {
        self.word(super::SEED)
    }
}

pub fn replay(case: &Case) -> Result<(), String> {
    if case.schema != "goal-effects-case/1" {
        return Err("unknown goal-effect schema".into());
    }
    let got = apply_goal_effects(&case.before, &case.request)?;
    if got.state != case.after {
        return Err(format!(
            "{} step {}: logical state differs",
            case.week, case.step
        ));
    }
    if got.commentary != case.commentary {
        return Err(format!(
            "{} step {}: numeric commentary differs",
            case.week, case.step
        ));
    }
    for (i, h) in case.helper_calls.iter().enumerate() {
        if byte6(&h.input) != h.output {
            return Err(format!(
                "{} step {}: helper {i} differs",
                case.week, case.step
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        placement::{GoalEffectRequest, GoalEvent},
        Draw,
    };
    fn state() -> State {
        let mut deltas = [1; 14];
        deltas[12] = 2;
        deltas[13] = 3;
        State {
            global_actor: 9,
            secondary_actor: 0,
            clubs: [1, 2],
            side_offsets: [0, 11],
            flags: Flags {
                d1ca: 0,
                d6dd: 1,
                d0b7: 0,
                d0b9: 0,
            },
            position_deltas: deltas,
            event_weights: [5, -3],
            scenes: (1..=22)
                .map(|i| Scene {
                    index: i,
                    player: i as u16,
                    slot: i,
                })
                .collect(),
            rosters: [
                std::array::from_fn(|i| i as u16 + 1),
                std::array::from_fn(|i| i as u16 + 12),
            ],
            players: (1..=31)
                .map(|id| Player {
                    id,
                    p8_position: if id == 1 || id == 12 { 0 } else { 12 },
                    p8_flag7: if id == 1 || id == 12 { 0 } else { 1 },
                    p15_word2: 100,
                    p15_byte5: 4,
                    p15_byte6: 10,
                })
                .collect(),
            event_totals: [[100; 20]; 2],
            cache: [[10; 20]; 2],
            event_counts: [1000, 2000],
            side_counts: [10, 20],
            log_counts: [0, 0],
            logs: std::array::from_fn(|_| {
                std::array::from_fn(|_| Log {
                    player: 0,
                    minute_byte: 0,
                })
            }),
            source_pair: [25, 42],
            target_pair: [3, 4],
            queue: Queue {
                head: 0,
                rows: (0..=2)
                    .map(|index| QueueRow {
                        index,
                        code: 255,
                        first_player: 0,
                        second_player: 0,
                        side: 0,
                    })
                    .collect(),
            },
        }
    }
    fn request(coin: u8) -> GoalEffectRequest {
        let before: u16 = if coin == 1 { 0 } else { 1 };
        let after = before.wrapping_mul(0x7ABD).wrapping_add(0x1B0F);
        let value = (((after as u32 * 2) >> 16) + 1) as u16;
        GoalEffectRequest {
            event: GoalEvent {
                minute: 27,
                actor: 9,
                side_flag: 1,
            },
            coin,
            draws: vec![Draw {
                site: "0AB9:5190".into(),
                n: 2,
                seed_before: before,
                seed_after: after,
                value,
            }],
        }
    }
    fn player(s: &State, id: u16) -> &Player {
        s.players.iter().find(|p| p.id == id).unwrap()
    }
    fn player_mut(s: &mut State, id: u16) -> &mut Player {
        s.players.iter_mut().find(|p| p.id == id).unwrap()
    }
    #[test]
    fn byte6_signed_word_clamp_and_signed_cache_reload() {
        for (old, delta, expected) in [
            (127, 3, 130),
            (255, -1, 1),
            (127, 32767, 1),
            (100, 200, 255),
        ] {
            let got = byte6(&Byte6Input {
                old_byte6: old,
                delta,
                cache_flag: 1,
                cache_before: 77,
            });
            assert_eq!(got.byte6, expected);
            assert_eq!(
                got.cache,
                if expected == 130 || expected == 255 {
                    expected
                } else {
                    1
                }
            );
        }
        assert_eq!(
            byte6(&Byte6Input {
                old_byte6: 10,
                delta: -1,
                cache_flag: 0,
                cache_before: 77
            }),
            Byte6Output {
                byte6: 9,
                cache: 77
            }
        );
    }
    #[test]
    fn standard_goal_updates_both_roles_and_logical_commentary() {
        let got = apply_goal_effects(&state(), &request(1)).unwrap();
        assert_eq!(player(&got.state, 9).p15_byte6, 12);
        assert_eq!(player(&got.state, 12).p15_byte6, 9);
        assert_eq!(
            (
                player(&got.state, 9).p15_word2,
                player(&got.state, 9).p15_byte5
            ),
            (101, 5)
        );
        assert_eq!(
            (
                player(&got.state, 12).p15_word2,
                player(&got.state, 12).p15_byte5
            ),
            (101, 5)
        );
        assert_eq!(got.state.cache[1][8], 12);
        assert_eq!(got.state.cache[0][0], 9);
        assert_eq!(got.state.event_totals[1][8], 105);
        assert_eq!(got.state.event_counts, [1000, 2001]);
        assert_eq!(got.state.side_counts, [10, 21]);
        assert_eq!(got.state.log_counts, [0, 1]);
        assert_eq!(
            got.state.logs[1][0],
            Log {
                player: 9,
                minute_byte: 27
            }
        );
        assert_eq!(got.state.queue.head, 1);
        assert_eq!(
            got.state.queue.rows[1],
            QueueRow {
                index: 1,
                code: 38,
                first_player: 9,
                second_player: 0,
                side: 1
            }
        );
        assert_eq!(got.state.target_pair, [25, 42]);
        assert_eq!(
            got.commentary,
            Commentary {
                code: 38,
                actor_slot: 9,
                secondary_slot: 0,
                event_side: 1,
                accepted: 1
            }
        );
    }
    #[test]
    fn zero_coin_still_updates_keeper_but_preserves_its_cache() {
        let got = apply_goal_effects(&state(), &request(0)).unwrap();
        assert_eq!(player(&got.state, 12).p15_byte6, 9);
        assert_eq!(got.state.cache[0][0], 10);
    }
    #[test]
    fn aliased_players_observe_sequential_mutations_in_one_record() {
        let mut s = state();
        s.rosters[1][0] = 9;
        s.scenes[11].player = 9;
        player_mut(&mut s, 9).p15_byte6 = 127;
        let got = apply_goal_effects(&s, &request(1)).unwrap();
        assert_eq!(player(&got.state, 9).p15_byte6, 1);
        assert_eq!(player(&got.state, 9).p15_word2, 101);
        assert_eq!(got.state.cache[1][8], 129);
        assert_eq!(got.state.cache[0][0], 1);
    }
    #[test]
    fn alternate_flag_flips_log_side_but_preserves_original_roster_lookup() {
        let mut s = state();
        s.flags.d1ca = 255;
        let mut r = request(1);
        r.event.minute = 128;
        let got = apply_goal_effects(&s, &r).unwrap();
        assert_eq!(player(&got.state, 9).p15_word2, 100);
        assert_eq!(player(&got.state, 12).p15_word2, 101);
        assert_eq!(player(&got.state, 9).p15_byte6, 9);
        assert_eq!(got.state.event_totals[1][8], 97);
        assert_eq!(
            got.state.logs[0][0],
            Log {
                player: 9,
                minute_byte: 0
            }
        );
        assert_eq!(got.state.side_counts, s.side_counts);
    }
    #[test]
    fn independent_global_actor_scene_slot_and_keeper_selectors() {
        let mut s = state();
        s.global_actor = 12;
        s.secondary_actor = 8;
        s.scenes[8].slot = 14;
        s.scenes[11].slot = 2;
        s.scenes[11].player = 13;
        let got = apply_goal_effects(&s, &request(1)).unwrap();
        assert_eq!(
            got.commentary,
            Commentary {
                code: 38,
                actor_slot: 2,
                secondary_slot: 8,
                event_side: 0,
                accepted: 1
            }
        );
        assert_eq!(got.state.queue.rows[1].first_player, 13);
        assert_eq!(got.state.queue.rows[1].second_player, 19);
        assert_eq!(got.state.logs[1][0].player, 3);
        assert_eq!(got.state.cache[0][2], 12);
        assert_eq!(player(&got.state, 12).p15_word2, 101);
        assert_eq!(player(&got.state, 12).p15_byte6, 10);
        assert_eq!(player(&got.state, 13).p15_byte6, 9);
    }
    #[test]
    fn duplicate_commentary_suppresses_queue_count_not_weight_or_player_effects() {
        let mut s = state();
        s.queue.head = 1;
        s.queue.rows = (1..=3)
            .map(|index| QueueRow {
                index,
                code: 255,
                first_player: 0,
                second_player: 0,
                side: 0,
            })
            .collect();
        s.queue.rows[0] = QueueRow {
            index: 1,
            code: 38,
            first_player: 9,
            second_player: 0,
            side: 1,
        };
        let got = apply_goal_effects(&s, &request(1)).unwrap();
        assert_eq!(got.commentary.accepted, 0);
        assert_eq!(got.state.queue, s.queue);
        assert_eq!(got.state.event_counts, s.event_counts);
        assert_eq!(got.state.target_pair, s.target_pair);
        assert_eq!(got.state.event_totals[1][8], 105);
        assert_eq!(player(&got.state, 9).p15_byte6, 12);
    }
    #[test]
    fn log_cap_overwrites_last_and_minute_marker_uses_wrapping_addition() {
        let mut s = state();
        s.flags.d0b7 = 1;
        s.log_counts[1] = 11;
        s.logs[1][10] = Log {
            player: 55,
            minute_byte: 5,
        };
        let mut r = request(1);
        r.event.minute = 128;
        let got = apply_goal_effects(&s, &r).unwrap();
        assert_eq!(got.state.log_counts[1], 11);
        assert_eq!(
            got.state.logs[1][10],
            Log {
                player: 9,
                minute_byte: 0
            }
        );
        assert_eq!(&got.state.logs[1][..10], &s.logs[1][..10]);
    }
    #[test]
    fn counters_and_queue_wrap_with_only_sentinel_code_replaced() {
        let mut s = state();
        for id in [9, 12] {
            let p = player_mut(&mut s, id);
            p.p15_word2 = 65535;
            p.p15_byte5 = 255;
        }
        s.event_totals[1][8] = 65535;
        s.event_counts[1] = 65535;
        s.side_counts[1] = 255;
        s.queue.head = 1700;
        s.queue.rows = [1700, 1, 2]
            .into_iter()
            .map(|index| QueueRow {
                index,
                code: 44,
                first_player: 5000,
                second_player: 6000,
                side: 200,
            })
            .collect();
        let got = apply_goal_effects(&s, &request(1)).unwrap();
        assert_eq!(player(&got.state, 9).p15_word2, 0);
        assert_eq!(player(&got.state, 12).p15_byte5, 0);
        assert_eq!(got.state.event_totals[1][8], 4);
        assert_eq!(got.state.event_counts[1], 0);
        assert_eq!(got.state.side_counts[1], 0);
        assert_eq!(got.state.queue.head, 1);
        assert_eq!(
            got.state.queue.rows[2],
            QueueRow {
                index: 2,
                code: 255,
                first_player: 5000,
                second_player: 6000,
                side: 200
            }
        );
    }
    #[test]
    fn finished_clock_suppresses_only_side_counter() {
        let mut s = state();
        s.flags.d0b9 = 1;
        let got = apply_goal_effects(&s, &request(1)).unwrap();
        assert_eq!(got.state.side_counts, s.side_counts);
        assert_eq!(got.state.event_totals[1][8], 105);
        assert_eq!(got.state.log_counts[1], 1);
    }
    #[test]
    fn invalid_handoff_and_missing_data_are_errors() {
        let s = state();
        let r = request(1);
        let mut bad = s.clone();
        bad.players.retain(|p| p.id != 9);
        assert!(apply_goal_effects(&bad, &r).is_err());
        let mut bad = r.clone();
        bad.coin = 0;
        assert!(apply_goal_effects(&s, &bad).is_err());
        for actor in [0, 23] {
            let mut bad = r.clone();
            bad.event.actor = actor;
            assert!(apply_goal_effects(&s, &bad).is_err());
        }
        let mut e = Engine::new(&ContestInput {
            k: 0,
            seed: 0,
            ratings: [0; 8],
            goals: [0; 2],
        });
        e.image[0xD09F] = s.global_actor;
        assert!(e.apply_goal_effects(&s, &r).is_err());
    }
}
