//! [designed] Native fixture receipt, distinct from original post-match/week effects.
//! [confirmed:T61 at273fb52fb7d217395f4c5ed95b686c9e6e28695a] The continuing watched caller's completed return is 0AB9:EBE2.
use crate::{Campaign, Fixture, Season};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// An in-memory request owns the full pre-match campaign, including its revision.
/// It cannot be deserialized from a worker packet or synthesized from an expected score.
#[derive(Debug, Clone)]
pub struct MatchTicket {
    campaign: Campaign,
    digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayedFixture {
    pub fixture: Fixture,
    pub season: Season,
    pub week_index: u8,
    pub request_revision: u64,
    pub request_sha256: String,
    /// Managed club first, as projected from actual native scorer/club identities.
    pub score: [u16; 2],
    pub native_return: String,
}

impl PlayedFixture {
    pub(crate) fn validate(&self, campaign: &Campaign) -> Result<(), String> {
        if self.fixture != campaign.next_fixture
            || self.season != campaign.season
            || self.week_index != campaign.week_index
            || self.request_revision >= campaign.revision
            || self.request_sha256.len() != 64
            || !self
                .request_sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || self.native_return != "0AB9:EBE2"
        {
            return Err("recorded result has a foreign or invalid fixture/request context".into());
        }
        Ok(())
    }
}

impl Campaign {
    pub fn match_ticket(&self) -> Result<MatchTicket, String> {
        self.validate()?;
        if self.played_fixture.is_some() {
            return Err("this fixture already has a recorded result".into());
        }
        let digest = format!("{:x}", Sha256::digest(self.encode()?));
        Ok(MatchTicket {
            campaign: self.clone(),
            digest,
        })
    }

    /// [designed] Store computed output exactly once. The caller must supply actual terminal
    /// native output; this API does not simulate a match or apply a week transaction.
    pub fn record_match(
        &mut self,
        ticket: &MatchTicket,
        score: [u16; 2],
        native_return: &str,
    ) -> Result<(), String> {
        self.validate()?;
        if self.played_fixture.is_some() {
            return Err("this fixture already has a recorded result".into());
        }
        if self != &ticket.campaign || native_return != "0AB9:EBE2" {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        self.transaction(|next| {
            next.played_fixture = Some(PlayedFixture {
                fixture: ticket.campaign.next_fixture.clone(),
                season: ticket.campaign.season.clone(),
                week_index: ticket.campaign.week_index,
                request_revision: ticket.campaign.revision,
                request_sha256: ticket.digest.clone(),
                score,
                native_return: native_return.into(),
            });
            Ok(())
        })
    }
}
