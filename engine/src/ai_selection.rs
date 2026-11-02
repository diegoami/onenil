//! [confirmed] F23–F28 at archaeology218490d: boundedA94B shared-image selection.
use crate::{rearrangement, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub roster: [u16; 20],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub index: u8,
    pub slot: u8,
    pub player: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub byte8: u8,
    pub flags11: u8,
    pub position: u8,
    pub style: u8,
    pub hand_selector: u8,
    pub hand_cell: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Preference {
    pub position: u8,
    pub styles: Vec<u8>,
    pub weights: [u8; 14],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub seed: u16,
    pub managed_clubs: [u8; 2],
    pub clubs: [Club; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub side_offsets: [u8; 2],
    pub remaining: u16,
    pub added: u16,
    pub allowance: u8,
    pub sub_flags: [u8; 2],
    pub reserves: [u8; 2],
    pub pending: u8,
    pub control: u8,
    pub key: u8,
    pub event_totals: [[u16; 20]; 2],
    pub limit_cell: u8,
    pub style_text: String,
    pub preferences: Vec<Preference>,
    pub candidates: [u8; 14],
    pub cache_selectors: [[u8; 20]; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReadSet {
    pub match_state: rearrangement::State,
    pub display_mode: u8,
    pub position_sentinel: u16,
    pub speaker_gate: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub context: Context,
    pub mode: u8,
    pub scene: u8,
    pub side: u8,
    pub replacement: Option<ReadSet>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: Context,
    pub replacement: Option<ReadSet>,
    pub helpers: Vec<Value>,
    pub presentation: Vec<Value>,
    pub draws: Vec<Draw>,
}
use crate::{
    placement, replacement,
    role_scores::{image, insert, Cells},
    scene_assignment, Engine,
};
use serde_json::json;
use std::collections::BTreeSet;
fn addr(i: u8) -> usize {
    0xCCEF + 25 * usize::from(i)
}
fn club(id: u16) -> usize {
    crate::layout::CLUB + 62 * usize::from(id - 1)
}
fn p8(id: u16) -> usize {
    crate::layout::P8 + 8 * usize::from(id)
}
fn p15(id: u16) -> usize {
    crate::layout::P15 + 15 * usize::from(id)
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn cw(c: &mut Cells, a: usize, v: u16) -> Result<(), String> {
    for (i, b) in v.to_le_bytes().into_iter().enumerate() {
        insert(c, a + i, b)?;
    }
    Ok(())
}
fn limit_address(allowance: u8) -> usize {
    usize::from(0xD58u16.wrapping_add(i16::from(allowance as i8) as u16))
}
pub(crate) fn cells(s: &State) -> Result<Cells, String> {
    let x = &s.context;
    if s.side > 1
        || !(1..=22).contains(&s.scene)
        || !matches!(x.side_offsets, [0, 11] | [11, 0])
        || x.clubs[0].id == x.clubs[1].id
        || x.clubs.iter().any(|c| !(1..=176).contains(&c.id))
    {
        return Err("unsupportedA94B side/physical scene/offset/club domain".into());
    }
    let offset = x.side_offsets[usize::from(s.side)];
    if s.scene <= offset || s.scene > offset + 11 {
        return Err("A94B scene does not belong to side".into());
    }
    if x.scenes.len() != 22
        || x.scenes
            .iter()
            .enumerate()
            .any(|(i, r)| usize::from(r.index) != i + 1 || !(1..=22).contains(&r.slot))
    {
        return Err("unordered/unsupported selection scenes".into());
    }
    if x.preferences.len() != 14
        || x.preferences.iter().enumerate().any(|(i, r)| {
            usize::from(r.position) != i || r.styles.len() > 7 || r.styles.iter().any(|v| *v > 13)
        })
        || x.style_text.chars().count() > 255
        || x.style_text.chars().any(|c| u32::from(c) > 255)
    {
        return Err("unsupported preference/string domain".into());
    }
    if 11 + u16::from(x.allowance) + u16::from(x.limit_cell) > 20
        || x.candidates
            .iter()
            .any(|v| *v != 0 && !(12..=20).contains(v))
    {
        return Err("unsupported reserve limit/candidate slot".into());
    }
    let mut c = if let Some(r) = &s.replacement {
        let mut c = rearrangement::cells(&r.match_state)?;
        insert(&mut c, 0xD074, r.display_mode)?;
        cw(&mut c, 0xD2E, r.position_sentinel)?;
        insert(&mut c, 0xA2EC, r.speaker_gate)?;
        c
    } else {
        Cells::new()
    };
    let mut ids = BTreeSet::new();
    for p in &x.players {
        if p.id > 3920 || p.position > 13 || p.style > 13 || !ids.insert(p.id) {
            return Err("ambiguous selection player/role/style".into());
        }
        for (a, v) in [
            (p8(p.id) + 6, p.position),
            (p8(p.id) + 7, p.style),
            (p15(p.id) + 8, p.byte8),
            (p15(p.id) + 11, p.flags11),
            (0x12BA + usize::from(p.id % 3), p.hand_selector),
            (
                0x129E + 14 * usize::from(p.hand_selector) + usize::from(p.style),
                p.hand_cell,
            ),
        ] {
            insert(&mut c, a, v)?;
        }
    }
    if let Some(r) = &s.replacement {
        let full: BTreeSet<_> = r.match_state.core.players.iter().map(|p| p.id).collect();
        if !ids.is_subset(&full) {
            return Err("replacement missing selection canonical players".into());
        }
    }
    for r in &x.scenes {
        if !ids.contains(&r.player) {
            return Err("scene missing selection canonical player".into());
        }
        cw(&mut c, addr(r.index) + 14, r.player)?;
        insert(&mut c, addr(r.index) + 7, r.slot)?;
    }
    for (side, h) in x.clubs.iter().enumerate() {
        cw(&mut c, 0xD6E4 + 2 * side, h.id)?;
        insert(&mut c, 0xD56 + side, x.side_offsets[side])?;
        insert(&mut c, 0xDCBA + side, x.managed_clubs[side])?;
        insert(&mut c, 0xD6DE + side, x.sub_flags[side])?;
        insert(&mut c, 0xD6E0 + side, x.reserves[side])?;
        for (i, id) in h.roster.iter().enumerate() {
            if !ids.contains(id) {
                return Err("roster missing selection player".into());
            }
            cw(&mut c, club(h.id) + 2 * i, *id)?;
        }
        for i in 0..20 {
            cw(&mut c, 0xD148 + 40 * side + 2 * i, x.event_totals[side][i])?;
            insert(&mut c, 0xD198 + 20 * side + i, x.cache_selectors[side][i])?;
        }
    }
    for r in &x.preferences {
        let a = 0xBF6 + 8 * usize::from(r.position);
        insert(&mut c, a, r.styles.len() as u8)?;
        for (i, b) in r.styles.iter().enumerate() {
            insert(&mut c, a + i + 1, *b)?;
        }
        for (i, b) in r.weights.iter().enumerate() {
            insert(&mut c, 0xDA6 + 14 * usize::from(r.position) + i, *b)?;
        }
    }
    insert(&mut c, 0xD9A, x.style_text.chars().count() as u8)?;
    for (i, ch) in x.style_text.chars().enumerate() {
        insert(&mut c, 0xD9B + i, ch as u8)?;
    }
    for (i, b) in x.candidates.iter().enumerate() {
        insert(&mut c, 0xD076 + i, *b)?;
    }
    for (a, v) in [
        (0xD6DD, x.allowance),
        (0xD6E2, x.pending),
        (0xD0BA, x.control),
        (0xDE8A, x.key),
        (limit_address(x.allowance), x.limit_cell),
    ] {
        insert(&mut c, a, v)?;
    }
    cw(&mut c, 0xA51C, x.seed)?;
    cw(&mut c, 0xD0AE, x.remaining)?;
    cw(&mut c, 0xD0B4, x.added)?;
    Ok(c)
}
pub(crate) fn snapshot(e: &Engine, x: &Context) -> Context {
    let mut o = x.clone();
    o.seed = word(e, 0xA51C);
    o.remaining = word(e, 0xD0AE);
    o.added = word(e, 0xD0B4);
    o.allowance = e.image[0xD6DD];
    o.pending = e.image[0xD6E2];
    o.control = e.image[0xD0BA];
    o.key = e.image[0xDE8A];
    o.limit_cell = e.image[limit_address(o.allowance)];
    for side in 0..2 {
        o.side_offsets[side] = e.image[0xD56 + side];
        o.managed_clubs[side] = e.image[0xDCBA + side];
        o.sub_flags[side] = e.image[0xD6DE + side];
        o.reserves[side] = e.image[0xD6E0 + side];
        o.clubs[side].id = word(e, 0xD6E4 + 2 * side);
        for i in 0..20 {
            o.clubs[side].roster[i] = word(e, club(o.clubs[side].id) + 2 * i);
            o.event_totals[side][i] = word(e, 0xD148 + 40 * side + 2 * i);
            o.cache_selectors[side][i] = e.image[0xD198 + 20 * side + i];
        }
    }
    for r in &mut o.scenes {
        r.player = word(e, addr(r.index) + 14);
        r.slot = e.image[addr(r.index) + 7];
    }
    for p in &mut o.players {
        p.position = e.image[p8(p.id) + 6];
        p.style = e.image[p8(p.id) + 7];
        p.byte8 = e.image[p15(p.id) + 8];
        p.flags11 = e.image[p15(p.id) + 11];
        p.hand_selector = e.image[0x12BA + usize::from(p.id % 3)];
        p.hand_cell = scene_assignment::hand(e, p.id);
    }
    o.candidates = e.image[0xD076..0xD084].try_into().unwrap();
    o
}
fn candidates(e: &Engine) -> Value {
    json!(&e.image[0xD076..0xD084])
}
fn trace(
    site: &str,
    kind: &str,
    args: Value,
    before: Value,
    after: Value,
    children: Vec<Value>,
    result: Option<u16>,
) -> Value {
    let mut v = json!({"site":site,"kind":kind,"arguments":args,"candidates_before":before,"candidates_after":after,"children":children,"draw_start":0,"draw_end":0});
    if let Some(result) = result {
        v["result"] = json!(result);
    }
    v
}
fn readonly(e: &Engine, site: &str, kind: &str, args: Value, result: u16) -> Value {
    let v = candidates(e);
    trace(site, kind, args, v.clone(), v, vec![], Some(result))
}
fn available(e: &Engine, id: u16) -> u8 {
    placement::unavailable(&placement::Player {
        id,
        p15_sbyte8: e.image[p15(id) + 8] as i8,
        p15_byte11: e.image[p15(id) + 11],
    })
}
fn limit(e: &Engine) -> u16 {
    let allowance = e.image[0xD6DD];
    11 + u16::from(allowance) + u16::from(e.image[limit_address(allowance)])
}
fn roster(e: &Engine, side: u8, slot: u16) -> u16 {
    word(
        e,
        club(word(e, 0xD6E4 + 2 * usize::from(side))) + 2 * usize::from(slot - 1),
    )
}
fn scan(e: &mut Engine, s: &State) -> Vec<Value> {
    let end = limit(e);
    let mut children = vec![readonly(e, "0AB9:A8A4", "limit", json!([]), end)];
    // [confirmed] A8D8 subtracts the side offset before reading the exclusion scene.
    let normalized = s.scene - e.image[0xD56 + usize::from(s.side)];
    let current = word(e, addr(normalized) + 14);
    for slot in 12..=end {
        let id = roster(e, s.side, slot);
        if id == current {
            continue;
        }
        let unavailable = available(e, id);
        children.push(readonly(
            e,
            "0AB9:A909",
            "available",
            json!([id]),
            u16::from(unavailable),
        ));
        if unavailable == 0 {
            let style = e.image[p8(id) + 7];
            e.image[0xD076 + usize::from(style)] = slot as u8;
        }
    }
    children
}
fn search(e: &Engine, role: u8) -> usize {
    (0..usize::from(e.image[0xD9A]))
        .find(|i| e.image[0xD9B + i] == role)
        .map_or(0, |i| i + 1)
}
fn preferred(e: &Engine, s: &State) -> (u16, Vec<Value>) {
    let id = word(e, addr(s.scene) + 14);
    let role = e.image[p8(id) + 6];
    let pos = search(e, role);
    let base = 0xBF6 + 8 * usize::from(role);
    let mut chosen = 0;
    let mut children = vec![];
    for i in 1..=usize::from(e.image[base]) {
        let style = e.image[base + i];
        let slot = e.image[0xD076 + usize::from(style)];
        if slot == 0 {
            continue;
        }
        if pos != 0 {
            let id = roster(e, s.side, u16::from(slot));
            let hand = scene_assignment::hand(e, id);
            children.push(readonly(
                e,
                "0AB9:993A",
                "handedness",
                json!([id]),
                u16::from(hand),
            ));
            if hand != u8::from(pos > 3) {
                continue;
            }
        }
        chosen = u16::from(slot);
    }
    (chosen, children)
}
fn weighted(e: &Engine, s: &State, role: u8, end: u16) -> (u16, Vec<Value>) {
    let pos = search(e, role);
    let mut chosen = 0;
    let mut best = 0;
    let mut children = vec![];
    for slot in 12..=end {
        let id = roster(e, s.side, slot);
        let unavailable = available(e, id);
        children.push(readonly(
            e,
            "1B55:505D",
            "available",
            json!([id]),
            u16::from(unavailable),
        ));
        if unavailable != 0 {
            continue;
        }
        let mut weight =
            u16::from(e.image[0xDA6 + 14 * usize::from(role) + usize::from(e.image[p8(id) + 7])]);
        if pos != 0 {
            let hand = scene_assignment::hand(e, id);
            children.push(readonly(
                e,
                "1B55:50A5",
                "handedness",
                json!([id]),
                u16::from(hand),
            ));
            if hand != u8::from(pos > 3) {
                weight >>= 1;
            }
        }
        if weight > best {
            best = weight;
            chosen = slot;
        }
    }
    (chosen, children)
}
fn exchange(e: &mut Engine, a: usize, b: usize, n: usize) {
    for i in 0..n {
        e.image.swap(a + i, b + i);
    }
}
// [designed] Standalone slices retain typed adapter events;the continuing image owns0F94 state.
pub(crate) fn run(e: &mut Engine, s: &State) -> Result<Outcome, String> {
    run_image(e, s, false)
}
pub(crate) fn run_with_commentary(e: &mut Engine, s: &State) -> Result<Outcome, String> {
    run_image(e, s, true)
}
fn run_image(e: &mut Engine, s: &State, compose_commentary: bool) -> Result<Outcome, String> {
    let side = usize::from(s.side);
    let id = word(e, addr(s.scene) + 14);
    let role = e.image[p8(id) + 6];
    let slot = s.scene - e.image[0xD56 + side];
    let special = e.image[0xD6DD] != 0 && role == 0 && e.image[0xD6DE + side] == 0;
    let capacity = u16::from(e.image[0xD6E0 + side]) + u16::from(special);
    let mut helpers = vec![];
    let mut presentation = vec![];
    if capacity == 0 {
        let roster_id = roster(e, s.side, u16::from(slot));
        if (e.image[p15(roster_id) + 8] as i8) < 1 {
            return Err("unsupported reachedA837 atA9D1".into());
        }
    } else {
        e.image[0xD076..0xD084].fill(0);
        let before = candidates(e);
        let children = scan(e, s);
        helpers.push(trace(
            "0AB9:AA02",
            "candidates",
            json!([]),
            before,
            candidates(e),
            children,
            None,
        ));
        let (mut chosen, children) = preferred(e, s);
        let before = candidates(e);
        helpers.push(trace(
            "0AB9:AA13",
            "preferred",
            json!([]),
            before.clone(),
            before,
            children,
            Some(chosen),
        ));
        if chosen == 0 && (s.mode != 0 || (role == 0 && e.image[0xD6E0 + side] != 0)) {
            let end = limit(e);
            helpers.push(readonly(e, "0AB9:AA4F", "limit", json!([]), end));
            let (value, children) = weighted(e, s, role, end);
            chosen = value;
            let before = candidates(e);
            helpers.push(trace(
                "0AB9:AA6D",
                "weighted",
                json!([role, end]),
                before.clone(),
                before,
                children,
                Some(chosen),
            ));
        }
        if chosen == 0 {
            if role == 0 {
                return Err("unsupported reachedA837 atAA9B".into());
            }
        } else {
            let extra = s
                .replacement
                .as_ref()
                .ok_or("replacement reached without full raw read set")?;
            let incoming = roster(e, s.side, chosen);
            e.image[p8(incoming) + 6] = role;
            let base = club(word(e, 0xD6E4 + 2 * side));
            exchange(
                e,
                base + 2 * usize::from(chosen - 1),
                base + 2 * usize::from(slot - 1),
                2,
            );
            exchange(
                e,
                0xD146 + 40 * side + 2 * usize::from(chosen),
                0xD146 + 40 * side + 2 * usize::from(slot),
                2,
            );
            exchange(
                e,
                0xD197 + 20 * side + usize::from(chosen),
                0xD197 + 20 * side + usize::from(slot),
                1,
            );
            let input = snapshot(e, &s.context);
            let before = candidates(e);
            let r = replacement::State {
                match_state: extra.match_state.clone(),
                incoming: chosen as u8,
                index: s.scene,
                side: s.side,
                allowance: e.image[0xD6DD],
                sub_flags: [e.image[0xD6DE], e.image[0xD6DF]],
                reserves: [e.image[0xD6E0], e.image[0xD6E1]],
                mode: extra.display_mode,
                position_sentinel: extra.position_sentinel,
                speaker_gate: extra.speaker_gate,
            };
            let out = if compose_commentary {
                replacement::run_with_commentary(e, &r)?
            } else {
                replacement::run(e, &r)?
            };
            presentation = out.presentation;
            let mut h = trace(
                "0AB9:ABAF",
                "replace",
                json!([chosen, s.scene, s.side]),
                before,
                candidates(e),
                vec![],
                None,
            );
            h["input"] = serde_json::to_value(input).unwrap();
            h["output"] = serde_json::to_value(snapshot(e, &s.context)).unwrap();
            helpers.push(h);
        }
    }
    let replacement = s.replacement.as_ref().map(|r| ReadSet {
        match_state: rearrangement::snapshot(e, &r.match_state),
        display_mode: e.image[0xD074],
        position_sentinel: word(e, 0xD2E),
        speaker_gate: e.image[0xA2EC],
    });
    Ok(Outcome {
        output: snapshot(e, &s.context),
        replacement,
        helpers,
        presentation,
        draws: vec![],
    })
}
pub fn select(s: &State) -> Result<Outcome, String> {
    run(&mut image(cells(s)?), s)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample(call: usize) -> State {
        let c: Value = serde_json::from_str(
            include_str!("../tests/fixtures/ai_selection/cases.jsonl")
                .lines()
                .nth(call)
                .unwrap(),
        )
        .unwrap();
        serde_json::from_value(c["input"].clone()).unwrap()
    }
    fn eligible(e: &mut Engine, s: &State) {
        for slot in 12..=14 {
            let id = roster(e, s.side, slot);
            e.image[p15(id) + 8] = 1;
            e.image[p15(id) + 11] = 0;
        }
    }
    fn hand(e: &mut Engine, id: u16, style: u8, value: u8) {
        e.image[p8(id) + 7] = style;
        let selector = e.image[0x12BA + usize::from(id % 3)];
        e.image[0x129E + 14 * usize::from(selector) + usize::from(style)] = value;
    }
    #[test]
    fn candidate_last_overwrite_unavailable_and_normalized_current_exclusion() {
        let s = sample(0);
        let mut e = image(cells(&s).unwrap());
        eligible(&mut e, &s);
        for (slot, style) in [(12, 2), (13, 2), (14, 3)] {
            let id = roster(&e, s.side, slot);
            e.image[p8(id) + 7] = style;
        }
        e.image[0xD076..0xD084].fill(0);
        let t = scan(&mut e, &s);
        assert_eq!((e.image[0xD078], e.image[0xD079]), (13, 14));
        assert_eq!(t.len(), 4);
        let id = roster(&e, s.side, 14);
        let normalized = s.scene - e.image[0xD56 + usize::from(s.side)];
        e.image[addr(normalized) + 14..addr(normalized) + 16].copy_from_slice(&id.to_le_bytes());
        e.image[0xD076..0xD084].fill(0);
        let t = scan(&mut e, &s);
        assert_eq!(e.image[0xD079], 0);
        assert_eq!(t.len(), 3);
        let id = roster(&e, s.side, 12);
        e.image[p15(id) + 11] = 4;
        let t = scan(&mut e, &s);
        assert_eq!(t[1]["result"], 1);
    }
    #[test]
    fn offset_exclusion_reads_normalized_scene_as_listing_a8d8_requires() {
        let mut s = sample(0);
        s.side = 0;
        s.scene = 19;
        let mut e = image(cells(&s).unwrap());
        eligible(&mut e, &s);
        let id = roster(&e, 0, 12);
        e.image[addr(8) + 14..addr(8) + 16].copy_from_slice(&id.to_le_bytes());
        assert_ne!(word(&e, addr(19) + 14), id);
        let t = scan(&mut e, &s);
        assert_eq!(t.len(), 3);
        assert!(!t.iter().any(|h| h["arguments"] == json!([id])));
    }
    #[test]
    fn preferred_retains_last_nonzero_candidate_not_first() {
        let s = sample(0);
        let mut e = image(cells(&s).unwrap());
        let id = word(&e, addr(s.scene) + 14);
        e.image[p8(id) + 6] = 2;
        e.image[0xD9A] = 0;
        let a = 0xBF6 + 16;
        e.image[a..a + 4].copy_from_slice(&[3, 2, 3, 4]);
        e.image[0xD078..0xD07B].copy_from_slice(&[12, 13, 14]);
        assert_eq!(preferred(&e, &s).0, 14);
        e.image[0xD07A] = 0;
        assert_eq!(preferred(&e, &s).0, 13);
    }
    #[test]
    fn pascal_search_includes_nul_and_raw_hand_must_equal_predicate() {
        let s = sample(0);
        let mut e = image(cells(&s).unwrap());
        let id = word(&e, addr(s.scene) + 14);
        let reserve = roster(&e, s.side, 12);
        e.image[0xD9A] = 4;
        e.image[0xD9B..0xD9F].copy_from_slice(&[0, 3, 0, 4]);
        assert_eq!(search(&e, 0), 1);
        assert_eq!(search(&e, 3), 2);
        assert_eq!(search(&e, 4), 4);
        assert_eq!(search(&e, 13), 0);
        e.image[0xD07B] = 12; // candidate style5
        for (role, h, want) in [(3, 0, 12), (3, 255, 0), (4, 1, 12), (4, 255, 0)] {
            e.image[p8(id) + 6] = role;
            let a = 0xBF6 + 8 * usize::from(role);
            e.image[a] = 1;
            e.image[a + 1] = 5;
            hand(&mut e, reserve, 5, h);
            let (chosen, children) = preferred(&e, &s);
            assert_eq!(chosen, want);
            assert_eq!(children.len(), 1);
            assert_eq!(children[0]["result"], h);
        }
    }
    #[test]
    fn weighted_strict_ties_skip_unavailable_and_zero_is_no_choice() {
        let s = sample(0);
        let mut e = image(cells(&s).unwrap());
        eligible(&mut e, &s);
        e.image[0xD9A] = 0;
        for (slot, style) in [(12, 5), (13, 6), (14, 7)] {
            let id = roster(&e, s.side, slot);
            e.image[p8(id) + 7] = style;
        }
        let a = 0xDA6 + 28;
        e.image[a + 5..a + 8].copy_from_slice(&[9, 9, 0]);
        assert_eq!(weighted(&e, &s, 2, 14).0, 12);
        let id = roster(&e, s.side, 12);
        e.image[p15(id) + 8] = 128;
        assert_eq!(weighted(&e, &s, 2, 14).0, 13);
        e.image[a + 6] = 0;
        assert_eq!(weighted(&e, &s, 2, 14).0, 0);
    }
    #[test]
    fn weighted_halves_on_raw_hand_mismatch_using_search_index_above_three() {
        let s = sample(0);
        let mut e = image(cells(&s).unwrap());
        eligible(&mut e, &s);
        e.image[0xD9A] = 4;
        e.image[0xD9B..0xD9F].copy_from_slice(&[9, 9, 9, 2]);
        for (slot, style, h) in [(12, 5, 0), (13, 6, 1), (14, 7, 0)] {
            let id = roster(&e, s.side, slot);
            hand(&mut e, id, style, h);
        }
        let a = 0xDA6 + 28;
        e.image[a + 5..a + 8].copy_from_slice(&[255, 254, 0]);
        let (chosen, t) = weighted(&e, &s, 2, 14);
        assert_eq!(chosen, 13);
        assert_eq!(t.iter().filter(|h| h["kind"] == "handedness").count(), 3);
    }
    #[test]
    fn capacity_gate_uses_signed_byte_only_and_special_keeper_capacity_reaches_later_fallback() {
        let s = sample(0);
        let mut e = image(cells(&s).unwrap());
        let id = word(&e, addr(s.scene) + 14);
        let roster_id = roster(&e, s.side, u16::from(s.scene));
        e.image[0xD6E1] = 0;
        e.image[0xD6DD] = 0;
        e.image[p8(id) + 6] = 2;
        e.image[p15(roster_id) + 8] = 1;
        e.image[p15(roster_id) + 11] = 128;
        assert!(run(&mut e, &s).unwrap().helpers.is_empty());
        e.image[p15(roster_id) + 8] = 0;
        assert!(run(&mut e, &s).unwrap_err().contains("A9D1"));
        e.image[p15(roster_id) + 8] = 1;
        e.image[p8(id) + 6] = 0;
        e.image[0xD6DD] = 1;
        e.image[0xD6DF] = 0;
        for slot in 12..=14 {
            let id = roster(&e, s.side, slot);
            e.image[p15(id) + 8] = 0;
        }
        assert!(run(&mut e, &s).unwrap_err().contains("AA9B"));
        e.image[0xD6DF] = 1;
        assert!(run(&mut e, &s).unwrap().helpers.is_empty());
    }
    #[test]
    fn reached_replacement_requires_raw_read_set_without_a_chosen_slot_input() {
        let mut s = sample(2);
        s.replacement = None;
        let before = serde_json::to_value(&s).unwrap();
        assert!(select(&s)
            .unwrap_err()
            .contains("without full raw read set"));
        assert_eq!(serde_json::to_value(&s).unwrap(), before);
    }
    #[test]
    fn replacement_uses_live_counter_mutation_without_reloading_recorded_state() {
        let s = sample(2);
        let mut e = image(cells(&s).unwrap());
        let id = roster(&e, s.side, 14);
        e.image[p15(id)..p15(id) + 2].copy_from_slice(&65535u16.to_le_bytes());
        let out = run(&mut e, &s).unwrap();
        assert_eq!(word(&e, p15(id)), 0);
        let p = out
            .replacement
            .unwrap()
            .match_state
            .core
            .players
            .into_iter()
            .find(|p| p.id == id)
            .unwrap();
        assert_eq!(&p.p15[..2], &[0, 0]);
        assert_eq!(out.helpers.last().unwrap()["arguments"], json!([14, 8, 1]));
        assert!(!out.presentation.is_empty());
    }
    #[test]
    fn canonical_table_extra_output_and_index_domains_reject_before_allocation() {
        let s = sample(0);
        let mut bad = vec![];
        let mut b = s.clone();
        b.side = 2;
        bad.push(b);
        let mut b = s.clone();
        b.scene = 23;
        bad.push(b);
        let mut b = s.clone();
        b.scene = 12;
        bad.push(b);
        let mut b = s.clone();
        b.context.side_offsets = [255, 0];
        bad.push(b);
        let mut b = s.clone();
        b.context.players.pop();
        bad.push(b);
        let mut b = s.clone();
        b.context.players[0].position = 14;
        bad.push(b);
        let mut b = s.clone();
        b.context.players[0].style = 14;
        bad.push(b);
        let mut b = s.clone();
        b.context.scenes[0].slot = 0;
        bad.push(b);
        let mut b = s.clone();
        b.context.clubs[0].id = 0;
        bad.push(b);
        let mut b = s.clone();
        b.context.preferences.pop();
        bad.push(b);
        let mut b = s.clone();
        b.context.preferences[0].styles = vec![0; 8];
        bad.push(b);
        let mut b = s.clone();
        b.context.style_text = "☃".into();
        bad.push(b);
        let mut b = s.clone();
        b.context.style_text = "A".repeat(20);
        bad.push(b);
        let mut b = s.clone();
        b.context.allowance = 255;
        bad.push(b);
        let mut b = s.clone();
        b.context.candidates[0] = 11;
        bad.push(b);
        let mut b = sample(2);
        b.replacement.as_mut().unwrap().match_state.core.players[0].p8[6] ^= 1;
        bad.push(b);
        // Two canonical players sharing ID modulo3 must name the same raw selector.
        let mut b = s.clone();
        let i = (1..b.context.players.len())
            .find(|i| b.context.players[*i].id % 3 == b.context.players[0].id % 3)
            .unwrap();
        b.context.players[i].hand_selector ^= 1;
        bad.push(b);
        for b in bad {
            assert!(cells(&b).is_err());
        }
        for field in [
            "output", "expected", "helpers", "chosen", "result", "rating",
        ] {
            let mut v = serde_json::to_value(&s).unwrap();
            v[field] = json!(14);
            assert!(serde_json::from_value::<State>(v).is_err());
            let mut v = serde_json::to_value(&s).unwrap();
            v["context"][field] = json!(14);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
    }
    #[test]
    fn standalone_selection_only_writes_candidate_cells() {
        let s = sample(0);
        let mut e = image(cells(&s).unwrap());
        let before = e.image.clone();
        run(&mut e, &s).unwrap();
        for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
            if old != new {
                assert!((0xD076..0xD084).contains(&a), "unexpected write {a:X}");
            }
        }
    }
    #[test]
    fn composed_selection_preserves_seed_and_only_adds_incoming_role_write_to_replacement_ownership(
    ) {
        let s = sample(2);
        let mut e = image(cells(&s).unwrap());
        e.image[0x8000] = 0xAB;
        let before = e.image.clone();
        let incoming = roster(&e, s.side, 14);
        let outgoing = word(&e, addr(s.scene) + 14);
        let readset = s.replacement.as_ref().unwrap();
        run(&mut e, &s).unwrap();
        for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
            if old == new {
                continue;
            }
            let scene = (1..=22).any(|i| a >= addr(i) && a < addr(i) + 25);
            let roster = readset
                .match_state
                .core
                .assignment
                .clubs
                .iter()
                .any(|c| a >= club(c.id) && a < club(c.id) + 40);
            let cache = (0..3).any(|i| a == 0xCF32 + 7 * i || a == 0xCF33 + 7 * i)
                || (0xCF5E..0xCF72).contains(&a);
            let globals = (0xD076..0xD084).contains(&a)
                || [0xD09F, 0xD0A0, 0xD0A2, 0xD0A3, 0xD0A4, 0xD0A5].contains(&a)
                || (0xD12A..0xD12E).contains(&a)
                || (0xD6DE..0xD6E2).contains(&a)
                || (0xD148..0xD198).contains(&a)
                || (0xD198..0xD1C0).contains(&a);
            let local = (0x40000..0x4000B).contains(&a)
                || (0x40010..0x4001B).contains(&a)
                || (0x40020..0x4002B).contains(&a);
            let player = a == p8(incoming) + 6
                || a == p15(outgoing) + 11
                || [0, 1, 4, 6, 10].iter().any(|i| a == p15(incoming) + i);
            assert!(
                scene || roster || cache || globals || local || player,
                "unexpected write {a:X}"
            );
        }
        assert_eq!(word(&e, 0xA51C), s.context.seed);
        assert_eq!(e.image[0x8000], 0xAB);
    }
}
