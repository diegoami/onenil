use onenil_engine::quick_reset::{self, State};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/quick_reset")
}
fn first_case() -> Value {
    serde_json::from_str(
        fs::read_to_string(fixtures().join("T21-r0001.jsonl"))
            .unwrap()
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap()
}
#[test]
fn first_actual_original_quick_reset_case_replays() {
    let c = first_case();
    let input: State = serde_json::from_value(c["input"].clone()).unwrap();
    let got = quick_reset::reset(&input).unwrap();
    assert_eq!(serde_json::to_value(got.output).unwrap(), c["output"]);
}
