//! [confirmed] DD02's DF35..DFDB prefix and 7B7B logical clock: archaeology
//! F12–F15 and supplemental T16/F16 raw observations.
//! [designed] Late DD02 callees are pending requests, not executed no-ops.
//! The prefix and clock remain separate across that unresolved boundary.

use crate::{
    goal_effects::{Commentary, Queue, QueueRow},
    placement, Engine, Input as ContestInput,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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

// [designed] Named logical pools; actual far segment metadata is returned for
// assertions, not interpreted as arbitrary physical aliasing with the DS image.
const HISTORY: usize = 0x10000;
const P15: usize = 0x20000;
const CONFIG: usize = 0x49000;
const SEED: u16 = 0xA51C;
const GOALS: usize = 0xD086;
const ACTOR: usize = 0xD09F;

fn blank(seed: u16, generated: [u8; 2]) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed,
        ratings: [0; 8],
        goals: generated,
    })
}
fn player_address(id: u16) -> usize {
    P15 + usize::from(id) * 15
}
fn scene_address(index: u8) -> usize {
    0xCCEF + usize::from(index) * 25
}
fn read_word(engine: &Engine, address: usize) -> u16 {
    u16::from_le_bytes([engine.image[address], engine.image[address + 1]])
}
fn put_word(engine: &mut Engine, address: usize, value: u16) {
    engine.image[address..address + 2].copy_from_slice(&value.to_le_bytes());
}
fn queue_address(index: u16) -> usize {
    0xA52A + usize::from(index) * 6
}
fn next_row(index: u16) -> u16 {
    if index == 1700 {
        1
    } else {
        index + 1
    }
}
fn queue_row(engine: &Engine, index: u16) -> QueueRow {
    let a = queue_address(index);
    QueueRow {
        index,
        code: engine.image[a],
        first_player: read_word(engine, a + 1),
        second_player: read_word(engine, a + 3),
        side: engine.image[a + 5],
    }
}
fn put_row(engine: &mut Engine, value: &QueueRow) {
    let a = queue_address(value.index);
    engine.image[a] = value.code;
    put_word(engine, a + 1, value.first_player);
    put_word(engine, a + 3, value.second_player);
    engine.image[a + 5] = value.side;
}
fn same_entry(a: &QueueRow, b: &QueueRow) -> bool {
    a.code == b.code
        && a.first_player == b.first_player
        && a.second_player == b.second_player
        && a.side == b.side
}
fn validate_prefix(input: &PrefixInput) -> Result<(), String> {
    if input.mode == 0
        || input.clubs.iter().any(|id| !(1..=176).contains(id))
        || input.placement.goal_flag > 1
        || input.placement.generated.iter().any(|&n| n > 11)
    {
        return Err("unsupported quick mode, clubs or placement frame fields".into());
    }
    if input.scenes.len() != 22
        || input
            .scenes
            .iter()
            .enumerate()
            .any(|(i, s)| usize::from(s.index) != i + 1 || s.player > 3920)
    {
        return Err("complete ordered scene inputs required".into());
    }
    let mut ids = BTreeSet::new();
    for player in &input.players {
        if player.id > 3920 || !ids.insert(player.id) {
            return Err("unsupported or duplicate shared player record".into());
        }
    }
    if ids != input.scenes.iter().map(|s| s.player).collect() {
        return Err("missing or unreferenced shared player definition".into());
    }
    Ok(())
}
fn validate_clock(input: &ClockState) -> Result<(), String> {
    let q = &input.commentary.queue;
    if q.head > 1700 || q.rows.len() != 3 {
        return Err("unsupported numeric queue head/context".into());
    }
    let expected = [q.head, next_row(q.head), next_row(next_row(q.head))];
    if q.rows
        .iter()
        .zip(expected)
        .any(|(row, index)| row.index != index)
    {
        return Err("numeric queue context must use entry-selected head and next two rows".into());
    }
    Ok(())
}
fn pending_calls(goal: u8, extra: u8) -> Vec<PendingCall> {
    let mut calls = vec![];
    if goal == 0 {
        if extra != 0 {
            calls.push(PendingCall {
                routine: "0AB9:DBC6".into(),
                args: vec![],
            });
        }
        calls.push(PendingCall {
            routine: "0AB9:6B2F".into(),
            args: vec![],
        });
    } else {
        calls.push(PendingCall {
            routine: "1998:045C".into(),
            args: vec![1, 0],
        });
        calls.push(PendingCall {
            routine: "0AB9:52B6".into(),
            args: vec![],
        });
    }
    calls.push(PendingCall {
        routine: "0AB9:78B7".into(),
        args: vec![],
    });
    calls
}

