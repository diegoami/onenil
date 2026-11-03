//! [designed] Native arithmetic/ABI support for compiled Rust procedures.
use crate::{Draw,Engine};
pub(super) type Proc=fn(&mut Cpu)->Result<(),String>;
pub(super) struct Cpu{
 pub r:[u16;12],pub pc:(u16,u16),pub cf:bool,pub z:bool,pub sf:bool,pub of:bool,pub pf:bool,
 pub e:Engine,pub draws:Vec<Draw>,
}
impl Cpu{
 pub fn new(e:Engine)->Self{Self{r:[0;12],pc:(0,0),cf:false,z:false,sf:false,of:false,pf:false,e,draws:vec![]}}
 pub fn read(&self,_s:u16,_o:u16,_w:u8)->Result<u16,String>{Err("missing native read".into())}
 pub fn write(&mut self,_s:u16,_o:u16,_w:u8,_v:u16)->Result<(),String>{Err("native write not implemented".into())}
 pub fn step(&mut self)->Result<(),String>{Ok(())}
 pub fn push(&mut self,_v:u16)->Result<(),String>{Err("push not implemented".into())}
 pub fn pop(&mut self)->Result<u16,String>{Err("pop not implemented".into())}
 pub fn enter(&mut self,_n:u16)->Result<(),String>{Err("enter not implemented".into())}
 pub fn ret(&mut self,_far:bool,_n:u16)->Result<(),String>{Err("ret not implemented".into())}
 pub fn invoke(&mut self,_f:Proc,_far:bool,_cs:u16,_ip:u16,_ret:u16)->Result<(),String>{Err("invoke not implemented".into())}
 pub fn multiply(&mut self,_v:u16,_signed:bool){}
 pub fn divide(&mut self,_v:u16,_signed:bool)->Result<(),String>{Err("divide not implemented".into())}
 pub fn add(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn sub(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn adc(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn sbb(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn and(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn or(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn xor(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn shl(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn shr(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn sar(&mut self,_x:u16,_y:u16,_w:u8)->u16{0}
 pub fn inc(&mut self,_x:u16,_w:u8)->u16{0}
 pub fn dec(&mut self,_x:u16,_w:u8)->u16{0}
 pub fn neg(&mut self,_x:u16,_w:u8)->u16{0}
 pub fn not(&mut self,_x:u16,_w:u8)->u16{0}
}
#[cfg(test)]
mod tests{
 use super::*;
 fn cpu()->Cpu{Cpu::new(crate::role_scores::image(Default::default()))}
 #[test]
 fn add_and_sub_signed_unsigned_boundaries(){let mut c=cpu();assert_eq!(c.add(0x7FFF,1,16),0x8000);assert!(c.of&&c.sf&&!c.cf);assert_eq!(c.add(255,1,8),0);assert!(c.cf&&c.z&&!c.of);assert_eq!(c.sub(0,1,16),65535);assert!(c.cf&&c.sf&&!c.of);assert_eq!(c.sub(128,1,8),127);assert!(c.of&&!c.sf);}
 #[test]
 fn inc_dec_preserve_carry_and_logic_clear_it(){let mut c=cpu();c.cf=true;assert_eq!(c.inc(65535,16),0);assert!(c.cf&&c.z);assert_eq!(c.dec(0,8),255);assert!(c.cf&&c.sf);assert_eq!(c.or(128,1,8),129);assert!(!c.cf&&!c.of&&c.sf);}
 #[test]
 fn shifts_mask_counts_and_signed_extension(){let mut c=cpu();assert_eq!(c.sar(0x8001,1,16),0xC000);assert!(c.cf&&c.sf);assert_eq!(c.shr(0x8001,1,16),0x4000);assert!(c.cf&&!c.sf&&c.of);assert_eq!(c.shl(1,33,16),2);assert_eq!(c.shr(255,8,16),0);}
 #[test]
 fn missing_engine_and_scratch_reads_diagnose(){let mut c=cpu();assert!(c.read(0,1,8).is_err());assert!(c.read(0xF000,100,16).is_err());c.write(0xF000,100,16,42).unwrap();assert_eq!(c.read(0xF000,100,16).unwrap(),42);}
 #[test]
 fn division_signed_word_and_overflow(){let mut c=cpu();c.r[0]=65529;c.r[2]=65535;c.divide(3,true).unwrap();assert_eq!(c.r[0],65534);assert_eq!(c.r[2],65535);c.r[0]=1;c.r[2]=1;assert!(c.divide(1,false).is_err());assert!(c.divide(0,false).is_err());}
}
