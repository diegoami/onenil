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
fn witnessed_replacement_resumes_original_owned_match() {
    let mut native = session(false);
    assert!(native.finish_with_interval_replacement(true).is_err());
    native.pause_at_interval().unwrap();
    let result = native
        .finish_with_interval_replacement(true)
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(result["match"]["return"], "0AB9:EBE2");
    assert!(!result["transaction"].as_array().unwrap().is_empty());
    assert!(native.finish_with_interval_replacement(true).is_err());
}
