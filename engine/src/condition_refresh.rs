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

// [designed:T83] A shared transition over explicit fields. The Cpu backend
// resolves live far pointers; the isolated backend retains its canonical layout.
#[derive(Clone, Copy)]
pub(crate) enum Field {
    Player8(u16, u8),
    Player15(u16, u8),
    Cap(u8),
    ClubStaff(u8),
    Facility(u16),
    Seed,
}
pub(crate) trait Memory {
    fn read(&mut self, field: Field) -> Result<u16, String>;
    fn write(&mut self, field: Field, value: u8, site: u16) -> Result<(), String>;
    fn draw(&mut self, n: u16, site: u16) -> Result<i16, String>;
}
struct Canonical<'a> {
    e: &'a mut Engine,
    draws: Vec<Draw>,
    writes: Vec<Write>,
}
fn address(field: Field) -> (usize, u8, &'static str) {
    match field {
        Field::Player8(p, o) => (
            layout::P8 + 8 * usize::from(p) + usize::from(o),
            1,
            "player8",
        ),
        Field::Player15(p, o) => (
            layout::P15 + 15 * usize::from(p) + usize::from(o),
            1,
            match o {
                7 => "base_condition",
                8 => "condition",
                14 => "recovery",
                _ => "player15",
            },
        ),
        Field::Cap(o) => (0xD72 + usize::from(o), 1, "cap"),
        Field::ClubStaff(c) => (layout::CLUB + 62 * usize::from(c - 1) + 44, 2, "staff"),
        Field::Facility(s) => (layout::STAFF + 4 * usize::from(s) + 2, 1, "facility"),
        Field::Seed => (usize::from(SEED), 2, "seed"),
    }
}
impl Memory for Canonical<'_> {
    fn read(&mut self, field: Field) -> Result<u16, String> {
        let (a, width, _) = address(field);
        let value = if width == 2 {
            u16::from_le_bytes([self.e.image[a], self.e.image[a + 1]])
        } else {
            u16::from(self.e.image[a])
        };
        self.e.image.check_initialized()?;
        Ok(value)
    }
    fn write(&mut self, field: Field, value: u8, _site: u16) -> Result<(), String> {
        let (a, _, name) = address(field);
        self.writes.push(Write {
            field: name.into(),
            before: u16::from(self.e.image[a]),
            after: u16::from(value),
            size: 1,
        });
        self.e.image[a] = value;
        Ok(())
    }
    fn draw(&mut self, n: u16, site: u16) -> Result<i16, String> {
        let before = self.e.word(SEED);
        let value = self.e.draw(n, &format!("1998:{site:04X}"), &mut self.draws);
        self.writes.push(Write {
            field: "seed".into(),
            before,
            after: self.e.word(SEED),
            size: 2,
        });
        Ok(value as i16)
    }
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
/// [confirmed: archaeology83b7ef7] The one shared 14A5/11EC/13F9 algorithm.
/// Preflight reached dependencies; output-only recovery projection belongs to
/// the isolated API, not the positive-path caller's semantic read set.
pub(crate) fn apply(m: &mut impl Memory, player: u16, flag: u8) -> Result<(), String> {
    if !(1..=3920).contains(&player) {
        return Err("condition player outside original domain".into());
    }
    let club = m.read(Field::Player8(player, 0))? as u8;
    let age = m.read(Field::Player8(player, 1))? as u8;
    let reference = m.read(Field::Player8(player, 2))? as u8;
    let caps = [m.read(Field::Cap(0))? as u8, m.read(Field::Cap(1))? as u8];
    let condition = m.read(Field::Player15(player, 8))? as i8;
    let base = i16::from(m.read(Field::Player15(player, 7))? as i8);
    if !(1..=176).contains(&club) || caps != [32, 25] {
        return Err("condition image club/cap identity differs".into());
    }
    let mut facility = 0;
    if age > 19 || condition <= 0 {
        let staff = m.read(Field::ClubStaff(club))?;
        if staff > 1000 {
            return Err("condition staff outside original domain".into());
        }
        facility = m.read(Field::Facility(staff))? as u8;
    }
    let recovery = if condition <= 0 {
        m.read(Field::Player15(player, 14))? as u8
    } else {
        0
    };
    m.read(Field::Seed)?;
    if condition > 0 {
        let (a, b) = if flag != 0 {
            (m.draw(2, 0x14D0)?, m.draw(8, 0x14D8)?)
        } else {
            (m.draw(3, 0x14FE)?, m.draw(2, 0x1506)?)
        };
        let value = clamp(base + b - a, age, reference, facility, caps);
        m.write(
            Field::Player15(player, 7),
            value,
            if flag != 0 { 0x14EE } else { 0x151C },
        )?;
        if flag == 0 && (value as i8) < 25 {
            m.write(Field::Player15(player, 7), value.wrapping_add(1), 0x1527)?;
        }
    } else {
        let a = m.draw(2, 0x1429)?;
        let b = m.draw(8, 0x1431)?;
        let value = clamp(base + a - b, age, reference, facility, caps);
        m.write(Field::Player15(player, 7), value, 0x1449)?;
        let decrement = m.draw(u16::from((facility >> 5) + 1), 0x147B)?;
        let value = recovery.wrapping_sub(decrement as u8);
        m.write(Field::Player15(player, 14), value, 0x147E)?;
        if (value as i8) < 0 {
            m.write(Field::Player15(player, 14), 0, 0x1489)?;
        }
    }
    if condition > 0 || m.read(Field::Player15(player, 14))? == 0 {
        let value = m.read(Field::Player15(player, 7))? as u8;
        m.write(Field::Player15(player, 8), value, 0x1546)?;
    }
    Ok(())
}
pub(crate) fn refresh_image(e: &mut Engine, player: u16, flag: u8) -> Result<Outcome, String> {
    if !(1..=3920).contains(&player) {
        return Err("condition player outside original domain".into());
    }
    // This API returns recovery even when the transition preserves it.
    let _recovery = e.image[layout::P15 + 15 * usize::from(player) + 14];
    e.image.check_initialized()?;
    let mut m = Canonical {
        e,
        draws: vec![],
        writes: vec![],
    };
    apply(&mut m, player, flag)?;
    Ok(Outcome {
        output: project(m.e, player),
        draws: m.draws,
        writes: m.writes,
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
