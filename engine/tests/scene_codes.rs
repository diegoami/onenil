use onenil_engine::scene_codes::{self, CodeInput, State};
use serde_json::Value;
fn cases() -> Vec<Value> {
    include_str!("fixtures/scene_codes/cases.jsonl")
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
#[test]
fn actual_original_scene_cache_parents_and_every_continuation() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "parents")
        .collect();
    assert_eq!(rows.len(), 2);
    let (mut scenes, mut caches, mut codes) = (0, 0, 0);
    for c in rows {
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        let o = scene_codes::refresh(&s).unwrap();
        for e in &o.events {
            if e["kind"] == "scene" {
                scenes += 1;
            } else {
                caches += 1;
                codes += e["calls"].as_array().unwrap().len();
            }
        }
        assert_eq!(serde_json::to_value(o).unwrap(), c["output"]);
    }
    assert_eq!((scenes, caches, codes), (44, 2, 6));
}
#[test]
fn actual_original_six_code_calls_and_scene_results() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|c| c["stream"] == "codes")
        .collect();
    assert_eq!(rows.len(), 6);
    for c in rows {
        let s: CodeInput = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(scene_codes::code(&s).unwrap()).unwrap(),
            c["output"]
        );
    }
}
