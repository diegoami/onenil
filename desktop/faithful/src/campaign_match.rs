//! [derived:T65/T66/T68] The exact supplied campaign owns this fresh match request.
//! [designed] Supported profile chosen from owned campaign; foreign edits refuse before a worker.
use onenil_campaign::Campaign;
use onenil_engine::watched_compiled::prepared::Input;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Original,
    Edited,
}

pub const REFUSAL: &str = "Use original team or swap Pearce and Hendry. Reload the original save.";

pub fn profile(campaign: &Campaign) -> Result<Profile, String> {
    campaign.validate()?;
    if campaign.played_fixture.is_some() {
        return Err(
            "This fixture already has a result. Week progression is not available yet.".into(),
        );
    }
    let mut initial = Campaign::initial()?;
    if campaign == &initial {
        return Ok(Profile::Original);
    }
    // [confirmed:T69 original keys;T68 full parity] One exact owned edit.
    initial.exchange(
        onenil_campaign::Location::First(1),
        onenil_campaign::Location::First(2),
    )?;
    if campaign == &initial {
        return Ok(Profile::Edited);
    }
    Err(REFUSAL.into())
}

pub fn validate(campaign: &Campaign) -> Result<(), String> {
    profile(campaign).map(|_| ())
}

pub fn metadata() -> Result<Value, String> {
    let raw = include_bytes!("../assets/campaign-match-names.json");
    if format!("{:x}", Sha256::digest(raw))
        != "a9100aabe62c67162a717864392fd8a7c30dd40b4032d5c2665a1253bd6f1d6c"
    {
        return Err("campaign match name identity differs".into());
    }
    serde_json::from_slice(raw).map_err(|error| error.to_string())
}

pub fn admission(campaign: &Campaign) -> Result<Input, String> {
    let profile = profile(campaign)?;
    let (source_raw, support_raw) = match profile {
        Profile::Original => (
            include_str!("../../../engine/tests/fixtures/fresh_preparation/admission.json"),
            include_str!("../../../engine/tests/fixtures/fresh_watched/incoming.json"),
        ),
        Profile::Edited => (
            include_str!("../../../engine/tests/fixtures/edited_preparation/admission.json"),
            include_str!("../../../engine/tests/fixtures/edited_watched/incoming.json"),
        ),
    };
    let source: Value = serde_json::from_str(source_raw).map_err(|e| e.to_string())?;
    let support: Value = serde_json::from_str(support_raw).map_err(|e| e.to_string())?;
    let mut input: Input = serde_json::from_value(json!({
        "preparation":source["input"], "support":support["support"], "no_pending_key":true
    }))
    .map_err(|error| error.to_string())?;
    let setup = &mut input.preparation.setup;
    let legacy = &mut setup.legacy;
    let roster = campaign.club.first_team.map(|id| id.unwrap_or(0));
    let club = legacy
        .substitution
        .context
        .clubs
        .iter_mut()
        .find(|club| club.id == campaign.club.id)
        .ok_or("missing active campaign club")?;
    club.roster = roster;
    legacy.history.club = campaign.club.id;
    // [confirmed:T65 fixture layout;T66 E24C] The next fixture slot is the
    // watched preparation week. Other setup scalars remain authenticated inputs.
    legacy.config[0] = campaign.next_fixture.slot;
    for player in &campaign.players {
        let row = legacy
            .players
            .iter_mut()
            .find(|row| row.id == player.id)
            .ok_or("campaign player lacks original preparation input")?;
        row.p8[0] = campaign.club.id as u8;
        row.p8[1] = player.age;
        row.p8[6] = player.current_position;
        row.p8[7] = player.club_position;
        row.p15[8] = player.availability.condition_signed as u8;
        row.p15[10] = player.individual_instructions;
        row.p15[11] = player.availability.status_bits;
    }
    for row in &mut setup.income {
        if row.club == campaign.club.id {
            row.fixture = campaign.next_fixture.competition_code;
            row.fixture_flags = campaign.next_fixture.flags;
        } else if row.club == campaign.next_fixture.opponent {
            row.fixture = campaign.next_fixture.competition_code;
            row.fixture_flags = campaign.next_fixture.opponent_flags;
        } else {
            return Err("foreign campaign fixture preparation input".into());
        }
    }
    let names = metadata()?;
    for player in &campaign.players {
        if names["players"][player.id.to_string()] != player.name {
            return Err("active campaign player/name namespace differs".into());
        }
    }
    Ok(input)
}
