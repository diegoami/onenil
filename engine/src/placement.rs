//! [confirmed] DD7C..DF35 and 4F89/5C6E: archaeology F12 at 77841e9;
//! T13/F13 supplies complete original inputs and the callback observations.
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

pub fn unavailable(_player: &Player) -> u8 {
    todo!("T02 availability routine not implemented")
}

pub fn first_roster_slot(_roster: &[u16; 20], _player: u16) -> u16 {
    todo!("T02 first-roster-match routine not implemented")
}

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
    pub fn place_goals(&mut self, _input: &Input) -> Result<Outcome, String> {
        todo!("T02 original goal-placement block not implemented")
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
