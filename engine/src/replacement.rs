//! [confirmed] F24–F28 at archaeology218490d: bounded logicalAEB8/53FB composition.
use crate::{rearrangement, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FlagsInput {
    pub player: u16,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_5: u8,
    pub p8_6: u8,
    pub flags10: u8,
}
#[derive(Debug, Serialize)]
pub struct FlagsOutcome {
    pub flags10: u8,
    pub draws: Vec<Draw>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub match_state: rearrangement::State,
    pub incoming: u8,
    pub index: u8,
    pub side: u8,
    pub allowance: u8,
    pub sub_flags: [u8; 2],
    pub reserves: [u8; 2],
    pub mode: u8,
    pub position_sentinel: u16,
    pub speaker_gate: u8,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub helpers: Vec<Value>,
    pub presentation: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn flags(_: &FlagsInput) -> Result<FlagsOutcome, String> {
    Err("T20 native53FB not implemented".into())
}
pub fn replace(_: &State) -> Result<Outcome, String> {
    Err("T20 nativeAEB8 not implemented".into())
}
