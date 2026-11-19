//! [confirmed:archaeology 3d41a0db T89] The witnessed Sevilla reference.
//!
//! The capture recorded no machine-state admission for this fixture: the
//! witnessed match presents through the pinned identity and presentation, and
//! the sealed post-match caller replays from its recorded initial fields plus
//! the pool-pointer headers the record's own machine declaration implies (the
//! T88 binding principle; the identical bases are declared by both witnessed
//! callers). No expected match state, seed or outcome initializes anything; the
//! week-0 PSG records never load here. The research capture's emulated-hardware
//! PIT restoration stays a research concept and appears nowhere in this module.
use super::post_match::{self, Input};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const IDENTITY: &[u8] = include_bytes!("../../tests/fixtures/sevilla_watched/identity.json");
const PRESENTATION: &[u8] =
    include_bytes!("../../tests/fixtures/sevilla_watched/presentation-r0001.json");
const CALLER: &[u8] =
    include_bytes!("../../tests/fixtures/sevilla_post_match/sevilla-result-r0001.json");

pub const IDENTITY_SHA256: &str =
    "316583caaaa639cd037eb2026b142f7a510bf2fc922608ec7466bea04577316c";
pub const PRESENTATION_SHA256: &str =
    "939938d33d5b9cc4682cc285ead3c71270de6b2f438f159747bde59c30065053";
pub const CALLER_SHA256: &str = "b33901b1d30ceb2a7e9907982bdbe539172dabbec025bd113e313f47fccff509";

/// The DS pool-pointer header slots, as `binding` canonicalises them for the
/// fresh route; the witnessed callers declare the identical nine pool bases.
const POINTER_SLOTS: [(&str, u16); 9] = [
    ("player8", 0xD7BA),
    ("player15", 0xD7BE),
    ("club", 0xD7C2),
    ("club_stats", 0xD7C6),
    ("fixture", 0xD7CA),
    ("history", 0xD7CE),
    ("game", 0xD7D2),
    ("club_aux", 0xDCC0),
    ("staff", 0xDCC8),
];

fn sealed(data: &[u8], pin: &str, name: &str) -> Result<Value, String> {
    if format!("{:x}", Sha256::digest(data)) != pin {
        return Err(format!("sealed Sevilla {name} digest differs"));
    }
    serde_json::from_slice(data).map_err(|error| format!("sealed Sevilla {name} differs: {error}"))
}

/// The pinned identity: counts, boundaries, clubs, caller census and the
/// witnessed fixture facts. Refuses a foreign or tampered document.
pub fn identity() -> Result<Value, String> {
    let identity = sealed(IDENTITY, IDENTITY_SHA256, "identity")?;
    if identity["schema"] != "sevilla-watched-identity/1" {
        return Err("sealed Sevilla identity schema differs".into());
    }
    if identity["clubs"]["managed"] != 130
        || identity["clubs"]["opponent"] != 176
        || identity["clubs"]["competition"] != 1
    {
        return Err("sealed Sevilla identity clubs differ".into());
    }
    if identity["frame"]["scores"] != json!([1, 1])
        || identity["match_return"]["site"] != "0AB9:EBE2"
    {
        return Err("sealed Sevilla identity outcome differs".into());
    }
    Ok(identity)
}

/// The pinned witnessed presentation: the compact playback fields of the
/// natural match, digest-pinned by the identity.
pub fn presentation() -> Result<Value, String> {
    let presentation = sealed(PRESENTATION, PRESENTATION_SHA256, "presentation")?;
    let identity = identity()?;
    let counts = &identity["counts"];
    if presentation["schema"] != "sevilla-watched-presentation/1" {
        return Err("sealed Sevilla presentation schema differs".into());
    }
    let ticks = presentation["ticks"]
        .as_array()
        .ok_or("presentation ticks differ")?;
    let actions = presentation["actions"]
        .as_array()
        .ok_or("presentation actions differ")?;
    if ticks.len() != counts["ticks"].as_u64().unwrap() as usize
        || actions.len() != counts["actions"].as_u64().unwrap() as usize
    {
        return Err("sealed Sevilla presentation counts differ".into());
    }
    if ticks.first().ok_or("empty presentation")?["tick"] != 1
        || ticks.last().ok_or("empty presentation")?["tick"] != counts["ticks"]
    {
        return Err("sealed Sevilla presentation tick chain differs".into());
    }
    Ok(presentation)
}

