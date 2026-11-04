//! [confirmed] Mechanically compiled numeric control flow;F39/c7637bd,static d9e20a4.
//! [designed] Direct Rust calls/blocks;no runtime decoder or original executable.
use super::action_graph::*;
use super::compiled::Cpu;
pub(super) fn p_0ab9_4665(c: &mut Cpu) -> Result<(), String> {
    let mut label = 18021u16;
    loop {
        match label {
            18021 => {
                c.pc = (2745, 18021);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 18022);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 18024);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53431u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18027);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 18028);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 18030);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2712u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18034);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 18036);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 18037);
                c.step()?;
                c.invoke(p_24bc_0d1c, true, 9404, 3356, 18042)?;
                label = 18042;
                continue;
            }
            18042 => {
                c.pc = (2745, 18042);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18044);
                c.step()?;
                label = if c.z { 18128 } else { 18046 };
                continue;
            }
            18046 => {
                c.pc = (2745, 18046);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53435u16), 8)?;
                let y = 5u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 18051);
                c.step()?;
                label = if c.z { 18068 } else { 18053 };
                continue;
            }
            18053 => {
                c.pc = (2745, 18053);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                let y = 27u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 18057);
                c.step()?;
                label = if !c.z { 18070 } else { 18059 };
                continue;
            }
            18059 => {
                c.pc = (2745, 18059);
                c.step()?;
                c.invoke(p_24bc_0d3f, true, 9404, 3391, 18064)?;
                label = 18064;
                continue;
            }
            18064 => {
                c.pc = (2745, 18064);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18066);
                c.step()?;
                label = if c.z { 18070 } else { 18068 };
                continue;
            }
            18068 => {
                c.pc = (2745, 18068);
                c.step()?;
                label = 18128;
                continue;
            }
            18070 => {
                c.pc = (2745, 18070);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53071u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 18073);
                c.step()?;
                let x = c.r[0];
                let v = c.neg(x, 16);
                c.r[0] = v;
                c.pc = (2745, 18075);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53071u16), 16, v)?;
                c.pc = (2745, 18078);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53069u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 18081);
                c.step()?;
                let x = c.r[0];
                let v = c.neg(x, 16);
                c.r[0] = v;
                c.pc = (2745, 18083);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53069u16), 16, v)?;
                c.pc = (2745, 18086);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 18088);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 18090);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53073u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 18093);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 18096);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 18098);
                c.step()?;
                let v = c.r[2];
                c.push(v)?;
                c.pc = (2745, 18099);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 18100);
                c.step()?;
                c.invoke(p_24bc_04c0, true, 9404, 1216, 18105)?;
                label = 18105;
                continue;
            }
            18105 => {
                c.pc = (2745, 18105);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53073u16), 16, v)?;
                c.pc = (2745, 18108);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18111);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 18112);
                c.step()?;
                c.invoke(p_0ab9_1db7, false, 2745, 7607, 18115)?;
                label = 18115;
                continue;
            }
            18115 => {
                c.pc = (2745, 18115);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 18118);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 18123);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53431u16), 8, v)?;
                label = 18128;
                continue;
            }
            18128 => {
                c.pc = (2745, 18128);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 18129);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_46d4(c: &mut Cpu) -> Result<(), String> {
    let mut label = 18132u16;
    loop {
        match label {
            18132 => {
                c.pc = (2745, 18132);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 18136);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53432u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 18141);
                c.step()?;
                label = if !c.z { 18150 } else { 18143 };
                continue;
            }
            18143 => {
                c.pc = (2745, 18143);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53431u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 18148);
                c.step()?;
                label = if c.z { 18246 } else { 18150 };
                continue;
            }
            18150 => {
                c.pc = (2745, 18150);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 18154);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18157);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 18159);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 18160);
                c.step()?;
                c.invoke(p_0ab9_0000, false, 2745, 0, 18163)?;
                label = 18163;
                continue;
            }
            18163 => {
                c.pc = (2745, 18163);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18165);
                c.step()?;
                label = if !c.z { 18174 } else { 18167 };
                continue;
            }
            18167 => {
                c.pc = (2745, 18167);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53073u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 18172);
                c.step()?;
                label = if !c.z { 18176 } else { 18174 };
                continue;
            }
            18174 => {
                c.pc = (2745, 18174);
                c.step()?;
                label = 18246;
                continue;
            }
            18176 => {
                c.pc = (2745, 18176);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53063u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18179);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 18180);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 18181);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 18183);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 18185);
                c.step()?;
                let x = c.r[0];
                let y = 37u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 18188);
                c.step()?;
                label = if !c.z { 18246 } else { 18190 };
                continue;
            }
            18190 => {
                c.pc = (2745, 18190);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53062u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18193);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 18194);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 18195);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 18197);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 18199);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 18202);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 18246 } else { 18204 };
                continue;
            }
            18204 => {
                c.pc = (2745, 18204);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53062u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18207);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 18208);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 18209);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 18211);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 18213);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 18215);
                c.step()?;
                label = if c.z { 18232 } else { 18217 };
                continue;
            }
            18217 => {
                c.pc = (2745, 18217);
                c.step()?;
                let v = 4u16;
                c.push(v)?;
                c.pc = (2745, 18219);
                c.step()?;
                c.invoke(p_24bc_0d1c, true, 9404, 3356, 18224)?;
                label = 18224;
                continue;
            }
            18224 => {
                c.pc = (2745, 18224);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18226);
                c.step()?;
                label = if !c.z { 18232 } else { 18228 };
                continue;
            }
            18228 => {
                c.pc = (2745, 18228);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18230);
                c.step()?;
                label = 18234;
                continue;
            }
            18232 => {
                c.pc = (2745, 18232);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 18234;
                continue;
            }
            18234 => {
                c.pc = (2745, 18234);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 18235);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 18237);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2710u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18241);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 18242);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 18243);
                c.step()?;
                c.invoke(p_0ab9_4665, false, 2745, 18021, 18246)?;
                label = 18246;
                continue;
            }
            18246 => {
                c.pc = (2745, 18246);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 18249);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 18250);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_4f95(c: &mut Cpu) -> Result<(), String> {
    let mut label = 20373u16;
    loop {
        match label {
            20373 => {
                c.pc = (2745, 20373);
                c.step()?;
                c.enter(4)?;
                c.pc = (2745, 20377);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20380);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 20382);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 20383);
                c.step()?;
                c.invoke(p_0ab9_0000, false, 2745, 0, 20386)?;
                label = 20386;
                continue;
            }
            20386 => {
                c.pc = (2745, 20386);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20388);
                c.step()?;
                label = if c.z { 20457 } else { 20390 };
                continue;
            }
            20390 => {
                c.pc = (2745, 20390);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20393);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 20395);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 20398);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20402);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 20403);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 20404);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 20407);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 20410);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 20414);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 20415);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 20416);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 20417);
                c.step()?;
                c.invoke(p_0ab9_1f5e, false, 2745, 8030, 20420)?;
                label = 20420;
                continue;
            }
            20420 => {
                c.pc = (2745, 20420);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20423);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 16, v)?;
                c.pc = (2745, 20427);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 20430)?;
                label = 20430;
                continue;
            }
            20430 => {
                c.pc = (2745, 20430);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20431);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20433);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20437);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 20439);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 20440);
                c.step()?;
                let v = 10u16;
                c.push(v)?;
                c.pc = (2745, 20442);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 20447)?;
                label = 20447;
                continue;
            }
            20447 => {
                c.pc = (2745, 20447);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 20448);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 20449);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 20451);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53407u16), 8, v)?;
                c.pc = (2745, 20454);
                c.step()?;
                c.invoke(p_0ab9_1d3a, false, 2745, 7482, 20457)?;
                label = 20457;
                continue;
            }
            20457 => {
                c.pc = (2745, 20457);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20460);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 20462);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 20465);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20469);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 20470);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 20471);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 20474);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 20477);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20482);
                c.step()?;
                label = if c.z { 20493 } else { 20484 };
                continue;
            }
            20484 => {
                c.pc = (2745, 20484);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 16)?;
                c.r[0] = v;
                c.pc = (2745, 20487);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 16, v)?;
                c.pc = (2745, 20491);
                c.step()?;
                label = 20609;
                continue;
            }
            20493 => {
                c.pc = (2745, 20493);
                c.step()?;
                c.invoke(p_24bc_0d3f, true, 9404, 3391, 20498)?;
                label = 20498;
                continue;
            }
            20498 => {
                c.pc = (2745, 20498);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20500);
                c.step()?;
                label = if c.z { 20558 } else { 20502 };
                continue;
            }
            20502 => {
                c.pc = (2745, 20502);
                c.step()?;
                let v = 25u16;
                c.push(v)?;
                c.pc = (2745, 20504);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 20509)?;
                label = 20509;
                continue;
            }
            20509 => {
                c.pc = (2745, 20509);
                c.step()?;
                let x = c.r[0];
                let y = 10u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 20512);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 20513);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20516);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 20517);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 20518);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 20519);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 20522)?;
                label = 20522;
                continue;
            }
            20522 => {
                c.pc = (2745, 20522);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20523);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 20524);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 20526);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20529);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(9u16), 8, v)?;
                c.pc = (2745, 20533);
                c.step()?;
                c.invoke(p_24bc_0d3f, true, 9404, 3391, 20538)?;
                label = 20538;
                continue;
            }
            20538 => {
                c.pc = (2745, 20538);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20539);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20541);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23752u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20545);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20546);
                c.step()?;
                let x = c.r[0];
                let y = 7u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 20549);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20552);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8, v)?;
                c.pc = (2745, 20556);
                c.step()?;
                label = 20609;
                continue;
            }
            20558 => {
                c.pc = (2745, 20558);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20561);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 20562);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 20563);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 20564);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 20567)?;
                label = 20567;
                continue;
            }
            20567 => {
                c.pc = (2745, 20567);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20568);
                c.step()?;
                let x = c.r[0];
                let y = 36u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 20571);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20574);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(9u16), 8, v)?;
                c.pc = (2745, 20578);
                c.step()?;
                let v = 5u16;
                c.push(v)?;
                c.pc = (2745, 20580);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 20585)?;
                label = 20585;
                continue;
            }
            20585 => {
                c.pc = (2745, 20585);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 20586);
                c.step()?;
                c.invoke(p_24bc_0d3f, true, 9404, 3391, 20591)?;
                label = 20591;
                continue;
            }
            20591 => {
                c.pc = (2745, 20591);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20592);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20594);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23752u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20598);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20599);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 20600);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 20602);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20605);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8, v)?;
                label = 20609;
                continue;
            }
            20609 => {
                c.pc = (2745, 20609);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 20610);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_5083(c: &mut Cpu) -> Result<(), String> {
    let mut label = 20611u16;
    loop {
        match label {
            20611 => {
                c.pc = (2745, 20611);
                c.step()?;
                c.enter(16)?;
                c.pc = (2745, 20615);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 20619);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20623);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 20624);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 20625);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 20628);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 20631);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20636);
                c.step()?;
                label = if c.z { 20645 } else { 20638 };
                continue;
            }
            20638 => {
                c.pc = (2745, 20638);
                c.step()?;
                let v = 39u16;
                c.push(v)?;
                c.pc = (2745, 20640);
                c.step()?;
                c.invoke(p_0ab9_1db7, false, 2745, 7607, 20643)?;
                label = 20643;
                continue;
            }
            20643 => {
                c.pc = (2745, 20643);
                c.step()?;
                label = 20716;
                continue;
            }
            20645 => {
                c.pc = (2745, 20645);
                c.step()?;
                let v = 38u16;
                c.push(v)?;
                c.pc = (2745, 20647);
                c.step()?;
                c.invoke(p_0ab9_1db7, false, 2745, 7607, 20650)?;
                label = 20650;
                continue;
            }
            20650 => {
                c.pc = (2745, 20650);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(55005u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20655);
                c.step()?;
                label = if c.z { 20716 } else { 20657 };
                continue;
            }
            20657 => {
                c.pc = (2745, 20657);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20660);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 20664);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 20667);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20671);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20673);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20678);
                c.step()?;
                label = if c.z { 20716 } else { 20680 };
                continue;
            }
            20680 => {
                c.pc = (2745, 20680);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20683);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 20688);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20692);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20694);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 16, v)?;
                c.pc = (2745, 20698);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20701);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 20706);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20710);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20712);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(5u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(5u16), 8, v)?;
                label = 20716;
                continue;
            }
            20716 => {
                c.pc = (2745, 20716);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20721);
                c.step()?;
                label = if c.z { 20738 } else { 20723 };
                continue;
            }
            20723 => {
                c.pc = (2745, 20723);
                c.step()?;
                let v = 65535u16;
                c.push(v)?;
                c.pc = (2745, 20725);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 20727);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20730);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 20731);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 20732);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 20733);
                c.step()?;
                c.invoke(p_0ab9_67d0, false, 2745, 26576, 20736)?;
                label = 20736;
                continue;
            }
            20736 => {
                c.pc = (2745, 20736);
                c.step()?;
                label = 20779;
                continue;
            }
            20738 => {
                c.pc = (2745, 20738);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20741);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 20745);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 20748);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20752);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20754);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20758);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 20760);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20762);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3354u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20766);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20767);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 20768);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 20770);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20773);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 20774);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 20775);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 20776);
                c.step()?;
                c.invoke(p_0ab9_67d0, false, 2745, 26576, 20779)?;
                label = 20779;
                continue;
            }
            20779 => {
                c.pc = (2745, 20779);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20783);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20785);
                c.step()?;
                label = if !c.z { 20788 } else { 20787 };
                continue;
            }
            20787 => {
                c.pc = (2745, 20787);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 20788;
                continue;
            }
            20788 => {
                c.pc = (2745, 20788);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20789);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20791);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20793);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 20798);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20802);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20804);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(62u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 20808);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 20811);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 20814);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 20817);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20821);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20823);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(12u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 20826);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 20829);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 20833);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20837);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20839);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(16u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 20842);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(14u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 20845);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(55005u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20850);
                c.step()?;
                label = if c.z { 20873 } else { 20852 };
                continue;
            }
            20852 => {
                c.pc = (2745, 20852);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(12u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20855);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20860);
                c.step()?;
                label = if !c.z { 20873 } else { 20862 };
                continue;
            }
            20862 => {
                c.pc = (2745, 20862);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(16u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 20865);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(5u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(5u16), 8, v)?;
                c.pc = (2745, 20869);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 16, v)?;
                label = 20873;
                continue;
            }
            20873 => {
                c.pc = (2745, 20873);
                c.step()?;
                let v = 65535u16;
                c.push(v)?;
                c.pc = (2745, 20875);
                c.step()?;
                c.invoke(p_24bc_0d3f, true, 9404, 3391, 20880)?;
                label = 20880;
                continue;
            }
            20880 => {
                c.pc = (2745, 20880);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 20881);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20885);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20887);
                c.step()?;
                label = if !c.z { 20890 } else { 20889 };
                continue;
            }
            20889 => {
                c.pc = (2745, 20889);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 20890;
                continue;
            }
            20890 => {
                c.pc = (2745, 20890);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20891);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20893);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20897);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 20899);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 20900);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 20903);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20907);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 20908);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 20909);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 20910);
                c.step()?;
                c.invoke(p_0ab9_67d0, false, 2745, 26576, 20913)?;
                label = 20913;
                continue;
            }
            20913 => {
                c.pc = (2745, 20913);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20916);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8, v)?;
                c.pc = (2745, 20919);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20924);
                c.step()?;
                label = if c.z { 20938 } else { 20926 };
                continue;
            }
            20926 => {
                c.pc = (2745, 20926);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20930);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20932);
                c.step()?;
                label = if !c.z { 20935 } else { 20934 };
                continue;
            }
            20934 => {
                c.pc = (2745, 20934);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 20935;
                continue;
            }
            20935 => {
                c.pc = (2745, 20935);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8, v)?;
                label = 20938;
                continue;
            }
            20938 => {
                c.pc = (2745, 20938);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20941);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20942);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20944);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(11992u16), 8)?;
                let y = 11u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 20949);
                c.step()?;
                label = if !c.cf { 20961 } else { 20951 };
                continue;
            }
            20951 => {
                c.pc = (2745, 20951);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20954);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20955);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20957);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(11992u16), 8)?;
                let v = c.inc(x, 8);
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11992u16),
                    8,
                    v,
                )?;
                label = 20961;
                continue;
            }
            20961 => {
                c.pc = (2745, 20961);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20964);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20965);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 20967);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(11992u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20971);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 20973);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (2745, 20975);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 20977);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 20979);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 20981);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 20984);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 20985);
                c.step()?;
                let x = c.r[0];
                let y = 33u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 20988);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20990);
                c.step()?;
                let x = c.r[7];
                let y = 53475u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 20994);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 20995);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 20996);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(12u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 20999);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 21002);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21005);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21006);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21007);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21008);
                c.step()?;
                c.invoke(p_0ab9_1e9f, false, 2745, 7839, 21011)?;
                label = 21011;
                continue;
            }
            21011 => {
                c.pc = (2745, 21011);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21013);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 21015);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 21017);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21020);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21021);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21023);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21025);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 21030);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21034);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21036);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21038);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(64u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 21042);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(12u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21045);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]), 16, v)?;
                c.pc = (2745, 21048);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21051);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8, v)?;
                c.pc = (2745, 21055);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53431u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21060);
                c.step()?;
                label = if !c.z { 21069 } else { 21062 };
                continue;
            }
            21062 => {
                c.pc = (2745, 21062);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21067);
                c.step()?;
                label = if c.z { 21077 } else { 21069 };
                continue;
            }
            21069 => {
                c.pc = (2745, 21069);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(12u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21072);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                let y = 128u16;
                let v = c.add(x, y, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8, v)?;
                label = 21077;
                continue;
            }
            21077 => {
                c.pc = (2745, 21077);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21082);
                c.step()?;
                label = if !c.z { 21091 } else { 21084 };
                continue;
            }
            21084 => {
                c.pc = (2745, 21084);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21087);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21088);
                c.step()?;
                c.invoke(p_0ab9_2cb4, false, 2745, 11444, 21091)?;
                label = 21091;
                continue;
            }
            21091 => {
                c.pc = (2745, 21091);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 21092);
                c.step()?;
                c.ret(false, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_5267(c: &mut Cpu) -> Result<(), String> {
    let mut label = 21095u16;
    loop {
        match label {
            21095 => {
                c.pc = (2745, 21095);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 21099);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 21104);
                c.step()?;
                label = 21109;
                continue;
            }
            21106 => {
                c.pc = (2745, 21106);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 21109;
                continue;
            }
            21109 => {
                c.pc = (2745, 21109);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 21113);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21117);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21118);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 21119);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 21122);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 21125);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21126);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21127);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21128);
                c.step()?;
                c.invoke(p_0ab9_1e4e, false, 2745, 7758, 21131)?;
                label = 21131;
                continue;
            }
            21131 => {
                c.pc = (2745, 21131);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21133);
                c.step()?;
                label = if c.z { 21155 } else { 21135 };
                continue;
            }
            21135 => {
                c.pc = (2745, 21135);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 21138);
                c.step()?;
                c.invoke(p_0ab9_0000, false, 2745, 0, 21141)?;
                label = 21141;
                continue;
            }
            21141 => {
                c.pc = (2745, 21141);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21143);
                c.step()?;
                label = if !c.z { 21155 } else { 21145 };
                continue;
            }
            21145 => {
                c.pc = (2745, 21145);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21148);
                c.step()?;
                let v = 6u16;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(19u16), 8, v)?;
                c.pc = (2745, 21153);
                c.step()?;
                label = 21163;
                continue;
            }
            21155 => {
                c.pc = (2745, 21155);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21158);
                c.step()?;
                let v = 0u16;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(19u16), 8, v)?;
                label = 21163;
                continue;
            }
            21163 => {
                c.pc = (2745, 21163);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 21167);
                c.step()?;
                label = if !c.z { 21106 } else { 21169 };
                continue;
            }
            21169 => {
                c.pc = (2745, 21169);
                c.step()?;
                c.invoke(p_0ab9_4f95, false, 2745, 20373, 21172)?;
                label = 21172;
                continue;
            }
            21172 => {
                c.pc = (2745, 21172);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 21173);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_52b6(c: &mut Cpu) -> Result<(), String> {
    let mut label = 21174u16;
    loop {
        match label {
            21174 => {
                c.pc = (2745, 21174);
                c.step()?;
                c.enter(8)?;
                c.pc = (2745, 21178);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53432u16), 8, v)?;
                c.pc = (2745, 21183);
                c.step()?;
                let v = 4u16;
                c.push(v)?;
                c.pc = (2745, 21185);
                c.step()?;
                c.invoke(p_1998_01e0, true, 6552, 480, 21190)?;
                label = 21190;
                continue;
            }
            21190 => {
                c.pc = (2745, 21190);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 21195);
                c.step()?;
                label = 21200;
                continue;
            }
            21197 => {
                c.pc = (2745, 21197);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 21200;
                continue;
            }
            21200 => {
                c.pc = (2745, 21200);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 21204);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(13073u16),
                    16,
                )?;
                c.r[0] = v;
                c.pc = (2745, 21208);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 21212);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(13065u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 21216);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 21220);
                c.step()?;
                label = if !c.z { 21197 } else { 21222 };
                continue;
            }
            21222 => {
                c.pc = (2745, 21222);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21225);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8, v)?;
                c.pc = (2745, 21228);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21233);
                c.step()?;
                label = if c.z { 21238 } else { 21235 };
                continue;
            }
            21235 => {
                c.pc = (2745, 21235);
                c.step()?;
                label = 21370;
                continue;
            }
            21238 => {
                c.pc = (2745, 21238);
                c.step()?;
                c.invoke(p_0ab9_5267, false, 2745, 21095, 21241)?;
                label = 21241;
                continue;
            }
            21241 => {
                c.pc = (2745, 21241);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21245);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21247);
                c.step()?;
                label = if !c.z { 21250 } else { 21249 };
                continue;
            }
            21249 => {
                c.pc = (2745, 21249);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 21250;
                continue;
            }
            21250 => {
                c.pc = (2745, 21250);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21251);
                c.step()?;
                c.invoke(p_0ab9_a4f2, false, 2745, 42226, 21254)?;
                label = 21254;
                continue;
            }
            21254 => {
                c.pc = (2745, 21254);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 21259);
                c.step()?;
                label = 21264;
                continue;
            }
            21261 => {
                c.pc = (2745, 21261);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 21264;
                continue;
            }
            21264 => {
                c.pc = (2745, 21264);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 21268);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21272);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21273);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 21274);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 21277);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 21280);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21281);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21282);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21283);
                c.step()?;
                c.invoke(p_0ab9_1ca1, false, 2745, 7329, 21286)?;
                label = 21286;
                continue;
            }
            21286 => {
                c.pc = (2745, 21286);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21288);
                c.step()?;
                label = if !c.z { 21364 } else { 21290 };
                continue;
            }
            21290 => {
                c.pc = (2745, 21290);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21295);
                c.step()?;
                label = if !c.z { 21328 } else { 21297 };
                continue;
            }
            21297 => {
                c.pc = (2745, 21297);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21300);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21301);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21302);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21303);
                c.step()?;
                c.invoke(p_0ab9_1e4e, false, 2745, 7758, 21306)?;
                label = 21306;
                continue;
            }
            21306 => {
                c.pc = (2745, 21306);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21307);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21309);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23752u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21313);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21314);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21315);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 21317);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21320);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21321);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21322);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21323);
                c.step()?;
                c.invoke(p_0ab9_67d0, false, 2745, 26576, 21326)?;
                label = 21326;
                continue;
            }
            21326 => {
                c.pc = (2745, 21326);
                c.step()?;
                label = 21364;
                continue;
            }
            21328 => {
                c.pc = (2745, 21328);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21331);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21332);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21333);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21334);
                c.step()?;
                c.invoke(p_0ab9_1e4e, false, 2745, 7758, 21337)?;
                label = 21337;
                continue;
            }
            21337 => {
                c.pc = (2745, 21337);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21339);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21341);
                c.step()?;
                label = if !c.z { 21344 } else { 21343 };
                continue;
            }
            21343 => {
                c.pc = (2745, 21343);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 21344;
                continue;
            }
            21344 => {
                c.pc = (2745, 21344);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21345);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21347);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23752u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21351);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21352);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21353);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 21355);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21358);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21359);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21360);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21361);
                c.step()?;
                c.invoke(p_0ab9_67d0, false, 2745, 26576, 21364)?;
                label = 21364;
                continue;
            }
            21364 => {
                c.pc = (2745, 21364);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 21368);
                c.step()?;
                label = if !c.z { 21261 } else { 21370 };
                continue;
            }
            21370 => {
                c.pc = (2745, 21370);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 21371);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_537c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 21372u16;
    loop {
        match label {
            21372 => {
                c.pc = (2745, 21372);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 21373);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 21375);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53063u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21380);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21382);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 21385 } else { 21384 };
                continue;
            }
            21384 => {
                c.pc = (2745, 21384);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 21385;
                continue;
            }
            21385 => {
                c.pc = (2745, 21385);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21386);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21388);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23752u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21392);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21393);
                c.step()?;
                let x = c.r[0];
                let y = 39u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 21396);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53063u16), 8, v)?;
                c.pc = (2745, 21399);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53062u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21404);
                c.step()?;
                label = if !c.z { 21413 } else { 21406 };
                continue;
            }
            21406 => {
                c.pc = (2745, 21406);
                c.step()?;
                let v = 40u16;
                c.push(v)?;
                c.pc = (2745, 21408);
                c.step()?;
                c.invoke(p_0ab9_1db7, false, 2745, 7607, 21411)?;
                label = 21411;
                continue;
            }
            21411 => {
                c.pc = (2745, 21411);
                c.step()?;
                label = 21418;
                continue;
            }
            21413 => {
                c.pc = (2745, 21413);
                c.step()?;
                let v = 41u16;
                c.push(v)?;
                c.pc = (2745, 21415);
                c.step()?;
                c.invoke(p_0ab9_1db7, false, 2745, 7607, 21418)?;
                label = 21418;
                continue;
            }
            21418 => {
                c.pc = (2745, 21418);
                c.step()?;
                let v = 20u16;
                c.write(c.r[11], 0u16.wrapping_add(53438u16), 16, v)?;
                c.pc = (2745, 21424);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53409u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21429);
                c.step()?;
                label = if c.z { 21438 } else { 21431 };
                continue;
            }
            21431 => {
                c.pc = (2745, 21431);
                c.step()?;
                c.invoke(p_0ab9_6d83, false, 2745, 28035, 21434)?;
                label = 21434;
                continue;
            }
            21434 => {
                c.pc = (2745, 21434);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21436);
                c.step()?;
                label = if c.z { 21456 } else { 21438 };
                continue;
            }
            21438 => {
                c.pc = (2745, 21438);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 21441)?;
                label = 21441;
                continue;
            }
            21441 => {
                c.pc = (2745, 21441);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21443);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21445);
                c.step()?;
                label = if !c.z { 21448 } else { 21447 };
                continue;
            }
            21447 => {
                c.pc = (2745, 21447);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 21448;
                continue;
            }
            21448 => {
                c.pc = (2745, 21448);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21449);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 21451);
                c.step()?;
                c.invoke(p_0ab9_5a81, false, 2745, 23169, 21454)?;
                label = 21454;
                continue;
            }
            21454 => {
                c.pc = (2745, 21454);
                c.step()?;
                label = 21459;
                continue;
            }
            21456 => {
                c.pc = (2745, 21456);
                c.step()?;
                c.invoke(p_0ab9_6dc0, false, 2745, 28096, 21459)?;
                label = 21459;
                continue;
            }
            21459 => {
                c.pc = (2745, 21459);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 21460);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_53d7(c: &mut Cpu) -> Result<(), String> {
    let mut label = 21463u16;
    loop {
        match label {
            21463 => {
                c.pc = (2745, 21463);
                c.step()?;
                c.enter(8)?;
                c.pc = (2745, 21467);
                c.step()?;
                let v = 53063u16;
                c.r[7] = v;
                c.pc = (2745, 21470);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21471);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21472);
                c.step()?;
                let v = 65498u16;
                c.push(v)?;
                c.pc = (2745, 21474);
                c.step()?;
                let v = 38u16;
                c.push(v)?;
                c.pc = (2745, 21476);
                c.step()?;
                c.invoke(p_24bc_0402, true, 9404, 1026, 21481)?;
                label = 21481;
                continue;
            }
            21481 => {
                c.pc = (2745, 21481);
                c.step()?;
                let v = 53062u16;
                c.r[7] = v;
                c.pc = (2745, 21484);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21485);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21486);
                c.step()?;
                let v = 65535u16;
                c.push(v)?;
                c.pc = (2745, 21488);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 21490);
                c.step()?;
                c.invoke(p_24bc_0402, true, 9404, 1026, 21495)?;
                label = 21495;
                continue;
            }
            21495 => {
                c.pc = (2745, 21495);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21500);
                c.step()?;
                label = if c.z { 21517 } else { 21502 };
                continue;
            }
            21502 => {
                c.pc = (2745, 21502);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 21505)?;
                label = 21505;
                continue;
            }
            21505 => {
                c.pc = (2745, 21505);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21507);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21509);
                c.step()?;
                label = if !c.z { 21512 } else { 21511 };
                continue;
            }
            21511 => {
                c.pc = (2745, 21511);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 21512;
                continue;
            }
            21512 => {
                c.pc = (2745, 21512);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 8, v)?;
                c.pc = (2745, 21515);
                c.step()?;
                label = 21523;
                continue;
            }
            21517 => {
                c.pc = (2745, 21517);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 21520)?;
                label = 21520;
                continue;
            }
            21520 => {
                c.pc = (2745, 21520);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 8, v)?;
                label = 21523;
                continue;
            }
            21523 => {
                c.pc = (2745, 21523);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21528);
                c.step()?;
                label = if c.z { 21536 } else { 21530 };
                continue;
            }
            21530 => {
                c.pc = (2745, 21530);
                c.step()?;
                let v = 39u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8, v)?;
                c.pc = (2745, 21534);
                c.step()?;
                label = 21540;
                continue;
            }
            21536 => {
                c.pc = (2745, 21536);
                c.step()?;
                let v = 38u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8, v)?;
                label = 21540;
                continue;
            }
            21540 => {
                c.pc = (2745, 21540);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21543);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21544);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21546);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16), 8)?;
                let v = c.inc(x, 8);
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 21550);
                c.step()?;
                let v = 56664u16;
                c.r[7] = v;
                c.pc = (2745, 21553);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21554);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21555);
                c.step()?;
                c.invoke(p_1998_02df, true, 6552, 735, 21560)?;
                label = 21560;
                continue;
            }
            21560 => {
                c.pc = (2745, 21560);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21564);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21566);
                c.step()?;
                label = if !c.z { 21569 } else { 21568 };
                continue;
            }
            21568 => {
                c.pc = (2745, 21568);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 21569;
                continue;
            }
            21569 => {
                c.pc = (2745, 21569);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21570);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21572);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21576);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21578);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 21579);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 21582);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21586);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21587);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 21588);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 21591);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 21594);
                c.step()?;
                c.invoke(p_24bc_0d3f, true, 9404, 3391, 21599)?;
                label = 21599;
                continue;
            }
            21599 => {
                c.pc = (2745, 21599);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21601);
                c.step()?;
                label = if c.z { 21616 } else { 21603 };
                continue;
            }
            21603 => {
                c.pc = (2745, 21603);
                c.step()?;
                let v = 36u16;
                c.push(v)?;
                c.pc = (2745, 21605);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21608);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21609);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21610);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21611);
                c.step()?;
                c.invoke(p_0ab9_1e28, false, 2745, 7720, 21614)?;
                label = 21614;
                continue;
            }
            21614 => {
                c.pc = (2745, 21614);
                c.step()?;
                label = 21627;
                continue;
            }
            21616 => {
                c.pc = (2745, 21616);
                c.step()?;
                let v = 37u16;
                c.push(v)?;
                c.pc = (2745, 21618);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21621);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 21622);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21623);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21624);
                c.step()?;
                c.invoke(p_0ab9_1e28, false, 2745, 7720, 21627)?;
                label = 21627;
                continue;
            }
            21627 => {
                c.pc = (2745, 21627);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21632);
                c.step()?;
                label = if c.z { 21643 } else { 21634 };
                continue;
            }
            21634 => {
                c.pc = (2745, 21634);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21637);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21638);
                c.step()?;
                c.invoke(p_0ab9_1db7, false, 2745, 7607, 21641)?;
                label = 21641;
                continue;
            }
            21641 => {
                c.pc = (2745, 21641);
                c.step()?;
                label = 21723;
                continue;
            }
            21643 => {
                c.pc = (2745, 21643);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53409u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21648);
                c.step()?;
                label = if c.z { 21657 } else { 21650 };
                continue;
            }
            21650 => {
                c.pc = (2745, 21650);
                c.step()?;
                c.invoke(p_0ab9_6d83, false, 2745, 28035, 21653)?;
                label = 21653;
                continue;
            }
            21653 => {
                c.pc = (2745, 21653);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21655);
                c.step()?;
                label = if c.z { 21691 } else { 21657 };
                continue;
            }
            21657 => {
                c.pc = (2745, 21657);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21660);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21662);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 21665);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21669);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21670);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21671);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21672);
                c.step()?;
                c.invoke(p_0ab9_1cbb, false, 2745, 7355, 21675)?;
                label = 21675;
                continue;
            }
            21675 => {
                c.pc = (2745, 21675);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21676);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21679);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21681);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21682);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53426u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 21686);
                c.step()?;
                c.invoke(p_0ab9_5083, false, 2745, 20611, 21689)?;
                label = 21689;
                continue;
            }
            21689 => {
                c.pc = (2745, 21689);
                c.step()?;
                label = 21723;
                continue;
            }
            21691 => {
                c.pc = (2745, 21691);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21694);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21695);
                c.step()?;
                c.invoke(p_0ab9_1db7, false, 2745, 7607, 21698)?;
                label = 21698;
                continue;
            }
            21698 => {
                c.pc = (2745, 21698);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21701);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21702);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21704);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16), 8)?;
                let v = c.dec(x, 8);
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 21708);
                c.step()?;
                c.invoke(p_0ab9_6dc0, false, 2745, 28096, 21711)?;
                label = 21711;
                continue;
            }
            21711 => {
                c.pc = (2745, 21711);
                c.step()?;
                let v = 56664u16;
                c.r[7] = v;
                c.pc = (2745, 21714);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21715);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21716);
                c.step()?;
                c.invoke(p_1998_02df, true, 6552, 735, 21721)?;
                label = 21721;
                continue;
            }
            21721 => {
                c.pc = (2745, 21721);
                c.step()?;
                label = 21726;
                continue;
            }
            21723 => {
                c.pc = (2745, 21723);
                c.step()?;
                c.invoke(p_0ab9_52b6, false, 2745, 21174, 21726)?;
                label = 21726;
                continue;
            }
            21726 => {
                c.pc = (2745, 21726);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 21727);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_54e2(c: &mut Cpu) -> Result<(), String> {
    let mut label = 21730u16;
    loop {
        match label {
            21730 => {
                c.pc = (2745, 21730);
                c.step()?;
                c.enter(8)?;
                c.pc = (2745, 21734);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53435u16), 8)?;
                let y = 4u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21739);
                c.step()?;
                label = if !c.cf && !c.z { 21765 } else { 21741 };
                continue;
            }
            21741 => {
                c.pc = (2745, 21741);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 21744)?;
                label = 21744;
                continue;
            }
            21744 => {
                c.pc = (2745, 21744);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21745);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21747);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16), 8)?;
                let y = 15u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21752);
                c.step()?;
                label = if !c.z { 21761 } else { 21754 };
                continue;
            }
            21754 => {
                c.pc = (2745, 21754);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21759);
                c.step()?;
                label = if c.z { 21765 } else { 21761 };
                continue;
            }
            21761 => {
                c.pc = (2745, 21761);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21763);
                c.step()?;
                label = 21767;
                continue;
            }
            21765 => {
                c.pc = (2745, 21765);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 21767;
                continue;
            }
            21767 => {
                c.pc = (2745, 21767);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8, v)?;
                c.pc = (2745, 21770);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21774);
                c.step()?;
                label = if !c.z { 21889 } else { 21776 };
                continue;
            }
            21776 => {
                c.pc = (2745, 21776);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55014u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 21780);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21784);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21787);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21789);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21790);
                c.step()?;
                c.invoke(p_1998_0273, true, 6552, 627, 21795)?;
                label = 21795;
                continue;
            }
            21795 => {
                c.pc = (2745, 21795);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21797);
                c.step()?;
                label = if c.z { 21889 } else { 21799 };
                continue;
            }
            21799 => {
                c.pc = (2745, 21799);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 21802)?;
                label = 21802;
                continue;
            }
            21802 => {
                c.pc = (2745, 21802);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21803);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21805);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21807);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 21811);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21815);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21818);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21820);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21821);
                c.step()?;
                c.invoke(p_1998_022b, true, 6552, 555, 21826)?;
                label = 21826;
                continue;
            }
            21826 => {
                c.pc = (2745, 21826);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21827);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 21831);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21834);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21836);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21837);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                c.pc = (2745, 21840);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 21841);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21842);
                c.step()?;
                c.invoke(p_1b55_17f9, true, 6997, 6137, 21847)?;
                label = 21847;
                continue;
            }
            21847 => {
                c.pc = (2745, 21847);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 21850)?;
                label = 21850;
                continue;
            }
            21850 => {
                c.pc = (2745, 21850);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 21851);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21853);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21857);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21859);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 21861);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21864);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21866);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 21868);
                c.step()?;
                let x = c.r[0];
                let y = 15u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 21871);
                c.step()?;
                label = if !c.z { 21880 } else { 21873 };
                continue;
            }
            21873 => {
                c.pc = (2745, 21873);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21878);
                c.step()?;
                label = if c.z { 21884 } else { 21880 };
                continue;
            }
            21880 => {
                c.pc = (2745, 21880);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21882);
                c.step()?;
                label = 21886;
                continue;
            }
            21884 => {
                c.pc = (2745, 21884);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 21886;
                continue;
            }
            21886 => {
                c.pc = (2745, 21886);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8, v)?;
                label = 21889;
                continue;
            }
            21889 => {
                c.pc = (2745, 21889);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21893);
                c.step()?;
                label = if c.z { 21901 } else { 21895 };
                continue;
            }
            21895 => {
                c.pc = (2745, 21895);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 21896);
                c.step()?;
                c.invoke(p_0ab9_537c, false, 2745, 21372, 21899)?;
                label = 21899;
                continue;
            }
            21899 => {
                c.pc = (2745, 21899);
                c.step()?;
                label = 21987;
                continue;
            }
            21901 => {
                c.pc = (2745, 21901);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53706u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21906);
                c.step()?;
                label = if !c.z { 21977 } else { 21908 };
                continue;
            }
            21908 => {
                c.pc = (2745, 21908);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53705u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21913);
                c.step()?;
                label = if !c.z { 21977 } else { 21915 };
                continue;
            }
            21915 => {
                c.pc = (2745, 21915);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53435u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21918);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 3u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21920);
                c.step()?;
                label = if c.cf { 21983 } else { 21922 };
                continue;
            }
            21922 => {
                c.pc = (2745, 21922);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 4u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 21924);
                c.step()?;
                label = if !c.cf && !c.z { 21983 } else { 21926 };
                continue;
            }
            21926 => {
                c.pc = (2745, 21926);
                c.step()?;
                let v = 32u16;
                c.push(v)?;
                c.pc = (2745, 21928);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 21933)?;
                label = 21933;
                continue;
            }
            21933 => {
                c.pc = (2745, 21933);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21934);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53435u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21937);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21939);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 21941);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2724u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21945);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21947);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21948);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 21951);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 21953);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 21956);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 21960);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 21961);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 21962);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 21963);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 21966)?;
                label = 21966;
                continue;
            }
            21966 => {
                c.pc = (2745, 21966);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 21967);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 21972)?;
                label = 21972;
                continue;
            }
            21972 => {
                c.pc = (2745, 21972);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 21973);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 21975);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 21983 } else { 21977 };
                continue;
            }
            21977 => {
                c.pc = (2745, 21977);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 21978);
                c.step()?;
                c.invoke(p_0ab9_53d7, false, 2745, 21463, 21981)?;
                label = 21981;
                continue;
            }
            21981 => {
                c.pc = (2745, 21981);
                c.step()?;
                label = 21987;
                continue;
            }
            21983 => {
                c.pc = (2745, 21983);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 21984);
                c.step()?;
                c.invoke(p_0ab9_537c, false, 2745, 21372, 21987)?;
                label = 21987;
                continue;
            }
            21987 => {
                c.pc = (2745, 21987);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53431u16), 8, v)?;
                c.pc = (2745, 21992);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53705u16), 8, v)?;
                c.pc = (2745, 21997);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 21998);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_55ef(c: &mut Cpu) -> Result<(), String> {
    let mut label = 21999u16;
    loop {
        match label {
            21999 => {
                c.pc = (2745, 21999);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 22000);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 22002);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 22005);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 22007);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 22008);
                c.step()?;
                c.invoke(p_0ab9_0000, false, 2745, 0, 22011)?;
                label = 22011;
                continue;
            }
            22011 => {
                c.pc = (2745, 22011);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 22013);
                c.step()?;
                label = if !c.z { 22026 } else { 22015 };
                continue;
            }
            22015 => {
                c.pc = (2745, 22015);
                c.step()?;
                let v = 3u16;
                c.push(v)?;
                c.pc = (2745, 22017);
                c.step()?;
                c.invoke(p_24bc_0d1c, true, 9404, 3356, 22022)?;
                label = 22022;
                continue;
            }
            22022 => {
                c.pc = (2745, 22022);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 22024);
                c.step()?;
                label = if !c.z { 22031 } else { 22026 };
                continue;
            }
            22026 => {
                c.pc = (2745, 22026);
                c.step()?;
                c.invoke(p_0ab9_5a06, false, 2745, 23046, 22029)?;
                label = 22029;
                continue;
            }
            22029 => {
                c.pc = (2745, 22029);
                c.step()?;
                label = 22044;
                continue;
            }
            22031 => {
                c.pc = (2745, 22031);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53435u16), 8, v)?;
                c.pc = (2745, 22036);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53706u16), 8, v)?;
                c.pc = (2745, 22041);
                c.step()?;
                c.invoke(p_0ab9_54e2, false, 2745, 21730, 22044)?;
                label = 22044;
                continue;
            }
            22044 => {
                c.pc = (2745, 22044);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 22045);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_59b7(c: &mut Cpu) -> Result<(), String> {
    let mut label = 22967u16;
    loop {
        match label {
            22967 => {
                c.pc = (2745, 22967);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 22968);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 22970);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 22975);
                c.step()?;
                label = if !c.z { 23044 } else { 22977 };
                continue;
            }
            22977 => {
                c.pc = (2745, 22977);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53409u16), 8, v)?;
                c.pc = (2745, 22982);
                c.step()?;
                c.invoke(p_0ab9_6b2f, false, 2745, 27439, 22985)?;
                label = 22985;
                continue;
            }
            22985 => {
                c.pc = (2745, 22985);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 22988)?;
                label = 22988;
                continue;
            }
            22988 => {
                c.pc = (2745, 22988);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 22990);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 22992);
                c.step()?;
                label = if !c.z { 22995 } else { 22994 };
                continue;
            }
            22994 => {
                c.pc = (2745, 22994);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 22995;
                continue;
            }
            22995 => {
                c.pc = (2745, 22995);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 22996);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 22998);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12124u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23002);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 23004);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 23005);
                c.step()?;
                let v = 7u16;
                c.push(v)?;
                c.pc = (2745, 23007);
                c.step()?;
                c.invoke(p_0ab9_57e3, false, 2745, 22499, 23010)?;
                label = 23010;
                continue;
            }
            23010 => {
                c.pc = (2745, 23010);
                c.step()?;
                let v = 43u16;
                c.push(v)?;
                c.pc = (2745, 23012);
                c.step()?;
                c.invoke(p_0ab9_1db7, false, 2745, 7607, 23015)?;
                label = 23015;
                continue;
            }
            23015 => {
                c.pc = (2745, 23015);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23018);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 23020);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 23023);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 23027);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 23028);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 23029);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 23030);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 23033)?;
                label = 23033;
                continue;
            }
            23033 => {
                c.pc = (2745, 23033);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23035);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23037);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 23040 } else { 23039 };
                continue;
            }
            23039 => {
                c.pc = (2745, 23039);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 23040;
                continue;
            }
            23040 => {
                c.pc = (2745, 23040);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 23041);
                c.step()?;
                c.invoke(p_0ab9_1a18, false, 2745, 6680, 23044)?;
                label = 23044;
                continue;
            }
            23044 => {
                c.pc = (2745, 23044);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 23045);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_5a06(c: &mut Cpu) -> Result<(), String> {
    let mut label = 23046u16;
    loop {
        match label {
            23046 => {
                c.pc = (2745, 23046);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 23047);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 23049);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53433u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 23054);
                c.step()?;
                label = if !c.z { 23124 } else { 23056 };
                continue;
            }
            23056 => {
                c.pc = (2745, 23056);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53409u16), 8, v)?;
                c.pc = (2745, 23061);
                c.step()?;
                c.invoke(p_0ab9_6b2f, false, 2745, 27439, 23064)?;
                label = 23064;
                continue;
            }
            23064 => {
                c.pc = (2745, 23064);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 23067)?;
                label = 23067;
                continue;
            }
            23067 => {
                c.pc = (2745, 23067);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23069);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23071);
                c.step()?;
                label = if !c.z { 23074 } else { 23073 };
                continue;
            }
            23073 => {
                c.pc = (2745, 23073);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 23074;
                continue;
            }
            23074 => {
                c.pc = (2745, 23074);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 23075);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 23077);
                c.step()?;
                let v = 9u16;
                c.push(v)?;
                c.pc = (2745, 23079);
                c.step()?;
                c.invoke(p_0ab9_5902, false, 2745, 22786, 23082)?;
                label = 23082;
                continue;
            }
            23082 => {
                c.pc = (2745, 23082);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23085);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53406u16), 8, v)?;
                c.pc = (2745, 23088);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 23090);
                c.step()?;
                let v = 45u16;
                c.push(v)?;
                c.pc = (2745, 23092);
                c.step()?;
                c.invoke(p_0ab9_5996, false, 2745, 22934, 23095)?;
                label = 23095;
                continue;
            }
            23095 => {
                c.pc = (2745, 23095);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23098);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 23100);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 23103);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 23107);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 23108);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 23109);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 23110);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 23113)?;
                label = 23113;
                continue;
            }
            23113 => {
                c.pc = (2745, 23113);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23115);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 23117);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 23120 } else { 23119 };
                continue;
            }
            23119 => {
                c.pc = (2745, 23119);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 23120;
                continue;
            }
            23120 => {
                c.pc = (2745, 23120);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 23121);
                c.step()?;
                c.invoke(p_0ab9_1a18, false, 2745, 6680, 23124)?;
                label = 23124;
                continue;
            }
            23124 => {
                c.pc = (2745, 23124);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 23125);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7423(c: &mut Cpu) -> Result<(), String> {
    let mut label = 29731u16;
    loop {
        match label {
            29731 => {
                c.pc = (2745, 29731);
                c.step()?;
                c.enter(4)?;
                c.pc = (2745, 29735);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29738);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 16)?;
                c.r[0] = v;
                c.pc = (2745, 29741);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 29744);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(7u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29748);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                let y = (c.r[0] >> 0) & 255;
                let v = c.add(x, y, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8, v)?;
                c.pc = (2745, 29751);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(9u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29755);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = (c.r[0] >> 0) & 255;
                let v = c.add(x, y, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 29758);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 29763);
                c.step()?;
                label = if c.cf || c.z { 29769 } else { 29765 };
                continue;
            }
            29765 => {
                c.pc = (2745, 29765);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 16)?;
                let v = c.dec(x, 16);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 16, v)?;
                label = 29769;
                continue;
            }
            29769 => {
                c.pc = (2745, 29769);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29772);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 29777);
                c.step()?;
                label = if c.z { 29791 } else { 29779 };
                continue;
            }
            29779 => {
                c.pc = (2745, 29779);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 29780);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 29781);
                c.step()?;
                c.invoke(p_1998_0b3f, true, 6552, 2879, 29786)?;
                label = 29786;
                continue;
            }
            29786 => {
                c.pc = (2745, 29786);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 29789);
                c.step()?;
                label = if !c.cf && !c.z { 29795 } else { 29791 };
                continue;
            }
            29791 => {
                c.pc = (2745, 29791);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29793);
                c.step()?;
                label = 29797;
                continue;
            }
            29795 => {
                c.pc = (2745, 29795);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 29797;
                continue;
            }
            29797 => {
                c.pc = (2745, 29797);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29800);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 8, v)?;
                c.pc = (2745, 29804);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 29807);
                c.step()?;
                c.invoke(p_1998_081b, true, 6552, 2075, 29812)?;
                label = 29812;
                continue;
            }
            29812 => {
                c.pc = (2745, 29812);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29814);
                c.step()?;
                label = if c.z { 29825 } else { 29816 };
                continue;
            }
            29816 => {
                c.pc = (2745, 29816);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 29819);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29822);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]), 16, v)?;
                label = 29825;
                continue;
            }
            29825 => {
                c.pc = (2745, 29825);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29828);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 29829);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 29830);
                c.step()?;
                c.invoke(p_1998_0a16, true, 6552, 2582, 29835)?;
                label = 29835;
                continue;
            }
            29835 => {
                c.pc = (2745, 29835);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29838);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 16)?;
                c.push(v)?;
                c.pc = (2745, 29841);
                c.step()?;
                c.invoke(p_1998_084b, true, 6552, 2123, 29846)?;
                label = 29846;
                continue;
            }
            29846 => {
                c.pc = (2745, 29846);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29848);
                c.step()?;
                label = if c.z { 29853 } else { 29850 };
                continue;
            }
            29850 => {
                c.pc = (2745, 29850);
                c.step()?;
                label = 30102;
                continue;
            }
            29853 => {
                c.pc = (2745, 29853);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 29855);
                c.step()?;
                c.invoke(p_1998_01e0, true, 6552, 480, 29860)?;
                label = 29860;
                continue;
            }
            29860 => {
                c.pc = (2745, 29860);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53409u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 29865);
                c.step()?;
                label = if c.cf || c.z { 29932 } else { 29867 };
                continue;
            }
            29867 => {
                c.pc = (2745, 29867);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29870);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29873);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 29874);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 29875);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 29877);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 29879);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 29882);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 29919 } else { 29884 };
                continue;
            }
            29884 => {
                c.pc = (2745, 29884);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29887);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 29889);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 29892);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 29896);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 29897);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 29898);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 29899);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 29902)?;
                label = 29902;
                continue;
            }
            29902 => {
                c.pc = (2745, 29902);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 29903);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 29905);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29908);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29912);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 29913);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 29915);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 29917);
                c.step()?;
                label = if c.sf == c.of { 29932 } else { 29919 };
                continue;
            }
            29919 => {
                c.pc = (2745, 29919);
                c.step()?;
                c.invoke(p_0ab9_6d83, false, 2745, 28035, 29922)?;
                label = 29922;
                continue;
            }
            29922 => {
                c.pc = (2745, 29922);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29924);
                c.step()?;
                label = if !c.z { 29932 } else { 29926 };
                continue;
            }
            29926 => {
                c.pc = (2745, 29926);
                c.step()?;
                c.invoke(p_0ab9_6dc0, false, 2745, 28096, 29929)?;
                label = 29929;
                continue;
            }
            29929 => {
                c.pc = (2745, 29929);
                c.step()?;
                label = 30102;
                continue;
            }
            29932 => {
                c.pc = (2745, 29932);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53408u16), 8, v)?;
                c.pc = (2745, 29937);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53406u16), 8, v)?;
                c.pc = (2745, 29942);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53432u16), 8, v)?;
                c.pc = (2745, 29947);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29950);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29953);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 29954);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 29955);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 29957);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 29959);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 29962);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 30009 } else { 29964 };
                continue;
            }
            29964 => {
                c.pc = (2745, 29964);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29967);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 29969);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 29972);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 29976);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 29977);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 29978);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 29979);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 29982)?;
                label = 29982;
                continue;
            }
            29982 => {
                c.pc = (2745, 29982);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 29983);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 29985);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 29988);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 29992);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 29993);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 29995);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 29997);
                c.step()?;
                label = if c.sf == c.of { 30004 } else { 29999 };
                continue;
            }
            29999 => {
                c.pc = (2745, 29999);
                c.step()?;
                c.invoke(p_0ab9_55ef, false, 2745, 21999, 30002)?;
                label = 30002;
                continue;
            }
            30002 => {
                c.pc = (2745, 30002);
                c.step()?;
                label = 30007;
                continue;
            }
            30004 => {
                c.pc = (2745, 30004);
                c.step()?;
                c.invoke(p_0ab9_54e2, false, 2745, 21730, 30007)?;
                label = 30007;
                continue;
            }
            30007 => {
                c.pc = (2745, 30007);
                c.step()?;
                label = 30088;
                continue;
            }
            30009 => {
                c.pc = (2745, 30009);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 30012);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30016);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30017);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 30018);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 30020);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 30022);
                c.step()?;
                let x = c.r[0];
                let y = 38u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 30025);
                c.step()?;
                label = if c.sf == c.of { 30032 } else { 30027 };
                continue;
            }
            30027 => {
                c.pc = (2745, 30027);
                c.step()?;
                c.invoke(p_0ab9_59b7, false, 2745, 22967, 30030)?;
                label = 30030;
                continue;
            }
            30030 => {
                c.pc = (2745, 30030);
                c.step()?;
                label = 30088;
                continue;
            }
            30032 => {
                c.pc = (2745, 30032);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30035);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 30037);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 30040);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 30044);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 30045);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 30046);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 30047);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 30050)?;
                label = 30050;
                continue;
            }
            30050 => {
                c.pc = (2745, 30050);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30051);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 30053);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 30056);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30060);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30061);
                c.step()?;
                let v = c.r[2];
                c.multiply(v, false);
                c.pc = (2745, 30063);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 30065);
                c.step()?;
                label = if c.sf == c.of { 30072 } else { 30067 };
                continue;
            }
            30067 => {
                c.pc = (2745, 30067);
                c.step()?;
                c.invoke(p_0ab9_5a06, false, 2745, 23046, 30070)?;
                label = 30070;
                continue;
            }
            30070 => {
                c.pc = (2745, 30070);
                c.step()?;
                label = 30088;
                continue;
            }
            30072 => {
                c.pc = (2745, 30072);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 30075)?;
                label = 30075;
                continue;
            }
            30075 => {
                c.pc = (2745, 30075);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30077);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30079);
                c.step()?;
                label = if !c.z { 30082 } else { 30081 };
                continue;
            }
            30081 => {
                c.pc = (2745, 30081);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 30082;
                continue;
            }
            30082 => {
                c.pc = (2745, 30082);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 30083);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 30085);
                c.step()?;
                c.invoke(p_0ab9_5a81, false, 2745, 23169, 30088)?;
                label = 30088;
                continue;
            }
            30088 => {
                c.pc = (2745, 30088);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 30091);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 30093);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(11u16), 16, v)?;
                c.pc = (2745, 30097);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53435u16), 8, v)?;
                label = 30102;
                continue;
            }
            30102 => {
                c.pc = (2745, 30102);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 30103);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_78b7(c: &mut Cpu) -> Result<(), String> {
    super::projections::neutral_watch_input(c)
}
pub(super) fn p_0ab9_7a02(c: &mut Cpu) -> Result<(), String> {
    let mut label = 31234u16;
    loop {
        match label {
            31234 => {
                c.pc = (2745, 31234);
                c.step()?;
                c.enter(8)?;
                c.pc = (2745, 31238);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 31243);
                c.step()?;
                label = 31248;
                continue;
            }
            31245 => {
                c.pc = (2745, 31245);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 31248;
                continue;
            }
            31248 => {
                c.pc = (2745, 31248);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53075u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31253);
                c.step()?;
                label = if !c.z { 31258 } else { 31255 };
                continue;
            }
            31255 => {
                c.pc = (2745, 31255);
                c.step()?;
                label = 31504;
                continue;
            }
            31258 => {
                c.pc = (2745, 31258);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 31261);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53076u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31266);
                c.step()?;
                label = if c.z { 31279 } else { 31268 };
                continue;
            }
            31268 => {
                c.pc = (2745, 31268);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53435u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31271);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31273);
                c.step()?;
                label = if c.cf { 31283 } else { 31275 };
                continue;
            }
            31275 => {
                c.pc = (2745, 31275);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 4u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31277);
                c.step()?;
                label = if !c.cf && !c.z { 31283 } else { 31279 };
                continue;
            }
            31279 => {
                c.pc = (2745, 31279);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31281);
                c.step()?;
                label = 31285;
                continue;
            }
            31283 => {
                c.pc = (2745, 31283);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 31285;
                continue;
            }
            31285 => {
                c.pc = (2745, 31285);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 31286);
                c.step()?;
                let x = c.r[0];
                let y = 22u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 31289);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 31291);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2861u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31295);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 31297);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[11], 0u16.wrapping_add(53073u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31301);
                c.step()?;
                label = if c.cf || c.z { 31306 } else { 31303 };
                continue;
            }
            31303 => {
                c.pc = (2745, 31303);
                c.step()?;
                label = 31504;
                continue;
            }
            31306 => {
                c.pc = (2745, 31306);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53063u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31311);
                c.step()?;
                label = if c.z { 31357 } else { 31313 };
                continue;
            }
            31313 => {
                c.pc = (2745, 31313);
                c.step()?;
                let v = 52488u16;
                c.r[7] = v;
                c.pc = (2745, 31316);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31317);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31318);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31319);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 31322)?;
                label = 31322;
                continue;
            }
            31322 => {
                c.pc = (2745, 31322);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.r[2] = (c.r[2] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31324);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53063u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31329);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31331);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 31334 } else { 31333 };
                continue;
            }
            31333 => {
                c.pc = (2745, 31333);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 31334;
                continue;
            }
            31334 => {
                c.pc = (2745, 31334);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 31335);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 31337);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23752u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31341);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[2] >> 0) & 255;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31343);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31345);
                c.step()?;
                label = if !c.z { 31348 } else { 31347 };
                continue;
            }
            31347 => {
                c.pc = (2745, 31347);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 31348;
                continue;
            }
            31348 => {
                c.pc = (2745, 31348);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 31349);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 31351);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 31353);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11836u16),
                    16,
                )?;
                let v = c.inc(x, 16);
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11836u16),
                    16,
                    v,
                )?;
                label = 31357;
                continue;
            }
            31357 => {
                c.pc = (2745, 31357);
                c.step()?;
                let v = 53062u16;
                c.r[7] = v;
                c.pc = (2745, 31360);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31361);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31362);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31363);
                c.step()?;
                c.invoke(p_0ab9_7423, false, 2745, 29731, 31366)?;
                label = 31366;
                continue;
            }
            31366 => {
                c.pc = (2745, 31366);
                c.step()?;
                c.invoke(p_0ab9_21d7, false, 2745, 8663, 31369)?;
                label = 31369;
                continue;
            }
            31369 => {
                c.pc = (2745, 31369);
                c.step()?;
                c.invoke(p_0ab9_1cd6, false, 2745, 7382, 31372)?;
                label = 31372;
                continue;
            }
            31372 => {
                c.pc = (2745, 31372);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31374);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31376);
                c.step()?;
                label = if !c.z { 31379 } else { 31378 };
                continue;
            }
            31378 => {
                c.pc = (2745, 31378);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 31379;
                continue;
            }
            31379 => {
                c.pc = (2745, 31379);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 31380);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 31382);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31386);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 31388);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 31389);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 31392);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 31396);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31397);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 31398);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 31401);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 31404);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53435u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31407);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31409);
                c.step()?;
                label = if c.cf { 31460 } else { 31411 };
                continue;
            }
            31411 => {
                c.pc = (2745, 31411);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 4u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31413);
                c.step()?;
                label = if !c.cf && !c.z { 31460 } else { 31415 };
                continue;
            }
            31415 => {
                c.pc = (2745, 31415);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 31416);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31417);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31418);
                c.step()?;
                c.invoke(p_0ab9_1fb6, false, 2745, 8118, 31421)?;
                label = 31421;
                continue;
            }
            31421 => {
                c.pc = (2745, 31421);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31423);
                c.step()?;
                label = if !c.z { 31460 } else { 31425 };
                continue;
            }
            31425 => {
                c.pc = (2745, 31425);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 31428);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31431);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[11], 0u16.wrapping_add(53062u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31435);
                c.step()?;
                label = if !c.z { 31451 } else { 31437 };
                continue;
            }
            31437 => {
                c.pc = (2745, 31437);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53062u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 31441);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 31442);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31443);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31444);
                c.step()?;
                c.invoke(p_0ab9_207f, false, 2745, 8319, 31447)?;
                label = 31447;
                continue;
            }
            31447 => {
                c.pc = (2745, 31447);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31449);
                c.step()?;
                label = if c.z { 31460 } else { 31451 };
                continue;
            }
            31451 => {
                c.pc = (2745, 31451);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 31454);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 31455);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31456);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31457);
                c.step()?;
                c.invoke(p_0ab9_72bb, false, 2745, 29371, 31460)?;
                label = 31460;
                continue;
            }
            31460 => {
                c.pc = (2745, 31460);
                c.step()?;
                c.invoke(p_0ab9_46d4, false, 2745, 18132, 31463)?;
                label = 31463;
                continue;
            }
            31463 => {
                c.pc = (2745, 31463);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31465);
                c.step()?;
                label = if !c.z { 31504 } else { 31467 };
                continue;
            }
            31467 => {
                c.pc = (2745, 31467);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 31472);
                c.step()?;
                label = 31477;
                continue;
            }
            31474 => {
                c.pc = (2745, 31474);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 31477;
                continue;
            }
            31477 => {
                c.pc = (2745, 31477);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53075u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31482);
                c.step()?;
                label = if c.z { 31498 } else { 31484 };
                continue;
            }
            31484 => {
                c.pc = (2745, 31484);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 31488);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 31492);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31493);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31494);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31495);
                c.step()?;
                c.invoke(p_0ab9_4dff, false, 2745, 19967, 31498)?;
                label = 31498;
                continue;
            }
            31498 => {
                c.pc = (2745, 31498);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31502);
                c.step()?;
                label = if !c.z { 31474 } else { 31504 };
                continue;
            }
            31504 => {
                c.pc = (2745, 31504);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 31507);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2839u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31511);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 31513);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 31516);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 31520);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31521);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 31522);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 31525);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 31528);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 31529);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31530);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31531);
                c.step()?;
                c.invoke(p_0ab9_1ca1, false, 2745, 7329, 31534)?;
                label = 31534;
                continue;
            }
            31534 => {
                c.pc = (2745, 31534);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31536);
                c.step()?;
                label = if !c.z { 31547 } else { 31538 };
                continue;
            }
            31538 => {
                c.pc = (2745, 31538);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 31541);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 31542);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31543);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31544);
                c.step()?;
                c.invoke(p_0ab9_72bb, false, 2745, 29371, 31547)?;
                label = 31547;
                continue;
            }
            31547 => {
                c.pc = (2745, 31547);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 31550);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2839u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31554);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31558);
                c.step()?;
                label = if !c.z { 31563 } else { 31560 };
                continue;
            }
            31560 => {
                c.pc = (2745, 31560);
                c.step()?;
                c.invoke(p_0ab9_21d7, false, 2745, 8663, 31563)?;
                label = 31563;
                continue;
            }
            31563 => {
                c.pc = (2745, 31563);
                c.step()?;
                c.invoke(p_0ab9_79ed, false, 2745, 31213, 31566)?;
                label = 31566;
                continue;
            }
            31566 => {
                c.pc = (2745, 31566);
                c.step()?;
                let v = 53062u16;
                c.r[7] = v;
                c.pc = (2745, 31569);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31570);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31571);
                c.step()?;
                c.invoke(p_1998_0a16, true, 6552, 2582, 31576)?;
                label = 31576;
                continue;
            }
            31576 => {
                c.pc = (2745, 31576);
                c.step()?;
                c.invoke(p_0ab9_78b7, false, 2745, 30903, 31579)?;
                label = 31579;
                continue;
            }
            31579 => {
                c.pc = (2745, 31579);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31584);
                c.step()?;
                label = if c.z { 31588 } else { 31586 };
                continue;
            }
            31586 => {
                c.pc = (2745, 31586);
                c.step()?;
                label = 31609;
                continue;
            }
            31588 => {
                c.pc = (2745, 31588);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31592);
                c.step()?;
                label = if c.z { 31597 } else { 31594 };
                continue;
            }
            31594 => {
                c.pc = (2745, 31594);
                c.step()?;
                label = 31245;
                continue;
            }
            31597 => {
                c.pc = (2745, 31597);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53438u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 31602);
                c.step()?;
                label = if c.cf || c.z { 31609 } else { 31604 };
                continue;
            }
            31604 => {
                c.pc = (2745, 31604);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53438u16), 16)?;
                let y = 2u16;
                let v = c.sub(x, y, 16);
                c.write(c.r[11], 0u16.wrapping_add(53438u16), 16, v)?;
                label = 31609;
                continue;
            }
            31609 => {
                c.pc = (2745, 31609);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 31610);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_a23c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 41532u16;
    loop {
        match label {
            41532 => {
                c.pc = (2745, 41532);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 41536);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41539);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41540);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41542);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41544);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 41549);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41553);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41555);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41559);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 41562);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 41565);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41568);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41569);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41571);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41573);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 41577);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41580);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41581);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41583);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41585);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.r[7] = v;
                c.pc = (2745, 41589);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(10107u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41593);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 41595);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41597);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(27657u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41601);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41602);
                c.step()?;
                c.invoke(p_1998_0c23, true, 6552, 3107, 41607)?;
                label = 41607;
                continue;
            }
            41607 => {
                c.pc = (2745, 41607);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 41611);
                c.step()?;
                label = if !c.z { 41654 } else { 41613 };
                continue;
            }
            41613 => {
                c.pc = (2745, 41613);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 41615);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41618);
                c.step()?;
                let x = c.r[7];
                let y = 57u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41622);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 41623);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 41624);
                c.step()?;
                c.invoke(p_232a_00d5, true, 9002, 213, 41629)?;
                label = 41629;
                continue;
            }
            41629 => {
                c.pc = (2745, 41629);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41632);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41633);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41635);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41637);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 41641);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41644);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(57u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41648);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41649);
                c.step()?;
                c.invoke(p_1998_0c23, true, 6552, 3107, 41654)?;
                label = 41654;
                continue;
            }
            41654 => {
                c.pc = (2745, 41654);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 41658);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41660);
                c.step()?;
                label = if !c.z { 41663 } else { 41662 };
                continue;
            }
            41662 => {
                c.pc = (2745, 41662);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 41663;
                continue;
            }
            41663 => {
                c.pc = (2745, 41663);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41664);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41666);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41668);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 41672);
                c.step()?;
                c.invoke(p_1b55_265b, true, 6997, 9819, 41677)?;
                label = 41677;
                continue;
            }
            41677 => {
                c.pc = (2745, 41677);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41678);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41681);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41682);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41684);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3174u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41688);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 41690);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41691);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41694);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41695);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41697);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41699);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 41703);
                c.step()?;
                c.invoke(p_1b55_265b, true, 6997, 9819, 41708)?;
                label = 41708;
                continue;
            }
            41708 => {
                c.pc = (2745, 41708);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 41709);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 41711);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 41712);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 41714);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 41779 } else { 41716 };
                continue;
            }
            41716 => {
                c.pc = (2745, 41716);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 41721);
                c.step()?;
                label = 41726;
                continue;
            }
            41723 => {
                c.pc = (2745, 41723);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 41726;
                continue;
            }
            41726 => {
                c.pc = (2745, 41726);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 41728);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 41733)?;
                label = 41733;
                continue;
            }
            41733 => {
                c.pc = (2745, 41733);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41734);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41737);
                c.step()?;
                let x = c.r[7];
                let y = 57u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41741);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 41742);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 41743);
                c.step()?;
                c.invoke(p_232a_00d5, true, 9002, 213, 41748)?;
                label = 41748;
                continue;
            }
            41748 => {
                c.pc = (2745, 41748);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 41752);
                c.step()?;
                label = if !c.z { 41723 } else { 41754 };
                continue;
            }
            41754 => {
                c.pc = (2745, 41754);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41757);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41758);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41760);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41762);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 41766);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41769);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(57u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41773);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41774);
                c.step()?;
                c.invoke(p_1998_0c23, true, 6552, 3107, 41779)?;
                label = 41779;
                continue;
            }
            41779 => {
                c.pc = (2745, 41779);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41782);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41783);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41785);
                c.step()?;
                let v = 0u16;
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12084u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 41790);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41793);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41794);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41796);
                c.step()?;
                let v = 0u16;
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12082u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 41801);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 41802);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_a34d(c: &mut Cpu) -> Result<(), String> {
    let mut label = 41805u16;
    loop {
        match label {
            41805 => {
                c.pc = (2745, 41805);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 41809);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41812);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41813);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41815);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41817);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 41822);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41826);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41828);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41832);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 41835);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 41838);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41841);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41842);
                c.step()?;
                c.invoke(p_0ab9_a23c, false, 2745, 41532, 41845)?;
                label = 41845;
                continue;
            }
            41845 => {
                c.pc = (2745, 41845);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 41850);
                c.step()?;
                label = 41855;
                continue;
            }
            41852 => {
                c.pc = (2745, 41852);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 41855;
                continue;
            }
            41855 => {
                c.pc = (2745, 41855);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 41859);
                c.step()?;
                label = if c.z { 41885 } else { 41861 };
                continue;
            }
            41861 => {
                c.pc = (2745, 41861);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 41863);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 41868)?;
                label = 41868;
                continue;
            }
            41868 => {
                c.pc = (2745, 41868);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41869);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41872);
                c.step()?;
                let x = c.r[7];
                let y = 57u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41876);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 41877);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 41878);
                c.step()?;
                c.invoke(p_232a_00d5, true, 9002, 213, 41883)?;
                label = 41883;
                continue;
            }
            41883 => {
                c.pc = (2745, 41883);
                c.step()?;
                label = 41907;
                continue;
            }
            41885 => {
                c.pc = (2745, 41885);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 41887);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 41892)?;
                label = 41892;
                continue;
            }
            41892 => {
                c.pc = (2745, 41892);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41893);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41896);
                c.step()?;
                let x = c.r[7];
                let y = 57u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41900);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 41901);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 41902);
                c.step()?;
                c.invoke(p_232a_00f8, true, 9002, 248, 41907)?;
                label = 41907;
                continue;
            }
            41907 => {
                c.pc = (2745, 41907);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 3u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 41911);
                c.step()?;
                label = if !c.z { 41852 } else { 41913 };
                continue;
            }
            41913 => {
                c.pc = (2745, 41913);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41916);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41917);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41919);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41921);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 41925);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41928);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(57u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41932);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 41933);
                c.step()?;
                c.invoke(p_1998_0c23, true, 6552, 3107, 41938)?;
                label = 41938;
                continue;
            }
            41938 => {
                c.pc = (2745, 41938);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 41939);
                c.step()?;
                c.ret(false, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_a3d6(c: &mut Cpu) -> Result<(), String> {
    let mut label = 41942u16;
    loop {
        match label {
            41942 => {
                c.pc = (2745, 41942);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 41946);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41949);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41950);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41952);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41954);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 41959);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 41963);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41965);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 41969);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 41972);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 41975);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 41978);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 41979);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 41981);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12084u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 41986);
                c.step()?;
                label = if c.z { 41990 } else { 41988 };
                continue;
            }
            41988 => {
                c.pc = (2745, 41988);
                c.step()?;
                label = 42080;
                continue;
            }
            41990 => {
                c.pc = (2745, 41990);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 41995);
                c.step()?;
                label = 42000;
                continue;
            }
            41997 => {
                c.pc = (2745, 41997);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 42000;
                continue;
            }
            42000 => {
                c.pc = (2745, 42000);
                c.step()?;
                let v = 5u16;
                c.push(v)?;
                c.pc = (2745, 42002);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 42007)?;
                label = 42007;
                continue;
            }
            42007 => {
                c.pc = (2745, 42007);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42008);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 42011);
                c.step()?;
                let x = c.r[7];
                let y = 57u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42015);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 42016);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 42017);
                c.step()?;
                c.invoke(p_232a_00d5, true, 9002, 213, 42022)?;
                label = 42022;
                continue;
            }
            42022 => {
                c.pc = (2745, 42022);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 5u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 42026);
                c.step()?;
                label = if !c.z { 41997 } else { 42028 };
                continue;
            }
            42028 => {
                c.pc = (2745, 42028);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 42030);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 42033);
                c.step()?;
                let x = c.r[7];
                let y = 57u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42037);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 42038);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 42039);
                c.step()?;
                c.invoke(p_232a_00d5, true, 9002, 213, 42044)?;
                label = 42044;
                continue;
            }
            42044 => {
                c.pc = (2745, 42044);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42047);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42048);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42050);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42052);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 42056);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 42059);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(57u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42063);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42064);
                c.step()?;
                c.invoke(p_1998_0c23, true, 6552, 3107, 42069)?;
                label = 42069;
                continue;
            }
            42069 => {
                c.pc = (2745, 42069);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42072);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42073);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42075);
                c.step()?;
                let v = 1u16;
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12084u16),
                    8,
                    v,
                )?;
                label = 42080;
                continue;
            }
            42080 => {
                c.pc = (2745, 42080);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 42081);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_a464(c: &mut Cpu) -> Result<(), String> {
    let mut label = 42084u16;
    loop {
        match label {
            42084 => {
                c.pc = (2745, 42084);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 42088);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42091);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42092);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42094);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42096);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 42101);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 42105);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42107);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42111);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 42114);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 42117);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42120);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42121);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42123);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12082u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 42128);
                c.step()?;
                label = if c.z { 42132 } else { 42130 };
                continue;
            }
            42130 => {
                c.pc = (2745, 42130);
                c.step()?;
                label = 42222;
                continue;
            }
            42132 => {
                c.pc = (2745, 42132);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 42137);
                c.step()?;
                label = 42142;
                continue;
            }
            42139 => {
                c.pc = (2745, 42139);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 42142;
                continue;
            }
            42142 => {
                c.pc = (2745, 42142);
                c.step()?;
                let v = 5u16;
                c.push(v)?;
                c.pc = (2745, 42144);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 42149)?;
                label = 42149;
                continue;
            }
            42149 => {
                c.pc = (2745, 42149);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42150);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 42153);
                c.step()?;
                let x = c.r[7];
                let y = 57u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42157);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 42158);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 42159);
                c.step()?;
                c.invoke(p_232a_00f8, true, 9002, 248, 42164)?;
                label = 42164;
                continue;
            }
            42164 => {
                c.pc = (2745, 42164);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 5u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 42168);
                c.step()?;
                label = if !c.z { 42139 } else { 42170 };
                continue;
            }
            42170 => {
                c.pc = (2745, 42170);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 42172);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 42175);
                c.step()?;
                let x = c.r[7];
                let y = 57u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42179);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 42180);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 42181);
                c.step()?;
                c.invoke(p_232a_00f8, true, 9002, 248, 42186)?;
                label = 42186;
                continue;
            }
            42186 => {
                c.pc = (2745, 42186);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42189);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42190);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42192);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42194);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 42198);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 42201);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(57u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42205);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42206);
                c.step()?;
                c.invoke(p_1998_0c23, true, 6552, 3107, 42211)?;
                label = 42211;
                continue;
            }
            42211 => {
                c.pc = (2745, 42211);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42214);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42215);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42217);
                c.step()?;
                let v = 1u16;
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12082u16),
                    8,
                    v,
                )?;
                label = 42222;
                continue;
            }
            42222 => {
                c.pc = (2745, 42222);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 42223);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_a4f2(c: &mut Cpu) -> Result<(), String> {
    let mut label = 42226u16;
    loop {
        match label {
            42226 => {
                c.pc = (2745, 42226);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 42230);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53445u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42233);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42234);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 42236);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42239);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42240);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 42242);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 42245);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8, v)?;
                c.pc = (2745, 42249);
                c.step()?;
                label = 42254;
                continue;
            }
            42251 => {
                c.pc = (2745, 42251);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8, v)?;
                label = 42254;
                continue;
            }
            42254 => {
                c.pc = (2745, 42254);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42257);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42258);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42260);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 42262);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 42266);
                c.step()?;
                c.invoke(p_1b55_0e7a, true, 6997, 3706, 42271)?;
                label = 42271;
                continue;
            }
            42271 => {
                c.pc = (2745, 42271);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42273);
                c.step()?;
                label = if c.z { 42278 } else { 42275 };
                continue;
            }
            42275 => {
                c.pc = (2745, 42275);
                c.step()?;
                label = 42417;
                continue;
            }
            42278 => {
                c.pc = (2745, 42278);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42281);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42282);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 42283);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 42285);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 42287);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 42290);
                c.step()?;
                label = if c.sf == c.of { 42301 } else { 42292 };
                continue;
            }
            42292 => {
                c.pc = (2745, 42292);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42295);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42296);
                c.step()?;
                c.invoke(p_0ab9_a23c, false, 2745, 41532, 42299)?;
                label = 42299;
                continue;
            }
            42299 => {
                c.pc = (2745, 42299);
                c.step()?;
                label = 42417;
                continue;
            }
            42301 => {
                c.pc = (2745, 42301);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42304);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42305);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 42306);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 42308);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 42310);
                c.step()?;
                let x = c.r[0];
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 42313);
                c.step()?;
                label = if !c.z { 42369 } else { 42315 };
                continue;
            }
            42315 => {
                c.pc = (2745, 42315);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 42319);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42321);
                c.step()?;
                label = if !c.z { 42324 } else { 42323 };
                continue;
            }
            42323 => {
                c.pc = (2745, 42323);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 42324;
                continue;
            }
            42324 => {
                c.pc = (2745, 42324);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42325);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42327);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16), 8)?;
                c.r[2] = (c.r[2] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42331);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42334);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42335);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42337);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42341);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[2] >> 0) & 255;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 42343);
                c.step()?;
                label = if c.cf || c.z { 42357 } else { 42345 };
                continue;
            }
            42345 => {
                c.pc = (2745, 42345);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 42347);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42350);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42351);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 42352);
                c.step()?;
                c.invoke(p_0ab9_a34d, false, 2745, 41805, 42355)?;
                label = 42355;
                continue;
            }
            42355 => {
                c.pc = (2745, 42355);
                c.step()?;
                label = 42367;
                continue;
            }
            42357 => {
                c.pc = (2745, 42357);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 42359);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42362);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42363);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 42364);
                c.step()?;
                c.invoke(p_0ab9_a34d, false, 2745, 41805, 42367)?;
                label = 42367;
                continue;
            }
            42367 => {
                c.pc = (2745, 42367);
                c.step()?;
                label = 42417;
                continue;
            }
            42369 => {
                c.pc = (2745, 42369);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 42373);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42375);
                c.step()?;
                label = if !c.z { 42378 } else { 42377 };
                continue;
            }
            42377 => {
                c.pc = (2745, 42377);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 42378;
                continue;
            }
            42378 => {
                c.pc = (2745, 42378);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42379);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42381);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16), 8)?;
                c.r[2] = (c.r[2] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42385);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42388);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 42389);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 42391);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12092u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42395);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[2] >> 0) & 255;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 42397);
                c.step()?;
                label = if c.cf || c.z { 42409 } else { 42399 };
                continue;
            }
            42399 => {
                c.pc = (2745, 42399);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42402);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42403);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 42404);
                c.step()?;
                c.invoke(p_0ab9_a464, false, 2745, 42084, 42407)?;
                label = 42407;
                continue;
            }
            42407 => {
                c.pc = (2745, 42407);
                c.step()?;
                label = 42417;
                continue;
            }
            42409 => {
                c.pc = (2745, 42409);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 42412);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 42413);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 42414);
                c.step()?;
                c.invoke(p_0ab9_a3d6, false, 2745, 41942, 42417)?;
                label = 42417;
                continue;
            }
            42417 => {
                c.pc = (2745, 42417);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 42421);
                c.step()?;
                label = if c.z { 42426 } else { 42423 };
                continue;
            }
            42423 => {
                c.pc = (2745, 42423);
                c.step()?;
                label = 42251;
                continue;
            }
            42426 => {
                c.pc = (2745, 42426);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 42427);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1998_022b(c: &mut Cpu) -> Result<(), String> {
    let mut label = 555u16;
    loop {
        match label {
            555 => {
                c.pc = (6552, 555);
                c.step()?;
                c.enter(2)?;
                c.pc = (6552, 559);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.r[0] = v;
                c.pc = (6552, 562);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 565);
                c.step()?;
                label = if !c.z { 574 } else { 567 };
                continue;
            }
            567 => {
                c.pc = (6552, 567);
                c.step()?;
                let v = 10u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (6552, 572);
                c.step()?;
                label = 620;
                continue;
            }
            574 => {
                c.pc = (6552, 574);
                c.step()?;
                let x = c.r[0];
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 577);
                c.step()?;
                label = if !c.z { 586 } else { 579 };
                continue;
            }
            579 => {
                c.pc = (6552, 579);
                c.step()?;
                let v = 20u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (6552, 584);
                c.step()?;
                label = 620;
                continue;
            }
            586 => {
                c.pc = (6552, 586);
                c.step()?;
                let x = c.r[0];
                let y = 33u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 589);
                c.step()?;
                label = if !c.z { 598 } else { 591 };
                continue;
            }
            591 => {
                c.pc = (6552, 591);
                c.step()?;
                let v = 27u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (6552, 596);
                c.step()?;
                label = 620;
                continue;
            }
            598 => {
                c.pc = (6552, 598);
                c.step()?;
                let x = c.r[0];
                let y = 43u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 601);
                c.step()?;
                label = if !c.z { 610 } else { 603 };
                continue;
            }
            603 => {
                c.pc = (6552, 603);
                c.step()?;
                let v = 39u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (6552, 608);
                c.step()?;
                label = 620;
                continue;
            }
            610 => {
                c.pc = (6552, 610);
                c.step()?;
                let x = c.r[0];
                let y = 53u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 613);
                c.step()?;
                label = if !c.z { 620 } else { 615 };
                continue;
            }
            615 => {
                c.pc = (6552, 615);
                c.step()?;
                let v = 48u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 620;
                continue;
            }
            620 => {
                c.pc = (6552, 620);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (6552, 623);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6552, 624);
                c.step()?;
                c.ret(true, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1998_0273(c: &mut Cpu) -> Result<(), String> {
    let mut label = 627u16;
    loop {
        match label {
            627 => {
                c.pc = (6552, 627);
                c.step()?;
                c.enter(6)?;
                c.pc = (6552, 631);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.r[0] = v;
                c.pc = (6552, 634);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (6552, 636);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 638);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 640);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (6552, 642);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 16)?;
                let y = 174u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6552, 647);
                c.step()?;
                let o = 0u16.wrapping_add(55242u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 651);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 653);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 655);
                c.step()?;
                let x = c.r[7];
                let y = 65359u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 659);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (6552, 662);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (6552, 665);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                let y = 6u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 669);
                c.step()?;
                label = if !c.z { 682 } else { 671 };
                continue;
            }
            671 => {
                c.pc = (6552, 671);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 674);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 12u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 676);
                c.step()?;
                label = if c.z { 723 } else { 678 };
                continue;
            }
            678 => {
                c.pc = (6552, 678);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 22u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 680);
                c.step()?;
                label = if c.z { 723 } else { 682 };
                continue;
            }
            682 => {
                c.pc = (6552, 682);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 685);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 688);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 7u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 690);
                c.step()?;
                label = if c.cf { 719 } else { 692 };
                continue;
            }
            692 => {
                c.pc = (6552, 692);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 8u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 694);
                c.step()?;
                label = if !c.cf && !c.z { 719 } else { 696 };
                continue;
            }
            696 => {
                c.pc = (6552, 696);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 699);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 12u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 701);
                c.step()?;
                label = if c.z { 723 } else { 703 };
                continue;
            }
            703 => {
                c.pc = (6552, 703);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 22u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 705);
                c.step()?;
                label = if c.z { 723 } else { 707 };
                continue;
            }
            707 => {
                c.pc = (6552, 707);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 33u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 709);
                c.step()?;
                label = if c.z { 723 } else { 711 };
                continue;
            }
            711 => {
                c.pc = (6552, 711);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 43u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 713);
                c.step()?;
                label = if c.z { 723 } else { 715 };
                continue;
            }
            715 => {
                c.pc = (6552, 715);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 53u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 717);
                c.step()?;
                label = if c.z { 723 } else { 719 };
                continue;
            }
            719 => {
                c.pc = (6552, 719);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 721);
                c.step()?;
                label = 725;
                continue;
            }
            723 => {
                c.pc = (6552, 723);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 725;
                continue;
            }
            725 => {
                c.pc = (6552, 725);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (6552, 728);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 731);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6552, 732);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1998_02df(c: &mut Cpu) -> Result<(), String> {
    let mut label = 735u16;
    loop {
        match label {
            735 => {
                c.pc = (6552, 735);
                c.step()?;
                c.enter(260)?;
                c.pc = (6552, 739);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 742);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (6552, 745);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (6552, 748);
                c.step()?;
                let v = 112u16;
                c.push(v)?;
                c.pc = (6552, 750);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(260u16);
                c.pc = (6552, 754);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (6552, 755);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6552, 756);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53445u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 759);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6552, 761);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (6552, 763);
                c.step()?;
                let v = c.r[2];
                c.push(v)?;
                c.pc = (6552, 764);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6552, 765);
                c.step()?;
                c.invoke(p_232a_07c9, true, 9002, 1993, 770)?;
                label = 770;
                continue;
            }
            770 => {
                c.pc = (6552, 770);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (6552, 772);
                c.step()?;
                let v = 19u16;
                c.push(v)?;
                c.pc = (6552, 774);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 777);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (6552, 778);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6552, 779);
                c.step()?;
                c.invoke(p_232a_030c, true, 9002, 780, 784)?;
                label = 784;
                continue;
            }
            784 => {
                c.pc = (6552, 784);
                c.step()?;
                let v = 112u16;
                c.push(v)?;
                c.pc = (6552, 786);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(260u16);
                c.pc = (6552, 790);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (6552, 791);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6552, 792);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 795);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6552, 797);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (6552, 799);
                c.step()?;
                let v = c.r[2];
                c.push(v)?;
                c.pc = (6552, 800);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6552, 801);
                c.step()?;
                c.invoke(p_232a_07c9, true, 9002, 1993, 806)?;
                label = 806;
                continue;
            }
            806 => {
                c.pc = (6552, 806);
                c.step()?;
                let v = 4u16;
                c.push(v)?;
                c.pc = (6552, 808);
                c.step()?;
                let v = 19u16;
                c.push(v)?;
                c.pc = (6552, 810);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 813);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (6552, 814);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6552, 815);
                c.step()?;
                c.invoke(p_232a_030c, true, 9002, 780, 820)?;
                label = 820;
                continue;
            }
            820 => {
                c.pc = (6552, 820);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6552, 821);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1998_081b(c: &mut Cpu) -> Result<(), String> {
    let mut label = 2075u16;
    loop {
        match label {
            2075 => {
                c.pc = (6552, 2075);
                c.step()?;
                c.enter(2)?;
                c.pc = (6552, 2079);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 2082);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (6552, 2083);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (6552, 2084);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 2086);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 2088);
                c.step()?;
                let x = c.r[0];
                let y = 9u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 2091);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 2107 } else { 2093 };
                continue;
            }
            2093 => {
                c.pc = (6552, 2093);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(7u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 2096);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (6552, 2097);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (6552, 2098);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 2100);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 2102);
                c.step()?;
                let x = c.r[0];
                let y = 39u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 2105);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 2111 } else { 2107 };
                continue;
            }
            2107 => {
                c.pc = (6552, 2107);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 2109);
                c.step()?;
                label = 2113;
                continue;
            }
            2111 => {
                c.pc = (6552, 2111);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 2113;
                continue;
            }
            2113 => {
                c.pc = (6552, 2113);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (6552, 2116);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 2119);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6552, 2120);
                c.step()?;
                c.ret(true, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1998_0c23(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3107u16;
    loop {
        match label {
            3107 => {
                c.pc = (6552, 3107);
                c.step()?;
                c.enter(6)?;
                c.pc = (6552, 3111);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 16)?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6552, 3115);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3119);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3121);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3125);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (6552, 3128);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (6552, 3131);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3134);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(57u16), 8, v)?;
                c.pc = (6552, 3138);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (6552, 3143);
                c.step()?;
                label = 3148;
                continue;
            }
            3145 => {
                c.pc = (6552, 3145);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 3148;
                continue;
            }
            3148 => {
                c.pc = (6552, 3148);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (6552, 3151);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 3153);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3156);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3158);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 3163);
                c.step()?;
                label = if c.z { 3184 } else { 3165 };
                continue;
            }
            3165 => {
                c.pc = (6552, 3165);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (6552, 3168);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 3170);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3173);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3175);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (6552, 3179);
                c.step()?;
                c.invoke(p_1b55_53fb, true, 6997, 21499, 3184)?;
                label = 3184;
                continue;
            }
            3184 => {
                c.pc = (6552, 3184);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 20u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 3188);
                c.step()?;
                label = if !c.z { 3145 } else { 3190 };
                continue;
            }
            3190 => {
                c.pc = (6552, 3190);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6552, 3191);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1b55_17f9(c: &mut Cpu) -> Result<(), String> {
    let mut label = 6137u16;
    loop {
        match label {
            6137 => {
                c.pc = (6997, 6137);
                c.step()?;
                c.enter(2)?;
                c.pc = (6997, 6141);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(12u16), 16)?;
                c.r[2] = v;
                c.pc = (6997, 6144);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(14u16), 16)?;
                let y = 58u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6997, 6148);
                c.step()?;
                let o = 0u16.wrapping_add(55246u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 6152);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6154);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6156);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(59u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6160);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6997, 6161);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16);
                c.pc = (6997, 6164);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (6997, 6165);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6997, 6166);
                c.step()?;
                c.invoke(p_232a_01e3, true, 9002, 483, 6171)?;
                label = 6171;
                continue;
            }
            6171 => {
                c.pc = (6997, 6171);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6174);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 6177);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]), 8, v)?;
                c.pc = (6997, 6180);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6183);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8, v)?;
                c.pc = (6997, 6187);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.r[2] = v;
                c.pc = (6997, 6190);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(14u16), 16)?;
                let y = 58u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6997, 6194);
                c.step()?;
                let o = 0u16.wrapping_add(55246u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 6198);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6200);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6202);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(59u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6206);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6997, 6207);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16);
                c.pc = (6997, 6210);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (6997, 6211);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6997, 6212);
                c.step()?;
                c.invoke(p_232a_01e3, true, 9002, 483, 6217)?;
                label = 6217;
                continue;
            }
            6217 => {
                c.pc = (6997, 6217);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6220);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 6223);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                let y = (c.r[0] >> 0) & 255;
                let v = c.add(x, y, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]), 8, v)?;
                c.pc = (6997, 6226);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6229);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                let y = (c.r[0] >> 0) & 255;
                let v = c.add(x, y, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8, v)?;
                c.pc = (6997, 6233);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6997, 6234);
                c.step()?;
                c.ret(true, 10)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_232a_01e3(c: &mut Cpu) -> Result<(), String> {
    let mut label = 483u16;
    loop {
        match label {
            483 => {
                c.pc = (9002, 483);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (9002, 484);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (9002, 486);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 489);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 15u16;
                let v = c.and(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 491);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9002, 494);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]), 8, v)?;
                c.pc = (9002, 497);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 500);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 240u16;
                let v = c.and(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 502);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 504);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (9002, 507);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8, v)?;
                c.pc = (9002, 511);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9002, 512);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
