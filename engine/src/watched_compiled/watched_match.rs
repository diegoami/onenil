//! [confirmed] Original lifecycle7C67→7DCB→E05E→E435→EBE2,staticd9e20a4/F40.
//! [designed] One initial fact image;all gameplay/lifecycle executes in Rust.
use super::{
    action::{self, Fault, Kind, State},
    compiled::Cpu,
};
use serde_json::{json, Value};
pub(super) fn snapshot(c: &Cpu) -> Result<Value, String> {
    let ds = c.r[11];
    let read = |offset, width| c.read(ds, offset, width);
    let mut scenes = Vec::new();
    for index in 1..=22u16 {
        let mut scene = json!({"index":index});
        for offset in [0, 2, 4, 8, 10, 14, 16, 20] {
            scene[format!("word{offset}")] = json!(read(0xCCEF + 25 * index + offset, 16)?);
        }
        for offset in [6, 7, 12, 13, 18, 19, 22, 23, 24] {
            scene[format!("byte{offset}")] = json!(read(0xCCEF + 25 * index + offset, 8)?);
        }
        scenes.push(scene);
    }
    let mut ball = json!({});
    for offset in [0, 2, 4, 7, 9, 11] {
        ball[format!("word{offset}")] = json!(read(0xCF46 + offset, 16)?);
    }
    for offset in [6, 13, 14] {
        ball[format!("byte{offset}")] = json!(read(0xCF46 + offset, 8)?);
    }
    // Reference's legacy `scores` are D086/D087 scratch during play;live counters D0C4/C5.
    Ok(
        json!({"seed":read(0xA51C,16)?,"elapsed":read(0xD0B2,16)?,"remaining":read(0xD0AE,16)?,"quick":read(0xD074,8)?,"actor":read(0xD09F,8)?,"secondary":read(0xD0A0,8)?,"ball":ball,"scenes":scenes,"scores":[read(0xD086,8)?,read(0xD087,8)?],"phase":read(0xD0AC,8)?,"active":read(0xD0B8,8)?,"hold":read(0xD0B9,8)?,"stoppage":read(0xD0B4,16)?}),
    )
}
#[derive(serde::Serialize)]
pub struct MatchFault {
    #[serde(flatten)]
    pub fault: Fault,
    pub ticks: Vec<Value>,
    pub draws: Vec<crate::Draw>,
    pub events: Vec<Value>,
}
impl From<Fault> for MatchFault {
    fn from(fault: Fault) -> Self {
        Self {
            fault,
            ticks: vec![],
            draws: vec![],
            events: vec![],
        }
    }
}
pub fn run(s: &State, no_pending_key: bool) -> Result<Value, MatchFault> {
    let failure = |message| {
        MatchFault::from(Fault {
            message,
            missing: None,
        })
    };
    if s.kind != Kind::WatchedMatch || !no_pending_key || s.regs[5] != s.regs[4].wrapping_add(6) {
        return Err(failure(
            "watched match requires original parent frame and no-pending-key admission".into(),
        ));
    }
    let mut c = action::machine(s).map_err(failure)?;
    let ds = s.regs[11];
    let ss = s.regs[10];
    let sp = s.regs[4];
    for (offset, expected) in [(8, 0x7DCB), (16, 0xE05E), (28, 0xE435), (46, 0xEBE2)] {
        if c.read(ss, sp.wrapping_add(offset), 16)
            .map_err(|message| Fault {
                message,
                missing: c.missing.get(),
            })?
            != expected
        {
            return Err(failure("watched match outer return frame differs".into()));
        }
    }
    c.watch_trace = Some(Vec::new());
    let result: Result<Value, String> = (|| {
        c.invoke_entry(super::match_graph::p_0ab9_7c67, 0x0AB9, 0x7DCB)?;
        c.invoke_entry(super::match_graph::p_0ab9_7dcb, 0x0AB9, 0xE05E)?;
        c.invoke_entry(super::match_graph::p_0ab9_e05e, 0x0AB9, 0xE435)?;
        c.invoke_entry(super::match_graph::p_0ab9_e435, 0x0AB9, 0xEBE2)?;
        if c.watch_pending.is_some() {
            return Err("watched match left an unfinished tick".into());
        }
        if c.r[4] != sp + 52 || c.r[11] != ds {
            return Err("watched match final return ABI differs".into());
        }
        let final_state = snapshot(&c)?;
        let live = [c.read(ds, 0xD0C4, 8)?, c.read(ds, 0xD0C5, 8)?];
        Ok(
            json!({"ticks":c.watch_trace,"draws":c.draws,"events":c.events,"final":final_state,"live_goals":live,"return":"0AB9:EBE2"}),
        )
    })();
    result.map_err(|message| MatchFault {
        fault: Fault {
            message: format!(
                "watched tick{} draw{}: {message}",
                c.watch_tick,
                c.draws.len()
            ),
            missing: c.missing.get(),
        },
        ticks: c.watch_trace.take().unwrap_or_default(),
        draws: std::mem::take(&mut c.draws),
        events: std::mem::take(&mut c.events),
    })
}
