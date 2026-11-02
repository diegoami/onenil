//! [confirmed] F33/1455 and8973 at archaeology218490d; bounded role selection.
use crate::{role_scores::TableInput, terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub club: u8,
    pub byte6: u8,
    pub byte8: u8,
    pub flags11: u8,
    pub table: TableInput,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Roster {
    pub club: u16,
    pub roster: [u16; 11],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub kind: u8,
    pub criterion: u16,
    pub side: u8,
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub clubs: [u16; 2],
    pub starts: [u8; 6],
    pub cache: [[u16; 5]; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub rosters: Vec<Roster>,
    pub bonus: u8,
    pub criterion_cell: u8,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub clear: Value,
    pub visits: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn select(_: &State) -> Result<Outcome, String> {
    Err("T13 native1455/8973 not implemented".into())
}
