use onenil_engine::extra_event::{self, Input};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/extra_event")
}
fn first_case() -> (Value, Input) {
    let text = fs::read_to_string(fixtures().join("T17-r0005.jsonl")).unwrap();
    let case: Value = serde_json::from_str(text.lines().next().unwrap()).unwrap();
    let text = fs::read_to_string(fixtures().join("inputs/T18-r0001.jsonl")).unwrap();
    let cell: Value = serde_json::from_str(text.lines().next().unwrap()).unwrap();
    assert_eq!(cell["step"], case["step"]);
    assert_eq!(cell["seed"], case["input"]["seed"]);
    let input = Input {
        state: serde_json::from_value(case["input"].clone()).unwrap(),
        opponent_d0b: cell["value"].as_u64().unwrap() as u8,
    };
    (case, input)
}
#[test]
fn first_actual_original_dbc6_case_replays() {
    let (case, input) = first_case();
    let got = extra_event::decision(&input).unwrap();
    let expected = if case["dispatch"].is_null() {
        &case["output"]
    } else {
        &case["dispatch"]["input"]
    };
    assert_eq!(serde_json::to_value(got.output).unwrap(), *expected);
}
