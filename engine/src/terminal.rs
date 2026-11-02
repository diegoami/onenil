//! [confirmed] Archaeology F34/F35 at218490d; bounded terminal helpers only.
use crate::{placement, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub index: u8,
    pub word0: u16,
    pub word2: u16,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub byte7: u8,
    pub word8: u16,
    pub word10: u16,
    pub word12: u16,
    pub word14: u16,
    pub byte16: u8,
    pub byte17: u8,
    pub byte18: u8,
    pub byte19: u8,
    pub word20: u16,
    pub byte22: u8,
    pub word23: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub position: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub position: u8,
    pub word: u16,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Selectors {
    pub near: [u8; 2],
    pub ahead: [u8; 2],
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub actor: u8,
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub positions: Vec<Position>,
    pub signs: [u8; 2],
    pub target: [u8; 2],
    pub phase: u8,
    pub time: u16,
    pub selectors: Selectors,
}
#[derive(Debug, Serialize)]
pub struct PositionsOutcome {
    pub output: State,
    pub visits: Vec<Value>,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct ProximityOutcome {
    pub output: State,
    pub visits: Vec<Value>,
    pub initializations: Vec<Value>,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct TerminalOutcome {
    pub positions: PositionsOutcome,
    pub proximity: ProximityOutcome,
}
const P8: usize = crate::layout::P8;
const P15: usize = crate::layout::P15;
fn address(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
impl Scene {
    pub(crate) fn read(e: &Engine, index: u8) -> Self {
        let a = address(index);
        Self {
            index,
            word0: word(e, a),
            word2: word(e, a + 2),
            byte4: e.image[a + 4],
            byte5: e.image[a + 5],
            byte6: e.image[a + 6],
            byte7: e.image[a + 7],
            word8: word(e, a + 8),
            word10: word(e, a + 10),
            word12: word(e, a + 12),
            word14: word(e, a + 14),
            byte16: e.image[a + 16],
            byte17: e.image[a + 17],
            byte18: e.image[a + 18],
            byte19: e.image[a + 19],
            word20: word(e, a + 20),
            byte22: e.image[a + 22],
            word23: word(e, a + 23),
        }
    }
    pub(crate) fn bytes(&self) -> [u8; 25] {
        let mut bytes = [0; 25];
        for (offset, v) in [
            (0, self.word0),
            (2, self.word2),
            (8, self.word8),
            (10, self.word10),
            (12, self.word12),
            (14, self.word14),
            (20, self.word20),
            (23, self.word23),
        ] {
            bytes[offset..offset + 2].copy_from_slice(&v.to_le_bytes())
        }
        for (offset, v) in [
            (4, self.byte4),
            (5, self.byte5),
            (6, self.byte6),
            (7, self.byte7),
            (16, self.byte16),
            (17, self.byte17),
            (18, self.byte18),
            (19, self.byte19),
            (22, self.byte22),
        ] {
            bytes[offset] = v
        }
        bytes
    }
    fn load(&self, e: &mut Engine) {
        let a = address(self.index);
        e.image[a..a + 25].copy_from_slice(&self.bytes());
    }
}
pub(crate) fn validate(s: &State) -> Result<(), String> {
    if !matches!(s.side_offsets, [0, 11] | [11, 0])
        || !(1..=22).contains(&s.actor)
        || s.scenes.len() != 22
        || s.scenes.iter().enumerate().any(|(i, r)| {
            usize::from(r.index) != i + 1 || r.word14 > 3920 || !(1..=22).contains(&r.byte7)
        })
    {
        return Err("unsupported terminal actor/physical scene/offset domain".into());
    }
    let mut players = BTreeMap::new();
    for p in &s.players {
        if p.id > 3920 || players.insert(p.id, p).is_some() {
            return Err("ambiguous terminal player input".into());
        }
    }
    if s.scenes.iter().any(|r| !players.contains_key(&r.word14)) {
        return Err("missing terminal scene player".into());
    }
    let mut cells = BTreeSet::new();
    for p in &s.positions {
        if !cells.insert(p.position) {
            return Err("ambiguous position cell".into());
        }
    }
    if cells != s.players.iter().map(|p| p.position).collect() {
        return Err("missing or unreferenced position cell".into());
    }
    Ok(())
}
struct Machine<E: std::ops::DerefMut<Target = Engine>> {
    engine: E,
    header: State,
}
impl Machine<Box<Engine>> {
    fn new(s: &State) -> Result<Self, String> {
        validate(s)?;
        let mut e = Engine::new(&ContestInput {
            k: 0,
            seed: s.seed,
            ratings: [0; 8],
            goals: [0; 2],
        });
        for r in &s.scenes {
            r.load(&mut e)
        }
        for p in &s.players {
            e.image[P8 + 8 * usize::from(p.id) + 6] = p.position;
            e.image[P15 + 15 * usize::from(p.id) + 8] = p.byte8;
            e.image[P15 + 15 * usize::from(p.id) + 11] = p.flags11
        }
        for p in &s.positions {
            put(&mut e, 0xA3A + 2 * usize::from(p.position), p.word)
        }
        for side in 0..2 {
            e.image[0xD56 + side] = s.side_offsets[side];
            e.image[0xA338 + side] = s.signs[side];
            e.image[0xCF46 + side] = s.target[side];
            e.image[0xD0A4 + side] = s.selectors.near[side];
            e.image[0xD0A2 + side] = s.selectors.ahead[side]
        }
        e.image[0xD09F] = s.actor;
        e.image[0xD0AC] = s.phase;
        put(&mut e, 0xD0AE, s.time);
        Ok(Self {
            engine: Box::new(e),
            header: s.clone(),
        })
    }
}
impl<E: std::ops::DerefMut<Target = Engine>> Machine<E> {
    fn selectors(&self) -> Selectors {
        let e = &self.engine;
        Selectors {
            near: [e.image[0xD0A4], e.image[0xD0A5]],
            ahead: [e.image[0xD0A2], e.image[0xD0A3]],
        }
    }
    fn state(&self) -> State {
        let mut s = self.header.clone();
        s.seed = word(&self.engine, 0xA51C);
        s.scenes = (1..=22).map(|i| Scene::read(&self.engine, i)).collect();
        s.selectors = self.selectors();
        s.actor = self.engine.image[0xD09F];
        s.phase = self.engine.image[0xD0AC];
        s.time = word(&self.engine, 0xD0AE);
        for side in 0..2 {
            s.side_offsets[side] = self.engine.image[0xD56 + side];
            s.signs[side] = self.engine.image[0xA338 + side];
            s.target[side] = self.engine.image[0xCF46 + side];
        }
        for p in &mut s.players {
            let id = usize::from(p.id);
            p.position = self.engine.image[P8 + 8 * id + 6];
            p.byte8 = self.engine.image[P15 + 15 * id + 8];
            p.flags11 = self.engine.image[P15 + 15 * id + 11];
        }
        for p in &mut s.positions {
            p.word = word(&self.engine, 0xA3A + 2 * usize::from(p.position));
        }
        s
    }
    fn positions(&mut self) -> PositionsOutcome {
        // [confirmed] F34: each side's first visit skips every use of the initial dead local.
        let mut visits = Vec::new();
        for side in 0..2 {
            let offset = self.engine.image[0xD56 + side];
            let mut previous = 0;
            for slot in 1..=11 {
                let index = offset + slot;
                let input = Scene::read(&self.engine, index);
                let previous_input = if previous == 0 {
                    None
                } else {
                    Some(Scene::read(&self.engine, previous))
                };
                let position = self.engine.image[P8 + 8 * usize::from(input.word14) + 6];
                let value = word(&self.engine, 0xA3A + 2 * usize::from(position));
                put(&mut self.engine, address(index) + 10, value);
                let base = Scene::read(&self.engine, index);
                let mut branch = None;
                if previous != 0 && value == word(&self.engine, address(previous) + 10) {
                    let a = address(index) + 10;
                    let p = address(previous) + 10;
                    let low = self.engine.image[a] as i8;
                    if i16::from(low).abs() > 4 {
                        branch = Some("0AB9:3223");
                        let sign = self.engine.image[0xA338 + usize::from(low > 0)] as i8;
                        self.engine.image[a] =
                            self.engine.image[a].wrapping_sub(sign.wrapping_mul(3) as u8);
                        self.engine.image[p + 1] = self.engine.image[p + 1].wrapping_sub(3);
                    } else {
                        branch = Some("0AB9:3235");
                        self.engine.image[a] = self.engine.image[a].wrapping_add(3);
                        self.engine.image[p] = self.engine.image[p].wrapping_sub(3);
                    }
                }
                let output = Scene::read(&self.engine, index);
                let previous_output = if previous == 0 {
                    None
                } else {
                    Some(Scene::read(&self.engine, previous))
                };
                visits.push(json!({"side":side,"slot":slot,"scene":index,"previous":previous,"input":input,"previous_input":previous_input,"draw_start":0,"branch":branch,"player":input.word14,"table":{"position":position,"word":value},"base":base,"output":output,"previous_output":previous_output,"draw_end":0}));
                previous = index;
            }
        }
        PositionsOutcome {
            output: self.state(),
            visits,
            draws: Vec::new(),
        }
    }
    fn unavailable(&self, row: &Scene) -> u8 {
        let id = usize::from(row.word14);
        placement::unavailable(&placement::Player {
            id: row.word14,
            p15_sbyte8: self.engine.image[P15 + 15 * id + 8] as i8,
            p15_byte11: self.engine.image[P15 + 15 * id + 11],
        })
    }
    fn direction(&self, row: &Scene) -> (u8, u8) {
        direction_image(&self.engine, row)
    }
    fn proximity(&mut self) -> ProximityOutcome {
        let mut visits = Vec::new();
        let mut initializations = Vec::new();
        for side in 0..2 {
            let offset = self.engine.image[0xD56 + side];
            self.engine.image[0xD0A4 + side] = offset + 1;
            self.engine.image[0xD0A2 + side] = 0;
            initializations.push(json!({"side":side,"output":self.selectors(),"draw":0}));
            let mut nearest = 100u16;
            let mut ahead = 100u16;
            for slot in 2..=11 {
                let index = offset + slot;
                let row = Scene::read(&self.engine, index);
                let before = self.selectors();
                let available = self.unavailable(&row);
                let child = observation(
                    "0AB9:1CAC",
                    "player-available",
                    row.word14,
                    &row,
                    available,
                    vec![],
                );
                let mut helpers = vec![observation(
                    "0AB9:223D",
                    "available",
                    u16::from(index),
                    &row,
                    available,
                    vec![child],
                )];
                let mut stores = Vec::new();
                if available == 0 && row.byte7 != self.engine.image[0xD09F] && row.byte19 != 13 {
                    let x = i16::from(row.word0 as u8 as i8);
                    let y = i16::from((row.word0 >> 8) as u8 as i8);
                    let tx = i16::from(self.engine.image[0xCF46] as i8);
                    let ty = i16::from(self.engine.image[0xCF47] as i8);
                    let distance = (x - tx)
                        .unsigned_abs()
                        .wrapping_add((y - ty).unsigned_abs());
                    if distance < nearest {
                        self.engine.image[0xD0A4 + side] = row.byte7;
                        nearest = distance;
                        stores.push(json!({"site":"0AB9:22A0","kind":"near","output":self.selectors(),"draw":0}))
                    }
                    let (target_direction, target_side) = self.direction(&row);
                    helpers.push(observation(
                        "0AB9:22AC",
                        "target-direction",
                        u16::from(index),
                        &row,
                        target_direction,
                        vec![observation(
                            "0AB9:1EF5",
                            "side",
                            u16::from(index),
                            &row,
                            target_side,
                            vec![],
                        )],
                    ));
                    let (candidate_direction, candidate_side) = self.direction(&row);
                    helpers.push(observation(
                        "0AB9:22BF",
                        "candidate-direction",
                        u16::from(index),
                        &row,
                        candidate_direction,
                        vec![observation(
                            "0AB9:1EF5",
                            "side",
                            u16::from(index),
                            &row,
                            candidate_side,
                            vec![],
                        )],
                    ));
                    let target_product = i16::from(target_direction as i8).wrapping_mul(ty);
                    let candidate_product = i16::from(candidate_direction as i8).wrapping_mul(y);
                    if candidate_product < target_product && distance < ahead {
                        self.engine.image[0xD0A2 + side] = row.byte7;
                        ahead = distance;
                        stores.push(json!({"site":"0AB9:22EA","kind":"ahead","output":self.selectors(),"draw":0}))
                    }
                }
                visits.push(json!({"side":side,"slot":slot,"scene":index,"input":row,"before":before,"draw_start":0,"helpers":helpers,"stores":stores,"output":row,"after":self.selectors(),"draw_end":0}));
            }
        }
        ProximityOutcome {
            output: self.state(),
            visits,
            initializations,
            draws: Vec::new(),
        }
    }
}
pub(crate) fn direction_image(e: &Engine, row: &Scene) -> (u8, u8) {
    // [confirmed] F35/1ECC: unsigned time versus sign-extended phase, then XOR side.
    let flip = i32::from(word(e, 0xD0AE)) > i32::from(e.image[0xD0AC] as i8);
    let side = row.byte7 < 12;
    (e.image[0xA338 + usize::from(flip ^ side)], u8::from(side))
}
fn observation(
    site: &str,
    kind: &str,
    argument: u16,
    row: &Scene,
    result: u8,
    children: Vec<Value>,
) -> Value {
    json!({"site":site,"kind":kind,"arguments":[argument],"input":row,"draw_start":0,"children":children,"output":row,"result":result,"draw_end":0})
}
pub(crate) fn positions_image(e: &mut Engine, s: &State) -> PositionsOutcome {
    Machine {
        engine: e,
        header: s.clone(),
    }
    .positions()
}
pub(crate) fn proximity_image(e: &mut Engine, s: &State) -> ProximityOutcome {
    Machine {
        engine: e,
        header: s.clone(),
    }
    .proximity()
}
pub(crate) fn snapshot(e: &mut Engine, s: &State) -> State {
    Machine {
        engine: e,
        header: s.clone(),
    }
    .state()
}
pub fn positions(input: &State) -> Result<PositionsOutcome, String> {
    Ok(Machine::new(input)?.positions())
}
pub fn proximity(input: &State) -> Result<ProximityOutcome, String> {
    Ok(Machine::new(input)?.proximity())
}
pub fn terminal(input: &State) -> Result<TerminalOutcome, String> {
    let mut machine = Machine::new(input)?;
    let positions = machine.positions();
    let proximity = machine.proximity();
    Ok(TerminalOutcome {
        positions,
        proximity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn image_preserves_unrelated_bytes_and_seed() {
        let c: Value = serde_json::from_str(
            include_str!("../tests/fixtures/terminal/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let input: State = serde_json::from_value(c["input"].clone()).unwrap();
        let mut m = Machine::new(&input).unwrap();
        m.engine.image[0x8000] = 0xab;
        let before = m.engine.image.clone();
        m.positions();
        m.proximity();
        let allowed = (1..=22)
            .flat_map(|i| [address(i) + 10, address(i) + 11])
            .chain(0xD0A2..=0xD0A5)
            .collect::<BTreeSet<_>>();
        for (a, (old, new)) in before.iter().zip(m.engine.image.iter()).enumerate() {
            if !allowed.contains(&a) {
                assert_eq!(old, new, "unowned byte {a:X}");
            }
        }

        assert_eq!(m.engine.image[0x8000], 0xab);
        assert_eq!(word(&m.engine, 0xA51C), input.seed);
    }
}
