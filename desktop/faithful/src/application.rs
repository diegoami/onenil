//! [designed] Adapter owns the controller, paths and exactly one cancellable worker.
use crate::{
    controller::{Controller, Key, Screen},
    live_worker::{Decision, LiveJob, Poll as LivePoll},
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
    pub live_job: Option<LiveJob>,
    live_id: Option<u64>,
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
            live_job: None,
            live_id: None,
            save_candidates: Vec::new(),
            last_step: Instant::now(),
        })
    }
    pub fn key(&mut self, key: Key) {
        self.controller.key(key);
        self.sync_jobs();
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
    fn cancel_workers(&mut self) {
        if let Some(mut job) = self.job.take() {
            if let Err(error) = job.cancel() {
                self.controller.message = error;
            }
        }
        if let Some(mut job) = self.live_job.take() {
            if let Err(error) = job.cancel() {
                self.controller.message = error;
            }
        }
        self.live_id = None;
    }
    fn sync_jobs(&mut self) {
        if self.controller.screen != Screen::Preparing {
            if let Some(mut job) = self.job.take() {
                if let Err(error) = job.cancel() {
                    self.controller.message = error;
                }
            }
        }
        if self.live_job.is_some()
            && !self
                .live_id
                .is_some_and(|id| self.controller.owns_live_request(id))
        {
            if let Some(mut job) = self.live_job.take() {
                if let Err(error) = job.cancel() {
                    self.controller.message = error;
                }
            }
            self.live_id = None;
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
                self.cancel_workers();
                self.controller.clear_match();
                self.controller.job_id += 1;
                self.controller.screen = Screen::SavedGames;
                self.controller.message.clear();
            }
        }
    }
    fn restore(&mut self, path: PathBuf) {
        match self.controller.editor.load(&path) {
            Ok(()) => {
                // Editor validates the complete candidate before replacing live state.
                self.cancel_workers();
                self.controller.job_id += 1;
                self.controller.game_name = game_name(&path);
                self.save_path = path;
                self.controller.screen = Screen::Main;
                self.controller.load_entries.clear();
                self.save_candidates.clear();
                self.controller.clear_match();
                self.controller.message = "Native campaign restored.".into();
            }
            Err(e) => self.controller.message = format!("Load refused: {e}"),
        }
    }
    pub fn update(&mut self) {
        self.update_worker(std::env::current_exe().map_err(|e| e.to_string()));
    }
    /// [designed:T77] Real executable injection for launchers and headless lifecycle tests.
    pub fn update_with_executable(&mut self, executable: &std::path::Path) {
        self.update_worker(Ok(executable.to_owned()));
    }
    fn update_worker(&mut self, executable: Result<PathBuf, String>) {
        self.sync_jobs();
        if self.controller.prepare_requested && self.job.is_none() && self.live_job.is_none() {
            let id = self.controller.job_id;
            if self.controller.exhibition {
                match executable
                    .as_ref()
                    .map_err(Clone::clone)
                    .and_then(|exe| Job::spawn(exe, &self.root, id))
                {
                    Ok(job) => self.job = Some(job),
                    Err(error) => {
                        self.controller.accept_error(id, error);
                    }
                }
            } else {
                match executable.as_ref().map_err(Clone::clone).and_then(|exe| {
                    LiveJob::spawn_campaign(exe, id, &self.controller.editor.campaign)
                }) {
                    Ok(job) => {
                        self.live_job = Some(job);
                        self.live_id = Some(id);
                    }
                    Err(error) => {
                        self.controller.accept_live_error(id, error);
                    }
                }
            }
        }
        if self.controller.resume_requested || self.controller.replace_requested {
            self.controller.resume_requested = false;
            let replace = self.controller.replace_requested;
            self.controller.replace_requested = false;
            let id = self.controller.job_id;
            let decision = if replace {
                Decision::ReplaceVerified
            } else {
                Decision::Continue
            };
            let result = self
                .live_job
                .as_mut()
                .ok_or_else(|| "missing live match worker".to_owned())
                .and_then(|job| job.decide(decision));
            if let Err(error) = result {
                self.controller.accept_live_error(id, error);
                self.sync_jobs();
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
                Poll::Failed(id, error) => {
                    self.controller.accept_error(id, error);
                    self.job = None;
                }
            }
        }
        if let Some(job) = self.live_job.as_mut() {
            match job.poll() {
                LivePoll::Pending => (),
                LivePoll::Interval(id, prefix) => {
                    self.controller.accept_interval(id, prefix);
                    self.last_step = Instant::now();
                }
                LivePoll::Ready(id, completion) => {
                    self.controller.accept_continuation(id, completion);
                    self.live_job = None;
                    self.live_id = None;
                    self.last_step = Instant::now();
                }
                LivePoll::Failed(id, error) => {
                    self.controller.accept_live_error(id, error);
                    self.live_job = None;
                    self.live_id = None;
                }
            }
        }
        self.sync_jobs();
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
        let playback = c.match_view();
        serde_json::json!({"history_top":c.history_top,"visible_history":if c.screen == Screen::History {Some(c.visible_history())} else {None},"pitch_markers":playback.map(|p| p.view().players.iter().zip(p.markers()).map(|(player,m)|serde_json::json!({"scene":player.scene,"player":player.player,"selector":m[0],"attribute":m[1],"position":player.position,"label":crate::timeline::numbered_marker(m[0]).to_string()})).collect::<Vec<_>>()),"match_clubs":playback.map(|p|p.club_names),"recent_actions":playback.map(|p|p.events.iter().rev().take(8).rev().map(|e|serde_json::json!({"text":e.text,"native":e.native})).collect::<Vec<_>>()),"instruction":c.instruction,"option_available":(0..8).map(|i|c.option_available(i)).collect::<Vec<_>>(),"save_path":self.save_path,"game_name":c.game_name,"load_entries":c.load_entries,"load_index":c.load_index,"menus":[c.menu,c.preparation_menu,c.option],"speed":c.speed,"close_requested":c.close_requested,"screen":format!("{:?}",c.screen),"panel":format!("{:?}",c.editor.panel),"selected":format!("{:?}",c.editor.selected),"card":c.editor.card(),"carried":c.editor.carried_player(),"first":c.editor.campaign.club.first_team,"reserve":c.editor.campaign.club.reserves,"campaign":c.editor.campaign,"exhibition":c.exhibition,"job":c.job_id,"worker_pid":self.job.as_ref().map(|j|j.pid()).or_else(||self.live_job.as_ref().map(|j|j.pid())),"live_worker":self.live_job.is_some(),"match_stage":if c.interval_playback.is_some(){Some("incomplete")}else if c.playback.is_some(){Some("complete")}else{None},"resuming":c.is_resuming(),"loaded_frames":playback.map(|p|p.loaded_frames),"phase":playback.map(|p|format!("{:?}",p.phase)),"frames":playback.map(|p|p.cursor),"events":playback.map(|p|p.events.len()),"clock":playback.map(|p|p.view().clock),"score":playback.map(|p|p.view().score),"players":playback.map(|p|p.view().players.len()),"message":c.message})
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
