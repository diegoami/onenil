//! [confirmed: archaeology ba499b9/T86] E679 caller-entry contract.
//! [designed:T83] Input-only isolated replay; campaign admission is separate.
use crate::Draw;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub address: usize,
    pub field: String,
    pub size: u8,
    pub value: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Region {
    pub base: usize,
    pub count: usize,
    pub name: String,
    pub origin: usize,
    pub stride: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub registers: [u16; 12],
    pub flags: u16,
    pub regions: Vec<Region>,
    pub initial: Vec<Fact>,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Write {
    pub address: usize,
    pub size: u8,
    pub before: u16,
    pub value: u16,
    pub site: String,
    pub draw: usize,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub final_fields: Vec<Fact>,
    pub draws: Vec<Draw>,
    pub writes: Vec<Write>,
    pub registers: [u16; 12],
}
pub fn replay(_input: &Input) -> Result<Outcome, String> {
    Err("unimplemented original E679 caller".into())
}
