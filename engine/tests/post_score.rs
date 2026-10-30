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

fn read_json(p: PathBuf) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn prefixes() -> std::collections::BTreeMap<(String, u64), Value> {
    let mut all = std::collections::BTreeMap::new();
    for e in fs::read_dir(fixtures().join("../step_tail")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|e| e == "jsonl") {
            for s in fs::read_to_string(p).unwrap().lines() {
                let c: Value = serde_json::from_str(s).unwrap();
                all.insert(
                    (
                        c["week"].as_str().unwrap().into(),
                        c["step"].as_u64().unwrap(),
                    ),
                    c,
                );
            }
        }
    }
    all
}
fn corpus() -> Vec<Value> {
    use sha2::{Digest, Sha256};
    use std::collections::BTreeSet;
    let dir = fixtures();
    let index = read_json(dir.join("index.json"));
    let pin = read_json(dir.join("provenance.json"));
    assert_eq!(
        pin["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    assert_eq!(pin["commit"].as_str().unwrap().len(), 40);
    assert_eq!(
        pin["index_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(dir.join("index.json")).unwrap())
        )
    );
    assert_eq!(index["schema"], "post-score-corpus/1");
    for (k, v) in [
        ("weeks", 6),
        ("calls", 17),
        ("draws", 12),
        ("steps", 558),
        ("late_calls", 1317),
        ("traces", 15),
    ] {
        assert_eq!(index[k], v);
    }
    assert_eq!(
        index["t16_index_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(dir.join("../step_tail/index.json")).unwrap())
        )
    );
    let previous = prefixes();
    let mut all = vec![];
    let mut files = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for r in index["records"].as_array().unwrap() {
        let name = r["cases"].as_str().unwrap();
        assert_eq!(PathBuf::from(name).file_name().unwrap(), name);
        assert!(files.insert(name));
        let bytes = fs::read(dir.join(name)).unwrap();
        assert_eq!(r["cases_sha256"], format!("{:x}", Sha256::digest(&bytes)));
        let rows: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(rows.len() as u64, r["calls"].as_u64().unwrap());
        for c in rows {
            assert_eq!(c["schema"], "post-score-case/1");
            assert_eq!(c["week"], r["week"]);
            let key = (
                c["week"].as_str().unwrap().into(),
                c["step"].as_u64().unwrap(),
            );
            assert!(seen.insert(key.clone()));
            assert_eq!(c["prefix"], previous[&key]);
            all.push(c);
        }
    }
    let wanted: BTreeSet<_> = previous
        .iter()
        .filter(|(_, c)| {
            c["prefix"]["pending_calls"]
                .as_array()
                .unwrap()
                .iter()
                .any(|p| p["routine"] == "0AB9:52B6")
        })
        .map(|(k, _)| k.clone())
        .collect();
    assert_eq!(seen, wanted);
    assert_eq!(all.len(), 17);
    assert_eq!(files.len(), 6);
    assert_eq!(index["coverage"].as_array().unwrap().len(), 15);
    let mut refs = BTreeSet::new();
    for r in index["coverage"].as_array().unwrap() {
        assert!(refs.insert(r["trace"].as_str().unwrap()));
        assert!(files.contains(r["cases"].as_str().unwrap()));
    }
    all
}
fn expected(c: &Value) -> Value {
    let draws:Vec<Value>=c["draws"].as_array().unwrap().iter().map(|d| {
        let sites:Vec<_>=d["callers"].as_array().unwrap().iter().filter(|s|matches!(s.as_str(),Some("0AB9:A305"|"0AB9:A38C"|"0AB9:A3A4"|"0AB9:A417"|"0AB9:A4A5"))).collect();
        assert_eq!(sites.len(),1);serde_json::json!({"site":sites[0],"n":d["n"],"value":d["value"],"seed_before":d["seed_before"],"seed_after":d["seed_after"]})
    }).collect();
    let mut obj = serde_json::Map::new();
    for k in ["output", "paths", "setters", "normalizers", "getters"] {
        obj.insert(k.into(), c[k].clone());
    }
    obj.insert("draws".into(), Value::Array(draws));
    Value::Object(obj)
}
fn compare(a: &Value, b: &Value, path: &str) -> Result<(), String> {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            if a.keys().collect::<Vec<_>>() != b.keys().collect::<Vec<_>>() {
                return Err(format!("{path}: keys differ"));
            }
            for (k, v) in a {
                compare(v, &b[k], &format!("{path}.{k}"))?;
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Err(format!("{path}: lengths {} vs {}", a.len(), b.len()));
            }
            for (i, (x, y)) in a.iter().zip(b).enumerate() {
                compare(x, y, &format!("{path}[{i}]"))?;
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
fn input(c: &Value) -> Input {
    Input {
        state: serde_json::from_value(c["input"].clone()).unwrap(),
        argument: c["argument"].as_u64().unwrap().try_into().unwrap(),
    }
}
#[test]
fn all_original_strategies_and_ordered_helpers_replay() {
    let mut draws = 0;
    for c in corpus() {
        let got = post_score::strategy(&input(&c)).unwrap();
        draws += got.draws.len();
        compare(
            &serde_json::to_value(got).unwrap(),
            &expected(&c),
            &format!("{} step {}", c["week"], c["step"]),
        )
        .unwrap();
    }
    assert_eq!(draws, 12);
}
#[test]
fn actual_prefix_shared_footprints_feed_all_original_strategies() {
    for c in corpus() {
        let placement = serde_json::from_value(c["prefix"]["placement"]["input"].clone()).unwrap();
        let prefix = serde_json::from_value(c["prefix"]["prefix"]["input"].clone()).unwrap();
        let got = post_score::composed(&placement, &prefix, &input(&c)).unwrap();
        compare(
            &serde_json::to_value(got.strategy).unwrap(),
            &expected(&c),
            "composed strategy",
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(got.predecessor.prefix.output).unwrap(),
            c["prefix"]["prefix"]["output"]
        );
    }
}
#[test]
fn expected_results_do_not_steer_strategy_or_helpers() {
    let c = first_case();
    let input = input(&c);
    let got = post_score::strategy(&input).unwrap();
    for damage in 0..3 {
        let mut e = expected(&c);
        match damage {
            0 => e["output"]["seed"] = serde_json::json!(65535),
            1 => e["getters"][0]["value"] = serde_json::json!(65535),
            _ => e["normalizers"][0]["after"] = serde_json::json!(255),
        };
        assert!(compare(&serde_json::to_value(&got).unwrap(), &e, "modified").is_err());
        assert_eq!(post_score::strategy(&input).unwrap(), got);
    }
}
#[test]
fn invalid_prefix_footprints_and_keeper_preludes_diagnose() {
    let c = first_case();
    let placement = serde_json::from_value(c["prefix"]["placement"]["input"].clone()).unwrap();
    let prefix: onenil_engine::step_tail::PrefixInput =
        serde_json::from_value(c["prefix"]["prefix"]["input"].clone()).unwrap();
    for damage in 0..4 {
        let mut i = input(&c);
        match damage {
            0 => i.state.seed ^= 1,
            1 => i.state.scores[0] ^= 1,
            2 => i.state.clubs[0].id ^= 1,
            _ => i.state.clubs[0].roster.swap(0, 1),
        };
        assert!(post_score::composed(&placement, &prefix, &i).is_err());
    }
    for actor in [0, 1, 12, 23] {
        let mut p = prefix.clone();
        p.placement.actor = actor;
        assert!(post_score::composed(&placement, &p, &input(&c))
            .unwrap_err()
            .contains("prelude"));
    }
}
