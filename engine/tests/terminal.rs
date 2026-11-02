use onenil_engine::terminal::{self, State};
use serde_json::Value;
use std::{fs, path::PathBuf};
fn cases() -> Vec<Value> {
    fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/terminal/cases.jsonl"),
    )
    .unwrap()
    .lines()
    .map(|s| serde_json::from_str(s).unwrap())
    .collect()
}
#[test]
fn actual_original_terminal_cases_replay() {
    for c in cases() {
        let input: State = serde_json::from_value(c["input"].clone()).unwrap();
        let got = terminal::positions(&input).unwrap();
        assert_eq!(serde_json::to_value(got).unwrap(), c["positions"]);
        let input2: State = serde_json::from_value(c["proximity_input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(terminal::proximity(&input2).unwrap()).unwrap(),
            c["proximity"]
        );
        assert_eq!(
            serde_json::to_value(terminal::terminal(&input).unwrap()).unwrap(),
            serde_json::json!({"positions":c["positions"],"proximity":c["proximity"]})
        );
    }
}

// [designed] Synthetic static-branch tests below are not additional original coverage.
fn synthetic() -> State {
    let mut s: State = serde_json::from_value(cases()[0]["input"].clone()).unwrap();
    s.actor = 22;
    s.target = [0, 0];
    s.phase = 0;
    s.time = 0;
    s.signs = [255, 1];
    s.players = (1..=22)
        .map(|id| terminal::Player {
            id,
            position: id as u8,
            byte8: 10,
            flags11: 0,
        })
        .collect();
    s.positions = (1..=22)
        .map(|position| terminal::Position {
            position,
            word: u16::from(100 + position) << 8,
        })
        .collect();
    for r in &mut s.scenes {
        r.word14 = u16::from(r.index);
        r.byte7 = r.index;
        r.word0 = 0;
        r.word10 = 0x7777;
        r.byte19 = 0;
    }
    s
}
fn table(s: &mut State, id: u8, value: u16) {
    s.positions
        .iter_mut()
        .find(|p| p.position == id)
        .unwrap()
        .word = value
}
fn only(s: &mut State, ids: &[u16]) {
    for p in &mut s.players {
        p.byte8 = if ids.contains(&p.id) { 10 } else { 0 }
    }
}
fn xy(x: i8, y: i8) -> u16 {
    u16::from_le_bytes([x as u8, y as u8])
}
fn stores(out: &terminal::ProximityOutcome, kind: &str) -> Vec<Value> {
    out.visits
        .iter()
        .flat_map(|v| v["stores"].as_array().unwrap())
        .filter(|s| s["kind"] == kind)
        .cloned()
        .collect()
}
#[test]
fn synthetic_near_duplicate_wraps_previous_low_byte_without_borrow() {
    let mut s = synthetic();
    table(&mut s, 12, 0x1200);
    table(&mut s, 13, 0x1200);
    let out = terminal::positions(&s).unwrap();
    assert_eq!(out.output.scenes[11].word10, 0x12fd);
    assert_eq!(out.output.scenes[12].word10, 0x1203);
    assert_eq!(out.visits[1]["branch"], "0AB9:3235");
    assert_eq!(out.visits[1]["previous"], 12);
}
#[test]
fn synthetic_wide_duplicate_sign_and_previous_high_byte_wrap() {
    for (value, signs, current, previous) in [
        (0x1206, [255, 1], 0x1203, 0x0f06),
        (0x12fa, [255, 1], 0x12fd, 0x0ffa),
        (0x1280, [255, 1], 0x1283, 0x0f80),
        (0x0206, [255, 1], 0x0203, 0xff06),
        (0x1206, [1, 255], 0x1209, 0x0f06),
        (0x1206, [255, 128], 0x1286, 0x0f06),
    ] {
        let mut s = synthetic();
        s.signs = signs;
        table(&mut s, 12, value);
        table(&mut s, 13, value);
        let out = terminal::positions(&s).unwrap();
        assert_eq!(out.output.scenes[12].word10, current);
        assert_eq!(out.output.scenes[11].word10, previous);
        assert_eq!(out.visits[1]["branch"], "0AB9:3223");
    }
}
#[test]
fn synthetic_first_scene_never_compares_previous_side_tail() {
    let mut s = synthetic();
    table(&mut s, 22, 0x1200);
    table(&mut s, 1, 0x1200);
    let out = terminal::positions(&s).unwrap();
    assert_eq!(out.output.scenes[21].word10, 0x1200);
    assert_eq!(out.output.scenes[0].word10, 0x1200);
    assert_eq!(out.visits[11]["previous"], 0);
    assert!(out.visits[11]["branch"].is_null());
}
#[test]
fn synthetic_duplicate_uses_current_previous_row_not_its_table_value() {
    let mut s = synthetic();
    for id in [12, 13, 14] {
        table(&mut s, id, 0x1200)
    }
    let out = terminal::positions(&s).unwrap();
    assert_eq!(out.output.scenes[12].word10, 0x1203);
    assert_eq!(out.output.scenes[13].word10, 0x1200);
    assert!(out.visits[2]["branch"].is_null());
}
#[test]
fn synthetic_canonical_shared_player_reads_one_position_cell() {
    let mut s = synthetic();
    s.scenes[12].word14 = 12;
    table(&mut s, 12, 0x1200);
    let out = terminal::positions(&s).unwrap();
    assert_eq!(out.output.scenes[11].word10, 0x12fd);
    assert_eq!(out.output.scenes[12].word10, 0x1203);
}
#[test]
fn synthetic_near_tie_keeps_first_and_threshold_is_unsigned_strict() {
    let mut s = synthetic();
    only(&mut s, &[2, 3]);
    s.scenes[1].word0 = xy(-5, 0);
    s.scenes[2].word0 = xy(5, 0);
    let out = terminal::proximity(&s).unwrap();
    assert_eq!(out.output.selectors.near[1], 2);
    assert_eq!(stores(&out, "near").len(), 1);
    for x in [-100, -128] {
        let mut s = synthetic();
        only(&mut s, &[2]);
        s.scenes[1].word0 = xy(x, 0);
        let out = terminal::proximity(&s).unwrap();
        assert_eq!(out.output.selectors.near[1], 1);
        assert!(stores(&out, "near").is_empty())
    }
}
#[test]
fn synthetic_ahead_uses_signed_y_direction_and_keeps_distance_tie() {
    let mut s = synthetic();
    only(&mut s, &[2, 3]);
    s.scenes[1].word0 = xy(0, -1);
    s.scenes[2].word0 = xy(0, -1);
    let out = terminal::proximity(&s).unwrap();
    assert_eq!(out.output.selectors.ahead[1], 2);
    assert_eq!(stores(&out, "ahead").len(), 1);
    s.phase = 128;
    s.time = 0;
    s.scenes[1].word0 = xy(0, 1);
    s.scenes[2].word0 = xy(0, 1);
    let out = terminal::proximity(&s).unwrap();
    assert_eq!(out.output.selectors.ahead[1], 2);
    assert_eq!(out.visits[10]["helpers"][1]["result"], 255);
}
#[test]
fn synthetic_direction_time_boundary_and_independent_scene_selector() {
    let mut s = synthetic();
    only(&mut s, &[2]);
    s.phase = 45;
    s.time = 45;
    s.scenes[1].byte7 = 13;
    let out = terminal::proximity(&s).unwrap();
    assert_eq!(out.output.selectors.near[1], 13);
    assert_eq!(out.visits[10]["scene"], 2);
    assert_eq!(out.visits[10]["helpers"][1]["result"], 255);
    s.time = 46;
    let out = terminal::proximity(&s).unwrap();
    assert_eq!(out.visits[10]["helpers"][1]["result"], 1);
}
#[test]
fn synthetic_actor_byte19_and_unavailable_skip_direction_and_stores() {
    for skip in 0..7 {
        let mut s = synthetic();
        only(&mut s, &[2]);
        match skip {
            0 => s.actor = 2,
            1 => s.scenes[1].byte19 = 13,
            2 => s.players[1].byte8 = 0,
            3 => s.players[1].byte8 = 128,
            4 => s.players[1].flags11 = 0x80,
            5 => s.players[1].flags11 = 0x26,
            _ => {
                s.players[1].id = 0;
                s.scenes[1].word14 = 0
            }
        }
        let out = terminal::proximity(&s).unwrap();
        assert_eq!(out.visits[10]["helpers"].as_array().unwrap().len(), 1);
        assert!(out.visits[10]["stores"].as_array().unwrap().is_empty());
        assert_eq!(out.output.selectors.near[1], 1);
    }
}
#[test]
fn synthetic_signed_product_extremes_and_distance_domain() {
    let mut s = synthetic();
    only(&mut s, &[2]);
    s.signs = [128, 128];
    s.target = [0, 127];
    s.scenes[1].word0 = xy(0, -128);
    // Signed-byte products fit i16; distance255 excludes stores at the extreme.
    let out = terminal::proximity(&s).unwrap();
    assert!(stores(&out, "ahead").is_empty());
    assert_eq!(out.visits[10]["helpers"][1]["result"], 128);
    s.target = [0, 0];
    s.scenes[1].word0 = xy(0, 1);
    let out = terminal::proximity(&s).unwrap();
    assert_eq!(out.output.selectors.ahead[1], 2);
}
#[test]
fn synthetic_composition_keeps_actual_position_writes_and_all_other_fields() {
    let mut s = synthetic();
    s.scenes[1].word10 = 0xabcd;
    s.scenes[1].word2 = 0xfedc;
    let original = s.clone();
    let out = terminal::terminal(&s).unwrap();
    assert_ne!(out.positions.output.scenes[1].word10, 0xabcd);
    assert_eq!(
        out.proximity.visits[10]["input"]["word10"],
        out.positions.output.scenes[1].word10
    );
    assert_eq!(out.proximity.output.scenes[1].word2, 0xfedc);
    assert_eq!(out.proximity.output.seed, s.seed);
    assert!(out.positions.draws.is_empty() && out.proximity.draws.is_empty());
    assert_eq!(s, original);
    for (before, after) in s.scenes.iter().zip(&out.positions.output.scenes) {
        let mut after = after.clone();
        after.word10 = before.word10;
        assert_eq!(before, &after)
    }
}
#[test]
fn synthetic_swapped_physical_offsets_have_independent_defaults() {
    let mut s = synthetic();
    s.side_offsets = [0, 11];
    only(&mut s, &[]);
    let out = terminal::proximity(&s).unwrap();
    assert_eq!(out.output.selectors.near, [1, 12]);
    assert_eq!(out.output.selectors.ahead, [0, 0]);
}
#[test]
fn malformed_states_diagnose_before_mutating_input() {
    for damage in 0..10 {
        let mut s = synthetic();
        match damage {
            0 => s.side_offsets = [0, 0],
            1 => s.actor = 0,
            2 => {
                s.scenes.pop();
            }
            3 => s.scenes[0].index = 2,
            4 => s.scenes[0].byte7 = 0,
            5 => s.scenes[0].word14 = 3921,
            6 => {
                s.players.pop();
            }
            7 => s.players.push(s.players[0].clone()),
            8 => {
                s.positions.pop();
            }
            _ => s.positions.push(s.positions[0].clone()),
        }
        let before = s.clone();
        assert!(terminal::terminal(&s).is_err());
        assert_eq!(s, before);
    }
    let mut value = serde_json::to_value(synthetic()).unwrap();
    for key in ["output", "expected", "visits"] {
        value[key] = serde_json::json!({});
        assert!(serde_json::from_value::<State>(value.clone()).is_err());
        value.as_object_mut().unwrap().remove(key);
    }
}
#[test]
fn fixture_hashes_bind_originals_indexes_and_projection() {
    use sha2::{Digest, Sha256};
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/terminal");
    let pin: Value =
        serde_json::from_slice(&fs::read(root.join("provenance.json")).unwrap()).unwrap();
    assert_eq!(pin["commit"], "218490d846e365babdd158d09c5ab865124e30ed");
    assert_eq!(
        pin["repository"],
        "https://github.com/diegoami/onenil-archaeology"
    );
    assert_eq!(
        pin["projection_sha256"],
        format!(
            "{:x}",
            Sha256::digest(fs::read(root.join("cases.jsonl")).unwrap())
        )
    );
    for (name, hash) in pin["indices"].as_object().unwrap() {
        let path = root.join(name);
        let data = fs::read(&path).unwrap();
        assert_eq!(
            hash.as_str().unwrap(),
            format!("{:x}", Sha256::digest(&data))
        );
        let index: Value = serde_json::from_slice(&data).unwrap();
        for meta in index["records"].as_array().unwrap() {
            let data =
                fs::read(path.parent().unwrap().join(meta["cases"].as_str().unwrap())).unwrap();
            assert_eq!(meta["cases_sha256"], format!("{:x}", Sha256::digest(data)));
        }
    }
}

#[test]
fn changed_expected_observations_cannot_steer_native_computation() {
    let mut c = cases()[0].clone();
    let input: State = serde_json::from_value(c["input"].clone()).unwrap();
    let before = serde_json::to_value(terminal::terminal(&input).unwrap()).unwrap();
    c["positions"]["output"]["scenes"][0]["word10"] = serde_json::json!(0);
    c["positions"]["visits"][1]["branch"] = serde_json::json!("injected");
    c["proximity"]["output"]["selectors"]["near"][0] = serde_json::json!(0);
    let input: State = serde_json::from_value(c["input"].clone()).unwrap();
    let after = serde_json::to_value(terminal::terminal(&input).unwrap()).unwrap();
    assert_eq!(before, after);
    assert_ne!(after["positions"], c["positions"]);
    assert_ne!(after["proximity"], c["proximity"]);
}
