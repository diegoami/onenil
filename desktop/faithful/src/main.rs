use std::path::{Path, PathBuf};
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 4 && args[1] == "--worker" {
        return args[3]
            .parse()
            .map_err(|e| format!("invalid job identity: {e}"))
            .and_then(|id| onenil_desktop::worker::run(Path::new(&args[2]), id));
    }
    let argument = |name: &str| -> Result<Option<PathBuf>, String> {
        match args.iter().position(|x| x == name) {
            Some(i) => Ok(Some(PathBuf::from(
                args.get(i + 1)
                    .ok_or_else(|| format!("missing value for {name}"))?,
            ))),
            None => Ok(None),
        }
    };
    let root = argument("--root")?.unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned()
    });
    let font =
        argument("--font")?.unwrap_or_else(|| root.join(".cache/T51-font/PxPlus_IBM_VGA_8x16.ttf"));
    let save =
        argument("--save")?.unwrap_or_else(|| root.join(".cache/desktop-experiment/squad.json"));
    let load = args.iter().any(|s| s == "--load");
    #[cfg(feature = "egui")]
    if !cfg!(feature = "software") || args.iter().any(|s| s == "--egui") {
        return onenil_desktop::egui_window::run(root, font, save, load, argument("--observe")?);
    }
    #[cfg(feature = "software")]
    {
        onenil_desktop::software::run(root, font, save, load, argument("--observe")?)
    }
    #[cfg(all(feature = "egui", not(feature = "software")))]
    unreachable!();
    #[cfg(not(any(feature = "software", feature = "egui")))]
    {
        let _ = (root, font, save, load);
        Err("Build with --features software for the native window.".into())
    }
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
