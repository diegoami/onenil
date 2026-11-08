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
    // [derived:archaeology1d6b5b9,CBF0;232A:052F..0545]
    // These editor callers use the same proven destination-page-only copy.
    // Admit them only under the owned fresh replacement controller.
    let editor_template = c.replacement.is_some()
        && c.read(c.r[11], 0xD0BA, 8)? == 1
        && c.call_chain().contains(&(0x0AB9, 0xD1FC))
        && matches!(
            (template, call),
            (0x22AC, Some((0x0AB9, 0xACA3)))
                | (0x4924, Some((0x0AB9, 0xACE3)))
                | (0x498C, Some((0x0AB9, 0xCDBB)))
                | (0x1D5E, Some((0x1B55, 0x479F)))
        );
    // [confirmed:T41/final-footer-r0001] E1BF is after all watched ticks.
    let footer = template == 0x4B12
        && call == Some((0x0AB9, 0xE1BF))
        && header == 0xDD58
        && c.watch_tick > 0
        && c.watch_pending.is_none()
        && c.read(c.r[11], 0xD0AE, 16)? == 0;
    if c.watch_trace.is_none()
        || !(((supported || editor_template) && header == 0xDD5C) || footer)
        || header_segment != c.r[11]
        || template_segment != c.r[11]
    {
        return Err(format!("menu template outside proven presentation caller/page: template={template:04X} header={header:04X} call={call:?} chain={:?}",c.call_chain()));
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
        c.pc = (0x1B55, 0x2070);
        let role = c.read(
            segment,
            offset.wrapping_add(player.wrapping_mul(8)).wrapping_add(6),
            8,
        )?;
        if role > 13 {
            return Err("unsupported player-name role style".into());
        }
        c.pc = (0x1B55, 0x2078);
        let style = c.read(c.r[11], 0x8708 + role, 8)?;
        c.pc = (0x232A, 0x043B);
        c.write(c.r[11], 0xDD64, 16, style)?;
    }
    c.presentation.push(serde_json::json!({"site":"1B55:1FF1","kind":"player_name","player":player,"column":col,"row":row}));
    c.ret(true, 6)
}

// [confirmed:T41/formation-boundary-r0001] Original10AF counts roles before
// formatting an output string used only at33AE→0283 on MainShown. Retain the
// numeric algorithm and original local frame;emit structured counts for the UI.
pub(super) fn menu_formation(c: &mut Cpu) -> Result<(), String> {
    if c.watch_trace.is_none() || c.call_chain().last().copied() != Some((0x1B55, 0x33AE)) {
        return Err("formation outside proven presentation caller".into());
    }
    let ss = c.r[10];
    let ds = c.r[11];
    let sp = c.r[4];
    let club = c.read(ss, sp.wrapping_add(4), 16)?;
    let (output, segment) = c.far(ss, sp.wrapping_add(6))?;
    if club == 0 || club > 176 || segment != ss || output != c.r[5].wrapping_sub(0x106) {
        return Err("formation hidden-output frame differs".into());
    }
    c.enter(0x30A)?;
    let bp = c.r[5];
    c.pc = (0x1B55, 0x10B7);
    let (offset, segment) = c.far(ds, 0xD7C2)?;
    let club_offset = offset.wrapping_add((club - 1).wrapping_mul(62));
    c.write(ss, bp.wrapping_sub(10), 16, club_offset)?;
    c.write(ss, bp.wrapping_sub(8), 16, segment)?;
    c.write(ss, bp.wrapping_sub(4), 16, 0)?;
    c.write(ss, bp.wrapping_sub(2), 16, 0)?;
    for slot in 1..=11u16 {
        c.write(ss, bp.wrapping_sub(6), 16, slot)?;
        c.pc = (0x1B55, 0x10E3);
        let player = c.read(segment, club_offset.wrapping_add(2 * (slot - 1)), 16)?;
        c.push(player)?;
        c.invoke(
            super::action_graph::p_1b55_4f89,
            true,
            0x1B55,
            0x4F89,
            0x10EB,
        )?;
        if c.r[0] & 255 == 0 {
            let (offset, segment) = c.far(ds, 0xD7BA)?;
            let role = c.read(
                segment,
                offset.wrapping_add(player.wrapping_mul(8)).wrapping_add(6),
                8,
            )?;
            if role > 13 {
                return Err("unsupported formation role mapping".into());
            }
            c.pc = (0x1B55, 0x110E);
            let category = c.read(ds, 0x0F22 + role, 8)?;
            if category > 3 {
                return Err("unsupported formation role category".into());
            }
            let address = bp.wrapping_sub(4).wrapping_add(category);
            let value = c.read(ss, address, 8)?;
            let value = c.inc(value, 8);
            c.pc = (0x1B55, 0x1116);
            c.write(ss, address, 8, value)?;
        }
    }
    let mut counts = Vec::new();
    for index in 0..4 {
        counts.push(c.read(ss, bp.wrapping_sub(4).wrapping_add(index), 8)?);
    }
    c.presentation.push(
        serde_json::json!({"site":"1B55:10AF","kind":"formation","club":club,"counts":counts}),
    );
    c.r[4] = bp;
    c.r[5] = c.pop()?;
    c.ret(true, 2)
}

