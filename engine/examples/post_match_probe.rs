//! T84 release probe: input-only selected admission and owned continuation.
use onenil_engine::watched_compiled::{
    owned_post_match,
    prepared::{Input, Session},
};
use serde::Deserialize;
use serde_json::json;
use std::io::{self, Read};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    input: Input,
    admission: owned_post_match::Admission,
    prefix_steps: usize,
}
fn main() -> Result<(), String> {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    let request: Request = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if ![0, 171].contains(&request.prefix_steps) {
        return Err("unproven probe prefix".into());
    }
    let mut session = Session::new_with_post_match(request.input, request.admission)?;
    for _ in 0..request.prefix_steps {
        session.advance().map_err(|e| format!("{e:?}"))?;
    }
    let got = session.finish_with_post_match()?;
    println!(
        "{}",
        json!({"ticks":got["match"]["ticks"].as_array().unwrap().len(),
        "match_draws":got["match"]["draws"].as_array().unwrap().len(),
        "actions":got["match"]["events"].as_array().unwrap().len(),
        "match_return":got["match"]["return"],"post_match":got["post_match"],"admission":got["admission"]})
    );
    Ok(())
}
