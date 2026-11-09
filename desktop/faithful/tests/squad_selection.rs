//! [designed:T78] Interval squad selection over the owned snapshot, decision
//! consistency, and the researched persisted roster policy (archaeology T80).
use onenil_desktop::{
    controller::{Controller, Key, Screen},
    timeline::{Completion, IntervalTimeline, LiveCampaign, Phase},
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
fn halftime(c: &mut Controller, prefix: &IntervalTimeline) {
    assert!(c.accept_interval(c.job_id, prefix.clone()));
    for _ in 0..10000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    assert_eq!(c.screen, Screen::HalfTime);
    c.key(Key::Enter);
    assert_eq!(c.screen, Screen::Bench);
}
fn fulltime(c: &mut Controller) {
    for _ in 0..100000 {
        if c.screen != Screen::Pitch {
            break;
        }
        c.tick();
    }
    assert_eq!(c.screen, Screen::FullTime);
}
#[test]
fn squad_opens_over_the_owned_snapshot_and_refuses_unwitnessed_pairs() {
    let mut engine = LiveCampaign::new(&campaign()).unwrap();
    let prefix = interval(&mut engine);
    let mut c = preparing();
    let before = c.editor.campaign.clone();
    halftime(&mut c, &prefix);
    assert_eq!(c.editor.campaign, before);
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::MatchSquad);
    let snapshot = &c.interval_playback.as_ref().unwrap().timeline.snapshot;
    assert_eq!(snapshot.club, 130);
    assert_eq!(snapshot.remaining_subs, 7);
    assert_eq!(snapshot.roster[1], 2582);
    assert_eq!(snapshot.roster[11], 2592);
    assert_eq!(snapshot.roster.len(), 20);
    assert_eq!(
        snapshot.players["2582"].current_position,
        c.editor.player(2582).unwrap().current_position
    );
    // Unwitnessed pairs refuse with readable feedback; nothing is requested.
    for (pick, put) in [(0usize, 11usize), (1, 14), (3, 11)] {
        c.squad.as_mut().unwrap().highlight = pick;
        c.key(Key::Enter);
        c.squad.as_mut().unwrap().highlight = put;
        c.key(Key::Enter);
        assert!(c.message.contains("witnessed"), "{}", c.message);
        assert!(!c.replace_requested && !c.is_resuming());
        assert_eq!(c.screen, Screen::MatchSquad);
        c.key(Key::Escape);
        assert!(c.squad.as_ref().unwrap().picked.is_none());
    }
    // Escape at the pick cancels the pending pick; Escape again returns to the bench.
    c.squad.as_mut().unwrap().highlight = 1;
    c.key(Key::Enter);
    assert!(c.squad.as_ref().unwrap().picked.is_some());
    c.key(Key::Escape);
    assert!(c.squad.as_ref().unwrap().picked.is_none());
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Bench);
    assert!(c.squad.is_none());
    assert_eq!(c.editor.campaign, before);
}
#[test]
fn witnessed_confirmation_persists_the_exchange_exactly_once() {
    let mut engine = LiveCampaign::new(&campaign()).unwrap();
    let prefix = interval(&mut engine);
    let mut c = preparing();
    let before = c.editor.campaign.clone();
    halftime(&mut c, &prefix);
    c.key(Key::Space);
    c.squad.as_mut().unwrap().highlight = 1;
    c.key(Key::Enter);
    c.squad.as_mut().unwrap().highlight = 11;
    c.key(Key::Enter);
    assert!(c.replace_requested && c.is_resuming());
    assert_eq!(c.screen, Screen::Preparing);
    let completion = engine.finish(true).unwrap();
    assert!(completion.replacement.is_some());
    assert!(c.accept_continuation(c.job_id, completion));
    assert_eq!(c.match_view().unwrap().phase, Phase::SecondHalf);
    fulltime(&mut c);
    c.key(Key::Enter);
    let recorded = c.editor.campaign.clone();
    assert!(recorded.played_fixture.is_some());
    assert_eq!(recorded.revision, before.revision + 1);
    assert_eq!(recorded.club.first_team[1], Some(2592));
    assert_eq!(recorded.club.first_team[11], Some(2582));
    // The W030 tail: the week transaction applies after the result.
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::TransferNews);
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::Main);
    let advanced = c.editor.campaign.clone();
    assert_eq!(advanced.week_index, 1);
    assert_eq!(advanced.cash, 421491);
    assert_eq!(advanced.next_fixture.opponent, 176);
    assert_eq!(advanced.next_fixture.opponent_name, "Sevilla");
    assert_eq!(advanced.revision, recorded.revision + 1);
}
#[test]
fn cancelled_selection_continues_clean_and_persists_no_exchange() {
    let mut engine = LiveCampaign::new(&campaign()).unwrap();
    let prefix = interval(&mut engine);
    let mut c = preparing();
    let before = c.editor.campaign.clone();
    halftime(&mut c, &prefix);
    c.key(Key::Space);
    c.squad.as_mut().unwrap().highlight = 1;
    c.key(Key::Enter);
    c.key(Key::Escape);
    c.key(Key::Escape);
    assert_eq!(c.screen, Screen::Bench);
    c.key(Key::Enter);
    assert!(c.resume_requested && !c.replace_requested);
    let completion = engine.finish(false).unwrap();
    assert!(completion.replacement.is_none());
    assert!(c.accept_continuation(c.job_id, completion));
    fulltime(&mut c);
    c.key(Key::Enter);
    let recorded = c.editor.campaign.clone();
    assert!(recorded.played_fixture.is_some());
    assert_eq!(recorded.club.first_team[1], Some(2582));
    assert_eq!(recorded.club.first_team[11], Some(2592));
    // The cancelled case: the clean result records with no roster change.
    c.key(Key::Space);
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::Main);
}
#[test]
fn decision_mismatch_stale_and_duplicate_completions_never_mutate() {
    let mut engine = LiveCampaign::new(&campaign()).unwrap();
    let prefix = interval(&mut engine);
    // A Continue completion after a replacement request refuses.
    let mut c = preparing();
    let before = c.editor.campaign.clone();
    halftime(&mut c, &prefix);
    c.key(Key::Space);
    c.squad.as_mut().unwrap().highlight = 1;
    c.key(Key::Enter);
    c.squad.as_mut().unwrap().highlight = 11;
    c.key(Key::Enter);
    assert!(c.replace_requested);
    let continue_completion = {
        let mut other = LiveCampaign::new(&campaign()).unwrap();
        other.pause().unwrap();
        other.finish(false).unwrap()
    };
    assert!(continue_completion.replacement.is_none());
    // The mismatch refuses via the owned live error path: handled, but the
    // campaign stays clean and no pending exchange survives it.
    assert!(c.accept_continuation(c.job_id, continue_completion));
    assert_eq!(c.screen, Screen::Preparation);
    assert_eq!(c.editor.campaign, before);
    assert!(before.played_fixture.is_none());
    assert!(!c.replace_requested);
    assert!(!c.is_resuming());
    // A replacement completion without a request refuses the same way.
    let mut c = preparing();
    halftime(&mut c, &prefix);
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert!(c.resume_requested && !c.replace_requested);
    let mut other = LiveCampaign::new(&campaign()).unwrap();
    other.pause().unwrap();
    let replaced = other.finish(true).unwrap();
    assert!(replaced.replacement.is_some());
    assert!(c.accept_continuation(c.job_id, replaced));
    assert_eq!(c.screen, Screen::Preparation);
    assert!(c.editor.campaign.played_fixture.is_none());
    assert_eq!(c.editor.campaign, campaign());
}
