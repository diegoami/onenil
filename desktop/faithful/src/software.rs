//! [designed] Experimental native framebuffer adapter; no browser or network runtime.
use crate::{application::Application, controller::Key, render::Renderer};
use minifb::{InputCallback, Key as NativeKey, Window, WindowOptions};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
struct Input(Arc<Mutex<Vec<NativeKey>>>);
impl InputCallback for Input {
    fn add_char(&mut self, _: u32) {}
    fn set_key_state(&mut self, key: NativeKey, pressed: bool) {
        if pressed {
            self.0.lock().unwrap().push(key);
        }
    }
}
pub fn run(
    root: PathBuf,
    font: PathBuf,
    save: PathBuf,
    load: bool,
    observe: Option<PathBuf>,
) -> Result<(), String> {
    let mut app = Application::new(root, save, load)?;
    let mut renderer = Renderer::new(&font)?;
    let mut window = Window::new(
        "One-Nil | native desktop feasibility",
        1440,
        900,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .map_err(|e| e.to_string())?;
    window.set_target_fps(60);
    let input = Arc::new(Mutex::new(Vec::new()));
    window.set_input_callback(Box::new(Input(input.clone())));
    let mut last_observation = Vec::new();
    let mut last_visual = Vec::new();
    let mut scaled = Vec::new();
    let mut last_size = (0, 0);
    while window.is_open() && !app.controller.close_requested {
        for key in std::mem::take(&mut *input.lock().unwrap()) {
            let mapped = match key {
                NativeKey::Up => Some(Key::Up),
                NativeKey::Down => Some(Key::Down),
                NativeKey::Left => Some(Key::Left),
                NativeKey::Right => Some(Key::Right),
                NativeKey::Enter => Some(Key::Enter),
                NativeKey::Escape => Some(Key::Escape),
                NativeKey::Space => Some(Key::Space),
                NativeKey::Insert => Some(Key::Insert),
                NativeKey::F7 => Some(Key::F7),
                NativeKey::F9 => Some(Key::F9),
                NativeKey::F10 => Some(Key::F10),
                NativeKey::Y => Some(Key::Y),
                NativeKey::N => Some(Key::N),
                _ => None,
            };
            if let Some(key) = mapped {
                app.key(key);
            }
        }
        app.update();
        let visual = serde_json::to_vec(&app.snapshot()).map_err(|e| e.to_string())?;
        let changed = visual != last_visual;
        if changed {
            renderer.draw(&app.controller);
            last_visual = visual;
        }
        if let Some(path) = &observe {
            let bytes = serde_json::to_vec(&app.snapshot()).map_err(|e| e.to_string())?;
            if bytes != last_observation {
                let temporary = path.with_extension("tmp");
                std::fs::write(&temporary, &bytes).map_err(|e| e.to_string())?;
                std::fs::rename(temporary, path).map_err(|e| e.to_string())?;
                last_observation = bytes;
            }
        }
        let (w, h) = window.get_size();
        if w == 0 || h == 0 {
            window.update();
            continue;
        }
        if changed || last_size != (w, h) {
            scaled = renderer.scaled(w, h);
            last_size = (w, h);
        }
        window
            .update_with_buffer(&scaled, w, h)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
