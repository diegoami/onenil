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
        self.record_match_inner(ticket, score, native_return, None)
    }

    /// [confirmed:archaeology 3f6fd50 T80] The researched persisted policy: the executed
    /// interval replacement's durable effect is the roster slot exchange in the managed
    /// club's first team (save block [DS:D7C2] equivalent), committed together with the
    /// recorded result in this single save transaction. The in-match status flag and the
    /// players' condition are week-processing outputs and are not written here.
    pub fn record_match_with_replacement(
        &mut self,
        ticket: &MatchTicket,
        score: [u16; 2],
        native_return: &str,
        out_slot: usize,
        in_slot: usize,
        out_id: u16,
        in_id: u16,
    ) -> Result<(), String> {
        // The original exchange requires the two slots on opposite sides of
        // slot 12: zero-based first team 0..=10, bench 11..=19 (research T74).
        if out_slot >= 11 || in_slot < 11 || in_slot > 19 {
            return Err("replacement slots must lie on opposite sides of slot 12".into());
        }
        if self.club.first_team.get(out_slot).copied().flatten() != Some(out_id)
            || self.club.first_team.get(in_slot).copied().flatten() != Some(in_id)
        {
            return Err("replacement slots do not hold the executed players".into());
        }
        self.record_match_inner(
            ticket,
            score,
            native_return,
            Some((out_slot, in_id, in_slot, out_id)),
        )
    }

    fn record_match_inner(
        &mut self,
        ticket: &MatchTicket,
        score: [u16; 2],
        native_return: &str,
        exchange: Option<(usize, u16, usize, u16)>,
    ) -> Result<(), String> {
        self.validate()?;
        if self.played_fixture.is_some() {
            return Err("this fixture already has a recorded result".into());
        }
        if self != &ticket.campaign || native_return != "0AB9:EBE2" {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        self.transaction(|next| {
            if let Some((out_slot, in_id, in_slot, out_id)) = exchange {
                next.club.first_team[out_slot] = Some(in_id);
                next.club.first_team[in_slot] = Some(out_id);
            }
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