// [confirmed:T41/grouped-integer-r0001] Native numeric implementation of
// 1B55:0F95 signed32 decimal grouping. Punctuation follows source0FE4/101E:1059;
// no original string asset or expected helper result is read at runtime.
pub(super) fn grouped_integer(c: &mut Cpu) -> Result<(), String> {
    let ss = c.r[10];
    let sp = c.r[4];
    let low = c.read(ss, sp.wrapping_add(4), 16)?;
    let high = c.read(ss, sp.wrapping_add(6), 16)?;
    let (output, segment) = c.far(ss, sp.wrapping_add(8))?;
    if segment != ss {
        return Err("grouped integer requires an owned stack output buffer".into());
    }
    let number = (u32::from(low) | (u32::from(high) << 16)) as i32;
    let digits = i64::from(number).abs().to_string();
    let mut text = Vec::new();
    if number < 0 {
        text.push(45u8);
    }
    for (index, digit) in digits.bytes().enumerate() {
        if index != 0 && (digits.len() - index) % 3 == 0 {
            text.push(44);
        }
        text.push(digit);
    }
    c.enter(0x11A)?;
    c.pc = (0x26C9, 0x0B85);
    c.write(segment, output, 8, text.len() as u16)?;
    for (index, byte) in text.into_iter().enumerate() {
        c.pc = (0x26C9, 0x0B8A);
        c.write(
            segment,
            output.wrapping_add(1 + index as u16),
            8,
            u16::from(byte),
        )?;
    }
    c.r[4] = c.r[5];
    c.r[5] = c.pop()?;
    c.ret(true, 4)
}

// [confirmed:T41/fixture-title-boundary-r0001] Original142B changes only
// scratch and title output,which D75D passes only to projected MainShown text.
// [derived:T66;static1B55:142B..153F] Fresh week1/stage1 uses the same
// Pascal-string helpers and output-only path;its week<23 excludes1334.
// Admit this exact source case alongside6/0,keeping caller/output ABI guards.
pub(super) fn menu_fixture_title(c: &mut Cpu) -> Result<(), String> {
    let ss = c.r[10];
    let sp = c.r[4];
    let week = c.read(ss, sp.wrapping_add(4), 8)?;
    let stage = c.read(ss, sp.wrapping_add(6), 8)?;
    let (output, segment) = c.far(ss, sp.wrapping_add(8))?;
    if c.watch_trace.is_none()
        || c.call_chain().last().copied() != Some((0x0AB9, 0xD75D))
        || !matches!((week, stage), (6, 0) | (1, 1))
        || segment != ss
        || output != c.r[5].wrapping_sub(0x106)
    {
        return Err("fixture title outside proven presentation caller/arguments".into());
    }
    c.presentation.push(
        serde_json::json!({"site":"1B55:142B","kind":"fixture_title","week":week,"stage":stage}),
    );
    c.ret(true, 4)
}

