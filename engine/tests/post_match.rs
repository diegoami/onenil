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
        for (index, name) in [
            "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "es", "cs", "ss", "ds",
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(json!(actual.registers[index]), c["returned"][*name]);
        }
    }
}

#[test]
fn every_initial_operand_or_write_context_is_required_and_refuses_atomically() {
    let record: Value = serde_json::from_slice(RUNS[0]).unwrap();
    let full = input(&record);
    for index in 0..full.initial.len() {
        let mut bad = full.clone();
        let removed = bad.initial.remove(index);
        if let Ok(mut session) = post_match::Session::new(&bad) {
            let before = session.snapshot().unwrap();
            assert!(
                session.advance().is_err(),
                "missing {} was accepted",
                removed.field
            );
            assert_eq!(
                session.snapshot().unwrap(),
                before,
                "{} changed state on refusal",
                removed.field
            );
        }
    }
}
fn replace(input: &mut Input, name: &str, value: u16) {
    input
        .initial
        .iter_mut()
        .find(|f| f.field == name)
        .unwrap()
        .value = value;
}
#[test]
fn unsupported_branches_and_injury_refuse_after_no_committed_changes() {
    let record: Value = serde_json::from_slice(RUNS[0]).unwrap();
    for (field, value) in [
        ("DS:D075", 1),
        ("DS:D6DD", 0),
        ("frame:3ED5", 0),
        ("fixture[134]+15", 6),
        ("DS:D086", 257),
        ("player15[2661]+8", 0),
        ("DS:0D72", 31),
        ("club[134]+44", 1001),
    ] {
        let mut bad = input(&record);
        replace(&mut bad, field, value);
        let mut session = post_match::Session::new(&bad).unwrap();
        let before = session.snapshot().unwrap();
        assert!(session.advance().is_err(), "{field}={value} was accepted");
        assert_eq!(
            session.snapshot().unwrap(),
            before,
            "{field} committed a partial result"
        );
    }
}
#[test]
fn malformed_input_geometry_and_unclassified_data_refuse() {
    let record: Value = serde_json::from_slice(RUNS[0]).unwrap();
    for damage in 0..12 {
        let mut bad = input(&record);
        match damage {
            0 => bad.registers[4] = 0,
            1 => bad.registers[5] = 65535,
            2 => bad.registers[9] = 0,
            3 => bad.regions[0].base = bad.regions[1].base,
            4 => bad.regions[0].stride += 1,
            5 => {
                bad.regions.pop();
            }
            6 => bad.initial[0].address += 1,
            7 => bad.initial[0].field = "DS:FFFF".into(),
            8 => {
                bad.initial.push(bad.initial[0].clone());
            }
            9 => replace(&mut bad, "DS:DE84", 0),
            10 => bad.initial[0].size = 4,
            11 => bad.initial[0].field = "player8[1]+18446744073709551615".into(),
            _ => unreachable!(),
        }
        assert!(
            post_match::Session::new(&bad).is_err(),
            "damage {damage} was accepted"
        );
    }
}
#[test]
fn input_schema_has_no_expected_state_or_rng_ledger_and_completed_frame_cannot_repeat() {
    let record: Value = serde_json::from_slice(RUNS[0]).unwrap();
    let input = input(&record);
    for name in ["expected", "final", "draws", "exit_seed"] {
        let mut value = serde_json::to_value(&input).unwrap();
        value[name] = json!(0);
        assert!(serde_json::from_value::<Input>(value).is_err());
    }
    let mut session = post_match::Session::new(&input).unwrap();
    session.advance().unwrap();
    let after = session.snapshot().unwrap();
    assert!(session.advance().is_err());
    assert_eq!(session.snapshot().unwrap(), after);
}
