//! [confirmed] F30/T30 at archaeology218490d: bounded scene/cache code refresh.
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeInput {
    pub scene: Scene,
    pub a: u16,
    pub b: u16,
    pub threshold: u8,
    pub clock: u16,
    pub direction_signs: [u8; 2],
    pub codes: [u8; 2],
}
#[derive(Debug, Serialize)]
pub struct CodeOutput {
    pub value: u16,
    pub scene: Scene,
}
#[derive(Debug, Serialize)]
pub struct CodeOutcome {
    pub output: CodeOutput,
    pub draws: Vec<Draw>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub role: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cache {
    pub index: u8,
    pub word0: u16,
    pub word2: u16,
    pub word4: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub actor: u8,
    pub directions: [u8; 4],
    pub mode: u8,
    pub threshold: u8,
    pub clock: u16,
    pub direction_signs: [u8; 2],
    pub codes: [u8; 6],
    pub role_codes: [u8; 14],
    pub cache: [Cache; 3],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub events: Vec<Value>,
    pub draws: Vec<Draw>,
}

use crate::{terminal, Engine, Input as ContestInput};
use serde_json::json;
use std::collections::BTreeSet;
const P8: usize = 0x10000;
// [designed] Canonical storage for the byte pointed to by original D0C0.
const MODE: usize = 0x40080;
fn blank(seed: u16) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed,
        ratings: [0; 8],
        goals: [0; 2],
    })
}
fn address(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn load_scene(e: &mut Engine, r: &Scene) {
    let a = address(r.index);
    e.image[a..a + 25].copy_from_slice(&r.bytes());
}
fn load_direction(e: &mut Engine, threshold: u8, clock: u16, signs: [u8; 2]) {
    e.image[0xD0AC] = threshold;
    put(e, 0xD0AE, clock);
    e.image[0xA338..0xA33A].copy_from_slice(&signs);
}
// [confirmed] F30/0143: signed high-byte products retain low words;return before toggle.
pub(crate) fn code_image(e: &mut Engine, index: u8, a: u16, b: u16) -> u16 {
    let row = Scene::read(e, index);
    let direction = terminal::direction_image(e, &row).0;
    let initial = direction == 255;
    let mut base = u16::from(e.image[0x096A + usize::from(initial)]);
    if (a >> 8) != (b >> 8) {
        let da = i16::from(terminal::direction_image(e, &row).0 as i8) as u16;
        let pa = (i16::from((a >> 8) as u8 as i8) as u16).wrapping_mul(da);
        let db = i16::from(terminal::direction_image(e, &row).0 as i8) as u16;
        let pb = (i16::from((b >> 8) as u8 as i8) as u16).wrapping_mul(db);
        let selected = ((pb as i16) > (pa as i16)) == initial;
        base = u16::from(e.image[0x096A + usize::from(selected)]);
    } else if (a as u8) != (b as u8) {
        base = u16::from(e.image[0x096B]) + 2;
    }
    let value = base.wrapping_add(6).wrapping_add(u16::from(row.byte6));
    if a != b {
        e.image[address(index) + 6] = u8::from(row.byte6 == 0);
    }
    value
}
fn code_machine(s: &CodeInput) -> Result<Engine, String> {
    if !(1..=22).contains(&s.scene.index) {
        return Err("unsupported0143 physical scene".into());
    }
    let mut e = blank(0xBEEF);
    load_scene(&mut e, &s.scene);
    load_direction(&mut e, s.threshold, s.clock, s.direction_signs);
    e.image[0x096A..0x096C].copy_from_slice(&s.codes);
    Ok(e)
}
fn machine(s: &State) -> Result<Engine, String> {
    if s.mode > 4 {
        return Err("unsupported0207 mode:original local is undefined above4".into());
    }
    if !matches!(s.side_offsets, [0, 11] | [11, 0])
        || s.scenes.len() != 22
        || s.cache
            .iter()
            .enumerate()
            .any(|(i, c)| usize::from(c.index) != i)
    {
        return Err("unsupported scene-code offset/scene/cache domain".into());
    }
    let mut ids = BTreeSet::new();
    for p in &s.players {
        if p.id > 3920 || p.role > 13 || !ids.insert(p.id) {
            return Err("ambiguous scene-code canonical player/role".into());
        }
    }
    if s.scenes
        .iter()
        .enumerate()
        .any(|(i, r)| usize::from(r.index) != i + 1 || !ids.contains(&r.word14))
    {
        return Err("missing or unordered scene-code canonical input".into());
    }
    let mut e = blank(s.seed);
    load_direction(&mut e, s.threshold, s.clock, s.direction_signs);
    e.image[0xD56..0xD58].copy_from_slice(&s.side_offsets);
    e.image[0xD09F] = s.actor;
    e.image[MODE] = s.mode;
    e.image[0xD0C8..0xD0CC].copy_from_slice(&s.directions);
    e.image[0x096A..0x0970].copy_from_slice(&s.codes);
    e.image[0x8716..0x8724].copy_from_slice(&s.role_codes);
    for r in &s.scenes {
        load_scene(&mut e, r);
    }
    for p in &s.players {
        e.image[P8 + 8 * usize::from(p.id) + 6] = p.role;
    }
    for c in &s.cache {
        let a = 0xCF2E + 7 * usize::from(c.index);
        for (offset, v) in [(0, c.word0), (2, c.word2), (4, c.word4)] {
            put(&mut e, a + offset, v);
        }
    }
    Ok(e)
}
fn low_image(e: &mut Engine, index: u8) -> Result<u8, String> {
    let row = Scene::read(e, index);
    let selector = row.byte7;
    match e.image[MODE] {
        0 => Ok(code_image(e, index, row.word2, row.word0) as u8),
        1 => {
            if !(1..=22).contains(&selector) {
                return Err("unsupported0207 mode1 stored selector".into());
            }
            let player = Scene::read(e, selector).word14;
            let role = e.image[P8 + 8 * usize::from(player) + 6];
            if role > 13 {
                return Err("unsupported0207 mode1 player role".into());
            }
            Ok(e.image[0x8716 + usize::from(role)])
        }
        2 => {
            let flip = i32::from(word(e, 0xD0AE)) > i32::from(e.image[0xD0AC] as i8);
            Ok(e.image[0x096E + usize::from((selector < 12) ^ flip)])
        }
        3 => Ok(e.image[0x096C + usize::from(selector < 12)]),
        4 => Ok((u16::from(selector).wrapping_sub(1) % 11 + 47) as u8),
        _ => Err("unsupported0207 mode:original local is undefined above4".into()),
    }
}
pub(crate) fn scene_image(e: &mut Engine, index: u8) -> Result<(), String> {
    let row = Scene::read(e, index);
    let side = usize::from(row.byte7 < 12);
    let value = if matches!(row.byte7, 1 | 12) {
        (u16::from(e.image[0xD0CA + side]) << 8) + 1
    } else {
        let low = low_image(e, index)?;
        (u16::from(e.image[0xD0C8 + side]) << 8) + u16::from(low)
    };
    put(e, address(index) + 4, value);
    Ok(())
}
pub(crate) fn snapshot(e: &Engine, s: &State) -> State {
    let mut out = s.clone();
    out.seed = word(e, 0xA51C);
    out.side_offsets.copy_from_slice(&e.image[0xD56..0xD58]);
    out.actor = e.image[0xD09F];
    out.mode = e.image[MODE];
    out.threshold = e.image[0xD0AC];
    out.clock = word(e, 0xD0AE);
    out.directions.copy_from_slice(&e.image[0xD0C8..0xD0CC]);
    out.direction_signs
        .copy_from_slice(&e.image[0xA338..0xA33A]);
    out.codes.copy_from_slice(&e.image[0x096A..0x0970]);
    out.role_codes.copy_from_slice(&e.image[0x8716..0x8724]);
    for p in &mut out.players {
        p.role = e.image[P8 + 8 * usize::from(p.id) + 6];
    }

    out.scenes = (1..=22).map(|i| Scene::read(e, i)).collect();
    for c in &mut out.cache {
        let a = 0xCF2E + 7 * usize::from(c.index);
        c.word0 = word(e, a);
        c.word2 = word(e, a + 2);
        c.word4 = word(e, a + 4);
    }
    out
}
// [confirmed] F30/033E: all three calls share one keeper scene and ordered byte6 state.
pub(crate) fn cache_image(e: &mut Engine) -> Vec<Value> {
    let mut calls = vec![];
    for i in 0..3 {
        let address = 0xCF2E + 7 * i;
        let a = word(e, address + 2);
        let b = word(e, address);
        let index = e.image[0xD56 + usize::from(e.image[0xD09F] < 12)] + 1;
        let input = Scene::read(e, index);
        let value = code_image(e, index, a, b);
        calls.push(json!({"site":if i<2{"0AB9:0378"}else{"0AB9:03B0"},"arguments":[index,a,b],"input":input,"output":Scene::read(e,index),"result":value,"draw_start":0,"draw_end":0}));
        put(e, address + 4, value.wrapping_add(0x0500));
    }
    calls
}
pub(crate) fn refresh_image(e: &mut Engine, s: &State) -> Result<Outcome, String> {
    let mut events = vec![];
    for index in 1..=22 {
        let input = Scene::read(e, index);
        scene_image(e, index)?;
        events.push(json!({"site":"0AB9:03D4","kind":"scene","arguments":[index],"input":input,"output":Scene::read(e,index),"result":null,"draw_start":0,"draw_end":0}));
    }
    let input = snapshot(e, s);
    let calls = cache_image(e);
    events.push(json!({"site":"0AB9:03DD","kind":"cache","arguments":[],"input":input,"output":snapshot(e,s),"calls":calls,"result":null,"draw_start":0,"draw_end":0}));
    Ok(Outcome {
        output: snapshot(e, s),
        events,
        draws: vec![],
    })
}
pub fn code(s: &CodeInput) -> Result<CodeOutcome, String> {
    let mut e = code_machine(s)?;
    let value = code_image(&mut e, s.scene.index, s.a, s.b);
    Ok(CodeOutcome {
        output: CodeOutput {
            value,
            scene: Scene::read(&e, s.scene.index),
        },
        draws: vec![],
    })
}
pub fn refresh(s: &State) -> Result<Outcome, String> {
    refresh_image(&mut machine(s)?, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    // [designed] Static/synthetic edges are separate from original dynamic coverage.
    fn code_input() -> CodeInput {
        let c: Value = include_str!("../tests/fixtures/scene_codes/cases.jsonl")
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .find(|c| c["stream"] == "codes")
            .unwrap();
        let mut s: CodeInput = serde_json::from_value(c["input"].clone()).unwrap();
        s.scene.index = 3;
        s.scene.byte7 = 2;
        s.scene.byte6 = 0;
        s.a = 0x0201;
        s.b = s.a;
        s.clock = 34;
        s.threshold = 45;
        s.direction_signs = [255, 1];
        s.codes = [10, 20];
        s
    }
    fn sample() -> State {
        let row = code_input().scene;
        State {
            seed: 0xBEEF,
            side_offsets: [0, 11],
            actor: 12,
            directions: [10, 20, 30, 40],
            mode: 3,
            threshold: 45,
            clock: 34,
            direction_signs: [255, 1],
            codes: [10, 20, 30, 40, 50, 60],
            role_codes: std::array::from_fn(|i| 100 + i as u8),
            cache: std::array::from_fn(|i| Cache {
                index: i as u8,
                word0: 0,
                word2: 0,
                word4: 60000,
            }),
            scenes: (1..=22)
                .map(|i| {
                    let mut r = row.clone();
                    r.index = i;
                    r.byte7 = i;
                    r.byte6 = 0;
                    r.word14 = u16::from(i);
                    r.word0 = 0;
                    r.word2 = 0;
                    r
                })
                .collect(),
            players: (0..=22)
                .map(|id| Player {
                    id,
                    role: (id % 14) as u8,
                })
                .collect(),
        }
    }
    #[test]
    fn equal_arguments_return_old_byte6_and_preserve_it() {
        let mut s = code_input();
        s.scene.byte6 = 5;
        let o = code(&s).unwrap();
        assert_eq!(o.output.value, 21);
        assert_eq!(o.output.scene.byte6, 5);
    }
    #[test]
    fn unequal_low_bytes_use_alternate_base_and_logical_not_after_return() {
        let mut s = code_input();
        s.b = 0x0202;
        let o = code(&s).unwrap();
        assert_eq!(o.output.value, 28);
        assert_eq!(o.output.scene.byte6, 1);
        s.scene.byte6 = 255;
        let o = code(&s).unwrap();
        assert_eq!(o.output.value, 283);
        assert_eq!(o.output.scene.byte6, 0);
    }
    #[test]
    fn signed_high_bytes_and_signed_low_word_comparison_choose_offsets() {
        let mut s = code_input();
        s.a = 0x7F00;
        s.b = 0x8000;
        assert_eq!(code(&s).unwrap().output.value, 26);
        s.a = 0x8000;
        s.b = 0x7F00;
        assert_eq!(code(&s).unwrap().output.value, 16);
        s.direction_signs = [128, 128];
        assert_eq!(code(&s).unwrap().output.value, 26);
        s.a = 0x7F00;
        s.b = 0x8000;
        assert_eq!(code(&s).unwrap().output.value, 16);
    }
    #[test]
    fn direction_bytes_are_sign_extended_and_special_ff_test_is_literal() {
        let mut s = code_input();
        s.a = 0x0100;
        s.b = 0x0200;
        s.direction_signs = [255, 255];
        assert_eq!(code(&s).unwrap().output.value, 16);
        s.direction_signs = [128, 128];
        assert_eq!(code(&s).unwrap().output.value, 26);
        s.direction_signs = [0, 0];
        assert_eq!(code(&s).unwrap().output.value, 26);
    }
    #[test]
    fn clock_is_unsigned_phase_is_signed_and_stored_selector_selects_side() {
        let mut s = code_input();
        s.clock = 45;
        assert_eq!(code(&s).unwrap().output.value, 16);
        s.clock = 46;
        assert_eq!(code(&s).unwrap().output.value, 26);
        s.clock = 0;
        s.threshold = 255;
        assert_eq!(code(&s).unwrap().output.value, 26);
        s.scene.byte7 = 12;
        assert_eq!(code(&s).unwrap().output.value, 16);
    }
    #[test]
    fn all_five_modes_select_native_low_codes() {
        for (mode, expected) in [(0, 16), (1, 103), (2, 60), (3, 40), (4, 49)] {
            let mut s = sample();
            s.mode = mode;
            let o = refresh(&s).unwrap();
            assert_eq!(o.output.scenes[2].byte4, expected);
            assert_eq!(o.output.scenes[2].byte5, 20);
        }
    }
    #[test]
    fn mode_zero_uses_word2_then_word0_and_truncates_only_the_low_code() {
        let mut s = sample();
        s.mode = 0;
        s.scenes[2].word2 = 0x7F00;
        s.scenes[2].word0 = 0x8000;
        let o = refresh(&s).unwrap();
        assert_eq!(o.output.scenes[2].byte4, 26);
        assert_eq!(o.output.scenes[2].byte6, 1);
        s.codes[0] = 255;
        s.codes[1] = 255;
        s.scenes[2].word2 = 0x0201;
        s.scenes[2].word0 = 0x0202;
        s.scenes[2].byte6 = 255;
        let o = refresh(&s).unwrap();
        assert_eq!(o.output.scenes[2].byte4, 6);
        assert_eq!(o.output.scenes[2].byte5, 20);
        assert_eq!(o.output.scenes[2].byte6, 0);
        let mut h = code_input();
        h.codes = [255, 255];
        h.scene.byte6 = 255;
        h.a = 0x0201;
        h.b = 0x0202;
        assert_eq!(code(&h).unwrap().output.value, 518);
    }
    #[test]
    fn mode_one_uses_stored_selector_scene_player_not_physical_scene_player() {
        let mut s = sample();
        s.mode = 1;
        s.scenes[2].byte7 = 20;
        let o = refresh(&s).unwrap();
        assert_eq!(o.output.scenes[2].byte4, 106);
        assert_eq!(o.output.scenes[2].byte5, 10);
        assert_eq!(s.players[3].role, 3);
        assert_eq!(s.players[20].role, 6);
    }
    #[test]
    fn mode_two_uses_signed_phase_and_boolean_side_without_direction_byte() {
        let mut s = sample();
        s.mode = 2;
        s.direction_signs = [0, 0];
        s.threshold = 128;
        s.clock = 0;
        assert_eq!(refresh(&s).unwrap().output.scenes[2].byte4, 50);
        s.threshold = 127;
        s.clock = 127;
        assert_eq!(refresh(&s).unwrap().output.scenes[2].byte4, 60);
        s.clock = 128;
        assert_eq!(refresh(&s).unwrap().output.scenes[2].byte4, 50);
        s.scenes[2].byte7 = 20;
        assert_eq!(refresh(&s).unwrap().output.scenes[2].byte4, 60);
    }
    #[test]
    fn mode_three_ignores_clock_and_mode_four_wraps_selector_as_word() {
        let mut s = sample();
        s.mode = 3;
        s.threshold = 128;
        s.clock = 65535;
        s.scenes[2].byte7 = 20;
        assert_eq!(refresh(&s).unwrap().output.scenes[2].byte4, 30);
        s.mode = 4;
        for (selector, value) in [(0, 55), (255, 48), (11, 57), (22, 57)] {
            s.scenes[2].byte7 = selector;
            assert_eq!(refresh(&s).unwrap().output.scenes[2].byte4, value);
        }
    }
    #[test]
    fn keeper_branch_uses_stored_selector_and_keeper_high_byte_in_every_mode() {
        for mode in 0..=4 {
            let mut s = sample();
            s.mode = mode;
            s.scenes[3].byte7 = 1;
            s.scenes[3].byte6 = 255;
            let o = refresh(&s).unwrap();
            assert_eq!(o.output.scenes[3].byte4, 1);
            assert_eq!(o.output.scenes[3].byte5, 40);
            assert_eq!(o.output.scenes[3].byte6, 255);
            assert_eq!(o.output.scenes[11].byte4, 1);
            assert_eq!(o.output.scenes[11].byte5, 30);
        }
    }
    #[test]
    fn scene_to_cache_and_three_cache_calls_share_actual_byte6_handoff() {
        let mut s = sample();
        s.mode = 0;
        s.scenes[0].byte7 = 2;
        s.scenes[0].word2 = 1;
        s.scenes[0].word0 = 2;
        for c in &mut s.cache {
            c.word2 = 1;
            c.word0 = 2;
        }
        s.cache[2].word2 = 0;
        s.cache[2].word0 = 0;
        let o = refresh(&s).unwrap();
        let cache = &o.events[22];
        assert_eq!(o.events[0]["output"]["byte6"], 1);
        assert_eq!(cache["input"]["scenes"][0]["byte6"], 1);
        let calls = cache["calls"].as_array().unwrap();
        assert_eq!(
            calls
                .iter()
                .map(|c| c["result"].as_u64().unwrap())
                .collect::<Vec<_>>(),
            vec![29, 28, 17]
        );
        assert_eq!(calls[0]["output"]["byte6"], 0);
        assert_eq!(calls[1]["input"]["byte6"], 0);
        assert_eq!(calls[2]["input"]["byte6"], 1);
        assert_eq!(o.output.scenes[0].byte6, 1);
        assert_eq!(o.output.scenes[0].byte4, 28);
        assert_eq!(
            o.output.cache.iter().map(|c| c.word4).collect::<Vec<_>>(),
            vec![1309, 1308, 1297]
        );
    }
    #[test]
    fn actor_predicate_and_side_offsets_choose_shared_cache_target() {
        for (actor, index) in [(12, 1), (5, 12), (0, 12), (255, 1)] {
            let mut s = sample();
            s.actor = actor;
            let o = refresh(&s).unwrap();
            for c in o.events[22]["calls"].as_array().unwrap() {
                assert_eq!(c["arguments"][0], index);
            }
        }
        let mut s = sample();
        s.actor = 5;
        s.side_offsets = [11, 0];
        assert_eq!(
            refresh(&s).unwrap().events[22]["calls"][0]["arguments"][0],
            1
        );
    }
    #[test]
    fn undefined_modes_and_reached_invalid_mode_one_selectors_diagnose() {
        for mode in [5, 255] {
            let mut s = sample();
            s.mode = mode;
            assert!(refresh(&s).unwrap_err().contains("undefined"));
        }
        for selector in [0, 23, 255] {
            let mut s = sample();
            s.mode = 1;
            s.scenes[2].byte7 = selector;
            assert!(refresh(&s).unwrap_err().contains("mode1 stored selector"));
        }
    }
    #[test]
    fn canonical_domains_and_expected_result_fields_are_checked() {
        let s = sample();
        let before = serde_json::to_value(&s).unwrap();
        let mut bad = s.clone();
        bad.side_offsets = [0, 0];
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.scenes.pop();
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.scenes.swap(0, 1);
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.players.push(bad.players[0].clone());
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.players[0].role = 14;
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.players[0].id = 3921;
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.players.pop();
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.cache[0].index = 1;
        assert!(refresh(&bad).is_err());
        let mut h = code_input();
        h.scene.index = 0;
        assert!(code(&h).is_err());
        for field in ["output", "expected", "events", "result"] {
            let mut v = serde_json::to_value(&s).unwrap();
            v[field] = json!(0);
            assert!(serde_json::from_value::<State>(v).is_err());
            let mut v = serde_json::to_value(code_input()).unwrap();
            v[field] = json!(0);
            assert!(serde_json::from_value::<CodeInput>(v).is_err());
        }
        let mut v = serde_json::to_value(&s).unwrap();
        v["players"][0]["expected"] = json!(1);
        assert!(serde_json::from_value::<State>(v).is_err());
        assert_eq!(serde_json::to_value(s).unwrap(), before);
    }
    #[test]
    fn reusable_image_path_reads_current_mode_clock_and_player_bytes() {
        let s = sample();
        let mut e = machine(&s).unwrap();
        e.image[MODE] = 1;
        e.image[P8 + 8 * 3 + 6] = 7;
        put(&mut e, 0xD0AE, 901);
        let o = refresh_image(&mut e, &s).unwrap();
        assert_eq!(o.output.mode, 1);
        assert_eq!(o.output.clock, 901);
        assert_eq!(o.output.players[3].role, 7);
        assert_eq!(o.output.scenes[2].byte4, 107);
    }
    #[test]
    fn scalar_code_preserves_every_image_byte_except_selected_byte6() {
        let mut s = code_input();
        s.b = 0x0202;
        let mut e = code_machine(&s).unwrap();
        e.image[0x8000] = 0xAB;
        let before = e.image.clone();
        code_image(&mut e, s.scene.index, s.a, s.b);
        for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
            if a != address(s.scene.index) + 6 {
                assert_eq!(old, new, "unowned byte {a:X}");
            }
        }
        assert_eq!(word(&e, 0xA51C), 0xBEEF);
    }
    #[test]
    fn original_parent_image_ownership_includes_hidden_cache_writes() {
        for line in include_str!("../tests/fixtures/scene_codes/cases.jsonl").lines() {
            let c: Value = serde_json::from_str(line).unwrap();
            if c["stream"] != "parents" {
                continue;
            }
            let s: State = serde_json::from_value(c["input"].clone()).unwrap();
            let mut e = machine(&s).unwrap();
            e.image[0x8000] = 0xAB;
            for i in 0..3 {
                e.image[0xCF2E + 7 * i + 6] = 0xA5;
            }
            let before = e.image.clone();
            refresh_image(&mut e, &s).unwrap();
            let owned: BTreeSet<_> = (1..=22)
                .flat_map(|i| {
                    [
                        0xCCEF + 25 * i + 4,
                        0xCCEF + 25 * i + 5,
                        0xCCEF + 25 * i + 6,
                    ]
                })
                .chain((0..3).flat_map(|i| [0xCF2E + 7 * i + 4, 0xCF2E + 7 * i + 5]))
                .collect();
            for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
                if !owned.contains(&a) {
                    assert_eq!(old, new, "unowned byte {a:X}");
                }
            }
            assert_eq!(word(&e, 0xA51C), s.seed);
        }
    }
}
