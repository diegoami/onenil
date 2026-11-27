//! [designed:T80] The authenticated friendly-week receipt (research T81/T82),
//! pinned by digest: the one observed week's cash movement, transfer rows and
//! following fixture. No value here generalises beyond that week.
use serde::Deserialize;
use sha2::{Digest, Sha256};

pub const WEEK_RECEIPT: &str = include_str!("../assets/week-receipt.json");
pub const WEEK_RECEIPT_SHA256: &str =
    "821dd0c1fce2dd5024ec332aa3a3ff7c02bfb53cd37978f9b1991aa958536332";

/// [confirmed:archaeology T102] The witnessed week-28 cash flow and transfer
/// news of the late-season league campaign.
pub const LEAGUE_RECEIPT: &str = include_str!("../assets/league-week-receipt.json");
pub const LEAGUE_RECEIPT_SHA256: &str =
    "f4445d243e67e3bcc1ed693658831bd604bcd5567b9aa29b8edc51d0957b16a3";

/// The late-season league campaign (prepared or played week 28).
pub fn is_league(campaign: &onenil_campaign::Campaign) -> bool {
    campaign.prepared_fixture.is_some() || campaign.league_played.is_some()
}

pub fn league_receipt() -> WeekReceipt {
    let digest = format!("{:x}", Sha256::digest(LEAGUE_RECEIPT.as_bytes()));
    assert_eq!(
        digest, LEAGUE_RECEIPT_SHA256,
        "league receipt identity differs"
    );
    serde_json::from_str(LEAGUE_RECEIPT).expect("the league receipt parses")
}

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
    #[serde(default)]
    pub tv_coverage: i32,
    #[serde(default)]
    pub radio_coverage: i32,
    #[serde(default)]
    pub players_sold: i32,
    #[serde(default)]
    pub sponsorship: i32,
    #[serde(default)]
    pub directors_handout: i32,
    pub total_in: i32,
    pub player_salaries: i32,
    pub win_bonuses: i32,
    pub staff_salaries: i32,
    pub ground_maintenance: i32,
    #[serde(default)]
    pub players_bought: i32,
    #[serde(default)]
    pub contracts: i32,
    #[serde(default)]
    pub interest: i32,
    #[serde(default)]
    pub sundries: i32,
    pub total_out: i32,
    pub after: i32,
    pub overdraft_facility: i32,
}
impl WeekReceipt {
    /// [confirmed:archaeology 788b833 T93] Choose the completed match's week
    /// receipt. A persisted second result retains its own receipt after advance.
    pub fn for_campaign(campaign: &onenil_campaign::Campaign) -> Self {
        if is_league(campaign) {
            league_receipt()
        } else if campaign.second_fixture.is_some() {
            let receipt = onenil_campaign::sevilla_week::SevillaWeek::pinned()
                .expect("the pinned Sevilla week receipt authenticates");
            serde_json::from_value(receipt.presentation).expect("the pinned presentation parses")
        } else {
            Self::pinned()
        }
    }

    /// Panic-free construction: the pinned digest guards every field, so a
    /// mismatch is a build/runtime invariant, not a recoverable error.
    pub fn pinned() -> Self {
        let digest = format!("{:x}", Sha256::digest(WEEK_RECEIPT.as_bytes()));
        assert_eq!(digest, WEEK_RECEIPT_SHA256, "week receipt identity differs");
        serde_json::from_str(WEEK_RECEIPT).expect("the pinned week receipt parses")
    }
}
