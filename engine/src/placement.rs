//! [confirmed] DD7C..DF35 and 4F89/5C6E: archaeology F12 at 77841e9;
//! T13/F13 at a39ac70 supplies complete original inputs and callback observations.
//!
//! [designed] The persistent seed/generated/actor fields use the existing DS image.
//! Saved scores and minute arrays represent temporary Pascal frame fields. A goal
//! emits the modeled 5083 RNG projection and an effect request; its player/log/UI
//! writes remain outside this slice.

use crate::{Draw, Engine, Input as ContestInput, Output as ContestOutput};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Player {
    pub id: u16,
    pub p15_sbyte8: i8,
    pub p15_byte11: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Input {
    pub seed: u16,
    pub generated: [u8; 2],
    pub scores: [u8; 2],
    pub elapsed: u16,
    pub minute_limit: u16,
    pub side_offsets: [u8; 2],
    pub actor_before: u8,
    pub scorer_slots: [u8; 30],
    pub rosters: [[u16; 20]; 2],
    pub players: Vec<Player>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GoalEvent {
    pub minute: u16,
    pub actor: u8,
    pub side_flag: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
/// [designed] A pending non-RNG 5083 effect. Its coin draw has already advanced
/// the shared seed; a future statistics/log/display handler must reuse `coin`.
pub struct GoalEffectRequest {
    pub event: GoalEvent,
    pub coin: u8,
    pub draws: Vec<Draw>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MinuteFill {
    pub outer_side: u8,
    pub side: u8,
    pub goal_index: u8,
    pub minute: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Attempt {
    pub side: u8,
    pub sampled_slot: u16,
    pub player: u16,
    pub unavailable: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Mapping {
    pub player: u16,
    pub side: u8,
    pub first_slot: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Output {
    pub seed: u16,
    pub generated: [u8; 2],
    pub scores: [u8; 2],
    pub goal_flag: u8,
    pub actor: u8,
    pub goal: Option<GoalEvent>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: Output,
    pub draws: Vec<Draw>,
    pub callbacks: Vec<GoalEffectRequest>,
    pub minute_fills: Vec<MinuteFill>,
    pub attempts: Vec<Attempt>,
    pub mappings: Vec<Mapping>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContestObservation {
    pub input: ContestInput,
    pub output: ContestOutput,
    pub draws: Vec<Draw>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Case {
    pub schema: String,
    pub week: String,
    pub step: usize,
    pub input: Input,
    pub output: Output,
    pub draws: Vec<Draw>,
    pub callbacks: Vec<GoalEffectRequest>,
    pub minute_fills: Vec<MinuteFill>,
    pub attempts: Vec<Attempt>,
    pub mappings: Vec<Mapping>,
    pub contest: ContestObservation,
    pub source: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AvailabilityCase {
    pub schema: String,
    pub week: String,
    pub step: usize,
    pub scene_slot: u8,
    pub input: Player,
    pub unavailable: u8,
}

pub fn unavailable(player: &Player) -> u8 {
    // [confirmed] 4F89 and the one-based membership helper, F12/F13:
    // status bits 8,6,2,3 correspond to masks 80h,20h,02h,04h.
    u8::from(player.id == 0 || player.p15_sbyte8 < 1 || player.p15_byte11 & 0xA6 != 0)
}

pub fn first_roster_slot(roster: &[u16; 20], player: u16) -> u16 {
    // [confirmed] 5C6E searches all twenty words, including a query for ID zero.
    roster
        .iter()
        .position(|&id| id == player)
        .map_or(0, |i| i as u16 + 1)
}

/// Compute placement and the consumed 5083 coin; return remaining effects as requests.
/// Expected outputs and callback observations are not arguments to this function.
pub fn goal_placement(input: &Input) -> Result<Outcome, String> {
    let mut engine = Engine::new(&ContestInput {
        k: 0,
        seed: input.seed,
        ratings: [0; 8],
        goals: input.generated,
    });
    engine.place_goals(input)
}

impl Engine {
    pub fn place_goals(&mut self, input: &Input) -> Result<Outcome, String> {
        use std::collections::BTreeMap;

        // [designed] Reject unsupported/incomplete replay inputs before execution.
        // Normal DD02 callers use k=8/11; count 12 aliases another Pascal table.
        if input.generated.iter().any(|&count| count > 11) {
            return Err("generated counts must be at most 11".into());
        }
        if input.minute_limit > 255 {
            return Err("minute limit must fit the original byte field".into());
        }
        if input
            .scorer_slots
            .iter()
            .any(|&slot| !(1..=20).contains(&slot))
        {
            return Err("scorer slot is outside the twenty-word roster".into());
        }
        let mut players = BTreeMap::new();
        for player in &input.players {
            if players.insert(player.id, player).is_some() {
                return Err(format!("duplicate player input {}", player.id));
            }
        }
        for &id in input.rosters.iter().flatten() {
            if id != 0 && !players.contains_key(&id) {
                return Err(format!("missing player input {id}"));
            }
        }
        if self.word(super::SEED) != input.seed
            || [
                self.image[super::GOALS as usize],
                self.image[super::GOALS as usize + 1],
            ] != input.generated
        {
            return Err("placement seed/generated handoff differs from the DS image".into());
        }

        // [designed] Initialize the remaining decoded entry fields. The seed and
        // generated bytes already in the image are checked, never reset here.
        const ACTOR: usize = 0xD09F;
        const SCORER_TABLE: usize = 0x0CE0;
        self.image[ACTOR] = input.actor_before;
        self.put_word(0xD0B2, input.elapsed);
        self.image[0xD0C4] = input.scores[1];
        self.image[0xD0C5] = input.scores[0];
        self.image[0x0D57] = input.side_offsets[0];
        self.image[0x0D56] = input.side_offsets[1];
        self.image[SCORER_TABLE..SCORER_TABLE + 30].copy_from_slice(&input.scorer_slots);

        let unavailable_id = |id: u16| -> u8 {
            if id == 0 {
                1
            } else {
                unavailable(players[&id])
            }
        };
        let mut scores = input.scores;
        let mut minutes = [[0u8; 11]; 2];
        let mut draws = Vec::new();
        let mut minute_fills = Vec::new();
        let mut attempts = Vec::new();
        let mut mappings = Vec::new();
        let mut callbacks = Vec::new();
        let mut goal = None;

        'outer: for outer in 0..2 {
            let address = super::GOALS as usize + outer;
            if self.image[address] <= scores[outer] {
                continue;
            }
            // [confirmed] DD98..DDA1 subtracts only the entered outer side.
            self.image[address] = self.image[address].wrapping_sub(scores[outer]);
            for side in 0..2 {
                let count = self.image[super::GOALS as usize + side] as usize;
                for index in 0..count {
                    let minute = self.draw(input.minute_limit, "0AB9:DDE4", &mut draws) as u8;
                    minutes[side][index] = minute;
                    minute_fills.push(MinuteFill {
                        outer_side: outer as u8 + 1,
                        side: side as u8 + 1,
                        goal_index: index as u8 + 1,
                        minute,
                    });
                }
            }
            for side in 0..2 {
                let count = self.image[super::GOALS as usize + side] as usize;
                for index in 0..count {
                    if u16::from(minutes[side][index]) != self.word(0xD0B2) || scores[side] >= 15 {
                        continue;
                    }
                    // [designed] Inputs outside the terminating replay domain get
                    // a diagnostic instead of the original's endless retry loop.
                    if !input
                        .scorer_slots
                        .iter()
                        .any(|&slot| unavailable_id(input.rosters[side][slot as usize - 1]) == 0)
                    {
                        return Err(format!("no eligible scorer for side {side}"));
                    }
                    let mut fuel = 65_536usize;
                    let player = loop {
                        if fuel == 0 {
                            return Err(format!("scorer retry fuel exhausted for side {side}"));
                        }
                        fuel -= 1;
                        let sample = self.draw(30, "0AB9:DE5C", &mut draws);
                        let slot = self.image[SCORER_TABLE + sample as usize - 1];
                        let player = input.rosters[side][slot as usize - 1];
                        let rejected = unavailable_id(player);
                        attempts.push(Attempt {
                            side: side as u8 + 1,
                            sampled_slot: u16::from(slot),
                            player,
                            unavailable: rejected,
                        });
                        if rejected == 0 {
                            break player;
                        }
                    };
                    scores[side] = scores[side].wrapping_add(1);
                    let first_slot = first_roster_slot(&input.rosters[side], player);
                    mappings.push(Mapping {
                        player,
                        side: side as u8 + 1,
                        first_slot,
                    });
                    let offset = self.image[if side == 0 { 0x0D57 } else { 0x0D56 }];
                    let actor = (first_slot as u8).wrapping_add(offset);
                    self.image[ACTOR] = actor;
                    let event = GoalEvent {
                        minute: u16::from(minutes[side][index]),
                        actor,
                        side_flag: u8::from(side == 0),
                    };

                    // [confirmed] 5083's RNG projection has one unconditional
                    // random(2); 0D3F returns boolean 1 only for raw value 1.
                    let value = self.draw(2, "0AB9:5190", &mut draws);
                    callbacks.push(GoalEffectRequest {
                        event: event.clone(),
                        coin: u8::from(value == 1),
                        draws: vec![draws.last().unwrap().clone()],
                    });
                    goal = Some(event);
                    // [derived] DF0F/DF13's goal flag and forced final outer index
                    // leave no further semantic effect within this boundary.
                    break 'outer;
                }
            }
        }
        Ok(Outcome {
            output: Output {
                seed: self.word(super::SEED),
                generated: [
                    self.image[super::GOALS as usize],
                    self.image[super::GOALS as usize + 1],
                ],
                scores,
                goal_flag: u8::from(goal.is_some()),
                actor: self.image[ACTOR],
                goal,
            },
            draws,
            callbacks,
            minute_fills,
            attempts,
            mappings,
        })
    }
}

pub fn compare(case: &Case, got: &Outcome) -> Result<(), String> {
    let label = format!("{} step {}", case.week, case.step);
    for (i, (actual, expected)) in got.draws.iter().zip(&case.draws).enumerate() {
        if actual != expected {
            return Err(format!(
                "{label}: draw {i}: expected {expected:?}, got {actual:?}"
            ));
        }
    }
    if got.draws.len() != case.draws.len() {
        return Err(format!("{label}: draw count differs"));
    }
    if got.callbacks != case.callbacks {
        return Err(format!("{label}: goal-effect request differs"));
    }
    if got.minute_fills != case.minute_fills {
        return Err(format!("{label}: minute fills differ"));
    }
    if got.attempts != case.attempts {
        return Err(format!("{label}: scorer attempts differ"));
    }
    if got.mappings != case.mappings {
        return Err(format!("{label}: roster mappings differ"));
    }
    if got.output != case.output {
        return Err(format!(
            "{label}: output: expected {:?}, got {:?}",
            case.output, got.output
        ));
    }
    Ok(())
}

pub fn replay(case: &Case) -> Result<Outcome, String> {
    if case.schema != "placement-case/1" {
        return Err(format!("unknown placement schema {}", case.schema));
    }
    let got = goal_placement(&case.input)?;
    compare(case, &got)?;
    Ok(got)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn availability_uses_signed_field_and_one_based_status_bits() {
        for status in 0..=255 {
            let player = Player {
                id: 1,
                p15_sbyte8: 1,
                p15_byte11: status,
            };
            let rejected = [0x80, 0x20, 0x02, 0x04].iter().any(|bit| status & bit != 0);
            assert_eq!(unavailable(&player), u8::from(rejected));
        }
        for value in [-128, -1, 0] {
            assert_eq!(
                unavailable(&Player {
                    id: 1,
                    p15_sbyte8: value,
                    p15_byte11: 0
                }),
                1
            );
        }
        assert_eq!(
            unavailable(&Player {
                id: 0,
                p15_sbyte8: 127,
                p15_byte11: 0
            }),
            1
        );
    }

    #[test]
    fn roster_search_returns_first_match_including_zero() {
        let mut roster = [0; 20];
        roster[0] = 55;
        roster[2] = 55;
        assert_eq!(first_roster_slot(&roster, 55), 1);
        assert_eq!(first_roster_slot(&roster, 0), 2);
        assert_eq!(first_roster_slot(&roster, 99), 0);
    }

    fn simple() -> Input {
        Input {
            seed: 0,
            generated: [1, 0],
            scores: [0, 0],
            elapsed: 1,
            minute_limit: 1,
            side_offsets: [0, 11],
            actor_before: 99,
            scorer_slots: [2; 30],
            rosters: [[1; 20], [2; 20]],
            players: vec![
                Player {
                    id: 1,
                    p15_sbyte8: 10,
                    p15_byte11: 0,
                },
                Player {
                    id: 2,
                    p15_sbyte8: 10,
                    p15_byte11: 0,
                },
            ],
        }
    }

    #[test]
    fn zero_counts_preserve_seed_scores_and_incoming_actor() {
        let mut input = simple();
        input.generated = [0, 0];
        input.scores = [3, 2];
        let result = goal_placement(&input).unwrap();
        assert_eq!(result.output.seed, input.seed);
        assert_eq!(result.output.generated, [0, 0]);
        assert_eq!(result.output.scores, input.scores);
        assert_eq!(result.output.actor, 99);
        assert_eq!(result.output.goal_flag, 0);
        assert!(result.output.goal.is_none());
        assert!(result.draws.is_empty());
    }

    #[test]
    fn full_elapsed_word_is_not_narrowed_to_a_minute_byte() {
        let mut input = simple();
        input.elapsed = 0x0101;
        let result = goal_placement(&input).unwrap();
        assert_eq!(result.minute_fills[0].minute, 1);
        assert!(result.output.goal.is_none());
        assert_eq!(result.output.actor, 99);
        assert!(result.attempts.is_empty());
    }

    #[test]
    fn one_goal_uses_first_roster_match_and_wrapping_actor_offset() {
        let mut input = simple();
        input.side_offsets[0] = 255;
        let result = goal_placement(&input).unwrap();
        assert_eq!(result.attempts[0].sampled_slot, 2);
        assert_eq!(result.mappings[0].first_slot, 1);
        assert_eq!(result.output.actor, 0);
        assert_eq!(result.output.scores, [1, 0]);
        assert_eq!(result.output.goal_flag, 1);
        assert_eq!(result.callbacks.len(), 1);
        assert_eq!(result.callbacks[0].draws.len(), 1);
        assert_eq!(result.callbacks[0].draws[0].site, "0AB9:5190");
        assert_eq!(result.callbacks[0].draws[0].n, 2);
        assert_eq!(result.callbacks[0].draws[0].value, 2);
        assert_eq!(result.callbacks[0].coin, 0);
        assert_eq!(
            result.callbacks[0].coin,
            u8::from(result.callbacks[0].draws[0].value == 1)
        );
    }

    #[test]
    fn repeated_passes_refill_both_sides_and_preserve_residual_counts() {
        let mut input = simple();
        input.generated = [2, 2];
        input.scores = [1, 1];
        input.elapsed = 0; // All generated minutes are 1, so no scorer call occurs.
        let result = goal_placement(&input).unwrap();
        assert_eq!(result.output.generated, [1, 1]);
        assert_eq!(result.output.scores, [1, 1]);
        assert_eq!(result.minute_fills.len(), 5);
        assert_eq!(
            result
                .minute_fills
                .iter()
                .map(|fill| (fill.outer_side, fill.side, fill.goal_index))
                .collect::<Vec<_>>(),
            vec![(1, 1, 1), (1, 2, 1), (1, 2, 2), (2, 1, 1), (2, 2, 1)]
        );
    }

    #[test]
    fn a_goal_stops_before_the_other_outer_side_can_be_subtracted() {
        let mut input = simple();
        input.generated = [1, 2];
        input.scores = [0, 1];
        let result = goal_placement(&input).unwrap();
        assert_eq!(result.output.generated, [1, 2]);
        assert_eq!(result.output.scores, [1, 1]);
        assert_eq!(result.minute_fills.len(), 3);
        assert_eq!(result.callbacks.len(), 1);
        assert_eq!(result.attempts.len(), 1);
    }

    #[test]
    fn score_cap_prevents_an_earlier_side_from_recording_a_goal() {
        let mut input = simple();
        input.generated = [1, 1];
        input.scores = [15, 0];
        let result = goal_placement(&input).unwrap();
        // Side 1's entered pass fills/scans side 0 first, whose minute matches but
        // whose cumulative byte is already 15. Side 1 can then record the sole goal.
        assert_eq!(result.output.scores, [15, 1]);
        assert_eq!(result.output.goal.as_ref().unwrap().side_flag, 0);
        assert_eq!(result.attempts.len(), 1);
        assert_eq!(result.attempts[0].side, 2);
    }

    #[test]
    fn invalid_inputs_fail_instead_of_fabricating_player_data_or_aliasing_frames() {
        let mut input = simple();
        input.players.clear();
        assert!(goal_placement(&input)
            .unwrap_err()
            .contains("missing player"));
        let mut input = simple();
        input.generated = [12, 0];
        assert!(goal_placement(&input).unwrap_err().contains("11"));
        let mut input = simple();
        input.scorer_slots[0] = 0;
        assert!(goal_placement(&input).unwrap_err().contains("slot"));
    }

    #[test]
    fn no_eligible_scorer_is_an_explicit_replay_domain_error() {
        let mut input = simple();
        input.players[0].p15_sbyte8 = 0;
        assert!(goal_placement(&input)
            .unwrap_err()
            .contains("no eligible scorer"));
    }

    #[test]
    fn unavailable_sample_retries_and_consumes_the_next_scorer_draw() {
        let mut input = simple();
        input.rosters[0][1] = 3;
        input.players[0].p15_sbyte8 = 0;
        input.players.push(Player {
            id: 3,
            p15_sbyte8: 10,
            p15_byte11: 0,
        });
        // [derived] F02's seed-0 generator: after the minute draw, scorer
        // requests return indices 7 then 29. Only index 7 selects unavailable ID 1.
        input.scorer_slots[6] = 1;
        let result = goal_placement(&input).unwrap();
        assert_eq!(result.attempts.len(), 2);
        assert_eq!(
            result.attempts[0],
            Attempt {
                side: 1,
                sampled_slot: 1,
                player: 1,
                unavailable: 1
            }
        );
        assert_eq!(
            result.attempts[1],
            Attempt {
                side: 1,
                sampled_slot: 2,
                player: 3,
                unavailable: 0
            }
        );
        assert_eq!(result.output.goal.as_ref().unwrap().actor, 2);
        assert_eq!(
            result
                .draws
                .iter()
                .filter(|d| d.site == "0AB9:DE5C")
                .count(),
            2
        );
    }
}
