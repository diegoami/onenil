//! [confirmed] F39,c7637bd;56EE/2625/helper listings at d9e20a4.
//! [designed] Initial inactive kickoff with disabled sound;active launches deferred.
use crate::{role_scores::{Attributes,Cell},terminal::Scene,Draw,Engine};
use serde::{Deserialize,Serialize};
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {pub id:u16,pub byte8:u8,pub flags11:u8}
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Criterion {pub kind:u8,pub criterion:u8,pub first:Cell,pub second:Cell,pub bonus_flag:u8}
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Score {pub player:Attributes,pub byte6:u8,pub byte8:u8,pub bonus:u8,pub criteria:Vec<Criterion>}
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
 pub index:u8,pub scenes:Vec<Scene>,pub players:Vec<Player>,pub score:Option<Score>,
 pub target:u16,pub phase:u8,pub remaining:u16,pub signs:[u8;2],pub sound:u8,pub gate_bounds:[u16;2],
 pub actor:u8,pub secondary:u8,pub active:u8,pub flag:u8,pub b6:u8,pub mode:u8,
 pub ball:u16,pub dx:u16,pub dy:u16,pub distance:u16,pub endpoint:u16,pub seed:u16,
}
#[derive(Debug,Serialize)]
pub struct Outcome {pub output:State,pub ready:u8,pub draws:Vec<Draw>}
fn machine(_s:&State)->Result<Engine,String>{Err("56EE not implemented".into())}
fn advance_image(_e:&mut Engine,_index:u8)->Result<(u8,Vec<Draw>),String>{Err("56EE not implemented".into())}
pub fn advance(_s:&State)->Result<Outcome,String>{Err("56EE not implemented".into())}
#[cfg(test)]
mod tests {
 use super::*;
 fn packed(x:i8,y:i8)->u16{u16::from_le_bytes([x as u8,y as u8])}
 fn sample()->State {
  let c:serde_json::Value=serde_json::from_str(include_str!("../tests/fixtures/watched_target/cases.jsonl").lines().next().unwrap()).unwrap();
  let template:Scene=serde_json::from_value(c["input"]["scene"].clone()).unwrap();
  let scenes=(1..=22).map(|index|{let mut r=template.clone();r.index=index;r.byte7=index;r.word14=u16::from(index);r.word0=0;r.word2=0;r.byte19=0;r.byte18=0;r.byte22=0;r}).collect();
  State{index:21,scenes,players:(1..=22).map(|id|Player{id,byte8:1,flags11:0}).collect(),score:Some(Score{player:Attributes{id:21,p8_4:255,p8_5:0,p8_7:0},byte6:31,byte8:1,bonus:0,criteria:[6,7].into_iter().map(|kind|Criterion{kind,criterion:21,first:Cell{index:21,value:0},second:Cell{index:0,value:255},bonus_flag:0}).collect()}),target:packed(4,10),phase:45,remaining:91,signs:[255,1],sound:0,gate_bounds:[100;2],actor:21,secondary:22,active:0,flag:0,b6:0,mode:99,ball:0,dx:123,dy:456,distance:789,endpoint:321,seed:0}
 }
 #[test]
 fn original_initial_kickoff_roots(){
  let source=std::fs::read_to_string(format!("{}/tests/fixtures/watched_kickoff/cases.jsonl",env!("CARGO_MANIFEST_DIR"))).unwrap();
  for line in source.lines(){let c:serde_json::Value=serde_json::from_str(line).unwrap();let s:State=serde_json::from_value(c["input"].clone()).unwrap();assert_eq!(serde_json::to_value(advance(&s).unwrap()).unwrap(),c["output"]);}
 }
 #[test]
 fn readiness_scans_and_short_circuits_actual_availability(){
  let mut s=sample();s.scenes[0].byte19=1;let o=advance(&s).unwrap();assert_eq!(o.ready,0);assert_eq!(o.output,s);assert!(o.draws.is_empty());
  s.players[0].flags11=2;assert_eq!(advance(&s).unwrap().ready,1);
  s.players[0].flags11=0;s.players[0].byte8=255;assert_eq!(advance(&s).unwrap().ready,1);
  s=sample();s.players.clear();s.score=None;s.scenes[0].byte19=1;assert!(advance(&s).unwrap_err().contains("availability"));
  s.scenes[0].byte19=0;s.scenes[20].byte19=2;s.scenes[21].byte19=2;
  // Root/secondary are exempt;force mode8 to skip reached score facts.
  s.index=1;s.actor=21;s.secondary=22;s.target=packed(0,-25);s.ball=packed(0,-30);s.signs=[1;2];
  let o=advance(&s).unwrap();assert_eq!(o.ready,1);assert_eq!(o.output.mode,8);assert!(o.draws.is_empty());
 }
 #[test]
 fn native_launch_state_and_all_twenty_two_action_stores(){
  let s=sample();let o=advance(&s).unwrap();assert_eq!(o.ready,1);assert_eq!((o.output.dx,o.output.dy,o.output.distance,o.output.endpoint),(0,1,10,packed(0,10)));
  assert_eq!((o.output.active,o.output.flag,o.output.actor,o.output.secondary,o.output.mode,o.output.b6),(1,1,21,0,6,0));
  assert!(o.output.scenes.iter().all(|r|r.byte19==5));assert_eq!(o.output.scenes[20].byte16,5);assert_eq!(o.draws.len(),1);assert_eq!(o.draws[0].site,"0AB9:1BF1");
 }
 #[test]
 fn orientation_signed_byte_wrap_axis_ties_and_half_distance(){
  for (target,dx,dy,distance) in [(packed(10,4),1,0,10),(packed(10,5),1,1,7),(packed(4,10),0,1,10),(packed(5,10),1,1,7),(packed(2,2),1,2,2)]{
   let mut s=sample();s.target=target;let o=advance(&s).unwrap();assert_eq!((o.output.dx,o.output.dy,o.output.distance),(dx,dy,distance));
  }
  let mut s=sample();s.ball=packed(127,127);s.target=packed(-128,-127);let o=advance(&s).unwrap();assert_eq!((o.output.dx,o.output.dy),(1,1));assert_eq!(o.output.endpoint,packed(-127,-127));
  s=sample();s.phase=255;s.remaining=0;s.signs=[1,255];s.target=packed(4,10);assert_eq!(advance(&s).unwrap().output.dy,65535);
 }
 #[test]
 fn mode_upgrades_score_success_and_failure_and_original_rng_sites(){
  let mut s=sample();s.target=packed(0,30);let o=advance(&s).unwrap();assert_eq!(o.output.mode,7);assert_eq!(o.draws.len(),1);
  s=sample();s.index=1;s.ball=packed(0,-30);s.target=packed(0,-25);s.signs=[1;2];s.score=None;let o=advance(&s).unwrap();assert_eq!(o.output.mode,8);assert!(o.draws.is_empty());
  s=sample();s.score.as_mut().unwrap().criteria.iter_mut().for_each(|c|c.second.value=0);let o=advance(&s).unwrap();assert!(o.draws.len()>1);assert_eq!(o.draws[1].site,"0AB9:2852");
  // A changed raw role table changes native skill and therefore the actual RNG path.
  assert_ne!(o.draws.len(),advance(&sample()).unwrap().draws.len());
 }
 #[test]
 fn invalid_alias_missing_reached_facts_and_explicit_domains(){
  for mode in 0..8 {let mut s=sample();match mode{0=>s.active=1,1=>s.sound=1,2=>s.actor=23,3=>s.scenes[0].index=2,4=>s.score.as_mut().unwrap().byte8=2,5=>s.score.as_mut().unwrap().criteria[0].first.index=0,6=>s.score=None,_=>s.score.as_mut().unwrap().criteria.clear()};assert!(advance(&s).is_err());}
  let mut s=sample();s.signs=[0;2];assert!(advance(&s).unwrap_err().contains("divisor"));
  let v=serde_json::to_value(sample()).unwrap();for key in ["expected","ready","vector","helper_results","draws"]{let mut v=v.clone();v[key]=serde_json::json!(1);assert!(serde_json::from_value::<State>(v).is_err());}
 }
 #[test]
 fn exact_whole_image_ownership(){
  let mut cases=vec![sample()];let mut pending=sample();pending.scenes[0].byte19=1;cases.push(pending);
  for s in cases{let mut e=machine(&s).unwrap();e.image[0x68000]=42;let before=e.image.clone();let (ready,_)=advance_image(&mut e,s.index).unwrap();
   let mut owned=vec![];if ready!=0{owned.extend([0xD09F,0xD0A0,0xD0B8,0xCF53,0xD0B6,0xD0BB,0xD0A6,0xD0A7,0xCF4D,0xCF4E,0xCF4F,0xCF50,0xCF51,0xCF52,usize::from(crate::SEED),usize::from(crate::SEED)+1,0xCCEF+25*usize::from(s.index)+16]);for i in 1..=22{owned.push(0xCCEF+25*i+19)}}
   for(i,(old,new))in before.iter().zip(&e.image).enumerate(){if !owned.contains(&i){assert_eq!(old,new,"unexpected write {i:05X}");}}
  }
 }
}
