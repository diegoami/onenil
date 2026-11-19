//! [confirmed:archaeology 788b833 T93] Real controller week completion,
//! automatic native save and the continuing refusal of Friendly #3.
use onenil_desktop::{
    controller::{Controller, Key, Screen},
    timeline::LiveWitnessed,
};
use serde_json::Value;
#[allow(dead_code)]
fn witnessed_conditions() -> Vec<(u16, i8)> {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../campaign/fixtures/sevilla-post-match.json"
    ))
    .unwrap();
    fixture["witnessed"]["conditions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["player"].as_u64().unwrap() as u16,
                row["condition_after"].as_i64().unwrap() as i8,
            )
        })
        .collect()
}
fn sevilla_campaign() -> onenil_campaign::Campaign {
    let mut c = onenil_campaign::Campaign::initial().unwrap();
    let ticket = c.match_ticket().unwrap();
    c.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    c.record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    c
}
fn preparing() -> Controller {
    let mut c = Controller::default();
    c.editor.campaign = sevilla_campaign();
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparing);
    assert!(c.prepare_requested);
    c
}
fn half_time(c: &mut Controller) {
    for _ in 0..10000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    assert_eq!(c.screen, Screen::HalfTime);
}
fn full_time(c: &mut Controller) {
    for _ in 0..20000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    assert_eq!(c.screen, Screen::FullTime);
}
#[test]
fn sevilla_week_route_automatically_saves_and_refuses_the_third_match() {
    let path = std::env::temp_dir().join(format!("T93-route-{}.json", std::process::id()));
    let mut app = onenil_desktop::application::Application::new(
        std::path::PathBuf::from("."),
        path.clone(),
        false,
    )
    .unwrap();
    app.controller = preparing();
    let mut engine = LiveWitnessed::new(&app.controller.editor.campaign).unwrap();
    let prefix = engine.pause().unwrap();
    let id = app.controller.job_id;
    assert!(app.controller.accept_interval(id, prefix));
    half_time(&mut app.controller);
    app.controller.key(Key::Enter);
    app.controller.key(Key::Enter);
    let completion = engine.finish(false).unwrap();
    assert!(app.controller.accept_continuation(id, completion));
    full_time(&mut app.controller);
    app.key(Key::Enter);
    assert_eq!(app.controller.screen, Screen::CashFlow);
    let recorded = app.controller.editor.campaign.clone();
    assert_eq!(recorded.week_index, 1);
    assert!(!path.exists());
    app.key(Key::Space);
    assert_eq!(app.controller.screen, Screen::TransferNews);
    app.key(Key::Space);
    assert_eq!(app.controller.screen, Screen::Main);
    let advanced = app.controller.editor.campaign.clone();
    assert_eq!(advanced.cash, 421_631);
    assert_eq!(advanced.week_index, 2);
    assert_eq!(advanced.players, recorded.players);
    assert_eq!(advanced.revision, recorded.revision + 1);
    assert_eq!(onenil_campaign::Campaign::load(&path).unwrap(), advanced);
    let mut restart = onenil_desktop::application::Application::new(
        std::path::PathBuf::from("."),
        path.clone(),
        true,
    )
    .unwrap();
    assert_eq!(restart.controller.editor.campaign, advanced);
    restart.key(Key::Enter);
    restart.key(Key::Enter);
    assert!(!restart.controller.prepare_requested);
    assert!(restart.controller.message.contains("Celtic"));
    assert!(restart.controller.message.contains("unwitnessed"));
    std::fs::remove_file(path).unwrap();
}

fn post_caller() -> onenil_campaign::Campaign {
    let mut c = sevilla_campaign();
    let ticket = c.match_ticket().unwrap();
    c.record_sevilla_result(&ticket, [1, 1], "0AB9:EBE2", &witnessed_conditions())
        .unwrap();
    c
}
#[test]
fn automatic_save_failure_is_visible_and_the_completed_state_can_be_saved_again() {
    let root = std::env::temp_dir().join(format!("T93-save-failure-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let parent = root.join("blocked");
    std::fs::write(&parent, b"not a directory").unwrap();
    let path = parent.join("SAVE1.json");
    let mut app = onenil_desktop::application::Application::new(
        std::path::PathBuf::from("."),
        path.clone(),
        false,
    )
    .unwrap();
    app.controller.editor.campaign = post_caller();
    app.controller.screen = Screen::TransferNews;
    app.key(Key::Space);
    assert_eq!(app.controller.editor.campaign.week_index, 2);
    assert!(app.controller.message.contains("Save refused"));
    assert!(!path.exists());
    std::fs::remove_file(&parent).unwrap();
    app.key(Key::F7);
    assert_eq!(
        onenil_campaign::Campaign::load(&path).unwrap(),
        app.controller.editor.campaign
    );
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
#[cfg(feature = "raster")]
fn screens_show_the_sevilla_receipt_and_celtic_advertisement() {
    use onenil_desktop::render::Renderer;
    let font = std::env::var_os("ONENIL_DESKTOP_FONT").expect("verified font path");
    let mut r = Renderer::new(std::path::Path::new(&font)).unwrap();
    let mut c = Controller::default();
    c.editor.campaign = post_caller();
    c.screen = Screen::CashFlow;
    r.draw(&c);
    let text = r.screen_text();
    for expected in [
        "69,410",
        "69,270",
        "421,631",
        "57,320",
        "4,200",
        "7,750",
        "2,920,000",
        "TV coverage",
        "Radio coverage",
        "Players sold",
        "Sponsorship",
        "Directors hand-out",
        "Players bought",
        "Contracts",
        "Interest",
        "Sundries",
    ] {
        assert!(text.contains(expected), "{expected}: {text}");
    }
    assert!(!text.contains("38,761"));
    c.key(Key::Space);
    r.draw(&c);
    let text = r.screen_text();
    for expected in [
        "G.Popescu",
        "Tottenham H",
        "Chelsea",
        "S.Smith",
        "L.Howarth",
        "J.Byrne",
        "K.Ing'risten",
    ] {
        assert!(text.contains(expected), "{expected}: {text}");
    }
    assert!(!text.contains("M.Kree"));
    c.key(Key::Space);
    r.draw(&c);
    let text = r.screen_text();
    for expected in ["Celtic", "Friendly #3", "away", "421,631"] {
        assert!(text.contains(expected), "{expected}: {text}");
    }
    assert!(!text.contains("Aragones"));
    assert!(!text.contains("Fernandez"));
}
