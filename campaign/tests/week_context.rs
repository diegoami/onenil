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

#[test]
fn the_witnessed_sevilla_fixture_admits_and_rejects_foreign_week_totals() {
    // [T91; archaeology 3d41a0db T89, runs/T89/sevilla-r0001] The advertised
    // following fixture IS the witnessed Sevilla friendly: the request admits
    // and binds the prepared match's own context, the caller's witnessed facts
    // record exactly once, the replay attempt refuses accurately, and the week
    // transaction afterwards rejects week-0 totals. No PSG value transplants
    // into the Sevilla path or vice versa (T93 admits only its own receipt).
    let mut c = Campaign::initial().unwrap();
    let ticket = c.match_ticket().unwrap();
    c.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    c.record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    let witnessed: serde_json::Value = serde_json::from_str::<serde_json::Value>(include_str!(
        "../fixtures/sevilla-post-match.json"
    ))
    .unwrap();
    let conditions: Vec<(u16, i8)> = witnessed["witnessed"]["conditions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["player"].as_u64().unwrap() as u16,
                row["condition_after"].as_i64().unwrap() as i8,
            )
        })
        .collect();
    // The witnessed request admits.
    let ticket = c.match_ticket().unwrap();
    // A PSG-score transplant refuses before anything records.
    assert!(c
        .record_sevilla_result(&ticket, [1, 4], "0AB9:EBE2", &conditions)
        .is_err());
    // Foreign condition outcomes refuse: the PSG-era wholesale values and a
    // single-byte tamper both.
    let psg: Vec<(u16, i8)> = c
        .players
        .iter()
        .map(|p| (p.id, p.availability.condition_signed))
        .collect();
    assert!(c
        .record_sevilla_result(&ticket, [1, 1], "0AB9:EBE2", &psg)
        .is_err());
    let mut tampered = conditions.clone();
    tampered[3].1 ^= 1;
    assert!(c
        .record_sevilla_result(&ticket, [1, 1], "0AB9:EBE2", &tampered)
        .is_err());
    assert!(c.second_fixture.is_none());
    // The witnessed posting records exactly once, result and conditions.
    c.record_sevilla_result(&ticket, [1, 1], "0AB9:EBE2", &conditions)
        .unwrap();
    let recorded = c.clone();
    let second = recorded.second_fixture.as_ref().unwrap();
    assert_eq!(second.score, [1, 1]);
    assert_eq!(second.week_index, 1);
    assert_eq!(second.native_return, "0AB9:EBE2");
    for row in &recorded.players {
        let witnessed_row = witnessed["witnessed"]["conditions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|witnessed| witnessed["player"] == row.id)
            .unwrap();
        assert_eq!(
            row.availability.condition_signed,
            witnessed_row["condition_after"]
        );
    }
    // The replay attempt of the completed fixture refuses accurately, and the
    // week transaction rejects transplanted week-0 totals.
    assert_eq!(
        c.match_ticket().err().unwrap(),
        "this fixture already has a recorded result"
    );
    assert_eq!(
        c.record_week_completion(38_761, 69_270, 1, 28, 0, "Celtic")
            .err()
            .unwrap(),
        "the Sevilla week's authenticated movement, fixture or conditions differ"
    );
    assert_eq!(c, recorded);
    // The post-caller state saves and reloads identically.
    let image = serde_json::to_vec(&c).unwrap();
    let mut reloaded = Campaign::initial().unwrap();
    reloaded.replace_from_bytes(&image).unwrap();
    assert_eq!(reloaded, c);
    assert_eq!(
        reloaded.match_ticket().err().unwrap(),
        "this fixture already has a recorded result"
    );
    // A doctored week-two/Celtic state stays outside the supported saved
    // context: this doctored image omits the authenticated cash and flags.
    let mut celtic = recorded.clone();
    celtic.week_index = 2;
    celtic.next_fixture.opponent = 28;
    celtic.next_fixture.opponent_name = "Celtic".into();
    celtic.next_fixture.slot = 3;
    assert!(celtic.validate().is_err());
    assert!(celtic.match_ticket().is_err());
}
