//! [designed:T74] Verification transport, never a source-state replay backend.
use onenil_engine::watched_compiled::prepared::{Input, Session};
use serde::Deserialize;
use std::io::{self, Read};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    input: Input,
    profile: String,
    cancel_first: bool,
}
fn run(request: Request) -> Result<serde_json::Value, String> {
    let mut session = match request.profile.as_str() {
        "fresh" => Session::new_fresh(request.input),
        "edited" => Session::new_edited(request.input),
        _ => return Err("unsupported interval profile".into()),
    }?;
    let interval = session.pause_at_interval()?;
    if interval != session.interval_observation()? {
        return Err("interval inspection changed".into());
    }
    let ok = session.finish_with_interval_replacement(request.cancel_first)?;
    if session.finish().is_ok() || session.pause_at_interval().is_ok() || session.advance().is_ok()
    {
        return Err("finished interval session advanced again".into());
    }
    Ok(serde_json::json!({"interval":interval,"ok":ok}))
}
fn main() {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line).unwrap();
    let result = serde_json::from_str::<Request>(&line)
        .map_err(|e| e.to_string())
        .and_then(run);
    println!(
        "{}",
        match result {
            Ok(value) => value,
            Err(error) => serde_json::json!({"error":error}),
        }
    );
}
