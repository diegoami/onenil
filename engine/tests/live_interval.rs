//! [confirmed] Existing T61/T68 full streams remain the no-change authority.
//! [designed:T74] Pause only the live owned image, never replay a later snapshot.
use onenil_engine::watched_compiled::prepared::{Input, Session};
use serde_json::Value;
fn session(edited: bool) -> Session {
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let profile=if edited {"edited"} else {"fresh"};
    let prep:Value=serde_json::from_slice(&std::fs::read(root.join(format!("{profile}_preparation/admission.json"))).unwrap()).unwrap();
    let facts:Value=serde_json::from_slice(&std::fs::read(root.join(format!("{profile}_watched/incoming.json"))).unwrap()).unwrap();
    let input:Input=serde_json::from_value(serde_json::json!({"preparation":prep["input"],"support":facts["support"],"no_pending_key":true})).unwrap();
    if edited {Session::new_edited(input)} else {Session::new_fresh(input)}.unwrap()
}
#[test]
fn owned_interval_preserves_full_original_and_edited_streams() {
    for edited in [false,true] {
        let expected=session(edited).finish().unwrap();
        let mut native=session(edited);
        assert!(native.interval_observation().is_err());
        if edited {for _ in 0..171 {native.advance().unwrap();}}
        let interval=native.pause_at_interval().unwrap();
        assert_eq!(interval["site"],"0AB9:7E4A");
        assert_eq!(interval["control"],1);
        assert!(interval["state"]["remaining"].as_u64().unwrap()>0);
        assert!(interval["state"]["elapsed"].as_u64().unwrap()>=45);
        let observed=native.observation();
        assert_eq!(interval,native.interval_observation().unwrap());
        assert_eq!(interval,native.interval_observation().unwrap());
        assert!(native.pause_at_interval().is_err());
        assert!(native.advance().is_err());
        assert_eq!(observed,native.observation());
        assert_eq!(interval,native.interval_observation().unwrap());
        let got=native.finish().unwrap();
        assert!(got==expected,"paused continuation differs from the complete accepted profile: edited={edited}");
        assert_eq!(got["return"],"0AB9:EBE2");
        assert!(native.finish().is_err());
        assert!(native.pause_at_interval().is_err());
        assert!(native.advance().is_err());
        assert!(native.interval_observation().is_err());
    }
}
