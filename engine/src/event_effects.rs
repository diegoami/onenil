//! [confirmed] 69B8 quick logical effect closure: archaeology T19/F19.
//! [designed] AI roster-changing calls diagnose; render/timer effects stay external.
use crate::{
    extra_event,
    goal_effects::{Commentary, Queue},
    Draw,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_0: u8,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_4: u8,
    pub p8_5: u8,
    pub p8_6: u8,
    pub p8_7: u8,
    pub p15_6: u8,
    pub p15_8: u8,
    pub p15_10: u8,
    pub p15_11: u8,
    pub p15_14: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Roster {
    pub club: u16,
    pub players: [u16; 20],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub index: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Selectors {
    pub head: u16,
    pub cells: [Cell; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneExtra {
    pub index: u8,
    pub word8: u16,
    pub motion: u8,
    pub motion_word: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Weight {
    pub code: u8,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub handoff: extra_event::State,
    pub players: Vec<Player>,
    pub rosters: [Roster; 2],
    pub selectors: Selectors,
    pub managed_clubs: [u8; 2],
    pub mode: u8,
    pub pending: u8,
    pub cf53: u8,
    pub sub_flags: [u8; 2],
    pub reserves: [u8; 2],
    pub added: u16,
    pub active: u8,
    pub control: u8,
    pub clear_word: u16,
    pub target_word: u16,
    pub scene_extra: Vec<SceneExtra>,
    pub counters: [[u8; 3]; 2],
    pub cache: [[u8; 20]; 2],
    pub event_totals: [[u16; 20]; 2],
    pub weights: [Weight; 3],
    pub event_counts: [u16; 2],
    pub queue: Queue,
    pub source_pair: [u16; 2],
    pub target_pair: [u16; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Selector {
    pub site: String,
    pub head_before: u16,
    pub head_after: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Action {
    pub routine: String,
    pub scene: u8,
    pub input: State,
    pub output: State,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: State,
    pub selectors: Vec<Selector>,
    pub draws: Vec<Draw>,
    pub actions: Vec<Action>,
    pub helpers: Vec<serde_json::Value>,
    pub events: Vec<Commentary>,
    pub external_calls: Vec<serde_json::Value>,
}
pub fn handler(_input: &State) -> Result<Outcome, String> {
    Err("T07 69B8 effects not implemented".into())
}
