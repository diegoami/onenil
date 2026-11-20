//! [confirmed:archaeology T103 17cf78c] The captured round-28 table: applying
//! the eight results to the pre-round rows reproduces the after-round rows.
use onenil_campaign::league_table::LeagueRound;

#[test]
fn the_captured_round_reproduces_the_captured_table() {
    let round = LeagueRound::captured().expect("the captured round is committed and valid");
    assert_eq!(round.match_no, 28);
    assert_eq!(round.clubs.len(), 16);
    assert_eq!(round.results.len(), 8);
    assert!(
        round.reproduces_captured_table(),
        "two points for a win and one for a draw must reproduce every captured row"
    );
}

#[test]
fn the_rule_is_load_bearing() {
    // A modern three-point rule must NOT reproduce the captured table; otherwise
    // the reproduction would prove nothing about the points rule.
    let round = LeagueRound::captured().unwrap();
    let modern: Vec<_> = round.apply_with(3, 1);
    let differs = round.clubs.iter().any(|club| {
        modern
            .iter()
            .find(|(name, _)| name == &club.club)
            .map(|(_, row)| row != &club.post)
            .unwrap_or(true)
    });
    assert!(
        differs,
        "a three-point rule must disagree with the captured table"
    );
}

#[test]
fn a_win_adds_two_a_draw_one_and_a_loss_none() {
    let round = LeagueRound::captured().unwrap();
    let applied: Vec<_> = round.apply_round();
    for club in &round.clubs {
        let (_, row) = applied.iter().find(|(name, _)| name == &club.club).unwrap();
        assert_eq!(row.played, club.pre.played + 1);
        assert_eq!(
            row.home[0] + row.home[1] + row.home[2] + row.away[0] + row.away[1] + row.away[2],
            row.played,
            "the W-D-L columns must cover every match"
        );
        assert_eq!(row.goals_for - club.pre.goals_for, {
            let scored: u16 = round
                .results
                .iter()
                .filter(|result| &result.home == &club.club)
                .map(|result| result.home_goals as u16)
                .chain(
                    round
                        .results
                        .iter()
                        .filter(|result| &result.away == &club.club)
                        .map(|result| result.away_goals as u16),
                )
                .sum();
            scored
        });
    }
}
