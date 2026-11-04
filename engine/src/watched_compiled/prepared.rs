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
}
fn support_cells(facts: &[ByteFact]) -> Result<Cells, String> {
    let mut cells = Cells::new();
    for fact in facts {
        let o = usize::try_from(fact.offset).map_err(|_| "negative initial support offset")?;
        let base = match fact.area {
            // Source-owned setup, queue, actors, ball, ABI and presentation never enter here.
            // Incoming D09E is read at2573 (stores5A2D/69AF/74F1);
            // D0B6 is preserved through setup,first prefix store28C1.
            Area::Ds
                if (0x0D5C..0x0D5E).contains(&o) // 1998:01E0 tone-burst duration multiplier
                    || (0xD0A6..0xD0AA).contains(&o) // incoming projection scratch;first stores28FA/5821,unchanged through setup
                    || (0x09FB..0x0B5A).contains(&o) // 1B91/21AF..5683 numeric tables;7A3B/7B13 three22-entry tick lists
                    || (0x097A..0x09BC).contains(&o) // 0F70 numeric commentary weights (codes0..65)
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
                    || [0xD084, 0xD09E, 0xD0B6, 0xE5DD, 0xCF5A, 0xCF5B].contains(&o) =>
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
        let support = support_cells(&input.support)?;
        let mut prepared = setup::Session::before_watched_call(
            input.preparation.setup,
            input.preparation.incoming,
            &support,
        )?;
        let preparation = prepared.preparation();
        let e = &mut prepared.match_state.inner.e;
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
    pub fn advance(&mut self) -> Result<Value, String> {
        if self.failed {
            return Err("prepared caller previously failed".into());
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
            let mut clock = json!({});
            for (name, offset, width) in [
                ("active", 0xD0B8, 8),
                ("counter", 0xD0B0, 16),
                ("remaining", 0xD0AE, 16),
                ("stoppage", 0xD0B4, 16),
                ("elapsed", 0xD0B2, 16),
                ("phase", 0xD0AC, 8),
                ("hold", 0xD0B9, 8),
            ] {
                clock[name] = json!(self.cpu.read(0, offset, width)?);
            }
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
