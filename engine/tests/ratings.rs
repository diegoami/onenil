use onenil_engine::ratings::{self, Case};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ratings")
}

fn first_case() -> Case {
    let mut paths: Vec<_> = fs::read_dir(fixtures())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension().is_some_and(|ext| ext == "jsonl")
                && path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with("T15-r")
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
fn first_original_ratings_replay() {
    ratings::replay(&first_case()).unwrap();
}

#[test]
fn changed_expected_ratings_or_helper_outputs_cannot_steer_computation() {
    let original = first_case();
    let mut altered = original.clone();
    altered.output.ratings[usize::from(altered.input.side)] ^= 1;
    assert!(ratings::replay(&altered).is_err());
    altered = original.clone();
    altered.helpers.players[0].output.value ^= 1;
    assert!(ratings::replay(&altered).is_err());
    altered = original;
    altered.input.players[0].p8_4 ^= 1;
    assert!(ratings::replay(&altered).is_err());
}

fn corpus() -> (Vec<Case>, Value) {
    let data = fs::read(fixtures().join("index.json")).unwrap();
    let index: Value = serde_json::from_slice(&data).unwrap();
    let pin: Value =
        serde_json::from_slice(&fs::read(fixtures().join("provenance.json")).unwrap()).unwrap();
    assert_eq!(
        pin["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&data)),
        pin["index_sha256"].as_str().unwrap()
    );
    let commit = pin["commit"].as_str().unwrap();
    assert_eq!(commit.len(), 40);
    assert!(commit.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(index["schema"], "ratings-corpus/1");
    assert_eq!(index["calls"], 1128);
    assert_eq!(index["weeks"], 6);
    assert_eq!(index["traces"], 15);
    assert_eq!(index["coverage"].as_array().unwrap().len(), 15);
    let t11 = fs::read(fixtures().parent().unwrap().join("index.json")).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(t11)),
        index["t11_index_sha256"].as_str().unwrap()
    );
    let mut all = vec![];
    for record in index["records"].as_array().unwrap() {
        let data = fs::read(fixtures().join(record["cases"].as_str().unwrap())).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&data)),
            record["cases_sha256"].as_str().unwrap()
        );
        let rows: Vec<Case> = std::str::from_utf8(&data)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(rows.len() as u64, record["calls"].as_u64().unwrap());
        all.extend(rows);
    }
    assert_eq!(all.len(), 1128);
    (all, index)
}

fn indexed_rows(index: &Value, key: &str) -> Vec<Value> {
    let meta = &index[key];
    let data = fs::read(fixtures().join(meta["file"].as_str().unwrap())).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&data)),
        meta["sha256"].as_str().unwrap()
    );
    let rows: Vec<Value> = std::str::from_utf8(&data)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len() as u64, meta["cases"].as_u64().unwrap());
    rows
}

#[test]
fn all_original_side_ratings_replay() {
    for case in corpus().0 {
        ratings::replay(&case).unwrap();
    }
}

#[test]
fn all_original_player_and_bonus_helpers_replay() {
    let (_, index) = corpus();
    let (mut players, mut bonuses) = (0, 0);
    for case in indexed_rows(&index, "helpers") {
        match case["kind"].as_str().unwrap() {
            "player" => {
                let input = serde_json::from_value(case["input"].clone()).unwrap();
                let expected: ratings::PlayerOutput =
                    serde_json::from_value(case["output"].clone()).unwrap();
                assert_eq!(ratings::player_value(&input).unwrap(), expected);
                players += 1;
            }
            "bonus" => {
                let input = serde_json::from_value(case["input"].clone()).unwrap();
                let expected: ratings::BonusOutput =
                    serde_json::from_value(case["output"].clone()).unwrap();
                assert_eq!(ratings::club_bonus(&input).unwrap(), expected);
                bonuses += 1;
            }
            kind => panic!("unknown ratings helper {kind}"),
        }
    }
    assert_eq!((players, bonuses), (12614, 2495));
}

#[test]
fn all_original_computed_ratings_contests_replay() {
    let (cases, index) = corpus();
    let rows = indexed_rows(&index, "handoffs");
    assert_eq!(rows.len(), 558);
    for row in rows {
        let week = row["week"].as_str().unwrap();
        let sides: [ratings::Input; 2] = std::array::from_fn(|i| {
            let n = row["sides"][i].as_u64().unwrap() as usize;
            cases
                .iter()
                .find(|case| case.week == week && case.call == n)
                .unwrap()
                .input
                .clone()
        });
        let contest: onenil_engine::Case = serde_json::from_value(row["contest"].clone()).unwrap();
        let request = ratings::ContestRequest {
            k: contest.input.k,
            seed: contest.input.seed,
            goals: contest.input.goals,
        };
        let got = ratings::computed_contests(&sides, &request).unwrap();
        assert_eq!(got.sides[1].output.ratings, contest.input.ratings);
        assert_eq!(got.sides[1].output.seed, contest.input.seed);
        assert_eq!(got.contest.draws, contest.draws);
        assert_eq!(got.contest.seed, contest.output.seed);
        assert_eq!(got.contest.goals, contest.output.goals);
    }
}
