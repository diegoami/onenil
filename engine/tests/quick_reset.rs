use onenil_engine::quick_reset::{self, State};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/quick_reset")
}
fn first_case() -> Value {
    serde_json::from_str(
        fs::read_to_string(fixtures().join("T21-r0001.jsonl"))
            .unwrap()
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap()
}
#[test]
fn first_actual_original_quick_reset_case_replays() {
    let c = first_case();
    let input: State = serde_json::from_value(c["input"].clone()).unwrap();
    let got = quick_reset::reset(&input).unwrap();
    assert_eq!(serde_json::to_value(got.output).unwrap(), c["output"]);
}
fn read_json(p: PathBuf) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn prior_cases() -> std::collections::BTreeMap<(String, u64), Value> {
    let mut all = std::collections::BTreeMap::new();
    for p in fs::read_dir(fixtures().join("../post_score")).unwrap() {
        let p = p.unwrap().path();
        if p.extension().is_some_and(|s| s == "jsonl") {
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
    assert_eq!(index["schema"], "quick-reset-corpus/1");
    for (k, v) in [
        ("weeks", 6),
        ("calls", 17),
        ("draws", 12),
        ("copies", 374),
        ("motions", 374),
        ("helpers", 747),
    ] {
        assert_eq!(index[k], v);
    }
    assert_eq!(
        index["t20_index_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(dir.join("../post_score/index.json")).unwrap())
        )
    );
    let old = prior_cases();
    let mut all = vec![];
    let mut seen = BTreeSet::new();
    let mut files = BTreeSet::new();
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
            assert_eq!(c["schema"], "quick-reset-case/1");
            assert_eq!(c["week"], r["week"]);
            let key = (
                c["week"].as_str().unwrap().into(),
                c["step"].as_u64().unwrap(),
            );
            assert!(seen.insert(key.clone()));
            assert_eq!(c["post_score"], old[&key]);
            all.push(c);
        }
    }
    assert_eq!(seen, old.keys().cloned().collect());
    assert_eq!(all.len(), 17);
    assert_eq!(files.len(), 6);
    all
}
fn expected(c: &Value) -> Value {
    let mut obj = serde_json::Map::new();
    for k in [
        "output",
        "copies",
        "motions",
        "keeper",
        "strategy_input",
        "strategy_output",
        "helpers",
    ] {
        obj.insert(k.into(), c[k].clone());
    }
    let begin = c["draw_start"].as_u64().unwrap();
    if !obj["keeper"].is_null() {
        for k in ["draw_start", "draw_end"] {
            obj["keeper"][k] = serde_json::json!(obj["keeper"][k].as_u64().unwrap() - begin);
        }
    }
    let old = &c["post_score"];
    let mut strategy = serde_json::Map::new();
    for k in ["output", "paths", "setters", "normalizers", "getters"] {
        strategy.insert(k.into(), old[k].clone());
    }
    fn normalized(rows: &Value) -> Value {
        Value::Array(rows.as_array().unwrap().iter().map(|d|{
        let sites:Vec<_>=d["callers"].as_array().unwrap().iter().filter(|s|matches!(s.as_str(),Some("0AB9:4FDF"|"0AB9:A305"|"0AB9:A38C"|"0AB9:A3A4"|"0AB9:A417"|"0AB9:A4A5"))).collect();
        assert_eq!(sites.len(),1);serde_json::json!({"site":sites[0],"n":d["n"],"value":d["value"],"seed_before":d["seed_before"],"seed_after":d["seed_after"]})
    }).collect())
    }
    strategy.insert("draws".into(), normalized(&old["draws"]));
    obj.insert("strategy".into(), Value::Object(strategy));
    obj.insert("draws".into(), normalized(&c["draws"]));
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
#[test]
fn all_original_resets_and_ordered_copies_motions_helpers_strategy_replay() {
    let (mut copies, mut motions, mut helpers, mut draws) = (0, 0, 0, 0);
    for c in corpus() {
        let s = serde_json::from_value(c["input"].clone()).unwrap();
        let out = quick_reset::reset(&s).unwrap();
        compare(
            &serde_json::to_value(&out).unwrap(),
            &expected(&c),
            &format!("{} step {}", c["week"], c["step"]),
        )
        .unwrap();
        copies += out.copies.len();
        motions += out.motions.len();
        helpers += out.helpers.len();
        draws += out.draws.len();
    }
    assert_eq!((copies, motions, helpers, draws), (374, 374, 747, 12));
}
#[test]
fn actual_prefix_image_feeds_all_resets_and_strategy_boundaries() {
    for c in corpus() {
        let p = &c["post_score"]["prefix"];
        let placement = serde_json::from_value(p["placement"]["input"].clone()).unwrap();
        let prefix = serde_json::from_value(p["prefix"]["input"].clone()).unwrap();
        let input = serde_json::from_value(c["input"].clone()).unwrap();
        let got = quick_reset::composed(&placement, &prefix, &input).unwrap();
        compare(
            &serde_json::to_value(got.reset).unwrap(),
            &expected(&c),
            "composed reset",
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(got.predecessor.prefix.output).unwrap(),
            p["prefix"]["output"]
        );
    }
}
#[test]
fn expected_intermediate_and_helper_results_cannot_steer_reset() {
    let c = first_case();
    let input = serde_json::from_value(c["input"].clone()).unwrap();
    let got = quick_reset::reset(&input).unwrap();
    for damage in 0..3 {
        let mut bad = expected(&c);
        match damage {
            0 => bad["output"]["actor"] = serde_json::json!(1),
            1 => bad["strategy_input"]["strategy"]["seed"] = serde_json::json!(65535),
            _ => bad["helpers"][0]["result"] = serde_json::json!(255),
        }
        assert!(compare(&serde_json::to_value(&got).unwrap(), &bad, "changed").is_err());
        assert_eq!(quick_reset::reset(&input).unwrap(), got);
    }
}
#[test]
fn composed_reset_rejects_inconsistent_shared_fields() {
    let c = first_case();
    let p = &c["post_score"]["prefix"];
    let placement = serde_json::from_value(p["placement"]["input"].clone()).unwrap();
    let prefix = serde_json::from_value(p["prefix"]["input"].clone()).unwrap();
    let input: State = serde_json::from_value(c["input"].clone()).unwrap();
    for damage in 0..7 {
        let mut bad = input.clone();
        match damage {
            0 => bad.strategy.seed ^= 1,
            1 => bad.strategy.scores[0] ^= 1,
            2 => bad.actor ^= 1,
            3 => bad.remaining ^= 1,
            4 => bad.scenes[0].word0 ^= 1,
            5 => {
                let id = bad.scenes[0].player;
                bad.players.iter_mut().find(|p| p.id == id).unwrap().byte8 ^= 1;
            }
            _ => {
                let id = bad.scenes[0].player;
                bad.players.iter_mut().find(|p| p.id == id).unwrap().flags11 ^= 1;
            }
        }
        assert!(
            quick_reset::composed(&placement, &prefix, &bad).is_err(),
            "damage {damage}"
        );
    }
}
