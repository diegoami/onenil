use onenil_engine::substitution::{self, State};
use serde_json::Value;
#[test]
fn original_ninety_one_dispatchers_401_events_and_37_ai_continuations() {
    let mut n = 0;
    for line in include_str!("fixtures/substitution/cases.jsonl").lines() {
        let c: Value = serde_json::from_str(line).unwrap();
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(substitution::dispatch(&s).unwrap()).unwrap(),
            c["output"]
        );
        n += 1;
    }
    assert_eq!(n, 91);
}
