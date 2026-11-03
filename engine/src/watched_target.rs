//! [confirmed] F39,c7637bd;561E and helper listings at static d9e20a4.
//! [designed] Action-one preparation/numeric effects;not a full watched tick.
use crate::{goal_effects::Commentary, terminal::Scene, Draw, Engine};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub index: u8,
    pub slot: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Row {
    pub index: u16,
    pub code: u8,
    pub first_player: u16,
    pub second_player: u16,
    pub side: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Numeric {
    pub clubs: [u16; 2],
    pub rosters: [[u16; 20]; 2],
    pub weight: u8,
    pub weighted_words: Vec<u16>,
    pub event_counts: [u16; 2],
    pub source_pair: [u16; 2],
    pub target_pair: [u16; 2],
    pub head: u16,
    pub rows: Vec<Row>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub scene: Scene,
    pub actor: u8,
    pub secondary: u8,
    pub role: u8,
    pub role_offset: u8,
    pub seed: u16,
    pub flag: u8,
    pub hold: u8,
    pub phase: u8,
    pub remaining: u16,
    pub direction_signs: [u8; 2],
    pub side_offsets: [u8; 2],
    pub special_target: u16,
    pub actor_target: u16,
    pub identities: Vec<Identity>,
    pub numeric: Numeric,
}
#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: State,
    pub draws: Vec<Draw>,
    pub commentary: Option<Commentary>,
}
fn scene(index: u8) -> usize {
    0xCCEF + 25 * usize::from(index)
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn next(head: u16) -> u16 {
    if head == 1700 {
        1
    } else {
        head + 1
    }
}
fn machine(s: &State) -> Result<Engine, String> {
    if !(1..=22).contains(&s.scene.index)
        || s.scene.byte7 > 22
        || s.scene.word14 > 3920
        || !(1..=22).contains(&s.actor)
        || s.secondary > 22
    {
        return Err("unsupported target identity".into());
    }
    let n = &s.numeric;
    if n.weighted_words.len() != 41
        || n.head > 1700
        || n.clubs.iter().any(|c| !(1..=1000).contains(c))
    {
        return Err("unsupported numeric commentary state".into());
    }
    if n.clubs[0] == n.clubs[1] && n.rosters[0] != n.rosters[1] {
        return Err("conflicting roster aliases".into());
    }
    let required = [n.head, next(n.head), next(next(n.head))];
    if n.rows.len() != 3
        || required
            .iter()
            .any(|i| n.rows.iter().filter(|r| r.index == *i).count() != 1)
    {
        return Err("missing or aliased queue rows".into());
    }
    let mut e = crate::role_scores::image(Default::default());
    let a = scene(s.scene.index);
    e.image[a..a + 25].copy_from_slice(&s.scene.bytes());
    for identity in &s.identities {
        if !(1..=22).contains(&identity.index) || identity.slot > 22 {
            return Err("unsupported slot identity".into());
        }
        let i = usize::from(identity.index);
        if e.image[crate::layout::WATCHED_IDENTITY_PRESENT + i] != 0 {
            return Err("duplicate scene identity".into());
        }
        if identity.index == s.scene.index && identity.slot != s.scene.byte7 {
            return Err("conflicting scene identity alias".into());
        }
        e.image[crate::layout::WATCHED_IDENTITY_PRESENT + i] = 1;
        e.image[scene(identity.index) + 7] = identity.slot;
    }
    // The full root scene itself is also a raw identity witness.
    e.image[crate::layout::WATCHED_IDENTITY_PRESENT + usize::from(s.scene.index)] = 1;
    e.image[crate::layout::P8 + 8 * usize::from(s.scene.word14) + 6] = s.role;
    put(&mut e, 0xAB8, s.actor_target);
    put(&mut e, 0xABA, s.special_target);
    let role_at = 0xAAA + usize::from(s.role);
    if (0xAB8..0xABC).contains(&role_at) && e.image[role_at] != s.role_offset {
        return Err("conflicting role table/target alias".into());
    }
    e.image[role_at] = s.role_offset;
    e.image[0xD09F] = s.actor;
    e.image[0xD0A0] = s.secondary;
    e.image[0xCF53] = s.flag;
    e.image[0xD0B9] = s.hold;
    e.image[0xD0AC] = s.phase;
    put(&mut e, 0xD0AE, s.remaining);
    e.image[0xA338..0xA33A].copy_from_slice(&s.direction_signs);
    e.image[0xD56..0xD58].copy_from_slice(&s.side_offsets);
    put(&mut e, crate::SEED as usize, s.seed);
    e.image[0x97A + 49] = n.weight;
    for (i, v) in n.weighted_words.iter().enumerate() {
        put(&mut e, 0xD146 + 2 * i, *v)
    }
    for side in 0..2 {
        put(
            &mut e,
            if side == 0 { 0xD6E4 } else { 0xD6E6 },
            n.clubs[side],
        );
        put(&mut e, 0xD1C0 + 2 * side, n.event_counts[side]);
        for (i, v) in n.rosters[side].iter().enumerate() {
            put(
                &mut e,
                crate::layout::CLUB + 62 * (usize::from(n.clubs[side]) - 1) + 2 * i,
                *v,
            )
        }
    }
    for (a, v) in [
        (0xD08A, n.source_pair[0]),
        (0xD08C, n.source_pair[1]),
        (0xD09A, n.target_pair[0]),
        (0xD09C, n.target_pair[1]),
        (0x380, n.head),
    ] {
        put(&mut e, a, v)
    }
    for row in &n.rows {
        let a = 0xA52A + 6 * usize::from(row.index);
        e.image[a] = row.code;
        put(&mut e, a + 1, row.first_player);
        put(&mut e, a + 3, row.second_player);
        e.image[a + 5] = row.side;
    }
    Ok(e)
}
fn identity(e: &Engine, index: u8) -> Result<u16, String> {
    if e.image[crate::layout::WATCHED_IDENTITY_PRESENT + usize::from(index)] == 0 {
        return Err("missing reached commentary identity".into());
    }
    let slot = e.image[scene(index) + 7];
    Ok(u16::from(if slot > 11 { slot - 11 } else { slot }))
}
pub(crate) fn orient(e: &Engine, index: u8, target: u16) -> u16 {
    let (sign, _) = crate::terminal::direction_image(e, &Scene::read(e, index));
    let bytes = target
        .to_le_bytes()
        .map(|b| (i16::from(b as i8) * i16::from(sign as i8)) as u8);
    u16::from_le_bytes(bytes)
}
pub(crate) fn advance_image(e: &mut Engine, index: u8) -> Result<Option<Commentary>, String> {
    if !(1..=22).contains(&index) {
        return Err("unsupported watched target scene".into());
    }
    let a = scene(index);
    let id = word(e, a + 14);
    let role = e.image[crate::layout::P8 + 8 * usize::from(id) + 6];
    let offset = e.image[0xAAA + usize::from(role)];
    let base = word(e, a + 10);
    put(e, a + 8, base);
    e.image[a + 13] = offset;
    e.image[a + 9] = e.image[a + 9].wrapping_add(offset);
    let target = orient(e, index, word(e, a + 8));
    put(e, a + 8, target);
    let actor = e.image[0xD09F];
    let slot = e.image[a + 7];
    let side = usize::from(actor < 12);
    if u16::from(slot) == 11 + u16::from(e.image[0xD56 + side]) {
        let target = orient(e, index, word(e, 0xABA));
        put(e, a + 8, target);
    }
    if actor == slot {
        let target = orient(e, index, word(e, 0xAB8));
        put(e, a + 8, target);
    }
    if e.image[0xD0B9] != 0 || word(e, a) != word(e, a + 8) {
        return Ok(None);
    }
    if actor != slot {
        e.image[a + 19] = 0;
        return Ok(None);
    }
    e.image[a + 19] = 2;
    e.image[0xCF53] = 0;
    let primary = identity(e, actor)?;
    let secondary = e.image[0xD0A0];
    let other = if secondary == 0 {
        0
    } else {
        identity(e, secondary)?
    };
    Ok(Some(crate::goal_effects::numeric_commentary(
        e, 49, primary, other, side as u8,
    )))
}
fn snapshot(e: &Engine, s: &State) -> State {
    let mut out = s.clone();
    out.scene = Scene::read(e, s.scene.index);
    out.seed = word(e, crate::SEED as usize);
    out.flag = e.image[0xCF53];
    let n = &mut out.numeric;
    n.head = word(e, 0x380);
    for (i, v) in n.weighted_words.iter_mut().enumerate() {
        *v = word(e, 0xD146 + 2 * i)
    }
    for (i, v) in n.event_counts.iter_mut().enumerate() {
        *v = word(e, 0xD1C0 + 2 * i)
    }
    n.target_pair = [word(e, 0xD09A), word(e, 0xD09C)];
    for row in &mut n.rows {
        let a = 0xA52A + 6 * usize::from(row.index);
        row.code = e.image[a];
        row.first_player = word(e, a + 1);
        row.second_player = word(e, a + 3);
        row.side = e.image[a + 5];
    }
    out
}
pub fn advance(s: &State) -> Result<Outcome, String> {
    let mut e = machine(s)?;
    let commentary = advance_image(&mut e, s.scene.index)?;
    Ok(Outcome {
        output: snapshot(&e, s),
        draws: vec![],
        commentary,
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
        let mut s: State = serde_json::from_value(c["input"].clone()).unwrap();
        s.scene.index = 1;
        s.scene.byte7 = 1;
        s.scene.word14 = 1;
        s.scene.word0 = 0;
        s.scene.word10 = packed(3, 4);
        s.scene.byte19 = 1;
        s.actor = 2;
        s.secondary = 3;
        s.role = 1;
        s.role_offset = 2;
        s.seed = 123;
        s.flag = 1;
        s.hold = 0;
        s.phase = 45;
        s.remaining = 91;
        s.direction_signs = [1, 1];
        s.side_offsets = [0, 0];
        s.special_target = packed(5, 6);
        s.actor_target = packed(7, 8);
        s.identities = vec![
            Identity { index: 1, slot: 1 },
            Identity { index: 2, slot: 2 },
            Identity { index: 3, slot: 14 },
        ];
        s.numeric.weight = 254;
        s.numeric.weighted_words = vec![0; 41];
        s.numeric.event_counts = [0; 2];
        s.numeric.head = 0;
        s.numeric.rows = (0..=2)
            .map(|index| Row {
                index,
                code: 255,
                first_player: 0,
                second_player: 0,
                side: 0,
            })
            .collect();
        s.numeric.source_pair = [123, 456];
        s.numeric.target_pair = [789, 321];
        s
    }
    #[test]
    fn original_target_parent_calls() {
        for name in ["cases.jsonl", "admission-cases.jsonl"] {
            let source = std::fs::read_to_string(format!(
                "{}/tests/fixtures/watched_target/{name}",
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
    }
    #[test]
    fn orientation_wrap_and_original_override_order() {
        let mut s = sample();
        let o = advance(&s).unwrap();
        assert_eq!(o.output.scene.word8, packed(3, 6));
        assert_eq!(o.output.scene.word12 >> 8, 2);
        s.scene.word10 = packed(-128, 127);
        s.role_offset = 1;
        s.direction_signs = [255; 2];
        assert_eq!(advance(&s).unwrap().output.scene.word8, packed(-128, -128));
        s = sample();
        s.direction_signs = [255, 1];
        s.remaining = 0;
        s.phase = 255;
        assert_eq!(advance(&s).unwrap().output.scene.word8, packed(-3, -6));
        s.scene.byte7 = 12;
        s.identities[0].slot = 12;
        assert_eq!(advance(&s).unwrap().output.scene.word8, packed(3, 6));
        s = sample();
        s.scene.byte7 = 11;
        s.identities[0].slot = 11;
        assert_eq!(advance(&s).unwrap().output.scene.word8, s.special_target);
        s.actor = 1;
        assert_eq!(advance(&s).unwrap().output.scene.word8, s.special_target); // actor index1 does not equal slot11.
    }
    #[test]
    fn selector_matches_slot_not_index_and_actor_override_wins() {
        let mut s = sample();
        s.scene.byte7 = 11;
        s.actor = 11;
        s.identities = vec![
            Identity { index: 1, slot: 11 },
            Identity {
                index: 11,
                slot: 22,
            },
            Identity { index: 3, slot: 14 },
        ];
        assert_eq!(advance(&s).unwrap().output.scene.word8, s.actor_target);
        s.actor = 12;
        s.identities.push(Identity {
            index: 12,
            slot: 12,
        });
        s.side_offsets = [5, 0];
        s.scene.byte7 = 16;
        s.identities[0].slot = 16;
        assert_eq!(advance(&s).unwrap().output.scene.word8, s.special_target);
    }
    fn arrived() -> State {
        let mut s = sample();
        s.scene.byte7 = 2;
        s.identities[0].slot = 2;
        s.scene.word0 = s.actor_target;
        s
    }
    #[test]
    fn at_target_hold_and_actor_action_transitions() {
        let mut s = arrived();
        let o = advance(&s).unwrap();
        assert_eq!(o.output.scene.byte19, 2);
        assert_eq!(o.output.flag, 0);
        let c = o.commentary.unwrap();
        assert_eq!(
            (
                c.code,
                c.actor_slot,
                c.secondary_slot,
                c.event_side,
                c.accepted
            ),
            (49, 2, 3, 1, 1)
        );
        s.hold = 1;
        let o = advance(&s).unwrap();
        assert_eq!(o.output.scene.byte19, 1);
        assert_eq!(o.output.flag, 1);
        assert!(o.commentary.is_none());
        s = sample();
        s.scene.word0 = packed(3, 6);
        let o = advance(&s).unwrap();
        assert_eq!(o.output.scene.byte19, 0);
        assert_eq!(o.output.flag, 1);
        assert!(o.commentary.is_none());
        s.scene.word0 = 1;
        assert_eq!(advance(&s).unwrap().output.scene.byte19, 1);
    }
    #[test]
    fn code49_queue_weight_identity_dedup_and_wrap() {
        let s = arrived();
        let o = advance(&s).unwrap();
        assert_eq!(o.output.numeric.weighted_words[22], 65534);
        assert_eq!(o.output.numeric.event_counts, [0, 1]);
        assert_eq!(o.output.numeric.target_pair, s.numeric.source_pair);
        let row = o.output.numeric.rows.iter().find(|r| r.index == 1).unwrap();
        assert_eq!(row.first_player, s.numeric.rosters[1][1]);
        assert_eq!(row.second_player, s.numeric.rosters[1][2]);
        let mut duplicate = o.output.clone();
        duplicate.scene.byte19 = 1;
        duplicate.numeric.weighted_words[22] = 1;
        // Rotate witnesses to head1,next2,sentinel3 before a new root call.
        duplicate.numeric.rows.retain(|r| r.index != 0);
        duplicate.numeric.rows.push(Row {
            index: 3,
            code: 99,
            first_player: 0,
            second_player: 0,
            side: 0,
        });
        let o = advance(&duplicate).unwrap();
        assert_eq!(o.commentary.unwrap().accepted, 0);
        assert_eq!(o.output.numeric.head, 1);
        assert_eq!(o.output.numeric.event_counts, [0, 1]);
        assert_eq!(o.output.numeric.weighted_words[22], 65535);
        let mut s = arrived();
        s.secondary = 0;
        s.identities.retain(|i| i.index != 3);
        s.numeric.head = 1700;
        s.numeric.event_counts[1] = 65535;
        s.numeric
            .rows
            .iter_mut()
            .find(|r| r.index == 0)
            .unwrap()
            .index = 1700;
        let o = advance(&s).unwrap();
        assert_eq!(o.commentary.unwrap().secondary_slot, 0);
        assert_eq!(o.output.numeric.head, 1);
        assert_eq!(o.output.numeric.event_counts[1], 0);
        assert_eq!(
            o.output
                .numeric
                .rows
                .iter()
                .find(|r| r.index == 2)
                .unwrap()
                .code,
            255
        );
    }
    #[test]
    fn commentary_uses_actor_scene_identity_and_zero_slots_on_opposite_side() {
        let mut s = sample();
        s.actor = 12;
        s.secondary = 3;
        s.scene.byte7 = 12;
        s.scene.word0 = s.actor_target;
        s.identities = vec![
            Identity { index: 1, slot: 12 },
            Identity {
                index: 12,
                slot: 22,
            },
            Identity { index: 3, slot: 0 },
        ];
        s.numeric.weight = 3;
        s.numeric.weighted_words[11] = 65535;
        let o = advance(&s).unwrap();
        let c = o.commentary.unwrap();
        assert_eq!((c.actor_slot, c.secondary_slot, c.event_side), (11, 0, 0));
        assert_eq!(o.output.numeric.weighted_words[11], 2);
        assert_eq!(
            o.output
                .numeric
                .rows
                .iter()
                .find(|r| r.index == 1)
                .unwrap()
                .first_player,
            s.numeric.rosters[0][10]
        );
    }
    #[test]
    fn strict_inputs_aliases_and_reached_missing_identities() {
        let s = sample();
        let source = serde_json::to_value(&s).unwrap();
        for field in [
            "expected",
            "target",
            "commentary",
            "output",
            "helper_results",
        ] {
            let mut v = source.clone();
            v[field] = serde_json::json!(1);
            assert!(serde_json::from_value::<State>(v).is_err());
        }
        let mut v = source.clone();
        v["numeric"]["accepted"] = serde_json::json!(1);
        assert!(serde_json::from_value::<State>(v).is_err());
        for mode in 0..6 {
            let mut s = sample();
            match mode {
                0 => s.actor = 23,
                1 => s.scene.word14 = 3921,
                2 => s.identities[0].slot = 2,
                3 => s.numeric.weighted_words.pop().map(|_| ()).unwrap(),
                4 => s.numeric.rows.push(s.numeric.rows[0].clone()),
                _ => {
                    s.role = 14;
                    s.role_offset = 255
                }
            };
            assert!(advance(&s).is_err());
        }
        let mut s = arrived();
        s.identities.retain(|i| i.index != 2);
        assert!(advance(&s).unwrap_err().contains("identity"));
        s.hold = 1;
        assert!(advance(&s).is_ok());
    }
    fn check_ownership(s: &State, event: Option<&Commentary>) {
        let mut e = machine(s).unwrap();
        e.image[0x68000] = 42;
        let before = e.image.clone();
        let a = scene(s.scene.index);
        advance_image(&mut e, s.scene.index).unwrap();
        let mut owned = vec![a + 8, a + 9, a + 13, a + 19, 0xCF53];
        if let Some(c) = event {
            let w = 0xD146 + 40 * usize::from(c.event_side) + 2 * usize::from(c.actor_slot);
            owned.extend([w, w + 1]);
            if c.accepted != 0 {
                let count = 0xD1C0 + 2 * usize::from(c.event_side);
                owned.extend([
                    count,
                    count + 1,
                    0x380,
                    0x381,
                    0xD09A,
                    0xD09B,
                    0xD09C,
                    0xD09D,
                ]);
                let row = 0xA52A + 6 * usize::from(next(s.numeric.head));
                owned.extend(row..row + 6);
                owned.push(0xA52A + 6 * usize::from(next(next(s.numeric.head))));
            }
        }
        for (i, (old, new)) in before.iter().zip(&e.image).enumerate() {
            if !owned.contains(&i) {
                assert_eq!(old, new, "unexpected write at {i:05X}");
            }
        }
        assert_eq!(e.word(crate::SEED), s.seed);
    }
    #[test]
    fn whole_image_write_ownership() {
        check_ownership(&sample(), None);
        check_ownership(
            &arrived(),
            Some(&Commentary {
                code: 49,
                actor_slot: 2,
                secondary_slot: 3,
                event_side: 1,
                accepted: 1,
            }),
        );
        for name in ["cases.jsonl", "admission-cases.jsonl"] {
            let source = std::fs::read_to_string(format!(
                "{}/tests/fixtures/watched_target/{name}",
                env!("CARGO_MANIFEST_DIR")
            ))
            .unwrap();
            for line in source.lines() {
                let c: serde_json::Value = serde_json::from_str(line).unwrap();
                let s: State = serde_json::from_value(c["input"].clone()).unwrap();
                let event: Option<Commentary> =
                    serde_json::from_value(c["output"]["commentary"].clone()).unwrap();
                check_ownership(&s, event.as_ref());
            }
        }
    }
}
