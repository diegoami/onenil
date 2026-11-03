//! [designed] Arithmetic/ABI support for compiled native Rust procedures.
//! No opcode decoding,original executable or instruction stream exists here.
use crate::{Draw, Engine};
use std::cell::Cell;
#[derive(Clone,Copy,Debug,serde::Serialize,PartialEq,Eq)]
pub(super) struct Missing {pub scratch:bool,pub address:usize,pub writing:bool,pub pc:(u16,u16)}
pub(super) type Proc = fn(&mut Cpu) -> Result<(), String>;
pub(super) struct Cpu {
    pub r: [u16; 12],
    pub pc: (u16, u16),
    pub cf: bool,
    pub z: bool,
    pub sf: bool,
    pub of: bool,
    pub pf: bool,
    pub e: Engine,
    pub draws: Vec<Draw>,
    pub known: Vec<bool>,
    pub missing:Cell<Option<Missing>>,
    pub require_write_baseline:bool,
    pub flat_memory:bool,
    pub code_base:u16,
    pub stack_bounds:Option<(usize,usize)>,
    stack: Vec<u8>,
    stack_known: Vec<bool>,
    frames: Vec<(u16, u16)>,
    fuel: usize,
}
impl Cpu {
    pub fn new(e: Engine) -> Self {
        let n = e.image.len();
        let mut c = Self {
            r: [0; 12],
            pc: (0, 0),
            cf: false,
            z: false,
            sf: false,
            of: false,
            pf: false,
            e,
            draws: vec![],
            known: vec![false; n],
            missing:Cell::new(None),
            require_write_baseline:false,
            flat_memory:false,
            code_base:0,
            stack_bounds:None,
            stack: vec![0; 65540],
            stack_known: vec![false; 65540],
            frames: vec![],
            fuel: 1_000_000,
        };
        c.r[4] = 0xFF00;
        c.r[9] = 0x0AB9;
        c.r[10] = 0xF000;
        c
    }
    pub fn exchange_registers(&mut self,_first:usize,_second:usize){}
    pub fn read(&self, s: u16, o: u16, w: u8) -> Result<u16, String> {
        let mut value = 0u16;
        for i in 0..usize::from(w / 8) {
            let at = usize::from(o) + i;
            let b = if s == 0xF000 {
                if !self.stack_known.get(at).copied().unwrap_or(false) {
                    return Err(format!(
                        "uninitialized native scratch at {:04X}:{:04X} SS:{at:04X}",
                        self.pc.0, self.pc.1
                    ));
                }
                self.stack[at]
            } else {
                let a = 16 * usize::from(s) + at;
                if !self.known.get(a).copied().unwrap_or(false) {
                    return Err(format!(
                        "missing reached native data at {:04X}:{:04X} {s:04X}:{o:04X}",
                        self.pc.0, self.pc.1
                    ));
                }
                self.e.image[a]
            };
            value |= u16::from(b) << (8 * i);
        }
        Ok(value)
    }
    pub fn far(&self, s: u16, o: u16) -> Result<(u16, u16), String> {
        let first = self.read(s, o, 16)?;
        let at = usize::from(o) + 2;
        let mut second = 0;
        for i in 0..2 {
            let a = at + i;
            let b = if s == 0xF000 {
                if !self.stack_known[a] {
                    return Err("uninitialized far scratch".into());
                }
                self.stack[a]
            } else {
                let a = 16 * usize::from(s) + a;
                if !self.known.get(a).copied().unwrap_or(false) {
                    return Err("missing reached far-pointer data".into());
                }
                self.e.image[a]
            };
            second |= u16::from(b) << (8 * i)
        }
        Ok((first, second))
    }
    pub fn write(&mut self, s: u16, o: u16, w: u8, v: u16) -> Result<(), String> {
        for i in 0..usize::from(w / 8) {
            let at = usize::from(o) + i;
            let b = (v >> (8 * i)) as u8;
            if s == 0xF000 {
                self.stack[at] = b;
                self.stack_known[at] = true
            } else {
                let a = 16 * usize::from(s) + at;
                if a >= self.e.image.len() {
                    return Err("unsupported native data write".into());
                }
                self.e.image[a] = b;
                self.known[a] = true;
            }
        }
        Ok(())
    }
    pub fn step(&mut self) -> Result<(), String> {
        if self.fuel == 0 {
            return Err("compiled procedure budget exhausted".into());
        }
        self.fuel -= 1;
        Ok(())
    }
    pub fn push(&mut self, v: u16) -> Result<(), String> {
        self.r[4] = self.r[4].wrapping_sub(2);
        self.write(self.r[10], self.r[4], 16, v)
    }
    pub fn pop(&mut self) -> Result<u16, String> {
        let v = self.read(self.r[10], self.r[4], 16)?;
        self.r[4] = self.r[4].wrapping_add(2);
        Ok(v)
    }
    pub fn enter(&mut self, n: u16) -> Result<(), String> {
        self.push(self.r[5])?;
        self.r[5] = self.r[4];
        self.r[4] = self.r[4].wrapping_sub(n);
        Ok(())
    }
    pub fn ret(&mut self, far: bool, n: u16) -> Result<(), String> {
        let _return_ip = self.pop()?;
        if far {
            self.r[9] = self.pop()?
        }
        self.r[4] = self.r[4].wrapping_add(n);
        Ok(())
    }
    pub fn invoke(&mut self, f: Proc, far: bool, cs: u16, ip: u16, ret: u16) -> Result<(), String> {
        let caller = self.r[9];
        if far {
            self.push(caller)?;
            self.r[9] = cs
        }
        self.push(ret)?;
        self.frames.push((caller, ret));
        let rng = (cs, ip) == (0x24BC, 0x0CDC);
        let n = if rng {
            self.read(self.r[10], self.r[4].wrapping_add(4), 16)?
        } else {
            0
        };
        let before = self.e.word(crate::SEED);
        f(self)?;
        if rng {
            let count = self.frames.len();
            let site = if self.frames[count - 1] == (0x24BC, 0x0D4F) {
                self.frames[count - 2]
            } else {
                self.frames[count - 3]
            };
            self.draws.push(Draw {
                site: format!("{:04X}:{:04X}", site.0, site.1),
                n,
                seed_before: before,
                seed_after: self.e.word(crate::SEED),
                value: self.r[0],
            });
        }
        self.frames.pop();
        Ok(())
    }
    fn mask(w: u8) -> u32 {
        if w == 8 {
            255
        } else {
            65535
        }
    }
    fn status(&mut self, v: u16, w: u8) {
        let sign = 1u16 << (w - 1);
        self.z = v == 0;
        self.sf = v & sign != 0;
        self.pf = (v as u8).count_ones() % 2 == 0;
    }
    pub fn add(&mut self, x: u16, y: u16, w: u8) -> u16 {
        self.add_carry(x, y, w, 0)
    }
    fn add_carry(&mut self, x: u16, y: u16, w: u8, carry: u32) -> u16 {
        let m = Self::mask(w);
        let x = u32::from(x) & m;
        let y = u32::from(y) & m;
        let sum = x + y + carry;
        let v = (sum & m) as u16;
        self.cf = sum > m;
        self.of = (!(x ^ y) & (x ^ u32::from(v)) & (1 << (w - 1))) != 0;
        self.status(v, w);
        v
    }
    pub fn sub(&mut self, x: u16, y: u16, w: u8) -> u16 {
        self.sub_borrow(x, y, w, 0)
    }
    fn sub_borrow(&mut self, x: u16, y: u16, w: u8, borrow: u32) -> u16 {
        let m = Self::mask(w);
        let x = u32::from(x) & m;
        let y = u32::from(y) & m;
        let v = (x.wrapping_sub(y + borrow) & m) as u16;
        self.cf = x < y + borrow;
        self.of = ((x ^ y) & (x ^ u32::from(v)) & (1 << (w - 1))) != 0;
        self.status(v, w);
        v
    }
    pub fn adc(&mut self, x: u16, y: u16, w: u8) -> u16 {
        self.add_carry(x, y, w, u32::from(self.cf))
    }
    pub fn sbb(&mut self, x: u16, y: u16, w: u8) -> u16 {
        self.sub_borrow(x, y, w, u32::from(self.cf))
    }
    fn logic(&mut self, v: u16, w: u8) -> u16 {
        let v = v & (Self::mask(w) as u16);
        self.cf = false;
        self.of = false;
        self.status(v, w);
        v
    }
    pub fn and(&mut self, x: u16, y: u16, w: u8) -> u16 {
        self.logic(x & y, w)
    }
    pub fn or(&mut self, x: u16, y: u16, w: u8) -> u16 {
        self.logic(x | y, w)
    }
    pub fn xor(&mut self, x: u16, y: u16, w: u8) -> u16 {
        self.logic(x ^ y, w)
    }
    pub fn shl(&mut self, x: u16, y: u16, w: u8) -> u16 {
        let n = y & 31;
        let mut v = x & (Self::mask(w) as u16);
        for _ in 0..n {
            self.cf = v & (1 << (w - 1)) != 0;
            v = v.wrapping_shl(1) & (Self::mask(w) as u16)
        }
        if n > 0 {
            self.status(v, w);
            if n == 1 {
                self.of = self.sf ^ self.cf
            }
        }
        v
    }
    pub fn shr(&mut self, x: u16, y: u16, w: u8) -> u16 {
        let n = y & 31;
        let mut v = x & (Self::mask(w) as u16);
        for _ in 0..n {
            self.cf = v & 1 != 0;
            v >>= 1
        }
        if n > 0 {
            self.status(v, w);
            if n == 1 {
                self.of = x & (1 << (w - 1)) != 0
            }
        }
        v
    }
    pub fn sar(&mut self, x: u16, y: u16, w: u8) -> u16 {
        let n = y & 31;
        let mut v = x & (Self::mask(w) as u16);
        for _ in 0..n {
            self.cf = v & 1 != 0;
            v = (v >> 1) | (v & (1 << (w - 1)))
        }
        if n > 0 {
            self.status(v, w);
            if n == 1 {
                self.of = false
            }
        }
        v
    }
    pub fn inc(&mut self, x: u16, w: u8) -> u16 {
        let old = self.cf;
        let v = self.add(x, 1, w);
        self.cf = old;
        v
    }
    pub fn dec(&mut self, x: u16, w: u8) -> u16 {
        let old = self.cf;
        let v = self.sub(x, 1, w);
        self.cf = old;
        v
    }
    pub fn neg(&mut self, x: u16, w: u8) -> u16 {
        self.sub(0, x, w)
    }
    pub fn not(&mut self, x: u16, w: u8) -> u16 {
        !x & (Self::mask(w) as u16)
    }
    pub fn imul3(&mut self, x: u16, y: u16) -> u16 {
        let v = i32::from(x as i16) * i32::from(y as i16);
        self.cf = v != i32::from(v as i16);
        self.of = self.cf;
        v as u16
    }
    pub fn multiply(&mut self, v: u16, signed: bool) {
        let product = if signed {
            (i32::from(self.r[0] as i16) * i32::from(v as i16)) as u32
        } else {
            u32::from(self.r[0]) * u32::from(v)
        };
        self.r[0] = product as u16;
        self.r[2] = (product >> 16) as u16;
        self.cf = if signed {
            (product as i32) != i32::from(self.r[0] as i16)
        } else {
            self.r[2] != 0
        };
        self.of = self.cf;
    }
    pub fn divide(&mut self, v: u16, signed: bool) -> Result<(), String> {
        if v == 0 {
            return Err("native integer division by zero".into());
        }
        let bits = (u32::from(self.r[2]) << 16) | u32::from(self.r[0]);
        let (q, r) = if signed {
            let x = i64::from(bits as i32);
            let d = i64::from(v as i16);
            (x / d, x % d)
        } else {
            (
                i64::from(bits / u32::from(v)),
                i64::from(bits % u32::from(v)),
            )
        };
        if (signed && !(-32768..=32767).contains(&q)) || (!signed && !(0..=65535).contains(&q)) {
            return Err("native integer division overflow".into());
        }
        self.r[0] = q as u16;
        self.r[2] = r as u16;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn cpu() -> Cpu {
        Cpu::new(crate::role_scores::image(Default::default()))
    }
    #[test]
    fn add_and_sub_signed_unsigned_boundaries() {
        let mut c = cpu();
        assert_eq!(c.add(0x7FFF, 1, 16), 0x8000);
        assert!(c.of && c.sf && !c.cf);
        assert_eq!(c.add(255, 1, 8), 0);
        assert!(c.cf && c.z && !c.of);
        assert_eq!(c.sub(0, 1, 16), 65535);
        assert!(c.cf && c.sf && !c.of);
        assert_eq!(c.sub(128, 1, 8), 127);
        assert!(c.of && !c.sf);
    }
    #[test]
    fn inc_dec_preserve_carry_and_logic_clear_it() {
        let mut c = cpu();
        c.cf = true;
        assert_eq!(c.inc(65535, 16), 0);
        assert!(c.cf && c.z);
        assert_eq!(c.dec(0, 8), 255);
        assert!(c.cf && c.sf);
        assert_eq!(c.or(128, 1, 8), 129);
        assert!(!c.cf && !c.of && c.sf);
    }
    #[test]
    fn shifts_mask_counts_and_signed_extension() {
        let mut c = cpu();
        assert_eq!(c.sar(0x8001, 1, 16), 0xC000);
        assert!(c.cf && c.sf);
        assert_eq!(c.shr(0x8001, 1, 16), 0x4000);
        assert!(c.cf && !c.sf && c.of);
        assert_eq!(c.shl(1, 33, 16), 2);
        assert_eq!(c.shr(255, 8, 16), 0);
    }
    #[test]
    fn missing_engine_and_scratch_reads_diagnose() {
        let mut c = cpu();
        assert!(c.read(0, 1, 8).is_err());
        assert!(c.read(0xF000, 100, 16).is_err());
        c.write(0xF000, 100, 16, 42).unwrap();
        assert_eq!(c.read(0xF000, 100, 16).unwrap(), 42);
    }
    #[test]
    fn far_and_near_frames_have_original_pascal_argument_offsets() {
        fn child(c: &mut Cpu) -> Result<(), String> {
            c.enter(2)?;
            let arg = c.read(c.r[10], c.r[5] + 6, 16)?;
            c.r[0] = arg;
            c.r[4] = c.r[5];
            c.r[5] = c.pop()?;
            c.ret(true, 2)
        }
        fn parent(c: &mut Cpu) -> Result<(), String> {
            c.enter(2)?;
            c.push(42)?;
            c.push(c.r[9])?;
            c.invoke(child, false, 0x0AB9, 0x1234, 0x5678)?;
            assert_eq!(c.r[0], 42);
            c.r[4] = c.r[5];
            c.r[5] = c.pop()?;
            c.ret(true, 0)
        }
        let mut c = cpu();
        let sp = c.r[4];
        c.invoke(parent, true, 0x0AB9, 0x9999, 0).unwrap();
        assert_eq!((c.r[4], c.r[5], c.r[9]), (sp, 0, 0x0AB9));
    }
    #[test]
    fn word_crosses_segment_end_after_effective_offset_wrap() {
        let mut c = cpu();
        c.write(0x1000, 65535, 16, 0x1234).unwrap();
        c.write(0x1000, 0, 8, 0x56).unwrap();
        assert_eq!(c.read(0x1000, 65535, 16).unwrap(), 0x1234);
        c.write(0x1000, 65533, 16, 42).unwrap();
        assert_eq!(c.far(0x1000, 65533).unwrap(), (42, 0x1234));
    }
    #[test]
    fn division_signed_word_and_overflow() {
        let mut c = cpu();
        c.r[0] = 65529;
        c.r[2] = 65535;
        c.divide(3, true).unwrap();
        assert_eq!(c.r[0], 65534);
        assert_eq!(c.r[2], 65535);
        c.r[0] = 1;
        c.r[2] = 1;
        assert!(c.divide(1, false).is_err());
        assert!(c.divide(0, false).is_err());
    }
    #[test]
    fn missing_diagnostics_identify_actual_data_and_scratch_bytes(){
        let c=cpu();assert!(c.read(0,42,8).is_err());
        assert_eq!(c.missing.get().unwrap().address,42);
        assert!(!c.missing.get().unwrap().scratch);
        assert!(c.read(0xF000,99,8).is_err());
        assert!(c.missing.get().unwrap().scratch);
        assert_eq!(c.missing.get().unwrap().address,99);
    }
    #[test]
    fn strict_first_write_requires_original_baseline_even_for_zero(){
        let mut c=cpu();c.require_write_baseline=true;
        assert!(c.write(0,0xCF46,8,0).is_err());
        let m=c.missing.get().unwrap();assert!(m.writing&&!m.scratch);assert_eq!(m.address,0xCF46);
        c.require_write_baseline=false;c.write(0,0xCF46,8,7).unwrap();
        c.require_write_baseline=true;c.write(0,0xCF46,8,0).unwrap();
        assert_eq!(c.read(0,0xCF46,8).unwrap(),0);
        c.write(0xF000,42,8,1).unwrap();
    }

    #[test]
    fn flat_memory_preserves_segment_aliases_for_stack_data(){
        let mut e=crate::role_scores::image(Default::default());e.image.resize(0x110000,0);
        let mut c=Cpu::new(e);c.flat_memory=true;c.stack_bounds=Some((0xFF000,0x100000));
        c.write(0xF000,0xFF00,16,0x8123).unwrap();
        assert_eq!(c.e.image[0xFFF00],0x23);
        assert_eq!(c.read(0xFFF0,0,16).unwrap(),0x8123);
    }
    #[test]
    fn relocated_calls_keep_real_cs_words_and_logical_procedure_names(){
        fn child(c:&mut Cpu)->Result<(),String>{c.r[0]=c.r[9];c.ret(true,0)}
        let mut c=cpu();c.code_base=0x1000;c.r[9]=0x1AB9;
        c.invoke(child,true,0x24BC,0x100,0x1234).unwrap();
        assert_eq!(c.r[0],0x34BC);assert_eq!(c.r[9],0x1AB9);
    }
    #[test]
    fn exchange_preserves_flags_and_unrelated_registers(){
        let mut c=cpu();c.r[0]=0x8000;c.r[2]=0xFFFF;c.r[1]=0x55;c.cf=true;c.z=false;c.sf=true;c.of=true;c.pf=false;
        c.exchange_registers(0,2);
        assert_eq!((c.r[0],c.r[2],c.r[1]),(0xFFFF,0x8000,0x55));
        assert_eq!((c.cf,c.z,c.sf,c.of,c.pf),(true,false,true,true,false));
    }
    #[test]
    fn rng_trace_uses_actual_seed_parameter_not_unrelated_global(){
        fn inner(c:&mut Cpu)->Result<(),String>{
            c.enter(0)?;c.push(0xF000)?;c.push(0x100)?;c.push(2)?;
            c.invoke(crate::watched_compiled::adjustment::p_24bc_0cdc,true,0x24BC,0x0CDC,0x3333)?;
            c.r[4]=c.r[5];c.r[5]=c.pop()?;c.ret(true,0)
        }
        fn outer(c:&mut Cpu)->Result<(),String>{
            c.enter(0)?;c.invoke(inner,true,0x0AB9,0x100,0x1111)?;
            c.r[4]=c.r[5];c.r[5]=c.pop()?;c.ret(true,0)
        }
        let mut c=cpu();c.write(0xF000,0x100,16,7).unwrap();
        c.invoke(outer,true,0x0AB9,0x200,0x2222).unwrap();
        assert_eq!(c.draws[0].seed_before,7);assert_eq!(c.draws[0].seed_after,30266);
        assert_eq!(c.read(0xF000,0x100,16).unwrap(),30266);assert_eq!(c.e.word(crate::SEED),0);
    }

}
