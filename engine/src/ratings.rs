//! [confirmed] 1B55:50EB and its helpers: archaeology F09 and supplemental T15/F15.
//! [designed] Only decoded raw inputs enter computation; observed helper outputs
//! and ratings remain replay assertions. This is not a whole DD02 implementation.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
pub struct Staff {
    pub id: u16,
    pub byte2: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Tables {
    pub halving_group: [u8; 3],
    pub halving_flags: [[u8; 14]; 2],
    pub position_class: [u8; 14],
    pub drop_divisors: [u8; 2],
    pub home_divisors: [u8; 2],
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
    _sides: &[Input; 2],
    _request: &ContestRequest,
) -> Result<ComposedOutcome, String> {
    todo!("T04 actual computed ratings to D983")
}

pub fn rate_side(_input: &Input) -> Result<Outcome, String> {
    todo!("T04 native raw-input side ratings")
}

pub fn player_value(_input: &PlayerInput) -> Result<PlayerOutput, String> {
    todo!("T04 native raw-input player rating")
}

pub fn club_bonus(_input: &BonusInput) -> Result<BonusOutput, String> {
    todo!("T04 native raw-input club bonus")
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
}
