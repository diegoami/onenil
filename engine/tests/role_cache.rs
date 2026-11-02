use onenil_engine::role_cache::{self, State};
use serde_json::Value;
fn cases() -> Vec<Value> {
    include_str!("fixtures/role_cache/cases.jsonl")
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
#[test]
fn actual_original_role_roots_and_all_continuations_replay() {
    let cases = cases();
    assert_eq!(cases.len(), 8);
    for c in cases {
        let input: State = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(role_cache::select(&input).unwrap()).unwrap(),
            c["output"]
        );
    }
}
#[test]
fn actual_original_nested_helper_and_store_census() {
    let (mut visits, mut slots, mut scores, mut tables, mut stores) = (0, 0, 0, 0, 0);
    for c in cases() {
        let input: State = serde_json::from_value(c["input"].clone()).unwrap();
        let out = role_cache::select(&input).unwrap();
        for v in out.visits {
            visits += 1;
            stores += v["stored"].as_u64().unwrap();
            for h in v["helpers"].as_array().unwrap() {
                match h["kind"].as_str().unwrap() {
                    "slot" => slots += 1,
                    "old-score" | "new-score" => scores += 1,
                    _ => (),
                }
                tables += h["children"].as_array().unwrap().len();
            }
        }
    }
    assert_eq!(
        (visits, slots, scores, tables, stores),
        (82, 74, 148, 148, 27)
    );
}
