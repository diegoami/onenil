//! [confirmed] Research W015/W016 and 0AB9:ABD9/AC52/B271 at68cd3396.
use onenil_campaign::Campaign;
#[test]
fn original_position_guard_cycle_and_club_restore() {
    let mut c = Campaign::initial().unwrap();
    let initial = c.clone();
    assert!(c.reposition(2598).is_err());
    assert_eq!(c, initial);
    c.restore_club_position(2598).unwrap();
    assert_eq!(c, initial);
    for position in [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0, 1, 2] {
        let before = c.clone();
        c.reposition(2582).unwrap();
        let mut expected = before;
        expected.players[1].current_position = position;
        expected.revision += 1;
        assert_eq!(c, expected);
    }
    c.reposition(2582).unwrap();
    let before = c.clone();
    c.restore_club_position(2582).unwrap();
    let mut expected = before;
    expected.players[1].current_position = 2;
    expected.revision += 1;
    assert_eq!(c, expected);
    let unchanged = c.clone();
    c.restore_club_position(2582).unwrap();
    assert_eq!(c, unchanged);
    // AC52's age guard: no original club-position restore for under18s.
    c.players[1].age = 17;
    c.set_position(2582, 3).unwrap();
    let before = c.clone();
    assert!(c.restore_club_position(2582).is_err());
    assert_eq!(c, before);
}
#[test]
fn exact_goalkeeper_cycle_and_unrelated_bits() {
    let mut c = Campaign::initial().unwrap();
    for observed in [1, 16, 17, 1, 16] {
        c.cycle_goalkeeper_positioning(2598).unwrap();
        assert_eq!(c.players[17].individual_instructions, observed);
    }
    for club_choice in [0u8, 1] {
        for bits in 0u8..=255 {
            let mut c = Campaign::initial().unwrap();
            c.club.team_instructions = (c.club.team_instructions & !1) | club_choice;
            c.players[17].individual_instructions = bits;
            let before = c.clone();
            c.cycle_goalkeeper_positioning(2598).unwrap();
            let result = c.players[17].individual_instructions;
            assert_eq!(result & !0x11, bits & !0x11);
            let pair = match bits & 0x11 {
                0 | 1 => 16,
                16 => 17,
                17 => club_choice,
                _ => unreachable!(),
            };
            assert_eq!(result & 0x11, pair);
            let mut expected = before;
            expected.players[17].individual_instructions = result;
            expected.revision += 1;
            assert_eq!(c, expected);
        }
    }
    let mut c = Campaign::initial().unwrap();
    let before = c.clone();
    assert!(c.cycle_goalkeeper_positioning(2582).is_err());
    assert_eq!(c, before);
    c.revision = u64::MAX;
    let before = c.clone();
    assert!(c.reposition(2582).is_err());
    assert!(c.cycle_goalkeeper_positioning(2598).is_err());
    assert_eq!(c, before);
}
