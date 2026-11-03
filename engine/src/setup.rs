//! [confirmed] ArchaeologyT38@d9e20a4:selected-squad admission and native preparation.
//! [designed] Career squad/colour preparation is supplied input;one continuing image.
use crate::{ai_selection, continuous, match_session, role_scores};
use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub index: u16,
    pub value: u8,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Weight {
    pub code: u8,
    pub value: u8,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClubFields {
    pub club: u16,
    pub byte37: u8,
    pub byte54: u8,
    pub byte55: u8,
    pub byte57: u8,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cells {
    pub numeric_weights: Vec<Weight>,
    pub reserve_cell: Cell,
    pub scene_mode: u8,
    pub club_fields: Vec<ClubFields>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Income {
    pub club: u16,
    pub config: u8,
    pub fixture: u8,
    pub fixture_flags: u8,
    pub club54: u8,
    pub strength: u8,
    pub allowance: u8,
    pub table: Vec<Cell>,
    pub minimums: [u8; 2],
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Capacity {
    pub club: u16,
    pub byte: u8,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Factor {
    pub club: u16,
    pub selector: u8,
    pub address: u16,
    pub value: u8,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Supplement {
    pub capacities: Vec<Capacity>,
    pub start_gate: u8,
    pub club_factors: Vec<Factor>,
    pub fixture_factors: Vec<Cell>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Data {
    pub legacy: continuous::Admission,
    pub replacement: ai_selection::ReadSet,
    pub gates: match_session::Gates,
    pub skill_tables: Vec<role_scores::TableInput>,
    pub setup_cells: Cells,
    pub income: Vec<Income>,
    pub supplement: Supplement,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn live_views_follow_scene_and_cache_reference_changes() {
        let a: Data =
            serde_json::from_str(include_str!("../tests/fixtures/setup/admission.json")).unwrap();
        let mut m =
            match_session::Session::from_legacy(a.legacy, a.replacement, a.gates, a.skill_tables)
                .unwrap();
        let address = crate::layout::P15 + 15 * 2071;
        let preserved = m.inner.e.image[address..address + 15].to_vec();
        assert_eq!(m.current().match_state.core.players.len(), 47);
        for i in 1..=22 {
            crate::scene_assignment::rebuild_image(&mut m.inner.e, i, i).unwrap();
        }
        m.replacement = m.current();
        assert_eq!(m.replacement.match_state.core.players.len(), 37);
        assert_eq!(m.inner.selection().players.len(), 36);
        assert!(m
            .replacement
            .match_state
            .core
            .players
            .iter()
            .any(|p| p.id == 2070));
        m.inner.e.image[0xCF5E..0xCF72].fill(0);
        m.replacement = m.current();
        assert_eq!(m.replacement.match_state.core.players.len(), 36);
        assert!(!m
            .replacement
            .match_state
            .core
            .players
            .iter()
            .any(|p| p.id == 2070));
        assert_eq!(m.inner.e.image[address..address + 15], preserved);
    }
    #[test]
    fn selected_input_has_old_legacy_not_ready_match() {
        let a: Data =
            serde_json::from_str(include_str!("../tests/fixtures/setup/admission.json")).unwrap();
        assert_eq!(a.legacy.contest.seed, 65341);
        assert_eq!(a.legacy.event.selectors.head, 152);
        assert_eq!(a.legacy.clock.clock.elapsed, 94);
        assert_eq!(a.legacy.clock.clock.remaining, 0);
        assert_eq!(a.legacy.players.len(), 47);
        let session = continuous::Session::new(a.legacy).unwrap();
        assert_eq!(session.snapshot()["seed"], serde_json::json!(65341));
    }
}
