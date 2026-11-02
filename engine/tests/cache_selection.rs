use onenil_engine::cache_selection::{self, RatingInput, SelectorInput};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cache_selection")
}
fn rows(name: &str) -> Vec<Value> {
    fs::read_to_string(root().join(name))
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
#[test]
fn actual_original_cached_ratings_replay() {
    for c in rows("ratings.jsonl") {
        let input: RatingInput = serde_json::from_value(c["input"].clone()).unwrap();
        let out = cache_selection::rating(&input).unwrap();
        assert_eq!(serde_json::to_value(out.output).unwrap(), c["output"]);
        assert!(out.draws.is_empty());
    }
}
#[test]
fn actual_original_cached_selectors_replay() {
    for c in rows("selectors.jsonl") {
        let input: SelectorInput = serde_json::from_value(c["input"].clone()).unwrap();
        let out = cache_selection::select(&input).unwrap();
        assert_eq!(serde_json::to_value(out.output).unwrap(), c["output"]);
        assert_eq!(out.visits.len(), 11);
        assert!(out.draws.is_empty());
    }
}

// [designed] Synthetic tests below establish static edge behavior,not new original coverage.
fn input() -> SelectorInput {
    SelectorInput {
        side: 0,
        club: 1,
        roster: std::array::from_fn(|i| i as u16 + 1),
        players: (1..=11)
            .map(|id| cache_selection::Player {
                id,
                p8_1: 21,
                p8_3: 0,
                p8_5: 0,
                byte8: 10,
                flags11: 0,
            })
            .collect(),
    }
}
fn eligible(s: &mut SelectorInput, ids: &[u16]) {
    for p in &mut s.players {
        p.byte8 = if ids.contains(&p.id) { 10 } else { 0 }
    }
}
fn rate(class: u8, a: u8, b: u8) -> u16 {
    cache_selection::rating(&RatingInput {
        p8_1: class,
        p8_3: a,
        p8_5: b,
    })
    .unwrap()
    .output
    .value
}
#[test]
fn signed_remainder_is_not_quotient_and_logical_shift_preserves_word_bits() {
    for (class, a, b, expected) in [
        (21, 217, 14, 3),
        (27, 174, 15, 7),
        (21, 255, 0, 7),
        (21, 0, 255, 65529),
        (21, 0, 1, 65535),
        (20, 0, 1, 32767),
        (20, 0, 7, 32764),
        (20, 255, 0, 3),
        (21, 8, 0, 0),
        (0, 0, 0, 0),
        (255, 0, 8, 0),
    ] {
        assert_eq!(rate(class, a, b), expected);
    }
}
#[test]
fn ordinary_tie_keeps_first_but_high_bit_current_is_not_unsigned_maximum() {
    let mut s = input();
    eligible(&mut s, &[1, 2]);
    s.players[0].p8_3 = 7;
    s.players[1].p8_3 = 7;
    let out = cache_selection::select(&s).unwrap();
    assert_eq!(out.output.player, 1);
    assert!(out.visits[0].selected);
    assert!(!out.visits[1].selected);
    s.players[0].p8_3 = 0;
    s.players[0].p8_5 = 1;
    s.players[1].p8_3 = 0;
    let out = cache_selection::select(&s).unwrap();
    assert_eq!(out.output.player, 2);
    assert_eq!(out.visits[0].rating, Some(65535));
    assert!(out.visits[1].selected);
}
#[test]
fn equal_raw_high_bit_ratings_replace_because_current_is_sign_extended() {
    let mut s = input();
    eligible(&mut s, &[1, 2]);
    s.players[0].p8_5 = 1;
    s.players[1].p8_5 = 1;
    let out = cache_selection::select(&s).unwrap();
    assert_eq!(out.output.player, 2);
    assert_eq!(out.visits[1].retained_score, 65535);
    assert!(out.visits[1].selected);
}
#[test]
fn shifted_negative_score_is_positive_and_keeps_an_ordinary_tie() {
    let mut s = input();
    eligible(&mut s, &[1, 2]);
    for p in &mut s.players[..2] {
        p.p8_1 = 20;
        p.p8_5 = 1;
    }
    let out = cache_selection::select(&s).unwrap();
    assert_eq!(out.output.player, 1);
    assert_eq!(out.visits[1].rating, Some(32767));
    assert!(!out.visits[1].selected);
}
#[test]
fn first_eligible_after_unavailable_entries_is_selected_even_with_zero_score() {
    let mut s = input();
    eligible(&mut s, &[5]);
    let out = cache_selection::select(&s).unwrap();
    assert_eq!(out.output.player, 5);
    assert_eq!(out.visits[4].retained_score, 0);
    assert_eq!(out.visits[4].rating_calls, 2);
    assert!(out.visits[..4]
        .iter()
        .all(|v| v.rating.is_none() && v.rating_calls == 0));
}
#[test]
fn native_visit_census_follows_static_double_rating_on_improvement() {
    let mut s = input();
    for (n, p) in s.players.iter_mut().enumerate() {
        p.p8_3 = (n % 8) as u8;
    }
    let out = cache_selection::select(&s).unwrap();
    assert_eq!(out.output.player, 8);
    assert_eq!(out.visits.iter().filter(|v| v.selected).count(), 8);
    assert_eq!(
        out.visits
            .iter()
            .map(|v| u32::from(v.rating_calls))
            .sum::<u32>(),
        19
    );
    assert!(out.draws.is_empty());
}
#[test]
fn zero_signed_byte_and_status_flags_skip_candidates_without_rating() {
    for mode in 0..7 {
        let mut s = input();
        eligible(&mut s, &[1, 2]);
        s.players[0].p8_3 = 7;
        match mode {
            0 => s.players[0].byte8 = 0,
            1 => s.players[0].byte8 = 128,
            2 => s.players[0].byte8 = 255,
            3 => s.players[0].flags11 = 0x80,
            4 => s.players[0].flags11 = 0x20,
            5 => s.players[0].flags11 = 0x02,
            _ => s.players[0].flags11 = 0x04,
        }
        let out = cache_selection::select(&s).unwrap();
        assert_eq!(out.output.player, 2);
        assert_eq!(out.visits[0].unavailable, 1);
        assert_eq!(out.visits[0].rating, None);
    }
    let mut s = input();
    eligible(&mut s, &[1]);
    s.players[0].flags11 = 0x59;
    assert_eq!(cache_selection::select(&s).unwrap().output.player, 1);
}
#[test]
fn shared_roster_ids_use_one_canonical_record_and_zero_id_is_unavailable() {
    let mut s = input();
    s.roster[1] = 1;
    s.players.retain(|p| p.id != 2);
    eligible(&mut s, &[1]);
    let out = cache_selection::select(&s).unwrap();
    assert_eq!(out.output.player, 1);
    assert_eq!(out.visits[1].player, 1);
    assert!(!out.visits[1].selected);
    s.roster[0] = 0;
    s.roster[1] = 1;
    s.players.push(cache_selection::Player {
        id: 0,
        p8_1: 21,
        p8_3: 7,
        p8_5: 0,
        byte8: 10,
        flags11: 0,
    });
    let out = cache_selection::select(&s).unwrap();
    assert_eq!(out.output.player, 1);
    assert_eq!(out.visits[0].unavailable, 1);
}
#[test]
fn no_eligible_reports_undefined_local_instead_of_inventing_a_player() {
    let mut s = input();
    eligible(&mut s, &[]);
    let before = s.clone();
    assert!(cache_selection::select(&s)
        .unwrap_err()
        .contains("no eligible"));
    assert_eq!(s, before);
    s.roster = [0; 11];
    s.players = vec![cache_selection::Player {
        id: 0,
        p8_1: 0,
        p8_3: 0,
        p8_5: 0,
        byte8: 10,
        flags11: 0,
    }];
    assert!(cache_selection::select(&s).is_err());
}
#[test]
fn independent_side_club_bounds_and_maximum_player_are_supported() {
    let mut s = input();
    s.side = 1;
    s.club = 176;
    s.roster[0] = 3920;
    s.players[0].id = 3920;
    eligible(&mut s, &[3920]);
    assert_eq!(cache_selection::select(&s).unwrap().output.player, 3920);
}
#[test]
fn malformed_and_expected_inputs_reject_without_mutating_callers() {
    for damage in 0..7 {
        let mut s = input();
        match damage {
            0 => s.side = 2,
            1 => s.club = 0,
            2 => s.club = 177,
            3 => s.roster[0] = 3921,
            4 => {
                s.players.pop();
            }
            5 => s.players.push(s.players[0].clone()),
            _ => s.players.push(cache_selection::Player {
                id: 3920,
                ..s.players[0].clone()
            }),
        }
        let before = s.clone();
        assert!(cache_selection::select(&s).is_err());
        assert_eq!(s, before);
    }
    for key in ["output", "expected", "visits"] {
        let mut v = serde_json::to_value(input()).unwrap();
        v[key] = serde_json::json!({});
        assert!(serde_json::from_value::<SelectorInput>(v).is_err());
        let mut v = serde_json::json!({"p8_1":0,"p8_3":0,"p8_5":0});
        v[key] = serde_json::json!({});
        assert!(serde_json::from_value::<RatingInput>(v).is_err());
    }
}
#[test]
fn changed_expected_results_cannot_steer_native_selection_or_rating() {
    let mut c = rows("selectors.jsonl")[0].clone();
    let input: SelectorInput = serde_json::from_value(c["input"].clone()).unwrap();
    let before = cache_selection::select(&input).unwrap();
    c["output"]["player"] = serde_json::json!(0);
    assert_eq!(
        cache_selection::select(&input).unwrap().output.player,
        before.output.player
    );
    assert_ne!(serde_json::to_value(before.output).unwrap(), c["output"]);
    let mut c = rows("ratings.jsonl")[0].clone();
    let input: RatingInput = serde_json::from_value(c["input"].clone()).unwrap();
    let before = cache_selection::rating(&input).unwrap();
    c["output"]["value"] = serde_json::json!(1234);
    assert_eq!(
        cache_selection::rating(&input).unwrap().output.value,
        before.output.value
    );
    assert_ne!(serde_json::to_value(before.output).unwrap(), c["output"]);
}
#[test]
fn original_fixture_provenance_and_stream_hashes_are_pinned() {
    use sha2::{Digest, Sha256};
    let pin: Value =
        serde_json::from_slice(&fs::read(root().join("provenance.json")).unwrap()).unwrap();
    let data = fs::read(root().join("index.json")).unwrap();
    assert_eq!(pin["commit"], "218490d846e365babdd158d09c5ab865124e30ed");
    assert_eq!(pin["index_sha256"], format!("{:x}", Sha256::digest(&data)));
    let index: Value = serde_json::from_slice(&data).unwrap();
    assert_eq!(index["schema"], "cache-selection-corpus/1");
    for name in ["selectors", "ratings"] {
        let data = fs::read(root().join(format!("{name}.jsonl"))).unwrap();
        assert_eq!(
            index["streams"][name]["cases_sha256"],
            format!("{:x}", Sha256::digest(data))
        );
    }
}