/// The witnessed completion digest: one identity over the sealed reference.
pub fn completion_digest() -> String {
    let mut hasher = Sha256::new();
    hasher.update(IDENTITY);
    hasher.update(PRESENTATION);
    hasher.update(CALLER);
    format!("{:x}", hasher.finalize())
}

/// The pool-pointer header values the record's machine declaration implies.
fn pointer_headers(regions: &Value) -> Result<Vec<(u16, u16, u16)>, String> {
    let declared = |name: &str| -> Result<u64, String> {
        regions
            .as_array()
            .ok_or_else(|| "caller regions differ".to_string())?
            .iter()
            .find(|region| region["name"] == name)
            .ok_or_else(|| "caller region declaration lacks a witnessed pool".to_string())?["base"]
            .as_u64()
            .ok_or_else(|| "caller region base differs".to_string())
    };
    let mut headers = Vec::new();
    for (name, slot) in POINTER_SLOTS {
        let base = usize::try_from(declared(name).map_err(|e| e)?)
            .map_err(|_| "caller region base differs")?;
        let low = u16::try_from(if base < 0x10000 { base } else { base & 15 })
            .map_err(|_| "caller pool header differs")?;
        let high = u16::try_from(if base < 0x10000 { 0 } else { base >> 4 })
            .map_err(|_| "caller pool header differs")?;
        headers.push((slot, low, high));
    }
    Ok(headers)
}

/// The replay input: the recorded registers, regions and initial fields, plus
/// the canonical pool-pointer headers. A recorded pointer that contradicts the
/// declaration refuses; nothing else is added.
fn caller_input(caller: &Value) -> Result<Input, String> {
    let entry = &caller["entry"];
    let order = [
        "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "es", "cs", "ss", "ds",
    ];
    let mut registers = [0u16; 12];
    for (index, name) in order.iter().enumerate() {
        registers[index] = entry[name]
            .as_u64()
            .ok_or("caller entry registers differ")? as u16;
    }
    let regions = caller["regions"].clone();
    let player8_base = regions
        .as_array()
        .ok_or("caller regions differ")?
        .iter()
        .find(|region| region["name"] == "player8")
        .ok_or("caller regions lack the player pool")?["base"]
        .as_u64()
        .ok_or("caller region base differs")?;
    let mut initial = caller["initial"].clone();
    let ds = u32::from(registers[11]) * 16;
    // [confirmed:3d41a0db/T89 + ba499b9/T86] The machine-declared cells: the
    // pool-pointer headers follow from the record's own region bases, and the
    // managed-club cell holds this route's club identity (the witnessed
    // condition contexts bind club 130; the league record observed the same
    // cell at the same campaign). Match-derived bytes such as the win-points
    // cell DS:DCE0 stay unwitnessed and are never initialized here.
    let declared: [(u16, u8, u16); 1] = [(0xDCBA, 2, 130)];
    for (offset, size, value) in declared {
        let cells = initial.as_array_mut().ok_or("caller initial differs")?;
        if let Some(cell) = cells
            .iter_mut()
            .find(|cell| cell["field"] == json!(format!("DS:{offset:04X}")))
        {
            if cell["size"] != size || cell["value"].as_u64() != Some(u64::from(value)) {
                return Err("recorded managed-club cell contradicts the declaration".into());
            }
        } else {
            cells.push(json!({
                "address": ds + u32::from(offset),
                "field": format!("DS:{offset:04X}"),
                "size": size,
                "value": value,
            }));
        }
    }
    // [confirmed:3d41a0db/T89] The witnessed player rows carry their club byte
    // uniformly (every Sevilla row +0 = 176, every managed row +0 = 130); a
    // row the original never read at that granularity is completed from its
    // condition context's roster binding. A pinned byte that contradicts the
    // context refuses; match-derived bytes are never synthesized.
    let condition = sealed(CALLER, CALLER_SHA256, "caller record")?["condition"].clone();
    let mut club_of: std::collections::BTreeMap<u64, u64> = std::collections::BTreeMap::new();
    for context in condition["contexts"]
        .as_array()
        .ok_or("condition contexts differ")?
    {
        let club = context["club"].as_u64().ok_or("condition club differs")?;
        for player in context["roster"]
            .as_array()
            .ok_or("condition roster differs")?
        {
            let player = player.as_u64().unwrap_or(0);
            if player != 0 {
                club_of.insert(player, club);
            }
        }
    }
    let cells = initial.as_array_mut().ok_or("caller initial differs")?;
    for (player, club) in &club_of {
        let field = format!("player8[{player}]+0");
        if let Some(cell) = cells.iter_mut().find(|cell| cell["field"] == json!(field)) {
            if cell["value"].as_u64() != Some(*club) {
                return Err("recorded player club byte contradicts the condition context".into());
            }
        } else if cells
            .iter()
            .any(|cell| cell["field"] == json!(format!("player8[{player}]+1")))
        {
            cells.push(json!({
                "address": player8_base + player * 8,
                "field": field,
                "size": 1,
                "value": club,
            }));
        }
    }
    for (offset, value) in pointer_headers(&regions)?
        .iter()
        .flat_map(|&(slot, low, high)| [(slot, low), (slot + 2, high)])
    {
        let cells = initial.as_array_mut().ok_or("caller initial differs")?;
        if let Some(cell) = cells
            .iter_mut()
            .find(|cell| cell["field"] == json!(format!("DS:{offset:04X}")))
        {
            if cell["value"].as_u64() != Some(u64::from(value)) {
                return Err(format!(
                    "recorded pool header DS:{offset:04X} contradicts the machine declaration"
                ));
            }
        } else {
            cells.push(json!({
                "address": ds + u32::from(offset),
                "field": format!("DS:{offset:04X}"),
                "size": 2,
                "value": value,
            }));
        }
    }
    serde_json::from_value(json!({
        "registers": registers,
        "flags": entry["eflags"],
        "regions": regions,
        "initial": initial,
    }))
    .map_err(|error| format!("caller input differs: {error}"))
}

