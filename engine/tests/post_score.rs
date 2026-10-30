use onenil_engine::post_score::{self, Input};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/post_score")
}
fn first_case() -> Value {
    serde_json::from_str(
        fs::read_to_string(fixtures().join("T20-r0001.jsonl"))
            .unwrap()
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap()
}
#[test]
fn first_actual_original_post_score_case_replays() {
    let c = first_case();
    let input = Input {
        state: serde_json::from_value(c["input"].clone()).unwrap(),
        argument: c["argument"].as_u64().unwrap().try_into().unwrap(),
    };
    let got = post_score::strategy(&input).unwrap();
    assert_eq!(serde_json::to_value(got.output).unwrap(), c["output"]);
}
