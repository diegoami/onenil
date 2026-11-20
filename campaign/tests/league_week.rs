//! [confirmed:archaeology T105 3a0f14d] The league campaign seed: the
//! witnessed league week posts the captured result and the captured cash
//! movement through one transaction, the table reproduces through T104's
//! model, the post-state round-trips, and every foreign value is refused.
use onenil_campaign::league_campaign::LeagueSeed;
use onenil_campaign::league_table::LeagueRound;
use onenil_campaign::{Campaign, MatchTicket};

fn seed() -> LeagueSeed {
    LeagueSeed::pinned().expect("the committed league seed is pinned and valid")
}

fn pre_week() -> Campaign {
    Campaign::initial_league().expect("the seed's pre-week machine is admitted")
}

const WITNESSED_RETURN: &str = "0AB9:EBE2";

#[test]
fn the_committed_seed_is_pinned() {
    let seed = seed();
    assert_eq!(seed.schema, "league-campaign/1");
    assert_eq!(seed.cash.before, 1_045_060);
    assert_eq!(seed.cash.total_in, 726_972);
    assert_eq!(seed.cash.total_out, 80_639);
    assert_eq!(seed.cash.after, 1_691_393);
    assert_eq!(seed.prepared_fixture.opponent, 122);
    assert_eq!(seed.prepared_fixture.name, "Newcastle Utd");
    assert_eq!(seed.prepared_fixture.week, 53);
    assert_eq!(seed.witnessed.match_no, 28);
    assert_eq!(seed.witnessed.score, [0, 2]);
    assert_eq!(seed.witnessed.attendance, 36_986);
}

#[test]
fn initial_league_projects_the_seed_field_for_field() {
    let campaign = pre_week();
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/league-campaign.json"))
            .expect("the committed fixture parses");
    let block = &fixture["campaign"];
    assert_eq!(block["week_index"], 54);
    assert_eq!(block["cash"], 1_045_060);
    assert_eq!(block["players"].as_array().expect("the roster").len(), 15);
    assert_eq!(block["next_fixture"]["opponent"], 103);
    assert_eq!(block["next_fixture"]["opponent_name"], "Chelsea");
    assert_eq!(block["next_fixture"]["slot"], 55);
    assert_eq!(
        campaign
            .source_projection()
            .expect("the pre-week state projects"),
        *block,
        "the admitted machine must be the seed's campaign block"
    );
    assert_eq!(campaign.revision, 0);
    assert!(campaign.league_played.is_none());
}

#[test]
fn the_witnessed_week_completes_the_campaign() {
    let mut campaign = pre_week();
    let ticket: MatchTicket = campaign
        .match_ticket()
        .expect("the pre-week machine admits a ticket");
    campaign
        .record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_972,
            80_639,
            0,
            103,
            0,
            "Chelsea",
        )
        .expect("the witnessed values complete the week");
    assert_eq!(campaign.cash, 1_691_393);
    assert_eq!(
        campaign.week_index, 54,
        "the pointer was already pre-advanced"
    );
    assert_eq!(campaign.next_fixture.opponent, 103);
    assert_eq!(campaign.next_fixture.slot, 55, "the live row is unchanged");
    let played = campaign
        .league_played
        .as_ref()
        .expect("the result is recorded");
    assert_eq!(played.score, [0, 2]);
    assert_eq!(played.prepared.name, "Newcastle Utd");
    assert_eq!(played.request_revision, 0);
    assert_eq!(campaign.revision, 1);
    // Persist/reload: the completed state round-trips byte-identically.
    let bytes = campaign.encode().expect("the completed state encodes");
    let reloaded = Campaign::decode(&bytes).expect("the completed state reloads");
    assert_eq!(reloaded, campaign);
    let projection = campaign
        .source_projection()
        .expect("the post state projects");
    assert_eq!(projection["cash"], 1_691_393);
}

