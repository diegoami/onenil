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
    pub fn new_edited(input: Input) -> Result<Self, String> {
        Ok(Self {
            session: setup::Session::new_edited_watched(input.setup, input.incoming)?,
        })
    }
    pub fn new_fresh(input: Input) -> Result<Self, String> {
        Ok(Self {
            session: setup::Session::new_fresh_watched(input.setup, input.incoming)?,
        })
    }
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

    fn edited_input() -> Input {
        let value: Value = serde_json::from_str(include_str!(
            "../tests/fixtures/edited_preparation/admission.json"
        ))
        .unwrap();
        serde_json::from_value(value["input"].clone()).unwrap()
    }

    #[test]
    fn edited_original_profile_computes_exact_entry_without_widening_fresh() {
        let input = edited_input();
        assert!(Preparation::new_fresh(input.clone()).is_err());
        assert!(Preparation::new(input.clone()).is_err());
        let prepared = Preparation::new_edited(input).unwrap();
        let original: Value = serde_json::from_str(include_str!(
            "../../records/T66/source-r0001/admission-r0002.json"
        ))
        .unwrap();
        assert_eq!(
            prepared.snapshot().unwrap(),
            original["payload"]["first_entry"]
        );
        assert_eq!(
            prepared.preparation()["draws"].as_array().unwrap().len(),
            258
        );
        let before = prepared.session.match_state.inner.e.image.clone();
        let trace = prepared.preparation();
        assert_eq!(prepared.snapshot().unwrap(), prepared.snapshot().unwrap());
        assert_eq!(trace, prepared.preparation());
        assert_eq!(before, prepared.session.match_state.inner.e.image);
    }

    #[test]
    fn edited_profile_refuses_changed_inputs_and_computed_output_injection() {
        for field in ["fixture", "roster", "role", "seed", "income", "incoming"] {
            let mut input = edited_input();
            match field {
                "fixture" => input.setup.legacy.config[0] ^= 1,
                "roster" => input.setup.legacy.substitution.context.clubs[0]
                    .roster
                    .swap(3, 4),
                "role" => input.setup.legacy.players[0].p8[6] ^= 1,
                "seed" => input.setup.legacy.contest.seed ^= 1,
                "income" => input.setup.legacy_income_total = 25841,
                "incoming" => input.incoming.ball_word7 ^= 1,
                _ => unreachable!(),
            }
            assert!(Preparation::new_edited(input).is_err(), "{field}");
        }
        let mut value = serde_json::to_value(edited_input()).unwrap();
        value["expected_first_entry"] = json!({});
        assert!(serde_json::from_value::<Input>(value).is_err());
        let unchanged: Value = serde_json::from_str(include_str!(
            "../tests/fixtures/fresh_preparation/admission.json"
        ))
        .unwrap();
        assert!(Preparation::new_edited(
            serde_json::from_value(unchanged["input"].clone()).unwrap()
        )
        .is_err());
    }

    #[test]
    fn source_zero_role_exception_keeps_ordinary_and_nonzero_roles_strict() {
        let input = edited_input();
        let assignment = input.setup.replacement.match_state.core.assignment.clone();
        assert!(crate::scene_assignment::cells(&assignment).is_err());
        assert!(crate::scene_assignment::fresh_source_cells(&assignment).is_ok());
        for zero in [true, false] {
            let mut bad = assignment.clone();
            let row = bad
                .players
                .iter_mut()
                .find(|p| (p.id == 0) == zero)
                .unwrap();
            row.role = if zero { 15 } else { 14 };
            assert!(crate::scene_assignment::fresh_source_cells(&bad).is_err());
        }
        let context = input.setup.legacy.substitution.context;
        let state = crate::ai_selection::State {
            scene: context.side_offsets[0] + 1,
            side: 0,
            mode: 0,
            context,
            replacement: None,
        };
        assert!(crate::ai_selection::fresh_source_cells(&state).is_ok());
        for zero in [true, false] {
            let mut bad = state.clone();
            let row = bad
                .context
                .players
                .iter_mut()
                .find(|p| (p.id == 0) == zero)
                .unwrap();
            row.position = if zero { 15 } else { 14 };
            assert!(crate::ai_selection::fresh_source_cells(&bad).is_err());
        }
    }

    #[test]
    fn fresh_original_profile_reaches_the_exact_watched_entry() {
        let value: Value = serde_json::from_str(include_str!(
            "../tests/fixtures/fresh_preparation/admission.json"
        ))
        .unwrap();
        let input: Input = serde_json::from_value(value["input"].clone()).unwrap();
        assert!(Preparation::new(input.clone()).is_err());
        let prepared = Preparation::new_fresh(input).unwrap();
        let original: Value = serde_json::from_str(include_str!(
            "../../records/T60/source-r0001/admission-r0001.json"
        ))
        .unwrap();
        assert_eq!(
            prepared.snapshot().unwrap(),
            original["payload"]["first_entry"]
        );
        assert_eq!(
            prepared.preparation()["draws"].as_array().unwrap().len(),
            258
        );
        let trace = prepared.preparation();
        assert_eq!(trace["points"][2]["caller"]["codes"]["mode"], json!(12));
        assert_eq!(trace["points"][3]["caller"]["codes"]["mode"], json!(0));
        assert_eq!(
            trace["points"][6]["caller"]["ratings"],
            original["payload"]["points"][6]["state"]["caller"]["ratings"]
        );
        let pointer = prepared.session.match_state.inner.e.image.as_ptr();
        let before = prepared.session.match_state.inner.e.image.clone();
        assert_eq!(prepared.snapshot().unwrap(), prepared.snapshot().unwrap());
        assert_eq!(prepared.session.match_state.inner.e.image.as_ptr(), pointer);
        assert_eq!(prepared.session.match_state.inner.e.image, before);
    }

    #[test]
    fn fresh_source_refuses_fixture_side_roster_seed_and_output_alias_changes() {
        let value: Value = serde_json::from_str(include_str!(
            "../tests/fixtures/fresh_preparation/admission.json"
        ))
        .unwrap();
        for field in ["fixture", "side", "roster", "seed", "clock", "quick"] {
            let mut input: Input = serde_json::from_value(value["input"].clone()).unwrap();
            match field {
                "fixture" => input.setup.legacy.config[0] = 6,
                "side" => input
                    .setup
                    .legacy
                    .substitution
                    .context
                    .managed_clubs
                    .swap(0, 1),
                "roster" => input.setup.legacy.substitution.context.clubs[0]
                    .roster
                    .swap(0, 1),
                "seed" => input.setup.legacy.contest.seed ^= 1,
                "clock" => input.setup.legacy.clock.clock.kind = 45,
                "quick" => input.setup.legacy.contest.k = 8,
                _ => unreachable!(),
            }
            assert!(Preparation::new_fresh(input).is_err(), "{field}");
        }
        let mut bad = value["input"].clone();
        bad["expected_first_entry"] = json!({});
        assert!(serde_json::from_value::<Input>(bad).is_err());
        let mut bad = value["input"].clone();
        bad["setup"]
            .as_object_mut()
            .unwrap()
            .remove("legacy_queue_zero");
        assert!(serde_json::from_value::<Input>(bad).is_err());
    }

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
