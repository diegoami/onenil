use onenil_desktop::controller::{Controller, Key, Screen};

fn edited_controller() -> Controller {
    let mut c = Controller::default();
    for key in [
        Key::Space,
        Key::Down,
        Key::Enter,
        Key::Down,
        Key::Enter,
        Key::Escape,
        Key::Up,
    ] {
        c.key(key);
    }
    assert_eq!(c.screen, Screen::Main);
    assert_eq!(c.menu, 0);
    assert_eq!(
        c.editor.campaign.club.first_team[1..3],
        [Some(2583), Some(2582)]
    );
    c
}

#[test]
fn original_keyboard_edit_starts_owned_desktop_preparation() {
    let mut c = edited_controller();
    let before = c.editor.campaign.clone();
    onenil_desktop::campaign_match::admission(&before).unwrap();
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparing);
    assert!(c.prepare_requested);
    assert!(!c.exhibition);
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Preparation);
    assert_eq!(c.editor.campaign, before);
}

#[test]
fn edited_admission_equals_pinned_source_and_rejects_other_changes() {
    use onenil_desktop::campaign_match::{self, Profile};
    let campaign = edited_controller().editor.campaign;
    let restored = onenil_campaign::Campaign::decode(&campaign.encode().unwrap()).unwrap();
    assert_eq!(campaign_match::profile(&restored).unwrap(), Profile::Edited);
    let input = serde_json::to_value(campaign_match::admission(&restored).unwrap()).unwrap();
    let source: serde_json::Value = serde_json::from_str(include_str!(
        "../../../engine/tests/fixtures/edited_preparation/admission.json"
    ))
    .unwrap();
    let support: serde_json::Value = serde_json::from_str(include_str!(
        "../../../engine/tests/fixtures/edited_watched/incoming.json"
    ))
    .unwrap();
    assert_eq!(input["preparation"], source["input"]);
    assert_eq!(input["support"], support["support"]);
    for damage in 0..9 {
        let mut changed = campaign.clone();
        match damage {
            0 => changed.players[0].individual_instructions ^= 1,
            1 => changed.players[0].availability.condition_signed -= 1,
            2 => changed.players[0].current_position ^= 1,
            3 => changed.club.team_instructions ^= 1,
            4 => changed.next_fixture.opponent = 134,
            5 => changed.origin.league_sha256 = "foreign".into(),
            6 => changed.revision += 1,
            7 => changed.club.first_team.swap(0, 3),
            _ => changed.cash += 1,
        }
        assert!(
            campaign_match::admission(&changed).is_err(),
            "damage {damage}"
        );
    }
}

#[test]
fn edited_child_worker_computes_owned_result_and_native_save() {
    use onenil_desktop::worker::{Job, Poll};
    use std::{
        path::Path,
        time::{Duration, Instant},
    };
    let mut c = edited_controller();
    let before = c.editor.campaign.clone();
    c.key(Key::Enter);
    c.key(Key::Enter);
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut job = Job::spawn_campaign(
        Path::new(env!("CARGO_BIN_EXE_onenil-desktop")),
        root,
        c.job_id,
        &before,
    )
    .unwrap();
    let start = Instant::now();
    let timeline = loop {
        match job.poll() {
            Poll::Ready(id, t) => {
                assert_eq!(id, c.job_id);
                break t;
            }
            Poll::Failed(_, e) => panic!("{e}"),
            Poll::Pending => {
                assert!(start.elapsed() < Duration::from_secs(180));
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    };
    assert!(job.reaped());
    assert_eq!(timeline.frames.len(), 3970);
    assert_eq!(timeline.draws, 170212);
    assert_eq!(timeline.frames.last().unwrap().view.score, [0, 2]);
    assert_eq!(timeline.native_return, "0AB9:EBE2");
    let events = timeline
        .frames
        .iter()
        .map(|f| f.events.len())
        .sum::<usize>();
    assert_eq!(events, 869);
    assert_eq!(timeline.setup_events.len(), 1);
    assert!(timeline
        .frames
        .iter()
        .flat_map(|f| &f.events)
        .any(|e| e.text.contains("Hendry") || e.text.contains("Pearce")));
    let stale = c.job_id;
    assert!(c.accept_timeline(stale, timeline.clone()));
    for _ in 0..5000 {
        match c.screen {
            Screen::Pitch => c.tick(),
            Screen::HalfTime => {
                c.key(Key::Enter);
                c.key(Key::Enter);
            }
            Screen::FullTime => break,
            _ => panic!("unexpected {:?}", c.screen),
        }
    }
    assert_eq!(c.screen, Screen::FullTime);
    assert_eq!(c.editor.campaign, before);
    c.key(Key::Enter);
    let result = c.editor.campaign.clone();
    assert_eq!(result.revision, 2);
    assert_eq!(result.played_fixture.as_ref().unwrap().score, [0, 2]);
    let mut unchanged = result.clone();
    unchanged.revision = before.revision;
    unchanged.played_fixture = None;
    assert_eq!(unchanged, before);
    assert!(!c.accept_timeline(stale, timeline));
    let path = std::env::temp_dir().join(format!("T69-edited-result-{}.json", std::process::id()));
    onenil_campaign::storage::save(&result, &path).unwrap();
    let loaded = onenil_campaign::storage::load(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(loaded, result);
    assert!(onenil_desktop::campaign_match::admission(&loaded)
        .err()
        .unwrap()
        .contains("already"));
    // [designed:T80] The W030 tail applies the researched week transaction.
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::TransferNews);
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::Main);
    let advanced = c.editor.campaign.clone();
    assert_eq!(advanced.week_index, 1);
    assert_eq!(advanced.cash, 421491);
    assert_eq!(advanced.next_fixture.opponent, 176);
    assert_eq!(advanced.next_fixture.opponent_name, "Sevilla");
    assert_eq!(advanced.revision, result.revision + 1);
    assert!(advanced.played_fixture.is_some());
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparation);
    assert!(!c.prepare_requested);
    assert_eq!(c.editor.campaign, advanced);
}
