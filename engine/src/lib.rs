//! First One-Nil 96 routine slice, not a whole match or emulator.
//!
//! [confirmed] RNG and D983 semantics: onenil-archaeology F02/F07/F09,
//! commit 7859d7d; T11/F11 provides the independently recorded routine cases.
//! [designed] This image represents only the DS state touched by this slice;
//! scratch registers, stack locals and asynchronous timer writes are excluded.

use serde::{Deserialize, Serialize};

const SEED: u16 = 0xA51C;
const RATINGS: u16 = 0xD674;
const GOALS: u16 = 0xD086;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Input {
    pub k: u16,
    pub seed: u16,
    pub ratings: [u16; 8],
    pub goals: [u8; 2],
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Draw {
    pub site: String,
    pub n: u16,
    pub seed_before: u16,
    pub seed_after: u16,
    pub value: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Output {
    pub seed: u16,
    pub goals: [u8; 2],
}

#[derive(Clone, Debug, Deserialize)]
pub struct Case {
    pub schema: String,
    pub week: String,
    pub call: usize,
    pub input: Input,
    pub output: Output,
    pub draws: Vec<Draw>,
}

#[derive(Debug, Serialize)]
pub struct Outcome {
    pub seed: u16,
    pub goals: [u8; 2],
    pub draws: Vec<Draw>,
}

pub struct Engine {
    image: Box<[u8; 65536]>,
}

impl Engine {
    pub fn new(input: &Input) -> Self {
        let mut engine = Self {
            image: Box::new([0; 65536]),
        };
        engine.put_word(SEED, input.seed);
        for (i, value) in input.ratings.iter().enumerate() {
            engine.put_word(RATINGS + 2 * i as u16, *value);
        }
        for (side, value) in input.goals.iter().enumerate() {
            engine.image[GOALS as usize + side] = *value;
        }
        engine
    }

    fn word(&self, offset: u16) -> u16 {
        let i = offset as usize;
        u16::from_le_bytes([self.image[i], self.image[i + 1]])
    }

    fn put_word(&mut self, offset: u16, value: u16) {
        let i = offset as usize;
        self.image[i..i + 2].copy_from_slice(&value.to_le_bytes());
    }

    pub fn quick_contests(&mut self, _k: u16) -> Outcome {
        // Red-test scaffold: implement only after the original fixture replay fails.
        todo!("D983 has not been implemented")
    }
}

/// [designed] Report the first divergence, rather than accepting a final score alone.
pub fn replay(case: &Case) -> Result<(), String> {
    let label = format!("{} call {}", case.week, case.call);
    if case.schema != "routine-case/1" {
        return Err(format!("{label}: unknown case schema {}", case.schema));
    }
    let got = Engine::new(&case.input).quick_contests(case.input.k);
    for (i, (actual, expected)) in got.draws.iter().zip(&case.draws).enumerate() {
        if actual != expected {
            return Err(format!(
                "{label}: draw {i}: expected {expected:?}, got {actual:?}"
            ));
        }
    }
    if got.draws.len() != case.draws.len() {
        return Err(format!(
            "{label}: draw count: expected {}, got {}",
            case.draws.len(),
            got.draws.len()
        ));
    }
    if got.seed != case.output.seed || got.goals != case.output.goals {
        return Err(format!(
            "{label}: output: expected {:?}, got seed={} goals={:?}",
            case.output, got.seed, got.goals
        ));
    }
    Ok(())
}

#[cfg(feature = "python")]
mod python {
    use super::*;
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    #[pyfunction]
    fn quick_contests(k: u16, seed: u16, ratings: Vec<u16>, goals: Vec<u8>) -> PyResult<String> {
        let ratings = ratings
            .try_into()
            .map_err(|_| PyValueError::new_err("eight rating words required"))?;
        let goals = goals
            .try_into()
            .map_err(|_| PyValueError::new_err("two goal bytes required"))?;
        let input = Input {
            k,
            seed,
            ratings,
            goals,
        };
        let out = Engine::new(&input).quick_contests(k);
        serde_json::to_string(&out).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[pymodule]
    fn onenil_engine(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add_function(wrap_pyfunction!(quick_contests, module)?)
    }
}
