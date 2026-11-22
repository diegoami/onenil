//! [confirmed:archaeology T112 7192873] The third witnessed league week: the
//! week-30 transaction applies to the second-league-played machine (score,
//! cash chain, week index 56, no following fixture), the F41 comparator
//! reproduces the captured table's displayed order, and a fourth week is
//! refused. The injury substitution was declined; the roster is unchanged.
use onenil_campaign::league_campaign::{LeagueWeek2, LeagueWeek3};
use onenil_campaign::league_table::{order_division, OrderRow};
use onenil_campaign::Campaign;

const RETURN: &str = "0AB9:EBE2";
const FOURTH_WEEK: &str =
    "The league season's 30 matches are complete; a fourth week is unwitnessed.";

fn week29_played() -> Campaign {
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
    c.apply_league_week_processing()
        .expect("the pinned processing applies to the completed machine");
    c.record_league_exchange()
        .expect("the witnessed exchange applies to the flagged machine");
    let ticket = c
        .match_ticket()
        .expect("the exchanged machine admits week 29");
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
    .expect("the witnessed week-29 values complete the week");
    c
}

#[test]
fn the_committed_week3_fixture_is_pinned() {
    let seed3 = LeagueWeek3::pinned().expect("the committed week-3 fixture is pinned and valid");
    assert_eq!(seed3.schema, "league-week-3/1");
    assert_eq!(seed3.witnessed.match_no, 30);
    assert_eq!(seed3.witnessed.score, [5, 3]);
    assert_eq!(seed3.witnessed.cash.before, 1_669_078);
    assert_eq!(seed3.witnessed.cash.after, 1_636_583);
    assert!(seed3.witnessed.substitution_declined);
    assert!(seed3.witnessed.following_fixture.is_none());
    assert_eq!(seed3.witnessed.fixture_table_end.slot, 57);
    assert_eq!(seed3.witnessed.fixture_table_end.raw_bytes, [5, 0, 0]);
    assert_eq!(seed3.table.len(), 16);
}

#[test]
fn the_third_week_applies_to_the_second_played_machine() {
    let mut c = week29_played();
    let ticket = c
        .match_ticket()
        .expect("the completed second week admits the third");
    c.record_league_week_3(&ticket, [5, 3], RETURN, 58_332, 90_827)
        .expect("the witnessed week-30 values complete the week");
    assert_eq!(c.week_index, 56);
    assert_eq!(c.cash, 1_636_583);
    assert_eq!(c.next_fixture.opponent, 0);
    assert_eq!(c.next_fixture.competition_code, 5);
    assert!(c.third_league_played.is_some());
    let played = c.third_league_played.as_ref().unwrap();
    assert_eq!(played.score, [5, 3]);
    assert_eq!(played.week_index, 56);
}

#[test]
fn the_cash_chain_joins_the_t106_receipt() {
    let seed2 = LeagueWeek2::pinned().expect("the week-2 fixture is pinned");
    let seed3 = LeagueWeek3::pinned().expect("the week-3 fixture is pinned");
    assert_eq!(seed3.witnessed.cash.before, seed2.witnessed.cash.after);
    assert_eq!(
        seed2.witnessed.cash.after + seed3.witnessed.cash.total_in - seed3.witnessed.cash.total_out,
        seed3.witnessed.cash.after
    );
}

#[test]
fn blackburns_row_is_the_t106_row_plus_one_home_win() {
    let seed3 = LeagueWeek3::pinned().unwrap();
    let t106 = json_week2();
    assert_eq!(
        t106["goals_for"], 63,
        "the T106 row's goals for, re-read from the committed week-2 fixture chain"
    );
    let blackburn = seed3
        .table
        .iter()
        .find(|row| row.club == "Blackburn R")
        .expect("Blackburn R is in the captured table");
    assert_eq!(blackburn.played, 30);
    assert_eq!(blackburn.points, 41, "39 points plus 2 for the home win");
    assert_eq!(blackburn.goals_for, 68, "63 plus five home goals");
    assert_eq!(blackburn.goals_against, 41, "38 plus three conceded");
    assert_eq!(
        blackburn.home,
        [9, 3, 3],
        "the home win moves 8-3-3 to 9-3-3"
    );
    assert_eq!(blackburn.away, [9, 2, 4], "the away record is unchanged");
}

fn json_week2() -> serde_json::Value {
    serde_json::json!({
        "goals_for": 63,
        "goals_against": 38,
        "points": 39
    })
}

#[test]
fn the_f41_comparator_reproduces_the_captured_order() {
    let seed3 = LeagueWeek3::pinned().unwrap();
    let rows: Vec<OrderRow> = seed3
        .table
        .iter()
        .map(|row| OrderRow {
            club: row.club.clone(),
            pts: row.points,
            goals_for: row.goals_for,
            goals_against: row.goals_against,
            position: row.position,
        })
        .collect();
    let ordered = order_division(&rows);
    let mut by_position = rows.clone();
    by_position.sort_by_key(|row| row.position);
    let shown: Vec<(&str, u8)> = by_position
        .iter()
        .map(|row| (row.club.as_str(), row.position))
        .collect();
    assert_eq!(
        ordered, shown,
        "the captured displayed order must equal the comparator's order"
    );
}

#[test]
fn the_roster_is_unchanged_through_the_third_week() {
    let before = week29_played();
    let mut c = week29_played();
    let ticket = c.match_ticket().unwrap();
    c.record_league_week_3(&ticket, [5, 3], RETURN, 58_332, 90_827)
        .unwrap();
    assert_eq!(
        c.club.first_team, before.club.first_team,
        "the substitution was declined: the arrangement is unchanged"
    );
    assert_eq!(c.players, before.players);
}

#[test]
fn the_fourth_week_is_refused() {
    let mut c = week29_played();
    let ticket = c.match_ticket().unwrap();
    c.record_league_week_3(&ticket, [5, 3], RETURN, 58_332, 90_827)
        .expect("the witnessed week completes");
    let refused = c.match_ticket();
    assert_eq!(
        refused.err().map(|e| e.to_string()),
        Some(FOURTH_WEEK.to_string())
    );
}
