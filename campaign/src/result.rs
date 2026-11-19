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
    /// [designed:T91] The witnessed route the ticket admits; the recording API
    /// must match it, so a PSG result can never post into the Sevilla path or
    /// vice versa.
    pub(crate) kind: TicketKind,
}
impl MatchTicket {
    /// [designed:T91] Whether this ticket admits the witnessed Sevilla route;
    /// the recording API must match it.
    pub fn is_sevilla(&self) -> bool {
        self.kind == TicketKind::SevillaFriendly
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TicketKind {
    /// The week-0 fresh friendly (research T66/T88).
    FirstFriendly,
    /// The week-1 Sevilla friendly (archaeology T89, runs/T89/sevilla-r0001).
    SevillaFriendly,
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

/// [confirmed:research T81 dc54fe9] The completed friendly week's authenticated
/// movement and funds (runs/T81/session-r0007.json): the recorded in/out totals
/// and the resulting club funds. These are research values, not caller policy.
pub const AUTHENTICATED_WEEK_TOTAL_IN: i32 = 38_761;
pub const AUTHENTICATED_WEEK_TOTAL_OUT: i32 = 69_270;
pub const WEEK_ONE_CASH: i32 = 421_491;

/// [confirmed:archaeology 3d41a0db T89] The witnessed Sevilla friendly's own
/// facts (runs/T89/sevilla-r0001): the witnessed result and the caller's own
/// condition outcomes for the managed roster, digest-pinned from
/// campaign/fixtures/sevilla-post-match.json. [confirmed:archaeology 788b833 T93]
/// The following week route preserves these conditions without another refresh.
const SEVILLA_POST_MATCH_SHA: &str =
    "91fc66e730b3a6d382982d8258d528f88b174756c0257d386a18535631e9efe7";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct SevillaPostMatch {
    schema: String,
    #[allow(dead_code)]
    research_commit: String,
    #[allow(dead_code)]
    caller_fixture_sha256: String,
    managed_club: u16,
    witnessed: SevillaWitnessed,
    #[allow(dead_code)]
    scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct SevillaWitnessed {
    score: [u16; 2],
    native_return: String,
    conditions: Vec<SevillaCondition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct SevillaCondition {
    player: u16,
    #[allow(dead_code)]
    condition_before: i8,
    condition_after: i8,
}

fn sevilla_post_match() -> Result<SevillaPostMatch, String> {
    let raw = include_bytes!("../fixtures/sevilla-post-match.json");
    if format!("{:x}", Sha256::digest(raw)) != SEVILLA_POST_MATCH_SHA {
        return Err("witnessed Sevilla post-match identity differs".into());
    }
    serde_json::from_slice(raw).map_err(|e| e.to_string())
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
    /// [designed:T91] The witnessed Sevilla result binds the authenticated
    /// week-one context itself: the fixture is the week-one fixture, the week
    /// index in the history stays 1 after week completion, and the
    /// request digest is a valid native identity.
    pub(crate) fn validate_second(&self, campaign: &Campaign) -> Result<(), String> {
        let source = Campaign::initial()?;
        let week_one_fixture = Fixture {
            slot: 2,
            club: source.club.id,
            opponent: 176,
            opponent_name: "Sevilla".to_owned(),
            competition_code: 1,
            flags: 0,
            venue_code: 0,
            opponent_flags: 0,
        };
        if self.fixture != week_one_fixture
            || self.season != source.season
            || self.week_index != 1
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
    /// [designed:T94; #202] The witnessed presentation supports the unchanged
    /// native setup only. This does not equate native pre-match conditions
    /// with the original's preparation image. History/revision and player
    /// storage order are not setup inputs; undoing edits restores admission.
    fn validate_sevilla_setup(&self) -> Result<(), String> {
        let source = Self::initial()?;
        if self.club != source.club
            || self.players.len() != source.players.len()
            || self.players.iter().any(|player| {
                source
                    .players
                    .iter()
                    .find(|original| original.id == player.id)
                    != Some(player)
            })
        {
            return Err(
                "Sevilla needs the unchanged team. Undo edits or reload your pre-Sevilla save."
                    .into(),
            );
        }
        Ok(())
    }

    pub fn match_ticket(&self) -> Result<MatchTicket, String> {
        self.validate()?;
        if self.week_index == 2 {
            return Err(crate::sevilla_week::THIRD_MATCH_REFUSAL.into());
        }
        if self.played_fixture.is_some() {
            if self.week_index >= 1 {
                if self.second_fixture.is_some() {
                    // [T91] The completed fixture replays nothing: the accurate
                    // recorded-result refusal names the state, not a gap.
                    return Err("this fixture already has a recorded result".into());
                }
                // [confirmed:archaeology 3d41a0db T89] The advertised following
                // fixture IS the witnessed Sevilla friendly; its request admits
                // and binds the prepared match's own context. Any other week-1
                // fixture keeps the honest unwitnessed boundary.
                self.validate_sevilla_setup()?;
                return Ok(MatchTicket {
                    campaign: self.clone(),
                    digest: format!("{:x}", Sha256::digest(self.encode()?)),
                    kind: TicketKind::SevillaFriendly,
                });
            }
            return Err("this fixture already has a recorded result".into());
        }
        let digest = format!("{:x}", Sha256::digest(self.encode()?));
        Ok(MatchTicket {
            campaign: self.clone(),
            digest,
            kind: TicketKind::FirstFriendly,
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

    /// [confirmed:archaeology 3d41a0db T89] The witnessed Sevilla friendly's
    /// result posts through the caller's own facts: the witnessed score and the
    /// caller's condition outcomes for the managed roster. The values must be
    /// exactly the witnessed ones - no PSG value transplants into the Sevilla
    /// path. T93 admits the separately authenticated week transaction afterwards.
    /// The recorded week-0 result and the updated conditions persist together.
    pub fn record_sevilla_result(
        &mut self,
        ticket: &MatchTicket,
        score: [u16; 2],
        native_return: &str,
        conditions: &[(u16, i8)],
    ) -> Result<(), String> {
        if ticket.kind != TicketKind::SevillaFriendly {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        let witnessed = sevilla_post_match()?;
        if witnessed.witnessed.score != score || witnessed.witnessed.native_return != native_return
        {
            return Err("the witnessed Sevilla result differs".into());
        }
        let expected: Vec<(u16, i8)> = witnessed
            .witnessed
            .conditions
            .iter()
            .map(|condition| (condition.player, condition.condition_after))
            .collect();
        let mut supplied = conditions.to_vec();
        supplied.sort_unstable();
        let mut expected_sorted = expected.clone();
        expected_sorted.sort_unstable();
        if supplied != expected_sorted {
            return Err("the witnessed Sevilla condition outcomes differ".into());
        }
        self.validate()?;
        if self.second_fixture.is_some() {
            return Err("this fixture already has a recorded result".into());
        }
        if self != &ticket.campaign || native_return != "0AB9:EBE2" {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        let mut conditions_by_player = conditions.to_vec();
        conditions_by_player.sort_unstable_by_key(|&(player, _)| player);
        self.transaction(|next| {
            next.second_fixture = Some(PlayedFixture {
                fixture: ticket.campaign.next_fixture.clone(),
                season: ticket.campaign.season.clone(),
                week_index: ticket.campaign.week_index,
                request_revision: ticket.campaign.revision,
                request_sha256: ticket.digest.clone(),
                score,
                native_return: native_return.into(),
            });
            // [designed:T91] The caller's witnessed outcomes are authoritative
            // for the managed roster: the native campaign's own pre-caller
            // conditions are its week-0 state, and the original's weekly
            // recovery between the two matches is unwitnessed. Nothing here
            // models it; the witnessed outcomes apply exactly.
            for (player, condition) in conditions_by_player.iter() {
                let row = next
                    .players
                    .iter_mut()
                    .find(|row| row.id == *player)
                    .ok_or("witnessed condition names a foreign player")?;
                row.availability.condition_signed = *condition;
            }
            Ok(())
        })
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
        if self.week_index == 1 {
            return self.complete_sevilla_week(
                total_in,
                total_out,
                following_competition,
                following_opponent,
                following_flags,
                following_name,
            );
        }
        if self.week_index == 2 {
            return Err("this week's transaction is already complete".into());
        }
        // [review #176 F1] The week's movement and following fixture are the
        // authenticated research values (T81 dc54fe9, T82 c13195a): any other
        // argument set refuses before the transaction, and the post-state is
        // bound again by validate().
        if total_in != AUTHENTICATED_WEEK_TOTAL_IN
            || total_out != AUTHENTICATED_WEEK_TOTAL_OUT
            || following_competition != 1
            || following_opponent != 176
            || following_flags != 0
            || following_name != "Sevilla"
        {
            return Err("the friendly week's authenticated movement differs".into());
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
