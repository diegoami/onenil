use onenil_engine::{placement, Engine};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::HashSet, fs, path::PathBuf};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/placement")
}

fn first_case() -> placement::Case {
    let mut paths: Vec<_> = fs::read_dir(fixtures())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("T13-r")
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

fn corpus() -> Vec<placement::Case> {
    let index_bytes = fs::read(fixtures().join("index.json")).unwrap();
    let index: Value = serde_json::from_slice(&index_bytes).unwrap();
    let provenance: Value =
        serde_json::from_slice(&fs::read(fixtures().join("provenance.json")).unwrap()).unwrap();
    assert_eq!(
        provenance["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    assert_eq!(provenance["commit"].as_str().unwrap().len(), 40);
    assert_eq!(
        provenance["index_sha256"],
        format!("{:x}", Sha256::digest(index_bytes))
    );
    assert_eq!(index["schema"], "placement-corpus/1");
    assert_eq!(index["calls"], 558);
    assert_eq!(index["draws"], 1804);
    assert_eq!(index["callbacks"], 17);
    assert_eq!(index["weeks"], 6);
    assert_eq!(index["traces"], 15);
    let records = index["records"].as_array().unwrap();
    assert_eq!(records.len(), 6);
    let mut files = HashSet::new();
    let mut cases = Vec::new();
    for record in records {
        let name = record["cases"].as_str().unwrap();
        assert!(files.insert(name));
        assert_eq!(PathBuf::from(name).file_name().unwrap(), name);
        let data = fs::read(fixtures().join(name)).unwrap();
        assert_eq!(
            record["cases_sha256"],
            format!("{:x}", Sha256::digest(&data))
        );
        let rows: Vec<placement::Case> = std::str::from_utf8(&data)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(rows.len(), record["calls"].as_u64().unwrap() as usize);
        for (step, case) in rows.iter().enumerate() {
            assert_eq!(case.step, step);
            assert_eq!(case.week, record["week"].as_str().unwrap());
            assert_eq!(case.schema, "placement-case/1");
            assert_eq!(case.source["t12_step"], step);
            assert_eq!(
                case.source["draw_end"].as_u64().unwrap()
                    - case.source["draw_start"].as_u64().unwrap(),
                case.draws.len() as u64
            );
        }
        cases.extend(rows);
    }
    let coverage = index["coverage"].as_array().unwrap();
    assert_eq!(coverage.len(), 15);
    let mut traced = HashSet::new();
    let mut covered = HashSet::new();
    for entry in coverage {
        assert!(traced.insert(entry["trace"].as_str().unwrap()));
        let name = entry["cases"].as_str().unwrap();
        assert!(files.contains(name));
        covered.insert(name);
    }
    assert_eq!(covered, files);
    assert_eq!(cases.len(), 558);
    cases
}

#[test]
fn first_original_placement_replays() {
    placement::replay(&first_case()).unwrap();
}

#[test]
fn all_original_placements_and_callbacks_replay() {
    let cases = corpus();
    let mut draws = 0;
    let mut callbacks = 0;
    for case in cases {
        let got = placement::replay(&case).unwrap();
        draws += got.draws.len();
        callbacks += got.callbacks.len();
    }
    assert_eq!(draws, 1804);
    assert_eq!(callbacks, 17);
}

#[test]
fn all_original_availability_predicates_match() {
    let index: Value =
        serde_json::from_slice(&fs::read(fixtures().join("index.json")).unwrap()).unwrap();
    let helper = &index["helper_checks"];
    let data = fs::read(fixtures().join(helper["file"].as_str().unwrap())).unwrap();
    assert_eq!(helper["sha256"], format!("{:x}", Sha256::digest(&data)));
    let mut count = 0;
    for line in std::str::from_utf8(&data).unwrap().lines() {
        let case: placement::AvailabilityCase = serde_json::from_str(line).unwrap();
        assert_eq!(case.schema, "availability-case/1");
        assert_eq!(
            placement::unavailable(&case.input),
            case.unavailable,
            "{} step {} scene {}",
            case.week,
            case.step,
            case.scene_slot
        );
        count += 1;
    }
    assert_eq!(count, 12276);
    assert_eq!(helper["cases"], count);
}

#[test]
fn contests_compose_without_resetting_seed_or_generated_handoff() {
    for case in corpus() {
        let mut engine = Engine::new(&case.contest.input);
        let contests = engine.quick_contests(case.contest.input.k);
        assert_eq!(contests.seed, case.contest.output.seed);
        assert_eq!(contests.goals, case.contest.output.goals);
        assert_eq!(contests.draws, case.contest.draws);
        let got = engine.place_goals(&case.input).unwrap();
        placement::compare(&case, &got).unwrap();
    }
}