// [confirmed:T41/menu-input-boundary-r0001] Down/Up/Return normalize to6/5/2;
// original0F8B/0EC4 computes cursor changes and consumes the external key.
// [designed] Reference controller uses the original harness's PLAY GAME intent:
// option3 at sourceD8A2/D8A6. No keys are supplied during watched gameplay ticks.
pub(super) fn continue_menu_key(c: &mut Cpu) -> Result<(), String> {
    let chain = c.call_chain();
    let halftime = chain.ends_with(&[(0x0AB9, 0xD81C), (0x232A, 0x0FA6), (0x232A, 0x09A3)]);
    let players = chain.ends_with(&[(0x0AB9, 0xCE3D), (0x232A, 0x09A3)]);
    let final_screen = chain.ends_with(&[(0x0AB9, 0xE1C4), (0x232A, 0x09A3)])
        && c.watch_tick > 0
        && c.watch_pending.is_none()
        && c.read(c.r[11], 0xD0AE, 16)? == 0;
    if c.watch_trace.is_none()
        || !(halftime || final_screen || (players && c.replacement.is_some()))
    {
        return Err("external menu input outside proven halftime/final controller".into());
    }
    let ss = c.r[10];
    let ds = c.r[11];
    c.pc = (0x24BC, 0x0C1A);
    if c.read(ds, 0xDE8E, 16)? != 0 || c.read(ds, 0xDE8C, 16)? != 0 {
        return Err("external menu input requires empty original key queues".into());
    }
    let replacement_key = if c.replacement.is_some() {
        super::interval_replacement::key(c, halftime, players)?
    } else {
        None
    };
    let (key, current, intent) = if let Some(key) = replacement_key {
        (key, None, "interval_replacement")
    } else if final_screen {
        // [confirmed:T41/final-footer-r0001] Original Week.play presses n;
        // E1C8 uppercases it and E1EA exits the finished match screen.
        (110, None, "finish_match")
    } else {
        // At0C17 entry BP belongs to099B;its saved BP is the0F8B frame.
        let menu_bp = c.read(ss, c.r[5], 16)?;
        let controller_bp = c.read(ss, menu_bp, 16)?;
        let count = c.read(ss, menu_bp.wrapping_add(6), 16)?;
        let (cursor, segment) = c.far(ss, menu_bp.wrapping_add(8))?;
        if count != 4 || segment != ss || cursor != controller_bp.wrapping_sub(5) {
            return Err("external menu input cursor/frame differs".into());
        }
        let current = c.read(segment, cursor, 8)?;
        if !(1..=4).contains(&current) {
            return Err("unsupported external menu cursor".into());
        }
        let key = if current < 3 {
            6
        } else if current > 3 {
            5
        } else {
            2
        };
        (key, Some(current), "play_game")
    };
    c.pc = (0x24BC, 0x0C48);
    c.write(ds, 0xDE88, 16, key)?;
    c.pc = (0x24BC, 0x0C4D);
    c.write(ds, 0xDE8C, 16, 0)?;
    c.pc = (0x24BC, 0x0C53);
    c.write(ds, 0xDE8A, 16, key)?;
    c.xor(c.r[0], c.r[0], 16);
    c.r[0] = key;
    c.presentation.push(serde_json::json!({"site":"24BC:0C17","kind":"external_menu_key","intent":intent,"cursor_before":current,"key":key}));
    c.ret(true, 0)
}

// Native numeric validator;only construction/display of warning text is external.
pub(super) fn validate_menu_lineup(c: &mut Cpu) -> Result<(), String> {
    super::lineup_validation::run(c)
}

