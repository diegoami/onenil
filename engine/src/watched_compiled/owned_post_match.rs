//! [confirmed: research ef8dad0/T87] Early observations for the T45 league source.
//! [designed:T84] Only input values enter admission; late results are test references.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub field: String,
    pub size: u8,
    pub value: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    pub source_sha256: String,
    pub fields: Vec<Field>,
}
pub const SOURCE_SHA: &str = "d301ca8d2cb507522b8f2ab8882520e86047bb43671a957714ede0f5138d435a";
pub fn original_admission() -> Admission {
    let r: serde_json::Value = serde_json::from_str(include_str!(
        "../../tests/fixtures/post_match_incoming/incoming-r0001.json"
    ))
    .unwrap();
    Admission {
        source_sha256: SOURCE_SHA.into(),
        fields: r["payload"]["fields"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| Field {
                field: f["field"].as_str().unwrap().into(),
                size: f["size"].as_u64().unwrap() as u8,
                value: f["value"].as_u64().unwrap() as u16,
            })
            .collect(),
    }
}
