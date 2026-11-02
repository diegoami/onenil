//! [confirmed] F31/F32 at archaeology218490d: bounded1539 helper/store closure.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedInput {
    pub club: u16,
    pub managed_clubs: [u8; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SlotInput {
    pub side: u8,
    pub kind: u8,
    pub club: u16,
    pub roster: [u16; 11],
    pub cache: [[u16; 5]; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_5: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoostInput {
    pub side: u8,
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub cache: [[u16; 5]; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
}
#[derive(Debug, Serialize)]
pub struct Output {
    pub value: u16,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: Output,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct Visit {
    pub site: &'static str,
    pub store_site: &'static str,
    pub arguments: [u16; 3],
    pub input: Scene,
    pub rating: u16,
    pub output: Scene,
    pub draw_start: u8,
    pub draw_end: u8,
}
#[derive(Debug, Serialize)]
pub struct BoostOutcome {
    pub output: BoostInput,
    pub visits: Vec<Visit>,
    pub draws: Vec<Draw>,
}
pub fn managed(_: &ManagedInput) -> Result<Outcome, String> {
    Err("T14 native0E7A not implemented".into())
}
pub fn slot(_: &SlotInput) -> Result<Outcome, String> {
    Err("T14 native577F not implemented".into())
}
pub fn boost(_: &BoostInput) -> Result<BoostOutcome, String> {
    Err("T14 native1539 rating stores not implemented".into())
}
