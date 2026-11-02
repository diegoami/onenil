use onenil_engine::role_scores::{self, SceneInput};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/role_scores")
}
fn cases() -> Vec<Value> {
    fs::read_to_string(root().join("cases.jsonl"))
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
#[test]
fn actual_original_scene_scores_and_nested_handoff_replay() {
    for c in cases() {
        let input: SceneInput = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(role_scores::scene(&input).unwrap()).unwrap(),
            c["output"]
        );
    }
}
#[test]
fn actual_original_table_scores_replay() {
    for c in cases() {
        let input: SceneInput = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(role_scores::table(&input.table).unwrap()).unwrap(),
            c["table"]
        );
    }
}
