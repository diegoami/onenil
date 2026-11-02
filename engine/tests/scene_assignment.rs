use onenil_engine::scene_assignment::{self, State};
use serde_json::Value;
fn cases() -> Vec<Value> {
    include_str!("fixtures/scene_assignment/cases.jsonl")
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
#[test]
fn actual_original_forty_four_assignment_boundaries() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "assignments")
        .collect();
    assert_eq!(rows.len(), 44);
    for c in rows {
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        let o = scene_assignment::assign(&s).unwrap();
        assert_eq!(
            serde_json::to_value(o.output).unwrap(),
            c["output"]["output"]
        );
        assert!(o.draws.is_empty());
    }
}
#[test]
fn actual_original_two_rebuilds_and_twenty_observed_helpers() {
    fn count(v: &[Value]) -> usize {
        v.iter()
            .map(|e| 1 + count(e["children"].as_array().unwrap()))
            .sum()
    }
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "rebuilds")
        .collect();
    assert_eq!(rows.len(), 2);
    let mut n = 0;
    for c in rows {
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        let o = scene_assignment::rebuild(&s).unwrap();
        n += count(&o.helpers);
        assert_eq!(serde_json::to_value(o).unwrap(), c["output"]);
    }
    assert_eq!(n, 20);
}
