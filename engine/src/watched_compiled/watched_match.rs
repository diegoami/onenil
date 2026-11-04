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
    #[test]
    fn menu_templates_preserve_gameplay_and_reject_unproven_page_or_caller() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_caller/case-00.json");
        let fixture: Value = serde_json::from_slice(&std::fs::read(root).unwrap()).unwrap();
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        for (template, caller, ret, header, watching, succeeds) in [
            (0x132E, 0x1B55, 0x346C, 0xDD5C, true, true),
            (0x6346, 0x1B55, 0x347B, 0xDD5C, true, true),
            (0x132E, 0x0AB9, 0xD6D6, 0xDD5C, true, true),
            (0x25CC, 0x0AB9, 0xD6E5, 0xDD5C, true, true),
            (0x4F20, 0x0AB9, 0x971A, 0xDD5C, true, true),
            (0x132E, 0x1B55, 0x346C, 0xDD58, true, false),
            (0x132E, 0x1B55, 0x1234, 0xDD5C, true, false),
            (0x132E, 0x1B55, 0x346C, 0xDD5C, false, false),
        ] {
            let mut c = action::machine(&state).unwrap();
            let pointer = state
                .pointers
                .iter()
                .find(|p| p.kind == action::PointerKind::MainShown)
                .unwrap();
            let ds = 16 * usize::from(c.r[11]);
            for (index, value) in [pointer.offset.to_le_bytes(), pointer.segment.to_le_bytes()]
                .concat()
                .into_iter()
                .enumerate()
            {
                c.known[ds + 0xDD5C + index] = true;
                c.e.image[ds + 0xDD5C + index] = value;
            }
            if watching {
                c.watch_trace = Some(Vec::new());
            }
            c.r[9] = c.segment_const(caller);
            let regs = c.r;
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            let before = c.e.image.clone();
            for value in [c.r[11], template, c.r[11], header] {
                c.push(value).unwrap();
            }
            let outcome = c.invoke(
                super::super::projections::menu_template,
                true,
                0x232A,
                0x052F,
                ret,
            );
            assert_eq!(
                outcome.is_ok(),
                succeeds,
                "template{template:X}/caller{caller:X}:{ret:X}"
            );
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at) {
                    assert_eq!(old, new, "address{at:X}");
                }
            }
            if succeeds {
                assert_eq!(c.r, regs);
                assert_eq!(c.presentation.len(), 1);
            }
        }
    }
    #[test]
    fn original_ui_leaves_match_all_48_style_results_and_preserve_gameplay() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let read = |path: &str| -> Value {
            serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
        };
        let fixture = read("engine/tests/fixtures/watched_caller/case-00.json");
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let inputs = read("records/T41/ui-leaves-input-r0001.json");
        let proof = read("records/T41/ui-leaves-boundary-r0001/result.json");
        let observations = proof["observations"].as_array().unwrap();
        assert_eq!(observations.len(), 48);
        for observation in observations {
            let mut c = action::machine(&state).unwrap();
            for fact in inputs["facts"].as_array().unwrap() {
                let address = fact["address"].as_u64().unwrap() as usize;
                c.e.image[address] = fact["value"].as_u64().unwrap() as u8;
                c.known[address] = true;
            }
            let shown = state
                .pointers
                .iter()
                .find(|p| p.kind == action::PointerKind::MainShown)
                .unwrap();
            let ds = 16 * usize::from(c.r[11]);
            for (index, value) in [shown.offset.to_le_bytes(), shown.segment.to_le_bytes()]
                .concat()
                .into_iter()
                .enumerate()
            {
                c.e.image[ds + 0xDD5C + index] = value;
                c.known[ds + 0xDD5C + index] = true;
            }
            c.watch_trace = Some(Vec::new());
            let args: Vec<u16> = observation["arguments"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as u16)
                .collect();
            let root = observation["root"].as_str().unwrap();
            let (function, cs, ip): (super::super::compiled::Proc, u16, u16) = match root {
                "232A:052F" => (super::super::projections::menu_template, 0x232A, 0x052F),
                "232A:0283" => (super::super::projections::menu_right_text, 0x232A, 0x0283),
                "232A:039D" => (super::super::projections::menu_left_text, 0x232A, 0x039D),
                "232A:041B" => (super::super::projections::menu_styled_text, 0x232A, 0x041B),
                "1B55:0EE3" => (super::super::projections::menu_team_name, 0x1B55, 0x0EE3),
                "1B55:1FF1" => (super::super::projections::menu_player_name, 0x1B55, 0x1FF1),
                _ => panic!("unknown proof root"),
            };
            let (caller, ret) = if root == "232A:052F" {
                match args[2] {
                    0x132E => (0x1B55, 0x346C),
                    0x6346 => (0x1B55, 0x347B),
                    0x25CC => (0x0AB9, 0xD6E5),
                    0x4F20 => (0x0AB9, 0x971A),
                    0x3B82 => (0x1B55, 0x334F),
                    0x3D72 => (0x0AB9, 0x9FE3),
                    _ => unreachable!(),
                }
            } else {
                (0x0AB9, 0xEBE2)
            };
            c.r[9] = c.segment_const(caller);
            let regs = c.r;
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            let before = c.e.image.clone();
            for value in args.iter().rev() {
                c.push(*value).unwrap();
            }
            c.invoke(function, true, cs, ip, ret)
                .unwrap_or_else(|e| panic!("{root} {args:?}: {e}"));
            assert_eq!(
                (c.r[4], c.r[5], c.r[11]),
                (regs[4], regs[5], regs[11]),
                "{root} {args:?}"
            );
            assert_eq!(
                c.read(c.r[11], 0xDD64, 16).unwrap(),
                observation["style_after"].as_u64().unwrap() as u16,
                "{root} {args:?}"
            );
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at) && !(ds + 0xDD64..ds + 0xDD66).contains(&at)
                {
                    assert_eq!(old, new, "{root} {args:?} address{at:X}");
                }
            }
        }
    }
    #[test]
    fn formation_counts_match_both_original_clubs_without_text_or_gameplay_writes() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let read = |path: &str| -> Value {
            serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
        };
        let fixture = read("engine/tests/fixtures/watched_caller/case-00.json");
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let inputs = read("records/T41/formation-input-r0001.json");
        let proof = read("records/T41/formation-boundary-r0001/result.json");
        for observation in proof["observations"].as_array().unwrap() {
            let mut c = action::machine(&state).unwrap();
            for fact in inputs["facts"].as_array().unwrap() {
                let address = fact["address"].as_u64().unwrap() as usize;
                c.e.image[address] = fact["value"].as_u64().unwrap() as u8;
                c.known[address] = true;
            }
            c.watch_trace = Some(Vec::new());
            c.r[9] = c.segment_const(0x1B55);
            let regs = c.r;
            let before = c.e.image.clone();
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            let club = observation["club"].as_u64().unwrap() as u16;
            for value in [c.r[10], c.r[5] - 0x106, club] {
                c.push(value).unwrap();
            }
            c.invoke(
                super::super::projections::menu_formation,
                true,
                0x1B55,
                0x10AF,
                0x33AE,
            )
            .unwrap();
            assert_eq!((c.r[4], c.r[5], c.r[11]), (regs[4] - 4, regs[5], regs[11]));
            assert_eq!(
                c.presentation,
                vec![
                    json!({"site":"1B55:10AF","kind":"formation","club":club,"counts":observation["counts"]})
                ]
            );
            assert!(c.draws.is_empty());
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at) {
                    assert_eq!(old, new, "club{club}/address{at:X}");
                }
            }
        }
    }
    #[test]
    fn grouped_number_native_helper_matches_all_original_outputs_and_hidden_abi() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let read = |path: &str| -> Value {
            serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
        };
        let fixture = read("engine/tests/fixtures/watched_caller/case-00.json");
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let proof = read("records/T41/grouped-integer-r0001/result.json");
        assert_eq!(proof["observations"].as_array().unwrap().len(), 12);
        for observation in proof["observations"].as_array().unwrap() {
            let mut c = action::machine(&state).unwrap();
            let regs = c.r;
            let before = c.e.image.clone();
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            let number = observation["raw_value"].as_u64().unwrap() as u32;
            let output = c.r[4] - 512;
            for value in [c.r[10], output, (number >> 16) as u16, number as u16] {
                c.push(value).unwrap();
            }
            c.invoke(
                super::super::projections::grouped_integer,
                true,
                0x1B55,
                0x0F95,
                0x1038,
            )
            .unwrap();
            for (index, byte) in observation["pascal_bytes"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
            {
                assert_eq!(
                    c.read(c.r[10], output + index as u16, 8).unwrap(),
                    byte.as_u64().unwrap() as u16,
                    "number{number}"
                );
            }
            assert_eq!((c.r[4], c.r[5], c.r[11]), (regs[4] - 4, regs[5], regs[11]));
            assert!(c.draws.is_empty());
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at) {
                    assert_eq!(old, new, "number{number}/address{at:X}");
                }
            }
        }
    }
    #[test]
    fn fixture_title_preserves_hidden_output_frame_and_rejects_unproven_inputs() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_caller/case-00.json");
        let fixture: Value = serde_json::from_slice(&std::fs::read(root).unwrap()).unwrap();
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        for (week, stage, ret, valid) in [
            (6, 0, 0xD75D, true),
            (7, 0, 0xD75D, false),
            (6, 1, 0xD75D, false),
            (6, 0, 0x1234, false),
        ] {
            let mut c = action::machine(&state).unwrap();
            c.watch_trace = Some(Vec::new());
            let regs = c.r;
            let before = c.e.image.clone();
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            for value in [c.r[10], c.r[5] - 0x106, stage, week] {
                c.push(value).unwrap();
            }
            let result = c.invoke(
                super::super::projections::menu_fixture_title,
                true,
                0x1B55,
                0x142B,
                ret,
            );
            assert_eq!(result.is_ok(), valid);
            if valid {
                assert_eq!((c.r[4], c.r[5], c.r[11]), (regs[4] - 4, regs[5], regs[11]));
                assert_eq!(c.presentation.len(), 1);
            }
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at) {
                    assert_eq!(old, new, "address{at:X}");
                }
            }
        }
    }
    #[test]
    fn original_menu_cursor_helpers_apply_external_continue_keys_and_reject_other_callers() {
        fn controller(c: &mut Cpu) -> Result<(), String> {
            c.enter(0x106)?;
            let cursor = c.r[5].wrapping_sub(5);
            c.write(c.r[10], cursor, 8, c.r[0])?;
            for value in [c.r[10], cursor, 4] {
                c.push(value)?;
            }
            c.invoke(
                super::super::action_graph::p_232a_0f8b,
                true,
                0x232A,
                0x0F8B,
                0xD81C,
            )?;
            c.r[2] = c.read(c.r[10], cursor, 8)?;
            c.r[4] = c.r[5];
            c.r[5] = c.pop()?;
            c.ret(true, 0)
        }
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let read = |path: &str| -> Value {
            serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
        };
        let fixture = read("engine/tests/fixtures/watched_caller/case-00.json");
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let proof = read("records/T41/menu-input-boundary-r0001/result.json");
        for observation in proof["observations"].as_array().unwrap() {
            let mut c = action::machine(&state).unwrap();
            c.watch_trace = Some(Vec::new());
            let ds = 16 * usize::from(c.r[11]);
            let main = state
                .pointers
                .iter()
                .find(|p| p.kind == action::PointerKind::Main)
                .unwrap();
            let shown = state
                .pointers
                .iter()
                .find(|p| p.kind == action::PointerKind::MainShown)
                .unwrap();
            for (header, pointer) in [(0xDD58, main), (0xDD5C, shown)] {
                for (index, byte) in [pointer.offset.to_le_bytes(), pointer.segment.to_le_bytes()]
                    .concat()
                    .into_iter()
                    .enumerate()
                {
                    c.e.image[ds + header + index] = byte;
                    c.known[ds + header + index] = true;
                }
                for index in 0..4000 {
                    let at = pointer.address() + index;
                    c.e.image[at] = if header == 0xDD58 {
                        0
                    } else {
                        (index % 251) as u8
                    };
                    c.known[at] = true;
                }
            }
            for offset in 0xDE88..0xDE90 {
                c.e.image[ds + offset] = 0;
                c.known[ds + offset] = true;
            }
            c.r[0] = observation["cursor_before"].as_u64().unwrap() as u16;
            let regs = c.r;
            let before = c.e.image.clone();
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            c.invoke(controller, true, 0x0AB9, 0xD5D6, 0x7E4F).unwrap();
            assert_eq!(c.r[2], observation["cursor_after"].as_u64().unwrap() as u16);
            let key = observation["key_code"].as_u64().unwrap() as u16;
            assert_eq!(c.read(c.r[11], 0xDE88, 16).unwrap(), key);
            assert_eq!(c.read(c.r[11], 0xDE8A, 16).unwrap(), key);
            assert_eq!(c.read(c.r[11], 0xDE8C, 16).unwrap(), 0);
            assert_eq!((c.r[4], c.r[5], c.r[11]), (regs[4], regs[5], regs[11]));
            assert_eq!(
                &c.e.image[main.address()..main.address() + 4000],
                &before[shown.address()..shown.address() + 4000]
            );
            assert!(c.draws.is_empty());
            assert!(c.events.is_empty());
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at)
                    && !(main.address()..main.address() + 4000).contains(&at)
                    && !(ds + 0xDE88..ds + 0xDE8E).contains(&at)
                {
                    assert_eq!(old, new, "address{at:X}");
                }
            }
        }
        let mut c = action::machine(&state).unwrap();
        c.watch_trace = Some(Vec::new());
        assert!(c
            .invoke(
                super::super::projections::continue_menu_key,
                true,
                0x24BC,
                0x0C17,
                0x1234
            )
            .unwrap_err()
            .contains("outside proven halftime"));
    }
    #[test]
    fn native_lineup_validator_matches_nine_original_squad_cases_and_preserves_gameplay() {
        fn controller(c: &mut Cpu) -> Result<(), String> {
            c.enter(0x106)?;
            let bp = c.r[5];
            let sp = c.r[4];
            let club = c.read(c.r[11], 0xD6E4, 16)?;
            c.write(c.r[10], bp.wrapping_sub(4), 8, club)?;
            c.push(bp)?;
            c.invoke(
                super::super::lineup_validation::run,
                false,
                0x0AB9,
                0xD29E,
                0xD8D6,
            )?;
            if (c.r[5], c.r[4]) != (bp, sp) {
                return Err("validator changed parent frame".into());
            }
            c.r[2] = c.read(c.r[10], bp.wrapping_sub(4), 8)?;
            c.r[4] = bp;
            c.r[5] = c.pop()?;
            c.ret(true, 0)
        }
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let read = |path: &str| -> Value {
            serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
        };
        let fixture = read("engine/tests/fixtures/watched_caller/case-00.json");
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let facts = [
            read("records/T41/formation-input-r0001.json"),
            read("records/T41/ui-leaves-input-r0001.json"),
        ];
        let protocol = read("records/T41/lineup-validation-r0002/start.json");
        let proof = read("records/T41/lineup-validation-r0002/result.json");
        assert_eq!(protocol["cases"].as_array().unwrap().len(), 9);
        for (case, original) in protocol["cases"]
            .as_array()
            .unwrap()
            .iter()
            .zip(proof["observations"].as_array().unwrap())
        {
            let mut c = action::machine(&state).unwrap();
            c.watch_trace = Some(Vec::new());
            for input in &facts {
                for fact in input["facts"].as_array().unwrap() {
                    let address = fact["address"].as_u64().unwrap() as usize;
                    c.e.image[address] = fact["value"].as_u64().unwrap() as u8;
                    c.known[address] = true;
                }
            }
            let ds = 16 * usize::from(c.r[11]);
            c.e.image[ds + 0xD18] = case["mode"].as_u64().unwrap() as u8;
            c.known[ds + 0xD18] = true;
            for patch in case["patches"].as_array().unwrap() {
                let address = patch["address"].as_u64().unwrap() as usize;
                c.e.image[address] = patch["value"].as_u64().unwrap() as u8;
                c.known[address] = true;
            }
            let regs = c.r;
            let before = c.e.image.clone();
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            c.invoke(controller, true, 0x0AB9, 0xD5D6, 0x7E4F)
                .unwrap_or_else(|e| panic!("{}:{e}", case["name"]));
            assert_eq!(
                c.r[0] & 255,
                original["returned_valid"].as_u64().unwrap() as u16,
                "{}",
                case["name"]
            );
            assert_eq!(
                c.r[2],
                original["restored_active_club"].as_u64().unwrap() as u16
            );
            assert_eq!((c.r[4], c.r[5], c.r[11]), (regs[4], regs[5], regs[11]));
            let counts: Vec<Value> = c
                .presentation
                .iter()
                .filter(|v| v["kind"] == "lineup_validation")
                .map(|v| json!({"side":v["side"],"counts":v["counts"]}))
                .collect();
            assert_eq!(
                counts,
                *original["role_counts"].as_array().unwrap(),
                "{}",
                case["name"]
            );
            assert!(c.draws.is_empty());
            assert!(c.events.is_empty());
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at) {
                    assert_eq!(old, new, "{} address{at:X}", case["name"]);
                }
            }
        }
        let mut c = action::machine(&state).unwrap();
        c.watch_trace = Some(Vec::new());
        assert!(c
            .invoke(
                super::super::lineup_validation::run,
                false,
                0x0AB9,
                0xD29E,
                0x1234
            )
            .unwrap_err()
            .contains("outside proven menu"));
    }
    #[test]
    fn replacement_announcement_matches_eight_original_styles_and_restores_gameplay() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let read = |path: &str| -> Value {
            serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
        };
        let fixture = read("engine/tests/fixtures/watched_caller/case-00.json");
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        let facts = [
            read("records/T41/formation-input-r0001.json"),
            read("records/T41/ui-leaves-input-r0001.json"),
        ];
        let protocol = read("records/T41/replacement-banner-r0002/start.json");
        let proof = read("records/T41/replacement-banner-r0002/result.json");
        assert_eq!(protocol["cases"].as_array().unwrap().len(), 8);
        for (case, original) in protocol["cases"]
            .as_array()
            .unwrap()
            .iter()
            .zip(proof["observations"].as_array().unwrap())
        {
            let mut c = action::machine(&state).unwrap();
            c.watch_trace = Some(Vec::new());
            for input in &facts {
                for fact in input["facts"].as_array().unwrap() {
                    let address = fact["address"].as_u64().unwrap() as usize;
                    c.e.image[address] = fact["value"].as_u64().unwrap() as u8;
                    c.known[address] = true;
                }
            }
            let ds = 16 * usize::from(c.r[11]);
            for offset in 0xDE88..0xDE90 {
                c.e.image[ds + offset] = 0;
                c.known[ds + offset] = true;
            }
            for patch in case["patches"].as_array().unwrap() {
                let address = patch["address"].as_u64().unwrap() as usize;
                c.e.image[address] = patch["value"].as_u64().unwrap() as u8;
                c.known[address] = true;
            }
            let regs = c.r;
            let before = c.e.image.clone();
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            for value in [
                case["second"].as_u64().unwrap() as u16,
                case["first"].as_u64().unwrap() as u16,
            ] {
                c.push(value).unwrap();
            }
            c.invoke(
                super::super::projections::replacement_announcement,
                false,
                0x0AB9,
                0xADD0,
                0xAF62,
            )
            .unwrap_or_else(|e| panic!("{}:{e}", case["name"]));
            assert_eq!(
                c.read(c.r[11], 0xDD64, 16).unwrap(),
                original["style_after"].as_u64().unwrap() as u16,
                "{}",
                case["name"]
            );
            assert_eq!((c.r[4], c.r[5], c.r[11]), (regs[4], regs[5], regs[11]));
            assert!(c.draws.is_empty());
            assert!(c.events.is_empty());
            assert_eq!(
                c.presentation.last().unwrap()["kind"],
                "replacement_announcement"
            );
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at) && !(ds + 0xDD64..ds + 0xDD66).contains(&at)
                {
                    assert_eq!(old, new, "{} address{at:X}", case["name"]);
                }
            }
        }
        let mut c = action::machine(&state).unwrap();
        c.watch_trace = Some(Vec::new());
        assert!(c
            .invoke(
                super::super::projections::replacement_announcement,
                false,
                0x0AB9,
                0xADD0,
                0x1234
            )
            .unwrap_err()
            .contains("outside proven caller"));
    }
    #[test]
    fn final_footer_and_exit_key_preserve_world_and_require_finished_clock() {
        fn key_only(c: &mut Cpu) -> Result<(), String> {
            c.invoke(
                super::super::action_graph::p_232a_099b,
                true,
                0x232A,
                0x099B,
                0xE1C4,
            )
        }
        fn footer(c: &mut Cpu) -> Result<(), String> {
            c.enter(0)?;
            for value in [c.r[11], 0x4B12, c.r[11], 0xDD58] {
                c.push(value)?;
            }
            c.invoke(
                super::super::action_graph::p_232a_052f,
                true,
                0x232A,
                0x052F,
                0xE1BF,
            )?;
            key_only(c)?;
            let key = c.read(c.r[11], 0xDE88, 8)?;
            c.push(key)?;
            c.invoke(
                super::super::action_graph::p_26c9_1986,
                true,
                0x26C9,
                0x1986,
                0xE1CD,
            )?;
            if c.r[0] & 255 != 78 {
                return Err("final exit key did not uppercase to N".into());
            }
            c.r[4] = c.r[5];
            c.r[5] = c.pop()?;
            c.ret(false, 2)
        }
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_caller/case-00.json");
        let fixture: Value = serde_json::from_slice(&std::fs::read(root).unwrap()).unwrap();
        let state: State = serde_json::from_value(fixture["initial"].clone()).unwrap();
        for finished in [true, false] {
            let mut c = action::machine(&state).unwrap();
            c.watch_trace = Some(Vec::new());
            c.watch_tick = 1;
            let ds = 16 * usize::from(c.r[11]);
            for offset in 0xDE88..0xDE90 {
                c.e.image[ds + offset] = 0;
                c.known[ds + offset] = true;
            }
            let main = state
                .pointers
                .iter()
                .find(|p| p.kind == action::PointerKind::Main)
                .unwrap();
            for (index, byte) in [main.offset.to_le_bytes(), main.segment.to_le_bytes()]
                .concat()
                .into_iter()
                .enumerate()
            {
                c.e.image[ds + 0xDD58 + index] = byte;
                c.known[ds + 0xDD58 + index] = true;
            }
            if finished {
                c.e.image[ds + 0xD0AE] = 0;
                c.e.image[ds + 0xD0AF] = 0;
            }
            let before = c.e.image.clone();
            let regs = c.r;
            let stack = 16 * usize::from(c.r[10]) + usize::from(c.r[4]);
            if finished {
                c.push(0).unwrap();
                c.invoke(footer, false, 0x0AB9, 0xE1B0, 0xEBE2).unwrap();
                assert_eq!((c.r[4], c.r[5], c.r[11]), (regs[4], regs[5], regs[11]));
                assert_eq!(c.read(c.r[11], 0xDE88, 16).unwrap(), 110);
                assert_eq!(c.read(c.r[11], 0xDE8A, 16).unwrap(), 110);
                assert_eq!(c.presentation.last().unwrap()["intent"], "finish_match");
            } else {
                assert!(key_only(&mut c)
                    .unwrap_err()
                    .contains("outside proven halftime/final"));
            }
            assert!(c.draws.is_empty());
            assert!(c.events.is_empty());
            for (at, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
                if !(stack - 4096..stack).contains(&at) && !(ds + 0xDE88..ds + 0xDE8E).contains(&at)
                {
                    assert_eq!(old, new, "address{at:X}");
                }
            }
        }
    }
}