// [confirmed:T41/replacement-banner-r0002;staticd9e20a4 ADD0..AEB5]
// Numeric styles are computed by724A;
// the announcement restores Main's saved line. Names/template/delay belong to UI.
pub(super) fn replacement_announcement(c: &mut Cpu) -> Result<(), String> {
    if c.watch_trace.is_none() || c.call_chain().last().copied() != Some((0x0AB9, 0xAF62)) {
        return Err("replacement announcement outside proven caller".into());
    }
    let ss = c.r[10];
    let ds = c.r[11];
    let sp = c.r[4];
    let first = c.read(ss, sp.wrapping_add(2), 16)?;
    let second = c.read(ss, sp.wrapping_add(4), 16)?;
    if !(1..=3920).contains(&first) || !(1..=3920).contains(&second) {
        return Err("unsupported announcement player".into());
    }
    if c.read(ds, 0xDE8C, 16)? != 0 || c.read(ds, 0xDE8E, 16)? != 0 {
        return Err("announcement requires no pending external key".into());
    }
    c.enter(0x1A0)?;
    for (player, ret) in [(second, 0xAE16), (first, 0xAE4E)] {
        let (offset, segment) = c.far(ds, 0xD7BA)?;
        let role = c.read(
            segment,
            offset.wrapping_add(player.wrapping_mul(8)).wrapping_add(6),
            8,
        )?;
        if role > 13 {
            return Err("unsupported announcement role".into());
        }
        c.push(player)?;
        c.push(role)?;
        c.invoke(super::action_graph::p_1b55_724a, true, 0x1B55, 0x724A, ret)?;
        c.pc = (0x232A, 0x043B);
        c.write(ds, 0xDD64, 16, c.r[0] & 255)?;
    }
    c.presentation.push(serde_json::json!({"site":"0AB9:ADD0","kind":"replacement_announcement","first_player":first,"second_player":second,"duration_seconds":2}));
    c.r[4] = c.r[5];
    c.r[5] = c.pop()?;
    c.ret(false, 4)
}

// [derived:archaeology1d6b5b9,C866;232A:0098..00D2] The original helper
// copies a warning string even for a false predicate, then returns that same
// predicate byte. For false only, no dialog is called and all writes are local
// scratch. The caller immediately tests AL. Never admit a true/unproven dialog.
pub(super) fn replacement_guard_message(c: &mut Cpu) -> Result<(), String> {
    c.pc = (0x232A, 0x0098);
    let sp = c.r[4];
    let ss = c.r[10];
    let (text, segment) = c.far(ss, sp.wrapping_add(4))?;
    let predicate = c.read(ss, sp.wrapping_add(8), 8)?;
    let call = c.call_chain().last().copied();
    let local_warning =
        text == c.r[5].wrapping_sub(0x108) && segment == ss && call == Some((0x0AB9, 0xCA58));
    let literal_warning = segment == c.code_base.wrapping_add(0x0AB9)
        && matches!(
            (text, call),
            (0xC7CB, Some((0x0AB9, 0xC899)))
                | (0xC7E3, Some((0x0AB9, 0xC8E3)))
                | (0xC80E, Some((0x0AB9, 0xC92C)))
                | (0xC823, Some((0x0AB9, 0xCA07)))
                | (0xC85D, Some((0x0AB9, 0xCA8E)))
        );
    if c.replacement.is_none()
        || !c.call_chain().contains(&(0x0AB9, 0xD1FC))
        || c.read(c.r[11], 0xD0BA, 8)? != 1
        || predicate != 0
        || !(local_warning || literal_warning)
    {
        return Err(format!("replacement warning outside false witnessed guard: text={text:04X} predicate={predicate} call={call:?}"));
    }
    c.df = false;
    c.r[0] &= 0xFF00;
    c.presentation.push(serde_json::json!({"site":"232A:0098","kind":"false_replacement_guard","text_address":text,"predicate":predicate}));
    c.ret(true, 6)
}

