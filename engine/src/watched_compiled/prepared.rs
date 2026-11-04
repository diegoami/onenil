//! [confirmed] T43 selected setup;T44 original E008 caller and field-preservation proof.
//! [designed] Move one tracked native image into the compiled caller;never admit prepared state.
use super::{
    action::{Area, ByteFact, PointerKind},
    compiled::Cpu,
    watched_match,
};
use crate::{layout, role_scores::Cells, setup, watched_setup};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub preparation: watched_setup::Input,
    pub support: Vec<ByteFact>,
    pub no_pending_key: bool,
}
pub struct Session {
    cpu: Cpu,
    failed: bool,
    finished: bool,
}
fn support_cells(facts: &[ByteFact]) -> Result<Cells, String> {
    let mut cells = Cells::new();
    for fact in facts {
        let o = usize::try_from(fact.offset).map_err(|_| "negative initial support offset")?;
        let base = match fact.area {
            // Source-owned setup, queue, actors, ball, ABI and presentation never enter here.
            // Incoming D09E is read at2573 (stores5A2D/69AF/74F1);
            // D0B6/D0BD survive setup,first prefix stores28C1/260F.
            // D1C9 also survives setup;1CEC/2D4B/55E8 own its later action stores.
            Area::Ds
                if (0xD144..0xD148).contains(&o) // incoming weekly award baselines: EAB6:EAC1/EAC4;B526 computes current award
                    || (0xD1D4..0xD1DC).contains(&o) // incoming display aggregates;311E recomputes them from3138..3147
                    || (0x0D34..0x0D36).contains(&o) // 2D4E/3E1C read-only packed goal target
                    || (0x0D5C..0x0D5E).contains(&o) // 1998:01E0 tone-burst duration multiplier
                    || (0xD0A6..0xD0AC).contains(&o) // incoming scratch;stores28FA/5821/260C,unchanged through setup
                    || (0x09FB..0x0B5A).contains(&o) // 1B91/21AF..5683 numeric tables;7A3B/7B13 three22-entry tick lists
                    || (0x097A..0x09BE).contains(&o) // 0F70 numeric commentary weights (codes0..67)
                    || (0x0D36..0x0D56).contains(&o) // 70BF/788B read-only256-bit background character set
                    || (0x0BF6..0x0C68).contains(&o)
                    || (0x0CD7..0x0CE0).contains(&o)
                    || (0x0D60..0x0D91).contains(&o)
                    || (0x0D92..0x0E6A).contains(&o)
                    || (0x0F20..0x0F30).contains(&o)
                    || (0x1274..0x1288).contains(&o)
                    || (0x129E..0x12CE).contains(&o)
                    || (0x8708..0x8716).contains(&o)
                    || (0x935B..0xA51C).contains(&o)
                    || (0xD886..0xD936).contains(&o)
                    || [0xD084, 0xD09E, 0xD0B6, 0xD0BD, 0xD1C9, 0xE5DD, 0xCF5A, 0xCF5B].contains(&o) =>
            {
                0
            }
            Area::Config if o < 256 && ![0, 1, 2, 0xF2, 0xF6].contains(&o) => layout::CONFIG,
            Area::Counts if o < 4004 => layout::CLUB_COUNTS,
            Area::Results if o < 58 * 176 => layout::HISTORY,
            Area::Fixtures if o < 174 * 176 => 0x80000,
            _ => {
                return Err(format!(
                    "unsupported initial support {:?}+{o:04X}",
                    fact.area
                ))
            }
        };
        if cells.insert(base + o, fact.value).is_some() {
            return Err("duplicate initial support".into());
        }
    }
    Ok(cells)
}
impl Session {
    pub fn new(input: Input) -> Result<Self, String> {
        if !input.no_pending_key {
            return Err("prepared watched session requires no pending key".into());
        }
        if input.preparation.setup.legacy.history.base_offset != 0 {
            return Err("unsupported prepared results base offset".into());
        }
        let caller_club = input.preparation.setup.legacy.history.club;
        let fixture_flags = input
            .preparation
            .setup
            .income
            .iter()
            .find(|row| row.club == caller_club)
            .ok_or("missing current caller fixture flags")?
            .fixture_flags;
        let support = support_cells(&input.support)?;
        let mut prepared = setup::Session::before_watched_call(
            input.preparation.setup,
            input.preparation.incoming,
            &support,
        )?;
        let preparation = prepared.preparation();
        let e = &mut prepared.match_state.inner.e;
        if e.word(0xD6E6) != caller_club || e.image[0xD29] != u8::from(fixture_flags & 15 == 1) {
            return Err("current fixture/caller/home-flag alias differs".into());
        }
        let flag_offset = caller_club
            .wrapping_mul(174)
            .wrapping_add(3 * u16::from(e.image[layout::CONFIG]))
            .wrapping_sub(175);
        let address = 0x80000 + usize::from(flag_offset);
        if support
            .get(&address)
            .is_some_and(|value| *value != fixture_flags)
        {
            return Err("current raw fixture flags alias differs".into());
        }
        e.image[address] = fixture_flags;
        // [confirmed] E239/E249 decode the initial fixture byte before19CA.
        // Derive D2A from the supplied raw flags;never admit a captured marker.
        e.image[0xD2A] = u8::from((fixture_flags >> 4) == 1);
        // [designed] Disjoint native flat namespace,DS0;wrapped code base keeps original CS/DS relation.
        let headers = [
            (PointerKind::P8, layout::P8),
            (PointerKind::P15, layout::P15),
            (PointerKind::Club, layout::CLUB),
            (PointerKind::Config, layout::CONFIG),
            (PointerKind::Style, layout::SCENE_MODE),
            (PointerKind::Main, 0x90000),
            (PointerKind::MainShown, 0x91000),
            (PointerKind::Background, 0x92000),
            (PointerKind::BackgroundShown, 0x93000),
            (PointerKind::Commentary, 0xB8000),
            (PointerKind::Seed, usize::from(crate::SEED)),
            (PointerKind::Fixtures, 0x80000),
            (PointerKind::Counts, layout::CLUB_COUNTS),
            (PointerKind::Screen, 0x90000),
            (PointerKind::ScreenShown, 0x91000),
            (PointerKind::ScreenSaved, 0x93000),
        ];
        for (kind, address) in headers {
            let slot = usize::from(kind.slot());
            let offset = if address < 0x10000 {
                address as u16
            } else {
                (address & 15) as u16
            };
            let segment = if address < 0x10000 {
                0
            } else {
                (address >> 4) as u16
            };
            e.image[slot..slot + 2].copy_from_slice(&offset.to_le_bytes());
            e.image[slot + 2..slot + 4].copy_from_slice(&segment.to_le_bytes());
        }
        // Results deliberately retain the history pointer and its already loaded/computed byte.
        // [derived] T44 headless-original-r0001 preserves every measured original prefix result.
        for base in [0x90000, 0x91000, 0x92000, 0x93000] {
            for i in 0..2000 {
                e.image[base + 2 * i] = 32;
                e.image[base + 2 * i + 1] = 7;
            }
            e.image[base + 0x90E] = 9;
        }
        for (address, value) in [(0xDD64, 0u16), (0xDE8C, 0), (0xDE8E, 0)] {
            e.image[address..address + 2].copy_from_slice(&value.to_le_bytes());
        }
        let week = u16::from(e.image[layout::CONFIG]);
        let club = e.word(0xD6E6);
        let known = e.image.take_initialized()?;
        let mut c = Cpu::new(prepared.match_state.inner.e);
        c.known = known;
        c.flat_memory = true;
        c.code_base = 0xD79C;
        c.r[9] = c.code_base.wrapping_add(0x0AB9);
        c.r[11] = 0;
        c.r[10] = 0xB000;
        c.r[5] = 0xFE00;
        c.r[4] = 0xFDF4;
        c.stack_bounds = Some((0xBEDF4, 0xBFE0A));
        c.draws =
            serde_json::from_value(preparation["draws"].clone()).map_err(|e| e.to_string())?;
        for (offset, value) in [
            (0xFE00, 0),
            (0xFE02, 0xEBE2),
            (0xFE04, week),
            (0xFE06, club),
        ] {
            c.write(c.r[10], offset, 16, value)?;
        }
        // [confirmed] D5D6 presentation preparation computes marker glyphs via033E.
        c.invoke(
            super::action_graph::p_0ab9_033e,
            false,
            0x0AB9,
            0x033E,
            0xE303,
        )
        .map_err(|e| {
            format!(
                "owned marker initialization: {e};missing={:?}",
                c.missing.get()
            )
        })?;
        // Source E008/7D86/7C3F still owns setup writes (including CF43 at7607).
        // Reads remain strict;enable first-write baselines only at the watched entry.
        c.track_events = true;
        c.watch_trace = Some(vec![]);
        c.watch_clocks = Some(vec![]);
        c.watch_entry_yield = true;
        c.push(c.r[5])?;
        let result = c.invoke(
            super::prepared_graph::p_0ab9_e008,
            false,
            0x0AB9,
            0xE008,
            0xE435,
        );
        if result.as_ref().err().map(String::as_str) != Some("native watched entry yield")
            || !c.watch_entry_yielded
            || c.missing.get().is_some()
            || c.watch_tick != 0
        {
            return Err(format!(
                "prepared watched entry: {result:?};pc={:?};missing={:?}",
                c.pc,
                c.missing.get()
            ));
        }
        c.watch_entry_yield = false;
        c.require_write_baseline = true;
        let out = Self {
            cpu: c,
            failed: false,
            finished: false,
        };
        out.validate_abi()?;
        Ok(out)
    }
    fn validate_abi(&self) -> Result<(), String> {
        let c = &self.cpu;
        if c.r[11] != 0
            || c.r[10] != 0xB000
            || c.r[9] != 0xE255
            || c.r[4] != 0xFDD4
            || c.r[5] != 0xFDDA
            || c.call_chain() != vec![(0x0AB9, 0xE435), (0x0AB9, 0xE05E), (0x0AB9, 0x7DCB)]
        {
            return Err(format!(
                "prepared caller ABI differs: {:?};frames={:?}",
                c.r,
                c.call_chain()
            ));
        }
        for (offset, value) in [(8, 0x7DCB), (16, 0xE05E), (28, 0xE435), (46, 0xEBE2)] {
            if c.read(c.r[10], c.r[4] + offset, 16)? != value {
                return Err("prepared return frame differs".into());
            }
        }
        for (kind, expected) in [
            (PointerKind::P8, layout::P8),
            (PointerKind::P15, layout::P15),
            (PointerKind::Club, layout::CLUB),
            (PointerKind::Config, layout::CONFIG),
            (PointerKind::Style, layout::SCENE_MODE),
            (PointerKind::Main, 0x90000),
            (PointerKind::MainShown, 0x91000),
            (PointerKind::Background, 0x92000),
            (PointerKind::BackgroundShown, 0x93000),
            (PointerKind::Commentary, 0x90000),
            (PointerKind::Seed, usize::from(crate::SEED)),
            (PointerKind::Fixtures, 0x80000),
            (PointerKind::Results, layout::HISTORY),
            (PointerKind::Counts, layout::CLUB_COUNTS),
            (PointerKind::Screen, 0x90000),
            (PointerKind::ScreenShown, 0x91000),
            (PointerKind::ScreenSaved, 0x93000),
        ] {
            let (offset, segment) = c.far(0, kind.slot())?;
            if usize::from(segment) * 16 + usize::from(offset) != expected {
                return Err(format!(
                    "prepared pointer alias/ABI differs: {kind:?}={segment:04X}:{offset:04X}"
                ));
            }
        }
        Ok(())
    }
    pub fn snapshot(&self) -> Result<Value, String> {
        watched_match::snapshot(&self.cpu)
    }
    pub fn observation(&self) -> Value {
        json!({"ticks":self.cpu.watch_trace,"draws":self.cpu.draws,"events":self.cpu.events,"regs":self.cpu.r})
    }
    /// Continue the already owned source frames through the actual outer return.
    pub fn finish(&mut self) -> Result<Value, watched_match::MatchFault> {
        let failure = |message: String| {
            watched_match::MatchFault::from(super::action::Fault {
                message,
                missing: None,
            })
        };
        if self.failed || self.finished {
            return Err(failure(
                "prepared session cannot finish again/after a failure".into(),
            ));
        }
        let result = (|| -> Result<Value, String> {
            self.validate_abi()?;
            let c = &mut self.cpu;
            c.resume_frame(super::match_graph::p_0ab9_7c67, 0x0AB9, 0x7DCB)?;
            c.resume_frame(super::match_graph::p_0ab9_7dcb, 0x0AB9, 0xE05E)?;
            c.resume_frame(super::match_graph::p_0ab9_e05e, 0x0AB9, 0xE435)?;
            if !c.call_chain().is_empty() {
                return Err("prepared outer continuation retained nested frames".into());
            }
            c.invoke_entry(super::match_graph::p_0ab9_e435, 0x0AB9, 0xEBE2)?;
            if c.last_return != Some((0x0AB9, 0xEBE2)) {
                return Err("prepared actual outer guest return differs".into());
            }
            if c.watch_pending.is_some()
                || c.r[4] != 0xFE08
                || c.r[5] != 0
                || c.r[11] != 0
                || c.r[10] != 0xB000
                || c.r[9] != 0xE255
                || !c.call_chain().is_empty()
            {
                return Err("prepared fulltime outer return ABI differs".into());
            }
            let final_state = watched_match::snapshot(c)?;
            let live = [c.read(0, 0xD0C4, 8)?, c.read(0, 0xD0C5, 8)?];
            let setup_events = c
                .events
                .iter()
                .filter(|e| e["tick"] == 0)
                .collect::<Vec<_>>();
            let events = c
                .events
                .iter()
                .filter(|e| e["tick"] != 0)
                .collect::<Vec<_>>();
            Ok(
                json!({"ticks":c.watch_trace,"draws":c.draws,"events":events,"setup_events":setup_events,
                "final":final_state,"live_goals":live,"tick_live_goals":c.watch_live_goals,
                "caller_clocks":c.watch_clocks,"return":"0AB9:EBE2"}),
            )
        })();
        match result {
            Ok(out) => {
                self.finished = true;
                Ok(out)
            }
            Err(message) => {
                self.failed = true;
                let c = &mut self.cpu;
                let missing = c.missing.get();
                let partial = if c.r[11] == 0 {
                    watched_match::snapshot(c).ok()
                } else {
                    None
                };
                Err(watched_match::MatchFault {
                    fault: super::action::Fault {
                        message: format!(
                            "prepared tick{} draw{}: {message}",
                            c.watch_tick,
                            c.draws.len()
                        ),
                        missing,
                    },
                    ticks: c.watch_trace.clone().unwrap_or_default(),
                    draws: c.draws.clone(),
                    events: c.events.clone(),
                    pending: c.watch_pending.clone(),
                    diagnostic: json!({"regs":c.r,"call_chain":c.call_chain(),"partial":partial}),
                })
            }
        }
    }
    pub fn advance(&mut self) -> Result<Value, String> {
        if self.failed {
            return Err("prepared caller previously failed".into());
        }
        if self.finished {
            return Err("prepared caller already finished".into());
        }
        if self.cpu.watch_tick >= 171 {
            return Err(
                "prepared caller prefix boundary reached;native continuation retained".into(),
            );
        }
        let result = (|| {
            self.validate_abi()?;
            super::caller::iteration(&mut self.cpu)?;
            if self.cpu.r[0] & 255 != 0 {
                return Err("prepared prefix reached an unsupported phase transition".into());
            }
            self.validate_abi()?;
            let clock = self.cpu.watched_clock()?;
            Ok(json!({"state":self.snapshot()?,"clock":clock,"tick":self.cpu.watch_tick}))
        })();
        if result.is_err() {
            self.failed = true;
        }
        result.map_err(|e: String| {
            format!(
                "{e};pc={:?};missing={:?}",
                self.cpu.pc,
                self.cpu.missing.get()
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Input {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_setup/admission.json");
        let fixture: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        Input {
            preparation: serde_json::from_value(fixture["input"].clone()).unwrap(),
            support: vec![],
            no_pending_key: true,
        }
    }
    #[test]
    fn weekly_award_baselines_are_incoming_but_current_award_is_computed() {
        let facts = (0xD144..0xD148)
            .map(|offset| ByteFact {
                area: Area::Ds,
                offset,
                value: 0,
            })
            .collect::<Vec<_>>();
        let cells = support_cells(&facts).unwrap();
        assert_eq!(cells.len(), 4);
        for offset in [0xD142, 0xD143, 0xD148, 0xD149] {
            assert!(support_cells(&[ByteFact {
                area: Area::Ds,
                offset,
                value: 0
            }])
            .is_err());
        }
    }
    #[test]
    fn current_fixture_flags_compute_the_caller_cup_marker() {
        for (flags, expected) in [(0x01, 0), (0x11, 1), (0x21, 0)] {
            let mut value = input();
            let club = value.preparation.setup.legacy.history.club;
            value
                .preparation
                .setup
                .income
                .iter_mut()
                .find(|row| row.club == club)
                .unwrap()
                .fixture_flags = flags;
            let session = Session::new(value).unwrap();
            assert_eq!(session.cpu.read(0, 0x0D2A, 8).unwrap(), expected);
            assert!(session.cpu.known[0x0D2A]);
        }
    }
    #[test]
    fn native_editor_owns_both_bytes_of_normalized_key_words() {
        let session = Session::new(input()).unwrap();
        for address in [0xDE88, 0xDE8A] {
            assert_eq!(session.cpu.read(0, address, 16).unwrap(), 2);
            assert!(
                session.cpu.known[usize::from(address)]
                    && session.cpu.known[usize::from(address) + 1]
            );
        }
    }
    #[test]
    fn selected_native_image_computes_the_original_watched_entry_and_retains_its_frames() {
        let mut session = Session::new(input()).unwrap();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("records/T43/original-r0001/result.json");
        let original: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(session.snapshot().unwrap(), original["first_entry"]);
        assert_eq!(
            serde_json::to_value(&session.cpu.draws).unwrap(),
            original["draws"]
        );
        assert_eq!(session.cpu.events.len(), 1);
        assert_eq!(session.cpu.events[0]["code"], 59);
        assert_eq!(session.cpu.events[0]["tick"], 0);
        let pointer = session.cpu.e.image.as_ptr();
        let before = session.cpu.e.image.clone();
        session.snapshot().unwrap();
        session.observation();
        assert_eq!(pointer, session.cpu.e.image.as_ptr());
        assert_eq!(before, session.cpu.e.image);
        let error = session.advance().unwrap_err();
        assert!(error.contains("0000:0B18"), "{error}");
        assert!(session.advance().unwrap_err().contains("previously failed"));
    }
    #[test]
    fn one_selected_image_matches_all_171_original_caller_iterations() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let fixture: Value = serde_json::from_slice(
            &std::fs::read(root.join("tests/fixtures/prepared_watched/admission.json")).unwrap(),
        )
        .unwrap();
        let original: Value = serde_json::from_slice(
            &std::fs::read(
                root.parent()
                    .unwrap()
                    .join("records/T44/headless-original-r0001/result.json"),
            )
            .unwrap(),
        )
        .unwrap();
        let mut session =
            Session::new(serde_json::from_value(fixture["input"].clone()).unwrap()).unwrap();
        let pointer = session.cpu.e.image.as_ptr();
        for index in 0..171 {
            let out = session.advance().unwrap();
            assert_eq!(out["tick"], index + 1);
            assert_eq!(
                session.cpu.watch_trace.as_ref().unwrap()[index],
                original["ticks"][index],
                "tick{}",
                index + 1
            );
            let path = root.parent().unwrap().join(format!(
                "records/T40/oracle-r0001/iteration-{:03}.json",
                index + 1
            ));
            let caller: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            assert_eq!(
                out["clock"],
                caller["output"]["clock"],
                "caller clock{}",
                index + 1
            );
            assert_eq!(pointer, session.cpu.e.image.as_ptr());
        }
        assert_eq!(session.snapshot().unwrap(), original["clock_end"]);
        assert_eq!(
            serde_json::to_value(&session.cpu.draws).unwrap(),
            original["draws"]
        );
        let events = session
            .cpu
            .events
            .iter()
            .filter(|e| e["tick"] != 0)
            .collect::<Vec<_>>();
        let expected = original["events"]
            .as_array()
            .unwrap()
            .iter()
            .cloned()
            .map(|mut e| {
                e.as_object_mut().unwrap().remove("event");
                e
            })
            .collect::<Vec<_>>();
        assert_eq!(
            serde_json::to_value(events).unwrap(),
            serde_json::to_value(expected).unwrap()
        );
        assert_eq!(session.cpu.watch_tick, 171);
        assert_eq!(session.cpu.draws.len(), 7280);
        session.validate_abi().unwrap();
        let before = session.cpu.e.image.clone();
        assert!(session
            .advance()
            .unwrap_err()
            .contains("continuation retained"));
        assert!(!session.failed);
        assert_eq!(session.cpu.e.image, before);
    }
    #[test]
    fn native_entry_guards_reject_aliases_frames_quick_mode_and_unknown_writes() {
        for mutation in 0..5 {
            let mut session = Session::new(input()).unwrap();
            match mutation {
                0 => session.cpu.e.image[0xD7BD] = 0x20,
                1 => session.cpu.e.image[0xBFDDC] = 0,
                2 => session.cpu.r[9] = 0,
                3 => session.cpu.e.image[0xD074] = 1,
                _ => {
                    assert!(!session.cpu.known[0x0330]);
                    assert!(session
                        .cpu
                        .write(0, 0x0330, 8, 1)
                        .unwrap_err()
                        .contains("write baseline"));
                    assert!(session.cpu.missing.get().unwrap().writing);
                    assert!(!session.cpu.known[0x0330]);
                    continue;
                }
            }
            let error = session.advance().unwrap_err();
            assert!(
                error.contains(match mutation {
                    0 => "pointer alias",
                    1 => "return frame",
                    2 => "ABI",
                    _ => "quick fallback",
                }),
                "{error}"
            );
        }
    }
    #[test]
    fn prepared_inputs_reject_outcomes_and_pending_keys_before_execution() {
        let mut bad = input();
        bad.no_pending_key = false;
        assert!(Session::new(bad).err().unwrap().contains("pending key"));
        for area in [
            Area::Frame,
            Area::Main,
            Area::Background,
            Area::Style,
            Area::P8,
            Area::P15,
            Area::Club,
        ] {
            let mut bad = input();
            bad.support.push(ByteFact {
                area,
                offset: 0,
                value: 0,
            });
            assert!(Session::new(bad)
                .err()
                .unwrap()
                .contains("unsupported initial support"));
        }
        for offset in [
            0x380, 0xA51C, 0xCCEF, 0xCF32, 0xCF43, 0xCF46, 0xD074, 0xD7BA, 0xDE88,
        ] {
            let mut bad = input();
            bad.support.push(ByteFact {
                area: Area::Ds,
                offset,
                value: 0,
            });
            assert!(Session::new(bad)
                .err()
                .unwrap()
                .contains("unsupported initial support"));
        }
    }
}
