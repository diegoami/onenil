//! [confirmed] F22/F23 at archaeology218490d: fixed-reference6B2F composition.
use crate::{
    ai_selection::{Context, ReadSet},
    Draw,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub context: Context,
    pub replacement: Option<ReadSet>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: Context,
    pub replacement: Option<ReadSet>,
    pub events: Vec<Value>,
    pub ai: Vec<Value>,
    pub presentation: Vec<Value>,
    pub draws: Vec<Draw>,
}
use crate::{ai_selection, cache_helpers, placement, rearrangement, role_scores, Engine};
use serde_json::json;
fn addr(i: u8) -> usize {
    0xCCEF + 25 * usize::from(i)
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn input(s: &State) -> ai_selection::State {
    ai_selection::State {
        context: s.context.clone(),
        replacement: s.replacement.clone(),
        mode: 0,
        side: 0,
        scene: s.context.side_offsets[0].saturating_add(1),
    }
}
fn machine(s: &State) -> Result<Engine, String> {
    Ok(role_scores::image(ai_selection::cells(&input(s))?))
}
fn compact(s: &Context) -> Value {
    let mut v = serde_json::to_value(s).unwrap();
    let object = v.as_object_mut().unwrap();
    for key in [
        "limit_cell",
        "style_text",
        "preferences",
        "candidates",
        "cache_selectors",
    ] {
        object.remove(key);
    }
    for p in object.get_mut("players").unwrap().as_array_mut().unwrap() {
        let p = p.as_object_mut().unwrap();
        for key in ["position", "style", "hand_selector", "hand_cell"] {
            p.remove(key);
        }
    }
    v
}
fn capacity(e: &Engine, side: usize) -> u16 {
    u16::from(e.image[0xD6E0 + side])
        .wrapping_add(u16::from(e.image[0xD6DD]))
        .wrapping_sub(u16::from(e.image[0xD6DE + side]))
}
fn cap_event(e: &Engine, side: usize, site: &str) -> Value {
    json!({"site":site,"kind":"capacity","side":side,"argument":side,"allowance":e.image[0xD6DD],"sub_flag":e.image[0xD6DE+side],"reserves":e.image[0xD6E0+side],"result":capacity(e,side)})
}
fn available(e: &Engine, id: u16) -> u8 {
    let a = crate::layout::P15 + 15 * usize::from(id);
    placement::unavailable(&placement::Player {
        id,
        p15_sbyte8: e.image[a + 8] as i8,
        p15_byte11: e.image[a + 11],
    })
}
fn run(e: &mut Engine, s: &State) -> Result<Outcome, String> {
    let mut events = vec![];
    let mut calls = vec![];
    let mut presentation = vec![];
    for side in 0..2 {
        let club = word(e, 0xD6E4 + 2 * side);
        let managed = cache_helpers::managed_image(e, club);
        events.push(json!({"site":"0AB9:6B48","kind":"managed","side":side,"argument":club,"managed_clubs":[e.image[0xDCBA],e.image[0xDCBB]],"result":managed}));
        let offset = e.image[0xD56 + side];
        if managed != 0 {
            let id = word(e, addr(offset + 1) + 14);
            let unavailable = available(e, id);
            let a = crate::layout::P15 + 15 * usize::from(id);
            events.push(json!({"site":"0AB9:6B68","kind":"keeper","side":side,"argument":id,"byte8":e.image[a+8],"flags11":e.image[a+11],"result":unavailable}));
            if e.image[0xD6E2] != 0 {
                if unavailable == 0 {
                    if word(e, 0xD0AE).wrapping_add(word(e, 0xD0B4)) == 0 {
                        continue;
                    }
                    events.push(cap_event(e, side, "0AB9:6B8C"));
                    if capacity(e, side) == 0 {
                        continue;
                    }
                }
                events.push(cap_event(e, side, "0AB9:6B99"));
                return Err(
                    "unsupported reached6A7A prompt/D5D6 editor;explicit UI response required"
                        .into(),
                );
            }
        } else {
            events.push(cap_event(e, side, "0AB9:6BD5"));
            if capacity(e, side) == 0 {
                continue;
            }
            for slot in (1..=11).rev() {
                let index = offset + slot;
                let id = word(e, addr(index) + 14);
                let a = crate::layout::P15 + 15 * usize::from(id);
                if e.image[a + 11] & 0x20 != 0 || (e.image[a + 8] as i8) <= 0 {
                    continue;
                }
                let remaining = word(e, 0xD0AE);
                if remaining <= 1 || slot <= 1 || remaining >= 35 {
                    continue;
                }
                let total = word(e, 0xD146 + 40 * side + 2 * usize::from(slot)) as i16;
                if i32::from(total) >= i32::from(90 - remaining) {
                    continue;
                }
                let selector = e.image[addr(index) + 7];
                if selector <= offset || selector > offset + 11 {
                    return Err("unsupported6B2F stored-selector side domain".into());
                }
                let state = ai_selection::State {
                    context: s.context.clone(),
                    replacement: s.replacement.clone(),
                    mode: 0,
                    side: side as u8,
                    scene: selector,
                };
                let before = ai_selection::snapshot(e, &s.context);
                let out = ai_selection::run(e, &state)?;
                calls.push(json!({"event":events.len(),"input":before,"output":out.output,"helpers":out.helpers,"presentation":out.presentation,"draws":out.draws}));
                presentation.extend(out.presentation);
                events.push(json!({"site":"0AB9:6CA5","kind":"ai","side":side,"arguments":[0,side,selector,club],"input":compact(&before),"output":compact(&out.output),"draw_start":0,"draw_end":0}));
                if e.image[0xD6E0 + side] == 0 {
                    break;
                }
            }
        }
    }
    let replacement = s.replacement.as_ref().map(|r| ReadSet {
        match_state: rearrangement::snapshot(e, &r.match_state),
        display_mode: e.image[0xD074],
        position_sentinel: word(e, 0xD2E),
        speaker_gate: e.image[0xA2EC],
    });
    Ok(Outcome {
        output: ai_selection::snapshot(e, &s.context),
        replacement,
        events,
        ai: calls,
        presentation,
        draws: vec![],
    })
}
pub fn dispatch(s: &State) -> Result<Outcome, String> {
    run(&mut machine(s)?, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample(step: u16) -> State {
        let c: Value = include_str!("../tests/fixtures/substitution/cases.jsonl")
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .find(|c| c["step"] == step)
            .unwrap();
        serde_json::from_value(c["input"].clone()).unwrap()
    }
    fn put(e: &mut Engine, a: usize, v: u16) {
        e.image[a..a + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn prepare(e: &mut Engine, reserves: u8) {
        put(e, 0xD0AE, 34);
        e.image[0xD6E1] = reserves;
        e.image[0xD6DD] = 1;
        e.image[0xD6DF] = 0;
        e.image[0xBF6 + 16] = 0;
        for slot in 1..=11 {
            let id = word(e, addr(slot) + 14);
            e.image[crate::layout::P8 + 8 * usize::from(id) + 6] = 2;
            e.image[crate::layout::P15 + 15 * usize::from(id) + 8] = 1;
            e.image[crate::layout::P15 + 15 * usize::from(id) + 11] = 0;
            put(e, 0xD146 + 40 + 2 * usize::from(slot), 0);
        }
    }
    #[test]
    fn capacity_is_unsigned_word_wrapping_not_saturating_or_signed() {
        let s = sample(0);
        let mut e = machine(&s).unwrap();
        for (reserve, allowance, flags, want) in [
            (0, 0, 1, 65535),
            (255, 255, 0, 510),
            (255, 0, 255, 0),
            (0, 1, 1, 0),
            (255, 1, 0, 256),
        ] {
            e.image[0xD6E1] = reserve;
            e.image[0xD6DD] = allowance;
            e.image[0xD6DF] = flags;
            assert_eq!(capacity(&e, 1), want);
        }
    }
    #[test]
    fn managed_pending_gate_precedes_all_ui_paths_even_for_unavailable_keeper() {
        let s = sample(0);
        let mut e = machine(&s).unwrap();
        let id = word(&e, addr(12) + 14);
        let a = crate::layout::P15 + 15 * usize::from(id);
        e.image[a + 8] = 0;
        e.image[0xD6E2] = 0;
        let out = run(&mut e, &s).unwrap();
        assert_eq!(out.events[1]["result"], 1);
        assert_eq!(out.output.pending, 0);
        e.image[0xD6E2] = 1;
        assert!(run(&mut e, &s).unwrap_err().contains("6A7A"));
    }
    #[test]
    fn managed_available_time_wrap_and_zero_capacity_skip_prompt_but_eligible_ui_diagnoses() {
        let s = sample(0);
        let mut e = machine(&s).unwrap();
        e.image[0xD6E2] = 1;
        put(&mut e, 0xD0AE, 1);
        put(&mut e, 0xD0B4, 65535);
        let o = run(&mut e, &s).unwrap();
        assert!(!o.events.iter().any(|e| e["site"] == "0AB9:6B8C"));
        put(&mut e, 0xD0AE, 92);
        put(&mut e, 0xD0B4, 0);
        e.image[0xD6E0] = 0;
        e.image[0xD6DE] = 1;
        let o = run(&mut e, &s).unwrap();
        assert_eq!(o.events[2]["site"], "0AB9:6B8C");
        assert_eq!(o.events[2]["result"], 0);
        e.image[0xD6DE] = 0;
        assert!(run(&mut e, &s)
            .unwrap_err()
            .contains("explicit UI response"));
    }
    #[test]
    fn both_sides_are_visited_in_order_and_ai_scan_is_reverse_physical_slots() {
        let s = sample(0);
        let mut e = machine(&s).unwrap();
        prepare(&mut e, 2);
        let out = run(&mut e, &s).unwrap();
        let managed: Vec<_> = out
            .events
            .iter()
            .filter(|e| e["kind"] == "managed")
            .map(|e| e["side"].as_u64().unwrap())
            .collect();
        assert_eq!(managed, vec![0, 1]);
        let calls: Vec<_> = out
            .events
            .iter()
            .filter(|e| e["kind"] == "ai")
            .map(|e| e["arguments"][2].as_u64().unwrap())
            .collect();
        assert_eq!(calls, (2..=11).rev().collect::<Vec<_>>());
    }
    #[test]
    fn ai_uses_only_mask20_and_signed_positive_byte8_before_selection() {
        let s = sample(0);
        for (flags, byte8, count) in [
            (0x20, 1, 0),
            (0, 0, 0),
            (0, 128, 0),
            (0, 255, 0),
            (0x80, 1, 10),
            (0, 127, 10),
        ] {
            let mut e = machine(&s).unwrap();
            prepare(&mut e, 2);
            for slot in 2..=11 {
                let id = word(&e, addr(slot) + 14);
                let a = crate::layout::P15 + 15 * usize::from(id);
                e.image[a + 8] = byte8;
                e.image[a + 11] = flags;
            }
            assert_eq!(run(&mut e, &s).unwrap().ai.len(), count);
        }
    }
    #[test]
    fn ai_time_endpoints_and_signed_event_total_strict_comparison() {
        let s = sample(0);
        for (time, total, count) in [
            (1, 0, 0),
            (2, 87, 10),
            (2, 88, 0),
            (34, 55, 10),
            (34, 56, 0),
            (34, 65535, 10),
            (34, 32767, 0),
            (35, 0, 0),
            (65535, 65535, 0),
        ] {
            let mut e = machine(&s).unwrap();
            prepare(&mut e, 2);
            put(&mut e, 0xD0AE, time);
            for slot in 2..=11 {
                put(&mut e, 0xD146 + 40 + 2 * slot, total);
            }
            assert_eq!(run(&mut e, &s).unwrap().ai.len(), count);
        }
    }
    #[test]
    fn stored_selector_is_passed_instead_of_loop_index_and_bad_side_mapping_diagnoses() {
        let s = sample(0);
        let mut e = machine(&s).unwrap();
        prepare(&mut e, 0);
        e.image[addr(11) + 7] = 10;
        let out = run(&mut e, &s).unwrap();
        let h = out.events.iter().find(|h| h["kind"] == "ai").unwrap();
        assert_eq!(h["arguments"], json!([0, 1, 10, 134]));
        e.image[addr(11) + 7] = 22;
        assert!(run(&mut e, &s).unwrap_err().contains("stored-selector"));
    }
    #[test]
    fn zero_reserves_stop_immediately_after_ai_even_when_it_makes_no_change() {
        let s = sample(0);
        let mut e = machine(&s).unwrap();
        prepare(&mut e, 0);
        let out = run(&mut e, &s).unwrap();
        assert_eq!(out.ai.len(), 1);
        assert!(out.ai[0]["helpers"].as_array().unwrap().is_empty());
        assert_eq!(out.output.reserves[1], 0);
    }
    #[test]
    fn five_selection_calls_share_replacement_state_and_live_counter_mutations() {
        let s = sample(57);
        let mut e = machine(&s).unwrap();
        let id = s.context.clubs[1].roster[13];
        put(&mut e, crate::layout::P15 + 15 * usize::from(id), 65535);
        let out = run(&mut e, &s).unwrap();
        assert_eq!(out.ai.len(), 5);
        assert_eq!(
            out.ai
                .iter()
                .flat_map(|c| c["helpers"].as_array().unwrap())
                .filter(|h| h["kind"] == "replace")
                .count(),
            2
        );
        assert_eq!(word(&e, crate::layout::P15 + 15 * usize::from(id)), 0);
        assert_eq!(out.output.reserves[1], 0);
        let final_state = out.replacement.unwrap();
        let p = final_state
            .match_state
            .core
            .players
            .iter()
            .find(|p| p.id == id)
            .unwrap();
        assert_eq!(&p.p15[..2], &[0, 0]);
    }
    #[test]
    fn missing_reached_read_set_aliases_and_expected_outputs_reject() {
        let mut s = sample(57);
        s.replacement = None;
        let before = serde_json::to_value(&s).unwrap();
        assert!(dispatch(&s)
            .unwrap_err()
            .contains("without full raw read set"));
        assert_eq!(serde_json::to_value(&s).unwrap(), before);
        let mut s = sample(57);
        s.replacement.as_mut().unwrap().match_state.core.players[0].p8[6] ^= 1;
        assert!(machine(&s).is_err());
        let mut s = sample(0);
        s.context.side_offsets = [255, 0];
        assert!(dispatch(&s).is_err());
        for field in ["output", "expected", "events", "ai", "chosen", "result"] {
            let mut v = serde_json::to_value(sample(0)).unwrap();
            v[field] = json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
    }
    #[test]
    fn no_ai_dispatch_preserves_every_byte_and_seed() {
        let s = sample(0);
        let mut e = machine(&s).unwrap();
        e.image[0x8000] = 0xAB;
        let before = e.image.clone();
        let out = run(&mut e, &s).unwrap();
        assert!(out.ai.is_empty());
        assert_eq!(e.image, before);
        assert_eq!(word(&e, 0xA51C), s.context.seed);
    }
    #[test]
    fn shared_parent_image_write_ownership_includes_only_reached_selection_replacement_cells() {
        let s = sample(57);
        let mut e = machine(&s).unwrap();
        let before = e.image.clone();
        let new_ids = [s.context.clubs[1].roster[13], s.context.clubs[1].roster[12]];
        let old_ids = [s.context.scenes[7].player, s.context.scenes[5].player];
        run(&mut e, &s).unwrap();
        for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
            if old == new {
                continue;
            }
            let scene = (1..=22).any(|i| a >= addr(i) && a < addr(i) + 25);
            let roster = s.context.clubs.iter().any(|c| {
                let base = crate::layout::CLUB + 62 * usize::from(c.id - 1);
                a >= base && a < base + 40
            });
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
            let player = new_ids.iter().any(|id| {
                a == crate::layout::P8 + 8 * usize::from(*id) + 6
                    || [0, 1, 4, 6, 10]
                        .iter()
                        .any(|i| a == crate::layout::P15 + 15 * usize::from(*id) + i)
            }) || old_ids
                .iter()
                .any(|id| a == crate::layout::P15 + 15 * usize::from(*id) + 11);
            assert!(
                scene || roster || cache || globals || local || player,
                "unexpected write {a:X}"
            );
        }
        assert_eq!(word(&e, 0xA51C), s.context.seed);
    }
}
