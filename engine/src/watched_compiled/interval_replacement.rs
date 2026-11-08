//! [confirmed:archaeology1d6b5b9,T74] Fresh original bench keys and bounded
//! transaction observation. Gameplay remains in original CBF0/C866/AEB8.
use super::compiled::Cpu;
use serde_json::{json, Value};
use std::collections::{BTreeSet, VecDeque};

pub(super) struct Intent {
    pub keys: VecDeque<u16>,
    pub samples: Vec<Value>,
    pub opened: bool,
    pub closed: bool,
}
impl Intent {
    pub fn new(cancel_first: bool) -> Self {
        let mut keys = vec![32, 6, 2];
        if cancel_first {
            keys.extend([1, 2]);
        }
        keys.extend(std::iter::repeat_n(6, 10));
        keys.extend([2, 1]);
        Self {
            keys: keys.into(),
            samples: vec![],
            opened: false,
            closed: false,
        }
    }
}
// Same decoded field census as the authenticated original observer. Reads go
// through known-memory checks; no output or expected state enters the engine.
pub(super) fn projection(c: &Cpu) -> Result<Value, String> {
    let ds = c.r[11];
    let rd = |at, width| c.read(ds, at, width);
    let mut clubs = vec![];
    let mut ids = BTreeSet::new();
    let (club_off, club_seg) = c.far(ds, 0xD7C2)?;
    for side in 0..2u16 {
        let id = rd(0xD6E4 + 2 * side, 16)?;
        let base = club_off.wrapping_add(id.wrapping_sub(1).wrapping_mul(62));
        let mut roster = vec![];
        for i in 0..20 {
            let p = c.read(club_seg, base.wrapping_add(2 * i), 16)?;
            roster.push(p);
            if p != 0 {
                ids.insert(p);
            }
        }
        clubs.push(json!({"id":id,"roster":roster}));
    }
    let (p8, s8) = c.far(ds, 0xD7BA)?;
    let (p15, s15) = c.far(ds, 0xD7BE)?;
    let mut players = vec![];
    for id in ids {
        let a = p8.wrapping_add(id.wrapping_mul(8));
        let b = p15.wrapping_add(id.wrapping_mul(15));
        players.push(json!({"id":id,"current_position":c.read(s8,a.wrapping_add(6),8)?,"club_position":c.read(s8,a.wrapping_add(7),8)?,"condition":c.read(s15,b.wrapping_add(8),8)?,"instructions":c.read(s15,b.wrapping_add(10),8)?,"status":c.read(s15,b.wrapping_add(11),8)?}));
    }
    let (seed, seedseg) = c.far(ds, 0xDE84)?;
    let state = super::watched_match::snapshot(c)?;
    let mut totals = vec![];
    let mut flags = vec![];
    for side in 0..2u16 {
        totals.push(
            (0..20)
                .map(|i| rd(0xD148 + 40 * side + 2 * i, 16))
                .collect::<Result<Vec<_>, _>>()?,
        );
        flags.push(
            (0..20)
                .map(|i| rd(0xD198 + 20 * side + i, 8))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    Ok(
        json!({"seed":c.read(seedseg,seed,16)?,"clubs":clubs,"players":players,"scenes":state["scenes"],"elapsed":state["elapsed"],"remaining":state["remaining"],"stoppage":state["stoppage"],"active":state["active"],"phase":state["phase"],"live_goals":[rd(0xD0C4,8)?,rd(0xD0C5,8)?],"allowance":rd(0xD6DD,8)?,"sub_flags":[rd(0xD6DE,8)?,rd(0xD6DF,8)?],"reserves":[rd(0xD6E0,8)?,rd(0xD6E1,8)?],"pending":rd(0xD6E2,8)?,"control":rd(0xD0BA,8)?,"event_totals":totals,"event_flags":flags}),
    )
}

pub(super) fn key(c: &mut Cpu, bench: bool, players: bool) -> Result<Option<u16>, String> {
    if c.replacement.as_ref().is_none_or(|i| i.closed) {
        return Ok(None);
    }
    if c.read(c.r[11], 0xD0BA, 8)? != 1
        || c.read(c.r[11], 0xD0AE, 16)? == 0
        || c.watch_pending.is_some()
    {
        return Err("replacement key outside live interval".into());
    }
    let intent = c.replacement.as_ref().unwrap();
    if (!intent.opened && !bench)
        || (intent.opened && !intent.keys.is_empty() && !players)
        || (intent.keys.is_empty() && !bench)
    {
        return Err(format!(
            "replacement input caller differs: {:?}",
            c.call_chain()
        ));
    }
    let state = projection(c)?;
    let (selected, picked) = if players {
        // At 0C17 entry,099B's saved BP belongs to CBF0 (direct call C E38).
        let bp = c.read(c.r[10], c.r[5], 16)?;
        (
            Some(c.read(c.r[11], 0x037E, 8)?),
            Some(c.read(c.r[10], bp.wrapping_sub(10), 8)?),
        )
    } else {
        (None, None)
    };
    let (club_base, club_segment) = c.far(c.r[11], 0xD7C2)?;
    let club_roster_count = c.read(club_segment, club_base.wrapping_add(8048), 8)?;
    let intent = c.replacement.as_mut().unwrap();
    let key = intent.keys.pop_front();
    intent.samples.push(
        json!({"key":key,"state":state,"selected_slot":selected,"picked":picked,"bench":bench,"club_roster_count":club_roster_count}),
    );
    intent.opened = true;
    if key.is_none() {
        intent.closed = true;
    }
    Ok(key)
}
