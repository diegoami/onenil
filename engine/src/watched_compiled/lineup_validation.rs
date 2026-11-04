//! [confirmed] Original0AB9:D29E numeric control flow,staticd9e20a4;
//! nine original positive/negative cases in T41/lineup-validation-r0002.
//! Warning0098 returns its boolean argument unchanged. Text construction and
//! modal drawing are presentation;availability/keeper/role checks stay native.
use super::{action_graph, compiled::Cpu};
use serde_json::json;
fn put(c: &mut Cpu, bp: u16, relative: i16, width: u8, value: u16, pc: u16) -> Result<(), String> {
    c.pc = (0x0AB9, pc);
    c.write(c.r[10], bp.wrapping_add(relative as u16), width, value)
}
fn managed(c: &mut Cpu, club: u16, ret: u16) -> Result<bool, String> {
    c.push(club)?;
    c.invoke(action_graph::p_1b55_0e7a, true, 0x1B55, 0x0E7A, ret)?;
    Ok(c.r[0] & 255 != 0)
}
fn unavailable(c: &mut Cpu, player: u16) -> Result<bool, String> {
    c.push(player)?;
    c.invoke(action_graph::p_1b55_4f89, true, 0x1B55, 0x4F89, 0xD378)?;
    Ok(c.r[0] & 255 != 0)
}
pub(super) fn run(c: &mut Cpu) -> Result<(), String> {
    if c.watch_trace.is_none() || c.call_chain().last().copied() != Some((0x0AB9, 0xD8D6)) {
        return Err("lineup validation outside proven menu caller".into());
    }
    let ss = c.r[10];
    let ds = c.r[11];
    let parent = c.read(ss, c.r[4].wrapping_add(2), 16)?;
    if parent != c.r[5] {
        return Err("lineup validation parent frame differs".into());
    }
    c.pc = (0x0AB9, 0xD2A5);
    let saved_club = c.read(ss, parent.wrapping_sub(4), 8)?;
    c.enter(0x218)?;
    let bp = c.r[5];
    put(c, bp, -0x16, 16, saved_club, 0xD2AB)?;
    let clubs = [c.read(ds, 0xD6E4, 16)?, c.read(ds, 0xD6E6, 16)?];
    let mut valid = [false; 2];
    for (side, club) in clubs.into_iter().enumerate() {
        if !(1..=176).contains(&club) {
            return Err("unsupported lineup-validation club".into());
        }
        valid[side] = !managed(c, club, 0xD2C8)?;
        put(
            c,
            bp,
            -0x18 + side as i16,
            8,
            u16::from(valid[side]),
            0xD2D7,
        )?;
    }
    for (side, club) in clubs.into_iter().enumerate() {
        put(c, bp, -0x13, 8, side as u16, 0xD2E6)?;
        if !managed(c, club, 0xD2FA)? {
            continue;
        }
        c.pc = (0x0AB9, 0xD310);
        c.write(ss, parent.wrapping_sub(4), 8, club)?;
        let mut counts = [0u8; 15];
        for index in 0..15 {
            put(c, bp, -0x10 + index, 8, 0, 0xD31D)?;
        }
        valid[side] = true;
        let (club_offset, club_segment) = c.far(ds, 0xD7C2)?;
        let club_offset = club_offset.wrapping_add((club - 1).wrapping_mul(62));
        let in_match = c.read(ds, 0xD18, 8)? != 0;
        for slot in 0..11u16 {
            put(c, bp, -0x12, 16, slot + 1, 0xD333)?;
            c.pc = (0x0AB9, 0xD359);
            let player = c.read(club_segment, club_offset.wrapping_add(2 * slot), 16)?;
            let unavailable = if player == 0 {
                true
            } else {
                unavailable(c, player)?
            };
            let index = if player == 0 {
                Some(14)
            } else if !unavailable {
                let (offset, segment) = c.far(ds, 0xD7BA)?;
                c.pc = (0x0AB9, 0xD393);
                let role = c.read(
                    segment,
                    offset.wrapping_add(player.wrapping_mul(8)).wrapping_add(6),
                    8,
                )?;
                if role > 13 {
                    return Err("unsupported lineup-validation role".into());
                }
                Some(usize::from(role))
            } else {
                None
            };
            if let Some(index) = index {
                counts[index] = counts[index].wrapping_add(1);
                put(
                    c,
                    bp,
                    -0x10 + index as i16,
                    8,
                    u16::from(counts[index]),
                    if index == 14 { 0xD360 } else { 0xD39B },
                )?;
            }
            if !in_match && unavailable {
                valid[side] = false;
            }
        }
        let mut warning = None;
        if !valid[side] {
            warning = Some(if counts[14] != 0 {
                "missing_players"
            } else {
                "unavailable_players"
            });
        } else if counts[0] > 1 {
            valid[side] = false;
            warning = Some("multiple_keepers");
        } else if counts[0] == 0 {
            valid[side] = false;
            warning = Some("missing_keeper");
        } else {
            for role in 1..=13 {
                if counts[role] > 3 {
                    valid[side] = false;
                    warning = Some("overfull_role");
                    break;
                }
            }
        }
        put(
            c,
            bp,
            -0x18 + side as i16,
            8,
            u16::from(valid[side]),
            0xD594,
        )?;
        c.presentation.push(json!({"site":"0AB9:D29E","kind":"lineup_validation","side":side,"club":club,"counts":counts.to_vec(),"valid":valid[side],"warning":warning}));
    }
    let result = u16::from(valid[0] && valid[1]);
    put(c, bp, -1, 8, result, 0xD5B8)?;
    c.pc = (0x0AB9, 0xD5C1);
    c.write(ss, parent.wrapping_sub(4), 8, saved_club)?;
    c.r[0] = (c.r[0] & 0xFF00) | result;
    c.r[4] = bp;
    c.r[5] = c.pop()?;
    c.ret(false, 2)
}
