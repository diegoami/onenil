//! [confirmed] F28–F35 at archaeology218490d: fixed-match164F shared-state closure.
use crate::{player_cache, role_pairing, scene_assignment, scene_codes, terminal, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8: [u8; 8],
    pub p15: [u8; 15],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Core {
    pub assignment: scene_assignment::State,
    pub players: Vec<Player>,
    pub rosters: [[u16; 20]; 2],
    pub side_offsets: [u8; 2],
    pub actor: u8,
    pub secondary: u8,
    pub cpu_class: u8,
    pub club_totals: [u16; 2],
    pub event_totals: [[u16; 20]; 2],
    pub cache_selectors: [[u8; 20]; 2],
    pub cached_players: [u16; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub core: Core,
    pub pairing: role_pairing::State,
    pub codes: scene_codes::State,
    pub cache: player_cache::State,
    pub terminal: terminal::State,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub events: Vec<Value>,
    pub draws: Vec<Draw>,
}
pub fn rearrange(_: &State) -> Result<Outcome, String> {
    Err("T19 native164F not implemented".into())
}
