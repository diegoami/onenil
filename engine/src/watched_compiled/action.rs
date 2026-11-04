//! [confirmed] F39/c7637bd;original448C/4DFF source-addressed numeric kernels.
//! [designed] Strict named entry facts;private original RAM is never a runtime input.
use super::compiled::{Cpu, Missing};
use crate::{terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Action5,
    Contact,
    Actor,
    Tick,
    Caller,
    WatchedMatch,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum PointerKind {
    P8,
    P15,
    Club,
    Config,
    Style,
    Main,
    MainShown,
    Background,
    BackgroundShown,
    Commentary,
    Seed,
    Results,
    Fixtures,
    Screen,
    ScreenShown,
    ScreenSaved,
    Counts,
}
impl PointerKind {
    pub(super) fn slot(self) -> u16 {
        match self {
            Self::P8 => 0xD7BA,
            Self::P15 => 0xD7BE,
            Self::Club => 0xD7C2,
            Self::Config => 0xD7D2,
            Self::Style => 0xD0C0,
            Self::Main => 0xD08A,
            Self::MainShown => 0xD08E,
            Self::Background => 0xD092,
            Self::BackgroundShown => 0xD096,
            Self::Commentary => 0xD09A,
            Self::Seed => 0xDE84,
            Self::Results => 0xD7CE,
            Self::Fixtures => 0xD7CA,
            Self::Screen => 0xDD58,
            Self::ScreenShown => 0xDD5C,
            Self::ScreenSaved => 0xDD60,
            Self::Counts => 0xDCC8,
        }
    }
    fn length(self) -> usize {
        match self {
            Self::P8 => 8 * 3921,
            Self::P15 => 15 * 3921,
            Self::Club => 62 * 175,
            Self::Config => 256,
            Self::Style => 1,
            Self::Seed => 2,
            Self::Results => 58 * 176,
            Self::Fixtures => 174 * 176,
            Self::Counts => 1001 * 4,
            _ => 4000,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Pointer {
    pub kind: PointerKind,
    pub offset: u16,
    pub segment: u16,
}
impl Pointer {
    pub(super) fn address(&self) -> usize {
        16 * usize::from(self.segment) + usize::from(self.offset)
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Area {
    Ds,
    P8,
    P15,
    Club,
    Config,
    Style,
    Main,
    MainShown,
    Background,
    BackgroundShown,
    Frame,
    Results,
    Fixtures,
    Counts,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ByteFact {
    pub area: Area,
    pub offset: i32,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub kind: Kind,
    pub scene: Scene,
    pub seed: u16,
    pub regs: [u16; 12],
    pub flags: u32,
    pub stack_words: [u16; 4],
    pub pointers: Vec<Pointer>,
    pub cells: Vec<ByteFact>,
}
#[derive(Debug, Serialize)]
pub struct Fault {
    pub message: String,
    pub missing: Option<Missing>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub scene: Scene,
    pub seed: u16,
    pub pointers: Vec<Pointer>,
    pub cells: Vec<ByteFact>,
    pub draws: Vec<Draw>,
    pub events: Vec<serde_json::Value>,
    #[serde(skip)]
    pub presentation: Vec<serde_json::Value>,
}
fn find_pointer(s: &State, kind: PointerKind) -> Result<&Pointer, String> {
    s.pointers
        .iter()
        .find(|p| p.kind == kind)
        .ok_or_else(|| "missing named pointer".into())
}
fn area_pointer(area: Area) -> Option<PointerKind> {
    Some(match area {
        Area::P8 => PointerKind::P8,
        Area::P15 => PointerKind::P15,
        Area::Club => PointerKind::Club,
        Area::Config => PointerKind::Config,
        Area::Style => PointerKind::Style,
        Area::Results => PointerKind::Results,
        Area::Fixtures => PointerKind::Fixtures,
        Area::Counts => PointerKind::Counts,
        Area::Main => PointerKind::Main,
        Area::MainShown => PointerKind::MainShown,
        Area::Background => PointerKind::Background,
        Area::BackgroundShown => PointerKind::BackgroundShown,
        _ => return None,
    })
}
fn pitch_offset(offset: i32) -> bool {
    (-8..=8).any(|row| {
        let start = 0x90E + 160 * row - 74;
        (start..start + 150).contains(&offset)
    })
}
pub(super) fn cell_address(s: &State, f: &ByteFact) -> Result<usize, String> {
    if f.area == Area::Frame {
        if !(-4096..=-1).contains(&f.offset)
            && !(s.kind == Kind::WatchedMatch && (8..64).contains(&f.offset))
        {
            return Err("unsupported entry frame field".into());
        }
        return Ok(16 * usize::from(s.regs[10]) + (i32::from(s.regs[4]) + f.offset) as usize);
    }
    if f.offset < 0 {
        return Err("negative named data offset".into());
    }
    let offset = f.offset as usize;
    if f.area == Area::Ds {
        if (0xD5A..0xD5C).contains(&offset) || (0xD088..0xD08A).contains(&offset) {
            return Err("presentation timer field outside action ownership".into());
        }
        if !((0x380..0x382).contains(&offset)
            || (0x900..0xC00).contains(&offset)
            || (0xD00..0xD80).contains(&offset)
            || (0x957D..=0xAD9A).contains(&offset)
            || (0xCCEF..0xD7BA).contains(&offset)
            || (s.kind == Kind::WatchedMatch
                && ((0xAD9B..0xCD00).contains(&offset)
                    || (0x0C66..0x0C68).contains(&offset)
                    || (0x0F20..0x0F30).contains(&offset)
                    || (0x129E..0x12BD).contains(&offset)
                    || (0x8708..0x8716).contains(&offset)
                    || (0x935B..0x94F7).contains(&offset)
                    || (0xD886..0xD936).contains(&offset)
                    || (0xD7D6..0xD7DA).contains(&offset)
                    || (0xD80..0xDA0).contains(&offset)
                    || (0xDCBA..0xDCBC).contains(&offset)
                    || (0xDD64..0xDD66).contains(&offset)
                    // [confirmed:T41 transient proof] 1D89 secondary255 reaches1EA6 atE5DD.
                    || [0xDD56, 0xDE88, 0xDE8A, 0xE5DD].contains(&offset))))
        {
            return Err("unsupported named DS field".into());
        }
        return Ok(16 * usize::from(s.regs[11]) + offset);
    }
    let kind = area_pointer(f.area).ok_or("unsupported named area")?;
    let p = find_pointer(s, kind)?;
    if offset >= kind.length() {
        return Err("named area offset outside bounds".into());
    }
    if matches!(
        f.area,
        Area::Main | Area::MainShown | Area::Background | Area::BackgroundShown
    ) && !(s.kind == Kind::WatchedMatch
        || pitch_offset(f.offset)
        || (matches!(
            s.kind,
            Kind::Actor | Kind::Tick | Kind::Caller | Kind::WatchedMatch
        ) && [-9, 9].into_iter().any(|row| {
            let start = 0x90E + 160 * row - 74;
            (start..start + 150).contains(&f.offset)
        })))
    {
        return Err("non-gameplay screen cell".into());
    }
    Ok(p.address() + offset)
}
pub(super) fn machine(s: &State) -> Result<Cpu, String> {
    if !(1..=22).contains(&s.scene.index)
        || s.scene.byte7 > 22
        || s.scene.word14 > 3920
        || s.regs[4] < 4096
        || s.regs[4] > 0xFFF0
    {
        return Err("unsupported action entry identity/stack".into());
    }
    let ds = 16 * usize::from(s.regs[11]);
    let scene = ds + 0xCCEF + 25 * usize::from(s.scene.index);
    let seed = ds + usize::from(crate::SEED);
    let stack = 16 * usize::from(s.regs[10]) + usize::from(s.regs[4]);
    let stack_end = stack
        + match s.kind {
            Kind::WatchedMatch => 64,
            Kind::Caller => 16,
            _ => 8,
        };
    if stack - 4096 < ds + 65536 {
        return Err("action stack overlaps data segment".into());
    }
    if !matches!(s.kind, Kind::Tick | Kind::Caller | Kind::WatchedMatch)
        && (usize::from(s.stack_words[2]) + 16 * usize::from(s.stack_words[3]) != scene
            || s.stack_words[1] != s.regs[9])
    {
        return Err("action entry far ABI differs".into());
    }
    let mut kinds = std::collections::BTreeSet::new();
    for p in &s.pointers {
        if !kinds.insert(p.kind)
            || p.address() + p.kind.length() > 0x110000
            || usize::from(p.offset) + p.kind.length() > 65536
        {
            return Err("duplicate/out-of-range named pointer".into());
        }
        if p.kind == PointerKind::Seed {
            if p.address() != seed {
                return Err("seed pointer aliases another field".into());
            }
        } else if p.kind != PointerKind::Commentary || p.address() != 0 {
            let range = p.address()..p.address() + p.kind.length();
            if p.address() < ds + 65536
                || range.contains(&seed)
                || range.contains(&scene)
                || (range.start < stack_end && stack - 4096 < range.end)
            {
                return Err("named pointer aliases protected entry state".into());
            }
        }
    }
    let mut required: std::collections::BTreeSet<_> = [
        PointerKind::P8,
        PointerKind::P15,
        PointerKind::Club,
        PointerKind::Config,
        PointerKind::Main,
        PointerKind::MainShown,
        PointerKind::Background,
        PointerKind::BackgroundShown,
        PointerKind::Commentary,
        PointerKind::Seed,
    ]
    .into_iter()
    .collect();
    if matches!(
        s.kind,
        Kind::Actor | Kind::Tick | Kind::Caller | Kind::WatchedMatch
    ) {
        required.insert(PointerKind::Style);
    }
    if s.kind == Kind::WatchedMatch {
        required.insert(PointerKind::Results);
        required.insert(PointerKind::Fixtures);
        required.insert(PointerKind::Counts);
        for ui in [
            PointerKind::Screen,
            PointerKind::ScreenShown,
            PointerKind::ScreenSaved,
        ] {
            required.insert(ui);
        }
    }
    if kinds != required {
        return Err("missing named action pointer".into());
    }
    if s.kind == Kind::WatchedMatch {
        for (ui, logical) in [
            (PointerKind::Screen, PointerKind::Main),
            (PointerKind::ScreenShown, PointerKind::MainShown),
            (PointerKind::ScreenSaved, PointerKind::BackgroundShown),
        ] {
            if find_pointer(s, ui)?.address() != find_pointer(s, logical)?.address() {
                return Err("watched UI page alias differs from initial source layout".into());
            }
        }
    }
    for (i, p) in s.pointers.iter().enumerate() {
        if matches!(p.kind, PointerKind::Seed | PointerKind::Commentary) {
            continue;
        }
        for q in &s.pointers[i + 1..] {
            if matches!(q.kind, PointerKind::Seed | PointerKind::Commentary) {
                continue;
            }
            let pool = |k| {
                matches!(
                    k,
                    PointerKind::P8
                        | PointerKind::P15
                        | PointerKind::Club
                        | PointerKind::Config
                        | PointerKind::Style
                        | PointerKind::Results
                        | PointerKind::Fixtures
                        | PointerKind::Counts
                )
            };
            if (pool(p.kind) || pool(q.kind))
                && p.address() < q.address() + q.kind.length()
                && q.address() < p.address() + p.kind.length()
            {
                return Err("named player/data pool overlaps another area".into());
            }
        }
    }

    let mut image = vec![0; 0x110000];
    let mut known = vec![false; image.len()];
    let mut seen = std::collections::BTreeSet::new();
    fn initial(
        image: &mut [u8],
        known: &mut [bool],
        seen: &mut std::collections::BTreeSet<usize>,
        at: usize,
        bytes: &[u8],
    ) -> Result<(), String> {
        if at + bytes.len() > image.len() {
            return Err("entry data exceeds image".into());
        }
        for (i, b) in bytes.iter().enumerate() {
            if !seen.insert(at + i) {
                return Err("aliased action entry fact".into());
            }
            image[at + i] = *b;
            known[at + i] = true;
        }
        Ok(())
    }
    initial(&mut image, &mut known, &mut seen, scene, &s.scene.bytes())?;
    initial(
        &mut image,
        &mut known,
        &mut seen,
        seed,
        &s.seed.to_le_bytes(),
    )?;
    for (i, w) in s.stack_words.iter().enumerate() {
        initial(
            &mut image,
            &mut known,
            &mut seen,
            stack + 2 * i,
            &w.to_le_bytes(),
        )?;
    }
    for p in &s.pointers {
        let at = ds + usize::from(p.kind.slot());
        initial(
            &mut image,
            &mut known,
            &mut seen,
            at,
            &p.offset.to_le_bytes(),
        )?;
        initial(
            &mut image,
            &mut known,
            &mut seen,
            at + 2,
            &p.segment.to_le_bytes(),
        )?;
    }
    for f in &s.cells {
        let a = cell_address(s, f)?;
        initial(&mut image, &mut known, &mut seen, a, &[f.value])?;
    }
    let mut e = crate::role_scores::image(Default::default());
    e.image = image.into_boxed_slice();
    let mut c = Cpu::new(e);
    c.known = known;
    c.flat_memory = true;
    c.code_base = s.regs[9].wrapping_sub(0x0AB9);
    c.r = s.regs;
    c.stack_bounds = Some((stack - 4096, stack_end));
    c.cf = s.flags & 1 != 0;
    c.pf = s.flags & 4 != 0;
    c.z = s.flags & 0x40 != 0;
    c.sf = s.flags & 0x80 != 0;
    c.of = s.flags & 0x800 != 0;
    c.df = s.flags & 0x400 != 0;
    c.track_events = true;
    c.require_write_baseline = true;
    Ok(c)
}
pub fn advance(s: &State) -> Result<Outcome, Fault> {
    let mut c = machine(s).map_err(|message| Fault {
        message,
        missing: None,
    })?;
    let style = if matches!(
        s.kind,
        Kind::Actor | Kind::Tick | Kind::Caller | Kind::WatchedMatch
    ) {
        let a = find_pointer(s, PointerKind::Style).unwrap().address();
        Some((a, c.e.image[a]))
    } else {
        None
    };
    let entry = match s.kind {
        Kind::Action5 => super::action_graph::p_0ab9_448c,
        Kind::Contact => super::action_graph::p_0ab9_4dff,
        Kind::Actor => super::action_graph::p_0ab9_72bb,
        Kind::Tick | Kind::Caller | Kind::WatchedMatch => {
            return Err(Fault {
                message: "tick requires a persistent TickSession".into(),
                missing: None,
            })
        }
    };
    if let Err(message) = c.invoke_entry(entry, 0x0AB9, s.stack_words[0]) {
        return Err(Fault {
            message,
            missing: c.missing.get(),
        });
    }
    if c.r[4] != s.regs[4].wrapping_add(8) || c.r[5] != s.regs[5] || c.r[11] != s.regs[11] {
        return Err(Fault {
            message: "action Pascal entry/return handoff differs".into(),
            missing: None,
        });
    }
    if let Some((a, before)) = style {
        if c.e.image[a] != before {
            return Err(Fault {
                message: "actor modified readonly logical style".into(),
                missing: None,
            });
        }
    }
    let mut output = s.clone();
    let ds = 16 * usize::from(s.regs[11]);
    let at = ds + 0xCCEF + 25 * usize::from(s.scene.index);
    // Scene's typed accessor uses canonical DS addresses;copy only the25 decoded fields.
    let mut canonical = crate::role_scores::image(Default::default());
    let to = 0xCCEF + 25 * usize::from(s.scene.index);
    canonical.image[to..to + 25].copy_from_slice(&c.e.image[at..at + 25]);
    output.scene = Scene::read(&canonical, s.scene.index);
    output.seed = u16::from_le_bytes([c.e.image[ds + 0xA51C], c.e.image[ds + 0xA51D]]);
    output.cells.retain(|f| f.area != Area::Frame);
    for f in &mut output.cells {
        f.value = c.e.image[cell_address(s, f).map_err(|message| Fault {
            message,
            missing: None,
        })?];
    }
    for p in &mut output.pointers {
        let a = ds + usize::from(p.kind.slot());
        p.offset = u16::from_le_bytes([c.e.image[a], c.e.image[a + 1]]);
        p.segment = u16::from_le_bytes([c.e.image[a + 2], c.e.image[a + 3]]);
    }
    Ok(Outcome {
        scene: output.scene,
        seed: output.seed,
        pointers: output.pointers,
        cells: output.cells,
        draws: c.draws,
        events: c.events,
        presentation: c.presentation,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn context() -> State {
        serde_json::from_str(include_str!(
            "../../tests/fixtures/watched_action/context.json"
        ))
        .unwrap()
    }
    #[test]
    fn original_entry_context_initializes_one_data_image_and_real_abi() {
        let s = context();
        let c = machine(&s).unwrap();
        let ds = usize::from(s.regs[11]) * 16;
        assert!(c.flat_memory);
        assert_eq!(c.r, s.regs);
        assert_eq!(c.e.image.len(), 0x110000);
        assert_eq!(
            u16::from_le_bytes([c.e.image[ds + 0xA51C], c.e.image[ds + 0xA51D]]),
            s.seed
        );
        let a = ds + 0xCCEF + 25 * usize::from(s.scene.index);
        assert_eq!(&c.e.image[a..a + 25], &s.scene.bytes());
        let stack = usize::from(s.regs[10]) * 16 + usize::from(s.regs[4]);
        for (i, w) in s.stack_words.iter().enumerate() {
            assert_eq!(
                u16::from_le_bytes([c.e.image[stack + 2 * i], c.e.image[stack + 2 * i + 1]]),
                *w
            );
        }
    }
    #[test]
    fn entry_schema_and_reserved_aliases_are_strict() {
        let s = context();
        let mut v = serde_json::to_value(&s).unwrap();
        v["expected_helpers"] = serde_json::json!([]);
        assert!(serde_json::from_value::<State>(v).is_err());
        let mut s = s.clone();
        s.cells.push(ByteFact {
            area: Area::Ds,
            offset: 0xA51C,
            value: 0,
        });
        assert!(machine(&s).is_err());
        let mut s = context();
        s.cells.push(ByteFact {
            area: Area::Ds,
            offset: 0xD7BA,
            value: 0,
        });
        assert!(machine(&s).is_err());
        let mut s = context();
        s.pointers.push(s.pointers[0].clone());
        assert!(machine(&s).is_err());
        let mut s = context();
        let p = s
            .pointers
            .iter_mut()
            .find(|p| p.kind == PointerKind::P8)
            .unwrap();
        p.segment = s.regs[11];
        p.offset = 0xA51C;
        assert!(machine(&s).is_err());
    }
    fn original_case(index: usize) -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "tests/fixtures/watched_action/case-{index:05}.json"
        ));
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
    }
    #[test]
    fn real_original_action_and_commentary_roots_replay() {
        for index in [0, 113] {
            let case = original_case(index);
            let input: State = serde_json::from_value(case["input"].clone()).unwrap();
            let actual = advance(&input).unwrap_or_else(|fault| panic!("case{index}: {fault:?}"));
            assert_eq!(
                serde_json::to_value(actual).unwrap(),
                case["expected"],
                "case{index}"
            );
        }
    }
    #[test]
    fn actual_required_entry_fact_cannot_be_silently_defaulted() {
        let case = original_case(0);
        let mut input: State = serde_json::from_value(case["input"].clone()).unwrap();
        input.cells.remove(0);
        let fault = advance(&input).unwrap_err();
        assert!(fault.missing.is_some(), "{fault:?}");
    }

    #[test]
    fn entry_player_pools_cannot_alias_each_other_or_pitch_storage() {
        let mut s = context();
        let p8 = s
            .pointers
            .iter()
            .find(|p| p.kind == PointerKind::P8)
            .unwrap()
            .clone();
        let p15 = s
            .pointers
            .iter_mut()
            .find(|p| p.kind == PointerKind::P15)
            .unwrap();
        p15.offset = p8.offset;
        p15.segment = p8.segment;
        assert!(machine(&s).is_err());
        let mut s = context();
        let p8 = s
            .pointers
            .iter()
            .find(|p| p.kind == PointerKind::P8)
            .unwrap()
            .clone();
        let main = s
            .pointers
            .iter_mut()
            .find(|p| p.kind == PointerKind::Main)
            .unwrap();
        main.offset = p8.offset;
        main.segment = p8.segment;
        assert!(machine(&s).is_err());
    }

    #[test]
    fn original_wait_requests_presentation_time_without_gameplay_or_rng_effects() {
        let mut s = context();
        s.cells.push(ByteFact {
            area: Area::Config,
            offset: 2,
            value: 1,
        });
        let mut c = machine(&s).unwrap();
        let before = c.e.image.clone();
        let sp = c.r[4];
        c.push(17).unwrap();
        c.invoke(
            super::super::projections::sound_delay,
            true,
            0x1998,
            0x00D6,
            0x2222,
        )
        .unwrap();
        assert_eq!(c.r[4], sp);
        assert_eq!(
            c.presentation,
            vec![serde_json::json!({"site":"1998:00D6","kind":"delay","ticks":17})]
        );
        assert!(c.draws.is_empty() && c.events.is_empty());
        let (lo, hi) = c.stack_bounds.unwrap();
        for (a, (old, new)) in before.iter().zip(c.e.image.iter()).enumerate() {
            if a < lo || a >= hi {
                assert_eq!(old, new, "gameplay byte{a:05X}");
            }
        }
        for offset in [0xD5A, 0xD5B, 0xD088, 0xD089] {
            let mut s = s.clone();
            s.cells.push(ByteFact {
                area: Area::Ds,
                offset,
                value: 0,
            });
            assert!(machine(&s).is_err());
        }
    }

    #[test]
    fn a_pool_cannot_clip_the_entry_stack_after_its_first_byte() {
        let mut s = context();
        let ss = s.regs[10];
        let sp = s.regs[4];
        let p = s
            .pointers
            .iter_mut()
            .find(|p| p.kind == PointerKind::Main)
            .unwrap();
        p.segment = ss;
        p.offset = sp + 1;
        assert!(machine(&s).is_err());
    }
    #[test]
    fn the_private_call_stack_cannot_alias_the_data_segment() {
        let mut s = context();
        s.regs[10] = s.regs[11];
        assert!(machine(&s).is_err());
    }

    #[test]
    fn every_captured_original_action_boundary_replays() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(
            &std::fs::read(root.join("engine/tests/fixtures/watched_action/manifest.json"))
                .unwrap(),
        )
        .unwrap();
        let cases = manifest["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 3768);
        for item in cases {
            let case: serde_json::Value = serde_json::from_slice(
                &std::fs::read(root.join(item["path"].as_str().unwrap())).unwrap(),
            )
            .unwrap();
            let input: State = serde_json::from_value(case["input"].clone()).unwrap();
            let index = case["case"].as_u64().unwrap();
            let actual = advance(&input).unwrap_or_else(|fault| panic!("case{index}: {fault:?}"));
            assert_eq!(
                serde_json::to_value(actual).unwrap(),
                case["expected"],
                "original case{index}"
            );
        }
    }
    #[test]
    fn actor_entry_requires_an_explicit_logical_render_style_pointer() {
        let mut s = context();
        s.kind = Kind::Actor;
        assert!(
            machine(&s).is_err(),
            "actor entry accepted no typed renderer-style pointer"
        );
        s.pointers.push(Pointer {
            kind: PointerKind::Style,
            offset: 31,
            segment: 28274,
        });
        s.cells.push(ByteFact {
            area: Area::Style,
            offset: 0,
            value: 0,
        });
        let c = machine(&s).unwrap();
        let at = 16 * 28274 + 31;
        assert!(c.known[at]);
        assert_eq!(c.e.image[at], 0);
        let mut extra = s.clone();
        extra.cells[0].offset = 1;
        assert!(machine(&extra).is_err());
        let mut wrong = s.clone();
        wrong.kind = Kind::Action5;
        assert!(machine(&wrong).is_err());
        let mut missing = s.clone();
        missing.pointers.retain(|p| p.kind != PointerKind::P8);
        assert!(machine(&missing).is_err());
    }

    #[test]
    fn actor_set_piece_margin_keeps_logical_occupancy_and_prior_bounds() {
        let mut s = context();
        s.kind = Kind::Actor;
        s.pointers.push(Pointer {
            kind: PointerKind::Style,
            offset: 31,
            segment: 28274,
        });
        let p = find_pointer(&s, PointerKind::Main).unwrap().address();
        let fact = ByteFact {
            area: Area::Main,
            offset: 0x36e,
            value: 0,
        };
        assert_eq!(cell_address(&s, &fact).unwrap(), p + 0x36e);
        s.kind = Kind::Action5;
        assert!(cell_address(&s, &fact).is_err());
    }
    #[test]
    fn all_original_parent_dispatch_and_shot_fixtures_replay() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(
            &std::fs::read(root.join("engine/tests/fixtures/watched_actor/manifest.json")).unwrap(),
        )
        .unwrap();
        let items = manifest["cases"].as_array().unwrap();
        assert_eq!(items.len(), 50);
        for item in items {
            let case: serde_json::Value = serde_json::from_slice(
                &std::fs::read(root.join(item["path"].as_str().unwrap())).unwrap(),
            )
            .unwrap();
            let state: State = serde_json::from_value(case["input"].clone()).unwrap();
            assert_eq!(
                serde_json::to_value(advance(&state).unwrap()).unwrap(),
                case["expected"],
                "original parent{}",
                case["case"]
            );
        }
    }
    #[test]
    fn valid_original_parent_fixture_replays() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/watched_actor/case-0000.json");
        let case: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let input: State = serde_json::from_value(case["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(advance(&input).unwrap()).unwrap(),
            case["expected"]
        );
    }
}
