//! [confirmed] Full original lifecycle, archaeology F40/60179ee; T41 reference pins.
use onenil_engine::watched_compiled::{action::State, watched_match};
use serde_json::{json, Value};
use std::path::Path;

fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

#[test]
fn one_initial_admission_matches_every_original_tick_draw_action_and_final_return() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let fixture = read(&root.join("engine/tests/fixtures/watched_match/initial.json"));
    let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
    let got = watched_match::run(&state, true)
        .unwrap_or_else(|error| panic!("native lifecycle failed: {}", error.fault.message));
    assert_eq!(got["ticks"].as_array().unwrap().len(), 4199);
    assert_eq!(got["draws"].as_array().unwrap().len(), 182553);
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
            let mut expected = original.clone();
            for key in ["draw_start", "draw_end"] {
                expected[key] = json!(expected[key].as_u64().unwrap() - 258);
            }
            assert_eq!(got["ticks"][ticks], expected, "tick {}", ticks + 1);
            ticks += 1;
        }
        for original in payload["draws"].as_array().unwrap() {
            if draws >= 258 {
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
                assert_eq!(got["draws"][draws - 258], expected, "draw {draws}");
            }
            draws += 1;
        }
        for original in payload["actions"].as_array().unwrap() {
            let mut expected = original.clone();
            expected.as_object_mut().unwrap().remove("event");
            for key in ["draw_start", "draw_end"] {
                expected[key] = json!(expected[key].as_u64().unwrap() - 258);
            }
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
