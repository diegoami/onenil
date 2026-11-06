//! [derived] T51 cell compositions, replacement font and independent vector primitives.
use crate::{
    controller::{Controller, Screen},
    editor::{Location, Panel},
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
struct Labels {
    positions: Vec<String>,
    role_glyphs: Vec<String>,
    instructions: Vec<Vec<String>>,
    goalkeeper_position: Vec<String>,
    goalkeeper_passing: Vec<String>,
}
#[derive(Deserialize)]
struct Assets {
    screens: HashMap<String, Vec<Cell>>,
}
pub struct Renderer {
    pub pixels: Vec<u32>,
    font: Font,
    glyphs: HashMap<char, (fontdue::Metrics, Vec<u8>)>,
    assets: Assets,
    labels: Labels,
    initial: onenil_campaign::Campaign,
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
            labels: serde_json::from_str(include_str!("../assets/campaign-labels.json"))
                .map_err(|e| e.to_string())?,
            initial: onenil_campaign::Campaign::initial()?,
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
        let reserve = matches!(c.editor.selected, Location::Reserve(_));
        self.text(
            x,
            0,
            if reserve { " RESERVES" } else { " FIRST TEAM" },
            CYAN,
            GROUND,
            14,
        );
        for i in 0..20 {
            let row = if reserve { i + 1 } else { Self::roster_row(i) };
            let id = if reserve {
                c.editor.campaign.club.reserves[i]
            } else {
                c.editor.campaign.club.first_team[i]
            };
            let selected = c.editor.selected
                == if reserve {
                    Location::Reserve(i)
                } else {
                    Location::First(i)
                };
            let (fg, bg) = if selected {
                (GROUND, TEXT)
            } else {
                (TEXT, GROUND)
            };
            let name = id
                .and_then(|id| c.editor.player(id))
                .map(|p| p.name.as_str())
                .unwrap_or("");
            self.text(x, row, &format!(" {name}"), fg, bg, 14);
            let role = id
                .and_then(|id| c.editor.player(id))
                .map(|p| {
                    self.labels.role_glyphs[p.current_position as usize]
                        .chars()
                        .next()
                        .unwrap_or('?')
                })
                .unwrap_or(' ');
            self.cell(x + 15, row, role, TEXT, GROUND);
        }
    }
    pub fn instruction_text(
        &self,
        c: &Controller,
        player: &onenil_campaign::Player,
        row: usize,
    ) -> String {
        let explicit = player.individual_instructions & (1 << (row + 4)) != 0;
        let bits = if explicit {
            player.individual_instructions
        } else {
            c.editor.campaign.club.team_instructions
        };
        let choice = usize::from(bits & (1 << row) != 0);
        let table = if player.current_position == 0 && row == 0 {
            &self.labels.goalkeeper_position
        } else if player.current_position == 0 && row == 2 {
            &self.labels.goalkeeper_passing
        } else {
            &self.labels.instructions[row]
        };
        format!("{}{}", table[choice], if explicit { " P" } else { "" })
    }
    fn players(&mut self, c: &Controller) {
        let player = c.editor.card().and_then(|id| c.editor.player(id));
        let unchanged = player
            .is_some_and(|p| self.initial.players.iter().any(|original| p == original))
            && c.editor.campaign.club.team_instructions == self.initial.club.team_instructions;
        let name = if unchanged {
            match (c.editor.card(), c.editor.panel) {
                (Some(2581), Panel::Skills) => Some("skills"),
                (Some(2581), Panel::Details) => Some("details"),
                (Some(2582), Panel::Skills) => Some("pearce-skills"),
                (Some(2582), Panel::Details) => Some("pearce-details"),
                (Some(2598), Panel::Details) => Some("mimms-details"),
                _ => None,
            }
        } else {
            None
        };
        self.sheet(name.unwrap_or(if c.editor.panel == Panel::Details {
            "details"
        } else {
            "skills"
        }));
        if name.is_none() {
            // [derived] Keep original card borders; remove uncaptured values before drawing
            // the owned identity/position/instruction fields in their original grouping.
            let sheet = if c.editor.panel == Panel::Details {
                "details"
            } else {
                "skills"
            };
            let cells = self.assets.screens[sheet].clone();
            for (i, (ch, fg, _)) in cells.into_iter().enumerate() {
                if i % 80 < 54 && i / 80 < 24 {
                    let border = "─│┌┐└┘├┤┬┴┼═║╔╗╚╝╠╣╦╩╬".contains(ch);
                    self.cell(i % 80, i / 80, if border { ch } else { ' ' }, fg, GROUND);
                }
            }
            self.text(
                2,
                0,
                if c.editor.panel == Panel::Details {
                    "PLAYER DETAILS"
                } else {
                    "PLAYER SKILLS"
                },
                CYAN,
                GROUND,
                18,
            );
            if let Some(player) = player {
                self.text(2, 1, &player.name, TEXT, GROUND, 16);
                self.text(18, 1, &format!("Age {}", player.age), TEXT, GROUND, 8);
                self.text(
                    26,
                    1,
                    &format!("Team {}", c.editor.campaign.club.name),
                    TEXT,
                    GROUND,
                    27,
                );
                self.text(
                    2,
                    2,
                    &format!(
                        "Pos. {}",
                        self.labels.positions[player.current_position as usize]
                    ),
                    TEXT,
                    GROUND,
                    24,
                );
                self.text(
                    26,
                    2,
                    &format!(
                        "1st pos {}",
                        self.labels.positions[player.club_position as usize]
                    ),
                    TEXT,
                    GROUND,
                    27,
                );
                self.text(
                    2,
                    5,
                    "Remaining player data not yet decoded",
                    TEXT,
                    GROUND,
                    49,
                );
                if c.editor.panel == Panel::Details {
                    for (i, label) in ["Positioning", "Possession", "Passing", "Tackling"]
                        .iter()
                        .enumerate()
                    {
                        self.text(2, 13 + i, label, TEXT, GROUND, 15);
                        self.text(
                            18,
                            13 + i,
                            &self.instruction_text(c, player, i),
                            TEXT,
                            GROUND,
                            18,
                        );
                    }
                    self.text(2, 17, "Mark", TEXT, GROUND, 15);
                    self.text(18, 17, "unavailable", TEXT, GROUND, 18);
                }
            }
            self.text(
                2,
                23,
                if c.editor.panel == Panel::Skills {
                    "INS-Details"
                } else {
                    "INS-Player skills"
                },
                0xffff55,
                GROUND,
                25,
            );
        }
        if matches!(c.editor.selected, Location::Reserve(_)) {
            // Reserve rows have no first-XI/substitute dividers. New vector rules preserve the
            // original list grouping without carrying stale FIRST TEAM section labels.
            self.rect(513, 0, 162, 376, GROUND);
            for (x, y, ex, ey) in [
                (513, 8, 666, 8),
                (513, 375, 666, 375),
                (513, 8, 513, 375),
                (666, 8, 666, 375),
                (648, 8, 648, 375),
            ] {
                self.line(x, y, ex, ey, TEXT);
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
            self.control_region("player-options");
            if let Some(player) = player {
                self.text(59, 2, &player.name, TEXT, GROUND, 18);
            }
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
            for (i, label) in options.iter().enumerate() {
                self.text(
                    58,
                    5 + i * 2,
                    &format!(" {label}"),
                    if i == c.option { GROUND } else { TEXT },
                    if i == c.option { TEXT } else { GROUND },
                    17,
                );
                self.cell(
                    76,
                    5 + i * 2,
                    if c.option_available(i) { '√' } else { 'x' },
                    0xffff55,
                    GROUND,
                );
            }
        } else if c.screen == Screen::Instructions {
            self.control_region("player-instructions");
            if let Some(player) = player {
                for (i, label) in ["Positioning", "Possession", "Passing", "Tackling"]
                    .iter()
                    .enumerate()
                {
                    self.text(
                        2,
                        13 + i,
                        &format!("{label:15} {}", self.instruction_text(c, player, i)),
                        if c.instruction == i { GROUND } else { TEXT },
                        if c.instruction == i { TEXT } else { GROUND },
                        34,
                    );
                }
            }
        }
    }
    /// [derived] Only the original right control grouping; the card is owned live state.
    fn control_region(&mut self, name: &str) {
        let cells = self.assets.screens[name].clone();
        for (i, (ch, fg, bg)) in cells.into_iter().enumerate() {
            if i % 80 >= 56 {
                self.cell(i % 80, i / 80, ch, fg, bg);
            }
        }
    }

    fn team_panel(&mut self, c: &Controller) {
        self.text(22, 0, &c.editor.campaign.club.name, TEXT, GROUND, 15);
        if c.editor.campaign.club.first_team != self.initial.club.first_team
            || c.editor.campaign.players != self.initial.players
            || c.editor.campaign.club.team_instructions != self.initial.club.team_instructions
        {
            self.rect(21 * 9, 16, 33 * 9, 14 * 16, GROUND);
            self.text(22, 3, "Team averages pending", TEXT, GROUND, 30);
            self.text(22, 5, "for this selection", TEXT, GROUND, 30);
        }
        for i in 0..8 {
            let choice = usize::from(c.editor.campaign.club.team_instructions & (1 << i) != 0);
            let label = self.labels.instructions[i][choice].clone();
            let start = 53 - label.len();
            self.text(start, 15 + i, &label, TEXT, GROUND, label.len());
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
            Screen::SavedGames => {
                self.sheet("saved-games");
                self.rect(26 * 9, 14 * 16, 18 * 9, 9 * 16, 0x280082);
                let start = c.load_index.saturating_sub(8);
                for (row, name) in c.load_entries.iter().enumerate().skip(start).take(9) {
                    let selected = row == c.load_index;
                    self.text(
                        26,
                        14 + row - start,
                        name,
                        if selected { 0 } else { TEXT },
                        if selected { TEXT } else { 0x280082 },
                        18,
                    );
                }
                self.text(
                    0,
                    24,
                    "Native saves: Enter-load  F9-return  Esc-keeps list",
                    CYAN,
                    0,
                    80,
                );
            }
            Screen::Players | Screen::Options | Screen::Instructions => self.players(c),
            Screen::Pitch | Screen::History | Screen::HalfTime | Screen::FullTime => {
                self.match_view(c)
            }
            Screen::Preparation | Screen::Preparing => {
                if c.exhibition {
                    self.pixels.fill(GROUND);
                    self.text(2, 1, "SEPARATE EXHIBITION", CYAN, GROUND, 60);
                    self.text(2, 3, "Blackburn R v Sheffield Wed", TEXT, GROUND, 60);
                    self.text(2, 5, "Fixed reference64 lineup", TEXT, GROUND, 60);
                    self.text(2, 9, "PLAY GAME", GROUND, TEXT, 20);
                    self.text(
                        1,
                        24,
                        "Enter-prepare  Esc-return  Campaign squad is separate",
                        TEXT,
                        GROUND,
                        78,
                    );
                } else {
                    self.sheet("preparation");
                    self.roster(c, 57);
                    self.team_panel(c);
                    self.text(2, 4, &c.editor.campaign.club.name, TEXT, GROUND, 16);
                    self.text(
                        2,
                        6,
                        &c.editor.campaign.next_fixture.opponent_name,
                        TEXT,
                        GROUND,
                        16,
                    );
                    self.text(1, 9, "Admission pending", TEXT, GROUND, 17);
                    self.text(
                        0,
                        24,
                        "SPACEBAR-Squad  E-separate exhibition  Esc-return",
                        CYAN,
                        GROUND,
                        80,
                    );
                }
                if c.screen == Screen::Preparing {
                    self.rect(90, 112, 540, 96, 0);
                    self.text(12, 8, "Preparing actual Rust exhibition...", CYAN, 0, 56);
                    self.text(
                        12,
                        10,
                        "Esc-cancel; saved campaign is unchanged",
                        TEXT,
                        0,
                        56,
                    );
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
                self.team_panel(c);
                self.text(70, 1, &c.game_name, TEXT, GROUND, 8);
                self.text(2, 2, &c.editor.campaign.club.name, TEXT, GROUND, 16);
                self.text(2, 4, &c.editor.campaign.manager.name, TEXT, GROUND, 16);
                let digits = c.editor.campaign.cash.to_string();
                let grouped = digits
                    .chars()
                    .rev()
                    .enumerate()
                    .flat_map(|(i, ch)| {
                        if i > 0 && i % 3 == 0 && ch != '-' {
                            vec![',', ch]
                        } else {
                            vec![ch]
                        }
                    })
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>();
                self.text(2, 6, &grouped, TEXT, GROUND, 16);
                self.text(
                    9,
                    8,
                    &format!(
                        "{}/{:02}",
                        c.editor.campaign.season.start_year,
                        c.editor.campaign.season.end_year % 100
                    ),
                    TEXT,
                    GROUND,
                    9,
                );
                self.text(
                    2,
                    12,
                    &c.editor.campaign.next_fixture.opponent_name,
                    TEXT,
                    GROUND,
                    16,
                );
                self.text(
                    0,
                    24,
                    "INS-Information  SPACEBAR-Squad  E-separate exhibition",
                    TEXT,
                    GROUND,
                    80,
                );
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
    /// [designed] Read-only diagnostic binds a state observation to its rendered RGB pixels.
    pub fn observed(&self, mut state: serde_json::Value) -> serde_json::Value {
        let rgb: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8])
            .collect();
        state["raster_sha256"] = format!("{:x}", Sha256::digest(rgb)).into();
        state
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

#[cfg(test)]
mod campaign_labels_tests {
    use super::*;
    #[test]
    fn saved_instruction_labels_follow_original_bits() {
        let font = std::env::var_os("ONENIL_DESKTOP_FONT").expect("set verified replacement font");
        let renderer = Renderer::new(Path::new(&font)).unwrap();
        let mut c = Controller::default();
        let keeper = c.editor.player(2598).unwrap();
        assert_eq!(
            renderer.instruction_text(&c, keeper, 0),
            "come out of box P"
        );
        c.editor.campaign.set_instructions(2598, 16).unwrap();
        let keeper = c.editor.player(2598).unwrap();
        assert_eq!(renderer.instruction_text(&c, keeper, 0), "stay in box P");
        assert_eq!(renderer.labels.positions[1], "sweeper");
        let pearce = c.editor.player(2582).unwrap();
        assert_eq!(renderer.instruction_text(&c, pearce, 0), "stay back P");
        assert_eq!(renderer.instruction_text(&c, pearce, 2), "short passes");
    }
}
