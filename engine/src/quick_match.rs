//! [confirmed] DD02/7B7B quick match: archaeology F07/F12–F23, T12/T13/T16/T21/T36;
//! the T100 league admission T119 and its original action stream T123.
//! [designed] One prepared quick session from the preparation state to the outer
//! return; the watched branch, renderer and week transaction stay outside.
use crate::{setup, watched_compiled::prepared};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    League,
    Fresh,
}

/// Admit the quick match from the same preparation/support JSON as the watched
/// session. Watch off is the admitted setting: the supplied preparation may have
/// been captured with Watch on, and the gate only selects the step body.
pub fn session(profile: Profile, input_json: &str) -> Result<setup::Session, String> {
    let input: prepared::Input =
        serde_json::from_str(input_json).map_err(|error| error.to_string())?;
    if !input.no_pending_key {
        return Err("quick match requires explicit no-pending-key admission".into());
    }
    let cells = prepared::support_cells(&input.support)?;
    match profile {
        Profile::League => setup::Session::before_league_quick_call(
            input.preparation.setup,
            input.preparation.incoming,
            &cells,
        ),
        Profile::Fresh => setup::Session::before_fresh_quick_call(
            input.preparation.setup,
            input.preparation.incoming,
            &cells,
        ),
    }
}

#[derive(Serialize)]
pub struct GoalRecord {
    pub minute: u16,
    pub actor: u8,
    pub side_flag: u8,
    pub player: u16,
    pub club: u16,
}

#[derive(Serialize)]
pub struct SubstitutionRecord {
    pub step: usize,
    pub minute: u16,
    pub scene: u8,
    pub off: u16,
    pub on: u16,
    pub club: u16,
}

#[derive(Serialize)]
pub struct Records {
    pub goals: Vec<GoalRecord>,
    pub substitutions: Vec<SubstitutionRecord>,
}

#[derive(Serialize)]
pub struct Report {
    pub steps: usize,
    pub setup_draws: usize,
    pub step_draws: usize,
    pub half_time: Option<usize>,
    pub goals: Vec<Value>,
    pub events: Vec<Value>,
    /// One row per accepted step: elapsed minute, score pair and events.
    pub timeline: Vec<Value>,
    /// [confirmed:T123/T49] The original scorer and substitution player records.
    pub records: Records,
    pub score: Value,
    pub snapshot: Value,
}

fn scene_players(session: &setup::Session) -> Result<std::collections::BTreeMap<u8, u16>, String> {
    let snapshot = session.match_state.snapshot();
    let mut players = std::collections::BTreeMap::new();
    for row in snapshot["players"]
        .as_array()
        .ok_or("missing scene players")?
    {
        let scene = row["scene_slot"].as_u64().ok_or("missing scene slot")? as u8;
        let player = row["player"].as_u64().ok_or("missing scene player")? as u16;
        players.insert(scene, player);
    }
    Ok(players)
}

fn player_club(session: &setup::Session, player: u16) -> u16 {
    let e = &session.match_state.inner.e;
    u16::from(e.image[crate::layout::P8 + 8 * usize::from(player)])
}

/// Play the admitted quick match to full time. The halftime board is answered
/// without lineup changes, as the original quick screen does.
pub fn play(session: &mut setup::Session) -> Result<Report, String> {
    let preparation = session.preparation();
    let setup_draws = preparation["draws"].as_array().map_or(0, Vec::len);
    let mut steps = 0;
    let mut step_draws = 0;
    let mut half_time = None;
    let mut goals = vec![];
    let mut events = vec![];
    let mut timeline = vec![];
    let mut substitutions: Vec<SubstitutionRecord> = vec![];
    let mut players = scene_players(session)?;
    loop {
        match session.match_state.phase() {
            "full-time" => break,
            "half-time" => {
                session.match_state.continue_without_changes()?;
                half_time = Some(steps);
                continue;
            }
            _ => {}
        }
        let out = session.match_state.step()?;
        steps += 1;
        let draws = out["draws"].as_array().map_or(0, Vec::len);
        step_draws += draws;
        let elapsed = out["input"]["clock"]["elapsed"].as_u64().unwrap_or(0) as u16;
        let after = scene_players(session)?;
        for (scene, on) in &after {
            if let Some(off) = players.get(scene) {
                if off != on {
                    substitutions.push(SubstitutionRecord {
                        step: steps,
                        minute: elapsed,
                        scene: *scene,
                        off: *off,
                        on: *on,
                        club: player_club(session, *on),
                    });
                }
            }
        }
        players = after;
        let step_events = out["events"].as_array().cloned().unwrap_or_default();
        for event in &step_events {
            if event["kind"] == "goal" {
                goals.push(event.clone());
            }
        }
        timeline.push(serde_json::json!({
            "step": steps,
            "elapsed": out["input"]["clock"]["elapsed"],
            "draws": draws,
            "events": step_events,
            "score": out["output"]["score_side"],
        }));
        events.extend(step_events);
    }
    let snapshot = session.match_state.snapshot();
    let logs = &snapshot["scorer_logs"];
    let mut records = Records {
        goals: vec![],
        substitutions,
    };
    for goal in &goals {
        let minute = goal["event"]["minute"].as_u64().unwrap_or(0) as u16;
        let actor = goal["event"]["actor"].as_u64().unwrap_or(0) as u8;
        let side = goal["event"]["side_flag"].as_u64().unwrap_or(0) as usize;
        let player = logs[side]
            .as_array()
            .and_then(|entries| {
                entries
                    .iter()
                    .find(|entry| entry["minute_byte"].as_u64() == Some(u64::from(minute)))
            })
            .and_then(|entry| entry["player"].as_u64())
            .unwrap_or(0) as u16;
        records.goals.push(GoalRecord {
            minute,
            actor,
            side_flag: side as u8,
            player,
            club: player_club(session, player),
        });
    }
    Ok(Report {
        steps,
        setup_draws,
        step_draws,
        half_time,
        goals,
        events,
        timeline,
        records,
        score: session.match_state.result()?,
        snapshot,
    })
}
