//! [designed] Adapter owns the controller, paths and exactly one cancellable worker.
use crate::{
    controller::{Controller, Key, Screen},
    worker::{Job, Poll},
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
pub struct Application {
    pub controller: Controller,
    pub root: PathBuf,
    pub save_path: PathBuf,
    pub job: Option<Job>,
    last_step: Instant,
    save_candidates: Vec<PathBuf>,
}
impl Application {
    pub fn new(root: PathBuf, save_path: PathBuf, load: bool) -> Result<Self, String> {
        let mut controller = Controller::default();
        if load {
            if let Err(e) = controller.editor.load(&save_path) {
                controller.message = format!("Load refused: {e}");
            }
        }
        controller.game_name = game_name(&save_path);
        Ok(Self {
            controller,
            root,
            save_path,
            job: None,
            save_candidates: Vec::new(),
            last_step: Instant::now(),
        })
    }
    pub fn key(&mut self, key: Key) {
        self.controller.key(key);
        if self.controller.screen != Screen::Preparing {
            if let Some(mut job) = self.job.take() {
                if let Err(e) = job.cancel() {
                    self.controller.message = e;
                }
            }
        }
        if self.controller.save_requested {
            self.controller.save_requested = false;
            self.controller.message = match self
                .save_path
                .parent()
                .map(std::fs::create_dir_all)
                .transpose()
                .map_err(|e| e.to_string())
                .and_then(|_| self.controller.editor.save(&self.save_path))
            {
                Ok(outcome) => {
                    if outcome.directory_synced {
                        "Native campaign saved (F9 reloads).".into()
                    } else {
                        "Native campaign saved; directory durability not confirmed.".into()
                    }
                }
                Err(e) => format!("Save refused: {e}"),
            };
        }
        if self.controller.load_requested {
            self.controller.load_requested = false;
            if self.controller.screen == Screen::SavedGames {
                if let Some(path) = self
                    .save_candidates
                    .get(self.controller.load_index)
                    .cloned()
                {
                    self.restore(path);
                }
            } else {
                self.choose_save();
            }
        }
    }
    /// [designed] Discover regular native JSON saves in the configured directory.
    /// [confirmed] W001/W006: one save directly, otherwise first selected list.
    pub fn choose_save(&mut self) {
        let directory = self
            .save_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(std::path::Path::new("."));
        let discovered = (|| -> Result<Vec<PathBuf>, String> {
            let entries = match std::fs::read_dir(directory) {
                Ok(entries) => entries,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
                Err(e) => return Err(e.to_string()),
            };
            let mut paths = Vec::new();
            for entry in entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if entry.file_type().map_err(|e| e.to_string())?.is_file()
                    && path
                        .extension()
                        .is_some_and(|s| s.eq_ignore_ascii_case("json"))
                {
                    paths.push(path);
                    if paths.len() > 1024 {
                        return Err("more than 1024 native save files".into());
                    }
                }
            }
            paths.sort_by_key(|p| (game_name(p).to_lowercase(), p.clone()));
            Ok(paths)
        })();
        let paths = match discovered {
            Ok(paths) => paths,
            Err(e) => {
                self.controller.message = format!("Save list refused: {e}");
                return;
            }
        };
        self.controller.load_entries = paths.iter().map(|p| game_name(p)).collect();
        self.controller.load_index = 0;
        self.save_candidates = paths;
        match self.save_candidates.len() {
            0 => self.controller.message = "No native saves in the configured directory.".into(),
            1 => self.restore(self.save_candidates[0].clone()),
            _ => {
                self.controller.screen = Screen::SavedGames;
                self.controller.message.clear();
            }
        }
    }
    fn restore(&mut self, path: PathBuf) {
        match self.controller.editor.load(&path) {
            Ok(()) => {
                // Editor validates the complete candidate before replacing live state.
                self.controller.game_name = game_name(&path);
                self.save_path = path;
                self.controller.screen = Screen::Main;
                self.controller.load_entries.clear();
                self.save_candidates.clear();
                self.controller.exhibition = false;
                self.controller.message = "Native campaign restored.".into();
            }
            Err(e) => self.controller.message = format!("Load refused: {e}"),
        }
    }
    pub fn update(&mut self) {
        if self.controller.prepare_requested && self.job.is_none() {
            let id = self.controller.job_id;
            match std::env::current_exe()
                .map_err(|e| e.to_string())
                .and_then(|exe| Job::spawn(&exe, &self.root, id))
            {
                Ok(job) => self.job = Some(job),
                Err(e) => {
                    self.controller.accept_error(id, e);
                }
            }
        }
        if let Some(job) = self.job.as_mut() {
            match job.poll() {
                Poll::Pending => (),
                Poll::Ready(id, t) => {
                    self.controller.accept_timeline(id, t);
                    self.job = None;
                    self.last_step = Instant::now();
                }
                Poll::Failed(id, e) => {
                    self.controller.accept_error(id, e);
                    self.job = None;
                }
            }
        }
        if self.last_step.elapsed() >= Duration::from_millis(16) {
            self.controller.tick();
            self.last_step = Instant::now();
        }
    }
}
impl Application {
    /// [designed] Optional read-only diagnostic snapshot; never accepts state or commands.
    pub fn snapshot(&self) -> serde_json::Value {
        let c = &self.controller;
        serde_json::json!({"save_path":self.save_path,"game_name":c.game_name,"load_entries":c.load_entries,"load_index":c.load_index,"menus":[c.menu,c.preparation_menu,c.option],"speed":c.speed,"close_requested":c.close_requested,"screen":format!("{:?}",c.screen),"panel":format!("{:?}",c.editor.panel),"selected":format!("{:?}",c.editor.selected),"card":c.editor.card(),"carried":c.editor.carried_player(),"first":c.editor.campaign.club.first_team,"reserve":c.editor.campaign.club.reserves,"campaign":c.editor.campaign,"exhibition":c.exhibition,"job":c.job_id,"worker_pid":self.job.as_ref().map(|j|j.pid()),"phase":c.playback.as_ref().map(|p|format!("{:?}",p.phase)),"frames":c.playback.as_ref().map(|p|p.cursor),"events":c.playback.as_ref().map(|p|p.events.len()),"score":c.playback.as_ref().map(|p|p.view().score),"players":c.playback.as_ref().map(|p|p.view().players.len()),"message":c.message})
    }
}

fn game_name(path: &std::path::Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .chars()
        .map(|ch| if ch.is_control() { '?' } else { ch })
        .collect()
}
