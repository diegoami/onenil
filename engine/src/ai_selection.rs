//! [confirmed] F23–F28 at archaeology218490d: boundedA94B shared-image selection.
use crate::{rearrangement, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub roster: [u16; 20],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub index: u8,
    pub slot: u8,
    pub player: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub byte8: u8,
    pub flags11: u8,
    pub position: u8,
    pub style: u8,
    pub hand_selector: u8,
    pub hand_cell: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Preference {
    pub position: u8,
    pub styles: Vec<u8>,
    pub weights: [u8; 14],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub seed: u16,
    pub managed_clubs: [u8; 2],
    pub clubs: [Club; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub side_offsets: [u8; 2],
    pub remaining: u16,
    pub added: u16,
    pub allowance: u8,
    pub sub_flags: [u8; 2],
    pub reserves: [u8; 2],
    pub pending: u8,
    pub control: u8,
    pub key: u8,
    pub event_totals: [[u16; 20]; 2],
    pub limit_cell: u8,
    pub style_text: String,
    pub preferences: Vec<Preference>,
    pub candidates: [u8; 14],
    pub cache_selectors: [[u8; 20]; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReadSet {
    pub match_state: rearrangement::State,
    pub display_mode: u8,
    pub position_sentinel: u16,
    pub speaker_gate: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub context: Context,
    pub mode: u8,
    pub scene: u8,
    pub side: u8,
    pub replacement: Option<ReadSet>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: Context,
    pub replacement: Option<ReadSet>,
    pub helpers: Vec<Value>,
    pub presentation: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn select(_: &State) -> Result<Outcome, String> {
    Err("T21 nativeA94B not implemented".into())
}