#[test]
fn an_edit_before_the_match_is_admitted() {
    let mut campaign = pre_week();
    campaign
        .set_position(2582, 1)
        .expect("the pre-week machine admits squad edits");
    let ticket = campaign
        .match_ticket()
        .expect("the edited machine admits a ticket");
    campaign
        .record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_972,
            80_639,
            0,
            103,
            0,
            "Chelsea",
        )
        .expect("the witnessed week still completes");
    let played = campaign
        .league_played
        .as_ref()
        .expect("the result is recorded");
    assert_eq!(played.request_revision, 1);
    assert_eq!(campaign.revision, 2);
}

#[test]
fn the_table_reproduces_through_the_model() {
    let round = LeagueRound::captured().expect("the captured round is committed and valid");
    assert_eq!(round.match_no, seed().witnessed.match_no);
    let applied = round.apply_round();
    for club in &round.clubs {
        let (_, row) = applied
            .iter()
            .find(|(name, _)| name == &club.club)
            .expect("captured result names a captured club");
        assert_eq!(row, &club.post, "{}'s after-round row", club.club);
    }
    // A modern three-point rule must not reproduce the captured table; the
    // week transaction's gate would refuse there.
    let modern = round.apply_with(3, 1);
    assert!(
        round.clubs.iter().any(|club| {
            modern
                .iter()
                .find(|(name, _)| name == &club.club)
                .map(|(_, row)| row != &club.post)
                .unwrap_or(true)
        }),
        "a three-point rule must disagree with the captured table"
    );
}

#[test]
fn foreign_week_values_are_refused() {
    let wrong_cash = {
        let mut campaign = pre_week();
        let ticket = campaign.match_ticket().unwrap();
        campaign.record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_971,
            80_639,
            0,
            103,
            0,
            "Chelsea",
        )
    };
    assert_eq!(
        wrong_cash.err().map(|e| e.to_string()),
        Some("the league week's authenticated movement differs".into())
    );
    let wrong_score = {
        let mut campaign = pre_week();
        let ticket = campaign.match_ticket().unwrap();
        campaign.record_league_week(
            &ticket,
            [2, 0],
            WITNESSED_RETURN,
            726_972,
            80_639,
            0,
            103,
            0,
            "Chelsea",
        )
    };
    assert_eq!(
        wrong_score.err().map(|e| e.to_string()),
        Some("the witnessed league result differs".into())
    );
    let wrong_return = {
        let mut campaign = pre_week();
        let ticket = campaign.match_ticket().unwrap();
        campaign.record_league_week(
            &ticket,
            [0, 2],
            "0AB9:0000",
            726_972,
            80_639,
            0,
            103,
            0,
            "Chelsea",
        )
    };
    assert_eq!(
        wrong_return.err().map(|e| e.to_string()),
        Some("the witnessed league result differs".into())
    );
    let wrong_opponent = {
        let mut campaign = pre_week();
        let ticket = campaign.match_ticket().unwrap();
        campaign.record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_972,
            80_639,
            0,
            134,
            0,
            "Chelsea",
        )
    };
    assert_eq!(
        wrong_opponent.err().map(|e| e.to_string()),
        Some("the league week's following fixture differs".into())
    );
    let friendly_competition = {
        let mut campaign = pre_week();
        let ticket = campaign.match_ticket().unwrap();
        campaign.record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_972,
            80_639,
            1,
            103,
            0,
            "Chelsea",
        )
    };
    assert_eq!(
        friendly_competition.err().map(|e| e.to_string()),
        Some("the league week's following fixture differs".into())
    );
    let wrong_name = {
        let mut campaign = pre_week();
        let ticket = campaign.match_ticket().unwrap();
        campaign.record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_972,
            80_639,
            0,
            103,
            0,
            "Chelsea FC",
        )
    };
    assert_eq!(
        wrong_name.err().map(|e| e.to_string()),
        Some("the league week's following fixture differs".into())
    );
    let wrong_flags = {
        let mut campaign = pre_week();
        let ticket = campaign.match_ticket().unwrap();
        campaign.record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_972,
            80_639,
            0,
            103,
            1,
            "Chelsea",
        )
    };
    assert_eq!(
        wrong_flags.err().map(|e| e.to_string()),
        Some("the league week's following fixture differs".into())
    );
}

