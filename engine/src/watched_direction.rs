//! [confirmed] F39,c7637bd; original 0AB9:70D5 at static d9e20a4.
//! [designed] Root direction/retry composition; not a complete watched tick.
use crate::{terminal::Scene, watched_movement::Cell, Draw, Engine};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub scene: Scene,
    pub active: u8,
    pub actor: u8,
    pub secondary: u8,
    pub role: u8,
    pub blocking_mask: [u8; 32],
    pub cells: Vec<Cell>,
    pub seed: u16,
    pub flag: u8,
    pub ball_y: u8,
    pub phase: u8,
    pub remaining: u16,
    pub direction_signs: [u8; 2],
    pub line_bytes: [u8; 2],
}
#[derive(Debug, Serialize)]
pub struct Attempt {
    pub direction: u8,
    pub coordinate: u16,
    pub allowed: u8,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub draws: Vec<Draw>,
    pub attempts: Vec<Attempt>,
}
fn machine(s: &State) -> Result<Engine, String> {
    let base = crate::watched_movement::State {
        scene: s.scene.clone(),
        active: s.active,
        actor: s.actor,
        secondary: s.secondary,
        role: s.role,
        blocking_mask: s.blocking_mask,
        cell: None,
        seed: s.seed,
    };
    let mut e = crate::watched_movement::machine(&base)?;
    for c in &s.cells {
        let i = crate::watched_movement::grid_index(c.coordinate)
            .ok_or("invalid direction occupancy witness coordinate")?;
        if e.image[crate::layout::PITCH_PRESENT + i] != 0 {
            return Err("duplicate direction occupancy witness".into());
        }
        e.image[crate::layout::PITCH + 2 * i] = c.occupant;
        e.image[crate::layout::PITCH + 2 * i + 1] = c.tag;
        e.image[crate::layout::PITCH_PRESENT + i] = 1;
    }
    e.image[0xCF53] = s.flag;
    e.image[0xCF47] = s.ball_y;
    e.image[0xD0AC] = s.phase;
    e.put_word(0xD0AE, s.remaining);
    e.image[0xA338..0xA33A].copy_from_slice(&s.direction_signs);
    e.image[0xCF2F] = s.line_bytes[0];
    e.image[0xCF36] = s.line_bytes[1];
    Ok(e)
}
pub(crate) fn advance_image(
    e: &mut Engine,
    index: u8,
) -> Result<(Vec<Draw>, Vec<Attempt>), String> {
    if !(1..=22).contains(&index) {
        return Err("unsupported watched direction scene".into());
    }
    let a = 0xCCEF + 25 * usize::from(index);
    let row = Scene::read(e, index);
    let (sign, side) = crate::terminal::direction_image(e, &row);
    let signed = |v: u8| i16::from(v as i8);
    let target_y = e.image[a + 9];
    let threshold = e.image[0xCF2F + 7 * usize::from(side == 0)];
    // [confirmed] 70D9..71DA: signed products,sign tests and absolute values;
    // only byte9 changes. The threshold's football meaning is not established.
    let ahead = e.image[0xCF53] != 0
        && signed(e.image[0xCF47]) * signed(sign) > signed(e.image[a + 1]) * signed(sign);
    if !ahead
        && row.byte7 != e.image[0xD09F]
        && row.byte7 != e.image[0xD0A0]
        && (sign as i8 > 0) == (target_y as i8 > 0)
        && signed(target_y).abs() >= signed(threshold).abs()
        && matches!(row.byte19, 5 | 7 | 8 | 11)
    {
        e.image[a + 9] = (signed(threshold) - signed(sign)) as u8;
    }
    let mut draws = Vec::new();
    let mut attempts = Vec::new();
    let dx = e.image[a + 8].wrapping_sub(e.image[a]) as i8;
    let dy = e.image[a + 9].wrapping_sub(e.image[a + 1]) as i8;
    e.image[a + 16] = if e.word(a as u16) == e.word((a + 8) as u16) {
        5
    } else {
        let x_first = e.draw(2, "0AB9:7217", &mut draws) == 1;
        if (x_first && dx != 0) || (!x_first && dy == 0) {
            1 + u8::from(dx > 0)
        } else {
            3 + u8::from(dy > 0)
        }
    };
    loop {
        let allowed = crate::watched_movement::advance_image(e, index)?;
        attempts.push(Attempt {
            direction: e.image[a + 16],
            coordinate: e.word(a as u16),
            allowed,
        });
        if allowed != 0 {
            break;
        }
        e.image[a + 16] = e.draw(5, "0AB9:72AA", &mut draws) as u8;
    }
    Ok((draws, attempts))
}
pub fn advance(s: &State) -> Result<Outcome, String> {
    let mut e = machine(s)?;
    let (draws, attempts) = advance_image(&mut e, s.scene.index)?;
    let mut output = s.clone();
    output.scene = Scene::read(&e, s.scene.index);
    output.seed = e.word(crate::SEED);
    Ok(Outcome {
        output,
        draws,
        attempts,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn packed(x: i8, y: i8) -> u16 {
        u16::from_le_bytes([x as u8, y as u8])
    }
    fn sample() -> State {
        let c: serde_json::Value = serde_json::from_str(
            include_str!("../tests/fixtures/watched_direction/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let mut s: State = serde_json::from_value(c["input"].clone()).unwrap();
        s.scene.index = 1;
        s.scene.byte7 = 1;
        s.scene.word14 = 1;
        s.scene.word0 = 0;
        s.scene.word2 = 0;
        s.scene.word8 = packed(3, 4);
        s.scene.byte19 = 5;
        s.seed = 0;
        s.actor = 2;
        s.secondary = 3;
        s.active = 0;
        s.direction_signs = [1, 1];
        s.line_bytes = [10, 246];
        s.flag = 0;
        s.cells.clear();
        s
    }
    #[test]
    fn signed_clamp_guards_and_only_target_high_byte() {
        for (target, sign, line, expected) in [
            (20, 1, 10, 9),
            (-20, -1, -10, -9),
            (-128, -1, -128, -127),
            (10, 1, 10, 9),
            (9, 1, 10, 9),
        ] {
            let mut s = sample();
            s.scene.word8 = packed(3, target);
            s.direction_signs = [sign as u8; 2];
            s.line_bytes = [line as u8; 2];
            assert_eq!(advance(&s).unwrap().output.scene.word8, packed(3, expected));
        }
        for mode in 0..7 {
            let mut s = sample();
            s.scene.word8 = packed(3, 20);
            match mode {
                0 => s.actor = 1,
                1 => s.secondary = 1,
                2 => s.direction_signs = [255; 2],
                3 => s.line_bytes = [21; 2],
                4 => s.scene.byte19 = 6,
                5 => {
                    s.flag = 1;
                    s.ball_y = 1
                }
                _ => {
                    s.flag = 1;
                    s.ball_y = 127
                }
            }
            assert_eq!(advance(&s).unwrap().output.scene.word8, packed(3, 20));
        }
        for action in [5, 7, 8, 11] {
            let mut s = sample();
            s.scene.word8 = packed(3, 20);
            s.scene.byte19 = action;
            s.flag = 1;
            s.ball_y = 255;
            assert_eq!(advance(&s).unwrap().output.scene.word8, packed(3, 9));
        }
        let mut s = sample();
        s.scene.byte7 = 12;
        s.direction_signs = [255, 1];
        s.phase = 255;
        s.remaining = 0;
        s.scene.word8 = packed(3, 20);
        // signed phase -1 makes flip true even with remaining zero; slot12 chooses sign1.
        assert_eq!(
            advance(&s).unwrap().output.scene.word8,
            packed(3, 245u8 as i8)
        );
    }
    #[test]
    fn coin_axes_wrapped_byte_deltas_and_equal_target() {
        for (seed, target, direction) in [
            (0, packed(3, 4), 2),
            (1, packed(3, 4), 4),
            (0, packed(0, -4), 3),
            (1, packed(-3, 0), 1),
            (0, packed(-128, 0), 1),
        ] {
            let mut s = sample();
            s.seed = seed;
            s.scene.word8 = target;
            s.scene.byte19 = 0;
            let o = advance(&s).unwrap();
            assert_eq!(o.attempts[0].direction, direction);
            assert_eq!(o.draws.len(), 1);
            assert_eq!(o.draws[0].site, "0AB9:7217");
            assert_eq!(o.draws[0].n, 2);
        }
        let mut s = sample();
        s.scene.word0 = packed(127, 0);
        s.scene.word2 = 0;
        s.scene.word8 = packed(-128, 0);
        s.scene.byte19 = 0;
        assert_eq!(advance(&s).unwrap().attempts[0].direction, 2);
        s.scene.word0 = s.scene.word8;
        s.scene.word2 = packed(127, -128);
        let o = advance(&s).unwrap();
        assert_eq!(o.attempts[0].direction, 5);
        assert_eq!(o.output.scene.word0, s.scene.word2);
        assert!(o.draws.is_empty());
    }
    #[test]
    fn blocked_retries_share_image_and_reached_witnesses_are_required() {
        let mut s = sample();
        s.active = 1;
        s.role = 1;
        s.blocking_mask = [255; 32];
        s.scene.byte19 = 0;
        s.cells = [packed(-1, 0), packed(1, 0), packed(0, -1), packed(0, 1)]
            .into_iter()
            .map(|coordinate| Cell {
                coordinate,
                occupant: 255,
                tag: 0,
            })
            .collect();
        let o = advance(&s).unwrap();
        assert!(o.attempts.len() > 2);
        assert_eq!(o.attempts.last().unwrap().direction, 5);
        assert_eq!(o.output.scene.word0, 0);
        assert!(o.attempts[..o.attempts.len() - 1]
            .iter()
            .all(|a| a.allowed == 0));
        assert_eq!(o.draws.len(), o.attempts.len());
        assert!(o.draws[1..]
            .iter()
            .all(|d| d.n == 5 && d.site == "0AB9:72AA"));
        s.cells.clear();
        assert!(advance(&s).unwrap_err().contains("witness"));
    }
    #[test]
    fn strict_input_and_write_ownership() {
        let s = sample();
        let source = serde_json::to_value(&s).unwrap();
        for field in ["expected", "attempts", "draws", "candidate", "direction"] {
            let mut v = source.clone();
            v[field] = serde_json::json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
        for mode in 0..4 {
            let mut invalid = s.clone();
            match mode {
                0 => invalid.scene.index = 0,
                1 => invalid.actor = 23,
                2 => invalid.scene.word14 = 3921,
                _ => invalid.scene.byte7 = 23,
            };
            assert!(advance(&invalid).is_err());
        }
        let mut invalid = s.clone();
        invalid.cells = vec![
            Cell {
                coordinate: 0,
                occupant: 1,
                tag: 0
            };
            2
        ];
        assert!(advance(&invalid).is_err());
        invalid.cells = vec![Cell {
            coordinate: packed(9, 0),
            occupant: 1,
            tag: 0,
        }];
        assert!(advance(&invalid).is_err());
        let mut e = machine(&s).unwrap();
        e.image[0x68000] = 42;
        let before = e.image.clone();
        let a = 0xCCEF + 25 * usize::from(s.scene.index);
        advance_image(&mut e, s.scene.index).unwrap();
        for (i, (old, new)) in before.iter().zip(&e.image).enumerate() {
            if ![
                a,
                a + 1,
                a + 9,
                a + 16,
                usize::from(crate::SEED),
                usize::from(crate::SEED) + 1,
            ]
            .contains(&i)
            {
                assert_eq!(old, new, "unexpected write at {i:05X}");
            }
        }
    }
    #[test]
    fn original_root_direction_and_retry_calls() {
        for name in ["cases.jsonl", "active-cases.jsonl"] {
            let source = std::fs::read_to_string(format!(
                "{}/tests/fixtures/watched_direction/{name}",
                env!("CARGO_MANIFEST_DIR")
            ))
            .unwrap();
            for line in source.lines() {
                let c: serde_json::Value = serde_json::from_str(line).unwrap();
                let input: State = serde_json::from_value(c["input"].clone()).unwrap();
                assert_eq!(
                    serde_json::to_value(advance(&input).unwrap()).unwrap(),
                    c["output"]
                );
            }
        }
    }
}
