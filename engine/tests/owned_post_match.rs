use onenil_engine::watched_compiled::{
    owned_post_match as owned,
    prepared::{Input, Session},
};
use serde_json::{json, Value};
use std::path::Path;
fn input() -> Input {
    let v: Value =
        serde_json::from_str(include_str!("fixtures/prepared_match/admission.json")).unwrap();
    serde_json::from_value(v["input"].clone()).unwrap()
}
#[test]
fn continuing_fresh_and_retained_source_matches_original_match_then_caller() {
    for prefix in [0, 171] {
        let mut s = Session::new_with_post_match(input(), owned::original_admission())
            .expect("source-matched continuing caller");
        for _ in 0..prefix {
            s.advance().unwrap();
        }
        let got = s.finish_with_post_match().unwrap();
        assert!(s.finish_with_post_match().is_err());
        assert!(s.finish().is_err());
        assert!(s.advance().is_err());
        compare_match(&got["match"]);
        compare_post(&got["post_match"]);
    }
}
fn compare_match(got: &Value) {
    assert_eq!(got["return"], "0AB9:EBE2");
    assert_eq!(got["ticks"].as_array().unwrap().len(), 4199);
    assert_eq!(got["draws"].as_array().unwrap().len(), 182811);
    assert_eq!(got["events"].as_array().unwrap().len(), 831);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let reference = root.join("records/T41/reference");
    let index: Value =
        serde_json::from_slice(&std::fs::read(reference.join("index.json")).unwrap()).unwrap();
    let (mut t, mut d, mut a) = (0, 0, 0);
    for leaf in index["chunks"].as_array().unwrap() {
        let name = Path::new(leaf["path"].as_str().unwrap())
            .file_name()
            .unwrap();
        let chunk: Value =
            serde_json::from_slice(&std::fs::read(reference.join(name)).unwrap()).unwrap();
        for v in chunk["payload"]["ticks"].as_array().unwrap() {
            assert_eq!(got["ticks"][t], *v);
            t += 1;
        }
        for v in chunk["payload"]["draws"].as_array().unwrap() {
            let callers = v["callers"].as_array().unwrap();
            let site = if callers[0] == "24BC:0D4F" {
                &callers[1]
            } else {
                &callers[2]
            };
            assert_eq!(
                got["draws"][d],
                json!({"site":site,"n":v["n"],"seed_before":v["seed_before"],"seed_after":v["seed_after"],"value":v["value"]})
            );
            d += 1;
        }
        for v in chunk["payload"]["actions"].as_array().unwrap() {
            let mut v = v.clone();
            v.as_object_mut().unwrap().remove("event");
            assert_eq!(got["events"][a], v);
            a += 1;
        }
    }
    assert_eq!((t, d, a), (4199, 182811, 831));
}
fn compare_post(got: &Value) {
    let source: Value =
        serde_json::from_str(include_str!("fixtures/post_match/tail-r0009.json")).unwrap();
    let c = &source["payload"]["caller"];
    let start = c["draw_start"].as_u64().unwrap();
    assert_eq!(got["return"], "0AB9:EBE6");
    assert_eq!(got["source_final"], c["final"]);
    let expected:Vec<_>=source["payload"]["draws"].as_array().unwrap().iter().filter(|v|v["i"].as_u64().unwrap()>=start)
        .map(|v|json!({"n":v["n"],"seed_before":v["seed_before"],"seed_after":v["seed_after"],"value":v["value"]})).collect();
    let actual:Vec<_>=got["draws"].as_array().unwrap().iter().map(|v|json!({"n":v["n"],"seed_before":v["seed_before"],"seed_after":v["seed_after"],"value":v["value"]})).collect();
    assert_eq!(actual.len(), 103);
    assert_eq!(actual, expected);
    let writes:Vec<_>=c["operations"].as_array().unwrap().iter().filter(|v|v["kind"]=="write").map(|v|json!({
        "address":v["address"],"size":v["size"],"before":v["before"],"value":v["value"],"site":v["site"],"draw":v["draw"].as_u64().unwrap()-start})).collect();
    assert_eq!(got["source_writes"].as_array().unwrap().len(), 439);
    assert_eq!(got["source_writes"], json!(writes));
    for name in [
        "ax", "bx", "cx", "dx", "si", "di", "bp", "sp", "cs", "ds", "es", "ss",
    ] {
        assert_eq!(got["source_registers"][name], c["returned"][name]);
    }
}
