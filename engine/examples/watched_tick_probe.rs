use onenil_engine::watched_compiled::{action::State, tick::TickSession};
use serde::Deserialize;
use std::io::{self, BufRead};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    initial: State,
    steps: usize,
    no_pending_key: bool,
}
fn main() {
    for line in io::stdin().lock().lines() {
        let value: Request = serde_json::from_str(&line.unwrap()).unwrap();
        let result = match TickSession::admit(&value.initial, value.no_pending_key) {
            Err(e) => serde_json::json!({"error":e}),
            Ok(mut s) => {
                let mut outputs = Vec::new();
                let mut error = None;
                for _ in 0..value.steps {
                    match s.advance() {
                        Ok(o) => outputs.push(o),
                        Err(e) => {
                            error = Some(e);
                            break;
                        }
                    }
                }
                if let Some(e) = error {
                    serde_json::json!({"error":e})
                } else {
                    serde_json::json!({"ok":outputs})
                }
            }
        };
        println!("{}", result);
    }
}
