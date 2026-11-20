//! [confirmed:archaeology T105 3a0f14d] The decoded late-season league campaign,
//! published as data with no original byte: week 54, Dalglish's Blackburn R
//! (130) with its 15 players, cash 1,045,060, the prepared Newcastle Utd
//! fixture beside the pre-advanced live Chelsea fixture, and the witnessed
//! league week's own cash movement and result. The native line admits exactly
//! the seed's pre- and post-week states; the table stays in T104's model and
//! the week transaction validates it through that model.
use crate::{Club, Campaign, Origin, Source, Season};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// The committed fixture's own identity, pinned so a drifted value is refused.
const LEAGUE_SEED_SHA: &str = "9854d368bc5ed1e50c09182e18b989468602532317f2326b2700a25674f45202";

/// [derived:archaeology T61; T102 472badb] The league capture played through
/// the same watched PLAY GAME caller whose completed return T61 confirmed, so
/// the recorded return must be that one.
const LEAGUE_NATIVE_RETURN: &str = "0AB9:EBE2";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueSeed {
    pub schema: String,
    #[allow(dead_code)]
    pub research_commit: String,
    #[allow(dead_code)]
    pub sources: BTreeMap<String, String>,
    pub cash: LeagueCash,
    pub prepared_fixture: PreparedRow,
    pub witnessed: LeagueWitnessed,
    pub(crate) campaign: Source,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueCash {
    pub before: i32,
    pub total_in: i32,
    pub total_out: i32,
    pub after: i32,
}

/// [confirmed:archaeology T101 868a012; T105 3a0f14d] The prepared fixture is
/// published only in the fields the receipts carry - week, competition,
/// opponent, flags and the name. No slot, venue or opponent flags exist in the
/// evidence, so the native binding carries exactly these five and never
/// invents a full row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedRow {
    pub week: u8,
    pub competition: u8,
    pub opponent: u16,
    pub flags: u8,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueWitnessed {
    pub match_no: u8,
    /// Managed club first: Blackburn R away lost 0-2 at Newcastle Utd.
    pub score: [u16; 2],
    pub home: String,
    pub away: String,
    pub home_goals: u8,
    pub away_goals: u8,
    pub attendance: u32,
}

/// The witnessed league week's recorded result: the league analogue of
/// PlayedFixture, bound to the prepared row the receipts actually carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeaguePlayed {
    pub prepared: PreparedRow,
    pub season: Season,
    pub week_index: u8,
    pub request_revision: u64,
    pub request_sha256: String,
    pub score: [u16; 2],
    pub native_return: String,
}

impl LeagueSeed {
    pub fn pinned() -> Result<Self, String> {
        let raw = include_bytes!("../fixtures/league-campaign.json");
        if format!("{:x}", Sha256::digest(raw)) != LEAGUE_SEED_SHA {
            return Err("league campaign seed identity differs".into());
        }
        serde_json::from_slice(raw).map_err(|e| e.to_string())
    }
}

impl LeaguePlayed {
    /// [designed:T95; #204] The recorded result binds its own witnessed facts
    /// and a valid native request identity. The revision is a lower bound:
    /// legitimate edits before the match raise it further.
    pub(crate) fn binds(&self, campaign: &Campaign, seed: &LeagueSeed) -> Result<bool, String> {
        Ok(self.prepared == seed.prepared_fixture
            && self.season == campaign.season
            && self.week_index == campaign.week_index
            && self.score == seed.witnessed.score
            && self.native_return == LEAGUE_NATIVE_RETURN
            && self.request_revision < campaign.revision
            && self.request_sha256.len() == 64
            && self
                .request_sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
    }
}

/// The league line's own origin: the seed record, the preparation savestate it
/// re-projects from, and the league file the T101 receipt pins.
pub(crate) fn league_origin() -> Origin {
    Origin {
        research_commit: "3a0f14d2bae65a68bb29753d9d966f4cd6468d43".into(),
        record_sha256: "90efe4c62473c9e2a0896b2afee27b97ee57faa7eab189e91e82c452d223cbb7".into(),
        save_sha256: "3abfd6b00f8024900b93a20d1ff1eec60cfd2d570e927a00eca495d7f7b1d1c8".into(),
        league_sha256: "fc4bfa045b71ddede9692f40d6a4c0e3b29cc7334c1ec6e910c275f63f714352".into(),
    }
}

pub(crate) fn league_source() -> Result<Source, String> {
    Ok(LeagueSeed::pinned()?.campaign)
}

/// [confirmed:archaeology T103 17cf78c; native T104 ec4611b] The captured
/// table must reproduce through the model under the two-point rule; a
/// three-point rule does not, and the week transaction refuses there.
pub(crate) fn league_table_reproduces() -> Result<(), String> {
    let round = crate::league_table::LeagueRound::captured()?;
    if !round.reproduces_captured_table() {
        return Err("the league table does not reproduce under the two-point rule".into());
    }
    Ok(())
}

impl Campaign {
    /// [confirmed:archaeology T105 3a0f14d] The late-season machine exactly as
    /// the seed publishes it: the pre-week state.
    pub fn initial_league() -> Result<Self, String> {
        let seed = LeagueSeed::pinned()?;
        let s = &seed.campaign;
        let c = Self {
            schema: "onenil-campaign".into(),
            version: 1,
            origin: league_origin(),
            revision: 0,
            namespace: s.namespace.clone(),
            season: s.season.clone(),
            week_index: s.week_index,
            manager: s.manager.clone(),
            club: Club {
                id: s.club.id,
                name: s.club.name.clone(),
                first_team: s
                    .club
                    .first_team
                    .map(|id| if id == 0 { None } else { Some(id) }),
                reserves: s
                    .club
                    .reserves
                    .map(|id| if id == 0 { None } else { Some(id) }),
                team_instructions: s.club.team_instructions,
            },
            cash: s.cash,
            next_fixture: s.next_fixture.clone(),
            prepared_fixture: Some(seed.prepared_fixture.clone()),
            played_fixture: None,
            second_fixture: None,
            league_played: None,
            players: s.players.clone(),
        };
        c.validate()?;
        Ok(c)
    }

    pub(crate) fn validate_league_context(&self, source: &Source) -> Result<(), String> {
        let seed = LeagueSeed::pinned()?;
        if self.week_index != source.week_index
            || self.season != source.season
            || self.manager != source.manager
            || self.club.id != source.club.id
            || self.club.name != source.club.name
            || self.prepared_fixture.as_ref() != Some(&seed.prepared_fixture)
            || self.next_fixture != source.next_fixture
            || self.played_fixture.is_some()
            || self.second_fixture.is_some()
        {
            return Err("campaign context outside the supported saved club/fixture".into());
        }
        let admitted = match self.league_played.as_ref() {
            // [confirmed:archaeology T105 3a0f14d] The pre-week machine.
            None => self.cash == seed.cash.before,
            // [confirmed:archaeology T102 472badb] The completed week: the
            // recorded result and the moved cash, together. A result without
            // the cash movement, or cash without the result, is a state no
            // native API produced (T95's principle).
            Some(played) => self.cash == seed.cash.after && played.binds(self, &seed)?,
        };
        if !admitted {
            return Err("campaign context outside the supported saved club/fixture".into());
        }
        self.validate_roster_and_players(source)
    }
}
