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

// [confirmed:T41 statistics-boundary-r0001..r0004] Original045C changes only
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
    c.pc = (0x1998, 0x0812);
    c.write(ds, 0xDE88, 8, 0)?; // Original0812 clears the consumed presentation key.
    c.pc = (0x1998, 0x0818);
    c.ret(true, 4)
}

// [confirmed:T41/template-boundary-r0001] Source232A:052F→24BC:0000
// writes only the destination text page and restores DS/BP/SP. MainShown is
// presentation here;all lineup/substitution computations stay source-compiled.
pub(super) fn menu_template(c: &mut Cpu) -> Result<(), String> {
    let ss = c.r[10];
    let sp = c.r[4];
    let (header, header_segment) = c.far(ss, sp.wrapping_add(4))?;
    let (template, template_segment) = c.far(ss, sp.wrapping_add(8))?;
    let call = c.call_chain().last().copied();
    let supported = matches!(
        (template, call),
        (0x132E, Some((0x1B55, 0x346C)) | Some((0x0AB9, 0xD6D6)))
            | (0x6346, Some((0x1B55, 0x347B)))
            | (0x25CC, Some((0x0AB9, 0xD6E5)))
            | (0x4F20, Some((0x0AB9, 0x971A)))
            | (0x3B82, Some((0x1B55, 0x334F)))
            | (0x3D72, Some((0x0AB9, 0x9FE3)))
    );
    if c.watch_trace.is_none()
        || !supported
        || header != 0xDD5C
        || header_segment != c.r[11]
        || template_segment != c.r[11]
    {
        return Err("menu template outside proven presentation caller/page".into());
    }
    c.far(header_segment, header)?; // Requires the admitted typed presentation pointer.
    c.presentation
        .push(serde_json::json!({"site":"232A:052F","kind":"menu_template","template":template}));
    c.ret(true, 8)
}

// [confirmed:T41/ui-leaves-boundary-r0001] Text/name rendering affects only
// MainShown/style/stack. Preserve the numeric style writes from the source.
fn menu_text(c: &mut Cpu, root: u16, styled: bool) -> Result<(), String> {
    let sp = c.r[4];
    let ss = c.r[10];
    let ds = c.r[11];
    let (header, segment) = c.far(ss, sp.wrapping_add(4))?;
    let col = c.read(ss, sp.wrapping_add(8), 8)?;
    let row = c.read(ss, sp.wrapping_add(10), 8)?;
    if c.watch_trace.is_none()
        || header != 0xDD5C
        || segment != ds
        || col > 80
        || row == 0
        || row > 25
    {
        return Err("menu text outside proven presentation page/domain".into());
    }
    c.far(ds, header)?;
    if styled {
        let style = c.read(ss, sp.wrapping_add(16), 8)?;
        c.pc = (0x232A, 0x043B);
        c.write(ds, 0xDD64, 16, style)?;
    }
    c.presentation.push(serde_json::json!({"site":format!("232A:{root:04X}"),"kind":"menu_text","column":col,"row":row}));
    c.ret(true, if styled { 14 } else { 12 })
}
pub(super) fn menu_right_text(c: &mut Cpu) -> Result<(), String> {
    menu_text(c, 0x0283, false)
}
pub(super) fn menu_left_text(c: &mut Cpu) -> Result<(), String> {
    menu_text(c, 0x039D, false)
}
pub(super) fn menu_styled_text(c: &mut Cpu) -> Result<(), String> {
    menu_text(c, 0x041B, true)
}
pub(super) fn menu_team_name(c: &mut Cpu) -> Result<(), String> {
    if c.watch_trace.is_none() {
        return Err("team name outside watched presentation".into());
    }
    let col = c.read(c.r[10], c.r[4].wrapping_add(4), 8)?;
    let row = c.read(c.r[10], c.r[4].wrapping_add(6), 8)?;
    let club = c.read(c.r[10], c.r[4].wrapping_add(8), 16)?;
    if club > 176 || col > 80 || row == 0 || row > 25 {
        return Err("unsupported team-name presentation arguments".into());
    }
    c.enter(0)?;
    c.pc = (0x232A, 0x043B);
    c.write(c.r[11], 0xDD64, 16, 9)?;
    if club != 0 {
        c.push(club)?;
        c.invoke(
            super::action_graph::p_1b55_0ea2,
            true,
            0x1B55,
            0x0EA2,
            0x0F0E,
        )?;
        c.pc = (0x1B55, 0x0F10);
        c.write(c.r[11], 0xDD64, 16, c.r[0] & 255)?;
    }
    c.presentation.push(serde_json::json!({"site":"1B55:0EE3","kind":"team_name","club":club,"column":col,"row":row}));
    c.r[4] = c.r[5];
    c.r[5] = c.pop()?;
    c.ret(true, 6)
}
pub(super) fn menu_player_name(c: &mut Cpu) -> Result<(), String> {
    if c.watch_trace.is_none() {
        return Err("player name outside watched presentation".into());
    }
    let col = c.read(c.r[10], c.r[4].wrapping_add(4), 8)?;
    let row = c.read(c.r[10], c.r[4].wrapping_add(6), 8)?;
    let player = c.read(c.r[10], c.r[4].wrapping_add(8), 16)?;
    if player > 3920 || col > 80 || row == 0 || row > 25 {
        return Err("unsupported player-name presentation arguments".into());
    }
    if player != 0 {
        let (offset, segment) = c.far(c.r[11], 0xD7BA)?;
        let role = c.read(
            segment,
            offset.wrapping_add(player.wrapping_mul(8)).wrapping_add(6),
            8,
        )?;
        if role > 13 {
            return Err("unsupported player-name role style".into());
        }
        let style = c.read(c.r[11], 0x8708 + role, 8)?;
        c.pc = (0x232A, 0x043B);
        c.write(c.r[11], 0xDD64, 16, style)?;
    }
    c.presentation.push(serde_json::json!({"site":"1B55:1FF1","kind":"player_name","player":player,"column":col,"row":row}));
    c.ret(true, 6)
}
