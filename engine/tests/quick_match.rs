//! [confirmed] Prepared quick match: T49/T102 results and T123's action stream.
//! The expected facts are original observations; the session computes them.
use onenil_engine::quick_match::{self, Profile};
use serde_json::Value;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn expected() -> Value {
    serde_json::from_slice(
        &std::fs::read(root().join("engine/tests/fixtures/quick_match/expected.json")).unwrap(),
    )
    .unwrap()
}

fn preparation(match_row: &Value) -> String {
    let base = root().join("engine/tests/fixtures");
    let preparation: Value = serde_json::from_slice(
        &std::fs::read(base.join(match_row["preparation"].as_str().unwrap())).unwrap(),
    )
    .unwrap();
    let support: Value = serde_json::from_slice(
        &std::fs::read(base.join(match_row["support"].as_str().unwrap())).unwrap(),
    )
    .unwrap();
    serde_json::json!({
        "preparation": preparation["input"],
        "support": support["support"],
        "no_pending_key": true,
    })
    .to_string()
}

fn assert_match(profile: Profile, row: &Value, report: &quick_match::Report) {
    assert_eq!(report.steps, row["steps"].as_u64().unwrap() as usize);
    assert_eq!(
        report.setup_draws,
        row["setup_draws"].as_u64().unwrap() as usize
    );
    assert_eq!(
        report.step_draws,
        row["step_draws"].as_u64().unwrap() as usize
    );
    assert_eq!(
        report.setup_draws + report.step_draws,
        row["total_draws"].as_u64().unwrap() as usize
    );
    assert_eq!(
        report.half_time,
        Some(row["half_time"].as_u64().unwrap() as usize)
    );
    assert_eq!(report.score, row["score"], "final score differs");

    let goals = row["goals"].as_array().unwrap();
    assert_eq!(report.goals.len(), goals.len(), "goal count differs");
    let logs = &report.snapshot["scorer_logs"];
    for (got, want) in report.goals.iter().zip(goals) {
        assert_eq!(got["event"]["minute"], want["minute"]);
        assert_eq!(got["event"]["actor"], want["actor"]);
        assert_eq!(got["event"]["side_flag"], want["side_flag"]);
        let side = want["side_flag"].as_u64().unwrap() as usize;
        let minute = want["minute"].as_u64().unwrap() as u16;
        let scorer = logs[side]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["minute_byte"].as_u64().unwrap() as u16 == minute)
            .unwrap_or_else(|| panic!("{profile:?}: no scorer log at minute {minute}"));
        assert_eq!(
            scorer["player"], want["player"],
            "{profile:?}: scorer record differs"
        );
    }

    for substitution in row["substitutions"].as_array().unwrap() {
        let elapsed = substitution["elapsed"].as_u64().unwrap();
        let mut got = vec![];
        for step in report.timeline.iter() {
            if step["elapsed"].as_u64() != Some(elapsed) {
                continue;
            }
            for event in step["events"].as_array().unwrap() {
                if event["kind"] == "replacement-log" {
                    got.push(event["arguments"].clone());
                }
            }
        }
        assert_eq!(
            Value::Array(got),
            substitution["signatures"],
            "{profile:?}: substitution signatures at minute {elapsed} differ"
        );
    }

    let players: Vec<Value> = report.snapshot["players"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["player"].clone())
        .collect();
    assert_eq!(
        Value::Array(players),
        row["scene_players"],
        "{profile:?}: final scene players differ"
    );
}

#[test]
fn friendly_quick_match_reproduces_the_original_result_and_records() {
    let expected = expected();
    let row = &expected["matches"]["fresh"];
    let mut session = quick_match::session(Profile::Fresh, &preparation(row)).unwrap();
    let report = quick_match::play(&mut session).unwrap();
    assert_match(Profile::Fresh, row, &report);
}

#[test]
fn league_quick_match_reproduces_the_original_result_and_action_records() {
    let expected = expected();
    let row = &expected["matches"]["league"];
    let mut session = quick_match::session(Profile::League, &preparation(row)).unwrap();
    let report = quick_match::play(&mut session).unwrap();
    assert_match(Profile::League, row, &report);
}
