//! [confirmed] F39/c7637bd;compiled original35F8 numeric helper graph.
//! [designed] Typed root facts and temporary Pascal frames;no runtime decoding.
mod adjustment;
mod compiled;
use crate::{terminal::Scene,Draw,Engine};
use serde::{Deserialize,Serialize};
#[derive(Clone,Copy,Debug,Deserialize,Serialize,PartialEq,Eq,PartialOrd,Ord)]
#[serde(rename_all="lowercase")]
pub enum Region{Ds,P8,P15,Club}
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Cell{pub region:Region,pub offset:u16,pub value:u8}
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct State{pub scene:Scene,pub seed:u16,pub cells:Vec<Cell>}
#[derive(Debug,Serialize)]
pub struct Outcome{pub output:State,pub draws:Vec<Draw>}
fn machine(_s:&State)->Result<compiled::Cpu,String>{Err("compiled35F8 not implemented".into())}
pub fn advance(_s:&State)->Result<Outcome,String>{Err("compiled35F8 not implemented".into())}
#[cfg(test)]
mod tests{
 use super::*;
 fn sample()->State{
  let c:serde_json::Value=serde_json::from_str(include_str!("../../tests/fixtures/watched_target/cases.jsonl").lines().next().unwrap()).unwrap();
  State{scene:serde_json::from_value(c["input"]["scene"].clone()).unwrap(),seed:123,cells:vec![]}
 }
 #[test]
 fn original_adjustment_roots(){
  let source=std::fs::read_to_string(format!("{}/tests/fixtures/watched_adjustment/cases.jsonl",env!("CARGO_MANIFEST_DIR"))).unwrap();
  for line in source.lines(){let c:serde_json::Value=serde_json::from_str(line).unwrap();let s:State=serde_json::from_value(c["input"].clone()).unwrap();assert_eq!(serde_json::to_value(advance(&s).unwrap()).unwrap(),c["output"]);}
 }
 #[test]
 fn strict_missing_alias_and_expected_inputs(){
  assert!(advance(&sample()).unwrap_err().contains("missing"));
  let v=serde_json::to_value(sample()).unwrap();for field in ["expected","helper_results","target","draws"]{let mut v=v.clone();v[field]=serde_json::json!(1);assert!(serde_json::from_value::<State>(v).is_err());}
 }
}
