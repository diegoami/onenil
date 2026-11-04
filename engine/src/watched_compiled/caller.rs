//! [confirmed] Original7C67–7C94 caller and7B7B logical clock,staticd9e20a4.
//! [designed] One image;advance has no intermediate state or clock inputs.
use super::{
    action::{self, Fault, Kind, State},
    compiled::Cpu,
    tick,
};
use serde::Serialize;
#[derive(Serialize)]
pub struct Clock {
    pub active: u8,
    pub counter: u16,
    pub remaining: u16,
    pub stoppage: u16,
    pub elapsed: u16,
    pub phase: u8,
    pub hold: u8,
}
#[derive(Serialize)]
pub struct Outcome {
    #[serde(flatten)]
    pub gameplay: tick::Outcome,
    pub clock: Clock,
    pub phase_complete: bool,
}
pub struct CallerSession {
    cpu: Cpu,
    admission: State,
    style: (usize, u8),
    failed: bool,
    ended: bool,
    used: bool,
}
fn fault(message: String, c: &Cpu) -> Fault {
    Fault {
        message,
        missing: c.missing.get(),
    }
}
fn at(c: &mut Cpu, pc: u16) -> Result<(), String> {
    c.pc = (0x0AB9, pc);
    c.step()
}
fn iteration(c: &mut Cpu) -> Result<(), String> {
    // Source7C67–7C94 in order;the backedge yields to the native session driver.
    at(c, 0x7C67)?;
    let quick = c.read(c.r[11], 0xD074, 8)?;
    c.sub(quick, 0, 8);
    if quick != 0 {
        return Err("watched caller cannot enter a quick fallback".into());
    }
    at(c, 0x7C73)?;
    c.invoke(
        super::tick_graph::p_0ab9_7a02,
        false,
        0x0AB9,
        0x7A02,
        0x7C76,
    )?;
    at(c, 0x7C76)?;
    let quick = c.read(c.r[11], 0xD074, 8)?;
    c.sub(quick, 0, 8);
    if quick != 0 {
        return Err("watched tick requested an unsupported quick transition".into());
    }
    at(c, 0x7C7D)?;
    c.invoke(
        super::action_graph::p_0ab9_033e,
        false,
        0x0AB9,
        0x033E,
        0x7C80,
    )?;
    at(c, 0x7C80)?;
    c.invoke(
        super::caller_graph::p_0ab9_760c,
        false,
        0x0AB9,
        0x760C,
        0x7C83,
    )?;
    at(c, 0x7C83)?;
    c.push(0)?;
    at(c, 0x7C85)?;
    c.r[7] = 0xCF3C;
    at(c, 0x7C88)?;
    c.push(c.r[11])?;
    at(c, 0x7C89)?;
    c.push(c.r[7])?;
    at(c, 0x7C8B)?;
    c.invoke(
        super::action_graph::p_0ab9_770e,
        true,
        0x0AB9,
        0x770E,
        0x7C8E,
    )?;
    clock_call(c)?;
    at(c, 0x7C92)?;
    c.logic(c.r[0] & 255, 8);
    at(c, 0x7C94)
}
fn clock_call(c: &mut Cpu) -> Result<(), String> {
    at(c, 0x7C8E)?;
    c.push(c.r[5])?;
    at(c, 0x7C8F)?;
    c.invoke(
        super::caller_graph::p_0ab9_7b7b,
        false,
        0x0AB9,
        0x7B7B,
        0x7C92,
    )
}
impl CallerSession {
    pub fn admit(s: &State, no_pending_key: bool) -> Result<Self, Fault> {
        if s.kind != Kind::Caller || !no_pending_key || s.regs[5] != s.regs[4].wrapping_add(6) {
            return Err(Fault {
                message:
                    "watched caller requires its actual parent frame and no-pending-key admission"
                        .into(),
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
            style,
            failed: false,
            ended: false,
            used: false,
        })
    }
    pub fn advance(&mut self) -> Result<Outcome, Fault> {
        self.execute(false)
    }
    pub fn clock_boundary(&mut self) -> Result<Outcome, Fault> {
        if self.used {
            return Err(fault(
                "isolated clock boundary requires a fresh admission".into(),
                &self.cpu,
            ));
        }
        self.execute(true)
    }
    fn execute(&mut self, clock_only: bool) -> Result<Outcome, Fault> {
        if self.failed || self.ended {
            return Err(fault("watched caller session is closed".into(), &self.cpu));
        }
        let result = self.execute_computed(clock_only);
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn execute_computed(&mut self, clock_only: bool) -> Result<Outcome, Fault> {
        let c = &mut self.cpu;
        let s = &self.admission;
        c.reset_root_budget();
        c.presentation.clear();
        let d = c.draws.len();
        let e = c.events.len();
        // Actual first-phase caller return7DCB is proven by the captured parent ABI.
        c.invoke_entry(
            if clock_only { clock_call } else { iteration },
            0x0AB9,
            0x7DCB,
        )
        .map_err(|e| fault(e, c))?;
        if c.r[4] != s.regs[4] || c.r[5] != s.regs[5] || c.r[11] != s.regs[11] {
            return Err(fault("watched caller changed its parent frame".into(), c));
        }
        if c.e.image[self.style.0] != self.style.1 {
            return Err(fault(
                "watched caller changed readonly logical style".into(),
                c,
            ));
        }
        let read = |o, w| c.read(s.regs[11], o, w).map_err(|e| fault(e, c));
        let clock = Clock {
            active: read(0xD0B8, 8)? as u8,
            counter: read(0xD0B0, 16)?,
            remaining: read(0xD0AE, 16)?,
            stoppage: read(0xD0B4, 16)?,
            elapsed: read(0xD0B2, 16)?,
            phase: read(0xD0AC, 8)? as u8,
            hold: read(0xD0B9, 8)? as u8,
        };
        let phase_complete = c.r[0] & 255 != 0;
        let gameplay = tick::snapshot(c, s, d, e)?;
        self.used = true;
        self.ended = phase_complete;
        Ok(Outcome {
            gameplay,
            clock,
            phase_complete,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use action::Area;
    use serde_json::Value;
    use std::collections::BTreeMap;
    fn load(name: &str) -> Value {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        serde_json::from_slice(&std::fs::read(root.join(name)).unwrap()).unwrap()
    }
    fn initial_values(s: &State) -> BTreeMap<(Area, i32), u8> {
        let mut values: BTreeMap<_, _> = s
            .cells
            .iter()
            .filter(|f| f.area != Area::Frame)
            .map(|f| ((f.area, f.offset), f.value))
            .collect();
        for (i, b) in s.scene.bytes().into_iter().enumerate() {
            values.insert(
                (Area::Ds, 0xCCEF + 25 * i32::from(s.scene.index) + i as i32),
                b,
            );
        }
        for (i, b) in s.seed.to_le_bytes().into_iter().enumerate() {
            values.insert((Area::Ds, 0xA51C + i as i32), b);
        }
        for p in &s.pointers {
            for (i, b) in p
                .offset
                .to_le_bytes()
                .into_iter()
                .chain(p.segment.to_le_bytes())
                .enumerate()
            {
                values.insert((Area::Ds, i32::from(p.kind.slot()) + i as i32), b);
            }
        }
        values
    }
    fn compare(
        out: Outcome,
        source: &Value,
        input: &State,
        values: &mut BTreeMap<(Area, i32), u8>,
        base: u64,
    ) {
        for change in source["changes"].as_array().unwrap() {
            let area: Area = serde_json::from_value(change["area"].clone()).unwrap();
            let key = (area, change["offset"].as_i64().unwrap() as i32);
            assert_eq!(
                values.get(&key).copied(),
                Some(change["before"].as_u64().unwrap() as u8),
                "original change baseline {key:?}"
            );
            values.insert(key, change["after"].as_u64().unwrap() as u8);
        }
        let expected_cells: Vec<_> = input
            .cells
            .iter()
            .filter(|f| f.area != Area::Frame)
            .collect();
        assert_eq!(out.gameplay.cells.len(), expected_cells.len());
        for (actual, location) in out.gameplay.cells.iter().zip(expected_cells) {
            assert_eq!(
                (actual.area, actual.offset),
                (location.area, location.offset)
            );
            assert_eq!(
                Some(actual.value),
                values.get(&(actual.area, actual.offset)).copied(),
                "gameplay cell {:?}:{}",
                actual.area,
                actual.offset
            );
        }
        assert_eq!(
            serde_json::to_value(out.gameplay.scenes).unwrap(),
            source["output"]["scenes"]
        );
        assert_eq!(
            u64::from(out.gameplay.seed),
            source["output"]["seed"].as_u64().unwrap()
        );
        assert_eq!(
            serde_json::to_value(out.gameplay.pointers).unwrap(),
            source["output"]["pointers"]
        );
        assert_eq!(
            serde_json::to_value(out.gameplay.draws).unwrap(),
            source["draws"]
        );
        let mut events = source["events"].clone();
        for e in events.as_array_mut().unwrap() {
            for field in ["draw_start", "draw_end"] {
                e[field] = serde_json::json!(e[field].as_u64().unwrap() - base);
            }
        }
        assert_eq!(serde_json::to_value(out.gameplay.events).unwrap(), events);
        assert_eq!(
            serde_json::to_value(out.clock).unwrap(),
            source["output"]["clock"]
        );
        assert_eq!(
            out.phase_complete,
            source["phase_complete"].as_bool().unwrap()
        );
    }
    fn fixture_input(number: usize) -> State {
        serde_json::from_value(
            load(&format!(
                "engine/tests/fixtures/watched_caller/case-{number:02}.json"
            ))["initial"]
                .clone(),
        )
        .unwrap()
    }
    #[test]
    fn caller_admission_and_failed_advance_keep_alias_and_domain_guards() {
        let s = fixture_input(0);
        assert!(CallerSession::admit(&s, false).is_err());
        let mut frame = s.clone();
        frame.regs[5] += 2;
        assert!(CallerSession::admit(&frame, true).is_err());
        let mut alias = s.clone();
        alias.pointers[0].segment = s.regs[11];
        assert!(CallerSession::admit(&alias, true).is_err());
        let mut duplicate = s.clone();
        duplicate.cells.push(s.cells[0].clone());
        assert!(CallerSession::admit(&duplicate, true).is_err());
        let mut quick = s;
        quick
            .cells
            .iter_mut()
            .find(|f| f.area == Area::Ds && f.offset == 0xD074)
            .unwrap()
            .value = 1;
        let mut session = CallerSession::admit(&quick, true).unwrap();
        assert!(session
            .advance()
            .err()
            .unwrap()
            .message
            .contains("quick fallback"));
        let image = session.cpu.e.image.clone();
        let regs = session.cpu.r;
        assert!(session.advance().err().unwrap().message.contains("closed"));
        assert_eq!(image, session.cpu.e.image);
        assert_eq!(regs, session.cpu.r);
    }
    #[test]
    fn missing_offside_word_read_closes_the_caller_before_the_later_store() {
        let mut s = fixture_input(0);
        let n = s.cells.len();
        s.cells
            .retain(|f| !(f.area == Area::Ds && f.offset == 0xCF2F));
        assert!(s.cells.len() < n);
        let mut session = CallerSession::admit(&s, true).unwrap();
        let error = session.advance().err().unwrap();
        let missing = error.missing.unwrap();
        assert!(!missing.writing, "{error:?}");
        assert_eq!(missing.pc, (0x0AB9, 0x0352));
        assert!(session.advance().err().unwrap().message.contains("closed"));
    }
    #[test]
    fn original_compiled_offside_store_rejects_a_missing_write_baseline() {
        let mut input = fixture_input(0);
        input
            .cells
            .retain(|f| !(f.area == Area::Ds && f.offset == 0xCF2F));
        let mut session = CallerSession::admit(&input, true).unwrap();
        let result = session.cpu.invoke(
            super::super::caller_graph::p_0ab9_760c,
            false,
            0x0AB9,
            0x760C,
            0x7C83,
        );
        assert!(result.is_err());
        let missing = session.cpu.missing.get().unwrap();
        assert!(missing.writing);
        assert_eq!(missing.address, 16 * usize::from(input.regs[11]) + 0xCF2F);
    }
    #[test]
    fn computed_half_and_full_time_clock_results_close_the_parent_loop() {
        for number in [3, 6] {
            let s = fixture_input(number);
            let mut session = CallerSession::admit(&s, true).unwrap();
            assert!(session.clock_boundary().unwrap().phase_complete);
            let image = session.cpu.e.image.clone();
            assert!(session.advance().is_err());
            assert!(session.clock_boundary().is_err());
            assert_eq!(image, session.cpu.e.image);
        }
    }
    #[test]
    fn original171caller_iterations_share_one_native_image_and_clock() {
        let fixture = load("engine/tests/fixtures/watched_caller/case-00.json");
        let input: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let mut session = CallerSession::admit(&input, true).unwrap();
        let mut values = initial_values(&input);
        let sources = fixture["originals"].as_array().unwrap();
        assert_eq!(sources.len(), 171);
        let base = fixture["event_draw_base"].as_u64().unwrap();
        for (number, item) in sources.iter().enumerate() {
            let original = load(item["path"].as_str().unwrap());
            let out = session
                .advance()
                .unwrap_or_else(|e| panic!("iteration{}: {e:?}", number + 1));
            compare(out, &original, &input, &mut values, base);
        }
    }
    #[test]
    fn all_eight_original_clock_boundaries_compute_without_outcome_inputs() {
        for number in 1..=8 {
            let fixture = load(&format!(
                "engine/tests/fixtures/watched_caller/case-{number:02}.json"
            ));
            let input: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
            let mut session = CallerSession::admit(&input, true).unwrap();
            let mut values = initial_values(&input);
            let original = load(fixture["originals"][0]["path"].as_str().unwrap());
            compare(
                session.clock_boundary().unwrap(),
                &original,
                &input,
                &mut values,
                0,
            );
        }
    }
}
