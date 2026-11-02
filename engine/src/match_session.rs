//! [confirmed] F07/F09/F12-F35 and T37@461ceb7:fixed-reference caller continuation.
//! [designed] Starts after setup;explicit no-edit halftime response;one continuing image.
use crate::{
    ai_selection, continuous, layout, placement, rearrangement, role_scores, substitution, Engine,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Gates {
    pub cache_gate: u8,
    pub editor: u8,
    pub kickoff_side: u8,
    pub delay_word: u16,
    pub control: u8,
    pub d0a1: u8,
    pub cf53: u8,
    pub active: u8,
    pub quick: u8,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    pub first_half: continuous::Admission,
    pub replacement: ai_selection::ReadSet,
    pub gates: Gates,
    pub skill_tables: Vec<role_scores::TableInput>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    FirstHalf,
    HalfTime,
    SecondHalf,
    FullTime,
}
impl Phase {
    fn name(self) -> &'static str {
        match self {
            Self::FirstHalf => "first-half",
            Self::HalfTime => "half-time",
            Self::SecondHalf => "second-half",
            Self::FullTime => "full-time",
        }
    }
}
pub struct Session {
    inner: continuous::Session,
    replacement: ai_selection::ReadSet,
    phase: Phase,
    points: Vec<Value>,
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn scene(i: u8) -> usize {
    0xCCEF + 25 * usize::from(i)
}
impl Session {
    pub fn new(a: Admission) -> Result<Self, String> {
        if a.gates.quick == 0
            || a.gates.editor != 1
            || a.gates.kickoff_side > 1
            || a.gates.cache_gate != a.replacement.match_state.cache.gate
            || a.first_half.clock.clock.remaining != 91
            || a.first_half.clock.clock.elapsed != 0
        {
            return Err("unsupported full-match initial/caller domain".into());
        }
        let ids = a
            .first_half
            .players
            .iter()
            .map(|p| p.id)
            .collect::<BTreeSet<_>>();
        let mut expected = BTreeSet::new();
        for id in &ids {
            expected.insert((6, *id));
            expected.insert((14, *id));
        }
        let got = a
            .skill_tables
            .iter()
            .map(|t| (t.criterion, t.player.id))
            .collect::<BTreeSet<_>>();
        if got != expected || got.len() != a.skill_tables.len() {
            return Err("incomplete/duplicate full-match skill read set".into());
        }
        let mut inner = continuous::Session::new(a.first_half)?;
        let probe = ai_selection::State {
            context: inner.a.substitution.context.clone(),
            replacement: Some(a.replacement.clone()),
            mode: 0,
            scene: inner.a.substitution.context.side_offsets[0] + 1,
            side: 0,
        };
        let mut cells = ai_selection::cells(&probe)?;
        for t in &a.skill_tables {
            for (address, value) in role_scores::table_cells(t)? {
                role_scores::insert(&mut cells, address, value)?;
            }
        }
        for (address, value) in cells {
            inner.e.image[address] = value;
        }
        for (address, value) in [
            (0xD18, a.gates.cache_gate),
            (0xD1CB, a.gates.editor),
            (0xD0AD, a.gates.kickoff_side),
            (0xD0BA, a.gates.control),
            (0xD0A1, a.gates.d0a1),
            (0xCF53, a.gates.cf53),
            (0xD0B8, a.gates.active),
            (0xD074, a.gates.quick),
        ] {
            inner.e.image[address] = value;
        }
        // [derived:F14/F25] AF7F code0 leaves the captured weighted actor total unchanged;
        // numeric_commentary's signed-byte delta therefore has weight0 in this fixed profile.
        inner.e.image[0x097A] = 0;
        put(&mut inner.e, 0xD0BE, a.gates.delay_word);
        inner.validate_admission()?;
        let managed = inner
            .a
            .substitution
            .context
            .managed_clubs
            .iter()
            .copied()
            .filter(|id| *id != 0)
            .collect::<Vec<_>>();
        if managed.len() != 1 || u16::from(managed[0]) != word(&inner.e, 0xD6E4) {
            return Err("unsupported managed side in no-edit continuation".into());
        }
        Ok(Self {
            inner,
            replacement: a.replacement,
            phase: Phase::FirstHalf,
            points: vec![],
        })
    }
    pub fn phase(&self) -> &'static str {
        self.phase.name()
    }
    pub fn snapshot(&self) -> Value {
        self.inner.snapshot()
    }
    fn gates(&self) -> Gates {
        let e = &self.inner.e;
        Gates {
            cache_gate: e.image[0xD18],
            editor: e.image[0xD1CB],
            kickoff_side: e.image[0xD0AD],
            delay_word: word(e, 0xD0BE),
            control: e.image[0xD0BA],
            d0a1: e.image[0xD0A1],
            cf53: e.image[0xCF53],
            active: e.image[0xD0B8],
            quick: e.image[0xD074],
        }
    }
    fn current(&mut self) -> ai_selection::ReadSet {
        let state = rearrangement::snapshot(&mut self.inner.e, &self.replacement.match_state);
        ai_selection::ReadSet {
            match_state: state,
            display_mode: self.inner.e.image[0xD074],
            position_sentinel: word(&self.inner.e, 0xD2E),
            speaker_gate: self.inner.e.image[0xA2EC],
        }
    }
    fn point(&mut self) -> Value {
        let r = self.current();
        let e = &self.inner.e;
        json!({"core":r.match_state.core,"pairing":r.match_state.pairing,"codes":r.match_state.codes,"selection":ai_selection::snapshot(e,&self.inner.a.substitution.context),"global_read_set":{"display_mode":r.display_mode,"position_sentinel":r.position_sentinel,"speaker_gate":r.speaker_gate},"gates":self.gates(),"ratings":e.ratings_snapshot(),"seed":e.word(crate::SEED)})
    }
    fn transaction(
        &mut self,
        action: fn(&mut Self) -> Result<Value, String>,
    ) -> Result<Value, String> {
        // [designed] Roll back an unsupported reached branch;success keeps the continuing image.
        let image = self.inner.e.image.clone();
        let replacement = self.replacement.clone();
        let phase = self.phase;
        let ended = self.inner.ended;
        let points = self.points.clone();
        match action(self) {
            Ok(v) => Ok(v),
            Err(e) => {
                self.inner.e.image = image;
                self.replacement = replacement;
                self.phase = phase;
                self.inner.ended = ended;
                self.points = points;
                Err(e)
            }
        }
    }
    pub fn step(&mut self) -> Result<Value, String> {
        self.transaction(Self::step_inner)
    }
    fn step_inner(&mut self) -> Result<Value, String> {
        if !matches!(self.phase, Phase::FirstHalf | Phase::SecondHalf) {
            return Err("match awaits a halftime response or has finished".into());
        }
        let r = self.current();
        let mut ai = vec![];
        let mut out = self.inner.step_with_readset(Some(&r), Some(&mut ai))?;
        self.replacement = self.current();
        if out["clock"]["result"] == 1 {
            self.inner.e.image[0xD0B8] = 0; //7CC8,after the observed7C92 clock boundary.
            if self.phase == Phase::FirstHalf && word(&self.inner.e, 0xD0AE) == 45 {
                self.inner.e.image[0xCF53] = 0;
                self.phase = Phase::HalfTime;
            } else if self.phase == Phase::SecondHalf
                && word(&self.inner.e, 0xD0AE).wrapping_add(word(&self.inner.e, 0xD0B4)) == 0
            {
                self.phase = Phase::FullTime;
                let p = self.point();
                self.points.push(p);
            } else {
                return Err("unsupported caller clock boundary".into());
            }
        }
        out["ai"] = json!(ai);
        out["phase"] = json!(self.phase.name());
        Ok(out)
    }
    pub fn continue_without_changes(&mut self) -> Result<Value, String> {
        self.transaction(Self::continue_inner)
    }
    fn rearrange(&mut self) -> Result<(), String> {
        let r = self.current();
        rearrangement::run(&mut self.inner.e, &r.match_state)?;
        self.replacement = self.current();
        Ok(())
    }
    fn continue_inner(&mut self) -> Result<Value, String> {
        if self.phase != Phase::HalfTime {
            return Err("no-edit continuation requires the actual halftime boundary".into());
        }
        let p = self.point();
        self.points.push(p); //7DD3 entry.
                             // [confirmed:T37]7DD3 restores each participating raw condition byte in unsigned1..30.
        for i in 1..=22 {
            let id = word(&self.inner.e, scene(i) + 14);
            let a = layout::P15 + 15 * usize::from(id) + 8;
            let v = self.inner.e.image[a];
            if (1..=30).contains(&v) {
                self.inner.e.image[a] = v.wrapping_add(2);
            }
        }
        self.inner.e.image[0xD0BA] = 1;
        if self.inner.e.image[0xD1CB] != 1 {
            return Err("unsupported halftime editor branch".into());
        }
        // [confirmed:T37;1998:080D ->232A:099B ->24BC:0C17/0C53]
        // Explicit presentation response:Space acknowledges the halftime board.
        put(&mut self.inner.e, 0xDE8A, 0x3920);
        let p = self.point();
        self.points.push(p); //7E4C,beforeD5D6.
        self.rearrange()?; //D5F4.
                           // [confirmed:T37/1B55:321F] Neutral managed display's logical rating writes.
        let r = self.inner.rating(1);
        self.inner.e.rate_side(&r)?;
        self.project_display_ratings();
        // [designed] The supported external response is explicitly no lineup changes.
        // [confirmed:T37] Editor response leaves normalized key2 before its final164F.
        self.inner.e.image[0xDE8A] = 2;
        self.inner.e.image[0xD18] = 1;
        self.rearrange()?; //D8F8/D8FD.
        let p = self.point();
        self.points.push(p); //7E4F.
        let opposite = usize::from(self.inner.e.image[0xD0AD] == 0);
        self.inner.e.image[0xD09F] = self.inner.e.image[0xD56 + opposite].wrapping_add(10);
        put(&mut self.inner.e, 0xD0BE, 80);
        let p = self.point();
        self.points.push(p); //7E6E,before7C3F.
        self.inner.e.image[0xD0BA] = 0;
        self.inner.e.image[0xD0A1] = 0;
        let sub = substitution::State {
            context: ai_selection::snapshot(&self.inner.e, &self.inner.a.substitution.context),
            replacement: Some(self.current()),
        };
        let out = substitution::run_with_commentary(&mut self.inner.e, &sub)?;
        if !out.draws.is_empty() {
            return Err("unexpected caller dispatch RNG".into());
        }
        self.resolve_actor()?;
        self.inner.e.image[0xCF53] = 0;
        self.inner.ended = false;
        self.phase = Phase::SecondHalf;
        self.replacement = self.current();
        Ok(
            json!({"phase":self.phase.name(),"presentation":[{"kind":"halftime-no-lineup-changes"}],"draws":[]}),
        )
    }
    fn project_display_ratings(&mut self) {
        for class in 1..4 {
            let a = usize::from(crate::RATINGS) + 4 * class + 2;
            let value = word(&self.inner.e, a) >> 2;
            put(&mut self.inner.e, a, value);
        }
        for class in 0..4 {
            let a = usize::from(crate::RATINGS) + 4 * class + 2;
            let value = word(&self.inner.e, a).min(256);
            put(&mut self.inner.e, a, value);
        }
    }
    fn resolve_actor(&mut self) -> Result<(), String> {
        // [confirmed:F21/1D3A] Physical cyclic availability fallback;bounded unsupported case.
        if self.inner.e.image[0xD09F] == 0 {
            return Ok(());
        }
        for _ in 0..10 {
            let e = &self.inner.e;
            let actor = e.image[0xD09F];
            let id = word(e, scene(actor) + 14);
            let a = layout::P15 + 15 * usize::from(id);
            if placement::unavailable(&placement::Player {
                id,
                p15_sbyte8: e.image[a + 8] as i8,
                p15_byte11: e.image[a + 11],
            }) == 0
            {
                return Ok(());
            }
            let offset = e.image[0xD56 + usize::from(actor < 12)];
            self.inner.e.image[0xD09F] = if actor == offset + 11 {
                offset + 2
            } else {
                actor + 1
            };
        }
        Err("no eligible kickoff actor in physical cycle".into())
    }
    pub fn caller_points(&self) -> &[Value] {
        &self.points
    }
    pub fn result(&self) -> Result<Value, String> {
        if self.phase != Phase::FullTime {
            return Err("match has not reached full time".into());
        }
        let e = &self.inner.e;
        Ok(
            json!({"clubs":[word(e,0xD6E6),word(e,0xD6E4)],"goals":[e.image[0xD0C5],e.image[0xD0C4]]}),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn admission() -> Admission {
        serde_json::from_str(include_str!("../tests/fixtures/full_match/admission.json")).unwrap()
    }
    fn play(s: &mut Session) -> (usize, Vec<Value>) {
        let mut steps = 0;
        let mut ai = vec![];
        while s.phase() != "full-time" {
            if s.phase() == "half-time" {
                s.continue_without_changes().unwrap();
            }
            let v = s.step().unwrap();
            ai.extend(v["ai"].as_array().unwrap().iter().cloned());
            steps += 1;
            assert!(steps < 200)
        }
        (steps, ai)
    }
    #[test]
    fn one_native_match_reaches_original_result_and_two_actual_replacements() {
        let mut s = Session::new(admission()).unwrap();
        s.inner.e.image[0x8000] = 0x7B;
        let (steps, ai) = play(&mut s);
        assert_eq!(steps, 95);
        assert_eq!(
            s.result().unwrap(),
            json!({"clubs":[134,130],"goals":[1,3]})
        );
        assert_eq!(ai.len(), 37);
        let replacements = ai
            .iter()
            .flat_map(|v| v["helpers"].as_array().unwrap())
            .filter(|h| h["kind"] == "replace")
            .map(|h| h["arguments"].clone())
            .collect::<Vec<_>>();
        assert_eq!(replacements, vec![json!([14, 8, 1]), json!([13, 6, 1])]);
        assert_eq!(s.points.len(), 5);
        assert_eq!(s.inner.e.image[0x8000], 0x7B);
    }
    #[test]
    fn invalid_phase_actions_preserve_state() {
        let mut s = Session::new(admission()).unwrap();
        let before = s.inner.e.image.clone();
        assert!(s.continue_without_changes().is_err());
        assert_eq!(s.inner.e.image, before);
        assert!(s.result().is_err());
        while s.phase() != "half-time" {
            s.step().unwrap();
        }
        let before = s.inner.e.image.clone();
        assert!(s.step().is_err());
        assert_eq!(s.inner.e.image, before);
        assert_eq!(s.phase(), "half-time");
    }
    #[test]
    fn live_condition_flows_through_halftime_into_second_half() {
        let mut s = Session::new(admission()).unwrap();
        while s.phase() != "half-time" {
            s.step().unwrap();
        }
        let id = word(&s.inner.e, scene(1) + 14);
        let a = layout::P15 + 15 * usize::from(id) + 8;
        s.inner.e.image[a] = 1;
        s.continue_without_changes().unwrap();
        assert_eq!(s.inner.e.image[a], 3);
        let v = s.step().unwrap();
        assert_eq!(v["input"]["players"][0]["p15_sbyte8"], 3);
    }
    #[test]
    fn failed_reached_continuation_rolls_back_all_mutations() {
        let mut s = Session::new(admission()).unwrap();
        while s.phase() != "half-time" {
            s.step().unwrap();
        }
        s.inner.e.image[0xD1CB] = 0;
        let before = s.inner.e.image.clone();
        assert!(s.continue_without_changes().is_err());
        assert_eq!(s.inner.e.image, before);
        assert!(s.points.is_empty());
        assert_eq!(s.phase(), "half-time");
    }
    #[test]
    fn incomplete_or_conflicting_initial_read_sets_rejected() {
        let mut a = admission();
        a.skill_tables.pop();
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.skill_tables[0].first.value ^= 1;
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.gates.cache_gate ^= 1;
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.replacement.match_state.core.actor ^= 1;
        assert!(Session::new(a).is_err());
    }
    #[test]
    fn halftime_continue_records_the_actual_space_response() {
        let mut s = Session::new(admission()).unwrap();
        while s.phase() != "half-time" {
            s.step().unwrap();
        }
        s.continue_without_changes().unwrap();
        assert_eq!(s.points[1]["selection"]["key"], json!(32));
        assert_eq!(s.points[2]["selection"]["key"], json!(2));
    }
    #[test]
    fn presentation_reads_live_signed_coordinates_without_mutation() {
        let mut s = Session::new(admission()).unwrap();
        let original = s.inner.e.image.clone();
        let view = s.view_state();
        assert_eq!(view["formation"].as_array().unwrap().len(),22);
        assert_eq!(view["formation"][0]["player"],json!(2661));
        assert_eq!(view["formation"][0]["x"],json!(0));
        assert_eq!(view["formation"][0]["y"],json!(-37));
        assert_eq!(s.inner.e.image,original);
        put(&mut s.inner.e,scene(1)+10,0x7F80);
        put(&mut s.inner.e,scene(1)+14,2672);
        let changed=s.inner.e.image.clone();
        let view=s.view_state();
        assert_eq!(view["formation"][0]["x"],json!(-128));
        assert_eq!(view["formation"][0]["y"],json!(127));
        assert_eq!(view["formation"][0]["player"],json!(2672));
        assert_eq!(s.inner.e.image,changed);
    }
    #[test]
    fn static_unsigned_display_cap_edges() {
        let mut s = Session::new(admission()).unwrap();
        for class in 0..4 {
            put(
                &mut s.inner.e,
                usize::from(crate::RATINGS) + 4 * class + 2,
                u16::MAX,
            );
        }
        s.project_display_ratings();
        for class in 0..4 {
            assert_eq!(
                word(&s.inner.e, usize::from(crate::RATINGS) + 4 * class + 2),
                256
            );
        }
    }
    #[test]
    fn repeated_supported_sessions_finish_identically() {
        let mut first = Session::new(admission()).unwrap();
        let mut second = Session::new(admission()).unwrap();
        play(&mut first);
        play(&mut second);
        assert_eq!(first.inner.e.image, second.inner.e.image);
        assert_eq!(first.result().unwrap(), second.result().unwrap());
    }
    #[test]
    fn substitution_numeric_queue_is_live_at_clock_boundary() {
        let expected: Value = include_str!("../tests/fixtures/continuous/T36-r0001.jsonl")
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .find(|v| v.get("step") == Some(&json!(57)))
            .unwrap();
        let mut s = Session::new(admission()).unwrap();
        let mut got = Value::Null;
        for step in 0..=57 {
            if s.phase() == "half-time" {
                s.continue_without_changes().unwrap();
            }
            let v = s.step().unwrap();
            if step == 57 {
                got = v["clock_input"].clone();
            }
        }
        assert_eq!(got, expected["clock"]["input"]);
    }
}
