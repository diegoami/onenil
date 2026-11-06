use onenil_campaign::{Campaign, Location};
#[test]
fn source_identity_and_observed_changes_are_preserved() {
    let mut c = Campaign::initial().unwrap();
    assert_eq!(c.club.id, 130);
    assert_eq!(c.club.name, "Blackburn R");
    assert_eq!(c.cash, 452000);
    assert_eq!(c.week_index, 0);
    assert_eq!(c.club.first_team[0], Some(2581));
    assert_eq!(c.next_fixture.opponent, 17);
    c.exchange(Location::First(0), Location::First(17)).unwrap();
    c.set_position(2582, 1).unwrap();
    c.set_instructions(2598, 16).unwrap();
    assert_eq!(
        c.source_projection().unwrap(),
        Campaign::observed_changed_projection().unwrap()
    );
    assert_eq!(c.revision, 3);
    let bytes = c.encode().unwrap();
    assert_eq!(Campaign::decode(&bytes).unwrap(), c);
    let before = c.clone();
    assert!(c
        .exchange(Location::First(20), Location::Reserve(0))
        .is_err());
    assert_eq!(c, before);
}
#[test]
fn malformed_graphs_never_replace_live_state() {
    let mut c = Campaign::initial().unwrap();
    let before = c.clone();
    let good = serde_json::to_value(&c).unwrap();
    for change in [
        "version",
        "namespace",
        "duplicate",
        "missing",
        "foreign",
        "fixture",
        "instructions",
        "unknown",
    ] {
        let mut v = good.clone();
        match change {
            "version" => v["version"] = 2.into(),
            "namespace" => v["namespace"]["league"] = "ENGL-94.DAT".into(),
            "duplicate" => v["club"]["first_team"][1] = 2581.into(),
            "missing" => v["club"]["first_team"][0] = serde_json::Value::Null,
            "foreign" => v["players"][0]["id"] = 2661.into(),
            "fixture" => v["next_fixture"]["slot"] = 2.into(),
            "instructions" => v["players"][0]["individual_instructions"] = 256.into(),
            _ => v["extra"] = true.into(),
        }
        assert!(
            c.replace_from_bytes(&serde_json::to_vec(&v).unwrap())
                .is_err(),
            "{change}"
        );
        assert_eq!(c, before);
    }
    assert!(c.replace_from_bytes(b"{").is_err());
    assert_eq!(c, before);
    let old_revision = c.revision;
    c.exchange(Location::First(0), Location::First(0)).unwrap();
    assert_eq!(c.revision, old_revision);
    c.revision = u64::MAX;
    let before = c.clone();
    assert!(c.exchange(Location::First(0), Location::First(1)).is_err());
    assert_eq!(c, before);
}
