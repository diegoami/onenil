use onenil_engine::watched_compiled::prepared::{Input, Session};
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let request: serde_json::Value = serde_json::from_str(&line.unwrap()).unwrap();
        let input: Input = serde_json::from_value(request["input"].clone()).unwrap();
        let result = (|| {
            let mut s = if request["fresh"].as_bool().unwrap_or(false) {
                Session::new_fresh(input)?
            } else {
                Session::new(input)?
            };
            let first = s.snapshot()?;
            if request["finish"].as_bool().unwrap_or(false) {
                let complete = s.finish().map_err(|error| {
                    format!("{};missing={:?}", error.fault.message, error.fault.missing)
                })?;
                return Ok(serde_json::json!({"first_entry": first, "finished": complete}));
            }
            let mut clocks = vec![];
            for _ in 0..request["steps"].as_u64().unwrap_or(0) {
                clocks.push(s.advance()?);
            }
            Ok::<_, String>(
                serde_json::json!({"first_entry":first,"observation":s.observation(),"clocks":clocks}),
            )
        })();
        println!(
            "{}",
            match result {
                Ok(out) => serde_json::json!({"ok":out}),
                Err(e) => serde_json::json!({"error":e}),
            }
        );
    }
}
