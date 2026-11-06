//! [confirmed] Research T65 at39e289a: coherent96 SAVE1/ENGL-95 club130, week0.
//! [designed] Native owned state schema; no engine formulas, progression or result posting.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, path::Path};
pub mod storage;
const SOURCE_SHA: &str = "ebafc57db43544adcf13d3c002f372e587d8cc201e4a1ae8aa1a74dd37e31483";
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Namespace {
    pub edition: u16,
    pub league: String,
    pub league_year: u16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Season {
    pub counter: u8,
    pub start_year: u16,
    pub end_year: u16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manager {
    pub slot: u8,
    pub name_id: u16,
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fixture {
    pub slot: u8,
    pub club: u16,
    pub opponent: u16,
    pub opponent_name: String,
    pub competition_code: u8,
    pub flags: u8,
    pub venue_code: u8,
    pub opponent_flags: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Availability {
    pub condition_signed: i8,
    pub status_bits: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: u16,
    pub name: String,
    pub age: u8,
    pub current_position: u8,
    pub club_position: u8,
    pub individual_instructions: u8,
    pub availability: Availability,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Club {
    pub id: u16,
    pub name: String,
    pub first_team: [Option<u16>; 20],
    pub reserves: [Option<u16>; 20],
    pub team_instructions: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Origin {
    pub research_commit: String,
    pub record_sha256: String,
    pub save_sha256: String,
    pub league_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Campaign {
    pub schema: String,
    pub version: u32,
    pub origin: Origin,
    pub revision: u64,
    pub namespace: Namespace,
    pub season: Season,
    pub week_index: u8,
    pub manager: Manager,
    pub club: Club,
    pub cash: i32,
    pub next_fixture: Fixture,
    pub players: Vec<Player>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceClub {
    id: u16,
    name: String,
    first_team: [u16; 20],
    reserves: [u16; 20],
    team_instructions: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    schema: String,
    namespace: Namespace,
    season: Season,
    week_index: u8,
    manager: Manager,
    club: SourceClub,
    cash: i32,
    next_fixture: Fixture,
    players: Vec<Player>,
}
fn source_record() -> Result<Value, String> {
    let raw = include_bytes!("../fixtures/research-seed.json");
    if format!("{:x}", Sha256::digest(raw)) != SOURCE_SHA {
        return Err("original seed source identity differs".into());
    }
    serde_json::from_slice(raw).map_err(|e| e.to_string())
}
fn initial_source() -> Result<Source, String> {
    serde_json::from_value(source_record()?["initial"].clone()).map_err(|e| e.to_string())
}
fn origin() -> Origin {
    Origin {
        research_commit: "39e289a393d12ce562ee8b9fba16b413fa17647e".into(),
        record_sha256: SOURCE_SHA.into(),
        save_sha256: "6ef61bcf78af17a78cbbdc50b68839be169552464b4f2ee9eec5ea9b3740bc36".into(),
        league_sha256: "fc4bfa045b71ddede9692f40d6a4c0e3b29cc7334c1ec6e910c275f63f714352".into(),
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    First(usize),
    Reserve(usize),
}
impl Campaign {
    pub fn initial() -> Result<Self, String> {
        let s = initial_source()?;
        let c = Self {
            schema: "onenil-campaign".into(),
            version: 1,
            origin: origin(),
            revision: 0,
            namespace: s.namespace,
            season: s.season,
            week_index: s.week_index,
            manager: s.manager,
            club: Club {
                id: s.club.id,
                name: s.club.name,
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
            next_fixture: s.next_fixture,
            players: s.players,
        };
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != "onenil-campaign" || self.version != 1 {
            return Err("unsupported native campaign schema/version".into());
        }
        let source = initial_source()?;
        if self.origin != origin() || self.namespace != source.namespace {
            return Err("campaign source/league namespace differs".into());
        }
        if self.season != source.season
            || self.week_index != source.week_index
            || self.manager != source.manager
            || self.club.id != source.club.id
            || self.club.name != source.club.name
            || self.next_fixture != source.next_fixture
        {
            return Err("campaign context outside the supported saved club/fixture".into());
        }
        let expected: BTreeSet<_> = source.players.iter().map(|p| p.id).collect();
        let roster: Vec<_> = self
            .club
            .first_team
            .iter()
            .chain(&self.club.reserves)
            .copied()
            .flatten()
            .collect();
        let ids: BTreeSet<_> = roster.iter().copied().collect();
        if roster.len() != ids.len() || ids != expected {
            return Err("duplicate, missing or foreign roster identity".into());
        }
        let player_ids: BTreeSet<_> = self.players.iter().map(|p| p.id).collect();
        if self.players.len() != player_ids.len() || player_ids != expected {
            return Err("duplicate, missing or foreign player record".into());
        }
        for player in &self.players {
            let original = source
                .players
                .iter()
                .find(|p| p.id == player.id)
                .ok_or("unknown player")?;
            if player.name != original.name || player.club_position != original.club_position {
                return Err("canonical player/club-position identity differs".into());
            }
            if player.current_position > 13 {
                return Err("position outside the14 observed role codes".into());
            }
        }
        Ok(())
    }
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        self.validate()?;
        serde_json::to_vec_pretty(self).map_err(|e| e.to_string())
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > 1024 * 1024 {
            return Err("campaign file exceeds native size limit".into());
        }
        let c: Self = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        c.validate()?;
        Ok(c)
    }
    pub fn replace_from_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        let candidate = Self::decode(bytes)?;
        *self = candidate;
        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self, String> {
        storage::load(path)
    }
    pub fn save(&self, path: &Path) -> Result<storage::SaveOutcome, String> {
        storage::save(self, path)
    }
    pub fn at(&self, location: Location) -> Result<Option<u16>, String> {
        match location {
            Location::First(i) => self.club.first_team.get(i),
            Location::Reserve(i) => self.club.reserves.get(i),
        }
        .copied()
        .ok_or_else(|| "roster location outside20slots".into())
    }
    fn put(&mut self, location: Location, value: Option<u16>) {
        match location {
            Location::First(i) => self.club.first_team[i] = value,
            Location::Reserve(i) => self.club.reserves[i] = value,
        }
    }
    fn transaction(
        &mut self,
        edit: impl FnOnce(&mut Self) -> Result<(), String>,
    ) -> Result<(), String> {
        self.validate()?;
        let mut next = self.clone();
        edit(&mut next)?;
        if next == *self {
            return Ok(());
        }
        next.revision = self
            .revision
            .checked_add(1)
            .ok_or("campaign revision exhausted")?;
        next.validate()?;
        *self = next;
        Ok(())
    }
    pub fn exchange(&mut self, source: Location, destination: Location) -> Result<(), String> {
        self.transaction(|next| {
            let a = next.at(source)?;
            let b = next.at(destination)?;
            if a.is_none() {
                return Err("empty move source".into());
            }
            next.put(source, b);
            next.put(destination, a);
            Ok(())
        })
    }
    pub fn set_position(&mut self, player: u16, position: u8) -> Result<(), String> {
        self.transaction(|next| {
            next.players
                .iter_mut()
                .find(|p| p.id == player)
                .ok_or("unknown player")?
                .current_position = position;
            Ok(())
        })
    }
    pub fn set_instructions(&mut self, player: u16, value: u8) -> Result<(), String> {
        self.transaction(|next| {
            next.players
                .iter_mut()
                .find(|p| p.id == player)
                .ok_or("unknown player")?
                .individual_instructions = value;
            Ok(())
        })
    }
    pub fn source_projection(&self) -> Result<Value, String> {
        self.validate()?;
        let mut players = self.players.clone();
        players.sort_by_key(|p| p.id);
        Ok(
            serde_json::json!({"schema":"original-club-seed/1","namespace":self.namespace,"season":self.season,"week_index":self.week_index,"manager":self.manager,"club":{"id":self.club.id,"name":self.club.name,"first_team":self.club.first_team.map(|p|p.unwrap_or(0)),"reserves":self.club.reserves.map(|p|p.unwrap_or(0)),"team_instructions":self.club.team_instructions},"cash":self.cash,"next_fixture":self.next_fixture,"players":players}),
        )
    }
}
