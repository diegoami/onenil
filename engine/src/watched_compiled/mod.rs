//! [confirmed] F39/c7637bd;compiled original35F8 numeric helper graph.
//! [designed] Typed root facts and temporary Pascal frames;no runtime decoding.
mod adjustment;
mod compiled;
use crate::{terminal::Scene, Draw, Engine};
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
                || (0xA338..=0xA339).contains(&offset)
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
#[cfg(test)]
mod tests {
    use super::*;
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
}
