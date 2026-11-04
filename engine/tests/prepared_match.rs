//! [confirmed] T45 owned selected setup and retained lifecycle; original T41 reference pins.
use onenil_engine::watched_compiled::prepared::{Input, Session};
use serde_json::{json, Value};
use std::path::Path;

fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

#[test]
fn fresh_and_retained_prepared_sessions_match_every_original_tick_draw_action_and_return() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let fixture = read(&root.join("engine/tests/fixtures/prepared_match/admission.json"));
    for prefix_steps in [0, 171] {
        let input: Input = serde_json::from_value(fixture["input"].clone()).unwrap();
        let mut session = Session::new(input).unwrap();
        for _ in 0..prefix_steps {
            session.advance().unwrap();
        }
        let got = session
            .finish()
            .unwrap_or_else(|error| panic!("prepared lifecycle failed: {}", error.fault.message));
        assert!(session.finish().is_err());
        assert!(session.advance().is_err());
        compare_original(root, &got);
    }
}
fn compare_original(root: &Path, got: &Value) {
    assert_eq!(got["ticks"].as_array().unwrap().len(), 4199);
    assert_eq!(got["draws"].as_array().unwrap().len(), 182811);
    assert_eq!(got["events"].as_array().unwrap().len(), 831);
    let live = got["tick_live_goals"].as_array().unwrap();
    assert_eq!(live.len(), 4199);
    assert_eq!(live.first().unwrap(), &json!([0, 0]));
    assert_eq!(live.last().unwrap(), &got["live_goals"]);
    let goals = got["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["code"] == 38 || e["code"] == 39)
        .count();
    let mut previous = [0u64; 2];
    let mut changes = 0;
    for score in live {
        let current = [score[0].as_u64().unwrap(), score[1].as_u64().unwrap()];
        for side in 0..2 {
            assert!(current[side] >= previous[side]);
            changes += current[side] - previous[side];
        }
        previous = current;
    }
    assert_eq!(changes as usize, goals);
    let reference = root.join("records/T41/reference");
    let index = read(&reference.join("index.json"));
    let (mut ticks, mut draws, mut actions) = (0, 0, 0);
    for entry in index["chunks"].as_array().unwrap() {
        let name = Path::new(entry["path"].as_str().unwrap())
            .file_name()
            .unwrap();
        let chunk = read(&reference.join(name));
        let payload = &chunk["payload"];
        for original in payload["ticks"].as_array().unwrap() {
            let expected = original.clone();
            assert_eq!(got["ticks"][ticks], expected, "tick {}", ticks + 1);
            ticks += 1;
        }
        for original in payload["draws"].as_array().unwrap() {
            {
                let callers = original["callers"].as_array().unwrap();
                let site = if callers[0] == "24BC:0D4F" {
                    &callers[1]
                } else {
                    &callers[2]
                };
                let mut expected = json!({"site":site});
                for key in ["n", "seed_before", "seed_after", "value"] {
                    expected[key] = original[key].clone();
                }
                assert_eq!(got["draws"][draws], expected, "draw {draws}");
            }
            draws += 1;
        }
        for original in payload["actions"].as_array().unwrap() {
            let mut expected = original.clone();
            expected.as_object_mut().unwrap().remove("event");
            assert_eq!(got["events"][actions], expected, "action {actions}");
            actions += 1;
        }
    }
    assert_eq!((ticks, draws, actions), (4199, 182811, 831));
    let finish = read(
        &reference.join(
            Path::new(index["finish"]["path"].as_str().unwrap())
                .file_name()
                .unwrap(),
        ),
    );
    assert_eq!(got["final"], finish["final"]);
    assert_eq!(got["return"], finish["boundary"]);
    assert_eq!(
        got["live_goals"],
        json!([finish["final"]["scores"][1], finish["final"]["scores"][0]])
    );
}
