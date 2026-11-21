//! [confirmed:archaeology T106 8d1edcd] The second witnessed league week: the
//! flagged machine refuses PLAY GAME with the game's own message, the witnessed
//! squad exchange and week transaction complete the chain, and everything
//! foreign is refused.
use onenil_campaign::league_campaign::LeagueWeek2;
use onenil_campaign::Campaign;

const INELIGIBLE: &str = "Blackburn R has an ineligible player in the first team : hit any key";
const THIRD_WEEK: &str = "The next fixture (Wimbledon) has no authenticated original match evidence; playing it is unwitnessed.";

fn committed() -> serde_json::Value {
    serde_json::from_str(include_str!("../fixtures/league-week-2.json"))
        .expect("the committed fixture parses")
}

const RETURN: &str = "0AB9:EBE2";

fn seed2() -> LeagueWeek2 {
    LeagueWeek2::pinned().expect("the committed week-2 fixture is pinned and valid")
}

fn week28_done() -> Campaign {
    let mut c = Campaign::initial_league().expect("the league seed machine is admitted");
    let ticket = c
        .match_ticket()
        .expect("the pre-week machine admits a ticket");
    c.record_league_week(
        &ticket,
        [0, 2],
        RETURN,
        726_972,
        80_639,
        0,
        103,
        0,
        "Chelsea",
    )
    .expect("the witnessed week-28 values complete the week");
    c
}

fn flagged() -> Campaign {
    let mut c = week28_done();
    c.apply_league_week_processing()
        .expect("the pinned processing applies to the completed machine");
    c
}

fn exchanged() -> Campaign {
    let mut c = flagged();
    c.record_league_exchange()
        .expect("the witnessed exchange applies to the flagged machine");
    c
}

#[test]
fn the_committed_week2_fixture_is_pinned() {
    let seed2 = seed2();
    assert_eq!(seed2.schema, "league-week-2/1");
    assert_eq!(seed2.witnessed.match_no, 29);
    assert_eq!(seed2.witnessed.score, [3, 3]);
    assert_eq!(seed2.witnessed.attendance, 29_158);
    assert_eq!(seed2.witnessed.cash.before, 1_691_393);
    assert_eq!(seed2.witnessed.cash.total_in, 58_316);
    assert_eq!(seed2.witnessed.cash.total_out, 80_631);
    assert_eq!(seed2.witnessed.cash.after, 1_669_078);
    assert_eq!(seed2.witnessed.following_fixture.opponent, 102);
    assert_eq!(seed2.witnessed.following_fixture.opponent_name, "Wimbledon");
    assert_eq!(seed2.witnessed.following_fixture.slot, 56);
    assert_eq!(seed2.witnessed.following_fixture.venue_code, 1);
    assert_eq!(seed2.witnessed.exchanges.len(), 4);
    let fixture = committed();
    assert_eq!(fixture["source"]["week_index"], 54);
    assert_eq!(fixture["source"]["cash"], 1_691_393);
    assert_eq!(fixture["target"]["week_index"], 55);
    assert_eq!(fixture["target"]["cash"], 1_669_078);
    assert_eq!(
        fixture["target"]["next_fixture"]["opponent_name"],
        "Wimbledon"
    );
}

#[test]
fn the_flagged_machine_refuses_with_the_game_s_own_message() {
    let c = flagged();
    let refused = c.match_ticket();
    assert_eq!(
        refused.err().map(|e| e.to_string()),
        Some(INELIGIBLE.to_string())
    );
}

#[test]
fn the_unprocessed_machine_keeps_t108_s_refusal() {
    let c = week28_done();
    let refused = c.match_ticket();
    assert_eq!(
        refused.err().map(|e| e.to_string()),
        Some("this fixture already has a recorded result".to_string())
    );
}

#[test]
fn the_witnessed_chain_opens_and_completes_the_second_week() {
    let mut c = exchanged();
    let ticket = c
        .match_ticket()
        .expect("the exchanged machine admits the second week");
    assert!(!ticket.is_sevilla());
    c.record_league_week_2(
        &ticket,
        [3, 3],
        RETURN,
        58_316,
        80_631,
        0,
        102,
        1,
        "Wimbledon",
    )
    .expect("the witnessed week-29 values complete the second week");
    assert_eq!(c.cash, 1_669_078);
    assert_eq!(c.week_index, 55, "this pointer was not pre-advanced");
    assert_eq!(c.next_fixture.opponent_name, "Wimbledon");
    assert_eq!(c.next_fixture.slot, 56);
    assert_eq!(c.next_fixture.venue_code, 1);
    let second = c
        .second_league_played
        .as_ref()
        .expect("the second result is recorded");
    assert_eq!(second.score, [3, 3]);
    assert_eq!(second.prepared.name, "Chelsea");
    assert_eq!(second.prepared.week, 54);
    assert_eq!(second.request_revision, 3);
    assert_eq!(c.revision, 4);
    let bytes = c.encode().expect("the completed state encodes");
    let reloaded = Campaign::decode(&bytes).expect("the completed state reloads");
    assert_eq!(reloaded, c);
}

#[test]
fn an_edit_before_the_exchange_is_admitted() {
    let mut c = week28_done();
    c.set_position(2582, 1).expect("squad edits stay admitted");
    c.apply_league_week_processing()
        .expect("the processing follows the edits");
    c.record_league_exchange()
        .expect("the exchange follows the processing");
    let ticket = c
        .match_ticket()
        .expect("the exchanged machine admits the second week");
    c.record_league_week_2(
        &ticket,
        [3, 3],
        RETURN,
        58_316,
        80_631,
        0,
        102,
        1,
        "Wimbledon",
    )
    .expect("the witnessed week still completes");
    assert_eq!(c.revision, 5);
}

