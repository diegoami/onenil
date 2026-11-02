use onenil_engine::neutral_input::{self, State};
use serde_json::Value;
#[test]
fn fourteen_static_neutral_branch_cases_are_not_original_observations() {
    let mut n = 0;
    for line in include_str!("fixtures/neutral_input/cases.jsonl").lines() {
        let c: Value = serde_json::from_str(line).unwrap();
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(neutral_input::neutral(&s).unwrap()).unwrap(),
            c["output"]
        );
        n += 1;
    }
    assert_eq!(n, 14);
}
