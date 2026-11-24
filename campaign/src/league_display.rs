//! [confirmed: archaeology W033, runs/T49/session-r0002 steps 23-28] The
//! unplayed division tables and the empty League results notice, exactly as the
//! original displays them after a friendly. Read-only: no played row, tie-break
//! or scheduling rule lives here.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const EMPTY_RESULTS: &str = "No league results to see : hit any key";
pub const COLUMNS: [&str; 10] = ["P", "HW", "HD", "HL", "AW", "AD", "AL", "F", "A", "PTS"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Division {
    pub name: String,
    pub clubs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRecord {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub research_repository: String,
    pub research_commit: String,
    pub records: Vec<SourceRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueDisplay {
    pub schema: String,
    pub source: Source,
    pub columns: Vec<String>,
    pub divisions: Vec<Division>,
    pub empty_results: String,
}

pub fn league_display() -> Result<LeagueDisplay, String> {
    let value: LeagueDisplay =
        serde_json::from_str(include_str!("../fixtures/league-display.json"))
            .map_err(|error| error.to_string())?;
    if value.schema != "league-display/1" {
        return Err("league display schema differs".into());
    }
    if value.columns != COLUMNS {
        return Err("league display columns differ".into());
    }
    if value.divisions.len() != 2
        || value.divisions[0].name != "PREMIER LEAGUE"
        || value.divisions[1].name != "DIVISION ONE"
    {
        return Err("league display divisions differ".into());
    }
    for division in &value.divisions {
        let unique: BTreeSet<_> = division.clubs.iter().collect();
        if division.clubs.len() != 16 || unique.len() != 16 {
            return Err(format!(
                "{} does not carry 16 distinct clubs",
                division.name
            ));
        }
    }
    if value.empty_results != EMPTY_RESULTS {
        return Err("league empty-results notice differs".into());
    }
    if value.source.records.len() != 3 {
        return Err("league display source records differ".into());
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_captured_display_is_valid_and_ordered() {
        let display = super::league_display().unwrap();
        assert_eq!(display.divisions[0].clubs[0], "Blackburn R");
        assert_eq!(display.divisions[1].clubs[0], "Manchester City");
        assert_eq!(display.columns.len(), 10);
        assert_eq!(display.empty_results, super::EMPTY_RESULTS);
    }
}
