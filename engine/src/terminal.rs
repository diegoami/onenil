//! [confirmed] Archaeology F34/F35 at218490d; bounded terminal helpers only.
use crate::{placement, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub index: u8,
    pub word0: u16,
    pub word2: u16,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub byte7: u8,
    pub word8: u16,
    pub word10: u16,
    pub word12: u16,
    pub word14: u16,
    pub byte16: u8,
    pub byte17: u8,
    pub byte18: u8,
    pub byte19: u8,
    pub word20: u16,
    pub byte22: u8,
    pub word23: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub position: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub position: u8,
    pub word: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Selectors {
    pub near: [u8; 2],
    pub ahead: [u8; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub actor: u8,
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub positions: Vec<Position>,
    pub signs: [u8; 2],
    pub target: [u8; 2],
    pub phase: u8,
    pub time: u16,
    pub selectors: Selectors,
}
#[derive(Debug, Serialize)]
pub struct PositionsOutcome {
    pub output: State,
    pub visits: Vec<Value>,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct ProximityOutcome {
    pub output: State,
    pub visits: Vec<Value>,
    pub initializations: Vec<Value>,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct TerminalOutcome {
    pub positions: PositionsOutcome,
    pub proximity: ProximityOutcome,
}
pub fn positions(_: &State) -> Result<PositionsOutcome, String> {
    Err("T10 native3155 not implemented".into())
}
pub fn proximity(_: &State) -> Result<ProximityOutcome, String> {
    Err("T10 native21D7 not implemented".into())
}
pub fn terminal(_: &State) -> Result<TerminalOutcome, String> {
    Err("T10 shared image not implemented".into())
}
