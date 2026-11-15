use onenil_engine::watched_compiled::post_match::{self, Input};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
const RUNS: [&[u8]; 2] = [
    include_bytes!("fixtures/post_match/tail-r0009.json"),
    include_bytes!("fixtures/post_match/tail-r0010.json"),
];
const PIN: &str = "1163ecc7f55c46d35a3cc74d80792e8e0bb7511ea21b301792effceef1a7d4d0";
fn input(record: &Value) -> Input {
    let c = &record["payload"]["caller"];
    let r = &c["entry"];
    serde_json::from_value(json!({
        "registers":[r["ax"],r["cx"],r["dx"],r["bx"],r["sp"],r["bp"],r["si"],r["di"],r["es"],r["cs"],r["ss"],r["ds"]],
        "flags":r["eflags"],"regions":c["regions"],"initial":c["initial"]
    })).unwrap()
}
#[test]
fn both_original_callers_match_all_outputs_rng_and_ordered_writes() {
    for bytes in RUNS {
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), PIN);
        let record: Value = serde_json::from_slice(bytes).unwrap();
        let c = &record["payload"]["caller"];
        let actual = post_match::replay(&input(&record)).expect("original caller must execute");
        assert_eq!(
            serde_json::to_value(&actual.final_fields).unwrap(),
            c["final"]
        );
        let expected: Vec<_> = record["payload"]["draws"].as_array().unwrap().iter()
            .filter(|d| d["i"].as_u64().unwrap() >= c["draw_start"].as_u64().unwrap())
            .map(|d| json!({"n":d["n"],"seed_before":d["seed_before"],"seed_after":d["seed_after"],"value":d["value"]})).collect();
        let draws: Vec<_> = actual.draws.iter().map(|d|json!({"n":d.n,"seed_before":d.seed_before,"seed_after":d.seed_after,"value":d.value})).collect();
        assert_eq!(draws.len(), 103);
        assert_eq!(draws, expected);
        let start = c["draw_start"].as_u64().unwrap();
        let expected: Vec<_> = c["operations"].as_array().unwrap().iter().filter(|v|v["kind"]=="write")
            .map(|v|json!({"address":v["address"],"size":v["size"],"before":v["before"],"value":v["value"],"site":v["site"],"draw":v["draw"].as_u64().unwrap()-start})).collect();
        assert_eq!(actual.writes.len(), 439);
        assert_eq!(
            serde_json::to_value(&actual.writes).unwrap(),
            json!(expected)
        );
        for (index, name) in [(4, "sp"), (5, "bp"), (9, "cs"), (10, "ss"), (11, "ds")] {
            assert_eq!(json!(actual.registers[index]), c["returned"][name]);
        }
    }
}
