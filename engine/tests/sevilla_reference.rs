//! [confirmed:archaeology 3d41a0db T89] The witnessed Sevilla reference:
//! identity and presentation authenticate, the sealed caller replays to the
//! recorded outcome through the compiled closure, the witnessed friendly
//! selectors refuse on the league route, and the condition facts bind the
//! managed roster. No PSG value enters the Sevilla path or vice versa.
use onenil_engine::watched_compiled::{post_match, sevilla_reference as reference};
use serde_json::{json, Value};

fn source() -> Value {
    serde_json::from_str(include_str!("fixtures/sevilla_post_match/sevilla-result-r0001.json"))
        .unwrap()
}
#[test]
fn the_witnessed_sevilla_caller_replays_to_the_recorded_outcome() {
    let source = source();
    let caller = &source["caller"];
    // caller_replay refuses unless every recorded final field, write, draw and
    // returned register is reproduced; reaching Ok proves the parity set.
    let outcome = reference::caller_replay().unwrap();
    let replayed: std::collections::BTreeMap<String, u64> = serde_json::to_value(&outcome.final_fields)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|field| {
            (
                field["field"].as_str().unwrap_or("").to_owned(),
                field["value"].as_u64().unwrap_or(0),
            )
        })
        .collect();
    for expected in caller["final"].as_array().unwrap() {
        let name = expected["field"].as_str().unwrap();
        assert_eq!(
            replayed.get(name).copied(),
            Some(expected["value"].as_u64().unwrap()),
            "final {name}"
        );
    }
    assert_eq!(outcome.writes.len(), 409);
    assert_eq!(outcome.draws.len(), 107);
    assert_eq!(outcome.registers[0], caller["returned"]["ax"].as_u64().unwrap() as u16);
    assert_eq!(outcome.registers[1], caller["returned"]["cx"].as_u64().unwrap() as u16);
    assert_eq!(outcome.registers[3], caller["returned"]["bx"].as_u64().unwrap() as u16);
}
#[test]
fn the_witnessed_friendly_selectors_refuse_on_the_league_route() {
    // [designed:T91] The Sevilla caller runs with the statistics flag clear;
    // the league guard admits only the statistics-flagged league caller, so
    // the same recorded fields must refuse there - no route confusion.
    let caller = &source()["caller"];
    let entry = &caller["entry"];
    let registers: [u16; 12] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "es", "cs", "ss", "ds"]
        .map(|name| entry[name].as_u64().unwrap() as u16);
    let input: post_match::Input = serde_json::from_value(json!({
        "registers": registers,
        "flags": entry["eflags"],
        "regions": caller["regions"],
        "initial": caller["initial"],
    }))
    .unwrap();
    assert!(post_match::replay(&input).is_err());
}
#[test]
fn the_sevilla_identity_pins_the_witnessed_counts_and_boundaries() {
    let identity = reference::identity().unwrap();
    assert_eq!(identity["counts"]["ticks"], 3927);
    assert_eq!(identity["counts"]["draws"], 169996);
    assert_eq!(identity["counts"]["actions"], 855);
    assert_eq!(identity["entry"]["seed"], 50238);
    assert_eq!(identity["match_return"]["site"], "0AB9:EBE2");
    assert_eq!(identity["match_return"]["seed"], 12917);
    assert_eq!(identity["frame"]["scores"], json!([1, 1]));
    assert_eq!(identity["frame"]["club"], 176);
    assert_eq!(identity["frame"]["opponent"], 130);
    assert_eq!(identity["frame"]["competition"], 1);
    assert_eq!(identity["clubs"]["managed"], 130);
    assert_eq!(identity["clubs"]["opponent"], 176);
    assert_eq!(identity["caller"]["condition_calls"], 36);
    assert_eq!(identity["caller"]["census"]["reads"], 2275);
    assert_eq!(identity["caller"]["census"]["writes"], 409);
    assert_eq!(identity["caller"]["changed_bytes"], 169);
    assert_eq!(identity["boundaries"]["half_time_tick"], 1949);
    assert_eq!(identity["boundaries"]["full_time_tick"], 3881);
    assert_eq!(identity["goals"].as_array().unwrap().len(), 2);
}
#[test]
fn the_witnessed_presentation_carries_the_natural_match() {
    let presentation = reference::presentation().unwrap();
    let identity = reference::identity().unwrap();
    let ticks = presentation["ticks"].as_array().unwrap();
    let actions = presentation["actions"].as_array().unwrap();
    assert_eq!(ticks.len(), 3927);
    assert_eq!(actions.len(), 855);
    assert_eq!(ticks[0]["tick"], 1);
    assert_eq!(ticks[3926]["tick"], 3927);
    assert_eq!(ticks[3926]["scores"], identity["frame"]["scores"]);
    let first = &presentation["first_entry"];
    assert_eq!(first["elapsed"], 0);
    assert_eq!(first["scores"], json!([0, 0]));
    assert_eq!(first["scenes"].as_array().unwrap().len(), 22);
    // The witnessed goals: Blackburn 27', Sevilla 52'; 1-1 at full time.
    assert_eq!(actions.iter().filter(|a| a["code"] == 38).count(), 2);
    assert_eq!(actions.iter().filter(|a| a["code"] == 64).count(), 1);
    assert_eq!(actions.iter().filter(|a| a["code"] == 63).count(), 1);
    assert_eq!(reference::completion_digest().len(), 64);
}
#[test]
fn managed_condition_updates_carry_only_the_witnessed_roster() {
    let updates = reference::managed_condition_updates().unwrap();
    let mut ids: Vec<u16> = updates.iter().map(|(id, _)| *id).collect();
    ids.sort_unstable();
    assert_eq!(ids, [2581, 2582, 2583, 2584, 2585, 2586, 2587, 2588, 2589, 2590, 2591, 2592, 2593, 2594, 2595, 2596, 2597, 2598]);
    let source = source();
    let calls: Vec<(u64, u64)> = source["condition"]["calls"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|call| call["context"] != 0)
        .map(|call| {
            (
                call["input"]["player"].as_u64().unwrap(),
                call["output"]["condition"].as_u64().unwrap(),
            )
        })
        .collect();
    for (player, condition) in &calls {
        assert!(
            updates.contains(&(*player as u16, *condition as u8)),
            "witnessed outcome missing for {player}"
        );
    }
}
