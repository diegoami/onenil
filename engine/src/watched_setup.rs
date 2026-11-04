//! [confirmed] T43 original watched preparation;T28 native selected-squad setup.
//! [designed] Own the computed Engine;observations never reconstruct its state.
use crate::{setup, watched_compiled::watched_match};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub setup: setup::Data,
    pub incoming: setup::WatchedIncoming,
}

pub struct Preparation {
    pub(crate) session: setup::Session,
}
impl Preparation {
    pub fn new(input: Input) -> Result<Self, String> {
        Ok(Self {
            session: setup::Session::new_watched(input.setup, input.incoming)?,
        })
    }
    pub fn snapshot(&self) -> Result<Value, String> {
        watched_match::logical_snapshot(&self.session.match_state.inner.e)
    }
    pub fn preparation(&self) -> Value {
        self.session.preparation()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::path::Path;

    fn fixture() -> Value {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_setup/admission.json");
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
    }
    fn input() -> Input {
        serde_json::from_value(fixture()["input"].clone()).unwrap()
    }
    #[test]
    fn tracked_selected_setup_reaches_pre_call_without_additional_support() {
        let input = input();
        match setup::Session::before_watched_call(input.setup, input.incoming, &Default::default())
        {
            Ok(mut prepared) => {
                let known = prepared
                    .match_state
                    .inner
                    .e
                    .image
                    .take_initialized()
                    .unwrap();
                assert!(known[0xD074]);
                assert!(!known[0x0330]);
                assert_eq!(
                    prepared.preparation()["points"]
                        .as_array()
                        .unwrap()
                        .last()
                        .unwrap()["site"],
                    "0AB9:E008"
                );
            }
            Err(error) => panic!("tracked setup probe: {error}"),
        }
    }
    #[test]
    fn selected_squad_computes_original_draws_and_every_first_watched_entry_field() {
        let original = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("records/T43/original-r0001/result.json");
        let original: Value = serde_json::from_slice(&std::fs::read(original).unwrap()).unwrap();
        let prepared = Preparation::new(input()).unwrap();
        assert_eq!(prepared.snapshot().unwrap(), original["first_entry"]);
        let trace = prepared.preparation();
        assert_eq!(trace["draws"], original["draws"]);
        assert_eq!(trace["points"].as_array().unwrap().len(), 11);
        assert_eq!(trace["points"][10]["site"], "0AB9:7C67");
        let image = prepared.session.match_state.inner.e.image.as_ptr();
        let before = prepared.session.match_state.inner.e.image.clone();
        assert_eq!(prepared.snapshot().unwrap(), original["first_entry"]);
        assert_eq!(prepared.preparation(), trace);
        assert_eq!(prepared.session.match_state.inner.e.image.as_ptr(), image);
        assert_eq!(prepared.session.match_state.inner.e.image, before);
    }
    #[test]
    fn genuinely_incoming_ball_words_survive_owned_initialization() {
        let mut input = input();
        input.incoming.ball_byte6 = 1;
        input.incoming.ball_word7 = 0x1234;
        input.incoming.ball_word9 = 0xABCD;
        let prepared = Preparation::new(input).unwrap();
        let ball = prepared.snapshot().unwrap()["ball"].clone();
        assert_eq!(ball["byte6"], 1);
        assert_eq!(ball["word7"], 0x1234);
        assert_eq!(ball["word9"], 0xABCD);
        assert_eq!(ball["word11"], 0);
        assert_eq!(ball["byte13"], 0);
        assert_eq!(ball["byte14"], 0);
    }
    #[test]
    fn incomplete_computed_outcome_and_inconsistent_selected_inputs_reject() {
        let baseline = fixture()["input"].clone();
        let mut bad = baseline.clone();
        bad["expected_first_entry"] = json!({});
        assert!(serde_json::from_value::<Input>(bad).is_err());
        let mut bad = baseline.clone();
        bad["incoming"]["ball_word11"] = json!(1);
        assert!(serde_json::from_value::<Input>(bad).is_err());
        let mut bad = baseline.clone();
        bad["incoming"]
            .as_object_mut()
            .unwrap()
            .remove("ball_byte6");
        assert!(serde_json::from_value::<Input>(bad).is_err());
        let mut bad: Input = serde_json::from_value(baseline.clone()).unwrap();
        bad.incoming.scores = [1, 0];
        assert!(Preparation::new(bad).is_err());
        for identifier in [0, 3921] {
            let mut bad = baseline.clone();
            bad["setup"]["legacy"]["substitution"]["context"]["clubs"][0]["roster"][0] =
                json!(identifier);
            assert!(Preparation::new(serde_json::from_value(bad).unwrap()).is_err());
        }
    }
}
