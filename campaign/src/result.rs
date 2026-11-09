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
        // [designed:T80] The recorded result validates against the pinned source
        // context, not the live next_fixture: the researched week completion
        // (research T82 c13195a) advances the fixture and the week after the
        // result is stored, and the stored request stays bound to its week.
        let source = Campaign::initial()?;
        if self.fixture != source.next_fixture
            || self.season != source.season
            || self.week_index != source.week_index
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

    /// [confirmed:research T81 dc54fe9/T82 c13195a] The observed friendly week's
    /// transaction: the club's cash moves by the authenticated gate receipts and
    /// weekly outgoings, the week index advances exactly once, and the following
    /// fixture (competition 1, opponent 176, Sevilla) becomes the next fixture.
    /// One atomic save transaction, gated on the recorded result; the original's
    /// week condition refresh is NOT applied here (research T81 names it a
    /// follow-up: it depends on the match's own events).
    pub fn record_week_completion(
        &mut self,
        total_in: i32,
        total_out: i32,
        following_competition: u8,
        following_opponent: u16,
        following_flags: u8,
        following_name: &str,
    ) -> Result<(), String> {
        self.validate()?;
        let played = self
            .played_fixture
            .as_ref()
            .ok_or("the friendly result must be recorded before the week completes")?;
        if self.week_index != 0 {
            return Err("this campaign has already completed its first week".into());
        }
        let following_slot = played
            .fixture
            .slot
            .checked_add(1)
            .ok_or("fixture slot exhausted")?;
        self.transaction(|next| {
            next.cash = next
                .cash
                .checked_add(total_in)
                .and_then(|v| v.checked_sub(total_out))
                .ok_or("cash overflow")?;
            next.week_index = next
                .week_index
                .checked_add(1)
                .ok_or("week index exhausted")?;
            next.next_fixture = Fixture {
                slot: following_slot,
                club: next.club.id,
                opponent: following_opponent,
                opponent_name: following_name.to_owned(),
                competition_code: following_competition,
                flags: following_flags,
                venue_code: 0,
                opponent_flags: 0,
            };
            Ok(())
        })
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