#[test]
fn foreign_week2_values_are_refused() {
    let attempt = |score, native_return, total_in, total_out, comp, opp, flags, name| {
        let mut c = exchanged();
        let ticket = c.match_ticket().unwrap();
        c.record_league_week_2(
            &ticket,
            score,
            native_return,
            total_in,
            total_out,
            comp,
            opp,
            flags,
            name,
        )
    };
    assert_eq!(
        attempt([0, 0], RETURN, 58_316, 80_631, 0, 102, 1, "Wimbledon")
            .err()
            .map(|e| e.to_string()),
        Some("the witnessed league result differs".to_string())
    );
    assert_eq!(
        attempt([3, 3], "0AB9:0000", 58_316, 80_631, 0, 102, 1, "Wimbledon")
            .err()
            .map(|e| e.to_string()),
        Some("the witnessed league result differs".to_string())
    );
    assert_eq!(
        attempt([3, 3], RETURN, 58_315, 80_631, 0, 102, 1, "Wimbledon")
            .err()
            .map(|e| e.to_string()),
        Some("the league week's authenticated movement differs".to_string())
    );
    assert_eq!(
        attempt([3, 3], RETURN, 58_316, 80_632, 0, 102, 1, "Wimbledon")
            .err()
            .map(|e| e.to_string()),
        Some("the league week's authenticated movement differs".to_string())
    );
    assert_eq!(
        attempt([3, 3], RETURN, 58_316, 80_631, 0, 103, 1, "Wimbledon")
            .err()
            .map(|e| e.to_string()),
        Some("the league week's following fixture differs".to_string())
    );
    assert_eq!(
        attempt([3, 3], RETURN, 58_316, 80_631, 1, 102, 1, "Wimbledon")
            .err()
            .map(|e| e.to_string()),
        Some("the league week's following fixture differs".to_string())
    );
    assert_eq!(
        attempt([3, 3], RETURN, 58_316, 80_631, 0, 102, 0, "Wimbledon")
            .err()
            .map(|e| e.to_string()),
        Some("the league week's following fixture differs".to_string())
    );
    assert_eq!(
        attempt([3, 3], RETURN, 58_316, 80_631, 0, 102, 1, "Wimbledon FC")
            .err()
            .map(|e| e.to_string()),
        Some("the league week's following fixture differs".to_string())
    );
}

#[test]
fn the_exchange_and_processing_apply_once() {
    let mut c = flagged();
    c.record_league_exchange()
        .expect("the exchange applies once");
    assert_eq!(
        c.record_league_exchange().err().map(|e| e.to_string()),
        Some("the exchange applies to the flagged, unexchanged week-28 machine".to_string())
    );
    assert_eq!(
        c.apply_league_week_processing()
            .err()
            .map(|e| e.to_string()),
        Some("the week processing applies to the completed week-28 machine".to_string())
    );
}

#[test]
fn exchanging_before_the_processing_is_refused() {
    let mut c = week28_done();
    assert_eq!(
        c.record_league_exchange().err().map(|e| e.to_string()),
        Some("the exchange applies to the flagged, unexchanged week-28 machine".to_string())
    );
}

#[test]
fn the_third_week_is_refused() {
    let mut c = exchanged();
    let ticket = c.match_ticket().unwrap();
    c.record_league_week_2(
        &ticket,
        [3, 3],
        RETURN,
        58_316,
        80_631,
        0,
        102,
        1,
        "Wimbledon",
    )
    .expect("the witnessed week completes");
    let refused = c.match_ticket();
    assert_eq!(
        refused.err().map(|e| e.to_string()),
        Some(THIRD_WEEK.to_string())
    );
}

#[test]
fn forged_week2_states_are_refused_on_reload() {
    let mut c = exchanged();
    let ticket = c.match_ticket().unwrap();
    c.record_league_week_2(
        &ticket,
        [3, 3],
        RETURN,
        58_316,
        80_631,
        0,
        102,
        1,
        "Wimbledon",
    )
    .expect("the witnessed week completes");
    let bytes = c.encode().expect("the completed state encodes");
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
    assert!(
        mutate(&|v| { v["cash"] = 1_691_393.into() }).is_err(),
        "the unmoved week"
    );
    assert!(
        mutate(&|v| { v["second_league_played"] = serde_json::Value::Null }).is_err(),
        "the second result lost"
    );
    assert!(
        mutate(&|v| { v["week_index"] = 54.into() }).is_err(),
        "the rolled-back week byte"
    );
    assert!(
        mutate(&|v| { v["second_league_played"]["score"] = serde_json::json!([0, 0]) }).is_err()
    );
    assert!(
        mutate(&|v| { v["second_league_played"]["prepared"]["name"] = "Arsenal".into() }).is_err()
    );
    assert!(mutate(&|v| { v["league_played"]["score"] = serde_json::json!([2, 0]) }).is_err());
    assert!(mutate(&|v| { v["next_fixture"]["opponent"] = 103.into() }).is_err());
    assert!(
        mutate(&|v| {
            let ft = v["club"]["first_team"].as_array_mut().expect("the roster");
            let i = ft
                .iter()
                .position(|p| *p == 2597)
                .expect("Wright is fielded");
            ft[i] = 2585.into();
        })
        .is_err(),
        "the unexchanged roster with the moved cash"
    );
}
