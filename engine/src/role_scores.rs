//! [confirmed] F33/9F6E/1AE2 at archaeology218490d; read-only defined-domain role scores.
use crate::{terminal::Scene, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Attributes {
    pub id: u16,
    pub p8_4: u8,
    pub p8_5: u8,
    pub p8_7: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub index: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TableInput {
    pub criterion: u16,
    pub player: Attributes,
    pub first: Cell,
    pub second: Cell,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneInput {
    pub table: TableInput,
    pub scene: Scene,
    pub byte6: u8,
    pub byte8: u8,
    pub bonus: u8,
    pub criterion_cell: u8,
}
#[derive(Debug, Serialize)]
pub struct Output {
    pub value: u16,
}
#[derive(Debug, Serialize)]
pub struct TableOutcome {
    pub output: Output,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct SceneOutcome {
    pub output: Output,
    pub table: TableOutcome,
    pub scene: Scene,
    pub draws: Vec<Draw>,
}
pub fn table(_: &TableInput) -> Result<TableOutcome, String> {
    Err("T12 native9F6E not implemented".into())
}
pub fn scene(_: &SceneInput) -> Result<SceneOutcome, String> {
    Err("T12 native1AE2 not implemented".into())
}
