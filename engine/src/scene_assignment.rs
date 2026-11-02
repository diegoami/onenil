//! [confirmed] F26/F28 at archaeology218490d: fixed-match8599/8663 closure.
use crate::{
    role_scores::{image, insert, Cells},
    terminal::Scene,
    Draw, Engine,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub club: u8,
    pub age: u8,
    pub role: u8,
    pub style: u8,
    pub value: u16,
    pub hand_selector: u8,
    pub hand_cell: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub roster: [u16; 11],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Count {
    pub id: u16,
    pub index40: u16,
    pub index42: u16,
    pub cell40: u8,
    pub cell42: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub index: u8,
    pub selector: u8,
    pub seed: u16,
    pub position_default: u16,
    pub position12: u16,
    pub directions: [u8; 4],
    pub clubs: [Club; 2],
    pub players: Vec<Player>,
    pub club_counts: Vec<Count>,
    pub scenes: Vec<Scene>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub helpers: Vec<Value>,
    pub draws: Vec<Draw>,
}

// [designed] Separate normalized far-table pool, disjoint from pairing's frame.
const TABLE: usize = 0x50000;
fn addr(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
}
fn club(id: u16) -> usize {
    0x30000 + 62 * usize::from(id - 1)
}
fn table(index: u16) -> usize {
    TABLE + usize::from(index.wrapping_shl(2).wrapping_add(2))
}
fn hand_address(selector: u8, style: u8) -> usize {
    usize::from(
        0x129Eu16
            .wrapping_add(14 * u16::from(selector))
            .wrapping_add(u16::from(style)),
    )
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, value: u16) {
    e.image[a..a + 2].copy_from_slice(&value.to_le_bytes());
}
fn cell_word(c: &mut Cells, a: usize, v: u16) -> Result<(), String> {
    for (i, b) in v.to_le_bytes().into_iter().enumerate() {
        insert(c, a + i, b)?;
    }
    Ok(())
}
fn domains(index: u8, selector: u8) -> Result<(), String> {
    if !(1..=22).contains(&index) || !(1..=22).contains(&selector) {
        return Err("unsupported physical scene index/selector".into());
    }
    Ok(())
}
// [derived] Admit all canonical identities and raw aliases before image allocation.
pub(crate) fn cells(s: &State) -> Result<Cells, String> {
    domains(s.index, s.selector)?;
    if s.clubs[0].id == s.clubs[1].id || s.clubs.iter().any(|c| !(1..=176).contains(&c.id)) {
        return Err("unsupported header clubs".into());
    }
    if s.scenes.len() != 22
        || s.scenes
            .iter()
            .enumerate()
            .any(|(i, r)| usize::from(r.index) != i + 1)
    {
        return Err("expected 22 ordered scenes".into());
    }
    let mut c = Cells::new();
    let mut ids = BTreeSet::new();
    let mut clubs = BTreeSet::new();
    for p in &s.players {
        if p.id > 3920
            || !ids.insert(p.id)
            || p.role > 13
            || p.style > 13
            || (p.id != 0 && !(1..=176).contains(&p.club))
            || (p.id == 0 && p.club != 0)
        {
            return Err("unsupported/duplicate canonical player".into());
        }
        if p.id != 0 {
            clubs.insert(u16::from(p.club));
        }
        let a = 0x10000 + 8 * usize::from(p.id);
        for (i, v) in [(0, p.club), (1, p.age), (6, p.role), (7, p.style)] {
            insert(&mut c, a + i, v)?;
        }
        cell_word(&mut c, 0x20000 + 15 * usize::from(p.id), p.value)?;
        insert(&mut c, 0x12BA + usize::from(p.id % 3), p.hand_selector)?;
        insert(&mut c, hand_address(p.hand_selector, p.style), p.hand_cell)?;
    }
    for (side, h) in s.clubs.iter().enumerate() {
        cell_word(&mut c, 0xD6E4 + 2 * side, h.id)?;
        for (slot, id) in h.roster.iter().enumerate() {
            if !ids.contains(id) {
                return Err("roster missing canonical player".into());
            }
            cell_word(&mut c, club(h.id) + 2 * slot, *id)?;
        }
    }
    for r in &s.scenes {
        if !ids.contains(&r.word14) {
            return Err("scene missing canonical player".into());
        }
        for (i, v) in r.bytes().into_iter().enumerate() {
            insert(&mut c, addr(r.index) + i, v)?;
        }
    }
    let mut counts = BTreeSet::new();
    for t in &s.club_counts {
        if !(1..=176).contains(&t.id) || !counts.insert(t.id) {
            return Err("unsupported/duplicate count club".into());
        }
        cell_word(&mut c, club(t.id) + 40, t.index40)?;
        cell_word(&mut c, club(t.id) + 42, t.index42)?;
        insert(&mut c, table(t.index40), t.cell40)?;
        insert(&mut c, table(t.index42), t.cell42)?;
    }
    if counts != clubs {
        return Err("club-count canonical set differs".into());
    }
    cell_word(&mut c, 0xA51C, s.seed)?;
    cell_word(&mut c, 0xD32, s.position_default)?;
    cell_word(&mut c, 0xD2C, s.position12)?;
    for (i, v) in s.directions.into_iter().enumerate() {
        insert(&mut c, 0xD0C8 + i, v)?;
    }
    Ok(c)
}
// [confirmed] 24BC:04FA: signed high word, then unsigned low word; complete DX:AX.
fn minimum32(a: u32, b: u32) -> u32 {
    if (a as i32) <= (b as i32) {
        a
    } else {
        b
    }
}
fn node(
    site: &str,
    kind: &str,
    args: Value,
    before: &Scene,
    after: &Scene,
    result: Value,
    children: Vec<Value>,
) -> Value {
    json!({"site":site,"kind":kind,"arguments":args,"scene_input":before,"scene_output":after,
        "result":result,"children":children,"draw_start":0,"draw_end":0})
}
fn scalar(
    e: &Engine,
    index: u8,
    site: &str,
    kind: &str,
    args: Value,
    result: Value,
    children: Vec<Value>,
) -> Value {
    let row = Scene::read(e, index);
    node(site, kind, args, &row, &row, result, children)
}
pub(crate) fn hand(e: &Engine, id: u16) -> u8 {
    let selector = e.image[0x12BA + usize::from(id % 3)];
    e.image[hand_address(selector, e.image[0x10000 + 8 * usize::from(id) + 7])]
}
fn half(e: &Engine, index: u8, id: u16) -> (u8, Vec<Value>) {
    let p = 0x10000 + 8 * usize::from(id);
    let role = e.image[p + 6];
    let style = e.image[p + 7];
    let h = hand(e, id);
    let mut nodes = vec![scalar(
        e,
        index,
        "1B55:4F1F",
        "hand",
        json!([id]),
        json!(h),
        vec![],
    )];
    if h != 0 && [3, 7, 10].contains(&role) {
        return (1, nodes);
    }
    let h = hand(e, id);
    nodes.push(scalar(
        e,
        index,
        "1B55:4F3D",
        "hand",
        json!([id]),
        json!(h),
        vec![],
    ));
    (
        u8::from((h == 0 && [4, 8, 11].contains(&role)) || ((role == 0) != (style == 0))),
        nodes,
    )
}
// [confirmed] F26/F28 8599; all callees read/write this same image.
pub(crate) fn assign_image(e: &mut Engine, index: u8, selector: u8) -> Result<Vec<Value>, String> {
    domains(index, selector)?;
    let a = addr(index);
    e.image[a + 7] = selector;
    let slot = if selector > 11 {
        selector - 11
    } else {
        selector
    };
    let side = u8::from(selector < 12);
    let mut nodes = vec![
        scalar(
            e,
            index,
            "0AB9:85AA",
            "slot",
            json!([index]),
            json!(slot),
            vec![],
        ),
        scalar(
            e,
            index,
            "0AB9:85B8",
            "side",
            json!([index]),
            json!(side),
            vec![],
        ),
    ];
    let header = word(e, 0xD6E4 + 2 * usize::from(side));
    if !(1..=176).contains(&header) {
        return Err("unsupported header club lookup".into());
    }
    let id = word(e, club(header) + 2 * usize::from(slot - 1));
    if id == 0 || id > 3920 {
        return Err("selected player zero/unsupported".into());
    }
    put(e, a + 14, id);
    let p = 0x10000 + 8 * usize::from(id);
    let player_club = u16::from(e.image[p]);
    if !(1..=176).contains(&player_club) {
        return Err("unsupported selected player club lookup".into());
    }
    let count = 16
        + u16::from(e.image[table(word(e, club(player_club) + 42))] >> 4)
        + u16::from(e.image[table(word(e, club(player_club) + 40))] >> 5);
    nodes.push(scalar(
        e,
        index,
        "0AB9:85EC",
        "club-count",
        json!([player_club]),
        json!(count),
        vec![],
    ));
    let value = word(e, 0x20000 + 15 * usize::from(id));
    let minimum = minimum32(u32::from(value), u32::from(count));
    nodes.push(scalar(
        e,
        index,
        "0AB9:860A",
        "minimum32",
        json!([value, count]),
        json!(minimum),
        vec![],
    ));
    let age = e.image[p + 1];
    e.image[a + 18] = (minimum as u8).wrapping_add(if age > 18 {
        32
    } else if age > 17 {
        16
    } else {
        0
    });
    let (flag, children) = half(e, index, id);
    nodes.push(scalar(
        e,
        index,
        "0AB9:8653",
        "half",
        json!([id]),
        json!(flag),
        children,
    ));
    e.image[a + 22] = flag;
    Ok(nodes)
}
pub(crate) fn rebuild_image(e: &mut Engine, index: u8, selector: u8) -> Result<Vec<Value>, String> {
    let before = Scene::read(e, index);
    let children = assign_image(e, index, selector)?;
    let mut nodes = vec![node(
        "0AB9:866F",
        "assign",
        json!([index, selector]),
        &before,
        &Scene::read(e, index),
        Value::Null,
        children,
    )];
    let a = addr(index);
    let position = word(e, 0xD32);
    put(e, a, position);
    put(e, a + 2, position);
    let keeper = u8::from(selector == 1 || selector == 12);
    let side = u8::from(selector < 12);
    nodes.push(scalar(
        e,
        index,
        "0AB9:8689",
        "keeper",
        json!([selector]),
        json!(keeper),
        vec![],
    ));
    nodes.push(scalar(
        e,
        index,
        if keeper != 0 {
            "0AB9:8696"
        } else {
            "0AB9:86AF"
        },
        "direction-side",
        json!([index]),
        json!(side),
        vec![],
    ));
    e.image[a + 5] = e.image[0xD0C8 + 2 * usize::from(keeper) + usize::from(side)];
    let position12 = word(e, 0xD2C);
    put(e, a + 12, position12);
    put(e, a + 20, 0);
    e.image[a + 17] = 0;
    put(e, a + 23, 0);
    e.image[a + 6] = 0;
    e.image[a + 16] = 5;
    Ok(nodes)
}
// [derived] Identity lists select records; values are reread from the live image.
pub(crate) fn snapshot(e: &Engine, s: &State) -> State {
    let mut out = s.clone();
    out.seed = word(e, 0xA51C);
    out.position_default = word(e, 0xD32);
    out.position12 = word(e, 0xD2C);
    out.directions.copy_from_slice(&e.image[0xD0C8..0xD0CC]);
    for (side, c) in out.clubs.iter_mut().enumerate() {
        c.id = word(e, 0xD6E4 + 2 * side);
        for (i, id) in c.roster.iter_mut().enumerate() {
            *id = word(e, club(c.id) + 2 * i);
        }
    }
    for p in &mut out.players {
        let a = 0x10000 + 8 * usize::from(p.id);
        p.club = e.image[a];
        p.age = e.image[a + 1];
        p.role = e.image[a + 6];
        p.style = e.image[a + 7];
        p.value = word(e, 0x20000 + 15 * usize::from(p.id));
        p.hand_selector = e.image[0x12BA + usize::from(p.id % 3)];
        p.hand_cell = e.image[hand_address(p.hand_selector, p.style)];
    }
    for t in &mut out.club_counts {
        t.index40 = word(e, club(t.id) + 40);
        t.index42 = word(e, club(t.id) + 42);
        t.cell40 = e.image[table(t.index40)];
        t.cell42 = e.image[table(t.index42)];
    }
    out.scenes = (1..=22).map(|i| Scene::read(e, i)).collect();
    out
}
pub fn assign(s: &State) -> Result<Outcome, String> {
    let mut e = image(cells(s)?);
    let helpers = assign_image(&mut e, s.index, s.selector)?;
    Ok(Outcome {
        output: snapshot(&e, s),
        helpers,
        draws: vec![],
    })
}
pub fn rebuild(s: &State) -> Result<Outcome, String> {
    let mut e = image(cells(s)?);
    let helpers = rebuild_image(&mut e, s.index, s.selector)?;
    Ok(Outcome {
        output: snapshot(&e, s),
        helpers,
        draws: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // [designed] Synthetic static-branch tests; independent of observed-case census.
    fn sample() -> State {
        let v: Value = serde_json::from_str(
            include_str!("../tests/fixtures/scene_assignment/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let mut s: State = serde_json::from_value(v["input"].clone()).unwrap();
        s.index = 8;
        s.selector = 8;
        s.seed = 0x1234;
        s.clubs = [
            Club {
                id: 130,
                roster: [1; 11],
            },
            Club {
                id: 134,
                roster: [2; 11],
            },
        ];
        s.players = (0..=2)
            .map(|id| Player {
                id,
                club: if id == 0 {
                    0
                } else if id == 1 {
                    130
                } else {
                    134
                },
                age: 17,
                role: 2,
                style: id as u8,
                value: 100,
                hand_selector: 0,
                hand_cell: 0,
            })
            .collect();
        for r in &mut s.scenes {
            r.word14 = 1;
        }
        s.club_counts = vec![
            Count {
                id: 130,
                index40: 10,
                index42: 11,
                cell40: 32,
                cell42: 64,
            },
            Count {
                id: 134,
                index40: 12,
                index42: 13,
                cell40: 64,
                cell42: 80,
            },
        ];
        s.directions = [3, 4, 5, 6];
        s.position_default = 0xBEEF;
        s.position12 = 0xABCD;
        s
    }
    #[test]
    fn selector_is_independent_of_physical_index_and_reads_live_roster() {
        let mut s = sample();
        let o = assign(&s).unwrap();
        assert_eq!(o.output.scenes[7].word14, 2);
        assert_eq!(o.output.scenes[7].byte18, 23);
        s.selector = 13;
        let o = assign(&s).unwrap();
        assert_eq!(o.output.scenes[7].byte7, 13);
        assert_eq!(o.output.scenes[7].word14, 1);
        assert_eq!(o.output.scenes[7].byte18, 21);
        assert_eq!(o.helpers[0]["result"], 2);
        assert_eq!(o.helpers[1]["result"], 0);
        s.clubs[0].roster[1] = 2;
        assert_eq!(assign(&s).unwrap().output.scenes[7].word14, 2);
    }
    #[test]
    fn counts_follow_actual_player_club_and_minimum_then_age() {
        let mut s = sample();
        s.players[2].club = 130;
        s.club_counts.pop();
        // All nonzero canonical players now belong to130, regardless of header134.
        assert_eq!(assign(&s).unwrap().output.scenes[7].byte18, 21);
        for (age, expected) in [(0, 7), (17, 7), (18, 23), (19, 39), (255, 39)] {
            s.players[2].value = 7;
            s.players[2].age = age;
            let o = assign(&s).unwrap();
            assert_eq!(o.output.scenes[7].byte18, expected);
            assert_eq!(o.helpers[3]["arguments"], json!([7, 21]));
        }
        s.players[2].value = u16::MAX;
        s.players[2].age = 19;
        s.club_counts[0].cell40 = 255;
        s.club_counts[0].cell42 = 255;
        assert_eq!(assign(&s).unwrap().output.scenes[7].byte18, 70);
    }
    #[test]
    fn signed32_minimum_keeps_high_and_low_words() {
        for (a, b, want) in [
            (0x80000000, 0, 0x80000000),
            (0xFFFFFFFF, 0x80000000, 0x80000000),
            (0x1234FFFF, 0x12348000, 0x12348000),
            (0x7FFFFFFF, 0x10000, 0x10000),
            (0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF),
        ] {
            assert_eq!(minimum32(a, b), want);
            assert_eq!(minimum32(b, a), want);
        }
    }
    #[test]
    fn wrapped_table_aliases_share_a_byte_and_reject_conflicts() {
        let mut s = sample();
        s.club_counts[0].index40 = 0xFFFF;
        s.club_counts[0].index42 = 0x3FFF;
        s.club_counts[0].cell40 = 255;
        s.club_counts[0].cell42 = 255;
        assert_eq!(assign(&s).unwrap().output.scenes[7].byte18, 23);
        let e = image(cells(&s).unwrap());
        assert_eq!(e.image[TABLE + 0xFFFE], 255);
        s.club_counts[0].cell42 = 254;
        assert!(cells(&s).unwrap_err().contains("alias"));
        let mut s = sample();
        s.club_counts[1].index40 = s.club_counts[0].index40 + 0x4000;
        assert!(cells(&s).unwrap_err().contains("alias"));
        s.club_counts[1].cell40 = s.club_counts[0].cell40;
        assert!(cells(&s).is_ok());
    }
    #[test]
    fn raw_hand_values_and_predicate_branch_order() {
        for (role, style, h, flag, calls) in [
            (3, 2, 255, 1, 1),
            (7, 2, 1, 1, 1),
            (10, 2, 2, 1, 1),
            (4, 2, 0, 1, 2),
            (8, 2, 0, 1, 2),
            (11, 2, 0, 1, 2),
            (4, 2, 255, 0, 2),
            (0, 2, 0, 1, 2),
            (2, 0, 0, 1, 2),
            (0, 0, 0, 0, 2),
            (2, 2, 0, 0, 2),
        ] {
            let mut s = sample();
            s.players[2].role = role;
            s.players[2].style = style;
            // All records sharing this raw cell must agree.
            for p in &mut s.players {
                if p.style == style {
                    p.hand_cell = h;
                }
            }
            let o = assign(&s).unwrap();
            assert_eq!(o.output.scenes[7].byte22, flag);
            let children = o.helpers[4]["children"].as_array().unwrap();
            assert_eq!(children.len(), calls);
            assert_eq!(children[0]["result"], h);
            assert_eq!(children[0]["site"], "1B55:4F1F");
            if calls == 2 {
                assert_eq!(children[1]["site"], "1B55:4F3D");
            }
        }
    }
    #[test]
    fn hand_selector_and_cell_cross_aliases_reject_before_image_setup() {
        let mut s = sample();
        s.players.push(Player {
            id: 3,
            ..s.players[0].clone()
        });
        s.players[3].club = 130;
        s.players[3].hand_selector = 1;
        assert!(cells(&s).unwrap_err().contains("alias"));
        let mut s = sample();
        s.players[2].style = 1;
        s.players[2].hand_cell = 255;
        assert!(cells(&s).unwrap_err().contains("alias"));
        let mut s = sample();
        for p in &mut s.players {
            p.hand_selector = 2;
        }
        // selector2/style0 aliases DS12BA: raw cell must equal selector2.
        assert!(cells(&s).unwrap_err().contains("alias"));
        for p in &mut s.players {
            p.hand_cell = 2;
        }
        assert!(cells(&s).is_ok());
        let e = image(cells(&s).unwrap());
        assert_eq!(hand(&e, 2), 2);
    }
    #[test]
    fn keeper_direction_and_reset_order_preserve_callers_word2_responsibility() {
        for (selector, direction, keeper) in [(1, 6, 1), (12, 5, 1), (11, 4, 0), (22, 3, 0)] {
            let mut s = sample();
            s.selector = selector;
            let before = s.scenes[7].clone();
            let o = rebuild(&s).unwrap();
            let r = &o.output.scenes[7];
            assert_eq!((r.word0, r.word2, r.word12), (0xBEEF, 0xBEEF, 0xABCD));
            assert_eq!(r.byte5, direction);
            assert_eq!(r.byte4, before.byte4);
            assert_eq!(
                (r.byte6, r.byte16, r.byte17, r.word20, r.word23),
                (0, 5, 0, 0, 0)
            );
            assert_eq!(
                (r.word8, r.word10, r.byte19),
                (before.word8, before.word10, before.byte19)
            );
            assert_eq!(o.helpers[1]["result"], keeper);
            assert_eq!(o.helpers[1]["scene_input"]["word2"], 0xBEEF);
            assert_eq!(
                o.helpers[2]["site"],
                if keeper == 1 {
                    "0AB9:8696"
                } else {
                    "0AB9:86AF"
                }
            );
            assert_eq!(o.helpers[2]["scene_output"]["byte5"], before.byte5);
        }
    }
    #[test]
    fn admission_errors_do_not_mutate_inputs() {
        let s = sample();
        let before = serde_json::to_value(&s).unwrap();
        let mut cases = vec![];
        for i in [0, 23, 255] {
            let mut b = s.clone();
            b.index = i;
            cases.push(b);
            let mut b = s.clone();
            b.selector = i;
            cases.push(b);
        }
        let mut b = s.clone();
        b.players.pop();
        cases.push(b);
        let mut b = s.clone();
        b.players[1].id = 3921;
        cases.push(b);
        let mut b = s.clone();
        b.players[1].role = 14;
        cases.push(b);
        let mut b = s.clone();
        b.players[1].style = 14;
        cases.push(b);
        let mut b = s.clone();
        b.players[1].club = 177;
        cases.push(b);
        let mut b = s.clone();
        b.players.push(b.players[1].clone());
        cases.push(b);
        let mut b = s.clone();
        b.club_counts.pop();
        cases.push(b);
        let mut b = s.clone();
        b.club_counts.push(b.club_counts[0].clone());
        cases.push(b);
        let mut b = s.clone();
        b.club_counts[0].id = 177;
        cases.push(b);
        let mut b = s.clone();
        b.clubs[0].id = 0;
        cases.push(b);
        let mut b = s.clone();
        b.clubs[0].id = b.clubs[1].id;
        cases.push(b);
        let mut b = s.clone();
        b.scenes[0].index = 2;
        cases.push(b);
        let mut b = s.clone();
        b.scenes.pop();
        cases.push(b);
        let mut b = s.clone();
        b.scenes[0].word14 = 333;
        cases.push(b);
        for b in cases {
            assert!(assign(&b).is_err());
            assert!(rebuild(&b).is_err());
        }
        assert_eq!(serde_json::to_value(&s).unwrap(), before);
    }
    #[test]
    fn sentinel_is_allowed_unselected_but_never_invents_club_zero_count() {
        let mut s = sample();
        s.clubs[1].roster[7] = 0;
        assert!(assign(&s).unwrap_err().contains("zero"));
        s.selector = 13;
        assert!(assign(&s).is_ok());
        s.players[0].club = 1;
        assert!(assign(&s).is_err());
    }
    #[test]
    fn injected_expected_and_intermediate_fields_are_rejected() {
        for field in ["expected", "output", "helpers", "count", "minimum", "half"] {
            let mut v = serde_json::to_value(sample()).unwrap();
            v[field] = json!(99);
            assert!(serde_json::from_value::<State>(v).is_err());
            for member in ["players", "clubs", "club_counts", "scenes"] {
                let mut v = serde_json::to_value(sample()).unwrap();
                v[member][0][field] = json!(99);
                assert!(serde_json::from_value::<State>(v).is_err());
            }
        }
    }
    #[test]
    fn whole_image_write_ownership_and_separate_table_pool() {
        let s = sample();
        for rebuild in [false, true] {
            let mut e = image(cells(&s).unwrap());
            for a in [0x8000, 0x40000, 0x40010, 0x40020, 0x40080] {
                e.image[a] = 0xAB;
            }
            let before = e.image.clone();
            if rebuild {
                rebuild_image(&mut e, s.index, s.selector).unwrap();
            } else {
                assign_image(&mut e, s.index, s.selector).unwrap();
            }
            let allowed: Vec<usize> = if rebuild {
                vec![
                    0, 1, 2, 3, 5, 6, 7, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24,
                ]
            } else {
                vec![7, 14, 15, 18, 22]
            };
            for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
                if old != new {
                    assert!(
                        allowed.iter().any(|i| a == addr(s.index) + i),
                        "unexpected write {a:X}"
                    );
                }
            }
            assert_eq!(word(&e, 0xA51C), 0x1234);
            assert_eq!(e.image.len(), 0x60000);
        }
    }
    #[test]
    fn kernels_compose_without_reloading_observed_state() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        put(&mut e, club(134) + 14, 1); // roster changed by a caller, not a fixture reload.
        assign_image(&mut e, 8, 8).unwrap();
        assert_eq!(Scene::read(&e, 8).word14, 1);
        put(&mut e, 0xD32, 999);
        rebuild_image(&mut e, 8, 8).unwrap();
        assert_eq!(Scene::read(&e, 8).word2, 999);
        assert_eq!(snapshot(&e, &s).clubs[1].roster[7], 1);
    }
}
