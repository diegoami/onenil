use onenil_engine::cache_selection::{self, RatingInput, SelectorInput};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cache_selection")
}
fn rows(name: &str) -> Vec<Value> {
    fs::read_to_string(root().join(name))
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
#[test]
fn actual_original_cached_ratings_replay() {
    for c in rows("ratings.jsonl") {
        let input: RatingInput = serde_json::from_value(c["input"].clone()).unwrap();
        let out = cache_selection::rating(&input).unwrap();
        assert_eq!(serde_json::to_value(out.output).unwrap(), c["output"]);
        assert!(out.draws.is_empty());
    }
}
#[test]
fn actual_original_cached_selectors_replay() {
    for c in rows("selectors.jsonl") {
        let input: SelectorInput = serde_json::from_value(c["input"].clone()).unwrap();
        let out = cache_selection::select(&input).unwrap();
        assert_eq!(serde_json::to_value(out.output).unwrap(), c["output"]);
        assert_eq!(out.visits.len(), 11);
        assert!(out.draws.is_empty());
    }
}