#[test]
fn a_second_week_is_refused() {
    let mut campaign = pre_week();
    let ticket = campaign
        .match_ticket()
        .expect("the pre-week machine admits a ticket");
    campaign
        .record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_972,
            80_639,
            0,
            103,
            0,
            "Chelsea",
        )
        .expect("the witnessed week completes");
    let stale = campaign.record_league_week(
        &ticket,
        [0, 2],
        WITNESSED_RETURN,
        726_972,
        80_639,
        0,
        103,
        0,
        "Chelsea",
    );
    assert_eq!(
        stale.err().map(|e| e.to_string()),
        Some("this fixture already has a recorded result".into())
    );
    let replay = campaign.match_ticket();
    assert_eq!(
        replay.err().map(|e| e.to_string()),
        Some("this fixture already has a recorded result".into())
    );
}

#[test]
fn a_sevilla_ticket_never_posts_into_the_league_week() {
    let mut campaign = pre_week();
    let ticket = campaign
        .match_ticket()
        .expect("the league machine admits a ticket");
    let refused = campaign.record_sevilla_result(&ticket, [1, 0], WITNESSED_RETURN, &[]);
    assert_eq!(
        refused.err().map(|e| e.to_string()),
        Some("match completion is stale, foreign or unfinished".into())
    );
}

#[test]
fn forged_league_states_are_refused_on_reload() {
    let mut campaign = pre_week();
    let ticket = campaign
        .match_ticket()
        .expect("the pre-week machine admits a ticket");
    campaign
        .record_league_week(
            &ticket,
            [0, 2],
            WITNESSED_RETURN,
            726_972,
            80_639,
            0,
            103,
            0,
            "Chelsea",
        )
        .expect("the witnessed week completes");
    let bytes = campaign.encode().expect("the completed state encodes");

    let mutate = |edit: &dyn Fn(&mut serde_json::Value)| -> Result<Campaign, String> {
        let mut value: serde_json::Value =
            serde_json::from_slice(&bytes).expect("the state parses");
        edit(&mut value);
        Campaign::decode(
            serde_json::to_vec(&value)
                .expect("the forged state serializes")
                .as_slice(),
        )
    };

    // Cash without the recorded result, and the unmoved cash beside a result.
    assert!(mutate(&|v| { v["league_played"] = serde_json::Value::Null }).is_err());
    assert!(mutate(&|v| { v["cash"] = 1_045_060.into() }).is_err());

    // A reversed or foreign result, a foreign return, a broken request chain.
    assert!(
        mutate(&|v| { v["league_played"]["score"] = serde_json::json!([2, 0]) }).is_err(),
        "a reversed score must not reload"
    );
    assert!(mutate(&|v| { v["league_played"]["native_return"] = "0AB9:0000".into() }).is_err());
    assert!(mutate(&|v| { v["league_played"]["request_sha256"] = "nothex".into() }).is_err());
    assert!(mutate(&|v| { v["league_played"]["request_revision"] = 9.into() }).is_err());

    // A foreign prepared row, week, live fixture, club or revision chain.
    assert!(mutate(&|v| { v["prepared_fixture"]["name"] = "Sunderland".into() }).is_err());
    assert!(mutate(&|v| { v["week_index"] = 55.into() }).is_err());
    assert!(mutate(&|v| { v["next_fixture"]["opponent"] = 134.into() }).is_err());
    assert!(mutate(&|v| { v["club"]["name"] = "Sunderland".into() }).is_err());
    assert!(mutate(&|v| { v["revision"] = 0.into() }).is_err());

    // A Sevilla-style recorded result has no place in the league line.
    assert!(
        mutate(&|v| {
            v["played_fixture"] = serde_json::json!({
                "fixture": {
                    "slot": 2, "club": 130, "opponent": 176, "opponent_name": "Sevilla",
                    "competition_code": 1, "flags": 0, "venue_code": 0, "opponent_flags": 0
                },
                "season": {"counter": 1, "start_year": 1994, "end_year": 1995},
                "week_index": 1,
                "request_revision": 0,
                "request_sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "score": [1, 0],
                "native_return": "0AB9:EBE2"
            });
        })
        .is_err(),
        "a Sevilla record must not reload in the league line"
    );
}
