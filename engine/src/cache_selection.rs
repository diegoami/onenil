//! [confirmed] Archaeology F32/13B8/4EBF at218490d; read-only cached-player helpers.
use crate::{placement, Draw, Engine, Input as ContestInput};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RatingInput {
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_5: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_5: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SelectorInput {
    pub side: u8,
    pub club: u16,
    pub roster: [u16; 11],
    pub players: Vec<Player>,
}
#[derive(Debug, Serialize)]
pub struct RatingOutput {
    pub value: u16,
}
#[derive(Debug, Serialize)]
pub struct SelectorOutput {
    pub player: u16,
}
#[derive(Debug, Serialize)]
pub struct Visit {
    pub slot: u8,
    pub player: u16,
    pub unavailable: u8,
    pub rating: Option<u16>,
    pub selected: bool,
    pub retained_score: u16,
    pub rating_calls: u8,
}
#[derive(Debug, Serialize)]
pub struct RatingOutcome {
    pub output: RatingOutput,
    pub draws: Vec<Draw>,
}
#[derive(Debug, Serialize)]
pub struct SelectorOutcome {
    pub output: SelectorOutput,
    pub visits: Vec<Visit>,
    pub draws: Vec<Draw>,
}
const P8: usize = 0x10000;
const P15: usize = 0x20000;
const CLUBS: usize = 0x30000;
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, value: u16) {
    e.image[a..a + 2].copy_from_slice(&value.to_le_bytes())
}
fn blank() -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed: 0,
        ratings: [0; 8],
        goals: [0; 2],
    })
}
fn load_rating(e: &mut Engine, id: u16, input: &RatingInput) {
    let a = P8 + 8 * usize::from(id);
    e.image[a + 1] = input.p8_1;
    e.image[a + 3] = input.p8_3;
    e.image[a + 5] = input.p8_5
}
pub(crate) fn rating_image(e: &Engine, id: u16) -> u16 {
    // [confirmed] F32/4EBF: IDIV's signed remainder is exchanged into AX,not quotient.
    let a = P8 + 8 * usize::from(id);
    let difference = i16::from(e.image[a + 3]).wrapping_sub(i16::from(e.image[a + 5]));
    let remainder = (difference % 8) as u16;
    if e.image[a + 1] < 21 {
        remainder >> 1
    } else {
        remainder
    }
}
fn validate(input: &SelectorInput) -> Result<(), String> {
    if input.side > 1
        || !(1..=176).contains(&input.club)
        || input.roster.iter().any(|&id| id > 3920)
    {
        return Err("unsupported cached-selector side/club/player domain".into());
    }
    let mut players = BTreeMap::new();
    for p in &input.players {
        if p.id > 3920 || players.insert(p.id, p).is_some() {
            return Err("ambiguous cached-selector player input".into());
        }
    }
    let roster = input.roster.iter().copied().collect::<BTreeSet<_>>();
    if roster != players.keys().copied().collect() {
        return Err("missing or unreferenced cached-selector player input".into());
    }
    Ok(())
}
fn selector_image(input: &SelectorInput) -> Result<Engine, String> {
    validate(input)?;
    let mut engine = blank();
    put(
        &mut engine,
        0xD6E4 + 2 * usize::from(input.side),
        input.club,
    );
    for (slot, &id) in input.roster.iter().enumerate() {
        put(
            &mut engine,
            CLUBS + 62 * usize::from(input.club - 1) + 2 * slot,
            id,
        )
    }
    for p in &input.players {
        load_rating(
            &mut engine,
            p.id,
            &RatingInput {
                p8_1: p.p8_1,
                p8_3: p.p8_3,
                p8_5: p.p8_5,
            },
        );
        let a = P15 + 15 * usize::from(p.id);
        engine.image[a + 8] = p.byte8;
        engine.image[a + 11] = p.flags11;
    }
    Ok(engine)
}
pub(crate) fn select_image(engine: &Engine, side: u8) -> Result<SelectorOutcome, String> {
    let club = word(engine, 0xD6E4 + 2 * usize::from(side));
    let mut current = 0xffffu16;
    let mut chosen = None;
    let mut visits = Vec::new();
    for slot in 0..11 {
        let id = word(engine, CLUBS + 62 * usize::from(club - 1) + 2 * slot);
        let p = P15 + 15 * usize::from(id);
        let unavailable = placement::unavailable(&placement::Player {
            id,
            p15_sbyte8: engine.image[p + 8] as i8,
            p15_byte11: engine.image[p + 11],
        });
        let mut rating = None;
        let mut selected = false;
        let mut rating_calls = 0;
        if unavailable == 0 {
            let candidate = rating_image(engine, id);
            rating = Some(candidate);
            rating_calls = 1;
            // [confirmed] F32/13B8: signed current long versus zero-extended candidate long.
            if i32::from(current as i16) < i32::from(candidate) {
                chosen = Some(id);
                current = rating_image(engine, id);
                selected = true;
                rating_calls = 2;
            }
        }
        visits.push(Visit {
            slot: slot as u8 + 1,
            player: id,
            unavailable,
            rating,
            selected,
            retained_score: current,
            rating_calls,
        });
    }
    let player = chosen
        .ok_or("no eligible cached-player candidate: original13B8 selected local is undefined")?;
    Ok(SelectorOutcome {
        output: SelectorOutput { player },
        visits,
        draws: Vec::new(),
    })
}
pub fn rating(input: &RatingInput) -> Result<RatingOutcome, String> {
    let mut engine = blank();
    load_rating(&mut engine, 0, input);
    Ok(RatingOutcome {
        output: RatingOutput {
            value: rating_image(&engine, 0),
        },
        draws: Vec::new(),
    })
}
pub fn select(input: &SelectorInput) -> Result<SelectorOutcome, String> {
    let engine = selector_image(input)?;
    select_image(&engine, input.side)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_only_helpers_preserve_entire_image_and_seed() {
        let c: serde_json::Value = serde_json::from_str(
            include_str!("../tests/fixtures/cache_selection/selectors.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let input: SelectorInput = serde_json::from_value(c["input"].clone()).unwrap();
        let mut engine = selector_image(&input).unwrap();
        engine.image[0x8000] = 0xab;
        put(&mut engine, 0xA51C, 0xbeef);
        let before = engine.image.clone();
        select_image(&engine, input.side).unwrap();
        for p in input.players {
            rating_image(&engine, p.id);
        }
        assert_eq!(engine.image, before);
    }
}
