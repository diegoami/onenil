//! [confirmed] Logical 5083/67D0 effects: archaeology F12/F13 at a39ac70;
//! T14/F14 at da4f9affc0f147c32ba5db132c6cfd5e7db731b4 supplies original projections.
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

// [designed] Canonical far pools for this bounded projection. DS remains zero
// based; only named fields are loaded and unmodelled bytes are preserved.
const P15_BASE: usize = 0x20000;
const P8_BASE: usize = 0x30000;
const CLUB_BASE: usize = 0x40000;
fn word(e: &Engine, address: usize) -> u16 {
    u16::from_le_bytes([e.image[address], e.image[address + 1]])
}
fn put_word(e: &mut Engine, address: usize, value: u16) {
    e.image[address..address + 2].copy_from_slice(&value.to_le_bytes());
}
fn p15(id: u16) -> usize {
    P15_BASE + usize::from(id) * 15
}
fn p8(id: u16) -> usize {
    P8_BASE + usize::from(id) * 8
}
fn scene(index: u8) -> usize {
    0xCCEF + usize::from(index) * 25
}
fn roster(club: u16, slot: usize) -> usize {
    CLUB_BASE + (usize::from(club) - 1) * 62 + slot * 2
}
fn row(index: u16) -> usize {
    0xA52A + usize::from(index) * 6
}
fn next_head(head: u16) -> u16 {
    if head == 1700 {
        1
    } else {
        head + 1
    }
}
fn normalized(slot: u8) -> u16 {
    u16::from(if slot > 11 { slot - 11 } else { slot })
}
fn queue_row(e: &Engine, index: u16) -> QueueRow {
    let a = row(index);
    QueueRow {
        index,
        code: e.image[a],
        first_player: word(e, a + 1),
        second_player: word(e, a + 3),
        side: e.image[a + 5],
    }
}
fn put_queue_row(e: &mut Engine, r: &QueueRow) {
    let a = row(r.index);
    e.image[a] = r.code;
    put_word(e, a + 1, r.first_player);
    put_word(e, a + 3, r.second_player);
    e.image[a + 5] = r.side;
}
fn same_record(a: &QueueRow, b: &QueueRow) -> bool {
    a.code == b.code
        && a.first_player == b.first_player
        && a.second_player == b.second_player
        && a.side == b.side
}

pub fn byte6(input: &Byte6Input) -> Byte6Output {
    // [confirmed] 67E8..6808 and signed word clamp 24BC:03D9.
    let sum = i16::from(input.old_byte6 as i8).wrapping_add(input.delta);
    let stored = sum.clamp(1, 255) as u8;
    // 6812 reloads the STORED byte with CBW before signed-long min(32).
    let cache = if input.cache_flag != 0 {
        i16::from(stored as i8).min(32) as u8
    } else {
        input.cache_before
    };
    Byte6Output {
        byte6: stored,
        cache,
    }
}

fn validate(state: &State, request: &GoalEffectRequest) -> Result<(), String> {
    use std::collections::BTreeSet;
    let event = &request.event;
    if !(1..=22).contains(&event.actor)
        || event.side_flag > 1
        || request.coin > 1
        || !(1..=22).contains(&state.global_actor)
        || state.secondary_actor > 22
    {
        return Err("unsupported actor or goal selector".into());
    }
    if request.draws.len() != 1 {
        return Err("expected one already-consumed coin draw".into());
    }
    let d = &request.draws[0];
    let expected_seed = d.seed_before.wrapping_mul(0x7ABD).wrapping_add(0x1B0F);
    let value = (((u32::from(expected_seed) * 2) >> 16) + 1) as u16;
    if d.site != "0AB9:5190"
        || d.n != 2
        || d.seed_after != expected_seed
        || d.value != value
        || request.coin != u8::from(value == 1)
    {
        return Err("consumed coin contract differs".into());
    }
    if state.clubs.iter().any(|&id| !(1..=176).contains(&id))
        || state.side_offsets.iter().any(|&v| v > 21)
    {
        return Err("unsupported club or keeper scene".into());
    }
    if state.clubs[0] == state.clubs[1] && state.rosters[0] != state.rosters[1] {
        return Err("conflicting aliased club rosters".into());
    }
    if state.scenes.len() != 22
        || state
            .scenes
            .iter()
            .enumerate()
            .any(|(i, s)| usize::from(s.index) != i + 1 || s.player > 3920)
    {
        return Err("complete ordered scene inputs required".into());
    }
    let mut refs = vec![
        event.actor,
        state.global_actor,
        state.side_offsets[usize::from(event.side_flag)] + 1,
    ];
    if state.secondary_actor != 0 {
        refs.push(state.secondary_actor);
    }
    if refs
        .iter()
        .any(|&i| !(1..=22).contains(&state.scenes[usize::from(i) - 1].slot))
    {
        return Err("unsupported referenced scene slot".into());
    }
    let mut ids = BTreeSet::new();
    for p in &state.players {
        if p.id > 3920 || p.p8_position > 13 || !ids.insert(p.id) {
            return Err("invalid or duplicate player input".into());
        }
    }
    if state
        .rosters
        .iter()
        .flatten()
        .chain(state.scenes.iter().map(|s| &s.player))
        .any(|id| !ids.contains(id))
    {
        return Err("missing referenced player input".into());
    }
    if state.log_counts.iter().any(|&count| count > 11) || state.queue.head > 1700 {
        return Err("unsupported log count or queue head".into());
    }
    let head = state.queue.head;
    let indices = [head, next_head(head), next_head(next_head(head))];
    if state.queue.rows.len() != 3
        || state
            .queue
            .rows
            .iter()
            .zip(indices)
            .any(|(r, i)| r.index != i)
    {
        return Err("queue context must include entry head and its next two rows".into());
    }
    Ok(())
}

