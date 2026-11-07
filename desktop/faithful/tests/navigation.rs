use onenil_desktop::controller::{Controller, Key, Screen};
use onenil_desktop::editor::{Location, Panel};
#[test]
fn original_t69_keyboard_trace_matches_visible_ui_states() {
    use serde_json::{json, Value};
    use sha2::{Digest, Sha256};
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../records/T66/source-r0001/admission-r0002.json");
    let raw = std::fs::read(source).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&raw)),
        "45ca77eda64349d90700eb3983f6c725b5308e0dcf87c8a788bb081fde3ab627"
    );
    let record: Value = serde_json::from_slice(&raw).unwrap();
    let mut c = Controller::default();
    for step in record["payload"]["ui"].as_array().unwrap() {
        if let Some(key) = step["key"].as_str() {
            c.key(match key {
                "space" => Key::Space,
                "Down" => Key::Down,
                "Return" => Key::Enter,
                "Escape" => Key::Escape,
                "Up" => Key::Up,
                _ => panic!("unmapped original key {key}"),
            });
        }
        let stage = step["stage"].as_str().unwrap();
        assert_eq!(
            json!(c.editor.campaign.club.first_team.map(|id| id.unwrap_or(0))),
            step["roster"],
            "{stage} roster"
        );
        assert_eq!(
            json!(c.editor.carried_player().is_some()),
            step["put_here"],
            "{stage} carry"
        );
        if let Some(menu) = step["menu"].as_str() {
            assert_eq!(c.screen, Screen::Main, "{stage} screen");
            let index = match menu {
                "PLAY NEXT GAME" => 0,
                "Players" => 1,
                _ => panic!("unknown menu {menu}"),
            };
            assert_eq!(c.menu, index, "{stage} menu");
        } else {
            assert_eq!(c.screen, Screen::Players, "{stage} screen");
            assert_eq!(
                json!(c.editor.card()),
                step["selected_player"],
                "{stage} player"
            );
        }
    }
}

#[test]
fn players_shortcut_reopens_players_and_preserves_preparation_caller() {
    let mut c = Controller::default();
    c.key(Key::Space);
    c.key(Key::Escape);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Players);
    c.key(Key::Escape);
    c.key(Key::Up);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparation);
    c.key(Key::Space);
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Preparation);
    assert_eq!(c.menu, 0);
}
#[test]
fn remembered_view_and_caller_and_stale_results() {
    let mut c = Controller::default();
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::Players);
    c.key(Key::Insert);
    c.key(Key::Enter);
    c.key(Key::Down);
    assert_eq!(c.editor.carried_player(), Some(2581));
    assert_eq!(c.editor.card(), Some(2582));
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Players);
    assert_eq!(c.editor.card(), Some(2582));
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Main);
    c.key(Key::Up);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparation);
    c.key(Key::Space);
    assert_eq!(c.editor.panel, Panel::Details);
    assert_eq!(c.editor.selected, Location::First(1));
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Preparation);
    c.key(Key::Exhibition);
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
