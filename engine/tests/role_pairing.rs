use onenil_engine::role_pairing::{self, RangeInput, State};
use serde_json::Value;
fn cases() -> Vec<Value> {
    include_str!("fixtures/role_pairing/cases.jsonl")
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
#[test]
fn actual_original_parent_pairings_and_every_ordered_range() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "pairings")
        .collect();
    assert_eq!(rows.len(), 2);
    let mut ranges = 0;
    for c in rows {
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        let o = role_pairing::pair(&s).unwrap();
        ranges += o.ranges.len();
        assert_eq!(serde_json::to_value(o).unwrap(), c["output"]);
    }
    assert_eq!(ranges, 20);
}
#[test]
fn actual_original_twenty_standalone_range_boundaries() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "ranges")
        .collect();
    assert_eq!(rows.len(), 20);
    for c in rows {
        let s: RangeInput = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(role_pairing::range(&s).unwrap()).unwrap(),
            c["output"]
        );
    }
}
