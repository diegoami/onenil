use onenil_engine::watched_compiled::prepared::{Input, Session};
use serde::Deserialize;
use std::io::{self, BufRead};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    input: Input,
    #[serde(default)]
    prefix_steps: usize,
}
fn main() {
    for line in io::stdin().lock().lines() {
        let request: Request = serde_json::from_str(&line.unwrap()).unwrap();
        match Session::new(request.input) {
            Err(error) => println!("{}", serde_json::json!({"error":{"message":error}})),
            Ok(mut session) => {
                let mut failed = None;
                for _ in 0..request.prefix_steps {
                    if let Err(error) = session.advance() {
                        failed = Some(error);
                        break;
                    }
                }
                let result = if let Some(error) = failed {
                    serde_json::json!({"error":{"message":error}})
                } else {
                    match session.finish() {
                        Ok(ok) => serde_json::json!({"ok":ok}),
                        Err(error) => serde_json::json!({"error":error}),
                    }
                };
                println!("{}", result);
            }
        }
    }
}
