//! [confirmed] F30/T30 at archaeology218490d: bounded scene/cache code refresh.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeInput {
    pub scene: Scene,
    pub a: u16,
    pub b: u16,
    pub threshold: u8,
    pub clock: u16,
    pub direction_signs: [u8; 2],
    pub codes: [u8; 2],
}
#[derive(Debug, Serialize)]
pub struct CodeOutput {
    pub value: u16,
    pub scene: Scene,
}
#[derive(Debug, Serialize)]
pub struct CodeOutcome {
    pub output: CodeOutput,
    pub draws: Vec<Draw>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub role: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cache {
    pub index: u8,
    pub word0: u16,
    pub word2: u16,
    pub word4: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub actor: u8,
    pub directions: [u8; 4],
    pub mode: u8,
    pub threshold: u8,
    pub clock: u16,
    pub direction_signs: [u8; 2],
    pub codes: [u8; 6],
    pub role_codes: [u8; 14],
    pub cache: [Cache; 3],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub events: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn code(_: &CodeInput) -> Result<CodeOutcome, String> {
    Err("T17 native0143 not implemented".into())
}
pub fn refresh(_: &State) -> Result<Outcome, String> {
    Err("T17 native03BB not implemented".into())
}
