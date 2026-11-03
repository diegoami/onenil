//! [confirmed] F39/c7637bd;original448C/4DFF source-addressed numeric kernels.
//! [designed] Strict named entry facts;private original RAM is never a runtime input.
use super::compiled::{Cpu, Missing};
use crate::{terminal::Scene, Draw};
use serde::{Deserialize,Serialize};
#[derive(Clone,Copy,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Kind{Action5,Contact}
#[derive(Clone,Copy,Debug,Deserialize,Serialize,PartialEq,Eq,PartialOrd,Ord)]
#[serde(rename_all="snake_case")]
pub enum PointerKind{P8,P15,Club,Config,Main,MainShown,Background,BackgroundShown,Commentary,Seed}
impl PointerKind{
 fn slot(self)->u16{match self{Self::P8=>0xD7BA,Self::P15=>0xD7BE,Self::Club=>0xD7C2,Self::Config=>0xD7D2,Self::Main=>0xD08A,Self::MainShown=>0xD08E,Self::Background=>0xD092,Self::BackgroundShown=>0xD096,Self::Commentary=>0xD09A,Self::Seed=>0xDE84}}
 fn length(self)->usize{match self{Self::P8=>8*3921,Self::P15=>15*3921,Self::Club=>62*175,Self::Config=>256,Self::Seed=>2,_=>4000}}
}
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Pointer{pub kind:PointerKind,pub offset:u16,pub segment:u16}
impl Pointer{fn address(&self)->usize{16*usize::from(self.segment)+usize::from(self.offset)}}
#[derive(Clone,Copy,Debug,Deserialize,Serialize,PartialEq,Eq,PartialOrd,Ord)]
#[serde(rename_all="snake_case")]
pub enum Area{Ds,P8,P15,Club,Config,Main,MainShown,Background,BackgroundShown,Frame}
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct ByteFact{pub area:Area,pub offset:i32,pub value:u8}
#[derive(Clone,Debug,Deserialize,Serialize)]
#[serde(deny_unknown_fields)]
pub struct State{
 pub kind:Kind,pub scene:Scene,pub seed:u16,pub regs:[u16;12],pub flags:u32,
 pub stack_words:[u16;4],pub pointers:Vec<Pointer>,pub cells:Vec<ByteFact>,
}
#[derive(Debug,Serialize)]
pub struct Fault{pub message:String,pub missing:Option<Missing>}
#[derive(Debug,Serialize)]
pub struct Outcome{pub scene:Scene,pub seed:u16,pub pointers:Vec<Pointer>,pub cells:Vec<ByteFact>,pub draws:Vec<Draw>,pub events:Vec<serde_json::Value>}
fn machine(_s:&State)->Result<Cpu,String>{Err("T37 entry data builder not implemented".into())}
pub fn advance(_s:&State)->Result<Outcome,Fault>{Err(Fault{message:"T37 action kernels not implemented".into(),missing:None})}
#[cfg(test)]
mod tests{
 use super::*;
 fn context()->State{serde_json::from_str(include_str!("../../tests/fixtures/watched_action/context.json")).unwrap()}
 #[test]
 fn original_entry_context_initializes_one_data_image_and_real_abi(){
  let s=context();let c=machine(&s).unwrap();let ds=usize::from(s.regs[11])*16;
  assert!(c.flat_memory);assert_eq!(c.r,s.regs);assert_eq!(c.e.image.len(),0x110000);
  assert_eq!(u16::from_le_bytes([c.e.image[ds+0xA51C],c.e.image[ds+0xA51D]]),s.seed);
  let a=ds+0xCCEF+25*usize::from(s.scene.index);assert_eq!(&c.e.image[a..a+25],&s.scene.bytes());
  let stack=usize::from(s.regs[10])*16+usize::from(s.regs[4]);
  for(i,w)in s.stack_words.iter().enumerate(){assert_eq!(u16::from_le_bytes([c.e.image[stack+2*i],c.e.image[stack+2*i+1]]),*w);}
 }
 #[test]
 fn entry_schema_and_reserved_aliases_are_strict(){
  let s=context();let mut v=serde_json::to_value(&s).unwrap();v["expected_helpers"]=serde_json::json!([]);assert!(serde_json::from_value::<State>(v).is_err());
  let mut s=s.clone();s.cells.push(ByteFact{area:Area::Ds,offset:0xA51C,value:0});assert!(machine(&s).is_err());
  let mut s=context();s.cells.push(ByteFact{area:Area::Ds,offset:0xD7BA,value:0});assert!(machine(&s).is_err());
  let mut s=context();s.pointers.push(s.pointers[0].clone());assert!(machine(&s).is_err());
  let mut s=context();let p=s.pointers.iter_mut().find(|p|p.kind==PointerKind::P8).unwrap();p.segment=s.regs[11];p.offset=0xA51C;assert!(machine(&s).is_err());
 }
}
