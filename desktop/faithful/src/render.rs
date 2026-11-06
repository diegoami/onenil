//! [derived] T51 cell compositions, replacement font and independent vector primitives.
use crate::{
    controller::{Controller, Screen},
    editor::{Location, Panel, NAMES},
};
use fontdue::{Font, FontSettings};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, path::Path};
pub const WIDTH: usize = 720;
pub const HEIGHT: usize = 400;
const GROUND: u32 = 0x002800;
const TEXT: u32 = 0xaaaaaa;
const CYAN: u32 = 0x55ffff;
type Cell = (char, u32, u32);
#[derive(Deserialize)]
struct Assets {
    screens: HashMap<String, Vec<Cell>>,
}
pub struct Renderer {
    pub pixels: Vec<u32>,
    font: Font,
    glyphs: HashMap<char, (fontdue::Metrics, Vec<u8>)>,
    assets: Assets,
}
impl Renderer {
    pub fn new(font: &Path) -> Result<Self, String> {
        let bytes = std::fs::read(font).map_err(|e| e.to_string())?;
        if format!("{:x}", Sha256::digest(&bytes))
            != "c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f"
        {
            return Err("replacement font digest differs".into());
        }
        Ok(Self {
            pixels: vec![GROUND; WIDTH * HEIGHT],
            glyphs: HashMap::new(),
            font: Font::from_bytes(bytes, FontSettings::default()).map_err(|e| e.to_string())?,
            assets: serde_json::from_str(include_str!("../assets/cells.json"))
                .map_err(|e| e.to_string())?,
        })
    }
    fn pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && y >= 0 && x < WIDTH as i32 && y < HEIGHT as i32 {
            self.pixels[y as usize * WIDTH + x as usize] = color;
        }
    }
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        for py in y..y + h {
            for px in x..x + w {
                self.pixel(px, py, color);
            }
        }
    }
    fn line(&mut self, x: i32, y: i32, ex: i32, ey: i32, color: u32) {
        let steps = (ex - x).abs().max((ey - y).abs()).max(1);
        for i in 0..=steps {
            self.pixel(x + (ex - x) * i / steps, y + (ey - y) * i / steps, color);
        }
    }
    fn cell(&mut self, x: usize, y: usize, ch: char, fg: u32, bg: u32) {
        let (px, py) = (x as i32 * 9, y as i32 * 16);
        self.rect(px, py, 9, 16, bg);
        let all = "─│┌┐└┘├┤┬┴┼═║╔╗╚╝╠╣╦╩╬";
        if let Some(i) = all.chars().position(|c| c == ch) {
            let arms = [
                "LR", "UD", "RD", "LD", "RU", "LU", "URD", "ULD", "LRD", "LRU", "ULRD",
            ][i % 11];
            for arm in arms.chars() {
                for off in if i >= 11 { vec![-1, 1] } else { vec![0] } {
                    let (ex, ey) = match arm {
                        'L' => (0, 8),
                        'R' => (9, 8),
                        'U' => (4, 0),
                        _ => (4, 16),
                    };
                    let (dx, dy) = if arm == 'U' || arm == 'D' {
                        (off, 0)
                    } else {
                        (0, off)
                    };
                    self.line(px + 4 + dx, py + 8 + dy, px + ex + dx, py + ey + dy, fg);
                }
            }
            return;
        }
        match ch {
            '■' => {
                self.rect(px + 2, py + 4, 5, 7, fg);
                return;
            }
            '▀' => {
                self.rect(px, py, 9, 8, fg);
                return;
            }
            '▄' => {
                self.rect(px, py + 8, 9, 8, fg);
                return;
            }
            '█' => {
                self.rect(px, py, 9, 16, fg);
                return;
            }
            '▌' => {
                self.rect(px, py, 5, 16, fg);
                return;
            }
            '▐' => {
                self.rect(px + 4, py, 5, 16, fg);
                return;
            }
            ' ' => return,
            _ => (),
        }
        let (m, bitmap) = self
            .glyphs
            .entry(ch)
            .or_insert_with(|| self.font.rasterize(ch, 16.0))
            .clone();
        let ascent = self
            .font
            .horizontal_line_metrics(16.0)
            .map_or(12.0, |m| m.ascent)
            .round() as i32;
        for gy in 0..m.height {
            for gx in 0..m.width {
                let alpha = bitmap[gy * m.width + gx] as u32;
                let blend = |shift: u32| -> u32 {
                    let f = (fg >> shift) & 255;
                    let b = (bg >> shift) & 255;
                    (f * alpha + b * (255 - alpha)) / 255
                };
                let color = blend(16) << 16 | blend(8) << 8 | blend(0);
                let tx =
                    ((gx as i32 + m.xmin) as f32 * 9.0 / m.advance_width.max(1.0)).round() as i32;
                self.pixel(
                    px + tx,
                    py + ascent - m.height as i32 - m.ymin + gy as i32,
                    color,
                );
            }
        }
    }
    fn text(&mut self, x: usize, y: usize, text: &str, fg: u32, bg: u32, width: usize) {
        let chars: Vec<_> = text.chars().collect();
        for i in 0..width.min(80 - x) {
            self.cell(x + i, y, chars.get(i).copied().unwrap_or(' '), fg, bg);
        }
    }
    fn sheet(&mut self, name: &str) {
        let cells = self.assets.screens[name].clone();
        for (i, (ch, fg, bg)) in cells.into_iter().enumerate() {
            self.cell(i % 80, i / 80, ch, fg, bg);
        }
    }
    fn roster_row(i: usize) -> usize {
        i + 1 + usize::from(i >= 11) + usize::from(i >= 14)
    }
    fn roster(&mut self, c: &Controller, x: usize) {
        let reserve = c.editor.selected == Location::Reserve;
        self.text(
            x,
            0,
            if reserve { "RESERVES" } else { "FIRST TEAM" },
            CYAN,
            GROUND,
            13,
        );
        for i in 0..18 {
            let row = Self::roster_row(i);
            let player = if reserve {
                if i == 0 {
                    c.editor.reserve
                } else {
                    None
                }
            } else {
                c.editor.first[i]
            };
            let selected = if reserve {
                i == 0
            } else {
                c.editor.selected == Location::First(i)
            };
            let (fg, bg) = if selected {
                (GROUND, TEXT)
            } else {
                (TEXT, GROUND)
            };
            self.text(
                x,
                row,
                player.map(|p| NAMES[p as usize]).unwrap_or(""),
                fg,
                bg,
                13,
            );
            let role = player
                .map(|p| self.assets.screens["skills"][Self::roster_row(p as usize) * 80 + 72].0)
                .unwrap_or(' ');
            self.cell(x + 14, row, role, fg, bg);
        }
    }
    fn players(&mut self, c: &Controller) {
        let name = match (c.editor.card(), c.editor.panel) {
            (Some(0), Panel::Skills) => Some("skills"),
            (Some(0), Panel::Details) => Some("details"),
            (Some(1), Panel::Skills) => Some("pearce-skills"),
            (Some(1), Panel::Details) => Some("pearce-details"),
            (Some(17), Panel::Details) => Some("mimms-details"),
            _ => None,
        };
        self.sheet(name.unwrap_or("skills"));
        if name.is_none() {
            self.rect(0, 0, 480, 376, GROUND);
            self.text(
                1,
                0,
                if c.editor.panel == Panel::Skills {
                    "PLAYER SKILLS"
                } else {
                    "PLAYER DETAILS"
                },
                CYAN,
                GROUND,
                51,
            );
            if let Some(id) = c.editor.card() {
                self.text(1, 2, NAMES[id as usize], TEXT, GROUND, 51);
                self.text(
                    1,
                    4,
                    "Card not captured in this bounded study.",
                    TEXT,
                    GROUND,
                    51,
                );
            }
        }
        self.roster(c, 58);
        if c.editor.carried_player().is_some() {
            self.text(
                0,
                24,
                "PUT HERE                 Enter-place   Esc-cancel",
                CYAN,
                GROUND,
                80,
            );
        }
        if c.screen == Screen::Options {
            let options = [
                "Reposition",
                "Club position",
                "Sell",
                "Take off T.M",
                "Change tactics",
                "New contract",
                "Fire",
                "Continue",
            ];
            self.rect(180, 64, 270, 160, 0);
            for (i, s) in options.iter().enumerate() {
                self.text(
                    21,
                    5 + i,
                    s,
                    if i == c.option { GROUND } else { TEXT },
                    if i == c.option { TEXT } else { 0 },
                    26,
                );
            }
        }
    }
    fn match_header(&mut self, c: &Controller) {
        self.rect(0, 0, 720, 80, GROUND);
        if let Some(p) = &c.playback {
            self.text(1, 0, "BLACKBURN R", TEXT, GROUND, 16);
            self.text(1, 2, "SHEFFIELD WED", TEXT, GROUND, 16);
            self.text(17, 0, &p.view().score[0].to_string(), CYAN, GROUND, 2);
            self.text(17, 2, &p.view().score[1].to_string(), CYAN, GROUND, 2);
            self.text(
                23,
                1,
                &format!("{:02} min", p.view().clock),
                CYAN,
                GROUND,
                14,
            );
            let lines = wrapped(
                &p.events
                    .iter()
                    .rev()
                    .take(3)
                    .rev()
                    .map(|e| e.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
                37,
            );
            for (i, line) in lines
                .iter()
                .rev()
                .take(4)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .enumerate()
            {
                self.text(43, i, line, TEXT, GROUND, 37);
            }
        }
    }
    fn match_view(&mut self, c: &Controller) {
        self.pixels.fill(GROUND);
        self.match_header(c);
        if c.screen == Screen::Pitch {
            self.rect(18, 85, 684, 288, 0x004000);
            for (x, y, ex, ey) in [
                (18, 85, 702, 85),
                (18, 373, 702, 373),
                (18, 85, 18, 373),
                (702, 85, 702, 373),
                (360, 85, 360, 373),
                (18, 160, 126, 160),
                (126, 160, 126, 298),
                (126, 298, 18, 298),
                (702, 160, 594, 160),
                (594, 160, 594, 298),
                (594, 298, 702, 298),
            ] {
                self.line(x, y, ex, ey, TEXT);
            }
            if let Some(p) = &c.playback {
                // [designed] Proportional accepted adapter projection: x=500+guest.y*12, y=300+guest.x*29 on1000x600.
                let point = |x: i16, y: i16| {
                    (
                        (360 + (y as i32) * 684 * 12 / 1000).clamp(18, 702),
                        (229 + (x as i32) * 288 * 29 / 600).clamp(85, 373),
                    )
                };
                for player in &p.view().players {
                    let (x, y) = point(player.position.x, player.position.y);
                    let color = if player.club == 130 {
                        0x55ffff
                    } else {
                        0xffff55
                    };
                    self.rect(x - 2, y - 6, 4, 4, color);
                    self.line(x, y - 2, x, y + 3, color);
                    self.line(x - 3, y, x + 3, y, color);
                    self.line(x, y + 3, x - 3, y + 6, color);
                    self.line(x, y + 3, x + 3, y + 6, color);
                }
                let (x, y) = point(p.view().ball.x, p.view().ball.y);
                self.rect(x - 1, y - 1, 3, 3, 0xffffff);
            }
            self.text(
                1,
                24,
                &format!(
                    "Space-commentary  Esc,Enter-bench   Left/Right-speed {}",
                    c.speed
                ),
                TEXT,
                GROUND,
                78,
            );
        } else if c.screen == Screen::History {
            if let Some(p) = &c.playback {
                let lines = wrapped(
                    &p.events
                        .iter()
                        .map(|e| format!("{} {}", e.native["minute"], e.text))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    77,
                );
                for (i, s) in lines
                    .iter()
                    .rev()
                    .take(18)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .enumerate()
                {
                    self.text(1, 5 + i, s, TEXT, GROUND, 78);
                }
            }
            self.text(1, 24, "Space or Esc-return to pitch", TEXT, GROUND, 78);
        } else {
            self.text(
                25,
                7,
                if c.screen == Screen::HalfTime {
                    "HALF TIME"
                } else {
                    "FULL TIME"
                },
                CYAN,
                GROUND,
                30,
            );
            self.text(
                4,
                10,
                "Score and actions are computed by the native engine.",
                TEXT,
                GROUND,
                72,
            );
            self.text(
                4,
                12,
                "Unmapped statistics remain unavailable in this experiment.",
                TEXT,
                GROUND,
                72,
            );
            self.text(20, 24, "press any key", TEXT, GROUND, 40);
        }
    }
    pub fn draw(&mut self, c: &Controller) {
        match c.screen {
            Screen::Players | Screen::Options => self.players(c),
            Screen::Pitch | Screen::History | Screen::HalfTime | Screen::FullTime => {
                self.match_view(c)
            }
            Screen::Preparation | Screen::Preparing => {
                self.sheet("preparation");
                self.roster(c, 57);
                self.text(0, 0, "NATIVE EXHIBITION", CYAN, GROUND, 19);
                self.text(2, 4, "Blackburn R", TEXT, GROUND, 16);
                self.text(2, 6, "Sheffield Wed", TEXT, GROUND, 16);
                self.text(
                    0,
                    24,
                    "Reference64 exhibition; captured squad edits do not set its lineup.",
                    CYAN,
                    GROUND,
                    80,
                );
                if c.screen == Screen::Preparing {
                    self.rect(90, 112, 540, 96, 0);
                    self.text(12, 8, "Preparing actual Rust match...", CYAN, 0, 56);
                    self.text(12, 10, "Esc-cancel; window remains responsive", TEXT, 0, 56);
                }
            }
            Screen::Bench => {
                self.pixels.fill(GROUND);
                self.match_header(c);
                self.text(2, 7, "BENCH", CYAN, GROUND, 25);
                self.text(2, 10, "PLAY GAME", GROUND, TEXT, 20);
                self.text(
                    2,
                    13,
                    "Managed substitutions are not implemented in this experiment.",
                    TEXT,
                    GROUND,
                    76,
                );
                self.text(2, 24, "Enter-resume computed match", TEXT, GROUND, 76);
            }
            Screen::Main | Screen::Quit => {
                self.sheet("main");
                let items = [
                    "PLAY NEXT GAME",
                    "Players",
                    "Next opponents",
                    "Transfer market",
                    "Use scout",
                    "New staff",
                    "League tables",
                    "League results",
                    "Cup results",
                    "Weekly fixtures",
                    "Fixture list",
                    "Statistics",
                    "Player/team history",
                    "Seasonal history",
                    "Other teams",
                    "Game configuration",
                    "Cashflow",
                ];
                for (i, s) in items.iter().enumerate() {
                    let row = if i == 0 { 4 } else { i + 5 };
                    self.text(
                        58,
                        row,
                        s,
                        if i == c.menu { GROUND } else { TEXT },
                        if i == c.menu { TEXT } else { GROUND },
                        20,
                    );
                }
                if c.screen == Screen::Quit {
                    self.rect(180, 144, 360, 80, 0);
                    self.text(22, 10, "Quit? Y / N", CYAN, 0, 36);
                }
            }
        }
        if !c.message.is_empty() {
            self.text(0, 24, &c.message, 0xffff55, 0, 80);
        }
    }
    pub fn scaled(&self, width: usize, height: usize) -> Vec<u32> {
        let mut out = vec![0x111111; width * height];
        let scale = (width as f64 / WIDTH as f64).min(height as f64 / HEIGHT as f64);
        let w = (WIDTH as f64 * scale).floor() as usize;
        let h = (HEIGHT as f64 * scale).floor() as usize;
        let (ox, oy) = ((width - w) / 2, (height - h) / 2);
        for y in 0..h {
            for x in 0..w {
                out[(oy + y) * width + ox + x] =
                    self.pixels[(y * HEIGHT / h) * WIDTH + x * WIDTH / w];
            }
        }
        out
    }
}
fn wrapped(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            if !line.is_empty() && line.len() + 1 + word.len() > width {
                lines.push(line);
                line = String::new();
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        if !line.is_empty() {
            lines.push(line);
        }
    }
    lines
}
