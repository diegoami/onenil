//! [confirmed] Original lifecycle7C67→7DCB→E05E→E435→EBE2,staticd9e20a4/F40.
//! [designed] One initial fact image;all gameplay/lifecycle executes in Rust.
use super::{
    action::{self, Fault, Kind, State},
    compiled::Cpu,
};
use serde_json::{json, Value};
pub(super) fn snapshot(c: &Cpu) -> Result<Value, String> {
    let ds = c.r[11];
    let read = |offset, width| c.read(ds, offset, width);
    let mut scenes = Vec::new();
    for index in 1..=22u16 {
        let mut scene = json!({"index":index});
        for offset in [0, 2, 4, 8, 10, 14, 16, 20] {
            scene[format!("word{offset}")] = json!(read(0xCCEF + 25 * index + offset, 16)?);
        }
        for offset in [6, 7, 12, 13, 18, 19, 22, 23, 24] {
            scene[format!("byte{offset}")] = json!(read(0xCCEF + 25 * index + offset, 8)?);
        }
        scenes.push(scene);
    }
    let mut ball = json!({});
    for offset in [0, 2, 4, 7, 9, 11] {
        ball[format!("word{offset}")] = json!(read(0xCF46 + offset, 16)?);
    }
    for offset in [6, 13, 14] {
        ball[format!("byte{offset}")] = json!(read(0xCF46 + offset, 8)?);
    }
    // Reference's legacy `scores` are D086/D087 scratch during play;live counters D0C4/C5.
    Ok(
        json!({"seed":read(0xA51C,16)?,"elapsed":read(0xD0B2,16)?,"remaining":read(0xD0AE,16)?,"quick":read(0xD074,8)?,"actor":read(0xD09F,8)?,"secondary":read(0xD0A0,8)?,"ball":ball,"scenes":scenes,"scores":[read(0xD086,8)?,read(0xD087,8)?],"phase":read(0xD0AC,8)?,"active":read(0xD0B8,8)?,"hold":read(0xD0B9,8)?,"stoppage":read(0xD0B4,16)?}),
    )
}
#[derive(serde::Serialize)]
pub struct MatchFault {
    #[serde(flatten)]
    pub fault: Fault,
    pub ticks: Vec<Value>,
    pub draws: Vec<crate::Draw>,
    pub events: Vec<Value>,
    pub pending: Option<Value>,
    pub diagnostic: Value,
}
impl From<Fault> for MatchFault {
    fn from(fault: Fault) -> Self {
        Self {
            fault,
            ticks: vec![],
            draws: vec![],
            events: vec![],
            pending: None,
            diagnostic: Value::Null,
        }
    }
}
pub fn run(s: &State, no_pending_key: bool) -> Result<Value, MatchFault> {
    let failure = |message| {
        MatchFault::from(Fault {
            message,
            missing: None,
        })
    };
    if s.kind != Kind::WatchedMatch || !no_pending_key || s.regs[5] != s.regs[4].wrapping_add(6) {
        return Err(failure(
            "watched match requires original parent frame and no-pending-key admission".into(),
        ));
    }
    let mut c = action::machine(s).map_err(failure)?;
    let ds = s.regs[11];
    let ss = s.regs[10];
    let sp = s.regs[4];
    for (offset, expected) in [(8, 0x7DCB), (16, 0xE05E), (28, 0xE435), (46, 0xEBE2)] {
        if c.read(ss, sp.wrapping_add(offset), 16)
            .map_err(|message| Fault {
                message,
                missing: c.missing.get(),
            })?
            != expected
        {
            return Err(failure("watched match outer return frame differs".into()));
        }
    }
    c.watch_trace = Some(Vec::new());
    let result: Result<Value, String> = (|| {
        c.invoke_entry(super::match_graph::p_0ab9_7c67, 0x0AB9, 0x7DCB)?;
        c.invoke_entry(super::match_graph::p_0ab9_7dcb, 0x0AB9, 0xE05E)?;
        c.invoke_entry(super::match_graph::p_0ab9_e05e, 0x0AB9, 0xE435)?;
        c.invoke_entry(super::match_graph::p_0ab9_e435, 0x0AB9, 0xEBE2)?;
        if c.watch_pending.is_some() {
            return Err("watched match left an unfinished tick".into());
        }
        if c.r[4] != sp + 52 || c.r[11] != ds {
            return Err("watched match final return ABI differs".into());
        }
        let final_state = snapshot(&c)?;
        let live = [c.read(ds, 0xD0C4, 8)?, c.read(ds, 0xD0C5, 8)?];
        Ok(
            json!({"ticks":c.watch_trace,"draws":c.draws,"events":c.events,"final":final_state,"live_goals":live,"return":"0AB9:EBE2"}),
        )
    })();
    result.map_err(|message| {
        let original_missing=c.missing.get();
        let partial=if c.r[11]==ds {snapshot(&c).ok()}else{None};
        let diagnostic=json!({"regs":c.r,"actor":c.e.image[16*usize::from(ds)+0xD09F],"secondary":c.e.image[16*usize::from(ds)+0xD0A0],"call_chain":c.call_chain(),"partial":partial});
        MatchFault {
        fault: Fault {
            message: format!(
                "watched tick{} draw{}: {message}",
                c.watch_tick,
                c.draws.len()
            ),
            missing: original_missing,
        },
        ticks: c.watch_trace.take().unwrap_or_default(),
        draws: std::mem::take(&mut c.draws),
        events: std::mem::take(&mut c.events),
        pending:c.watch_pending.take(),
        diagnostic,
    }})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_hardware_clock_restoration_preserves_all_admitted_gameplay_and_far_abi() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_caller/case-00.json");
        let fixture: Value = serde_json::from_slice(&std::fs::read(root).unwrap()).unwrap();
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let mut c = action::machine(&state).unwrap();
        let cs = c.r[9];
        c.push(cs).unwrap();
        c.push(0xE475).unwrap();
        let before = c.e.image.clone();
        let known = c.known.clone();
        let registers = c.r;
        super::super::projections::restore_presentation_clock(&mut c).unwrap();
        assert_eq!(c.e.image, before);
        assert_eq!(c.known, known);
        let mut expected = registers;
        expected[4] += 4;
        expected[9] = cs;
        assert_eq!(c.r, expected);
        assert_eq!(
            c.presentation,
            vec![json!({"site":"1998:00CD","kind":"restore_presentation_clock"})]
        );
    }
    #[test]
    fn original_attribute_loop_writes_only_attributes_and_checks_baselines() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_caller/case-00.json");
        let fixture: Value = serde_json::from_slice(&std::fs::read(root).unwrap()).unwrap();
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let pointer = state
            .pointers
            .iter()
            .find(|p| p.kind == action::PointerKind::Main)
            .unwrap();
        let run = |c: &mut Cpu, count| -> Result<(), String> {
            c.push(pointer.segment)?;
            c.push(pointer.offset + 0x90E)?;
            c.push(0x47)?;
            c.push(count)?;
            c.invoke(
                super::super::action_graph::p_24bc_034b,
                true,
                0x24BC,
                0x034B,
                0x036A,
            )
        };
        let mut c = action::machine(&state).unwrap();
        let before = c.e.image.clone();
        let sp = c.r[4];
        let bp = c.r[5];
        let ds = c.r[11];
        run(&mut c, 2).unwrap();
        let base = pointer.address() + 0x90E;
        assert_eq!(
            [c.e.image[base], c.e.image[base + 2]],
            [before[base], before[base + 2]]
        );
        assert_eq!([c.e.image[base + 1], c.e.image[base + 3]], [0x47, 0x47]);
        assert_eq!((c.r[4], c.r[5], c.r[11], c.r[1]), (sp, bp, ds, 0));
        let mut missing = action::machine(&state).unwrap();
        missing.known[base + 1] = false;
        assert!(run(&mut missing, 2).is_err());
        let fault = missing.missing.get().unwrap();
        assert!(fault.writing);
        assert_eq!(fault.address, base + 1);
        assert_eq!(fault.pc, (0x24BC, 0x0365));
        let mut empty = action::machine(&state).unwrap();
        empty.known[base + 1] = false;
        run(&mut empty, 0).unwrap();
        assert_eq!(&empty.e.image[base..base + 4], &before[base..base + 4]);
    }
    #[test]
    fn original_reverse_glyph_copy_preserves_attributes_ax_and_direction() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_caller/case-00.json");
        let fixture: Value = serde_json::from_slice(&std::fs::read(root).unwrap()).unwrap();
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let pointer = state
            .pointers
            .iter()
            .find(|p| p.kind == action::PointerKind::Main)
            .unwrap();
        let mut c = action::machine(&state).unwrap();
        let sp = c.r[4];
        let bp = c.r[5];
        let ds = c.r[11];
        let ax = c.r[0];
        let source = sp - 512;
        for (i, value) in [2, 52, 50].into_iter().enumerate() {
            c.write(c.r[10], source + i as u16, 8, value).unwrap();
        }
        let base = pointer.address() + 0x90E;
        let attributes = [c.e.image[base + 1], c.e.image[base + 3]];
        c.push(c.r[10]).unwrap();
        c.push(source).unwrap();
        c.push(pointer.segment).unwrap();
        c.push(pointer.offset + 0x910).unwrap();
        c.invoke(
            super::super::action_graph::p_24bc_0294,
            true,
            0x24BC,
            0x0294,
            0x02AF,
        )
        .unwrap();
        assert_eq!([c.e.image[base], c.e.image[base + 2]], [52, 50]);
        assert_eq!([c.e.image[base + 1], c.e.image[base + 3]], attributes);
        assert_eq!(
            (c.r[0], c.r[1], c.r[4], c.r[5], c.r[6], c.r[11]),
            (ax, 0, sp, bp, source, ds)
        );
        assert!(c.df);
        assert!(!c.z);
        assert!(!c.cf);
    }
    #[test]
    fn statistics_projection_changes_only_the_consumed_key_and_checks_callers() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_caller/case-00.json");
        let fixture: Value = serde_json::from_slice(&std::fs::read(root).unwrap()).unwrap();
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        for return_site in [0x7E2F, 0xE45E] {
            let mut c = action::machine(&state).unwrap();
            let ds = 16 * usize::from(c.r[11]);
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            let regs = c.r;
            // Derived presentation-key input,matching the original isolation probes.
            c.known[ds + 0xDE88] = true;
            c.e.image[ds + 0xDE88] = 7;
            let before = c.e.image.clone();
            c.push(1).unwrap();
            c.push(0).unwrap();
            c.invoke(
                super::super::projections::statistics_display,
                true,
                0x1998,
                0x045C,
                return_site,
            )
            .unwrap();
            for (at, (actual, original)) in c.e.image.iter().zip(before.iter()).enumerate() {
                if !(stack - 4096..stack + 16).contains(&at) && at != ds + 0xDE88 {
                    assert_eq!(
                        actual, original,
                        "statistics projection changed non-presentation address{at:X}"
                    );
                }
            }
            assert_eq!(c.e.image[ds + 0xDE88], 0);
            assert_eq!(c.r, regs);
            assert_eq!(c.presentation.len(), 1);
        }
        let mut c = action::machine(&state).unwrap();
        c.push(0).unwrap();
        c.push(0).unwrap();
        assert!(c
            .invoke(
                super::super::projections::statistics_display,
                true,
                0x1998,
                0x045C,
                0x1234
            )
            .unwrap_err()
            .contains("outside the proven lifecycle"));
    }
}
