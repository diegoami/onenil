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

// [confirmed:archaeology T110 fda53b8 / findings/F41-league-order.md] The
// league table's display order: per division of 16 clubs, a quicksort on the
// points byte (01EF:4227), then a bubble pass over adjacent index slots
// (01EF:4305) that swaps only on equal points and - the signed 8-bit low byte
// of F-A strictly greater, or those bytes equal and goals for strictly greater
// as unsigned 16-bit. The goal difference is compared at 8 bits: the 16-bit
// F-A is computed but only its low byte survives (`mov dl, al`), signed by the
// `jl` that follows. The screen renders the sorted index array in array order.

/// The committed order fixture's own identity, pinned so a drifted row is
/// refused.
const ORDER_SHA: &str = "651611fcefd88a01bb78a13e5cb521b904163ec23d2e6495410970cf546a4195";

/// One club's witnessed week: the decoded row and the position the original
/// displayed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderRow {
    pub club: String,
    pub pts: u8,
    pub goals_for: u16,
    pub goals_against: u16,
    pub position: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderSource {
    research: String,
    receipt: String,
    receipt_sha256: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderWeekSource {
    receipt: String,
    receipt_sha256: String,
    #[serde(default)]
    round_results: Option<String>,
    #[serde(default)]
    round_results_sha256: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderWeekFixture {
    week: u8,
    source: OrderWeekSource,
    rows: Vec<OrderRow>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderFixture {
    schema: String,
    source: OrderSource,
    divisions: u8,
    per_division: u8,
    weeks: Vec<OrderWeekFixture>,
}

/// One witnessed week's table, decoded and validated against its fixture.
#[derive(Debug, Clone)]
pub struct OrderWeek {
    pub week: u8,
    pub rows: Vec<OrderRow>,
}

fn order_fixture() -> Result<OrderFixture, String> {
    let raw = include_bytes!("../fixtures/league-order.json");
    if format!("{:x}", Sha256::digest(raw)) != ORDER_SHA {
        return Err("league-order fixture identity differs".into());
    }
    serde_json::from_slice(raw).map_err(|error| error.to_string())
}

/// [confirmed:archaeology T110] The comparator's goal difference: the signed
/// 8-bit low byte of F - A. A difference of +130 carries -126 here; the
/// captured tables never exercise that (their range is -44..+37), so this is
/// the static reading standing alone.
pub fn goal_difference_low_byte(goals_for: u16, goals_against: u16) -> i8 {
    ((goals_for.wrapping_sub(goals_against)) & 0xFF) as u8 as i8
}

/// 01EF:4227: a Hoare quicksort on the points byte, descending - the low side
/// advances while `points > pivot` and the high side retreats while
/// `points < pivot`, the pivot taken from the middle slot. `index` holds club
/// slots into the division's rows.
fn quicksort_on_points(points: &[u8], index: &mut [u8], lo: isize, hi: isize) {
    if lo >= hi {
        return;
    }
    let pivot = points[index[((lo + hi) >> 1) as usize] as usize];
    let (mut i, mut j) = (lo, hi);
    loop {
        while points[index[i as usize] as usize] > pivot {
            i += 1;
        }
        while points[index[j as usize] as usize] < pivot {
            j -= 1;
        }
        if i <= j {
            index.swap(i as usize, j as usize);
            i += 1;
            j -= 1;
        }
        if i > j {
            break;
        }
    }
    if lo < j {
        quicksort_on_points(points, index, lo, j);
    }
    if i < hi {
        quicksort_on_points(points, index, i, hi);
    }
}

/// 01EF:4305: the bubble pass over adjacent index slots. A pair swaps only
/// when the points are equal and the upper slot's 8-bit goal difference is
/// strictly smaller than the lower slot's - or those bytes are equal and the
/// upper slot has strictly fewer goals for. Clubs with different points never
/// swap here: the quicksort owns the points order.
fn bubble_on_goal_difference(rows: &[OrderRow], index: &mut [u8], lo: u8, hi: u8) {
    if lo + 1 > hi {
        return;
    }
    let mut i = lo + 1;
    loop {
        let mut j = hi;
        loop {
            let k = j - 1;
            let upper = &rows[index[k as usize] as usize];
            let lower = &rows[index[j as usize] as usize];
            if upper.pts == lower.pts {
                let upper_gd = goal_difference_low_byte(upper.goals_for, upper.goals_against);
                let lower_gd = goal_difference_low_byte(lower.goals_for, lower.goals_against);
                if upper_gd < lower_gd
                    || (upper_gd == lower_gd && upper.goals_for < lower.goals_for)
                {
                    index.swap(k as usize, j as usize);
                }
            }
            if j == i {
                break;
            }
            j -= 1;
        }
        if i == hi {
            break;
        }
        i += 1;
    }
}

/// The decoded comparator over one division's rows: the ordered
/// `(club, position 1..=n)` the original's five-group sort produces and the
/// screen displays. No witnessed pair ties in (points, goal difference, goals
/// for), so the witnessed orders are fully determined; clubs that tie that
/// far keep whatever arrangement the quicksort left, exactly as the original's
/// passes do.
pub fn order_division(rows: &[OrderRow]) -> Vec<(&str, u8)> {
    assert!(!rows.is_empty(), "a division sorts at least one club");
    let count = rows.len();
    let mut index: Vec<u8> = (0..count as u8).collect();
    let points: Vec<u8> = rows.iter().map(|row| row.pts).collect();
    quicksort_on_points(&points, &mut index, 0, (count - 1) as isize);
    bubble_on_goal_difference(rows, &mut index, 0, (count - 1) as u8);
    index
        .iter()
        .enumerate()
        .map(|(position, slot)| (rows[*slot as usize].club.as_str(), position as u8 + 1))
        .collect()
}

impl OrderWeek {
    /// [confirmed:archaeology T110] A witnessed week from the committed
    /// fixture, refusing an unknown schema, a foreign shape, or positions
    /// that are not 1..=16.
    pub fn captured(week: u8) -> Result<Self, String> {
        let fixture = order_fixture()?;
        if fixture.schema != "league-order/1" {
            return Err("unsupported league-order schema".into());
        }
        if fixture.divisions != 5 || fixture.per_division != 16 {
            return Err("the original sorts five divisions of sixteen clubs".into());
        }
        let found = fixture
            .weeks
            .iter()
            .find(|entry| entry.week == week)
            .ok_or_else(|| format!("no witnessed week {}", week))?;
        if found.rows.len() != 16 {
            return Err("a witnessed division holds sixteen clubs".into());
        }
        let mut positions: Vec<u8> = found.rows.iter().map(|row| row.position).collect();
        positions.sort_unstable();
        if positions != (1..=16).collect::<Vec<u8>>() {
            return Err("the witnessed positions are not 1..=16".into());
        }
        Ok(Self {
            week: found.week,
            rows: found.rows.clone(),
        })
    }

    /// The displayed order: the week's rows by the position the capture shows.
    pub fn displayed(&self) -> Vec<(&str, u8)> {
        let mut rows: Vec<(&OrderRow, u8)> =
            self.rows.iter().map(|row| (row, row.position)).collect();
        rows.sort_by_key(|(_, position)| *position);
        rows.into_iter()
            .map(|(row, position)| (row.club.as_str(), position))
            .collect()
    }

    /// The comparator's order for the week's rows.
    pub fn order(&self) -> Vec<(&str, u8)> {
        order_division(&self.rows)
    }
}
