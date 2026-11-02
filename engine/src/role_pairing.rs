//! [confirmed] F29/T29 at archaeology218490d: bounded1000/10B9 role pairing.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Locals {
    pub own: [u8; 11],
    pub opponent: [u8; 11],
    pub assigned: [u8; 11],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RangeInput {
    pub low: u16,
    pub high: u16,
    pub locals: Locals,
    pub preferences: [[u8; 13]; 4],
}
#[derive(Debug, Serialize)]
pub struct RangeOutcome {
    pub output: Locals,
    pub draws: Vec<Draw>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub role: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub roster: [u16; 11],
    pub byte57: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub clubs: [Club; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub preferences: [[u8; 13]; 4],
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub ranges: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn range(_: &RangeInput) -> Result<RangeOutcome, String> {
    Err("T16 native1000 not implemented".into())
}
pub fn pair(_: &State) -> Result<Outcome, String> {
    Err("T16 native10B9 not implemented".into())
}