/// Replay the sealed caller from its recorded fields through the compiled
/// E679→EBE6 closure, validating every observed effect against the record.
pub fn caller_replay() -> Result<post_match::Outcome, String> {
    let source = sealed(CALLER, CALLER_SHA256, "caller record")?;
    if source["schema"] != "sevilla-postmatch-result/1" {
        return Err("sealed Sevilla caller schema differs".into());
    }
    let caller = &source["caller"];
    if caller["census"]["reads"] != 2275
        || caller["census"]["writes"] != 409
        || caller["census"]["initial"] != 431
    {
        return Err("sealed Sevilla caller census differs".into());
    }
    if caller["preservation"]["unexpected_count"] != 0
        || caller["preservation"]["changed_bytes"] != 169
    {
        return Err("sealed Sevilla caller preservation differs".into());
    }
    let outcome = post_match::replay_friendly(&caller_input(caller)?)?;
    let start = caller["draw_start"]
        .as_u64()
        .ok_or("caller draw span differs")?;
    let outcome_final = serde_json::to_value(&outcome.final_fields).map_err(|e| e.to_string())?;
    let replayed: std::collections::BTreeMap<String, u64> = outcome_final
        .as_array()
        .ok_or("caller final fields differ")?
        .iter()
        .map(|field| {
            (
                field["field"].as_str().unwrap_or("").to_owned(),
                field["value"].as_u64().unwrap_or(0),
            )
        })
        .collect();
    let mut final_mismatch = Vec::new();
    for expected in caller["final"]
        .as_array()
        .ok_or("caller final fields differ")?
    {
        let name = expected["field"].as_str().unwrap_or("");
        let value = expected["value"].as_u64().unwrap_or(0);
        match replayed.get(name) {
            Some(actual) if *actual == value => {}
            actual => {
                if final_mismatch.len() < 4 {
                    final_mismatch.push(format!(
                        "{name}: recorded {value} replayed {}",
                        actual.copied().unwrap_or(u64::MAX)
                    ));
                }
            }
        }
    }
    if !final_mismatch.is_empty() {
        return Err(format!(
            "replayed caller final fields differ: {}",
            final_mismatch.join("; ")
        ));
    }
    let expected_writes: Vec<_> = caller["ops"]
        .as_array()
        .ok_or("caller ops differ")?
        .iter()
        .filter(|op| op["kind"] == "write")
        .map(|op| {
            json!({
                "address": op["address"], "size": op["size"], "before": op["before"],
                "value": op["value"], "site": op["site"],
                "draw": op["draw"].as_u64().unwrap_or(0) - start,
            })
        })
        .collect();
    if serde_json::to_value(&outcome.writes).map_err(|e| e.to_string())? != json!(expected_writes) {
        return Err("replayed caller writes differ".into());
    }
    let expected_draws: Vec<_> = source["post_draws"]
        .as_array()
        .ok_or("caller draw ledger differs")?
        .iter()
        .map(|draw| {
            json!({
                "n": draw["n"], "seed_before": draw["seed_before"],
                "seed_after": draw["seed_after"], "value": draw["value"],
            })
        })
        .collect();
    let outcome_draws: Vec<_> = outcome
        .draws
        .iter()
        .map(|draw| {
            json!({
                "n": draw.n, "seed_before": draw.seed_before,
                "seed_after": draw.seed_after, "value": draw.value,
            })
        })
        .collect();
    let mut draw_mismatch = Vec::new();
    for (index, (expected, actual)) in expected_draws.iter().zip(outcome_draws.iter()).enumerate() {
        if expected != actual {
            if draw_mismatch.len() < 4 {
                draw_mismatch.push(format!("#{index}: recorded {expected} replayed {actual}"));
            }
        }
    }
    if !draw_mismatch.is_empty() || expected_draws.len() != outcome.draws.len() {
        return Err(format!(
            "replayed caller draws differ ({} recorded vs {} replayed): {}",
            expected_draws.len(),
            outcome.draws.len(),
            draw_mismatch.join("; ")
        ));
    }
    for (index, name) in [
        "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "es", "cs", "ss", "ds",
    ]
    .iter()
    .enumerate()
    {
        if outcome.registers[index] as u64 != caller["returned"][*name].as_u64().unwrap_or(1) {
            return Err(format!("replayed caller registers differ at {name}"));
        }
    }
    Ok(outcome)
}

