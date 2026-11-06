use onenil_desktop::controller::{Controller, Key, Screen};
use onenil_desktop::editor::{Location, Panel};
#[test]
fn remembered_view_and_caller_and_stale_results() {
    let mut c = Controller::default();
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::Players);
    c.key(Key::Insert);
    c.key(Key::Enter);
    c.key(Key::Down);
    assert_eq!(c.editor.carried_player(), Some(0));
    assert_eq!(c.editor.card(), Some(1));
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Players);
    assert_eq!(c.editor.card(), Some(1));
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Main);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparation);
    c.key(Key::Space);
    assert_eq!(c.editor.panel, Panel::Details);
    assert_eq!(c.editor.selected, Location::First(1));
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Preparation);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparing);
    assert!(c.prepare_requested);
    let before = c.editor.clone();
    let generation = c.job_id;
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Preparation);
    assert_ne!(c.job_id, generation);
    assert!(!c.accept_error(generation, "late failure".into()));
    assert_eq!(c.editor, before);
}
