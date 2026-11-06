//! [designed] Exclusive staged write, sync, atomic replacement, then explicit directory durability.
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
pub struct PendingSave {
    target: PathBuf,
    temporary: PathBuf,
    committed: bool,
}
impl PendingSave {
    pub fn prepare(campaign: &Campaign, path: &Path) -> Result<Self, String> {
        let bytes = campaign.encode()?;
        let name = path
            .file_name()
            .ok_or("save target has no filename")?
            .to_string_lossy();
        let temporary = parent(path).join(format!(
            ".{name}.pending-{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        let staged = Self {
            target: path.to_owned(),
            temporary,
            committed: false,
        };
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        Ok(staged)
    }
    pub fn pending_path(&self) -> &Path {
        &self.temporary
    }
    pub fn commit(mut self) -> Result<SaveOutcome, String> {
        fs::rename(&self.temporary, &self.target).map_err(|e| e.to_string())?;
        self.committed = true;
        // The rename already committed complete state. A directory-sync error is a durability
        // warning, never a false claim that the old file still owns the target.
        let sync = File::open(parent(&self.target)).and_then(|directory| directory.sync_all());
        Ok(SaveOutcome {
            committed: true,
            directory_synced: sync.is_ok(),
            durability_note: sync.err().map(|e| e.to_string()),
        })
    }
}
impl Drop for PendingSave {
    fn drop(&mut self) {
        if !self.committed {
            let _ = fs::remove_file(&self.temporary);
        }
    }
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
    PendingSave::prepare(campaign, path)?.commit()
}
