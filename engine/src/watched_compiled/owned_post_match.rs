//! [confirmed: research ef8dad0/T87] Early observations for the T45 league source.
//! [designed:T84] Only input values enter admission; late results are test references.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub field: String,
    pub size: u8,
    pub value: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    pub source_sha256: String,
    pub fields: Vec<Field>,
}
pub const SOURCE_SHA: &str = "d301ca8d2cb507522b8f2ab8882520e86047bb43671a957714ede0f5138d435a";
pub fn original_admission() -> Admission {
    Admission {
        source_sha256: SOURCE_SHA.into(),
        fields: fields(&source().expect("sealed early source")).expect("early fields"),
    }
}
// [confirmed: archaeology 18ce483/T88] Sealed natural fresh-friendly caller:
// result-r0004 (byte-identical to restored repeat r0005) from the accepted T67
// lineage. Its caller-initial operands are the fresh caller-early dependencies.
const FRESH_SOURCE: &[u8] =
    include_bytes!("../../tests/fixtures/post_match/fresh-result-r0004.json");
const FRESH_SOURCE_SHA: &str =
    "9064243194d1bbbdd4dba1602d784c41e4bcba034e4ddbd94cc68b1e718d7c29";
pub fn fresh_original_admission() -> Admission {
    Admission {
        source_sha256: FRESH_SOURCE_SHA.into(),
        fields: fresh_fields(&fresh_source().expect("sealed fresh caller source"))
            .expect("fresh caller fields"),
    }
}
fn fresh_source() -> Result<Value, String> {
    if format!("{:x}", Sha256::digest(FRESH_SOURCE)) != FRESH_SOURCE_SHA {
        return Err("sealed fresh caller source digest differs".into());
    }
    serde_json::from_slice(FRESH_SOURCE).map_err(|e| e.to_string())
}
fn fresh_fields(v: &Value) -> Result<Vec<Field>, String> {
    v["caller"]["initial"]
        .as_array()
        .ok_or("missing fresh caller initial operands")?
        .iter()
        .map(|f| {
            Ok(Field {
                field: f["field"].as_str().ok_or("missing field")?.into(),
                size: f["size"].as_u64().ok_or("missing width")? as u8,
                value: f["value"].as_u64().ok_or("missing value")? as u16,
            })
        })
        .collect()
}

use super::{compiled::Cpu, post_match};
use crate::{layout, Engine};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
pub(crate) const PARENT: u16 = 0xFE16;
const REPEAT_SHA: &str = "e84b5c800cf6bdfd8ac6bc6873514accef954df24bf237b86d68dd6281bdd4f0";
const RAW: &[u8] = include_bytes!("../../tests/fixtures/post_match_incoming/incoming-r0001.json");
const REPEAT: &[u8] =
    include_bytes!("../../tests/fixtures/post_match_incoming/incoming-r0002.json");
