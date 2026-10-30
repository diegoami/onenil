//! [confirmed] 1B55:50EB and its helpers: archaeology F09 and supplemental T15/F15.
//! [designed] Only decoded raw inputs enter computation; observed helper outputs
//! and ratings remain replay assertions. This is not a whole DD02 implementation.

use crate::{Engine, Input as ContestInput, RATINGS, SEED};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub p8_4: u8,
    pub p8_6: u8,
    pub p8_7: u8,
    pub p15_6: u8,
    pub p15_8: u8,
    pub p15_11: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Staff {
    pub id: u16,
    pub byte2: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Tables {
    pub halving_group: [u8; 3],
    pub halving_flags: [[u8; 14]; 2],
    pub position_class: [u8; 14],
    pub drop_divisors: [u8; 2],
    pub home_divisors: [u8; 2],
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub club: u16,
    pub side: u8,
    pub flag: u8,
    pub club_37: u8,
    pub roster: [u16; 11],
    pub players: Vec<Player>,
    pub staff_ids: [u16; 2],
    pub staff: Vec<Staff>,
    pub tables: Tables,
    pub ratings: [u16; 8],
    pub seed: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Output {
    pub ratings: [u16; 8],
    pub seed: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlayerInput {
    pub player: Player,
    pub mode: u8,
    pub tables: Tables,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PlayerOutput {
    pub value: u16,
    pub unavailable: u8,
    pub halved: Option<u8>,
    pub halving_selectors: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BonusInput {
    pub club: u16,
    pub staff_ids: [u16; 2],
    pub staff: Vec<Staff>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BonusOutput {
    pub bonus: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PlayerObservation {
    pub input: PlayerInput,
    pub output: PlayerOutput,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BonusObservation {
    pub input: BonusInput,
    pub output: BonusOutput,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Helpers {
    pub bonus: BonusObservation,
    pub players: Vec<PlayerObservation>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Outcome {
    pub output: Output,
    pub helpers: Helpers,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Case {
    pub schema: String,
    pub week: String,
    pub call: usize,
    pub input: Input,
    pub output: Output,
    pub helpers: Helpers,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContestRequest {
    pub k: u16,
    pub seed: u16,
    pub goals: [u8; 2],
}

#[derive(Debug, Serialize)]
pub struct ComposedOutcome {
    pub sides: [Outcome; 2],
    pub contest: crate::Outcome,
}

pub fn computed_contests(
    sides: &[Input; 2],
    request: &ContestRequest,
) -> Result<ComposedOutcome, String> {
    validate_pair(sides)?;
    // [designed] The initial ratings are the legitimate BEFORE image, not
    // expected D983 ratings. Its request deliberately has no ratings field.
    let mut engine = Engine::new(&ContestInput {
        k: request.k,
        seed: request.seed,
        goals: request.goals,
        ratings: sides[0].ratings,
    });
    let first = engine.rate_side(&sides[0])?;
    let second = engine.rate_side(&sides[1])?;
    let contest = engine.quick_contests(request.k);
    Ok(ComposedOutcome {
        sides: [first, second],
        contest,
    })
}

pub fn rate_side(input: &Input) -> Result<Outcome, String> {
    let mut engine = blank_engine(input.seed, input.ratings);
    engine.rate_side(input)
}

pub fn player_value(input: &PlayerInput) -> Result<PlayerOutput, String> {
    validate_player(&input.player)?;
    validate_tables(&input.tables)?;
    let mut engine = blank_engine(0, [0; 8]);
    load_player(&mut engine, &input.player);
    load_tables(&mut engine, &input.tables);
    Ok(engine.rating_player(input.player.id, input.mode))
}

pub fn club_bonus(input: &BonusInput) -> Result<BonusOutput, String> {
    validate_bonus(input)?;
    let mut engine = blank_engine(0, [0; 8]);
    load_bonus(&mut engine, input);
    Ok(BonusOutput {
        bonus: engine.rating_bonus(input.club),
    })
}

const P15: usize = 0x20000;
const P8: usize = 0x30000;
const CLUB: usize = 0x40000;
const STAFF: usize = 0x48000;

fn blank_engine(seed: u16, ratings: [u16; 8]) -> Engine {
    Engine::new(&ContestInput {
        k: 0,
        seed,
        ratings,
        goals: [0; 2],
    })
}

fn p8(id: u16, field: usize) -> usize {
    P8 + usize::from(id) * 8 + field
}
fn p15(id: u16, field: usize) -> usize {
    P15 + usize::from(id) * 15 + field
}
fn club(id: u16, field: usize) -> usize {
    CLUB + (usize::from(id) - 1) * 62 + field
}
fn staff(id: u16) -> usize {
    STAFF + usize::from(id) * 4 + 2
}
fn word(engine: &Engine, address: usize) -> u16 {
    u16::from_le_bytes([engine.image[address], engine.image[address + 1]])
}
fn put(engine: &mut Engine, address: usize, value: u16) {
    engine.image[address..address + 2].copy_from_slice(&value.to_le_bytes());
}

fn validate_player(player: &Player) -> Result<(), String> {
    if player.id > 3920 || player.p8_6 > 13 || player.p8_7 > 13 {
        return Err("player ID/position/style outside the named-record/table domain".into());
    }
    Ok(())
}

fn validate_tables(tables: &Tables) -> Result<(), String> {
    if tables.halving_group.iter().any(|&value| value > 1)
        || tables.position_class.iter().any(|&value| value > 5)
        || tables.drop_divisors.contains(&0)
        || tables.home_divisors.contains(&0)
    {
        return Err("unsupported group/class table or zero divisor".into());
    }
    Ok(())
}

fn validate_bonus(input: &BonusInput) -> Result<(), String> {
    if !(1..=176).contains(&input.club) || input.staff_ids.iter().any(|&id| id > 1000) {
        return Err("club/staff ID outside the named-record domain".into());
    }
    let mut staff_ids = BTreeMap::new();
    for entry in &input.staff {
        if entry.id > 1000 || staff_ids.insert(entry.id, entry).is_some() {
            return Err("unsupported or duplicate staff record".into());
        }
    }
    if input.staff_ids.iter().any(|id| !staff_ids.contains_key(id)) {
        return Err("missing referenced staff record".into());
    }
    Ok(())
}

fn bonus_input(input: &Input) -> BonusInput {
    BonusInput {
        club: input.club,
        staff_ids: input.staff_ids,
        staff: input.staff.clone(),
    }
}

fn validate(input: &Input) -> Result<(), String> {
    if input.side > 1 {
        return Err("unsupported rating side".into());
    }
    validate_tables(&input.tables)?;
    validate_bonus(&bonus_input(input))?;
    let mut ids = BTreeMap::new();
    for player in &input.players {
        validate_player(player)?;
        if ids.insert(player.id, player).is_some() {
            return Err("duplicate player input".into());
        }
    }
    if input
        .roster
        .iter()
        .any(|id| *id != 0 && !ids.contains_key(id))
    {
        return Err("missing referenced roster player".into());
    }
    Ok(())
}

fn validate_pair(sides: &[Input; 2]) -> Result<(), String> {
    for side in sides {
        validate(side)?;
    }
    let [first, second] = sides;
    if first.side == second.side || first.seed != second.seed || first.tables != second.tables {
        return Err("inconsistent paired side selectors/seed/shared tables".into());
    }
    if first.club == second.club
        && (first.roster != second.roster
            || first.club_37 != second.club_37
            || first.staff_ids != second.staff_ids)
    {
        return Err("conflicting aliased club input".into());
    }
    for p in &first.players {
        if second.players.iter().any(|q| p.id == q.id && p != q) {
            return Err("conflicting aliased player input".into());
        }
    }
    for p in &first.staff {
        if second.staff.iter().any(|q| p.id == q.id && p != q) {
            return Err("conflicting aliased staff input".into());
        }
    }
    Ok(())
}

fn load_player(engine: &mut Engine, player: &Player) {
    for (offset, value) in [(4, player.p8_4), (6, player.p8_6), (7, player.p8_7)] {
        engine.image[p8(player.id, offset)] = value;
    }
    for (offset, value) in [(6, player.p15_6), (8, player.p15_8), (11, player.p15_11)] {
        engine.image[p15(player.id, offset)] = value;
    }
}

fn load_tables(engine: &mut Engine, tables: &Tables) {
    for (base, values) in [
        (0x12BA, tables.halving_group.as_slice()),
        (0x12BE, tables.position_class.as_slice()),
        (0x1286, tables.drop_divisors.as_slice()),
        (0x12CC, tables.home_divisors.as_slice()),
    ] {
        engine.image[base..base + values.len()].copy_from_slice(values);
    }
    for (i, row) in tables.halving_flags.iter().enumerate() {
        engine.image[0x129E + i * 14..0x129E + (i + 1) * 14].copy_from_slice(row);
    }
}

fn load_bonus(engine: &mut Engine, input: &BonusInput) {
    for (i, &id) in input.staff_ids.iter().enumerate() {
        put(engine, club(input.club, 0x28 + 2 * i), id);
    }
    for entry in &input.staff {
        engine.image[staff(entry.id)] = entry.byte2;
    }
}

impl Engine {
    fn ratings_snapshot(&self) -> [u16; 8] {
        std::array::from_fn(|index| self.word(RATINGS + 2 * index as u16))
    }

    fn rating_bonus(&self, id: u16) -> u16 {
        // [confirmed] 4A37 loads both bytes unsigned and returns AX, not AL.
        let a = self.image[staff(word(self, club(id, 0x28)))];
        let b = self.image[staff(word(self, club(id, 0x2A)))];
        u16::from(a >> 5)
            .wrapping_add(16)
            .wrapping_add(u16::from(b >> 4))
    }

    fn rating_selector(&self, id: u16) -> u8 {
        // [confirmed] 4E81: unsigned ID remainder, row and style byte indices.
        let row = usize::from(self.image[0x12BA + usize::from(id % 3)]);
        self.image[0x129E + row * 14 + usize::from(self.image[p8(id, 7)])]
    }

    fn rating_halving(&self, id: u16) -> (u8, Vec<u8>) {
        let pos = self.image[p8(id, 6)];
        let style = self.image[p8(id, 7)];
        let first = self.rating_selector(id);
        let mut selectors = vec![first];
        if first != 0 && [3, 7, 10].contains(&pos) {
            return (1, selectors);
        }
        let second = self.rating_selector(id);
        selectors.push(second);
        let half = (second == 0 && [4, 8, 11].contains(&pos))
            || (pos == 0 && style != 0)
            || (style == 0 && pos != 0);
        (u8::from(half), selectors)
    }

    fn rating_player(&self, id: u16, mode: u8) -> PlayerOutput {
        // [confirmed] 4F89 short-circuits 4B4A before the halving calls.
        let unavailable = crate::placement::unavailable(&crate::placement::Player {
            id,
            p15_sbyte8: self.image[p15(id, 8)] as i8,
            p15_byte11: self.image[p15(id, 11)],
        });
        if unavailable != 0 {
            return PlayerOutput {
                value: 0,
                unavailable,
                halved: None,
                halving_selectors: vec![],
            };
        }
        let mut value = i16::from(self.image[p8(id, 4)])
            .wrapping_sub(62)
            .wrapping_add(i16::from(self.image[p15(id, 8)] as i8));
        let form = self.image[p15(id, 6)] as i8;
        let style = self.image[p8(id, 7)];
        if form < 32 {
            value = value.wrapping_add(i16::from(form));
        } else if mode != 0 && style == 0 {
            // [confirmed] 4BDB IDIV truncates toward zero before the clamp.
            let divisor = i16::from(self.image[0x1287]);
            value = value.wrapping_sub(value / divisor);
        } else if style == 0 {
            value = value.wrapping_add(16);
        }
        value = value.clamp(0, 256);
        let (halved, selectors) = self.rating_halving(id);
        if halved != 0 {
            value /= 2;
        }
        PlayerOutput {
            value: value as u16,
            unavailable,
            halved: Some(halved),
            halving_selectors: selectors,
        }
    }

    /// Compute this side using the current ratings/seed handoff, without resetting it.
    pub fn rate_side(&mut self, input: &Input) -> Result<Outcome, String> {
        validate(input)?;
        if self.word(SEED) != input.seed || self.ratings_snapshot() != input.ratings {
            return Err("native rating seed/image handoff differs from raw before-state".into());
        }
        load_tables(self, &input.tables);
        for player in &input.players {
            load_player(self, player);
        }
        let bonus_input = bonus_input(input);
        load_bonus(self, &bonus_input);
        self.image[club(input.club, 0x37)] = input.club_37;
        for (slot, &id) in input.roster.iter().enumerate() {
            put(self, club(input.club, 2 * slot), id);
        }
        // [confirmed] 50EB calls the bonus helper BEFORE clearing its four words.
        let bonus = self.rating_bonus(input.club);
        for class in 0..4 {
            self.put_word(RATINGS + 4 * class + 2 * u16::from(input.side), 0);
        }
        let mut players = Vec::new();
        for slot in 0..11 {
            let id = word(self, club(input.club, 2 * slot));
            if id == 0 {
                continue;
            }
            let result = self.rating_player(id, 0);
            let raw = input
                .players
                .iter()
                .find(|player| player.id == id)
                .unwrap()
                .clone();
            let mut value = result.value;
            if value != 0 {
                value = value.wrapping_add(bonus);
            }
            let class = self.image[0x12BE + usize::from(self.image[p8(id, 6)])];
            let destinations: &[u8] = if class < 4 {
                std::slice::from_ref(&class)
            } else {
                value >>= 1;
                // [confirmed] 50EB splits with an unsigned SHR, discarding odd remainder.
                &[class - 3, class - 2]
            };
            for &destination in destinations {
                let offset = RATINGS + 4 * u16::from(destination) + 2 * u16::from(input.side);
                self.put_word(offset, self.word(offset).wrapping_add(value));
            }
            players.push(PlayerObservation {
                input: PlayerInput {
                    player: raw,
                    mode: 0,
                    tables: input.tables.clone(),
                },
                output: result,
            });
        }
        if input.flag != 0 {
            let divisor = u16::from(
                self.image[0x12CC + usize::from(self.image[club(input.club, 0x37)] == 0)],
            );
            for class in 0..4 {
                let offset = RATINGS + 4 * class + 2 * u16::from(input.side);
                let value = self.word(offset);
                self.put_word(offset, value.wrapping_add(value / divisor));
            }
        }
        Ok(Outcome {
            output: Output {
                ratings: self.ratings_snapshot(),
                seed: self.word(SEED),
            },
            helpers: Helpers {
                bonus: BonusObservation {
                    input: bonus_input,
                    output: BonusOutput { bonus },
                },
                players,
            },
        })
    }
}

pub fn replay(case: &Case) -> Result<(), String> {
    let got = rate_side(&case.input)?;
    if got.output != case.output || got.helpers != case.helpers {
        return Err(format!("{} side call {} differs", case.week, case.call));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tables() -> Tables {
        Tables {
            halving_group: [0, 1, 0],
            halving_flags: [[0; 14], [1; 14]],
            position_class: [0, 1, 1, 4, 4, 2, 2, 2, 2, 5, 5, 5, 3, 3],
            drop_divisors: [8, 16],
            home_divisors: [16, 64],
        }
    }

    fn player() -> Player {
        Player {
            id: 2,
            p8_4: 100,
            p8_6: 0,
            p8_7: 0,
            p15_6: 0,
            p15_8: 10,
            p15_11: 0,
        }
    }

    fn helper(p: Player, mode: u8) -> PlayerInput {
        PlayerInput {
            player: p,
            mode,
            tables: tables(),
        }
    }

    fn input() -> Input {
        Input {
            club: 1,
            side: 0,
            flag: 0,
            club_37: 1,
            roster: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            players: vec![player()],
            staff_ids: [0, 0],
            staff: vec![Staff { id: 0, byte2: 0 }],
            tables: tables(),
            ratings: [65535, 17, 65535, 19, 65535, 23, 65535, 29],
            seed: 123,
        }
    }

    #[test]
    fn unavailable_short_circuits_and_all_status_bits_use_guest_numbering() {
        for (id, raw8, flags) in [
            (0, 10, 0),
            (2, 0, 0),
            (2, 128, 0),
            (2, 255, 0),
            (2, 10, 0x80),
            (2, 10, 0x20),
            (2, 10, 2),
            (2, 10, 4),
        ] {
            let mut p = player();
            p.id = id;
            p.p15_8 = raw8;
            p.p15_11 = flags;
            assert_eq!(
                player_value(&helper(p, 0)).unwrap(),
                PlayerOutput {
                    value: 0,
                    unavailable: 1,
                    halved: None,
                    halving_selectors: vec![]
                }
            );
        }
        for flags in [1, 8, 16, 64] {
            let mut p = player();
            p.p15_11 = flags;
            assert_eq!(player_value(&helper(p, 0)).unwrap().unavailable, 0);
        }
    }

    #[test]
    fn raw_signed_bytes_clamp_and_mode_branches_are_distinct() {
        let mut p = player();
        p.p15_6 = 255;
        assert_eq!(player_value(&helper(p.clone(), 0)).unwrap().value, 47);
        p.p15_6 = 128;
        assert_eq!(player_value(&helper(p.clone(), 0)).unwrap().value, 0);
        p.p15_6 = 31;
        p.p8_4 = 255;
        p.p15_8 = 127;
        assert_eq!(player_value(&helper(p.clone(), 0)).unwrap().value, 256);
        p = player();
        p.p15_6 = 31;
        assert_eq!(player_value(&helper(p.clone(), 0)).unwrap().value, 79);
        p.p15_6 = 32;
        assert_eq!(player_value(&helper(p.clone(), 0)).unwrap().value, 64);
        assert_eq!(player_value(&helper(p.clone(), 255)).unwrap().value, 45);
        p.p8_4 = 0;
        p.p15_8 = 1;
        assert_eq!(player_value(&helper(p, 1)).unwrap().value, 0);
    }

    #[test]
    fn raw_selector_tables_and_short_circuit_occurrences_are_observed() {
        for pos in [3, 7, 10] {
            let mut p = player();
            p.id = 1;
            p.p8_6 = pos;
            p.p8_7 = 1;
            let got = player_value(&helper(p, 0)).unwrap();
            assert_eq!(
                (got.value, got.halved, got.halving_selectors),
                (24, Some(1), vec![1])
            );
        }
        for pos in [4, 8, 11] {
            let mut p = player();
            p.p8_6 = pos;
            p.p8_7 = 1;
            let got = player_value(&helper(p, 0)).unwrap();
            assert_eq!(
                (got.value, got.halved, got.halving_selectors),
                (24, Some(1), vec![0, 0])
            );
        }
        let mut p = player();
        p.p8_6 = 3;
        p.p8_7 = 1;
        let mut h = helper(p, 0);
        h.tables.halving_flags[0][1] = 7;
        let got = player_value(&h).unwrap();
        assert_eq!(got.halving_selectors, vec![7]);
        assert_eq!(got.halved, Some(1));
        h.tables.halving_group[2] = 1;
        h.tables.halving_flags[1][1] = 0;
        assert_eq!(player_value(&h).unwrap().halved, Some(0));
    }

    #[test]
    fn position_style_mismatches_halve_and_odd_value_truncates() {
        for (pos, style) in [(0, 1), (1, 0)] {
            let mut p = player();
            p.p8_6 = pos;
            p.p8_7 = style;
            p.p8_4 = 101;
            assert_eq!(player_value(&helper(p, 0)).unwrap().value, 24);
        }
        let mut p = player();
        p.p8_6 = 1;
        p.p8_7 = 1;
        assert_eq!(player_value(&helper(p, 0)).unwrap().value, 48);
    }

    #[test]
    fn drop_divisor_is_raw_and_truncates_signed_quotient() {
        let mut p = player();
        p.p15_6 = 32;
        let mut h = helper(p, 1);
        h.tables.drop_divisors[1] = 7;
        assert_eq!(player_value(&h).unwrap().value, 42);
    }

    #[test]
    fn bonus_supports_zero_staff_id_and_shared_staff_records() {
        let b = BonusInput {
            club: 176,
            staff_ids: [0, 0],
            staff: vec![Staff { id: 0, byte2: 255 }],
        };
        assert_eq!(club_bonus(&b).unwrap().bonus, 38);
        let b = BonusInput {
            club: 1,
            staff_ids: [4, 3],
            staff: vec![Staff { id: 3, byte2: 16 }, Staff { id: 4, byte2: 32 }],
        };
        assert_eq!(club_bonus(&b).unwrap().bonus, 18);
    }

    #[test]
    fn side_repeated_ids_skip_zero_and_preserve_other_side_seed_and_raw_records() {
        let mut i = input();
        i.roster[1] = 2;
        let before = i.clone();
        let got = rate_side(&i).unwrap();
        assert_eq!(
            got.output,
            Output {
                ratings: [128, 17, 0, 19, 0, 23, 0, 29],
                seed: 123
            }
        );
        assert_eq!(got.helpers.players.len(), 2);
        assert_eq!(got.helpers.players[0], got.helpers.players[1]);
        assert_eq!(i, before);
    }

    #[test]
    fn split_odd_values_lose_the_remainder_and_classes_are_raw() {
        let mut i = input();
        i.players[0].p8_6 = 3;
        i.players[0].p8_7 = 1;
        i.players[0].p8_4 = 101;
        let got = rate_side(&i).unwrap();
        assert_eq!(got.output.ratings, [0, 17, 32, 19, 32, 23, 0, 29]);
        i.tables.position_class[3] = 0;
        assert_eq!(rate_side(&i).unwrap().output.ratings[0], 65);
    }

    #[test]
    fn nonzero_flag_and_both_club_divisors_use_raw_tables() {
        let mut i = input();
        i.flag = 255;
        assert_eq!(rate_side(&i).unwrap().output.ratings[0], 68);
        i.club_37 = 0;
        assert_eq!(rate_side(&i).unwrap().output.ratings[0], 65);
        i.tables.home_divisors[1] = 3;
        assert_eq!(rate_side(&i).unwrap().output.ratings[0], 85);
    }

    #[test]
    fn zero_value_players_never_receive_the_club_bonus() {
        let mut i = input();
        i.players[0].p15_11 = 2;
        assert_eq!(
            rate_side(&i).unwrap().output.ratings,
            [0, 17, 0, 19, 0, 23, 0, 29]
        );
    }

    #[test]
    fn valid_raw_domain_keeps_accumulators_bounded_even_at_maximum_player_and_staff_bytes() {
        let mut i = input();
        i.roster = [2; 11];
        i.players[0].p8_4 = 255;
        i.players[0].p15_8 = 127;
        i.players[0].p15_6 = 31;
        i.staff[0].byte2 = 255;
        i.flag = 255;
        i.tables.home_divisors[0] = 1;
        // [derived] 11*(clamp256+bonus38), then the largest allowed quotient.
        assert_eq!(rate_side(&i).unwrap().output.ratings[0], 6468);
    }

    #[test]
    fn unsupported_missing_and_duplicate_raw_records_are_errors() {
        let base = input();
        let mut variants = vec![];
        let mut i = base.clone();
        i.players.clear();
        variants.push(i);
        let mut i = base.clone();
        i.players.push(i.players[0].clone());
        variants.push(i);
        let mut i = base.clone();
        i.staff.clear();
        variants.push(i);
        let mut i = base.clone();
        i.staff.push(i.staff[0].clone());
        variants.push(i);
        let mut i = base.clone();
        i.side = 2;
        variants.push(i);
        let mut i = base.clone();
        i.club = 0;
        variants.push(i);
        let mut i = base.clone();
        i.club = 177;
        variants.push(i);
        let mut i = base.clone();
        i.players[0].p8_6 = 14;
        variants.push(i);
        let mut i = base.clone();
        i.players[0].p8_7 = 14;
        variants.push(i);
        let mut i = base.clone();
        i.players[0].id = 3921;
        variants.push(i);
        let mut i = base.clone();
        i.tables.halving_group[0] = 2;
        variants.push(i);
        let mut i = base.clone();
        i.tables.position_class[0] = 6;
        variants.push(i);
        let mut i = base.clone();
        i.tables.drop_divisors[1] = 0;
        variants.push(i);
        let mut i = base.clone();
        i.tables.home_divisors[0] = 0;
        variants.push(i);
        for i in variants {
            assert!(rate_side(&i).is_err(), "accepted {i:?}");
        }
    }

    fn paired() -> ([Input; 2], ContestRequest) {
        let first = input();
        let mut second = first.clone();
        second.side = 1;
        second.ratings = [64, 17, 0, 19, 0, 23, 0, 29];
        (
            [first, second],
            ContestRequest {
                k: 0,
                seed: 123,
                goals: [7, 9],
            },
        )
    }

    #[test]
    fn paired_computation_keeps_actual_seed_goals_and_both_side_words() {
        let (pair, request) = paired();
        let got = computed_contests(&pair, &request).unwrap();
        assert_eq!(got.sides[1].output.ratings, [64, 64, 0, 0, 0, 0, 0, 0]);
        assert_eq!(got.contest.seed, 123);
        assert_eq!(got.contest.goals, [7, 9]);
        assert!(got.contest.draws.is_empty());
    }

    #[test]
    fn paired_computation_rejects_wrong_handoffs_without_reseeding_or_resetting_ratings() {
        let (pair, mut request) = paired();
        request.seed = 124;
        assert!(computed_contests(&pair, &request).is_err());
        let (mut pair, request) = paired();
        pair[1].ratings[0] = 65;
        assert!(computed_contests(&pair, &request).is_err());
        pair[1].ratings[0] = 64;
        pair[1].seed = 124;
        assert!(computed_contests(&pair, &request).is_err());
        assert!(serde_json::from_value::<ContestRequest>(serde_json::json!({
            "k": 0, "seed": 123, "goals": [7,9], "ratings": [1,2,3,4,5,6,7,8]
        }))
        .is_err());
    }

    #[test]
    fn conflicting_shared_records_and_tables_cannot_be_overwritten_between_sides() {
        let (pair, request) = paired();
        let mut variants = vec![];
        let mut p = pair.clone();
        p[1].players[0].p8_4 ^= 1;
        variants.push(p);
        let mut p = pair.clone();
        p[1].staff[0].byte2 ^= 1;
        variants.push(p);
        let mut p = pair.clone();
        p[1].club_37 ^= 1;
        variants.push(p);
        let mut p = pair.clone();
        p[1].roster[1] = 2;
        variants.push(p);
        let mut p = pair.clone();
        p[1].tables.halving_flags[0][0] ^= 1;
        variants.push(p);
        let mut p = pair.clone();
        p[1].side = 0;
        variants.push(p);
        for p in variants {
            assert!(computed_contests(&p, &request).is_err());
        }
    }

    #[test]
    fn image_updates_preserve_unmodelled_fields_and_actual_seed_goals() {
        let raw = input();
        let mut engine = Engine::new(&ContestInput {
            k: 0,
            seed: 123,
            ratings: raw.ratings,
            goals: [251, 9],
        });
        engine.image[0x1234] = 77;
        engine.image[p8(2, 5)] = 88;
        engine.image[p15(2, 2)] = 99;
        let out = engine.rate_side(&raw).unwrap();
        assert_eq!(out.output.seed, 123);
        assert_eq!(
            (
                engine.image[0x1234],
                engine.image[p8(2, 5)],
                engine.image[p15(2, 2)]
            ),
            (77, 88, 99)
        );
        let contest = engine.quick_contests(0);
        assert_eq!(contest.goals, [251, 9]);
        assert_eq!(contest.seed, 123);
    }
}