fn load(e: &mut Engine, s: &State) {
    e.image[0xD0A0] = s.secondary_actor;
    for (a, v) in [
        (0xD6E6, s.clubs[0]),
        (0xD6E4, s.clubs[1]),
        (0xD7BA, 0),
        (0xD7BC, 0x3000),
        (0xD7BE, 0),
        (0xD7C0, 0x2000),
        (0xD7C2, 0),
        (0xD7C4, 0x4000),
    ] {
        put_word(e, a, v);
    }
    e.image[0x0D57] = s.side_offsets[0];
    e.image[0x0D56] = s.side_offsets[1];
    for (a, v) in [
        (0xD1CA, s.flags.d1ca),
        (0xD6DD, s.flags.d6dd),
        (0xD0B7, s.flags.d0b7),
        (0xD0B9, s.flags.d0b9),
    ] {
        e.image[a] = v;
    }
    for (i, &v) in s.position_deltas.iter().enumerate() {
        e.image[0x0D1A + i] = v as u8;
    }
    for (i, &v) in s.event_weights.iter().enumerate() {
        e.image[0x097A + 0x26 + i] = v as u8;
    }
    for sc in &s.scenes {
        let a = scene(sc.index);
        e.image[a + 7] = sc.slot;
        put_word(e, a + 14, sc.player);
    }
    for (side, r) in s.rosters.iter().enumerate() {
        for (i, &id) in r.iter().enumerate() {
            put_word(e, roster(s.clubs[side], i), id);
        }
    }
    for p in &s.players {
        e.image[p8(p.id) + 6] = p.p8_position;
        e.image[p8(p.id) + 7] = p.p8_flag7;
        put_word(e, p15(p.id) + 2, p.p15_word2);
        e.image[p15(p.id) + 5] = p.p15_byte5;
        e.image[p15(p.id) + 6] = p.p15_byte6;
    }
    for side in 0..2 {
        for i in 0..20 {
            put_word(e, 0xD146 + 40 * side + 2 * (i + 1), s.event_totals[side][i]);
            e.image[0xD197 + 20 * side + i + 1] = s.cache[side][i];
        }
        put_word(e, 0xD1C0 + 2 * side, s.event_counts[side]);
        e.image[0xD0D6 + 10 * side] = s.side_counts[side];
        e.image[0xD128 + side] = s.log_counts[side];
        for (i, entry) in s.logs[side].iter().enumerate() {
            let a = 0xD0E3 + 33 * side + 3 * (i + 1);
            put_word(e, a, entry.player);
            e.image[a + 2] = entry.minute_byte;
        }
    }
    for (a, v) in [
        (0xD08A, s.source_pair[0]),
        (0xD08C, s.source_pair[1]),
        (0xD09A, s.target_pair[0]),
        (0xD09C, s.target_pair[1]),
        (0x0380, s.queue.head),
    ] {
        put_word(e, a, v);
    }
    for r in &s.queue.rows {
        put_queue_row(e, r);
    }
}

