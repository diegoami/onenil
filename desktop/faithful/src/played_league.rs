//! [confirmed:archaeology T102/T103/T104] The played round-28 league table and
//! results: T104's captured rows/order and T103's eight results.
//! [designed] Read-only presentation; no new comparator or week.
use onenil_campaign::{league_table, Campaign};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PlayedRow {
    pub position: u8,
    pub club: String,
    pub row: league_table::Row,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PlayedLeague {
    pub rows: Vec<PlayedRow>,
    pub results: Vec<league_table::Score>,
}

/// The league campaign after its witnessed week; the unplayed or friendly
/// campaigns return None and keep the T116 screens.
pub fn played_league(campaign: &Campaign) -> Option<PlayedLeague> {
    if !crate::week::is_league(campaign) || campaign.league_played.is_none() {
        return None;
    }
    let round = league_table::LeagueRound::captured().ok()?;
    let order = league_table::OrderWeek::captured(round.match_no).ok()?;
    let rows: std::collections::BTreeMap<String, league_table::Row> =
        round.apply_round().into_iter().collect();
    let mut table = vec![];
    for (club, position) in order.displayed() {
        table.push(PlayedRow {
            position,
            club: club.to_owned(),
            row: rows.get(club)?.clone(),
        });
    }
    Some(PlayedLeague {
        rows: table,
        results: round.results,
    })
}