pub fn prefix(input: &PrefixInput) -> Result<PrefixOutcome, String> {
    let mut engine = blank(input.placement.seed, input.placement.generated);
    engine.image[ACTOR] = input.placement.actor;
    engine.finish_prefix(input, &input.placement)
}

pub fn composed_prefix(
    placement_input: &placement::Input,
    input: &PrefixInput,
) -> Result<ComposedOutcome, String> {
    validate_prefix(input)?;
    // [designed] Primitive fields read in both stages must name one shared
    // record; a second expected snapshot cannot silently replace the first.
    let players: BTreeMap<_, _> = placement_input.players.iter().map(|p| (p.id, p)).collect();
    for player in &input.players {
        if player.id == 0 {
            continue;
        }
        let original = players
            .get(&player.id)
            .ok_or("prefix player missing from placement input")?;
        if original.p15_sbyte8 as u8 != player.byte8 || original.p15_byte11 != player.byte11 {
            return Err("prefix player raw fields conflict with placement input".into());
        }
    }
    let mut engine = blank(placement_input.seed, placement_input.generated);
    let placement = engine.place_goals(placement_input)?;
    let actual = PlacementHandoff::from(&placement.output);
    let prefix = engine.finish_prefix(input, &actual)?;
    Ok(ComposedOutcome { placement, prefix })
}

impl Engine {
    /// Finish only DF35..DFDB using the actual placement frame handoff.
    /// Pending late calls are returned without executing their logical effects.
    pub fn finish_prefix(
        &mut self,
        input: &PrefixInput,
        actual: &PlacementHandoff,
    ) -> Result<PrefixOutcome, String> {
        validate_prefix(input)?;
        if &input.placement != actual
            || self.word(SEED) != actual.seed
            || [self.image[GOALS], self.image[GOALS + 1]] != actual.generated
            || self.image[ACTOR] != actual.actor
        {
            return Err("actual placement seed/generated/actor/frame handoff differs".into());
        }
        // [designed] Load only named raw fields; seed, actor and generated bytes
        // above remain the actual predecessor's image, never reset to a fixture.
        self.image[0xD074] = input.mode;
        self.image[0xD6DD] = input.d6dd;
        self.put_word(0xD0AE, input.remaining);
        self.put_word(0x0D32, input.replacement_word);
        self.put_word(0xD6E6, input.clubs[0]);
        self.put_word(0xD6E4, input.clubs[1]);
        self.put_word(0xD7CE, input.history.base_offset);
        self.put_word(0xD7D0, 0x1000);
        self.put_word(0xD7D2, 0);
        self.put_word(0xD7D4, 0x4900);
        self.image[CONFIG] = input.history.selector;
        for s in &input.scenes {
            let a = scene_address(s.index);
            put_word(self, a, s.word0);
            put_word(self, a + 14, s.player);
        }
        for p in &input.players {
            let a = player_address(p.id);
            self.image[a + 8] = p.byte8;
            self.image[a + 11] = p.byte11;
        }
        let history_offset = self
            .word(0xD7CE)
            .wrapping_add(self.word(0xD6E6).wrapping_mul(0x3A))
            .wrapping_add(u16::from(self.image[CONFIG]))
            .wrapping_sub(0x3B);
        let history_address = HISTORY + usize::from(history_offset);
        self.image[history_address] = input.history.value;
        // [confirmed] DF35..DF40 restore cumulative scores in opposite orders.
        self.image[GOALS..GOALS + 2].copy_from_slice(&actual.scores);
        self.image[0xD0C4] = actual.scores[1];
        self.image[0xD0C5] = actual.scores[0];
        // 232A:01C5 is ADD, not OR. Only AL is stored, even for large scores.
        self.image[history_address] =
            u16::from(actual.scores[0]).wrapping_add(u16::from(actual.scores[1]) << 4) as u8;
        let mut visits = vec![];
        for index in 1..=22 {
            let a = scene_address(index);
            let id = read_word(self, a + 14);
            let p = player_address(id);
            let before = VisitState {
                word0: read_word(self, a),
                byte8: self.image[p + 8],
                byte11: self.image[p + 11],
            };
            let unavailable = placement::unavailable(&placement::Player {
                id,
                p15_sbyte8: self.image[p + 8] as i8,
                p15_byte11: self.image[p + 11],
            });
            if unavailable != 0 {
                put_word(self, a, self.word(0x0D32));
            } else if self.word(0xD0AE) % 5 == 0 && (self.image[p + 8] as i8) > 1 {
                self.image[p + 8] = self.image[p + 8].wrapping_sub(1);
            }
            let after = VisitState {
                word0: read_word(self, a),
                byte8: self.image[p + 8],
                byte11: self.image[p + 11],
            };
            visits.push(Visit {
                index,
                player: id,
                unavailable,
                before,
                after,
            });
        }
        let scenes = input
            .scenes
            .iter()
            .map(|s| {
                let a = scene_address(s.index);
                Scene {
                    index: s.index,
                    player: read_word(self, a + 14),
                    word0: read_word(self, a),
                }
            })
            .collect();
        let players = input
            .players
            .iter()
            .map(|p| {
                let a = player_address(p.id);
                Player {
                    id: p.id,
                    byte8: self.image[a + 8],
                    byte11: self.image[a + 11],
                }
            })
            .collect();
        Ok(PrefixOutcome {
            output: PrefixOutput {
                seed: self.word(SEED),
                actor: self.image[ACTOR],
                goal_bytes: [self.image[GOALS], self.image[GOALS + 1]],
                score_bytes: [self.image[0xD0C4], self.image[0xD0C5]],
                history_offset,
                history_segment: input.history.base_segment,
                history_byte: self.image[history_address],
                scenes,
                players,
            },
            visits,
            pending_calls: pending_calls(actual.goal_flag, input.d6dd),
        })
    }
}

