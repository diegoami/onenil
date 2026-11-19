//! [designed:T94] Every witnessed entry point refuses ignored squad/settings edits.
use onenil_campaign::{Campaign, Location};
use onenil_desktop::{
    campaign_match::{self, Profile},
    controller::{Controller, Key, Screen},
    timeline::LiveWitnessed,
};
fn week_one() -> Campaign {
    let mut c = Campaign::initial().unwrap();
    let ticket = c.match_ticket().unwrap();
    c.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    c.record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    c
}
fn edited(kind: usize) -> Campaign {
    let mut c = week_one();
    match kind {
        0 => c.exchange(Location::First(1), Location::First(2)).unwrap(),
        1 => c.set_position(2582, 3).unwrap(),
        2 => c.set_instructions(2582, 0).unwrap(),
        _ => c.club.team_instructions ^= 1,
    }
    c
}
#[test]
fn profile_refuses_unsupported_setups_after_reload() {
    for kind in 0..4 {
        let c = Campaign::decode(&edited(kind).encode().unwrap()).unwrap();
        let message = campaign_match::profile(&c).err().unwrap_or_default();
        assert!(
            message.contains("Sevilla") && message.contains("Undo"),
            "edit {kind}: {message:?}"
        );
    }
}
#[test]
fn witnessed_session_refuses_unsupported_setups() {
    for kind in 0..4 {
        let c = edited(kind);
        let message = LiveWitnessed::new(&c).err().unwrap_or_default();
        assert!(
            message.contains("Sevilla") && message.contains("Undo"),
            "edit {kind}: {message:?}"
        );
    }
}
#[test]
fn real_preparation_refuses_before_a_worker_request_without_resetting_edits() {
    for kind in 0..4 {
        for reload in [false, true] {
            let c = edited(kind);
            let c = if reload {
                Campaign::decode(&c.encode().unwrap()).unwrap()
            } else {
                c
            };
            let mut controller = Controller::default();
            controller.editor.campaign = c.clone();
            controller.key(Key::Enter);
            assert_eq!(controller.screen, Screen::Preparation);
            controller.key(Key::Enter);
            assert_eq!(controller.screen, Screen::Preparation, "edit {kind}");
            assert!(!controller.prepare_requested);
            assert!(controller.message.contains("Sevilla") && controller.message.contains("Undo"));
            assert_eq!(controller.editor.campaign, c);
        }
    }
}
#[test]
fn unchanged_and_reverted_setups_keep_the_witnessed_route() {
    let original = week_one();
    let mut reverted = original.clone();
    reverted
        .exchange(Location::First(1), Location::First(2))
        .unwrap();
    reverted
        .exchange(Location::First(1), Location::First(2))
        .unwrap();
    for c in [original, reverted] {
        assert_eq!(campaign_match::profile(&c).unwrap(), Profile::Sevilla);
        assert!(LiveWitnessed::new(&c).is_ok());
        let mut controller = Controller::default();
        controller.editor.campaign = c;
        controller.key(Key::Enter);
        controller.key(Key::Enter);
        assert_eq!(controller.screen, Screen::Preparing);
        assert!(controller.prepare_requested);
    }
}
#[test]
fn worker_spawn_refuses_before_touching_the_executable() {
    for kind in 0..4 {
        let c = edited(kind);
        let result = onenil_desktop::live_worker::LiveJob::spawn_campaign(
            std::path::Path::new("/this-executable-must-not-be-started"),
            1,
            &c,
        );
        let message = result.err().unwrap_or_default();
        assert!(
            message.contains("Sevilla") && message.contains("Undo"),
            "edit {kind}: {message:?}"
        );
    }
}
