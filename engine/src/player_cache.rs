//! [confirmed] F31–F33 at archaeology218490d: bounded1539 shared-image parent.
use crate::{role_cache::Roster, role_scores::TableInput, terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub club: u8,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_4: u8,
    pub p8_5: u8,
    pub p8_7: u8,
    pub byte6: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CriterionCell {
    pub criterion: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub gate: u8,
    pub criteria: [u8; 6],
    pub cache: [[u16; 5]; 2],
    pub managed_clubs: [u8; 2],
    pub clubs: [u16; 2],
    pub side_offsets: [u8; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub rosters: Vec<Roster>,
    pub tables: Vec<TableInput>,
    pub bonus: u8,
    pub criterion_cells: Vec<CriterionCell>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub events: Vec<Value>,
    pub roles: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn refresh(_: &State) -> Result<Outcome, String> {
    Err("T15 native1539 parent not implemented".into())
}
