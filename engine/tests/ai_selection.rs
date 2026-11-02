use onenil_engine::ai_selection::{self, State};
use serde_json::Value;
#[test]
fn original_thirty_seven_calls_and_thirty_three_helpers() {
    let mut n = 0;
    for line in include_str!("fixtures/ai_selection/cases.jsonl").lines() {
        let c: Value = serde_json::from_str(line).unwrap();
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(ai_selection::select(&s).unwrap()).unwrap(),
            c["output"]
        );
        n += 1;
    }
    assert_eq!(n, 37);
}
