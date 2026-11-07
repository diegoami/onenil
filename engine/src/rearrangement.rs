//! [confirmed] F28–F35 at archaeology218490d: fixed-match164F shared-state closure.
use crate::{player_cache, role_pairing, scene_assignment, scene_codes, terminal, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8: [u8; 8],
    pub p15: [u8; 15],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Core {
    pub assignment: scene_assignment::State,
    pub players: Vec<Player>,
    pub rosters: [[u16; 20]; 2],
    pub side_offsets: [u8; 2],
    pub actor: u8,
    pub secondary: u8,
    pub cpu_class: u8,
    pub club_totals: [u16; 2],
    pub event_totals: [[u16; 20]; 2],
    pub cache_selectors: [[u8; 20]; 2],
    pub cached_players: [u16; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub core: Core,
    pub pairing: role_pairing::State,
    pub codes: scene_codes::State,
    pub cache: player_cache::State,
    pub terminal: terminal::State,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub events: Vec<Value>,
    pub draws: Vec<Draw>,
}
use crate::{
    placement,
    role_scores::{image, insert, Cells},
    Engine,
};
use serde_json::json;
use std::collections::BTreeSet;
fn addr(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
}
fn club(id: u16) -> usize {
    crate::layout::CLUB + 62 * usize::from(id - 1)
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes());
}
fn cw(c: &mut Cells, a: usize, v: u16) -> Result<(), String> {
    for (i, b) in v.to_le_bytes().into_iter().enumerate() {
        insert(c, a + i, b)?;
    }
    Ok(())
}
fn bytes(c: &mut Cells, a: usize, v: &[u8]) -> Result<(), String> {
    for (i, b) in v.iter().enumerate() {
        insert(c, a + i, *b)?;
    }
    Ok(())
}
fn ids<I: Iterator<Item = u16>>(it: I) -> BTreeSet<u16> {
    it.collect()
}
// [derived] Reconcile every redundant component and raw table before allocating one image.
pub(crate) fn cells(s: &State) -> Result<Cells, String> {
    load_cells(s, false)
}
pub(crate) fn fresh_source_cells(s: &State) -> Result<Cells, String> {
    load_cells(s, true)
}
fn load_cells(s: &State, fresh: bool) -> Result<Cells, String> {
    let r = &s.core;
    if !fresh {
        if !(1..=22).contains(&r.actor)
            || r.secondary > 22
            || r.assignment.index != 1
            || r.assignment.selector != 1
        {
            return Err(
                "unsupported164F actor/secondary or noncanonical assignment metadata".into(),
            );
        }
        role_pairing::validate(&s.pairing)?;
        scene_codes::validate(&s.codes)?;
        terminal::validate(&s.terminal)?;
    }
    let mut c = if fresh {
        scene_assignment::fresh_source_cells(&r.assignment)?
    } else {
        scene_assignment::cells(&r.assignment)?
    };
    for (a, b) in player_cache::cells(&s.cache)? {
        insert(&mut c, a, b)?;
    }
    let canonical = ids(r.assignment.players.iter().map(|p| p.id));
    for other in [
        ids(r.players.iter().map(|p| p.id)),
        ids(s.pairing.players.iter().map(|p| p.id)),
        ids(s.codes.players.iter().map(|p| p.id)),
        ids(s.cache.players.iter().map(|p| p.id)),
        ids(s.terminal.players.iter().map(|p| p.id)),
    ] {
        if other != canonical {
            return Err("164F canonical component identities differ".into());
        }
    }
    if r.players.len() != canonical.len() {
        return Err("duplicate164F full canonical player".into());
    }
    for p in &r.players {
        bytes(&mut c, crate::layout::P8 + 8 * usize::from(p.id), &p.p8)?;
        bytes(&mut c, crate::layout::P15 + 15 * usize::from(p.id), &p.p15)?;
    }
    for (side, h) in r.assignment.clubs.iter().enumerate() {
        for (i, id) in r.rosters[side].iter().enumerate() {
            if !canonical.contains(id) {
                return Err("164F full roster missing player".into());
            }
            cw(&mut c, club(h.id) + 2 * i, *id)?;
        }
        insert(&mut c, 0xD56 + side, r.side_offsets[side])?;
        cw(&mut c, 0xD12A + 2 * side, r.club_totals[side])?;
        cw(&mut c, 0xCF5E + 10 * side, r.cached_players[side])?;
        for i in 0..20 {
            cw(&mut c, 0xD148 + 40 * side + 2 * i, r.event_totals[side][i])?;
            insert(&mut c, 0xD198 + 20 * side + i, r.cache_selectors[side][i])?;
        }
    }
    insert(&mut c, 0xD09F, r.actor)?;
    insert(&mut c, 0xD0A0, r.secondary)?;
    insert(&mut c, 0xA52A, r.cpu_class)?;
    let p = &s.pairing;
    cw(&mut c, 0xA51C, p.seed)?;
    bytes(&mut c, 0xD56, &p.side_offsets)?;
    for (side, h) in p.clubs.iter().enumerate() {
        cw(&mut c, 0xD6E4 + 2 * side, h.id)?;
        insert(&mut c, club(h.id) + 57, h.byte57)?;
        for (i, id) in h.roster.iter().enumerate() {
            cw(&mut c, club(h.id) + 2 * i, *id)?;
        }
    }
    for r in &p.scenes {
        bytes(&mut c, addr(r.index), &r.bytes())?;
    }
    for p in &p.players {
        insert(
            &mut c,
            crate::layout::P8 + 8 * usize::from(p.id) + 6,
            p.role,
        )?;
        insert(
            &mut c,
            crate::layout::P15 + 15 * usize::from(p.id) + 8,
            p.byte8,
        )?;
        insert(
            &mut c,
            crate::layout::P15 + 15 * usize::from(p.id) + 11,
            p.flags11,
        )?;
    }
    for (pass, row) in p.preferences.iter().enumerate() {
        bytes(&mut c, 0x09B0 + 14 * (pass + 1) + 1, row)?;
    }
    let p = &s.codes;
    cw(&mut c, 0xA51C, p.seed)?;
    bytes(&mut c, 0xD56, &p.side_offsets)?;
    insert(&mut c, 0xD09F, p.actor)?;
    insert(&mut c, crate::layout::SCENE_MODE, p.mode)?;
    insert(&mut c, 0xD0AC, p.threshold)?;
    cw(&mut c, 0xD0AE, p.clock)?;
    bytes(&mut c, 0xA338, &p.direction_signs)?;
    bytes(&mut c, 0xD0C8, &p.directions)?;
    bytes(&mut c, 0x096A, &p.codes)?;
    bytes(&mut c, 0x8716, &p.role_codes)?;
    for r in &p.scenes {
        bytes(&mut c, addr(r.index), &r.bytes())?;
    }
    for p in &p.players {
        insert(
            &mut c,
            crate::layout::P8 + 8 * usize::from(p.id) + 6,
            p.role,
        )?;
    }
    for cache in &p.cache {
        for (offset, value) in [(0, cache.word0), (2, cache.word2), (4, cache.word4)] {
            cw(
                &mut c,
                0xCF2E + 7 * usize::from(cache.index) + offset,
                value,
            )?;
        }
    }
    let p = &s.terminal;
    cw(&mut c, 0xA51C, p.seed)?;
    bytes(&mut c, 0xD56, &p.side_offsets)?;
    insert(&mut c, 0xD09F, p.actor)?;
    insert(&mut c, 0xD0AC, p.phase)?;
    cw(&mut c, 0xD0AE, p.time)?;
    bytes(&mut c, 0xA338, &p.signs)?;
    bytes(&mut c, 0xCF46, &p.target)?;
    bytes(&mut c, 0xD0A4, &p.selectors.near)?;
    bytes(&mut c, 0xD0A2, &p.selectors.ahead)?;
    for p in &p.positions {
        cw(&mut c, 0xA3A + 2 * usize::from(p.position), p.word)?;
    }
    for r in &p.scenes {
        bytes(&mut c, addr(r.index), &r.bytes())?;
    }
    for p in &p.players {
        insert(
            &mut c,
            crate::layout::P8 + 8 * usize::from(p.id) + 6,
            p.position,
        )?;
        insert(
            &mut c,
            crate::layout::P15 + 15 * usize::from(p.id) + 8,
            p.byte8,
        )?;
        insert(
            &mut c,
            crate::layout::P15 + 15 * usize::from(p.id) + 11,
            p.flags11,
        )?;
    }
    Ok(c)
}
pub(crate) fn core(e: &Engine, s: &Core) -> Core {
    let mut out = s.clone();
    out.assignment = scene_assignment::snapshot(e, &s.assignment);
    for p in &mut out.players {
        p.p8.copy_from_slice(
            &e.image[crate::layout::P8 + 8 * usize::from(p.id)
                ..crate::layout::P8 + 8 * usize::from(p.id) + 8],
        );
        p.p15.copy_from_slice(
            &e.image[crate::layout::P15 + 15 * usize::from(p.id)
                ..crate::layout::P15 + 15 * usize::from(p.id) + 15],
        );
    }
    out.actor = e.image[0xD09F];
    out.secondary = e.image[0xD0A0];
    out.cpu_class = e.image[0xA52A];
    for side in 0..2 {
        out.side_offsets[side] = e.image[0xD56 + side];
        out.club_totals[side] = word(e, 0xD12A + 2 * side);
        out.cached_players[side] = word(e, 0xCF5E + 10 * side);
        for i in 0..20 {
            out.rosters[side][i] = word(e, club(out.assignment.clubs[side].id) + 2 * i);
            out.event_totals[side][i] = word(e, 0xD148 + 40 * side + 2 * i);
            out.cache_selectors[side][i] = e.image[0xD198 + 20 * side + i];
        }
    }
    out
}
pub(crate) fn snapshot(e: &mut Engine, s: &State) -> State {
    let mut out = s.clone();
    out.core = core(e, &s.core);
    out.codes = scene_codes::snapshot(e, &s.codes);
    out.terminal = terminal::snapshot(e, &s.terminal);
    out.pairing.seed = out.core.assignment.seed;
    out.pairing.side_offsets = out.core.side_offsets;
    out.pairing.scenes = out.core.assignment.scenes.clone();
    for (side, c) in out.pairing.clubs.iter_mut().enumerate() {
        c.id = out.core.assignment.clubs[side].id;
        c.roster = out.core.assignment.clubs[side].roster;
        c.byte57 = e.image[club(c.id) + 57];
    }
    for p in &mut out.pairing.players {
        p.role = e.image[crate::layout::P8 + 8 * usize::from(p.id) + 6];
        p.byte8 = e.image[crate::layout::P15 + 15 * usize::from(p.id) + 8];
        p.flags11 = e.image[crate::layout::P15 + 15 * usize::from(p.id) + 11];
    }
    out.cache.seed = out.core.assignment.seed;
    out.cache.gate = e.image[0xD18];
    out.cache.side_offsets = out.core.side_offsets;
    out.cache.scenes = out.core.assignment.scenes.clone();
    for side in 0..2 {
        out.cache.clubs[side] = out.core.assignment.clubs[side].id;
        for i in 0..5 {
            out.cache.cache[side][i] = word(e, 0xCF5E + 10 * side + 2 * i);
        }
    }
    for p in &mut out.cache.players {
        let a = crate::layout::P8 + 8 * usize::from(p.id);
        let b = crate::layout::P15 + 15 * usize::from(p.id);
        p.club = e.image[a];
        p.p8_1 = e.image[a + 1];
        p.p8_3 = e.image[a + 3];
        p.p8_4 = e.image[a + 4];
        p.p8_5 = e.image[a + 5];
        p.p8_7 = e.image[a + 7];
        p.byte6 = e.image[b + 6];
        p.byte8 = e.image[b + 8];
        p.flags11 = e.image[b + 11];
    }
    for r in &mut out.cache.rosters {
        for (i, id) in r.roster.iter_mut().enumerate() {
            *id = word(e, club(r.club) + 2 * i);
        }
    }
    out
}
fn selectors(e: &Engine) -> Value {
    json!({"actor":e.image[0xD09F],"secondary":e.image[0xD0A0]})
}
fn totals(e: &Engine) -> Value {
    json!({"actor":e.image[0xD09F],"secondary":e.image[0xD0A0],"club_totals":[word(e,0xD12A),word(e,0xD12C)]})
}
fn event(site: &str, kind: &str, args: Value, input: Value, output: Value, result: Value) -> Value {
    json!({"site":site,"kind":kind,"arguments":args,"input":input,"output":output,"result":result,"draw_start":0,"draw_end":0})
}
fn unavailable(e: &Engine, id: u16) -> u8 {
    let a = crate::layout::P15 + 15 * usize::from(id);
    placement::unavailable(&placement::Player {
        id,
        p15_sbyte8: e.image[a + 8] as i8,
        p15_byte11: e.image[a + 11],
    })
}
fn availability(e: &Engine, base: usize, id: u16) -> Value {
    let a = crate::layout::P8 + 8 * usize::from(id);
    let b = crate::layout::P15 + 15 * usize::from(id);
    json!({"actor":e.image[0xD09F],"secondary":e.image[0xD0A0],"club_totals":[word(e,0xD12A),word(e,0xD12C)],"roster":(0..20).map(|i|word(e,base+2*i)).collect::<Vec<_>>(),"player":id,"p8_4":e.image[a+4],"position":e.image[a+6],"byte8":e.image[b+8],"flags11":e.image[b+11]})
}
// [confirmed] Logical26C9:0A67 signed32 quotient/remainder. CPU-specific instruction path normalized.
fn divide(a: i32, b: i32) -> Result<(i32, i32), String> {
    Ok((
        a.checked_div(b)
            .ok_or("unsupported division zero/overflow")?,
        a.checked_rem(b)
            .ok_or("unsupported division zero/overflow")?,
    ))
}
fn scene_value(e: &Engine, index: u8) -> Value {
    let mut v = serde_json::to_value(terminal::Scene::read(e, index)).unwrap();
    v.as_object_mut().unwrap().remove("index");
    v
}
fn exchange(e: &mut Engine, a: usize, b: usize, n: usize) {
    for i in 0..n {
        e.image.swap(a + i, b + i);
    }
}
fn pair_value(e: &Engine, kind: &str, a: usize, b: usize, n: usize) -> Value {
    if kind == "scenes" {
        json!({"a":scene_value(e,((a-0xCCEF)/25)as u8),"b":scene_value(e,((b-0xCCEF)/25)as u8)})
    } else if n == 2 {
        json!({"a":word(e,a),"b":word(e,b)})
    } else {
        json!({"a":e.image[a],"b":e.image[b]})
    }
}
fn prefix(e: &mut Engine, s: &State) -> Result<Vec<Value>, String> {
    let mut events = vec![];
    for side in 0..2 {
        let header = word(e, 0xD6E4 + 2 * side);
        let base = club(header);
        let mut sum = 0i32;
        for slot in 1..=11 {
            let id = word(e, base + 2 * (slot - 1));
            let value = unavailable(e, id);
            let observation = availability(e, base, id);
            events.push(event(
                "0AB9:16B6",
                "sum-available",
                json!([side, slot, id]),
                observation.clone(),
                observation,
                json!(value),
            ));
            if value == 0 {
                sum += i32::from(e.image[crate::layout::P8 + 8 * usize::from(id) + 4]);
            }
        }
        let (quotient, remainder) = divide(sum, 22)?;
        let observation = totals(e);
        events.push(event(
            "0AB9:16E1",
            "division",
            json!([side, sum, 22]),
            observation.clone(),
            observation,
            json!({"quotient":quotient,"remainder":remainder}),
        ));
        put(
            e,
            0xD12A + 2 * (1 - side),
            (quotient as u16).wrapping_add(1),
        );
        let mut roles = [0u8; 11];
        for slot in 1..=11 {
            let id = word(e, base + 2 * (slot - 1));
            let value = unavailable(e, id);
            let observation = availability(e, base, id);
            events.push(event(
                "0AB9:174A",
                "role-available",
                json!([side, slot, id]),
                observation.clone(),
                observation,
                json!(value),
            ));
            roles[slot - 1] = if value != 0 {
                14
            } else {
                e.image[crate::layout::P8 + 8 * usize::from(id) + 6]
            };
        }
        let offset = e.image[0xD56 + side];
        for low in 1..=10 {
            for high in low + 1..=11 {
                if roles[high - 1] < roles[low - 1] {
                    let lo = offset + low as u8;
                    let hi = offset + high as u8;
                    for (kind, site, a, b, n) in [
                        (
                            "roster",
                            "0AB9:17C0",
                            base + 2 * (high - 1),
                            base + 2 * (low - 1),
                            2,
                        ),
                        ("roles", "0AB9:17D7", high - 1, low - 1, 1),
                        ("scenes", "0AB9:180E", addr(hi), addr(lo), 25),
                        (
                            "events",
                            "0AB9:1841",
                            0xD146 + 40 * side + 2 * high,
                            0xD146 + 40 * side + 2 * low,
                            2,
                        ),
                        (
                            "cache",
                            "0AB9:186C",
                            0xD197 + 20 * side + high,
                            0xD197 + 20 * side + low,
                            1,
                        ),
                    ] {
                        let sel = selectors(e);
                        let before;
                        let after;
                        if kind == "roles" {
                            before = json!({"a":roles[a],"b":roles[b]});
                            roles.swap(a, b);
                            after = json!({"a":roles[a],"b":roles[b]});
                        } else {
                            before = pair_value(e, kind, a, b, n);
                            exchange(e, a, b, n);
                            after = pair_value(e, kind, a, b, n);
                        }
                        events.push(json!({"site":site,"kind":kind,"arguments":[side,low,high,n],"input":before,"output":after,"selectors":sel,"selectors_after":selectors(e),"draw_start":0,"draw_end":0}));
                    }
                    let before = selectors(e);
                    for a in [0xD09F, 0xD0A0] {
                        e.image[a] = if e.image[a] == hi {
                            lo
                        } else if e.image[a] == lo {
                            hi
                        } else {
                            e.image[a]
                        };
                    }
                    events.push(json!({"site":"0AB9:1925","kind":"selectors","arguments":[side,low,high],"input":before,"output":selectors(e),"draw_start":0,"draw_end":0}));
                }
            }
        }
        for index in offset + 1..=offset + 11 {
            let before = serde_json::to_value(core(e, &s.core)).unwrap();
            scene_assignment::assign_image(e, index, index)?;
            events.push(event(
                "0AB9:1971",
                "assign",
                json!([side, index, index]),
                before,
                serde_json::to_value(core(e, &s.core)).unwrap(),
                Value::Null,
            ));
        }
    }
    Ok(events)
}
pub(crate) fn run(e: &mut Engine, s: &State) -> Result<Outcome, String> {
    let mut events = prefix(e, s)?;
    for site in [
        "0AB9:1985",
        "0AB9:1988",
        "0AB9:198B",
        "0AB9:198E",
        "0AB9:1991",
    ] {
        let before = serde_json::to_value(core(e, &s.core)).unwrap();
        match site {
            "0AB9:1985" => {
                role_pairing::pair_image(e);
            }
            "0AB9:1988" => {
                scene_codes::refresh_image(e, &s.codes)?;
            }
            "0AB9:198B" => {
                player_cache::run(e, &s.cache)?;
            }
            "0AB9:198E" => {
                terminal::positions_image(e, &s.terminal);
            }
            "0AB9:1991" => {
                terminal::proximity_image(e, &s.terminal);
            }
            _ => unreachable!(),
        }
        events.push(event(
            site,
            "opaque",
            json!([]),
            before,
            serde_json::to_value(core(e, &s.core)).unwrap(),
            Value::Null,
        ));
    }
    put(e, addr(1) + 12, 0);
    put(e, addr(12) + 12, 0);
    Ok(Outcome {
        output: snapshot(e, s),
        events,
        draws: vec![],
    })
}
pub fn rearrange(s: &State) -> Result<Outcome, String> {
    run(&mut image(cells(s)?), s)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> State {
        let c: Value = serde_json::from_str(
            include_str!("../tests/fixtures/rearrangement/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        serde_json::from_value(c["input"].clone()).unwrap()
    }
    // [designed] Static branches via live-image mutations; these are not new original observations.
    fn ordinary(e: &mut Engine) {
        for side in 0..2 {
            let base = club(word(e, 0xD6E4 + 2 * side));
            for i in 0..11 {
                let id = word(e, base + 2 * i);
                let p = crate::layout::P8 + 8 * usize::from(id);
                let q = crate::layout::P15 + 15 * usize::from(id);
                e.image[p + 6] = i as u8;
                e.image[q + 8] = 1;
                e.image[q + 11] = 0;
            }
        }
    }
    #[test]
    fn strict_equal_roles_do_not_exchange_and_unsigned_byte_sum_sets_opposite_total() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        ordinary(&mut e);
        for side in 0..2 {
            let base = club(word(&e, 0xD6E4 + 2 * side));
            for i in 0..11 {
                let id = word(&e, base + 2 * i);
                e.image[crate::layout::P8 + 8 * usize::from(id) + 6] = 3;
                e.image[crate::layout::P8 + 8 * usize::from(id) + 4] =
                    if side == 0 { 255 } else { 0 };
            }
        }
        let trace = prefix(&mut e, &s).unwrap();
        assert!(!trace.iter().any(|e| e["kind"] == "roster"));
        assert_eq!((word(&e, 0xD12A), word(&e, 0xD12C)), (1, 128));
        let division: Vec<_> = trace.iter().filter(|e| e["kind"] == "division").collect();
        assert_eq!(division[0]["arguments"], json!([0, 2805, 22]));
        assert_eq!(
            division[0]["result"],
            json!({"quotient":127,"remainder":11})
        );
        assert_eq!(
            division[0]["input"]["club_totals"],
            json!(s.core.club_totals)
        );
        assert_eq!(division[1]["input"]["club_totals"][1], 128);
    }
    #[test]
    fn signed_availability_and_flags_exclude_sum_and_sort_as_fourteen() {
        for (byte8, flags) in [
            (0, 0),
            (128, 0),
            (255, 0),
            (1, 0x80),
            (1, 0x20),
            (1, 2),
            (1, 4),
        ] {
            let s = sample();
            let mut e = image(cells(&s).unwrap());
            ordinary(&mut e);
            let base = club(word(&e, 0xD6E4));
            let id = word(&e, base);
            let q = crate::layout::P15 + 15 * usize::from(id);
            e.image[q + 8] = byte8;
            e.image[q + 11] = flags;
            let trace = prefix(&mut e, &s).unwrap();
            assert_eq!(trace[0]["result"], 1);
            assert_eq!(word(&e, base + 20), id); // unavailable original keeper sorts behind role10.
            assert!(trace
                .iter()
                .any(|e| e["kind"] == "roles" && e["input"]["b"] == 14));
        }
    }
    #[test]
    fn mixed_roles_use_strict_unsigned_nested_comparisons_not_stable_sort() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        ordinary(&mut e);
        let base = club(word(&e, 0xD6E4));
        let before: Vec<_> = (0..11).map(|i| word(&e, base + 2 * i)).collect();
        for (i, role) in [0, 3, 3, 2, 5, 6, 7, 8, 9, 10, 11].into_iter().enumerate() {
            let id = before[i];
            e.image[crate::layout::P8 + 8 * usize::from(id) + 6] = role;
        }
        let trace = prefix(&mut e, &s).unwrap();
        let swaps: Vec<_> = trace.iter().filter(|e| e["kind"] == "roster").collect();
        assert_eq!(swaps.len(), 1);
        assert_eq!(swaps[0]["arguments"], json!([0, 2, 4, 2]));
        assert_eq!(
            (word(&e, base + 2), word(&e, base + 4), word(&e, base + 6)),
            (before[3], before[2], before[1])
        );
    }
    #[test]
    fn exchange_transaction_moves_entire_scene_totals_cache_and_both_selectors() {
        for offset in [0u8, 11] {
            let s = sample();
            let mut e = image(cells(&s).unwrap());
            ordinary(&mut e);
            e.image[0xD56] = offset;
            e.image[0xD57] = 11 - offset;
            let base = club(word(&e, 0xD6E4));
            let p1 = word(&e, base);
            let p2 = word(&e, base + 2);
            e.image[crate::layout::P8 + 8 * usize::from(p1) + 6] = 1;
            e.image[crate::layout::P8 + 8 * usize::from(p2) + 6] = 0;
            let lo = offset + 1;
            let hi = offset + 2;
            e.image[0xD09F] = lo;
            e.image[0xD0A0] = hi;
            put(&mut e, addr(lo) + 8, 0x1234);
            put(&mut e, addr(hi) + 8, 0x5678);
            put(&mut e, 0xD148, 111);
            put(&mut e, 0xD14A, 222);
            e.image[0xD198] = 31;
            e.image[0xD199] = 17;
            let trace = prefix(&mut e, &s).unwrap();
            assert_eq!((word(&e, base), word(&e, base + 2)), (p2, p1));
            assert_eq!(
                (word(&e, addr(lo) + 8), word(&e, addr(hi) + 8)),
                (0x5678, 0x1234)
            );
            assert_eq!((word(&e, 0xD148), word(&e, 0xD14A)), (222, 111));
            assert_eq!((e.image[0xD198], e.image[0xD199]), (17, 31));
            assert_eq!(selectors(&e), json!({"actor":hi,"secondary":lo}));
            let scenes = trace.iter().find(|e| e["kind"] == "scenes").unwrap();
            assert_eq!(scenes["input"]["a"]["word8"], 0x5678);
            assert_eq!(scenes["output"]["a"]["word8"], 0x1234);
            let mark = trace.iter().find(|e| e["kind"] == "selectors").unwrap();
            assert_eq!(mark["input"], json!({"actor":lo,"secondary":hi}));
        }
    }
    #[test]
    fn signed32_division_quotient_remainder_and_exception_domains() {
        for (a, b, q, r) in [
            (7, 3, 2, 1),
            (-7, 3, -2, -1),
            (7, -3, -2, 1),
            (-7, -3, 2, -1),
            (i32::MIN, 1, i32::MIN, 0),
            (i32::MAX, 22, 97612893, 1),
        ] {
            assert_eq!(divide(a, b).unwrap(), (q, r));
        }
        assert!(divide(1, 0).is_err());
        assert!(divide(i32::MIN, -1).is_err());
    }
    #[test]
    fn conflicting_component_aliases_and_missing_canonical_references_reject() {
        let s = sample();
        let mut cases = vec![];
        let mut b = s.clone();
        b.core.players[1].p8[6] ^= 1;
        cases.push(b);
        let mut b = s.clone();
        b.core.rosters[0][0] = 0;
        cases.push(b);
        let mut b = s.clone();
        b.core.cached_players[0] = 0;
        cases.push(b);
        let mut b = s.clone();
        b.core.side_offsets = [0, 0];
        cases.push(b);
        let mut b = s.clone();
        b.codes.clock ^= 1;
        cases.push(b);
        let mut b = s.clone();
        b.terminal.signs[0] ^= 1;
        cases.push(b);
        let mut b = s.clone();
        b.pairing.clubs[0].roster[0] = 0;
        cases.push(b);
        let mut b = s.clone();
        b.core.players.pop();
        cases.push(b);
        let mut b = s.clone();
        b.core.players.push(b.core.players[0].clone());
        cases.push(b);
        let mut b = s.clone();
        b.codes.players.pop();
        cases.push(b);
        let mut b = s.clone();
        b.core.rosters[0][19] = 3920;
        cases.push(b);
        let mut b = s.clone();
        b.core.assignment.club_counts[0].cell40 ^= 1;
        b.core.assignment.club_counts[0].index40 = b.core.assignment.club_counts[1].index40;
        cases.push(b);
        for b in cases {
            assert!(cells(&b).is_err());
        }
        assert!(cells(&s).is_ok());
    }
    #[test]
    fn unsupported_actor_offset_mode_and_assignment_metadata_reject() {
        let s = sample();
        let mut cases = vec![];
        for actor in [0, 23, 255] {
            let mut b = s.clone();
            b.core.actor = actor;
            cases.push(b);
        }
        let mut b = s.clone();
        b.core.secondary = 23;
        cases.push(b);
        let mut b = s.clone();
        b.core.assignment.index = 2;
        cases.push(b);
        let mut b = s.clone();
        b.core.assignment.selector = 2;
        cases.push(b);
        let mut b = s.clone();
        b.codes.mode = 5;
        cases.push(b);
        let mut b = s.clone();
        b.pairing.side_offsets = [0, 0];
        cases.push(b);
        let mut b = s.clone();
        b.terminal.positions.pop();
        cases.push(b);
        for b in cases {
            assert!(rearrange(&b).is_err());
        }
    }
    #[test]
    fn expected_results_and_all_nested_intermediates_are_not_inputs() {
        for field in [
            "output", "expected", "events", "roles", "quotient", "result",
        ] {
            let mut v = serde_json::to_value(sample()).unwrap();
            v[field] = json!(42);
            assert!(serde_json::from_value::<State>(v).is_err());
            for key in ["core", "pairing", "codes", "cache", "terminal"] {
                let mut v = serde_json::to_value(sample()).unwrap();
                v[key][field] = json!(42);
                assert!(serde_json::from_value::<State>(v).is_err());
            }
        }
    }
    #[test]
    fn five_tails_use_current_image_and_final_keeper_word12_clear_is_owned_by_parent() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        put(&mut e, addr(1) + 12, 111);
        put(&mut e, addr(12) + 12, 222);
        let out = run(&mut e, &s).unwrap();
        let tail: Vec<_> = out
            .events
            .iter()
            .filter(|e| e["kind"] == "opaque")
            .collect();
        assert_eq!(tail.len(), 5);
        for pair in tail.windows(2) {
            assert_eq!(pair[0]["output"], pair[1]["input"]);
        }
        assert_eq!(tail[4]["output"]["assignment"]["scenes"][0]["word12"], 111);
        assert_eq!(tail[4]["output"]["assignment"]["scenes"][11]["word12"], 222);
        assert_eq!(
            (
                out.output.core.assignment.scenes[0].word12,
                out.output.core.assignment.scenes[11].word12
            ),
            (0, 0)
        );
        // A live raw byte supplied by a caller reaches the sum, independently of observation templates.
        let mut e = image(cells(&s).unwrap());
        let id = word(&e, club(word(&e, 0xD6E4)));
        e.image[crate::layout::P8 + 8 * usize::from(id) + 4] = 0;
        let t = prefix(&mut e, &s).unwrap();
        assert_eq!(t[0]["input"]["p8_4"], 0);
    }
    #[test]
    fn complete_parent_whole_image_ownership_and_seed_preservation() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        e.image[0x8000] = 0xAB;
        let before = e.image.clone();
        let out = run(&mut e, &s).unwrap();
        for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
            if old == new {
                continue;
            }
            let owned_scene = (1..=22).any(|i| a >= addr(i) && a < addr(i) + 25);
            let owned_roster = s
                .core
                .assignment
                .clubs
                .iter()
                .any(|c| a >= club(c.id) && a < club(c.id) + 22);
            let owned_cache = (0..3).any(|i| a == 0xCF32 + 7 * i || a == 0xCF33 + 7 * i)
                || (0xCF5E..0xCF72).contains(&a);
            let owned_globals = [0xD09F, 0xD0A0, 0xD0A2, 0xD0A3, 0xD0A4, 0xD0A5].contains(&a)
                || (0xD12A..0xD12E).contains(&a)
                || (0..2).any(|side| {
                    (0xD148 + 40 * side..0xD148 + 40 * side + 22).contains(&a)
                        || (0xD198 + 20 * side..0xD198 + 20 * side + 11).contains(&a)
                });
            let locals = (0x40000..0x4000B).contains(&a)
                || (0x40010..0x4001B).contains(&a)
                || (0x40020..0x4002B).contains(&a);
            assert!(
                owned_scene || owned_roster || owned_cache || owned_globals || locals,
                "unexpected write {a:X}"
            );
        }
        assert_eq!(word(&e, 0xA51C), s.core.assignment.seed);
        assert_eq!(e.image[0x8000], 0xAB);
        assert!(out.draws.is_empty());
        for p in &s.core.players {
            assert_eq!(
                &e.image[crate::layout::P8 + 8 * usize::from(p.id)
                    ..crate::layout::P8 + 8 * usize::from(p.id) + 8],
                &p.p8
            );
            assert_eq!(
                &e.image[crate::layout::P15 + 15 * usize::from(p.id)
                    ..crate::layout::P15 + 15 * usize::from(p.id) + 15],
                &p.p15
            );
        }
    }
}
