//! [confirmed] F39/c7637bd;compiled original35F8 numeric helper graph.
//! [designed] Typed root facts and temporary Pascal frames;no runtime decoding.
pub mod action;
mod action_graph;
mod adjustment;
mod compiled;
mod projections;
mod selection;
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Region {
    Ds,
    P8,
    P15,
    Club,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub region: Region,
    pub offset: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub scene: Scene,
    pub seed: u16,
    pub cells: Vec<Cell>,
}
/// [confirmed] Original3846's keeper path reads future BP-5 without a store.
/// [designed] One required raw entry ABI byte;other scratch stays checked.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SelectionState {
    pub scene: Scene,
    pub seed: u16,
    pub cells: Vec<Cell>,
    pub frame_byte5: u8,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub draws: Vec<Draw>,
}
fn location(cell: &Cell) -> Result<usize, String> {
    let offset = usize::from(cell.offset);
    let base = match cell.region {
        Region::Ds => {
            if !((0x900..=0xBFF).contains(&offset)
                || (0xD00..=0xD7F).contains(&offset)
                || ((0x957D..=0xAD9A).contains(&offset) && !(0xA51C..=0xA51D).contains(&offset))
                || (0xCCEF..0xD7BA).contains(&offset))
            {
                return Err("unsupported compiled data field".into());
            }
            0
        }
        Region::P8 => {
            if offset >= 8 * 3921 {
                return Err("unsupported compiled P8 field".into());
            }
            crate::layout::P8
        }
        Region::P15 => {
            if offset >= 15 * 3921 {
                return Err("unsupported compiled P15 field".into());
            }
            crate::layout::P15
        }
        Region::Club => {
            if offset >= 62 * 1000 {
                return Err("unsupported compiled club field".into());
            }
            crate::layout::CLUB
        }
    };
    Ok(base + offset)
}
fn machine(s: &State) -> Result<compiled::Cpu, String> {
    if !(1..=22).contains(&s.scene.index) || s.scene.byte7 > 22 || s.scene.word14 > 3920 {
        return Err("unsupported compiled scene identity".into());
    }
    let mut c = compiled::Cpu::new(crate::role_scores::image(Default::default()));
    let root = 0xCCEF + 25 * usize::from(s.scene.index);
    for (i, b) in s.scene.bytes().into_iter().enumerate() {
        c.write(0, (root + i) as u16, 8, u16::from(b))?
    }
    c.write(0, crate::SEED, 16, s.seed)?;
    for (at, offset, segment) in [
        (0xD7BA, 0, 0x1000),
        (0xD7BE, 0, 0x2000),
        (0xD7C2, 0, 0x3000),
        (0xDE84, crate::SEED, 0),
    ] {
        c.write(0, at, 16, offset)?;
        c.write(0, at + 2, 16, segment)?;
    }
    let mut seen = std::collections::BTreeSet::new();
    for field in &s.cells {
        let a = location(field)?;
        if !seen.insert(a) || (root..root + 25).contains(&a) {
            return Err("aliased compiled input field".into());
        }
        c.e.image[a] = field.value;
        c.known[a] = true;
    }
    Ok(c)
}
fn run(c: &mut compiled::Cpu, index: u8) -> Result<(), String> {
    let sp = c.r[4];
    c.push(0)?;
    c.push(0xCCEF + 25 * u16::from(index))?;
    c.invoke(adjustment::p_0ab9_35f8, true, 0x0AB9, 0x35F8, 0)?;
    if c.r[4] != sp || c.r[5] != 0 || c.r[11] != 0 {
        return Err("compiled Pascal frame/DS handoff differs".into());
    }
    Ok(())
}
pub fn advance(s: &State) -> Result<Outcome, String> {
    let mut c = machine(s)?;
    run(&mut c, s.scene.index)?;
    let mut output = s.clone();
    output.scene = Scene::read(&c.e, s.scene.index);
    output.seed = c.e.word(crate::SEED);
    for field in &mut output.cells {
        field.value = c.e.image[location(field)?]
    }
    Ok(Outcome {
        output,
        draws: c.draws,
    })
}
fn run_selection(c: &mut compiled::Cpu, index: u8, frame_byte5: u8) -> Result<(), String> {
    let sp = c.r[4];
    // Four argument bytes,four far-return bytes,saved BP,and original BP-5.
    c.write(c.r[10], sp.wrapping_sub(15), 8, u16::from(frame_byte5))?;
    c.push(0)?;
    c.push(0xCCEF + 25 * u16::from(index))?;
    c.invoke(selection::p_0ab9_3846, true, 0x0AB9, 0x3846, 0)?;
    if c.r[4] != sp || c.r[5] != 0 || c.r[11] != 0 {
        return Err("compiled selection Pascal frame/DS handoff differs".into());
    }
    Ok(())
}
pub fn select_target(input: &SelectionState) -> Result<Outcome, String> {
    let s = &State {
        scene: input.scene.clone(),
        seed: input.seed,
        cells: input.cells.clone(),
    };
    let mut c = machine(s)?;
    run_selection(&mut c, s.scene.index, input.frame_byte5)?;
    let mut output = s.clone();
    output.scene = Scene::read(&c.e, s.scene.index);
    output.seed = c.e.word(crate::SEED);
    for field in &mut output.cells {
        field.value = c.e.image[location(field)?];
    }
    Ok(Outcome {
        output,
        draws: c.draws,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn selection(s: &State, frame_byte5: u8) -> SelectionState {
        SelectionState {
            scene: s.scene.clone(),
            seed: s.seed,
            cells: s.cells.clone(),
            frame_byte5,
        }
    }
    fn sample() -> State {
        let c: serde_json::Value = serde_json::from_str(
            include_str!("../../tests/fixtures/watched_target/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        State {
            scene: serde_json::from_value(c["input"]["scene"].clone()).unwrap(),
            seed: 123,
            cells: vec![],
        }
    }
    fn synthetic() -> State {
        let mut s = sample();
        s.scene.index = 1;
        s.scene.byte7 = 1;
        s.scene.word14 = 1;
        s.scene.word0 = 0x0403;
        s.scene.word10 = 0x0403;
        s.scene.word12 = 0x0509;
        let mut fields = std::collections::BTreeMap::new();
        for (a, v) in [
            (0xD09F, 1),
            (0xD0AC, 45),
            (0xD0AE, 91),
            (0xD0AF, 0),
            (0xA338, 255),
            (0xA339, 1),
            (0xCF47, 0),
            (0xD6E4, 2),
            (0xD6E5, 0),
            (0xD6E6, 1),
            (0xD6E7, 0),
            (0xD6DD, 0),
        ] {
            fields.insert((Region::Ds, a), v);
        }
        for (i, v) in [1, 1, 2, 3].into_iter().enumerate() {
            fields.insert((Region::Ds, 0xA56 + i as u16), v);
        }
        for i in 0..16 {
            fields.insert((Region::Ds, 0xA67 + i), 0);
        }
        for i in 0..8 {
            fields.insert((Region::Ds, 0xA77 + i), 20);
        }
        fields.insert((Region::P8, 14), 5);
        fields.insert((Region::P15, 25), 0);
        fields.insert((Region::Club, 57), 255);
        fields.insert((Region::Club, 119), 255);
        s.cells = fields
            .into_iter()
            .map(|((region, offset), value)| Cell {
                region,
                offset,
                value,
            })
            .collect();
        s
    }
    fn set(s: &mut State, region: Region, offset: u16, value: u8) {
        s.cells
            .iter_mut()
            .find(|c| c.region == region && c.offset == offset)
            .unwrap()
            .value = value;
    }
    fn owned(s: &State) {
        let mut c = machine(s).unwrap();
        c.e.image[0x68000] = 42;
        let before = c.e.image.clone();
        run(&mut c, s.scene.index).unwrap();
        let a = 0xCCEF + 25 * usize::from(s.scene.index);
        for (i, (old, new)) in before.iter().zip(&c.e.image).enumerate() {
            if ![
                a + 12,
                a + 13,
                usize::from(crate::SEED),
                usize::from(crate::SEED) + 1,
            ]
            .contains(&i)
            {
                assert_eq!(old, new, "unexpected write {i:05X}");
            }
        }
    }
    #[test]
    fn adjustment_paths_tactic_precedence_and_wrapped_increment() {
        let mut s = synthetic();
        assert_eq!(advance(&s).unwrap().output.scene.word12, 0x0609);
        owned(&s);
        set(&mut s, Region::Ds, 0xD09F, 12);
        set(&mut s, Region::Ds, 0xCF47, 5);
        assert_eq!(advance(&s).unwrap().output.scene.word12, 0x0409);
        s.scene.byte7 = 2;
        assert_eq!(advance(&s).unwrap().output.scene.word12, 0x04FD);
        owned(&s);
        s = synthetic();
        s.scene.word12 = 0x7F09;
        for i in 0..8 {
            set(&mut s, Region::Ds, 0xA77 + i, 128);
        }
        assert_eq!(advance(&s).unwrap().output.scene.word12, 0x8009);
        s = synthetic();
        set(&mut s, Region::Ds, 0xA77 + 2, 5);
        set(&mut s, Region::Ds, 0xA77 + 6, 5);
        set(&mut s, Region::P15, 25, 16);
        assert_eq!(advance(&s).unwrap().output.scene.word12, 0x0509);
    }
    #[test]
    fn actual_compiled_coin_and_seed_control_adjustment() {
        let mut s = synthetic();
        set(&mut s, Region::P15, 25, 16);
        set(&mut s, Region::Ds, 0xA56 + 2, 1);
        set(&mut s, Region::Ds, 0xCF47, 5);
        for (seed, value, offset) in [(0, 1, 6), (1, 2, 5)] {
            s.seed = seed;
            let o = advance(&s).unwrap();
            assert_eq!(o.draws.len(), 1);
            assert_eq!(o.draws[0].site, "0AB9:348C");
            assert_eq!(o.draws[0].n, 2);
            assert_eq!(o.draws[0].value, value);
            assert_eq!(o.output.scene.word12 >> 8, offset);
            owned(&s);
        }
    }
    #[test]
    fn strict_regions_duplicates_and_readonly_aliases() {
        for mode in 0..6 {
            let mut s = synthetic();
            match mode {
                0 => s.scene.index = 0,
                1 => s.scene.word14 = 3921,
                2 => s.scene.byte7 = 23,
                3 => s.cells.push(s.cells[0].clone()),
                4 => s.cells.push(Cell {
                    region: Region::Ds,
                    offset: 0xCCEF + 25,
                    value: 0,
                }),
                _ => s.cells.push(Cell {
                    region: Region::Ds,
                    offset: 0xD7BA,
                    value: 0,
                }),
            };
            assert!(advance(&s).is_err());
        }
        let mut s = synthetic();
        s.cells
            .retain(|c| !(c.region == Region::P8 && c.offset == 14));
        assert!(advance(&s).unwrap_err().contains("missing"));
    }
    #[test]
    fn original_adjustment_roots() {
        let source = std::fs::read_to_string(format!(
            "{}/tests/fixtures/watched_adjustment/cases.jsonl",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        for line in source.lines() {
            let c: serde_json::Value = serde_json::from_str(line).unwrap();
            let s: State = serde_json::from_value(c["input"].clone()).unwrap();
            assert_eq!(
                serde_json::to_value(advance(&s).unwrap()).unwrap(),
                c["output"]
            );
            owned(&s);
        }
    }
    #[test]
    fn strict_missing_alias_and_expected_inputs() {
        assert!(advance(&sample()).unwrap_err().contains("missing"));
        let v = serde_json::to_value(sample()).unwrap();
        for field in ["expected", "helper_results", "target", "draws"] {
            let mut v = v.clone();
            v[field] = serde_json::json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
    }
    #[test]
    fn selection_active_side_random_short_circuit() {
        let mut s = synthetic();
        s.seed = 100;
        s.cells.push(Cell {
            region: Region::P15,
            offset: 23,
            value: 0,
        });
        let o = select_target(&selection(&s, 0)).unwrap();
        assert_eq!(o.output.scene, s.scene);
        assert_eq!(o.draws.len(), 1);
        assert_ne!(o.output.seed, s.seed);
    }
    #[test]
    fn selection_missing_and_alias_guards() {
        assert!(select_target(&selection(&sample(), 0))
            .unwrap_err()
            .contains("missing"));
        for offset in [0xA51C, 0xA51D, 0xD7BA, 0xDE84, 0xCCEF + 25] {
            let mut s = synthetic();
            s.cells.push(Cell {
                region: Region::Ds,
                offset,
                value: 42,
            });
            assert!(select_target(&selection(&s, 0)).is_err());
        }
    }
    #[test]
    fn original_selection_roots() {
        let source = std::fs::read_to_string(format!(
            "{}/tests/fixtures/watched_selection/cases.jsonl",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let frames = std::fs::read_to_string(format!(
            "{}/tests/fixtures/watched_selection/frames.jsonl",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        assert_eq!(source.lines().count(), 64);
        assert_eq!(frames.lines().count(), 64);
        for (number, (line, frame_line)) in source.lines().zip(frames.lines()).enumerate() {
            let frame_row: serde_json::Value = serde_json::from_str(frame_line).unwrap();
            let row: serde_json::Value = serde_json::from_str(line).unwrap();
            let s: State = serde_json::from_value(row["input"].clone()).unwrap();
            let frame: SelectionState = serde_json::from_value(frame_row["input"].clone()).unwrap();
            assert!(frame.cells.is_empty());
            assert_eq!(frame.scene, s.scene);
            assert_eq!(frame.seed, s.seed);
            assert_eq!(row["case"], number);
            assert_eq!(frame_row["case"], number);
            for k in ["tick", "draw_start", "draw_end"] {
                assert_eq!(row[k], frame_row[k]);
            }
            for k in ["scene", "seed"] {
                assert_eq!(row["output"]["output"][k], frame_row["output"]["output"][k]);
            }
            assert_eq!(row["output"]["draws"], frame_row["output"]["draws"]);

            assert_eq!(
                serde_json::to_value(select_target(&selection(&s, frame.frame_byte5)).unwrap())
                    .unwrap(),
                row["output"]
            );
            let mut c = machine(&s).unwrap();
            let before = c.e.image.clone();
            run_selection(&mut c, s.scene.index, frame.frame_byte5).unwrap();
            let root = 0xCCEF + 25 * usize::from(s.scene.index);
            for (i, (a, b)) in before.iter().zip(&c.e.image).enumerate() {
                if ![
                    root + 8,
                    root + 9,
                    root + 12,
                    root + 20,
                    root + 21,
                    usize::from(crate::SEED),
                    usize::from(crate::SEED) + 1,
                ]
                .contains(&i)
                {
                    assert_eq!(a, b, "selection unexpected write {i:05X}");
                }
            }
        }
    }

    #[test]
    fn selection_formation_updates_offset_and_target() {
        let mut s = synthetic();
        s.seed = 4;
        s.cells.push(Cell {
            region: Region::P15,
            offset: 23,
            value: 0,
        });
        set(&mut s, Region::P8, 14, 0);
        let o = select_target(&selection(&s, 0)).unwrap();
        assert_eq!(o.output.scene.word12, 0x0500);
        // Original1F5E negates both signed target bytes for this orientation.
        assert_eq!(o.output.scene.word8, 0xF7FD);
        assert_eq!(o.draws.len(), 1);
    }
    #[test]
    fn selection_raw_frame_flag_controls_inactive_keeper_clear() {
        let mut s = synthetic();
        set(&mut s, Region::Ds, 0xD09F, 12);
        set(&mut s, Region::P8, 14, 0);
        s.scene.word20 = 7;
        s.cells.push(Cell {
            region: Region::Ds,
            offset: 0xD0B8,
            value: 0,
        });
        for flag in [0, 1, 255] {
            let o = select_target(&selection(&s, flag)).unwrap();
            assert_eq!(o.output.scene.word20, if flag == 0 { 7 } else { 0 });
            assert!(o.draws.is_empty());
        }
    }
    #[test]
    fn selection_frame_is_required_and_does_not_extend_adjustment_schema() {
        let v = serde_json::to_value(selection(&synthetic(), 0)).unwrap();
        assert!(serde_json::from_value::<State>(v.clone()).is_err());
        let mut missing = v.clone();
        missing.as_object_mut().unwrap().remove("frame_byte5");
        assert!(serde_json::from_value::<SelectionState>(missing).is_err());
        for bad in [
            serde_json::json!(-1),
            serde_json::json!(256),
            serde_json::json!(true),
        ] {
            let mut bad_value = v.clone();
            bad_value["frame_byte5"] = bad;
            assert!(serde_json::from_value::<SelectionState>(bad_value).is_err());
        }
        let mut helper = v;
        helper["expected_frame_result"] = serde_json::json!(1);
        assert!(serde_json::from_value::<SelectionState>(helper).is_err());
    }
}
