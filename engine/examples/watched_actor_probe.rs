//! [designed] JSON-line harness for strict native action kernels;no game model.
use onenil_engine::watched_compiled::action::{self, State};
use std::io::{self, BufRead, Write};
fn main() {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line.expect("read JSON input");
        let value = match serde_json::from_str::<State>(&line) {
            Ok(state) => match action::advance(&state) {
                Ok(out) => serde_json::json!({"ok":out}),
                Err(fault) => serde_json::json!({"error":fault}),
            },
            Err(error) => serde_json::json!({"schema_error":error.to_string()}),
        };
        serde_json::to_writer(&mut stdout, &value).expect("serialize native result");
        writeln!(&mut stdout).unwrap();
        stdout.flush().unwrap();
    }
}
