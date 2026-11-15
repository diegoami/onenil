//! [confirmed: archaeology 83b7ef7, T83/T84] Original 1998:14A5 routine contract.
//! [designed:T81] Isolated adapter; campaign/caller composition remains separate.
use crate::{layout, Draw, Engine, Input as ContestInput, SEED};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub player: u16,
    pub club: u8,
    pub age: u8,
    pub age_reference: u8,
    pub facility: u8,
    pub caps: [u8; 2],
    pub base_condition: i8,
    pub condition: i8,
    pub recovery: i8,
    pub flag: u8,
    pub seed: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Output {
    pub base_condition: i8,
    pub condition: i8,
    pub recovery: i8,
    pub seed: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Write {
    pub field: String,
    pub before: u16,
    pub after: u16,
    pub size: u8,
}

#[derive(Debug, Serialize)]
pub struct Outcome {
    pub output: Output,
    pub draws: Vec<Draw>,
    pub writes: Vec<Write>,
}

/// [designed] Own one admitted image; subsequent calls retain its live state.
pub struct Session {
    engine: Engine,
    player: u16,
}

impl Session {
    pub fn new(input: &Input) -> Result<Self, String> {
        if !(1..=3920).contains(&input.player)
            || !(1..=176).contains(&input.club)
            || input.caps != [32, 25]
        {
            return Err("condition input player/club/cap identity differs".into());
        }
        let mut engine = Engine::new(&ContestInput {
            k: 0,
            seed: input.seed,
            ratings: [0; 8],
            goals: [0; 2],
        });
        let p8 = layout::P8 + 8 * usize::from(input.player);
        let p15 = layout::P15 + 15 * usize::from(input.player);
        engine.image[p8] = input.club;
        engine.image[p8 + 1] = input.age;
        engine.image[p8 + 2] = input.age_reference;
        engine.image[p15 + 7] = input.base_condition as u8;
        engine.image[p15 + 8] = input.condition as u8;
        engine.image[p15 + 14] = input.recovery as u8;
        engine.image[0xD72..0xD74].copy_from_slice(&input.caps);
        // [designed] The isolated adapter uses staff slot zero. The image
        // kernel below resolves the actual club staff word and preserves aliases.
        let staff_word = layout::CLUB + 62 * usize::from(input.club - 1) + 44;
        engine.image[staff_word..staff_word + 2].copy_from_slice(&0u16.to_le_bytes());
        engine.image[layout::STAFF + 2] = input.facility;
        Ok(Self {
            engine,
            player: input.player,
        })
    }

    pub fn refresh(&mut self, flag: u8) -> Result<Outcome, String> {
        refresh_image(&mut self.engine, self.player, flag)
    }

    pub fn snapshot(&self) -> Output {
        project(&self.engine, self.player)
    }
}

pub fn replay(input: &Input) -> Result<Outcome, String> {
    Session::new(input)?.refresh(input.flag)
}

fn project(e: &Engine, player: u16) -> Output {
    let p15 = layout::P15 + 15 * usize::from(player);
    Output {
        base_condition: e.image[p15 + 7] as i8,
        condition: e.image[p15 + 8] as i8,
        recovery: e.image[p15 + 14] as i8,
        seed: e.word(SEED),
    }
}

fn store(e: &mut Engine, address: usize, value: u8, field: &str, writes: &mut Vec<Write>) {
    writes.push(Write {
        field: field.into(),
        before: u16::from(e.image[address]),
        after: u16::from(value),
        size: 1,
    });
    e.image[address] = value;
}

fn draw(e: &mut Engine, n: u16, site: &str, draws: &mut Vec<Draw>, writes: &mut Vec<Write>) -> i16 {
    let before = e.word(SEED);
    let value = e.draw(n, site, draws);
    writes.push(Write {
        field: "seed".into(),
        before,
        after: e.word(SEED),
        size: 2,
    });
    value as i16
}

// [confirmed: T83/T84 83b7ef7] 11EC receives only the signed low byte.
fn clamp(candidate: i16, age: u8, reference: u8, facility: u8, caps: [u8; 2]) -> u8 {
    let mut value = (candidate as i8).max(1);
    let cap = caps[usize::from(u16::from(age) > u16::from(reference) + 3)];
    value = value.min(cap as i8);
    if age > 19 {
        value = value.min((27 + (facility >> 5)) as i8);
    }
    value as u8
}

/// [confirmed: archaeology 83b7ef7] 14A5 + bounded 11EC/13F9 effects.
/// The caller owns admission and RNG sequencing; no fixture is consulted here.
pub(crate) fn refresh_image(e: &mut Engine, player: u16, flag: u8) -> Result<Outcome, String> {
    if !(1..=3920).contains(&player) {
        return Err("condition player outside original domain".into());
    }
    let p8 = layout::P8 + 8 * usize::from(player);
    let p15 = layout::P15 + 15 * usize::from(player);
    let club = e.image[p8];
    let age = e.image[p8 + 1];
    let reference = e.image[p8 + 2];
    let caps = [e.image[0xD72], e.image[0xD73]];
    let condition = e.image[p15 + 8] as i8;
    let base = i16::from(e.image[p15 + 7] as i8);
    if !(1..=176).contains(&club) || caps != [32, 25] {
        return Err("condition image club/cap identity differs".into());
    }
    let mut facility = 0;
    if age > 19 || condition <= 0 {
        let address = layout::CLUB + 62 * usize::from(club - 1) + 44;
        let staff = u16::from_le_bytes([e.image[address], e.image[address + 1]]);
        if staff > 1000 {
            return Err("condition staff outside original domain".into());
        }
        facility = e.image[layout::STAFF + 4 * usize::from(staff) + 2];
    }
    // Resolve all reached input dependencies before any mutation or RNG draw.
    // The output also projects the preserved recovery byte on positive calls.
    let recovery = e.image[p15 + 14];
    let _seed = e.word(SEED);
    e.image.check_initialized()?;
    let mut draws = Vec::new();
    let mut writes = Vec::new();
    if condition > 0 {
        let (a, b) = if flag != 0 {
            (
                draw(e, 2, "1998:14D0", &mut draws, &mut writes),
                draw(e, 8, "1998:14D8", &mut draws, &mut writes),
            )
        } else {
            (
                draw(e, 3, "1998:14FE", &mut draws, &mut writes),
                draw(e, 2, "1998:1506", &mut draws, &mut writes),
            )
        };
        let value = clamp(base + b - a, age, reference, facility, caps);
        store(e, p15 + 7, value, "base_condition", &mut writes);
        if flag == 0 && (value as i8) < 25 {
            store(
                e,
                p15 + 7,
                value.wrapping_add(1),
                "base_condition",
                &mut writes,
            );
        }
    } else {
        let a = draw(e, 2, "1998:1429", &mut draws, &mut writes);
        let b = draw(e, 8, "1998:1431", &mut draws, &mut writes);
        let value = clamp(base + a - b, age, reference, facility, caps);
        store(e, p15 + 7, value, "base_condition", &mut writes);
        let decrement = draw(
            e,
            u16::from((facility >> 5) + 1),
            "1998:147B",
            &mut draws,
            &mut writes,
        );
        let value = recovery.wrapping_sub(decrement as u8);
        store(e, p15 + 14, value, "recovery", &mut writes);
        if (value as i8) < 0 {
            store(e, p15 + 14, 0, "recovery", &mut writes);
        }
    }
    if condition > 0 || e.image[p15 + 14] == 0 {
        let value = e.image[p15 + 7];
        store(e, p15 + 8, value, "condition", &mut writes);
    }
    Ok(Outcome {
        output: project(e, player),
        draws,
        writes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Input {
        Input {
            player: 3920,
            club: 176,
            age: 25,
            age_reference: 25,
            facility: 128,
            caps: [32, 25],
            base_condition: 40,
            condition: 10,
            recovery: 0,
            flag: 1,
            seed: 7,
        }
    }

    #[test]
    fn image_resolves_the_actual_club_staff_word_and_pool_alias() {
        let s = input();
        let expected = replay(&s).unwrap();
        let mut session = Session::new(&s).unwrap();
        let word = layout::CLUB + 62 * 175 + 44;
        session.engine.image[word..word + 2].copy_from_slice(&1000u16.to_le_bytes());
        session.engine.image[layout::STAFF + 2] = 0;
        session.engine.image[layout::STAFF + 4 * 1000 + 2] = s.facility;
        let actual = session.refresh(s.flag).unwrap();
        assert_eq!(actual.output, expected.output);
        assert_eq!(actual.draws, expected.draws);
        assert_eq!(actual.writes, expected.writes);
    }

    #[test]
    fn bad_image_admission_does_not_mutate_fields_or_seed() {
        for damage in 0..4 {
            let s = input();
            let mut session = Session::new(&s).unwrap();
            if damage == 0 {
                session.engine.image[layout::P8 + 8 * 3920] = 0;
            }
            if damage == 1 {
                session.engine.image[0xD72] = 31;
            }
            if damage == 2 {
                let word = layout::CLUB + 62 * 175 + 44;
                session.engine.image[word..word + 2].copy_from_slice(&1001u16.to_le_bytes());
            }
            let before = session.engine.image.iter().copied().collect::<Vec<_>>();
            let player = if damage == 3 { 3921 } else { s.player };
            assert!(refresh_image(&mut session.engine, player, s.flag).is_err());
            assert_eq!(
                session.engine.image.iter().copied().collect::<Vec<_>>(),
                before
            );
        }
    }

    #[test]
    fn a_missing_live_seed_refuses_before_any_mutation() {
        let s = input();
        let mut session = Session::new(&s).unwrap();
        let admitted = session.engine.image.clone();
        session.engine.image = crate::native_image::Image::tracked(layout::IMAGE_LEN);
        let p8 = layout::P8 + 8 * 3920;
        let p15 = layout::P15 + 15 * 3920;
        let club = layout::CLUB + 62 * 175 + 44;
        for (start, count) in [
            (p8, 3),
            (p15 + 7, 2),
            (p15 + 14, 1),
            (0xD72, 2),
            (club, 2),
            (layout::STAFF + 2, 1),
        ] {
            session.engine.image[start..start + count]
                .copy_from_slice(&admitted[start..start + count]);
        }
        let before = session.engine.image.iter().copied().collect::<Vec<_>>();
        assert!(session
            .refresh(s.flag)
            .unwrap_err()
            .contains("uninitialized"));
        assert_eq!(
            session.engine.image.iter().copied().collect::<Vec<_>>(),
            before
        );
    }

    #[test]
    fn prior_native_rng_consumption_flows_into_the_refresh() {
        let s = input();
        let mut session = Session::new(&s).unwrap();
        let mut earlier = Vec::new();
        session
            .engine
            .draw(9, "synthetic preceding native call", &mut earlier);
        let actual = session.refresh(s.flag).unwrap();
        assert_eq!(actual.draws[0].seed_before, earlier[0].seed_after);
        assert_ne!(actual.draws[0].seed_before, s.seed);
    }
}
