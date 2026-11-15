//! [confirmed: archaeology ba499b9/T86] E679 caller-entry contract.
//! [designed:T83] Input-only isolated replay; campaign admission is separate.
use crate::Draw;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub address: usize,
    pub field: String,
    pub size: u8,
    pub value: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Region {
    pub base: usize,
    pub count: usize,
    pub name: String,
    pub origin: usize,
    pub stride: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub registers: [u16; 12],
    pub flags: u16,
    pub regions: Vec<Region>,
    pub initial: Vec<Fact>,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Write {
    pub address: usize,
    pub size: u8,
    pub before: u16,
    pub value: u16,
    pub site: String,
    pub draw: usize,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub final_fields: Vec<Fact>,
    pub draws: Vec<Draw>,
    pub writes: Vec<Write>,
    pub registers: [u16; 12],
}
pub fn replay(input: &Input) -> Result<Outcome, String> {
    Session::new(input)?.advance()
}

use super::{action_graph, compiled::Cpu, post_match_graph};
use crate::condition_refresh::{self, Field, Memory};
use std::collections::BTreeSet;
const POOLS: [(&str, u16, usize, usize, usize); 9] = [
    ("player8", 0xD7BA, 3921, 8, 0),
    ("player15", 0xD7BE, 3921, 15, 0),
    ("club", 0xD7C2, 176, 62, 1),
    ("club_stats", 0xD7C6, 176, 12, 1),
    ("fixture", 0xD7CA, 176, 174, 1),
    ("history", 0xD7CE, 176, 58, 1),
    ("game", 0xD7D2, 1, 247, 0),
    ("club_aux", 0xDCC0, 176, 46, 1),
    ("staff", 0xDCC8, 1001, 4, 0),
];
fn allowed_ds(o: usize, size: usize) -> bool {
    [
        (0xD72, 0xD74),
        (0xD074, 0xD076),
        (0xD086, 0xD088),
        (0xD0C4, 0xD0C6),
        (0xD146, 0xD196),
        (0xD4FB, 0xD576),
        (0xD6DD, 0xD6DE),
        (0xD6E4, 0xD6E8),
        (0xD7BA, 0xD7D6),
        (0xDCBA, 0xDCBC),
        (0xDCC0, 0xDCC4),
        (0xDCC8, 0xDCCC),
        (0xDCE0, 0xDCE1),
        (0xDE84, 0xDE88),
    ]
    .iter()
    .any(|&(lo, hi)| lo <= o && o.checked_add(size).is_some_and(|end| end <= hi))
}
fn coordinate(f: &Fact, input: &Input) -> Result<usize, String> {
    let size = usize::from(f.size);
    if !(1..=2).contains(&size) || (size == 1 && f.value > 255) {
        return Err("post-match field width/value differs".into());
    }
    let ds = usize::from(input.registers[11]) * 16;
    let ss = usize::from(input.registers[10]) * 16;
    let bp = usize::from(input.registers[5]);
    if f.field == "seed+0" && size == 2 {
        return Ok(ds + usize::from(crate::SEED));
    }
    if let Some(text) = f.field.strip_prefix("DS:") {
        let o = usize::from_str_radix(text, 16).map_err(|_| "invalid DS coordinate")?;
        if allowed_ds(o, size) {
            return Ok(ds + o);
        }
    } else if let Some(text) = f.field.strip_prefix("frame:") {
        let o = usize::from_str_radix(text, 16).map_err(|_| "invalid frame coordinate")?;
        if [(bp - 7, 1), (bp - 6, 2), (bp - 2, 2), (bp + 6, 2)].contains(&(o, size)) {
            return Ok(ss + o);
        }
    } else if let Some((name, tail)) = f.field.split_once('[') {
        if let Some((index, offset)) = tail.split_once("]+") {
            let index: usize = index.parse().map_err(|_| "invalid pool index")?;
            let offset: usize = offset.parse().map_err(|_| "invalid pool offset")?;
            if let Some(r) = input.regions.iter().find(|r| r.name == name) {
                if index >= r.origin
                    && index - r.origin < r.count
                    && offset.checked_add(size).is_some_and(|end| end <= r.stride)
                {
                    return Ok(r.base + (index - r.origin) * r.stride + offset);
                }
            }
        }
    }
    Err(format!("unclassified post-match field {}", f.field))
}
fn geometry(r: [u16; 12], regions: &[Region]) -> Result<(usize, usize), String> {
    if r[11] < 0x2864
        || r[9] != 0x0AB9u16.wrapping_add(r[11] - 0x2864)
        || r[4] < 1024
        || usize::from(r[5]) < usize::from(r[4]) + 6
        || usize::from(r[5]) + 8 > 65536
        || usize::from(r[5]) > usize::from(r[4]) + 1024
    {
        return Err("post-match entry frame/register contract differs".into());
    }
    let ds = usize::from(r[11]) * 16;
    let ss = usize::from(r[10]) * 16;
    let stack = (ss + usize::from(r[4]) - 1024, ss + usize::from(r[4]) + 6);
    let frame_end = ss + usize::from(r[5]) + 8;
    if ds + 65536 > 0x110000 || frame_end > 0x110000 || (stack.0 < ds + 65536 && ds < frame_end) {
        return Err("post-match DS/frame overlap or range differs".into());
    }
    if regions.len() != POOLS.len() {
        return Err("post-match pool census differs".into());
    }
    let mut ranges = vec![(ds, ds + 65536), (stack.0, frame_end)];
    let mut names = BTreeSet::new();
    for region in regions {
        let spec = POOLS
            .iter()
            .find(|p| p.0 == region.name)
            .ok_or("unknown post-match pool")?;
        if !names.insert(&region.name)
            || (region.count, region.stride, region.origin) != (spec.2, spec.3, spec.4)
            || region.base > 0x110000
            || region.base + region.count * region.stride > 0x110000
        {
            return Err("post-match pool geometry differs".into());
        }
        let end = region.base + region.count * region.stride;
        if ranges.iter().any(|&(lo, hi)| region.base < hi && lo < end) {
            return Err("post-match pool alias differs".into());
        }
        ranges.push((region.base, end));
    }
    Ok(stack)
}
fn machine(input: &Input) -> Result<Cpu, String> {
    let r = input.registers;
    let stack = geometry(r, &input.regions)?;
    let ss = 16 * usize::from(r[10]);
    let mut e = crate::Engine::new(&crate::Input {
        k: 0,
        seed: 0,
        ratings: [0; 8],
        goals: [0; 2],
    });
    // Unknown storage is never a semantic default: Cpu.known guards all reads.
    e.image = vec![0; 0x110000].into_boxed_slice().into();
    let mut c = Cpu::new(e);
    c.r = r;
    c.code_base = r[11] - 0x2864;
    c.flat_memory = true;
    c.require_write_baseline = true;
    c.stack_bounds = Some(stack);
    c.cf = input.flags & 1 != 0;
    c.pf = input.flags & 4 != 0;
    c.z = input.flags & 64 != 0;
    c.sf = input.flags & 128 != 0;
    c.df = input.flags & 1024 != 0;
    c.of = input.flags & 2048 != 0;
    for f in &input.initial {
        let a = coordinate(f, input)?;
        if a != f.address {
            return Err("post-match coordinate/address differs".into());
        }
        for i in 0..usize::from(f.size) {
            if c.known[a + i] {
                return Err("duplicate post-match initial byte".into());
            }
            c.e.image[a + i] = (f.value >> (8 * i)) as u8;
            c.known[a + i] = true;
        }
    }
    // [confirmed:T86] The real near-call ABI: EBE6 return and parent static link.
    for (o, value) in [(r[4], 0xEBE6), (r[4] + 2, r[5])] {
        let a = ss + usize::from(o);
        c.e.image[a..a + 2].copy_from_slice(&value.to_le_bytes());
        c.known[a..a + 2].fill(true);
    }
    validate_bindings(&c, &input.regions)?;
    Ok(c)
}
fn validate_bindings(c: &Cpu, regions: &[Region]) -> Result<(), String> {
    for &(name, slot, _, _, _) in &POOLS {
        let (o, s) = c.far(c.r[11], slot)?;
        let a = 16 * usize::from(s) + usize::from(o);
        if regions.iter().find(|r| r.name == name).map(|r| r.base) != Some(a) {
            return Err("post-match far-pointer binding differs".into());
        }
        // Every pool is contained in the original 16-bit offset domain.
        let r = regions.iter().find(|r| r.name == name).unwrap();
        if usize::from(o) + r.count * r.stride > 65536 {
            return Err("post-match far-pointer offset wraps".into());
        }
    }
    let (o, s) = c.far(c.r[11], 0xDE84)?;
    if (o, s) != (crate::SEED, c.r[11]) {
        return Err("post-match RNG pointer aliases non-seed data".into());
    }
    Ok(())
}
fn guard(c: &Cpu) -> Result<(), String> {
    let ds = c.r[11];
    let ss = c.r[10];
    let sp = c.r[4];
    let bp = c.r[5];
    if c.df
        || !c.flat_memory
        || c.r[9].wrapping_sub(c.code_base) != 0x0AB9
        || c.read(ss, sp, 16)? != 0xEBE6
        || c.read(ss, sp + 2, 16)? != bp
    {
        return Err("post-match live caller frame differs".into());
    }
    let week = c.read(ss, bp + 6, 16)?;
    let schedule = c.read(ss, bp - 2, 16)?;
    if !(1..=58).contains(&week)
        || schedule >= 123
        || c.read(ss, bp - 7, 8)? != 1
        || c.read(ds, 0xD075, 8)? != 0
        || c.read(ds, 0xD6DD, 8)? != 1
    {
        return Err("unsupported post-match branch/frame selectors".into());
    }
    let club = c.read(ds, 0xD4FB + schedule, 8)?;
    if !(1..=176).contains(&club) {
        return Err("invalid scheduled post-match club".into());
    }
    let (o, s) = c.far(ds, 0xD7CA)?;
    let fixture = o.wrapping_add(174 * (club - 1) + 3 * (week - 1));
    let opponent = c.read(s, fixture + 1, 8)?;
    if c.read(s, fixture, 8)? != 0
        || !(1..=176).contains(&opponent)
        || opponent == club
        || c.read(ds, 0xD086, 8)? == c.read(ds, 0xD087, 8)?
    {
        return Err("unsupported post-match competition/draw/club pair".into());
    }
    // [confirmed:ba499b9,1998:11C9/1B99] A club's watched flag selects
    // performance processing. D075 alone does not rule out the random branch.
    let (co, cs) = c.far(ds, 0xD7C2)?;
    if c.read(cs, co.wrapping_add(62 * (club - 1) + 58), 8)? == 0
        && c.read(cs, co.wrapping_add(62 * (opponent - 1) + 58), 8)? == 0
    {
        return Err("unsupported post-match unwatched random branch".into());
    }
    let a = c.read(ds, 0xD6E4, 16)?;
    let b = c.read(ds, 0xD6E6, 16)?;
    if !((a == club && b == opponent) || (a == opponent && b == club)) {
        return Err("post-match live club pair differs from fixture".into());
    }
    Ok(())
}
/// [designed:T83] Owned caller state; failures commit no bytes/registers/RNG.
pub struct Session {
    cpu: Cpu,
    initial: Vec<Fact>,
    regions: Vec<Region>,
}
impl Session {
    pub fn new(input: &Input) -> Result<Self, String> {
        Ok(Self {
            cpu: machine(input)?,
            initial: input.initial.clone(),
            regions: input.regions.clone(),
        })
    }
    pub fn snapshot(&self) -> Result<(Vec<Fact>, [u16; 12]), String> {
        Ok((project(&self.cpu, &self.initial)?, self.cpu.r))
    }
    pub fn advance(&mut self) -> Result<Outcome, String> {
        let (draws, writes) = advance_image(&mut self.cpu, &self.regions)?;
        Ok(Outcome {
            final_fields: project(&self.cpu, &self.initial)?,
            registers: self.cpu.r,
            draws,
            writes,
        })
    }
}
fn project(c: &Cpu, fields: &[Fact]) -> Result<Vec<Fact>, String> {
    fields
        .iter()
        .map(|f| {
            let mut f = f.clone();
            let a = f.address;
            f.value = u16::from(c.e.image[a]);
            if f.size == 2 {
                f.value |= u16::from(c.e.image[a + 1]) << 8;
            }
            Ok(f)
        })
        .collect()
}
/// Internal live-image entry: no replay facts, outputs or recorded seeds enter.
/// Trial execution retains the original owned image on every refusal.
pub(super) fn advance_image(
    c: &mut Cpu,
    regions: &[Region],
) -> Result<(Vec<Draw>, Vec<Write>), String> {
    let mut trial = c.clone();
    if !trial.flat_memory
        || trial.e.image.len() != 0x110000
        || trial.known.len() != 0x110000
        || trial.post_writes.is_some()
    {
        return Err("post-match live image/audit contract differs".into());
    }
    trial.e.image.check_initialized()?;
    trial.stack_bounds = Some(geometry(trial.r, regions)?);
    trial.require_write_baseline = true;
    validate_bindings(&trial, regions)?;
    guard(&trial)?;
    let draw_start = trial.draws.len();
    trial.post_writes = Some(vec![]);
    trial.reset_root_budget();
    let original_bp = trial.r[5];
    let original_sp = trial.r[4];
    trial.invoke_entry(post_match_graph::p_0ab9_e679, 0x0AB9, 0xEBE6)?;
    if trial.r[5] != original_bp
        || trial.r[4] != original_sp + 4
        || trial.last_return != Some((0x0AB9, 0xEBE6))
    {
        return Err("post-match return ABI differs".into());
    }
    let draws = trial.draws[draw_start..].to_vec();
    let mut writes = trial.post_writes.take().unwrap();
    for w in &mut writes {
        w.draw -= draw_start;
    }
    trial.e.image.check_initialized()?;
    *c = trial;
    Ok((draws, writes))
}
struct Live<'a>(&'a mut Cpu);
impl Live<'_> {
    fn location(&self, field: Field) -> Result<(u16, u16, u8), String> {
        let c = &self.0;
        let ds = c.r[11];
        let (slot, index, offset, width) = match field {
            Field::Player8(p, o) => (0xD7BA, 8 * p, u16::from(o), 8),
            Field::Player15(p, o) => (0xD7BE, 15 * p, u16::from(o), 8),
            Field::ClubStaff(club) => (0xD7C2, 62 * u16::from(club - 1), 44, 16),
            Field::Facility(staff) => (0xDCC8, 4 * staff, 2, 8),
            Field::Cap(o) => return Ok((ds, 0xD72 + u16::from(o), 8)),
            Field::Seed => return Ok((ds, crate::SEED, 16)),
        };
        let (o, s) = c.far(ds, slot)?;
        Ok((s, o.wrapping_add(index).wrapping_add(offset), width))
    }
}
impl Memory for Live<'_> {
    fn read(&mut self, field: Field) -> Result<u16, String> {
        let (s, o, w) = self.location(field)?;
        self.0.read(s, o, w)
    }
    fn write(&mut self, field: Field, value: u8, site: u16) -> Result<(), String> {
        let (s, o, w) = self.location(field)?;
        self.0.pc = (0x1998, site);
        self.0.write(s, o, w, u16::from(value))
    }
    fn draw(&mut self, n: u16, site: u16) -> Result<i16, String> {
        let c = &mut self.0;
        c.push(n)?;
        c.invoke(action_graph::p_232a_076d, true, 0x232A, 0x076D, site)?;
        Ok(c.r[0] as i16)
    }
}
/// [derived:ba499b9,1998:193A/19FB] Both caller continuations reload volatile
/// operands and compare anew. Only BP/SP/CS/DS/SS and memory survive this boundary.
pub(super) fn condition(c: &mut Cpu) -> Result<(), String> {
    c.enter(4)?;
    let player = c.read(c.r[10], c.r[5] + 6, 16)?;
    if !(1..=3920).contains(&player) {
        return Err("condition player outside original domain".into());
    }
    let flag = c.read(c.r[10], c.r[5] + 4, 8)? as u8;
    let mut m = Live(c);
    if m.read(Field::Player15(player, 8))? as i8 <= 0 {
        return Err("unexercised post-match injury branch".into());
    }
    condition_refresh::apply(&mut m, player, flag)?;
    let c = m.0;
    c.r[4] = c.r[5];
    c.r[5] = c.pop()?;
    c.ret(false, 4)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Input {
        let v: serde_json::Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/post_match/tail-r0009.json"
        ))
        .unwrap();
        let c = &v["payload"]["caller"];
        let r = &c["entry"];
        serde_json::from_value(serde_json::json!({
            "registers":[r["ax"],r["cx"],r["dx"],r["bx"],r["sp"],r["bp"],r["si"],r["di"],r["es"],r["cs"],r["ss"],r["ds"]],
            "flags":r["eflags"],"regions":c["regions"],"initial":c["initial"]
        })).unwrap()
    }
    #[test]
    fn prior_rng_and_current_player_fields_flow_into_the_owned_caller() {
        let input = input();
        let mut c = machine(&input).unwrap();
        c.push(9).unwrap();
        c.invoke(action_graph::p_232a_076d, true, 0x232A, 0x076D, 0xEBE2)
            .unwrap();
        let seed = c.draws.last().unwrap().seed_after;
        let (o, s) = c.far(c.r[11], 0xD7BE).unwrap();
        // A current live condition differs from the retained specimen. No
        // caller-entry fixture is consulted by advance_image or the kernel.
        c.write(s, o + 15 * 2661 + 7, 8, 15).unwrap();
        let (draws, writes) = advance_image(&mut c, &input.regions).unwrap();
        assert_eq!(draws[0].seed_before, seed);
        assert_ne!(seed, 15361);
        assert_eq!(c.draws.len(), 104);
        assert_eq!(draws.len(), 103);
        assert!(writes
            .iter()
            .any(|w| w.address == 337600 + 15 * 2661 + 7 && w.before == 15));
        assert_eq!(
            c.read(c.r[11], crate::SEED, 16).unwrap(),
            draws.last().unwrap().seed_after
        );
    }
    #[test]
    fn live_refusal_preserves_full_image_registers_presence_rng_and_frames() {
        let input = input();
        let mut c = machine(&input).unwrap();
        // This dependency is reached after the caller's initial mutations.
        let a = 337600 + 15 * 2661 + 8;
        c.known[a] = false;
        let before = c.clone();
        assert!(advance_image(&mut c, &input.regions).is_err());
        assert_eq!(c.e.image, before.e.image);
        assert_eq!(c.r, before.r);
        assert_eq!(c.upper, before.upper);
        assert_eq!(c.known, before.known);
        assert_eq!(c.draws, before.draws);
        assert_eq!(c.pc, before.pc);
        assert_eq!(c.call_chain(), before.call_chain());
        assert_eq!(c.last_return, before.last_return);
        assert_eq!(c.post_writes, before.post_writes);
    }
}
