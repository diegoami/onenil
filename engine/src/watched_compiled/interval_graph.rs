//! [confirmed] Mechanically compiled numeric control flow;F39/c7637bd,static d9e20a4.
//! [designed] Direct Rust calls/blocks;no runtime decoder or original executable.
use super::action_graph::*;
use super::compiled::Cpu;
use super::match_graph::*;
pub(super) fn p_0ab9_7e4a(c: &mut Cpu) -> Result<(), String> {
    let mut label = 32330u16;
    loop {
        match label {
            32330 => {
                c.pc = (2745, 32330);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 32332);
                c.step()?;
                c.invoke(p_0ab9_d5d6, false, 2745, 54742, 32335)?;
                label = 32335;
                continue;
            }
            32335 => {
                c.pc = (2745, 32335);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53421u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32340);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32342);
                c.step()?;
                label = if !c.z { 32345 } else { 32344 };
                continue;
            }
            32344 => {
                c.pc = (2745, 32344);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 32345;
                continue;
            }
            32345 => {
                c.pc = (2745, 32345);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32346);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 32348);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32352);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 32354);
                c.step()?;
                let x = c.r[0];
                let y = 10u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32357);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53407u16), 8, v)?;
                c.pc = (2745, 32360);
                c.step()?;
                let v = 80u16;
                c.write(c.r[11], 0u16.wrapping_add(53438u16), 16, v)?;
                c.pc = (2745, 32366);
                c.step()?;
                c.invoke(p_0ab9_7c3f, false, 2745, 31807, 32369)?;
                label = 32369;
                continue;
            }
            32369 => {
                c.pc = (2745, 32369);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 32370);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_e064(c: &mut Cpu) -> Result<(), String> {
    let mut label = 57444u16;
    loop {
        match label {
            57444 => {
                c.pc = (2745, 57444);
                c.step()?;
                c.invoke(p_232a_022e, true, 9002, 558, 57449)?;
                label = 57449;
                continue;
            }
            57449 => {
                c.pc = (2745, 57449);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57452);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 0) & 255;
                c.r[0] = (c.r[0] & 255) | ((y & 255) << 8);
                c.r[0] = (c.r[0] & 65280) | ((x & 255) << 0);
                c.pc = (2745, 57454);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53382u16), 16, v)?;
                c.pc = (2745, 57457);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53382u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57461);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 57464);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(4u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 57468);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 16)?;
                let y = 58u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 57473);
                c.step()?;
                let o = 0u16.wrapping_add(55246u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57477);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57479);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57481);
                c.step()?;
                let x = c.r[7];
                let y = 65477u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57485);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 57486);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57487);
                c.step()?;
                c.invoke(p_232a_01c5, true, 9002, 453, 57492)?;
                label = 57492;
                continue;
            }
            57492 => {
                c.pc = (2745, 57492);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(3370u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57497);
                c.step()?;
                label = if !c.z { 57502 } else { 57499 };
                continue;
            }
            57499 => {
                c.pc = (2745, 57499);
                c.step()?;
                label = 57744;
                continue;
            }
            57502 => {
                c.pc = (2745, 57502);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 57505);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57509);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(4u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57513);
                c.step()?;
                c.invoke(p_0ab9_d904, false, 2745, 55556, 57516)?;
                label = 57516;
                continue;
            }
            57516 => {
                c.pc = (2745, 57516);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57518);
                c.step()?;
                label = if !c.z { 57523 } else { 57520 };
                continue;
            }
            57520 => {
                c.pc = (2745, 57520);
                c.step()?;
                label = 57744;
                continue;
            }
            57523 => {
                c.pc = (2745, 57523);
                c.step()?;
                c.invoke(p_0ab9_7e73, false, 2745, 32371, 57526)?;
                label = 57526;
                continue;
            }
            57526 => {
                c.pc = (2745, 57526);
                c.step()?;
                c.invoke(p_0ab9_7ebe, false, 2745, 32446, 57529)?;
                label = 57529;
                continue;
            }
            57529 => {
                c.pc = (2745, 57529);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57532);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 0) & 255;
                c.r[0] = (c.r[0] & 255) | ((y & 255) << 8);
                c.r[0] = (c.r[0] & 65280) | ((x & 255) << 0);
                c.pc = (2745, 57534);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53382u16), 16, v)?;
                c.pc = (2745, 57537);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53382u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57541);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 57544);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(4u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 57548);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 16)?;
                let y = 58u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 57553);
                c.step()?;
                let o = 0u16.wrapping_add(55246u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57557);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57559);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57561);
                c.step()?;
                let x = c.r[7];
                let y = 65477u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57565);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 57566);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57567);
                c.step()?;
                c.invoke(p_232a_01c5, true, 9002, 453, 57572)?;
                label = 57572;
                continue;
            }
            57572 => {
                c.pc = (2745, 57572);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53430u16), 8, v)?;
                c.pc = (2745, 57577);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 57580);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57584);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(4u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57588);
                c.step()?;
                c.invoke(p_0ab9_d904, false, 2745, 55556, 57591)?;
                label = 57591;
                continue;
            }
            57591 => {
                c.pc = (2745, 57591);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57593);
                c.step()?;
                label = if !c.z { 57598 } else { 57595 };
                continue;
            }
            57595 => {
                c.pc = (2745, 57595);
                c.step()?;
                label = 57739;
                continue;
            }
            57598 => {
                c.pc = (2745, 57598);
                c.step()?;
                c.invoke(p_0ab9_8434, false, 2745, 33844, 57601)?;
                label = 57601;
                continue;
            }
            57601 => {
                c.pc = (2745, 57601);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53445u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57604);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[11], 0u16.wrapping_add(53444u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57608);
                c.step()?;
                label = if c.cf || c.z { 57660 } else { 57610 };
                continue;
            }
            57610 => {
                c.pc = (2745, 57610);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 57613);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57617);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (2745, 57619);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57621);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57623);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 57625);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 16)?;
                let y = 174u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 57631);
                c.step()?;
                let o = 0u16.wrapping_add(55242u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57635);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57637);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57639);
                c.step()?;
                let x = c.r[7];
                let y = 65359u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57643);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57647);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 15u16;
                let v = c.and(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57649);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 57651);
                c.step()?;
                let x = c.r[0];
                let y = 128u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57654);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8, v)?;
                c.pc = (2745, 57658);
                c.step()?;
                label = 57739;
                continue;
            }
            57660 => {
                c.pc = (2745, 57660);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 57663);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57667);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (2745, 57669);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57671);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57673);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 57675);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57679);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (2745, 57681);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57683);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57685);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 57687);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 16)?;
                let y = 174u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 57693);
                c.step()?;
                let o = 0u16.wrapping_add(55242u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57697);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57699);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57701);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(176u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57706);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 57708);
                c.step()?;
                let x = c.r[0];
                let y = 174u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 57712);
                c.step()?;
                let o = 0u16.wrapping_add(55242u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57716);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57718);
                c.step()?;
                let x = c.r[7];
                let y = c.r[1];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57720);
                c.step()?;
                let x = c.r[7];
                let y = 65359u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57724);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57728);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 15u16;
                let v = c.and(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57730);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 57732);
                c.step()?;
                let x = c.r[0];
                let y = 128u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57735);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8, v)?;
                label = 57739;
                continue;
            }
            57739 => {
                c.pc = (2745, 57739);
                c.step()?;
                c.invoke(p_232a_022e, true, 9002, 558, 57744)?;
                label = 57744;
                continue;
            }
            57744 => {
                c.pc = (2745, 57744);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57749);
                c.step()?;
                label = if c.z { 57753 } else { 57751 };
                continue;
            }
            57751 => {
                c.pc = (2745, 57751);
                c.step()?;
                label = 57838;
                continue;
            }
            57753 => {
                c.pc = (2745, 57753);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53445u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57756);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[11], 0u16.wrapping_add(53444u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57760);
                c.step()?;
                label = if c.z { 57765 } else { 57762 };
                continue;
            }
            57762 => {
                c.pc = (2745, 57762);
                c.step()?;
                c.invoke(p_0ab9_7cd4, false, 2745, 31956, 57765)?;
                label = 57765;
                continue;
            }
            57765 => {
                c.pc = (2745, 57765);
                c.step()?;
                c.invoke(p_0ab9_7d53, false, 2745, 32083, 57768)?;
                label = 57768;
                continue;
            }
            57768 => {
                c.pc = (2745, 57768);
                c.step()?;
                c.invoke(p_0ab9_b42e, false, 2745, 46126, 57771)?;
                label = 57771;
                continue;
            }
            57771 => {
                c.pc = (2745, 57771);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(56662u16), 8, v)?;
                label = 57776;
                continue;
            }
            57776 => {
                c.pc = (2745, 57776);
                c.step()?;
                let v = 19218u16;
                c.r[7] = v;
                c.pc = (2745, 57779);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 57780);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57781);
                c.step()?;
                let v = 56664u16;
                c.r[7] = v;
                c.pc = (2745, 57784);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 57785);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57786);
                c.step()?;
                c.invoke(p_232a_052f, true, 9002, 1327, 57791)?;
                label = 57791;
                continue;
            }
            57791 => {
                c.pc = (2745, 57791);
                c.step()?;
                c.invoke(p_232a_099b, true, 9002, 2459, 57796)?;
                label = 57796;
                continue;
            }
            57796 => {
                c.pc = (2745, 57796);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57799);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 57800);
                c.step()?;
                c.invoke(p_26c9_1986, true, 9929, 6534, 57805)?;
                label = 57805;
                continue;
            }
            57805 => {
                c.pc = (2745, 57805);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 80u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57807);
                c.step()?;
                label = if !c.z { 57814 } else { 57809 };
                continue;
            }
            57809 => {
                c.pc = (2745, 57809);
                c.step()?;
                c.invoke(p_0ab9_0577, false, 2745, 1399, 57812)?;
                label = 57812;
                continue;
            }
            57812 => {
                c.pc = (2745, 57812);
                c.step()?;
                label = 57836;
                continue;
            }
            57814 => {
                c.pc = (2745, 57814);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 32u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57816);
                c.step()?;
                label = if !c.z { 57823 } else { 57818 };
                continue;
            }
            57818 => {
                c.pc = (2745, 57818);
                c.step()?;
                c.invoke(p_0ab9_0b53, false, 2745, 2899, 57821)?;
                label = 57821;
                continue;
            }
            57821 => {
                c.pc = (2745, 57821);
                c.step()?;
                label = 57836;
                continue;
            }
            57823 => {
                c.pc = (2745, 57823);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 2u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57825);
                c.step()?;
                label = if !c.z { 57834 } else { 57827 };
                continue;
            }
            57827 => {
                c.pc = (2745, 57827);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 57829);
                c.step()?;
                c.invoke(p_0ab9_d5d6, false, 2745, 54742, 57832)?;
                label = 57832;
                continue;
            }
            57832 => {
                c.pc = (2745, 57832);
                c.step()?;
                label = 57836;
                continue;
            }
            57834 => {
                c.pc = (2745, 57834);
                c.step()?;
                label = 57838;
                continue;
            }
            57836 => {
                c.pc = (2745, 57836);
                c.step()?;
                label = 57776;
                continue;
            }
            57838 => {
                c.pc = (2745, 57838);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 57839);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
