use onenil_engine::watched_compiled::{action::State, watched_match};
use serde::Deserialize;
use std::io::{self, BufRead};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    initial: State,
    no_pending_key: bool,
}
fn main() {
    for line in io::stdin().lock().lines() {
        let r: Request = serde_json::from_str(&line.unwrap()).unwrap();
        let result = match watched_match::run(&r.initial, r.no_pending_key) {
            Ok(ok) => serde_json::json!({"ok":ok}),
            Err(error) => serde_json::json!({"error":error}),
        };
        println!("{result}");
    }
}
