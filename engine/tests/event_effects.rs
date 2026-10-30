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

fn read_json(path: PathBuf) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn old_cases(folder: &str, prefix: &str) -> std::collections::BTreeMap<(String, u64), Value> {
    let mut out = std::collections::BTreeMap::new();
    for entry in fs::read_dir(fixtures().join("..").join(folder)).unwrap() {
        let path = entry.unwrap().path();
        if path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(prefix)
            && path.extension().is_some_and(|e| e == "jsonl")
        {
            for line in fs::read_to_string(path).unwrap().lines() {
                let row: Value = serde_json::from_str(line).unwrap();
                out.insert(
                    (
                        row["week"].as_str().unwrap().into(),
                        row["step"].as_u64().unwrap(),
                    ),
                    row,
                );
            }
        }
    }
    out
}
fn corpus() -> Vec<Value> {
    use sha2::{Digest, Sha256};
    use std::collections::{BTreeMap, BTreeSet};
    let root = fixtures();
    let index = read_json(root.join("index.json"));
    let pin = read_json(root.join("provenance.json"));
    assert_eq!(
        pin["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    assert_eq!(pin["commit"].as_str().unwrap().len(), 40);
    assert_eq!(
        pin["index_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(root.join("index.json")).unwrap())
        )
    );
    assert_eq!(index["schema"], "event-effects-corpus/1");
    for (k, n) in [
        ("weeks", 2),
        ("calls", 22),
        ("draws", 17),
        ("selectors", 44),
        ("actions", 6),
        ("helpers", 23),
        ("events", 6),
    ] {
        assert_eq!(index[k], n);
    }
    for (k, p) in [
        ("t17_index_sha256", "../extra_event/index.json"),
        ("t18_index_sha256", "../extra_event/inputs/index.json"),
    ] {
        assert_eq!(
            index[k],
            format!("{:x}", Sha256::digest(fs::read(root.join(p)).unwrap()))
        );
    }
    let previous = old_cases("extra_event", "T17-r");
    let old_index = read_json(root.join("../extra_event/index.json"));
    let mut lookup = BTreeMap::new();
    let lookup_index = read_json(root.join("../extra_event/inputs/index.json"));
    for r in lookup_index["records"].as_array().unwrap() {
        let bytes = fs::read(
            root.join("../extra_event/inputs")
                .join(r["cases"].as_str().unwrap()),
        )
        .unwrap();
        assert_eq!(r["cases_sha256"], format!("{:x}", Sha256::digest(&bytes)));
        for s in std::str::from_utf8(&bytes).unwrap().lines() {
            let c: Value = serde_json::from_str(s).unwrap();
            lookup.insert(
                (
                    r["week"].as_str().unwrap().to_owned(),
                    c["step"].as_u64().unwrap(),
                ),
                c,
            );
        }
    }
    let mut all = vec![];
    let mut seen = BTreeSet::new();
    for r in index["records"].as_array().unwrap() {
        let name = r["cases"].as_str().unwrap();
        assert_eq!(PathBuf::from(name).file_name().unwrap(), name);
        let bytes = fs::read(root.join(name)).unwrap();
        assert_eq!(r["cases_sha256"], format!("{:x}", Sha256::digest(&bytes)));
        let rows: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(rows.len() as u64, r["calls"].as_u64().unwrap());
        let meta = old_index["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["week"] == r["week"])
            .unwrap();
        for c in rows {
            assert_eq!(c["schema"], "event-effects-case/1");
            assert_eq!(c["week"], r["week"]);
            let key = (
                c["week"].as_str().unwrap().to_owned(),
                c["step"].as_u64().unwrap(),
            );
            assert!(seen.insert(key.clone()));
            assert_eq!(c["decision"], previous[&key]);
            assert_eq!(c["lookup"], lookup[&key]);
            assert_eq!(c["input"]["handoff"], previous[&key]["dispatch"]["input"]);
            assert_eq!(c["output"]["handoff"], previous[&key]["output"]);
            assert_eq!(c["draw_start"], previous[&key]["dispatch"]["draw_start"]);
            assert_eq!(c["draw_end"], previous[&key]["draw_end"]);
            for (a, b) in [
                ("t17_record", "record"),
                ("t17_record_sha256", "record_sha256"),
                ("t17_cases", "cases"),
                ("t17_cases_sha256", "cases_sha256"),
            ] {
                assert_eq!(c["source"][a], meta[b]);
            }
            all.push(c);
        }
    }
    assert_eq!(
        seen,
        previous
            .iter()
            .filter(|(_, v)| !v["dispatch"].is_null())
            .map(|(k, _)| k.clone())
            .collect()
    );
    assert_eq!(all.len(), 22);
    all
}
fn expected(case: &Value) -> Value {
    let draws:Vec<Value>=case["draws"].as_array().unwrap().iter().map(|d| {
        let sites:Vec<_>=d["callers"].as_array().unwrap().iter().filter(|s|matches!(s.as_str(),Some("0AB9:6A15"|"1998:13EE"))).collect();
        assert_eq!(sites.len(),1);serde_json::json!({"site":sites[0],"n":d["n"],"seed_before":d["seed_before"],"seed_after":d["seed_after"],"value":d["value"]})
    }).collect();
    let mut object = serde_json::Map::new();
    for k in [
        "output",
        "selectors",
        "actions",
        "helpers",
        "events",
        "external_calls",
    ] {
        object.insert(k.into(), case[k].clone());
    }
    object.insert("draws".into(), Value::Array(draws));
    Value::Object(object)
}
fn difference(a: &Value, b: &Value, path: &str) -> Result<(), String> {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            if a.keys().collect::<Vec<_>>() != b.keys().collect::<Vec<_>>() {
                return Err(format!("{path}: keys differ"));
            }
            for (k, v) in a {
                difference(v, &b[k], &format!("{path}.{k}"))?;
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Err(format!("{path}: lengths {} vs {}", a.len(), b.len()));
            }
            for (i, (x, y)) in a.iter().zip(b).enumerate() {
                difference(x, y, &format!("{path}[{i}]"))?;
            }
        }
        _ => {
            if a != b {
                return Err(format!("{path}: got {a}, expected {b}"));
            }
        }
    }
    Ok(())
}
#[test]
fn all_22_original_logical_effects_selectors_draws_and_ordered_helpers_replay() {
    let (mut draws, mut selectors, mut helpers, mut events) = (0, 0, 0, 0);
    for case in corpus() {
        let state = serde_json::from_value(case["input"].clone()).unwrap();
        let got = event_effects::handler(&state).unwrap();
        difference(
            &serde_json::to_value(&got).unwrap(),
            &expected(&case),
            &format!("{} step {}", case["week"], case["step"]),
        )
        .unwrap();
        draws += got.draws.len();
        selectors += got.selectors.len();
        helpers += got.helpers.len();
        events += got.events.len();
    }
    assert_eq!((draws, selectors, helpers, events), (17, 44, 23, 6));
}
#[test]
fn computed_placement_prefix_decision_requests_feed_all_effects() {
    let tails = old_cases("step_tail", "T16-r");
    for case in corpus() {
        let key = (
            case["week"].as_str().unwrap().to_owned(),
            case["step"].as_u64().unwrap(),
        );
        let old = &tails[&key];
        let placement = serde_json::from_value(old["placement"]["input"].clone()).unwrap();
        let prefix = serde_json::from_value(old["prefix"]["input"].clone()).unwrap();
        let decision = crate_decision(&case);
        let state = serde_json::from_value(case["input"].clone()).unwrap();
        let got = event_effects::composed(&placement, &prefix, &decision, &state).unwrap();
        difference(
            &serde_json::to_value(&got.effects).unwrap(),
            &expected(&case),
            "composed effects",
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(&got.predecessor.prefix.prefix.output).unwrap(),
            old["prefix"]["output"]
        );
        assert_eq!(
            serde_json::to_value(&got.predecessor.decision.dispatch.as_ref().unwrap().input)
                .unwrap(),
            case["input"]["handoff"]
        );
    }
}
fn crate_decision(case: &Value) -> onenil_engine::extra_event::Input {
    onenil_engine::extra_event::Input {
        state: serde_json::from_value(case["decision"]["input"].clone()).unwrap(),
        opponent_d0b: case["lookup"]["value"].as_u64().unwrap() as u8,
    }
}
#[test]
fn expected_after_states_helpers_and_selectors_cannot_steer_updates() {
    let case = first_case();
    let state = serde_json::from_value(case["input"].clone()).unwrap();
    let got = event_effects::handler(&state).unwrap();
    for damage in 0..3 {
        let mut bad = expected(&case);
        match damage {
            0 => bad["output"]["handoff"]["seed"] = serde_json::json!(65535),
            1 => bad["events"][0]["accepted"] = serde_json::json!(0),
            _ => bad["selectors"][0]["value"] = serde_json::json!(255),
        }
        assert!(difference(&serde_json::to_value(&got).unwrap(), &bad, "tampered").is_err());
        assert_eq!(event_effects::handler(&state).unwrap(), got);
    }
}
#[test]
fn composed_effects_reject_wrong_handoffs_and_rosters() {
    let case = first_case();
    let key = (
        case["week"].as_str().unwrap().to_owned(),
        case["step"].as_u64().unwrap(),
    );
    let tails = old_cases("step_tail", "T16-r");
    let old = &tails[&key];
    let placement = serde_json::from_value(old["placement"]["input"].clone()).unwrap();
    let prefix = serde_json::from_value(old["prefix"]["input"].clone()).unwrap();
    let decision = crate_decision(&case);
    let state: State = serde_json::from_value(case["input"].clone()).unwrap();
    for damage in 0..7 {
        let mut bad = state.clone();
        match damage {
            0 => bad.handoff.seed ^= 1,
            1 => bad.handoff.actor ^= 1,
            2 => bad.handoff.secondary ^= 1,
            3 => bad.handoff.scenes[0].word0 ^= 1,
            4 => bad.players[0].p15_6 ^= 1,
            5 => bad.rosters[0].players.swap(0, 1),
            _ => bad.handoff.clubs[0].id ^= 1,
        }
        assert!(
            event_effects::composed(&placement, &prefix, &decision, &bad).is_err(),
            "damage {damage}"
        );
    }
}
