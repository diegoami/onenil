//! [confirmed:archaeology T100/T119/T102/T123] T119: the league campaign's
//! preparation is the exact T119 fixture, the native quick engine plays it to
//! the witnessed result, and the existing week transaction posts the week.
use onenil_campaign::Campaign;
use onenil_desktop::campaign_match::{self, Profile};
use onenil_engine::quick_match;

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
