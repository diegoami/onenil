//! First One-Nil 96 routine slice, not a whole match or emulator.
//!
//! [confirmed] RNG and D983 semantics: onenil-archaeology F02/F07/F09,
//! commit 7859d7d; T11/F11 at 78bdebc provides the recorded routine cases.
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

    fn draw(&mut self, n: u16, site: &str, draws: &mut Vec<Draw>) -> u16 {
        // [confirmed] F02: 24BC:0CDC, unsigned MUL and a wrapping low-word LCG.
        // The recorded seed input was read through the guest's DS:DE84 far pointer;
        // this slice stores that decoded word at its canonical DS:A51C location.
        let before = self.word(SEED);
        let after = before.wrapping_mul(0x7ABD).wrapping_add(0x1B0F);
        self.put_word(SEED, after);
        let value = (((after as u32 * n as u32) >> 16) + 1) as u16;
        draws.push(Draw {
            site: site.to_owned(),
            n,
            seed_before: before,
            seed_after: after,
            value,
        });
        value
    }

    pub fn quick_contests(&mut self, k: u16) -> Outcome {
        // [confirmed] F07 D983..DA3E: unsigned positive-chance loop; side 0 then 1.
        // Opponent/own rating-class pairs: midfield/midfield, defence/attack,
        // keeper/attack. These are return addresses of the six recorded call sites.
        let contests = [
            (2, 2, "0AB9:D9BD", "0AB9:D9CC"),
            (1, 3, "0AB9:D9E4", "0AB9:D9F3"),
            (0, 3, "0AB9:DA0B", "0AB9:DA1A"),
        ];
        let mut draws = Vec::new();
        for _ in 0..k {
            for side in 0..2u16 {
                let mut won = true;
                for &(opponent_class, own_class, opponent_site, own_site) in &contests {
                    let opponent_n = self.word(RATINGS + 4 * opponent_class + 2 * (side ^ 1));
                    let own_n = self.word(RATINGS + 4 * own_class + 2 * side);
                    let opponent = self.draw(opponent_n, opponent_site, &mut draws);
                    let own = self.draw(own_n, own_site, &mut draws);
                    // [confirmed] CMP/JLE, not an unsigned comparison; a tie loses.
                    if (own as i16) <= (opponent as i16) {
                        won = false;
                        break;
                    }
                }
                if won {
                    let goal = &mut self.image[(GOALS + side) as usize];
                    *goal = goal.wrapping_add(1);
                }
            }
        }
        Outcome {
            seed: self.word(SEED),
            goals: [self.image[GOALS as usize], self.image[GOALS as usize + 1]],
            draws,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(k: u16, seed: u16, ratings: [u16; 8], goals: [u8; 2]) -> Input {
        Input {
            k,
            seed,
            ratings,
            goals,
        }
    }

    #[test]
    fn zero_chances_preserve_the_entire_slice_image() {
        let mut engine = Engine::new(&input(0, 65535, [65535; 8], [255, 17]));
        let before = engine.image.clone();
        let result = engine.quick_contests(0);
        assert_eq!(engine.image, before);
        assert!(result.draws.is_empty());
        assert_eq!(result.seed, 65535);
        assert_eq!(result.goals, [255, 17]);
    }

    #[test]
    fn zero_arguments_still_draw_and_ties_do_not_score() {
        let mut engine = Engine::new(&input(1, 0, [0; 8], [3, 4]));
        let result = engine.quick_contests(1);
        assert_eq!(result.draws.len(), 4);
        assert_eq!(result.draws[0].seed_after, 0x1B0F);
        assert!(result.draws.iter().all(|d| d.n == 0 && d.value == 1));
        assert_eq!(result.goals, [3, 4]);
        assert_ne!(result.seed, 0);
    }

    #[test]
    fn comparison_is_signed_even_when_rating_and_draw_are_large() {
        let mut engine = Engine::new(&input(1, 1, [0, 0, 0, 0, 1, 65535, 1, 0], [0, 0]));
        let result = engine.quick_contests(1);
        assert_eq!(result.draws[0].value, 0x95CC);
        assert_eq!(result.draws[1].value, 1);
        // Signed 1 > -27188 advances side 0 into its second contest.
        assert_eq!(result.draws[2].site, "0AB9:D9E4");
        assert_eq!(result.draws[2].n, 0);
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
        module.add_function(wrap_pyfunction!(quick_contests, module)?)?;
        module.add("__version__", env!("CARGO_PKG_VERSION"))?;
        module.add("__debug_assertions__", cfg!(debug_assertions))
    }
}
