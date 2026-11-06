//! [designed] Headless native campaign application and deterministic persistence probe.
use onenil_campaign::{storage::PendingSave, Campaign, Location};
use std::{
    io::{self, Write},
    path::Path,
};
fn location(text: &str) -> Result<Location, String> {
    let (kind, index) = text
        .split_once(':')
        .ok_or("use first:INDEX or reserve:INDEX")?;
    let index = index.parse().map_err(|_| "invalid roster index")?;
    match kind {
        "first" => Ok(Location::First(index)),
        "reserve" => Ok(Location::Reserve(index)),
        _ => Err("unknown roster list".into()),
    }
}
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        return Err("new|inspect|project FILE; swap FILE first:I reserve:J; position|instruction FILE PLAYER VALUE; stage FILE SOURCE DEST".into());
    }
    let path = Path::new(&args[1]);
    let mut state = if args[0] == "new" {
        if path.exists() {
            return Err("new campaign target already exists".into());
        }
        Campaign::initial()?
    } else {
        Campaign::load(path)?
    };
    match args[0].as_str() {
        "new" => (),
        "load" if args.len() == 3 => {
            state = Campaign::load(Path::new(&args[2]))?;
        }
        "inspect" => {
            println!(
                "{}",
                serde_json::to_string(&state).map_err(|e| e.to_string())?
            );
            return Ok(());
        }
        "project" => {
            println!("{}", state.source_projection()?);
            return Ok(());
        }
        "swap" | "stage" if args.len() == 4 => {
            state.exchange(location(&args[2])?, location(&args[3])?)?
        }
        "position" | "instruction" if args.len() == 4 => {
            let player = args[2].parse().map_err(|_| "invalid player identity")?;
            let value = args[3].parse().map_err(|_| "value outside byte domain")?;
            if args[0] == "position" {
                state.set_position(player, value)?;
            } else {
                state.set_instructions(player, value)?;
            }
        }
        _ => return Err("unsupported campaign command/arguments".into()),
    }
    if args[0] == "stage" {
        let staged = PendingSave::prepare(&state, path)?;
        println!(
            "{}",
            serde_json::json!({"stage":"synced-before-replace","temporary":staged.pending_path()})
        );
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .map_err(|e| e.to_string())?;
        if line.trim() != "commit" {
            return Err("staged save cancelled".into());
        }
        println!(
            "{}",
            serde_json::to_string(&staged.commit()?).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    println!(
        "{}",
        serde_json::to_string(&state.save(path)?).map_err(|e| e.to_string())?
    );
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
