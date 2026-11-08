use onenil_desktop::controller::{Controller, Key, Screen};

#[test]
fn actual_supported_campaign_starts_and_cancels_without_becoming_exhibition() {
    let mut controller = Controller::default();
    let before = controller.editor.campaign.clone();
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparation);
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparing);
    assert!(controller.prepare_requested);
    assert!(!controller.exhibition);
    let generation = controller.job_id;
    controller.key(Key::Escape);
    assert_eq!(controller.screen, Screen::Preparation);
    assert!(!controller.exhibition);
    assert!(!controller.prepare_requested);
    assert!(!controller.accept_error(generation, "late failure".into()));
    assert_eq!(controller.editor.campaign, before);
}

#[test]
fn changed_campaign_and_pending_move_refuse_without_mutation() {
    use onenil_desktop::editor::Location;
    let mut controller = Controller::default();
    controller
        .editor
        .campaign
        .exchange(
            onenil_campaign::Location::First(0),
            onenil_campaign::Location::First(1),
        )
        .unwrap();
    let before = controller.editor.campaign.clone();
    controller.key(Key::Enter);
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparation);
    assert!(!controller.prepare_requested);
    assert!(controller.message.is_ascii() && controller.message.len() <= 80);
    assert_eq!(controller.message, onenil_desktop::campaign_match::REFUSAL);
    assert!(controller.message.ends_with("Reload the original save."));
    assert_eq!(controller.editor.campaign, before);
    let mut controller = Controller::default();
    controller.editor.select(Location::First(0)).unwrap();
    controller.editor.pick().unwrap();
    controller.key(Key::Enter);
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparation);
    assert!(!controller.prepare_requested);
    assert!(controller.editor.carried_player().is_some());
}

#[test]
fn admission_uses_active_ids_and_refuses_foreign_or_changed_inputs() {
    use onenil_desktop::campaign_match;
    let initial = onenil_campaign::Campaign::initial().unwrap();
    let input = campaign_match::admission(&initial).unwrap();
    assert_eq!(input.preparation.setup.legacy.history.club, initial.club.id);
    let club = input
        .preparation
        .setup
        .legacy
        .substitution
        .context
        .clubs
        .iter()
        .find(|club| club.id == initial.club.id)
        .unwrap();
    assert_eq!(
        club.roster,
        initial.club.first_team.map(|id| id.unwrap_or(0))
    );
    let prepared = onenil_engine::watched_compiled::prepared::Session::new_fresh(input).unwrap();
    assert_eq!(prepared.snapshot().unwrap()["scenes"][0]["word14"], 2581);
    for damage in 0..5 {
        let mut changed = initial.clone();
        match damage {
            0 => changed.players[0].individual_instructions ^= 1,
            1 => changed.players[0].availability.condition_signed -= 1,
            2 => changed.club.team_instructions ^= 1,
            3 => changed.next_fixture.opponent = 134,
            _ => changed.origin.league_sha256 = "foreign".into(),
        }
        let before = changed.clone();
        assert!(campaign_match::admission(&changed).is_err());
        assert_eq!(changed, before);
    }
}

#[test]
fn real_campaign_worker_returns_fresh_players_actions_and_phase_controls() {
    use onenil_desktop::{
        timeline::{Phase, Playback},
        worker::{Job, Poll},
    };
    use std::{
        path::Path,
        time::{Duration, Instant},
    };
    let campaign = onenil_campaign::Campaign::initial().unwrap();
    let before = campaign.clone();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut job = Job::spawn_campaign(
        Path::new(env!("CARGO_BIN_EXE_onenil-desktop")),
        root,
        62,
        &campaign,
    )
    .unwrap();
    let started = Instant::now();
    let timeline = loop {
        match job.poll() {
            Poll::Ready(id, timeline) => {
                assert_eq!(id, 62);
                break timeline;
            }
            Poll::Failed(_, error) => panic!("{error}"),
            Poll::Pending => {
                assert!(started.elapsed() < Duration::from_secs(180));
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    };
    assert!(job.reaped());
    assert_eq!(timeline.club_names, ["Blackburn R", "P St Germain"]);
    let original: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.join("records/T60/source-r0001/admission-r0001.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        timeline.seed,
        original["payload"]["draws"][0]["seed_before"]
            .as_u64()
            .unwrap()
    );
    assert_eq!((timeline.frames.len(), timeline.draws), (3825, 158992));
    assert_eq!(timeline.frames.last().unwrap().view.score, [1, 4]);
    let mut scored = [0, 0];
    for frame in &timeline.frames {
        for event in &frame.events {
            if event.native["code"] == 38 {
                let side = if event.native["club"] == 130 {
                    0
                } else {
                    assert_eq!(event.native["club"], 17);
                    1
                };
                scored[side] += 1;
            }
        }
        assert_eq!(
            frame.view.score, scored,
            "score must identify the scoring player's club"
        );
    }
    assert_eq!(
        timeline
            .frames
            .iter()
            .map(|frame| frame.events.len())
            .sum::<usize>(),
        748
    );
    assert!(timeline
        .initial
        .players
        .iter()
        .any(|player| player.club == 17 && player.player == 321));
    assert!(!timeline
        .initial
        .players
        .iter()
        .any(|player| player.club == 134));
    let text = timeline
        .frames
        .iter()
        .flat_map(|frame| &frame.events)
        .map(|event| event.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(text.contains("Lama"));
    assert!(text.contains("Shearer"));
    assert!(!text.contains("Play continues"));
    let mut playback = Playback::new(timeline);
    playback.start_or_continue().unwrap();
    while playback.phase == Phase::FirstHalf {
        playback.step(8).unwrap()
    }
    assert_eq!(playback.phase, Phase::HalfTime);
    playback.start_or_continue().unwrap();
    while playback.phase == Phase::SecondHalf {
        playback.step(8).unwrap()
    }
    assert_eq!(playback.phase, Phase::FullTime);
    assert_eq!(playback.events.len(), 749); // Includes the original setup action.
    assert_eq!(campaign, before);
}
