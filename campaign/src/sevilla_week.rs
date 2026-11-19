//! [confirmed:archaeology 788b833 T93] One witnessed Sevilla week, not finance rules.
use crate::{Campaign, Fixture};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const THIRD_MATCH_REFUSAL: &str = "The next fixture (Celtic) has no authenticated original match evidence; playing it is unwitnessed.";
const RECEIPT_SHA: &str = "fe51b0a36de636ca3ccb4bcd645395ce87a2183171fcdaddd5c7e9897d261990";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SevillaWeek {
    pub schema: String,
    pub research_commit: String,
    pub sources: BTreeMap<String, String>,
    pub presentation: Value,
    pub following_fixture: Fixture,
    pub conditions: BTreeMap<u16, i8>,
    pub original_save: Value,
}
impl SevillaWeek {
    pub fn pinned() -> Result<Self, String> {
        let raw = include_bytes!("../fixtures/sevilla-week.json");
        if format!("{:x}", Sha256::digest(raw)) != RECEIPT_SHA {
            return Err("Sevilla week receipt identity differs".into());
        }
        serde_json::from_slice(raw).map_err(|e| e.to_string())
    }
    pub(crate) fn conditions_match(&self, campaign: &Campaign) -> bool {
        campaign.players.len() == self.conditions.len()
            && campaign
                .players
                .iter()
                .all(|p| self.conditions.get(&p.id) == Some(&p.availability.condition_signed))
    }
}
impl Campaign {
    pub(crate) fn complete_sevilla_week(
        &mut self,
        total_in: i32,
        total_out: i32,
        competition: u8,
        opponent: u16,
        flags: u8,
        name: &str,
    ) -> Result<(), String> {
        let receipt = SevillaWeek::pinned()?;
        if self.second_fixture.is_none() || self.played_fixture.is_none() {
            return Err("the Sevilla result must be recorded before the week completes".into());
        }
        let cash = &receipt.presentation["cash"];
        if total_in as i64 != cash["total_in"].as_i64().ok_or("missing total in")?
            || total_out as i64 != cash["total_out"].as_i64().ok_or("missing total out")?
            || competition != receipt.following_fixture.competition_code
            || opponent != receipt.following_fixture.opponent
            || flags != receipt.following_fixture.flags
            || name != receipt.following_fixture.opponent_name
            || !receipt.conditions_match(self)
        {
            return Err(
                "the Sevilla week's authenticated movement, fixture or conditions differ".into(),
            );
        }
        self.transaction(|next| {
            next.cash = next
                .cash
                .checked_add(total_in)
                .and_then(|v| v.checked_sub(total_out))
                .ok_or("cash overflow")?;
            // [designed:T93] Native week_index tracks the committed management
            // week. The original pre-advanced at preparation; do not advance its
            // already-week-2 projection again or touch the caller's conditions.
            next.week_index = 2;
            next.next_fixture = receipt.following_fixture;
            Ok(())
        })
    }
}
