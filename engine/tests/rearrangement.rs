use onenil_engine::rearrangement::{self, State};
use serde_json::Value;
#[test]
fn original_two_parents_and_all_164_caller_observations() {
    let mut n = 0;
    for line in include_str!("fixtures/rearrangement/cases.jsonl").lines() {
        let c: Value = serde_json::from_str(line).unwrap();
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        let out = rearrangement::rearrange(&s).unwrap();
        assert_eq!(serde_json::to_value(out).unwrap(), c["output"]);
        n += 1;
    }
    assert_eq!(n, 2);
}
