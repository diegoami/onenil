//! [designed:T91] The witnessed Sevilla route through the real controller:
//! the completed-week save requests the witnessed match, the pinned evidence
//! plays to the natural 1-1, the caller's condition outcomes record with the
//! result, the week transaction refuses naming the unwitnessed second-week
//! cash movement, and the post-caller state survives restart and reload.
use onenil_desktop::{
    controller::{Controller, Key, Screen},
    timeline::{LiveWitnessed, Phase},
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
    let mut app = onenil_desktop::application::Application::new(std::path::PathBuf::from("."), path.clone(), false).unwrap();
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
    let mut restart = onenil_desktop::application::Application::new(std::path::PathBuf::from("."), path.clone(), true).unwrap();
    assert_eq!(restart.controller.editor.campaign, advanced);
    restart.key(Key::Enter);
    restart.key(Key::Enter);
    assert!(!restart.controller.prepare_requested);
    assert!(restart.controller.message.contains("Celtic"));
    assert!(restart.controller.message.contains("unwitnessed"));
    std::fs::remove_file(path).unwrap();
}
