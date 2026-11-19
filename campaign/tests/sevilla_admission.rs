//! [designed:T94; native #202] Unsupported Sevilla edits stay editable but cannot play.
use onenil_campaign::{Campaign, Location};
fn week_one() -> Campaign {
    let mut c = Campaign::initial().unwrap();
    let ticket = c.match_ticket().unwrap();
    c.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    c.record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    c
}
#[test]
fn changed_setup_refuses_without_mutation_and_after_reload() {
    for edit in 0..9 {
        let mut c = week_one();
        match edit {
            0 => c.exchange(Location::First(1), Location::First(2)).unwrap(),
            1 => c.set_position(2582, 3).unwrap(),
            2 => c.set_instructions(2582, 0).unwrap(),
            3 => c.club.team_instructions ^= 1,
            4 => c.players[0].availability.condition_signed ^= 1,
            5 => c.players[0].availability.status_bits ^= 1,
            6 => c.players[0].age += 1,
            7 => c
                .exchange(Location::First(17), Location::Reserve(0))
                .unwrap(),
            _ => c
                .exchange(Location::First(10), Location::First(11))
                .unwrap(),
        }
        c.validate().unwrap();
        let before = c.clone();
        let message = c.match_ticket().err().unwrap_or_default();
        assert!(
            message.contains("Sevilla") && message.contains("Undo"),
            "edit {edit}: {message:?}"
        );
        assert_eq!(c, before);
        let loaded = Campaign::decode(&c.encode().unwrap()).unwrap();
        assert_eq!(loaded, before);
        assert_eq!(loaded.match_ticket().err().unwrap(), message);
    }
}
#[test]
fn unmodified_reverted_and_reordered_player_storage_still_admit() {
    let original = week_one();
    assert!(original.match_ticket().unwrap().is_sevilla());
    let mut reverted = original.clone();
    reverted
        .exchange(Location::First(1), Location::First(2))
        .unwrap();
    reverted
        .exchange(Location::First(1), Location::First(2))
        .unwrap();
    assert_ne!(reverted.revision, original.revision);
    assert!(reverted.match_ticket().unwrap().is_sevilla());
    reverted.players.reverse();
    assert!(reverted.match_ticket().unwrap().is_sevilla());
    let loaded = Campaign::decode(&reverted.encode().unwrap()).unwrap();
    assert!(loaded.match_ticket().unwrap().is_sevilla());
}
