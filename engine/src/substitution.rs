//! [confirmed] F22/F23 at archaeology218490d: fixed-reference6B2F composition.
use crate::{
    ai_selection::{Context, ReadSet},
    Draw,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub context: Context,
    pub replacement: Option<ReadSet>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: Context,
    pub replacement: Option<ReadSet>,
    pub events: Vec<Value>,
    pub ai: Vec<Value>,
    pub presentation: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn dispatch(_: &State) -> Result<Outcome, String> {
    Err("T22 native6B2F not implemented".into())
}
