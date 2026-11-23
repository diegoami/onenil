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
    // [review #218] the result stores the REQUEST's week (55, the pre-match
    // value); the machine's own week index (56) lives on the campaign.
    assert_eq!(played.week_index, 55);
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
    // [review #222] the transition derives from the witnessed week-29 row the
    // seed carries (the T106 receipt's own row), not from literals; only the
    // witnessed result supplies the delta.
    let before = &seed3.witnessed.week_29_row;
    assert_eq!(before.club, "Blackburn R");
    let blackburn = seed3
        .table
        .iter()
        .find(|row| row.club == before.club)
        .expect("Blackburn R is in the captured table");
    let [scored, conceded] = seed3.witnessed.score;
    assert_eq!(blackburn.played, before.played + 1);
    assert_eq!(
        blackburn.points,
        before.points + 2,
        "two points for the home win"
    );
    assert_eq!(blackburn.goals_for, before.goals_for + scored);
    assert_eq!(blackburn.goals_against, before.goals_against + conceded);
    assert_eq!(
        blackburn.home,
        [before.home[0] + 1, before.home[1], before.home[2]],
        "the home win's column advances by one"
    );
    assert_eq!(blackburn.away, before.away, "the away record is unchanged");
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
    // [review #219] the witnessed condition refreshes apply: the six players
    // whose condition_signed the post-league30 projection carries change,
    // while the arrangement and every other field stay as they were.
    let changed: Vec<&str> = [
        "H Berg",
        "T Sherwood",
        "S Ripley",
        "T Gale",
        "A Wright",
        "B Mimms",
    ]
    .iter()
    .copied()
    .collect();
    for (after, was) in c.players.iter().zip(before.players.iter()) {
        assert_eq!(after.id, was.id);
        assert_eq!(after.name, was.name);
        if changed.contains(&after.name.as_str()) {
            assert_ne!(
                after.availability.condition_signed, was.availability.condition_signed,
                "{}'s witnessed condition refresh applies",
                after.name
            );
        } else {
            assert_eq!(after.availability, was.availability);
        }
    }
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

#[test]
fn a_foreign_ticket_completing_week_30_is_refused() {
    // [review #218] record_league_week_3 must require the ticket to be this
    // machine's: a pre-week-28 ticket completing week 30 on the
    // second-league-played machine is refused.
    let foreign = Campaign::initial_league().expect("the league seed machine is admitted");
    let foreign_ticket = foreign
        .match_ticket()
        .expect("the pre-week machine admits its own ticket");
    let mut c = week29_played();
    assert!(c
        .record_league_week_3(&foreign_ticket, [5, 3], RETURN, 58_332, 90_827)
        .is_err());
}

#[test]
fn the_third_result_stores_the_request_week() {
    // [review #218] the third result binds the REQUEST's week (55, the
    // pre-match value), the way week 29's result stored 54.
    let mut c = week29_played();
    let ticket = c.match_ticket().unwrap();
    c.record_league_week_3(&ticket, [5, 3], RETURN, 58_332, 90_827)
        .expect("the witnessed week completes");
    let played = c.third_league_played.as_ref().unwrap();
    assert_eq!(
        played.week_index, 55,
        "the result stores the request's week"
    );
    assert_eq!(
        c.week_index, 56,
        "the machine's own week index is post-week"
    );
}

#[test]
fn the_witnessed_condition_refreshes_apply() {
    // [review #219] the six witnessed condition refreshes: the transaction
    // applies the post-league30 projection's condition_signed values.
    let mut c = week29_played();
    let ticket = c.match_ticket().unwrap();
    c.record_league_week_3(&ticket, [5, 3], RETURN, 58_332, 90_827)
        .expect("the witnessed week completes");
    let expected = [
        ("H Berg", 28),
        ("T Sherwood", 27),
        ("S Ripley", 23),
        ("T Gale", 24),
        ("A Wright", 28),
        ("B Mimms", 25),
    ];
    for (name, condition) in expected {
        let player = c
            .players
            .iter()
            .find(|p| p.name == name)
            .unwrap_or_else(|| panic!("the squad carries {name}"));
        assert_eq!(
            player.availability.condition_signed, condition,
            "{name}'s witnessed condition applies"
        );
    }
}

fn week30_played() -> Campaign {
    let mut c = week29_played();
    let ticket = c
        .match_ticket()
        .expect("the completed second week admits the third");
    c.record_league_week_3(&ticket, [5, 3], RETURN, 58_332, 90_827)
        .expect("the witnessed week-30 values complete the week");
    c
}

/// [review #220/#224] A week-3 save edited in memory must refuse through both
/// loaders, exactly as the review's temporary suite exercised them.
fn refuses(name: &str, forged: Campaign) {
    let bytes = serde_json::to_vec(&forged).unwrap();
    assert!(
        Campaign::decode(&bytes).is_err(),
        "{name} accepted by decode"
    );
    let mut target = week30_played();
    assert!(
        target.replace_from_bytes(&bytes).is_err(),
        "{name} accepted through a save image"
    );
    assert_eq!(target, week30_played(), "{name} replaced the live campaign");
}

#[test]
fn forged_third_result_identities_refuse() {
    // [review #220] the re-review's mutations of a valid completed week-30
    // save: each was accepted before this batch.
    let mut c = week30_played();
    c.third_league_played.as_mut().unwrap().request_revision = u64::MAX;
    refuses("request revision", c);

    let mut c = week30_played();
    c.third_league_played.as_mut().unwrap().request_sha256 = "forged".into();
    refuses("request digest", c);

    let mut c = week30_played();
    c.third_league_played.as_mut().unwrap().prepared.week = 0;
    refuses("prepared week", c);

    let mut c = week30_played();
    c.third_league_played.as_mut().unwrap().prepared.name = "Arsenal".into();
    refuses("prepared opponent name", c);

    let mut c = week30_played();
    c.third_league_played.as_mut().unwrap().season.counter = 99;
    refuses("season counter", c);
}

#[test]
fn earlier_results_and_roster_identities_refuse_in_week_three() {
    // [review #220] the week-3 dispatch previously bypassed the earlier
    // results' binding: the first result's score and the history holes.
    let mut c = week30_played();
    c.league_played.as_mut().unwrap().score = [0, 4];
    refuses("first result score", c);

    let mut c = week30_played();
    c.league_played = None;
    refuses("missing first result", c);

    let mut c = week30_played();
    c.second_league_played = None;
    refuses("missing second result", c);

    // [review #224] the roster and player-record identity checks must run on
    // the week-3 path too.
    let mut c = week30_played();
    c.players[0].name = "Forged Name".into();
    refuses("player name", c);

    let mut c = week30_played();
    c.players[0].id += 10_000;
    refuses("player id", c);

    let mut c = week30_played();
    c.players[0].current_position = 14;
    refuses("role code", c);
}
