//! [confirmed:archaeology T100/T119/T102/T123] T119: the league campaign's
//! preparation is the exact T119 fixture, the native quick engine plays it to
//! the witnessed result, and the existing week transaction posts the week.
use onenil_campaign::Campaign;
use onenil_desktop::campaign_match::{self, Profile};
use onenil_engine::quick_match;

#[test]
fn the_quick_board_shows_the_witnessed_goals_and_substitutions() {
    let mut campaign = Campaign::initial_league().unwrap();
    let ticket = campaign.match_ticket().unwrap();
    let mut quick = onenil_desktop::quick::Match::start(&campaign).unwrap();
    let half = quick.to_half_time().unwrap();
    assert_eq!(half.score, [2, 0]);
    assert_eq!(half.club_names, ["Newcastle Utd", "Blackburn R"]);
    assert_eq!(half.attendance, 36_986);
    assert_eq!(
        half.lines,
        vec![
            "**** GOAL **** by R Fox",
            "**** GOAL **** by S Howey",
            "---- HALF TIME ----"
        ]
    );
    assert_eq!(
        half.scorers,
        vec![
            onenil_desktop::quick::Scorer {
                name: "R Fox".into(),
                minute: 29,
                side: 0
            },
            onenil_desktop::quick::Scorer {
                name: "S Howey".into(),
                minute: 33,
                side: 0
            },
        ]
    );
    let full = quick.from_half_time().unwrap();
    assert_eq!(full.score, [2, 0]);
    assert_eq!(
        full.lines,
        vec![
            "**** GOAL **** by R Fox",
            "**** GOAL **** by S Howey",
            "Clark on for Venison",
            "Watson on for Albert",
            "---- FULL TIME ----"
        ]
    );
    quick.complete(&mut campaign, &ticket).unwrap();
    assert_eq!(campaign.cash, 1_691_393);
    assert_eq!(campaign.league_played.as_ref().unwrap().score, [0, 2]);
}

