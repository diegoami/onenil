//! [designed] Second native candidate: identical controller and raster, eframe/glow transport.
use crate::{
    application::Application,
    controller::Key,
    render::{Renderer, HEIGHT, WIDTH},
};
use eframe::egui;
use std::{path::PathBuf, time::Duration};
struct Candidate {
    app: Application,
    renderer: Renderer,
    texture: Option<egui::TextureHandle>,
    observe: Option<PathBuf>,
    last_observation: Vec<u8>,
    last_visual: Vec<u8>,
}
impl eframe::App for Candidate {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let events = ui.input(|i| i.events.clone());
        for event in events {
            if let egui::Event::Key {
                key, pressed: true, ..
            } = event
            {
                let key = match key {
                    egui::Key::ArrowUp => Some(Key::Up),
                    egui::Key::ArrowDown => Some(Key::Down),
                    egui::Key::ArrowLeft => Some(Key::Left),
                    egui::Key::ArrowRight => Some(Key::Right),
                    egui::Key::Enter => Some(Key::Enter),
                    egui::Key::Escape => Some(Key::Escape),
                    egui::Key::Space => Some(Key::Space),
                    egui::Key::Insert => Some(Key::Insert),
                    egui::Key::F7 => Some(Key::F7),
                    egui::Key::F9 => Some(Key::F9),
                    egui::Key::F10 => Some(Key::F10),
                    egui::Key::Y => Some(Key::Y),
                    egui::Key::N => Some(Key::N),
                    _ => None,
                };
                if let Some(key) = key {
                    self.app.key(key);
                }
            }
        }
        self.app.update();
        let visual = serde_json::to_vec(&self.app.snapshot()).unwrap();
        let changed = visual != self.last_visual;
        if changed {
            self.renderer.draw(&self.app.controller);
            self.last_visual = visual;
        }
        if let Some(path) = &self.observe {
            if let Ok(bytes) = serde_json::to_vec(&self.app.snapshot()) {
                if bytes != self.last_observation {
                    let temporary = path.with_extension("tmp");
                    if let Err(e) = std::fs::write(&temporary, &bytes)
                        .and_then(|_| std::fs::rename(&temporary, path))
                    {
                        self.app.controller.message = format!("Observation failed: {e}");
                    }
                    self.last_observation = bytes;
                }
            }
        }
        if changed {
            let rgba: Vec<_> = self
                .renderer
                .pixels
                .iter()
                .flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, 255])
                .collect();
            let image = egui::ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &rgba);
            match self.texture.as_mut() {
                Some(texture) => texture.set(image, egui::TextureOptions::NEAREST),
                None => {
                    self.texture = Some(ui.ctx().load_texture(
                        "faithful-game-surface",
                        image,
                        egui::TextureOptions::NEAREST,
                    ))
                }
            }
        }
        let available = ui.max_rect();
        let scale = (available.width() / WIDTH as f32).min(available.height() / HEIGHT as f32);
        let rect = egui::Rect::from_center_size(
            available.center(),
            egui::vec2(WIDTH as f32 * scale, HEIGHT as f32 * scale),
        );
        ui.painter()
            .rect_filled(available, 0.0, egui::Color32::from_rgb(17, 17, 17));
        ui.painter().image(
            self.texture.as_ref().unwrap().id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        if self.app.controller.close_requested {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if matches!(
            self.app.controller.screen,
            crate::controller::Screen::Pitch | crate::controller::Screen::Preparing
        ) {
            ui.ctx().request_repaint_after(Duration::from_millis(16));
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
    let candidate = Candidate {
        app: Application::new(root, save, load)?,
        renderer: Renderer::new(&font)?,
        texture: None,
        observe,
        last_observation: Vec::new(),
        last_visual: Vec::new(),
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "One-Nil | native desktop feasibility",
        options,
        Box::new(move |_| Ok(Box::new(candidate))),
    )
    .map_err(|e| e.to_string())
}
