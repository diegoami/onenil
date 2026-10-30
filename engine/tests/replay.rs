use onenil_engine::{replay, Case};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn first_case() -> Case {
    let mut files: Vec<_> = fs::read_dir(fixtures())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "jsonl"))
        .collect();
    files.sort();
    let text = fs::read_to_string(&files[0]).unwrap();
    serde_json::from_str(text.lines().next().unwrap()).unwrap()
}

#[test]
fn first_original_case_replays() {
    replay(&first_case()).unwrap();
}

#[test]
fn all_original_cases_replay() {
    let raw_index = fs::read(fixtures().join("index.json")).unwrap();
    let index: Value = serde_json::from_slice(&raw_index).unwrap();
    let provenance: Value =
        serde_json::from_slice(&fs::read(fixtures().join("provenance.json")).unwrap()).unwrap();
    assert_eq!(
        provenance["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    let commit = provenance["commit"].as_str().unwrap();
    assert_eq!(commit.len(), 40);
    assert!(commit.bytes().all(|b| b.is_ascii_hexdigit()));
    assert_eq!(
        format!("{:x}", Sha256::digest(&raw_index)),
        provenance["index_sha256"].as_str().unwrap()
    );
    assert_eq!(index["schema"], "quick-contests/1");
    assert_eq!(index["weeks"], 6);
    assert_eq!(index["traces"], 15);
    assert_eq!(index["coverage"].as_array().unwrap().len(), 15);
    let mut count = 0;
    let records = index["records"].as_array().unwrap();
    assert_eq!(records.len(), 6);
    let mut case_paths = std::collections::HashSet::new();
    let mut weeks = std::collections::HashSet::new();
    for record in records {
        let name = record["cases"].as_str().unwrap();
        assert!(case_paths.insert(name));
        assert!(weeks.insert(record["week"].as_str().unwrap()));
        let data = fs::read(fixtures().join(name)).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&data)),
            record["cases_sha256"].as_str().unwrap()
        );
        let mut calls = 0;
        for line in std::str::from_utf8(&data).unwrap().lines() {
            let case: Case = serde_json::from_str(line).unwrap();
            assert_eq!(case.call, calls);
            assert_eq!(case.week, record["week"].as_str().unwrap());
            replay(&case).unwrap();
            calls += 1;
        }
        assert_eq!(calls, record["calls"].as_u64().unwrap() as usize);
        count += calls;
    }
    let mut covered_cases = std::collections::HashSet::new();
    let mut covered_weeks = std::collections::HashSet::new();
    let mut covered_traces = std::collections::HashSet::new();
    for coverage in index["coverage"].as_array().unwrap() {
        assert!(case_paths.contains(coverage["cases"].as_str().unwrap()));
        covered_cases.insert(coverage["cases"].as_str().unwrap());
        assert!(covered_weeks.insert(coverage["t04_record"].as_str().unwrap()));
        assert!(covered_traces.insert(coverage["trace"].as_str().unwrap()));
    }
    assert_eq!(covered_cases, case_paths);
    assert_eq!(count, 558);
    assert_eq!(count, index["calls"].as_u64().unwrap() as usize);
}

#[test]
fn changed_expected_draw_is_rejected_at_first_divergence() {
    let mut case = first_case();
    case.draws[0].value = case.draws[0].value.wrapping_add(1);
    let error = replay(&case).unwrap_err();
    assert!(error.contains("draw 0:"), "{error}");
}

#[test]
fn changed_input_seed_is_rejected_at_first_divergence() {
    let mut case = first_case();
    case.input.seed = case.input.seed.wrapping_add(1);
    let error = replay(&case).unwrap_err();
    assert!(error.contains("draw 0:"), "{error}");
}

#[test]
fn existing_goal_bytes_accumulate_with_byte_wrapping() {
    let mut files: Vec<_> = fs::read_dir(fixtures())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "jsonl"))
        .collect();
    files.sort();
    let mut exercised = false;
    for file in files {
        for line in fs::read_to_string(file).unwrap().lines() {
            let mut case: Case = serde_json::from_str(line).unwrap();
            if case.input.goals == [0, 0] && case.output.goals != [0, 0] {
                case.input.goals = [255, 255];
                let got = onenil_engine::Engine::new(&case.input).quick_contests(case.input.k);
                assert_eq!(
                    got.goals,
                    [
                        255u8.wrapping_add(case.output.goals[0]),
                        255u8.wrapping_add(case.output.goals[1])
                    ]
                );
                assert_eq!(got.draws, case.draws);
                exercised = true;
                break;
            }
        }
        if exercised {
            break;
        }
    }
    assert!(
        exercised,
        "original corpus must contain a scoring routine call"
    );
}
