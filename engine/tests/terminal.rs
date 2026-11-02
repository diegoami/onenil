use onenil_engine::terminal::{self, State};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn cases() -> Vec<Value> {
    fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/terminal/cases.jsonl"),
    )
    .unwrap()
    .lines()
    .map(|s| serde_json::from_str(s).unwrap())
    .collect()
}
#[test]
fn actual_original_terminal_cases_replay() {
    for c in cases() {
        let input: State = serde_json::from_value(c["input"].clone()).unwrap();
        let got = terminal::positions(&input).unwrap();
        assert_eq!(serde_json::to_value(got).unwrap(), c["positions"]);
        let input2: State = serde_json::from_value(c["proximity_input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(terminal::proximity(&input2).unwrap()).unwrap(),
            c["proximity"]
        );
        assert_eq!(
            serde_json::to_value(terminal::terminal(&input).unwrap()).unwrap(),
            serde_json::json!({"positions":c["positions"],"proximity":c["proximity"]})
        );
    }
}
