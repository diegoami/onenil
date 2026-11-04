//! [confirmed] F24–F28 at archaeology218490d: bounded logicalAEB8/53FB composition.
use crate::{rearrangement, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FlagsInput {
    pub player: u16,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_5: u8,
    pub p8_6: u8,
    pub flags10: u8,
}
#[derive(Debug, Serialize)]
pub struct FlagsOutcome {
    pub flags10: u8,
    pub draws: Vec<Draw>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub match_state: rearrangement::State,
    pub incoming: u8,
    pub index: u8,
    pub side: u8,
    pub allowance: u8,
    pub sub_flags: [u8; 2],
    pub reserves: [u8; 2],
    pub mode: u8,
    pub position_sentinel: u16,
    pub speaker_gate: u8,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub helpers: Vec<Value>,
    pub presentation: Vec<Value>,
    pub draws: Vec<Draw>,
}
use crate::{
    cache_helpers, goal_effects, placement,
    role_scores::{image, insert, Cells},
    scene_assignment, Engine,
};
use serde_json::json;
fn addr(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
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
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes());
}
// [confirmed] F27/4E0C: signed word intermediate on young path; adult path returns raw AL.
fn rating(e: &Engine, id: u16) -> u8 {
    let a = p8(id);
    let age = e.image[a + 1];
    let value = u16::from(e.image[a + 5]);
    if age < 18 {
        let v = value.wrapping_sub(1).wrapping_sub(3 * u16::from(18 - age));
        i32::from(v as i16).max(0) as u8
    } else {
        value.wrapping_sub(u16::from(e.image[a + 3] & 1)) as u8
    }
}
fn flags_image(e: &mut Engine, id: u16) {
    let a = p15(id) + 10;
    if rating(e, id) < 4 {
        e.image[a] = (e.image[a] | 0x80) & !0x08;
    }
    match e.image[p8(id) + 6] {
        0 | 1 | 3 | 4 => e.image[a] |= 0x11,
        2 => e.image[a] |= 0x32,
        _ => {}
    }
}
pub fn flags(s: &FlagsInput) -> Result<FlagsOutcome, String> {
    if s.player > 3920 || s.p8_6 > 13 {
        return Err("unsupported53FB player/role".into());
    }
    let mut c = Cells::new();
    for (i, v) in [(1, s.p8_1), (3, s.p8_3), (5, s.p8_5), (6, s.p8_6)] {
        insert(&mut c, p8(s.player) + i, v)?;
    }
    insert(&mut c, p15(s.player) + 10, s.flags10)?;
    let mut e = image(c);
    flags_image(&mut e, s.player);
    Ok(FlagsOutcome {
        flags10: e.image[p15(s.player) + 10],
        draws: vec![],
    })
}
fn cw(c: &mut Cells, a: usize, v: u16) -> Result<(), String> {
    for (i, b) in v.to_le_bytes().into_iter().enumerate() {
        insert(c, a + i, b)?;
    }
    Ok(())
}
fn cells(s: &State) -> Result<Cells, String> {
    if !matches!(s.match_state.core.side_offsets, [0, 11] | [11, 0])
        || s.side > 1
        || !(12..=20).contains(&s.incoming)
        || !(1..=22).contains(&s.index)
    {
        return Err("unsupportedAEB8 arguments".into());
    }
    let side = usize::from(s.side);
    let offset = s.match_state.core.side_offsets[side];
    if s.index <= offset || s.index > offset + 11 {
        return Err("AEB8 scene does not belong to side".into());
    }
    let outgoing = s
        .match_state
        .core
        .assignment
        .scenes
        .get(usize::from(s.index - 1))
        .ok_or("missingAEB8 scene")?
        .word14;
    let incoming = s.match_state.core.rosters[side][usize::from(s.index - offset - 1)];
    if outgoing == 0
        || incoming == 0
        || outgoing == incoming
        || s.match_state.core.rosters[side][usize::from(s.incoming - 1)] != outgoing
    {
        return Err("AEB8 outgoing/selected/bench identity mismatch".into());
    }
    let mut c = rearrangement::cells(&s.match_state)?;
    insert(&mut c, 0xD6DD, s.allowance)?;
    insert(&mut c, 0xD074, s.mode)?;
    insert(&mut c, 0xA2EC, s.speaker_gate)?;
    cw(&mut c, 0xD2E, s.position_sentinel)?;
    for side in 0..2 {
        insert(&mut c, 0xD6DE + side, s.sub_flags[side])?;
        insert(&mut c, 0xD6E0 + side, s.reserves[side])?;
    }
    Ok(c)
}
fn snapshot(e: &mut Engine, s: &State) -> State {
    let mut out = s.clone();
    out.match_state = rearrangement::snapshot(e, &s.match_state);
    out.allowance = e.image[0xD6DD];
    out.mode = e.image[0xD074];
    out.speaker_gate = e.image[0xA2EC];
    out.position_sentinel = word(e, 0xD2E);
    out.sub_flags.copy_from_slice(&e.image[0xD6DE..0xD6E0]);
    out.reserves.copy_from_slice(&e.image[0xD6E0..0xD6E2]);
    out
}
fn observation(e: &Engine, s: &State) -> Value {
    json!({"core":rearrangement::core(e,&s.match_state.core),"allowance":e.image[0xD6DD],"mode":e.image[0xD074],"speaker_gate":e.image[0xA2EC],"position_sentinel":word(e,0xD2E),"sub_flags":[e.image[0xD6DE],e.image[0xD6DF]],"reserves":[e.image[0xD6E0],e.image[0xD6E1]]})
}
fn helper(site: &str, args: Value, input: Value, output: Value, result: Value) -> Value {
    json!({"site":site,"arguments":args,"input":input,"output":output,"result":result,"draw_start":0,"draw_end":0})
}
fn readonly(e: &Engine, s: &State, site: &str, args: Value, result: u8) -> Value {
    let v = observation(e, s);
    helper(site, args, v.clone(), v, json!(result))
}
fn available(e: &Engine, id: u16) -> u8 {
    placement::unavailable(&placement::Player {
        id,
        p15_sbyte8: e.image[p15(id) + 8] as i8,
        p15_byte11: e.image[p15(id) + 11],
    })
}
fn motion(e: &mut Engine, index: u8, delta: i16, cache_flag: u8) {
    let row = crate::terminal::Scene::read(e, index);
    let selector = row.byte7;
    let slot = if selector > 11 {
        selector - 11
    } else {
        selector
    };
    let side = usize::from(selector < 12);
    let cache = 0xD197 + 20 * side + usize::from(slot);
    let out = goal_effects::byte6(&goal_effects::Byte6Input {
        old_byte6: e.image[p15(row.word14) + 6],
        delta,
        cache_flag,
        cache_before: e.image[cache],
    });
    e.image[p15(row.word14) + 6] = out.byte6;
    if cache_flag != 0 {
        e.image[cache] = out.cache;
    }
}
fn season(e: &mut Engine, id: u16) {
    if e.image[0xD6DD] != 0 {
        e.image[p15(id) + 4] = e.image[p15(id) + 4].wrapping_add(1);
        put(e, p15(id), word(e, p15(id)).wrapping_add(1));
    }
}
// [designed] Presentation adapter events replace original port/display/log-scratch work.
// They are completed logical observations,not pending engine requests or original-render parity.
fn presentation(
    e: &Engine,
    s: &State,
    site: &str,
    kind: &str,
    args: Value,
    helpers: &mut Vec<Value>,
    events: &mut Vec<Value>,
) {
    let v = observation(e, s);
    helpers.push(helper(site, args.clone(), v.clone(), v, Value::Null));
    events.push(json!({"site":site,"kind":kind,"arguments":args}));
}
fn accounting(e: &mut Engine, incoming: u16, side: usize) {
    if e.image[p8(incoming) + 7] == 0 && e.image[0xD6DD] != 0 && e.image[0xD6DE + side] == 0 {
        e.image[0xD6DE + side] = 1;
    } else {
        e.image[0xD6E0 + side] = e.image[0xD6E0 + side].wrapping_sub(1);
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
    let a = addr(s.index);
    let outgoing = word(e, a + 14);
    let mut helpers = vec![];
    let mut events = vec![];
    let before = observation(e, s);
    e.image[p15(outgoing) + 11] = e.image[p15(outgoing) + 11] | 4;
    helpers.push(helper(
        "0AB9:AEDF",
        json!([3, outgoing]),
        before,
        observation(e, s),
        Value::Null,
    ));
    let club_id = word(e, 0xD6E4 + 2 * side);
    let managed = cache_helpers::managed_image(e, club_id);
    helpers.push(readonly(e, s, "0AB9:AEF0", json!([club_id]), managed));
    if managed == 0 {
        presentation(
            e,
            s,
            "0AB9:AF01",
            "speaker-off",
            json!([]),
            &mut helpers,
            &mut events,
        );
        let opposite = word(e, 0xD6E4 + 2 * (1 - side));
        let op = cache_helpers::managed_image(e, opposite);
        helpers.push(readonly(e, s, "0AB9:AF18", json!([opposite]), op));
        if op != 0 && e.image[0xD074] == 0 {
            let slot = s.index - e.image[0xD56 + side];
            let incoming = word(e, club(club_id) + 2 * usize::from(slot - 1));
            presentation(
                e,
                s,
                "0AB9:AF5F",
                "replacement-message",
                json!([outgoing, incoming]),
                &mut helpers,
                &mut events,
            );
        }
        let slot = s.index - e.image[0xD56 + side];
        presentation(
            e,
            s,
            "0AB9:AF7F",
            "replacement-log",
            json!([0, s.incoming, slot, s.side]),
            &mut helpers,
            &mut events,
        );
        if compose_commentary {
            // [confirmed:F14/F25] Execute atAF7F,before164F can reorder the roster.
            goal_effects::numeric_commentary(e, 0, u16::from(slot), u16::from(s.incoming), s.side);
        }
    }
    let old = word(e, a + 2);
    let saved = if old == word(e, 0xD2E) {
        word(e, 0xD32)
    } else {
        old
    };
    let before = observation(e, s);
    scene_assignment::rebuild_image(e, s.index, s.index)?;
    helpers.push(helper(
        "0AB9:AFAA",
        json!([s.index, s.index]),
        before,
        observation(e, s),
        Value::Null,
    ));
    let before = observation(e, s);
    motion(e, s.index, 1, 1);
    helpers.push(helper(
        "0AB9:AFB7",
        json!([s.index, 1, 1]),
        before,
        observation(e, s),
        Value::Null,
    ));
    let incoming = word(e, a + 14);
    if managed == 0 {
        e.image[p15(incoming) + 10] = 0;
        let before = observation(e, s);
        flags_image(e, incoming);
        helpers.push(helper(
            "0AB9:AFDA",
            json!([incoming]),
            before,
            observation(e, s),
            Value::Null,
        ));
        let slot = s.index - e.image[0xD56 + side];
        put(e, 0xD146 + 40 * side + 2 * usize::from(slot), 0x2747);
    }
    let before = observation(e, s);
    season(e, incoming);
    helpers.push(helper(
        "0AB9:B00C",
        json!([incoming]),
        before,
        observation(e, s),
        Value::Null,
    ));
    accounting(e, incoming, side);
    put(e, a + 2, saved);
    let mut fallback = [false; 2];
    for (side, flag) in fallback.iter_mut().enumerate() {
        let club_id = word(e, 0xD6E4 + 2 * side);
        let managed = cache_helpers::managed_image(e, club_id);
        helpers.push(readonly(e, s, "0AB9:B077", json!([side, club_id]), managed));
        if managed != 0 {
            let id = word(e, 0xCF5E + 10 * side);
            let unavailable = available(e, id);
            helpers.push(readonly(e, s, "0AB9:B08B", json!([side, id]), unavailable));
            if unavailable != 0 {
                *flag = true;
                let first = word(e, club(club_id));
                put(e, 0xCF5E + 10 * side, first);
            }
        }
    }
    let before = observation(e, s);
    rearrangement::run(e, &s.match_state)?;
    helpers.push(helper(
        "0AB9:B0C6",
        json!([]),
        before,
        observation(e, s),
        Value::Null,
    ));
    for (side, flag) in fallback.into_iter().enumerate() {
        if flag {
            put(e, 0xCF5E + 10 * side, 0);
        }
    }
    Ok(Outcome {
        output: snapshot(e, s),
        helpers,
        presentation: events,
        draws: vec![],
    })
}
pub fn replace(s: &State) -> Result<Outcome, String> {
    run(&mut image(cells(s)?), s)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> State {
        let c: Value = include_str!("../tests/fixtures/replacement/cases.jsonl")
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .find(|c| c["stream"] == "replacements")
            .unwrap();
        serde_json::from_value(c["input"].clone()).unwrap()
    }
    fn fi(age: u8, parity: u8, value: u8, role: u8, flags10: u8) -> FlagsInput {
        FlagsInput {
            player: 3920,
            p8_1: age,
            p8_3: parity,
            p8_5: value,
            p8_6: role,
            flags10,
        }
    }
    #[test]
    fn young_signed_word_clamp_and_adult_al_wrap_have_distinct_thresholds() {
        for (age, parity, value, want) in [
            (17, 0, 8, 0),
            (17, 0, 7, 128),
            (0, 0, 255, 0),
            (0, 0, 0, 128),
            (18, 1, 0, 0),
            (18, 0, 0, 128),
            (255, 255, 4, 128),
            (255, 254, 4, 0),
        ] {
            assert_eq!(flags(&fi(age, parity, value, 13, 0)).unwrap().flags10, want);
        }
    }
    #[test]
    fn role_bits_and_preset_flags_are_preserved_except_explicit_bit_four_clear() {
        for (role, want) in [
            (0, 0x11),
            (1, 0x11),
            (2, 0x32),
            (3, 0x11),
            (4, 0x11),
            (5, 0),
            (13, 0),
        ] {
            assert_eq!(flags(&fi(18, 0, 255, role, 0)).unwrap().flags10, want);
        }
        assert_eq!(flags(&fi(17, 0, 0, 13, 255)).unwrap().flags10, 0xF7);
        assert_eq!(flags(&fi(18, 0, 255, 13, 255)).unwrap().flags10, 255);
        assert_eq!(flags(&fi(18, 0, 0, 2, 0x08)).unwrap().flags10, 0xB2);
    }
    #[test]
    fn motion_reuses_signed_clamp_and_routes_by_stored_selector_not_physical_index() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        let id = word(&e, addr(s.index) + 14);
        e.image[addr(s.index) + 7] = 13;
        for (old, delta, flag, want, cache) in [
            (127, 1, 1, 128, 128),
            (255, 1, 1, 1, 1),
            (0, 32767, 1, 255, 255),
            (127, 32767, 1, 1, 1),
            (10, -1, 0, 9, 77),
        ] {
            e.image[p15(id) + 6] = old;
            e.image[0xD199] = 77;
            motion(&mut e, s.index, delta, flag);
            assert_eq!(e.image[p15(id) + 6], want);
            assert_eq!(e.image[0xD199], cache);
        }
    }
    #[test]
    fn season_counters_wrap_only_for_nonzero_allowance() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        let id = word(&e, addr(s.index) + 14);
        for allowance in [0, 1, 255] {
            put(&mut e, p15(id), 65535);
            e.image[p15(id) + 4] = 255;
            e.image[0xD6DD] = allowance;
            season(&mut e, id);
            assert_eq!(
                (word(&e, p15(id)), e.image[p15(id) + 4]),
                if allowance == 0 { (65535, 255) } else { (0, 0) }
            );
        }
    }
    #[test]
    fn special_substitution_and_reserve_choices_preserve_unselected_side() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        let id = word(&e, addr(s.index) + 14);
        let side = usize::from(s.side);
        for (style, allowance, flag, reserve, wantflag, wantreserve) in [
            (0, 1, 0, 2, 1, 2),
            (0, 0, 0, 0, 0, 255),
            (0, 1, 1, 2, 1, 1),
            (1, 1, 0, 2, 0, 1),
        ] {
            e.image[p8(id) + 7] = style;
            e.image[0xD6DD] = allowance;
            e.image[0xD6DE + side] = flag;
            e.image[0xD6E0 + side] = reserve;
            let other = e.image[0xD6E0 + 1 - side];
            accounting(&mut e, id, side);
            assert_eq!(
                (e.image[0xD6DE + side], e.image[0xD6E0 + side]),
                (wantflag, wantreserve)
            );
            assert_eq!(e.image[0xD6E0 + 1 - side], other);
        }
    }
    #[test]
    fn saved_word2_sentinel_and_ordinary_value_survive_rebuild_and_parent() {
        let s = sample();
        for (old, sentinel, want) in [(65535, 65535, 247), (32767, 11, 32767)] {
            let mut e = image(cells(&s).unwrap());
            put(&mut e, addr(s.index) + 2, old);
            put(&mut e, 0xD2E, sentinel);
            let o = run(&mut e, &s).unwrap();
            assert_eq!(word(&e, addr(s.index) + 2), want);
            let rebuild = o.helpers.iter().find(|h| h["site"] == "0AB9:AFAA").unwrap();
            assert_eq!(
                rebuild["output"]["core"]["assignment"]["scenes"][usize::from(s.index - 1)]
                    ["word2"],
                247
            );
        }
    }
    #[test]
    fn managed_replacement_skips_unmanaged_flags_and_presentation() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        let side = usize::from(s.side);
        let new_id = s.match_state.core.rosters[side]
            [usize::from(s.index - s.match_state.core.side_offsets[side] - 1)];
        e.image[0xDCBB] = word(&e, 0xD6E4 + 2 * side) as u8;
        let old = e.image[p15(new_id) + 10];
        let out = run(&mut e, &s).unwrap();
        assert_eq!(e.image[p15(new_id) + 10], old);
        assert!(out.presentation.is_empty());
        assert!(!out.helpers.iter().any(
            |h| ["0AB9:AF01", "0AB9:AF18", "0AB9:AFDA"].contains(&h["site"].as_str().unwrap())
        ));
    }
    #[test]
    fn temporary_cached_player_fallback_is_input_to_rearrangement_then_cleared() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        let cached = word(&e, 0xCF5E);
        e.image[p15(cached) + 8] = 0;
        let first = word(&e, club(word(&e, 0xD6E4)));
        let out = run(&mut e, &s).unwrap();
        let h = out
            .helpers
            .iter()
            .find(|h| h["site"] == "0AB9:B0C6")
            .unwrap();
        assert_eq!(h["input"]["core"]["cached_players"][0], first);
        assert_eq!(h["output"]["core"]["cached_players"][0], first);
        assert_eq!(word(&e, 0xCF5E), 0);
        assert_eq!(out.output.match_state.cache.cache[0][0], 0);
    }
    #[test]
    fn presentation_branches_emit_typed_reach_events_without_port_or_display_backend() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        e.image[0xD074] = 0;
        e.image[0xA2EC] = 0;
        let out = run(&mut e, &s).unwrap();
        assert_eq!(
            out.presentation
                .iter()
                .map(|p| p["kind"].as_str().unwrap())
                .collect::<Vec<_>>(),
            vec!["speaker-off", "replacement-message", "replacement-log"]
        );
        assert_eq!(out.presentation[2]["arguments"], json!([0, 14, 8, 1]));
        assert_eq!(e.image[0xA2EC], 0);
    }
    #[test]
    fn identity_offset_canonical_and_extra_output_errors_diagnose() {
        let s = sample();
        let mut bad = vec![];
        for side in [2, 255] {
            let mut b = s.clone();
            b.side = side;
            bad.push(b);
        }
        for index in [0, 23, 255, 12] {
            let mut b = s.clone();
            b.index = index;
            bad.push(b);
        }
        for incoming in [0, 11, 21, 255] {
            let mut b = s.clone();
            b.incoming = incoming;
            bad.push(b);
        }
        let mut b = s.clone();
        b.match_state.core.side_offsets = [255, 0];
        bad.push(b);
        let mut b = s.clone();
        b.match_state.core.rosters[1][13] = 0;
        bad.push(b);
        let mut b = s.clone();
        b.match_state.core.players[1].p15[11] ^= 1;
        bad.push(b);
        let mut b = s.clone();
        b.match_state.core.assignment.scenes.clear();
        bad.push(b);
        for b in bad {
            assert!(replace(&b).is_err());
        }
        assert!(flags(&fi(18, 0, 255, 14, 0)).is_err());
        let mut f = fi(18, 0, 255, 13, 0);
        f.player = 3921;
        assert!(flags(&f).is_err());
        for field in [
            "expected",
            "output",
            "rating",
            "helpers",
            "presentation",
            "result",
        ] {
            let mut v = serde_json::to_value(&s).unwrap();
            v[field] = json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
            let mut v = serde_json::to_value(&f).unwrap();
            v[field] = json!(1);
            assert!(serde_json::from_value::<FlagsInput>(v).is_err());
        }
    }
    #[test]
    fn whole_image_writes_belong_to_replacement_or_its_native_callees() {
        let s = sample();
        let mut e = image(cells(&s).unwrap());
        e.image[0x8000] = 0xAB;
        let before = e.image.clone();
        let outgoing = word(&e, addr(s.index) + 14);
        let incoming = s.match_state.core.rosters[usize::from(s.side)]
            [usize::from(s.index - s.match_state.core.side_offsets[usize::from(s.side)] - 1)];
        run(&mut e, &s).unwrap();
        for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
            if old == new {
                continue;
            }
            let scene = (1..=22).any(|i| a >= addr(i) && a < addr(i) + 25);
            let roster = s
                .match_state
                .core
                .assignment
                .clubs
                .iter()
                .any(|c| a >= club(c.id) && a < club(c.id) + 22);
            let cache = (0..3).any(|i| a == 0xCF32 + 7 * i || a == 0xCF33 + 7 * i)
                || (0xCF5E..0xCF72).contains(&a);
            let globals = [0xD09F, 0xD0A0, 0xD0A2, 0xD0A3, 0xD0A4, 0xD0A5].contains(&a)
                || (0xD12A..0xD12E).contains(&a)
                || (0xD6DE..0xD6E2).contains(&a)
                || (0..2).any(|side| {
                    (0xD148 + 40 * side..0xD148 + 40 * side + 22).contains(&a)
                        || (0xD198 + 20 * side..0xD198 + 20 * side + 11).contains(&a)
                });
            let local = (0x40000..0x4000B).contains(&a)
                || (0x40010..0x4001B).contains(&a)
                || (0x40020..0x4002B).contains(&a);
            let player =
                a == p15(outgoing) + 11 || [0, 1, 4, 6, 10].iter().any(|i| a == p15(incoming) + i);
            assert!(
                scene || roster || cache || globals || local || player,
                "unexpected write {a:X}"
            );
        }
        assert_eq!(word(&e, 0xA51C), s.match_state.core.assignment.seed);
        assert_eq!(e.image[0x8000], 0xAB);
        for p in &s.match_state.core.players {
            assert_eq!(&e.image[p8(p.id)..p8(p.id) + 8], &p.p8);
        }
    }
}
