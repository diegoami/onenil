//! [confirmed] Named quick52B6 reset closure: archaeology T21/F21.
//! [designed] Rendering/timers,045C buffers,6B2F/78B7 and full DD02 stay external.
use crate::{post_score, Draw};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub index: u8,
    pub player: u16,
    pub slot: u8,
    pub word0: u16,
    pub word8: u16,
    pub word10: u16,
    pub motion: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub byte6: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub strategy: post_score::State,
    pub mode: u8,
    pub actor: u8,
    pub secondary: u8,
    pub active: u8,
    pub zero: u8,
    pub alternate: u8,
    pub kind: u8,
    pub remaining: u16,
    pub side_offsets: [u8; 2],
    pub signs: [u8; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub cache: [[u8; 20]; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CopyVisit {
    pub index: u8,
    pub word0: u16,
    pub before: u16,
    pub after: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MotionVisit {
    pub index: u8,
    pub before: u8,
    pub after: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Keeper {
    pub input: State,
    pub draw_start: usize,
    pub output: State,
    pub draw_end: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: State,
    pub copies: Vec<CopyVisit>,
    pub motions: Vec<MotionVisit>,
    pub keeper: Option<Keeper>,
    pub strategy_input: Option<State>,
    pub strategy_output: Option<State>,
    pub strategy: Option<post_score::Outcome>,
    pub helpers: Vec<serde_json::Value>,
    pub draws: Vec<Draw>,
}
pub fn reset(_input: &State) -> Result<Outcome, String> {
    Err("T09 quick reset not implemented".into())
}
