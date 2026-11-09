//! [designed:T80] The W030 week route: the authenticated week transaction fires
//! once after the exactly-once result, the route screens carry the pinned
//! receipt values, and reload restores the advanced week. Fail-before: the
//! route screens and the campaign week transaction do not exist yet.
use onenil_desktop::{
    controller::{Controller, Key, Screen},
    timeline::{IntervalTimeline, LiveCampaign},
};
fn campaign() -> onenil_campaign::Campaign {
    onenil_campaign::Campaign::initial().unwrap()
}
fn preparing() -> Controller {
    let mut c = Controller::default();
    c.editor.campaign = campaign();
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Preparing);
    c
}
fn interval(engine: &mut LiveCampaign) -> IntervalTimeline {
    engine.pause().unwrap()
}
fn fulltime(c: &mut Controller, engine: &mut LiveCampaign, prefix: &IntervalTimeline) {
    assert!(c.accept_interval(c.job_id, prefix.clone()));
    for _ in 0..10000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    c.key(Key::Enter);
    c.key(Key::Enter);
    for _ in 0..10000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    let completion = engine.finish(false).unwrap();
    assert!(c.accept_continuation(c.job_id, completion));
    for _ in 0..100000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    assert_eq!(c.screen, Screen::FullTime);
}
#[test]
fn week_route_posts_result_walks_w030_and_persists_the_week_once() {
    let mut engine = LiveCampaign::new(&campaign()).unwrap();
    let prefix = interval(&mut engine);
    let mut c = preparing();
    let before = c.editor.campaign.clone();
    fulltime(&mut c, &mut engine, &prefix);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::CashFlow);
    let cash_screen_campaign = c.editor.campaign.clone();
    assert!(cash_screen_campaign.played_fixture.is_some());
    assert_eq!(cash_screen_campaign.cash, before.cash);
    assert_eq!(cash_screen_campaign.week_index, 0);
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::TransferNews);
    c.key(Key::Space);
    println!("DEBUG message after transfer space: {:?}", c.message);
    assert_eq!(c.screen, Screen::Main);
    let advanced = c.editor.campaign.clone();
    assert_eq!(advanced.cash, 421491);
    assert_eq!(advanced.week_index, 1);
    assert_eq!(advanced.next_fixture.opponent, 176);
    assert_eq!(advanced.next_fixture.opponent_name, "Sevilla");
    assert_eq!(advanced.revision, before.revision + 2);
    assert!(advanced.played_fixture.is_some());
    assert_eq!(advanced.club.first_team[1], Some(2582));
    c.screen = Screen::Main;
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert!(c.message.contains("already has a result"), "{}", c.message);
    assert_eq!(c.editor.campaign, advanced);
    let path = std::env::temp_dir().join(format!("T80-week-{}", std::process::id()));
    c.editor.campaign.save(&path).unwrap();
    let mut reloaded = Controller::default();
    reloaded.editor.load(&path).unwrap();
    assert_eq!(reloaded.editor.campaign, advanced);
    std::fs::remove_file(path).unwrap();
}
#[test]
fn exhibition_full_time_never_advances_the_week() {
    let mut c = Controller::default();
    c.exhibition = true;
    c.screen = Screen::FullTime;
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Main);
    assert_eq!(c.editor.campaign.week_index, 0);
    assert!(c.editor.campaign.played_fixture.is_none());
}
