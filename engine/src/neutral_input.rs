//! [confirmed static] 78B7/DB68 at archaeology218490d; no-key branch only.
//! [designed] Presentation events replace original rendering/hardware work;no dynamic census.
use crate::Draw;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub quick_mode: u8,
    pub mirror_mode: u8,
    pub auto_switch: u8,
    pub remaining: u16,
    pub active: u8,
    pub d0b9: u8,
    pub speaker_gate: u8,
    pub speed: u8,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub presentation: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn neutral(_: &State) -> Result<Outcome, String> {
    Err("T23 neutral78B7 not implemented".into())
}
