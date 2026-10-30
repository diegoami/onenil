//! [confirmed] A4F2 post-score strategy closure: archaeology T20/F20.
//! [designed] This is not the surrounding 52B6/6B2F/78B7 implementation.
use crate::Draw;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub age: u8,
    pub flags3: u8,
    pub level: u8,
    pub position: u8,
    pub flags10: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub roster: [u16; 20],
    pub byte51: u8,
    pub byte54: u8,
    pub strategy: u8,
    pub baseline_index: u8,
    pub baseline_value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub scores: [u8; 2],
    pub managed_clubs: [u8; 2],
    pub limits: [u8; 2],
    pub raised: [u8; 2],
    pub lowered: [u8; 2],
    pub clubs: [Club; 2],
    pub players: Vec<Player>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub state: State,
    pub argument: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Path {
    pub routine: String,
    pub side: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raise: Option<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Normalizer {
    pub player: u16,
    pub before: u8,
    pub after: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Getter {
    pub club: u16,
    pub value: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Setter {
    pub club: u16,
    pub value: u8,
    pub input: State,
    pub normalizer_start: usize,
    pub output: State,
    pub normalizer_end: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: State,
    pub draws: Vec<Draw>,
    pub paths: Vec<Path>,
    pub setters: Vec<Setter>,
    pub normalizers: Vec<Normalizer>,
    pub getters: Vec<Getter>,
}
pub fn strategy(_input: &Input) -> Result<Outcome, String> {
    Err("T08 post-score strategy not implemented".into())
}
