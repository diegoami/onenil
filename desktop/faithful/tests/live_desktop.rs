//! [designed:T77] Incomplete playback, request ownership and real desktop worker flow.
use onenil_desktop::{
    application::Application,
    controller::{Controller, Key, Screen},
    timeline::{Completion, IntervalTimeline, LiveCampaign, Phase},
};
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
fn campaign(edited: bool) -> onenil_campaign::Campaign {
    let mut c = onenil_campaign::Campaign::initial().unwrap();
    if edited {
        c.exchange(
            onenil_campaign::Location::First(1),
            onenil_campaign::Location::First(2),
        )
        .unwrap();
    }
    c
}
fn preparing(edited: bool) -> Controller {
    let mut c = Controller::default();
    c.editor.campaign = campaign(edited);
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparing);
    c
}
fn original() -> (IntervalTimeline, Completion) {
    static DATA: OnceLock<(IntervalTimeline, Completion)> = OnceLock::new();
    DATA.get_or_init(|| {
        let mut e = LiveCampaign::new(&campaign(false)).unwrap();
        let p = e.pause().unwrap();
        (p, e.finish(false).unwrap())
    })
    .clone()
}
fn halftime(c: &mut Controller) {
    for _ in 0..10000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    assert_eq!(c.screen, Screen::HalfTime);
}
fn fulltime(c: &mut Controller) {
    for _ in 0..10000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    assert_eq!(c.screen, Screen::FullTime);
}
#[test]
fn desktop_reached_interval_continuation_preserves_state_and_posts_once() {
    for edited in [false, true] {
        let mut c = preparing(edited);
        let before = c.editor.campaign.clone();
        let mut engine = LiveCampaign::new(&before).unwrap();
        let prefix = engine.pause().unwrap();
        let len = prefix.frames.len();
        assert!(c.accept_interval(c.job_id, prefix));
        assert!(c.playback.is_none());
        assert_eq!(c.match_view().unwrap().cursor, 0);
        for _ in 0..100 {
            c.tick();
        }
        c.key(Key::Space);
        let held = c.match_view().unwrap();
        let cursor = held.cursor;
        let events = held.events.to_vec();
        let view = held.view().clone();
        let markers = held.markers().to_vec();
        for key in [Key::Home, Key::Down, Key::End] {
            c.key(key);
            c.tick();
            assert_eq!(c.match_view().unwrap().cursor, cursor);
            assert_eq!(c.match_view().unwrap().events, events);
        }
        assert_eq!(*c.match_view().unwrap().view(), view);
        assert_eq!(c.match_view().unwrap().markers(), markers);
        c.key(Key::Escape);
        halftime(&mut c);
        assert_eq!(c.match_view().unwrap().cursor, len);
        let reached = c.match_view().unwrap().events.to_vec();
        assert_eq!(c.match_view().unwrap().view().clock, 46);
        assert_eq!(c.editor.campaign, before);
        c.key(Key::Enter);
        c.key(Key::Enter);
        assert!(c.resume_requested && c.is_resuming());
        assert_eq!(c.screen, Screen::Preparing);
        let completed = engine.finish(false).unwrap();
        let stale = completed.clone();
        assert!(!c.accept_continuation(c.job_id + 1, stale));
        assert_eq!(c.match_view().unwrap().events, reached);
        assert!(!c.accept_timeline(c.job_id, completed.timeline.clone()));
        assert!(c.accept_continuation(c.job_id, completed.clone()));
        assert_eq!(c.match_view().unwrap().cursor, len);
        assert_eq!(c.match_view().unwrap().events, reached);
        assert_eq!(c.match_view().unwrap().phase, Phase::SecondHalf);
        assert!(!c.accept_continuation(c.job_id, completed));
        fulltime(&mut c);
        assert_eq!(c.editor.campaign, before);
        c.key(Key::Enter);
        let result = c.editor.campaign.clone();
        assert_eq!(result.revision, before.revision + 1);
        assert!(result.played_fixture.is_some());
        c.screen = Screen::FullTime;
        c.key(Key::Enter);
        assert_eq!(c.editor.campaign, result);
    }
}
#[test]
fn incomplete_malformed_stale_and_changed_requests_never_record_results() {
    let (prefix, complete) = original();
    for damage in 0..5 {
        let mut c = preparing(false);
        let before = c.editor.campaign.clone();
        let mut bad = prefix.clone();
        match damage {
            0 => bad.frames.clear(),
            1 => bad.clubs = vec![130, 134],
            2 => bad.control = 0,
            3 => {
                bad.marker_rows.pop();
            }
            _ => bad.boundary.clear(),
        }
        c.accept_interval(c.job_id, bad);
        assert_eq!(c.screen, Screen::Preparation);
        assert_eq!(c.editor.campaign, before);
        assert!(c.match_view().is_none());
    }
    for damage in 0..6 {
        let mut c = preparing(false);
        let before = c.editor.campaign.clone();
        c.accept_interval(c.job_id, prefix.clone());
        halftime(&mut c);
        c.key(Key::Enter);
        c.key(Key::Enter);
        let mut bad = complete.clone();
        match damage {
            0 => bad.timeline.native_return.clear(),
            1 => bad.timeline.clubs = vec![130, 134],
            2 => {
                bad.timeline.marker_rows.pop();
            }
            3 => bad.timeline.frames[0].view.clock += 1,
            4 => bad.native_sha256.clear(),
            _ => {
                for f in &mut bad.timeline.frames {
                    f.events.retain(|e| e.kind != "full-time");
                }
            }
        }
        c.accept_continuation(c.job_id, bad);
        assert_eq!(c.screen, Screen::Preparation);
        assert_eq!(c.editor.campaign, before);
        assert!(c.editor.campaign.played_fixture.is_none());
    }
    let mut c = preparing(false);
    let id = c.job_id;
    c.key(Key::Escape);
    assert!(!c.accept_interval(id, prefix.clone()));
    let mut c = preparing(false);
    c.accept_interval(c.job_id, prefix.clone());
    c.screen = Screen::FullTime;
    c.key(Key::Enter);
    assert!(c.editor.campaign.played_fixture.is_none());
    let mut c = preparing(false);
    c.accept_interval(c.job_id, prefix);
    halftime(&mut c);
    c.key(Key::Enter);
    c.key(Key::Enter);
    c.editor.campaign.set_instructions(2598, 16).unwrap();
    let changed = c.editor.campaign.clone();
    c.accept_continuation(c.job_id, complete);
    assert_eq!(c.editor.campaign, changed);
    assert!(changed.played_fixture.is_none());
    assert_eq!(c.screen, Screen::Preparation);
}
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "T77-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}
fn exe() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_onenil-desktop"))
}
fn pump(app: &mut Application, check: impl Fn(&Application) -> bool) {
    let start = Instant::now();
    loop {
        app.update_with_executable(exe());
        if check(app) {
            return;
        }
        assert!(
            start.elapsed() < Duration::from_secs(180),
            "{}",
            app.snapshot()
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}
fn started(dir: &Path, edited: bool) -> Application {
    let mut a = Application::new(root(), dir.join("game.json"), false).unwrap();
    a.controller.editor.campaign = campaign(edited);
    a.key(Key::Enter);
    a.key(Key::Enter);
    pump(&mut a, |a| a.controller.screen == Screen::Pitch);
    a
}
fn gone(pid: u32) {
    #[cfg(target_os = "linux")]
    assert!(!Path::new(&format!("/proc/{pid}")).exists());
    #[cfg(not(target_os = "linux"))]
    let _ = pid;
}
#[test]
fn real_application_retains_worker_through_inspection_halftime_and_saved_completion() {
    for edited in [false, true] {
        let temp = Temp::new();
        let mut a = started(&temp.0, edited);
        let before = a.controller.editor.campaign.clone();
        let pid = a.live_job.as_ref().unwrap().pid();
        for _ in 0..100 {
            a.controller.tick();
        }
        a.key(Key::Space);
        let held = a.snapshot();
        for _ in 0..3 {
            a.update_with_executable(exe());
        }
        assert_eq!(a.snapshot()["frames"], held["frames"]);
        assert_eq!(a.snapshot()["recent_actions"], held["recent_actions"]);
        assert_eq!(a.live_job.as_ref().unwrap().pid(), pid);
        a.key(Key::Escape);
        a.key(Key::Enter);
        assert_eq!(a.controller.screen, Screen::Bench);
        a.key(Key::Enter);
        assert_eq!(a.controller.screen, Screen::Pitch);
        assert!(!a.controller.is_resuming());
        assert_eq!(a.live_job.as_ref().unwrap().pid(), pid);
        halftime(&mut a.controller);
        assert!(a.live_job.is_some());
        a.key(Key::Enter);
        let held = a.snapshot();
        a.key(Key::Enter);
        assert!(a.controller.is_resuming());
        assert_eq!(a.snapshot()["frames"], held["frames"]);
        assert_eq!(a.snapshot()["recent_actions"], held["recent_actions"]);
        pump(&mut a, |a| {
            a.controller.screen == Screen::Pitch && !a.controller.is_resuming()
        });
        assert!(a.live_job.is_none());
        gone(pid);
        assert_eq!(a.snapshot()["recent_actions"], held["recent_actions"]);
        assert_eq!(a.controller.editor.campaign, before);
        fulltime(&mut a.controller);
        a.key(Key::Enter);
        let recorded = a.controller.editor.campaign.clone();
        assert_eq!(recorded.revision, before.revision + 1);
        assert!(recorded.played_fixture.is_some());
        // [designed:T80] The W030 tail applies the researched week transaction
        // before the save: cash, week index and the following fixture.
        a.key(Key::Space);
        assert_eq!(a.controller.screen, Screen::TransferNews);
        a.key(Key::Space);
        assert_eq!(a.controller.screen, Screen::Main);
        let advanced = a.controller.editor.campaign.clone();
        assert_eq!(advanced.week_index, 1);
        assert_eq!(advanced.cash, 421491);
        assert_eq!(advanced.next_fixture.opponent, 176);
        assert_eq!(advanced.next_fixture.opponent_name, "Sevilla");
        assert_eq!(advanced.revision, recorded.revision + 1);
        a.key(Key::F7);
        assert!(a.save_path.exists());
        a.key(Key::F9);
        assert_eq!(a.controller.editor.campaign, advanced);
        assert_eq!(a.controller.screen, Screen::Main);
        assert!(a.live_job.is_none());
    }
}
#[test]
fn real_application_cancel_reload_failure_and_drop_reap_without_posting() {
    let temp = Temp::new();
    let initial = campaign(false);
    initial.save(&temp.0.join("game.json")).unwrap();
    let mut a = Application::new(root(), temp.0.join("game.json"), false).unwrap();
    a.key(Key::Enter);
    a.key(Key::Enter);
    a.update_with_executable(exe());
    let pid = a.live_job.as_ref().unwrap().pid();
    a.key(Key::Escape);
    assert!(a.live_job.is_none());
    gone(pid);
    assert_eq!(a.controller.editor.campaign, initial);
    let mut a = started(&temp.0, false);
    let pid = a.live_job.as_ref().unwrap().pid();
    a.choose_save();
    assert_eq!(a.controller.screen, Screen::Main);
    assert!(a.live_job.is_none());
    gone(pid);
    assert_eq!(a.controller.editor.campaign, initial);
    std::fs::write(temp.0.join("game.json"), b"bad save").unwrap();
    let mut a = started(&temp.0, false);
    let pid = a.live_job.as_ref().unwrap().pid();
    let id = a.controller.job_id;
    a.choose_save();
    assert_eq!(a.controller.screen, Screen::Pitch);
    assert_eq!(a.controller.job_id, id);
    assert_eq!(a.live_job.as_ref().unwrap().pid(), pid);
    #[cfg(unix)]
    {
        assert!(std::process::Command::new("kill")
            .args(["-KILL", &pid.to_string()])
            .status()
            .unwrap()
            .success());
        pump(&mut a, |a| a.live_job.is_none());
        assert_eq!(a.controller.screen, Screen::Preparation);
        assert_eq!(a.controller.editor.campaign, initial);
        gone(pid);
    }
    drop(a);
    gone(pid);
    let mut resuming = started(&temp.0, false);
    let resume_pid = resuming.live_job.as_ref().unwrap().pid();
    halftime(&mut resuming.controller);
    resuming.key(Key::Enter);
    resuming.key(Key::Enter);
    // Dispatch the real command, then cancel before processing any completion.
    resuming.controller.resume_requested = false;
    resuming
        .live_job
        .as_mut()
        .unwrap()
        .decide(onenil_desktop::live_worker::Decision::Continue)
        .unwrap();
    resuming.key(Key::Escape);
    assert!(resuming.live_job.is_none());
    gone(resume_pid);
    assert_eq!(resuming.controller.screen, Screen::Preparation);
    assert_eq!(resuming.controller.editor.campaign, initial);
    let a = started(&temp.0, false);
    let pid = a.live_job.as_ref().unwrap().pid();
    drop(a);
    gone(pid);
}
