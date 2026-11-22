//! [confirmed:archaeology T105 3a0f14d] The decoded late-season league campaign,
//! published as data with no original byte: week 54, Dalglish's Blackburn R
//! (130) with its 15 players, cash 1,045,060, the prepared Newcastle Utd
//! fixture beside the pre-advanced live Chelsea fixture, and the witnessed
//! league week's own cash movement and result. The native line admits exactly
//! the seed's pre- and post-week states; the table stays in T104's model and
//! the week transaction validates it through that model.
use crate::{result::TicketKind, Campaign, Club, Fixture, MatchTicket, Origin, Season, Source};
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

/// The committed week-2 fixture's own identity, pinned so a drifted value is refused.
const LEAGUE_WEEK2_SHA: &str = "9ce163a42223072f350ea54c5819be4b15d0715a05f10ec74fb41ad771ebc666";

/// [confirmed:archaeology T106 8d1edcd] The game's own answer to PLAY GAME with
/// a flagged player fielded.
pub(crate) const LEAGUE_INELIGIBLE: &str =
    "Blackburn R has an ineligible player in the first team : hit any key";

/// [designed:T109] The third league week has no witnessed capture; the post-week
/// machine advertises it, and the native line refuses it (the T90 boundary).
pub(crate) const LEAGUE_THIRD_MATCH_REFUSAL: &str =
    "The next fixture (Wimbledon) has no authenticated original match evidence; playing it is unwitnessed.";

/// [designed:T113] The league season's 30 matches are complete at the
/// third-played machine; a fourth week is unwitnessed.
pub(crate) const LEAGUE_FOURTH_MATCH_REFUSAL: &str =
    "The league season's 30 matches are complete; a fourth week is unwitnessed.";

