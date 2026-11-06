//! [confirmed] W006 list keys; [designed] native discovery and cancel/atomic target.
use onenil_campaign::Campaign;
use onenil_desktop::{
    application::Application,
    controller::{Key, Screen},
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "onenil-chooser-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn app(&self) -> Application {
        Application::new(self.0.clone(), self.0.join("SAVE1.json"), false).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
#[test]
fn empty_single_and_direct_load() {
    let f = Fixture::new();
    let mut a = f.app();
    let before = a.snapshot();
    a.choose_save();
    assert_eq!(a.controller.screen, Screen::Main);
    assert_eq!(a.snapshot()["campaign"], before["campaign"]);
    assert!(a.controller.message.contains("No native"));
    let mut saved = Campaign::initial().unwrap();
    saved.set_position(2582, 1).unwrap();
    saved.save(&f.0.join("SAVE1.json")).unwrap();
    a.choose_save();
    assert_eq!(a.controller.editor.campaign, saved);
    assert_eq!(a.controller.screen, Screen::Main);
    let direct = Application::new(f.0.clone(), f.0.join("SAVE1.json"), true).unwrap();
    assert_eq!(direct.controller.editor.campaign, saved);
}

#[test]
fn browsing_and_refusal_keep_state_and_save_target() {
    let f = Fixture::new();
    let mut a = f.app();
    let initial = a.controller.editor.campaign.clone();
    initial.save(&f.0.join("SAVE1.json")).unwrap();
    let mut changed = initial.clone();
    changed.set_position(2582, 1).unwrap();
    changed.save(&f.0.join("MANUAL.json")).unwrap();
    let old_target = a.save_path.clone();
    a.choose_save();
    assert_eq!(a.controller.screen, Screen::SavedGames);
    assert_eq!(a.controller.load_entries, vec!["MANUAL", "SAVE1"]);
    assert_eq!(a.controller.load_index, 0);
    a.key(Key::Up);
    assert_eq!(a.controller.load_index, 0); // Native list-edge choice, not a DOS claim.
    assert_eq!(a.controller.editor.campaign, initial);
    assert_eq!(a.save_path, old_target);
    a.key(Key::Down);
    a.key(Key::Down);
    assert_eq!(a.controller.load_index, 1);
    assert_eq!(a.controller.editor.campaign, initial);
    assert_eq!(a.save_path, old_target);
    a.key(Key::Escape);
    assert_eq!(a.controller.screen, Screen::SavedGames);
    assert_eq!(a.controller.load_index, 1);
    assert_eq!(a.controller.editor.campaign, initial);
    assert_eq!(a.save_path, old_target);
    a.key(Key::Up);
    a.key(Key::F9);
    assert_eq!(a.controller.screen, Screen::Main);
    assert_eq!(a.save_path, old_target);
    assert_eq!(a.controller.editor.campaign, initial);
    for bad in [
        "{".to_owned(),
        String::from_utf8(changed.encode().unwrap())
            .unwrap()
            .replace("\"version\": 1", "\"version\": 9"),
        String::from_utf8(changed.encode().unwrap())
            .unwrap()
            .replace("ENGL-95", "OTHER"),
    ] {
        std::fs::write(f.0.join("MANUAL.json"), bad).unwrap();
        a.choose_save();
        a.key(Key::Enter);
        assert_eq!(a.controller.screen, Screen::SavedGames);
        assert_eq!(a.controller.load_index, 0);
        assert!(a.controller.message.contains("refused"));
        assert_eq!(a.save_path, old_target);
        assert_eq!(a.controller.editor.campaign, initial);
        a.key(Key::F9);
    }
    changed.save(&f.0.join("MANUAL.json")).unwrap();
    a.choose_save();
    a.key(Key::Enter);
    assert_eq!(a.controller.editor.campaign, changed);
    assert_eq!(a.save_path, f.0.join("MANUAL.json"));
    assert_eq!(a.controller.game_name, "MANUAL");
    a.controller.editor.campaign.set_position(2582, 3).unwrap();
    a.key(Key::F7);
    assert_eq!(
        Campaign::load(&f.0.join("MANUAL.json")).unwrap(),
        a.controller.editor.campaign
    );
    assert_eq!(Campaign::load(&old_target).unwrap(), initial);
}
