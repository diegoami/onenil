//! [designed:T73] Synthetic presentation fixtures, never engine observations.
use onenil_desktop::{
    controller::{Controller, Key, Screen},
    timeline::{Playback, Timeline},
};
fn history(count: usize) -> Controller {
    let view = serde_json::json!({"clock":12,"score":[0,0],"ball":{"x":0,"y":0},"players":[]});
    let events:Vec<_>=(0..count).map(|i|serde_json::json!({"id":i,"native":{"minute":i},"kind":"test","text":format!("Test player action {i}")})).collect();
    let timeline:Timeline=serde_json::from_value(serde_json::json!({
        "club_names":["Test A","Test B"],"initial":view,"frames":[{"view":view,"events":[{"id":999,"native":{"minute":99},"kind":"test","text":"FUTURE UNREACHED"}]}],
        "setup_events":events,"native_return":"0AB9:EBE2","draws":0,"clock_rows":0,"seed":0,"clubs":[130,17],"marker_rows":[[],[]]
    })).unwrap();
    let mut p = Playback::new(timeline);
    p.start_or_continue().unwrap();
    let mut c = Controller::default();
    c.screen = Screen::Pitch;
    c.playback = Some(p);
    c.key(Key::Space);
    c
}
#[cfg(feature = "raster")]
#[test]
fn history_up_changes_visible_native_action_text() {
    let font = std::env::var_os("ONENIL_DESKTOP_FONT").expect("verified replacement font");
    let mut r = onenil_desktop::render::Renderer::new(std::path::Path::new(&font)).unwrap();
    let mut c = history(45);
    r.draw(&c);
    let latest = r.pixels.clone();
    c.key(Key::Up);
    r.draw(&c);
    assert_ne!(
        &r.pixels[5 * 16 * 720..23 * 16 * 720],
        &latest[5 * 16 * 720..23 * 16 * 720],
        "Up must reveal earlier history text"
    );
}

#[test]
fn all_history_controls_clamp_and_preserve_paused_match() {
    let mut c = history(60);
    let campaign = c.editor.campaign.clone();
    let p = c.playback.as_ref().unwrap();
    let (cursor, phase, events) = (p.cursor, p.phase, p.events.clone());
    assert_eq!(c.history_top, 42);
    assert_eq!(
        c.visible_history().first().unwrap(),
        "42 Test player action 42"
    );
    for (key, top) in [
        (Key::Up, 41),
        (Key::PageUp, 23),
        (Key::Down, 24),
        (Key::PageDown, 42),
        (Key::Home, 0),
        (Key::Up, 0),
        (Key::PageUp, 0),
        (Key::Down, 1),
        (Key::PageDown, 19),
        (Key::End, 42),
        (Key::Down, 42),
        (Key::PageDown, 42),
    ] {
        c.key(key);
        assert_eq!(c.history_top, top);
        c.tick();
        assert_eq!(c.playback.as_ref().unwrap().cursor, cursor);
        assert!(!c.history_lines().join("\n").contains("FUTURE"));
    }
    c.key(Key::Home);
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Pitch);
    let p = c.playback.as_ref().unwrap();
    assert_eq!((p.cursor, p.phase, &p.events), (cursor, phase, &events));
    c.key(Key::Space);
    assert_eq!(c.history_top, 42);
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::Pitch);
    assert_eq!(c.editor.campaign, campaign);
}

#[test]
fn empty_short_and_wrapped_histories_have_visible_reachable_edges() {
    for count in [0, 1, 18] {
        let mut c = history(count);
        for key in [
            Key::Up,
            Key::Down,
            Key::PageUp,
            Key::PageDown,
            Key::Home,
            Key::End,
        ] {
            c.key(key);
            assert_eq!(c.history_top, 0);
            assert_eq!(c.visible_history().len(), count);
        }
    }
    let mut c = history(1);
    c.playback.as_mut().unwrap().events[0].text = "wrap ".repeat(800);
    c.key(Key::End);
    let lines = c.history_lines();
    assert!(lines.len() > 18 && lines.iter().all(|line| line.chars().count() <= 77));
    assert_eq!(c.history_top, lines.len() - 18);
    c.key(Key::Home);
    assert_eq!(c.visible_history(), lines[..18]);
    c.key(Key::PageDown);
    assert_eq!(c.visible_history(), lines[18..36]);
    // No discarded suffix when even a single name/token exceeds a text row.
    c.playback.as_mut().unwrap().events[0].text = "é".repeat(180);
    c.key(Key::Home);
    let lines = c.history_lines();
    assert!(lines.iter().all(|line| line.chars().count() <= 77));
    assert_eq!(lines.join("").matches('é').count(), 180);
}

#[test]
fn actual_native_history_resumes_without_revealing_future_actions() {
    let timeline =
        onenil_desktop::timeline::compute_campaign(&onenil_campaign::Campaign::initial().unwrap())
            .unwrap();
    let mut c = Controller::default();
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert!(c.accept_timeline(c.job_id, timeline));
    for _ in 0..2000 {
        if c.playback.as_ref().unwrap().events.len() >= 70 {
            break;
        }
        assert_eq!(c.screen, Screen::Pitch);
        c.tick();
    }
    assert!(c.playback.as_ref().unwrap().events.len() >= 70);
    let campaign = c.editor.campaign.clone();
    let p = c.playback.as_ref().unwrap();
    let (cursor, phase, events) = (p.cursor, p.phase, p.events.clone());
    c.key(Key::Space);
    c.key(Key::Home);
    assert_eq!(c.history_lines(), onenil_desktop::history::lines(&events));
    for key in [Key::PageDown, Key::End, Key::Up, Key::Escape] {
        c.key(key);
    }
    let p = c.playback.as_ref().unwrap();
    assert_eq!((p.cursor, p.phase, &p.events), (cursor, phase, &events));
    c.tick();
    assert!(c.playback.as_ref().unwrap().cursor > cursor);
    assert_eq!(c.editor.campaign, campaign);
}
