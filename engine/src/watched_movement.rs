//! [confirmed] ArchaeologyF39 at c7637bd;6F98/1998:084B at d9e20a4.
//! [designed] Standalone original movement/legality kernel,not a watched match.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub coordinate: u16,
    pub occupant: u8,
    pub tag: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub scene: Scene,
    pub active: u8,
    pub actor: u8,
    pub secondary: u8,
    pub role: u8,
    pub blocking_mask: [u8; 32],
    pub cell: Option<Cell>,
    pub seed: u16,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub allowed: u8,
    pub draws: Vec<Draw>,
}
use crate::{layout, role_scores, Engine};
fn scene(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
}
pub(crate) fn grid_index(coordinate: u16) -> Option<usize> {
    let [x, y] = coordinate.to_le_bytes();
    let x = i16::from(x as i8);
    let y = i16::from(y as i8);
    if x.abs() > 8 || y.abs() > 37 {
        None
    } else {
        Some((x + 8) as usize * 75 + (y + 37) as usize)
    }
}
pub(crate) fn machine(s: &State) -> Result<Engine, String> {
    if !(1..=22).contains(&s.scene.index)
        || s.scene.byte7 > 22
        || s.scene.word14 > 3920
        || s.actor > 22
        || s.secondary > 22
    {
        return Err("unsupported watched movement identity".into());
    }
    let mut e = role_scores::image(Default::default());
    let a = scene(s.scene.index);
    e.image[a..a + 25].copy_from_slice(&s.scene.bytes());
    e.image[0xD0B8] = s.active;
    e.image[0xD09F] = s.actor;
    e.image[0xD0A0] = s.secondary;
    e.image[layout::P8 + 8 * usize::from(s.scene.word14) + 6] = s.role;
    e.image[0xD36..0xD56].copy_from_slice(&s.blocking_mask);
    e.put_word(crate::SEED, s.seed);
    if let Some(c) = &s.cell {
        let index =
            grid_index(c.coordinate).ok_or("invalid watched occupancy witness coordinate")?;
        e.image[layout::PITCH + 2 * index] = c.occupant;
        e.image[layout::PITCH + 2 * index + 1] = c.tag;
        e.image[layout::PITCH_PRESENT + index] = 1;
    }
    Ok(e)
}
pub(crate) fn advance_image(e: &mut Engine, index: u8) -> Result<u8, String> {
    if !(1..=22).contains(&index) {
        return Err("unsupported watched movement scene".into());
    }
    let a = scene(index);
    // [confirmed] 6FA7..6FF7 resets from word2,then changes just one byte.
    let previous = e.word((a + 2) as u16);
    e.put_word(a as u16, previous);
    let direction = e.image[a + 16];
    match direction {
        1 => e.image[a] = e.image[a].wrapping_sub(1),
        2 => e.image[a] = e.image[a].wrapping_add(1),
        3 => e.image[a + 1] = e.image[a + 1].wrapping_sub(1),
        4 => e.image[a + 1] = e.image[a + 1].wrapping_add(1),
        _ => {}
    }
    // Direction5 returns true even outside the usual pitch bounds.
    if direction == 5 {
        return Ok(1);
    }
    let point = e.word(a as u16);
    let Some(cell) = grid_index(point) else {
        return Ok(0);
    };
    let slot = e.image[a + 7];
    let id = e.word((a + 14) as u16);
    if e.image[0xD0B8] == 0
        || slot == e.image[0xD0A0]
        || slot == e.image[0xD09F]
        || e.image[layout::P8 + 8 * usize::from(id) + 6] == 0
    {
        return Ok(1);
    }
    // [designed] Missing raw indexed input diagnoses;it is never an empty cell.
    if e.image[layout::PITCH_PRESENT + cell] == 0 {
        return Err("missing reached watched occupancy witness".into());
    }
    let at = layout::PITCH + 2 * cell;
    if e.image[at + 1] == 5 {
        return Ok(1);
    }
    // [confirmed] Runtime0E1F(code,AH=1,DX=32) selects bit(code%8)
    // of byte(code/8),for the full byte domain0..255.
    let occupant = e.image[at];
    let mask = e.image[0xD36 + usize::from(occupant >> 3)];
    Ok(u8::from(mask & (1u8 << (occupant & 7)) == 0))
}
pub fn advance(s: &State) -> Result<Outcome, String> {
    let mut e = machine(s)?;
    let allowed = advance_image(&mut e, s.scene.index)?;
    let mut output = s.clone();
    output.scene = Scene::read(&e, s.scene.index);
    output.seed = e.word(crate::SEED);
    Ok(Outcome {
        output,
        allowed,
        draws: vec![],
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> State {
        let c: serde_json::Value = serde_json::from_str(
            include_str!("../tests/fixtures/watched_movement/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let mut s: State = serde_json::from_value(c["input"].clone()).unwrap();
        s.scene.word0 = 0xBEEF;
        s.scene.word2 = 0;
        s.scene.byte16 = 2;
        s.scene.byte7 = 1;
        s.scene.word14 = 1;
        s.active = 1;
        s.actor = 4;
        s.secondary = 5;
        s.role = 1;
        s.blocking_mask = [0; 32];
        s.seed = 0xBEEF;
        s.cell = Some(Cell {
            coordinate: 1,
            occupant: 7,
            tag: 0,
        });
        s
    }
    fn packed(x: i8, y: i8) -> u16 {
        u16::from_le_bytes([x as u8, y as u8])
    }
    #[test]
    fn original_watched_movement_calls() {
        for source in [
            include_str!("../tests/fixtures/watched_movement/cases.jsonl"),
            include_str!("../tests/fixtures/watched_movement/active-cases.jsonl"),
        ] {
            for line in source.lines() {
                let value: serde_json::Value = serde_json::from_str(line).unwrap();
                let input: State = serde_json::from_value(value["input"].clone()).unwrap();
                let out = advance(&input).unwrap();
                assert_eq!(serde_json::to_value(out).unwrap(), value["output"]);
            }
        }
    }
    #[test]
    fn directions_and_signed_byte_wrapping_use_old_position_not_target_or_input_candidate() {
        for (direction, point) in [
            (1, packed(-1, 0)),
            (2, packed(1, 0)),
            (3, packed(0, -1)),
            (4, packed(0, 1)),
            (5, 0),
            (0, 0),
            (255, 0),
        ] {
            let mut s = sample();
            s.scene.byte16 = direction;
            s.scene.word8 = 0x7F7F;
            s.cell = Some(Cell {
                coordinate: point,
                occupant: 0,
                tag: 0,
            });
            let o = advance(&s).unwrap();
            assert_eq!(o.output.scene.word0, point);
            assert_eq!(o.allowed, 1);
        }
        for (x, y, direction, point) in [
            (127, 0, 2, packed(-128, 0)),
            (-128, 0, 1, packed(127, 0)),
            (0, 127, 4, packed(0, -128)),
            (0, -128, 3, packed(0, 127)),
        ] {
            let mut s = sample();
            s.scene.word2 = packed(x, y);
            s.scene.byte16 = direction;
            s.cell = None;
            let o = advance(&s).unwrap();
            assert_eq!(o.output.scene.word0, point);
            assert_eq!(o.allowed, 0);
        }
    }
    #[test]
    fn bounds_endpoints_and_stay_precede_active_actor_and_role_guards() {
        for (x, y, direction, point) in [
            (7, 0, 2, packed(8, 0)),
            (-7, 0, 1, packed(-8, 0)),
            (0, 36, 4, packed(0, 37)),
            (0, -36, 3, packed(0, -37)),
        ] {
            let mut s = sample();
            s.scene.word2 = packed(x, y);
            s.scene.byte16 = direction;
            s.active = 0;
            s.cell = None;
            let o = advance(&s).unwrap();
            assert_eq!(o.allowed, 1);
            assert_eq!(o.output.scene.word0, point);
        }
        let mut s = sample();
        s.scene.word2 = packed(8, 0);
        s.active = 0;
        s.role = 0;
        s.actor = 1;
        s.cell = None;
        assert_eq!(advance(&s).unwrap().allowed, 0);
        s.scene.word2 = packed(-128, 127);
        s.scene.byte16 = 5;
        let o = advance(&s).unwrap();
        assert_eq!(o.allowed, 1);
        assert_eq!(o.output.scene.word0, packed(-128, 127));
    }
    #[test]
    fn original_guard_order_avoids_unreached_pitch_reads() {
        for mode in 0..4 {
            let mut s = sample();
            s.cell = None;
            match mode {
                0 => s.active = 0,
                1 => s.actor = 1,
                2 => s.secondary = 1,
                _ => s.role = 0,
            }
            assert_eq!(advance(&s).unwrap().allowed, 1);
        }
        assert!(advance(&{
            let mut s = sample();
            s.cell = None;
            s
        })
        .unwrap_err()
        .contains("witness"));
    }
    #[test]
    fn actual_tag_five_and_all_occupancy_mask_byte_edges() {
        for occupant in [0u8, 7, 8, 127, 128, 248, 255] {
            let mut s = sample();
            s.cell.as_mut().unwrap().occupant = occupant;
            s.blocking_mask[usize::from(occupant / 8)] = 1 << (occupant % 8);
            assert_eq!(advance(&s).unwrap().allowed, 0);
            s.cell.as_mut().unwrap().tag = 5;
            assert_eq!(advance(&s).unwrap().allowed, 1);
            s.cell.as_mut().unwrap().tag = 0;
            s.blocking_mask = [0; 32];
            assert_eq!(advance(&s).unwrap().allowed, 1);
        }
    }
    #[test]
    fn indexed_witness_is_not_an_expected_candidate_and_only_word_zero_is_owned() {
        let mut s = sample();
        s.cell.as_mut().unwrap().coordinate = 2;
        assert!(advance(&s).unwrap_err().contains("witness"));
        s.cell.as_mut().unwrap().coordinate = packed(9, 0);
        assert!(advance(&s).is_err());
        let s = sample();
        let mut e = machine(&s).unwrap();
        e.image[layout::P15 + 15..layout::P15 + 30].fill(0xA5);
        e.image[0x68000] = 42;
        let before = e.image.clone();
        assert_eq!(advance_image(&mut e, s.scene.index).unwrap(), 1);
        let a = scene(s.scene.index);
        for (i, (old, new)) in before.iter().zip(&e.image).enumerate() {
            if i != a && i != a + 1 {
                assert_eq!(old, new, "unexpected write at {i:05X}");
            }
        }
        assert_eq!(e.word(crate::SEED), s.seed);
        assert_eq!(e.word(a as u16), 1);
    }
    #[test]
    fn identities_and_expected_output_fields_are_not_admitted() {
        let source = serde_json::to_value(sample()).unwrap();
        for field in ["expected", "allowed", "output", "candidate"] {
            let mut v = source.clone();
            v[field] = serde_json::json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
        let mut s = sample();
        s.actor = 23;
        assert!(advance(&s).is_err());
        let mut s = sample();
        s.scene.index = 0;
        assert!(advance(&s).is_err());
        let mut s = sample();
        s.scene.word14 = 3921;
        assert!(advance(&s).is_err());
    }
}
