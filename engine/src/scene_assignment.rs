//! [confirmed] F26/F28 at archaeology218490d: fixed-match8599/8663 closure.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub club: u8,
    pub age: u8,
    pub role: u8,
    pub style: u8,
    pub value: u16,
    pub hand_selector: u8,
    pub hand_cell: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub roster: [u16; 11],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Count {
    pub id: u16,
    pub index40: u16,
    pub index42: u16,
    pub cell40: u8,
    pub cell42: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub index: u8,
    pub selector: u8,
    pub seed: u16,
    pub position_default: u16,
    pub position12: u16,
    pub directions: [u8; 4],
    pub clubs: [Club; 2],
    pub players: Vec<Player>,
    pub club_counts: Vec<Count>,
    pub scenes: Vec<Scene>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub helpers: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn assign(_: &State) -> Result<Outcome, String> {
    Err("T18 native8599 not implemented".into())
}
pub fn rebuild(_: &State) -> Result<Outcome, String> {
    Err("T18 native8663 not implemented".into())
}
