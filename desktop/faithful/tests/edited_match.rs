use onenil_desktop::controller::{Controller, Key, Screen};

fn edited_controller() -> Controller {
    let mut c = Controller::default();
    for key in [Key::Space, Key::Down, Key::Enter, Key::Down, Key::Enter, Key::Escape, Key::Up] {
        c.key(key);
    }
    assert_eq!(c.screen, Screen::Main);
    assert_eq!(c.menu, 0);
    assert_eq!(c.editor.campaign.club.first_team[1..3], [Some(2583), Some(2582)]);
    c
}

#[test]
fn original_keyboard_edit_starts_owned_desktop_preparation() {
    let mut c = edited_controller();
    let before = c.editor.campaign.clone();
    onenil_desktop::campaign_match::admission(&before).unwrap();
    c.key(Key::Enter); c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparing);
    assert!(c.prepare_requested);
    assert!(!c.exhibition);
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Preparation);
    assert_eq!(c.editor.campaign, before);
}