/// The suspension bit the week-28 processing set on the two flagged defenders
/// and the week-29 processing cleared once it was served.
const SUSPENSION_BIT: u8 = 0b10;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueWeek2 {
    pub schema: String,
    #[allow(dead_code)]
    pub research_commit: String,
    #[allow(dead_code)]
    pub receipt: BTreeMap<String, String>,
    #[allow(dead_code)]
    pub sources: BTreeMap<String, String>,
    pub witnessed: LeagueWitnessed2,
    pub(crate) source: Source,
    pub(crate) target: Source,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueWitnessed2 {
    pub match_no: u8,
    /// Managed club first: Blackburn R drew 3-3 at Chelsea.
    pub score: [u16; 2],
    pub attendance: u32,
    pub cash: LeagueCash,
    pub following_fixture: Fixture,
    pub exchanges: Vec<LeagueExchange>,
    pub exchanged_first_team: [u16; 20],
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueExchange {
    pub player: u16,
    pub name: String,
    pub from_slot: usize,
    pub to_slot: usize,
}

impl LeagueWeek2 {
    pub fn pinned() -> Result<Self, String> {
        let raw = include_bytes!("../fixtures/league-week-2.json");
        if format!("{:x}", Sha256::digest(raw)) != LEAGUE_WEEK2_SHA {
            return Err("league week-2 fixture identity differs".into());
        }
        serde_json::from_slice(raw).map_err(|e| e.to_string())
    }
}

/// The committed week-3 fixture's own identity, pinned so a drifted value is refused.
const LEAGUE_WEEK3_SHA: &str = "91fc548cf27c6509571e8e98f4ee8272ef0597f616b6cf30f35e585b71ded852";

/// [confirmed:archaeology T112 7192873] The third witnessed league week and
/// the season boundary: match #30 (a home win over Wimbledon), the cash
/// movement to 1,636,583, and a fixture table whose next slot carries no
/// opponent - the 16-club schedule is complete at 30 matches.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueWeek3 {
    pub schema: String,
    #[allow(dead_code)]
    pub research_commit: String,
    #[allow(dead_code)]
    pub receipt: BTreeMap<String, String>,
    #[allow(dead_code)]
    pub sources: BTreeMap<String, String>,
    pub witnessed: LeagueWitnessed3,
    pub(crate) target: LeagueTarget3,
    pub table: Vec<LeagueTableRow3>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueWitnessed3 {
    pub match_no: u8,
    /// Managed club first: Blackburn R beat Wimbledon 5-3 at home.
    pub score: [u16; 2],
    /// The week-29 row as the T106 receipt records it (no display position).
    pub week_29_row: Week29Row,
    /// The witnessed condition refreshes (the post-league30 projection).
    pub condition_changes: Vec<ConditionChange>,
    /// The witnessed request values; the ticket identity is not witnessed.
    pub third_request: ThirdRequest,
    #[allow(dead_code)]
    pub week_29_row_source: String,
    #[allow(dead_code)]
    pub home_club: u16,
    #[allow(dead_code)]
    pub away_club: u16,
    #[allow(dead_code)]
    pub home_name: String,
    #[allow(dead_code)]
    pub away_name: String,
    pub attendance: u32,
    pub cash: LeagueCash,
    pub substitution_declined: bool,
    pub injury: Injury3,
    pub following_fixture: Option<FixtureEnd3>,
    pub fixture_table_end: FixtureEnd3,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Injury3 {
    #[allow(dead_code)]
    pub player_name: String,
    #[allow(dead_code)]
    pub pause_expired_without_substitution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureEnd3 {
    pub slot: u8,
    pub raw_bytes: [u8; 3],
}

/// The week-29 row as the T106 receipt records it: no display position (the
/// receipt's rows carry no position - the position evidence lives in the
/// week-3 table's own rows).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Week29Row {
    pub club: String,
    pub played: u8,
    pub home: [u8; 3],
    pub away: [u8; 3],
    pub goals_for: u16,
    pub goals_against: u16,
    pub points: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionChange {
    pub name: String,
    /// The witnessed condition_signed after week 30 (the post-league30
    /// projection).
    pub condition_signed: i8,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThirdRequest {
    pub prepared: ThirdPrepared,
    pub season_counter: u8,
    /// The request identity (revision/sha) is the capture ticket's own and is
    /// not witnessed in the archaeology receipt; the validation binds the
    /// witnessed prepared row and season, not the ticket digest.
    #[allow(dead_code)]
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThirdPrepared {
    pub week: u8,
    pub competition: u8,
    pub opponent: u16,
    pub flags: u8,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueTarget3 {
    pub week_index: u8,
    #[allow(dead_code)]
    pub season_counter: u8,
    pub cash: i32,
    #[allow(dead_code)]
    pub next_fixture: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub league_position: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueTableRow3 {
    pub position: u8,
    pub club: String,
    pub played: u8,
    pub home: [u8; 3],
    pub away: [u8; 3],
    pub goals_for: u16,
    pub goals_against: u16,
    pub points: u8,
}

impl LeagueWeek3 {
    pub fn pinned() -> Result<Self, String> {
        let raw = include_bytes!("../fixtures/league-week-3.json");
        if format!("{:x}", Sha256::digest(raw)) != LEAGUE_WEEK3_SHA {
            return Err("league week-3 fixture identity differs".into());
        }
        serde_json::from_slice(raw).map_err(|e| e.to_string())
    }
}

/// The prepared row the second witnessed result binds: the live Chelsea fixture
/// at the machine's own week, derived from the pinned projection.
fn prepared_row_for_week2(seed2: &LeagueWeek2) -> PreparedRow {
    let s = &seed2.source;
    PreparedRow {
        week: s.week_index,
        competition: s.next_fixture.competition_code,
        opponent: s.next_fixture.opponent,
        flags: s.next_fixture.flags,
        name: s.next_fixture.opponent_name.clone(),
    }
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
        self.binds_at(campaign, seed, campaign.week_index)
    }

    /// The binding against an explicit week: the recorded result stays bound
    /// to the machine it was recorded at, even after the week byte moves.
    pub(crate) fn binds_at(
        &self,
        campaign: &Campaign,
        seed: &LeagueSeed,
        week_index: u8,
    ) -> Result<bool, String> {
        Ok(self.prepared == seed.prepared_fixture
            && self.season == campaign.season
            && self.week_index == week_index
            && self.score == seed.witnessed.score
            && self.native_return == LEAGUE_NATIVE_RETURN
            && self.request_revision < campaign.revision
            && self.request_sha256.len() == 64
            && self
                .request_sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
    }

    /// [designed:T109] The second result binds the live Chelsea fixture at the
    /// machine's own week, derived from the pinned projection.
    pub(crate) fn binds_week2(
        &self,
        campaign: &Campaign,
        seed2: &LeagueWeek2,
    ) -> Result<bool, String> {
        let s = &seed2.source;
        Ok(self.prepared == prepared_row_for_week2(seed2)
            && self.season == campaign.season
            && self.week_index == s.week_index
            && self.score == seed2.witnessed.score
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

/// [designed:T109] The exchanged machine admits the second witnessed week; every
/// other league state keeps its own refusal (the eligibility refusal binds W0,
/// the T108 already-recorded refusal binds the unexchanged machine).
pub(crate) fn is_exchanged_week2(campaign: &Campaign) -> bool {
    match LeagueWeek2::pinned() {
        Ok(seed2) => {
            campaign.week2_context(&seed2) == Some(&seed2.source)
                && campaign.players_equal(&seed2.source)
                && campaign.club.first_team.map(|p| p.unwrap_or(0))
                    == seed2.witnessed.exchanged_first_team
                && campaign.league_played.is_some()
        }
        Err(_) => false,
    }
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
            second_league_played: None,
            third_league_played: None,
            players: s.players.clone(),
        };
        c.validate()?;
        Ok(c)
    }

    pub(crate) fn validate_league_line(&self) -> Result<(), String> {
        // [designed:T113] The week-3 target state binds first: week 56, the
        // cash after week 30, and the opponent-free following fixture.
        if let Ok(seed3) = LeagueWeek3::pinned() {
            if self.week3_context(&seed3) {
                return self.validate_week3_state(&seed3);
            }
        }
        // [designed:T109] The week-2 projections bind their own three states
        // first; the T108 seed states keep their own pins underneath.
        if let Ok(seed2) = LeagueWeek2::pinned() {
            if let Some(source) = self.week2_context(&seed2) {
                return self.validate_week2_state(&seed2, source);
            }
        }
        let source = league_source()?;
        let seed = LeagueSeed::pinned()?;
        if self.origin != league_origin() || self.namespace != source.namespace {
            return Err("campaign source/league namespace differs".into());
        }
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
            None if self.second_league_played.is_none() => self.cash == seed.cash.before,
            Some(played) if self.second_league_played.is_none() => {
                self.cash == seed.cash.after && played.binds(self, &seed)?
            }
            _ => false,
        };
        if !admitted {
            return Err("campaign context outside the supported saved club/fixture".into());
        }
        self.validate_roster_and_players(&source)
    }

    /// Which week-2 projection the state's header and body match, if either:
    /// W0/W1 against the post-league28 machine, W2 against the post-league29 one.
    /// [designed:T113] The week-3 target state's header: week 56, the cash
    /// after week 30, and the opponent-free following fixture.
    fn week3_context(&self, seed3: &LeagueWeek3) -> bool {
        let t = &seed3.target;
        let end = &seed3.witnessed.fixture_table_end;
        self.week_index == t.week_index
            && self.cash == t.cash
            && self.next_fixture.opponent == 0
            && self.next_fixture.slot == end.slot
            && self.next_fixture.competition_code == end.raw_bytes[0]
            && self.third_league_played.is_some()
    }

    fn validate_week3_state(&self, seed3: &LeagueWeek3) -> Result<(), String> {
        let refused = "campaign context outside the supported saved club/fixture";
        let seed = LeagueSeed::pinned()?;
        let seed2 = LeagueWeek2::pinned()?;
        let source = &seed2.source;
        let t = &seed3.target;
        if self.origin != league_origin()
            || self.namespace != source.namespace
            || self.season != source.season
            || self.manager != source.manager
            || self.club.id != source.club.id
            || self.club.name != source.club.name
            || self.prepared_fixture.as_ref() != Some(&seed.prepared_fixture)
            || self.played_fixture.is_some()
            || self.second_fixture.is_some()
        {
            return Err(refused.into());
        }
        let second = self.second_league_played.as_ref().ok_or(refused)?;
        if self.league_played.is_none()
            || second.week_index != 54
            || second.score != seed2.witnessed.score
            || second.native_return != LEAGUE_NATIVE_RETURN
        {
            return Err(refused.into());
        }
        let third = self.third_league_played.as_ref().ok_or(refused)?;
        // [review #218] the result stores the REQUEST's week (55, the
        // pre-match value); the machine's post-week value (56) lives in the
        // campaign's own week index, checked after the walk below.
        if third.week_index != 55
            || third.score != seed3.witnessed.score
            || third.native_return != LEAGUE_NATIVE_RETURN
        {
            return Err(refused.into());
        }
        if third.prepared.week != 55
            || third.prepared.competition != 0
            || third.prepared.opponent != seed3.witnessed.away_club
            || third.prepared.flags != 1
            || third.prepared.name != seed3.witnessed.away_name
            || third.season.counter != seed3.target.season_counter
        {
            return Err(refused.into());
        }
        // the roster arrangement is unchanged through the third week: the
        // injury substitution was declined, so the exchanged arrangement
        // survives (the T112 capture's own state).
        if self.club.first_team.map(|p| p.unwrap_or(0)) != seed2.witnessed.exchanged_first_team {
            return Err(refused.into());
        }
        if self.week_index != t.week_index || self.cash != t.cash {
            return Err(refused.into());
        }
        // [review #221] the following fixture binds whole: the fixture table's
        // opponent-free slot 57 (competition 5, opponent 0, no name).
        let end = &seed3.witnessed.fixture_table_end;
        if self.next_fixture.slot != end.slot
            || self.next_fixture.club != seed3.witnessed.home_club
            || self.next_fixture.opponent != u16::from(end.raw_bytes[1])
            || !self.next_fixture.opponent_name.is_empty()
            || self.next_fixture.competition_code != end.raw_bytes[0]
            || self.next_fixture.flags != end.raw_bytes[2]
            || self.next_fixture.venue_code != 0
            || self.next_fixture.opponent_flags != 0
        {
            return Err(refused.into());
        }
        // [review #219] the six witnessed condition refreshes bind: the
        // players whose condition_signed the post-league30 projection carries.
        for change in &seed3.witnessed.condition_changes {
            let player = self
                .players
                .iter()
                .find(|p| p.name == change.name)
                .ok_or_else(|| refused.to_string())?;
            if player.availability.condition_signed != change.condition_signed {
                return Err(refused.into());
            }
        }
        Ok(())
    }

    fn week2_context<'a>(&self, seed2: &'a LeagueWeek2) -> Option<&'a Source> {
        let s = &seed2.source;
        let t = &seed2.target;
        let header = |src: &Source| {
            self.week_index == src.week_index
                && self.cash == src.cash
                && self.next_fixture == src.next_fixture
        };
        let original = self.club.first_team.map(|p| p.unwrap_or(0)) == s.club.first_team;
        let exchanged =
            self.club.first_team.map(|p| p.unwrap_or(0)) == seed2.witnessed.exchanged_first_team;
        if header(s) && self.players_equal(s) && (original || exchanged) {
            Some(s)
        } else if header(t) && self.players_equal(t) && exchanged {
            Some(t)
        } else {
            None
        }
    }

    fn players_equal(&self, s: &Source) -> bool {
        self.players.len() == s.players.len()
            && self
                .players
                .iter()
                .zip(s.players.iter())
                .all(|(a, b)| a == b)
    }

    fn validate_week2_state(&self, seed2: &LeagueWeek2, source: &Source) -> Result<(), String> {
        let seed = LeagueSeed::pinned()?;
        let s = &seed2.source;
        if self.origin != league_origin()
            || self.namespace != source.namespace
            || self.season != s.season
            || self.manager != s.manager
            || self.club.id != s.club.id
            || self.club.name != s.club.name
            || self.prepared_fixture.as_ref() != Some(&seed.prepared_fixture)
            || self.played_fixture.is_some()
            || self.second_fixture.is_some()
        {
            return Err("campaign context outside the supported saved club/fixture".into());
        }
        let refused = "campaign context outside the supported saved club/fixture";
        let original_roster = self.club.first_team.map(|p| p.unwrap_or(0)) == s.club.first_team;
        let played = self.league_played.as_ref().ok_or(refused)?;
        if !played.binds_at(self, &seed, s.week_index)? {
            return Err(refused.into());
        }
        if original_roster {
            // W0: the flagged, unexchanged machine; the second week unplayed.
            if self.second_league_played.is_some() {
                return Err(refused.into());
            }
        } else {
            // W1/W2: the exchanged roster.
            match self.second_league_played.as_ref() {
                None => {
                    // W1: the exchanged machine, the second week unplayed.
                    if self.week_index != s.week_index || self.cash != s.cash {
                        return Err(refused.into());
                    }
                }
                Some(second) => {
                    // W2: the completed second week.
                    if !second.binds_week2(self, seed2)? {
                        return Err(refused.into());
                    }
                    // W3 (T113): the completed third week - the target state,
                    // with the fixture table's opponent-free next slot.
                    if self.third_league_played.is_some() {
                        let seed3 = LeagueWeek3::pinned()?;
                        let target3 = &seed3.target;
                        let end = &seed3.witnessed.fixture_table_end;
                        if self.week_index != target3.week_index
                            || self.cash != target3.cash
                            || self.next_fixture.opponent != 0
                            || self.next_fixture.slot != end.slot
                            || self.next_fixture.competition_code != end.raw_bytes[0]
                            || self.next_fixture.opponent_name.is_empty()
                        {
                            return Err(refused.into());
                        }
                    }
                }
            }
        }
        self.validate_roster_and_players(source)
    }

    fn is_week2_source_header(&self, seed2: &LeagueWeek2) -> bool {
        let s = &seed2.source;
        self.week_index == s.week_index
            && self.cash == s.cash
            && self.next_fixture == s.next_fixture
            && self.season == s.season
            && self.manager == s.manager
            && self.club.id == s.club.id
            && self.club.name == s.club.name
    }

    pub(crate) fn first_team_has_suspended_player(&self) -> bool {
        // The fielded XI is slots 0..=10; the bench 11..=19 (research T74/T80)
        // - a suspended player on the bench is what the original allowed.
        self.club.first_team.iter().take(11).flatten().any(|id| {
            self.players
                .iter()
                .find(|p| p.id == *id)
                .is_some_and(|p| p.availability.status_bits & SUSPENSION_BIT != 0)
        })
    }

    /// [confirmed:archaeology T106 8d1edcd] The week-28 processing's own
    /// availability output, pinned by the post-league28 projection: the two
    /// flagged defenders and Wright's recovered condition.
    pub fn apply_league_week_processing(&mut self) -> Result<(), String> {
        let seed2 = LeagueWeek2::pinned()?;
        let source = &seed2.source;
        if !self.is_week2_source_header(&seed2)
            || self.league_played.is_none()
            || self.second_league_played.is_some()
            || self.players_equal(source)
            || self.club.first_team.map(|p| p.unwrap_or(0)) != source.club.first_team
        {
            return Err("the week processing applies to the completed week-28 machine".into());
        }
        self.transaction(|next| {
            next.players = source.players.clone();
            Ok(())
        })
    }

    /// [confirmed:archaeology T106 8d1edcd] The manager's witnessed squad
    /// exchange, one atomic transaction: Wright for Berg, Newell for Hendry -
    /// the arrangement the save itself holds afterwards (T80's DS:D7C2 slot
    /// exchange; the game re-fits the incoming players by role).
    pub fn record_league_exchange(&mut self) -> Result<(), String> {
        let seed2 = LeagueWeek2::pinned()?;
        let source = &seed2.source;
        if !self.is_week2_source_header(&seed2)
            || !self.players_equal(source)
            || self.club.first_team.map(|p| p.unwrap_or(0)) != source.club.first_team
            || self.league_played.is_none()
            || self.second_league_played.is_some()
        {
            return Err("the exchange applies to the flagged, unexchanged week-28 machine".into());
        }
        self.transaction(|next| {
            next.club.first_team =
                seed2
                    .witnessed
                    .exchanged_first_team
                    .map(|p| if p == 0 { None } else { Some(p) });
            Ok(())
        })
    }

    /// [confirmed:archaeology T106 8d1edcd] The witnessed second league week:
    /// the 3-3 draw at Chelsea, the cash movement, and the advance to
    /// Wimbledon at home with the week byte at 55 (this pointer was not
    /// pre-advanced - T93's no-advance rule bound week 28, not this one).
    pub fn record_league_week_2(
        &mut self,
        ticket: &MatchTicket,
        score: [u16; 2],
        native_return: &str,
        total_in: i32,
        total_out: i32,
        following_competition: u8,
        following_opponent: u16,
        following_flags: u8,
        following_name: &str,
    ) -> Result<(), String> {
        if ticket.kind != TicketKind::LeagueFixture {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        let seed2 = LeagueWeek2::pinned()?;
        let w = &seed2.witnessed;
        if w.score != score || native_return != LEAGUE_NATIVE_RETURN {
            return Err("the witnessed league result differs".into());
        }
        if total_in != w.cash.total_in || total_out != w.cash.total_out {
            return Err("the league week's authenticated movement differs".into());
        }
        if seed2
            .source
            .cash
            .checked_add(total_in)
            .and_then(|v| v.checked_sub(total_out))
            != Some(w.cash.after)
        {
            return Err("the league week's authenticated movement differs".into());
        }
        let following = &w.following_fixture;
        if following.competition_code != following_competition
            || following.opponent != following_opponent
            || following.flags != following_flags
            || following.opponent_name != following_name
        {
            return Err("the league week's following fixture differs".into());
        }
        league_table_reproduces()?;
        self.validate()?;
        if self.second_league_played.is_some() {
            return Err("this fixture already has a recorded result".into());
        }
        if self != &ticket.campaign {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        let target = seed2.target.clone();
        let request_revision = ticket.campaign.revision;
        let request_sha256 = ticket.digest.clone();
        let prepared = prepared_row_for_week2(&seed2);
        let season = ticket.campaign.season.clone();
        let week_index = seed2.source.week_index;
        self.transaction(|next| {
            next.week_index = target.week_index;
            next.cash = target.cash;
            next.next_fixture = target.next_fixture.clone();
            next.players = target.players.clone();
            next.second_league_played = Some(LeaguePlayed {
                prepared,
                season,
                week_index,
                request_revision,
                request_sha256,
                score,
                native_return: native_return.into(),
            });
            Ok(())
        })
    }

    /// [confirmed:archaeology T112 7192873] The third witnessed league week,
    /// atomically: the ticket admits the second-league-played machine, the
    /// score, return and cash movement must be exactly the witnessed values,
    /// and one transaction records the result, moves the cash, advances the
    /// week index to 56 and clears the following fixture - the fixture table's
    /// next slot (57) carries opponent byte 0 (competition 5): the 16-club
    /// schedule is complete at 30 matches. The injury substitution was
    /// declined; the roster is unchanged.
    pub fn record_league_week_3(
        &mut self,
        ticket: &MatchTicket,
        score: [u16; 2],
        native_return: &str,
        total_in: i32,
        total_out: i32,
    ) -> Result<(), String> {
        if ticket.kind != TicketKind::LeagueFixture {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        // [review #218] the ticket must be THIS machine's: a foreign ticket
        // (any other campaign's) completing week 30 is refused.
        if self != &ticket.campaign {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        let seed3 = LeagueWeek3::pinned()?;
        let w = &seed3.witnessed;
        if w.score != score || native_return != LEAGUE_NATIVE_RETURN {
            return Err("the witnessed league result differs".into());
        }
        if total_in != w.cash.total_in || total_out != w.cash.total_out {
            return Err("the league week's authenticated movement differs".into());
        }
        if self
            .cash
            .checked_add(total_in)
            .and_then(|v| v.checked_sub(total_out))
            != Some(w.cash.after)
        {
            return Err("the league week's authenticated movement differs".into());
        }
        if self.second_league_played.is_none() || self.third_league_played.is_some() {
            return Err("this fixture already has a recorded result".into());
        }
        let target = &seed3.target;
        let end = &w.fixture_table_end;
        let request_revision = ticket.campaign.revision;
        let request_sha256 = ticket.digest.clone();
        let season = ticket.campaign.season.clone();
        // the result binds the REQUEST's week (55, the pre-match value), the
        // way week 29's result stored 54 - the machine's post-week value (56)
        // lives in the campaign's own week index
        let week_index = ticket.campaign.week_index;
        let condition_changes = seed3.witnessed.condition_changes.clone();
        self.transaction(|next| {
            next.week_index = target.week_index;
            next.cash = target.cash;
            // [review #219] the witnessed condition refreshes: the six players
            // whose condition_signed the post-league30 projection carries
            for change in &condition_changes {
                let player = next
                    .players
                    .iter_mut()
                    .find(|p| p.name == change.name)
                    .ok_or_else(|| {
                        format!("the condition change names an unknown player: {}", change.name)
                    })?;
                player.availability.condition_signed = change.condition_signed;
            }
            next.next_fixture = crate::Fixture {
                slot: end.slot,
                club: w.home_club,
                opponent: u16::from(end.raw_bytes[1]),
                opponent_name: String::new(),
                competition_code: end.raw_bytes[0],
                flags: end.raw_bytes[2],
                venue_code: 0,
                opponent_flags: 0,
            };
            next.third_league_played = Some(LeaguePlayed {
                prepared: PreparedRow {
                    week: 55,
                    competition: 0,
                    opponent: w.away_club,
                    flags: 1,
                    name: w.away_name.clone(),
                },
                season,
                week_index,
                request_revision,
                request_sha256,
                score,
                native_return: native_return.into(),
            });
            Ok(())
        })
    }

    /// [confirmed:archaeology T102 472badb; T105 3a0f14d] The one witnessed
    /// league week, atomically: the ticket admits the pre-week machine, the
    /// score, return, cash movement and following fixture must be exactly the
    /// witnessed values, the table must reproduce through the model, and one
    /// transaction records the result and moves the cash. The live pointer was
    /// already pre-advanced to Chelsea at preparation (T101), so the week stays
    /// 54 and the next fixture stays the Chelsea row - T93's lesson: never
    /// advance an already-advanced projection.
    pub fn record_league_week(
        &mut self,
        ticket: &MatchTicket,
        score: [u16; 2],
        native_return: &str,
        total_in: i32,
        total_out: i32,
        following_competition: u8,
        following_opponent: u16,
        following_flags: u8,
        following_name: &str,
    ) -> Result<(), String> {
        if ticket.kind != TicketKind::LeagueFixture {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        let seed = LeagueSeed::pinned()?;
        if seed.witnessed.score != score || native_return != LEAGUE_NATIVE_RETURN {
            return Err("the witnessed league result differs".into());
        }
        if total_in != seed.cash.total_in || total_out != seed.cash.total_out {
            return Err("the league week's authenticated movement differs".into());
        }
        let following = &seed.campaign.next_fixture;
        if following.competition_code != following_competition
            || following.opponent != following_opponent
            || following.flags != following_flags
            || following.opponent_name != following_name
        {
            return Err("the league week's following fixture differs".into());
        }
        league_table_reproduces()?;
        self.validate()?;
        if self.league_played.is_some() {
            return Err("this fixture already has a recorded result".into());
        }
        if self != &ticket.campaign {
            return Err("match completion is stale, foreign or unfinished".into());
        }
        self.transaction(|next| {
            next.league_played = Some(LeaguePlayed {
                prepared: seed.prepared_fixture.clone(),
                season: ticket.campaign.season.clone(),
                week_index: ticket.campaign.week_index,
                request_revision: ticket.campaign.revision,
                request_sha256: ticket.digest.clone(),
                score,
                native_return: native_return.into(),
            });
            next.cash = next
                .cash
                .checked_add(total_in)
                .and_then(|v| v.checked_sub(total_out))
                .ok_or("cash overflow")?;
            Ok(())
        })
    }
}

/// [designed:T113] The completed-second-week machine: week 55, the
/// Wimbledon slot 56 as the following fixture, the cash after week 29,
/// the second result recorded and the third not yet.
pub(crate) fn is_week2_played_context(c: &crate::Campaign) -> bool {
    match LeagueWeek2::pinned() {
        Ok(seed2) => {
            let t = &seed2.target;
            let w = &seed2.witnessed;
            c.week_index == t.week_index
                && c.cash == w.cash.after
                && c.second_league_played.is_some()
                && c.third_league_played.is_none()
                && c.next_fixture == t.next_fixture
        }
        Err(_) => false,
    }
}

/// [designed:T113] The completed-third-week machine: week 56, no
/// opponent, the cash after week 30, the third result recorded.
pub(crate) fn is_week3_played_context(c: &crate::Campaign) -> bool {
    match LeagueWeek3::pinned() {
        Ok(seed3) => {
            let t = &seed3.target;
            let end = &seed3.witnessed.fixture_table_end;
            c.week_index == t.week_index
                && c.cash == t.cash
                && u16::from(c.next_fixture.opponent) == u16::from(end.raw_bytes[1])
                && u16::from(c.next_fixture.competition_code) == u16::from(end.raw_bytes[0])
                && c.next_fixture.slot == end.slot
                && c.third_league_played.is_some()
        }
        Err(_) => false,
    }
}
