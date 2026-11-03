//! [confirmed] Mechanically compiled numeric control flow;F39/c7637bd,static d9e20a4.
//! [designed] Direct Rust calls/blocks;no runtime decoder or original executable.
use super::compiled::Cpu;
pub(super) fn p_0ab9_1a4c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 6732u16;
    loop {
        match label {
            6732 => {
                c.pc = (2745, 6732);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 6736);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                let y = 4u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 6740);
                c.step()?;
                label = if !c.z { 6755 } else { 6742 };
                continue;
            }
            6742 => {
                c.pc = (2745, 6742);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(55005u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 6747);
                c.step()?;
                label = if !c.z { 6755 } else { 6749 };
                continue;
            }
            6749 => {
                c.pc = (2745, 6749);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 6753);
                c.step()?;
                label = 6875;
                continue;
            }
            6755 => {
                c.pc = (2745, 6755);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                let y = 5u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 6759);
                c.step()?;
                label = if !c.cf { 6832 } else { 6761 };
                continue;
            }
            6761 => {
                c.pc = (2745, 6761);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 6764);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 6767);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 6768);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6771);
                c.step()?;
                let v = (c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)? as i16
                    as i32)
                    .wrapping_mul(15u16 as i16 as i32) as u16;
                c.r[0] = v;
                c.pc = (2745, 6776);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6780);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 6782);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6786);
                c.step()?;
                let x = ((c.r[0] >> 8) & 255);
                let y = ((c.r[0] >> 8) & 255);
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 6788);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 6789);
                c.step()?;
                c.invoke(p_24bc_0259, true, 9404, 601, 6794)?;
                label = 6794;
                continue;
            }
            6794 => {
                c.pc = (2745, 6794);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6796);
                c.step()?;
                label = if c.z { 6832 } else { 6798 };
                continue;
            }
            6798 => {
                c.pc = (2745, 6798);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 6801);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6804);
                c.step()?;
                let v = (c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)? as i16
                    as i32)
                    .wrapping_mul(15u16 as i16 as i32) as u16;
                c.r[0] = v;
                c.pc = (2745, 6809);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6813);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 6815);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6819);
                c.step()?;
                let x = ((c.r[0] >> 8) & 255);
                let y = ((c.r[0] >> 8) & 255);
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 6821);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 6822);
                c.step()?;
                c.invoke(p_24bc_0259, true, 9404, 601, 6827)?;
                label = 6827;
                continue;
            }
            6827 => {
                c.pc = (2745, 6827);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 6830);
                c.step()?;
                label = 6875;
                continue;
            }
            6832 => {
                c.pc = (2745, 6832);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 6835);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6838);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 6839);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 6840);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 6841);
                c.step()?;
                c.invoke(p_0ab9_1cbb, false, 2745, 7355, 6844)?;
                label = 6844;
                continue;
            }
            6844 => {
                c.pc = (2745, 6844);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 6845);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 6847);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 6849);
                c.step()?;
                let v = (c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )? as i16 as i32)
                    .wrapping_mul(62u16 as i16 as i32) as u16;
                c.r[0] = v;
                c.pc = (2745, 6854);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6858);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 6860);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6864);
                c.step()?;
                let x = ((c.r[0] >> 8) & 255);
                let y = ((c.r[0] >> 8) & 255);
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 6866);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 6867);
                c.step()?;
                c.invoke(p_24bc_0259, true, 9404, 601, 6872)?;
                label = 6872;
                continue;
            }
            6872 => {
                c.pc = (2745, 6872);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 6875;
                continue;
            }
            6875 => {
                c.pc = (2745, 6875);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6878);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 6879);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_1cbb(c: &mut Cpu) -> Result<(), String> {
    let mut label = 7355u16;
    loop {
        match label {
            7355 => {
                c.pc = (2745, 7355);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 7359);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7362);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                let y = 12u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 7367);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7369);
                c.step()?;
                label = if !c.cf { 7372 } else { 7371 };
                continue;
            }
            7371 => {
                c.pc = (2745, 7371);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 7372;
                continue;
            }
            7372 => {
                c.pc = (2745, 7372);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 7375);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7378);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 7379);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_1cd6(c: &mut Cpu) -> Result<(), String> {
    let mut label = 7382u16;
    loop {
        match label {
            7382 => {
                c.pc = (2745, 7382);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 7386);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                let y = 12u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 7391);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7393);
                c.step()?;
                label = if !c.cf { 7396 } else { 7395 };
                continue;
            }
            7395 => {
                c.pc = (2745, 7395);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 7396;
                continue;
            }
            7396 => {
                c.pc = (2745, 7396);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 7399);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7402);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 7403);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_1e4e(c: &mut Cpu) -> Result<(), String> {
    let mut label = 7758u16;
    loop {
        match label {
            7758 => {
                c.pc = (2745, 7758);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 7762);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 7765)?;
                label = 7765;
                continue;
            }
            7765 => {
                c.pc = (2745, 7765);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 7766);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7769);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 7770);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 7771);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 7772);
                c.step()?;
                c.invoke(p_0ab9_1cbb, false, 2745, 7355, 7775)?;
                label = 7775;
                continue;
            }
            7775 => {
                c.pc = (2745, 7775);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 7776);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[2] >> 0) & 255);
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 7778);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7780);
                c.step()?;
                label = if !c.z { 7783 } else { 7782 };
                continue;
            }
            7782 => {
                c.pc = (2745, 7782);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 7783;
                continue;
            }
            7783 => {
                c.pc = (2745, 7783);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 7786);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7789);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 7790);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_1e9f(c: &mut Cpu) -> Result<(), String> {
    let mut label = 7839u16;
    loop {
        match label {
            7839 => {
                c.pc = (2745, 7839);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 7843);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7846);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                let y = 11u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 7851);
                c.step()?;
                label = if c.cf || c.z { 7867 } else { 7853 };
                continue;
            }
            7853 => {
                c.pc = (2745, 7853);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7857);
                c.step()?;
                let x = ((c.r[0] >> 8) & 255);
                let y = ((c.r[0] >> 8) & 255);
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 7859);
                c.step()?;
                let x = c.r[0];
                let y = 11u16;
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 7862);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 7865);
                c.step()?;
                label = 7877;
                continue;
            }
            7867 => {
                c.pc = (2745, 7867);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7870);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7874);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 7877;
                continue;
            }
            7877 => {
                c.pc = (2745, 7877);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7880);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 7881);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_1ecc(c: &mut Cpu) -> Result<(), String> {
    let mut label = 7884u16;
    loop {
        match label {
            7884 => {
                c.pc = (2745, 7884);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 7888);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53420u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7891);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 7892);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 7893);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 7895);
                c.step()?;
                let v = c.r[2];
                c.r[3] = v;
                c.pc = (2745, 7897);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53422u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 7900);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 7902);
                c.step()?;
                let x = c.r[2];
                let y = c.r[3];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 7904);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 7916 } else { 7906 };
                continue;
            }
            7906 => {
                c.pc = (2745, 7906);
                c.step()?;
                label = if c.sf != c.of { 7912 } else { 7908 };
                continue;
            }
            7908 => {
                c.pc = (2745, 7908);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 7910);
                c.step()?;
                label = if !c.cf && !c.z { 7916 } else { 7912 };
                continue;
            }
            7912 => {
                c.pc = (2745, 7912);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7914);
                c.step()?;
                label = 7918;
                continue;
            }
            7916 => {
                c.pc = (2745, 7916);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 7918;
                continue;
            }
            7918 => {
                c.pc = (2745, 7918);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 7919);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7922);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 7923);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 7924);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 7925);
                c.step()?;
                c.invoke(p_0ab9_1cbb, false, 2745, 7355, 7928)?;
                label = 7928;
                continue;
            }
            7928 => {
                c.pc = (2745, 7928);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 7929);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[2] >> 0) & 255);
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7931);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 7932);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 7934);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23752u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7938);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 7941);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7944);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 7945);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_1fd3(c: &mut Cpu) -> Result<(), String> {
    let mut label = 8147u16;
    loop {
        match label {
            8147 => {
                c.pc = (2745, 8147);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 8151);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 8154);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 8155);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 8156);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 8157);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 8160)?;
                label = 8160;
                continue;
            }
            8160 => {
                c.pc = (2745, 8160);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8161);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 8163);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 8166);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8170);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8171);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 8173);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 8174);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 8176);
                c.step()?;
                let v = c.r[2];
                c.r[3] = v;
                c.pc = (2745, 8178);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 8181);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 8183);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 8185);
                c.step()?;
                let x = c.r[2];
                let y = c.r[3];
                let v = c.adc(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 8187);
                c.step()?;
                let v = c.r[2];
                c.push(v)?;
                c.pc = (2745, 8188);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 8189);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 8190);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 8191);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 8192);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 8195)?;
                label = 8195;
                continue;
            }
            8195 => {
                c.pc = (2745, 8195);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8196);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 8198);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53063u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8201);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8202);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 8204);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 8205);
                c.step()?;
                let v = c.pop()?;
                c.r[1] = v;
                c.pc = (2745, 8206);
                c.step()?;
                let v = c.pop()?;
                c.r[3] = v;
                c.pc = (2745, 8207);
                c.step()?;
                let x = c.r[2];
                let y = c.r[3];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 8209);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 8221 } else { 8211 };
                continue;
            }
            8211 => {
                c.pc = (2745, 8211);
                c.step()?;
                label = if c.sf != c.of { 8217 } else { 8213 };
                continue;
            }
            8213 => {
                c.pc = (2745, 8213);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 8215);
                c.step()?;
                label = if !c.cf && !c.z { 8221 } else { 8217 };
                continue;
            }
            8217 => {
                c.pc = (2745, 8217);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8219);
                c.step()?;
                label = 8223;
                continue;
            }
            8221 => {
                c.pc = (2745, 8221);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 8223;
                continue;
            }
            8223 => {
                c.pc = (2745, 8223);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 8226);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8229);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 8230);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_2029(c: &mut Cpu) -> Result<(), String> {
    let mut label = 8233u16;
    loop {
        match label {
            8233 => {
                c.pc = (2745, 8233);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 8237);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 8240);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 8241);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 8242);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 8243);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 8246)?;
                label = 8246;
                continue;
            }
            8246 => {
                c.pc = (2745, 8246);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8247);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 8249);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 8252);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8256);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8257);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 8259);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 8260);
                c.step()?;
                let v = c.r[2];
                c.push(v)?;
                c.pc = (2745, 8261);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 8262);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 8263);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 8264);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 8265);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 8268)?;
                label = 8268;
                continue;
            }
            8268 => {
                c.pc = (2745, 8268);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8269);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 8271);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53063u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8274);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8275);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 8277);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 8278);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 8280);
                c.step()?;
                let v = c.r[2];
                c.r[3] = v;
                c.pc = (2745, 8282);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 8285);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 8287);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 8289);
                c.step()?;
                let x = c.r[2];
                let y = c.r[3];
                let v = c.adc(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 8291);
                c.step()?;
                let v = c.pop()?;
                c.r[1] = v;
                c.pc = (2745, 8292);
                c.step()?;
                let v = c.pop()?;
                c.r[3] = v;
                c.pc = (2745, 8293);
                c.step()?;
                let x = c.r[2];
                let y = c.r[3];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 8295);
                c.step()?;
                label = if c.sf != c.of { 8307 } else { 8297 };
                continue;
            }
            8297 => {
                c.pc = (2745, 8297);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 8303 } else { 8299 };
                continue;
            }
            8299 => {
                c.pc = (2745, 8299);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 8301);
                c.step()?;
                label = if c.cf { 8307 } else { 8303 };
                continue;
            }
            8303 => {
                c.pc = (2745, 8303);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8305);
                c.step()?;
                label = 8309;
                continue;
            }
            8307 => {
                c.pc = (2745, 8307);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 8309;
                continue;
            }
            8309 => {
                c.pc = (2745, 8309);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 8312);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8315);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 8316);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_325f(c: &mut Cpu) -> Result<(), String> {
    let mut label = 12895u16;
    loop {
        match label {
            12895 => {
                c.pc = (2745, 12895);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 12899);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 12902);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 12906);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 12909);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 12913);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 12915);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 12918);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 12921);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 12926);
                c.step()?;
                label = if !c.z { 12935 } else { 12928 };
                continue;
            }
            12928 => {
                c.pc = (2745, 12928);
                c.step()?;
                let v = 4u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 12933);
                c.step()?;
                label = 13011;
                continue;
            }
            12935 => {
                c.pc = (2745, 12935);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 12938);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                let y = 11u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 12943);
                c.step()?;
                label = if !c.cf && !c.z { 12977 } else { 12945 };
                continue;
            }
            12945 => {
                c.pc = (2745, 12945);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 12947);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 12950);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 12951);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 12952);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 12953);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 12956)?;
                label = 12956;
                continue;
            }
            12956 => {
                c.pc = (2745, 12956);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12958);
                c.step()?;
                label = if c.z { 12973 } else { 12960 };
                continue;
            }
            12960 => {
                c.pc = (2745, 12960);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 12963);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 12964);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 12965);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 12966);
                c.step()?;
                c.invoke(p_0ab9_1e4e, false, 2745, 7758, 12969)?;
                label = 12969;
                continue;
            }
            12969 => {
                c.pc = (2745, 12969);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12971);
                c.step()?;
                label = if !c.z { 12977 } else { 12973 };
                continue;
            }
            12973 => {
                c.pc = (2745, 12973);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12975);
                c.step()?;
                label = 12979;
                continue;
            }
            12977 => {
                c.pc = (2745, 12977);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 12979;
                continue;
            }
            12979 => {
                c.pc = (2745, 12979);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 12980);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 12982);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 12985);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                let y = 4u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 12990);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12992);
                c.step()?;
                label = if c.cf || c.z { 12995 } else { 12994 };
                continue;
            }
            12994 => {
                c.pc = (2745, 12994);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 12995;
                continue;
            }
            12995 => {
                c.pc = (2745, 12995);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 12996);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 12998);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13000);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13002);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2646u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13006);
                c.step()?;
                let x = ((c.r[0] >> 8) & 255);
                let y = ((c.r[0] >> 8) & 255);
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 13008);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 13011;
                continue;
            }
            13011 => {
                c.pc = (2745, 13011);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 13014);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 13015);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_340d(c: &mut Cpu) -> Result<(), String> {
    let mut label = 13325u16;
    loop {
        match label {
            13325 => {
                c.pc = (2745, 13325);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 13329);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 13333);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13336);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13340);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13341);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13342);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13343);
                c.step()?;
                c.invoke(p_0ab9_325f, false, 2745, 12895, 13346)?;
                label = 13346;
                continue;
            }
            13346 => {
                c.pc = (2745, 13346);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 13347);
                c.step()?;
                let v = 7u16;
                c.push(v)?;
                c.pc = (2745, 13349);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13352);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13356);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13357);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13358);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13359);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 13362)?;
                label = 13362;
                continue;
            }
            13362 => {
                c.pc = (2745, 13362);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13363);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 13365);
                c.step()?;
                let x = c.r[7];
                let y = 2u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13368);
                c.step()?;
                let v = c.pop()?;
                c.r[0] = v;
                c.pc = (2745, 13369);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13371);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2679u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13375);
                c.step()?;
                let x = ((c.r[0] >> 8) & 255);
                let y = ((c.r[0] >> 8) & 255);
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 13377);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 13379);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13382);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13386);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(13u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13390);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13391);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13393);
                c.step()?;
                label = if c.sf != c.of { 13398 } else { 13395 };
                continue;
            }
            13395 => {
                c.pc = (2745, 13395);
                c.step()?;
                label = 13573;
                continue;
            }
            13398 => {
                c.pc = (2745, 13398);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13399);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13400);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13401);
                c.step()?;
                c.invoke(p_0ab9_325f, false, 2745, 12895, 13404)?;
                label = 13404;
                continue;
            }
            13404 => {
                c.pc = (2745, 13404);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13407);
                c.step()?;
                label = if !c.z { 13467 } else { 13409 };
                continue;
            }
            13409 => {
                c.pc = (2745, 13409);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 13411);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13414);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13418);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13419);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13420);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13421);
                c.step()?;
                c.invoke(p_0ab9_1fd3, false, 2745, 8147, 13424)?;
                label = 13424;
                continue;
            }
            13424 => {
                c.pc = (2745, 13424);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13426);
                c.step()?;
                label = if !c.z { 13460 } else { 13428 };
                continue;
            }
            13428 => {
                c.pc = (2745, 13428);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 13430);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13433);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13437);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13438);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13439);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13440);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 13443)?;
                label = 13443;
                continue;
            }
            13443 => {
                c.pc = (2745, 13443);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13445);
                c.step()?;
                label = if !c.z { 13460 } else { 13447 };
                continue;
            }
            13447 => {
                c.pc = (2745, 13447);
                c.step()?;
                c.invoke(p_24bc_0d3f, true, 9404, 3391, 13452)?;
                label = 13452;
                continue;
            }
            13452 => {
                c.pc = (2745, 13452);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13454);
                c.step()?;
                label = if !c.z { 13460 } else { 13456 };
                continue;
            }
            13456 => {
                c.pc = (2745, 13456);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13458);
                c.step()?;
                label = 13462;
                continue;
            }
            13460 => {
                c.pc = (2745, 13460);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 13462;
                continue;
            }
            13462 => {
                c.pc = (2745, 13462);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 13465);
                c.step()?;
                label = 13573;
                continue;
            }
            13467 => {
                c.pc = (2745, 13467);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13470);
                c.step()?;
                label = if !c.z { 13521 } else { 13472 };
                continue;
            }
            13472 => {
                c.pc = (2745, 13472);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 13474);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13477);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13481);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13482);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13483);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13484);
                c.step()?;
                c.invoke(p_0ab9_1fd3, false, 2745, 8147, 13487)?;
                label = 13487;
                continue;
            }
            13487 => {
                c.pc = (2745, 13487);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13489);
                c.step()?;
                label = if !c.z { 13514 } else { 13491 };
                continue;
            }
            13491 => {
                c.pc = (2745, 13491);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 13493);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13496);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13500);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13501);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13502);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13503);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 13506)?;
                label = 13506;
                continue;
            }
            13506 => {
                c.pc = (2745, 13506);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13508);
                c.step()?;
                label = if !c.z { 13514 } else { 13510 };
                continue;
            }
            13510 => {
                c.pc = (2745, 13510);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13512);
                c.step()?;
                label = 13516;
                continue;
            }
            13514 => {
                c.pc = (2745, 13514);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 13516;
                continue;
            }
            13516 => {
                c.pc = (2745, 13516);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 13519);
                c.step()?;
                label = 13573;
                continue;
            }
            13521 => {
                c.pc = (2745, 13521);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13524);
                c.step()?;
                label = if !c.z { 13573 } else { 13526 };
                continue;
            }
            13526 => {
                c.pc = (2745, 13526);
                c.step()?;
                let v = 30u16;
                c.push(v)?;
                c.pc = (2745, 13528);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13531);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13535);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13536);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13537);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13538);
                c.step()?;
                c.invoke(p_0ab9_1fd3, false, 2745, 8147, 13541)?;
                label = 13541;
                continue;
            }
            13541 => {
                c.pc = (2745, 13541);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13543);
                c.step()?;
                label = if !c.z { 13568 } else { 13545 };
                continue;
            }
            13545 => {
                c.pc = (2745, 13545);
                c.step()?;
                let v = 7u16;
                c.push(v)?;
                c.pc = (2745, 13547);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13550);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13554);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13555);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13556);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13557);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 13560)?;
                label = 13560;
                continue;
            }
            13560 => {
                c.pc = (2745, 13560);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13562);
                c.step()?;
                label = if !c.z { 13568 } else { 13564 };
                continue;
            }
            13564 => {
                c.pc = (2745, 13564);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13566);
                c.step()?;
                label = 13570;
                continue;
            }
            13568 => {
                c.pc = (2745, 13568);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 13570;
                continue;
            }
            13570 => {
                c.pc = (2745, 13570);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 13573;
                continue;
            }
            13573 => {
                c.pc = (2745, 13573);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13576);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 13577);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_350c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 13580u16;
    loop {
        match label {
            13580 => {
                c.pc = (2745, 13580);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 13584);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 13588);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13591);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13595);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13596);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13597);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13598);
                c.step()?;
                c.invoke(p_0ab9_325f, false, 2745, 12895, 13601)?;
                label = 13601;
                continue;
            }
            13601 => {
                c.pc = (2745, 13601);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 13602);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 13604);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13607);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13611);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13612);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13613);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13614);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 13617)?;
                label = 13617;
                continue;
            }
            13617 => {
                c.pc = (2745, 13617);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13618);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13621);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 13622);
                c.step()?;
                let v = 5u16;
                c.push(v)?;
                c.pc = (2745, 13624);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13627);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13631);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13632);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13633);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13634);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 13637)?;
                label = 13637;
                continue;
            }
            13637 => {
                c.pc = (2745, 13637);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13638);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 13640);
                c.step()?;
                let x = c.r[7];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13643);
                c.step()?;
                let v = c.pop()?;
                c.r[0] = v;
                c.pc = (2745, 13644);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13646);
                c.step()?;
                let v = c.pop()?;
                c.r[0] = v;
                c.pc = (2745, 13647);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13649);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2663u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13653);
                c.step()?;
                let x = ((c.r[0] >> 8) & 255);
                let y = ((c.r[0] >> 8) & 255);
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 13655);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 13657);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13660);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13664);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(13u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13668);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13669);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13671);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 13676 } else { 13673 };
                continue;
            }
            13673 => {
                c.pc = (2745, 13673);
                c.step()?;
                label = 13809;
                continue;
            }
            13676 => {
                c.pc = (2745, 13676);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13677);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13678);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13679);
                c.step()?;
                c.invoke(p_0ab9_325f, false, 2745, 12895, 13682)?;
                label = 13682;
                continue;
            }
            13682 => {
                c.pc = (2745, 13682);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13685);
                c.step()?;
                label = if !c.z { 13707 } else { 13687 };
                continue;
            }
            13687 => {
                c.pc = (2745, 13687);
                c.step()?;
                let v = 15u16;
                c.push(v)?;
                c.pc = (2745, 13689);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13692);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13696);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13697);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13698);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13699);
                c.step()?;
                c.invoke(p_0ab9_2029, false, 2745, 8233, 13702)?;
                label = 13702;
                continue;
            }
            13702 => {
                c.pc = (2745, 13702);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 13705);
                c.step()?;
                label = 13809;
                continue;
            }
            13707 => {
                c.pc = (2745, 13707);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13710);
                c.step()?;
                label = if !c.z { 13732 } else { 13712 };
                continue;
            }
            13712 => {
                c.pc = (2745, 13712);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 13714);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13717);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13721);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13722);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13723);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13724);
                c.step()?;
                c.invoke(p_0ab9_2029, false, 2745, 8233, 13727)?;
                label = 13727;
                continue;
            }
            13727 => {
                c.pc = (2745, 13727);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 13730);
                c.step()?;
                label = 13809;
                continue;
            }
            13732 => {
                c.pc = (2745, 13732);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13735);
                c.step()?;
                label = if !c.z { 13809 } else { 13737 };
                continue;
            }
            13737 => {
                c.pc = (2745, 13737);
                c.step()?;
                let v = 10u16;
                c.push(v)?;
                c.pc = (2745, 13739);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13742);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13746);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13747);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13748);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13749);
                c.step()?;
                c.invoke(p_0ab9_1fd3, false, 2745, 8147, 13752)?;
                label = 13752;
                continue;
            }
            13752 => {
                c.pc = (2745, 13752);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13754);
                c.step()?;
                label = if c.z { 13804 } else { 13756 };
                continue;
            }
            13756 => {
                c.pc = (2745, 13756);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13759);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13763);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13764);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13765);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13766);
                c.step()?;
                c.invoke(p_0ab9_1e9f, false, 2745, 7839, 13769)?;
                label = 13769;
                continue;
            }
            13769 => {
                c.pc = (2745, 13769);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = 2u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 13771);
                c.step()?;
                label = if c.z { 13804 } else { 13773 };
                continue;
            }
            13773 => {
                c.pc = (2745, 13773);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13776);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13780);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 13784);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13787);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13791);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13793);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 13798);
                c.step()?;
                label = if c.z { 13804 } else { 13800 };
                continue;
            }
            13800 => {
                c.pc = (2745, 13800);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13802);
                c.step()?;
                label = 13806;
                continue;
            }
            13804 => {
                c.pc = (2745, 13804);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 13806;
                continue;
            }
            13806 => {
                c.pc = (2745, 13806);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 13809;
                continue;
            }
            13809 => {
                c.pc = (2745, 13809);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13812);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 13813);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_35f8(c: &mut Cpu) -> Result<(), String> {
    let mut label = 13816u16;
    loop {
        match label {
            13816 => {
                c.pc = (2745, 13816);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 13820);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13823);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13824);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13825);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13826);
                c.step()?;
                c.invoke(p_0ab9_1e4e, false, 2745, 7758, 13829)?;
                label = 13829;
                continue;
            }
            13829 => {
                c.pc = (2745, 13829);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13831);
                c.step()?;
                label = if c.z { 13884 } else { 13833 };
                continue;
            }
            13833 => {
                c.pc = (2745, 13833);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13836);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 13840);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13843);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13847);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13849);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                let y = 4u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 13854);
                c.step()?;
                label = if !c.cf && !c.z { 13871 } else { 13856 };
                continue;
            }
            13856 => {
                c.pc = (2745, 13856);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 13858);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13861);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13862);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13863);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13864);
                c.step()?;
                c.invoke(p_0ab9_1fd3, false, 2745, 8147, 13867)?;
                label = 13867;
                continue;
            }
            13867 => {
                c.pc = (2745, 13867);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = ((c.r[0] >> 0) & 255);
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13869);
                c.step()?;
                label = if c.z { 13884 } else { 13871 };
                continue;
            }
            13871 => {
                c.pc = (2745, 13871);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 13872);
                c.step()?;
                c.invoke(p_0ab9_340d, false, 2745, 13325, 13875)?;
                label = 13875;
                continue;
            }
            13875 => {
                c.pc = (2745, 13875);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13878);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(13u16), 8)?;
                let y = ((c.r[0] >> 0) & 255);
                let v = c.add(x, y, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(13u16), 8, v)?;
                c.pc = (2745, 13882);
                c.step()?;
                label = 13919;
                continue;
            }
            13884 => {
                c.pc = (2745, 13884);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 13885);
                c.step()?;
                c.invoke(p_0ab9_350c, false, 2745, 13580, 13888)?;
                label = 13888;
                continue;
            }
            13888 => {
                c.pc = (2745, 13888);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13891);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(13u16), 8)?;
                let y = ((c.r[0] >> 0) & 255);
                let v = c.sub(x, y, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(13u16), 8, v)?;
                c.pc = (2745, 13895);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13896);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13897);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13898);
                c.step()?;
                c.invoke(p_0ab9_1e9f, false, 2745, 7839, 13901)?;
                label = 13901;
                continue;
            }
            13901 => {
                c.pc = (2745, 13901);
                c.step()?;
                let x = ((c.r[0] >> 0) & 255);
                let y = 2u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 13903);
                c.step()?;
                label = if !c.z { 13919 } else { 13905 };
                continue;
            }
            13905 => {
                c.pc = (2745, 13905);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13908);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13912);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13913);
                c.step()?;
                let x = c.r[0];
                let v = c.neg(x, 16);
                c.r[0] = v;
                c.pc = (2745, 13915);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(12u16), 8, v)?;
                label = 13919;
                continue;
            }
            13919 => {
                c.pc = (2745, 13919);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 13920);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_24bc_0259(c: &mut Cpu) -> Result<(), String> {
    let mut label = 601u16;
    loop {
        match label {
            601 => {
                c.pc = (9404, 601);
                c.step()?;
                c.enter(2)?;
                c.pc = (9404, 605);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 16)?;
                c.r[0] = v;
                c.pc = (9404, 608);
                c.step()?;
                let x = c.r[0];
                let v = c.dec(x, 16);
                c.r[0] = v;
                c.pc = (9404, 609);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (9404, 611);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.r[0] = v;
                c.pc = (9404, 614);
                c.step()?;
                let v = c.r[2];
                c.r[1] = v;
                c.pc = (9404, 616);
                c.step()?;
                let x = c.r[0];
                let y = ((c.r[1] >> 0) & 255);
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (9404, 618);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.and(x, y, 16);
                c.r[0] = v;
                c.pc = (9404, 621);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (9404, 624);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9404, 627);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9404, 628);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_24bc_0cdc(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3292u16;
    loop {
        match label {
            3292 => {
                c.pc = (9404, 3292);
                c.step()?;
                c.enter(2)?;
                c.pc = (9404, 3296);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (9404, 3297);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(8u16);
                let s = c.r[10];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[6] = v;
                c.r[11] = g;
                c.pc = (9404, 3300);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (9404, 3301);
                c.step()?;
                let v = c.r[6];
                c.push(v)?;
                c.pc = (9404, 3302);
                c.step()?;
                c.r[0] = c.read(c.r[11], c.r[6], 16)?;
                c.r[6] = c.r[6].wrapping_add(2);
                c.pc = (9404, 3303);
                c.step()?;
                let v = 31421u16;
                c.r[3] = v;
                c.pc = (9404, 3306);
                c.step()?;
                let v = c.r[3];
                c.multiply(v, false);
                c.pc = (9404, 3308);
                c.step()?;
                let x = c.r[0];
                let y = 6927u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (9404, 3311);
                c.step()?;
                let v = c.pop()?;
                c.r[7] = v;
                c.pc = (9404, 3312);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (9404, 3313);
                c.step()?;
                c.write(c.r[8], c.r[7], 16, c.r[0])?;
                c.r[7] = c.r[7].wrapping_add(2);
                c.pc = (9404, 3314);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.multiply(v, false);
                c.pc = (9404, 3317);
                c.step()?;
                let v = c.pop()?;
                c.r[11] = v;
                c.pc = (9404, 3318);
                c.step()?;
                let x = c.r[2];
                let v = c.inc(x, 16);
                c.r[2] = v;
                c.pc = (9404, 3319);
                c.step()?;
                let v = c.r[2];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (9404, 3322);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (9404, 3325);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9404, 3326);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_24bc_0d3f(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3391u16;
    loop {
        match label {
            3391 => {
                c.pc = (9404, 3391);
                c.step()?;
                c.enter(2)?;
                c.pc = (9404, 3395);
                c.step()?;
                let o = 0u16.wrapping_add(56964u16);
                let s = c.r[11];
                let v = c.read(s, o, 16)?;
                let g = c.read(s, o.wrapping_add(2), 16)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9404, 3399);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (9404, 3400);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (9404, 3401);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (9404, 3403);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (9404, 3404);
                c.step()?;
                c.invoke(p_24bc_0cdc, false, 9404, 3292, 3407)?;
                label = 3407;
                continue;
            }
            3407 => {
                c.pc = (9404, 3407);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (9404, 3410);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9404, 3412);
                c.step()?;
                label = if !c.z { 3415 } else { 3414 };
                continue;
            }
            3414 => {
                c.pc = (9404, 3414);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 3415;
                continue;
            }
            3415 => {
                c.pc = (9404, 3415);
                c.step()?;
                let v = ((c.r[0] >> 0) & 255);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (9404, 3418);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9404, 3421);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9404, 3422);
                c.step()?;
                c.ret(true, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
