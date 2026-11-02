//! [confirmed] F33/1455 and8973 at archaeology218490d; bounded role selection.
use crate::{role_scores::TableInput, terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub club: u8,
    pub byte6: u8,
    pub byte8: u8,
    pub flags11: u8,
    pub table: TableInput,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Roster {
    pub club: u16,
    pub roster: [u16; 11],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub kind: u8,
    pub criterion: u16,
    pub side: u8,
    pub seed: u16,
    pub side_offsets: [u8; 2],
    pub clubs: [u16; 2],
    pub starts: [u8; 6],
    pub cache: [[u16; 5]; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub rosters: Vec<Roster>,
    pub bonus: u8,
    pub criterion_cell: u8,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub clear: Value,
    pub visits: Vec<Value>,
    pub draws: Vec<Draw>,
}

use crate::{placement, role_scores, Engine};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
const P8: usize = 0x10000;
const P15: usize = 0x20000;
const CLUB: usize = 0x30000;
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn cache_address(side: u8, kind: u8) -> usize {
    0xCF5C + 10 * usize::from(side) + 2 * usize::from(kind)
}
fn add_word(c: &mut role_scores::Cells, a: usize, v: u16) -> Result<(), String> {
    for (i, b) in v.to_le_bytes().into_iter().enumerate() {
        role_scores::insert(c, a + i, b)?;
    }
    Ok(())
}
struct Machine {
    e: Engine,
    s: State,
}
impl Machine {
    fn new(s: &State) -> Result<Self, String> {
        if s.side > 1
            || !(1..=5).contains(&s.kind)
            || s.criterion > 255
            || !matches!(s.side_offsets, [0, 11] | [11, 0])
            || s.clubs.iter().any(|c| !(1..=176).contains(c))
            || s.clubs[0] == s.clubs[1]
            || s.scenes.len() != 22
        {
            return Err("unsupported role side/kind/criterion/offset/club/scene domain".into());
        }
        let mut c = role_scores::Cells::new();
        let mut players = BTreeMap::new();
        let mut needed = BTreeSet::new();
        for p in &s.players {
            if p.id > 3920
                || p.table.player.id != p.id
                || p.table.criterion != s.criterion
                || (p.id != 0 && !(1..=176).contains(&u16::from(p.club)))
                || players.insert(p.id, p).is_some()
            {
                return Err("ambiguous or inconsistent canonical role player".into());
            }
            for (a, v) in role_scores::table_cells(&p.table)? {
                role_scores::insert(&mut c, a, v)?;
            }
            let a = P15 + 15 * usize::from(p.id);
            for (a, v) in [
                (P8 + 8 * usize::from(p.id), p.club),
                (a + 6, p.byte6),
                (a + 8, p.byte8),
                (a + 11, p.flags11),
            ] {
                role_scores::insert(&mut c, a, v)?;
            }
        }
        for (i, row) in s.scenes.iter().enumerate() {
            if usize::from(row.index) != i + 1 {
                return Err("missing or unordered physical scene".into());
            }
            needed.insert(row.word14);
            for (offset, v) in row.bytes().into_iter().enumerate() {
                role_scores::insert(&mut c, 0xCCEF + 25 * (i + 1) + offset, v)?;
            }
        }
        let mut clubs = BTreeSet::new();
        for r in &s.rosters {
            if !(1..=176).contains(&r.club) || !clubs.insert(r.club) {
                return Err("ambiguous role roster".into());
            }
            for (slot, id) in r.roster.iter().enumerate() {
                if *id > 3920 {
                    return Err("unsupported roster player".into());
                }
                needed.insert(*id);
                add_word(&mut c, CLUB + 62 * usize::from(r.club - 1) + 2 * slot, *id)?;
            }
        }
        let required: BTreeSet<_> = s
            .players
            .iter()
            .filter(|p| p.id != 0)
            .map(|p| u16::from(p.club))
            .collect();
        if clubs != required {
            return Err("missing or unreferenced canonical player-club roster".into());
        }
        for side in 0..2 {
            role_scores::insert(&mut c, 0xD56 + side, s.side_offsets[side])?;
            add_word(&mut c, 0xD6E4 + 2 * side, s.clubs[side])?;
            for (k, id) in s.cache[side].iter().enumerate() {
                if *id != 0 {
                    needed.insert(*id);
                }
                add_word(&mut c, cache_address(side as u8, k as u8 + 1), *id)?;
            }
        }
        if needed.iter().any(|id| !players.contains_key(id)) {
            return Err("missing canonical role player".into());
        }
        for (i, v) in s.starts.iter().enumerate() {
            role_scores::insert(&mut c, 0x09F6 + i, *v)?;
        }
        role_scores::insert(&mut c, 0xD29, s.bonus)?;
        role_scores::insert(&mut c, 0x09FB + usize::from(s.criterion), s.criterion_cell)?;
        add_word(&mut c, 0xA51C, s.seed)?;
        Ok(Self {
            e: role_scores::image(c),
            s: s.clone(),
        })
    }
    #[cfg(test)]
    fn slot(&self, id: u16) -> u16 {
        player_slot(&self.e, id)
    }
    fn run(&mut self) -> Result<Outcome, String> {
        let trace = select_image(&mut self.e, self.s.side, self.s.kind, self.s.criterion)?;
        let mut output = self.s.clone();
        output.cache[usize::from(self.s.side)][usize::from(self.s.kind - 1)] =
            word(&self.e, cache_address(self.s.side, self.s.kind));
        output.scenes = (1..=22).map(|i| Scene::read(&self.e, i)).collect();
        Ok(Outcome {
            output,
            clear: trace.clear,
            visits: trace.visits,
            draws: vec![],
        })
    }
}
// [confirmed] F33/8973: last matching first-eleven roster word; zero if absent.
fn player_slot(e: &Engine, id: u16) -> u16 {
    let club = e.image[P8 + 8 * usize::from(id)];
    let mut last = 0;
    for slot in 1..=11 {
        if word(e, CLUB + 62 * usize::from(club - 1) + 2 * (slot - 1)) == id {
            last = slot as u16;
        }
    }
    if last != 0 {
        last += u16::from(e.image[0xD56 + usize::from(u16::from(club) == word(e, 0xD6E6))]);
    }
    last
}
fn lookup(e: &Engine, id: u16, criterion: u16) -> Value {
    let a = P8 + 8 * usize::from(id);
    let p4 = e.image[a + 4];
    let p5 = e.image[a + 5];
    let p7 = e.image[a + 7];
    let first = u16::from(p7).wrapping_mul(22).wrapping_add(criterion);
    let first_cell = e.image[usize::from(0x957Du16.wrapping_add(first))];
    let second = u16::from(p5)
        .wrapping_mul(22)
        .wrapping_add(u16::from(first_cell));
    let second_cell = e.image[usize::from(0x96B1u16.wrapping_add(second))];
    json!({"player":id,"p8_4":p4,"p8_5":p5,"p8_7":p7,"first_index":first,"first_cell":first_cell,"second_index":second,"second_cell":second_cell})
}
fn score(
    e: &Engine,
    index: u8,
    criterion: u16,
    site: &str,
    kind: &str,
) -> Result<(u16, Value), String> {
    let row = Scene::read(e, index);
    let id = row.word14;
    if e.image[P15 + 15 * usize::from(id) + 6] > 31 {
        return Err("unsupported role score P15+6 domain".into());
    }
    let score = role_scores::scene_image(e, index, criterion);
    let child = json!({"site":"0AB9:1B60","kind":"table","arguments":[criterion,id],"draw_start":0,"children":[],"input":lookup(e,id,criterion),"result":score.table.output.value,"draw_end":0});
    let h = json!({"site":site,"kind":kind,"arguments":[index,criterion],"draw_start":0,"children":[child],"scene_input":row,"scene_output":score.scene,"result":score.output.value,"draw_end":0});
    Ok((score.output.value, h))
}
#[derive(Debug, Serialize)]
pub(crate) struct Trace {
    pub clear: Value,
    pub visits: Vec<Value>,
}
pub(crate) fn select_image(
    e: &mut Engine,
    side: u8,
    kind: u8,
    criterion: u16,
) -> Result<Trace, String> {
    if side > 1 || !(1..=5).contains(&kind) || criterion > 255 {
        return Err("unsupported role image arguments".into());
    }
    let a = cache_address(side, kind);
    let before = word(e, a);
    put(e, a, 0);
    let clear = json!({"before":before,"after":0,"draw":0});
    let mut visits = vec![];
    let first = u16::from(e.image[0x09F6 + usize::from(kind == 5)]) + 1;
    for slot in first..=11 {
        let index = e.image[0xD56 + usize::from(side)] + slot as u8;
        let row = Scene::read(e, index);
        let id = row.word14;
        let p = P15 + 15 * usize::from(id);
        let av = placement::unavailable(&placement::Player {
            id,
            p15_sbyte8: e.image[p + 8] as i8,
            p15_byte11: e.image[p + 11],
        });
        let cache_before = word(e, a);
        let mut stored = 0;
        let mut helpers = vec![
            json!({"site":"0AB9:14B8","kind":"available","arguments":[id],"draw_start":0,"children":[],"result":av,"draw_end":0}),
        ];
        if av == 0 {
            let improve = if cache_before == 0 {
                true
            } else {
                let old = player_slot(e, cache_before);
                if !(1..=22).contains(&old) || Scene::read(e, old as u8).word14 != cache_before {
                    return Err("missing or mismatched cached-player scene".into());
                }
                helpers.push(json!({"site":"0AB9:14EF","kind":"slot","arguments":[cache_before],"draw_start":0,"children":[],"result":old,"draw_end":0}));
                let (old_score, h) = score(e, old as u8, criterion, "0AB9:14FC", "old-score")?;
                helpers.push(h);
                let (new_score, h) = score(e, index, criterion, "0AB9:1509", "new-score")?;
                helpers.push(h);
                new_score > old_score
            };
            if improve {
                put(e, a, id);
                stored = 1;
            }
        }
        visits.push(json!({"slot":slot,"scene":index,"player":id,"cache_before":cache_before,"scene_input":row,"draw_start":0,"helpers":helpers,"stored":stored,"cache_after":word(e,a),"scene_output":Scene::read(e,index),"draw_end":0}));
    }
    Ok(Trace { clear, visits })
}

pub fn select(s: &State) -> Result<Outcome, String> {
    Machine::new(s)?.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::role_scores::{Attributes, Cell};
    // [designed] Static/synthetic cases separate from T33 original coverage.
    fn sample() -> State {
        let c: Value = serde_json::from_str(
            include_str!("../tests/fixtures/role_cache/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let row: Scene = serde_json::from_value(c["input"]["scenes"][0].clone()).unwrap();
        let players = (0..=22)
            .map(|id| Player {
                id,
                club: if id == 0 {
                    0
                } else if id < 12 {
                    1
                } else {
                    2
                },
                byte6: 0,
                byte8: 1,
                flags11: 0,
                table: TableInput {
                    criterion: 21,
                    player: Attributes {
                        id,
                        p8_4: 31,
                        p8_5: 0,
                        p8_7: 0,
                    },
                    first: Cell {
                        index: 21,
                        value: 0,
                    },
                    second: Cell {
                        index: 0,
                        value: 128,
                    },
                },
            })
            .collect();
        State {
            kind: 2,
            criterion: 21,
            side: 0,
            seed: 0xBEEF,
            side_offsets: [0, 11],
            clubs: [1, 2],
            starts: [0; 6],
            cache: [[7; 5], [12; 5]],
            scenes: (1..=22)
                .map(|id| {
                    let mut r = row.clone();
                    r.index = id;
                    r.word14 = u16::from(id);
                    r.byte7 = id;
                    r.byte18 = 0;
                    r.byte22 = 0;
                    r
                })
                .collect(),
            players,
            rosters: vec![
                Roster {
                    club: 1,
                    roster: std::array::from_fn(|i| i as u16 + 1),
                },
                Roster {
                    club: 2,
                    roster: std::array::from_fn(|i| i as u16 + 12),
                },
            ],
            bonus: 0,
            criterion_cell: 0,
        }
    }
    fn winner(s: &State) -> u16 {
        select(s).unwrap().output.cache[usize::from(s.side)][usize::from(s.kind - 1)]
    }
    #[test]
    fn clear_then_first_eligible_and_equal_scores_retain_first() {
        let s = sample();
        let o = select(&s).unwrap();
        assert_eq!(o.clear, json!({"before":7,"after":0,"draw":0}));
        assert_eq!(winner(&s), 1);
        assert_eq!(o.visits.len(), 11);
        assert_eq!(o.visits[0]["helpers"].as_array().unwrap().len(), 1);
        assert_eq!(o.visits.iter().filter(|v| v["stored"] == 1).count(), 1);
        let h = &o.visits[1]["helpers"];
        assert_eq!(h[2]["result"], h[3]["result"]);
        assert_eq!(h[2]["result"], 48);
    }
    #[test]
    fn strictly_better_score_stores_then_lesser_score_retains() {
        let mut s = sample();
        s.players[2].table.player.p8_4 = 63;
        let o = select(&s).unwrap();
        assert_eq!(winner(&s), 2);
        assert_eq!(o.visits[1]["stored"], 1);
        assert_eq!(o.visits[2]["stored"], 0);
        assert_eq!(o.visits[1]["helpers"][3]["result"], 97);
        assert_eq!(o.visits.iter().filter(|v| v["stored"] == 1).count(), 2);
    }
    #[test]
    fn blocked_and_zero_candidates_skip_all_other_helpers() {
        let mut s = sample();
        s.players[1].flags11 = 0x80;
        s.scenes[1].word14 = 0;
        let o = select(&s).unwrap();
        assert_eq!(winner(&s), 3);
        for v in &o.visits[..2] {
            assert_eq!(v["helpers"].as_array().unwrap().len(), 1);
            assert_eq!(v["helpers"][0]["result"], 1);
            assert_eq!(v["stored"], 0);
        }
    }
    #[test]
    fn all_blocked_leaves_defined_zero_cache_without_scoring() {
        let mut s = sample();
        for p in &mut s.players {
            p.flags11 = 0x80;
            p.byte6 = 255;
        }
        assert_eq!(winner(&s), 0);
        let o = select(&s).unwrap();
        assert!(o
            .visits
            .iter()
            .all(|v| v["helpers"].as_array().unwrap().len() == 1));
    }
    #[test]
    fn empty_ranges_clear_and_kind_five_uses_alternate_start_byte() {
        let mut s = sample();
        s.starts[0] = 255;
        let o = select(&s).unwrap();
        assert!(o.visits.is_empty());
        assert_eq!(winner(&s), 0);
        s.starts[0] = 10;
        s.starts[1] = 9;
        s.kind = 5;
        let o = select(&s).unwrap();
        assert_eq!(o.visits.len(), 2);
        assert_eq!(o.visits[0]["slot"], 10);
        assert_eq!(winner(&s), 10);
        s.kind = 4;
        let o = select(&s).unwrap();
        assert_eq!(o.visits.len(), 1);
        assert_eq!(winner(&s), 11);
    }
    #[test]
    fn lookup_uses_last_roster_match_and_shared_canonical_player() {
        let mut s = sample();
        s.rosters[0].roster[10] = 1;
        s.scenes[10].word14 = 1;
        let m = Machine::new(&s).unwrap();
        assert_eq!(m.slot(1), 11);
        let o = select(&s).unwrap();
        assert_eq!(winner(&s), 1);
        assert_eq!(o.visits[1]["helpers"][1]["result"], 11);
        assert_eq!(o.visits[1]["helpers"][2]["scene_input"]["index"], 11);
    }
    #[test]
    fn absent_roster_returns_zero_but_reached_row_zero_scoring_diagnoses() {
        let mut s = sample();
        s.rosters[0].roster[0] = 11;
        assert_eq!(Machine::new(&s).unwrap().slot(1), 0);
        assert!(select(&s).unwrap_err().contains("missing or mismatched"));
    }
    #[test]
    fn mismatched_last_match_scene_diagnoses() {
        let mut s = sample();
        s.rosters[0].roster[10] = 1;
        assert_eq!(Machine::new(&s).unwrap().slot(1), 11);
        assert!(select(&s).unwrap_err().contains("missing or mismatched"));
    }
    #[test]
    fn alternate_side_swapped_offsets_and_maximum_club_and_player() {
        let mut s = sample();
        s.side = 1;
        assert_eq!(winner(&s), 12);
        s.side_offsets = [11, 0];
        s.clubs = [2, 176];
        s.rosters[0].club = 176;
        for p in &mut s.players {
            if p.club == 1 {
                p.club = 176;
            }
        }
        assert_eq!(winner(&s), 1);
        let mut s = sample();
        s.side = 1;
        s.players[22].id = 3920;
        s.players[22].table.player.id = 3920;
        s.scenes[21].word14 = 3920;
        s.rosters[1].roster[10] = 3920;
        assert_eq!(winner(&s), 12);
    }
    #[test]
    fn unsupported_baseline_only_diagnoses_when_score_is_reached() {
        let mut s = sample();
        s.players[1].byte6 = 32;
        assert!(select(&s).unwrap_err().contains("P15+6"));
        s.starts[0] = 10;
        s.players[11].byte6 = 255;
        assert_eq!(winner(&s), 11);
    }
    #[test]
    fn contradictory_raw_table_and_global_aliases_reject_before_setup() {
        let mut s = sample();
        s.players[1].table.first.value = 1;
        s.players[1].table.second.index = 1;
        assert!(Machine::new(&s)
            .err()
            .unwrap()
            .contains("conflicting raw alias"));
        let mut s = sample();
        s.criterion = 0;
        for p in &mut s.players {
            p.table.criterion = 0;
            p.table.first.index = 0;
        }
        s.criterion_cell = 1; //09FB aliases the sixth captured criterion byte.
        assert!(Machine::new(&s)
            .err()
            .unwrap()
            .contains("conflicting raw alias"));
        s.starts[5] = 1;
        assert!(select(&s).is_ok());
    }
    #[test]
    fn malformed_canonical_inputs_reject_without_changing_caller() {
        let s = sample();
        let before = serde_json::to_value(&s).unwrap();
        let mut bad = s.clone();
        bad.players.pop();
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.players.push(bad.players[1].clone());
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.players[1].id = 3921;
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.players[1].table.player.id = 2;
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.players[1].table.first.index = 22;
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.rosters.pop();
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.rosters.push(bad.rosters[0].clone());
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.scenes.pop();
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.scenes.swap(0, 1);
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.kind = 0;
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.kind = 6;
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.side = 2;
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.side_offsets = [0, 0];
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.clubs = [0, 2];
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.clubs = [1, 1];
        assert!(select(&bad).is_err());
        bad = s.clone();
        bad.criterion = 256;
        assert!(select(&bad).is_err());
        assert_eq!(serde_json::to_value(&s).unwrap(), before);
    }
    #[test]
    fn expected_and_nested_score_fields_are_rejected() {
        let s = sample();
        for field in ["output", "expected", "visits", "scores", "selected"] {
            let mut v = serde_json::to_value(&s).unwrap();
            v[field] = json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
        let mut v = serde_json::to_value(&s).unwrap();
        v["players"][1]["table"]["result"] = json!(128);
        assert!(serde_json::from_value::<State>(v).is_err());
    }
    #[test]
    fn raw_cells_drive_actual_nested_computation_and_observation() {
        let mut s = sample();
        let before = select(&s).unwrap();
        for p in &mut s.players {
            p.table.second.value = 64;
        }
        let after = select(&s).unwrap();
        assert_eq!(before.visits[1]["helpers"][2]["result"], 48);
        assert_eq!(after.visits[1]["helpers"][2]["result"], 24);
        assert_eq!(after.visits[1]["helpers"][2]["children"][0]["result"], 64);
        assert_eq!(
            after.visits[1]["helpers"][2]["children"][0]["input"]["second_cell"],
            64
        );
    }
    #[test]
    fn complete_image_preserved_except_selected_cache_word_on_all_originals() {
        for line in include_str!("../tests/fixtures/role_cache/cases.jsonl").lines() {
            let c: Value = serde_json::from_str(line).unwrap();
            let s: State = serde_json::from_value(c["input"].clone()).unwrap();
            let mut m = Machine::new(&s).unwrap();
            m.e.image[0x8000] = 0xAB;
            let before = m.e.image.clone();
            let out = m.run().unwrap();
            let a = cache_address(s.side, s.kind);
            for (i, (old, new)) in before.iter().zip(m.e.image.iter()).enumerate() {
                if i != a && i != a + 1 {
                    assert_eq!(old, new, "unowned byte {i:X}");
                }
            }
            assert_eq!(word(&m.e, 0xA51C), s.seed);
            assert_eq!(
                word(&m.e, a),
                out.output.cache[usize::from(s.side)][usize::from(s.kind - 1)]
            );
            assert_eq!(out.output.scenes, s.scenes);
        }
    }
}
