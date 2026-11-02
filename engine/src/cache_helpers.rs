//! [confirmed] F31/F32 at archaeology218490d: bounded1539 helper/store closure.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedInput {
    pub club: u16,
    pub managed_clubs: [u8; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SlotInput {
    pub side: u8,
    pub kind: u8,
    pub club: u16,
    pub roster: [u16; 11],
    pub cache: [[u16; 5]; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_5: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoostInput {
    pub side: u8,
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub cache: [[u16; 5]; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
}
#[derive(Debug, Serialize)]
pub struct Output {
    pub value: u16,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: Output,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct Visit {
    pub site: &'static str,
    pub store_site: &'static str,
    pub arguments: [u16; 3],
    pub input: Scene,
    pub rating: u16,
    pub output: Scene,
    pub draw_start: u8,
    pub draw_end: u8,
}
#[derive(Debug, Serialize)]
pub struct BoostOutcome {
    pub output: BoostInput,
    pub visits: Vec<Visit>,
    pub draws: Vec<Draw>,
}

use crate::{cache_selection, Engine, Input as ContestInput};
use std::collections::BTreeSet;
const P8: usize = crate::layout::P8;
const CLUB: usize = crate::layout::CLUB;
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn cache_address(side: u8, kind: u8) -> usize {
    0xCF5C + 10 * usize::from(side) + 2 * usize::from(kind)
}
fn blank(seed: u16) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed,
        ratings: [0; 8],
        goals: [0; 2],
    })
}
fn load_cache(e: &mut Engine, cache: &[[u16; 5]; 2]) {
    for (side, rows) in cache.iter().enumerate() {
        for (k, id) in rows.iter().enumerate() {
            put(e, cache_address(side as u8, k as u8 + 1), *id);
        }
    }
}
// [confirmed] F31/0E7A: full-word query against two zero-extended bytes.
pub(crate) fn managed_image(e: &Engine, club: u16) -> u8 {
    u8::from([u16::from(e.image[0xDCBA]), u16::from(e.image[0xDCBB])].contains(&club))
}
// [confirmed] F31/577F: first of eleven matches,including ID0;miss returns12.
pub(crate) fn slot_image(e: &Engine, side: u8, kind: u8) -> u16 {
    let club = word(e, 0xD6E4 + 2 * usize::from(side));
    let id = word(e, cache_address(side, kind));
    for slot in 1..=11 {
        if word(e, CLUB + 62 * usize::from(club - 1) + 2 * (slot - 1)) == id {
            return slot as u16;
        }
    }
    12
}
// [confirmed] F31/1625–163E: every call uses kind1 cached player,not target player.
pub(crate) fn boost_one(e: &mut Engine, side: u8, slot: u8) -> Visit {
    let index = e.image[0xD56 + usize::from(side)] + slot;
    let input = Scene::read(e, index);
    let id = word(e, cache_address(side, 1));
    let rating = cache_selection::rating_image(e, id);
    let a = 0xCCEF + 25 * usize::from(index) + 18;
    e.image[a] = e.image[a].wrapping_add(rating.wrapping_shl(1) as u8);
    Visit {
        site: "0AB9:1630",
        store_site: "0AB9:163E",
        arguments: [u16::from(slot), u16::from(index), id],
        input,
        rating,
        output: Scene::read(e, index),
        draw_start: 0,
        draw_end: 0,
    }
}
pub(crate) fn boost_image(e: &mut Engine, side: u8) -> Vec<Visit> {
    (1..=11).map(|slot| boost_one(e, side, slot)).collect()
}

fn slot_machine(s: &SlotInput) -> Result<Engine, String> {
    if s.side > 1
        || !(1..=5).contains(&s.kind)
        || !(1..=176).contains(&s.club)
        || s.roster
            .iter()
            .chain(s.cache.iter().flatten())
            .any(|id| *id > 3920)
    {
        return Err("unsupported cached-slot side/kind/club/player domain".into());
    }
    let mut e = blank(0xBEEF);
    load_cache(&mut e, &s.cache);
    put(&mut e, 0xD6E4 + 2 * usize::from(s.side), s.club);
    for (i, id) in s.roster.iter().enumerate() {
        put(&mut e, CLUB + 62 * usize::from(s.club - 1) + 2 * i, *id);
    }
    Ok(e)
}
fn boost_machine(s: &BoostInput) -> Result<Engine, String> {
    if s.side > 1
        || !matches!(s.side_offsets, [0, 11] | [11, 0])
        || s.scenes.len() != 22
        || s.cache.iter().flatten().any(|id| *id > 3920)
    {
        return Err("unsupported cached-boost side/offset/scene/player domain".into());
    }
    let mut ids = BTreeSet::new();
    for p in &s.players {
        if p.id > 3920 || !ids.insert(p.id) {
            return Err("ambiguous cached-boost player".into());
        }
    }
    if s.scenes
        .iter()
        .enumerate()
        .any(|(i, r)| usize::from(r.index) != i + 1 || !ids.contains(&r.word14))
        || s.cache.iter().flatten().any(|id| !ids.contains(id))
    {
        return Err("missing/unordered cached-boost scene or canonical player".into());
    }
    let mut e = blank(s.seed);
    load_cache(&mut e, &s.cache);
    e.image[0xD56..0xD58].copy_from_slice(&s.side_offsets);
    for r in &s.scenes {
        let a = 0xCCEF + 25 * usize::from(r.index);
        e.image[a..a + 25].copy_from_slice(&r.bytes());
    }
    for p in &s.players {
        let a = P8 + 8 * usize::from(p.id);
        for (o, v) in [(1, p.p8_1), (3, p.p8_3), (5, p.p8_5)] {
            e.image[a + o] = v;
        }
    }
    Ok(e)
}
pub fn managed(s: &ManagedInput) -> Result<Outcome, String> {
    let mut e = blank(0xBEEF);
    e.image[0xDCBA..0xDCBC].copy_from_slice(&s.managed_clubs);
    Ok(Outcome {
        output: Output {
            value: u16::from(managed_image(&e, s.club)),
        },
        draws: vec![],
    })
}
pub fn slot(s: &SlotInput) -> Result<Outcome, String> {
    let e = slot_machine(s)?;
    Ok(Outcome {
        output: Output {
            value: slot_image(&e, s.side, s.kind),
        },
        draws: vec![],
    })
}
pub fn boost(s: &BoostInput) -> Result<BoostOutcome, String> {
    let mut e = boost_machine(s)?;
    let visits = boost_image(&mut e, s.side);
    let mut output = s.clone();
    output.scenes = (1..=22).map(|i| Scene::read(&e, i)).collect();
    Ok(BoostOutcome {
        output,
        visits,
        draws: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    // [designed] Static/synthetic edges; original specimens are replayed separately.
    fn phase() -> BoostInput {
        let c: Value = include_str!("../tests/fixtures/cache_helpers/cases.jsonl")
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .find(|c| c["stream"] == "boosts")
            .unwrap();
        let mut s: BoostInput = serde_json::from_value(c["input"].clone()).unwrap();
        for p in &mut s.players {
            p.p8_1 = 21;
            p.p8_3 = 0;
            p.p8_5 = 0;
        }
        for r in &mut s.scenes {
            r.byte18 = 250;
        }
        let id = s.cache[usize::from(s.side)][0];
        s.players.iter_mut().find(|p| p.id == id).unwrap().p8_3 = 7;
        s
    }
    fn slot_input() -> SlotInput {
        SlotInput {
            side: 0,
            kind: 1,
            club: 1,
            roster: std::array::from_fn(|i| i as u16 + 1),
            cache: [[9; 5], [2; 5]],
        }
    }
    #[test]
    fn managed_matches_either_byte_and_duplicate_zero_entries() {
        for (club, bytes, value) in [
            (130, [130, 0], 1),
            (134, [130, 134], 1),
            (134, [130, 0], 0),
            (0, [0, 0], 1),
            (255, [255, 255], 1),
        ] {
            assert_eq!(
                managed(&ManagedInput {
                    club,
                    managed_clubs: bytes
                })
                .unwrap()
                .output
                .value,
                value
            );
        }
    }
    #[test]
    fn managed_query_compares_full_word_without_low_byte_alias() {
        for club in [256, 384, 65535] {
            assert_eq!(
                managed(&ManagedInput {
                    club,
                    managed_clubs: [0, 128]
                })
                .unwrap()
                .output
                .value,
                0
            );
        }
        assert_eq!(
            managed(&ManagedInput {
                club: 128,
                managed_clubs: [0, 128]
            })
            .unwrap()
            .output
            .value,
            1
        );
    }
    #[test]
    fn slot_returns_first_duplicate_last_slot_or_twelve() {
        let mut s = slot_input();
        assert_eq!(slot(&s).unwrap().output.value, 9);
        s.roster[1] = 9;
        s.roster[10] = 9;
        assert_eq!(slot(&s).unwrap().output.value, 2);
        s.roster = std::array::from_fn(|i| i as u16 + 1);
        s.cache[0][0] = 11;
        assert_eq!(slot(&s).unwrap().output.value, 11);
        s.cache[0][0] = 99;
        assert_eq!(slot(&s).unwrap().output.value, 12);
    }
    #[test]
    fn slot_zero_id_matches_first_zero_without_availability_filter() {
        let mut s = slot_input();
        s.cache[0][0] = 0;
        s.roster[3] = 0;
        s.roster[7] = 0;
        assert_eq!(slot(&s).unwrap().output.value, 4);
        s.roster = [0; 11];
        assert_eq!(slot(&s).unwrap().output.value, 1);
    }
    #[test]
    fn slot_uses_requested_side_kind_and_maximum_club_player() {
        for side in [0, 1] {
            for kind in 1..=5 {
                let mut s = slot_input();
                s.side = side;
                s.kind = kind;
                s.club = 176;
                s.cache[usize::from(side)][usize::from(kind - 1)] = 3920;
                s.roster[5] = 3920;
                assert_eq!(slot(&s).unwrap().output.value, 6);
            }
        }
    }
    #[test]
    fn slot_rejects_unsupported_domains_without_mutating_caller() {
        let s = slot_input();
        let before = serde_json::to_value(&s).unwrap();
        for kind in [0, 6, 255] {
            let mut bad = s.clone();
            bad.kind = kind;
            assert!(slot(&bad).is_err());
        }
        for club in [0, 177, 65535] {
            let mut bad = s.clone();
            bad.club = club;
            assert!(slot(&bad).is_err());
        }
        let mut bad = s.clone();
        bad.side = 2;
        assert!(slot(&bad).is_err());
        bad = s.clone();
        bad.roster[10] = 3921;
        assert!(slot(&bad).is_err());
        bad = s.clone();
        bad.cache[1][4] = 3921;
        assert!(slot(&bad).is_err());
        assert_eq!(serde_json::to_value(s).unwrap(), before);
    }
    #[test]
    fn boost_calls_cached_player_for_every_target_and_wraps_byte() {
        let s = phase();
        let o = boost(&s).unwrap();
        assert_eq!(o.visits.len(), 11);
        let id = s.cache[usize::from(s.side)][0];
        for (i, v) in o.visits.iter().enumerate() {
            assert_eq!(
                v.arguments,
                [
                    i as u16 + 1,
                    u16::from(s.side_offsets[usize::from(s.side)]) + i as u16 + 1,
                    id
                ]
            );
            assert_eq!(v.rating, 7);
            assert_eq!(v.input.byte18, 250);
            assert_eq!(v.output.byte18, 8);
        }
        assert!(o.visits.iter().any(|v| v.input.word14 != id));
        assert_eq!(o.output.cache, s.cache);
        assert_eq!(o.output.seed, s.seed);
    }
    #[test]
    fn changed_raw_cached_rating_drives_actual_store() {
        let mut s = phase();
        let first = boost(&s).unwrap();
        let id = s.cache[usize::from(s.side)][0];
        s.players.iter_mut().find(|p| p.id == id).unwrap().p8_3 = 3;
        let after = boost(&s).unwrap();
        assert_eq!(first.visits[0].rating, 7);
        assert_eq!(after.visits[0].rating, 3);
        assert_eq!(after.visits[0].output.byte18, 0);
    }
    #[test]
    fn negative_remainder_and_logical_shift_preserve_word_and_low_byte() {
        let mut s = phase();
        let id = s.cache[usize::from(s.side)][0];
        let p = s.players.iter_mut().find(|p| p.id == id).unwrap();
        p.p8_1 = 21;
        p.p8_3 = 0;
        p.p8_5 = 7;
        let o = boost(&s).unwrap();
        assert_eq!(o.visits[0].rating, 65529);
        assert_eq!(o.visits[0].output.byte18, 236);
        s.players.iter_mut().find(|p| p.id == id).unwrap().p8_1 = 20;
        let o = boost(&s).unwrap();
        assert_eq!(o.visits[0].rating, 32764);
        assert_eq!(o.visits[0].output.byte18, 242);
    }
    #[test]
    fn boost_zero_cached_id_reads_its_canonical_rating() {
        let mut s = phase();
        s.cache[usize::from(s.side)][0] = 0;
        s.players.iter_mut().find(|p| p.id == 0).unwrap().p8_3 = 7;
        let o = boost(&s).unwrap();
        assert!(o
            .visits
            .iter()
            .all(|v| v.rating == 7 && v.arguments[2] == 0 && v.output.byte18 == 8));
    }
    #[test]
    fn alternate_sides_offsets_and_maximum_player_are_supported() {
        for side in [0, 1] {
            for offsets in [[0, 11], [11, 0]] {
                let mut s = phase();
                s.side = side;
                s.side_offsets = offsets;
                let id = s.cache[usize::from(side)][0];
                s.players.iter_mut().find(|p| p.id == id).unwrap().p8_3 = 7;
                let o = boost(&s).unwrap();
                assert_eq!(
                    o.visits[0].arguments[1],
                    u16::from(offsets[usize::from(side)]) + 1
                );
                assert_eq!(o.visits[10].output.byte18, 8);
            }
        }
        let mut s = phase();
        let id = s.cache[usize::from(s.side)][0];
        s.players.iter_mut().find(|p| p.id == id).unwrap().id = 3920;
        for row in &mut s.scenes {
            if row.word14 == id {
                row.word14 = 3920;
            }
        }
        for v in s.cache.iter_mut().flatten() {
            if *v == id {
                *v = 3920;
            }
        }
        assert_eq!(boost(&s).unwrap().visits[0].arguments[2], 3920);
    }
    #[test]
    fn malformed_boost_inputs_diagnose_without_changing_caller() {
        let s = phase();
        let before = serde_json::to_value(&s).unwrap();
        let mut bad = s.clone();
        bad.side = 2;
        assert!(boost(&bad).is_err());
        bad = s.clone();
        bad.side_offsets = [11, 11];
        assert!(boost(&bad).is_err());
        bad = s.clone();
        bad.scenes.pop();
        assert!(boost(&bad).is_err());
        bad = s.clone();
        bad.scenes.swap(0, 1);
        assert!(boost(&bad).is_err());
        bad = s.clone();
        bad.scenes[0].word14 = 3921;
        assert!(boost(&bad).is_err());
        bad = s.clone();
        bad.cache[0][0] = 3921;
        assert!(boost(&bad).is_err());
        bad = s.clone();
        bad.players[0].id = 3921;
        assert!(boost(&bad).is_err());
        bad = s.clone();
        bad.players.push(bad.players[0].clone());
        assert!(boost(&bad).is_err());
        bad = s.clone();
        let id = bad.cache[0][0];
        bad.players.retain(|p| p.id != id);
        assert!(boost(&bad).is_err());
        assert_eq!(serde_json::to_value(s).unwrap(), before);
    }
    #[test]
    fn expected_returns_and_nested_fields_are_not_inputs() {
        for field in ["output", "expected", "visits", "rating", "result"] {
            let mut v = json!({"club":1,"managed_clubs":[1,0]});
            v[field] = json!(1);
            assert!(serde_json::from_value::<ManagedInput>(v).is_err());
            let mut v = serde_json::to_value(slot_input()).unwrap();
            v[field] = json!(1);
            assert!(serde_json::from_value::<SlotInput>(v).is_err());
            let mut v = serde_json::to_value(phase()).unwrap();
            v[field] = json!(1);
            assert!(serde_json::from_value::<BoostInput>(v).is_err());
        }
        let mut v = serde_json::to_value(phase()).unwrap();
        v["players"][0]["rating"] = json!(7);
        assert!(serde_json::from_value::<BoostInput>(v).is_err());
    }
    #[test]
    fn primitive_image_and_seed_are_read_only() {
        let s = slot_input();
        let mut e = slot_machine(&s).unwrap();
        e.image[0x8000] = 0xAB;
        e.image[0xDCBA] = 130;
        e.image[0xDCBB] = 134;
        let before = e.image.clone();
        assert_eq!(managed_image(&e, 134), 1);
        assert_eq!(slot_image(&e, s.side, s.kind), 9);
        assert_eq!(before, e.image);
        assert_eq!(word(&e, 0xA51C), 0xBEEF);
    }
    #[test]
    fn whole_image_ownership_on_all_four_original_phases() {
        for line in include_str!("../tests/fixtures/cache_helpers/cases.jsonl").lines() {
            let c: Value = serde_json::from_str(line).unwrap();
            if c["stream"] != "boosts" {
                continue;
            }
            let s: BoostInput = serde_json::from_value(c["input"].clone()).unwrap();
            let mut e = boost_machine(&s).unwrap();
            e.image[0x8000] = 0xAB;
            let before = e.image.clone();
            boost_image(&mut e, s.side);
            let owned: BTreeSet<_> = (1..=11)
                .map(|slot| {
                    0xCCEF + 25 * (usize::from(s.side_offsets[usize::from(s.side)]) + slot) + 18
                })
                .collect();
            for (i, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
                if !owned.contains(&i) {
                    assert_eq!(old, new, "unowned byte {i:X}");
                }
            }
            assert_eq!(word(&e, 0xA51C), s.seed);
        }
    }
    #[test]
    fn both_side_phases_compose_on_one_image_without_cache_or_seed_changes() {
        let mut s = phase();
        for side in 0..2 {
            let id = s.cache[side][0];
            s.players.iter_mut().find(|p| p.id == id).unwrap().p8_3 = 7;
        }
        let mut e = boost_machine(&s).unwrap();
        let a = boost_image(&mut e, 0);
        let b = boost_image(&mut e, 1);
        assert_eq!(a.len() + b.len(), 22);
        assert!((1..=22).all(|i| Scene::read(&e, i).byte18 == 8));
        assert_eq!(word(&e, 0xA51C), s.seed);
        for side in 0..2 {
            for k in 1..=5 {
                assert_eq!(
                    word(&e, cache_address(side, k)),
                    s.cache[usize::from(side)][usize::from(k - 1)]
                );
            }
        }
    }
}
