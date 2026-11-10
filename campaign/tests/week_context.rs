use onenil_campaign::Campaign;
#[test]
fn the_week_one_context_binds_the_authenticated_cash_and_full_fixture() {
    // [review #176 F1] The completed friendly week's context must bind every
    // authenticated value: the moved funds (421,491) and the complete following
    // fixture (T81 dc54fe9, T82 c13195a). A hand-edited or corrupted week-one
    // save with arbitrary cash, venue, opponent flags or fixture club refuses.
    let mut c = Campaign::initial().unwrap();
    let ticket = c.match_ticket().unwrap();
    c.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    c.record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    let advanced = c.clone();
    let edits: [(&str, fn(&mut Campaign)); 5] = [
        ("cash", |p: &mut Campaign| p.cash = 999_999_999),
        ("venue_code", |p: &mut Campaign| {
            p.next_fixture.venue_code = 7
        }),
        ("opponent_flags", |p: &mut Campaign| {
            p.next_fixture.opponent_flags = 3
        }),
        ("fixture_club", |p: &mut Campaign| p.next_fixture.club = 999),
        ("opponent_name", |p: &mut Campaign| {
            p.next_fixture.opponent_name = "Sevilla B".into()
        }),
    ];
    for (name, edit) in edits {
        let mut probe = advanced.clone();
        edit(&mut probe);
        assert!(probe.validate().is_err(), "{name} accepted");
        let mut v = serde_json::to_value(&advanced).unwrap();
        match name {
            "cash" => v["cash"] = 999_999_999.into(),
            "venue_code" => v["next_fixture"]["venue_code"] = 7.into(),
            "opponent_flags" => v["next_fixture"]["opponent_flags"] = 3.into(),
            "fixture_club" => v["next_fixture"]["club"] = 999.into(),
            _ => v["next_fixture"]["opponent_name"] = "Sevilla B".into(),
        }
        let mut reloaded = advanced.clone();
        assert!(
            reloaded
                .replace_from_bytes(&serde_json::to_vec(&v).unwrap())
                .is_err(),
            "{name} accepted through a save image"
        );
        assert_eq!(reloaded, advanced);
    }
    // The week-zero context binds the source cash as well.
    let mut week_zero = Campaign::initial().unwrap();
    week_zero.cash += 1;
    assert!(week_zero.validate().is_err());
    // The transaction itself refuses any movement that is not the
    // authenticated one, and leaves the recorded result untouched.
    let mut c2 = Campaign::initial().unwrap();
    let ticket = c2.match_ticket().unwrap();
    c2.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    let before = c2.clone();
    assert!(c2
        .record_week_completion(1_000, 31_509, 1, 176, 0, "Sevilla")
        .is_err());
    assert_eq!(c2, before);
    assert!(c2
        .record_week_completion(38_761, 69_270, 1, 176, 1, "Sevilla")
        .is_err());
    assert_eq!(c2, before);
    assert!(c2
        .record_week_completion(38_761, 69_270, 1, 17, 0, "P St Germain")
        .is_err());
    assert_eq!(c2, before);
}