fn snapshot(e: &Engine, template: &State) -> State {
    let mut s = template.clone();
    s.global_actor = e.image[0xD09F];
    s.secondary_actor = e.image[0xD0A0];
    s.clubs = [word(e, 0xD6E6), word(e, 0xD6E4)];
    s.side_offsets = [e.image[0x0D57], e.image[0x0D56]];
    s.flags = Flags {
        d1ca: e.image[0xD1CA],
        d6dd: e.image[0xD6DD],
        d0b7: e.image[0xD0B7],
        d0b9: e.image[0xD0B9],
    };
    for (i, v) in s.position_deltas.iter_mut().enumerate() {
        *v = e.image[0x0D1A + i] as i8;
    }
    for (i, v) in s.event_weights.iter_mut().enumerate() {
        *v = e.image[0x097A + 0x26 + i] as i8;
    }
    for sc in &mut s.scenes {
        let a = scene(sc.index);
        sc.slot = e.image[a + 7];
        sc.player = word(e, a + 14);
    }
    for (side, r) in s.rosters.iter_mut().enumerate() {
        for (i, id) in r.iter_mut().enumerate() {
            *id = word(e, roster(s.clubs[side], i));
        }
    }
    for p in &mut s.players {
        p.p8_position = e.image[p8(p.id) + 6];
        p.p8_flag7 = e.image[p8(p.id) + 7];
        p.p15_word2 = word(e, p15(p.id) + 2);
        p.p15_byte5 = e.image[p15(p.id) + 5];
        p.p15_byte6 = e.image[p15(p.id) + 6];
    }
    for side in 0..2 {
        for i in 0..20 {
            s.event_totals[side][i] = word(e, 0xD146 + 40 * side + 2 * (i + 1));
            s.cache[side][i] = e.image[0xD197 + 20 * side + i + 1];
        }
        s.event_counts[side] = word(e, 0xD1C0 + 2 * side);
        s.side_counts[side] = e.image[0xD0D6 + 10 * side];
        s.log_counts[side] = e.image[0xD128 + side];
        for (i, entry) in s.logs[side].iter_mut().enumerate() {
            let a = 0xD0E3 + 33 * side + 3 * (i + 1);
            entry.player = word(e, a);
            entry.minute_byte = e.image[a + 2];
        }
    }
    s.source_pair = [word(e, 0xD08A), word(e, 0xD08C)];
    s.target_pair = [word(e, 0xD09A), word(e, 0xD09C)];
    s.queue.head = word(e, 0x0380);
    s.queue.rows = template
        .queue
        .rows
        .iter()
        .map(|r| queue_row(e, r.index))
        .collect();
    s
}

