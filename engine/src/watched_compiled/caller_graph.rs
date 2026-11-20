//! [confirmed] Mechanically compiled numeric control flow;F39/c7637bd,static d9e20a4.
//! [designed] Direct Rust calls/blocks;no runtime decoder or original executable.
use super::action_graph::*;
use super::compiled::Cpu;
use super::tick_graph::*;
pub(super) fn p_0ab9_00d6(c: &mut Cpu) -> Result<(), String> {
    super::projections::clock_display(c)
}
pub(super) fn p_0ab9_760c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 30220u16;
    loop {
        match label {
            30220 => {
                c.pc = (2745, 30220);
                c.step()?;
                c.enter(12)?;
                c.pc = (2745, 30224);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 30228);
                c.step()?;
                label = 30233;
                continue;
            }
            30230 => {
                c.pc = (2745, 30230);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 30233;
                continue;
            }
            30233 => {
                c.pc = (2745, 30233);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30236);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30237);
                c.step()?;
                let x = c.r[0];
                let y = 7u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 30240);
                c.step()?;
                let x = c.r[7];
                let y = 53038u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 30244);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 30245);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 30246);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 30249);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 30252);
                c.step()?;
                let v = 0u16;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8, v)?;
                c.pc = (2745, 30257);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30260);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30261);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 30263);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30267);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 30269);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 30270);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 30273);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 30277);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 30278);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 30279);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 30280);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 30283)?;
                label = 30283;
                continue;
            }
            30283 => {
                c.pc = (2745, 30283);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30284);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 30287);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30290);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30291);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 30293);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30297);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 30299);
                c.step()?;
                let x = c.r[0];
                let y = 11u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 30302);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(12u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 30305);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30308);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30309);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 30311);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30315);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 30317);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 30318);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 30319);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(12u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 30322);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 30385 } else { 30324 };
                continue;
            }
            30324 => {
                c.pc = (2745, 30324);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 30327);
                c.step()?;
                label = 30332;
                continue;
            }
            30329 => {
                c.pc = (2745, 30329);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 30332;
                continue;
            }
            30332 => {
                c.pc = (2745, 30332);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 30335);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30339);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30340);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 30342);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 30346);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(13072u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30350);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30351);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.multiply(v, false);
                c.pc = (2745, 30354);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 30356);
                c.step()?;
                label = if c.sf == c.of { 30377 } else { 30358 };
                continue;
            }
            30358 => {
                c.pc = (2745, 30358);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 30362);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(13072u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30366);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30367);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.multiply(v, false);
                c.pc = (2745, 30370);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 30373);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8, v)?;
                label = 30377;
                continue;
            }
            30377 => {
                c.pc = (2745, 30377);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 30380);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(12u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 30383);
                c.step()?;
                label = if !c.z { 30329 } else { 30385 };
                continue;
            }
            30385 => {
                c.pc = (2745, 30385);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 30388);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30392);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30393);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.multiply(v, false);
                c.pc = (2745, 30396);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8, v)?;
                c.pc = (2745, 30400);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 30401);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 30402);
                c.step()?;
                c.invoke(p_1998_087b, true, 6552, 2171, 30407)?;
                label = 30407;
                continue;
            }
            30407 => {
                c.pc = (2745, 30407);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 30410);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 30415);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30417);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 30420 } else { 30419 };
                continue;
            }
            30419 => {
                c.pc = (2745, 30419);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 30420;
                continue;
            }
            30420 => {
                c.pc = (2745, 30420);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.r[2] = (c.r[2] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30422);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53063u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 30427);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30429);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 30432 } else { 30431 };
                continue;
            }
            30431 => {
                c.pc = (2745, 30431);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 30432;
                continue;
            }
            30432 => {
                c.pc = (2745, 30432);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[2] >> 0) & 255;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 30434);
                c.step()?;
                label = if !c.z { 30467 } else { 30436 };
                continue;
            }
            30436 => {
                c.pc = (2745, 30436);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53436u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 30441);
                c.step()?;
                label = if c.z { 30467 } else { 30443 };
                continue;
            }
            30443 => {
                c.pc = (2745, 30443);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 30448);
                c.step()?;
                label = if c.z { 30467 } else { 30450 };
                continue;
            }
            30450 => {
                c.pc = (2745, 30450);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53082u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 30454);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 16)?;
                c.push(v)?;
                c.pc = (2745, 30457);
                c.step()?;
                let v = 53078u16;
                c.r[7] = v;
                c.pc = (2745, 30460);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 30461);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 30462);
                c.step()?;
                c.invoke(p_1998_09a3, true, 6552, 2467, 30467)?;
                label = 30467;
                continue;
            }
            30467 => {
                c.pc = (2745, 30467);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 30471);
                c.step()?;
                label = if c.z { 30476 } else { 30473 };
                continue;
            }
            30473 => {
                c.pc = (2745, 30473);
                c.step()?;
                label = 30230;
                continue;
            }
            30476 => {
                c.pc = (2745, 30476);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 30477);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7b7b(c: &mut Cpu) -> Result<(), String> {
    let mut label = 31611u16;
    loop {
        match label {
            31611 => {
                c.pc = (2745, 31611);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 31615);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 31619);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53432u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31624);
                c.step()?;
                label = if !c.z { 31629 } else { 31626 };
                continue;
            }
            31626 => {
                c.pc = (2745, 31626);
                c.step()?;
                label = 31800;
                continue;
            }
            31629 => {
                c.pc = (2745, 31629);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53424u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31634);
                c.step()?;
                label = if c.cf || c.z { 31643 } else { 31636 };
                continue;
            }
            31636 => {
                c.pc = (2745, 31636);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53424u16), 16)?;
                let v = c.dec(x, 16);
                c.write(c.r[11], 0u16.wrapping_add(53424u16), 16, v)?;
                c.pc = (2745, 31640);
                c.step()?;
                label = 31800;
                continue;
            }
            31643 => {
                c.pc = (2745, 31643);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53422u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 31646);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[11], 0u16.wrapping_add(53428u16), 16)?;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 31650);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 31652);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31654);
                c.step()?;
                label = if !c.z { 31657 } else { 31656 };
                continue;
            }
            31656 => {
                c.pc = (2745, 31656);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 31657;
                continue;
            }
            31657 => {
                c.pc = (2745, 31657);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53433u16), 8, v)?;
                c.pc = (2745, 31660);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31665);
                c.step()?;
                label = if !c.z { 31696 } else { 31667 };
                continue;
            }
            31667 => {
                c.pc = (2745, 31667);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53422u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31672);
                c.step()?;
                label = if c.cf || c.z { 31680 } else { 31674 };
                continue;
            }
            31674 => {
                c.pc = (2745, 31674);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53422u16), 16)?;
                let v = c.dec(x, 16);
                c.write(c.r[11], 0u16.wrapping_add(53422u16), 16, v)?;
                c.pc = (2745, 31678);
                c.step()?;
                label = 31684;
                continue;
            }
            31680 => {
                c.pc = (2745, 31680);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53428u16), 16)?;
                let v = c.dec(x, 16);
                c.write(c.r[11], 0u16.wrapping_add(53428u16), 16, v)?;
                label = 31684;
                continue;
            }
            31684 => {
                c.pc = (2745, 31684);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53426u16), 16)?;
                let y = 128u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31690);
                c.step()?;
                label = if !c.cf { 31696 } else { 31692 };
                continue;
            }
            31692 => {
                c.pc = (2745, 31692);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53426u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[11], 0u16.wrapping_add(53426u16), 16, v)?;
                label = 31696;
                continue;
            }
            31696 => {
                c.pc = (2745, 31696);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53422u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 31700);
                c.step()?;
                let v = 3u16;
                c.push(v)?;
                c.pc = (2745, 31702);
                c.step()?;
                c.invoke(p_0ab9_00d6, false, 2745, 214, 31705)?;
                label = 31705;
                continue;
            }
            31705 => {
                c.pc = (2745, 31705);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(3368u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31708);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 31710);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53424u16), 16, v)?;
                c.pc = (2745, 31713);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53420u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31716);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 31717);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 31718);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 31720);
                c.step()?;
                let v = c.r[2];
                c.r[3] = v;
                c.pc = (2745, 31722);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53422u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 31725);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 31727);
                c.step()?;
                let x = c.r[2];
                let y = c.r[3];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31729);
                c.step()?;
                label = if !c.z { 31735 } else { 31731 };
                continue;
            }
            31731 => {
                c.pc = (2745, 31731);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31733);
                c.step()?;
                label = if c.z { 31742 } else { 31735 };
                continue;
            }
            31735 => {
                c.pc = (2745, 31735);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31740);
                c.step()?;
                label = if c.z { 31800 } else { 31742 };
                continue;
            }
            31742 => {
                c.pc = (2745, 31742);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31747);
                c.step()?;
                label = if c.z { 31756 } else { 31749 };
                continue;
            }
            31749 => {
                c.pc = (2745, 31749);
                c.step()?;
                let v = 63u16;
                c.push(v)?;
                c.pc = (2745, 31751);
                c.step()?;
                c.invoke(p_0ab9_1e14, false, 2745, 7700, 31754)?;
                label = 31754;
                continue;
            }
            31754 => {
                c.pc = (2745, 31754);
                c.step()?;
                label = 31761;
                continue;
            }
            31756 => {
                c.pc = (2745, 31756);
                c.step()?;
                let v = 64u16;
                c.push(v)?;
                c.pc = (2745, 31758);
                c.step()?;
                c.invoke(p_0ab9_1e14, false, 2745, 7700, 31761)?;
                label = 31761;
                continue;
            }
            31761 => {
                c.pc = (2745, 31761);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 31763);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53073u16), 16, v)?;
                c.pc = (2745, 31766);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 31770);
                c.step()?;
                let v = 3u16;
                c.push(v)?;
                c.pc = (2745, 31772);
                c.step()?;
                c.invoke(p_1998_01e0, true, 6552, 480, 31777)?;
                label = 31777;
                continue;
            }
            31777 => {
                c.pc = (2745, 31777);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 31781);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31785);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 31787);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 31788);
                c.step()?;
                c.invoke(p_1998_00d6, true, 6552, 214, 31793)?;
                label = 31793;
                continue;
            }
            31793 => {
                c.pc = (2745, 31793);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 31795);
                c.step()?;
                c.invoke(p_1998_01e0, true, 6552, 480, 31800)?;
                label = 31800;
                continue;
            }
            31800 => {
                c.pc = (2745, 31800);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31803);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 31804);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
