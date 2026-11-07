use onenil_desktop::controller::{Controller, Key, Screen};

#[test]
fn actual_supported_campaign_starts_and_cancels_without_becoming_exhibition() {
    let mut controller = Controller::default();
    let before = controller.editor.campaign.clone();
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparation);
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparing);
    assert!(controller.prepare_requested);
    assert!(!controller.exhibition);
    let generation = controller.job_id;
    controller.key(Key::Escape);
    assert_eq!(controller.screen, Screen::Preparation);
    assert!(!controller.exhibition);
    assert!(!controller.prepare_requested);
    assert!(!controller.accept_error(generation, "late failure".into()));
    assert_eq!(controller.editor.campaign, before);
}
