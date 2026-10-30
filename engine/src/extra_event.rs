//! [confirmed] DBC6 decision/helpers: archaeology T17/F17 at 321b495.
//! [designed] 69B8 is an emitted external request, not an executed handler.
use crate::Draw;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub mode: u16,
    pub style_cell: u8,
    pub level_cell: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_0: u8,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_4: u8,
    pub p8_5: u8,
    pub p8_7: u8,
    pub p15_6: u8,
    pub p15_8: u8,
    pub p15_10: u8,
    pub p15_11: u8,
    pub skills: [Skill; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub index: u8,
    pub word0: u16,
    pub slot: u8,
    pub player: u16,
    pub rating_bonus: u8,
    pub half: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub byte57: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Flag {
    pub mode: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub actor: u8,
    pub secondary: u8,
    pub d6dd: u8,
    pub home_bonus: u8,
    pub side_offsets: [u8; 2],
    pub opponents: [u8; 10],
    pub clubs: [Club; 2],
    pub side_flags: [u8; 2],
    pub skill_bonus_flags: [Flag; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub state: State,
    pub opponent_d0b: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Helper {
    pub caller: String,
    pub kind: String,
    pub input: serde_json::Value,
    pub result: u16,
    pub draw_position: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Request {
    pub routine: String,
    pub input: State,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: State,
    pub draws: Vec<Draw>,
    pub helpers: Vec<Helper>,
    pub dispatch: Option<Request>,
}

pub fn decision(_input: &Input) -> Result<Outcome, String> {
    Err("T06 DBC6 decision not implemented".into())
}
