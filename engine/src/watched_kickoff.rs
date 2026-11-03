//! [confirmed] F39,c7637bd;56EE/2625/helper listings at d9e20a4.
//! [designed] Initial inactive gameplay projection;speaker/timer I/O external.
use crate::{
    role_scores::{Attributes, Cell},
    terminal::Scene,
    Draw, Engine,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub byte8: u8,
    pub flags11: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Criterion {
    pub kind: u8,
    pub criterion: u8,
    pub first: Cell,
    pub second: Cell,
    pub bonus_flag: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Score {
    pub player: Attributes,
    pub byte6: u8,
    pub byte8: u8,
    pub bonus: u8,
    pub criteria: Vec<Criterion>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub index: u8,
    pub scenes: Vec<Scene>,
    pub players: Vec<Player>,
    pub score: Option<Score>,
    pub target: u16,
    pub phase: u8,
    pub remaining: u16,
    pub signs: [u8; 2],
    pub sound: u8,
    pub gate_bounds: [u16; 2],
    pub actor: u8,
    pub secondary: u8,
    pub active: u8,
    pub flag: u8,
    pub b6: u8,
    pub mode: u8,
    pub ball: u16,
    pub dx: u16,
    pub dy: u16,
    pub distance: u16,
    pub endpoint: u16,
    pub seed: u16,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub ready: u8,
    pub draws: Vec<Draw>,
}
fn at(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn insert_word(c: &mut crate::role_scores::Cells, a: usize, v: u16) -> Result<(), String> {
    for (i, b) in v.to_le_bytes().into_iter().enumerate() {
        crate::role_scores::insert(c, a + i, b)?
    }
    Ok(())
}
fn machine(s: &State) -> Result<Engine, String> {
    use crate::{layout, role_scores as r};
    if s.active != 0 {
        return Err("active-play launch is outside initial kickoff domain".into());
    }
    if !(1..=22).contains(&s.index)
        || !(1..=22).contains(&s.actor)
        || s.secondary > 22
        || s.scenes.len() != 22
    {
        return Err("unsupported kickoff identity".into());
    }
    let mut c = r::Cells::new();
    let mut seen = std::collections::BTreeSet::new();
    for row in &s.scenes {
        if !(1..=22).contains(&row.index)
            || !seen.insert(row.index)
            || row.byte7 > 22
            || row.word14 > 3920
        {
            return Err("unsupported or aliased kickoff scene".into());
        }
        for (i, b) in row.bytes().into_iter().enumerate() {
            r::insert(&mut c, at(row.index) + i, b)?
        }
    }
    let root = s.scenes.iter().find(|r| r.index == s.index).unwrap();
    let mut ids = std::collections::BTreeSet::new();
    for p in &s.players {
        if p.id > 3920 || !ids.insert(p.id) {
            return Err("unsupported or duplicate availability player".into());
        }
        r::insert(&mut c, layout::P15 + 15 * usize::from(p.id) + 8, p.byte8)?;
        r::insert(&mut c, layout::P15 + 15 * usize::from(p.id) + 11, p.flags11)?;
        r::insert(
            &mut c,
            layout::WATCHED_PLAYER_PRESENT + usize::from(p.id),
            1,
        )?;
    }
    if let Some(score) = &s.score {
        if score.player.id != root.word14 {
            return Err("score/root identity alias differs".into());
        }
        r::insert(
            &mut c,
            layout::P15 + 15 * usize::from(score.player.id) + 6,
            score.byte6,
        )?;
        r::insert(
            &mut c,
            layout::P15 + 15 * usize::from(score.player.id) + 8,
            score.byte8,
        )?;
        r::insert(&mut c, 0xD29, score.bonus)?;
        let mut kinds = std::collections::BTreeSet::new();
        for cell in &score.criteria {
            if ![6, 7].contains(&cell.kind) || !kinds.insert(cell.kind) {
                return Err("unsupported or duplicate kickoff criterion".into());
            }
            let t = r::TableInput {
                criterion: u16::from(cell.criterion),
                player: score.player.clone(),
                first: cell.first.clone(),
                second: cell.second.clone(),
            };
            for (a, b) in r::table_cells(&t)? {
                r::insert(&mut c, a, b)?
            }
            r::insert(&mut c, 0xA15 + usize::from(cell.kind), cell.criterion)?;
            r::insert(&mut c, 0x9FB + usize::from(cell.criterion), cell.bonus_flag)?;
            r::insert(
                &mut c,
                layout::WATCHED_SCORE_PRESENT + usize::from(cell.kind),
                1,
            )?;
        }
    }
    for (a, v) in [
        (0xD09F, s.actor),
        (0xD0A0, s.secondary),
        (0xD0B8, s.active),
        (0xCF53, s.flag),
        (0xD0B6, s.b6),
        (0xD0BB, s.mode),
        (0xD0AC, s.phase),
        (0xA338, s.signs[0]),
        (0xA339, s.signs[1]),
        (layout::CONFIG + 2, s.sound),
    ] {
        r::insert(&mut c, a, v)?
    }
    for (a, v) in [
        (0xABC, s.target),
        (0xD0AE, s.remaining),
        (0xD12A, s.gate_bounds[0]),
        (0xD12C, s.gate_bounds[1]),
        (0xCF46, s.ball),
        (0xCF4D, s.dx),
        (0xCF4F, s.dy),
        (0xCF51, s.distance),
        (0xD0A6, s.endpoint),
        (crate::SEED as usize, s.seed),
    ] {
        insert_word(&mut c, a, v)?
    }
    Ok(r::image(c))
}
fn readiness(e: &Engine) -> Result<u8, String> {
    let mut count = 0;
    for index in 1..=22u8 {
        let a = at(index);
        if e.image[a + 19] == 0 || e.image[0xD09F] == index || e.image[0xD0A0] == index {
            count += 1;
            continue;
        }
        let id = word(e, a + 14);
        if e.image[crate::layout::WATCHED_PLAYER_PRESENT + usize::from(id)] == 0 {
            return Err("missing reached availability player".into());
        }
        let p = crate::layout::P15 + 15 * usize::from(id);
        count += crate::placement::unavailable(&crate::placement::Player {
            id,
            p15_sbyte8: e.image[p + 8] as i8,
            p15_byte11: e.image[p + 11],
        });
    }
    Ok(u8::from(count == 22))
}
fn ball_launch(
    e: &mut Engine,
    index: u8,
    target: u16,
    draws: &mut Vec<Draw>,
) -> Result<(), String> {
    let a = at(index);
    e.image[a + 16] = 5;
    e.image[0xCF53] = 1;
    e.image[0xD09F] = e.image[a + 7];
    let [tx, ty] = target.to_le_bytes();
    let dx = tx.wrapping_sub(e.image[0xCF46]) as i8;
    let dy = ty.wrapping_sub(e.image[0xCF47]) as i8;
    let x = i16::from(dx).abs() as u16;
    let y = i16::from(dy).abs() as u16;
    let sign = |e: &Engine, positive: bool| {
        i16::from(e.image[0xA338 + usize::from(positive)] as i8) as u16
    };
    let (mut vx, mut vy) = (0u16, 0u16);
    let mut distance;
    if x > y {
        vx = sign(e, dx > 0);
        distance = x;
        if y >= x / 2 {
            vy = sign(e, dy > 0);
            distance = distance.wrapping_add(y)
        }
    } else {
        vy = sign(e, dy > 0);
        distance = y;
        if x > 0 && x >= y / 2 {
            vx = sign(e, dx > 0);
            distance = distance.wrapping_add(x);
            if y >= x / 2 {
                vy = vy.wrapping_mul(2)
            }
        }
    }
    let mut mode = 6u8;
    let slot = e.image[a + 7];
    if distance > 20 {
        mode = 7
    } else if matches!(slot, 1 | 12) {
        let sy = i16::from(e.image[0xCF47] as i8)
            * i16::from(crate::terminal::direction_image(e, &Scene::read(e, index)).0 as i8);
        if sy < 0
            && i16::from(e.image[0xCF47] as i8).abs() >= 25
            && i16::from(e.image[0xCF46] as i8).abs() <= 5
        {
            mode = 8
        }
    }
    e.image[0xD0BB] = mode;
    if mode != 8 && distance > 1 && e.image[0xD09F] != e.image[0xD0A0] {
        if e.image[crate::layout::WATCHED_SCORE_PRESENT + usize::from(mode)] == 0 {
            return Err("missing reached kickoff score facts".into());
        }
        let id = word(e, a + 14);
        if e.image[crate::layout::P15 + 15 * usize::from(id) + 6] > 31 {
            return Err("unsupported reached score baseline".into());
        }
        let threshold = e.draw(
            word(e, 0xD12A + 2 * usize::from(slot < 12)),
            "0AB9:1BF1",
            draws,
        );
        let skill = crate::role_scores::scene_image(
            e,
            index,
            u16::from(e.image[0xA15 + usize::from(mode)]),
        )
        .output
        .value;
        // 1BD6 sign-extends the draw before its signed-long comparison with skill.
        if i32::from(skill) <= i32::from(threshold as i16) {
            e.image[0xD0A0] = 0;
            if !matches!(slot, 1 | 12) {
                if e.draw(2, "0AB9:2852", draws) == 1 || e.image[0xD0B6] != 0 {
                    distance = distance.wrapping_sub(e.draw(distance / 2, "0AB9:2868", draws));
                } else if vx == 0 {
                    let coin = e.draw(2, "0AB9:287A", draws) == 1;
                    vx = sign(e, coin);
                    vy = vy.wrapping_add(sign(e, (vy as i16) > 0));
                } else if (vy as i16).unsigned_abs() == 1 {
                    vy = 0
                } else {
                    let coin = e.draw(2, "0AB9:28B6", draws) == 1;
                    vy = sign(e, coin);
                }
            }
        }
    }
    e.image[0xD0B6] = 0;
    let divisor = i32::from(vx as i16).abs() + i32::from(vy as i16).abs();
    if divisor == 0 {
        return Err("zero reached ball-vector divisor".into());
    }
    distance = (i32::from(distance) / divisor).max(2) as u16;
    put(e, 0xCF4D, vx);
    put(e, 0xCF4F, vy);
    put(e, 0xCF51, distance);
    e.image[0xD0A6] = e.image[0xCF46].wrapping_add(vx.wrapping_mul(distance) as u8);
    e.image[0xD0A7] = e.image[0xCF47].wrapping_add(vy.wrapping_mul(distance) as u8);
    // [confirmed] Initial2505 branch sets active and returns before active-play scan.
    e.image[0xD0B8] = 1;
    Ok(())
}
pub(crate) fn advance_image(e: &mut Engine, index: u8) -> Result<(u8, Vec<Draw>), String> {
    if !(1..=22).contains(&index) || e.image[0xD0B8] != 0 {
        return Err("unsupported initial kickoff image entry".into());
    }
    let ready = readiness(e)?;
    let mut draws = Vec::new();
    if ready == 0 {
        return Ok((ready, draws));
    }
    e.image[0xD0A0] = 0;
    let target = crate::watched_target::orient(e, index, word(e, 0xABC));
    ball_launch(e, index, target, &mut draws)?;
    for i in 1..=22u8 {
        e.image[at(i) + 19] = 5
    }
    // [designed] Speaker I/O and presentation countdown/timer fields are external.
    Ok((ready, draws))
}
fn snapshot(e: &Engine, s: &State) -> State {
    let mut out = s.clone();
    for row in &mut out.scenes {
        *row = Scene::read(e, row.index)
    }
    out.actor = e.image[0xD09F];
    out.secondary = e.image[0xD0A0];
    out.active = e.image[0xD0B8];
    out.flag = e.image[0xCF53];
    out.b6 = e.image[0xD0B6];
    out.mode = e.image[0xD0BB];
    out.ball = word(e, 0xCF46);
    out.dx = word(e, 0xCF4D);
    out.dy = word(e, 0xCF4F);
    out.distance = word(e, 0xCF51);
    out.endpoint = word(e, 0xD0A6);
    out.seed = word(e, crate::SEED as usize);
    out
}
pub fn advance(s: &State) -> Result<Outcome, String> {
    let mut e = machine(s)?;
    let (ready, draws) = advance_image(&mut e, s.index)?;
    Ok(Outcome {
        output: snapshot(&e, s),
        ready,
        draws,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn packed(x: i8, y: i8) -> u16 {
        u16::from_le_bytes([x as u8, y as u8])
    }
    fn sample() -> State {
        let c: serde_json::Value = serde_json::from_str(
            include_str!("../tests/fixtures/watched_target/cases.jsonl")
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        let template: Scene = serde_json::from_value(c["input"]["scene"].clone()).unwrap();
        let scenes = (1..=22)
            .map(|index| {
                let mut r = template.clone();
                r.index = index;
                r.byte7 = index;
                r.word14 = u16::from(index);
                r.word0 = 0;
                r.word2 = 0;
                r.byte19 = 0;
                r.byte18 = 0;
                r.byte22 = 0;
                r
            })
            .collect();
        State {
            index: 21,
            scenes,
            players: (1..=22)
                .map(|id| Player {
                    id,
                    byte8: 1,
                    flags11: 0,
                })
                .collect(),
            score: Some(Score {
                player: Attributes {
                    id: 21,
                    p8_4: 255,
                    p8_5: 0,
                    p8_7: 0,
                },
                byte6: 31,
                byte8: 1,
                bonus: 0,
                criteria: [6, 7]
                    .into_iter()
                    .map(|kind| Criterion {
                        kind,
                        criterion: 21,
                        first: Cell {
                            index: 21,
                            value: 0,
                        },
                        second: Cell {
                            index: 0,
                            value: 255,
                        },
                        bonus_flag: 0,
                    })
                    .collect(),
            }),
            target: packed(4, 10),
            phase: 45,
            remaining: 91,
            signs: [255, 1],
            sound: 0,
            gate_bounds: [100; 2],
            actor: 21,
            secondary: 22,
            active: 0,
            flag: 0,
            b6: 0,
            mode: 99,
            ball: 0,
            dx: 123,
            dy: 456,
            distance: 789,
            endpoint: 321,
            seed: 0,
        }
    }
    #[test]
    fn original_initial_kickoff_roots() {
        let source = std::fs::read_to_string(format!(
            "{}/tests/fixtures/watched_kickoff/cases.jsonl",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        for line in source.lines() {
            let c: serde_json::Value = serde_json::from_str(line).unwrap();
            let s: State = serde_json::from_value(c["input"].clone()).unwrap();
            assert_eq!(
                serde_json::to_value(advance(&s).unwrap()).unwrap(),
                c["output"]
            );
        }
    }
    #[test]
    fn readiness_scans_and_short_circuits_actual_availability() {
        let mut s = sample();
        s.scenes[0].byte19 = 1;
        let o = advance(&s).unwrap();
        assert_eq!(o.ready, 0);
        assert_eq!(o.output, s);
        assert!(o.draws.is_empty());
        s.players[0].flags11 = 2;
        assert_eq!(advance(&s).unwrap().ready, 1);
        s.players[0].flags11 = 0;
        s.players[0].byte8 = 255;
        assert_eq!(advance(&s).unwrap().ready, 1);
        s = sample();
        s.players.clear();
        s.score = None;
        s.scenes[0].byte19 = 1;
        assert!(advance(&s).unwrap_err().contains("availability"));
        s.scenes[0].byte19 = 0;
        s.scenes[20].byte19 = 2;
        s.scenes[21].byte19 = 2;
        // Root/secondary are exempt;force mode8 to skip reached score facts.
        s.index = 1;
        s.actor = 21;
        s.secondary = 22;
        s.target = packed(0, -25);
        s.ball = packed(0, -30);
        s.signs = [1; 2];
        let o = advance(&s).unwrap();
        assert_eq!(o.ready, 1);
        assert_eq!(o.output.mode, 8);
        assert!(o.draws.is_empty());
    }
    #[test]
    fn native_launch_state_and_all_twenty_two_action_stores() {
        let s = sample();
        let o = advance(&s).unwrap();
        assert_eq!(o.ready, 1);
        assert_eq!(
            (
                o.output.dx,
                o.output.dy,
                o.output.distance,
                o.output.endpoint
            ),
            (0, 1, 10, packed(0, 10))
        );
        assert_eq!(
            (
                o.output.active,
                o.output.flag,
                o.output.actor,
                o.output.secondary,
                o.output.mode,
                o.output.b6
            ),
            (1, 1, 21, 0, 6, 0)
        );
        assert!(o.output.scenes.iter().all(|r| r.byte19 == 5));
        assert_eq!(o.output.scenes[20].byte16, 5);
        assert_eq!(o.draws.len(), 1);
        assert_eq!(o.draws[0].site, "0AB9:1BF1");
    }
    #[test]
    fn orientation_signed_byte_wrap_axis_ties_and_half_distance() {
        for (target, dx, dy, distance) in [
            (packed(10, 4), 1, 0, 10),
            (packed(10, 5), 1, 1, 7),
            (packed(4, 10), 0, 1, 10),
            (packed(5, 10), 1, 2, 5),
            (packed(2, 2), 1, 2, 2),
        ] {
            let mut s = sample();
            s.target = target;
            let o = advance(&s).unwrap();
            assert_eq!(
                (o.output.dx, o.output.dy, o.output.distance),
                (dx, dy, distance)
            );
        }
        let mut s = sample();
        s.ball = packed(127, 127);
        s.target = packed(-128, -127);
        let o = advance(&s).unwrap();
        assert_eq!((o.output.dx, o.output.dy), (1, 2));
        assert_eq!(o.output.endpoint, packed(-127, -125));
        s = sample();
        s.phase = 255;
        s.remaining = 0;
        s.signs = [1, 255];
        s.target = packed(4, 10);
        // Negative target delta selects raw sign[0],independent of orientation.
        assert_eq!(advance(&s).unwrap().output.dy, 1);
        s.signs = [255, 1];
        s.scenes[20].byte7 = 1;
        assert_eq!(advance(&s).unwrap().output.dy, 65535);
        s.phase = 0;
        assert_eq!(advance(&s).unwrap().output.dy, 1);
    }
    #[test]
    fn mode_upgrades_score_success_and_failure_and_original_rng_sites() {
        let mut s = sample();
        s.target = packed(0, 30);
        let o = advance(&s).unwrap();
        assert_eq!(o.output.mode, 7);
        assert_eq!(o.draws.len(), 1);
        s = sample();
        s.index = 1;
        s.ball = packed(0, -30);
        s.target = packed(0, -25);
        s.signs = [1; 2];
        s.score = None;
        let o = advance(&s).unwrap();
        assert_eq!(o.output.mode, 8);
        assert!(o.draws.is_empty());
        s = sample();
        s.score
            .as_mut()
            .unwrap()
            .criteria
            .iter_mut()
            .for_each(|c| c.second.value = 0);
        let o = advance(&s).unwrap();
        assert!(o.draws.len() > 1);
        assert_eq!(o.draws[1].site, "0AB9:2852");
        // A changed raw role table changes native skill and therefore the actual RNG path.
        assert_ne!(o.draws.len(), advance(&sample()).unwrap().draws.len());
    }
    #[test]
    fn all_rng_retry_branches_and_signed_gate_threshold() {
        let mut sites = std::collections::BTreeSet::new();
        for target in [packed(4, 10), packed(10, 5), packed(5, 10)] {
            for seed in 0..32 {
                let mut s = sample();
                s.target = target;
                s.seed = seed;
                s.score
                    .as_mut()
                    .unwrap()
                    .criteria
                    .iter_mut()
                    .for_each(|c| c.second.value = 0);
                let o = advance(&s).unwrap();
                for d in o.draws {
                    sites.insert(d.site);
                }
            }
        }
        for site in [
            "0AB9:1BF1",
            "0AB9:2852",
            "0AB9:2868",
            "0AB9:287A",
            "0AB9:28B6",
        ] {
            assert!(sites.contains(site), "missing branch {site}");
        }
        let mut s = sample();
        s.seed = 1;
        s.gate_bounds = [65535; 2];
        s.score
            .as_mut()
            .unwrap()
            .criteria
            .iter_mut()
            .for_each(|c| c.second.value = 0);
        let o = advance(&s).unwrap();
        assert!(o.draws[0].value > 32767);
        assert_eq!(o.draws.len(), 1);
        assert_eq!(o.output.distance, 10);
        s = sample();
        s.target = packed(1, 0);
        s.score = None;
        let o = advance(&s).unwrap();
        assert_eq!((o.output.dx, o.output.dy, o.output.distance), (1, 65535, 2));
        assert!(o.draws.is_empty());
        let mut sounding = sample();
        sounding.sound = 7;
        let mut out = advance(&sounding).unwrap().output;
        out.sound = 0;
        assert_eq!(out, advance(&sample()).unwrap().output);
    }
    #[test]
    fn invalid_alias_missing_reached_facts_and_explicit_domains() {
        for mode in 0..8 {
            let mut s = sample();
            match mode {
                0 => s.active = 1,
                1 => s.index = 0,
                2 => s.actor = 23,
                3 => s.scenes[0].index = 2,
                4 => s.score.as_mut().unwrap().byte8 = 2,
                5 => s.score.as_mut().unwrap().criteria[0].first.index = 0,
                6 => s.score = None,
                _ => s.score.as_mut().unwrap().criteria.clear(),
            };
            assert!(advance(&s).is_err());
        }
        let mut s = sample();
        s.signs = [0; 2];
        assert!(advance(&s).unwrap_err().contains("divisor"));
        let v = serde_json::to_value(sample()).unwrap();
        for key in ["expected", "ready", "vector", "helper_results", "draws"] {
            let mut v = v.clone();
            v[key] = serde_json::json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
    }
    #[test]
    fn exact_whole_image_ownership() {
        let mut cases = vec![sample()];
        let mut pending = sample();
        pending.scenes[0].byte19 = 1;
        cases.push(pending);
        let source = std::fs::read_to_string(format!(
            "{}/tests/fixtures/watched_kickoff/cases.jsonl",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        for line in source.lines() {
            let c: serde_json::Value = serde_json::from_str(line).unwrap();
            cases.push(serde_json::from_value(c["input"].clone()).unwrap());
        }
        for s in cases {
            let mut e = machine(&s).unwrap();
            e.image[0x68000] = 42;
            let before = e.image.clone();
            let (ready, _) = advance_image(&mut e, s.index).unwrap();
            let mut owned = vec![];
            if ready != 0 {
                owned.extend([
                    0xD09F,
                    0xD0A0,
                    0xD0B8,
                    0xCF53,
                    0xD0B6,
                    0xD0BB,
                    0xD0A6,
                    0xD0A7,
                    0xCF4D,
                    0xCF4E,
                    0xCF4F,
                    0xCF50,
                    0xCF51,
                    0xCF52,
                    usize::from(crate::SEED),
                    usize::from(crate::SEED) + 1,
                    0xCCEF + 25 * usize::from(s.index) + 16,
                ]);
                for i in 1..=22 {
                    owned.push(0xCCEF + 25 * i + 19)
                }
            }
            for (i, (old, new)) in before.iter().zip(&e.image).enumerate() {
                if !owned.contains(&i) {
                    assert_eq!(old, new, "unexpected write {i:05X}");
                }
            }
        }
    }
}
