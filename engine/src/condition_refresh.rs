//! [confirmed: archaeology 83b7ef7, T83/T84] Original 1998:14A5 routine contract.
//! [designed:T81] Isolated adapter; campaign/caller composition remains separate.
use crate::Draw;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub player: u16,
    pub club: u8,
    pub age: u8,
    pub age_reference: u8,
    pub facility: u8,
    pub caps: [u8; 2],
    pub base_condition: i8,
    pub condition: i8,
    pub recovery: i8,
    pub flag: u8,
    pub seed: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Output {
    pub base_condition: i8,
    pub condition: i8,
    pub recovery: i8,
    pub seed: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Write {
    pub field: String,
    pub before: u16,
    pub after: u16,
    pub size: u8,
}

#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: Output,
    pub draws: Vec<Draw>,
    pub writes: Vec<Write>,
}

pub fn replay(_input: &Input) -> Result<Outcome, String> {
    Err("original condition refresh is not implemented".into())
}
