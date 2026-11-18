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
fn the_witnessed_sevilla_route_plays_records_and_refuses_the_week() {
    let mut c = preparing();
    let before = c.editor.campaign.clone();
    let mut engine = LiveWitnessed::new(&before).unwrap();
    let prefix = engine.pause().unwrap();
    assert_eq!(prefix.frames.len(), 1949);
    assert_eq!(prefix.seed, 50238);
    assert!(c.accept_interval(c.job_id, prefix));
    half_time(&mut c);
    assert_eq!(c.editor.campaign, before);
    c.key(Key::Enter);
    c.key(Key::Enter);
    assert!(c.resume_requested && c.is_resuming());
    assert_eq!(c.screen, Screen::Preparing);
    // [designed:T91] The witnessed route admits no replacement.
    assert!(engine.finish(true).is_err());
    let completed = engine.finish(false).unwrap();
    let condition_updates = completed.condition_updates.clone().unwrap();
    assert!(completed.replacement.is_none());
    assert_eq!(condition_updates, witnessed_conditions());
    assert!(c.accept_continuation(c.job_id, completed));
    assert_eq!(c.match_view().unwrap().phase, Phase::SecondHalf);
    full_time(&mut c);
    assert_eq!(c.match_view().unwrap().loaded_frames, 3927);
    assert_eq!(c.match_view().unwrap().view().score, [1, 1]);
    let mut c = c;
    let entered = c.editor.campaign.clone();
    c.key(Key::Enter);
    // [T91] The witnessed result and the caller's condition outcomes recorded
    // once; the week index stays 1 with Sevilla played.
    assert_eq!(c.screen, Screen::CashFlow);
    let recorded = c.editor.campaign.clone();
    assert_eq!(recorded.revision, entered.revision + 1);
    assert_eq!(recorded.week_index, 1);
    assert_eq!(recorded.next_fixture.opponent, 176);
    let second = recorded.second_fixture.as_ref().unwrap();
    assert_eq!(second.score, [1, 1]);
    assert_eq!(second.week_index, 1);
    assert_eq!(second.native_return, "0AB9:EBE2");
    assert_eq!(second.request_sha256.len(), 64);
    let conditions = witnessed_conditions();
    for row in &recorded.players {
        let expected = conditions
            .iter()
            .find(|(player, _)| u64::from(*player) == u64::from(row.id))
            .unwrap();
        assert_eq!(row.availability.condition_signed, expected.1, "player {}", row.id);
    }
    assert_eq!(c.message, "Result recorded: Blackburn R 1-1 Sevilla. F7 saves. The week's cash movement is unwitnessed.");
    // The witnessed week's news screen carries no receipt rows, and the week
    // transaction refuses naming the unwitnessed second-week cash movement;
    // the state keeps week 1 with Sevilla played.
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::TransferNews);
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::TransferNews);
    assert_eq!(
        c.message,
        "the second week's cash movement is unwitnessed; the Sevilla week's full-time cash/news route has no accepted original evidence"
    );
    assert_eq!(c.editor.campaign, recorded);
    assert_eq!(
        recorded.match_ticket().err().unwrap(),
        "this fixture already has a recorded result"
    );
    assert_eq!(
        crate_profile_message(&recorded),
        "This fixture already has a result."
    );
}
#[test]
fn the_post_caller_state_survives_restart_and_reload_and_stays_refused() {
    let mut c = preparing();
    let mut engine = LiveWitnessed::new(&c.editor.campaign).unwrap();
    let prefix = engine.pause().unwrap();
    assert!(c.accept_interval(c.job_id, prefix));
    half_time(&mut c);
    c.key(Key::Enter);
    c.key(Key::Enter);
    let completed = engine.finish(false).unwrap();
    assert!(c.accept_continuation(c.job_id, completed));
    full_time(&mut c);
    c.key(Key::Enter);
    let recorded = c.editor.campaign.clone();
    // Restart and reload the same result/conditions/week.
    let image = serde_json::to_vec(&recorded).unwrap();
    let mut reloaded = onenil_campaign::Campaign::initial().unwrap();
    reloaded.replace_from_bytes(&image).unwrap();
    assert_eq!(reloaded, recorded);
    let mut c2 = Controller::default();
    c2.editor.campaign = reloaded;
    c2.key(Key::Enter);
    c2.key(Key::Enter);
    assert_eq!(c2.screen, Screen::Preparation);
    assert!(!c2.prepare_requested);
    assert!(c2.message.contains("already has a result"));
    assert_eq!(c2.editor.campaign, recorded);
    // The week transaction still refuses after the reload, and the witnessed
    // route never enables a third match: a doctored week-2/Celtic state is
    // outside the supported saved context.
    let mut celtic = recorded.clone();
    celtic.week_index = 2;
    celtic.next_fixture.opponent = 28;
    celtic.next_fixture.opponent_name = "Celtic".into();
    celtic.next_fixture.slot = 3;
    assert!(celtic.validate().is_err());
    assert!(celtic.match_ticket().is_err());
}
#[test]
fn the_witnessed_route_refuses_every_other_campaign() {
    // The week-0 fresh campaign is the computed route's, not the witnessed one.
    let fresh = onenil_campaign::Campaign::initial().unwrap();
    assert!(LiveWitnessed::new(&fresh).is_err());
    // The recorded-result state refuses through the preparation entry: the
    // witnessed route played once, and the accurate refusal follows.
    let mut c = preparing();
    let mut engine = LiveWitnessed::new(&c.editor.campaign).unwrap();
    let prefix = engine.pause().unwrap();
    assert!(c.accept_interval(c.job_id, prefix));
    half_time(&mut c);
    c.key(Key::Enter);
    c.key(Key::Enter);
    let completed = engine.finish(false).unwrap();
    assert!(c.accept_continuation(c.job_id, completed));
    full_time(&mut c);
    c.key(Key::Enter);
    assert!(c.editor.campaign.second_fixture.is_some());
    assert_eq!(
        c.editor.campaign.match_ticket().err().unwrap(),
        "this fixture already has a recorded result"
    );
    assert_eq!(
        crate_profile_message(&c.editor.campaign),
        "This fixture already has a result."
    );
}
fn crate_profile_message(
    campaign: &onenil_campaign::Campaign,
) -> String {
    onenil_desktop::campaign_match::profile(campaign)
        .err()
        .unwrap()
}
#[test]
fn the_witnessed_timeline_carries_no_psg_value() {
    // The PSG match ended 1-4 with PSG scoring four; the witnessed Sevilla
    // match ends 1-1 with the witnessed goals at minutes 27 and 52.
    let mut c = preparing();
    let mut engine = LiveWitnessed::new(&c.editor.campaign).unwrap();
    let prefix = engine.pause().unwrap();
    assert!(c.accept_interval(c.job_id, prefix));
    half_time(&mut c);
    c.key(Key::Enter);
    c.key(Key::Enter);
    let completed = engine.finish(false).unwrap();
    let goals: Vec<&Value> = completed
        .timeline
        .frames
        .iter()
        .flat_map(|frame| &frame.events)
        .filter(|event| event.native["code"] == 38)
        .map(|event| &event.native)
        .collect();
    assert_eq!(goals.len(), 2);
    assert_eq!(goals[0]["minute"], 27);
    assert_eq!(goals[0]["club"], 130);
    assert_eq!(goals[1]["minute"], 52);
    assert_eq!(goals[1]["club"], 176);
    assert_eq!(completed.timeline.frames.last().unwrap().view.score, [1, 1]);
}
