//! [designed] Actual computed native completion must own its campaign result.
use onenil_desktop::{
    controller::{Controller, Key, Screen},
    timeline::{self, Timeline},
};
use std::sync::OnceLock;
fn actual() -> Timeline {
    static MATCH: OnceLock<Timeline> = OnceLock::new();
    MATCH
        .get_or_init(|| {
            timeline::compute_campaign(&onenil_campaign::Campaign::initial().unwrap()).unwrap()
        })
        .clone()
}
fn preparing() -> Controller {
    let mut c = Controller::default();
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparing);
    c
}
fn finish(c: &mut Controller) {
    for _ in 0..5000 {
        match c.screen {
            Screen::Pitch => c.tick(),
            Screen::HalfTime => {
                c.key(Key::Enter);
                c.key(Key::Enter);
            }
            Screen::FullTime | Screen::Bench => return,
            _ => panic!("unexpected screen {:?}", c.screen),
        }
    }
    panic!("native playback did not terminate");
}
#[test]
fn actual_result_posts_only_on_acknowledgement_and_cannot_replay() {
    let mut c = preparing();
    let before = c.editor.campaign.clone();
    let timeline = actual();
    let score = timeline
        .frames
        .last()
        .unwrap()
        .view
        .score
        .map(|s| u16::try_from(s).unwrap());
    assert!(c.accept_timeline(c.job_id, timeline.clone()));
    finish(&mut c);
    assert_eq!(c.screen, Screen::FullTime);
    assert_eq!(c.editor.campaign, before);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Main);
    let recorded = c.editor.campaign.clone();
    assert_eq!(recorded.played_fixture.as_ref().unwrap().score, score);
    assert_eq!(recorded.revision, before.revision + 1);
    assert_eq!(recorded.cash, before.cash);
    assert_eq!(recorded.week_index, before.week_index);
    assert!(c.message.contains("recorded") && c.message.len() <= 80);
    assert!(!c.accept_timeline(c.job_id, timeline));
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparation);
    assert!(!c.prepare_requested);
    assert!(c.message.contains("already") && c.message.len() <= 80);
    assert_eq!(c.editor.campaign, recorded);
}
#[test]
fn cancel_stale_foreign_and_unfinished_outputs_never_record() {
    let mut c = preparing();
    let id = c.job_id;
    let before = c.editor.campaign.clone();
    c.key(Key::Escape);
    assert!(!c.accept_timeline(id, actual()));
    assert_eq!(c.editor.campaign, before);
    for damage in ["clubs", "return", "empty"] {
        let mut c = preparing();
        let mut t = actual();
        match damage {
            "clubs" => t.clubs = vec![130, 134],
            "return" => t.native_return.clear(),
            _ => t.frames.clear(),
        }
        c.accept_timeline(c.job_id, t);
        assert_eq!(c.screen, Screen::Preparation, "{damage}");
        assert_eq!(c.editor.campaign, before);
    }
    let mut c = preparing();
    c.accept_timeline(c.job_id, actual());
    finish(&mut c);
    c.editor.campaign.set_instructions(2598, 16).unwrap();
    let changed = c.editor.campaign.clone();
    c.key(Key::Enter);
    assert_eq!(c.editor.campaign, changed);
    assert!(c.message.contains("not recorded"));
    let mut c = preparing();
    let mut t = actual();
    for frame in &mut t.frames {
        frame.events.retain(|e| e.kind != "full-time");
    }
    c.accept_timeline(c.job_id, t);
    finish(&mut c);
    assert_eq!(c.screen, Screen::Bench);
    assert_eq!(c.editor.campaign, before);
}
#[test]
fn exhibition_and_missing_request_do_not_post_campaign_results() {
    let mut c = Controller::default();
    let before = c.editor.campaign.clone();
    c.screen = Screen::FullTime;
    c.key(Key::Enter);
    assert_eq!(c.editor.campaign, before);
    c.key(Key::Exhibition);
    c.key(Key::Enter);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    c.accept_timeline(c.job_id, timeline::compute(root).unwrap());
    finish(&mut c);
    assert_eq!(c.screen, Screen::FullTime);
    c.key(Key::Enter);
    assert_eq!(c.editor.campaign, before);
    assert!(c.message.contains("Exhibition"));
}
