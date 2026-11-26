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
pub struct Report {
    pub steps: usize,
    pub setup_draws: usize,
    pub step_draws: usize,
    pub half_time: Option<usize>,
    pub goals: Vec<Value>,
    pub events: Vec<Value>,
    /// One row per accepted step: elapsed minute, score pair and events.
    pub timeline: Vec<Value>,
    pub score: Value,
    pub snapshot: Value,
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
    Ok(Report {
        steps,
        setup_draws,
        step_draws,
        half_time,
        goals,
        events,
        timeline,
        score: session.match_state.result()?,
        snapshot: session.match_state.snapshot(),
    })
}