/// The witnessed condition outcomes for the managed club's roster, from the
/// sealed record's own condition calls. Each entry is (player, condition).
pub fn managed_condition_updates() -> Result<Vec<(u16, u8)>, String> {
    let source = sealed(CALLER, CALLER_SHA256, "caller record")?;
    let condition = &source["condition"];
    let managed = condition["contexts"]
        .as_array()
        .ok_or("caller condition contexts differ")?
        .iter()
        .find(|context| context["club"] == 130)
        .ok_or("caller condition lacks the managed context")?;
    let calls = condition["calls"]
        .as_array()
        .ok_or("caller condition calls differ")?
        .iter()
        .filter(|call| call["context"] == managed["index"])
        .map(|call| {
            let player = call["input"]["player"]
                .as_u64()
                .ok_or("condition player differs")?;
            let value = call["output"]["condition"]
                .as_u64()
                .ok_or("condition value differs")?;
            Ok((
                u16::try_from(player).map_err(|_| "condition player differs")?,
                u8::try_from(value).map_err(|_| "condition value differs")?,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    if calls.len() != 18 {
        return Err("witnessed managed condition calls differ".into());
    }
    Ok(calls)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sealed_reference_digests_hold() {
        identity().unwrap();
        presentation().unwrap();
        let identity = identity().unwrap();
        assert_eq!(identity["counts"]["ticks"], 3927);
        assert_eq!(identity["counts"]["draws"], 169996);
        assert_eq!(identity["counts"]["actions"], 855);
        assert_eq!(completion_digest().len(), 64);
    }
    #[test]
    fn pointer_headers_follow_the_machine_declaration() {
        let caller = sealed(CALLER, CALLER_SHA256, "caller record").unwrap();
        let headers = pointer_headers(&caller["caller"]["regions"]).unwrap();
        let player8 = headers.iter().find(|(slot, _, _)| *slot == 0xD7BA).unwrap();
        assert_eq!(player8.1, 8);
        assert_eq!(player8.2, u16::try_from(306_232 >> 4).unwrap());
    }
    #[test]
    fn the_witnessed_caller_replays_and_matches_the_record() {
        let outcome = caller_replay().unwrap();
        assert_eq!(outcome.writes.len(), 409);
        assert_eq!(outcome.draws.len(), 107);
    }
    #[test]
    fn managed_condition_updates_carry_the_witnessed_roster() {
        let updates = managed_condition_updates().unwrap();
        assert_eq!(updates.len(), 18);
        assert!(updates.contains(&(2581, 15)));
        assert!(updates.contains(&(2590, 31)));
    }
}
