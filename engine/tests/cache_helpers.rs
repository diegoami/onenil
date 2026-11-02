use onenil_engine::cache_helpers::{self, BoostInput, ManagedInput, SlotInput};
use serde_json::Value;
fn cases() -> Vec<Value> {
    include_str!("fixtures/cache_helpers/cases.jsonl")
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
#[test]
fn actual_original_managed_club_calls() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "managed")
        .collect();
    assert_eq!(rows.len(), 4);
    for c in rows {
        let s: ManagedInput = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(cache_helpers::managed(&s).unwrap()).unwrap(),
            c["output"]
        );
    }
}
#[test]
fn actual_original_cached_roster_slot_calls() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "slots")
        .collect();
    assert_eq!(rows.len(), 2);
    for c in rows {
        let s: SlotInput = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(cache_helpers::slot(&s).unwrap()).unwrap(),
            c["output"]
        );
    }
}
#[test]
fn actual_original_side_phases_and_all_rating_store_continuations() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "boosts")
        .collect();
    assert_eq!(rows.len(), 4);
    let mut calls = 0;
    for c in rows {
        let s: BoostInput = serde_json::from_value(c["input"].clone()).unwrap();
        let out = cache_helpers::boost(&s).unwrap();
        calls += out.visits.len();
        assert_eq!(serde_json::to_value(out).unwrap(), c["output"]);
    }
    assert_eq!(calls, 44);
}