#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub known_bytes: usize,
    pub selected_bytes: usize,
    pub admitted_bytes: usize,
    pub binding_bytes: usize,
    pub frame_bytes: usize,
}
fn source() -> Result<Value, String> {
    if format!("{:x}", Sha256::digest(RAW)) != SOURCE_SHA
        || format!("{:x}", Sha256::digest(REPEAT)) != REPEAT_SHA
    {
        return Err("sealed early source digest differs".into());
    }
    let a: Value = serde_json::from_slice(RAW).map_err(|e| e.to_string())?;
    let b: Value = serde_json::from_slice(REPEAT).map_err(|e| e.to_string())?;
    if a["payload"] != b["payload"] {
        return Err("independent early source differs".into());
    }
    Ok(a)
}
fn fields(v: &Value) -> Result<Vec<Field>, String> {
    v["payload"]["fields"]
        .as_array()
        .ok_or("missing early field census")?
        .iter()
        .map(|f| {
            Ok(Field {
                field: f["field"].as_str().ok_or("missing field")?.into(),
                size: f["size"].as_u64().ok_or("missing width")? as u8,
                value: f["value"].as_u64().ok_or("missing value")? as u16,
            })
        })
        .collect()
}
pub(crate) fn validate(admission: &Admission) -> Result<Value, String> {
    if admission.source_sha256 != SOURCE_SHA {
        return Err("foreign early source identity".into());
    }
    let v = source()?;
    let expected = fields(&v)?;
    let mut seen = BTreeSet::new();
    if admission.fields.len() != expected.len() {
        return Err("incomplete early field census".into());
    }
    for f in &admission.fields {
        if !seen.insert(&f.field) || !expected.contains(f) {
            return Err(format!("early source field differs: {}", f.field));
        }
    }
    Ok(v)
}
const BASELINE: &[u8] = include_bytes!("../../tests/fixtures/prepared_match/admission.json");
fn selected_profile() -> Result<Value, String> {
    if format!("{:x}", Sha256::digest(BASELINE))
        != "842f1453cae4ba9ab626d7e57316494cfe7e70f64ea80e746c3834de0e12989f"
    {
        return Err("selected admission source digest differs".into());
    }
    serde_json::from_slice(BASELINE).map_err(|e| e.to_string())
}
// [confirmed:archaeology ef8dad0/T87; selected admission842f1453/T45]
// The selected-squad constructor already owns its
// roster/position inputs. Three bytes differ from T87's earlier raw squad.
// Validate these against that sealed prior admission; never replace them with
// the earlier observations. Complete original-stream parity remains required.
fn selected_owners(v: &Value) -> Result<std::collections::BTreeMap<usize, u8>, String> {
    let profile = selected_profile()?;
    let legacy = &profile["input"]["preparation"]["setup"]["legacy"];
    let mut result = std::collections::BTreeMap::new();
    for f in v["payload"]["fields"].as_array().unwrap() {
        let coord = &f["coordinate"];
        let index = coord["index"].as_u64().unwrap();
        let off = coord["offset"].as_i64().unwrap();
        let value = if coord["region"] == "club" && (0..40).contains(&off) && f["size"] == 2 {
            let row = legacy["substitution"]["context"]["clubs"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["id"] == index)
                .ok_or("selected club missing")?;
            Some(row["roster"][off as usize / 2].as_u64().unwrap() as u16)
        } else if coord["region"] == "player8" && off == 6 {
            legacy["players"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["id"] == index)
                .map(|r| r["p8"][6].as_u64().unwrap() as u16)
        } else {
            None
        };
        if let Some(value) = value {
            if f["value"].as_u64().unwrap() != u64::from(value) {
                let a = location(f)?;
                for i in 0..f["size"].as_u64().unwrap() as usize {
                    result.insert(a + i, (value >> (8 * i)) as u8);
                }
            }
        }
    }
    Ok(result)
}
pub(crate) fn validate_profile(
    input: &super::prepared::Input,
    admission: &Admission,
) -> Result<(), String> {
    validate(admission)?;
    let original = selected_profile()?;
    let original: super::prepared::Input =
        serde_json::from_value(original["input"].clone()).map_err(|e| e.to_string())?;
    if serde_json::to_value(input).map_err(|e| e.to_string())?
        != serde_json::to_value(original).map_err(|e| e.to_string())?
    {
        return Err("unsupported source/profile for continuing league caller".into());
    }
    Ok(())
}
pub(crate) fn regions() -> Vec<post_match::Region> {
    [
        ("player8", layout::P8, 3921, 8, 0),
        ("player15", layout::P15, 3921, 15, 0),
        ("club", layout::CLUB, 176, 62, 1),
        ("club_stats", 0x70000, 176, 12, 1),
        ("fixture", 0x80000, 176, 174, 1),
        ("history", layout::HISTORY, 176, 58, 1),
        ("game", layout::CONFIG, 1, 247, 0),
        ("club_aux", 0x72000, 176, 46, 1),
        ("staff", layout::STAFF, 1001, 4, 0),
    ]
    .into_iter()
    .map(|(name, base, count, stride, origin)| post_match::Region {
        name: name.into(),
        base,
        count,
        stride,
        origin,
    })
    .collect()
}
const POINTERS: [(&str, u16); 10] = [
    ("player8", 0xD7BA),
    ("player15", 0xD7BE),
    ("club", 0xD7C2),
    ("club_stats", 0xD7C6),
    ("fixture", 0xD7CA),
    ("history", 0xD7CE),
    ("game", 0xD7D2),
    ("club_aux", 0xDCC0),
    ("staff", 0xDCC8),
    ("seed", 0xDE84),
];
fn binding(field: &Value) -> Option<(u16, usize)> {
    let c = &field["coordinate"];
    if c["region"] != "DS" {
        return None;
    }
    let offset = c["offset"].as_u64()? as u16;
    POINTERS.iter().find_map(|&(name, slot)| {
        if offset == slot || offset == slot + 2 {
            let a = if name == "seed" {
                usize::from(crate::SEED)
            } else {
                regions().iter().find(|r| r.name == name).unwrap().base
            };
            Some((
                offset,
                if offset == slot {
                    if a < 0x10000 {
                        a
                    } else {
                        a & 15
                    }
                } else {
                    if a < 0x10000 {
                        0
                    } else {
                        a >> 4
                    }
                },
            ))
        } else {
            None
        }
    })
}
fn location(field: &Value) -> Result<usize, String> {
    let c = &field["coordinate"];
    let region = c["region"].as_str().ok_or("invalid coordinate")?;
    let offset = c["offset"].as_i64().ok_or("invalid offset")?;
    if region == "frame" {
        return Ok((0xB0000i64 + i64::from(PARENT) + offset) as usize);
    }
    if region == "DS" {
        return usize::try_from(offset).map_err(|_| "negative DS field".into());
    }
    if region == "seed" {
        return Ok(usize::from(crate::SEED));
    }
    let r = regions()
        .into_iter()
        .find(|r| r.name == region)
        .ok_or("unknown early pool")?;
    let index = c["index"].as_u64().ok_or("invalid pool row")? as usize;
    Ok(r.base + (index - r.origin) * r.stride + offset as usize)
}
/// [designed:T84] Runs after existing input loading, before19CA. Known bytes
/// are compared, never overwritten. Only sealed early unknown fields enter.
pub(crate) fn admit(e: &mut Engine, admission: &Admission) -> Result<Report, String> {
    let v = validate(admission)?;
    let mut report = Report {
        known_bytes: 0,
        selected_bytes: 0,
        admitted_bytes: 0,
        binding_bytes: 0,
        frame_bytes: 0,
    };
    let selected = selected_owners(&v)?;
    let mut conflicts = vec![];
    let mut pending = vec![];
    for field in v["payload"]["fields"].as_array().unwrap() {
        let size = field["size"].as_u64().unwrap() as usize;
        if binding(field).is_some() {
            report.binding_bytes += size;
            continue;
        }
        if field["coordinate"]["region"] == "frame" {
            report.frame_bytes += size;
            continue;
        }
        let a = location(field)?;
        let value = field["value"].as_u64().unwrap() as u16;
        for i in 0..size {
            let value = (value >> (8 * i)) as u8;
            if e.image.is_initialized(a + i) {
                if let Some(&owned) = selected.get(&(a + i)) {
                    if e.image[a + i] != owned {
                        conflicts.push(format!("selected owner differs: {}", field["field"]));
                    }
                    if e.image[a + i] != value {
                        report.selected_bytes += 1;
                    }
                } else if e.image[a + i] != value {
                    conflicts.push(format!(
                        "{}@{:05X}:{}!={}",
                        field["field"],
                        a + i,
                        e.image[a + i],
                        value
                    ));
                }
                report.known_bytes += 1;
            } else {
                if selected.contains_key(&(a + i)) {
                    conflicts.push(format!("missing selected owner {}", field["field"]));
                }
                pending.push((a + i, value));
                report.admitted_bytes += 1;
            }
        }
    }
    if !conflicts.is_empty() {
        return Err(format!(
            "early ownership conflicts: {}",
            conflicts.join(", ")
        ));
    }
    for (a, value) in pending {
        e.image[a] = value;
    }
    Ok(report)
}
pub(super) fn initialize_abi(c: &mut Cpu) -> Result<(), String> {
    let source = source()?;
    // [confirmed:T87] Existing E1F2 frame has a parent22 bytes above it.
    let old = &source["payload"]["context"];
    if old["parent_bp"].as_u64().unwrap() - old["match_frame"]["bp"].as_u64().unwrap() != 22 {
        return Err("source parent-frame geometry differs".into());
    }
    if c.read(c.r[10], 0xFE00, 16)? != PARENT {
        return Err("initial retained parent link differs".into());
    }
    let mut pending = vec![];
    for field in source["payload"]["fields"].as_array().unwrap() {
        if field["coordinate"]["region"] == "frame" {
            let o = (i64::from(PARENT) + field["coordinate"]["offset"].as_i64().unwrap()) as u16;
            queue_binding(
                c,
                c.r[10],
                o,
                field["size"].as_u64().unwrap() as u8 * 8,
                field["value"].as_u64().unwrap() as u16,
                &mut pending,
            )?;
        }
    }
    // New canonical pool headers are layout bindings, never original pointers.
    for &(name, slot) in &POINTERS {
        let a = if name == "seed" {
            usize::from(crate::SEED)
        } else {
            regions().iter().find(|r| r.name == name).unwrap().base
        };
        let (o, s) = if a < 0x10000 {
            (a as u16, 0)
        } else {
            ((a & 15) as u16, (a >> 4) as u16)
        };
        queue_binding(c, 0, slot, 16, o, &mut pending)?;
        queue_binding(c, 0, slot + 2, 16, s, &mut pending)?;
    }
    for (s, o, w, value) in pending {
        c.write(s, o, w, value)?;
    }
    Ok(())
}
fn queue_binding(
    c: &Cpu,
    s: u16,
    o: u16,
    w: u8,
    value: u16,
    pending: &mut Vec<(u16, u16, u8, u16)>,
) -> Result<(), String> {
    let a = 16 * usize::from(s) + usize::from(o);
    let mut missing = false;
    for i in 0..usize::from(w / 8) {
        if c.known[a + i] {
            if c.e.image[a + i] != (value >> (8 * i)) as u8 {
                return Err("existing canonical ABI binding conflicts".into());
            }
        } else {
            missing = true;
        }
    }
    if missing {
        pending.push((s, o, w, value));
    }
    Ok(())
}
fn original_address(a: usize, v: &Value) -> Result<usize, String> {
    let old = &v["payload"]["context"];
    if a < 0x10000 {
        return Ok(old["registers"]["ds"].as_u64().unwrap() as usize * 16 + a);
    }
    if (0xB0000 + usize::from(PARENT) - 7..0xB0000 + usize::from(PARENT) + 8).contains(&a) {
        return Ok(old["registers"]["ss"].as_u64().unwrap() as usize * 16
            + old["parent_bp"].as_u64().unwrap() as usize
            + a
            - (0xB0000 + usize::from(PARENT)));
    }
    for r in regions() {
        if (r.base..r.base + r.count * r.stride).contains(&a) {
            let old = old["regions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|o| o["name"] == r.name)
                .unwrap();
            return Ok(old["base"].as_u64().unwrap() as usize + a - r.base);
        }
    }
    Err(format!("unclassified native post-match address {a:05X}"))
}
/// Output-only coordinate normalization. No reference output enters execution.
fn outcome(
    c: &Cpu,
    draws: Vec<crate::Draw>,
    writes: Vec<post_match::Write>,
) -> Result<Value, String> {
    let v = source()?;
    let old = &v["payload"]["context"];
    let mut final_fields = vec![];
    for field in v["payload"]["fields"].as_array().unwrap() {
        let a = location(field)?;
        let width = field["size"].as_u64().unwrap() as u8 * 8;
        let actual = c.read((a >> 4) as u16, (a & 15) as u16, width)?;
        let value = if let Some((_, want)) = binding(field) {
            if usize::from(actual) != want {
                return Err("computed native pointer differs".into());
            }
            field["value"].as_u64().unwrap() as u16
        } else {
            actual
        };
        final_fields.push(json!({"field":field["field"],"address":field["address"],"size":field["size"],"value":value}));
    }
    let mut normalized = vec![];
    for mut w in writes {
        w.address = original_address(w.address, &v)?;
        normalized.push(w);
    }
    let mut r = c.r;
    r[4] = (old["parent_bp"].as_u64().unwrap() as u16).wrapping_add(r[4].wrapping_sub(PARENT));
    r[5] = (old["parent_bp"].as_u64().unwrap() as u16).wrapping_add(r[5].wrapping_sub(PARENT));
    r[9] = r[9]
        .wrapping_sub(c.code_base)
        .wrapping_add(old["registers"]["ds"].as_u64().unwrap() as u16 - 0x2864);
    r[10] = old["registers"]["ss"].as_u64().unwrap() as u16;
    r[11] = old["registers"]["ds"].as_u64().unwrap() as u16;
    let far = 16 * usize::from(r[8]) + usize::from(r[7]);
    let region = regions()
        .into_iter()
        .find(|x| (x.base..x.base + x.count * x.stride).contains(&far))
        .ok_or("unsupported returned far register")?;
    let original = old["regions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["name"] == region.name)
        .unwrap();
    r[8] = original["segment"].as_u64().unwrap() as u16;
    r[7] = (original["offset"].as_u64().unwrap() as usize + far - region.base) as u16;
    let regs = json!({"ax":r[0],"cx":r[1],"dx":r[2],"bx":r[3],"sp":r[4],"bp":r[5],"si":r[6],"di":r[7],"es":r[8],"cs":r[9],"ss":r[10],"ds":r[11]});
    Ok(
        json!({"return":"0AB9:EBE6","source_final":final_fields,"draws":draws,"source_writes":normalized,"source_registers":regs}),
    )
}
pub(super) fn handoff(c: &mut Cpu) -> Result<Value, String> {
    if c.last_return != Some((0x0AB9, 0xEBE2))
        || c.r[4] != 0xFE08
        || c.r[5] != PARENT
        || !c.call_chain().is_empty()
    {
        return Err("missing or stale owned EBE2 parent frame".into());
    }
    let mut trial = c.clone();
    trial.push(trial.r[5])?;
    trial.push(0xEBE6)?;
    let (draws, writes) = post_match::advance_native_image(&mut trial, &regions())?;
    let out = outcome(&trial, draws, writes)?;
    *c = trial;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn early_image() -> Engine {
        let mut e = Engine::new(&crate::Input {
            k: 0,
            seed: 65341,
            ratings: [0; 8],
            goals: [0; 2],
        });
        e.image = crate::native_image::Image::tracked(0x110000);
        let v = source().unwrap();
        for f in v["payload"]["fields"].as_array().unwrap() {
            if binding(f).is_some() || f["coordinate"]["region"] == "frame" {
                continue;
            }
            let a = location(f).unwrap();
            let value = f["value"].as_u64().unwrap() as u16;
            for i in 0..f["size"].as_u64().unwrap() as usize {
                e.image[a + i] = (value >> (8 * i)) as u8;
            }
        }
        for (a, value) in selected_owners(&v).unwrap() {
            e.image[a] = value;
        }
        e
    }
    #[test]
    fn selected_and_known_ownership_conflicts_refuse_atomically() {
        for damage in [usize::from(crate::SEED), layout::CLUB + 62 * 133 + 18] {
            let mut e = early_image();
            e.image[damage] ^= 1;
            let before = e.image.clone();
            assert!(admit(&mut e, &original_admission()).is_err());
            assert_eq!(e.image, before);
        }
        let mut e = early_image();
        let before = e.image.clone();
        let report = admit(&mut e, &original_admission()).unwrap();
        assert_eq!(report.selected_bytes, 3);
        assert_eq!(report.admitted_bytes, 0);
        assert_eq!(e.image, before);
    }
    // Isolated fault fixture only. Continuing parity must use the separate
    // genuine full-match integration test; production has no late-state loader.
    fn isolated_end() -> Cpu {
        let late: Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/post_match/tail-r0009.json"
        ))
        .unwrap();
        let early = source().unwrap();
        let mut e = Engine::new(&crate::Input {
            k: 0,
            seed: 0,
            ratings: [0; 8],
            goals: [0; 2],
        });
        e.image = vec![0; 0x110000].into_boxed_slice().into();
        let mut c = Cpu::new(e);
        c.flat_memory = true;
        c.code_base = 0xD79C;
        c.r[9] = 0xE255;
        c.r[10] = 0xB000;
        c.r[11] = 0;
        c.r[5] = PARENT;
        c.r[4] = 0xFE08;
        c.stack_bounds = Some((0xBEDF4, 0xBFE0A));
        for f in late["payload"]["caller"]["initial"].as_array().unwrap() {
            let early = early["payload"]["fields"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["field"] == f["field"])
                .unwrap();
            if binding(early).is_some() {
                continue;
            }
            let a = location(early).unwrap();
            let value = f["value"].as_u64().unwrap() as u16;
            for i in 0..f["size"].as_u64().unwrap() as usize {
                c.e.image[a + i] = (value >> (8 * i)) as u8;
                c.known[a + i] = true;
            }
        }
        c.write(0xB000, 0xFE00, 16, PARENT).unwrap();
        initialize_abi(&mut c).unwrap();
        c.require_write_baseline = true;
        c.last_return = Some((0x0AB9, 0xEBE2));
        c
    }
    fn unchanged(a: &Cpu, b: &Cpu) {
        assert_eq!(a.e.image, b.e.image);
        assert_eq!(a.r, b.r);
        assert_eq!(a.upper, b.upper);
        assert_eq!(a.known, b.known);
        assert_eq!(a.draws, b.draws);
        assert_eq!(a.pc, b.pc);
        assert_eq!(a.call_chain(), b.call_chain());
        assert_eq!(a.last_return, b.last_return);
        assert_eq!(a.post_writes, b.post_writes);
    }
    #[test]
    fn post_failure_and_stale_frames_preserve_the_owned_end_image_and_seed() {
        for damage in 0..4 {
            let mut c = isolated_end();
            match damage {
                0 => c.known[layout::P15 + 15 * 2661 + 8] = false,
                1 => c.r[5] = 0,
                2 => c.last_return = None,
                3 => {
                    c.write(0, 0xDCC0, 16, 1).unwrap();
                }
                _ => unreachable!(),
            }
            let before = c.clone();
            assert!(handoff(&mut c).is_err());
            unchanged(&c, &before);
        }
    }
}
