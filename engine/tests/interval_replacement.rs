//! [confirmed] Existing T61/T68 full streams remain the no-change authority.
//! [designed:T74] Pause only the live owned image, never replay a later snapshot.
use onenil_engine::watched_compiled::prepared::{Input, Session};
use serde_json::Value;
fn input(edited: bool) -> Input {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let profile = if edited { "edited" } else { "fresh" };
    let prep: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("{profile}_preparation/admission.json"))).unwrap(),
    )
    .unwrap();
    let facts: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("{profile}_watched/incoming.json"))).unwrap(),
    )
    .unwrap();
    let mut packet = serde_json::json!({"preparation":prep["input"],"support":facts["support"],"no_pending_key":true});
    if !edited {
        let editor: Value =
            serde_json::from_str(include_str!("fixtures/halftime_replacement/incoming.json"))
                .unwrap();
        packet["support"]
            .as_array_mut()
            .unwrap()
            .extend(editor["support"].as_array().unwrap().iter().cloned());
    }
    serde_json::from_value(packet).unwrap()
}
fn session(edited: bool) -> Session {
    if edited {
        Session::new_edited(input(edited))
    } else {
        Session::new_fresh(input(edited))
    }
    .unwrap()
}
#[test]
fn witnessed_replacement_resumes_original_owned_match() {
    let witness: Value =
        serde_json::from_str(include_str!("fixtures/halftime_replacement/witness.json")).unwrap();
    for cancel in [false, true] {
        let mut native = session(false);
        let unstarted = native.observation();
        assert!(native.finish_with_interval_replacement(cancel).is_err());
        assert_eq!(unstarted, native.observation());
        native.pause_at_interval().unwrap();
        let prefix = native.observation();
        let result = native
            .finish_with_interval_replacement(cancel)
            .unwrap_or_else(|e| panic!("{e}"));
        let samples = result["transaction"].as_array().unwrap();
        let confirm = if cancel { 15 } else { 13 };
        assert_eq!(samples.len(), confirm + 3);
        assert!(samples.iter().all(|s| s["club_roster_count"] == 18));
        for sample in &samples[..=confirm] {
            assert_eq!(sample["state"], witness["before"]);
        }
        for sample in &samples[confirm + 1..] {
            assert_eq!(sample["state"], witness["after"]);
        }
        assert_eq!(samples[2]["selected_slot"], 2);
        assert_eq!(samples[confirm]["selected_slot"], 12);
        assert_eq!(samples[confirm + 1]["selected_slot"], 12);
        assert_eq!(samples[confirm]["picked"], 1);
        assert_eq!(samples[confirm + 1]["picked"], 0);
        if cancel {
            assert_eq!(samples[3]["picked"], 1);
            assert_eq!(samples[4]["picked"], 0);
        }
        let got = &result["match"];
        assert_eq!(got["return"], "0AB9:EBE2");
        for key in ["ticks", "draws"] {
            let old = prefix[key].as_array().unwrap();
            assert_eq!(&got[key].as_array().unwrap()[..old.len()], old);
        }
        let old_events = prefix["events"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|e| e["tick"] != 0)
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(
            &got["events"].as_array().unwrap()[..old_events.len()],
            old_events
        );
        let n = prefix["ticks"].as_array().unwrap().len();
        let scenes = got["ticks"][n]["before"]["scenes"].as_array().unwrap();
        assert!(scenes.iter().any(|s| s["word14"] == 2592));
        assert!(!scenes.iter().any(|s| s["word14"] == 2582));
        assert!(native.finish_with_interval_replacement(cancel).is_err());
        assert!(native.finish().is_err());
        assert!(native.advance().is_err());
    }
}
#[test]
fn unsupported_edited_request_preserves_owned_interval() {
    let mut native = session(true);
    native.pause_at_interval().unwrap();
    let before = native.observation();
    let interval = native.interval_observation().unwrap();
    assert!(native.finish_with_interval_replacement(true).is_err());
    assert_eq!(before, native.observation());
    assert_eq!(interval, native.interval_observation().unwrap());
}
#[test]
fn missing_editor_input_fails_closed_and_bad_values_are_not_admitted() {
    let mut bad = input(false);
    bad.support
        .iter_mut()
        .find(|f| {
            matches!(
                f.area,
                onenil_engine::watched_compiled::action::Area::Config
            ) && f.offset == 4
        })
        .unwrap()
        .value = 2;
    assert!(Session::new_fresh(bad).is_err());
    let mut missing = input(false);
    missing.support.retain(|f| {
        !(matches!(f.area, onenil_engine::watched_compiled::action::Area::Ds) && f.offset == 894)
    });
    let mut native = Session::new_fresh(missing).unwrap();
    native.pause_at_interval().unwrap();
    assert!(native
        .finish_with_interval_replacement(false)
        .unwrap_err()
        .contains("write baseline"));
    let failed = native.observation();
    assert!(native.finish_with_interval_replacement(false).is_err());
    assert!(native.finish().is_err());
    assert!(native.advance().is_err());
    assert!(native.pause_at_interval().is_err());
    assert_eq!(failed, native.observation());
}
