use onenil_engine::event_effects::{self, State};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/event_effects")
}
fn first_case() -> Value {
    let text = fs::read_to_string(fixtures().join("T19-r0002.jsonl")).unwrap();
    serde_json::from_str(text.lines().next().unwrap()).unwrap()
}
#[test]
fn first_actual_original_69b8_effect_case_replays() {
    let case = first_case();
    let input: State = serde_json::from_value(case["input"].clone()).unwrap();
    let got = event_effects::handler(&input).unwrap();
    assert_eq!(serde_json::to_value(&got.output).unwrap(), case["output"]);
}