// [derived:1d6b5b9,CA2B..CA58;1B55:44DC;26C9:0BD2] With the already
// computed predicate zero, the local name/suffix string feeds only the false
// warning above. Retain original stack cleanup; no string output is consumed.
// Refuse every other caller, buffer or predicate instead of inventing text.
pub(super) fn replacement_warning_name(c: &mut Cpu) -> Result<(), String> {
    let ss = c.r[10];
    let sp = c.r[4];
    let call = c.call_chain().last().copied();
    let player = c.read(ss, sp.wrapping_add(4), 16)?;
    let out = c.far(ss, sp.wrapping_add(6))?;
    if c.replacement.is_none()
        || call != Some((0x0AB9, 0xCA49))
        || !c.call_chain().contains(&(0x0AB9, 0xD1FC))
        || out != (c.r[5].wrapping_sub(0x108), ss)
        || player != 2592
        || c.read(ss, sp.wrapping_add(10), 8)? != 0
    {
        return Err("replacement name outside false witnessed warning".into());
    }
    c.presentation
        .push(serde_json::json!({"site":"1B55:44DC","kind":"false_warning_name","player":player}));
    c.ret(true, 2)
}
pub(super) fn replacement_warning_suffix(c: &mut Cpu) -> Result<(), String> {
    let ss = c.r[10];
    let sp = c.r[4];
    let call = c.call_chain().last().copied();
    if c.replacement.is_none()
        || call != Some((0x0AB9, 0xCA53))
        || !c.call_chain().contains(&(0x0AB9, 0xD1FC))
        || c.far(ss, sp.wrapping_add(4))? != (0xC850, c.code_base.wrapping_add(0x0AB9))
        || c.far(ss, sp.wrapping_add(8))? != (c.r[5].wrapping_sub(0x108), ss)
        || c.read(ss, sp.wrapping_add(12), 8)? != 0
    {
        return Err("replacement suffix outside false witnessed warning".into());
    }
    c.df = false;
    c.ret(true, 4)
}

#[cfg(test)]
mod replacement_tests {
    use super::*;
    fn warning_parent(c: &mut Cpu) -> Result<(), String> {
        c.push(c.read(0, 0x1000, 8)?)?;
        c.push(c.r[9])?;
        c.push(c.read(0, 0x1002, 16)?)?;
        let ret = c.read(0, 0x1004, 16)?;
        c.invoke(
            super::super::action_graph::p_232a_0098,
            true,
            0x232A,
            0x0098,
            ret,
        )?;
        c.ret(false, 0)
    }
    #[test]
    fn false_warning_projects_only_the_exact_non_dialog_source_case() {
        for (predicate, text, ret, root, success) in [
            (0, 0xC7CB, 0xC899, 0xD1FC, true),
            (1, 0xC7CB, 0xC899, 0xD1FC, false),
            (0, 0xC7E3, 0xC899, 0xD1FC, false),
            (0, 0xC7CB, 0xC8E3, 0xD1FC, false),
            (0, 0xC7CB, 0xC899, 0xD1FD, false),
        ] {
            let mut c = Cpu::new(crate::role_scores::image(Default::default()));
            c.r[9] = 0x0AB9;
            c.r[11] = 0;
            c.replacement = Some(super::super::interval_replacement::Intent::new(false));
            for (at, width, value) in [
                (0xD0BA, 8, 1),
                (0x1000, 8, predicate),
                (0x1002, 16, text),
                (0x1004, 16, ret),
            ] {
                c.write(0, at, width, value).unwrap();
            }
            let image = c.e.image.clone();
            let sp = c.r[4];
            let result = c.invoke(warning_parent, false, 0x0AB9, 0xD1DD, root);
            assert_eq!(result.is_ok(), success, "{result:?}");
            assert_eq!(
                c.e.image, image,
                "warning projection modified gameplay memory"
            );
            if success {
                assert_eq!(c.r[0] & 255, 0);
                assert_eq!(c.r[4], sp);
                assert_eq!(c.r[9], 0x0AB9);
            }
        }
    }
}
