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
    pub managed_selection_mode: u8,
    pub legacy_income_total: u32,
    pub legacy_queue_zero: crate::goal_effects::QueueRow,
    pub skill_tables: Vec<role_scores::TableInput>,
    pub setup_cells: Cells,
    pub income: Vec<Income>,
    pub supplement: Supplement,
}
/// [confirmed] Original09E9 preserves these incoming fields before watched play.
#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WatchedIncoming {
    pub ball_byte6: u8,
    pub ball_word7: u16,
    pub ball_word9: u16,
    pub scores: [u8; 2],
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
        m.inner.e.image[0xD18] = 1;
        m.replacement = m.current();
        assert_eq!(m.replacement.match_state.cache.gate, 1);
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
    fn native_preparation_reaches_ready_match_then_actual_full_time() {
        let data: Data =
            serde_json::from_str(include_str!("../tests/fixtures/setup/admission.json")).unwrap();
        let mut session = Session::new(data).unwrap();
        assert_eq!(session.draws.len(), 258);
        assert_eq!(session.match_state.inner.e.word(crate::SEED), 53858);
        assert_eq!(session.income_results, vec![3330, 20771]);
        assert_eq!(
            session.match_state.snapshot()["clock"]["remaining"],
            json!(91)
        );
        assert_eq!(session.match_state.snapshot()["clock"]["elapsed"], json!(0));
        let mut count = 0;
        while session.match_state.phase() != "full-time" {
            if session.match_state.phase() == "half-time" {
                session.match_state.continue_without_changes().unwrap();
            }
            session.match_state.step().unwrap();
            count += 1;
            assert!(count < 200);
        }
        assert_eq!(count, 95);
        assert_eq!(
            session.match_state.result().unwrap(),
            json!({"clubs":[134,130],"goals":[1,3]})
        );
    }
    #[test]
    fn setup_input_guards_reject_incomplete_and_conflicting_data() {
        let source = include_str!("../tests/fixtures/setup/admission.json");
        let data = || serde_json::from_str::<Data>(source).unwrap();
        let mut a = data();
        a.legacy.substitution.context.clubs[0].roster[0] = 0;
        assert!(Session::new(a).err().unwrap().contains("zero selected"));
        let mut a = data();
        a.legacy.substitution.context.clubs[0].roster[0] =
            a.legacy.substitution.context.clubs[1].roster[0];
        assert!(Session::new(a).err().unwrap().contains("foreign"));
        let mut a = data();
        a.legacy.substitution.context.clubs[0].roster[1] =
            a.legacy.substitution.context.clubs[0].roster[0];
        assert!(Session::new(a).err().unwrap().contains("duplicated"));
        let mut a = data();
        a.supplement.capacities.pop();
        assert!(Session::new(a).err().unwrap().contains("incomplete"));
        let mut a = data();
        a.income[0].table.pop();
        assert!(Session::new(a).err().unwrap().contains("incomplete"));
        let mut a = data();
        a.legacy.clock.seed ^= 1;
        assert!(Session::new(a).err().unwrap().contains("seed aliases"));
        let mut a = data();
        a.managed_selection_mode = 0;
        assert!(Session::new(a).err().unwrap().contains("fallbackB6BD"));
        let mut a = data();
        a.legacy_queue_zero.index = 1;
        assert!(Session::new(a).err().unwrap().contains("queue-zero index"));
        for field in ["expected", "output", "intermediate"] {
            let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
            value[field] = serde_json::json!({});
            assert!(serde_json::from_value::<Data>(value).is_err());
        }
        let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .remove("managed_selection_mode");
        assert!(serde_json::from_value::<Data>(value).is_err());
    }
    #[test]
    fn preparation_preserves_nonstarter_canonical_bytes_and_legacy_until_owned() {
        let data: Data =
            serde_json::from_str(include_str!("../tests/fixtures/setup/admission.json")).unwrap();
        let starters = data
            .legacy
            .substitution
            .context
            .clubs
            .iter()
            .flat_map(|c| c.roster[..11].iter().copied())
            .collect::<std::collections::BTreeSet<_>>();
        let legacy = data.legacy.players.clone();
        let session = Session::new(data).unwrap();
        for p in legacy.iter().filter(|p| !starters.contains(&p.id)) {
            let a = crate::layout::P15 + 15 * usize::from(p.id);
            assert_eq!(session.match_state.inner.e.image[a..a + 15], p.p15);
        }
        for point in &session.points[..4] {
            assert_eq!(point["income_total"], json!(30989));
        }
        for point in &session.points[4..] {
            assert_eq!(point["income_total"], json!(24101));
        }
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

use crate::{goal_effects, layout, placement, scene_assignment, substitution, Draw, Engine};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn club(id: u16) -> usize {
    layout::CLUB + 62 * usize::from(id - 1)
}
fn available(e: &Engine, id: u16) -> u8 {
    let p = layout::P15 + 15 * usize::from(id);
    placement::unavailable(&placement::Player {
        id,
        p15_sbyte8: e.image[p + 8] as i8,
        p15_byte11: e.image[p + 11],
    })
}
/// [designed] A prepared exhibition retains its original Engine through full time.
pub struct Session {
    pub match_state: match_session::Session,
    points: Vec<Value>,
    draws: Vec<Draw>,
    income_results: Vec<u32>,
}
impl Session {
    pub fn new(data: Data) -> Result<Self, String> {
        Self::new_mode(data, None, None, false, false)
    }
    pub(crate) fn new_watched(data: Data, incoming: WatchedIncoming) -> Result<Self, String> {
        if incoming.scores != data.legacy.contest.goals {
            return Err("incoming watched scratch-score alias differs".into());
        }
        Self::new_mode(data, Some(incoming), None, false, false)
    }
    pub(crate) fn new_fresh_watched(data: Data, incoming: WatchedIncoming) -> Result<Self, String> {
        Self::validate_fresh(&data, incoming)?;
        Self::new_mode(data, Some(incoming), None, false, true)
    }
    pub(crate) fn new_edited_watched(
        data: Data,
        incoming: WatchedIncoming,
    ) -> Result<Self, String> {
        // [confirmed: archaeology T69/T72@f646ac2] Exact supplied edited input.
        Self::validate_edited(&data, incoming)?;
        Self::new_mode(data, Some(incoming), None, false, true)
    }
    fn validate_edited(data: &Data, incoming: WatchedIncoming) -> Result<(), String> {
        let reference: Value = serde_json::from_str(include_str!(
            "../tests/fixtures/edited_preparation/admission.json"
        ))
        .map_err(|error| error.to_string())?;
        if json!({"setup": data, "incoming": incoming}) != reference["input"] {
            return Err("unsupported or conflicting edited campaign incoming profile".into());
        }
        Ok(())
    }
    pub(crate) fn before_edited_watched_call(
        data: Data,
        incoming: WatchedIncoming,
        initial: &crate::role_scores::Cells,
    ) -> Result<Self, String> {
        Self::validate_edited(&data, incoming)?;
        Self::new_mode(data, Some(incoming), Some(initial), true, true)
    }
    fn validate_fresh(data: &Data, incoming: WatchedIncoming) -> Result<(), String> {
        // [confirmed:archaeology T66] This first fresh profile admits only its
        // authenticated incoming values, including unused zero scene scratch.
        // Expected preparation/kickoff output never enters this fixture.
        let reference: Value = serde_json::from_str(include_str!(
            "../tests/fixtures/fresh_preparation/admission.json"
        ))
        .map_err(|error| error.to_string())?;
        let value = json!({"setup": data, "incoming": incoming});
        if value != reference["input"] {
            return Err("unsupported or conflicting fresh campaign incoming profile".into());
        }
        Ok(())
    }
    pub(crate) fn before_fresh_watched_call(
        data: Data,
        incoming: WatchedIncoming,
        initial: &crate::role_scores::Cells,
    ) -> Result<Self, String> {
        Self::validate_fresh(&data, incoming)?;
        Self::new_mode(data, Some(incoming), Some(initial), true, true)
    }
    pub(crate) fn before_watched_call(
        data: Data,
        incoming: WatchedIncoming,
        initial: &crate::role_scores::Cells,
    ) -> Result<Self, String> {
        if incoming.scores != data.legacy.contest.goals {
            return Err("incoming watched scratch-score alias differs".into());
        }
        Self::new_mode(data, Some(incoming), Some(initial), true, false)
    }
    fn new_mode(
        data: Data,
        incoming: Option<WatchedIncoming>,
        initial: Option<&crate::role_scores::Cells>,
        before_call: bool,
        fresh: bool,
    ) -> Result<Self, String> {
        if !fresh {
            Self::validate(&data)?;
        }
        let Data {
            legacy,
            replacement,
            gates,
            skill_tables,
            managed_selection_mode,
            legacy_income_total,
            legacy_queue_zero,
            setup_cells,
            income,
            supplement,
        } = data;
        let m = if fresh {
            match_session::Session::from_fresh_source(
                legacy,
                replacement,
                gates,
                skill_tables,
                initial,
            )?
        } else {
            match_session::Session::from_legacy_with_initial(
                legacy,
                replacement,
                gates,
                skill_tables,
                initial,
            )?
        };
        let mut out = Self {
            match_state: m,
            points: vec![],
            draws: vec![],
            income_results: vec![],
        };
        out.match_state.inner.e.image[layout::CONFIG + 0xF6] = managed_selection_mode;
        put(
            &mut out.match_state.inner.e,
            0xD6E8,
            legacy_income_total as u16,
        );
        put(
            &mut out.match_state.inner.e,
            0xD6EA,
            (legacy_income_total >> 16) as u16,
        );
        let e = &mut out.match_state.inner.e;
        if legacy_queue_zero.index != 0 {
            return Err("legacy queue-zero index differs".into());
        }
        e.image[0xA52A] = legacy_queue_zero.code;
        put(e, 0xA52B, legacy_queue_zero.first_player);
        put(e, 0xA52D, legacy_queue_zero.second_player);
        e.image[0xA52F] = legacy_queue_zero.side;
        out.load_inputs(&setup_cells, &income, &supplement, fresh)?;
        if let Some(support) = initial {
            for (&address, &value) in support {
                if out.match_state.inner.e.image[address] != value {
                    return Err(format!(
                        "initial support aliases selected/setup input at{address:05X}"
                    ));
                }
            }
        }
        if let Some(initial) = incoming {
            let e = &mut out.match_state.inner.e;
            // [designed] Select watched mode before19CA;original T39 intervention.
            e.image[layout::CONFIG + 0xF2] = 1;
            e.image[0xCF4C] = initial.ball_byte6;
            put(e, 0xCF4D, initial.ball_word7);
            put(e, 0xCF4F, initial.ball_word9);
        }
        let result = out.prepare(
            &setup_cells,
            &income,
            &supplement,
            incoming.is_some(),
            before_call,
        );
        out.match_state.inner.e.image.check_initialized()?;
        result?;
        Ok(out)
    }
    fn validate(a: &Data) -> Result<(), String> {
        let ids = a
            .legacy
            .players
            .iter()
            .map(|p| (p.id, p))
            .collect::<BTreeMap<_, _>>();
        let clubs = &a.legacy.substitution.context.clubs;
        if a.legacy.clock.clock.remaining != 0
            || a.legacy.clock.clock.kind != 45
            || a.legacy.config[0] != 6
            || a.gates.quick != 1
            || a.supplement.start_gate != 0
        {
            return Err("unsupported selected-squad preparation profile".into());
        }
        let managed = a
            .legacy
            .substitution
            .context
            .managed_clubs
            .iter()
            .copied()
            .filter(|id| *id != 0)
            .collect::<Vec<_>>();
        if managed.len() != 1
            || u16::from(managed[0]) != clubs[0].id
            || a.legacy.substitution.context.allowance != 1
            || a.setup_cells.reserve_cell.value > 9
        {
            return Err("unsupported managed-side/reserve profile".into());
        }
        let seed = a.legacy.contest.seed;
        if [
            a.legacy.extra.state.seed,
            a.legacy.clock.seed,
            a.legacy.event.handoff.seed,
            a.legacy.reset.strategy.seed,
            a.legacy.substitution.context.seed,
            a.replacement.match_state.core.assignment.seed,
        ]
        .iter()
        .any(|s| *s != seed)
            || a.legacy.ratings.iter().any(|r| r.seed != seed)
        {
            return Err("conflicting incoming legacy seed aliases".into());
        }
        for c in clubs {
            let mut seen = BTreeSet::new();
            for (slot, id) in c.roster.iter().enumerate() {
                if *id == 0 {
                    if slot < 11 {
                        return Err("zero selected starting player".into());
                    }
                    continue;
                }
                let p = ids.get(id).ok_or("missing selected canonical player")?;
                if u16::from(p.p8[0]) != c.id || !seen.insert(*id) {
                    return Err("foreign or duplicated selected player".into());
                }
            }
        }
        let headers = clubs.iter().map(|c| c.id).collect::<Vec<_>>();
        if a.setup_cells
            .club_fields
            .iter()
            .map(|c| c.club)
            .collect::<Vec<_>>()
            != headers
            || a.income.iter().map(|c| c.club).collect::<Vec<_>>() != headers
            || a.supplement
                .capacities
                .iter()
                .map(|c| c.club)
                .collect::<Vec<_>>()
                != headers
            || a.supplement
                .club_factors
                .iter()
                .map(|c| c.club)
                .collect::<Vec<_>>()
                != headers
        {
            return Err("incomplete setup club read set".into());
        }
        if a.setup_cells.scene_mode > 4
            || a.setup_cells.reserve_cell.index
                != u16::from(a.legacy.substitution.context.allowance)
            || a.setup_cells
                .numeric_weights
                .iter()
                .map(|w| w.code)
                .collect::<Vec<_>>()
                != vec![0, 59]
        {
            return Err("incomplete setup cells".into());
        }
        for c in &a.income {
            if c.config != a.legacy.config[0]
                || c.allowance != a.legacy.substitution.context.allowance
                || c.table.len() != 24
                || c.table
                    .iter()
                    .enumerate()
                    .any(|(i, v)| usize::from(v.index) != i)
            {
                return Err("incomplete income raw table/configuration".into());
            }
            if c.club54
                != a.setup_cells
                    .club_fields
                    .iter()
                    .find(|f| f.club == c.club)
                    .unwrap()
                    .byte54
            {
                return Err("income/club byte54 alias differs".into());
            }
        }
        if a.supplement.fixture_factors.len() != 5
            || a.supplement
                .fixture_factors
                .iter()
                .enumerate()
                .any(|(i, c)| usize::from(c.index) != i)
        {
            return Err("incomplete fixture factor cells".into());
        }
        for f in &a.supplement.club_factors {
            let index = f.selector as i8 as i16;
            if f.address != 0x1074u16.wrapping_add(index as u16)
                || f.selector
                    != a.setup_cells
                        .club_fields
                        .iter()
                        .find(|c| c.club == f.club)
                        .unwrap()
                        .byte55
            {
                return Err("indexed club factor alias differs".into());
            }
        }
        Ok(())
    }
    fn load_inputs(
        &mut self,
        c: &Cells,
        incomes: &[Income],
        s: &Supplement,
        fresh: bool,
    ) -> Result<(), String> {
        let mut cells = crate::role_scores::Cells::new();
        for w in &c.numeric_weights {
            crate::role_scores::insert(&mut cells, 0x097A + usize::from(w.code), w.value)?;
        }
        let index = c.reserve_cell.index as u8 as i8 as i16;
        crate::role_scores::insert(
            &mut cells,
            usize::from(0xD58u16.wrapping_add(index as u16)),
            c.reserve_cell.value,
        )?;
        if !fresh {
            crate::role_scores::insert(&mut cells, layout::SCENE_MODE, c.scene_mode)?;
        }
        crate::role_scores::insert(&mut cells, layout::CONFIG + 0xF2, s.start_gate)?;
        for f in &c.club_fields {
            for (off, value) in [
                (37, f.byte37),
                (54, f.byte54),
                (55, f.byte55),
                (57, f.byte57),
            ] {
                crate::role_scores::insert(&mut cells, club(f.club) + off, value)?;
            }
        }
        for input in incomes {
            crate::role_scores::insert(
                &mut cells,
                0xD935 + usize::from(input.club),
                input.strength,
            )?;
            for c in &input.table {
                crate::role_scores::insert(&mut cells, 0x1022 + usize::from(c.index), c.value)?;
            }
            for (i, v) in input.minimums.iter().enumerate() {
                crate::role_scores::insert(&mut cells, 0x1086 + i, *v)?;
            }
        }
        for c in &s.capacities {
            crate::role_scores::insert(&mut cells, 0xD9E5 + usize::from(c.club), c.byte)?;
        }
        for f in &s.club_factors {
            crate::role_scores::insert(&mut cells, usize::from(f.address), f.value)?;
        }
        for c in &s.fixture_factors {
            crate::role_scores::insert(&mut cells, 0x107C + usize::from(c.index), c.value)?;
        }
        for (address, value) in cells {
            self.match_state.inner.e.image[address] = value;
        }
        self.match_state.inner.validate_admission()?;
        Ok(())
    }
    fn mark(&mut self, site: &str) {
        let caller = self.match_state.point();
        let inner = &self.match_state.inner;
        let e = &inner.e;
        self.points.push(json!({"site":site,"draw":self.draws.len(),"caller":caller,
            "goal":inner.goal(),"event":inner.event(),"extra":inner.extra(),"reset":inner.reset(),"clock":inner.clock(),
            "selectors":e.image[0xCF72..0xD071].to_vec(),"selector_head":word(e,0xD072),
            "config":e.image[layout::CONFIG..layout::CONFIG+3].to_vec(),
            "income_total":u32::from(word(e,0xD6E8))|(u32::from(word(e,0xD6EA))<<16)}));
    }
    fn income(&mut self, input: &Income) -> Result<u32, String> {
        // [confirmed:T38/1B55:2735] All integer operations retain guest32-bit wrapping.
        let factor = if input.allowance == 0 {
            8
        } else if input.fixture == 0 {
            12
        } else {
            15
        };
        let random = self
            .match_state
            .inner
            .e
            .draw(999, "1B55:27B1", &mut self.draws);
        let index = usize::from((u16::from(input.strength) / 18).min(11))
            + if input.fixture_flags & 15 != 0 { 12 } else { 0 };
        let table = self.match_state.inner.e.image[0x1022 + index];
        let price = 31i32 - i32::from(input.club54);
        let raw = u32::from(table)
            .wrapping_mul(u32::from(input.strength))
            .wrapping_mul(factor)
            .wrapping_mul(price as u32);
        Ok((((raw as i32) / 30) as u32).wrapping_add(u32::from(random)))
    }
    fn income_parent(&mut self, inputs: &[Income]) -> Result<(), String> {
        for input in inputs {
            let value = self.income(input)?;
            self.income_results.push(value);
        }
        let home = inputs.get(1).ok_or("missing home income")?;
        let mut total = self.income_results[0].wrapping_add(self.income_results[1]);
        let e = &mut self.match_state.inner.e;
        if home.fixture_flags & 15 == 1 {
            let limit = u32::from(e.image[0xD9E5 + usize::from(home.club)]) * 1000;
            if (limit as i32) < (total as i32) {
                let delta = e.draw(50, "1B55:28C2", &mut self.draws);
                total = limit.wrapping_add(1).wrapping_sub(u32::from(delta));
            }
        } else {
            let kind = usize::from(matches!(home.fixture, 3 | 4));
            let limit = u32::from(e.image[0x1086 + kind]);
            if (limit as i32) < ((total as i32) / 1000) {
                let delta = e.draw(500, "1B55:294F", &mut self.draws);
                total = limit.wrapping_mul(1000).wrapping_sub(u32::from(delta));
            }
        }
        put(e, 0xD6E8, total as u16);
        put(e, 0xD6EA, (total >> 16) as u16);
        // [designed] Exhibition projects out the subsequent career-ledger persistence;
        // actual income,total/cap and all reached RNG remain native.
        Ok(())
    }
    fn clear_pair(e: &mut Engine, a: usize) {
        let v = word(e, 0xD32);
        put(e, a, v);
        put(e, a + 2, v);
        e.image[a + 6] = 0;
    }
    fn initialize_match(&mut self, reserve: u8) -> Result<(), String> {
        let e = &mut self.match_state.inner.e;
        put(e, 0x0380, 0);
        e.image[0xA530..0xCD08].fill(255);
        e.image[0xD0D2..0xD0E6].fill(0);
        e.image[0xD0E6..0xD128].fill(0);
        e.image[0xD1C0..0xD1C8].fill(0);
        let coin = e.draw(2, "0AB9:8750", &mut self.draws);
        e.image[0xD0AD] = u8::from(coin == 1);
        for a in [0xD0A0, 0xD0A1, 0xD1C8, 0xD1CA] {
            e.image[a] = 0;
        }
        let sentinel = word(e, 0xD2E);
        put(e, 0xCF46, sentinel);
        put(e, 0xCF48, sentinel);
        put(e, 0xCF4A, 0x0F07);
        // [confirmed] Original1998:09E9 owns these fields;bytes6..10 survive.
        put(e, 0xCF51, 0);
        e.image[0xCF53] = 0;
        e.image[0xCF54] = 0;
        Self::clear_pair(e, 0xCF3C);
        Self::clear_pair(e, 0xCF56);
        for i in 1..=22 {
            scene_assignment::rebuild_image(e, i, i)?;
        }
        e.image[0xD18] = 0;
        e.image[0xD0B9] = 0;
        e.image[0xD0BA] = 1;
        e.image[0xDD56] = 0;
        put(e, 0xD0AE, 91);
        e.image[0xD0AC] = 45;
        for side in 0..2 {
            e.image[0xD128 + side] = 0;
            Self::clear_pair(e, 0xCF2E + 7 * side);
            e.image[0xD6E0 + side] = reserve;
            e.image[0xD6DE + side] = 0;
            let own = word(e, 0xD6E4 + 2 * side);
            let other = word(e, 0xD6E4 + 2 * (1 - side));
            let managed = |id: u16| e.image[0xDCBA..0xDCBC].iter().any(|v| u16::from(*v) == id);
            e.image[0xD1CE + side] =
                u8::from(managed(own) || e.image[club(own) + 55] != 0 || !managed(other));
        }
        self.match_state.rearrange()?;
        let e = &mut self.match_state.inner.e;
        e.image[0xD6E2] = 0;
        for a in [0xD0B0, 0xD0B2, 0xD0B4] {
            put(e, a, 0);
        }
        for a in [0xD0B8, 0xD0B7, 0xD0BB, 0xD0BC] {
            e.image[a] = 0;
        }
        put(e, 0xD0BE, 100);
        e.image[0xD1CB] = u8::from(
            crate::cache_helpers::managed_image(e, word(e, 0xD6E4)) != 0
                || crate::cache_helpers::managed_image(e, word(e, 0xD6E6)) != 0,
        );
        Ok(())
    }
    fn editor(&mut self) -> Result<(), String> {
        self.match_state.rearrange()?;
        // [confirmed:original D5D6:D5FB..D687;1B55:321F..322B] The
        // displayed managed club is rated into column1 with bonus flag0.
        // Its selected-club side can differ from that display column.
        let e = &self.match_state.inner.e;
        let home = word(e, 0xD6E6);
        let displayed = if crate::cache_helpers::managed_image(e, home) != 0 {
            home
        } else {
            word(e, 0xD6E4)
        };
        let index = self
            .match_state
            .inner
            .a
            .ratings
            .iter()
            .position(|row| row.club == displayed)
            .ok_or("missing displayed club rating input")?;
        let mut input = self.match_state.inner.rating(index);
        input.side = 1;
        input.flag = 0;
        self.match_state.inner.e.rate_side(&input)?;
        self.match_state.project_display_ratings();
        // [designed] No lineup edit after supplied starting squads;actual return key2.
        // [confirmed] 24BC:0C17 stores both normalized-key words (T41 menu proof).
        put(&mut self.match_state.inner.e, 0xDE88, 2);
        put(&mut self.match_state.inner.e, 0xDE8A, 2);
        self.match_state.inner.e.image[0xD18] = 1;
        self.match_state.rearrange()?;
        Ok(())
    }
    fn caller_ready(&mut self) -> Result<(), String> {
        let e = &mut self.match_state.inner.e;
        for side in 0..2 {
            let base = club(word(e, 0xD6E4 + 2 * side));
            for i in 0..20 {
                let id = word(e, base + 2 * i);
                if id != 0 {
                    e.image[0xD198 + 20 * side + i] =
                        e.image[layout::P15 + 15 * usize::from(id) + 6];
                }
            }
            let count = e.image[0xD6E0 + side];
            for slot in 12..12 + usize::from(count) {
                let id = word(e, base + 2 * (slot - 1));
                if e.image[0xD6E0 + side] > 0 && available(e, id) != 0 {
                    e.image[0xD6E0 + side] = e.image[0xD6E0 + side].wrapping_sub(1);
                }
            }
        }
        if e.image[layout::CONFIG + 0xF2] != 0 {
            // [confirmed] E303..E432 bypasses DB68 when watched mode is selected.
            return Ok(());
        }
        // [confirmed:DB68/DBAC] Actual native quick-start state writes.
        e.image[0xD0B8] = 1;
        e.image[0xD074] = 1;
        e.image[0xD075] = 1;
        e.image[0xD1CC] = 0;
        e.image[0xDE88] = 0;
        Ok(())
    }
    fn kickoff(&mut self) -> Result<(), String> {
        let e = &mut self.match_state.inner.e;
        let side = usize::from(e.image[0xD0AD]);
        e.image[0xD09F] = e.image[0xD56 + side].wrapping_add(10);
        let slot = crate::cache_helpers::slot_image(e, side as u8, 1);
        goal_effects::numeric_commentary(e, 59, slot, 0, side as u8);
        Ok(())
    }
    fn pre_loop(&mut self) -> Result<(), String> {
        let e = &mut self.match_state.inner.e;
        e.image[0xD0BA] = 0;
        e.image[0xD0A1] = 0;
        let context = self.match_state.inner.selection();
        let replacement = self.match_state.current();
        let out = substitution::run_with_commentary(
            &mut self.match_state.inner.e,
            &substitution::State {
                context,
                replacement: Some(replacement),
            },
        )?;
        self.draws.extend(out.draws);
        self.match_state.resolve_actor()?;
        self.match_state.inner.e.image[0xCF53] = 0;
        self.match_state.replacement = self.match_state.current();
        Ok(())
    }
    fn prepare(
        &mut self,
        c: &Cells,
        incomes: &[Income],
        _s: &Supplement,
        watched: bool,
        before_call: bool,
    ) -> Result<(), String> {
        self.mark("0AB9:E24C");
        let e = &mut self.match_state.inner.e;
        let seed = u16::from(e.image[layout::CONFIG])
            .wrapping_add(58 * u16::from(e.image[layout::CONFIG + 1]));
        e.put_word(crate::SEED, seed);
        for i in 0..255 {
            let value = e.draw(16, "0AB9:1A01", &mut self.draws);
            e.image[0xCF72 + i] = (value - 1) as u8;
        }
        put(e, 0xD072, word(e, 0xD6E6));
        self.mark("0AB9:E24F");
        let e = &mut self.match_state.inner.e;
        put(e, 0xD0C4, 0);
        put(e, 0xD0C6, 65535);
        for i in 0..40 {
            put(e, 0xD148 + 2 * i, 55);
        }
        e.image[0xD198..0xD1C0].fill(0);
        for a in [0xD142, 0xD0CC, 0xD0CE] {
            put(e, a, 0);
        }
        self.mark("0AB9:86E9");
        // [confirmed:archaeology T66;seg_0AB9_14.asm86E9..86FA] The
        // original now points D0C0 at DCC0+23. Before this store the fresh
        // incoming pointer observes a different byte;retain that old view.
        self.match_state.inner.e.image[layout::SCENE_MODE] = c.scene_mode;
        self.initialize_match(c.reserve_cell.value)?;
        self.mark("0AB9:E2D4");
        self.income_parent(incomes)?;
        self.mark("0AB9:E2D9");
        let e = &mut self.match_state.inner.e;
        e.image[0xD91] = 1;
        e.image[0xCF5E..0xCF72].fill(0);
        self.mark("0AB9:E2EF");
        let e = &mut self.match_state.inner.e;
        e.image[0xD1CC] = 0;
        e.image[0xD074] = 0;
        e.image[0xD075] = 0;
        self.editor()?;
        self.mark("0AB9:E303");
        self.caller_ready()?;
        self.mark("0AB9:E008");
        if before_call {
            return Ok(());
        }
        // [confirmed] E03C..E053 calls1998:186C for every supplied starting XI
        // at archaeology d9e20a4;186C increments P15+4 and word0 if allowance !=0.
        let e = &mut self.match_state.inner.e;
        if e.image[0xD6DD] != 0 {
            for side in 0..2 {
                let base = club(word(e, 0xD6E4 + 2 * side));
                for slot in 0..11 {
                    let id = word(e, base + 2 * slot);
                    let p = layout::P15 + 15 * usize::from(id);
                    e.image[p + 4] = e.image[p + 4].wrapping_add(1);
                    put(e, p, word(e, p).wrapping_add(1));
                }
            }
        }
        self.mark("0AB9:7D86");
        self.kickoff()?;
        self.mark("0AB9:7DC8");
        self.pre_loop()?;
        if watched {
            // [confirmed] 7C5A/1C7E assigns state1 to every physical scene.
            let e = &mut self.match_state.inner.e;
            for index in 1..=22 {
                e.image[0xCCEF + 25 * index + 19] = 1;
            }
            // [confirmed] 75E5/759A initialize both offside records via1ECC.
            for side in [1, 0] {
                let index = usize::from(e.image[0xD56 + side]) + 1;
                let slot = e.image[0xCCEF + 25 * index + 7];
                let sign = crate::terminal::direction_slot_image(e, slot);
                let base = 0xCF2E + 7 * side;
                e.image[base] = sign.wrapping_shl(3);
                e.image[base + 1] = 0;
                put(e, base + 2, word(e, base));
            }
            e.image[0xCF3D] = 0;
            e.image[0xCF3C] = 1;
            let value = word(e, 0xCF3C);
            put(e, 0xCF3E, value);
            put(e, 0xCF43, value);
            self.mark("0AB9:7C67");
        } else {
            self.mark("0AB9:DD02");
        }
        Ok(())
    }
    pub fn preparation(&self) -> Value {
        json!({"points":self.points,"draws":self.draws,"incomes":self.income_results})
    }
}
