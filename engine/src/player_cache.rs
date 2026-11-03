//! [confirmed] F31–F33 at archaeology218490d: bounded1539 shared-image parent.
use crate::{role_cache::Roster, role_scores::TableInput, terminal::Scene, Draw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub club: u8,
    pub p8_1: u8,
    pub p8_3: u8,
    pub p8_4: u8,
    pub p8_5: u8,
    pub p8_7: u8,
    pub byte6: u8,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CriterionCell {
    pub criterion: u16,
    pub value: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub seed: u16,
    pub gate: u8,
    pub criteria: [u8; 6],
    pub cache: [[u16; 5]; 2],
    pub managed_clubs: [u8; 2],
    pub clubs: [u16; 2],
    pub side_offsets: [u8; 2],
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub rosters: Vec<Roster>,
    pub tables: Vec<TableInput>,
    pub bonus: u8,
    pub criterion_cells: Vec<CriterionCell>,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub events: Vec<Value>,
    pub roles: Vec<Value>,
    pub draws: Vec<Draw>,
}

use crate::{cache_helpers, cache_selection, placement, role_cache, role_scores, Engine};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
const P8: usize = crate::layout::P8;
const P15: usize = crate::layout::P15;
const CLUB: usize = crate::layout::CLUB;
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
pub(crate) fn cells(s: &State) -> Result<role_scores::Cells, String> {
    if !matches!(s.side_offsets, [0, 11] | [11, 0])
        || s.clubs.iter().any(|id| !(1..=176).contains(id))
        || s.clubs[0] == s.clubs[1]
        || s.scenes.len() != 22
    {
        return Err("unsupported1539 offset/club/scene domain".into());
    }
    let mut bytes = role_scores::Cells::new();
    let mut players = BTreeMap::new();
    let mut needed = BTreeSet::new();
    for p in &s.players {
        if p.id > 3920
            || p.club > 176
            || (p.id != 0 && p.club == 0)
            || players.insert(p.id, p).is_some()
        {
            return Err("ambiguous1539 canonical player".into());
        }
        let a = P8 + 8 * usize::from(p.id);
        let b = P15 + 15 * usize::from(p.id);
        for (a, v) in [
            (a, p.club),
            (a + 1, p.p8_1),
            (a + 3, p.p8_3),
            (a + 4, p.p8_4),
            (a + 5, p.p8_5),
            (a + 7, p.p8_7),
            (b + 6, p.byte6),
            (b + 8, p.byte8),
            (b + 11, p.flags11),
        ] {
            role_scores::insert(&mut bytes, a, v)?;
        }
    }
    let criteria: BTreeSet<u16> = s.criteria[2..].iter().copied().map(u16::from).collect();
    let mut tables = BTreeSet::new();
    for t in &s.tables {
        let p = players
            .get(&t.player.id)
            .ok_or("missing1539 table player")?;
        if !criteria.contains(&t.criterion)
            || !tables.insert((t.criterion, t.player.id))
            || (p.p8_4, p.p8_5, p.p8_7) != (t.player.p8_4, t.player.p8_5, t.player.p8_7)
        {
            return Err("duplicate or inconsistent1539 table alias".into());
        }
        for (a, v) in role_scores::table_cells(t)? {
            role_scores::insert(&mut bytes, a, v)?;
        }
    }
    let expected: BTreeSet<_> = criteria
        .iter()
        .flat_map(|c| players.keys().map(move |id| (*c, *id)))
        .collect();
    if tables != expected {
        return Err("incomplete1539 multi-criterion table read set".into());
    }
    let mut cells = BTreeSet::new();
    for c in &s.criterion_cells {
        if !cells.insert(c.criterion) || !criteria.contains(&c.criterion) {
            return Err("duplicate or unexpected1539 criterion cell".into());
        }
        role_scores::insert(&mut bytes, 0x09FB + usize::from(c.criterion), c.value)?;
    }
    if cells != criteria {
        return Err("incomplete1539 criterion read set".into());
    }
    for (i, row) in s.scenes.iter().enumerate() {
        if usize::from(row.index) != i + 1 {
            return Err("unordered1539 scene rows".into());
        }
        needed.insert(row.word14);
        for (o, v) in row.bytes().into_iter().enumerate() {
            role_scores::insert(&mut bytes, 0xCCEF + 25 * (i + 1) + o, v)?;
        }
    }
    let mut clubs = BTreeSet::new();
    for r in &s.rosters {
        if !(1..=176).contains(&r.club) || !clubs.insert(r.club) {
            return Err("ambiguous1539 club roster".into());
        }
        for (slot, id) in r.roster.iter().enumerate() {
            needed.insert(*id);
            add_word(
                &mut bytes,
                CLUB + 62 * usize::from(r.club - 1) + 2 * slot,
                *id,
            )?;
        }
    }
    let required: BTreeSet<_> = s
        .clubs
        .iter()
        .copied()
        .chain(
            s.players
                .iter()
                .filter(|p| p.id != 0)
                .map(|p| u16::from(p.club)),
        )
        .collect();
    if clubs != required {
        return Err("incomplete or unreferenced1539 roster set".into());
    }
    for side in 0..2 {
        add_word(&mut bytes, 0xD6E4 + 2 * side, s.clubs[side])?;
        role_scores::insert(&mut bytes, 0xD56 + side, s.side_offsets[side])?;
        role_scores::insert(&mut bytes, 0xDCBA + side, s.managed_clubs[side])?;
        for (k, id) in s.cache[side].iter().enumerate() {
            needed.insert(*id);
            add_word(&mut bytes, cache_address(side as u8, k as u8 + 1), *id)?;
        }
    }
    if needed.iter().any(|id| !players.contains_key(id)) {
        return Err("missing1539 referenced canonical player".into());
    }
    for (i, v) in s.criteria.iter().enumerate() {
        role_scores::insert(&mut bytes, 0x09F6 + i, *v)?;
    }
    for (a, v) in [(0xD18, s.gate), (0xD29, s.bonus)] {
        role_scores::insert(&mut bytes, a, v)?;
    }
    add_word(&mut bytes, 0xA51C, s.seed)?;
    Ok(bytes)
}
fn machine(s: &State) -> Result<Engine, String> {
    Ok(role_scores::image(cells(s)?))
}
// [derived] Closed helpers preserve all validated input fields except caches/byte18.
// Full-image tests check that invariant,including seed; snapshots read current mutable cells.
fn snapshot(e: &Engine, s: &State) -> State {
    let mut out = s.clone();
    out.seed = word(e, 0xA51C);
    for side in 0..2 {
        for k in 1..=5 {
            out.cache[side][k - 1] = word(e, cache_address(side as u8, k as u8));
        }
    }
    out.scenes = (1..=22).map(|i| Scene::read(e, i)).collect();
    out
}
fn event(
    site: &str,
    kind: &str,
    side: u8,
    args: Vec<u16>,
    input: State,
    output: State,
    result: Option<u16>,
) -> Value {
    json!({"site":site,"kind":kind,"side":side,"arguments":args,"input":input,"output":output,"result":result,"draw_start":0,"draw_end":0})
}
fn readonly(
    e: &Engine,
    s: &State,
    site: &str,
    kind: &str,
    side: u8,
    args: Vec<u16>,
    result: u16,
) -> Value {
    let state = snapshot(e, s);
    event(site, kind, side, args, state.clone(), state, Some(result))
}
pub(crate) fn run(e: &mut Engine, s: &State) -> Result<Outcome, String> {
    let mut events = vec![];
    let mut roles = vec![];
    // [confirmed] F31/1539: managed/gate,kind1 alias,roles,then eleven caller stores per side.
    for side in 0..=1 {
        let club = word(e, 0xD6E4 + 2 * usize::from(side));
        let managed = cache_helpers::managed_image(e, club);
        events.push(readonly(
            e,
            s,
            "0AB9:1569",
            "managed",
            side,
            vec![club],
            u16::from(managed),
        ));
        if managed == 0 || e.image[0xD18] == 0 {
            if managed == 0 {
                let selected = cache_selection::select_image(e, side)?.output.player;
                events.push(readonly(
                    e,
                    s,
                    "0AB9:1588",
                    "select",
                    side,
                    vec![u16::from(side)],
                    selected,
                ));
                put(e, cache_address(side, 1), selected);
            }
            for kind in 2..=5 {
                let criterion = u16::from(e.image[0x09F6 + usize::from(kind)]);
                let before = snapshot(e, s);
                let trace = role_cache::select_image(e, side, kind, criterion)?;
                let args = vec![u16::from(kind), criterion, u16::from(side)];
                roles.push(json!({"event":events.len(),"arguments":args,"clear":trace.clear,"visits":trace.visits}));
                events.push(event(
                    "0AB9:15B3",
                    "role",
                    side,
                    args,
                    before,
                    snapshot(e, s),
                    None,
                ));
            }
        } else {
            let id = word(e, cache_address(side, 1));
            let a = P15 + 15 * usize::from(id);
            let available = placement::unavailable(&placement::Player {
                id,
                p15_sbyte8: e.image[a + 8] as i8,
                p15_byte11: e.image[a + 11],
            });
            events.push(readonly(
                e,
                s,
                "0AB9:15D0",
                "available",
                side,
                vec![id],
                u16::from(available),
            ));
            let needs_selection = if available != 0 {
                true
            } else {
                let slot = cache_helpers::slot_image(e, side, 1);
                events.push(readonly(
                    e,
                    s,
                    "0AB9:15DF",
                    "slot",
                    side,
                    vec![1, u16::from(side)],
                    slot,
                ));
                slot > 11
            };
            if needs_selection {
                // [confirmed] seg_0AB9_18 B6DE..B73A at archaeology d9e20a4;
                // T28/settings-r0001 confirms the original config+F6 is nonzero.
                // The bounded debug API leaves this setting zero and still diagnoses UI.
                if e.image[crate::layout::CONFIG + 0xF6] == 0 {
                    return Err("unsupported1539 managed/UI fallbackB6BD".into());
                }
                let before = snapshot(e, s);
                let selected = cache_selection::select_image(e, side)?.output.player;
                put(e, cache_address(side, 1), selected);
                events.push(event(
                    "0AB9:15ED",
                    "managed_select",
                    side,
                    vec![1, u16::from(side)],
                    before,
                    snapshot(e, s),
                    Some(selected),
                ));
            }
        }
        for slot in 1..=11 {
            let before = snapshot(e, s);
            let v = cache_helpers::boost_one(e, side, slot);
            //4EBF is read-only;the captured helper return precedes its caller's single store.
            events.push(event(
                v.site,
                "rating",
                side,
                v.arguments.to_vec(),
                before.clone(),
                before,
                Some(v.rating),
            ));
            events.push(json!({"site":v.store_site,"kind":"store","side":side,"arguments":v.arguments,"input":v.input,"output":v.output,"result":null,"draw_start":0,"draw_end":0}));
        }
    }
    Ok(Outcome {
        output: snapshot(e, s),
        events,
        roles,
        draws: vec![],
    })
}
pub fn refresh(s: &State) -> Result<Outcome, String> {
    run(&mut machine(s)?, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::role_scores::{Attributes, Cell};
    // [designed] Static/synthetic inputs;original evidence is replayed separately.
    fn sample() -> State {
        let c: Value = serde_json::from_str(
            include_str!("../tests/fixtures/player_cache/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let row: Scene = serde_json::from_value(c["input"]["scenes"][0].clone()).unwrap();
        let players: Vec<_> = (0..=22)
            .map(|id| Player {
                id,
                club: if id == 0 {
                    0
                } else if id < 12 {
                    1
                } else {
                    2
                },
                p8_1: 21,
                p8_3: 7,
                p8_4: 31,
                p8_5: 0,
                p8_7: 0,
                byte6: 0,
                byte8: 1,
                flags11: 0,
            })
            .collect();
        let tables = [4, 13, 21, 22]
            .into_iter()
            .flat_map(|criterion| {
                players.iter().map(move |p| TableInput {
                    criterion,
                    player: Attributes {
                        id: p.id,
                        p8_4: 31,
                        p8_5: 0,
                        p8_7: 0,
                    },
                    first: Cell {
                        index: criterion,
                        value: 0,
                    },
                    second: Cell {
                        index: 0,
                        value: 128,
                    },
                })
            })
            .collect();
        State {
            seed: 0xBEEF,
            gate: 1,
            criteria: [1, 0, 13, 21, 22, 4],
            cache: [[1; 5], [12; 5]],
            managed_clubs: [1, 0],
            clubs: [1, 2],
            side_offsets: [0, 11],
            scenes: (1..=22)
                .map(|i| {
                    let mut r = row.clone();
                    r.index = i;
                    r.word14 = u16::from(i);
                    r.byte7 = i;
                    r.byte18 = 250;
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
            tables,
            bonus: 0,
            criterion_cells: [4, 13, 21, 22]
                .into_iter()
                .map(|criterion| CriterionCell {
                    criterion,
                    value: 0,
                })
                .collect(),
        }
    }
    fn count(o: &Outcome, kind: &str) -> usize {
        o.events.iter().filter(|e| e["kind"] == kind).count()
    }
    #[test]
    fn all_managed_gate_branch_combinations_follow_static_order() {
        for (managed, gate, selectors, roles, available) in [
            ([1, 0], 1, 1, 4, 1),
            ([0, 0], 1, 2, 8, 0),
            ([1, 2], 1, 0, 0, 2),
            ([1, 0], 0, 1, 8, 0),
            ([1, 2], 0, 0, 8, 0),
            ([0, 0], 0, 2, 8, 0),
            ([1, 2], 255, 0, 0, 2),
        ] {
            let mut s = sample();
            s.managed_clubs = managed;
            s.gate = gate;
            let o = refresh(&s).unwrap();
            assert_eq!(count(&o, "managed"), 2);
            assert_eq!(count(&o, "select"), selectors);
            assert_eq!(o.roles.len(), roles);
            assert_eq!(count(&o, "available"), available);
            assert_eq!(count(&o, "slot"), available);
            assert_eq!(count(&o, "rating"), 22);
            assert_eq!(count(&o, "store"), 22);
        }
    }
    #[test]
    fn selector_return_is_stored_by_caller_before_roles_and_ratings() {
        let mut s = sample();
        s.cache[1][0] = 22;
        for p in s.players.iter_mut().filter(|p| p.id >= 12) {
            p.p8_3 = 0;
        }
        s.players[12].p8_3 = 1;
        s.players[13].p8_3 = 7;
        let o = refresh(&s).unwrap();
        let selected = o.events.iter().find(|e| e["kind"] == "select").unwrap();
        assert_eq!(selected["result"], 13);
        assert_eq!(selected["input"]["cache"][1][0], 22);
        assert_eq!(selected["output"]["cache"][1][0], 22);
        let first_role = o.events.iter().find(|e| e["kind"] == "role").unwrap();
        assert_eq!(first_role["input"]["cache"][1][0], 13);
        assert_eq!(o.output.cache[1][0], 13);
        for e in o
            .events
            .iter()
            .filter(|e| e["kind"] == "rating" && e["side"] == 1)
        {
            assert_eq!(e["arguments"][2], 13);
            assert_eq!(e["result"], 7);
        }
        s.players[13].p8_3 = 2;
        let changed = refresh(&s).unwrap();
        assert_eq!(changed.output.cache[1][0], 13);
        assert!(changed.output.scenes[11..].iter().all(|r| r.byte18 == 254));
    }
    #[test]
    fn native_role_scores_drive_caches_and_subsequent_role_inputs() {
        let mut s = sample();
        s.players[14].p8_4 = 63;
        for t in s.tables.iter_mut().filter(|t| t.player.id == 14) {
            t.player.p8_4 = 63;
        }
        let o = refresh(&s).unwrap();
        assert_eq!(o.output.cache[1], [12, 14, 14, 14, 14]);
        let roles: Vec<_> = o.events.iter().filter(|e| e["kind"] == "role").collect();
        assert_eq!(roles[0]["output"]["cache"][1][1], 14);
        assert_eq!(roles[1]["input"]["cache"][1][1], 14);
        assert_eq!(o.roles[0]["visits"][1]["stored"], 1);
    }
    #[test]
    fn first_side_stores_precede_second_side_managed_and_roles() {
        let s = sample();
        let o = refresh(&s).unwrap();
        assert_eq!(o.output.cache, [[1; 5], [12, 13, 13, 13, 12]]);
        let next = o
            .events
            .iter()
            .find(|e| e["kind"] == "managed" && e["side"] == 1)
            .unwrap();
        for r in next["input"]["scenes"].as_array().unwrap().iter().take(11) {
            assert_eq!(r["byte18"], 8);
        }
        for r in next["input"]["scenes"].as_array().unwrap().iter().skip(11) {
            assert_eq!(r["byte18"], 250);
        }
        let roles: Vec<_> = o
            .events
            .iter()
            .filter(|e| e["kind"] == "role")
            .map(|e| e["arguments"][0].as_u64().unwrap())
            .collect();
        assert_eq!(roles, vec![2, 3, 4, 5]);
        assert!(o.output.scenes.iter().all(|r| r.byte18 == 8));
    }
    #[test]
    fn empty_candidate_ranges_clear_roles_and_still_apply_cached_ratings() {
        let mut s = sample();
        s.criteria[0] = 255;
        s.criteria[1] = 255;
        let o = refresh(&s).unwrap();
        assert_eq!(o.output.cache[1], [12, 0, 0, 0, 0]);
        assert!(o
            .roles
            .iter()
            .all(|r| r["visits"].as_array().unwrap().is_empty()));
        assert!(o.output.scenes.iter().all(|r| r.byte18 == 8));
    }
    #[test]
    fn managed_gate_zero_preserves_zero_kind_one_and_rates_its_raw_bytes() {
        let mut s = sample();
        s.gate = 0;
        s.cache[0][0] = 0;
        let o = refresh(&s).unwrap();
        assert_eq!(o.output.cache[0][0], 0);
        for e in o
            .events
            .iter()
            .filter(|e| e["kind"] == "rating" && e["side"] == 0)
        {
            assert_eq!(e["arguments"][2], 0);
            assert_eq!(e["result"], 7);
        }
    }
    #[test]
    fn automatic_managed_fallback_selects_from_current_image() {
        let mut s = sample();
        s.cache[0][0] = 0;
        let mut e = machine(&s).unwrap();
        e.image[crate::layout::CONFIG + 0xF6] = 1;
        let selected = cache_selection::select_image(&e, 0).unwrap().output.player;
        let before_seed = word(&e, 0xA51C);
        let result = run(&mut e, &s).unwrap();
        assert_eq!(result.output.cache[0][0], selected);
        assert_eq!(word(&e, 0xA51C), before_seed);
        assert!(result.draws.is_empty());
        // Automatic B6BD kind1 stores only that cache,not the other role caches.
        assert_eq!(result.output.cache[0][1..], s.cache[0][1..]);
    }
    #[test]
    fn reached_managed_fallback_diagnoses_and_caller_input_is_unchanged() {
        let s = sample();
        let before = serde_json::to_value(&s).unwrap();
        let mut bad = s.clone();
        bad.players[1].byte8 = 0;
        assert!(refresh(&bad).unwrap_err().contains("fallbackB6BD"));
        bad = s.clone();
        bad.rosters[0].roster[0] = 11;
        assert!(refresh(&bad).unwrap_err().contains("fallbackB6BD"));
        assert_eq!(serde_json::to_value(&s).unwrap(), before);
    }
    #[test]
    fn undefined_no_eligible_selector_is_not_replaced_with_player_zero() {
        let mut s = sample();
        for p in s.players.iter_mut().filter(|p| p.id >= 12) {
            p.flags11 = 0x80;
        }
        assert!(refresh(&s).unwrap_err().contains("undefined"));
    }
    #[test]
    fn reached_role_score_and_cached_scene_diagnostics_are_preserved() {
        let mut s = sample();
        s.players[13].byte6 = 32;
        assert!(refresh(&s).unwrap_err().contains("P15+6"));
        let mut s = sample();
        s.rosters[1].roster[1] = 12;
        assert!(refresh(&s).unwrap_err().contains("missing or mismatched"));
        let mut s = sample();
        s.players[14].byte6 = 255;
        s.players[14].flags11 = 0x80;
        assert!(refresh(&s).is_ok());
    }
    #[test]
    fn complete_multi_criterion_read_sets_and_aliases_are_required() {
        let s = sample();
        let mut bad = s.clone();
        bad.tables.pop();
        assert!(refresh(&bad).unwrap_err().contains("read set"));
        bad = s.clone();
        bad.tables.push(bad.tables[0].clone());
        assert!(refresh(&bad).unwrap_err().contains("duplicate"));
        bad = s.clone();
        bad.tables[0].player.p8_4 = 99;
        assert!(refresh(&bad).unwrap_err().contains("alias"));
        bad = s.clone();
        bad.tables[0].first.index += 1;
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.tables[0].first.value = 1;
        bad.tables[0].second.index = 1;
        assert!(refresh(&bad).unwrap_err().contains("conflicting raw alias"));
        bad = s.clone();
        bad.criterion_cells.pop();
        assert!(refresh(&bad).unwrap_err().contains("read set"));
        bad = s.clone();
        bad.criterion_cells.push(bad.criterion_cells[0].clone());
        assert!(refresh(&bad).unwrap_err().contains("duplicate"));
        bad = s.clone();
        bad.criterion_cells[0].criterion = 255;
        assert!(refresh(&bad).is_err());
    }
    #[test]
    fn criterion_zero_global_alias_must_agree_before_image_setup() {
        let mut s = sample();
        s.criteria[2] = 0;
        for t in s.tables.iter_mut().filter(|t| t.criterion == 13) {
            t.criterion = 0;
            t.first.index = 0;
        }
        let c = s
            .criterion_cells
            .iter_mut()
            .find(|c| c.criterion == 13)
            .unwrap();
        c.criterion = 0;
        c.value = 1;
        assert!(machine(&s).err().unwrap().contains("conflicting raw alias"));
        s.criterion_cells
            .iter_mut()
            .find(|c| c.criterion == 0)
            .unwrap()
            .value = 4; //09FB is also criteria[5].
        assert!(refresh(&s).is_ok());
    }
    #[test]
    fn canonical_domains_duplicates_and_missing_references_reject() {
        let s = sample();
        let mut bad = s.clone();
        bad.side_offsets = [0, 0];
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.clubs = [1, 1];
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.clubs[0] = 177;
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.scenes.swap(0, 1);
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.scenes.pop();
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.players.push(bad.players[0].clone());
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.players[1].id = 3921;
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.players[1].club = 0;
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.players.pop();
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.cache[0][0] = 3921;
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.rosters.pop();
        assert!(refresh(&bad).is_err());
        bad = s.clone();
        bad.rosters.push(bad.rosters[0].clone());
        assert!(refresh(&bad).is_err());
    }
    #[test]
    fn expected_results_and_nested_observations_are_not_inputs() {
        for field in [
            "output", "expected", "events", "roles", "selected", "rating",
        ] {
            let mut v = serde_json::to_value(sample()).unwrap();
            v[field] = json!(0);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
        let mut v = serde_json::to_value(sample()).unwrap();
        v["tables"][0]["result"] = json!(128);
        assert!(serde_json::from_value::<State>(v).is_err());
        let mut v = serde_json::to_value(sample()).unwrap();
        v["players"][0]["rating"] = json!(7);
        assert!(serde_json::from_value::<State>(v).is_err());
    }
    #[test]
    fn whole_image_owned_only_by_cache_words_and_byte18_on_both_originals() {
        for line in include_str!("../tests/fixtures/player_cache/cases.jsonl").lines() {
            let c: Value = serde_json::from_str(line).unwrap();
            let s: State = serde_json::from_value(c["input"].clone()).unwrap();
            let mut e = machine(&s).unwrap();
            e.image[0x8000] = 0xAB;
            let before = e.image.clone();
            let out = run(&mut e, &s).unwrap();
            let owned: BTreeSet<_> = (0..2)
                .flat_map(|side| {
                    (1..=5).flat_map(move |kind| {
                        let a = cache_address(side, kind);
                        [a, a + 1]
                    })
                })
                .chain((1..=22).map(|i| 0xCCEF + 25 * i + 18))
                .collect();
            for (a, (old, new)) in before.iter().zip(e.image.iter()).enumerate() {
                if !owned.contains(&a) {
                    assert_eq!(old, new, "unowned byte {a:X}");
                }
            }
            assert_eq!(word(&e, 0xA51C), s.seed);
            assert_eq!(
                serde_json::to_value(out.output).unwrap(),
                c["output"]["output"]
            );
        }
    }
}
