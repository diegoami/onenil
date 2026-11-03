//! [confirmed] F39,c7637bd;7423/1998 flight and transport at d9e20a4.
//! [designed] In-bounds gameplay/logical pitch transport;outside restarts deferred.
use crate::{watched_movement::Cell, Draw, Engine};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Ball {
    pub word0: u16,
    pub word2: u16,
    pub word7: u16,
    pub word9: u16,
    pub word11: u16,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub byte13: u8,
    pub byte14: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub ball: Ball,
    pub sentinel: u16,
    pub glyphs: [u8; 2],
    pub seed: u16,
    pub main: Vec<Cell>,
    pub background: Vec<Cell>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub within: u8,
    pub metrics: Vec<u16>,
    pub draws: Vec<Draw>,
}
const BALL: usize = 0xCF46;
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn read_ball(e: &Engine) -> Ball {
    Ball {
        word0: word(e, BALL),
        word2: word(e, BALL + 2),
        word7: word(e, BALL + 7),
        word9: word(e, BALL + 9),
        word11: word(e, BALL + 11),
        byte4: e.image[BALL + 4],
        byte5: e.image[BALL + 5],
        byte6: e.image[BALL + 6],
        byte13: e.image[BALL + 13],
        byte14: e.image[BALL + 14],
    }
}
fn bank(background: bool) -> (usize, usize) {
    if background {
        (
            crate::layout::PITCH_BACKGROUND,
            crate::layout::PITCH_BACKGROUND_PRESENT,
        )
    } else {
        (crate::layout::PITCH, crate::layout::PITCH_PRESENT)
    }
}
fn machine(s: &State) -> Result<Engine, String> {
    let mut e = crate::role_scores::image(Default::default());
    for (offset, v) in [
        (0, s.ball.word0),
        (2, s.ball.word2),
        (7, s.ball.word7),
        (9, s.ball.word9),
        (11, s.ball.word11),
    ] {
        put(&mut e, BALL + offset, v)
    }
    for (offset, v) in [
        (4, s.ball.byte4),
        (5, s.ball.byte5),
        (6, s.ball.byte6),
        (13, s.ball.byte13),
        (14, s.ball.byte14),
    ] {
        e.image[BALL + offset] = v
    }
    put(&mut e, 0xD2E, s.sentinel);
    e.image[0xD70..0xD72].copy_from_slice(&s.glyphs);
    put(&mut e, crate::SEED as usize, s.seed);
    for (background, cells) in [(false, &s.main), (true, &s.background)] {
        let (base, present) = bank(background);
        for c in cells {
            let i = crate::watched_movement::grid_index(c.coordinate)
                .ok_or("unsupported ball occupancy witness coordinate")?;
            if e.image[present + i] != 0 {
                return Err("duplicate ball occupancy witness".into());
            }
            e.image[base + 2 * i] = c.occupant;
            e.image[base + 2 * i + 1] = c.tag;
            e.image[present + i] = 1;
        }
    }
    Ok(e)
}
fn cell(e: &Engine, background: bool, coordinate: u16) -> Result<u16, String> {
    let i = crate::watched_movement::grid_index(coordinate)
        .ok_or("unsupported reached ball occupancy coordinate")?;
    let (base, present) = bank(background);
    if e.image[present + i] == 0 {
        return Err("missing reached ball occupancy witness".into());
    }
    Ok(word(e, base + 2 * i))
}
fn store(e: &mut Engine, coordinate: u16, value: u16) -> Result<(), String> {
    let i = crate::watched_movement::grid_index(coordinate)
        .ok_or("unsupported reached ball occupancy coordinate")?;
    if e.image[crate::layout::PITCH_PRESENT + i] == 0 {
        return Err("missing reached ball occupancy witness".into());
    }
    put(e, crate::layout::PITCH + 2 * i, value);
    Ok(())
}
fn inside(coordinate: u16, x: i16, y: i16) -> bool {
    let [a, b] = coordinate.to_le_bytes();
    i16::from(a as i8).abs() <= x && i16::from(b as i8).abs() <= y
}
fn metric(e: &Engine, metrics: &mut Vec<u16>) -> u16 {
    // [confirmed] 0B3F sign-extends a wrapped sum,then returns only the product's AX.
    let sum = (word(e, BALL + 7) as i16)
        .unsigned_abs()
        .wrapping_add((word(e, BALL + 9) as i16).unsigned_abs());
    let result = word(e, BALL + 11).wrapping_mul(sum);
    metrics.push(result);
    result
}
fn transport(e: &mut Engine, metrics: &mut Vec<u16>) -> Result<(), String> {
    let old = word(e, BALL + 2);
    let current = word(e, BALL);
    let sentinel = word(e, 0xD2E);
    if old != sentinel && old != current && matches!(cell(e, false, old)? as u8, 7 | 9) {
        let value = cell(e, true, old)?;
        store(e, old, value)?
    }
    put(e, BALL + 2, current);
    let raised = e.image[BALL + 14] != 0 && metric(e, metrics) > 2;
    e.image[BALL + 4] = e.image[0xD70 + usize::from(raised)];
    if current != sentinel {
        let background = cell(e, true, current)?;
        let foreground = cell(e, false, current)?;
        if foreground == background {
            store(e, current, word(e, BALL + 4))?
        }
    }
    Ok(())
}
pub(crate) fn advance_image(e: &mut Engine) -> Result<Vec<u16>, String> {
    let candidate = u16::from_le_bytes([
        e.image[BALL].wrapping_add(e.image[BALL + 7]),
        e.image[BALL + 1].wrapping_add(e.image[BALL + 9]),
    ]);
    let position = if inside(candidate, 9, 39) {
        candidate
    } else {
        word(e, BALL)
    };
    // [designed] Reject the unported restart graph before publishing any output.
    if !inside(position, 8, 37) {
        return Err("outside-pitch restart is not yet supported".into());
    }
    let distance = word(e, BALL + 11);
    if distance > 0 {
        put(e, BALL + 11, distance - 1)
    }
    let mut metrics = Vec::new();
    e.image[BALL + 14] = u8::from(e.image[BALL + 14] != 0 && metric(e, &mut metrics) > 2);
    put(e, BALL, position);
    transport(e, &mut metrics)?;
    Ok(metrics)
}
fn snapshot(e: &Engine, s: &State) -> Result<State, String> {
    let mut out = s.clone();
    out.ball = read_ball(e);
    out.seed = word(e, crate::SEED as usize);
    for (background, cells) in [(false, &mut out.main), (true, &mut out.background)] {
        for c in cells {
            let value = cell(e, background, c.coordinate)?;
            [c.occupant, c.tag] = value.to_le_bytes();
        }
    }
    Ok(out)
}
pub fn advance(s: &State) -> Result<Outcome, String> {
    let mut e = machine(s)?;
    let metrics = advance_image(&mut e)?;
    Ok(Outcome {
        output: snapshot(&e, s)?,
        within: 1,
        metrics,
        draws: vec![],
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn packed(x: i8, y: i8) -> u16 {
        u16::from_le_bytes([x as u8, y as u8])
    }
    fn cell(coordinate: u16, occupant: u8, tag: u8) -> Cell {
        Cell {
            coordinate,
            occupant,
            tag,
        }
    }
    fn sample() -> State {
        State {
            ball: Ball {
                word0: 0,
                word2: 0,
                word7: 1,
                word9: 0,
                word11: 4,
                byte4: 99,
                byte5: 2,
                byte6: 33,
                byte13: 1,
                byte14: 0,
            },
            sentinel: 0x7F7F,
            glyphs: [7, 9],
            seed: 0xBEEF,
            main: vec![cell(0, 7, 2), cell(1, 0, 0)],
            background: vec![cell(0, 0, 0), cell(1, 0, 0)],
        }
    }
    fn ownership(s: &State) {
        let mut e = machine(s).unwrap();
        e.image[0x68000] = 42;
        let before = e.image.clone();
        advance_image(&mut e).unwrap();
        let mut owned = vec![
            0xCF46, 0xCF47, 0xCF48, 0xCF49, 0xCF4A, 0xCF51, 0xCF52, 0xCF54,
        ];
        for coordinate in [s.ball.word2, e.word(0xCF46)] {
            if coordinate != s.sentinel {
                if let Some(i) = crate::watched_movement::grid_index(coordinate) {
                    owned.extend([
                        crate::layout::PITCH + 2 * i,
                        crate::layout::PITCH + 2 * i + 1,
                    ]);
                }
            }
        }
        for (i, (old, new)) in before.iter().zip(&e.image).enumerate() {
            if !owned.contains(&i) {
                assert_eq!(old, new, "unexpected write {i:05X}");
            }
        }
    }
    #[test]
    fn original_ball_roots() {
        let source = std::fs::read_to_string(format!(
            "{}/tests/fixtures/watched_ball/cases.jsonl",
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
            ownership(&s);
        }
    }
    #[test]
    fn original_restore_stamp_and_owned_packet_fields() {
        let s = sample();
        let o = advance(&s).unwrap();
        assert_eq!(o.output.ball.word0, 1);
        assert_eq!(o.output.ball.word2, 1);
        assert_eq!(o.output.ball.word11, 3);
        assert_eq!(o.output.ball.byte4, 7);
        assert_eq!(o.output.main, vec![cell(0, 0, 0), cell(1, 7, 2)]);
        assert_eq!(o.output.background, s.background);
        assert_eq!(o.output.seed, s.seed);
        assert!(o.metrics.is_empty());
        assert!(o.draws.is_empty());
        ownership(&s);
    }
    #[test]
    fn restore_guards_sentinel_same_coordinate_and_codes() {
        for occupant in [0, 7, 9, 255] {
            let mut s = sample();
            s.main[0].occupant = occupant;
            let o = advance(&s).unwrap();
            assert_eq!(
                o.output.main[0].occupant,
                if [7, 9].contains(&occupant) {
                    0
                } else {
                    occupant
                }
            );
        }
        let mut s = sample();
        s.ball.word2 = s.sentinel;
        s.main.remove(0);
        s.background.remove(0);
        assert!(advance(&s).is_ok());
        s = sample();
        s.ball.word7 = 0;
        s.main = vec![cell(0, 7, 2)];
        s.background = vec![cell(0, 99, 3)];
        s.glyphs[0] = 8;
        let o = advance(&s).unwrap();
        assert_eq!(o.output.main[0], cell(0, 7, 2));
        s = sample();
        s.main[1] = cell(1, 12, 5);
        let o = advance(&s).unwrap();
        assert_eq!(o.output.main[1], cell(1, 12, 5));
    }
    #[test]
    fn distance_air_metrics_overflow_and_second_helper_call() {
        for (remaining, air, expected, metrics, glyph) in [
            (0, 255, 0, vec![0], 7),
            (3, 1, 0, vec![2], 7),
            (4, 1, 1, vec![3, 3], 9),
            (4, 0, 0, vec![], 7),
        ] {
            let mut s = sample();
            s.ball.word11 = remaining;
            s.ball.byte14 = air;
            let o = advance(&s).unwrap();
            assert_eq!(o.output.ball.word11, remaining.saturating_sub(1));
            assert_eq!(o.output.ball.byte14, expected);
            assert_eq!(o.metrics, metrics);
            assert_eq!(o.output.ball.byte4, glyph);
        }
        for (dx, dy, remaining, metric) in [
            (32768, 32768, 65535, 0),
            (65535, 65535, 32769, 0),
            (32767, 1, 3, 0),
            (65535, 1, 3, 4),
        ] {
            let mut s = sample();
            s.ball.word7 = dx;
            s.ball.word9 = dy;
            s.ball.word11 = remaining;
            s.ball.byte14 = 1;
            // Block huge candidate moves in081B;other small moves get indexed witnesses.
            s.ball.word0 = 0;
            s.ball.word2 = s.sentinel;
            let coordinate = if dx == 65535 && dy == 65535 {
                packed(-1, -1)
            } else if (dx == 65535 || dx == 32767) && dy == 1 {
                packed(-1, 1)
            } else {
                0
            };
            s.main = vec![cell(coordinate, 0, 0)];
            s.background = s.main.clone();
            let o = advance(&s).unwrap();
            assert_eq!(o.metrics[0], metric);
        }
    }
    #[test]
    fn independent_wrapping_extended_move_guard_and_playing_bounds() {
        let mut s = sample();
        s.ball.word0 = packed(127, 0);
        s.ball.word2 = s.sentinel;
        s.ball.word7 = 129; // lowbyte +129 wraps to0.
        s.main = vec![cell(0, 0, 0)];
        s.background = s.main.clone();
        assert_eq!(advance(&s).unwrap().output.ball.word0, 0);
        for (dx, dy) in [(10, 0), (0, 40), (128, 0)] {
            let mut s = sample();
            s.ball.word7 = dx;
            s.ball.word9 = dy;
            s.ball.word2 = s.sentinel;
            s.main = vec![cell(0, 0, 0)];
            s.background = s.main.clone();
            assert_eq!(advance(&s).unwrap().output.ball.word0, 0);
        }
        for target in [packed(9, 0), packed(-9, 0), packed(0, 38), packed(0, -39)] {
            let mut s = sample();
            let [x, y] = target.to_le_bytes();
            s.ball.word7 = u16::from(x);
            s.ball.word9 = u16::from(y);
            assert!(advance(&s).unwrap_err().contains("restart"));
        }
    }
    #[test]
    fn strict_input_missing_reached_cells_and_aliases() {
        let mut s = sample();
        s.main.clear();
        assert!(advance(&s).unwrap_err().contains("witness"));
        s = sample();
        s.background.clear();
        assert!(advance(&s).unwrap_err().contains("witness"));
        s = sample();
        s.main.push(s.main[0].clone());
        assert!(advance(&s).is_err());
        s = sample();
        s.main[0].coordinate = packed(9, 0);
        assert!(advance(&s).is_err());
        let v = serde_json::to_value(sample()).unwrap();
        for key in [
            "expected",
            "metric",
            "candidate",
            "within",
            "helper_results",
        ] {
            let mut v = v.clone();
            v[key] = serde_json::json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
        ownership(&sample());
    }
}
