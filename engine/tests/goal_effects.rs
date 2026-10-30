use onenil_engine::goal_effects::{self, Case, HelperCase};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/goal_effects")
}
fn first_case() -> Case {
    let mut paths: Vec<_> = fs::read_dir(fixtures())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.extension().is_some_and(|x| x == "jsonl")
                && p.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with("T14-r")
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
fn first_original_goal_effect_replays() {
    goal_effects::replay(&first_case()).unwrap();
}
#[test]
fn all_original_goal_effects_replay() {
    let raw = fs::read(fixtures().join("index.json")).unwrap();
    let index: Value = serde_json::from_slice(&raw).unwrap();
    let pin: Value =
        serde_json::from_slice(&fs::read(fixtures().join("provenance.json")).unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&raw)),
        pin["index_sha256"].as_str().unwrap()
    );
    assert_eq!(
        pin["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    assert_eq!(pin["commit"].as_str().unwrap().len(), 40);
    let mut count = 0;
    for r in index["records"].as_array().unwrap() {
        let data = fs::read(fixtures().join(r["cases"].as_str().unwrap())).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&data)),
            r["cases_sha256"].as_str().unwrap()
        );
        let mut n = 0;
        for line in std::str::from_utf8(&data).unwrap().lines() {
            let c: Case = serde_json::from_str(line).unwrap();
            goal_effects::replay(&c).unwrap();
            n += 1;
        }
        assert_eq!(n, r["calls"].as_u64().unwrap());
        count += n;
    }
    assert_eq!(count, 17);
    assert_eq!(index["calls"], 17);
    assert_eq!(index["weeks"], 6);
    assert_eq!(index["traces"], 15);
    assert_eq!(index["coverage"].as_array().unwrap().len(), 15);
}
#[test]
fn all_original_byte6_helpers_replay() {
    let index: Value =
        serde_json::from_slice(&fs::read(fixtures().join("index.json")).unwrap()).unwrap();
    let h = &index["helper_checks"];
    let data = fs::read(fixtures().join(h["file"].as_str().unwrap())).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&data)),
        h["sha256"].as_str().unwrap()
    );
    let mut n = 0;
    for line in std::str::from_utf8(&data).unwrap().lines() {
        let c: HelperCase = serde_json::from_str(line).unwrap();
        assert_eq!(goal_effects::byte6(&c.input), c.output);
        n += 1;
    }
    assert_eq!(n, 34);
}

fn all_cases() -> Vec<Case> {
    let idx: Value =
        serde_json::from_slice(&fs::read(fixtures().join("index.json")).unwrap()).unwrap();
    idx["records"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|r| {
            fs::read_to_string(fixtures().join(r["cases"].as_str().unwrap()))
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
fn placement_cases() -> Vec<onenil_engine::placement::Case> {
    let base = fixtures().parent().unwrap().join("placement");
    let idx: Value = serde_json::from_slice(&fs::read(base.join("index.json")).unwrap()).unwrap();
    idx["records"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|r| {
            fs::read_to_string(base.join(r["cases"].as_str().unwrap()))
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
#[test]
fn composed_placement_effects_keep_the_real_consumed_seed() {
    let placements = placement_cases();
    for case in all_cases() {
        let p = placements
            .iter()
            .find(|p| p.week == case.week && p.step == case.step)
            .unwrap();
        let mut engine = onenil_engine::Engine::new(&p.contest.input);
        engine.quick_contests(p.contest.input.k);
        let placed = engine.place_goals(&p.input).unwrap();
        let request = placed.callbacks.first().unwrap();
        assert_eq!(request, &case.request);
        let seed = engine.goal_effect_seed();
        let got = engine.apply_goal_effects(&case.before, request).unwrap();
        assert_eq!(engine.goal_effect_seed(), seed);
        assert_eq!(got.state, case.after);
        assert_eq!(got.commentary, case.commentary);
    }
}
#[test]
fn changed_expectation_cannot_steer_native_effects() {
    let original = first_case();
    let got = goal_effects::apply_goal_effects(&original.before, &original.request).unwrap();
    let mut changed = original.clone();
    changed.after.players[0].p15_byte6 = changed.after.players[0].p15_byte6.wrapping_add(1);
    assert!(goal_effects::replay(&changed).is_err());
    assert_eq!(
        goal_effects::apply_goal_effects(&changed.before, &changed.request).unwrap(),
        got
    );
}
#[test]
fn changed_coin_request_or_composed_actor_is_rejected() {
    let original = first_case();
    let mut bad = original.request.clone();
    bad.coin ^= 1;
    assert!(goal_effects::apply_goal_effects(&original.before, &bad).is_err());
    let placements = placement_cases();
    let p = placements
        .iter()
        .find(|p| p.week == original.week && p.step == original.step)
        .unwrap();
    let mut engine = onenil_engine::Engine::new(&p.contest.input);
    engine.quick_contests(p.contest.input.k);
    let placed = engine.place_goals(&p.input).unwrap();
    let mut state = original.before.clone();
    state.global_actor = if state.global_actor == 1 { 2 } else { 1 };
    assert!(engine
        .apply_goal_effects(&state, &placed.callbacks[0])
        .is_err());
}
