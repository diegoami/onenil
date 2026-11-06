use std::path::Path;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let result = if args.len() == 4 && args[1] == "--worker" {
        args[3]
            .parse()
            .map_err(|e| format!("invalid job identity: {e}"))
            .and_then(|id| onenil_desktop::worker::run(Path::new(&args[2]), id))
    } else {
        Err("Desktop core only: native frontend is the next task (T53).".into())
    };
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
