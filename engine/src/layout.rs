//! [designed] T24 canonical logical pools shared by every native slice.
//! These are host image offsets,not claims about original physical far aliases.
pub(crate) const IMAGE_LEN: usize = 0x70000;
pub(crate) const P8: usize = 0x10000;
pub(crate) const P15: usize = 0x20000;
pub(crate) const CLUB: usize = 0x30000;
pub(crate) const PAIR_OWN: usize = 0x40000;
pub(crate) const PAIR_OPP: usize = 0x40010;
pub(crate) const PAIR_ASSIGNED: usize = 0x40020;
pub(crate) const SCENE_MODE: usize = 0x40080;
// [designed] Decoded watched occupancy cells and admission-presence metadata.
pub(crate) const PITCH: usize = 0x41000;
pub(crate) const PITCH_PRESENT: usize = 0x42000;
pub(crate) const PITCH_CELLS: usize = 17 * 75;
// [confirmed] Ratings4A37 and assignment4A37 share original DCC8.
pub(crate) const STAFF: usize = CLUB_COUNTS;
pub(crate) const CONFIG: usize = 0x49000;
pub(crate) const SPEED: usize = CONFIG + 2;
pub(crate) const CLUB_COUNTS: usize = 0x50000;
pub(crate) const HISTORY: usize = 0x60000;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn logical_pools_are_disjoint_and_domain_endpoints_fit() {
        let ranges = [
            (0, 0x10000),
            (P8, P8 + 0x10000),
            (P15, P15 + 0x10000),
            (CLUB, CLUB + 0x10000),
            (PAIR_OWN, PAIR_OWN + 11),
            (PAIR_OPP, PAIR_OPP + 11),
            (PAIR_ASSIGNED, PAIR_ASSIGNED + 11),
            (SCENE_MODE, SCENE_MODE + 1),
            (PITCH, PITCH + 2 * PITCH_CELLS),
            (PITCH_PRESENT, PITCH_PRESENT + PITCH_CELLS),
            (CONFIG, CONFIG + 256),
            (CLUB_COUNTS, CLUB_COUNTS + 0x10000),
            (HISTORY, HISTORY + 0x10000),
        ];
        for (i, (start, end)) in ranges.iter().copied().enumerate() {
            assert!(start < end && end <= IMAGE_LEN);
            for (other_start, other_end) in ranges.iter().copied().skip(i + 1) {
                assert!(end <= other_start || other_end <= start);
            }
        }
        assert!(P8 + 8 * 3920 + 7 < P8 + 0x10000);
        assert!(P15 + 15 * 3920 + 14 < P15 + 0x10000);
        assert!(CLUB + 62 * 175 + 61 < CLUB + 0x10000);
        assert_eq!(STAFF, CLUB_COUNTS);
        assert!(STAFF + 4 * 1000 + 2 < CLUB_COUNTS + 0x10000);
        assert_eq!(SPEED, CONFIG + 2);
        assert_eq!(HISTORY + 0x10000, IMAGE_LEN);
    }
}
