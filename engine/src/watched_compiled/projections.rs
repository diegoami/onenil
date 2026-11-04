//! [confirmed] Original presentation boundaries,static d9e20a4.
//! Numeric queue/counter effects remain in compiled0F94/0E45/0F42.
//! Sound countdown/ISR phase ownership:main amendment8fdce1d and
//! records/T37/presentation-clock-boundary-r0001.json.
use super::compiled::Cpu;
pub(super) fn commentary_display(c: &mut Cpu) -> Result<(), String> {
    c.ret(false, 6)
}
fn sound_request(c: &mut Cpu, burst: bool) -> Result<(), String> {
    c.enter(if burst { 6 } else { 0 })?;
    let (offset, segment) = c.far(c.r[11], 0xD7D2)?;
    if c.read(segment, offset.wrapping_add(2), 8)? != 0 {
        let duration = c.read(c.r[10], c.r[5].wrapping_add(6), 16)?;
        let ticks = if burst {
            duration.wrapping_mul(c.read(c.r[11], 0xD5C, 16)?)
        } else {
            duration
        };
        c.presentation.push(serde_json::json!({"site":if burst{"1998:01E0"}else{"1998:00D6"},"kind":if burst{"tone_burst"}else{"delay"},"ticks":ticks}));
    }
    c.r[4] = c.r[5];
    c.r[5] = c.pop()?;
    c.ret(true, 2)
}
pub(super) fn sound_delay(c: &mut Cpu) -> Result<(), String> {
    sound_request(c, false)
}
pub(super) fn sound_burst(c: &mut Cpu) -> Result<(), String> {
    sound_request(c, true)
}

// [confirmed] Original78B7 no-key path791B→79D5,DB68 guards (static d9e20a4).
// [designed] TickSession explicitly admits a no-pending-key external input.
// Status text lives outside the declared gameplay pitch;mode writes remain native.
pub(super) fn neutral_watch_input(c: &mut Cpu) -> Result<(), String> {
    let ds = c.r[11];
    let (o, s) = c.far(ds, 0xD7D2)?;
    let speed = c.read(s, o.wrapping_add(2), 8)?;
    c.presentation
        .push(serde_json::json!({"site":"0AB9:78B7","kind":"speed_status","speed":speed}));
    if c.read(ds, 0xD1CC, 8)? != 0
        && c.read(ds, 0xD074, 8)? == 0
        && c.read(ds, 0xD0AE, 16)? != 0
        && c.read(ds, 0xD0B9, 8)? == 0
    {
        if c.read(ds, 0xD0B8, 8)? == 0 {
            c.write(ds, 0xD1CC, 8, 1)?;
        } else {
            c.write(ds, 0xD074, 8, 1)?;
            c.write(ds, 0xD075, 8, 1)?;
            c.write(ds, 0xD1CC, 8, 0)?;
        }
    }
    if c.read(ds, 0xD074, 8)? != 0 {
        c.presentation
            .push(serde_json::json!({"site":"0AB9:79E6","kind":"speaker_off"}));
    }
    c.ret(false, 0)
}

// [confirmed] Original00D6 renders the remaining-time digits through DD58/UI buffers.
// [designed] Clock counters/events remain compiled7B7B;this emits only display data.
pub(super) fn clock_display(c: &mut Cpu) -> Result<(), String> {
    let remaining = c.read(c.r[10], c.r[4].wrapping_add(4), 16)?;
    c.presentation
        .push(serde_json::json!({"site":"0AB9:00D6","kind":"remaining_time","value":remaining}));
    c.ret(false, 4)
}

// [confirmed] 1998:00CD→00AB restores the PIT divisor and interrupt vector only.
// Source0058/0096 performs no gameplay write;hardware ownership is external.
// Called at E470 immediately before the outer LEAVE/RET4;no later register consumer.
pub(super) fn restore_presentation_clock(c: &mut Cpu) -> Result<(), String> {
    c.presentation
        .push(serde_json::json!({"site":"1998:00CD","kind":"restore_presentation_clock"}));
    c.ret(true, 0)
}

// [confirmed:T41 statistics-boundary-r0001/r0002] Original045C changes only
// presentation pages/style,key state and stack;all other memory is unchanged.
// At7E2F the source caller restores its saved pitch before any watched tick.
// E45E is after the final watched walk-off. Other callers remain unsupported.
pub(super) fn statistics_display(c: &mut Cpu) -> Result<(), String> {
    if !matches!(
        c.call_chain().last().copied(),
        Some((0x0AB9, 0x7E2F | 0xE45E))
    ) {
        return Err("statistics display caller is outside the proven lifecycle boundary".into());
    }
    let mode = c.read(c.r[10], c.r[4].wrapping_add(4), 16)?;
    let wait = c.read(c.r[10], c.r[4].wrapping_add(6), 16)?;
    if mode > 1 || wait > 1 {
        return Err("unsupported statistics presentation arguments".into());
    }
    let ds = c.r[11];
    let goals = [c.read(ds, 0xD0C4, 8)?, c.read(ds, 0xD0C5, 8)?];
    let elapsed = c.read(ds, 0xD0B2, 16)?;
    c.presentation.push(serde_json::json!({"site":"1998:045C","kind":"statistics","mode":mode,"wait":wait!=0,"live_goals":goals,"elapsed":elapsed}));
    c.write(ds, 0xDE88, 8, 0)?; // Original0812 clears the consumed presentation key.
    c.ret(true, 4)
}
