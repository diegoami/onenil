use onenil_engine::step_tail::{self, Case};
use std::{fs, path::PathBuf};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/step_tail")
}
fn first_case() -> Case {
    let mut paths: Vec<_> = fs::read_dir(fixtures())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("T16-r")
                && p.extension().is_some_and(|e| e == "jsonl")
        })
        .collect();
    paths.sort();
    serde_json::from_str(
        fs::read_to_string(&paths[0])
            .unwrap()
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap()
}
#[test]
fn first_original_prefix_replay() {
    step_tail::replay_prefix(&first_case()).unwrap();
}
#[test]
fn first_original_clock_replay() {
    step_tail::replay_clock(&first_case()).unwrap();
}

fn corpus() -> Vec<Case> {
    use serde_json::Value;
    use sha2::{Digest, Sha256};
    use std::collections::HashSet;
    let data = fs::read(fixtures().join("index.json")).unwrap();
    let index: Value = serde_json::from_slice(&data).unwrap();
    let pin: Value =
        serde_json::from_slice(&fs::read(fixtures().join("provenance.json")).unwrap()).unwrap();
    assert_eq!(
        pin["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    assert_eq!(pin["commit"].as_str().unwrap().len(), 40);
    assert_eq!(pin["index_sha256"], format!("{:x}", Sha256::digest(&data)));
    assert_eq!(index["schema"], "step-tail-corpus/1");
    for (key, expected) in [
        ("calls", 558),
        ("clock_calls", 558),
        ("scene_visits", 12276),
        ("clock_events", 12),
        ("weeks", 6),
        ("traces", 15),
    ] {
        assert_eq!(index[key], expected);
    }
    let t13 = fs::read(fixtures().parent().unwrap().join("placement/index.json")).unwrap();
    assert_eq!(
        index["t13_index_sha256"],
        format!("{:x}", Sha256::digest(&t13))
    );
    let t13: Value = serde_json::from_slice(&t13).unwrap();
    assert_eq!(index["t12_index_sha256"], t13["t12_index_sha256"]);
    let mut all = vec![];
    let mut files = HashSet::new();
    for record in index["records"].as_array().unwrap() {
        let name = record["cases"].as_str().unwrap();
        assert!(files.insert(name));
        assert_eq!(PathBuf::from(name).file_name().unwrap(), name);
        let data = fs::read(fixtures().join(name)).unwrap();
        assert_eq!(
            record["cases_sha256"],
            format!("{:x}", Sha256::digest(&data))
        );
        let rows: Vec<Case> = std::str::from_utf8(&data)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(rows.len() as u64, record["calls"].as_u64().unwrap());
        for (i, case) in rows.iter().enumerate() {
            assert_eq!(case.schema, "step-tail-case/1");
            assert_eq!(case.step, i);
            assert_eq!(case.week, record["week"].as_str().unwrap());
        }
        all.extend(rows);
    }
    let coverage = index["coverage"].as_array().unwrap();
    assert_eq!(coverage.len(), 15);
    let mut seen = HashSet::new();
    for row in coverage {
        assert!(seen.insert(row["trace"].as_str().unwrap()));
        assert!(files.contains(row["cases"].as_str().unwrap()));
    }
    assert_eq!(all.len(), 558);
    all
}
#[test]
fn all_original_prefixes_and_sequential_scene_helpers_replay() {
    let mut visits = 0;
    for case in corpus() {
        step_tail::replay_prefix(&case).unwrap();
        visits += case.prefix.visits.len();
    }
    assert_eq!(visits, 12276);
}
#[test]
fn all_original_clocks_and_actor_zero_numeric_events_replay() {
    let mut events = 0;
    for case in corpus() {
        step_tail::replay_clock(&case).unwrap();
        assert_eq!(case.clock.draw_start, case.clock.draw_end);
        events += usize::from(case.clock.event.is_some());
    }
    assert_eq!(events, 12);
}
#[test]
fn actual_native_placement_outputs_feed_all_prefixes() {
    for case in corpus() {
        let got = step_tail::composed_prefix(&case.placement.input, &case.prefix.input).unwrap();
        assert_eq!(got.placement.output, case.placement.output);
        assert_eq!(got.placement.draws, case.placement.draws);
        assert_eq!(got.placement.callbacks, case.placement.callbacks);
        assert_eq!(got.prefix.output, case.prefix.output);
        assert_eq!(got.prefix.visits, case.prefix.visits);
        assert_eq!(got.prefix.pending_calls, case.prefix.pending_calls);
    }
}
#[test]
fn expected_outputs_cannot_steer_prefix_or_clock() {
    let mut case = first_case();
    case.prefix.output.history_byte ^= 1;
    assert!(step_tail::replay_prefix(&case).is_err());
    case = first_case();
    case.clock.output.clock.remaining ^= 1;
    assert!(step_tail::replay_clock(&case).is_err());
}
#[test]
fn composed_prefix_rejects_wrong_observed_handoffs_instead_of_resetting_them() {
    let case = first_case();
    for field in 0..5 {
        let mut input = case.prefix.input.clone();
        match field {
            0 => input.placement.seed ^= 1,
            1 => input.placement.generated[0] ^= 1,
            2 => input.placement.scores[0] ^= 1,
            3 => input.placement.goal_flag ^= 1,
            _ => input.placement.actor ^= 1,
        }
        assert!(step_tail::composed_prefix(&case.placement.input, &input).is_err());
    }
    let mut input = case.prefix.input.clone();
    input.players[0].byte8 ^= 1;
    assert!(step_tail::composed_prefix(&case.placement.input, &input).is_err());
    input = case.prefix.input.clone();
    input.players[0].byte11 ^= 1;
    assert!(step_tail::composed_prefix(&case.placement.input, &input).is_err());
}
