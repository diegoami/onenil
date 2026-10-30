//! [confirmed] DD02's DF35..DFDB prefix and 7B7B logical clock: archaeology
//! F12–F15 and supplemental T16/F16 raw observations.
//! [designed] Late DD02 callees are pending requests, not executed no-ops.
//! The prefix and clock remain separate across that unresolved boundary.

use crate::{
    goal_effects::{Commentary, Queue},
    placement,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PlacementHandoff {
    pub seed: u16,
    pub generated: [u8; 2],
    pub scores: [u8; 2],
    pub goal_flag: u8,
    pub actor: u8,
}
impl From<&placement::Output> for PlacementHandoff {
    fn from(value: &placement::Output) -> Self {
        Self {
            seed: value.seed,
            generated: value.generated,
            scores: value.scores,
            goal_flag: value.goal_flag,
            actor: value.actor,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct History {
    pub base_offset: u16,
    pub base_segment: u16,
    pub selector: u8,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Scene {
    pub index: u8,
    pub player: u16,
    pub word0: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Player {
    pub id: u16,
    pub byte8: u8,
    pub byte11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PrefixInput {
    pub placement: PlacementHandoff,
    pub mode: u8,
    pub d6dd: u8,
    pub remaining: u16,
    pub replacement_word: u16,
    pub clubs: [u16; 2],
    pub history: History,
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PrefixOutput {
    pub seed: u16,
    pub actor: u8,
    pub goal_bytes: [u8; 2],
    pub score_bytes: [u8; 2],
    pub history_offset: u16,
    pub history_segment: u16,
    pub history_byte: u8,
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct VisitState {
    pub word0: u16,
    pub byte8: u8,
    pub byte11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Visit {
    pub index: u8,
    pub player: u16,
    pub unavailable: u8,
    pub before: VisitState,
    pub after: VisitState,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PendingCall {
    pub routine: String,
    pub args: Vec<u16>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PrefixOutcome {
    pub output: PrefixOutput,
    pub visits: Vec<Visit>,
    pub pending_calls: Vec<PendingCall>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ClockFields {
    pub kind: u8,
    pub remaining: u16,
    pub delay: u16,
    pub elapsed: u16,
    pub added: u16,
    pub active: u8,
    pub zero: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ClockCommentary {
    pub event_word: u16,
    pub counts: [u16; 2],
    pub weights: [u8; 2],
    pub queue: Queue,
    pub source_pair: [u16; 2],
    pub target_pair: [u16; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ClockState {
    pub clock: ClockFields,
    pub reload: u8,
    pub cf51: u16,
    pub seed: u16,
    pub commentary: ClockCommentary,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ClockOutcome {
    pub output: ClockState,
    pub result: u8,
    pub event: Option<Commentary>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct ClockObservation {
    pub input: ClockState,
    pub output: ClockState,
    pub result: u8,
    pub event: Option<Commentary>,
    pub draw_start: usize,
    pub draw_end: usize,
}
#[derive(Clone, Debug, Deserialize)]
pub struct PrefixObservation {
    pub input: PrefixInput,
    pub output: PrefixOutput,
    pub visits: Vec<Visit>,
    pub pending_calls: Vec<PendingCall>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Case {
    pub schema: String,
    pub week: String,
    pub step: usize,
    pub prefix: PrefixObservation,
    pub clock: ClockObservation,
    pub placement: placement::Case,
}
#[derive(Debug, Serialize)]
pub struct ComposedOutcome {
    pub placement: placement::Outcome,
    pub prefix: PrefixOutcome,
}

pub fn prefix(_input: &PrefixInput) -> Result<PrefixOutcome, String> {
    todo!("T05 native deterministic tail prefix")
}
pub fn advance_clock(_input: &ClockState) -> Result<ClockOutcome, String> {
    todo!("T05 native logical caller clock")
}
pub fn composed_prefix(
    _placement: &placement::Input,
    _input: &PrefixInput,
) -> Result<ComposedOutcome, String> {
    todo!("T05 actual placement-to-prefix handoff")
}
pub fn replay_prefix(case: &Case) -> Result<(), String> {
    let got = prefix(&case.prefix.input)?;
    if got.output != case.prefix.output
        || got.visits != case.prefix.visits
        || got.pending_calls != case.prefix.pending_calls
    {
        return Err(format!("{} step {} prefix differs", case.week, case.step));
    }
    Ok(())
}
pub fn replay_clock(case: &Case) -> Result<(), String> {
    let got = advance_clock(&case.clock.input)?;
    if got.output != case.clock.output
        || got.result != case.clock.result
        || got.event != case.clock.event
    {
        return Err(format!("{} step {} clock differs", case.week, case.step));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::goal_effects::QueueRow;

    fn prefix_input() -> PrefixInput {
        PrefixInput {
            placement: PlacementHandoff {
                seed: 123,
                generated: [1, 2],
                scores: [3, 4],
                goal_flag: 0,
                actor: 9,
            },
            mode: 1,
            d6dd: 0,
            remaining: 5,
            replacement_word: 0x1234,
            clubs: [1, 2],
            history: History {
                base_offset: 0,
                base_segment: 0x4567,
                selector: 1,
                value: 255,
            },
            scenes: (1..=22)
                .map(|index| Scene {
                    index,
                    player: 7,
                    word0: 99,
                })
                .collect(),
            players: vec![Player {
                id: 7,
                byte8: 3,
                byte11: 0,
            }],
        }
    }
    fn row(index: u16) -> QueueRow {
        QueueRow {
            index,
            code: 255,
            first_player: 19,
            second_player: 23,
            side: 7,
        }
    }
    fn clock_input() -> ClockState {
        ClockState {
            clock: ClockFields {
                kind: 45,
                remaining: 46,
                delay: 0,
                elapsed: 0,
                added: 0,
                active: 1,
                zero: 0,
            },
            reload: 7,
            cf51: 0xDEAD,
            seed: 123,
            commentary: ClockCommentary {
                event_word: 65535,
                counts: [19, 65535],
                weights: [255, 2],
                queue: Queue {
                    head: 0,
                    rows: vec![row(0), row(1), row(2)],
                },
                source_pair: [7, 9],
                target_pair: [5, 6],
            },
        }
    }
    #[test]
    fn prefix_restores_scores_and_history_uses_addition_with_wrapped_far_offset() {
        let mut i = prefix_input();
        i.placement.scores = [16, 1];
        i.history.selector = 0;
        let got = prefix(&i).unwrap();
        assert_eq!(got.output.goal_bytes, [16, 1]);
        assert_eq!(got.output.score_bytes, [1, 16]);
        assert_eq!(got.output.history_byte, 32);
        assert_eq!(got.output.history_offset, 65535);
        assert_eq!(got.output.history_segment, 0x4567);
        assert_eq!((got.output.seed, got.output.actor), (123, 9));
    }
    #[test]
    fn shared_players_decrement_sequentially_then_stop_at_one() {
        let got = prefix(&prefix_input()).unwrap();
        assert_eq!(got.visits.len(), 22);
        assert_eq!(got.visits[0].before.byte8, 3);
        assert_eq!(got.visits[0].after.byte8, 2);
        assert_eq!(got.visits[1].before.byte8, 2);
        assert_eq!(got.visits[1].after.byte8, 1);
        assert!(got.visits[2..]
            .iter()
            .all(|v| v.before.byte8 == 1 && v.after.byte8 == 1));
        assert_eq!(got.output.players[0].byte8, 1);
    }
    #[test]
    fn unavailable_status_and_signed_bytes_replace_scene_words_without_decrement() {
        for (id, byte8, flags) in [
            (0, 10, 0),
            (7, 0, 0),
            (7, 128, 0),
            (7, 255, 0),
            (7, 10, 0x80),
            (7, 10, 0x20),
            (7, 10, 2),
            (7, 10, 4),
        ] {
            let mut i = prefix_input();
            for s in &mut i.scenes {
                s.player = id;
            }
            i.players[0] = Player {
                id,
                byte8,
                byte11: flags,
            };
            let got = prefix(&i).unwrap();
            assert!(got
                .visits
                .iter()
                .all(|v| v.unavailable == 1 && v.after.word0 == 0x1234 && v.after.byte8 == byte8));
        }
    }
    #[test]
    fn scene_decrement_uses_unsigned_remaining_modulo_and_signed_threshold() {
        let mut i = prefix_input();
        i.remaining = 65535;
        i.players[0].byte8 = 2;
        i.players[0].byte11 = 1;
        assert_eq!(prefix(&i).unwrap().output.players[0].byte8, 1);
        i.remaining = 65534;
        assert_eq!(prefix(&i).unwrap().output.players[0].byte8, 2);
    }
    #[test]
    fn pending_late_requests_remain_explicit_and_ordered() {
        let mut i = prefix_input();
        assert_eq!(
            prefix(&i)
                .unwrap()
                .pending_calls
                .iter()
                .map(|p| p.routine.as_str())
                .collect::<Vec<_>>(),
            ["0AB9:6B2F", "0AB9:78B7"]
        );
        i.d6dd = 255;
        assert_eq!(
            prefix(&i)
                .unwrap()
                .pending_calls
                .iter()
                .map(|p| p.routine.as_str())
                .collect::<Vec<_>>(),
            ["0AB9:DBC6", "0AB9:6B2F", "0AB9:78B7"]
        );
        i.placement.goal_flag = 1;
        let got = prefix(&i).unwrap();
        assert_eq!(
            got.pending_calls
                .iter()
                .map(|p| p.routine.as_str())
                .collect::<Vec<_>>(),
            ["1998:045C", "0AB9:52B6", "0AB9:78B7"]
        );
        assert_eq!(got.pending_calls[0].args, [1, 0]);
    }
    #[test]
    fn unsupported_prefix_fields_and_incomplete_or_aliased_definitions_reject() {
        let base = prefix_input();
        let mut bad = Vec::new();
        let mut i = base.clone();
        i.mode = 0;
        bad.push(i);
        let mut i = base.clone();
        i.clubs[0] = 0;
        bad.push(i);
        let mut i = base.clone();
        i.clubs[1] = 177;
        bad.push(i);
        let mut i = base.clone();
        i.scenes.pop();
        bad.push(i);
        let mut i = base.clone();
        i.scenes[0].index = 0;
        bad.push(i);
        let mut i = base.clone();
        i.players.clear();
        bad.push(i);
        let mut i = base.clone();
        i.players.push(i.players[0].clone());
        bad.push(i);
        let mut i = base.clone();
        i.players[0].id = 3921;
        bad.push(i);
        let mut i = base.clone();
        i.placement.goal_flag = 2;
        bad.push(i);
        let mut i = base.clone();
        i.placement.generated[0] = 12;
        bad.push(i);
        for i in bad {
            assert!(prefix(&i).is_err(), "accepted {i:?}");
        }
    }
    #[test]
    fn inactive_and_delay_clock_paths_do_not_reload_or_touch_other_state() {
        let mut i = clock_input();
        i.clock.active = 0;
        let got = advance_clock(&i).unwrap();
        assert_eq!(got.output, i);
        assert_eq!(got.result, 0);
        assert_eq!(got.event, None);
        i.clock.active = 255;
        i.clock.delay = 65535;
        i.clock.zero = 7;
        let got = advance_clock(&i).unwrap();
        let mut expected = i.clone();
        expected.clock.delay = 65534;
        assert_eq!(got.output, expected);
        assert_eq!(got.result, 0);
        assert_eq!(got.event, None);
    }
    #[test]
    fn clock_kind_is_sign_extended_to_long_not_cast_to_unsigned_word() {
        for (kind, remaining, expected) in [(128, 65409, 65408), (254, 65535, 65534)] {
            let mut i = clock_input();
            i.clock.kind = kind;
            i.clock.remaining = remaining;
            let got = advance_clock(&i).unwrap();
            assert_eq!(got.result, 0);
            assert_eq!(got.event, None);
            assert_eq!(got.output.clock.remaining, expected);
            assert_eq!(got.output.cf51, i.cf51);
            assert_eq!(got.output.commentary, i.commentary);
        }
    }
    #[test]
    fn wrapped_sum_zero_terminates_without_advancing_and_signed_weight_wraps() {
        let mut i = clock_input();
        i.clock.remaining = 65535;
        i.clock.added = 1;
        i.clock.elapsed = 127;
        i.commentary.event_word = 0;
        let got = advance_clock(&i).unwrap();
        assert_eq!(got.output.clock.remaining, 65535);
        assert_eq!(got.output.clock.added, 1);
        assert_eq!(got.output.clock.elapsed, 127);
        assert_eq!(got.output.clock.zero, 1);
        assert_eq!(got.output.commentary.event_word, 65535);
        assert_eq!(got.event.unwrap().code, 63);
        assert_eq!(got.output.cf51, 0);
    }
    #[test]
    fn half_boundary_writes_actor_zero_event_counts_queue_and_pair() {
        let i = clock_input();
        let got = advance_clock(&i).unwrap();
        assert_eq!(got.output.clock.remaining, 45);
        assert_eq!(got.output.clock.elapsed, 1);
        assert_eq!(got.output.clock.delay, 7);
        assert_eq!(got.result, 1);
        assert_eq!(got.output.seed, 123);
        assert_eq!(got.output.commentary.event_word, 1);
        assert_eq!(got.output.commentary.counts, [19, 0]);
        let q = &got.output.commentary.queue;
        assert_eq!(q.head, 1);
        assert_eq!(
            q.rows[1],
            QueueRow {
                index: 1,
                code: 64,
                first_player: 0,
                second_player: 0,
                side: 1
            }
        );
        assert_eq!(q.rows[2].code, 255);
        assert_eq!(q.rows[2].first_player, 19);
        assert_eq!(got.output.commentary.target_pair, [7, 9]);
        assert_eq!(
            got.event.unwrap(),
            Commentary {
                code: 64,
                actor_slot: 0,
                secondary_slot: 0,
                event_side: 1,
                accepted: 1
            }
        );
    }
    #[test]
    fn duplicate_clock_event_still_weights_alias_and_clears_cf51() {
        let mut i = clock_input();
        i.commentary.queue = Queue {
            head: 7,
            rows: vec![
                QueueRow {
                    index: 7,
                    code: 64,
                    first_player: 0,
                    second_player: 0,
                    side: 1,
                },
                row(8),
                row(9),
            ],
        };
        let got = advance_clock(&i).unwrap();
        assert_eq!(got.output.commentary.event_word, 1);
        assert_eq!(got.output.commentary.queue, i.commentary.queue);
        assert_eq!(got.output.commentary.counts, i.commentary.counts);
        assert_eq!(got.output.commentary.target_pair, i.commentary.target_pair);
        assert_eq!(got.output.cf51, 0);
        assert_eq!(got.result, 1);
        assert_eq!(got.event.unwrap().accepted, 0);
    }
    #[test]
    fn queue_wrap_changes_only_new_entry_and_next_code_sentinel() {
        let mut i = clock_input();
        i.commentary.queue = Queue {
            head: 1700,
            rows: vec![row(1700), row(1), row(2)],
        };
        let got = advance_clock(&i).unwrap();
        let q = got.output.commentary.queue;
        assert_eq!(q.head, 1);
        assert_eq!(q.rows[0], row(1700));
        assert_eq!(q.rows[1].first_player, 0);
        assert_eq!(q.rows[1].code, 64);
        assert_eq!(q.rows[2], row(2));
    }
    #[test]
    fn added_clock_decrements_and_elapsed_saturates_without_event() {
        for elapsed in [128, 65535] {
            let mut i = clock_input();
            i.clock.remaining = 0;
            i.clock.added = 3;
            i.clock.elapsed = elapsed;
            let got = advance_clock(&i).unwrap();
            assert_eq!(got.output.clock.added, 2);
            assert_eq!(got.output.clock.elapsed, elapsed);
            assert_eq!(got.output.clock.zero, 0);
            assert_eq!(got.output.clock.delay, 7);
            assert_eq!(got.result, 0);
            assert_eq!(got.output.commentary, i.commentary);
        }
    }
    #[test]
    fn malformed_clock_queue_context_rejects() {
        let mut i = clock_input();
        i.commentary.queue.head = 1701;
        assert!(advance_clock(&i).is_err());
        i = clock_input();
        i.commentary.queue.rows.pop();
        assert!(advance_clock(&i).is_err());
        i = clock_input();
        i.commentary.queue.rows[1].index = 9;
        assert!(advance_clock(&i).is_err());
    }
}
