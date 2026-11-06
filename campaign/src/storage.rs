//! [designed] Native file transaction: exclusive temporary write, sync, atomic replace, dir sync.
use crate::Campaign;
use serde::Serialize;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
#[derive(Debug, Serialize)]
pub struct SaveOutcome {
    pub committed: bool,
    pub directory_synced: bool,
    pub durability_note: Option<String>,
}
fn parent(path: &Path) -> &Path {
    path.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}
fn temporary(path: &Path) -> Result<PathBuf, String> {
    let name = path
        .file_name()
        .ok_or("save target has no filename")?
        .to_string_lossy();
    Ok(parent(path).join(format!(
        ".{name}.pending-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    )))
}
pub fn load(path: &Path) -> Result<Campaign, String> {
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|e| e.to_string())?
        .take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    Campaign::decode(&bytes)
}
pub fn save(campaign: &Campaign, path: &Path) -> Result<SaveOutcome, String> {
    let bytes = campaign.encode()?;
    let temporary = temporary(path)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| e.to_string())?;
    let write = (|| {
        file.write_all(&bytes)?;
        file.sync_all()?;
        fs::rename(&temporary, path)
    })();
    if let Err(error) = write {
        let _ = fs::remove_file(&temporary);
        return Err(error.to_string());
    }
    // Rename already committed the complete state. A later directory sync error is a durability
    // warning, never a false claim that the previous file still owns the target.
    let sync = File::open(parent(path)).and_then(|directory| directory.sync_all());
    Ok(SaveOutcome {
        committed: true,
        directory_synced: sync.is_ok(),
        durability_note: sync.err().map(|e| e.to_string()),
    })
}
