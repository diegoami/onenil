//! [confirmed] Mechanically compiled numeric control flow;F39/c7637bd,static d9e20a4.
//! [designed] Direct Rust calls/blocks;no runtime decoder or original executable.
use super::action_graph::*;
use super::caller_graph::*;
use super::compiled::Cpu;
use super::tick_graph::*;
pub(super) fn p_0ab9_03e4(c: &mut Cpu) -> Result<(), String> {
    let mut label = 996u16;
    loop {
        match label {
            996 => {
                c.pc = (2745, 996);
                c.step()?;
                c.enter(552)?;
                c.pc = (2745, 1000);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 1003);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1005);
                c.step()?;
                let v = c.r[7];
                c.r[6] = v;
                c.pc = (2745, 1007);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1009);
                c.step()?;
                let x = c.r[7];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1011);
                c.step()?;
                let x = c.r[7];
                let y = 42282u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1015);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 1016);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 1017);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(40u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 1020);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(38u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 1023);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1026);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 1027);
                c.step()?;
                let x = c.r[0];
                let y = 21u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 1030);
                c.step()?;
                let x = c.r[7];
                let y = 898u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1034);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 1035);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1036);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16);
                c.pc = (2745, 1039);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1040);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1041);
                c.step()?;
                let v = 35u16;
                c.push(v)?;
                c.pc = (2745, 1043);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1048)?;
                label = 1048;
                continue;
            }
            1048 => {
                c.pc = (2745, 1048);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 1052);
                c.step()?;
                label = if !c.z { 1057 } else { 1054 };
                continue;
            }
            1054 => {
                c.pc = (2745, 1054);
                c.step()?;
                label = 1261;
                continue;
            }
            1057 => {
                c.pc = (2745, 1057);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(35u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 1061);
                c.step()?;
                label = if !c.z { 1107 } else { 1063 };
                continue;
            }
            1063 => {
                c.pc = (2745, 1063);
                c.step()?;
                let v = 32u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(35u16), 8, v)?;
                c.pc = (2745, 1067);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(296u16);
                c.pc = (2745, 1071);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1072);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1073);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(40u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1076);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 1080);
                c.step()?;
                c.invoke(p_1b55_44dc, true, 6997, 17628, 1085)?;
                label = 1085;
                continue;
            }
            1085 => {
                c.pc = (2745, 1085);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16);
                c.pc = (2745, 1088);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1089);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1090);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1095)?;
                label = 1095;
                continue;
            }
            1095 => {
                c.pc = (2745, 1095);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16);
                c.pc = (2745, 1098);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1099);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1100);
                c.step()?;
                let v = 35u16;
                c.push(v)?;
                c.pc = (2745, 1102);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1107)?;
                label = 1107;
                continue;
            }
            1107 => {
                c.pc = (2745, 1107);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1110);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1112);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 1114);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(36u16),
                    8,
                )?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 1118);
                c.step()?;
                label = if !c.z { 1184 } else { 1120 };
                continue;
            }
            1120 => {
                c.pc = (2745, 1120);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 8)?;
                let v = c.dec(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 8, v)?;
                c.pc = (2745, 1123);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(296u16);
                c.pc = (2745, 1127);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1128);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1129);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16);
                c.pc = (2745, 1132);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1133);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1134);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 1139)?;
                label = 1139;
                continue;
            }
            1139 => {
                c.pc = (2745, 1139);
                c.step()?;
                let v = 994u16;
                c.r[7] = v;
                c.pc = (2745, 1142);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 1143);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1144);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1149)?;
                label = 1149;
                continue;
            }
            1149 => {
                c.pc = (2745, 1149);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(552u16);
                c.pc = (2745, 1153);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1154);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1155);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(40u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1158);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 1162);
                c.step()?;
                c.invoke(p_1b55_44dc, true, 6997, 17628, 1167)?;
                label = 1167;
                continue;
            }
            1167 => {
                c.pc = (2745, 1167);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1172)?;
                label = 1172;
                continue;
            }
            1172 => {
                c.pc = (2745, 1172);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16);
                c.pc = (2745, 1175);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1176);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1177);
                c.step()?;
                let v = 35u16;
                c.push(v)?;
                c.pc = (2745, 1179);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1184)?;
                label = 1184;
                continue;
            }
            1184 => {
                c.pc = (2745, 1184);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1187);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1189);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 1191);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(36u16),
                    8,
                )?;
                let y = 2u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 1195);
                c.step()?;
                label = if !c.z { 1261 } else { 1197 };
                continue;
            }
            1197 => {
                c.pc = (2745, 1197);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 8)?;
                let v = c.dec(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 8, v)?;
                c.pc = (2745, 1200);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(296u16);
                c.pc = (2745, 1204);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1205);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1206);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16);
                c.pc = (2745, 1209);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1210);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1211);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 1216)?;
                label = 1216;
                continue;
            }
            1216 => {
                c.pc = (2745, 1216);
                c.step()?;
                let v = 994u16;
                c.r[7] = v;
                c.pc = (2745, 1219);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 1220);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1221);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1226)?;
                label = 1226;
                continue;
            }
            1226 => {
                c.pc = (2745, 1226);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(552u16);
                c.pc = (2745, 1230);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1231);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1232);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(40u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1235);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(3u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 1239);
                c.step()?;
                c.invoke(p_1b55_44dc, true, 6997, 17628, 1244)?;
                label = 1244;
                continue;
            }
            1244 => {
                c.pc = (2745, 1244);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1249)?;
                label = 1249;
                continue;
            }
            1249 => {
                c.pc = (2745, 1249);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16);
                c.pc = (2745, 1252);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1253);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1254);
                c.step()?;
                let v = 35u16;
                c.push(v)?;
                c.pc = (2745, 1256);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1261)?;
                label = 1261;
                continue;
            }
            1261 => {
                c.pc = (2745, 1261);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16);
                c.pc = (2745, 1264);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1265);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1266);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1269);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 1270);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1271);
                c.step()?;
                let v = 35u16;
                c.push(v)?;
                c.pc = (2745, 1273);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1278)?;
                label = 1278;
                continue;
            }
            1278 => {
                c.pc = (2745, 1278);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 1279);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_0577(c: &mut Cpu) -> Result<(), String> {
    let mut label = 1399u16;
    loop {
        match label {
            1399 => {
                c.pc = (2745, 1399);
                c.step()?;
                c.enter(1034)?;
                c.pc = (2745, 1403);
                c.step()?;
                c.invoke(p_24bc_126d, true, 9404, 4717, 1408)?;
                label = 1408;
                continue;
            }
            1408 => {
                c.pc = (2745, 1408);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1410);
                c.step()?;
                label = if !c.z { 1429 } else { 1412 };
                continue;
            }
            1412 => {
                c.pc = (2745, 1412);
                c.step()?;
                let v = 1282u16;
                c.r[7] = v;
                c.pc = (2745, 1415);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 1416);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1417);
                c.step()?;
                c.invoke(p_232a_06e5, true, 9002, 1765, 1422)?;
                label = 1422;
                continue;
            }
            1422 => {
                c.pc = (2745, 1422);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1424);
                c.step()?;
                label = if !c.z { 1429 } else { 1426 };
                continue;
            }
            1426 => {
                c.pc = (2745, 1426);
                c.step()?;
                label = 2489;
                continue;
            }
            1429 => {
                c.pc = (2745, 1429);
                c.step()?;
                let v = 25u16;
                c.push(v)?;
                c.pc = (2745, 1431);
                c.step()?;
                let v = 1324u16;
                c.r[7] = v;
                c.pc = (2745, 1434);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 1435);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1436);
                c.step()?;
                let v = 56664u16;
                c.r[7] = v;
                c.pc = (2745, 1439);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 1440);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1441);
                c.step()?;
                c.invoke(p_232a_0555, true, 9002, 1365, 1446)?;
                label = 1446;
                continue;
            }
            1446 => {
                c.pc = (2745, 1446);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 1448);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(57090u16), 16, v)?;
                c.pc = (2745, 1451);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(57092u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 1454);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 1457);
                c.step()?;
                let v = 4u16;
                c.write(c.r[11], 0u16.wrapping_add(57092u16), 16, v)?;
                c.pc = (2745, 1463);
                c.step()?;
                let v = 2416u16;
                c.r[7] = v;
                c.pc = (2745, 1466);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 1467);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1468);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 1470);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 1475)?;
                label = 1475;
                continue;
            }
            1475 => {
                c.pc = (2745, 1475);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8, v)?;
                c.pc = (2745, 1478);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(522u16);
                c.pc = (2745, 1482);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1483);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1484);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55014u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 1487);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 1490);
                c.step()?;
                let o = 0u16.wrapping_add(55218u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1494);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1496);
                c.step()?;
                let x = c.r[7];
                let y = 65520u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1500);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 1501);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1502);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 1507)?;
                label = 1507;
                continue;
            }
            1507 => {
                c.pc = (2745, 1507);
                c.step()?;
                let v = 1338u16;
                c.r[7] = v;
                c.pc = (2745, 1510);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 1511);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1512);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1517)?;
                label = 1517;
                continue;
            }
            1517 => {
                c.pc = (2745, 1517);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55012u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 1520);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 1523);
                c.step()?;
                let o = 0u16.wrapping_add(55218u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1527);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1529);
                c.step()?;
                let x = c.r[7];
                let y = 65520u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1533);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 1534);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1535);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1540)?;
                label = 1540;
                continue;
            }
            1540 => {
                c.pc = (2745, 1540);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(778u16);
                c.pc = (2745, 1544);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1545);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1546);
                c.step()?;
                let v = 2416u16;
                c.r[7] = v;
                c.pc = (2745, 1549);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 1550);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1551);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 1553);
                c.step()?;
                c.invoke(p_26c9_0c81, true, 9929, 3201, 1558)?;
                label = 1558;
                continue;
            }
            1558 => {
                c.pc = (2745, 1558);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1563)?;
                label = 1563;
                continue;
            }
            1563 => {
                c.pc = (2745, 1563);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 1567);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1568);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1569);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 1572);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1577)?;
                label = 1577;
                continue;
            }
            1577 => {
                c.pc = (2745, 1577);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(135u16);
                c.pc = (2745, 1581);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1582);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1583);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1587);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1589);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 1590);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 1595)?;
                label = 1595;
                continue;
            }
            1595 => {
                c.pc = (2745, 1595);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8, v)?;
                c.pc = (2745, 1598);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(778u16);
                c.pc = (2745, 1602);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1603);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1604);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(522u16);
                c.pc = (2745, 1608);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1609);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1610);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1614);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1617);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1619);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (2745, 1621);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 1623);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 1625);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 1627);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(55014u16), 16)?;
                let y = 174u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 1633);
                c.step()?;
                let o = 0u16.wrapping_add(55242u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1637);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1639);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1641);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(177u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1646);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 1647);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 1651);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1654);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 1655);
                c.step()?;
                c.invoke(p_1b55_142b, true, 6997, 5163, 1660)?;
                label = 1660;
                continue;
            }
            1660 => {
                c.pc = (2745, 1660);
                c.step()?;
                c.invoke(p_24bc_005a, true, 9404, 90, 1665)?;
                label = 1665;
                continue;
            }
            1665 => {
                c.pc = (2745, 1665);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(1034u16);
                c.pc = (2745, 1669);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1670);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1671);
                c.step()?;
                let v = 2416u16;
                c.r[7] = v;
                c.pc = (2745, 1674);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 1675);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1676);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 1678);
                c.step()?;
                c.invoke(p_26c9_0c81, true, 9929, 3201, 1683)?;
                label = 1683;
                continue;
            }
            1683 => {
                c.pc = (2745, 1683);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1688)?;
                label = 1688;
                continue;
            }
            1688 => {
                c.pc = (2745, 1688);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 1692);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1693);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1694);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 1697);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1702)?;
                label = 1702;
                continue;
            }
            1702 => {
                c.pc = (2745, 1702);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(135u16);
                c.pc = (2745, 1706);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1707);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1708);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1712);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1714);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 1715);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 1720)?;
                label = 1720;
                continue;
            }
            1720 => {
                c.pc = (2745, 1720);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1722);
                c.step()?;
                label = if c.z { 1740 } else { 1724 };
                continue;
            }
            1724 => {
                c.pc = (2745, 1724);
                c.step()?;
                let v = 2416u16;
                c.r[7] = v;
                c.pc = (2745, 1727);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 1728);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1729);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 1731);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 1736)?;
                label = 1736;
                continue;
            }
            1736 => {
                c.pc = (2745, 1736);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1738);
                c.step()?;
                label = if !c.z { 1744 } else { 1740 };
                continue;
            }
            1740 => {
                c.pc = (2745, 1740);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1742);
                c.step()?;
                label = 1746;
                continue;
            }
            1744 => {
                c.pc = (2745, 1744);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 1746;
                continue;
            }
            1746 => {
                c.pc = (2745, 1746);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8, v)?;
                c.pc = (2745, 1749);
                c.step()?;
                let v = 0u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(136u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 1754);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(896u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 1757);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(268u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 1761);
                c.step()?;
                let v = 1u16;
                c.r[0] = v;
                c.pc = (2745, 1764);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(268u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 1768);
                c.step()?;
                label = if c.cf || c.z { 1773 } else { 1770 };
                continue;
            }
            1770 => {
                c.pc = (2745, 1770);
                c.step()?;
                label = 2237;
                continue;
            }
            1773 => {
                c.pc = (2745, 1773);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 1776);
                c.step()?;
                label = 1781;
                continue;
            }
            1778 => {
                c.pc = (2745, 1778);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 1781;
                continue;
            }
            1781 => {
                c.pc = (2745, 1781);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 1784);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1786);
                c.step()?;
                let v = c.r[7];
                c.r[6] = v;
                c.pc = (2745, 1788);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1790);
                c.step()?;
                let x = c.r[7];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1792);
                c.step()?;
                let x = c.r[7];
                let y = 42282u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 1796);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 1797);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 1798);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                let y = 255u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 1802);
                c.step()?;
                label = if !c.z { 1812 } else { 1804 };
                continue;
            }
            1804 => {
                c.pc = (2745, 1804);
                c.step()?;
                let v = 1700u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 1809);
                c.step()?;
                label = 2225;
                continue;
            }
            1812 => {
                c.pc = (2745, 1812);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(784u16);
                c.pc = (2745, 1816);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1817);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1818);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 1822);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1823);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1824);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 1829)?;
                label = 1829;
                continue;
            }
            1829 => {
                c.pc = (2745, 1829);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(528u16);
                c.pc = (2745, 1833);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1834);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1835);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 1838);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 1839);
                c.step()?;
                c.invoke(p_0ab9_03e4, false, 2745, 996, 1842)?;
                label = 1842;
                continue;
            }
            1842 => {
                c.pc = (2745, 1842);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1847)?;
                label = 1847;
                continue;
            }
            1847 => {
                c.pc = (2745, 1847);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 1851);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1852);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1853);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 1856);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1861)?;
                label = 1861;
                continue;
            }
            1861 => {
                c.pc = (2745, 1861);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(896u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 1864);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 1865);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 1867);
                c.step()?;
                let v = c.r[2];
                c.r[3] = v;
                c.pc = (2745, 1869);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 1872);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 1874);
                c.step()?;
                let x = c.r[2];
                let y = c.r[3];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 1876);
                c.step()?;
                label = if c.sf != c.of { 1884 } else { 1878 };
                continue;
            }
            1878 => {
                c.pc = (2745, 1878);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 1925 } else { 1880 };
                continue;
            }
            1880 => {
                c.pc = (2745, 1880);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 1882);
                c.step()?;
                label = if !c.cf { 1925 } else { 1884 };
                continue;
            }
            1884 => {
                c.pc = (2745, 1884);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(528u16);
                c.pc = (2745, 1888);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1889);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1890);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 1894);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1895);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1896);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 1901)?;
                label = 1901;
                continue;
            }
            1901 => {
                c.pc = (2745, 1901);
                c.step()?;
                let v = 1347u16;
                c.r[7] = v;
                c.pc = (2745, 1904);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 1905);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1906);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 1911)?;
                label = 1911;
                continue;
            }
            1911 => {
                c.pc = (2745, 1911);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 1915);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1916);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1917);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 1920);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 1925)?;
                label = 1925;
                continue;
            }
            1925 => {
                c.pc = (2745, 1925);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16), 8)?;
                let y = 72u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 1930);
                c.step()?;
                label = if !c.cf { 1935 } else { 1932 };
                continue;
            }
            1932 => {
                c.pc = (2745, 1932);
                c.step()?;
                label = 2123;
                continue;
            }
            1935 => {
                c.pc = (2745, 1935);
                c.step()?;
                let v = 72u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8, v)?;
                label = 1939;
                continue;
            }
            1939 => {
                c.pc = (2745, 1939);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1942);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1944);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 1946);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(136u16),
                    8,
                )?;
                let y = 32u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 1951);
                c.step()?;
                label = if c.z { 1958 } else { 1953 };
                continue;
            }
            1953 => {
                c.pc = (2745, 1953);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                let v = c.dec(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8, v)?;
                c.pc = (2745, 1956);
                c.step()?;
                label = 1939;
                continue;
            }
            1958 => {
                c.pc = (2745, 1958);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(528u16);
                c.pc = (2745, 1962);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1963);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1964);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 1968);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 1969);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 1970);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1973);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1975);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 1976);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 1977);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1980);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1982);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 1984);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 1988);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 1990);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 1992);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 1993);
                c.step()?;
                c.invoke(p_26c9_0b91, true, 9929, 2961, 1998)?;
                label = 1998;
                continue;
            }
            1998 => {
                c.pc = (2745, 1998);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(266u16);
                c.pc = (2745, 2002);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2003);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2004);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 2007);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 2012)?;
                label = 2012;
                continue;
            }
            2012 => {
                c.pc = (2745, 2012);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2015);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 2017);
                c.step()?;
                let x = c.r[0];
                let v = c.dec(x, 16);
                c.r[0] = v;
                c.pc = (2745, 2018);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(136u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 2022);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(784u16);
                c.pc = (2745, 2026);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2027);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2028);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 2032);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2033);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2034);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 2039)?;
                label = 2039;
                continue;
            }
            2039 => {
                c.pc = (2745, 2039);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(528u16);
                c.pc = (2745, 2043);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2044);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2045);
                c.step()?;
                let v = 2416u16;
                c.r[7] = v;
                c.pc = (2745, 2048);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 2049);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2050);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 2052);
                c.step()?;
                c.invoke(p_26c9_0c81, true, 9929, 3201, 2057)?;
                label = 2057;
                continue;
            }
            2057 => {
                c.pc = (2745, 2057);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2062)?;
                label = 2062;
                continue;
            }
            2062 => {
                c.pc = (2745, 2062);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 2066);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2067);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2068);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 2071);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 2076)?;
                label = 2076;
                continue;
            }
            2076 => {
                c.pc = (2745, 2076);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 2080);
                c.step()?;
                label = if c.z { 2103 } else { 2082 };
                continue;
            }
            2082 => {
                c.pc = (2745, 2082);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(135u16);
                c.pc = (2745, 2086);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2087);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2088);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2092);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 2094);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 2095);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 2100)?;
                label = 2100;
                continue;
            }
            2100 => {
                c.pc = (2745, 2100);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8, v)?;
                label = 2103;
                continue;
            }
            2103 => {
                c.pc = (2745, 2103);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(266u16);
                c.pc = (2745, 2107);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2108);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2109);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 2113);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2114);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2115);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 2118);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 2123)?;
                label = 2123;
                continue;
            }
            2123 => {
                c.pc = (2745, 2123);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(896u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 2126);
                c.step()?;
                c.r[2] = if (c.r[0] as i16) < 0 { 65535 } else { 0 };
                c.pc = (2745, 2127);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 2129);
                c.step()?;
                let v = c.r[2];
                c.r[3] = v;
                c.pc = (2745, 2131);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 2134);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 2136);
                c.step()?;
                let x = c.r[2];
                let y = c.r[3];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 2138);
                c.step()?;
                label = if !c.z { 2225 } else { 2140 };
                continue;
            }
            2140 => {
                c.pc = (2745, 2140);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 2142);
                c.step()?;
                label = if !c.z { 2225 } else { 2144 };
                continue;
            }
            2144 => {
                c.pc = (2745, 2144);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(784u16);
                c.pc = (2745, 2148);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2149);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2150);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 2154);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2155);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2156);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 2161)?;
                label = 2161;
                continue;
            }
            2161 => {
                c.pc = (2745, 2161);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(528u16);
                c.pc = (2745, 2165);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2166);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2167);
                c.step()?;
                let v = 2416u16;
                c.r[7] = v;
                c.pc = (2745, 2170);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 2171);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2172);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 2174);
                c.step()?;
                c.invoke(p_26c9_0c81, true, 9929, 3201, 2179)?;
                label = 2179;
                continue;
            }
            2179 => {
                c.pc = (2745, 2179);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2184)?;
                label = 2184;
                continue;
            }
            2184 => {
                c.pc = (2745, 2184);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 2188);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2189);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2190);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 2193);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 2198)?;
                label = 2198;
                continue;
            }
            2198 => {
                c.pc = (2745, 2198);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 2202);
                c.step()?;
                label = if c.z { 2225 } else { 2204 };
                continue;
            }
            2204 => {
                c.pc = (2745, 2204);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(135u16);
                c.pc = (2745, 2208);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2209);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2210);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2214);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 2216);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 2217);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 2222)?;
                label = 2222;
                continue;
            }
            2222 => {
                c.pc = (2745, 2222);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8, v)?;
                label = 2225;
                continue;
            }
            2225 => {
                c.pc = (2745, 2225);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 2228);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(268u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 2232);
                c.step()?;
                label = if c.z { 2237 } else { 2234 };
                continue;
            }
            2234 => {
                c.pc = (2745, 2234);
                c.step()?;
                label = 1778;
                continue;
            }
            2237 => {
                c.pc = (2745, 2237);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 2241);
                c.step()?;
                label = if c.z { 2258 } else { 2243 };
                continue;
            }
            2243 => {
                c.pc = (2745, 2243);
                c.step()?;
                let v = 2416u16;
                c.r[7] = v;
                c.pc = (2745, 2246);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 2247);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2248);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 2250);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 2255)?;
                label = 2255;
                continue;
            }
            2255 => {
                c.pc = (2745, 2255);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8, v)?;
                label = 2258;
                continue;
            }
            2258 => {
                c.pc = (2745, 2258);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(522u16);
                c.pc = (2745, 2262);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2263);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2264);
                c.step()?;
                let v = 1351u16;
                c.r[7] = v;
                c.pc = (2745, 2267);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 2268);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2269);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 2274)?;
                label = 2274;
                continue;
            }
            2274 => {
                c.pc = (2745, 2274);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55014u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 2277);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 2280);
                c.step()?;
                let o = 0u16.wrapping_add(55218u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 2284);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 2286);
                c.step()?;
                let x = c.r[7];
                let y = 65520u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 2290);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 2291);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2292);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2297)?;
                label = 2297;
                continue;
            }
            2297 => {
                c.pc = (2745, 2297);
                c.step()?;
                let v = 1365u16;
                c.r[7] = v;
                c.pc = (2745, 2300);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 2301);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2302);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2307)?;
                label = 2307;
                continue;
            }
            2307 => {
                c.pc = (2745, 2307);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(778u16);
                c.pc = (2745, 2311);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2312);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2313);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53445u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2316);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 2318);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 2320);
                c.step()?;
                let v = c.r[2];
                c.push(v)?;
                c.pc = (2745, 2321);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 2322);
                c.step()?;
                c.invoke(p_232a_07c9, true, 9002, 1993, 2327)?;
                label = 2327;
                continue;
            }
            2327 => {
                c.pc = (2745, 2327);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2332)?;
                label = 2332;
                continue;
            }
            2332 => {
                c.pc = (2745, 2332);
                c.step()?;
                let v = 1365u16;
                c.r[7] = v;
                c.pc = (2745, 2335);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 2336);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2337);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2342)?;
                label = 2342;
                continue;
            }
            2342 => {
                c.pc = (2745, 2342);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55012u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 2345);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 2348);
                c.step()?;
                let o = 0u16.wrapping_add(55218u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 2352);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 2354);
                c.step()?;
                let x = c.r[7];
                let y = 65520u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 2358);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 2359);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2360);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2365)?;
                label = 2365;
                continue;
            }
            2365 => {
                c.pc = (2745, 2365);
                c.step()?;
                let v = 1365u16;
                c.r[7] = v;
                c.pc = (2745, 2368);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 2369);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2370);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2375)?;
                label = 2375;
                continue;
            }
            2375 => {
                c.pc = (2745, 2375);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(1034u16);
                c.pc = (2745, 2379);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2380);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2381);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2384);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 2386);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 2388);
                c.step()?;
                let v = c.r[2];
                c.push(v)?;
                c.pc = (2745, 2389);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 2390);
                c.step()?;
                c.invoke(p_232a_07c9, true, 9002, 1993, 2395)?;
                label = 2395;
                continue;
            }
            2395 => {
                c.pc = (2745, 2395);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 2400)?;
                label = 2400;
                continue;
            }
            2400 => {
                c.pc = (2745, 2400);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16);
                c.pc = (2745, 2404);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2405);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2406);
                c.step()?;
                let v = 128u16;
                c.push(v)?;
                c.pc = (2745, 2409);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 2414)?;
                label = 2414;
                continue;
            }
            2414 => {
                c.pc = (2745, 2414);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 2418);
                c.step()?;
                label = if c.z { 2467 } else { 2420 };
                continue;
            }
            2420 => {
                c.pc = (2745, 2420);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(135u16);
                c.pc = (2745, 2424);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 2425);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2426);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(136u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2430);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 2432);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 2433);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 2438)?;
                label = 2438;
                continue;
            }
            2438 => {
                c.pc = (2745, 2438);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2440);
                c.step()?;
                label = if c.z { 2458 } else { 2442 };
                continue;
            }
            2442 => {
                c.pc = (2745, 2442);
                c.step()?;
                let v = 2418u16;
                c.r[7] = v;
                c.pc = (2745, 2445);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 2446);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2447);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 2449);
                c.step()?;
                c.invoke(p_232a_11e3, true, 9002, 4579, 2454)?;
                label = 2454;
                continue;
            }
            2454 => {
                c.pc = (2745, 2454);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2456);
                c.step()?;
                label = if !c.z { 2462 } else { 2458 };
                continue;
            }
            2458 => {
                c.pc = (2745, 2458);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2460);
                c.step()?;
                label = 2464;
                continue;
            }
            2462 => {
                c.pc = (2745, 2462);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 2464;
                continue;
            }
            2464 => {
                c.pc = (2745, 2464);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8, v)?;
                label = 2467;
                continue;
            }
            2467 => {
                c.pc = (2745, 2467);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 2471);
                c.step()?;
                label = if !c.z { 2483 } else { 2473 };
                continue;
            }
            2473 => {
                c.pc = (2745, 2473);
                c.step()?;
                let v = 1367u16;
                c.r[7] = v;
                c.pc = (2745, 2476);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 2477);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2478);
                c.step()?;
                c.invoke(p_24bc_0702, true, 9404, 1794, 2483)?;
                label = 2483;
                continue;
            }
            2483 => {
                c.pc = (2745, 2483);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 2486);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(57092u16), 16, v)?;
                label = 2489;
                continue;
            }
            2489 => {
                c.pc = (2745, 2489);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 2490);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_0b53(c: &mut Cpu) -> Result<(), String> {
    let mut label = 2899u16;
    loop {
        match label {
            2899 => {
                c.pc = (2745, 2899);
                c.step()?;
                c.enter(14)?;
                c.pc = (2745, 2903);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(896u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 2908);
                c.step()?;
                label = if !c.z { 2913 } else { 2910 };
                continue;
            }
            2910 => {
                c.pc = (2745, 2910);
                c.step()?;
                label = 3452;
                continue;
            }
            2913 => {
                c.pc = (2745, 2913);
                c.step()?;
                let o = 0u16.wrapping_add(53386u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 2917);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 2918);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2919);
                c.step()?;
                let o = 0u16.wrapping_add(53398u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 2923);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 2924);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2925);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 2928);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 2933)?;
                label = 2933;
                continue;
            }
            2933 => {
                c.pc = (2745, 2933);
                c.step()?;
                let v = 6684u16;
                c.r[0] = v;
                c.pc = (2745, 2936);
                c.step()?;
                let v = c.r[11];
                c.r[2] = v;
                c.pc = (2745, 2938);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 2941);
                c.step()?;
                let v = c.r[2];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 2944);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53063u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 2949);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 2951);
                c.step()?;
                label = if c.sf == c.of { 2954 } else { 2953 };
                continue;
            }
            2953 => {
                c.pc = (2745, 2953);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 2954;
                continue;
            }
            2954 => {
                c.pc = (2745, 2954);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 2955);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 2957);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 2959);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2422u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 2963);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 2966);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]), 16, v)?;
                c.pc = (2745, 2969);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(896u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 2972);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 2975);
                c.step()?;
                let v = 6684u16;
                c.r[7] = v;
                c.pc = (2745, 2978);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 2979);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2980);
                c.step()?;
                let v = 56664u16;
                c.r[7] = v;
                c.pc = (2745, 2983);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 2984);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2985);
                c.step()?;
                c.invoke(p_232a_052f, true, 9002, 1327, 2990)?;
                label = 2990;
                continue;
            }
            2990 => {
                c.pc = (2745, 2990);
                c.step()?;
                let v = 6786u16;
                c.r[7] = v;
                c.pc = (2745, 2993);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 2994);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 2995);
                c.step()?;
                let v = 56664u16;
                c.r[7] = v;
                c.pc = (2745, 2998);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 2999);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3000);
                c.step()?;
                c.invoke(p_232a_052f, true, 9002, 1327, 3005)?;
                label = 3005;
                continue;
            }
            3005 => {
                c.pc = (2745, 3005);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3008);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 3011);
                c.step()?;
                let v = 17u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 3016);
                c.step()?;
                label = 3021;
                continue;
            }
            3018 => {
                c.pc = (2745, 3018);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let v = c.dec(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                label = 3021;
                continue;
            }
            3021 => {
                c.pc = (2745, 3021);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 3024);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 3026);
                c.step()?;
                let v = c.r[7];
                c.r[6] = v;
                c.pc = (2745, 3028);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 3030);
                c.step()?;
                let x = c.r[7];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 3032);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23254u16), 8)?;
                let y = 255u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3037);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 3039);
                c.step()?;
                label = if !c.z { 3042 } else { 3041 };
                continue;
            }
            3041 => {
                c.pc = (2745, 3041);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 3042;
                continue;
            }
            3042 => {
                c.pc = (2745, 3042);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(11u16), 8, v)?;
                c.pc = (2745, 3045);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(11u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3049);
                c.step()?;
                label = if c.z { 3099 } else { 3051 };
                continue;
            }
            3051 => {
                c.pc = (2745, 3051);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 3053);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 3056);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3059);
                c.step()?;
                let x = c.r[0];
                let y = 399u16;
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 3062);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 3063);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 3066);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3069);
                c.step()?;
                let x = c.r[0];
                let y = 367u16;
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 3072);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 3073);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3076);
                c.step()?;
                let x = c.r[0];
                let y = 6u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 3079);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 3080);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3083);
                c.step()?;
                let x = c.r[0];
                let y = 6u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 3086);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 3087);
                c.step()?;
                let v = 56664u16;
                c.r[7] = v;
                c.pc = (2745, 3090);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 3091);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3092);
                c.step()?;
                c.invoke(p_232a_0507, true, 9002, 1287, 3097)?;
                label = 3097;
                continue;
            }
            3097 => {
                c.pc = (2745, 3097);
                c.step()?;
                label = 3139;
                continue;
            }
            3099 => {
                c.pc = (2745, 3099);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 3102);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3105);
                c.step()?;
                let x = c.r[0];
                let y = 6u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 3108);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 3109);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 3112);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3115);
                c.step()?;
                let x = c.r[0];
                let y = 399u16;
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 3118);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 3119);
                c.step()?;
                c.invoke(p_0ab9_09bd, false, 2745, 2493, 3122)?;
                label = 3122;
                continue;
            }
            3122 => {
                c.pc = (2745, 3122);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                c.pc = (2745, 3125);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 3126);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3127);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 3129);
                c.step()?;
                let v = 1700u16;
                c.push(v)?;
                c.pc = (2745, 3132);
                c.step()?;
                let v = 65535u16;
                c.push(v)?;
                c.pc = (2745, 3134);
                c.step()?;
                c.invoke(p_24bc_042b, true, 9404, 1067, 3139)?;
                label = 3139;
                continue;
            }
            3139 => {
                c.pc = (2745, 3139);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 3143);
                c.step()?;
                label = if !c.z { 3018 } else { 3145 };
                continue;
            }
            3145 => {
                c.pc = (2745, 3145);
                c.step()?;
                c.invoke(p_232a_099b, true, 9002, 2459, 3150)?;
                label = 3150;
                continue;
            }
            3150 => {
                c.pc = (2745, 3150);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 3153);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 5u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3155);
                c.step()?;
                label = if c.z { 3161 } else { 3157 };
                continue;
            }
            3157 => {
                c.pc = (2745, 3157);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 11u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3159);
                c.step()?;
                label = if !c.z { 3233 } else { 3161 };
                continue;
            }
            3161 => {
                c.pc = (2745, 3161);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(11u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3165);
                c.step()?;
                label = if !c.z { 3230 } else { 3167 };
                continue;
            }
            3167 => {
                c.pc = (2745, 3167);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                let y = 11u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3172);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 3174);
                c.step()?;
                label = if !c.z { 3177 } else { 3176 };
                continue;
            }
            3176 => {
                c.pc = (2745, 3176);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 3177;
                continue;
            }
            3177 => {
                c.pc = (2745, 3177);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 3178);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 3180);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2420u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 3184);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 3186);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(14u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 3189);
                c.step()?;
                let v = 1u16;
                c.r[0] = v;
                c.pc = (2745, 3192);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(14u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 3195);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 3230 } else { 3197 };
                continue;
            }
            3197 => {
                c.pc = (2745, 3197);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 3200);
                c.step()?;
                label = 3205;
                continue;
            }
            3202 => {
                c.pc = (2745, 3202);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                label = 3205;
                continue;
            }
            3205 => {
                c.pc = (2745, 3205);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16);
                c.pc = (2745, 3208);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 3209);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3210);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 3212);
                c.step()?;
                let v = 1700u16;
                c.push(v)?;
                c.pc = (2745, 3215);
                c.step()?;
                let v = 65535u16;
                c.push(v)?;
                c.pc = (2745, 3217);
                c.step()?;
                c.invoke(p_24bc_042b, true, 9404, 1067, 3222)?;
                label = 3222;
                continue;
            }
            3222 => {
                c.pc = (2745, 3222);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3225);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(14u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 3228);
                c.step()?;
                label = if !c.z { 3202 } else { 3230 };
                continue;
            }
            3230 => {
                c.pc = (2745, 3230);
                c.step()?;
                label = 3410;
                continue;
            }
            3233 => {
                c.pc = (2745, 3233);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 6u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3235);
                c.step()?;
                label = if c.z { 3241 } else { 3237 };
                continue;
            }
            3237 => {
                c.pc = (2745, 3237);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 12u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3239);
                c.step()?;
                label = if !c.z { 3315 } else { 3241 };
                continue;
            }
            3241 => {
                c.pc = (2745, 3241);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                let y = 12u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3246);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 3248);
                c.step()?;
                label = if !c.z { 3251 } else { 3250 };
                continue;
            }
            3250 => {
                c.pc = (2745, 3250);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 3251;
                continue;
            }
            3251 => {
                c.pc = (2745, 3251);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 3252);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 3254);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(2420u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 3258);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 3260);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(14u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 3263);
                c.step()?;
                let v = 1u16;
                c.r[0] = v;
                c.pc = (2745, 3266);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(14u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 3269);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 3313 } else { 3271 };
                continue;
            }
            3271 => {
                c.pc = (2745, 3271);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 3274);
                c.step()?;
                label = 3279;
                continue;
            }
            3276 => {
                c.pc = (2745, 3276);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                label = 3279;
                continue;
            }
            3279 => {
                c.pc = (2745, 3279);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3282);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[11], 0u16.wrapping_add(896u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 3286);
                c.step()?;
                label = if c.z { 3305 } else { 3288 };
                continue;
            }
            3288 => {
                c.pc = (2745, 3288);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16);
                c.pc = (2745, 3291);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 3292);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3293);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 3295);
                c.step()?;
                let v = 1700u16;
                c.push(v)?;
                c.pc = (2745, 3298);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 3300);
                c.step()?;
                c.invoke(p_24bc_042b, true, 9404, 1067, 3305)?;
                label = 3305;
                continue;
            }
            3305 => {
                c.pc = (2745, 3305);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3308);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(14u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 3311);
                c.step()?;
                label = if !c.z { 3276 } else { 3313 };
                continue;
            }
            3313 => {
                c.pc = (2745, 3313);
                c.step()?;
                label = 3410;
                continue;
            }
            3315 => {
                c.pc = (2745, 3315);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 9u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3317);
                c.step()?;
                label = if !c.z { 3400 } else { 3319 };
                continue;
            }
            3319 => {
                c.pc = (2745, 3319);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 3322);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 3324);
                c.step()?;
                let v = c.r[7];
                c.r[6] = v;
                c.pc = (2745, 3326);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 3328);
                c.step()?;
                let x = c.r[7];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 3330);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23254u16), 8)?;
                let y = 255u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3335);
                c.step()?;
                label = if c.z { 3356 } else { 3337 };
                continue;
            }
            3337 => {
                c.pc = (2745, 3337);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16);
                c.pc = (2745, 3340);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 3341);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3342);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 3344);
                c.step()?;
                let v = 1700u16;
                c.push(v)?;
                c.pc = (2745, 3347);
                c.step()?;
                let v = 65535u16;
                c.push(v)?;
                c.pc = (2745, 3349);
                c.step()?;
                c.invoke(p_24bc_042b, true, 9404, 1067, 3354)?;
                label = 3354;
                continue;
            }
            3354 => {
                c.pc = (2745, 3354);
                c.step()?;
                label = 3319;
                continue;
            }
            3356 => {
                c.pc = (2745, 3356);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 3361);
                c.step()?;
                label = 3366;
                continue;
            }
            3363 => {
                c.pc = (2745, 3363);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                label = 3366;
                continue;
            }
            3366 => {
                c.pc = (2745, 3366);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3369);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[11], 0u16.wrapping_add(896u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 3373);
                c.step()?;
                label = if c.z { 3392 } else { 3375 };
                continue;
            }
            3375 => {
                c.pc = (2745, 3375);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16);
                c.pc = (2745, 3378);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 3379);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3380);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 3382);
                c.step()?;
                let v = 1700u16;
                c.push(v)?;
                c.pc = (2745, 3385);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 3387);
                c.step()?;
                c.invoke(p_24bc_042b, true, 9404, 1067, 3392)?;
                label = 3392;
                continue;
            }
            3392 => {
                c.pc = (2745, 3392);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let y = 17u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 3396);
                c.step()?;
                label = if !c.z { 3363 } else { 3398 };
                continue;
            }
            3398 => {
                c.pc = (2745, 3398);
                c.step()?;
                label = 3410;
                continue;
            }
            3400 => {
                c.pc = (2745, 3400);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 10u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3402);
                c.step()?;
                label = if !c.z { 3410 } else { 3404 };
                continue;
            }
            3404 => {
                c.pc = (2745, 3404);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(896u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 3407);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 3410;
                continue;
            }
            3410 => {
                c.pc = (2745, 3410);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 3413);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3415);
                c.step()?;
                label = if !c.cf { 3420 } else { 3417 };
                continue;
            }
            3417 => {
                c.pc = (2745, 3417);
                c.step()?;
                label = 3005;
                continue;
            }
            3420 => {
                c.pc = (2745, 3420);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 2u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 3422);
                c.step()?;
                label = if c.cf || c.z { 3427 } else { 3424 };
                continue;
            }
            3424 => {
                c.pc = (2745, 3424);
                c.step()?;
                label = 3005;
                continue;
            }
            3427 => {
                c.pc = (2745, 3427);
                c.step()?;
                let o = 0u16.wrapping_add(53398u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 3431);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 3432);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3433);
                c.step()?;
                let o = 0u16.wrapping_add(53386u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 3437);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 3438);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 3439);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 3442);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 3447)?;
                label = 3447;
                continue;
            }
            3447 => {
                c.pc = (2745, 3447);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(56968u16), 8, v)?;
                label = 3452;
                continue;
            }
            3452 => {
                c.pc = (2745, 3452);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 3453);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_759a(c: &mut Cpu) -> Result<(), String> {
    let mut label = 30106u16;
    loop {
        match label {
            30106 => {
                c.pc = (2745, 30106);
                c.step()?;
                c.enter(4)?;
                c.pc = (2745, 30110);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30113);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30114);
                c.step()?;
                let x = c.r[0];
                let y = 7u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 30117);
                c.step()?;
                let x = c.r[7];
                let y = 53038u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 30121);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 30122);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 30123);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 30126);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 30129);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30132);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30133);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 30135);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 30139);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 30141);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 30142);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 30145);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 30149);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 30150);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 30151);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 30152);
                c.step()?;
                c.invoke(p_0ab9_1ecc, false, 2745, 7884, 30155)?;
                label = 30155;
                continue;
            }
            30155 => {
                c.pc = (2745, 30155);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 30156);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 30159);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 30162);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]), 8, v)?;
                c.pc = (2745, 30165);
                c.step()?;
                let v = 0u16;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8, v)?;
                c.pc = (2745, 30170);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 16)?;
                c.r[0] = v;
                c.pc = (2745, 30173);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 16, v)?;
                c.pc = (2745, 30177);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 30178);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_75e5(c: &mut Cpu) -> Result<(), String> {
    let mut label = 30181u16;
    loop {
        match label {
            30181 => {
                c.pc = (2745, 30181);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 30182);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 30184);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 30186);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 30187);
                c.step()?;
                c.invoke(p_0ab9_759a, false, 2745, 30106, 30190)?;
                label = 30190;
                continue;
            }
            30190 => {
                c.pc = (2745, 30190);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 30192);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 30193);
                c.step()?;
                c.invoke(p_0ab9_759a, false, 2745, 30106, 30196)?;
                label = 30196;
                continue;
            }
            30196 => {
                c.pc = (2745, 30196);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53053u16), 8, v)?;
                c.pc = (2745, 30201);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53052u16), 8, v)?;
                c.pc = (2745, 30206);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53052u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 30209);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53054u16), 16, v)?;
                c.pc = (2745, 30212);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53052u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 30215);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53059u16), 16, v)?;
                c.pc = (2745, 30218);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 30219);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7c3f(c: &mut Cpu) -> Result<(), String> {
    let mut label = 31807u16;
    loop {
        match label {
            31807 => {
                c.pc = (2745, 31807);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 31811);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53434u16), 8, v)?;
                c.pc = (2745, 31816);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53409u16), 8, v)?;
                c.pc = (2745, 31821);
                c.step()?;
                c.invoke(p_0ab9_6b2f, false, 2745, 27439, 31824)?;
                label = 31824;
                continue;
            }
            31824 => {
                c.pc = (2745, 31824);
                c.step()?;
                c.invoke(p_0ab9_1d3a, false, 2745, 7482, 31827)?;
                label = 31827;
                continue;
            }
            31827 => {
                c.pc = (2745, 31827);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31832);
                c.step()?;
                label = if !c.z { 31842 } else { 31834 };
                continue;
            }
            31834 => {
                c.pc = (2745, 31834);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 31836);
                c.step()?;
                c.invoke(p_0ab9_1c7e, false, 2745, 7294, 31839)?;
                label = 31839;
                continue;
            }
            31839 => {
                c.pc = (2745, 31839);
                c.step()?;
                c.invoke(p_0ab9_75e5, false, 2745, 30181, 31842)?;
                label = 31842;
                continue;
            }
            31842 => {
                c.pc = (2745, 31842);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53075u16), 8, v)?;
                label = 31847;
                continue;
            }
            31847 => {
                c.pc = (2745, 31847);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31852);
                c.step()?;
                label = if c.z { 31859 } else { 31854 };
                continue;
            }
            31854 => {
                c.pc = (2745, 31854);
                c.step()?;
                c.invoke(p_0ab9_dd02, false, 2745, 56578, 31857)?;
                label = 31857;
                continue;
            }
            31857 => {
                c.pc = (2745, 31857);
                c.step()?;
                label = 31886;
                continue;
            }
            31859 => {
                c.pc = (2745, 31859);
                c.step()?;
                c.invoke(p_0ab9_7a02, false, 2745, 31234, 31862)?;
                label = 31862;
                continue;
            }
            31862 => {
                c.pc = (2745, 31862);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31867);
                c.step()?;
                label = if !c.z { 31886 } else { 31869 };
                continue;
            }
            31869 => {
                c.pc = (2745, 31869);
                c.step()?;
                c.invoke(p_0ab9_033e, false, 2745, 830, 31872)?;
                label = 31872;
                continue;
            }
            31872 => {
                c.pc = (2745, 31872);
                c.step()?;
                c.invoke(p_0ab9_760c, false, 2745, 30220, 31875)?;
                label = 31875;
                continue;
            }
            31875 => {
                c.pc = (2745, 31875);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 31877);
                c.step()?;
                let v = 53052u16;
                c.r[7] = v;
                c.pc = (2745, 31880);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31881);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31882);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31883);
                c.step()?;
                c.invoke(p_0ab9_770e, false, 2745, 30478, 31886)?;
                label = 31886;
                continue;
            }
            31886 => {
                c.pc = (2745, 31886);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 31887);
                c.step()?;
                c.invoke(p_0ab9_7b7b, false, 2745, 31611, 31890)?;
                label = 31890;
                continue;
            }
            31890 => {
                c.pc = (2745, 31890);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31892);
                c.step()?;
                label = if c.z { 31847 } else { 31894 };
                continue;
            }
            31894 => {
                c.pc = (2745, 31894);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31899);
                c.step()?;
                label = if !c.z { 31944 } else { 31901 };
                continue;
            }
            31901 => {
                c.pc = (2745, 31901);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 31905);
                c.step()?;
                label = 31910;
                continue;
            }
            31907 => {
                c.pc = (2745, 31907);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 31910;
                continue;
            }
            31910 => {
                c.pc = (2745, 31910);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31913);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 31914);
                c.step()?;
                let x = c.r[0];
                let y = 7u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 31917);
                c.step()?;
                let x = c.r[7];
                let y = 53038u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 31921);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31922);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31923);
                c.step()?;
                c.invoke(p_1998_093f, true, 6552, 2367, 31928)?;
                label = 31928;
                continue;
            }
            31928 => {
                c.pc = (2745, 31928);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31932);
                c.step()?;
                label = if !c.z { 31907 } else { 31934 };
                continue;
            }
            31934 => {
                c.pc = (2745, 31934);
                c.step()?;
                let v = 53052u16;
                c.r[7] = v;
                c.pc = (2745, 31937);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31938);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31939);
                c.step()?;
                c.invoke(p_1998_093f, true, 6552, 2367, 31944)?;
                label = 31944;
                continue;
            }
            31944 => {
                c.pc = (2745, 31944);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53432u16), 8, v)?;
                c.pc = (2745, 31949);
                c.step()?;
                c.invoke(p_232a_022e, true, 9002, 558, 31954)?;
                label = 31954;
                continue;
            }
            31954 => {
                c.pc = (2745, 31954);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 31955);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7c67(c: &mut Cpu) -> Result<(), String> {
    let mut label = 31847u16;
    loop {
        match label {
            31847 => {
                c.pc = (2745, 31847);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31852);
                c.step()?;
                label = if c.z { 31859 } else { 31854 };
                continue;
            }
            31854 => {
                c.pc = (2745, 31854);
                c.step()?;
                c.invoke(p_0ab9_dd02, false, 2745, 56578, 31857)?;
                label = 31857;
                continue;
            }
            31857 => {
                c.pc = (2745, 31857);
                c.step()?;
                label = 31886;
                continue;
            }
            31859 => {
                c.pc = (2745, 31859);
                c.step()?;
                c.invoke(p_0ab9_7a02, false, 2745, 31234, 31862)?;
                label = 31862;
                continue;
            }
            31862 => {
                c.pc = (2745, 31862);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31867);
                c.step()?;
                label = if !c.z { 31886 } else { 31869 };
                continue;
            }
            31869 => {
                c.pc = (2745, 31869);
                c.step()?;
                c.invoke(p_0ab9_033e, false, 2745, 830, 31872)?;
                label = 31872;
                continue;
            }
            31872 => {
                c.pc = (2745, 31872);
                c.step()?;
                c.invoke(p_0ab9_760c, false, 2745, 30220, 31875)?;
                label = 31875;
                continue;
            }
            31875 => {
                c.pc = (2745, 31875);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 31877);
                c.step()?;
                let v = 53052u16;
                c.r[7] = v;
                c.pc = (2745, 31880);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31881);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31882);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 31883);
                c.step()?;
                c.invoke(p_0ab9_770e, false, 2745, 30478, 31886)?;
                label = 31886;
                continue;
            }
            31886 => {
                c.pc = (2745, 31886);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 31887);
                c.step()?;
                c.invoke(p_0ab9_7b7b, false, 2745, 31611, 31890)?;
                label = 31890;
                continue;
            }
            31890 => {
                c.pc = (2745, 31890);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31892);
                c.step()?;
                label = if c.z { 31847 } else { 31894 };
                continue;
            }
            31894 => {
                c.pc = (2745, 31894);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31899);
                c.step()?;
                label = if !c.z { 31944 } else { 31901 };
                continue;
            }
            31901 => {
                c.pc = (2745, 31901);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 31905);
                c.step()?;
                label = 31910;
                continue;
            }
            31907 => {
                c.pc = (2745, 31907);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 31910;
                continue;
            }
            31910 => {
                c.pc = (2745, 31910);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31913);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 31914);
                c.step()?;
                let x = c.r[0];
                let y = 7u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 31917);
                c.step()?;
                let x = c.r[7];
                let y = 53038u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 31921);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31922);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31923);
                c.step()?;
                c.invoke(p_1998_093f, true, 6552, 2367, 31928)?;
                label = 31928;
                continue;
            }
            31928 => {
                c.pc = (2745, 31928);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31932);
                c.step()?;
                label = if !c.z { 31907 } else { 31934 };
                continue;
            }
            31934 => {
                c.pc = (2745, 31934);
                c.step()?;
                let v = 53052u16;
                c.r[7] = v;
                c.pc = (2745, 31937);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 31938);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 31939);
                c.step()?;
                c.invoke(p_1998_093f, true, 6552, 2367, 31944)?;
                label = 31944;
                continue;
            }
            31944 => {
                c.pc = (2745, 31944);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53432u16), 8, v)?;
                c.pc = (2745, 31949);
                c.step()?;
                c.invoke(p_232a_022e, true, 9002, 558, 31954)?;
                label = 31954;
                continue;
            }
            31954 => {
                c.pc = (2745, 31954);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 31955);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7cd4(c: &mut Cpu) -> Result<(), String> {
    let mut label = 31956u16;
    loop {
        match label {
            31956 => {
                c.pc = (2745, 31956);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 31960);
                c.step()?;
                let v = 10u16;
                c.push(v)?;
                c.pc = (2745, 31962);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 31967)?;
                label = 31967;
                continue;
            }
            31967 => {
                c.pc = (2745, 31967);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 31968);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 31970);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53445u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31973);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[11], 0u16.wrapping_add(53444u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 31977);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31979);
                c.step()?;
                label = if c.cf || c.z { 31982 } else { 31981 };
                continue;
            }
            31981 => {
                c.pc = (2745, 31981);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 31982;
                continue;
            }
            31982 => {
                c.pc = (2745, 31982);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 31983);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 31985);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 31989);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 31991);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 31993);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 31994);
                c.step()?;
                let v = 3u16;
                c.push(v)?;
                c.pc = (2745, 31996);
                c.step()?;
                c.invoke(p_0ab9_57e3, false, 2745, 22499, 31999)?;
                label = 31999;
                continue;
            }
            31999 => {
                c.pc = (2745, 31999);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53432u16), 8, v)?;
                c.pc = (2745, 32004);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 32009);
                c.step()?;
                label = 32014;
                continue;
            }
            32011 => {
                c.pc = (2745, 32011);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 32014;
                continue;
            }
            32014 => {
                c.pc = (2745, 32014);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 32018);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32022);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 32023);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 32024);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 32027);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 32030);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32031);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32032);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 32033);
                c.step()?;
                c.invoke(p_0ab9_1e4e, false, 2745, 7758, 32036)?;
                label = 32036;
                continue;
            }
            32036 => {
                c.pc = (2745, 32036);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32038);
                c.step()?;
                label = if c.z { 32048 } else { 32040 };
                continue;
            }
            32040 => {
                c.pc = (2745, 32040);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32043);
                c.step()?;
                let v = 6u16;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(19u16), 8, v)?;
                label = 32048;
                continue;
            }
            32048 => {
                c.pc = (2745, 32048);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 32052);
                c.step()?;
                label = if !c.z { 32011 } else { 32054 };
                continue;
            }
            32054 => {
                c.pc = (2745, 32054);
                c.step()?;
                c.invoke(p_0ab9_4f95, false, 2745, 20373, 32057)?;
                label = 32057;
                continue;
            }
            32057 => {
                c.pc = (2745, 32057);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32062);
                c.step()?;
                label = if !c.z { 32076 } else { 32064 };
                continue;
            }
            32064 => {
                c.pc = (2745, 32064);
                c.step()?;
                c.invoke(p_0ab9_7a02, false, 2745, 31234, 32067)?;
                label = 32067;
                continue;
            }
            32067 => {
                c.pc = (2745, 32067);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 32069);
                c.step()?;
                c.invoke(p_0ab9_1c1e, false, 2745, 7198, 32072)?;
                label = 32072;
                continue;
            }
            32072 => {
                c.pc = (2745, 32072);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32074);
                c.step()?;
                label = if c.z { 32064 } else { 32076 };
                continue;
            }
            32076 => {
                c.pc = (2745, 32076);
                c.step()?;
                c.invoke(p_232a_022e, true, 9002, 558, 32081)?;
                label = 32081;
                continue;
            }
            32081 => {
                c.pc = (2745, 32081);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 32082);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7d53(c: &mut Cpu) -> Result<(), String> {
    let mut label = 32083u16;
    loop {
        match label {
            32083 => {
                c.pc = (2745, 32083);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 32084);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 32086);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53432u16), 8, v)?;
                c.pc = (2745, 32091);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32096);
                c.step()?;
                label = if !c.z { 32103 } else { 32098 };
                continue;
            }
            32098 => {
                c.pc = (2745, 32098);
                c.step()?;
                let v = 3u16;
                c.push(v)?;
                c.pc = (2745, 32100);
                c.step()?;
                c.invoke(p_0ab9_1c7e, false, 2745, 7294, 32103)?;
                label = 32103;
                continue;
            }
            32103 => {
                c.pc = (2745, 32103);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53075u16), 8, v)?;
                c.pc = (2745, 32108);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53364u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32113);
                c.step()?;
                label = if !c.z { 32127 } else { 32115 };
                continue;
            }
            32115 => {
                c.pc = (2745, 32115);
                c.step()?;
                c.invoke(p_0ab9_7a02, false, 2745, 31234, 32118)?;
                label = 32118;
                continue;
            }
            32118 => {
                c.pc = (2745, 32118);
                c.step()?;
                let v = 4u16;
                c.push(v)?;
                c.pc = (2745, 32120);
                c.step()?;
                c.invoke(p_0ab9_1c1e, false, 2745, 7198, 32123)?;
                label = 32123;
                continue;
            }
            32123 => {
                c.pc = (2745, 32123);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32125);
                c.step()?;
                label = if c.z { 32115 } else { 32127 };
                continue;
            }
            32127 => {
                c.pc = (2745, 32127);
                c.step()?;
                c.invoke(p_232a_022e, true, 9002, 558, 32132)?;
                label = 32132;
                continue;
            }
            32132 => {
                c.pc = (2745, 32132);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 32133);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7dcb(c: &mut Cpu) -> Result<(), String> {
    let mut label = 32203u16;
    loop {
        match label {
            32203 => {
                c.pc = (2745, 32203);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 32206);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53446u16), 16, v)?;
                c.pc = (2745, 32209);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 32210);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7dd3(c: &mut Cpu) -> Result<(), String> {
    let mut label = 32211u16;
    loop {
        match label {
            32211 => {
                c.pc = (2745, 32211);
                c.step()?;
                c.enter(10)?;
                c.pc = (2745, 32215);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 32220);
                c.step()?;
                label = 32225;
                continue;
            }
            32222 => {
                c.pc = (2745, 32222);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 32225;
                continue;
            }
            32225 => {
                c.pc = (2745, 32225);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 32229);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32233);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 32234);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 32235);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 32240);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32244);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32246);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32250);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32252);
                c.step()?;
                label = if c.cf { 32263 } else { 32254 };
                continue;
            }
            32254 => {
                c.pc = (2745, 32254);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 30u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32256);
                c.step()?;
                label = if !c.cf && !c.z { 32263 } else { 32258 };
                continue;
            }
            32258 => {
                c.pc = (2745, 32258);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8)?;
                let y = 2u16;
                let v = c.add(x, y, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8, v)?;
                label = 32263;
                continue;
            }
            32263 => {
                c.pc = (2745, 32263);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 32267);
                c.step()?;
                label = if !c.z { 32222 } else { 32269 };
                continue;
            }
            32269 => {
                c.pc = (2745, 32269);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53434u16), 8, v)?;
                c.pc = (2745, 32274);
                c.step()?;
                let o = 0u16.wrapping_add(56664u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32278);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32279);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32280);
                c.step()?;
                let o = 0u16.wrapping_add(56672u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32284);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32285);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32286);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 32289);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 32294)?;
                label = 32294;
                continue;
            }
            32294 => {
                c.pc = (2745, 32294);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 32296);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 32298);
                c.step()?;
                c.invoke(p_1998_045c, true, 6552, 1116, 32303)?;
                label = 32303;
                continue;
            }
            32303 => {
                c.pc = (2745, 32303);
                c.step()?;
                let o = 0u16.wrapping_add(56672u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32307);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32308);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32309);
                c.step()?;
                let o = 0u16.wrapping_add(56664u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32313);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32314);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32315);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 32318);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 32323)?;
                label = 32323;
                continue;
            }
            32323 => {
                c.pc = (2745, 32323);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53707u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32328);
                c.step()?;
                label = if c.z { 32335 } else { 32330 };
                continue;
            }
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
pub(super) fn p_0ab9_7e73(c: &mut Cpu) -> Result<(), String> {
    let mut label = 32371u16;
    loop {
        match label {
            32371 => {
                c.pc = (2745, 32371);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 32372);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 32374);
                c.step()?;
                let v = 31u16;
                c.write(c.r[11], 0u16.wrapping_add(53422u16), 16, v)?;
                c.pc = (2745, 32380);
                c.step()?;
                let v = 89u16;
                c.write(c.r[11], 0u16.wrapping_add(53426u16), 16, v)?;
                c.pc = (2745, 32386);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53433u16), 8, v)?;
                c.pc = (2745, 32391);
                c.step()?;
                let v = 15u16;
                c.write(c.r[11], 0u16.wrapping_add(53420u16), 8, v)?;
                c.pc = (2745, 32396);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32398);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53424u16), 16, v)?;
                c.pc = (2745, 32401);
                c.step()?;
                let v = 62u16;
                c.push(v)?;
                c.pc = (2745, 32403);
                c.step()?;
                c.invoke(p_0ab9_1e14, false, 2745, 7700, 32406)?;
                label = 32406;
                continue;
            }
            32406 => {
                c.pc = (2745, 32406);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53434u16), 8, v)?;
                c.pc = (2745, 32411);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53707u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32416);
                c.step()?;
                label = if c.z { 32423 } else { 32418 };
                continue;
            }
            32418 => {
                c.pc = (2745, 32418);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 32420);
                c.step()?;
                c.invoke(p_0ab9_d5d6, false, 2745, 54742, 32423)?;
                label = 32423;
                continue;
            }
            32423 => {
                c.pc = (2745, 32423);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53421u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32426);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32427);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 32429);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32433);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 32435);
                c.step()?;
                let x = c.r[0];
                let y = 10u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32438);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53407u16), 8, v)?;
                c.pc = (2745, 32441);
                c.step()?;
                c.invoke(p_0ab9_7c3f, false, 2745, 31807, 32444)?;
                label = 32444;
                continue;
            }
            32444 => {
                c.pc = (2745, 32444);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 32445);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7ebe(c: &mut Cpu) -> Result<(), String> {
    let mut label = 32446u16;
    loop {
        match label {
            32446 => {
                c.pc = (2745, 32446);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 32447);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (2745, 32449);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53434u16), 8, v)?;
                c.pc = (2745, 32454);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53707u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32459);
                c.step()?;
                label = if c.z { 32466 } else { 32461 };
                continue;
            }
            32461 => {
                c.pc = (2745, 32461);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 32463);
                c.step()?;
                c.invoke(p_0ab9_d5d6, false, 2745, 54742, 32466)?;
                label = 32466;
                continue;
            }
            32466 => {
                c.pc = (2745, 32466);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53421u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 32471);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32473);
                c.step()?;
                label = if !c.z { 32476 } else { 32475 };
                continue;
            }
            32475 => {
                c.pc = (2745, 32475);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 32476;
                continue;
            }
            32476 => {
                c.pc = (2745, 32476);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32477);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 32479);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32483);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 32485);
                c.step()?;
                let x = c.r[0];
                let y = 10u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32488);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53407u16), 8, v)?;
                c.pc = (2745, 32491);
                c.step()?;
                c.invoke(p_0ab9_7c3f, false, 2745, 31807, 32494)?;
                label = 32494;
                continue;
            }
            32494 => {
                c.pc = (2745, 32494);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 32495);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_7ef0(c: &mut Cpu) -> Result<(), String> {
    let mut label = 32496u16;
    loop {
        match label {
            32496 => {
                c.pc = (2745, 32496);
                c.step()?;
                c.enter(522)?;
                c.pc = (2745, 32500);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 32503);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32507);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32508);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 32510);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32512);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 32517);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32521);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32523);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32527);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 32530);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 32533);
                c.step()?;
                let o = 0u16.wrapping_add(53386u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32537);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32538);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32539);
                c.step()?;
                let o = 0u16.wrapping_add(53398u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32543);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32544);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32545);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 32548);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 32553)?;
                label = 32553;
                continue;
            }
            32553 => {
                c.pc = (2745, 32553);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(39466u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32556);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8, v)?;
                c.pc = (2745, 32559);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(39466u16), 8, v)?;
                c.pc = (2745, 32564);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8, v)?;
                c.pc = (2745, 32568);
                c.step()?;
                let v = 11u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 32573);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 32578);
                c.step()?;
                label = 32583;
                continue;
            }
            32580 => {
                c.pc = (2745, 32580);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 32583;
                continue;
            }
            32583 => {
                c.pc = (2745, 32583);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 32586);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32588);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32591);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32593);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 32597);
                c.step()?;
                c.invoke(p_1b55_4f89, true, 6997, 20361, 32602)?;
                label = 32602;
                continue;
            }
            32602 => {
                c.pc = (2745, 32602);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32604);
                c.step()?;
                label = if c.z { 32609 } else { 32606 };
                continue;
            }
            32606 => {
                c.pc = (2745, 32606);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.dec(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 32609;
                continue;
            }
            32609 => {
                c.pc = (2745, 32609);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 11u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 32613);
                c.step()?;
                label = if !c.z { 32580 } else { 32615 };
                continue;
            }
            32615 => {
                c.pc = (2745, 32615);
                c.step()?;
                let v = 7278u16;
                c.r[7] = v;
                c.pc = (2745, 32618);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 32619);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32620);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 32623);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 32624);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32625);
                c.step()?;
                c.invoke(p_232a_052f, true, 9002, 1327, 32630)?;
                label = 32630;
                continue;
            }
            32630 => {
                c.pc = (2745, 32630);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 32633);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32637);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32638);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 32640);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32642);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 32646);
                c.step()?;
                c.invoke(p_1b55_0ea2, true, 6997, 3746, 32651)?;
                label = 32651;
                continue;
            }
            32651 => {
                c.pc = (2745, 32651);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 32652);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 32655);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32659);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32660);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 32662);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32664);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.r[0] = v;
                c.pc = (2745, 32668);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32671);
                c.step()?;
                let o = 0u16.wrapping_add(55218u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32675);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32677);
                c.step()?;
                let x = c.r[7];
                let y = 65520u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32681);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32682);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32683);
                c.step()?;
                let v = 25u16;
                c.push(v)?;
                c.pc = (2745, 32685);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 32687);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 32690);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 32691);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32692);
                c.step()?;
                c.invoke(p_232a_041b, true, 9002, 1051, 32697)?;
                label = 32697;
                continue;
            }
            32697 => {
                c.pc = (2745, 32697);
                c.step()?;
                let v = 7004u16;
                c.r[7] = v;
                c.pc = (2745, 32700);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 32701);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32702);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 32705);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 32706);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32707);
                c.step()?;
                c.invoke(p_232a_052f, true, 9002, 1327, 32712)?;
                label = 32712;
                continue;
            }
            32712 => {
                c.pc = (2745, 32712);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32715);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 32717);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32719);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32722);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32724);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 32728);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 32729);
                c.step()?;
                c.invoke(p_0ab9_95b3, false, 2745, 38323, 32732)?;
                label = 32732;
                continue;
            }
            32732 => {
                c.pc = (2745, 32732);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 32737);
                c.step()?;
                label = 32742;
                continue;
            }
            32739 => {
                c.pc = (2745, 32739);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 32742;
                continue;
            }
            32742 => {
                c.pc = (2745, 32742);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(266u16);
                c.pc = (2745, 32746);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 32747);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32748);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 32751);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32755);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32756);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 32758);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32762);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 32764);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32767);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 32768);
                c.step()?;
                c.invoke(p_26c9_0c6f, true, 9929, 3183, 32773)?;
                label = 32773;
                continue;
            }
            32773 => {
                c.pc = (2745, 32773);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 32776);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32780);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32781);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 32784);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32786);
                c.step()?;
                let x = c.r[7];
                let y = 65510u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32790);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 32791);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32792);
                c.step()?;
                c.invoke(p_26c9_0bfe, true, 9929, 3070, 32797)?;
                label = 32797;
                continue;
            }
            32797 => {
                c.pc = (2745, 32797);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32799);
                c.step()?;
                label = if !c.z { 32827 } else { 32801 };
                continue;
            }
            32801 => {
                c.pc = (2745, 32801);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 32804);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32806);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32809);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32811);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 32815);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 32818);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 32819);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 32820);
                c.step()?;
                let v = 60u16;
                c.push(v)?;
                c.pc = (2745, 32822);
                c.step()?;
                c.invoke(p_1b55_1ff1, true, 6997, 8177, 32827)?;
                label = 32827;
                continue;
            }
            32827 => {
                c.pc = (2745, 32827);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 11u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 32831);
                c.step()?;
                label = if !c.z { 32739 } else { 32833 };
                continue;
            }
            32833 => {
                c.pc = (2745, 32833);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32836);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 32838);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 32839);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 32840);
                c.step()?;
                let v = 59u16;
                c.push(v)?;
                c.pc = (2745, 32842);
                c.step()?;
                let v = 14u16;
                c.push(v)?;
                c.pc = (2745, 32844);
                c.step()?;
                c.invoke(p_232a_0203, true, 9002, 515, 32849)?;
                label = 32849;
                continue;
            }
            32849 => {
                c.pc = (2745, 32849);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 32852);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32856);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32857);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 32860);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32862);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32866);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 32868);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(12u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 32871);
                c.step()?;
                let v = 1u16;
                c.r[0] = v;
                c.pc = (2745, 32874);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(12u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 32877);
                c.step()?;
                label = if !c.cf && !c.z { 32946 } else { 32879 };
                continue;
            }
            32879 => {
                c.pc = (2745, 32879);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 32882);
                c.step()?;
                label = 32887;
                continue;
            }
            32884 => {
                c.pc = (2745, 32884);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 32887;
                continue;
            }
            32887 => {
                c.pc = (2745, 32887);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 32890);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 32893);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32897);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32898);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 32901);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32903);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32905);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32909);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 32911);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 32914);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 32918);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 32919);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 32920);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 32924);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 32927);
                c.step()?;
                let x = c.r[0];
                let y = 13u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 32930);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 32931);
                c.step()?;
                let v = 60u16;
                c.push(v)?;
                c.pc = (2745, 32933);
                c.step()?;
                c.invoke(p_1b55_1ff1, true, 6997, 8177, 32938)?;
                label = 32938;
                continue;
            }
            32938 => {
                c.pc = (2745, 32938);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 32941);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(12u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 32944);
                c.step()?;
                label = if !c.z { 32884 } else { 32946 };
                continue;
            }
            32946 => {
                c.pc = (2745, 32946);
                c.step()?;
                let o = 0u16.wrapping_add(56668u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32950);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32951);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32952);
                c.step()?;
                let o = 0u16.wrapping_add(56664u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 32956);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 32957);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32958);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 32961);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 32966)?;
                label = 32966;
                continue;
            }
            32966 => {
                c.pc = (2745, 32966);
                c.step()?;
                c.invoke(p_232a_099b, true, 9002, 2459, 32971)?;
                label = 32971;
                continue;
            }
            32971 => {
                c.pc = (2745, 32971);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16);
                c.pc = (2745, 32974);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 32975);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32976);
                c.step()?;
                let v = 11u16;
                c.push(v)?;
                c.pc = (2745, 32978);
                c.step()?;
                c.invoke(p_232a_0ec4, true, 9002, 3780, 32983)?;
                label = 32983;
                continue;
            }
            32983 => {
                c.pc = (2745, 32983);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(266u16);
                c.pc = (2745, 32987);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 32988);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 32989);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 32992);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 32996);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 32997);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 32999);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33003);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33005);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 33007);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33010);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33012);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 33014);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 33015);
                c.step()?;
                c.invoke(p_26c9_0c6f, true, 9929, 3183, 33020)?;
                label = 33020;
                continue;
            }
            33020 => {
                c.pc = (2745, 33020);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33023);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33027);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33028);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33031);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33033);
                c.step()?;
                let x = c.r[7];
                let y = 65510u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33037);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33038);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33039);
                c.step()?;
                c.invoke(p_26c9_0bfe, true, 9929, 3070, 33044)?;
                label = 33044;
                continue;
            }
            33044 => {
                c.pc = (2745, 33044);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 33046);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 33073 } else { 33048 };
                continue;
            }
            33048 => {
                c.pc = (2745, 33048);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33051);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33053);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 33055);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 33058);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33060);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 33064);
                c.step()?;
                c.invoke(p_1b55_4f89, true, 6997, 20361, 33069)?;
                label = 33069;
                continue;
            }
            33069 => {
                c.pc = (2745, 33069);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33071);
                c.step()?;
                label = if c.z { 33108 } else { 33073 };
                continue;
            }
            33073 => {
                c.pc = (2745, 33073);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16);
                c.pc = (2745, 33076);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33077);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33078);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 33080);
                c.step()?;
                let v = 11u16;
                c.push(v)?;
                c.pc = (2745, 33082);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                let y = 6u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33087);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33089);
                c.step()?;
                label = if !c.z { 33092 } else { 33091 };
                continue;
            }
            33091 => {
                c.pc = (2745, 33091);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 33092;
                continue;
            }
            33092 => {
                c.pc = (2745, 33092);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33093);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 33095);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(23752u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33099);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33100);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 33101);
                c.step()?;
                c.invoke(p_24bc_048c, true, 9404, 1164, 33106)?;
                label = 33106;
                continue;
            }
            33106 => {
                c.pc = (2745, 33106);
                c.step()?;
                label = 32983;
                continue;
            }
            33108 => {
                c.pc = (2745, 33108);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                let y = 2u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33113);
                c.step()?;
                label = if !c.z { 33222 } else { 33115 };
                continue;
            }
            33115 => {
                c.pc = (2745, 33115);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(522u16);
                c.pc = (2745, 33119);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33120);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33121);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33124);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33128);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33129);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33132);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33134);
                c.step()?;
                let x = c.r[7];
                let y = 65510u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33138);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33139);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33140);
                c.step()?;
                c.invoke(p_26c9_0b53, true, 9929, 2899, 33145)?;
                label = 33145;
                continue;
            }
            33145 => {
                c.pc = (2745, 33145);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(266u16);
                c.pc = (2745, 33149);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33150);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33151);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33154);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33158);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33159);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 33161);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33165);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33167);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 33169);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(5u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33172);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33174);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 33176);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 33177);
                c.step()?;
                c.invoke(p_26c9_0c6f, true, 9929, 3183, 33182)?;
                label = 33182;
                continue;
            }
            33182 => {
                c.pc = (2745, 33182);
                c.step()?;
                c.invoke(p_26c9_0bd2, true, 9929, 3026, 33187)?;
                label = 33187;
                continue;
            }
            33187 => {
                c.pc = (2745, 33187);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33190);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33194);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33195);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33198);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33200);
                c.step()?;
                let x = c.r[7];
                let y = 65510u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33204);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33205);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33206);
                c.step()?;
                let v = 11u16;
                c.push(v)?;
                c.pc = (2745, 33208);
                c.step()?;
                c.invoke(p_26c9_0b6d, true, 9929, 2925, 33213)?;
                label = 33213;
                continue;
            }
            33213 => {
                c.pc = (2745, 33213);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 33215);
                c.step()?;
                c.invoke(p_232a_0783, true, 9002, 1923, 33220)?;
                label = 33220;
                continue;
            }
            33220 => {
                c.pc = (2745, 33220);
                c.step()?;
                label = 33288;
                continue;
            }
            33222 => {
                c.pc = (2745, 33222);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                let y = 13u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33227);
                c.step()?;
                label = if !c.z { 33244 } else { 33229 };
                continue;
            }
            33229 => {
                c.pc = (2745, 33229);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(39466u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33234);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33236);
                c.step()?;
                label = if !c.z { 33239 } else { 33238 };
                continue;
            }
            33238 => {
                c.pc = (2745, 33238);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 33239;
                continue;
            }
            33239 => {
                c.pc = (2745, 33239);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(39466u16), 8, v)?;
                c.pc = (2745, 33242);
                c.step()?;
                label = 33288;
                continue;
            }
            33244 => {
                c.pc = (2745, 33244);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(56968u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33249);
                c.step()?;
                label = if !c.z { 33288 } else { 33251 };
                continue;
            }
            33251 => {
                c.pc = (2745, 33251);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33254);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33258);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33259);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33262);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33264);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33269);
                c.step()?;
                label = if c.cf || c.z { 33288 } else { 33271 };
                continue;
            }
            33271 => {
                c.pc = (2745, 33271);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33274);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33278);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33279);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33282);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33284);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                let v = c.dec(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8, v)?;
                label = 33288;
                continue;
            }
            33288 => {
                c.pc = (2745, 33288);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33291);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33295);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33296);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33299);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33301);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33305);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33307);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 33310);
                c.step()?;
                label = if c.z { 33315 } else { 33312 };
                continue;
            }
            33312 => {
                c.pc = (2745, 33312);
                c.step()?;
                label = 32697;
                continue;
            }
            33315 => {
                c.pc = (2745, 33315);
                c.step()?;
                let o = 0u16.wrapping_add(53398u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 33319);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 33320);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33321);
                c.step()?;
                let o = 0u16.wrapping_add(53386u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 33325);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 33326);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33327);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 33330);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 33335)?;
                label = 33335;
                continue;
            }
            33335 => {
                c.pc = (2745, 33335);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33338);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(39466u16), 8, v)?;
                label = 33341;
                continue;
            }
            33341 => {
                c.pc = (2745, 33341);
                c.step()?;
                c.invoke(p_24bc_0c58, true, 9404, 3160, 33346)?;
                label = 33346;
                continue;
            }
            33346 => {
                c.pc = (2745, 33346);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33348);
                c.step()?;
                label = if c.z { 33357 } else { 33350 };
                continue;
            }
            33350 => {
                c.pc = (2745, 33350);
                c.step()?;
                c.invoke(p_232a_099b, true, 9002, 2459, 33355)?;
                label = 33355;
                continue;
            }
            33355 => {
                c.pc = (2745, 33355);
                c.step()?;
                label = 33341;
                continue;
            }
            33357 => {
                c.pc = (2745, 33357);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(56968u16), 8, v)?;
                c.pc = (2745, 33362);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 33363);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_8256(c: &mut Cpu) -> Result<(), String> {
    let mut label = 33366u16;
    loop {
        match label {
            33366 => {
                c.pc = (2745, 33366);
                c.step()?;
                c.enter(4)?;
                c.pc = (2745, 33370);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 33375);
                c.step()?;
                label = 33380;
                continue;
            }
            33377 => {
                c.pc = (2745, 33377);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 33380;
                continue;
            }
            33380 => {
                c.pc = (2745, 33380);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33383);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33387);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33388);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 33390);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33394);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33396);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 33399);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.r[1] = (c.r[1] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33401);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 33404);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33407);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33411);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33412);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33415);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33417);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33419);
                c.step()?;
                let v = (c.r[1] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8, v)?;
                c.pc = (2745, 33423);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 11u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 33427);
                c.step()?;
                label = if !c.z { 33377 } else { 33429 };
                continue;
            }
            33429 => {
                c.pc = (2745, 33429);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 33434);
                c.step()?;
                label = 33439;
                continue;
            }
            33436 => {
                c.pc = (2745, 33436);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 33439;
                continue;
            }
            33439 => {
                c.pc = (2745, 33439);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 33442);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 33443);
                c.step()?;
                let x = c.r[0];
                let y = 11u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 33446);
                c.step()?;
                label = if c.cf || c.z { 33451 } else { 33448 };
                continue;
            }
            33448 => {
                c.pc = (2745, 33448);
                c.step()?;
                label = 33607;
                continue;
            }
            33451 => {
                c.pc = (2745, 33451);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 33454);
                c.step()?;
                label = 33459;
                continue;
            }
            33456 => {
                c.pc = (2745, 33456);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 33459;
                continue;
            }
            33459 => {
                c.pc = (2745, 33459);
                c.step()?;
                let v = 21u16;
                c.push(v)?;
                c.pc = (2745, 33461);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 33464);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33467);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33471);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33472);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33475);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33477);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33479);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33483);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33485);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 33488);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33492);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 33493);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33494);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 33495);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 33498)?;
                label = 33498;
                continue;
            }
            33498 => {
                c.pc = (2745, 33498);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 33499);
                c.step()?;
                let v = 21u16;
                c.push(v)?;
                c.pc = (2745, 33501);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 33504);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33507);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33511);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33512);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33515);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33517);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33519);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33523);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33525);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 33528);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33532);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 33533);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33534);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 33535);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 33538)?;
                label = 33538;
                continue;
            }
            33538 => {
                c.pc = (2745, 33538);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 33539);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 33541);
                c.step()?;
                label = if c.cf || c.z { 33598 } else { 33543 };
                continue;
            }
            33543 => {
                c.pc = (2745, 33543);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 33546);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33549);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33553);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33554);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33557);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33559);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33561);
                c.step()?;
                let x = c.r[7];
                let y = 65510u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33565);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33566);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33567);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 33570);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33573);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33577);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33578);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33581);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33583);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33585);
                c.step()?;
                let x = c.r[7];
                let y = 65510u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33589);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33590);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33591);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 33593);
                c.step()?;
                c.invoke(p_24bc_0203, true, 9404, 515, 33598)?;
                label = 33598;
                continue;
            }
            33598 => {
                c.pc = (2745, 33598);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 11u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 33602);
                c.step()?;
                label = if c.z { 33607 } else { 33604 };
                continue;
            }
            33604 => {
                c.pc = (2745, 33604);
                c.step()?;
                label = 33456;
                continue;
            }
            33607 => {
                c.pc = (2745, 33607);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 10u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 33611);
                c.step()?;
                label = if c.z { 33616 } else { 33613 };
                continue;
            }
            33613 => {
                c.pc = (2745, 33613);
                c.step()?;
                label = 33436;
                continue;
            }
            33616 => {
                c.pc = (2745, 33616);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 33617);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_8354(c: &mut Cpu) -> Result<(), String> {
    let mut label = 33620u16;
    loop {
        match label {
            33620 => {
                c.pc = (2745, 33620);
                c.step()?;
                c.enter(2)?;
                c.pc = (2745, 33624);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33627);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 33631);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33635);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33636);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 33638);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12088u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33642);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33644);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 33645);
                c.step()?;
                c.invoke(p_0ab9_00d6, false, 2745, 214, 33648)?;
                label = 33648;
                continue;
            }
            33648 => {
                c.pc = (2745, 33648);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33651);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33655);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33657);
                c.step()?;
                let x = c.r[0];
                let v = c.neg(x, 16);
                c.r[0] = v;
                c.pc = (2745, 33659);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53420u16), 8, v)?;
                c.pc = (2745, 33662);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33666);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33667);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33669);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(30u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33673);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33675);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 33677);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33680);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33684);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33685);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33688);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33690);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33692);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33696);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33698);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 33699);
                c.step()?;
                let v = 10u16;
                c.push(v)?;
                c.pc = (2745, 33701);
                c.step()?;
                c.invoke(p_0ab9_57e3, false, 2745, 22499, 33704)?;
                label = 33704;
                continue;
            }
            33704 => {
                c.pc = (2745, 33704);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33707);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33711);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33712);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33714);
                c.step()?;
                let x = c.r[7];
                let y = 65506u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33718);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33719);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33720);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 33722);
                c.step()?;
                let v = 11u16;
                c.push(v)?;
                c.pc = (2745, 33724);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 33726);
                c.step()?;
                c.invoke(p_24bc_048c, true, 9404, 1164, 33731)?;
                label = 33731;
                continue;
            }
            33731 => {
                c.pc = (2745, 33731);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33734);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33738);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33739);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33741);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(30u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33745);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33747);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 33749);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 33752);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33756);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33757);
                c.step()?;
                let x = c.r[0];
                let y = 12u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 33760);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33762);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33764);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(26u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33768);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33770);
                c.step()?;
                let x = c.r[0];
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 33773);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33777);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 33778);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33779);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 33780);
                c.step()?;
                c.invoke(p_0ab9_1ca1, false, 2745, 7329, 33783)?;
                label = 33783;
                continue;
            }
            33783 => {
                c.pc = (2745, 33783);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33785);
                c.step()?;
                label = if !c.z { 33704 } else { 33787 };
                continue;
            }
            33787 => {
                c.pc = (2745, 33787);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53431u16), 8, v)?;
                c.pc = (2745, 33792);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53432u16), 8, v)?;
                label = 33797;
                continue;
            }
            33797 => {
                c.pc = (2745, 33797);
                c.step()?;
                c.invoke(p_0ab9_7a02, false, 2745, 31234, 33800)?;
                label = 33800;
                continue;
            }
            33800 => {
                c.pc = (2745, 33800);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53431u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33805);
                c.step()?;
                label = if !c.z { 33797 } else { 33807 };
                continue;
            }
            33807 => {
                c.pc = (2745, 33807);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 33809);
                c.step()?;
                c.invoke(p_0ab9_1c7e, false, 2745, 7294, 33812)?;
                label = 33812;
                continue;
            }
            33812 => {
                c.pc = (2745, 33812);
                c.step()?;
                c.invoke(p_0ab9_7a02, false, 2745, 31234, 33815)?;
                label = 33815;
                continue;
            }
            33815 => {
                c.pc = (2745, 33815);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53073u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 33820);
                c.step()?;
                label = if !c.z { 33812 } else { 33822 };
                continue;
            }
            33822 => {
                c.pc = (2745, 33822);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 33823);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_8434(c: &mut Cpu) -> Result<(), String> {
    let mut label = 33844u16;
    loop {
        match label {
            33844 => {
                c.pc = (2745, 33844);
                c.step()?;
                c.enter(38)?;
                c.pc = (2745, 33848);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 33852);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(38u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 33855);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(36u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 33858);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33862);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 33864);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(34u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 33867);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                let y = 30u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33872);
                c.step()?;
                label = if !c.cf { 33878 } else { 33874 };
                continue;
            }
            33874 => {
                c.pc = (2745, 33874);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8, v)?;
                label = 33878;
                continue;
            }
            33878 => {
                c.pc = (2745, 33878);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(38u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 33881);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                let y = 2u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33886);
                c.step()?;
                label = if !c.cf { 33893 } else { 33888 };
                continue;
            }
            33888 => {
                c.pc = (2745, 33888);
                c.step()?;
                let v = 2u16;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8, v)?;
                label = 33893;
                continue;
            }
            33893 => {
                c.pc = (2745, 33893);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 33895);
                c.step()?;
                c.invoke(p_0ab9_1c7e, false, 2745, 7294, 33898)?;
                label = 33898;
                continue;
            }
            33898 => {
                c.pc = (2745, 33898);
                c.step()?;
                let v = 61u16;
                c.push(v)?;
                c.pc = (2745, 33900);
                c.step()?;
                c.invoke(p_0ab9_1e14, false, 2745, 7700, 33903)?;
                label = 33903;
                continue;
            }
            33903 => {
                c.pc = (2745, 33903);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53707u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 33908);
                c.step()?;
                label = if c.z { 33920 } else { 33910 };
                continue;
            }
            33910 => {
                c.pc = (2745, 33910);
                c.step()?;
                let v = 33826u16;
                c.r[7] = v;
                c.pc = (2745, 33913);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 33914);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33915);
                c.step()?;
                c.invoke(p_24bc_0702, true, 9404, 1794, 33920)?;
                label = 33920;
                continue;
            }
            33920 => {
                c.pc = (2745, 33920);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(53364u16), 8, v)?;
                c.pc = (2745, 33925);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(26u16);
                c.pc = (2745, 33928);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 33929);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 33930);
                c.step()?;
                let v = 24u16;
                c.push(v)?;
                c.pc = (2745, 33932);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 33934);
                c.step()?;
                c.invoke(p_24bc_0222, true, 9404, 546, 33939)?;
                label = 33939;
                continue;
            }
            33939 => {
                c.pc = (2745, 33939);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 33941);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53444u16), 16, v)?;
                c.pc = (2745, 33944);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8, v)?;
                c.pc = (2745, 33948);
                c.step()?;
                label = 33953;
                continue;
            }
            33950 => {
                c.pc = (2745, 33950);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8, v)?;
                label = 33953;
                continue;
            }
            33953 => {
                c.pc = (2745, 33953);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33956);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33957);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 33959);
                c.step()?;
                let v = 1u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(30u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 33963);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33966);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33967);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 33969);
                c.step()?;
                let v = 5u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(28u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 33973);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33976);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 33977);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 33979);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 33981);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 33985);
                c.step()?;
                c.invoke(p_1b55_0e7a, true, 6997, 3706, 33990)?;
                label = 33990;
                continue;
            }
            33990 => {
                c.pc = (2745, 33990);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 33992);
                c.step()?;
                label = if c.z { 34000 } else { 33994 };
                continue;
            }
            33994 => {
                c.pc = (2745, 33994);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 33995);
                c.step()?;
                c.invoke(p_0ab9_7ef0, false, 2745, 32496, 33998)?;
                label = 33998;
                continue;
            }
            33998 => {
                c.pc = (2745, 33998);
                c.step()?;
                label = 34004;
                continue;
            }
            34000 => {
                c.pc = (2745, 34000);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 34001);
                c.step()?;
                c.invoke(p_0ab9_8256, false, 2745, 33366, 34004)?;
                label = 34004;
                continue;
            }
            34004 => {
                c.pc = (2745, 34004);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 34008);
                c.step()?;
                label = if !c.z { 33950 } else { 34010 };
                continue;
            }
            34010 => {
                c.pc = (2745, 34010);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 34015);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53421u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34018);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8, v)?;
                c.pc = (2745, 34021);
                c.step()?;
                c.invoke(p_0ab9_da41, false, 2745, 55873, 34024)?;
                label = 34024;
                continue;
            }
            34024 => {
                c.pc = (2745, 34024);
                c.step()?;
                let o = 0u16.wrapping_add(53394u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 34028);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 34029);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 34030);
                c.step()?;
                let o = 0u16.wrapping_add(53386u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 34034);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 34035);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 34036);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 34039);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 34044)?;
                label = 34044;
                continue;
            }
            34044 => {
                c.pc = (2745, 34044);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34047);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 34049);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 34051);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(27u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34054);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 34056);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 34058);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53445u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34061);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 34063);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 34065);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 34067);
                c.step()?;
                label = if c.sf != c.of { 34183 } else { 34069 };
                continue;
            }
            34069 => {
                c.pc = (2745, 34069);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53445u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34072);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 34074);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (2745, 34076);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34079);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 34081);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 34083);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34086);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 34088);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 34090);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 34092);
                c.step()?;
                label = if c.sf != c.of { 34183 } else { 34094 };
                continue;
            }
            34094 => {
                c.pc = (2745, 34094);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 34095);
                c.step()?;
                c.invoke(p_0ab9_8354, false, 2745, 33620, 34098)?;
                label = 34098;
                continue;
            }
            34098 => {
                c.pc = (2745, 34098);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34101);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 34102);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 34104);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(28u16),
                    8,
                )?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 34108);
                c.step()?;
                label = if c.cf || c.z { 34119 } else { 34110 };
                continue;
            }
            34110 => {
                c.pc = (2745, 34110);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34113);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 34114);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 34116);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(28u16),
                    8,
                )?;
                let v = c.dec(x, 8);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(28u16),
                    8,
                    v,
                )?;
                label = 34119;
                continue;
            }
            34119 => {
                c.pc = (2745, 34119);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34122);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[11], 0u16.wrapping_add(53421u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 34126);
                c.step()?;
                label = if !c.z { 34156 } else { 34128 };
                continue;
            }
            34128 => {
                c.pc = (2745, 34128);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34131);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 34132);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 34134);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(28u16),
                    8,
                )?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 34138);
                c.step()?;
                label = if !c.z { 34156 } else { 34140 };
                continue;
            }
            34140 => {
                c.pc = (2745, 34140);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 34144);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34146);
                c.step()?;
                label = if !c.z { 34149 } else { 34148 };
                continue;
            }
            34148 => {
                c.pc = (2745, 34148);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 34149;
                continue;
            }
            34149 => {
                c.pc = (2745, 34149);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 34150);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 34152);
                c.step()?;
                let v = 1u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(28u16),
                    8,
                    v,
                )?;
                label = 34156;
                continue;
            }
            34156 => {
                c.pc = (2745, 34156);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34159);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[11], 0u16.wrapping_add(53421u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 34163);
                c.step()?;
                label = if c.z { 34168 } else { 34165 };
                continue;
            }
            34165 => {
                c.pc = (2745, 34165);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 34168;
                continue;
            }
            34168 => {
                c.pc = (2745, 34168);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 34172);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34174);
                c.step()?;
                label = if !c.z { 34177 } else { 34176 };
                continue;
            }
            34176 => {
                c.pc = (2745, 34176);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 34177;
                continue;
            }
            34177 => {
                c.pc = (2745, 34177);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(31u16), 8, v)?;
                c.pc = (2745, 34180);
                c.step()?;
                label = 34044;
                continue;
            }
            34183 => {
                c.pc = (2745, 34183);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(34u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34186);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(38u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 34189);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8, v)?;
                c.pc = (2745, 34193);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53365u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 34196);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53364u16), 8, v)?;
                c.pc = (2745, 34199);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 34200);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_b42e(c: &mut Cpu) -> Result<(), String> {
    let mut label = 46126u16;
    loop {
        match label {
            46126 => {
                c.pc = (2745, 46126);
                c.step()?;
                c.enter(10)?;
                c.pc = (2745, 46130);
                c.step()?;
                let v = 60u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 46135);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 46139);
                c.step()?;
                label = 46144;
                continue;
            }
            46141 => {
                c.pc = (2745, 46141);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 46144;
                continue;
            }
            46144 => {
                c.pc = (2745, 46144);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 46149);
                c.step()?;
                label = 46154;
                continue;
            }
            46151 => {
                c.pc = (2745, 46151);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 46154;
                continue;
            }
            46154 => {
                c.pc = (2745, 46154);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46157);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46159);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 46161);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46164);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46165);
                c.step()?;
                let x = c.r[0];
                let y = 40u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 46168);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46170);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11962u16),
                    16,
                )?;
                let y = 5000u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46176);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 46200 } else { 46178 };
                continue;
            }
            46178 => {
                c.pc = (2745, 46178);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46181);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46183);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 46185);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46188);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46189);
                c.step()?;
                let x = c.r[0];
                let y = 40u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 46192);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46194);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11962u16),
                    16,
                )?;
                let y = 10000u16;
                let v = c.sub(x, y, 16);
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11962u16),
                    16,
                    v,
                )?;
                label = 46200;
                continue;
            }
            46200 => {
                c.pc = (2745, 46200);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46203);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46205);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 46207);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46210);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46211);
                c.step()?;
                let x = c.r[0];
                let y = 40u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 46214);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46216);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11962u16),
                    16,
                )?;
                c.r[0] = v;
                c.pc = (2745, 46220);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46223);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 46283 } else { 46225 };
                continue;
            }
            46225 => {
                c.pc = (2745, 46225);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46228);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46230);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 46232);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46235);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46236);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 46238);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46240);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 46245);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46249);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46251);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46253);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(64u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46257);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53570u16), 16, v)?;
                c.pc = (2745, 46260);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46263);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46265);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 46267);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46270);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46271);
                c.step()?;
                let x = c.r[0];
                let y = 40u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 46274);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46276);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11962u16),
                    16,
                )?;
                c.r[0] = v;
                c.pc = (2745, 46280);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                label = 46283;
                continue;
            }
            46283 => {
                c.pc = (2745, 46283);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 20u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46287);
                c.step()?;
                label = if c.z { 46292 } else { 46289 };
                continue;
            }
            46289 => {
                c.pc = (2745, 46289);
                c.step()?;
                label = 46151;
                continue;
            }
            46292 => {
                c.pc = (2745, 46292);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 46296);
                c.step()?;
                label = if c.z { 46301 } else { 46298 };
                continue;
            }
            46298 => {
                c.pc = (2745, 46298);
                c.step()?;
                label = 46141;
                continue;
            }
            46301 => {
                c.pc = (2745, 46301);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53570u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46306);
                c.step()?;
                label = if c.z { 46378 } else { 46308 };
                continue;
            }
            46308 => {
                c.pc = (2745, 46308);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53570u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46311);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46314);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46318);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46320);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 46323);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 46326);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46329);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 46331);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 46332);
                c.step()?;
                c.invoke(p_1b55_0e7a, true, 6997, 3706, 46337)?;
                label = 46337;
                continue;
            }
            46337 => {
                c.pc = (2745, 46337);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46339);
                c.step()?;
                label = if c.z { 46378 } else { 46341 };
                continue;
            }
            46341 => {
                c.pc = (2745, 46341);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46344);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(1u16), 8)?;
                let y = 17u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 46349);
                c.step()?;
                label = if c.cf || c.z { 46378 } else { 46351 };
                continue;
            }
            46351 => {
                c.pc = (2745, 46351);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53570u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 46355);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46358);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[11], 0u16.wrapping_add(56506u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 46362);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46364);
                c.step()?;
                label = if c.z { 46367 } else { 46366 };
                continue;
            }
            46366 => {
                c.pc = (2745, 46366);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 46367;
                continue;
            }
            46367 => {
                c.pc = (2745, 46367);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 46369);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 46370);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 46372);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46374);
                c.step()?;
                let v = c.r[2];
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11966u16),
                    16,
                    v,
                )?;
                label = 46378;
                continue;
            }
            46378 => {
                c.pc = (2745, 46378);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 46379);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_b52c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 46380u16;
    loop {
        match label {
            46380 => {
                c.pc = (2745, 46380);
                c.step()?;
                c.enter(4)?;
                c.pc = (2745, 46384);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 46388);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 46393);
                c.step()?;
                label = 46398;
                continue;
            }
            46395 => {
                c.pc = (2745, 46395);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 46398;
                continue;
            }
            46398 => {
                c.pc = (2745, 46398);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46401);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (2745, 46403);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46405);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46407);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 46409);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 46412);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46416);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46417);
                c.step()?;
                let x = c.r[0];
                let y = 33u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 46420);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46422);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12061u16),
                    16,
                )?;
                c.r[0] = v;
                c.pc = (2745, 46426);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46429);
                c.step()?;
                label = if !c.z { 46435 } else { 46431 };
                continue;
            }
            46431 => {
                c.pc = (2745, 46431);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 46435;
                continue;
            }
            46435 => {
                c.pc = (2745, 46435);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 11u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46439);
                c.step()?;
                label = if !c.z { 46395 } else { 46441 };
                continue;
            }
            46441 => {
                c.pc = (2745, 46441);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46444);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 46445);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_b570(c: &mut Cpu) -> Result<(), String> {
    let mut label = 46448u16;
    loop {
        match label {
            46448 => {
                c.pc = (2745, 46448);
                c.step()?;
                c.enter(14)?;
                c.pc = (2745, 46452);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 46456);
                c.step()?;
                label = 46461;
                continue;
            }
            46458 => {
                c.pc = (2745, 46458);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 46461;
                continue;
            }
            46461 => {
                c.pc = (2745, 46461);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 46466);
                c.step()?;
                label = 46471;
                continue;
            }
            46468 => {
                c.pc = (2745, 46468);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 46471;
                continue;
            }
            46471 => {
                c.pc = (2745, 46471);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46474);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46475);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 46477);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46479);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 46484);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46488);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46490);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46494);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 46497);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 46500);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46503);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46505);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46507);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46512);
                c.step()?;
                label = if c.z { 46555 } else { 46514 };
                continue;
            }
            46514 => {
                c.pc = (2745, 46514);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[2] = v;
                c.pc = (2745, 46517);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46520);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46521);
                c.step()?;
                let x = c.r[0];
                let y = 20u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 46524);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46526);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(11881u16), 8)?;
                c.r[2] = (c.r[2] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46530);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46533);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46535);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46538);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46540);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 46545);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46549);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46551);
                c.step()?;
                let v = (c.r[2] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8, v)?;
                label = 46555;
                continue;
            }
            46555 => {
                c.pc = (2745, 46555);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 20u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46559);
                c.step()?;
                label = if !c.z { 46468 } else { 46561 };
                continue;
            }
            46561 => {
                c.pc = (2745, 46561);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 46565);
                c.step()?;
                label = if !c.z { 46458 } else { 46567 };
                continue;
            }
            46567 => {
                c.pc = (2745, 46567);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 46571);
                c.step()?;
                label = 46576;
                continue;
            }
            46573 => {
                c.pc = (2745, 46573);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 46576;
                continue;
            }
            46576 => {
                c.pc = (2745, 46576);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 46581);
                c.step()?;
                label = 46586;
                continue;
            }
            46583 => {
                c.pc = (2745, 46583);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                label = 46586;
                continue;
            }
            46586 => {
                c.pc = (2745, 46586);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46589);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46590);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 46592);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46594);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 46599);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46603);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46605);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46609);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 46612);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 46615);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46618);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46620);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46622);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                let y = 0u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46627);
                c.step()?;
                label = if c.z { 46753 } else { 46629 };
                continue;
            }
            46629 => {
                c.pc = (2745, 46629);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46632);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46634);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46637);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46639);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46643);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46646);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46650);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46652);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                let y = 13u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 46657);
                c.step()?;
                label = if !c.z { 46753 } else { 46659 };
                continue;
            }
            46659 => {
                c.pc = (2745, 46659);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46662);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46664);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 46666);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46669);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46670);
                c.step()?;
                let x = c.r[0];
                let y = 40u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 46673);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46675);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(11962u16),
                    16,
                )?;
                let y = 5000u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46681);
                c.step()?;
                label = if c.sf == c.of { 46753 } else { 46683 };
                continue;
            }
            46683 => {
                c.pc = (2745, 46683);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46686);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46688);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46691);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46693);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 46697);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 46698);
                c.step()?;
                c.invoke(p_0ab9_b52c, false, 2745, 46380, 46701)?;
                label = 46701;
                continue;
            }
            46701 => {
                c.pc = (2745, 46701);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46703);
                c.step()?;
                label = if c.z { 46753 } else { 46705 };
                continue;
            }
            46705 => {
                c.pc = (2745, 46705);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 46708);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46710);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46713);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46715);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 46720);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46724);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 46726);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(14u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 46729);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(12u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 46732);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 46736);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 46737);
                c.step()?;
                let x = c.r[0];
                let y = 65533u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 46740);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 46741);
                c.step()?;
                c.invoke(p_1998_0dae, true, 6552, 3502, 46746)?;
                label = 46746;
                continue;
            }
            46746 => {
                c.pc = (2745, 46746);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(14u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 46749);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8, v)?;
                label = 46753;
                continue;
            }
            46753 => {
                c.pc = (2745, 46753);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let y = 11u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 46757);
                c.step()?;
                label = if c.z { 46762 } else { 46759 };
                continue;
            }
            46759 => {
                c.pc = (2745, 46759);
                c.step()?;
                label = 46583;
                continue;
            }
            46762 => {
                c.pc = (2745, 46762);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 46766);
                c.step()?;
                label = if c.z { 46771 } else { 46768 };
                continue;
            }
            46768 => {
                c.pc = (2745, 46768);
                c.step()?;
                label = 46573;
                continue;
            }
            46771 => {
                c.pc = (2745, 46771);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 46772);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_d904(c: &mut Cpu) -> Result<(), String> {
    let mut label = 55556u16;
    loop {
        match label {
            55556 => {
                c.pc = (2745, 55556);
                c.step()?;
                c.enter(6)?;
                c.pc = (2745, 55560);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 55563);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (2745, 55565);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 55567);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 55569);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 55571);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                let y = 174u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 55576);
                c.step()?;
                let o = 0u16.wrapping_add(55242u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 55580);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55582);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55584);
                c.step()?;
                let x = c.r[7];
                let y = 65359u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55588);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 55591);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 55594);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 55598);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                let y = 3u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 55602);
                c.step()?;
                label = if c.z { 55637 } else { 55604 };
                continue;
            }
            55604 => {
                c.pc = (2745, 55604);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                let y = 4u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 55608);
                c.step()?;
                label = if c.z { 55637 } else { 55610 };
                continue;
            }
            55610 => {
                c.pc = (2745, 55610);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                let y = 6u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 55614);
                c.step()?;
                label = if !c.z { 55622 } else { 55616 };
                continue;
            }
            55616 => {
                c.pc = (2745, 55616);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                let y = 57u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 55620);
                c.step()?;
                label = if c.z { 55637 } else { 55622 };
                continue;
            }
            55622 => {
                c.pc = (2745, 55622);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 55625);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                let y = 7u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 55629);
                c.step()?;
                label = if !c.z { 55653 } else { 55631 };
                continue;
            }
            55631 => {
                c.pc = (2745, 55631);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                let y = 48u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 55635);
                c.step()?;
                label = if !c.z { 55653 } else { 55637 };
                continue;
            }
            55637 => {
                c.pc = (2745, 55637);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 55640);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 55643);
                c.step()?;
                c.invoke(p_1b55_17c2, true, 6997, 6082, 55648)?;
                label = 55648;
                continue;
            }
            55648 => {
                c.pc = (2745, 55648);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 55651);
                c.step()?;
                label = 55676;
                continue;
            }
            55653 => {
                c.pc = (2745, 55653);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 55656);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 55659);
                c.step()?;
                c.invoke(p_1998_022b, true, 6552, 555, 55664)?;
                label = 55664;
                continue;
            }
            55664 => {
                c.pc = (2745, 55664);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 55665);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 55668);
                c.step()?;
                c.invoke(p_1b55_1a1c, true, 6997, 6684, 55673)?;
                label = 55673;
                continue;
            }
            55673 => {
                c.pc = (2745, 55673);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 55676;
                continue;
            }
            55676 => {
                c.pc = (2745, 55676);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 55679);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 55680);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_d983(c: &mut Cpu) -> Result<(), String> {
    let mut label = 55683u16;
    loop {
        match label {
            55683 => {
                c.pc = (2745, 55683);
                c.step()?;
                c.enter(8)?;
                c.pc = (2745, 55687);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 55690);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 55693);
                c.step()?;
                let v = 1u16;
                c.r[0] = v;
                c.pc = (2745, 55696);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 55699);
                c.step()?;
                label = if c.cf || c.z { 55704 } else { 55701 };
                continue;
            }
            55701 => {
                c.pc = (2745, 55701);
                c.step()?;
                label = 55869;
                continue;
            }
            55704 => {
                c.pc = (2745, 55704);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 55707);
                c.step()?;
                label = 55712;
                continue;
            }
            55709 => {
                c.pc = (2745, 55709);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 55712;
                continue;
            }
            55712 => {
                c.pc = (2745, 55712);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 55714);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 55717);
                c.step()?;
                label = 55722;
                continue;
            }
            55719 => {
                c.pc = (2745, 55719);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 55722;
                continue;
            }
            55722 => {
                c.pc = (2745, 55722);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 55725);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 55728);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 55730);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55732);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10628u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 55736);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 55741)?;
                label = 55741;
                continue;
            }
            55741 => {
                c.pc = (2745, 55741);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 55742);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 55745);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55747);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10628u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 55751);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 55756)?;
                label = 55756;
                continue;
            }
            55756 => {
                c.pc = (2745, 55756);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 55757);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 55759);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 55849 } else { 55761 };
                continue;
            }
            55761 => {
                c.pc = (2745, 55761);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 55764);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 55767);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 55769);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55771);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10632u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 55775);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 55780)?;
                label = 55780;
                continue;
            }
            55780 => {
                c.pc = (2745, 55780);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 55781);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 55784);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55786);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10624u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 55790);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 55795)?;
                label = 55795;
                continue;
            }
            55795 => {
                c.pc = (2745, 55795);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 55796);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 55798);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 55849 } else { 55800 };
                continue;
            }
            55800 => {
                c.pc = (2745, 55800);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 55803);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 55806);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 55808);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55810);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10636u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 55814);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 55819)?;
                label = 55819;
                continue;
            }
            55819 => {
                c.pc = (2745, 55819);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 55820);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 55823);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55825);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10624u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 55829);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 55834)?;
                label = 55834;
                continue;
            }
            55834 => {
                c.pc = (2745, 55834);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 55835);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 55837);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 55849 } else { 55839 };
                continue;
            }
            55839 => {
                c.pc = (2745, 55839);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 55842);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 55843);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 55845);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12155u16), 8)?;
                let v = c.inc(x, 8);
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12155u16),
                    8,
                    v,
                )?;
                label = 55849;
                continue;
            }
            55849 => {
                c.pc = (2745, 55849);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 55853);
                c.step()?;
                label = if c.z { 55858 } else { 55855 };
                continue;
            }
            55855 => {
                c.pc = (2745, 55855);
                c.step()?;
                label = 55719;
                continue;
            }
            55858 => {
                c.pc = (2745, 55858);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 55861);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 55864);
                c.step()?;
                label = if c.z { 55869 } else { 55866 };
                continue;
            }
            55866 => {
                c.pc = (2745, 55866);
                c.step()?;
                label = 55709;
                continue;
            }
            55869 => {
                c.pc = (2745, 55869);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 55870);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_da41(c: &mut Cpu) -> Result<(), String> {
    let mut label = 55873u16;
    loop {
        match label {
            55873 => {
                c.pc = (2745, 55873);
                c.step()?;
                c.enter(514)?;
                c.pc = (2745, 55877);
                c.step()?;
                let v = 25570u16;
                c.r[7] = v;
                c.pc = (2745, 55880);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 55881);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55882);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 55885);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 55886);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55887);
                c.step()?;
                c.invoke(p_232a_052f, true, 9002, 1327, 55892)?;
                label = 55892;
                continue;
            }
            55892 => {
                c.pc = (2745, 55892);
                c.step()?;
                let v = 26292u16;
                c.r[7] = v;
                c.pc = (2745, 55895);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 55896);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55897);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 55900);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 55901);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55902);
                c.step()?;
                c.invoke(p_232a_052f, true, 9002, 1327, 55907)?;
                label = 55907;
                continue;
            }
            55907 => {
                c.pc = (2745, 55907);
                c.step()?;
                let v = 6622u16;
                c.r[7] = v;
                c.pc = (2745, 55910);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 55911);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55912);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 55915);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 55916);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55917);
                c.step()?;
                c.invoke(p_232a_052f, true, 9002, 1327, 55922)?;
                label = 55922;
                continue;
            }
            55922 => {
                c.pc = (2745, 55922);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 55926);
                c.step()?;
                label = 55931;
                continue;
            }
            55928 => {
                c.pc = (2745, 55928);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 55931;
                continue;
            }
            55931 => {
                c.pc = (2745, 55931);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 55934);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 55935);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 55937);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12088u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 55941);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 55942);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(258u16);
                c.pc = (2745, 55946);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 55947);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55948);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 55951);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 55952);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 55954);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55956);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.r[0] = v;
                c.pc = (2745, 55960);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 55963);
                c.step()?;
                let o = 0u16.wrapping_add(55218u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 55967);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55969);
                c.step()?;
                let x = c.r[7];
                let y = 65520u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 55973);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 55974);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55975);
                c.step()?;
                c.invoke(p_24bc_005a, true, 9404, 90, 55980)?;
                label = 55980;
                continue;
            }
            55980 => {
                c.pc = (2745, 55980);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 55983);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 55984);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 55986);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3294u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 55990);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 55991);
                c.step()?;
                let v = 2u16;
                c.push(v)?;
                c.pc = (2745, 55993);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 55996);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 55997);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 55998);
                c.step()?;
                c.invoke(p_232a_041b, true, 9002, 1051, 56003)?;
                label = 56003;
                continue;
            }
            56003 => {
                c.pc = (2745, 56003);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56007);
                c.step()?;
                label = if !c.z { 55928 } else { 56009 };
                continue;
            }
            56009 => {
                c.pc = (2745, 56009);
                c.step()?;
                let v = 5u16;
                c.push(v)?;
                c.pc = (2745, 56011);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(514u16);
                c.pc = (2745, 56015);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 56016);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56017);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(258u16);
                c.pc = (2745, 56021);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 56022);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56023);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56027);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56030);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56032);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (2745, 56034);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 56036);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 56038);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 56040);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(55014u16), 16)?;
                let y = 174u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 56046);
                c.step()?;
                let o = 0u16.wrapping_add(55242u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56050);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56052);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56054);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(177u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56059);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56060);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56064);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56067);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56068);
                c.step()?;
                c.invoke(p_1b55_142b, true, 6997, 5163, 56073)?;
                label = 56073;
                continue;
            }
            56073 => {
                c.pc = (2745, 56073);
                c.step()?;
                c.invoke(p_24bc_005a, true, 9404, 90, 56078)?;
                label = 56078;
                continue;
            }
            56078 => {
                c.pc = (2745, 56078);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 56080);
                c.step()?;
                let v = 42u16;
                c.push(v)?;
                c.pc = (2745, 56082);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 56085);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 56086);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56087);
                c.step()?;
                c.invoke(p_232a_041b, true, 9002, 1051, 56092)?;
                label = 56092;
                continue;
            }
            56092 => {
                c.pc = (2745, 56092);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 56095);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 56096);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56097);
                c.step()?;
                c.invoke(p_1998_02df, true, 6552, 735, 56102)?;
                label = 56102;
                continue;
            }
            56102 => {
                c.pc = (2745, 56102);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 56104);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(258u16);
                c.pc = (2745, 56108);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 56109);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56110);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55018u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 56114);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55016u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 56118);
                c.step()?;
                c.invoke(p_232a_07c9, true, 9002, 1993, 56123)?;
                label = 56123;
                continue;
            }
            56123 => {
                c.pc = (2745, 56123);
                c.step()?;
                let v = 3u16;
                c.push(v)?;
                c.pc = (2745, 56125);
                c.step()?;
                let v = 27u16;
                c.push(v)?;
                c.pc = (2745, 56127);
                c.step()?;
                let v = 56668u16;
                c.r[7] = v;
                c.pc = (2745, 56130);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 56131);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56132);
                c.step()?;
                c.invoke(p_232a_030c, true, 9002, 780, 56137)?;
                label = 56137;
                continue;
            }
            56137 => {
                c.pc = (2745, 56137);
                c.step()?;
                let o = 0u16.wrapping_add(56668u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56141);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 56142);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56143);
                c.step()?;
                let o = 0u16.wrapping_add(53394u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56147);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 56148);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56149);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 56152);
                c.step()?;
                c.invoke(p_24bc_01c2, true, 9404, 450, 56157)?;
                label = 56157;
                continue;
            }
            56157 => {
                c.pc = (2745, 56157);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 56158);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_dbc6(c: &mut Cpu) -> Result<(), String> {
    let mut label = 56262u16;
    loop {
        match label {
            56262 => {
                c.pc = (2745, 56262);
                c.step()?;
                c.enter(10)?;
                c.pc = (2745, 56266);
                c.step()?;
                let v = 10u16;
                c.push(v)?;
                c.pc = (2745, 56268);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 56273)?;
                label = 56273;
                continue;
            }
            56273 => {
                c.pc = (2745, 56273);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (2745, 56274);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 56277);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56280);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3328u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56284);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56286);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (2745, 56289);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 56293);
                c.step()?;
                label = 56298;
                continue;
            }
            56295 => {
                c.pc = (2745, 56295);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 56298;
                continue;
            }
            56298 => {
                c.pc = (2745, 56298);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56301);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56302);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56304);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56308);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56310);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 56312);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56315);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56316);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56318);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56320);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(6u16),
                    16,
                )?;
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(6u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56323);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56327);
                c.step()?;
                label = if !c.z { 56295 } else { 56329 };
                continue;
            }
            56329 => {
                c.pc = (2745, 56329);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 56333);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56337);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 56338);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56339);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 56340);
                c.step()?;
                c.invoke(p_0ab9_1ca1, false, 2745, 7329, 56343)?;
                label = 56343;
                continue;
            }
            56343 => {
                c.pc = (2745, 56343);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56345);
                c.step()?;
                label = if c.z { 56350 } else { 56347 };
                continue;
            }
            56347 => {
                c.pc = (2745, 56347);
                c.step()?;
                label = 56574;
                continue;
            }
            56350 => {
                c.pc = (2745, 56350);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 56354);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56358);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 56359);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56360);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 56361);
                c.step()?;
                c.invoke(p_0ab9_1ca1, false, 2745, 7329, 56364)?;
                label = 56364;
                continue;
            }
            56364 => {
                c.pc = (2745, 56364);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56366);
                c.step()?;
                label = if c.z { 56371 } else { 56368 };
                continue;
            }
            56368 => {
                c.pc = (2745, 56368);
                c.step()?;
                label = 56574;
                continue;
            }
            56371 => {
                c.pc = (2745, 56371);
                c.step()?;
                c.invoke(p_24bc_0d3f, true, 9404, 3391, 56376)?;
                label = 56376;
                continue;
            }
            56376 => {
                c.pc = (2745, 56376);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (2745, 56379);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56382);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56383);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56385);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56387);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(6u16),
                    16,
                )?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 56391);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56395);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 56396);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 56397);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(10u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56400);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (2745, 56403);
                c.step()?;
                let v = 14u16;
                c.push(v)?;
                c.pc = (2745, 56405);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56409);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56411);
                c.step()?;
                label = if !c.z { 56414 } else { 56413 };
                continue;
            }
            56413 => {
                c.pc = (2745, 56413);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 56414;
                continue;
            }
            56414 => {
                c.pc = (2745, 56414);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56415);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56417);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56419);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(6u16),
                    16,
                )?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 56423);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56427);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 56428);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56429);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 56430);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 56433)?;
                label = 56433;
                continue;
            }
            56433 => {
                c.pc = (2745, 56433);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56434);
                c.step()?;
                let v = 6u16;
                c.push(v)?;
                c.pc = (2745, 56436);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56439);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 56440);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56441);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 56442);
                c.step()?;
                c.invoke(p_0ab9_1ae2, false, 2745, 6882, 56445)?;
                label = 56445;
                continue;
            }
            56445 => {
                c.pc = (2745, 56445);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 56446);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56448);
                c.step()?;
                label = if !c.cf { 56574 } else { 56450 };
                continue;
            }
            56450 => {
                c.pc = (2745, 56450);
                c.step()?;
                let v = 4u16;
                c.push(v)?;
                c.pc = (2745, 56452);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56455);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 56456);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56457);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 56458);
                c.step()?;
                c.invoke(p_0ab9_1a4c, false, 2745, 6732, 56461)?;
                label = 56461;
                continue;
            }
            56461 => {
                c.pc = (2745, 56461);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56463);
                c.step()?;
                label = if !c.z { 56476 } else { 56465 };
                continue;
            }
            56465 => {
                c.pc = (2745, 56465);
                c.step()?;
                let v = 20u16;
                c.push(v)?;
                c.pc = (2745, 56467);
                c.step()?;
                c.invoke(p_24bc_0d1c, true, 9404, 3356, 56472)?;
                label = 56472;
                continue;
            }
            56472 => {
                c.pc = (2745, 56472);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56474);
                c.step()?;
                label = if c.z { 56574 } else { 56476 };
                continue;
            }
            56476 => {
                c.pc = (2745, 56476);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56480);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56482);
                c.step()?;
                label = if !c.z { 56485 } else { 56484 };
                continue;
            }
            56484 => {
                c.pc = (2745, 56484);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 56485;
                continue;
            }
            56485 => {
                c.pc = (2745, 56485);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56486);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56488);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56490);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(6u16),
                    16,
                )?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 56494);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(13059u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (2745, 56498);
                c.step()?;
                c.invoke(p_1b55_4e0c, true, 6997, 19980, 56503)?;
                label = 56503;
                continue;
            }
            56503 => {
                c.pc = (2745, 56503);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56505);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56506);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 56511)?;
                label = 56511;
                continue;
            }
            56511 => {
                c.pc = (2745, 56511);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56512);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56515);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 56519);
                c.step()?;
                c.invoke(p_1b55_4e0c, true, 6997, 19980, 56524)?;
                label = 56524;
                continue;
            }
            56524 => {
                c.pc = (2745, 56524);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56526);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56527);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 56532)?;
                label = 56532;
                continue;
            }
            56532 => {
                c.pc = (2745, 56532);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (2745, 56533);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56535);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 56574 } else { 56537 };
                continue;
            }
            56537 => {
                c.pc = (2745, 56537);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56540);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56541);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56543);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56545);
                c.step()?;
                let v = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(6u16),
                    8,
                )?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56548);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53408u16), 8, v)?;
                c.pc = (2745, 56551);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56555);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56557);
                c.step()?;
                label = if !c.z { 56560 } else { 56559 };
                continue;
            }
            56559 => {
                c.pc = (2745, 56559);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 56560;
                continue;
            }
            56560 => {
                c.pc = (2745, 56560);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56561);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56563);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56565);
                c.step()?;
                let v = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(6u16),
                    8,
                )?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56568);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53407u16), 8, v)?;
                c.pc = (2745, 56571);
                c.step()?;
                c.invoke(p_0ab9_69b8, false, 2745, 27064, 56574)?;
                label = 56574;
                continue;
            }
            56574 => {
                c.pc = (2745, 56574);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 56575);
                c.step()?;
                c.ret(false, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_dd02(c: &mut Cpu) -> Result<(), String> {
    let mut label = 56578u16;
    loop {
        match label {
            56578 => {
                c.pc = (2745, 56578);
                c.step()?;
                c.enter(46)?;
                c.pc = (2745, 56582);
                c.step()?;
                let o = 0u16.wrapping_add(56664u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56586);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 56587);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56588);
                c.step()?;
                let o = 0u16.wrapping_add(56672u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56592);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 56593);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56594);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 56597);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 56602)?;
                label = 56602;
                continue;
            }
            56602 => {
                c.pc = (2745, 56602);
                c.step()?;
                let v = 1u16;
                c.write(c.r[11], 0u16.wrapping_add(53432u16), 8, v)?;
                c.pc = (2745, 56607);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(38u16), 8, v)?;
                c.pc = (2745, 56611);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 56613);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53424u16), 16, v)?;
                c.pc = (2745, 56616);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53444u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 56619);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 0) & 255;
                c.r[0] = (c.r[0] & 255) | ((y & 255) << 8);
                c.r[0] = (c.r[0] & 65280) | ((x & 255) << 0);
                c.pc = (2745, 56621);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (2745, 56624);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(26u16);
                c.pc = (2745, 56627);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (2745, 56628);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 56629);
                c.step()?;
                let v = 22u16;
                c.push(v)?;
                c.pc = (2745, 56631);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 56633);
                c.step()?;
                c.invoke(p_24bc_0222, true, 9404, 546, 56638)?;
                label = 56638;
                continue;
            }
            56638 => {
                c.pc = (2745, 56638);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 56641);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (2745, 56644);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 56646);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53382u16), 16, v)?;
                c.pc = (2745, 56649);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55014u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 56653);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 56655);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(3369u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56658);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56659);
                c.step()?;
                c.invoke(p_1b55_50eb, true, 6997, 20715, 56664)?;
                label = 56664;
                continue;
            }
            56664 => {
                c.pc = (2745, 56664);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(55012u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 56668);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 56670);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 56672);
                c.step()?;
                c.invoke(p_1b55_50eb, true, 6997, 20715, 56677)?;
                label = 56677;
                continue;
            }
            56677 => {
                c.pc = (2745, 56677);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53420u16), 8)?;
                let y = 15u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56682);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56684);
                c.step()?;
                label = if !c.z { 56687 } else { 56686 };
                continue;
            }
            56686 => {
                c.pc = (2745, 56686);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 56687;
                continue;
            }
            56687 => {
                c.pc = (2745, 56687);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56688);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56690);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3326u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56694);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56696);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56697);
                c.step()?;
                c.invoke(p_0ab9_d983, false, 2745, 55683, 56700)?;
                label = 56700;
                continue;
            }
            56700 => {
                c.pc = (2745, 56700);
                c.step()?;
                let v = 1u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(34u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56705);
                c.step()?;
                label = 56710;
                continue;
            }
            56707 => {
                c.pc = (2745, 56707);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(34u16), 16)?;
                let v = c.inc(x, 16);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(34u16),
                    16,
                    v,
                )?;
                label = 56710;
                continue;
            }
            56710 => {
                c.pc = (2745, 56710);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(34u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56713);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12155u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56717);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(34u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56720);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(3u16),
                    8,
                )?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56723);
                c.step()?;
                label = if !c.cf && !c.z { 56728 } else { 56725 };
                continue;
            }
            56725 => {
                c.pc = (2745, 56725);
                c.step()?;
                label = 57132;
                continue;
            }
            56728 => {
                c.pc = (2745, 56728);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(34u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56731);
                c.step()?;
                let v = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(3u16),
                    8,
                )?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56734);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(34u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56737);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12155u16), 8)?;
                let y = (c.r[0] >> 0) & 255;
                let v = c.sub(x, y, 8);
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(12155u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 56741);
                c.step()?;
                let v = 1u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(28u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56746);
                c.step()?;
                label = 56751;
                continue;
            }
            56748 => {
                c.pc = (2745, 56748);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let v = c.inc(x, 16);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(28u16),
                    16,
                    v,
                )?;
                label = 56751;
                continue;
            }
            56751 => {
                c.pc = (2745, 56751);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56754);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12155u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56758);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56760);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(40u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56763);
                c.step()?;
                let v = 1u16;
                c.r[0] = v;
                c.pc = (2745, 56766);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(40u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56769);
                c.step()?;
                label = if !c.cf && !c.z { 56826 } else { 56771 };
                continue;
            }
            56771 => {
                c.pc = (2745, 56771);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(30u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56774);
                c.step()?;
                label = 56779;
                continue;
            }
            56776 => {
                c.pc = (2745, 56776);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                let v = c.inc(x, 16);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(30u16),
                    16,
                    v,
                )?;
                label = 56779;
                continue;
            }
            56779 => {
                c.pc = (2745, 56779);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(53420u16), 8)?;
                let y = 15u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56784);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56786);
                c.step()?;
                label = if !c.z { 56789 } else { 56788 };
                continue;
            }
            56788 => {
                c.pc = (2745, 56788);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 56789;
                continue;
            }
            56789 => {
                c.pc = (2745, 56789);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56790);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56792);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3328u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56796);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56798);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 56799);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 56804)?;
                label = 56804;
                continue;
            }
            56804 => {
                c.pc = (2745, 56804);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.r[2] = (c.r[2] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56806);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 56809);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 11u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 56813);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56815);
                c.step()?;
                let v = (c.r[2] >> 0) & 255;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(38u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 56818);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 56821);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(40u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56824);
                c.step()?;
                label = if !c.z { 56776 } else { 56826 };
                continue;
            }
            56826 => {
                c.pc = (2745, 56826);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56830);
                c.step()?;
                label = if !c.z { 56748 } else { 56832 };
                continue;
            }
            56832 => {
                c.pc = (2745, 56832);
                c.step()?;
                let v = 1u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(28u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56837);
                c.step()?;
                label = 56842;
                continue;
            }
            56839 => {
                c.pc = (2745, 56839);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let v = c.inc(x, 16);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(28u16),
                    16,
                    v,
                )?;
                label = 56842;
                continue;
            }
            56842 => {
                c.pc = (2745, 56842);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56845);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(12155u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56849);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56851);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(40u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56854);
                c.step()?;
                let v = 1u16;
                c.r[0] = v;
                c.pc = (2745, 56857);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(40u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56860);
                c.step()?;
                label = if c.cf || c.z { 56865 } else { 56862 };
                continue;
            }
            56862 => {
                c.pc = (2745, 56862);
                c.step()?;
                label = 57123;
                continue;
            }
            56865 => {
                c.pc = (2745, 56865);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(30u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56868);
                c.step()?;
                label = 56873;
                continue;
            }
            56870 => {
                c.pc = (2745, 56870);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                let v = c.inc(x, 16);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(30u16),
                    16,
                    v,
                )?;
                label = 56873;
                continue;
            }
            56873 => {
                c.pc = (2745, 56873);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(38u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56877);
                c.step()?;
                label = if c.z { 56882 } else { 56879 };
                continue;
            }
            56879 => {
                c.pc = (2745, 56879);
                c.step()?;
                label = 57112;
                continue;
            }
            56882 => {
                c.pc = (2745, 56882);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 56885);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 11u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 56889);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56891);
                c.step()?;
                let v = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(38u16),
                    8,
                )?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56894);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56896);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[11], 0u16.wrapping_add(53426u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56900);
                c.step()?;
                label = if c.z { 56905 } else { 56902 };
                continue;
            }
            56902 => {
                c.pc = (2745, 56902);
                c.step()?;
                label = 57112;
                continue;
            }
            56905 => {
                c.pc = (2745, 56905);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56908);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(3u16),
                    8,
                )?;
                let y = 15u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 56912);
                c.step()?;
                label = if c.cf { 56917 } else { 56914 };
                continue;
            }
            56914 => {
                c.pc = (2745, 56914);
                c.step()?;
                label = 57112;
                continue;
            }
            56917 => {
                c.pc = (2745, 56917);
                c.step()?;
                let v = 30u16;
                c.push(v)?;
                c.pc = (2745, 56919);
                c.step()?;
                c.invoke(p_232a_076d, true, 9002, 1901, 56924)?;
                label = 56924;
                continue;
            }
            56924 => {
                c.pc = (2745, 56924);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56926);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3295u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56930);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 56932);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(36u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 56935);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 56938);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 56940);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 56942);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56946);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56948);
                c.step()?;
                label = if !c.z { 56951 } else { 56950 };
                continue;
            }
            56950 => {
                c.pc = (2745, 56950);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 56951;
                continue;
            }
            56951 => {
                c.pc = (2745, 56951);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56952);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 56954);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56956);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 56961);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 56965);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56967);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 56969);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(64u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 56973);
                c.step()?;
                c.invoke(p_1b55_4f89, true, 6997, 20361, 56978)?;
                label = 56978;
                continue;
            }
            56978 => {
                c.pc = (2745, 56978);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56980);
                c.step()?;
                label = if !c.z { 56917 } else { 56982 };
                continue;
            }
            56982 => {
                c.pc = (2745, 56982);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                c.r[7] = v;
                c.pc = (2745, 56985);
                c.step()?;
                let x = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(3u16),
                    8,
                )?;
                let v = c.inc(x, 8);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(3u16),
                    8,
                    v,
                )?;
                c.pc = (2745, 56988);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 56992);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 56994);
                c.step()?;
                label = if !c.z { 56997 } else { 56996 };
                continue;
            }
            56996 => {
                c.pc = (2745, 56996);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 56997;
                continue;
            }
            56997 => {
                c.pc = (2745, 56997);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 56998);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 57000);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57002);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 57007);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57011);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57013);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57017);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(44u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 57020);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(42u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 57023);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(36u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57026);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57028);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57030);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57034);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(44u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57037);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 57038);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57039);
                c.step()?;
                c.invoke(p_1b55_5c6e, true, 6997, 23662, 57044)?;
                label = 57044;
                continue;
            }
            57044 => {
                c.pc = (2745, 57044);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 57046);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 57050);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57052);
                c.step()?;
                label = if !c.z { 57055 } else { 57054 };
                continue;
            }
            57054 => {
                c.pc = (2745, 57054);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 57055;
                continue;
            }
            57055 => {
                c.pc = (2745, 57055);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (2745, 57056);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (2745, 57058);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_add(3414u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57062);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 57064);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57066);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[11], 0u16.wrapping_add(53407u16), 8, v)?;
                c.pc = (2745, 57069);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 1u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 57073);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57075);
                c.step()?;
                label = if !c.z { 57078 } else { 57077 };
                continue;
            }
            57077 => {
                c.pc = (2745, 57077);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 57078;
                continue;
            }
            57078 => {
                c.pc = (2745, 57078);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 57079);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53407u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57082);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 57084);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 57085);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57088);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 11u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 57092);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57094);
                c.step()?;
                let v = c.read(
                    c.r[10],
                    0u16.wrapping_add(c.r[5])
                        .wrapping_add(c.r[7])
                        .wrapping_sub(38u16),
                    8,
                )?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57097);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 57099);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 57100);
                c.step()?;
                c.invoke(p_0ab9_5083, false, 2745, 20611, 57103)?;
                label = 57103;
                continue;
            }
            57103 => {
                c.pc = (2745, 57103);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(38u16), 8, v)?;
                c.pc = (2745, 57107);
                c.step()?;
                let v = 2u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(34u16),
                    16,
                    v,
                )?;
                label = 57112;
                continue;
            }
            57112 => {
                c.pc = (2745, 57112);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57115);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(40u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 57118);
                c.step()?;
                label = if c.z { 57123 } else { 57120 };
                continue;
            }
            57120 => {
                c.pc = (2745, 57120);
                c.step()?;
                label = 56870;
                continue;
            }
            57123 => {
                c.pc = (2745, 57123);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(28u16), 16)?;
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 57127);
                c.step()?;
                label = if c.z { 57132 } else { 57129 };
                continue;
            }
            57129 => {
                c.pc = (2745, 57129);
                c.step()?;
                label = 56839;
                continue;
            }
            57132 => {
                c.pc = (2745, 57132);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(34u16), 16)?;
                let y = 2u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 57136);
                c.step()?;
                label = if c.z { 57141 } else { 57138 };
                continue;
            }
            57138 => {
                c.pc = (2745, 57138);
                c.step()?;
                label = 56707;
                continue;
            }
            57141 => {
                c.pc = (2745, 57141);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57144);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53382u16), 16, v)?;
                c.pc = (2745, 57147);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53382u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57150);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 0) & 255;
                c.r[0] = (c.r[0] & 255) | ((y & 255) << 8);
                c.r[0] = (c.r[0] & 65280) | ((x & 255) << 0);
                c.pc = (2745, 57152);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(53444u16), 16, v)?;
                c.pc = (2745, 57155);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53382u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57159);
                c.step()?;
                let o = 0u16.wrapping_add(55250u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57163);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57166);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 57168);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (2745, 57170);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(55014u16), 16)?;
                let y = 58u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 57175);
                c.step()?;
                let o = 0u16.wrapping_add(55246u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57179);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57181);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57183);
                c.step()?;
                let x = c.r[7];
                let y = 65477u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57187);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 57188);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57189);
                c.step()?;
                c.invoke(p_232a_01c5, true, 9002, 453, 57194)?;
                label = 57194;
                continue;
            }
            57194 => {
                c.pc = (2745, 57194);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53422u16), 16)?;
                c.push(v)?;
                c.pc = (2745, 57198);
                c.step()?;
                let v = 3u16;
                c.push(v)?;
                c.pc = (2745, 57200);
                c.step()?;
                c.invoke(p_0ab9_00d6, false, 2745, 214, 57203)?;
                label = 57203;
                continue;
            }
            57203 => {
                c.pc = (2745, 57203);
                c.step()?;
                let v = 56664u16;
                c.r[7] = v;
                c.pc = (2745, 57206);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 57207);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57208);
                c.step()?;
                c.invoke(p_1998_02df, true, 6552, 735, 57213)?;
                label = 57213;
                continue;
            }
            57213 => {
                c.pc = (2745, 57213);
                c.step()?;
                let v = 1u16;
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(30u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 57218);
                c.step()?;
                label = 57223;
                continue;
            }
            57220 => {
                c.pc = (2745, 57220);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                let v = c.inc(x, 16);
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(30u16),
                    16,
                    v,
                )?;
                label = 57223;
                continue;
            }
            57223 => {
                c.pc = (2745, 57223);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                let y = 25u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (2745, 57227);
                c.step()?;
                let x = c.r[7];
                let y = 52463u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57231);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (2745, 57232);
                c.step()?;
                let v = c.pop()?;
                c.r[8] = v;
                c.pc = (2745, 57233);
                c.step()?;
                let v = c.r[7];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(42u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 57236);
                c.step()?;
                let v = c.r[8];
                c.write(
                    c.r[10],
                    0u16.wrapping_add(c.r[5]).wrapping_sub(40u16),
                    16,
                    v,
                )?;
                c.pc = (2745, 57239);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 57240);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57241);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (2745, 57242);
                c.step()?;
                c.invoke(p_0ab9_1ca1, false, 2745, 7329, 57245)?;
                label = 57245;
                continue;
            }
            57245 => {
                c.pc = (2745, 57245);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 57247);
                c.step()?;
                label = if c.z { 57261 } else { 57249 };
                continue;
            }
            57249 => {
                c.pc = (2745, 57249);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(42u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57252);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 57253);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 57254);
                c.step()?;
                c.invoke(p_1998_093f, true, 6552, 2367, 57259)?;
                label = 57259;
                continue;
            }
            57259 => {
                c.pc = (2745, 57259);
                c.step()?;
                label = 57301;
                continue;
            }
            57261 => {
                c.pc = (2745, 57261);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53422u16), 16)?;
                c.r[0] = v;
                c.pc = (2745, 57264);
                c.step()?;
                let x = c.r[2];
                let y = c.r[2];
                let v = c.xor(x, y, 16);
                c.r[2] = v;
                c.pc = (2745, 57266);
                c.step()?;
                let v = 5u16;
                c.r[1] = v;
                c.pc = (2745, 57269);
                c.step()?;
                let v = c.r[1];
                c.divide(v, false)?;
                c.pc = (2745, 57271);
                c.step()?;
                c.exchange_registers(2, 0);
                c.pc = (2745, 57272);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.or(x, y, 16);
                c.r[0] = v;
                c.pc = (2745, 57274);
                c.step()?;
                label = if !c.z { 57301 } else { 57276 };
                continue;
            }
            57276 => {
                c.pc = (2745, 57276);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(42u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57279);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(14u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (2745, 57284);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 57288);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (2745, 57290);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57295);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 57301 } else { 57297 };
                continue;
            }
            57297 => {
                c.pc = (2745, 57297);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8)?;
                let v = c.dec(x, 8);
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 8, v)?;
                label = 57301;
                continue;
            }
            57301 => {
                c.pc = (2745, 57301);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(30u16), 16)?;
                let y = 22u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 57305);
                c.step()?;
                label = if !c.z { 57220 } else { 57307 };
                continue;
            }
            57307 => {
                c.pc = (2745, 57307);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(55005u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57312);
                c.step()?;
                label = if c.z { 57324 } else { 57314 };
                continue;
            }
            57314 => {
                c.pc = (2745, 57314);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(38u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57318);
                c.step()?;
                label = if !c.z { 57324 } else { 57320 };
                continue;
            }
            57320 => {
                c.pc = (2745, 57320);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (2745, 57321);
                c.step()?;
                c.invoke(p_0ab9_dbc6, false, 2745, 56262, 57324)?;
                label = 57324;
                continue;
            }
            57324 => {
                c.pc = (2745, 57324);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(38u16), 8)?;
                let y = 0u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (2745, 57328);
                c.step()?;
                label = if !c.z { 57335 } else { 57330 };
                continue;
            }
            57330 => {
                c.pc = (2745, 57330);
                c.step()?;
                c.invoke(p_0ab9_6b2f, false, 2745, 27439, 57333)?;
                label = 57333;
                continue;
            }
            57333 => {
                c.pc = (2745, 57333);
                c.step()?;
                label = 57347;
                continue;
            }
            57335 => {
                c.pc = (2745, 57335);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 57337);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (2745, 57339);
                c.step()?;
                c.invoke(p_1998_045c, true, 6552, 1116, 57344)?;
                label = 57344;
                continue;
            }
            57344 => {
                c.pc = (2745, 57344);
                c.step()?;
                c.invoke(p_0ab9_52b6, false, 2745, 21174, 57347)?;
                label = 57347;
                continue;
            }
            57347 => {
                c.pc = (2745, 57347);
                c.step()?;
                c.invoke(p_0ab9_78b7, false, 2745, 30903, 57350)?;
                label = 57350;
                continue;
            }
            57350 => {
                c.pc = (2745, 57350);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 57351);
                c.step()?;
                c.ret(false, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_0ab9_e05e(c: &mut Cpu) -> Result<(), String> {
    let mut label = 57438u16;
    loop {
        match label {
            57438 => {
                c.pc = (2745, 57438);
                c.step()?;
                c.invoke(p_0ab9_7d53, false, 2745, 32083, 57441)?;
                label = 57441;
                continue;
            }
            57441 => {
                c.pc = (2745, 57441);
                c.step()?;
                c.invoke(p_0ab9_7dd3, false, 2745, 32211, 57444)?;
                label = 57444;
                continue;
            }
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
pub(super) fn p_0ab9_e435(c: &mut Cpu) -> Result<(), String> {
    let mut label = 58421u16;
    loop {
        match label {
            58421 => {
                c.pc = (2745, 58421);
                c.step()?;
                let o = 0u16.wrapping_add(53386u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 58425);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 58426);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 58427);
                c.step()?;
                let o = 0u16.wrapping_add(53398u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (2745, 58431);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (2745, 58432);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (2745, 58433);
                c.step()?;
                let v = 4000u16;
                c.push(v)?;
                c.pc = (2745, 58436);
                c.step()?;
                c.invoke(p_26c9_0a12, true, 9929, 2578, 58441)?;
                label = 58441;
                continue;
            }
            58441 => {
                c.pc = (2745, 58441);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(53380u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 58444);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (2745, 58446);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (2745, 58449);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (2745, 58451);
                c.step()?;
                label = if c.z { 58454 } else { 58453 };
                continue;
            }
            58453 => {
                c.pc = (2745, 58453);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 58454;
                continue;
            }
            58454 => {
                c.pc = (2745, 58454);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (2745, 58455);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (2745, 58457);
                c.step()?;
                c.invoke(p_1998_045c, true, 6552, 1116, 58462)?;
                label = 58462;
                continue;
            }
            58462 => {
                c.pc = (2745, 58462);
                c.step()?;
                c.invoke(p_0ab9_b570, false, 2745, 46448, 58465)?;
                label = 58465;
                continue;
            }
            58465 => {
                c.pc = (2745, 58465);
                c.step()?;
                c.invoke(p_1998_0c7a, true, 6552, 3194, 58470)?;
                label = 58470;
                continue;
            }
            58470 => {
                c.pc = (2745, 58470);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(3352u16), 8, v)?;
                c.pc = (2745, 58475);
                c.step()?;
                let v = 0u16;
                c.write(c.r[11], 0u16.wrapping_add(3473u16), 8, v)?;
                c.pc = (2745, 58480);
                c.step()?;
                c.invoke(p_1998_00cd, true, 6552, 205, 58485)?;
                label = 58485;
                continue;
            }
            58485 => {
                c.pc = (2745, 58485);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (2745, 58486);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1998_00cd(c: &mut Cpu) -> Result<(), String> {
    super::projections::restore_presentation_clock(c)
}
pub(super) fn p_1998_0c7a(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3194u16;
    loop {
        match label {
            3194 => {
                c.pc = (6552, 3194);
                c.step()?;
                c.enter(8)?;
                c.pc = (6552, 3198);
                c.step()?;
                let v = 0u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (6552, 3202);
                c.step()?;
                label = 3207;
                continue;
            }
            3204 => {
                c.pc = (6552, 3204);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 3207;
                continue;
            }
            3207 => {
                c.pc = (6552, 3207);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3210);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (6552, 3211);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (6552, 3213);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3215);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (6552, 3219);
                c.step()?;
                c.invoke(p_1b55_0e7a, true, 6997, 3706, 3224)?;
                label = 3224;
                continue;
            }
            3224 => {
                c.pc = (6552, 3224);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = (c.r[0] >> 0) & 255;
                let v = c.or(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3226);
                c.step()?;
                label = if c.z { 3231 } else { 3228 };
                continue;
            }
            3228 => {
                c.pc = (6552, 3228);
                c.step()?;
                label = 3438;
                continue;
            }
            3231 => {
                c.pc = (6552, 3231);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3234);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (6552, 3235);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (6552, 3237);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3239);
                c.step()?;
                let x = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                let y = 62u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6552, 3244);
                c.step()?;
                let o = 0u16.wrapping_add(55234u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3248);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3250);
                c.step()?;
                let x = c.r[7];
                let y = 65474u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3254);
                c.step()?;
                let v = c.r[7];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16), 16, v)?;
                c.pc = (6552, 3257);
                c.step()?;
                let v = c.r[8];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(6u16), 16, v)?;
                c.pc = (6552, 3260);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (6552, 3265);
                c.step()?;
                label = 3270;
                continue;
            }
            3267 => {
                c.pc = (6552, 3267);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 3270;
                continue;
            }
            3270 => {
                c.pc = (6552, 3270);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (6552, 3273);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 3275);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3278);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3280);
                c.step()?;
                let x = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                let y = 15u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6552, 3285);
                c.step()?;
                let o = 0u16.wrapping_add(55230u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3289);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3291);
                c.step()?;
                let v = 0u16;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 8, v)?;
                c.pc = (6552, 3296);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 20u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 3300);
                c.step()?;
                label = if !c.z { 3267 } else { 3302 };
                continue;
            }
            3302 => {
                c.pc = (6552, 3302);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3305);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (6552, 3306);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (6552, 3308);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3310);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (6552, 3314);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3317);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (6552, 3318);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (6552, 3320);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3322);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.r[7] = v;
                c.pc = (6552, 3326);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(10107u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3330);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6552, 3332);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (6552, 3334);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(27657u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3338);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6552, 3339);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (6552, 3340);
                c.step()?;
                c.invoke(p_1998_0c23, false, 6552, 3107, 3343)?;
                label = 3343;
                continue;
            }
            3343 => {
                c.pc = (6552, 3343);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3346);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (6552, 3347);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6552, 3348);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3351);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (6552, 3352);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (6552, 3354);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3356);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.push(v)?;
                c.pc = (6552, 3360);
                c.step()?;
                c.invoke(p_1b55_7777, true, 6997, 30583, 3365)?;
                label = 3365;
                continue;
            }
            3365 => {
                c.pc = (6552, 3365);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                c.pc = (6552, 3370);
                c.step()?;
                label = 3375;
                continue;
            }
            3372 => {
                c.pc = (6552, 3372);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16, v)?;
                label = 3375;
                continue;
            }
            3375 => {
                c.pc = (6552, 3375);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[2] = v;
                c.pc = (6552, 3378);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3381);
                c.step()?;
                c.r[0] = i16::from(c.r[0] as u8 as i8) as u16;
                c.pc = (6552, 3382);
                c.step()?;
                let v = c.r[0];
                c.r[7] = v;
                c.pc = (6552, 3384);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3386);
                c.step()?;
                let v = c.read(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(10524u16),
                    16,
                )?;
                c.r[7] = v;
                c.pc = (6552, 3390);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(10107u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3394);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6552, 3396);
                c.step()?;
                let x = c.r[0];
                let y = 14u16;
                let v = c.imul3(x, y);
                c.r[7] = v;
                c.pc = (6552, 3399);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3401);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[7]).wrapping_sub(27644u16), 8)?;
                c.r[2] = (c.r[2] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3405);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                c.r[0] = v;
                c.pc = (6552, 3408);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 3410);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_sub(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3413);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3415);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (6552, 3419);
                c.step()?;
                let x = c.r[0];
                let y = 3u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (6552, 3422);
                c.step()?;
                let o = 0u16.wrapping_add(55226u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6552, 3426);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6552, 3428);
                c.step()?;
                let v = (c.r[2] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 8, v)?;
                c.pc = (6552, 3432);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 16)?;
                let y = 11u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6552, 3436);
                c.step()?;
                label = if !c.z { 3372 } else { 3438 };
                continue;
            }
            3438 => {
                c.pc = (6552, 3438);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 3442);
                c.step()?;
                label = if c.z { 3447 } else { 3444 };
                continue;
            }
            3444 => {
                c.pc = (6552, 3444);
                c.step()?;
                label = 3204;
                continue;
            }
            3447 => {
                c.pc = (6552, 3447);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6552, 3448);
                c.step()?;
                c.ret(true, 0)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1998_0dae(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3502u16;
    loop {
        match label {
            3502 => {
                c.pc = (6552, 3502);
                c.step()?;
                c.enter(2)?;
                c.pc = (6552, 3506);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                let y = 1u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 3510);
                c.step()?;
                label = if c.sf == c.of { 3518 } else { 3512 };
                continue;
            }
            3512 => {
                c.pc = (6552, 3512);
                c.step()?;
                let v = 1u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8, v)?;
                c.pc = (6552, 3516);
                c.step()?;
                label = 3528;
                continue;
            }
            3518 => {
                c.pc = (6552, 3518);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                let y = 32u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6552, 3522);
                c.step()?;
                label = if c.z || (c.sf != c.of) { 3528 } else { 3524 };
                continue;
            }
            3524 => {
                c.pc = (6552, 3524);
                c.step()?;
                let v = 32u16;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8, v)?;
                label = 3528;
                continue;
            }
            3528 => {
                c.pc = (6552, 3528);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3531);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (6552, 3534);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6552, 3537);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6552, 3538);
                c.step()?;
                c.ret(true, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1b55_17c2(c: &mut Cpu) -> Result<(), String> {
    let mut label = 6082u16;
    loop {
        match label {
            6082 => {
                c.pc = (6997, 6082);
                c.step()?;
                c.enter(4)?;
                c.pc = (6997, 6086);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.r[2] = v;
                c.pc = (6997, 6089);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 16)?;
                let y = 58u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6997, 6093);
                c.step()?;
                let o = 0u16.wrapping_add(55246u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 6097);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6099);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6101);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(59u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6105);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6997, 6106);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                c.pc = (6997, 6109);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (6997, 6110);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6997, 6111);
                c.step()?;
                c.invoke(p_232a_01e3, true, 9002, 483, 6116)?;
                label = 6116;
                continue;
            }
            6116 => {
                c.pc = (6997, 6116);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6119);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6997, 6122);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6124);
                c.step()?;
                label = if !c.z { 6127 } else { 6126 };
                continue;
            }
            6126 => {
                c.pc = (6997, 6126);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                label = 6127;
                continue;
            }
            6127 => {
                c.pc = (6997, 6127);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (6997, 6130);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6133);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6997, 6134);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1b55_19cd(c: &mut Cpu) -> Result<(), String> {
    let mut label = 6605u16;
    loop {
        match label {
            6605 => {
                c.pc = (6997, 6605);
                c.step()?;
                c.enter(6)?;
                c.pc = (6997, 6609);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (6997, 6612);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 16)?;
                c.r[0] = v;
                c.pc = (6997, 6616);
                c.step()?;
                let v = c.r[0];
                c.r[6] = v;
                c.pc = (6997, 6618);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (6997, 6620);
                c.step()?;
                let x = c.r[0];
                let y = c.r[6];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (6997, 6622);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (6997, 6624);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(10u16), 16)?;
                let y = 174u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6997, 6630);
                c.step()?;
                let o = 0u16.wrapping_add(55242u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 6634);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6636);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6638);
                c.step()?;
                let x = c.r[7];
                let y = 65359u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6642);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_add(2u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6646);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 15u16;
                let v = c.and(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6648);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6997, 6650);
                c.step()?;
                let x = c.r[0];
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6997, 6653);
                c.step()?;
                label = if !c.z { 6667 } else { 6655 };
                continue;
            }
            6655 => {
                c.pc = (6997, 6655);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (6997, 6658);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(8u16), 16)?;
                c.r[0] = v;
                c.pc = (6997, 6662);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                c.pc = (6997, 6665);
                c.step()?;
                label = 6677;
                continue;
            }
            6667 => {
                c.pc = (6997, 6667);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(4u16), 16)?;
                c.r[7] = v;
                c.pc = (6997, 6670);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[7]).wrapping_add(6u16), 16)?;
                c.r[0] = v;
                c.pc = (6997, 6674);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16, v)?;
                label = 6677;
                continue;
            }
            6677 => {
                c.pc = (6997, 6677);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 16)?;
                c.r[0] = v;
                c.pc = (6997, 6680);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6997, 6681);
                c.step()?;
                c.ret(false, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_1b55_1a1c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 6684u16;
    loop {
        match label {
            6684 => {
                c.pc = (6997, 6684);
                c.step()?;
                c.enter(4)?;
                c.pc = (6997, 6688);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                c.push(v)?;
                c.pc = (6997, 6691);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 16)?;
                c.push(v)?;
                c.pc = (6997, 6694);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (6997, 6697);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16);
                c.pc = (6997, 6700);
                c.step()?;
                let v = c.r[10];
                c.push(v)?;
                c.pc = (6997, 6701);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (6997, 6702);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (6997, 6703);
                c.step()?;
                c.invoke(p_1b55_17f9, false, 6997, 6137, 6706)?;
                label = 6706;
                continue;
            }
            6706 => {
                c.pc = (6997, 6706);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(4u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6709);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(3u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (6997, 6712);
                c.step()?;
                label = if !c.z { 6779 } else { 6714 };
                continue;
            }
            6714 => {
                c.pc = (6997, 6714);
                c.step()?;
                let v = 1u16;
                c.push(v)?;
                c.pc = (6997, 6716);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (6997, 6717);
                c.step()?;
                c.invoke(p_1b55_19cd, false, 6997, 6605, 6720)?;
                label = 6720;
                continue;
            }
            6720 => {
                c.pc = (6997, 6720);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (6997, 6722);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                let y = 58u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6997, 6726);
                c.step()?;
                let o = 0u16.wrapping_add(55246u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 6730);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6732);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6734);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(59u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6738);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 240u16;
                let v = c.and(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6740);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6997, 6742);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shr(x, y, 16);
                c.r[0] = v;
                c.pc = (6997, 6745);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (6997, 6746);
                c.step()?;
                let v = 0u16;
                c.push(v)?;
                c.pc = (6997, 6748);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (6997, 6749);
                c.step()?;
                c.invoke(p_1b55_19cd, false, 6997, 6605, 6752)?;
                label = 6752;
                continue;
            }
            6752 => {
                c.pc = (6997, 6752);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (6997, 6754);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 16)?;
                let y = 58u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (6997, 6758);
                c.step()?;
                let o = 0u16.wrapping_add(55246u16);
                let s = c.r[11];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (6997, 6762);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6764);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (6997, 6766);
                c.step()?;
                let v = c.read(c.r[8], 0u16.wrapping_add(c.r[7]).wrapping_sub(59u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6770);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 15u16;
                let v = c.and(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6772);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (6997, 6774);
                c.step()?;
                let v = c.pop()?;
                c.r[2] = v;
                c.pc = (6997, 6775);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (6997, 6777);
                c.step()?;
                label = if c.z { 6783 } else { 6779 };
                continue;
            }
            6779 => {
                c.pc = (6997, 6779);
                c.step()?;
                let v = 0u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6781);
                c.step()?;
                label = 6785;
                continue;
            }
            6783 => {
                c.pc = (6997, 6783);
                c.step()?;
                let v = 1u16;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 6785;
                continue;
            }
            6785 => {
                c.pc = (6997, 6785);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (6997, 6788);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (6997, 6791);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (6997, 6792);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_232a_01c5(c: &mut Cpu) -> Result<(), String> {
    let mut label = 453u16;
    loop {
        match label {
            453 => {
                c.pc = (9002, 453);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (9002, 454);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (9002, 456);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(11u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 459);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 461);
                c.step()?;
                let x = c.r[0];
                let y = 4u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (9002, 464);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (9002, 466);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 469);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 471);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (9002, 473);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9002, 476);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[8], 0u16.wrapping_add(c.r[7]), 8, v)?;
                c.pc = (9002, 479);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9002, 480);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_232a_026c(c: &mut Cpu) -> Result<(), String> {
    let mut label = 620u16;
    loop {
        match label {
            620 => {
                c.pc = (9002, 620);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (9002, 621);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (9002, 623);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9002, 626);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (9002, 627);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (9002, 628);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(8u16), 16)?;
                c.push(v)?;
                c.pc = (9002, 631);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.push(v)?;
                c.pc = (9002, 634);
                c.step()?;
                c.invoke(p_24bc_0243, true, 9404, 579, 639)?;
                label = 639;
                continue;
            }
            639 => {
                c.pc = (9002, 639);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9002, 640);
                c.step()?;
                c.ret(true, 8)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_232a_04a1(c: &mut Cpu) -> Result<(), String> {
    let mut label = 1185u16;
    loop {
        match label {
            1185 => {
                c.pc = (9002, 1185);
                c.step()?;
                c.enter(2)?;
                c.pc = (9002, 1189);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1192);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8, v)?;
                c.pc = (9002, 1195);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(12u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1198);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (9002, 1201);
                c.step()?;
                label = if !c.cf && !c.z { 1283 } else { 1203 };
                continue;
            }
            1203 => {
                c.pc = (9002, 1203);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                c.pc = (9002, 1206);
                c.step()?;
                label = 1211;
                continue;
            }
            1208 => {
                c.pc = (9002, 1208);
                c.step()?;
                let x = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                let v = c.inc(x, 8);
                c.write(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8, v)?;
                label = 1211;
                continue;
            }
            1211 => {
                c.pc = (9002, 1211);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(16u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1214);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 1216);
                c.step()?;
                let x = c.r[0];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (9002, 1218);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (9002, 1220);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1223);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 1225);
                c.step()?;
                let x = c.r[0];
                let y = 160u16;
                let v = c.imul3(x, y);
                c.r[0] = v;
                c.pc = (9002, 1229);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9002, 1232);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[7]);
                let s = c.r[8];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9002, 1235);
                c.step()?;
                let x = c.r[7];
                let y = c.r[0];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (9002, 1237);
                c.step()?;
                let x = c.r[7];
                let y = c.r[2];
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (9002, 1239);
                c.step()?;
                let x = c.r[7];
                let y = 65374u16;
                let v = c.add(x, y, 16);
                c.r[7] = v;
                c.pc = (9002, 1243);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (9002, 1244);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (9002, 1245);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(16u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1248);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 1250);
                c.step()?;
                let v = c.r[0];
                c.r[2] = v;
                c.pc = (9002, 1252);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(14u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1255);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 1257);
                c.step()?;
                let x = c.r[0];
                let y = c.r[2];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (9002, 1259);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (9002, 1260);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (9002, 1261);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(56676u16), 16)?;
                c.r[0] = v;
                c.pc = (9002, 1264);
                c.step()?;
                let x = c.r[0];
                let y = 8u16;
                let v = c.shl(x, y, 16);
                c.r[0] = v;
                c.pc = (9002, 1267);
                c.step()?;
                let x = c.r[0];
                let y = 32u16;
                let v = c.add(x, y, 16);
                c.r[0] = v;
                c.pc = (9002, 1270);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (9002, 1271);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (9002, 1272);
                c.step()?;
                c.invoke(p_232a_026c, false, 9002, 620, 1275)?;
                label = 1275;
                continue;
            }
            1275 => {
                c.pc = (9002, 1275);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(1u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1278);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_sub(2u16), 8)?;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (9002, 1281);
                c.step()?;
                label = if !c.z { 1208 } else { 1283 };
                continue;
            }
            1283 => {
                c.pc = (9002, 1283);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9002, 1284);
                c.step()?;
                c.ret(true, 12)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_232a_0507(c: &mut Cpu) -> Result<(), String> {
    let mut label = 1287u16;
    loop {
        match label {
            1287 => {
                c.pc = (9002, 1287);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (9002, 1288);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (9002, 1290);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(18u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1293);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 1295);
                c.step()?;
                let v = c.r[0];
                c.write(c.r[11], 0u16.wrapping_add(56676u16), 16, v)?;
                c.pc = (9002, 1298);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(16u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1301);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (9002, 1302);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(14u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1305);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (9002, 1306);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(12u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1309);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (9002, 1310);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(10u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1313);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (9002, 1314);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9002, 1317);
                c.step()?;
                let v = c.r[8];
                c.push(v)?;
                c.pc = (9002, 1318);
                c.step()?;
                let v = c.r[7];
                c.push(v)?;
                c.pc = (9002, 1319);
                c.step()?;
                let v = c.r[9];
                c.push(v)?;
                c.pc = (9002, 1320);
                c.step()?;
                c.invoke(p_232a_04a1, false, 9002, 1185, 1323)?;
                label = 1323;
                continue;
            }
            1323 => {
                c.pc = (9002, 1323);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9002, 1324);
                c.step()?;
                c.ret(true, 14)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_232a_0783(c: &mut Cpu) -> Result<(), String> {
    let mut label = 1923u16;
    loop {
        match label {
            1923 => {
                c.pc = (9002, 1923);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (9002, 1924);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (9002, 1926);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9002, 1929);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9002, 1931);
                c.step()?;
                let v = c.r[0];
                c.push(v)?;
                c.pc = (9002, 1932);
                c.step()?;
                c.invoke(p_24bc_0c81, true, 9404, 3201, 1937)?;
                label = 1937;
                continue;
            }
            1937 => {
                c.pc = (9002, 1937);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9002, 1938);
                c.step()?;
                c.ret(true, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_24bc_005a(c: &mut Cpu) -> Result<(), String> {
    let mut label = 90u16;
    loop {
        match label {
            90 => {
                c.pc = (9404, 90);
                c.step()?;
                c.enter(256)?;
                c.pc = (9404, 94);
                c.step()?;
                let v = c.r[10];
                c.r[3] = v;
                c.pc = (9404, 96);
                c.step()?;
                let v = c.r[3];
                c.r[8] = v;
                c.pc = (9404, 98);
                c.step()?;
                let v = c.r[11];
                c.r[3] = v;
                c.pc = (9404, 100);
                c.step()?;
                c.df = false;
                c.pc = (9404, 101);
                c.step()?;
                c.r[7] = 0u16.wrapping_add(c.r[5]).wrapping_sub(256u16);
                c.pc = (9404, 105);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[6] = v;
                c.r[11] = g;
                c.pc = (9404, 108);
                c.step()?;
                c.load_string(8)?;
                c.pc = (9404, 109);
                c.step()?;
                c.store_string(8)?;
                c.pc = (9404, 110);
                c.step()?;
                c.exchange_registers(1, 0);
                c.pc = (9404, 111);
                c.step()?;
                let x = (c.r[1] >> 8) & 255;
                let y = (c.r[1] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[1] = (c.r[1] & 255) | ((v & 255) << 8);
                c.pc = (9404, 113);
                c.step()?;
                c.repeat_movs(8)?;
                c.pc = (9404, 115);
                c.step()?;
                let v = c.r[3];
                c.r[11] = v;
                c.pc = (9404, 117);
                c.step()?;
                let v = c.r[11];
                c.push(v)?;
                c.pc = (9404, 118);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[6] = v;
                c.r[11] = g;
                c.pc = (9404, 121);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[5]).wrapping_add(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9404, 124);
                c.step()?;
                c.df = false;
                c.pc = (9404, 125);
                c.step()?;
                c.load_string(8)?;
                c.pc = (9404, 126);
                c.step()?;
                c.store_string(8)?;
                c.pc = (9404, 127);
                c.step()?;
                let v = (c.r[0] >> 0) & 255;
                c.r[1] = (c.r[1] & 65280) | ((v & 255) << 0);
                c.pc = (9404, 129);
                c.step()?;
                let x = (c.r[1] >> 8) & 255;
                let y = (c.r[1] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[1] = (c.r[1] & 255) | ((v & 255) << 8);
                c.pc = (9404, 131);
                c.step()?;
                label = if c.r[1] == 0 { 147 } else { 133 };
                continue;
            }
            133 => {
                c.pc = (9404, 133);
                c.step()?;
                c.load_string(8)?;
                c.pc = (9404, 134);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 97u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (9404, 136);
                c.step()?;
                label = if c.cf { 144 } else { 138 };
                continue;
            }
            138 => {
                c.pc = (9404, 138);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 122u16;
                let v = c.sub(x, y, 8);
                let _ = v;
                c.pc = (9404, 140);
                c.step()?;
                label = if !c.cf && !c.z { 144 } else { 142 };
                continue;
            }
            142 => {
                c.pc = (9404, 142);
                c.step()?;
                let x = (c.r[0] >> 0) & 255;
                let y = 32u16;
                let v = c.sub(x, y, 8);
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                label = 144;
                continue;
            }
            144 => {
                c.pc = (9404, 144);
                c.step()?;
                c.store_string(8)?;
                c.pc = (9404, 145);
                c.step()?;
                c.r[1] = c.r[1].wrapping_sub(1);
                label = if c.r[1] != 0 { 133 } else { 147 };
                continue;
            }
            147 => {
                c.pc = (9404, 147);
                c.step()?;
                let v = c.pop()?;
                c.r[11] = v;
                c.pc = (9404, 148);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9404, 149);
                c.step()?;
                c.ret(true, 4)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_24bc_0243(_c: &mut Cpu) -> Result<(), String> {
    Err("unported original 24BC:0243".into())
}
pub(super) fn p_24bc_0c81(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3201u16;
    loop {
        match label {
            3201 => {
                c.pc = (9404, 3201);
                c.step()?;
                let v = c.r[5];
                c.push(v)?;
                c.pc = (9404, 3202);
                c.step()?;
                let v = c.r[4];
                c.r[5] = v;
                c.pc = (9404, 3204);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(56974u16), 16)?;
                let y = 10u16;
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (9404, 3209);
                c.step()?;
                label = if !c.cf { 3228 } else { 3211 };
                continue;
            }
            3211 => {
                c.pc = (9404, 3211);
                c.step()?;
                let x = c.read(c.r[11], 0u16.wrapping_add(56974u16), 16)?;
                let v = c.inc(x, 16);
                c.write(c.r[11], 0u16.wrapping_add(56974u16), 16, v)?;
                c.pc = (9404, 3215);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[5]).wrapping_add(6u16), 16)?;
                c.r[0] = v;
                c.pc = (9404, 3218);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(56974u16), 16)?;
                c.r[7] = v;
                c.pc = (9404, 3222);
                c.step()?;
                let x = c.r[7];
                let y = 1u16;
                let v = c.shl(x, y, 16);
                c.r[7] = v;
                c.pc = (9404, 3224);
                c.step()?;
                let v = c.r[0];
                c.write(
                    c.r[11],
                    0u16.wrapping_add(c.r[7]).wrapping_sub(8562u16),
                    16,
                    v,
                )?;
                label = 3228;
                continue;
            }
            3228 => {
                c.pc = (9404, 3228);
                c.step()?;
                c.r[4] = c.r[5];
                c.r[5] = c.pop()?;
                c.pc = (9404, 3229);
                c.step()?;
                c.ret(true, 2)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_26c9_0b91(c: &mut Cpu) -> Result<(), String> {
    let mut label = 2961u16;
    loop {
        match label {
            2961 => {
                c.pc = (9929, 2961);
                c.step()?;
                c.df = false;
                c.pc = (9929, 2962);
                c.step()?;
                let v = c.r[4];
                c.r[3] = v;
                c.pc = (9929, 2964);
                c.step()?;
                let v = c.r[11];
                c.r[2] = v;
                c.pc = (9929, 2966);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[3]).wrapping_add(12u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9929, 2970);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[3]).wrapping_add(8u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[6] = v;
                c.r[11] = g;
                c.pc = (9929, 2974);
                c.step()?;
                let v = c.read(c.r[11], 0u16.wrapping_add(c.r[6]), 8)?;
                c.r[0] = (c.r[0] & 65280) | ((v & 255) << 0);
                c.pc = (9929, 2976);
                c.step()?;
                let x = (c.r[0] >> 8) & 255;
                let y = (c.r[0] >> 8) & 255;
                let v = c.xor(x, y, 8);
                c.r[0] = (c.r[0] & 255) | ((v & 255) << 8);
                c.pc = (9929, 2978);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[3]).wrapping_add(6u16), 16)?;
                c.r[1] = v;
                c.pc = (9929, 2982);
                c.step()?;
                let x = c.r[1];
                let y = c.r[1];
                let v = c.or(x, y, 16);
                c.r[1] = v;
                c.pc = (9929, 2984);
                c.step()?;
                label = if !c.z && (c.sf == c.of) { 2989 } else { 2986 };
                continue;
            }
            2986 => {
                c.pc = (9929, 2986);
                c.step()?;
                let v = 1u16;
                c.r[1] = v;
                label = 2989;
                continue;
            }
            2989 => {
                c.pc = (9929, 2989);
                c.step()?;
                let x = c.r[6];
                let y = c.r[1];
                let v = c.add(x, y, 16);
                c.r[6] = v;
                c.pc = (9929, 2991);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                c.r[0] = v;
                c.pc = (9929, 2993);
                c.step()?;
                label = if c.cf { 3014 } else { 2995 };
                continue;
            }
            2995 => {
                c.pc = (9929, 2995);
                c.step()?;
                let x = c.r[0];
                let v = c.inc(x, 16);
                c.r[0] = v;
                c.pc = (9929, 2996);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[3]).wrapping_add(4u16), 16)?;
                c.r[1] = v;
                c.pc = (9929, 3000);
                c.step()?;
                let x = c.r[1];
                let y = c.r[1];
                let v = c.or(x, y, 16);
                c.r[1] = v;
                c.pc = (9929, 3002);
                c.step()?;
                label = if c.sf == c.of { 3006 } else { 3004 };
                continue;
            }
            3004 => {
                c.pc = (9929, 3004);
                c.step()?;
                let x = c.r[1];
                let y = c.r[1];
                let v = c.xor(x, y, 16);
                c.r[1] = v;
                label = 3006;
                continue;
            }
            3006 => {
                c.pc = (9929, 3006);
                c.step()?;
                let x = c.r[0];
                let y = c.r[1];
                let v = c.sub(x, y, 16);
                let _ = v;
                c.pc = (9929, 3008);
                c.step()?;
                label = if c.cf || c.z { 3016 } else { 3010 };
                continue;
            }
            3010 => {
                c.pc = (9929, 3010);
                c.step()?;
                let v = c.r[1];
                c.r[0] = v;
                c.pc = (9929, 3012);
                c.step()?;
                label = 3016;
                continue;
            }
            3014 => {
                c.pc = (9929, 3014);
                c.step()?;
                let x = c.r[0];
                let y = c.r[0];
                let v = c.xor(x, y, 16);
                c.r[0] = v;
                label = 3016;
                continue;
            }
            3016 => {
                c.pc = (9929, 3016);
                c.step()?;
                c.store_string(8)?;
                c.pc = (9929, 3017);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (9929, 3019);
                c.step()?;
                c.repeat_movs(8)?;
                c.pc = (9929, 3021);
                c.step()?;
                let v = c.r[2];
                c.r[11] = v;
                c.pc = (9929, 3023);
                c.step()?;
                c.ret(true, 8)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
pub(super) fn p_26c9_0c81(c: &mut Cpu) -> Result<(), String> {
    let mut label = 3201u16;
    loop {
        match label {
            3201 => {
                c.pc = (9929, 3201);
                c.step()?;
                c.df = false;
                c.pc = (9929, 3202);
                c.step()?;
                let v = c.r[4];
                c.r[3] = v;
                c.pc = (9929, 3204);
                c.step()?;
                let v = c.r[11];
                c.r[2] = v;
                c.pc = (9929, 3206);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[3]).wrapping_add(10u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[7] = v;
                c.r[8] = g;
                c.pc = (9929, 3210);
                c.step()?;
                let o = 0u16.wrapping_add(c.r[3]).wrapping_add(6u16);
                let s = c.r[10];
                let (v, g) = c.far(s, o)?;
                c.r[6] = v;
                c.r[11] = g;
                c.pc = (9929, 3214);
                c.step()?;
                let v = c.read(c.r[10], 0u16.wrapping_add(c.r[3]).wrapping_add(4u16), 16)?;
                c.r[0] = v;
                c.pc = (9929, 3218);
                c.step()?;
                c.store_string(8)?;
                c.pc = (9929, 3219);
                c.step()?;
                let v = c.r[0];
                c.r[1] = v;
                c.pc = (9929, 3221);
                c.step()?;
                c.repeat_movs(8)?;
                c.pc = (9929, 3223);
                c.step()?;
                let v = c.r[2];
                c.r[11] = v;
                c.pc = (9929, 3225);
                c.step()?;
                c.ret(true, 6)?;
                return Ok(());
            }
            _ => return Err(format!("invalid compiled label {label:04X}")),
        }
    }
}
