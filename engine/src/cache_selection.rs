//! [confirmed] Archaeology F32/13B8/4EBF at218490d; read-only cached-player helpers.
use crate::{placement, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RatingInput {
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_5: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_5: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SelectorInput {
    pub side: u8,
    pub club: u16,
    pub roster: [u16; 11],
    pub players: Vec<Player>,
}
#[derive(Debug, Serialize)]
pub struct RatingOutput {
    pub value: u16,
}
#[derive(Debug, Serialize)]
pub struct SelectorOutput {
    pub player: u16,
}
#[derive(Debug, Serialize)]
pub struct Visit {
    pub slot: u8,
    pub player: u16,
    pub unavailable: u8,
    pub rating: Option<u16>,
    pub selected: bool,
    pub retained_score: u16,
    pub rating_calls: u8,
}
#[derive(Debug, Serialize)]
pub struct RatingOutcome {
    pub output: RatingOutput,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct SelectorOutcome {
    pub output: SelectorOutput,
    pub visits: Vec<Visit>,
    pub draws: Vec<Draw>,
}
pub fn rating(_: &RatingInput) -> Result<RatingOutcome, String> {
    Err("T11 native4EBF not implemented".into())
}
pub fn select(_: &SelectorInput) -> Result<SelectorOutcome, String> {
    Err("T11 native13B8 not implemented".into())
}
