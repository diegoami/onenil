//! [confirmed] Original7A02/staticd9e20a4 and T39 original entry/repeated-call observations.
//! [designed] One admitted data image persists;advance takes no intermediate state inputs.
use super::{
    action::{self, Area, Fault, Kind, Pointer, State},
    compiled::Cpu,
};
use crate::{terminal::Scene, Draw};
use serde::Serialize;
#[derive(Serialize)]
pub struct Outcome {
    pub scenes: Vec<Scene>,
    pub seed: u16,
    pub pointers: Vec<Pointer>,
    pub cells: Vec<action::ByteFact>,
    pub draws: Vec<Draw>,
    pub events: Vec<serde_json::Value>,
}
pub struct TickSession {
    cpu: Cpu,
    admission: State,
    started: bool,
    failed: bool,
    style: (usize, u8),
}
fn fault(message: String, c: &Cpu) -> Fault {
    Fault {
        message,
        missing: c.missing.get(),
    }
}
impl TickSession {
    pub fn admit(s: &State, no_pending_key: bool) -> Result<Self, Fault> {
        if s.kind != Kind::Tick || !no_pending_key {
            return Err(Fault {
                message: "watched tick requires explicit no-pending-key admission".into(),
                missing: None,
            });
        }
        let c = action::machine(s).map_err(|message| Fault {
            message,
            missing: None,
        })?;
        let p = s
            .pointers
            .iter()
            .find(|p| p.kind == action::PointerKind::Style)
            .unwrap();
        let a = p.address();
        let style = (a, c.e.image[a]);
        Ok(Self {
            cpu: c,
            admission: s.clone(),
            started: false,
            failed: false,
            style,
        })
    }
    pub fn advance(&mut self) -> Result<Outcome, Fault> {
        if self.failed {
            return Err(Fault {
                message: "watched tick session is closed after a failed advance".into(),
                missing: None,
            });
        }
        let result = self.advance_computed();
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn advance_computed(&mut self) -> Result<Outcome, Fault> {
        self.cpu.reset_root_budget();
        let c = &mut self.cpu;
        let s = &self.admission;
        if self.started {
            c.push(s.stack_words[0]).map_err(|e| fault(e, c))?;
        }
        let d = c.draws.len();
        let e = c.events.len();
        c.invoke_entry(super::tick_graph::p_0ab9_7a02, 0x0AB9, s.stack_words[0])
            .map_err(|e| fault(e, c))?;
        if c.r[4] != s.regs[4].wrapping_add(2) || c.r[5] != s.regs[5] || c.r[11] != s.regs[11] {
            return Err(fault("watched tick near return ABI differs".into(), c));
        }
        if c.e.image[self.style.0] != self.style.1 {
            return Err(fault(
                "watched tick changed readonly logical style".into(),
                c,
            ));
        }
        self.started = true;
        let ds = 16 * usize::from(s.regs[11]);
        let mut canonical = crate::role_scores::image(Default::default());
        let mut scenes = Vec::new();
        for i in 1..=22 {
            let a = 0xCCEF + 25 * usize::from(i);
            for k in 0..25 {
                let b = c
                    .read(s.regs[11], (a + k) as u16, 8)
                    .map_err(|e| fault(e, c))?;
                canonical.image[a + k] = b as u8;
            }
            scenes.push(Scene::read(&canonical, i));
        }
        let mut pointers = s.pointers.clone();
        for p in &mut pointers {
            let slot = p.kind.slot();
            p.offset = c.read(s.regs[11], slot, 16).map_err(|e| fault(e, c))?;
            p.segment = c
                .read(s.regs[11], slot.wrapping_add(2), 16)
                .map_err(|e| fault(e, c))?;
        }
        let mut cells = s.cells.clone();
        cells.retain(|f| f.area != Area::Frame);
        for f in &mut cells {
            let a = action::cell_address(s, f).map_err(|e| fault(e, c))?;
            f.value = c.e.image[a];
        }
        Ok(Outcome {
            scenes,
            seed: u16::from_le_bytes([c.e.image[ds + 0xA51C], c.e.image[ds + 0xA51D]]),
            pointers,
            cells,
            draws: c.draws[d..].to_vec(),
            events: c.events[e..].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> State {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_tick/case-08.json");
        let v: serde_json::Value = serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap();
        serde_json::from_value(v["initial"].clone()).unwrap()
    }
    #[test]
    fn typed_tick_admission_rejects_external_key_and_pointer_frame_aliases() {
        let s = fixture();
        assert!(TickSession::admit(&s, false).is_err());
        let mut wrong = s.clone();
        wrong.kind = Kind::Actor;
        assert!(TickSession::admit(&wrong, true).is_err());
        let mut duplicate = s.clone();
        duplicate.cells.push(s.cells[0].clone());
        assert!(TickSession::admit(&duplicate, true).is_err());
        let mut pointer = s.clone();
        pointer.pointers[0].segment = s.regs[11];
        assert!(TickSession::admit(&pointer, true).is_err());
        let mut frame = s;
        frame.regs[10] = frame.regs[11];
        assert!(TickSession::admit(&frame, true).is_err());
    }
    #[test]
    fn missing_initial_cell_closes_the_session_without_retrying_a_partial_tick() {
        let mut s = fixture();
        let n = s.cells.len();
        s.cells
            .retain(|f| !(f.area == Area::Ds && f.offset == 0xCF53));
        assert!(s.cells.len() < n);
        let mut session = TickSession::admit(&s, true).unwrap();
        let error = session.advance().err().unwrap();
        assert!(error.missing.is_some());
        let before = session.cpu.e.image.clone();
        let regs = session.cpu.r;
        assert!(session.advance().err().unwrap().message.contains("closed"));
        assert_eq!(before, session.cpu.e.image);
        assert_eq!(regs, session.cpu.r);
    }
    #[test]
    fn all_original_watched_ticks_and_one_persistent_sequence() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let m: serde_json::Value = serde_json::from_slice(
            &std::fs::read(root.join("engine/tests/fixtures/watched_tick/manifest.json")).unwrap(),
        )
        .unwrap();
        let cases = m["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 9);
        let mut total = 0;
        for item in cases {
            let v: serde_json::Value = serde_json::from_slice(
                &std::fs::read(root.join(item["path"].as_str().unwrap())).unwrap(),
            )
            .unwrap();
            let input: State = serde_json::from_value(v["initial"].clone()).unwrap();
            let mut session = TickSession::admit(&input, true).unwrap();
            for expected in v["expected"].as_array().unwrap() {
                assert_eq!(
                    serde_json::to_value(session.advance().unwrap()).unwrap(),
                    *expected,
                    "session{} step{}",
                    v["case"],
                    total
                );
                total += 1;
            }
        }
        assert_eq!(total, 16);
    }
}
