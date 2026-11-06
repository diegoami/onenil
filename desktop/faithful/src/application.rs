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
}
impl Application {
    pub fn new(root: PathBuf, save_path: PathBuf, load: bool) -> Result<Self, String> {
        let mut controller = Controller::default();
        if load {
            controller.editor.load(&save_path)?;
        }
        Ok(Self {
            controller,
            root,
            save_path,
            job: None,
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
                Ok(()) => "Experiment squad saved (F9 reloads).".into(),
                Err(e) => format!("Save refused: {e}"),
            };
        }
        if self.controller.load_requested {
            self.controller.load_requested = false;
            self.controller.message = match self.controller.editor.load(&self.save_path) {
                Ok(()) => "Experiment squad restored.".into(),
                Err(e) => format!("Load refused: {e}"),
            };
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
        serde_json::json!({"menus":[c.menu,c.preparation_menu,c.option],"speed":c.speed,"close_requested":c.close_requested,"screen":format!("{:?}",c.screen),"panel":format!("{:?}",c.editor.panel),"selected":format!("{:?}",c.editor.selected),"card":c.editor.card(),"carried":c.editor.carried_player(),"first":c.editor.first,"reserve":c.editor.reserve,"job":c.job_id,"worker_pid":self.job.as_ref().map(|j|j.pid()),"phase":c.playback.as_ref().map(|p|format!("{:?}",p.phase)),"frames":c.playback.as_ref().map(|p|p.cursor),"events":c.playback.as_ref().map(|p|p.events.len()),"score":c.playback.as_ref().map(|p|p.view().score),"players":c.playback.as_ref().map(|p|p.view().players.len()),"message":c.message})
    }
}