#[test]
fn the_league_campaign_admits_the_preparation_and_the_quick_result_completes_the_week() {
    let mut campaign = Campaign::initial_league().unwrap();
    assert_eq!(campaign_match::profile(&campaign).unwrap(), Profile::League);
    let input = campaign_match::admission(&campaign).unwrap();
    let request = serde_json::to_string(&input).unwrap();
    let mut session = quick_match::session(quick_match::Profile::League, &request).unwrap();
    let report = quick_match::play(&mut session).unwrap();
    assert_eq!(
        report.score,
        serde_json::json!({"clubs": [122, 130], "goals": [2, 0]})
    );
    let goals: Vec<(u64, u64)> = report
        .goals
        .iter()
        .map(|goal| {
            (
                goal["event"]["minute"].as_u64().unwrap(),
                goal["event"]["actor"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(goals, [(29, 9), (33, 6)]);
    let scorers: Vec<u64> = report.snapshot["scorer_logs"][1]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["player"].as_u64().unwrap())
        .collect();
    assert_eq!(scorers, [2429, 2426]);
    assert_eq!(report.steps, 92);
    assert_eq!(report.setup_draws, 259);
    assert_eq!(report.step_draws, 5493);

    // [confirmed:T108] The campaign stores the score managed-first: Blackburn
    // 0, Newcastle 2. The rest of the week is the witnessed T108 transaction.
    let ticket = campaign.match_ticket().unwrap();
    campaign
        .record_league_week(
            &ticket,
            [0, 2],
            "0AB9:EBE2",
            726_972,
            80_639,
            0,
            103,
            0,
            "Chelsea",
        )
        .unwrap();
    assert_eq!(campaign.cash, 1_691_393);
    assert_eq!(campaign.week_index, 54);
    assert_eq!(campaign.next_fixture.slot, 55);
    assert_eq!(campaign.next_fixture.opponent, 103);
    let played = campaign.league_played.as_ref().unwrap();
    assert_eq!(played.score, [0, 2]);
    assert_eq!(played.prepared.name, "Newcastle Utd");
}

fn quick_board(stage: &'static str) -> onenil_desktop::quick::Board {
    onenil_desktop::quick::Board {
        clubs: [122, 130],
        club_names: ["Newcastle Utd".into(), "Blackburn R".into()],
        score: [2, 0],
        lines: vec!["---- HALF TIME ----".into()],
        scorers: vec![onenil_desktop::quick::Scorer {
            name: "R Fox".into(),
            minute: 29,
            side: 0,
        }],
        attendance: 36_986,
        stage,
    }
}

#[test]
fn the_configuration_panel_toggles_watch_off_and_back() {
    let mut controller = onenil_desktop::controller::Controller::default();
    controller.menu = 15;
    controller.key(onenil_desktop::controller::Key::Enter);
    assert_eq!(
        controller.screen,
        onenil_desktop::controller::Screen::Configuration
    );
    assert!(controller.watch_match);
    controller.key(onenil_desktop::controller::Key::Enter);
    assert!(!controller.watch_match);
    controller.key(onenil_desktop::controller::Key::Escape);
    assert_eq!(controller.screen, onenil_desktop::controller::Screen::Main);
}

#[test]
fn the_league_preparation_requires_watch_off_and_records_the_quick_week() {
    use onenil_desktop::controller::{Controller, Key, Screen};
    let mut controller = Controller::default();
    controller.editor.campaign = Campaign::initial_league().unwrap();
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparation);
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparation);
    assert!(controller.message.contains("Watch match off"));
    controller.watch_match = false;
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::Preparing);
    assert!(controller.quick_requested);
    let id = controller.job_id;
    assert!(controller.accept_quick(id, quick_board("HALF TIME")));
    assert_eq!(controller.screen, Screen::HalfTime);
    controller.key(Key::Space);
    assert_eq!(controller.screen, Screen::Preparing);
    assert!(controller.quick_resume_requested);
    assert!(controller.accept_quick(id, quick_board("FULL TIME")));
    assert_eq!(controller.screen, Screen::FullTime);
    controller.key(Key::Enter);
    assert_eq!(controller.screen, Screen::CashFlow);
    assert_eq!(controller.editor.campaign.cash, 1_691_393);
    assert!(controller.editor.campaign.league_played.is_some());
}

#[test]
fn the_application_computes_the_quick_match_at_its_real_boundaries() {
    use onenil_desktop::application::Application;
    use onenil_desktop::controller::{Key, Screen};
    use std::path::{Path, PathBuf};
    let mut app = Application::new(PathBuf::new(), PathBuf::from("league.1-0"), false).unwrap();
    app.controller.editor.campaign = Campaign::initial_league().unwrap();
    app.controller.watch_match = false;
    app.key(Key::Enter);
    assert_eq!(app.controller.screen, Screen::Preparation);
    app.key(Key::Enter);
    assert_eq!(app.controller.screen, Screen::Preparing);
    app.update_with_executable(Path::new("unused"));
    assert_eq!(app.controller.screen, Screen::HalfTime);
    let half = app.controller.quick.as_ref().unwrap();
    assert_eq!(half.score, [2, 0]);
    assert_eq!(half.stage, "HALF TIME");
    app.key(Key::Space);
    app.update_with_executable(Path::new("unused"));
    assert_eq!(app.controller.screen, Screen::FullTime);
    let full = app.controller.quick.as_ref().unwrap();
    assert_eq!(full.stage, "FULL TIME");
    assert!(full.lines.iter().any(|line| line == "Clark on for Venison"));
    app.key(Key::Enter);
    assert_eq!(app.controller.screen, Screen::CashFlow);
    assert_eq!(app.controller.editor.campaign.cash, 1_691_393);
    assert_eq!(app.controller.editor.campaign.week_index, 54);
}
