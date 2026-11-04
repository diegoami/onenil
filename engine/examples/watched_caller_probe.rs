use onenil_engine::watched_compiled::{action::State, caller::CallerSession};
use serde::Deserialize;
use std::io::{self, BufRead};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    initial: State,
    steps: usize,
    no_pending_key: bool,
    clock_only: bool,
}
fn main() {
    for line in io::stdin().lock().lines() {
        let r: Request = serde_json::from_str(&line.unwrap()).unwrap();
        let result = match CallerSession::admit(&r.initial, r.no_pending_key) {
            Err(e) => serde_json::json!({"error":e}),
            Ok(mut s) => {
                let mut outputs = Vec::new();
                let mut failure = None;
                for _ in 0..r.steps {
                    match if r.clock_only {
                        s.clock_boundary()
                    } else {
                        s.advance()
                    } {
                        Ok(v) => outputs.push(v),
                        Err(e) => {
                            failure = Some(e);
                            break;
                        }
                    }
                }
                if let Some(e) = failure {
                    serde_json::json!({"error":e})
                } else {
                    serde_json::json!({"ok":outputs})
                }
            }
        };
        println!("{}", result);
    }
}
