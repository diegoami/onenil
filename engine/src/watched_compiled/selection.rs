//! [confirmed] Mechanically compiled numeric control flow;F39/c7637bd,static d9e20a4.
//! [designed] Direct Rust calls/blocks;no runtime decoder or original executable.
use super::compiled::Cpu;
pub(super) fn p_0ab9_0000(c: &mut Cpu) -> Result<(), String> {
    let mut label = 0u16;
    loop {
        match label {
            0 => {
                c.pc = (2745, 0);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 4);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 8);
                c.step()?;
                label = if c.z { 20 } else { 10 };
                continue;
            }
            10 => {
                c.pc = (2745, 10);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                let y = 12u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 14);
                c.step()?;
                label = if c.z { 20 } else { 16 };
                continue;
            }
            16 => {
                c.pc = (2745, 16);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18);
                c.step()?;
                label = 22;
                continue;
            }
            20 => {
                c.pc = (2745, 20);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 22;
                continue;
            }
            22 => {
                c.pc = (2745, 22);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 25);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 28);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 29);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
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
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6771);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 6776);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
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
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
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
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
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
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6804);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 6809);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
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
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
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
                let v = (c.r[0] >> 0) & 255;
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
                let (v, g) = c.far(s, o)?;
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
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 6854);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
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
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
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
                let v = (c.r[0] >> 0) & 255;
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
pub(super) fn p_0ab9_1ae2(c: &mut Cpu) -> Result<(), String> {
    let mut label = 6882u16;
    loop {
        match label {
            6882 => {
                c.pc = (2745, 6882);
                c.step()?;
                c.enter(10)?;
                c.pc = (2745, 6886);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6889);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 6894);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6898);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 6900);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 6903);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 6906);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                let y = 32u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 6911);
                c.step()?;
                label = if c.sf == c.of { 6924 } else { 6913 };
                continue;
            }
            6913 => {
                c.pc = (2745, 6913);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6917);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 6918);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 6919);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 6922);
                c.step()?;
                label = 6975;
                continue;
            }
            6924 => {
                c.pc = (2745, 6924);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6927);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 6931);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 6934);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6938);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 6940);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6943);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 6945);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[11], 0u16.wrapping_add(55014u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 6949);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6951);
                c.step()?;
                label = if !c.z { 6954 } else { 6953 };
                continue;
            }
            6953 => {
                c.pc = (2745, 6953);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 6954;
                continue;
            }
            6954 => {
                c.pc = (2745, 6954);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 6955);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 6957);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(11826u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 6962);
                c.step()?;
                label = if !c.z { 6970 } else { 6964 };
                continue;
            }
            6964 => {
                c.pc = (2745, 6964);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 32u16;
                let v = c.add(x, y, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 6968);
                c.step()?;
                label = 6975;
                continue;
            }
            6970 => {
                c.pc = (2745, 6970);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 6972);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 6975;
                continue;
            }
            6975 => {
                c.pc = (2745, 6975);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                let y = 21u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 6979);
                c.step()?;
                label = if c.cf { 6987 } else { 6981 };
                continue;
            }
            6981 => {
                c.pc = (2745, 6981);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 32u16;
                let v = c.add(x, y, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 6985);
                c.step()?;
                label = 6998;
                continue;
            }
            6987 => {
                c.pc = (2745, 6987);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 6990);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 6994);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 6995);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 6998;
                continue;
            }
            6998 => {
                c.pc = (2745, 6998);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7001);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 7005);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 7008);
                c.step()?;
                c.invoke(p_0ab9_9f6e, false, 2745, 40814, 7011)?;
                label = 7011;
                continue;
            }
            7011 => {
                c.pc = (2745, 7011);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 7014);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7017);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(22u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 7022);
                c.step()?;
                label = if c.z { 7032 } else { 7024 };
                continue;
            }
            7024 => {
                c.pc = (2745, 7024);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 7027);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 7029);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                label = 7032;
                continue;
            }
            7032 => {
                c.pc = (2745, 7032);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 7035);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 7038);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 7040);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 7043);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.multiply(v, false);
                c.pc = (2745, 7046);
                c.step()?;
                let x = c.r[0];
                let y = 8u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 7049);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 7051);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 7054);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 7057);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2555u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 7062);
                c.step()?;
                label = if c.z { 7076 } else { 7064 };
                continue;
            }
            7064 => {
                c.pc = (2745, 7064);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7067);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(18u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7071);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 7073);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 7076;
                continue;
            }
            7076 => {
                c.pc = (2745, 7076);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7079);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                let y = 12u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 7084);
                c.step()?;
                label = if !c.cf { 7102 } else { 7086 };
                continue;
            }
            7086 => {
                c.pc = (2745, 7086);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(3369u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 7091);
                c.step()?;
                label = if c.z { 7102 } else { 7093 };
                continue;
            }
            7093 => {
                c.pc = (2745, 7093);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 7096);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 7099);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 7102;
                continue;
            }
            7102 => {
                c.pc = (2745, 7102);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 7106);
                c.step()?;
                label = if !c.cf { 7113 } else { 7108 };
                continue;
            }
            7108 => {
                c.pc = (2745, 7108);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 7113;
                continue;
            }
            7113 => {
                c.pc = (2745, 7113);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 7116);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 7119);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 7122);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 7123);
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
                let (v, g) = c.far(s, o)?;
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
                let v = (c.r[0] >> 0) & 255;
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
                let v = (c.r[0] >> 0) & 255;
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
                let (v, g) = c.far(s, o)?;
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
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[2] >> 0) & 255;
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
                let v = (c.r[0] >> 0) & 255;
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
pub(super) fn p_0ab9_1e71(c: &mut Cpu) -> Result<(), String> {
    let mut label = 7793u16;
    loop {
        match label {
            7793 => {
                c.pc = (2745, 7793);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 7797);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7800);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 7805);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7809);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 7811);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7815);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 7816);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 7818);
                c.step()?;
                let v = 33u16;
                c.r[0] = v;
                c.pc = (2745, 7821);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 7823);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 7824);
                c.step()?;
                c.invoke(p_24bc_0d1c, true, 9404, 3356, 7829)?;
                label = 7829;
                continue;
            }
            7829 => {
                c.pc = (2745, 7829);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 7832);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 7835);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 7836);
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
                let (v, g) = c.far(s, o)?;
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
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[2] >> 0) & 255;
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
                let v = (c.r[0] >> 0) & 255;
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
pub(super) fn p_0ab9_1f33(c: &mut Cpu) -> Result<(), String> {
    let mut label = 7987u16;
    loop {
        match label {
            7987 => {
                c.pc = (2745, 7987);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 7991);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 7994);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 7995);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 7996);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 7997);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 8000)?;
                label = 8000;
                continue;
            }
            8000 => {
                c.pc = (2745, 8000);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8001);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 8003);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 8006);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8010);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8011);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 8013);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 8015);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8017);
                c.step()?;
                label = if c.sf == c.of { 8020 } else { 8019 };
                continue;
            }
            8019 => {
                c.pc = (2745, 8019);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 8020;
                continue;
            }
            8020 => {
                c.pc = (2745, 8020);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 8023);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8026);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 8027);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_1f5e(c: &mut Cpu) -> Result<(), String> {
    let mut label = 8030u16;
    loop {
        match label {
            8030 => {
                c.pc = (2745, 8030);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 8034);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 8037);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 8038);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 8039);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 8040);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 8043)?;
                label = 8043;
                continue;
            }
            8043 => {
                c.pc = (2745, 8043);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8044);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 8046);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8049);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8050);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 8052);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(11u16), 8, v)?;
                c.pc = (2745, 8055);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 8058);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 8059);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 8060);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 8061);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 8064)?;
                label = 8064;
                continue;
            }
            8064 => {
                c.pc = (2745, 8064);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8065);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 8067);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8070);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8071);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 8073);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8, v)?;
                c.pc = (2745, 8076);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 8079);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 8082);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 8085);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 8086);
                c.step()?;
                c.ret(true, 6)?;
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
                let (v, g) = c.far(s, o)?;
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
                let (v, g) = c.far(s, o)?;
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
                let v = (c.r[0] >> 0) & 255;
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
pub(super) fn p_0ab9_207f(c: &mut Cpu) -> Result<(), String> {
    let mut label = 8319u16;
    loop {
        match label {
            8319 => {
                c.pc = (2745, 8319);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 8323);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 8326);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 8327);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 8328);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 8329);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 8332)?;
                label = 8332;
                continue;
            }
            8332 => {
                c.pc = (2745, 8332);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8333);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 8335);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8338);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8339);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 8341);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 8343);
                c.step()?;
                label = if c.sf == c.of { 8373 } else { 8345 };
                continue;
            }
            8345 => {
                c.pc = (2745, 8345);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8348);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8349);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 8350);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 8352);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 8354);
                c.step()?;
                let x = c.r[0];
                let y = 33u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 8357);
                c.step()?;
                label = if c.sf != c.of { 8373 } else { 8359 };
                continue;
            }
            8359 => {
                c.pc = (2745, 8359);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8362);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 8363);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 8364);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 8366);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 8368);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 8371);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 8377 } else { 8373 };
                continue;
            }
            8373 => {
                c.pc = (2745, 8373);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8375);
                c.step()?;
                label = 8379;
                continue;
            }
            8377 => {
                c.pc = (2745, 8377);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 8379;
                continue;
            }
            8379 => {
                c.pc = (2745, 8379);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 8382);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 8385);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 8386);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_3098(c: &mut Cpu) -> Result<(), String> {
    let mut label = 12440u16;
    loop {
        match label {
            12440 => {
                c.pc = (2745, 12440);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 12444);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12447);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 12449);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 12452);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 12456);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 12457);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 12458);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 12461);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 12464);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 12465);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 12466);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 12467);
                c.step()?;
                c.invoke(p_0ab9_1f33, false, 2745, 7987, 12470)?;
                label = 12470;
                continue;
            }
            12470 => {
                c.pc = (2745, 12470);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12472);
                c.step()?;
                label = if !c.z { 12532 } else { 12474 };
                continue;
            }
            12474 => {
                c.pc = (2745, 12474);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 12477)?;
                label = 12477;
                continue;
            }
            12477 => {
                c.pc = (2745, 12477);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12479);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12481);
                c.step()?;
                label = if !c.z { 12484 } else { 12483 };
                continue;
            }
            12483 => {
                c.pc = (2745, 12483);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 12484;
                continue;
            }
            12484 => {
                c.pc = (2745, 12484);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 12485);
                c.step()?;
                let x = c.r[0];
                let y = 7u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 12488);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12497u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12492);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 12493);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 12494);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 12496);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 12498);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 12500);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 12503);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12507);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 12508);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 12509);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 12511);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 12513);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 12515);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 12532 } else { 12517 };
                continue;
            }
            12517 => {
                c.pc = (2745, 12517);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12521);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 12522);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 12523);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 12525);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 12527);
                c.step()?;
                let x = c.r[0];
                let y = 35u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 12530);
                c.step()?;
                label = if c.sf != c.of { 12536 } else { 12532 };
                continue;
            }
            12532 => {
                c.pc = (2745, 12532);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12534);
                c.step()?;
                label = 12538;
                continue;
            }
            12536 => {
                c.pc = (2745, 12536);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 12538;
                continue;
            }
            12538 => {
                c.pc = (2745, 12538);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 12541);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 12544);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 12545);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_32da(c: &mut Cpu) -> Result<(), String> {
    let mut label = 13018u16;
    loop {
        match label {
            13018 => {
                c.pc = (2745, 13018);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 13019);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 13021);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13024);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 13028);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13031);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13035);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13037);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13041);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 13043);
                c.step()?;
                label = if !c.z { 13055 } else { 13045 };
                continue;
            }
            13045 => {
                c.pc = (2745, 13045);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13048);
                c.step()?;
                let v = 0u16;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(12u16), 8, v)?;
                c.pc = (2745, 13053);
                c.step()?;
                label = 13124;
                continue;
            }
            13055 => {
                c.pc = (2745, 13055);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 13u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 13057);
                c.step()?;
                label = if !c.z { 13107 } else { 13059 };
                continue;
            }
            13059 => {
                c.pc = (2745, 13059);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13062);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(19u16), 8)?;
                let y = 9u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 13067);
                c.step()?;
                label = if !c.z { 13088 } else { 13069 };
                continue;
            }
            13069 => {
                c.pc = (2745, 13069);
                c.step()?;
                let v = 5u16;
                c.push(v)?;
                c.pc = (2745, 13071);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 13076)?;
                label = 13076;
                continue;
            }
            13076 => {
                c.pc = (2745, 13076);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13079);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13082);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(12u16), 8, v)?;
                c.pc = (2745, 13086);
                c.step()?;
                label = 13105;
                continue;
            }
            13088 => {
                c.pc = (2745, 13088);
                c.step()?;
                let v = 15u16;
                c.push(v)?;
                c.pc = (2745, 13090);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 13095)?;
                label = 13095;
                continue;
            }
            13095 => {
                c.pc = (2745, 13095);
                c.step()?;
                let x = c.r[0];
                let y = 8u16;
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13098);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13101);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(12u16), 8, v)?;
                label = 13105;
                continue;
            }
            13105 => {
                c.pc = (2745, 13105);
                c.step()?;
                label = 13124;
                continue;
            }
            13107 => {
                c.pc = (2745, 13107);
                c.step()?;
                let v = 5u16;
                c.push(v)?;
                c.pc = (2745, 13109);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 13114)?;
                label = 13114;
                continue;
            }
            13114 => {
                c.pc = (2745, 13114);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13117);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13120);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(12u16), 8, v)?;
                label = 13124;
                continue;
            }
            13124 => {
                c.pc = (2745, 13124);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13127);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(13u16), 8)?;
                let y = 2u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 13132);
                c.step()?;
                label = if c.sf == c.of { 13149 } else { 13134 };
                continue;
            }
            13134 => {
                c.pc = (2745, 13134);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13138);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13139);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 13140);
                c.step()?;
                let v = 2u16;
                c.r[1] = v;
                c.pc = (2745, 13143);
                c.step()?;
                let v = c.r[1];
                c.divide(v, true)?;
                c.pc = (2745, 13145);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(12u16), 8)?;
                let y = (c.r[0] >> 0) & 255;
                let v = c.sub(x, y, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(12u16), 8, v)?;
                label = 13149;
                continue;
            }
            13149 => {
                c.pc = (2745, 13149);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13152);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(13u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13156);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13157);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 13159);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13163);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13164);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13166);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(9u16), 8, v)?;
                c.pc = (2745, 13170);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(12u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13174);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13175);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 13177);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13181);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13182);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13184);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8, v)?;
                c.pc = (2745, 13188);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 13192);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13193);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13194);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13195);
                c.step()?;
                c.invoke(p_0ab9_1f5e, false, 2745, 8030, 13198)?;
                label = 13198;
                continue;
            }
            13198 => {
                c.pc = (2745, 13198);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13201);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 16, v)?;
                c.pc = (2745, 13205);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 13206);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_3399(c: &mut Cpu) -> Result<(), String> {
    let mut label = 13209u16;
    loop {
        match label {
            13209 => {
                c.pc = (2745, 13209);
                c.step()?;
                c.enter(4)?;
                c.pc = (2745, 13213);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13216);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 13220);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 13223);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 13227);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13230);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13234);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13236);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13240);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 13242);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 13244);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2650u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13248);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8)?;
                let y = (c.r[0] >> 0) & 255;
                let v = c.add(x, y, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8, v)?;
                c.pc = (2745, 13251);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13254);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13255);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 13257);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13260);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13261);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13263);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 13264);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13266);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13268);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13271);
                c.step()?;
                label = if c.sf == c.of { 13309 } else { 13273 };
                continue;
            }
            13273 => {
                c.pc = (2745, 13273);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13276);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 13277);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 13278);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 13279);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 13282)?;
                label = 13282;
                continue;
            }
            13282 => {
                c.pc = (2745, 13282);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13283);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 13285);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13288);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13289);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 13291);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 13293);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13296);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13297);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13299);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 13300);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13302);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 13304);
                c.step()?;
                let x = c.r[0];
                let y = 9u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13307);
                c.step()?;
                label = if c.sf != c.of { 13313 } else { 13309 };
                continue;
            }
            13309 => {
                c.pc = (2745, 13309);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13311);
                c.step()?;
                label = 13315;
                continue;
            }
            13313 => {
                c.pc = (2745, 13313);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 13315;
                continue;
            }
            13315 => {
                c.pc = (2745, 13315);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 13318);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13321);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 13322);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_3663(c: &mut Cpu) -> Result<(), String> {
    let mut label = 13923u16;
    loop {
        match label {
            13923 => {
                c.pc = (2745, 13923);
                c.step()?;
                c.enter(10)?;
                c.pc = (2745, 13927);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 13930)?;
                label = 13930;
                continue;
            }
            13930 => {
                c.pc = (2745, 13930);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 13931);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 13933);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13935);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 13940);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13944);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13946);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 13950);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 13953);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 13956);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 13960);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13963);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13967);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(20u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13972);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 13974);
                c.step()?;
                label = if !c.z { 13977 } else { 13976 };
                continue;
            }
            13976 => {
                c.pc = (2745, 13976);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 13977;
                continue;
            }
            13977 => {
                c.pc = (2745, 13977);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13980);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(5u16), 8, v)?;
                c.pc = (2745, 13984);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 13987);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 13991);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(20u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 13996);
                c.step()?;
                label = if !c.z { 14101 } else { 13998 };
                continue;
            }
            13998 => {
                c.pc = (2745, 13998);
                c.step()?;
                let v = 2u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 14003;
                continue;
            }
            14003 => {
                c.pc = (2745, 14003);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 12u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 14007);
                c.step()?;
                label = if !c.cf { 14101 } else { 14009 };
                continue;
            }
            14009 => {
                c.pc = (2745, 14009);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14012);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14014);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14017);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14019);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 14023);
                c.step()?;
                c.invoke(p_1b55_4f89, true, 6997, 20361, 14028)?;
                label = 14028;
                continue;
            }
            14028 => {
                c.pc = (2745, 14028);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14030);
                c.step()?;
                label = if !c.z { 14096 } else { 14032 };
                continue;
            }
            14032 => {
                c.pc = (2745, 14032);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 14035)?;
                label = 14035;
                continue;
            }
            14035 => {
                c.pc = (2745, 14035);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 14036);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 14038);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14042);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 14044);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14047);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 14050);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(13073u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 14054);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14057);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14061);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14062);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14063);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14064);
                c.step()?;
                c.invoke(p_0ab9_3399, false, 2745, 13209, 14067)?;
                label = 14067;
                continue;
            }
            14067 => {
                c.pc = (2745, 14067);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14069);
                c.step()?;
                label = if c.z { 14096 } else { 14071 };
                continue;
            }
            14071 => {
                c.pc = (2745, 14071);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14074);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14076);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14079);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14081);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14085);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14088);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14092);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(20u16), 16, v)?;
                label = 14096;
                continue;
            }
            14096 => {
                c.pc = (2745, 14096);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 14099);
                c.step()?;
                label = 14003;
                continue;
            }
            14101 => {
                c.pc = (2745, 14101);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14104);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14108);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(20u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 14113);
                c.step()?;
                label = if !c.z { 14118 } else { 14115 };
                continue;
            }
            14115 => {
                c.pc = (2745, 14115);
                c.step()?;
                label = 14399;
                continue;
            }
            14118 => {
                c.pc = (2745, 14118);
                c.step()?;
                let v = 2u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 14123;
                continue;
            }
            14123 => {
                c.pc = (2745, 14123);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 12u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 14127);
                c.step()?;
                label = if c.cf { 14132 } else { 14129 };
                continue;
            }
            14129 => {
                c.pc = (2745, 14129);
                c.step()?;
                label = 14386;
                continue;
            }
            14132 => {
                c.pc = (2745, 14132);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14135);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14137);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14140);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14142);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14146);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14149);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14153);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(20u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 14157);
                c.step()?;
                label = if c.z { 14162 } else { 14159 };
                continue;
            }
            14159 => {
                c.pc = (2745, 14159);
                c.step()?;
                label = 14380;
                continue;
            }
            14162 => {
                c.pc = (2745, 14162);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14165);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14167);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14170);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14172);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 14176);
                c.step()?;
                c.invoke(p_1b55_4f89, true, 6997, 20361, 14181)?;
                label = 14181;
                continue;
            }
            14181 => {
                c.pc = (2745, 14181);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14183);
                c.step()?;
                label = if c.z { 14188 } else { 14185 };
                continue;
            }
            14185 => {
                c.pc = (2745, 14185);
                c.step()?;
                label = 14380;
                continue;
            }
            14188 => {
                c.pc = (2745, 14188);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 14191)?;
                label = 14191;
                continue;
            }
            14191 => {
                c.pc = (2745, 14191);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 14192);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 14194);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14198);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 14200);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14203);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14206);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 14210);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(4u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 14215);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(13073u16),
                    16,
                )?;
                c.r[0] = v;
                c.pc = (2745, 14219);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14222);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 14226);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(4u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 14231);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14235);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 14236);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14237);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14238);
                c.step()?;
                c.invoke(p_0ab9_1f33, false, 2745, 7987, 14241)?;
                label = 14241;
                continue;
            }
            14241 => {
                c.pc = (2745, 14241);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14243);
                c.step()?;
                label = if c.z { 14248 } else { 14245 };
                continue;
            }
            14245 => {
                c.pc = (2745, 14245);
                c.step()?;
                label = 14399;
                continue;
            }
            14248 => {
                c.pc = (2745, 14248);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 14253);
                c.step()?;
                let v = 8u16;
                c.push(v)?;
                c.pc = (2745, 14255);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14258);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14262);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14263);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14264);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14265);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 14268)?;
                label = 14268;
                continue;
            }
            14268 => {
                c.pc = (2745, 14268);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 14269);
                c.step()?;
                let v = 16u16;
                c.push(v)?;
                c.pc = (2745, 14271);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14274);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(4u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 14279);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14283);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 14284);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14285);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14286);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 14289)?;
                label = 14289;
                continue;
            }
            14289 => {
                c.pc = (2745, 14289);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 14290);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 14292);
                c.step()?;
                label = if c.cf || c.z { 14339 } else { 14294 };
                continue;
            }
            14294 => {
                c.pc = (2745, 14294);
                c.step()?;
                let v = 8u16;
                c.push(v)?;
                c.pc = (2745, 14296);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14299);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14303);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14304);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14305);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14306);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 14309)?;
                label = 14309;
                continue;
            }
            14309 => {
                c.pc = (2745, 14309);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 14310);
                c.step()?;
                let v = 16u16;
                c.push(v)?;
                c.pc = (2745, 14312);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14315);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(4u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 14320);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14324);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 14325);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14326);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14327);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 14330)?;
                label = 14330;
                continue;
            }
            14330 => {
                c.pc = (2745, 14330);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 14331);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14333);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14336);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                label = 14339;
                continue;
            }
            14339 => {
                c.pc = (2745, 14339);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 14342);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 14347)?;
                label = 14347;
                continue;
            }
            14347 => {
                c.pc = (2745, 14347);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 14348);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14351);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14355);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14356);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14357);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14358);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 14361)?;
                label = 14361;
                continue;
            }
            14361 => {
                c.pc = (2745, 14361);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 14362);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 14363);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 14365);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14368);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(1u16), 8)?;
                let y = (c.r[0] >> 0) & 255;
                let v = c.sub(x, y, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 14372);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 14376);
                c.step()?;
                label = 14399;
                continue;
            }
            14380 => {
                c.pc = (2745, 14380);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 14383);
                c.step()?;
                label = 14123;
                continue;
            }
            14386 => {
                c.pc = (2745, 14386);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 14389);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14393);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14395);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(20u16), 16, v)?;
                label = 14399;
                continue;
            }
            14399 => {
                c.pc = (2745, 14399);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14402);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 14403);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_3846(c: &mut Cpu) -> Result<(), String> {
    let mut label = 14406u16;
    loop {
        match label {
            14406 => {
                c.pc = (2745, 14406);
                c.step()?;
                c.enter(14)?;
                c.pc = (2745, 14410);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14413);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14417);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14420);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14424);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14426);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 14429);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 14432);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14435);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 14440);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14444);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 14446);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(14u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 14449);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(12u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 14452);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14455);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14456);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14457);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14458);
                c.step()?;
                c.invoke(p_0ab9_1e4e, false, 2745, 7758, 14461)?;
                label = 14461;
                continue;
            }
            14461 => {
                c.pc = (2745, 14461);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14463);
                c.step()?;
                label = if c.z { 14515 } else { 14465 };
                continue;
            }
            14465 => {
                c.pc = (2745, 14465);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14468);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14469);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14470);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14471);
                c.step()?;
                c.invoke(p_0ab9_1e71, false, 2745, 7793, 14474)?;
                label = 14474;
                continue;
            }
            14474 => {
                c.pc = (2745, 14474);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14476);
                c.step()?;
                label = if !c.z { 14503 } else { 14478 };
                continue;
            }
            14478 => {
                c.pc = (2745, 14478);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14481);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 14486);
                c.step()?;
                label = if !c.z { 14512 } else { 14488 };
                continue;
            }
            14488 => {
                c.pc = (2745, 14488);
                c.step()?;
                let v = 10u16;
                c.push(v)?;
                c.pc = (2745, 14490);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14493);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14494);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14495);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14496);
                c.step()?;
                c.invoke(p_0ab9_1fd3, false, 2745, 8147, 14499)?;
                label = 14499;
                continue;
            }
            14499 => {
                c.pc = (2745, 14499);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14501);
                c.step()?;
                label = if !c.z { 14512 } else { 14503 };
                continue;
            }
            14503 => {
                c.pc = (2745, 14503);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14506);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14507);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14508);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14509);
                c.step()?;
                c.invoke(p_0ab9_32da, false, 2745, 13018, 14512)?;
                label = 14512;
                continue;
            }
            14512 => {
                c.pc = (2745, 14512);
                c.step()?;
                label = 14707;
                continue;
            }
            14515 => {
                c.pc = (2745, 14515);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14518);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14519);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14520);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14521);
                c.step()?;
                c.invoke(p_0ab9_32da, false, 2745, 13018, 14524)?;
                label = 14524;
                continue;
            }
            14524 => {
                c.pc = (2745, 14524);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14527);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14531);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 14533);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 14534);
                c.step()?;
                c.invoke(p_0ab9_0000, false, 2745, 0, 14537)?;
                label = 14537;
                continue;
            }
            14537 => {
                c.pc = (2745, 14537);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14539);
                c.step()?;
                label = if c.z { 14599 } else { 14541 };
                continue;
            }
            14541 => {
                c.pc = (2745, 14541);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53432u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 14546);
                c.step()?;
                label = if c.z { 14597 } else { 14548 };
                continue;
            }
            14548 => {
                c.pc = (2745, 14548);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53062u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 14552);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14555);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14556);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14557);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14558);
                c.step()?;
                c.invoke(p_0ab9_207f, false, 2745, 8319, 14561)?;
                label = 14561;
                continue;
            }
            14561 => {
                c.pc = (2745, 14561);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14563);
                c.step()?;
                label = if !c.z { 14587 } else { 14565 };
                continue;
            }
            14565 => {
                c.pc = (2745, 14565);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 14567);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14570);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14571);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14572);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14573);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 14576)?;
                label = 14576;
                continue;
            }
            14576 => {
                c.pc = (2745, 14576);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14578);
                c.step()?;
                label = if c.z { 14597 } else { 14580 };
                continue;
            }
            14580 => {
                c.pc = (2745, 14580);
                c.step()?;
                c.invoke(p_0ab9_3098, false, 2745, 12440, 14583)?;
                label = 14583;
                continue;
            }
            14583 => {
                c.pc = (2745, 14583);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14585);
                c.step()?;
                label = if c.z { 14597 } else { 14587 };
                continue;
            }
            14587 => {
                c.pc = (2745, 14587);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53062u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14590);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14593);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 16, v)?;
                label = 14597;
                continue;
            }
            14597 => {
                c.pc = (2745, 14597);
                c.step()?;
                label = 14692;
                continue;
            }
            14599 => {
                c.pc = (2745, 14599);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 14600);
                c.step()?;
                c.invoke(p_0ab9_3663, false, 2745, 13923, 14603)?;
                label = 14603;
                continue;
            }
            14603 => {
                c.pc = (2745, 14603);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14605);
                c.step()?;
                label = if c.z { 14692 } else { 14607 };
                continue;
            }
            14607 => {
                c.pc = (2745, 14607);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53432u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 14612);
                c.step()?;
                label = if !c.z { 14624 } else { 14614 };
                continue;
            }
            14614 => {
                c.pc = (2745, 14614);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14617);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 14619);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 14622);
                c.step()?;
                label = if c.z { 14692 } else { 14624 };
                continue;
            }
            14624 => {
                c.pc = (2745, 14624);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 14626);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 14631)?;
                label = 14631;
                continue;
            }
            14631 => {
                c.pc = (2745, 14631);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 14633);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(14u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14636);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14640);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 14641);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 14643);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 14692 } else { 14645 };
                continue;
            }
            14645 => {
                c.pc = (2745, 14645);
                c.step()?;
                let v = 5u16;
                c.push(v)?;
                c.pc = (2745, 14647);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14650);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14651);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14652);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14653);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 14656)?;
                label = 14656;
                continue;
            }
            14656 => {
                c.pc = (2745, 14656);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14658);
                c.step()?;
                label = if !c.z { 14682 } else { 14660 };
                continue;
            }
            14660 => {
                c.pc = (2745, 14660);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14663);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 14664);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 14665);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 14666);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 14669)?;
                label = 14669;
                continue;
            }
            14669 => {
                c.pc = (2745, 14669);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 14670);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 14672);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 14675);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 14676);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 14678);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14680);
                c.step()?;
                label = if c.sf == c.of { 14692 } else { 14682 };
                continue;
            }
            14682 => {
                c.pc = (2745, 14682);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 14685);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14688);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 16, v)?;
                label = 14692;
                continue;
            }
            14692 => {
                c.pc = (2745, 14692);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 14696);
                c.step()?;
                label = if c.z { 14707 } else { 14698 };
                continue;
            }
            14698 => {
                c.pc = (2745, 14698);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 14701);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 14703);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(20u16), 16, v)?;
                label = 14707;
                continue;
            }
            14707 => {
                c.pc = (2745, 14707);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 14708);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_9f6e(c: &mut Cpu) -> Result<(), String> {
    let mut label = 40814u16;
    loop {
        match label {
            40814 => {
                c.pc = (2745, 40814);
                c.step()?;
                c.enter(10)?;
                c.pc = (2745, 40818);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 40821);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 40824);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 40828);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 40830);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 40833);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 40836);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 40840);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 40842);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 40845);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 40848);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 40851);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 40855);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 40857);
                c.step()?;
                let x = c.r[0];
                let y = 22u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 40860);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 40862);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(27267u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 40866);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 40868);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 40870);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 40873);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 40877);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 40879);
                c.step()?;
                let x = c.r[0];
                let y = 22u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 40882);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 40884);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(26959u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 40888);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 40890);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 40893);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 40896);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.multiply(v, false);
                c.pc = (2745, 40899);
                c.step()?;
                let x = c.r[0];
                let y = 5u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 40902);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 40905);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 40908);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 40909);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1b55_4f89(c: &mut Cpu) -> Result<(), String> {
    let mut label = 20361u16;
    loop {
        match label {
            20361 => {
                c.pc = (6997, 20361);
                c.step()?;
                c.enter(6)?;
                c.pc = (6997, 20365);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6997, 20369);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 20373);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 20375);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (6997, 20378);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (6997, 20381);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6997, 20385);
                c.step()?;
                label = if c.z { 20479 } else { 20387 };
                continue;
            }
            20387 => {
                c.pc = (6997, 20387);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6997, 20392);
                c.step()?;
                label = if c.sf != c.of { 20479 } else { 20394 };
                continue;
            }
            20394 => {
                c.pc = (6997, 20394);
                c.step()?;
                let v = 8u16;
                c.push(v)?;
                c.pc = (6997, 20396);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20400);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6997, 20402);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6997, 20403);
                c.step()?;
                c.invoke(p_24bc_0259, true, 9404, 601, 20408)?;
                label = 20408;
                continue;
            }
            20408 => {
                c.pc = (6997, 20408);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20410);
                c.step()?;
                label = if !c.z { 20479 } else { 20412 };
                continue;
            }
            20412 => {
                c.pc = (6997, 20412);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (6997, 20414);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 20417);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20421);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6997, 20423);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6997, 20424);
                c.step()?;
                c.invoke(p_24bc_0259, true, 9404, 601, 20429)?;
                label = 20429;
                continue;
            }
            20429 => {
                c.pc = (6997, 20429);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20431);
                c.step()?;
                label = if !c.z { 20479 } else { 20433 };
                continue;
            }
            20433 => {
                c.pc = (6997, 20433);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (6997, 20435);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 20438);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20442);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6997, 20444);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6997, 20445);
                c.step()?;
                c.invoke(p_24bc_0259, true, 9404, 601, 20450)?;
                label = 20450;
                continue;
            }
            20450 => {
                c.pc = (6997, 20450);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20452);
                c.step()?;
                label = if !c.z { 20479 } else { 20454 };
                continue;
            }
            20454 => {
                c.pc = (6997, 20454);
                c.step()?;
                let v = 3u16;
                c.push(v)?;
                c.pc = (6997, 20456);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 20459);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20463);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6997, 20465);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6997, 20466);
                c.step()?;
                c.invoke(p_24bc_0259, true, 9404, 601, 20471)?;
                label = 20471;
                continue;
            }
            20471 => {
                c.pc = (6997, 20471);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20473);
                c.step()?;
                label = if !c.z { 20479 } else { 20475 };
                continue;
            }
            20475 => {
                c.pc = (6997, 20475);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20477);
                c.step()?;
                label = 20481;
                continue;
            }
            20479 => {
                c.pc = (6997, 20479);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 20481;
                continue;
            }
            20481 => {
                c.pc = (6997, 20481);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (6997, 20484);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 20487);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6997, 20488);
                c.step()?;
                c.ret(true, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_232a_076d(c: &mut Cpu) -> Result<(), String> {
    let mut label = 1901u16;
    loop {
        match label {
            1901 => {
                c.pc = (9002, 1901);
                c.step()?;
                c.enter(2)?;
                c.pc = (9002, 1905);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (9002, 1908);
                c.step()?;
                c.invoke(p_24bc_0d01, true, 9404, 3329, 1913)?;
                label = 1913;
                continue;
            }
            1913 => {
                c.pc = (9002, 1913);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (9002, 1916);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (9002, 1919);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9002, 1920);
                c.step()?;
                c.ret(true, 2)?;
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
                let y = (c.r[1] >> 0) & 255;
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
                let v = (c.r[0] >> 0) & 255;
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
                let (v, g) = c.far(s, o)?;
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
pub(super) fn p_24bc_0d01(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3329u16;
    loop {
        match label {
            3329 => {
                c.pc = (9404, 3329);
                c.step()?;
                c.enter(2)?;
                c.pc = (9404, 3333);
                c.step()?;
                let o = 0u16.wrapping_add(56964u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9404, 3337);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (9404, 3338);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (9404, 3339);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (9404, 3342);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (9404, 3343);
                c.step()?;
                c.invoke(p_24bc_0cdc, false, 9404, 3292, 3346)?;
                label = 3346;
                continue;
            }
            3346 => {
                c.pc = (9404, 3346);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (9404, 3349);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (9404, 3352);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9404, 3353);
                c.step()?;
                c.ret(true, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_24bc_0d1c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3356u16;
    loop {
        match label {
            3356 => {
                c.pc = (9404, 3356);
                c.step()?;
                c.enter(2)?;
                c.pc = (9404, 3360);
                c.step()?;
                let o = 0u16.wrapping_add(56964u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9404, 3364);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (9404, 3365);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (9404, 3366);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (9404, 3369);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (9404, 3370);
                c.step()?;
                c.invoke(p_24bc_0cdc, false, 9404, 3292, 3373)?;
                label = 3373;
                continue;
            }
            3373 => {
                c.pc = (9404, 3373);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (9404, 3376);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9404, 3378);
                c.step()?;
                label = if !c.z { 3381 } else { 3380 };
                continue;
            }
            3380 => {
                c.pc = (9404, 3380);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 3381;
                continue;
            }
            3381 => {
                c.pc = (9404, 3381);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (9404, 3384);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9404, 3387);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9404, 3388);
                c.step()?;
                c.ret(true, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
