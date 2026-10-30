use onenil_engine::extra_event::{self, Input};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/extra_event")
}
fn first_case() -> (Value, Input) {
    let text = fs::read_to_string(fixtures().join("T17-r0005.jsonl")).unwrap();
    let case: Value = serde_json::from_str(text.lines().next().unwrap()).unwrap();
    let text = fs::read_to_string(fixtures().join("inputs/T18-r0001.jsonl")).unwrap();
    let cell: Value = serde_json::from_str(text.lines().next().unwrap()).unwrap();
    assert_eq!(cell["step"], case["step"]);
    assert_eq!(cell["seed"], case["input"]["seed"]);
    let input = Input {
        state: serde_json::from_value(case["input"].clone()).unwrap(),
        opponent_d0b: cell["value"].as_u64().unwrap() as u8,
    };
    (case, input)
}
#[test]
fn first_actual_original_dbc6_case_replays() {
    let (case, input) = first_case();
    let got = extra_event::decision(&input).unwrap();
    let expected = if case["dispatch"].is_null() {
        &case["output"]
    } else {
        &case["dispatch"]["input"]
    };
    assert_eq!(serde_json::to_value(got.output).unwrap(), *expected);
}

fn digest(path: PathBuf) -> Vec<u8> {
    fs::read(path).unwrap()
}
fn index(dir: &std::path::Path, schema: &str) -> Value {
    use sha2::{Digest, Sha256};
    let bytes = fs::read(dir.join("index.json")).unwrap();
    let pin: Value =
        serde_json::from_slice(&fs::read(dir.join("provenance.json")).unwrap()).unwrap();
    assert_eq!(
        pin["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    assert_eq!(pin["commit"].as_str().unwrap().len(), 40);
    assert_eq!(pin["index_sha256"], format!("{:x}", Sha256::digest(&bytes)));
    let index: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(index["schema"], schema);
    index
}
fn corpus() -> Vec<(Value, Input)> {
    use sha2::{Digest, Sha256};
    use std::collections::{BTreeMap, BTreeSet};
    let root = fixtures();
    let original = index(&root, "extra-event-corpus/1");
    for (key, n) in [
        ("calls", 184),
        ("decision_draws", 499),
        ("helpers", 909),
        ("dispatches", 22),
        ("weeks", 6),
        ("traces", 15),
    ] {
        assert_eq!(original[key], n);
    }
    assert_eq!(
        original["t16_index_sha256"],
        format!(
            "{:x}",
            Sha256::digest(digest(root.join("../step_tail/index.json")))
        )
    );
    let supplement = index(&root.join("inputs"), "extra-event-inputs/1");
    assert_eq!(supplement["cells"], 184);
    assert_eq!(supplement["weeks"], 2);
    assert_eq!(
        supplement["t17_index_sha256"],
        format!("{:x}", Sha256::digest(digest(root.join("index.json"))))
    );
    let mut cells = BTreeMap::new();
    for record in supplement["records"].as_array().unwrap() {
        let name = record["cases"].as_str().unwrap();
        assert_eq!(PathBuf::from(name).file_name().unwrap(), name);
        let bytes = digest(root.join("inputs").join(name));
        assert_eq!(
            record["cases_sha256"],
            format!("{:x}", Sha256::digest(&bytes))
        );
        let rows: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(rows.len() as u64, record["cells"].as_u64().unwrap());
        for row in rows {
            assert_eq!(row["schema"], "extra-event-input/1");
            assert_eq!(row["offset"], 0xD0B);
            assert_eq!(row["source"]["t17_record"], record["t17_record"]);
            assert_eq!(
                row["source"]["t17_record_sha256"],
                record["t17_record_sha256"]
            );
            assert!(cells
                .insert(
                    (
                        record["week"].as_str().unwrap().to_owned(),
                        row["step"].as_u64().unwrap()
                    ),
                    row
                )
                .is_none());
        }
    }
    let mut result = vec![];
    let mut files = BTreeSet::new();
    let mut covered = BTreeSet::new();
    for record in original["records"].as_array().unwrap() {
        let name = record["cases"].as_str().unwrap();
        assert!(files.insert(name));
        assert_eq!(PathBuf::from(name).file_name().unwrap(), name);
        let bytes = digest(root.join(name));
        assert_eq!(
            record["cases_sha256"],
            format!("{:x}", Sha256::digest(&bytes))
        );
        let rows: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(rows.len() as u64, record["calls"].as_u64().unwrap());
        for c in rows {
            assert_eq!(c["schema"], "extra-event-case/1");
            assert_eq!(c["week"], record["week"]);
            let key = (
                c["week"].as_str().unwrap().to_owned(),
                c["step"].as_u64().unwrap(),
            );
            let cell = cells.remove(&key).unwrap();
            assert_eq!(cell["seed"], c["input"]["seed"]);
            assert_eq!(cell["draw_start"], c["draw_start"]);
            for (k, v) in [
                ("t17_record", &record["record"]),
                ("t17_record_sha256", &record["record_sha256"]),
                ("t17_cases", &record["cases"]),
                ("t17_cases_sha256", &record["cases_sha256"]),
            ] {
                assert_eq!(&cell["source"][k], v);
            }
            let input = Input {
                state: serde_json::from_value(c["input"].clone()).unwrap(),
                opponent_d0b: cell["value"].as_u64().unwrap().try_into().unwrap(),
            };
            assert!(covered.insert(key));
            result.push((c, input));
        }
    }
    assert!(cells.is_empty());
    assert_eq!(result.len(), 184);
    assert_eq!(files.len(), 6);
    let coverage = original["coverage"].as_array().unwrap();
    assert_eq!(coverage.len(), 15);
    let mut seen = BTreeSet::new();
    for r in coverage {
        assert!(seen.insert(r["trace"].as_str().unwrap()));
        assert!(files.contains(r["cases"].as_str().unwrap()));
    }
    result
}
fn assert_case(case: &Value, got: &extra_event::Outcome) {
    let expected = if case["dispatch"].is_null() {
        &case["output"]
    } else {
        &case["dispatch"]["input"]
    };
    assert_eq!(
        &serde_json::to_value(&got.output).unwrap(),
        expected,
        "{} step {} output",
        case["week"],
        case["step"]
    );
    let start = case["draw_start"].as_u64().unwrap();
    let split = if case["dispatch"].is_null() {
        case["draw_end"].as_u64().unwrap()
    } else {
        case["dispatch"]["draw_start"].as_u64().unwrap()
    };
    let draws = case["draws"].as_array().unwrap();
    assert_eq!(got.draws.len() as u64, split - start);
    for (actual, old) in got.draws.iter().zip(draws) {
        let site = old["callers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| {
                matches!(
                    s.as_str(),
                    Some("0AB9:DBD1" | "0AB9:DC38" | "0AB9:DC98" | "0AB9:DCBF" | "0AB9:DCD4")
                )
            })
            .unwrap();
        assert_eq!(actual.site, site.as_str().unwrap());
        for key in ["n", "value", "seed_before", "seed_after"] {
            assert_eq!(serde_json::to_value(actual).unwrap()[key], old[key]);
        }
    }
    assert_eq!(got.helpers.len(), case["helpers"].as_array().unwrap().len());
    for (actual, old) in got.helpers.iter().zip(case["helpers"].as_array().unwrap()) {
        let mut inp = old["input"].clone();
        inp.as_object_mut().unwrap().remove("offset");
        inp.as_object_mut().unwrap().remove("segment");
        assert_eq!(actual.caller, old["caller"]);
        assert_eq!(actual.kind, old["kind"]);
        assert_eq!(actual.input, inp);
        assert_eq!(actual.result as u64, old["result"].as_u64().unwrap());
        assert_eq!(
            actual.draw_position as u64,
            old["draw_start"].as_u64().unwrap() - start
        );
    }
    assert_eq!(got.dispatch.is_some(), !case["dispatch"].is_null());
    if let Some(request) = &got.dispatch {
        assert_eq!(request.routine, "0AB9:69B8");
        assert_eq!(
            serde_json::to_value(&request.input).unwrap(),
            case["dispatch"]["input"]
        );
    }
}
#[test]
fn all_original_decisions_draws_helpers_and_dispatches_replay() {
    let (mut draws, mut helpers, mut requests) = (0, 0, 0);
    for (case, input) in corpus() {
        let got = extra_event::decision(&input).unwrap();
        assert_case(&case, &got);
        draws += got.draws.len();
        helpers += got.helpers.len();
        requests += usize::from(got.dispatch.is_some());
    }
    assert_eq!((draws, helpers, requests), (499, 909, 22));
}
#[test]
fn all_909_original_helpers_replay_independently_from_raw_inputs() {
    let mut count = 0;
    for (case, input) in corpus() {
        for h in case["helpers"].as_array().unwrap() {
            let kind = h["kind"].as_str().unwrap();
            let argument = h["input"][if kind == "threshold" {
                "player"
            } else {
                "scene"
            }]
            .as_u64()
            .unwrap() as u16;
            let mode = h["input"]["mode"].as_u64().unwrap_or(0) as u16;
            let stack = h["stack_word_before"].as_u64().map(|n| n as u16);
            assert_eq!(
                extra_event::helper(&input, kind, argument, mode, stack).unwrap() as u64,
                h["result"].as_u64().unwrap()
            );
            count += 1;
        }
    }
    assert_eq!(count, 909);
}
fn tails() -> std::collections::BTreeMap<(String, u64), Value> {
    let mut out = std::collections::BTreeMap::new();
    for p in fs::read_dir(fixtures().join("../step_tail")).unwrap() {
        let p = p.unwrap().path();
        if p.extension().is_some_and(|e| e == "jsonl") {
            for line in fs::read_to_string(p).unwrap().lines() {
                let v: Value = serde_json::from_str(line).unwrap();
                out.insert(
                    (
                        v["week"].as_str().unwrap().into(),
                        v["step"].as_u64().unwrap(),
                    ),
                    v,
                );
            }
        }
    }
    out
}
#[test]
fn all_decisions_use_actual_computed_placement_and_prefix_images() {
    let tails = tails();
    for (case, input) in corpus() {
        let old = &tails[&(
            case["week"].as_str().unwrap().into(),
            case["step"].as_u64().unwrap(),
        )];
        let placement = serde_json::from_value(old["placement"]["input"].clone()).unwrap();
        let prefix = serde_json::from_value(old["prefix"]["input"].clone()).unwrap();
        let got = extra_event::composed(&placement, &prefix, &input).unwrap();
        assert_case(&case, &got.decision);
        assert_eq!(
            serde_json::to_value(got.prefix.prefix.output).unwrap(),
            old["prefix"]["output"]
        );
        assert_eq!(
            serde_json::to_value(got.prefix.placement.output).unwrap(),
            old["placement"]["output"]
        );
    }
}
#[test]
fn conflicting_actual_handoffs_are_rejected_instead_of_reset() {
    let (case, input) = first_case();
    let tails = tails();
    let old = &tails[&(
        case["week"].as_str().unwrap().into(),
        case["step"].as_u64().unwrap(),
    )];
    let placement = serde_json::from_value(old["placement"]["input"].clone()).unwrap();
    let prefix = serde_json::from_value(old["prefix"]["input"].clone()).unwrap();
    for field in 0..8 {
        let mut bad = input.clone();
        match field {
            0 => bad.state.seed ^= 1,
            1 => bad.state.actor ^= 1,
            2 => bad.state.d6dd ^= 1,
            3 => bad.state.scenes[0].word0 ^= 1,
            4 => bad.state.players[0].p15_8 ^= 1,
            5 => bad.state.players[0].p15_11 ^= 1,
            6 => bad.state.clubs[0].id ^= 1,
            _ => bad.state.scenes.swap(0, 1),
        }
        assert!(
            extra_event::composed(&placement, &prefix, &bad).is_err(),
            "field {field}"
        );
    }
}

#[test]
fn changed_expected_output_helper_or_draw_cannot_steer_the_decision() {
    let (case, input) = first_case();
    let got = extra_event::decision(&input).unwrap();
    for damage in 0..3 {
        let mut bad = case.clone();
        match damage {
            0 => {
                bad["output"]["actor"] = serde_json::json!(255);
            }
            1 => {
                bad["helpers"][0]["result"] = serde_json::json!(255);
            }
            _ => {
                bad["draws"][0]["value"] = serde_json::json!(65535);
            }
        }
        assert!(std::panic::catch_unwind(|| assert_case(&bad, &got)).is_err());
        assert_eq!(extra_event::decision(&input).unwrap(), got);
    }
}
