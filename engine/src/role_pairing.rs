//! [confirmed] F29/T29 at archaeology218490d: bounded1000/10B9 role pairing.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Locals {
    pub own: [u8; 11],
    pub opponent: [u8; 11],
    pub assigned: [u8; 11],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RangeInput {
    pub low: u16,
    pub high: u16,
    pub locals: Locals,
    pub preferences: [[u8; 13]; 4],
}
#[derive(Debug, Serialize)]
pub struct RangeOutcome {
    pub output: Locals,
    pub draws: Vec<Draw>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub role: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub roster: [u16; 11],
    pub byte57: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub clubs: [Club; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub preferences: [[u8; 13]; 4],
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub ranges: Vec<Value>,
    pub draws: Vec<Draw>,
}

use crate::{placement, Engine, Input as ContestInput};
use serde_json::json;
use std::collections::BTreeSet;
const P8: usize = 0x10000;
const P15: usize = 0x20000;
const CLUB: usize = 0x30000;
// [designed] Normalized caller-frame arrays,not physical guest stack addresses.
const OWN: usize = 0x40000;
const OPP: usize = 0x40010;
const ASSIGNED: usize = 0x40020;
fn blank(seed: u16) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed,
        ratings: [0; 8],
        goals: [0; 2],
    })
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn address(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
}
fn physical(e: &Engine, side: usize, slot: u8) -> u8 {
    e.image[0xD56 + side] + slot
}
fn locals(e: &Engine) -> Locals {
    Locals {
        own: e.image[OWN..OWN + 11].try_into().unwrap(),
        opponent: e.image[OPP..OPP + 11].try_into().unwrap(),
        assigned: e.image[ASSIGNED..ASSIGNED + 11].try_into().unwrap(),
    }
}
fn load_locals(e: &mut Engine, s: &Locals) {
    e.image[OWN..OWN + 11].copy_from_slice(&s.own);
    e.image[OPP..OPP + 11].copy_from_slice(&s.opponent);
    e.image[ASSIGNED..ASSIGNED + 11].copy_from_slice(&s.assigned);
}
fn load_preferences(e: &mut Engine, p: &[[u8; 13]; 4]) {
    for (pass, roles) in p.iter().enumerate() {
        for (role, value) in roles.iter().enumerate() {
            e.image[0x09B0 + 14 * (pass + 1) + role + 1] = *value;
        }
    }
}
fn validate_locals(s: &Locals) -> Result<(), String> {
    if s.own[0] != 10
        || s.opponent[0] != 10
        || s.assigned[0] != 0
        || s.own[1..]
            .iter()
            .chain(s.opponent[1..].iter())
            .any(|v| *v > 13)
        || s.assigned.iter().any(|v| *v == 1 || *v > 11)
    {
        return Err("unsupported pairing local sentinel/role/assignment domain".into());
    }
    Ok(())
}
// [confirmed] F29/1000 and26C9:0C6F/0BFE: length-one Pascal search includes NUL.
pub(crate) fn range_image(e: &mut Engine, low: u16, high: u16) {
    for pass in 1..=4 {
        for opponent in (1..11).rev() {
            let role = e.image[OPP + opponent];
            if u16::from(role) < low || u16::from(role) > high {
                continue;
            }
            let character = e.image[0x09B0 + 14 * pass + usize::from(role)];
            if let Some(position) = e.image[OWN + 1..OWN + 11]
                .iter()
                .position(|v| *v == character)
            {
                let own = position + 1;
                e.image[OWN + own] = 0;
                e.image[OPP + opponent] = 0;
                e.image[ASSIGNED + own] = (opponent + 1) as u8;
            }
        }
    }
}
fn range_machine(s: &RangeInput) -> Result<Engine, String> {
    if s.low < 1 || s.high > 13 || s.low > s.high {
        return Err("unsupported pairing range bounds".into());
    }
    validate_locals(&s.locals)?;
    let mut e = blank(0xBEEF);
    load_locals(&mut e, &s.locals);
    load_preferences(&mut e, &s.preferences);
    Ok(e)
}
pub(crate) fn validate(s: &State) -> Result<(), String> {
    if !matches!(s.side_offsets, [0, 11] | [11, 0])
        || s.clubs.iter().any(|c| !(1..=176).contains(&c.id))
        || s.clubs[0].id == s.clubs[1].id
        || s.scenes.len() != 22
    {
        return Err("unsupported pairing offset/club/scene domain".into());
    }
    let mut ids = BTreeSet::new();
    for p in &s.players {
        if p.id > 3920 || p.role > 13 || !ids.insert(p.id) {
            return Err("ambiguous or unsupported pairing player/role".into());
        }
    }
    if s.scenes
        .iter()
        .enumerate()
        .any(|(i, r)| usize::from(r.index) != i + 1 || !ids.contains(&r.word14))
        || s.clubs
            .iter()
            .flat_map(|c| c.roster)
            .any(|id| !ids.contains(&id))
    {
        return Err("missing or unordered pairing canonical input".into());
    }
    Ok(())
}
fn machine(s: &State) -> Result<Engine, String> {
    validate(s)?;
    let mut e = blank(s.seed);
    e.image[0xD56..0xD58].copy_from_slice(&s.side_offsets);
    for (side, c) in s.clubs.iter().enumerate() {
        put(&mut e, 0xD6E4 + 2 * side, c.id);
        let a = CLUB + 62 * usize::from(c.id - 1);
        for (i, id) in c.roster.iter().enumerate() {
            put(&mut e, a + 2 * i, *id);
        }
        e.image[a + 57] = c.byte57;
    }
    for r in &s.scenes {
        let a = address(r.index);
        e.image[a..a + 25].copy_from_slice(&r.bytes());
    }
    for p in &s.players {
        let id = usize::from(p.id);
        e.image[P8 + 8 * id + 6] = p.role;
        e.image[P15 + 15 * id + 8] = p.byte8;
        e.image[P15 + 15 * id + 11] = p.flags11;
    }
    load_preferences(&mut e, &s.preferences);
    Ok(e)
}
fn unavailable(e: &Engine, id: u16) -> u8 {
    let a = P15 + 15 * usize::from(id);
    placement::unavailable(&placement::Player {
        id,
        p15_sbyte8: e.image[a + 8] as i8,
        p15_byte11: e.image[a + 11],
    })
}
fn player(e: &Engine, side: usize, slot: u8) -> u16 {
    word(e, address(physical(e, side, slot)) + 14)
}
fn roster(e: &Engine, side: usize, slot: u8) -> u16 {
    let club = word(e, 0xD6E4 + 2 * side);
    word(
        e,
        CLUB + 62 * usize::from(club - 1) + 2 * usize::from(slot - 1),
    )
}
// [confirmed] F29/10B9: initialized caller arrays and fixed ordered1000 ranges.
pub(crate) fn pair_image(e: &mut Engine) -> Vec<Value> {
    let mut ranges = Vec::new();
    for side in 0..2 {
        put(e, address(physical(e, side, 1)) + 20, 0);
        let second = player(e, side, 2);
        if e.image[P8 + 8 * usize::from(second) + 6] == 1 {
            put(e, address(physical(e, side, 2)) + 20, 0);
        }
        e.image[OWN] = 10;
        e.image[OPP] = 10;
        for slot in 2..=11 {
            let own = player(e, side, slot);
            let role = e.image[P8 + 8 * usize::from(own) + 6];
            let flags = e.image[P15 + 15 * usize::from(own) + 11];
            e.image[OWN + usize::from(slot - 1)] =
                if role == 1 || flags & 0x40 != 0 || unavailable(e, own) != 0 {
                    0
                } else {
                    role
                };
            let opponent = player(e, 1 - side, slot);
            e.image[OPP + usize::from(slot - 1)] = if unavailable(e, opponent) != 0 {
                0
            } else {
                e.image[P8 + 8 * usize::from(opponent) + 6]
            };
        }
        for slot in 2..=11 {
            let id = player(e, side, slot);
            if e.image[P15 + 15 * usize::from(id) + 11] & 0x40 != 0 {
                let selected = word(e, address(physical(e, side, slot)) + 20);
                for opponent in 2..=11 {
                    if roster(e, 1 - side, opponent) == selected {
                        e.image[OPP + usize::from(opponent - 1)] = 0;
                    }
                }
            }
        }
        e.image[ASSIGNED..ASSIGNED + 11].fill(0);
        for (low, high, site) in [
            (12, 13, "0AB9:12AD"),
            (9, 11, "0AB9:12B5"),
            (5, 8, "0AB9:12BD"),
            (1, 1, "0AB9:12C5"),
            (2, 4, "0AB9:12CD"),
        ] {
            let input = locals(e);
            range_image(e, low, high);
            ranges.push(json!({"site":site,"arguments":[side,low,high],"input":input,"output":locals(e),"result":null,"draw_start":0,"draw_end":0}));
        }
        let club = word(e, 0xD6E4 + 2 * side);
        let enabled = e.image[CLUB + 62 * usize::from(club - 1) + 57] & 0x80 != 0;
        for slot in 2..=11 {
            let id = player(e, side, slot);
            let a = address(physical(e, side, slot)) + 20;
            if e.image[P15 + 15 * usize::from(id) + 11] & 0x40 == 0 {
                put(e, a, 0);
                let assigned = e.image[ASSIGNED + usize::from(slot - 1)];
                if enabled && assigned != 0 {
                    put(e, a, roster(e, 1 - side, assigned));
                }
            }
        }
    }
    ranges
}
pub fn range(s: &RangeInput) -> Result<RangeOutcome, String> {
    let mut e = range_machine(s)?;
    range_image(&mut e, s.low, s.high);
    Ok(RangeOutcome {
        output: locals(&e),
        draws: vec![],
    })
}
pub fn pair(s: &State) -> Result<Outcome, String> {
    let mut e = machine(s)?;
    let ranges = pair_image(&mut e);
    let mut output = s.clone();
    output.scenes = (1..=22).map(|i| Scene::read(&e, i)).collect();
    Ok(Outcome {
        output,
        ranges,
        draws: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // [designed] Static/synthetic cases,not additional original observations.
    fn sample_range() -> RangeInput {
        let mut own = [0; 11];
        own[0] = 10;
        let mut opponent = own;
        own[1] = 2;
        own[2] = 2;
        opponent[10] = 12;
        opponent[9] = 12;
        let mut preferences = [[255; 13]; 4];
        preferences[0][11] = 2;
        RangeInput {
            low: 12,
            high: 13,
            locals: Locals {
                own,
                opponent,
                assigned: [0; 11],
            },
            preferences,
        }
    }
    fn sample() -> State {
        let c: Value = include_str!("../tests/fixtures/role_pairing/cases.jsonl")
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .find(|c| c["stream"] == "pairings")
            .unwrap();
        let row: Scene = serde_json::from_value(c["input"]["scenes"][0].clone()).unwrap();
        let mut preferences = [[255; 13]; 4];
        preferences[0][1] = 2;
        State {
            seed: 0xBEEF,
            side_offsets: [0, 11],
            clubs: [
                Club {
                    id: 1,
                    roster: std::array::from_fn(|i| i as u16 + 1),
                    byte57: 0x80,
                },
                Club {
                    id: 2,
                    roster: std::array::from_fn(|i| i as u16 + 12),
                    byte57: 0x80,
                },
            ],
            scenes: (1..=22)
                .map(|i| {
                    let mut r = row.clone();
                    r.index = i;
                    r.word14 = u16::from(i);
                    r.word20 = 60000;
                    r
                })
                .collect(),
            players: (0..=22)
                .map(|id| Player {
                    id,
                    role: if [0, 1, 12].contains(&id) { 0 } else { 2 },
                    byte8: 1,
                    flags11: 0,
                })
                .collect(),
            preferences,
        }
    }
    #[test]
    fn reverse_opponent_order_and_first_duplicate_own_match_are_material() {
        let o = range(&sample_range()).unwrap().output;
        assert_eq!(o.assigned[1], 11);
        assert_eq!(o.assigned[2], 10);
        assert_eq!(&o.own[..3], &[10, 0, 0]);
        assert_eq!(o.opponent[0], 10);
        assert_eq!(o.opponent[9], 0);
        assert_eq!(o.opponent[10], 0);
        assert_eq!(o.assigned[0], 0);
    }
    #[test]
    fn earlier_preference_pass_wins_and_fourth_pass_is_reached_after_misses() {
        let mut s = sample_range();
        s.locals.opponent[9] = 0;
        s.locals.own[2] = 3;
        s.preferences[0][11] = 3;
        s.preferences[1][11] = 2;
        let o = range(&s).unwrap().output;
        assert_eq!(o.assigned[2], 11);
        assert_eq!(o.own[1], 2);
        s.preferences = [[255; 13]; 4];
        s.preferences[3][11] = 2;
        let o = range(&s).unwrap().output;
        assert_eq!(o.assigned[1], 11);
        assert_eq!(o.own[2], 3);
    }
    #[test]
    fn inclusive_bounds_skip_zero_and_outside_roles() {
        let mut s = sample_range();
        s.locals.opponent[9] = 13;
        s.locals.opponent[8] = 11;
        s.locals.own[2] = 3;
        s.preferences[0][12] = 3;
        s.preferences[0][10] = 2;
        let o = range(&s).unwrap().output;
        assert_eq!(o.assigned[1], 11);
        assert_eq!(o.assigned[2], 10);
        assert_eq!(o.opponent[8], 11);
        assert_eq!(o.opponent[7], 0);
    }
    #[test]
    fn nul_preference_matches_zero_and_can_overwrite_existing_assignment() {
        let mut s = sample_range();
        s.locals.own[1] = 0;
        s.locals.assigned[1] = 5;
        s.preferences[0][11] = 0;
        let o = range(&s).unwrap().output;
        assert_eq!(o.assigned[1], 10);
        assert_eq!(o.own[1], 0);
        assert_eq!(o.own[2], 2);
        assert_eq!(o.opponent[9], 0);
        assert_eq!(o.opponent[10], 0);
    }
    #[test]
    fn miss_preserves_existing_assignments_and_length_byte_is_not_searched() {
        let mut s = sample_range();
        s.locals.assigned[3] = 7;
        s.preferences = [[255; 13]; 4];
        assert_eq!(range(&s).unwrap().output, s.locals);
        s.preferences = [[10; 13]; 4];
        assert_eq!(range(&s).unwrap().output, s.locals);
    }
    #[test]
    fn invalid_ranges_and_local_sentinels_or_roles_reject() {
        let s = sample_range();
        for (low, high) in [(0, 13), (1, 14), (13, 12), (65535, 65535)] {
            let mut bad = s.clone();
            bad.low = low;
            bad.high = high;
            assert!(range(&bad).is_err());
        }
        let mut bad = s.clone();
        bad.locals.own[0] = 9;
        assert!(range(&bad).is_err());
        bad = s.clone();
        bad.locals.opponent[0] = 0;
        assert!(range(&bad).is_err());
        bad = s.clone();
        bad.locals.assigned[0] = 2;
        assert!(range(&bad).is_err());
        bad = s.clone();
        bad.locals.own[1] = 14;
        assert!(range(&bad).is_err());
        bad = s.clone();
        bad.locals.opponent[1] = 255;
        assert!(range(&bad).is_err());
        for assigned in [1, 12] {
            let mut bad = s.clone();
            bad.locals.assigned[1] = assigned;
            assert!(range(&bad).is_err());
        }
    }
    #[test]
    fn parent_composes_actual_arrays_across_ordered_ranges() {
        let mut s = sample();
        s.players[22].role = 12;
        s.preferences[0][11] = 2;
        let o = pair(&s).unwrap();
        assert_eq!(o.ranges.len(), 10);
        assert_eq!(o.ranges[0]["output"]["assigned"][1], 11);
        assert_eq!(o.ranges[1]["input"]["assigned"][1], 11);
        assert_eq!(o.ranges[4]["output"]["assigned"][1], 11);
        assert_eq!(o.output.scenes[1].word20, 22);
        let args: Vec<_> = o.ranges.iter().map(|r| r["arguments"].clone()).collect();
        assert_eq!(
            &args[..5],
            &[
                json!([0, 12, 13]),
                json!([0, 9, 11]),
                json!([0, 5, 8]),
                json!([0, 1, 1]),
                json!([0, 2, 4])
            ]
        );
    }
    #[test]
    fn club_bit_eight_gate_and_roster_words_drive_actual_scene_stores() {
        let mut s = sample();
        s.clubs[0].byte57 = 0x40;
        let o = pair(&s).unwrap();
        assert!(o.output.scenes[..11].iter().all(|r| r.word20 == 0));
        assert_eq!(o.ranges[4]["output"]["assigned"][1], 11);
        s.clubs[0].byte57 = 0x80;
        assert_eq!(pair(&s).unwrap().output.scenes[1].word20, 22);
        s.clubs[1].roster[10] = 21;
        let o = pair(&s).unwrap();
        assert_eq!(o.output.scenes[1].word20, 21);
        assert_eq!(s.scenes[21].word14, 22);
    }
    #[test]
    fn early_keeper_and_second_role_one_clears_override_retained_flags() {
        let mut s = sample();
        s.players[1].flags11 = 0x40;
        s.players[2].role = 1;
        s.players[2].flags11 = 0x40;
        s.players[3].role = 1;
        s.players[3].flags11 = 0x40;
        let o = pair(&s).unwrap();
        assert_eq!(o.output.scenes[0].word20, 0);
        assert_eq!(o.output.scenes[1].word20, 0);
        assert_eq!(o.output.scenes[2].word20, 60000);
        assert_eq!(o.ranges[0]["input"]["own"][1], 0);
        assert_eq!(o.ranges[0]["input"]["own"][2], 0);
    }
    #[test]
    fn availability_and_reserved_flag_masks_have_distinct_effects() {
        let mut s = sample();
        s.players[2].flags11 = 0x40;
        s.scenes[1].word20 = 22;
        s.players[3].flags11 = 0x80;
        s.players[4].byte8 = 128;
        s.scenes[4].word14 = 0;
        s.players[13].flags11 = 0x40;
        s.players[14].flags11 = 0x80;
        s.players[15].byte8 = 0;
        let o = pair(&s).unwrap();
        let input = &o.ranges[0]["input"];
        for i in 1..=4 {
            assert_eq!(input["own"][i], 0);
        }
        assert_eq!(input["opponent"][1], 2);
        assert_eq!(input["opponent"][2], 0);
        assert_eq!(input["opponent"][3], 0);
        assert_eq!(input["opponent"][10], 0);
        assert_eq!(o.output.scenes[1].word20, 22);
        assert_eq!(o.output.scenes[2].word20, 0);
    }
    #[test]
    fn reserved_unavailable_player_excludes_every_matching_roster_slot() {
        let mut s = sample();
        s.players[2].flags11 = 0xC0;
        s.scenes[1].word20 = 22;
        s.clubs[1].roster[9] = 22;
        let o = pair(&s).unwrap();
        let opponent = &o.ranges[0]["input"]["opponent"];
        assert_eq!(opponent[9], 0);
        assert_eq!(opponent[10], 0);
        assert_eq!(o.output.scenes[1].word20, 22);
    }
    #[test]
    fn own_role_one_is_excluded_but_opponent_role_one_can_be_assigned() {
        let mut s = sample();
        s.players[2].role = 1;
        s.players[3].role = 1;
        s.players[22].role = 1;
        s.preferences[0][0] = 2;
        let o = pair(&s).unwrap();
        assert_eq!(o.ranges[0]["input"]["own"][1], 0);
        assert_eq!(o.ranges[0]["input"]["own"][2], 0);
        assert_eq!(o.ranges[0]["input"]["opponent"][10], 1);
        assert_eq!(o.ranges[3]["output"]["assigned"][3], 11);
        assert_eq!(o.output.scenes[3].word20, 22);
    }
    #[test]
    fn swapped_offsets_and_maximum_club_player_are_supported() {
        let mut s = sample();
        s.side_offsets = [11, 0];
        s.clubs.swap(0, 1);
        let o = pair(&s).unwrap();
        assert_eq!(o.output.scenes[12].word20, 11);
        assert_eq!(o.output.scenes[1].word20, 22);
        let mut s = sample();
        s.clubs[1].id = 176;
        s.players[22].id = 3920;
        s.scenes[21].word14 = 3920;
        s.clubs[1].roster[10] = 3920;
        assert_eq!(pair(&s).unwrap().output.scenes[1].word20, 3920);
    }
    #[test]
    fn malformed_canonical_inputs_reject_without_changing_caller() {
        let s = sample();
        let before = serde_json::to_value(&s).unwrap();
        let mut bad = s.clone();
        bad.side_offsets = [0, 0];
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.clubs[0].id = 0;
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.clubs[1].id = 177;
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.clubs[1].id = bad.clubs[0].id;
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.scenes.pop();
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.scenes.swap(0, 1);
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.players.push(bad.players[0].clone());
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.players[0].role = 14;
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.players[0].id = 3921;
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.players.pop();
        assert!(pair(&bad).is_err());
        bad = s.clone();
        bad.clubs[0].roster[0] = 3921;
        assert!(pair(&bad).is_err());
        assert_eq!(serde_json::to_value(s).unwrap(), before);
    }
    #[test]
    fn expected_or_parent_local_results_are_not_inputs() {
        for field in ["output", "expected", "ranges", "assignments", "locals"] {
            let mut v = serde_json::to_value(sample()).unwrap();
            v[field] = json!(0);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
        for field in ["output", "expected", "ranges", "assignments"] {
            let mut v = serde_json::to_value(sample_range()).unwrap();
            v[field] = json!(0);
            assert!(serde_json::from_value::<RangeInput>(v).is_err());
        }
        let mut v = serde_json::to_value(sample()).unwrap();
        v["players"][0]["assigned"] = json!(11);
        assert!(serde_json::from_value::<State>(v).is_err());
    }
    #[test]
    fn standalone_range_changes_only_normalized_caller_arrays() {
        let s = sample_range();
        let mut e = range_machine(&s).unwrap();
        e.image[0x8000] = 0xAB;
        e.image[OWN + 15] = 0xA5;
        let before = e.image.clone();
        range_image(&mut e, s.low, s.high);
        let owned: BTreeSet<_> = (OWN..OWN + 11)
            .chain(OPP..OPP + 11)
            .chain(ASSIGNED..ASSIGNED + 11)
            .collect();
        for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
            if !owned.contains(&a) {
                assert_eq!(old, new, "unowned byte {a:X}");
            }
        }
        assert_eq!(word(&e, 0xA51C), 0xBEEF);
    }
    #[test]
    fn parent_initializes_all_used_locals_without_zero_scratch_assumption() {
        let s = sample();
        let expected = pair(&s).unwrap();
        let mut e = machine(&s).unwrap();
        e.image[OWN..ASSIGNED + 16].fill(0xA5);
        let ranges = pair_image(&mut e);
        assert_eq!(ranges, expected.ranges);
        assert_eq!(
            (1..=22).map(|i| Scene::read(&e, i)).collect::<Vec<_>>(),
            expected.output.scenes
        );
        assert_eq!(e.image[OWN + 15], 0xA5);
    }
    #[test]
    fn original_parent_image_ownership_and_named_word_deltas() {
        let mut deltas = vec![];
        for line in include_str!("../tests/fixtures/role_pairing/cases.jsonl").lines() {
            let c: Value = serde_json::from_str(line).unwrap();
            if c["stream"] != "pairings" {
                continue;
            }
            let s: State = serde_json::from_value(c["input"].clone()).unwrap();
            let mut e = machine(&s).unwrap();
            e.image[0x8000] = 0xAB;
            let before = e.image.clone();
            let ranges = pair_image(&mut e);
            let owned: BTreeSet<_> = (OWN..OWN + 11)
                .chain(OPP..OPP + 11)
                .chain(ASSIGNED..ASSIGNED + 11)
                .chain((1..=22).flat_map(|i| {
                    let a = 0xCCEF + 25 * i + 20;
                    [a, a + 1]
                }))
                .collect();
            for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
                if !owned.contains(&a) {
                    assert_eq!(old, new, "unowned byte {a:X}");
                }
            }
            assert_eq!(word(&e, 0xA51C), s.seed);
            assert_eq!(
                ranges[4]["output"]["assigned"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|v| v.as_u64().unwrap() != 0)
                    .count(),
                9
            );
            assert_eq!(
                ranges[9]["output"]["assigned"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|v| v.as_u64().unwrap() != 0)
                    .count(),
                8
            );
            deltas.push(
                s.scenes
                    .iter()
                    .filter(|r| r.word20 != Scene::read(&e, r.index).word20)
                    .count(),
            );
        }
        assert_eq!(deltas, vec![2, 1]); //Named value changes,not an original instruction-write census.
    }
}
