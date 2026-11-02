//! [confirmed static] 78B7/DB68 at archaeology218490d; no-key branch only.
//! [designed] Presentation events replace original rendering/hardware work;no dynamic census.
use crate::Draw;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub quick_mode: u8,
    pub mirror_mode: u8,
    pub auto_switch: u8,
    pub remaining: u16,
    pub active: u8,
    pub d0b9: u8,
    pub speaker_gate: u8,
    pub speed: u8,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub presentation: Vec<Value>,
    pub draws: Vec<Draw>,
}
use crate::{
    role_scores::{image, insert, Cells},
    Engine,
};
use serde_json::json;
// [designed] Canonical pointed speed byte,adjacent to but disjoint from scene-code mode.
const SPEED: usize = 0x40081;
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn machine(s: &State) -> Result<Engine, String> {
    let mut c = Cells::new();
    for (a, v) in [
        (0xD074, s.quick_mode),
        (0xD075, s.mirror_mode),
        (0xD1CC, s.auto_switch),
        (0xD0B8, s.active),
        (0xD0B9, s.d0b9),
        (0xA2EC, s.speaker_gate),
        (SPEED, s.speed),
    ] {
        insert(&mut c, a, v)?;
    }
    for (i, b) in s.remaining.to_le_bytes().into_iter().enumerate() {
        insert(&mut c, 0xD0AE + i, b)?;
    }
    Ok(image(c))
}
fn snapshot(e: &Engine) -> State {
    State {
        quick_mode: e.image[0xD074],
        mirror_mode: e.image[0xD075],
        auto_switch: e.image[0xD1CC],
        remaining: word(e, 0xD0AE),
        active: e.image[0xD0B8],
        d0b9: e.image[0xD0B9],
        speaker_gate: e.image[0xA2EC],
        speed: e.image[SPEED],
    }
}
fn notice(site: &str, kind: &str, args: Value) -> Value {
    json!({"site":site,"kind":kind,"arguments":args})
}
// [confirmed static] DB68's logical bytes;presentation buffers become explicit adapter events.
fn switch_image(e: &mut Engine) -> Vec<Value> {
    let mut events = vec![];
    if e.image[0xD074] != 0 || word(e, 0xD0AE) == 0 || e.image[0xD0B9] != 0 {
        return events;
    }
    if e.image[0xD0B8] == 0 {
        if e.image[0xD1CC] == 0 {
            events.push(notice("0AB9:DBA0", "switch-hint", json!([])));
        }
        e.image[0xD1CC] = 1;
    } else {
        e.image[0xD074] = 1;
        e.image[0xD075] = 1;
        e.image[0xD1CC] = 0;
        events.push(notice("0AB9:DBBF", "quick-screen-refresh", json!([1, 0])));
    }
    events
}
// [designed] Caller explicitly supplies the no-pending-key path;keyboard acquisition is external.
// [confirmed static] 791B→79D5,conditionalDB68,then quick-mode speaker helper at79E6.
pub(crate) fn neutral_image(e: &mut Engine) -> Outcome {
    let speed = e.image[SPEED];
    let mut presentation = vec![notice(
        if speed == 0 { "0AB9:78D6" } else { "0AB9:7911" },
        "speed-status",
        json!([speed]),
    )];
    if e.image[0xD1CC] != 0 {
        presentation.extend(switch_image(e));
    }
    if e.image[0xD074] != 0 {
        presentation.push(notice("0AB9:79E6", "speaker-off", json!([])));
    }
    Outcome {
        output: snapshot(e),
        presentation,
        draws: vec![],
    }
}
pub fn neutral(s: &State) -> Result<Outcome, String> {
    Ok(neutral_image(&mut machine(s)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> State {
        State {
            quick_mode: 0,
            mirror_mode: 7,
            auto_switch: 1,
            remaining: 34,
            active: 1,
            d0b9: 0,
            speaker_gate: 1,
            speed: 0,
        }
    }
    #[test]
    fn borrowed_path_reads_current_bytes_and_preserves_unrelated_pools() {
        let mut s = sample();
        s.auto_switch = 0;
        let mut e = machine(&s).unwrap();
        for a in [0x8000, 0x40000, 0x40010, 0x40020, 0x40080, 0x50000] {
            e.image[a] = 0xAB;
        }
        e.image[0xD1CC] = 255;
        let out = neutral_image(&mut e);
        assert_eq!(
            (
                out.output.quick_mode,
                out.output.mirror_mode,
                out.output.auto_switch
            ),
            (1, 1, 0)
        );
        for a in [0x8000, 0x40000, 0x40010, 0x40020, 0x40080, 0x50000] {
            assert_eq!(e.image[a], 0xAB);
        }
        assert_eq!(word(&e, 0xA51C), 0x1234);
        assert_eq!(out.presentation[1]["arguments"], json!([1, 0]));
    }
    #[test]
    fn no_request_does_not_switch_even_if_active_and_watched() {
        let mut s = sample();
        s.auto_switch = 0;
        let out = neutral(&s).unwrap();
        assert_eq!(out.output.quick_mode, 0);
        assert_eq!(out.presentation.len(), 1);
    }
    #[test]
    fn direct_switch_hint_is_distinct_from_neutral_already_requested_path() {
        let mut s = sample();
        s.auto_switch = 0;
        s.active = 0;
        let mut e = machine(&s).unwrap();
        let events = switch_image(&mut e);
        assert_eq!(events, vec![notice("0AB9:DBA0", "switch-hint", json!([]))]);
        assert_eq!(e.image[0xD1CC], 1);
        assert!(switch_image(&mut e).is_empty());
        assert_eq!(neutral_image(&mut e).presentation.len(), 1);
    }
    #[test]
    fn guard_and_word_endpoints_preserve_requested_flags() {
        for (quick, time, guard) in [(1, 34, 0), (0, 0, 0), (0, 65535, 255)] {
            let mut s = sample();
            s.quick_mode = quick;
            s.remaining = time;
            s.d0b9 = guard;
            s.auto_switch = 255;
            let mut e = machine(&s).unwrap();
            let before = e.image.clone();
            assert!(switch_image(&mut e).is_empty());
            assert_eq!(e.image, before);
        }
    }
    #[test]
    fn all_static_cases_only_write_the_three_owned_mode_bytes() {
        for l in include_str!("../tests/fixtures/neutral_input/cases.jsonl").lines() {
            let c: Value = serde_json::from_str(l).unwrap();
            let s: State = serde_json::from_value(c["input"].clone()).unwrap();
            let mut e = machine(&s).unwrap();
            let before = e.image.clone();
            neutral_image(&mut e);
            for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
                if old != new {
                    assert!(
                        [0xD074, 0xD075, 0xD1CC].contains(&a),
                        "unexpected write {a:X}"
                    );
                }
            }
        }
    }
    #[test]
    fn keyboard_present_and_expected_result_fields_are_outside_neutral_input_contract() {
        for field in [
            "key",
            "keyboard_pending",
            "expected",
            "output",
            "presentation",
            "result",
        ] {
            let mut v = serde_json::to_value(sample()).unwrap();
            v[field] = json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
    }
}
