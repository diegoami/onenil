//! [confirmed] F33/9F6E/1AE2 at archaeology218490d; read-only defined-domain role scores.
use crate::{terminal::Scene, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Attributes {
    pub id: u16,
    pub p8_4: u8,
    pub p8_5: u8,
    pub p8_7: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub index: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TableInput {
    pub criterion: u16,
    pub player: Attributes,
    pub first: Cell,
    pub second: Cell,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneInput {
    pub table: TableInput,
    pub scene: Scene,
    pub byte6: u8,
    pub byte8: u8,
    pub bonus: u8,
    pub criterion_cell: u8,
}
#[derive(Debug, Serialize)]
pub struct Output {
    pub value: u16,
}
#[derive(Debug, Serialize)]
pub struct TableOutcome {
    pub output: Output,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct SceneOutcome {
    pub output: Output,
    pub table: TableOutcome,
    pub scene: Scene,
    pub draws: Vec<Draw>,
}
// [derived] Validate every raw-byte alias before allocating/loading the guest image.
pub(crate) type Cells = BTreeMap<usize, u8>;
pub(crate) fn insert(c: &mut Cells, a: usize, value: u8) -> Result<(), String> {
    if let Some(old) = c.insert(a, value) {
        if old != value {
            return Err(format!("conflicting raw alias at {a:04X}"));
        }
    }
    Ok(())
}
pub(crate) fn table_cells(t: &TableInput) -> Result<Cells, String> {
    if t.player.id > 3920 {
        return Err("unsupported player ID".into());
    }
    let first = u16::from(t.player.p8_7)
        .wrapping_mul(22)
        .wrapping_add(t.criterion);
    let second = u16::from(t.player.p8_5)
        .wrapping_mul(22)
        .wrapping_add(u16::from(t.first.value));
    if t.first.index != first || t.second.index != second {
        return Err("indexed table witness differs".into());
    }
    let mut c = Cells::new();
    let a = 0x10000 + 8 * usize::from(t.player.id);
    for (offset, value) in [(4, t.player.p8_4), (5, t.player.p8_5), (7, t.player.p8_7)] {
        insert(&mut c, a + offset, value)?;
    }
    insert(
        &mut c,
        usize::from(0x957Du16.wrapping_add(first)),
        t.first.value,
    )?;
    insert(
        &mut c,
        usize::from(0x96B1u16.wrapping_add(second)),
        t.second.value,
    )?;
    Ok(c)
}
fn scene_cells(s: &SceneInput) -> Result<Cells, String> {
    if s.byte6 > 31
        || s.table.criterion > 255
        || !(1..=22).contains(&s.scene.index)
        || s.scene.word14 != s.table.player.id
    {
        return Err("unsupported role scene/P15+6/criterion/identity domain".into());
    }
    let mut c = table_cells(&s.table)?;
    for (offset, value) in s.scene.bytes().into_iter().enumerate() {
        insert(
            &mut c,
            0xCCEF + 25 * usize::from(s.scene.index) + offset,
            value,
        )?;
    }
    let a = 0x20000 + 15 * usize::from(s.table.player.id);
    for (address, value) in [
        (a + 6, s.byte6),
        (a + 8, s.byte8),
        (0xD29, s.bonus),
        (0x09FB + usize::from(s.table.criterion), s.criterion_cell),
    ] {
        insert(&mut c, address, value)?;
    }
    Ok(c)
}
pub(crate) fn image(c: Cells) -> Engine {
    let mut e = Engine::new(&ContestInput {
        k: 0,
        seed: 0x1234,
        ratings: [0; 8],
        goals: [0; 2],
    });
    for (a, v) in c {
        e.image[a] = v;
    }
    e
}
// [confirmed] F33/9F6E: both index additions and DS addresses wrap as words.
fn table_image(e: &Engine, id: u16, criterion: u16) -> TableOutcome {
    let a = 0x10000 + 8 * usize::from(id);
    let first = u16::from(e.image[a + 7])
        .wrapping_mul(22)
        .wrapping_add(criterion);
    let cell = e.image[usize::from(0x957Du16.wrapping_add(first))];
    let second = u16::from(e.image[a + 5])
        .wrapping_mul(22)
        .wrapping_add(u16::from(cell));
    let value = (u16::from(e.image[a + 4]) + 1).wrapping_mul(u16::from(
        e.image[usize::from(0x96B1u16.wrapping_add(second))],
    )) >> 5;
    TableOutcome {
        output: Output { value },
        draws: vec![],
    }
}
// [confirmed] F33/1AE2: supported baseline branch; nested native score on this image.
pub(crate) fn scene_image(e: &Engine, index: u8, criterion: u16) -> SceneOutcome {
    let row = Scene::read(e, index);
    let a = 0x20000 + 15 * usize::from(row.word14);
    let mut baseline = (i16::from(e.image[a + 6] as i8) + 1) as u16;
    baseline = baseline.wrapping_add(if criterion >= 21 {
        32
    } else {
        i16::from(e.image[a + 8] as i8) as u16
    });
    let table = table_image(e, row.word14, criterion);
    let mut rating = table.output.value;
    if row.byte22 != 0 {
        rating >>= 1;
    }
    let mut value = (baseline.wrapping_mul(rating) >> 8).wrapping_add(rating >> 2);
    if e.image[0x09FB + usize::from(criterion)] != 0 {
        value = value.wrapping_add(u16::from(row.byte18));
    }
    if row.byte7 < 12 && e.image[0xD29] != 0 {
        value = value.wrapping_add(value >> 4);
    }
    SceneOutcome {
        output: Output {
            value: value.max(1),
        },
        table,
        scene: row,
        draws: vec![],
    }
}
pub fn table(t: &TableInput) -> Result<TableOutcome, String> {
    Ok(table_image(
        &image(table_cells(t)?),
        t.player.id,
        t.criterion,
    ))
}
pub fn scene(s: &SceneInput) -> Result<SceneOutcome, String> {
    Ok(scene_image(
        &image(scene_cells(s)?),
        s.scene.index,
        s.table.criterion,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    // [designed] Synthetic inputs exercise static F33 branches, not new original coverage.
    fn sample() -> SceneInput {
        let c: serde_json::Value = serde_json::from_str(
            include_str!("../tests/fixtures/role_scores/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let mut s: SceneInput = serde_json::from_value(c["input"].clone()).unwrap();
        s.table.player.p8_4 = 255;
        s.table.player.p8_5 = 0;
        s.table.player.p8_7 = 0;
        s.table.criterion = 21;
        s.table.first = Cell {
            index: 21,
            value: 0,
        };
        s.table.second = Cell {
            index: 0,
            value: 255,
        };
        s.byte6 = 31;
        s.byte8 = 0;
        s.bonus = 0;
        s.criterion_cell = 0;
        s.scene.byte7 = 12;
        s.scene.byte18 = 0;
        s.scene.byte22 = 0;
        s
    }
    fn value(s: &SceneInput) -> u16 {
        scene(s).unwrap().output.value
    }
    #[test]
    fn low_word_product_is_truncated_before_shift() {
        let s = sample();
        assert_eq!(table(&s.table).unwrap().output.value, 2040);
        assert_eq!(value(&s), 764); // 64*2040 low word=65024;254+510.
    }
    #[test]
    fn logical_halving_nonzero_flag() {
        let mut s = sample();
        s.scene.byte22 = 255;
        assert_eq!(value(&s), 510); // 64*1020=65280;255+255.
        assert_eq!(scene(&s).unwrap().table.output.value, 2040);
    }
    #[test]
    fn signed_baseline_and_criterion_boundary() {
        let mut s = sample();
        s.table.criterion = 20;
        s.table.first.index = 20;
        s.byte8 = 128;
        assert_eq!(value(&s), 513); // -96*2040 low=768.
        s.byte8 = 255;
        assert_eq!(value(&s), 757); // 31*2040=63240.
        s.byte8 = 127;
        assert_eq!(value(&s), 753); // 159*2040 low=62216.
        s.table.criterion = 21;
        s.table.first.index = 21;
        assert_eq!(value(&s), 764);
    }
    #[test]
    fn unsigned_scene_bonus_side_threshold_and_minimum() {
        let mut s = sample();
        s.criterion_cell = 255;
        s.scene.byte18 = 255;
        assert_eq!(value(&s), 1019);
        s.bonus = 255;
        s.scene.byte7 = 11;
        assert_eq!(value(&s), 1082);
        s.scene.byte7 = 12;
        assert_eq!(value(&s), 1019);
        s.table.second.value = 0;
        s.criterion_cell = 0;
        assert_eq!(value(&s), 1);
        s.criterion_cell = 1;
        assert_eq!(value(&s), 255);
    }
    #[test]
    fn indices_and_addresses_wrap_as_guest_words() {
        let mut t = sample().table;
        t.player.p8_7 = 255;
        t.criterion = 65535;
        t.first.index = 5609;
        assert_eq!(table(&t).unwrap().output.value, 2040);
        t.player.p8_7 = 0;
        t.criterion = 0xFFFF;
        t.first.index = 0xFFFF;
        let e = image(table_cells(&t).unwrap());
        assert_eq!(e.image[0x957C], 0);
        assert_eq!(table_image(&e, t.player.id, t.criterion).output.value, 2040);
    }
    #[test]
    fn consistent_alias_reads_one_byte_conflicting_alias_rejects() {
        let mut t = sample().table;
        t.first.value = 7;
        t.second.index = 7;
        t.second.value = 7;
        t.criterion = 0x96B1 + 7 - 0x957D;
        t.first.index = t.criterion;
        assert_eq!(table(&t).unwrap().output.value, 56);
        t.second.value = 8;
        assert!(table_cells(&t)
            .unwrap_err()
            .contains("conflicting raw alias"));
    }
    #[test]
    fn missing_or_incorrect_index_witness_rejects() {
        let mut t = sample().table;
        t.first.index += 1;
        assert!(table(&t).is_err());
        t = sample().table;
        t.second.index += 1;
        assert!(table(&t).is_err());
    }
    #[test]
    fn unsupported_domains_diagnose_without_changing_input() {
        let s = sample();
        let before = serde_json::to_value(&s).unwrap();
        for b in [32, 127, 128, 255] {
            let mut bad = s.clone();
            bad.byte6 = b;
            assert!(scene(&bad).is_err());
        }
        for index in [0, 23, 255] {
            let mut bad = s.clone();
            bad.scene.index = index;
            assert!(scene(&bad).is_err());
        }
        let mut bad = s.clone();
        bad.scene.word14 += 1;
        assert!(scene(&bad).is_err());
        bad = s.clone();
        bad.table.criterion = 256;
        assert!(scene(&bad).is_err());
        bad = s.clone();
        bad.table.player.id = 3921;
        assert!(table(&bad.table).is_err());
        assert_eq!(serde_json::to_value(&s).unwrap(), before);
    }
    #[test]
    fn canonical_zero_and_maximum_ids_are_defined() {
        for id in [0, 3920] {
            let mut s = sample();
            s.table.player.id = id;
            s.scene.word14 = id;
            assert_eq!(value(&s), 764);
        }
    }
    #[test]
    fn expected_results_and_nested_intermediates_are_not_inputs() {
        let s = sample();
        for field in ["output", "expected", "baseline", "table_result"] {
            let mut v = serde_json::to_value(&s).unwrap();
            v[field] = serde_json::json!(99);
            assert!(serde_json::from_value::<SceneInput>(v).is_err());
            let mut v = serde_json::to_value(&s.table).unwrap();
            v[field] = serde_json::json!(99);
            assert!(serde_json::from_value::<TableInput>(v).is_err());
        }
        let mut v = serde_json::to_value(&s).unwrap();
        v["table"]["result"] = serde_json::json!(2040);
        assert!(serde_json::from_value::<SceneInput>(v).is_err());
    }
    #[test]
    fn actual_raw_cell_changes_nested_computation() {
        let mut s = sample();
        let before = scene(&s).unwrap();
        s.table.second.value = 128;
        let after = scene(&s).unwrap();
        assert_eq!(after.table.output.value, 1024);
        assert_eq!(after.output.value, 256);
        assert_ne!(before.output.value, after.output.value);
    }
    #[test]
    fn whole_image_scene_and_seed_preserved() {
        let s = sample();
        let mut e = image(scene_cells(&s).unwrap());
        e.image[0x8000] = 0xAB;
        let before = e.image.clone();
        let result = scene_image(&e, s.scene.index, s.table.criterion);
        table_image(&e, s.table.player.id, s.table.criterion);
        assert_eq!(e.image, before);
        assert_eq!(result.scene, s.scene);
        assert_eq!(&e.image[0xA51C..0xA51E], &0x1234u16.to_le_bytes());
    }
}
