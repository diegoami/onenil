//! [confirmed] ArchaeologyF39 and6F98/1998:084B at d9e20a4.
//! [designed] Standalone original movement/legality kernel,not a watched match.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub coordinate: u16,
    pub occupant: u8,
    pub tag: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub scene: Scene,
    pub active: u8,
    pub actor: u8,
    pub secondary: u8,
    pub role: u8,
    pub blocking_mask: [u8; 32],
    pub cell: Option<Cell>,
    pub seed: u16,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub allowed: u8,
    pub draws: Vec<Draw>,
}
pub fn advance(_s: &State) -> Result<Outcome, String> {
    Err("unimplemented original watched movement".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_watched_movement_calls() {
        for source in [
            include_str!("../tests/fixtures/watched_movement/cases.jsonl"),
            include_str!("../tests/fixtures/watched_movement/active-cases.jsonl"),
        ] {
            for line in source.lines() {
                let value: serde_json::Value = serde_json::from_str(line).unwrap();
                let input: State = serde_json::from_value(value["input"].clone()).unwrap();
                let out = advance(&input).unwrap();
                assert_eq!(serde_json::to_value(out).unwrap(), value["output"]);
            }
        }
    }
}