fn load_clock(engine: &mut Engine, input: &ClockState) {
    let c = &input.clock;
    for (a, v) in [
        (0xD0AE, c.remaining),
        (0xD0B0, c.delay),
        (0xD0B2, c.elapsed),
        (0xD0B4, c.added),
        (0xCF51, input.cf51),
        (0xD16E, input.commentary.event_word),
    ] {
        engine.put_word(a, v);
    }
    for (a, v) in [
        (0xD0AC, c.kind),
        (0xD0B8, c.active),
        (0xD0B9, c.zero),
        (0x0D28, input.reload),
    ] {
        engine.image[a] = v;
    }
    for i in 0..2 {
        engine.image[0x097A + 63 + i] = input.commentary.weights[i];
        engine.put_word(0xD1C0 + 2 * i as u16, input.commentary.counts[i]);
        engine.put_word(0xD08A + 2 * i as u16, input.commentary.source_pair[i]);
        engine.put_word(0xD09A + 2 * i as u16, input.commentary.target_pair[i]);
    }
    engine.put_word(0x0380, input.commentary.queue.head);
    for r in &input.commentary.queue.rows {
        put_row(engine, r);
    }
}
fn clock_snapshot(engine: &Engine, context: &ClockState) -> ClockState {
    ClockState {
        clock: ClockFields {
            kind: engine.image[0xD0AC],
            remaining: engine.word(0xD0AE),
            delay: engine.word(0xD0B0),
            elapsed: engine.word(0xD0B2),
            added: engine.word(0xD0B4),
            active: engine.image[0xD0B8],
            zero: engine.image[0xD0B9],
        },
        reload: engine.image[0x0D28],
        cf51: engine.word(0xCF51),
        seed: engine.word(SEED),
        commentary: ClockCommentary {
            event_word: engine.word(0xD16E),
            counts: [engine.word(0xD1C0), engine.word(0xD1C2)],
            weights: [engine.image[0x097A + 63], engine.image[0x097A + 64]],
            queue: Queue {
                head: engine.word(0x0380),
                rows: context
                    .commentary
                    .queue
                    .rows
                    .iter()
                    .map(|r| queue_row(engine, r.index))
                    .collect(),
            },
            source_pair: [engine.word(0xD08A), engine.word(0xD08C)],
            target_pair: [engine.word(0xD09A), engine.word(0xD09C)],
        },
    }
}
fn clock_commentary(engine: &mut Engine, code: u8) -> Commentary {
    // [confirmed] Actor zero with side1 writes D146+40*1+2*0 = D16E,
    // aliasing the goal projection's side0 slot20, not side1 slot1.
    let weight = engine.image[0x097A + usize::from(code)] as i8;
    engine.put_word(
        0xD16E,
        engine.word(0xD16E).wrapping_add(i16::from(weight) as u16),
    );
    let head = engine.word(0x0380);
    let mut candidate = QueueRow {
        index: head,
        code,
        first_player: 0,
        second_player: 0,
        side: 1,
    };
    let accepted = head == 0 || !same_entry(&queue_row(engine, head), &candidate);
    if accepted {
        engine.put_word(0xD1C2, engine.word(0xD1C2).wrapping_add(1));
        let next = next_row(head);
        engine.put_word(0x0380, next);
        candidate.index = next;
        put_row(engine, &candidate);
        engine.image[queue_address(next_row(next))] = 255;
        engine.put_word(0xD09A, engine.word(0xD08A));
        engine.put_word(0xD09C, engine.word(0xD08C));
    }
    Commentary {
        code,
        actor_slot: 0,
        secondary_slot: 0,
        event_side: 1,
        accepted: u8::from(accepted),
    }
}
pub fn advance_clock(input: &ClockState) -> Result<ClockOutcome, String> {
    validate_clock(input)?;
    let mut engine = blank(input.seed, [0; 2]);
    load_clock(&mut engine, input);
    let mut result = 0;
    let mut event = None;
    if engine.image[0xD0B8] != 0 {
        let delay = engine.word(0xD0B0);
        if delay != 0 {
            engine.put_word(0xD0B0, delay.wrapping_sub(1));
        } else {
            let zero = engine.word(0xD0AE).wrapping_add(engine.word(0xD0B4)) == 0;
            engine.image[0xD0B9] = u8::from(zero);
            if !zero {
                let remaining = engine.word(0xD0AE);
                if remaining != 0 {
                    engine.put_word(0xD0AE, remaining.wrapping_sub(1));
                } else {
                    engine.put_word(0xD0B4, engine.word(0xD0B4).wrapping_sub(1));
                }
                let elapsed = engine.word(0xD0B2);
                if elapsed < 128 {
                    engine.put_word(0xD0B2, elapsed.wrapping_add(1));
                }
            }
            // 00D6's formatted output stays external; the logical reload follows it.
            engine.put_word(0xD0B0, u16::from(engine.image[0x0D28]));
            // CBW+CWD(kind) is compared to the zero-extended remaining word.
            let boundary = i32::from(engine.word(0xD0AE)) == i32::from(engine.image[0xD0AC] as i8);
            if zero || boundary {
                event = Some(clock_commentary(&mut engine, if zero { 63 } else { 64 }));
                // This write occurs after numeric commentary even on dedup rejection.
                engine.put_word(0xCF51, 0);
                result = 1;
            }
        }
    }
    Ok(ClockOutcome {
        output: clock_snapshot(&engine, input),
        result,
        event,
    })
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
    fn actor_zero_weights_touch_exact_ds_alias_and_preserve_adjacent_words() {
        let i = clock_input();
        let mut engine = blank(i.seed, [0; 2]);
        load_clock(&mut engine, &i);
        engine.put_word(0xD16C, 0x1234);
        engine.put_word(0xD170, 0x5678);
        let event = clock_commentary(&mut engine, 64);
        assert_eq!(event.actor_slot, 0);
        assert_eq!(engine.word(0xD16E), 1);
        assert_eq!(engine.word(0xD16C), 0x1234);
        assert_eq!(engine.word(0xD170), 0x5678);
        assert_eq!(engine.word(SEED), i.seed);
    }

    #[test]
    fn actual_prefix_image_guards_reject_before_loading_or_resetting_fields() {
        let i = prefix_input();
        for corrupt in 0..3 {
            let mut engine = blank(i.placement.seed, i.placement.generated);
            engine.image[ACTOR] = i.placement.actor;
            match corrupt {
                0 => engine.put_word(SEED, i.placement.seed ^ 1),
                1 => engine.image[ACTOR] ^= 1,
                _ => engine.image[GOALS] ^= 1,
            }
            let before = engine.image.clone();
            assert!(engine.finish_prefix(&i, &i.placement).is_err());
            assert_eq!(engine.image, before);
        }
    }

    #[test]
    fn prefix_preserves_prior_goal_statistics_and_other_history_cells() {
        let i = prefix_input();
        let mut engine = blank(i.placement.seed, i.placement.generated);
        engine.image[ACTOR] = i.placement.actor;
        let a = player_address(7);
        put_word(&mut engine, a + 2, 0x1234);
        engine.image[a + 5] = 23;
        engine.image[a + 6] = 31;
        engine.image[HISTORY + 1] = 0xA5;
        engine.image[HISTORY + 65535] = 0x5A;
        engine.finish_prefix(&i, &i.placement).unwrap();
        assert_eq!(read_word(&engine, a + 2), 0x1234);
        assert_eq!(engine.image[a + 5], 23);
        assert_eq!(engine.image[a + 6], 31);
        assert_eq!(engine.image[HISTORY + 1], 0xA5);
        assert_eq!(engine.image[HISTORY + 65535], 0x5A);
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