fn apply_at(e: &mut Engine, scene_index: u8, delta: i16, flag: u8) {
    let a = scene(scene_index);
    let id = word(e, a + 14);
    let slot = e.image[a + 7];
    let side = usize::from(slot < 12);
    let norm = usize::from(normalized(slot));
    let cache = 0xD197 + 20 * side + norm;
    let out = byte6(&Byte6Input {
        old_byte6: e.image[p15(id) + 6],
        delta,
        cache_flag: flag,
        cache_before: e.image[cache],
    });
    e.image[p15(id) + 6] = out.byte6;
    if flag != 0 {
        e.image[cache] = out.cache;
    }
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
        state: &State,
        request: &GoalEffectRequest,
    ) -> Result<Outcome, String> {
        validate(state, request)?;
        if self.word(super::SEED) != request.draws[0].seed_after
            || self.image[0xD09F] != state.global_actor
        {
            return Err("goal-effect seed/actor handoff differs from the image".into());
        }
        let seed = self.word(super::SEED);
        load(self, state);
        // [confirmed] 1DB7/0F42 updates the weighted word even when 0E45 dedups.
        let code = if self.image[0xD1CA] == 0 { 0x26 } else { 0x27 };
        let global = self.image[0xD09F];
        let event_side = usize::from(global < 12);
        let actor_slot = normalized(self.image[scene(global) + 7]);
        let other = self.image[0xD0A0];
        let secondary_slot = if other == 0 {
            0
        } else {
            normalized(self.image[scene(other) + 7])
        };
        let event_total = 0xD146 + 40 * event_side + 2 * usize::from(actor_slot);
        let weight = i16::from(self.image[0x097A + code] as i8) as u16;
        put_word(
            self,
            event_total,
            word(self, event_total).wrapping_add(weight),
        );
        let head = word(self, 0x0380);
        let next = next_head(head);
        let event_club = word(self, if event_side == 1 { 0xD6E6 } else { 0xD6E4 });
        let candidate = QueueRow {
            index: next,
            code: code as u8,
            first_player: word(self, roster(event_club, usize::from(actor_slot) - 1)),
            second_player: if secondary_slot == 0 {
                0
            } else {
                word(self, roster(event_club, usize::from(secondary_slot) - 1))
            },
            side: event_side as u8,
        };
        let accepted = head == 0 || !same_record(&queue_row(self, head), &candidate);
        if accepted {
            let a = 0xD1C0 + 2 * event_side;
            put_word(self, a, word(self, a).wrapping_add(1));
            put_word(self, 0x0380, next);
            put_queue_row(self, &candidate);
            self.image[row(next_head(next))] = 255;
            put_word(self, 0xD09A, word(self, 0xD08A));
            put_word(self, 0xD09C, word(self, 0xD08C));
        }
        let actor = request.event.actor;
        let original = usize::from(request.event.side_flag);
        let scorer = word(self, scene(actor) + 14);
        if self.image[0xD1CA] == 0 && self.image[0xD6DD] != 0 && self.image[p8(scorer) + 7] != 0 {
            let a = p15(scorer);
            put_word(self, a + 2, word(self, a + 2).wrapping_add(1));
            self.image[a + 5] = self.image[a + 5].wrapping_add(1);
        }
        let delta = if self.image[0xD1CA] != 0 {
            -1
        } else {
            i16::from(self.image[0x0D1A + usize::from(self.image[p8(scorer) + 6])] as i8)
        };
        apply_at(self, actor, delta, 1);
        // Stat keeper and scene keeper are intentionally resolved independently.
        let opposite_club = word(self, if original == 1 { 0xD6E4 } else { 0xD6E6 });
        let keeper = word(self, roster(opposite_club, 0));
        if self.image[0xD6DD] != 0 && self.image[p8(keeper) + 7] == 0 {
            let a = p15(keeper);
            self.image[a + 5] = self.image[a + 5].wrapping_add(1);
            put_word(self, a + 2, word(self, a + 2).wrapping_add(1));
        }
        let keeper_scene = self.image[if original == 0 { 0x0D57 } else { 0x0D56 }] + 1;
        apply_at(self, keeper_scene, -1, request.coin);
        let log_side = original ^ usize::from(self.image[0xD1CA] != 0);
        let count = self.image[0xD128 + log_side];
        let count = if count < 11 { count + 1 } else { count };
        self.image[0xD128 + log_side] = count;
        let scoring_club = word(self, if original == 1 { 0xD6E6 } else { 0xD6E4 });
        let slot = normalized(self.image[scene(actor) + 7]);
        let id = word(self, roster(scoring_club, usize::from(slot) - 1));
        let a = 0xD0E3 + 33 * log_side + 3 * usize::from(count);
        put_word(self, a, id);
        let minute = request.event.minute as u8;
        self.image[a + 2] = if self.image[0xD0B7] != 0 || self.image[0xD1CA] != 0 {
            minute.wrapping_add(0x80)
        } else {
            minute
        };
        if self.image[0xD1CA] == 0 && self.image[0xD0B9] == 0 {
            let a = 0xD0D6 + 10 * original;
            self.image[a] = self.image[a].wrapping_add(1);
        }
        debug_assert_eq!(self.word(super::SEED), seed);
        Ok(Outcome {
            state: snapshot(self, state),
            commentary: Commentary {
                code: code as u8,
                actor_slot,
                secondary_slot,
                event_side: event_side as u8,
                accepted: u8::from(accepted),
            },
        })
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
    pub(super) fn state() -> State {
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
    pub(super) fn request(coin: u8) -> GoalEffectRequest {
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

#[cfg(test)]
mod preservation_tests {
    use super::*;
    #[test]
    fn head_zero_accepts_an_equal_tuple_and_preserves_unmodelled_bytes() {
        let mut s = super::tests::state();
        let r = super::tests::request(1);
        s.queue.rows[0] = QueueRow {
            index: 0,
            code: 38,
            first_player: 9,
            second_player: 0,
            side: 1,
        };
        let mut engine = Engine::new(&ContestInput {
            k: 0,
            seed: r.draws[0].seed_after,
            ratings: [0; 8],
            goals: [7, 8],
        });
        engine.image[0xD09F] = s.global_actor;
        engine.image[0x10] = 171;
        engine.image[p15(9) + 8] = 99;
        engine.image[p8(9)] = 77;
        let seed = engine.goal_effect_seed();
        let got = engine.apply_goal_effects(&s, &r).unwrap();
        assert_eq!(got.commentary.accepted, 1);
        assert_eq!(engine.goal_effect_seed(), seed);
        assert_eq!(
            &engine.image[super::super::GOALS as usize..super::super::GOALS as usize + 2],
            &[7, 8]
        );
        assert_eq!(engine.image[0x10], 171);
        assert_eq!(engine.image[p15(9) + 8], 99);
        assert_eq!(engine.image[p8(9)], 77);
    }
}
