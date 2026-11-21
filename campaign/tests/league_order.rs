//! [confirmed:archaeology T110 fda53b8] The league table's display order: the
//! decoded comparator must reproduce both witnessed weeks' sixteen displayed
//! positions, the goal-difference tie-break must be load-bearing, and the
//! 8-bit goal-difference compare must hold as the static reading recorded it.
use onenil_campaign::league_table::{
    goal_difference_low_byte, order_division, OrderRow, OrderWeek,
};
use std::cmp::Reverse;

fn displayed_order(week: &OrderWeek) -> Vec<(&str, u8)> {
    week.displayed()
}

fn naive_points_then_goals_for(rows: &[OrderRow]) -> Vec<(&str, u8)> {
    // A modern reimplementation's guess: points descending, then goals for.
    let mut rows: Vec<&OrderRow> = rows.iter().collect();
    rows.sort_by_key(|row| (Reverse(row.pts), Reverse(row.goals_for)));
    rows.iter()
        .enumerate()
        .map(|(position, row)| (row.club.as_str(), position as u8 + 1))
        .collect()
}

#[test]
fn week_28s_sixteen_displayed_positions_are_reproduced() {
    let week = OrderWeek::captured(28).expect("the week-28 table is committed and valid");
    assert_eq!(
        week.order(),
        displayed_order(&week),
        "the comparator must reproduce every displayed position at week 28"
    );
}

#[test]
fn week_29s_sixteen_displayed_positions_are_reproduced() {
    let week = OrderWeek::captured(29).expect("the week-29 table is committed and valid");
    assert_eq!(
        week.order(),
        displayed_order(&week),
        "the comparator must reproduce every displayed position at week 29"
    );
}

#[test]
fn the_goal_difference_tie_break_is_load_bearing() {
    // Week 28 displays Q.P.R (F40) above Chelsea (F41) at 31 points, so a
    // goals-for tie-break alone must NOT reproduce the table - otherwise the
    // reproduction would prove nothing, as T104's points test argues.
    let week = OrderWeek::captured(28).unwrap();
    let naive = naive_points_then_goals_for(&week.rows);
    assert_ne!(
        naive,
        displayed_order(&week),
        "a goals-for tie-break must disagree with the displayed table"
    );
    assert_eq!(
        week.order(),
        displayed_order(&week),
        "the decoded comparator must still reproduce it"
    );
}

#[test]
fn fewest_goals_against_is_not_the_tie_break() {
    // Week 28 displays Wimbledon (A49) above Q.P.R (A37) and Chelsea (A40) at
    // 31 points: fewest goals against cannot be the rule.
    let week = OrderWeek::captured(28).unwrap();
    let position = |club: &str| -> u8 {
        week.order()
            .into_iter()
            .find(|(name, _)| *name == club)
            .map(|(_, position)| position)
            .unwrap()
    };
    assert!(position("Wimbledon") < position("Q.P.R"));
    assert!(position("Wimbledon") < position("Chelsea"));
}

#[test]
fn the_goal_difference_compare_is_eight_bits() {
    // F41's static-only quirk: the comparator computes F-A in 16 bits but only
    // the signed low byte survives. +130 (F 140, A 10) carries -126 and sorts
    // below +100 (F 120, A 20) at equal points, though its raw difference is
    // larger. The captured tables never exercise this (range -44..+37).
    assert_eq!(goal_difference_low_byte(140, 10), -126);
    assert_eq!(goal_difference_low_byte(120, 20), 100);
    let rows = vec![
        OrderRow {
            club: "big".into(),
            pts: 50,
            goals_for: 140,
            goals_against: 10,
            position: 1,
        },
        OrderRow {
            club: "small".into(),
            pts: 50,
            goals_for: 120,
            goals_against: 20,
            position: 2,
        },
    ];
    let ordered = order_division(&rows);
    assert_eq!(
        ordered,
        vec![("small", 1), ("big", 2)],
        "+130 sorts as its low byte -126, below +100"
    );
}

#[test]
fn different_points_never_swap_on_goal_difference() {
    // The bubble only acts within equal points: the quicksort owns the points
    // order, so 40 points with GD -5 stays above 39 points with GD +30.
    let rows = vec![
        OrderRow {
            club: "top".into(),
            pts: 40,
            goals_for: 30,
            goals_against: 35,
            position: 1,
        },
        OrderRow {
            club: "below".into(),
            pts: 39,
            goals_for: 70,
            goals_against: 40,
            position: 2,
        },
    ];
    assert_eq!(
        order_division(&rows),
        vec![("top", 1), ("below", 2)],
        "a points difference outranks any goal difference"
    );
}

#[test]
fn goals_for_decides_only_within_equal_points_and_goal_difference() {
    // Synthetic third-level witnessed shape: equal points, equal 8-bit goal
    // difference, goals for decides - the unsigned 16-bit compare at 01EF:4414.
    let rows = vec![
        OrderRow {
            club: "fewer".into(),
            pts: 40,
            goals_for: 50,
            goals_against: 40,
            position: 1,
        },
        OrderRow {
            club: "more".into(),
            pts: 40,
            goals_for: 90,
            goals_against: 80,
            position: 2,
        },
    ];
    assert_eq!(
        order_division(&rows),
        vec![("more", 1), ("fewer", 2)],
        "+10 and +10 tie on goal difference; 90 goals for outranks 50"
    );
}

#[test]
fn the_positions_are_exactly_one_through_sixteen() {
    for week in [28, 29] {
        let table = OrderWeek::captured(week).unwrap();
        assert_eq!(table.rows.len(), 16);
        let mut positions: Vec<u8> = table.rows.iter().map(|row| row.position).collect();
        positions.sort_unstable();
        assert_eq!(positions, (1..=16).collect::<Vec<u8>>());
        assert_eq!(table.order().len(), 16);
    }
}
