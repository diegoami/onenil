use onenil_engine::condition_refresh::{self, Input};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const RUN: &[u8] = include_bytes!("fixtures/condition_refresh/calls-r0007.json");
const REPEAT: &[u8] = include_bytes!("fixtures/condition_refresh/calls-r0008.json");
const SOURCE: &str = include_str!("fixtures/condition_refresh/source.json");

#[test]
fn original_twenty_one_calls_and_all_draws_and_writes() {
    let record: Value = serde_json::from_slice(RUN).unwrap();
    let mut count = 0;
    let mut draws = 0;
    for case in record["cases"].as_array().unwrap() {
        let input: Input = serde_json::from_value(case["input"].clone()).unwrap();
        let actual = condition_refresh::replay(&input).unwrap();
        assert_eq!(
            serde_json::to_value(&actual.output).unwrap(),
            case["output"],
            "{}",
            case["name"]
        );
        assert_eq!(
            serde_json::to_value(&actual.writes).unwrap(),
            case["writes"],
            "{}",
            case["name"]
        );
        let projected: Vec<_> = actual
            .draws
            .iter()
            .map(|d| {
                json!({
                    "n":d.n,"seed_before":d.seed_before,"seed_after":d.seed_after,"value":d.value
                })
            })
            .collect();
        assert_eq!(json!(projected), case["draws"], "{}", case["name"]);
        draws += actual.draws.len();
        count += 1;
    }
    assert_eq!((count, draws), (21, 47));
}

#[test]
fn immutable_original_source_identity_and_repeat() {
    let source: Value = serde_json::from_str(SOURCE).unwrap();
    assert_eq!(
        source["research_commit"],
        "83b7ef7c27eac15bcc5be1c8173e669d198cda24"
    );
    for (name, data) in [("calls-r0007.json", RUN), ("calls-r0008.json", REPEAT)] {
        assert_eq!(format!("{:x}", Sha256::digest(data)), source["files"][name]);
    }
    let a: Value = serde_json::from_slice(RUN).unwrap();
    let b: Value = serde_json::from_slice(REPEAT).unwrap();
    assert_eq!(a["cases"], b["cases"]);
    assert_eq!(a["producer"], "e04ca817177b892f47fed040244f0ddc881cb152");
}

#[test]
fn incomplete_or_foreign_input_cannot_enter_execution() {
    let record: Value = serde_json::from_slice(RUN).unwrap();
    let input = record["cases"][0]["input"].clone();
    for key in input.as_object().unwrap().keys() {
        let mut bad = input.clone();
        bad.as_object_mut().unwrap().remove(key);
        assert!(serde_json::from_value::<Input>(bad).is_err(), "{key}");
    }
    let mut steered = input.clone();
    steered["expected_output"] = json!({"condition":31});
    assert!(serde_json::from_value::<Input>(steered).is_err());
    for (key, value) in [("player", 0), ("player", 3921), ("club", 0), ("club", 177)] {
        let mut bad = input.clone();
        bad[key] = json!(value);
        assert!(condition_refresh::replay(&serde_json::from_value(bad).unwrap()).is_err());
    }
    let mut bad = input;
    bad["caps"] = json!([31, 25]);
    assert!(condition_refresh::replay(&serde_json::from_value(bad).unwrap()).is_err());
}

#[test]
fn consecutive_calls_retain_the_advanced_seed_and_updated_fields() {
    let record: Value = serde_json::from_slice(RUN).unwrap();
    let original = record["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["name"] == "seed_max")
        .unwrap();
    let input: Input = serde_json::from_value(original["input"].clone()).unwrap();
    let mut session = condition_refresh::Session::new(&input).unwrap();
    let first = session.refresh(input.flag).unwrap();
    assert_eq!(
        serde_json::to_value(&first.output).unwrap(),
        original["output"]
    );
    let second = session.refresh(0).unwrap();
    assert_eq!(second.draws[0].seed_before, first.output.seed);
    assert_ne!(second.draws[0].seed_before, input.seed);
    let write = second
        .writes
        .iter()
        .find(|w| w.field == "base_condition")
        .unwrap();
    assert_eq!(write.before, first.output.base_condition as u8 as u16);
    assert_ne!(write.before, input.base_condition as u8 as u16);
    assert_eq!(session.snapshot(), second.output);
}
