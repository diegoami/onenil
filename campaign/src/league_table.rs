//! [confirmed:archaeology T103 17cf78c / runs/T103/table-r0001.json] The league
//! table's captured round: the 16 Premier League rows before round 28 and the
//! eight results that produced the captured after-round table. A win is two
//! points and a draw one - Newcastle's 21 wins and 3 draws make 45.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// The committed fixture's own identity, pinned so a drifted row is refused.
const ROUND_SHA: &str = "5ea88d9e174ae96f6e599e7cfe4cfa31fad8feaa692b8c49eba17917f3ccf2af";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Row {
    pub played: u8,
    pub points: u8,
    /// Home wins, draws, losses, in the screen's column order.
    pub home: [u8; 3],
    /// Away wins, draws, losses.
    pub away: [u8; 3],
    pub goals_for: u16,
    pub goals_against: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub club: String,
    pub id: u16,
    pub pre: Row,
    pub post: Row,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Score {
    pub home: String,
    pub away: String,
    pub home_goals: u8,
    pub away_goals: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Points {
    win: u8,
    draw: u8,
    loss: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    research: String,
    receipt: String,
    receipt_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Round {
    schema: String,
    source: Source,
    match_no: u8,
    points: Points,
    results: Vec<Score>,
    clubs: Vec<Club>,
}

/// The captured round, decoded and validated against its own fixture.
#[derive(Debug, Clone)]
pub struct LeagueRound {
    pub match_no: u8,
    pub points: [u8; 3],
    pub results: Vec<Score>,
    pub clubs: Vec<Club>,
}

fn fixture() -> Result<Round, String> {
    let raw = include_bytes!("../fixtures/league-table.json");
    if format!("{:x}", Sha256::digest(raw)) != ROUND_SHA {
        return Err("league-table fixture identity differs".into());
    }
    serde_json::from_slice(raw).map_err(|error| error.to_string())
}

impl LeagueRound {
    /// [confirmed:archaeology T103] The captured round-28 fixture, refusing an
    /// unknown schema, a wrong points rule, or an incomplete table.
    pub fn captured() -> Result<Self, String> {
        let round = fixture()?;
        if round.schema != "league-table-round/1" {
            return Err("unsupported league-table schema".into());
        }
        if [round.points.win, round.points.draw, round.points.loss] != [2, 1, 0] {
            return Err("the captured points rule is two for a win and one for a draw".into());
        }
        if round.clubs.len() != 16 || round.results.len() != 8 {
            return Err("the captured round is incomplete".into());
        }
        Ok(Self {
            match_no: round.match_no,
            points: [2, 1, 0],
            results: round.results,
            clubs: round.clubs,
        })
    }

    /// The after-round rows under the given points values. Two and one are the
    /// captured rule; other values let a test prove the rule is load-bearing.
    pub fn apply_with(&self, win: u8, draw: u8) -> Vec<(String, Row)> {
        let mut rows: Vec<(String, Row)> = self
            .clubs
            .iter()
            .map(|club| (club.club.clone(), club.pre.clone()))
            .collect();
        for result in &self.results {
            let mut update = |name: &str, scored: u16, conceded: u16, at_home: bool| {
                let row = rows
                    .iter_mut()
                    .find(|(club, _)| club == name)
                    .map(|(_, row)| row)
                    .expect("captured result names a captured club");
                row.played += 1;
                row.goals_for += scored;
                row.goals_against += conceded;
                let column = if at_home {
                    &mut row.home
                } else {
                    &mut row.away
                };
                if scored > conceded {
                    row.points += win;
                    column[0] += 1;
                } else if scored == conceded {
                    row.points += draw;
                    column[1] += 1;
                } else {
                    column[2] += 1;
                }
            };
            update(
                &result.home,
                result.home_goals.into(),
                result.away_goals.into(),
                true,
            );
            update(
                &result.away,
                result.away_goals.into(),
                result.home_goals.into(),
                false,
            );
        }
        rows
    }

    /// The after-round rows under the captured rule.
    pub fn apply_round(&self) -> Vec<(String, Row)> {
        self.apply_with(self.points[0], self.points[1])
    }

    /// Whether every captured after-round row is what `apply_round` produces.
    pub fn reproduces_captured_table(&self) -> bool {
        let applied = self.apply_round();
        self.clubs.iter().all(|club| {
            applied
                .iter()
                .find(|(name, _)| name == &club.club)
                .map(|(_, row)| row == &club.post)
                .unwrap_or(false)
        })
    }
}
