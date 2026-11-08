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
    serde_json::from_value(serde_json::json!({"preparation":prep["input"],"support":facts["support"],"no_pending_key":true})).unwrap()
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
fn interval_history_is_read_only_incomplete_and_a_prefix_of_completion() {
    for edited in [false, true] {
        let mut native = session(edited);
        assert!(native.interval_history().is_err());
        let interval = native.pause_at_interval().unwrap();
        let observation = native.observation();
        let prefix = native.interval_history().unwrap();
        assert_eq!(prefix, native.interval_history().unwrap());
        assert_eq!(observation, native.observation());
        assert!(prefix.get("return").is_none());
        assert!(prefix.get("final").is_none());
        assert_eq!(prefix["boundary"], "0AB9:7E4A");
        assert_eq!(prefix["control"], 1);
        assert_eq!(
            prefix["ticks"].as_array().unwrap().len() as u64,
            interval["ticks"].as_u64().unwrap()
        );
        let finished = native
            .finish()
            .unwrap_or_else(|e| panic!("{}", e.fault.message));
        for key in [
            "ticks",
            "draws",
            "events",
            "tick_live_goals",
            "caller_clocks",
        ] {
            let reached = prefix[key].as_array().unwrap();
            assert_eq!(
                &finished[key].as_array().unwrap()[..reached.len()],
                reached,
                "{key}"
            );
        }
        assert_eq!(prefix["setup_events"], finished["setup_events"]);
        assert!(native.interval_history().is_err());
    }
}
