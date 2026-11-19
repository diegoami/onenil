//! [confirmed:archaeology 788b833 T93] The second witnessed week, never a third match.
use onenil_campaign::Campaign;
fn post_caller() -> Campaign {
    let mut c = Campaign::initial().unwrap();
    let t = c.match_ticket().unwrap();
    c.record_match(&t, [1, 4], "0AB9:EBE2").unwrap();
    c.record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    let v: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/sevilla-post-match.json")).unwrap();
    let conditions: Vec<_> = v["witnessed"]["conditions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["player"].as_u64().unwrap() as u16,
                r["condition_after"].as_i64().unwrap() as i8,
            )
        })
        .collect();
    let t = c.match_ticket().unwrap();
    c.record_sevilla_result(&t, [1, 1], "0AB9:EBE2", &conditions)
        .unwrap();
    c
}
#[test]
fn completes_once_preserves_conditions_and_refuses_celtic_after_reload() {
    let mut c = post_caller();
    let before = c.clone();
    c.record_week_completion(69_410, 69_270, 1, 28, 0, "Celtic")
        .unwrap();
    assert_eq!(c.cash, 421_631);
    assert_eq!(c.week_index, 2);
    assert_eq!(c.revision, before.revision + 1);
    assert_eq!(
        c.next_fixture,
        onenil_campaign::Fixture {
            slot: 3,
            club: 130,
            opponent: 28,
            opponent_name: "Celtic".into(),
            competition_code: 1,
            flags: 0,
            venue_code: 0,
            opponent_flags: 1,
        }
    );
    assert_eq!(c.players, before.players);
    assert_eq!(c.club, before.club);
    assert_eq!(c.played_fixture, before.played_fixture);
    assert_eq!(c.second_fixture, before.second_fixture);
    let advanced = c.clone();
    assert!(c
        .record_week_completion(69_410, 69_270, 1, 28, 0, "Celtic")
        .is_err());
    assert_eq!(c, advanced);
    let reloaded = Campaign::decode(&c.encode().unwrap()).unwrap();
    assert_eq!(reloaded, c);
    assert!(reloaded.match_ticket().unwrap_err().contains("Celtic"));
    assert!(reloaded.match_ticket().unwrap_err().contains("unwitnessed"));
    for edit in 0..5 {
        let mut bad = advanced.clone();
        match edit {
            0 => bad.cash += 1,
            1 => bad.next_fixture.opponent_flags = 0,
            2 => bad.next_fixture.slot = 4,
            3 => bad.second_fixture = None,
            _ => bad.played_fixture = None,
        }
        assert!(Campaign::decode(&serde_json::to_vec(&bad).unwrap()).is_err());
    }
}
#[test]
fn refuses_foreign_receipts_missing_result_and_condition_changes_atomically() {
    let before = post_caller();
    for args in [
        (38_761, 69_270, 1, 28, 0, "Celtic"),
        (69_411, 69_270, 1, 28, 0, "Celtic"),
        (69_410, 69_270, 1, 176, 0, "Sevilla"),
        (69_410, 69_270, 1, 28, 1, "Celtic"),
        (69_410, 69_270, 1, 28, 0, "Celtic B"),
    ] {
        let mut c = before.clone();
        assert!(c
            .record_week_completion(args.0, args.1, args.2, args.3, args.4, args.5)
            .is_err());
        assert_eq!(c, before);
    }
    let mut missing = before.clone();
    missing.second_fixture = None;
    let snapshot = missing.clone();
    assert!(missing
        .record_week_completion(69_410, 69_270, 1, 28, 0, "Celtic")
        .is_err());
    assert_eq!(missing, snapshot);
    let mut changed = before.clone();
    changed.players[0].availability.condition_signed ^= 1;
    let snapshot = changed.clone();
    assert!(changed
        .record_week_completion(69_410, 69_270, 1, 28, 0, "Celtic")
        .is_err());
    assert_eq!(changed, snapshot);
}
