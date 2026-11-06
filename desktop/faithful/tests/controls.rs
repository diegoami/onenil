use onenil_desktop::{
    controller::{Controller, Key, Screen},
    editor::{Location, Panel},
};
#[test]
fn original_controls_and_return_destinations() {
    let mut c = Controller::default();
    c.key(Key::Space);
    c.editor.select(Location::First(17)).unwrap();
    c.key(Key::Insert);
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::Options);
    let before = c.editor.campaign.clone();
    c.key(Key::Enter);
    assert_eq!(c.editor.campaign, before);
    assert!(c.message.contains("unavailable"));
    for _ in 0..4 {
        c.key(Key::Down);
    }
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Instructions);
    assert_eq!(c.instruction, 0);
    assert_eq!(c.editor.panel, Panel::Details);
    for value in [1, 16, 17, 1, 16] {
        c.key(Key::Enter);
        assert_eq!(
            c.editor.player(2598).unwrap().individual_instructions,
            value
        );
    }
    let before = c.editor.campaign.clone();
    c.key(Key::Down);
    c.key(Key::Enter);
    assert!(c.message.contains("unavailable"));
    assert_eq!(c.editor.campaign, before);
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Options);
    assert_eq!(c.option, 4);
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Players);
    c.editor.select(Location::First(1)).unwrap();
    c.key(Key::Space);
    assert_eq!(c.option, 4);
    for _ in 0..4 {
        c.key(Key::Up);
    }
    for _ in 0..13 {
        c.key(Key::Enter);
    }
    assert_eq!(c.editor.player(2582).unwrap().current_position, 1);
    c.key(Key::Down);
    c.key(Key::Enter);
    assert_eq!(c.editor.player(2582).unwrap().current_position, 2);
    assert_eq!(c.editor.player(2582).unwrap().club_position, 2);
    c.key(Key::Down);
    let before = c.editor.campaign.clone();
    c.key(Key::Enter);
    assert_eq!(c.editor.campaign, before);
    assert!(c.message.contains("unavailable"));
}

#[test]
fn options_force_details_and_insert_returns_from_instructions() {
    let mut c = Controller::default();
    c.key(Key::Space);
    assert_eq!(c.editor.panel, Panel::Skills);
    c.key(Key::Space);
    assert_eq!(c.editor.panel, Panel::Details);
    let before = c.editor.campaign.clone();
    c.key(Key::Insert);
    assert_eq!(c.screen, Screen::Options);
    assert_eq!(c.editor.panel, Panel::Skills);
    for _ in 0..4 {
        c.key(Key::Down);
    }
    c.key(Key::Enter);
    assert_eq!(c.editor.panel, Panel::Details);
    c.key(Key::Down);
    c.key(Key::Escape);
    c.key(Key::Enter);
    assert_eq!(c.instruction, 1);
    c.key(Key::Insert);
    assert_eq!(c.screen, Screen::Options);
    assert_eq!(c.editor.panel, Panel::Skills);
    assert_eq!(c.editor.campaign, before);
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Players);
    assert_eq!(c.editor.panel, Panel::Skills);
}
