//! [designed:T80] The authenticated friendly-week receipt (research T81/T82),
//! pinned by digest: the one observed week's cash movement, transfer rows and
//! following fixture. No value here generalises beyond that week.
use serde::Deserialize;
use sha2::{Digest, Sha256};

pub const WEEK_RECEIPT: &str = include_str!("../assets/week-receipt.json");
pub const WEEK_RECEIPT_SHA256: &str =
    "c2e5b0b54da56e3b037f52cb4f2612bf4d08a66897c6392fe5fd0b973dd0f381";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransferRow {
    pub player: String,
    pub from: String,
    pub to: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FollowingFixture {
    pub competition: u8,
    pub opponent: u16,
    pub flags: u8,
    pub name: String,
    pub manager: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceResearch {
    pub session: String,
    pub session_sha256: String,
    pub fixtures: String,
    pub fixtures_sha256: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeekReceipt {
    pub schema: String,
    pub source_research: SourceResearch,
    pub club: u16,
    pub manager: u8,
    pub week: WeekIndex,
    pub cash: WeekCash,
    pub transfer_rows: Vec<TransferRow>,
    pub following_fixture: FollowingFixture,
    pub roster_unchanged: bool,
    pub interpretation: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeekIndex {
    pub index_before: u8,
    pub index_after: u8,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeekCash {
    pub before: i32,
    pub gate_receipts: i32,
    pub prize_money: i32,
    pub total_in: i32,
    pub player_salaries: i32,
    pub win_bonuses: i32,
    pub staff_salaries: i32,
    pub total_out: i32,
    pub after: i32,
}
impl WeekReceipt {
    /// Panic-free construction: the pinned digest guards every field, so a
    /// mismatch is a build/runtime invariant, not a recoverable error.
    pub fn pinned() -> Self {
        let digest = format!("{:x}", Sha256::digest(WEEK_RECEIPT.as_bytes()));
        assert_eq!(digest, WEEK_RECEIPT_SHA256, "week receipt identity differs");
        serde_json::from_str(WEEK_RECEIPT).expect("the pinned week receipt parses")
    }
}
