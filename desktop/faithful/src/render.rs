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
                // [designed:T70] Keep the font's native columns. Stretching eight
                // source columns onto nine left gaps and split vertical strokes.
                let tx = gx as i32 + m.xmin;
                let ty = ascent - m.height as i32 - m.ymin + gy as i32;
                if (0..9).contains(&tx) && (0..16).contains(&ty) {
                    let color = if bitmap[gy * m.width + gx] >= 128 {
                        fg
                    } else {
                        bg
                    };
                    self.pixel(px + tx, py + ty, color);
                }
            }
        }
    }
    fn pitch_glyph(&mut self, px: i32, py: i32, ch: char, color: u32) {
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
                let x = gx as i32 + m.xmin;
                let y = ascent - m.height as i32 - m.ymin + gy as i32;
                if (0..9).contains(&x) && (0..16).contains(&y) && bitmap[gy * m.width + gx] >= 128 {
                    self.pixel(px + x, py + y, color);
                }
            }
        }
    }
    /// [designed] Independent standard text colours; no game palette asset.
    fn marker_color(attribute: u16) -> u32 {
        const COLORS: [u32; 16] = [
            0x000000, 0x0000aa, 0x00aa00, 0x00aaaa, 0xaa0000, 0xaa00aa, 0xaa5500, 0xaaaaaa,
            0x555555, 0x5555ff, 0x55ff55, 0x55ffff, 0xff5555, 0xff55ff, 0xffff55, 0xffffff,
        ];
        COLORS[usize::from(attribute & 15)]
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
                // [confirmed] S025 displays a fifth row; [designed] Mark editing is unavailable.
                self.text(2, 17, "Mark", TEXT, GROUND, 15);
                self.text(18, 17, "unavailable", TEXT, GROUND, 18);
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
        if let Some(p) = c.match_view() {
            self.text(1, 0, &p.club_names[0].to_uppercase(), TEXT, GROUND, 16);
            self.text(1, 2, &p.club_names[1].to_uppercase(), TEXT, GROUND, 16);
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
            if let Some(p) = c.match_view() {
                // [designed] Proportional accepted adapter projection: x=500+guest.y*12, y=300+guest.x*29 on1000x600.
                let point = |x: i16, y: i16| {
                    (
                        (360 + (y as i32) * 684 * 12 / 1000).clamp(18, 702),
                        (229 + (x as i32) * 288 * 29 / 600).clamp(85, 373),
                    )
                };
                for (player, marker) in p.view().players.iter().zip(p.markers()) {
                    let (x, y) = point(player.position.x, player.position.y);
                    self.pitch_glyph(
                        x - 4,
                        y - 8,
                        crate::timeline::numbered_marker(marker[0]),
                        Self::marker_color(marker[1]),
                    );
                }
                let (x, y) = point(p.view().ball.x, p.view().ball.y);
                self.rect(x - 1, y - 1, 3, 3, 0xffffff);
            }
            self.text(
                1,
                24,
                &format!(
                    "Players: numbers   Space: commentary   Esc/Enter: bench   Left/Right: speed {}",
                    c.speed
                ),
                TEXT,
                GROUND,
                78,
            );
        } else if c.screen == Screen::History {
            for (i, line) in c.visible_history().iter().enumerate() {
                self.text(1, 5 + i, line, TEXT, GROUND, 78);
            }
            self.text(
                1,
                23,
                "Up/Down: line   PgUp/PgDn: page   Home/End: first/latest",
                TEXT,
                GROUND,
                78,
            );
            self.text(1, 24, "Space or Esc-return to pitch", TEXT, GROUND, 78);
        } else {
            self.text(
                25,
                7,
                if c.screen == Screen::HalfTime || c.is_resuming() {
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
                if c.screen == Screen::HalfTime || c.is_resuming() {
                    if c.is_resuming() {
                        "Preparing second half..."
                    } else {
                        "Match paused. Press any key to review the bench."
                    }
                } else if c.exhibition {
                    "Exhibition complete."
                } else {
                    "Match complete. Press a key to record the result."
                },
                TEXT,
                GROUND,
                72,
            );
            self.text(
                4,
                12,
                if c.is_resuming() {
                    "Esc-cancel this match; campaign is unchanged."
                } else {
                    "Detailed match statistics are not available yet."
                },
                TEXT,
                GROUND,
                72,
            );
            self.text(
                20,
                24,
                if c.is_resuming() {
                    "Please wait"
                } else {
                    "press any key"
                },
                TEXT,
                GROUND,
                40,
            );
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
            Screen::Preparing if c.is_resuming() => self.match_view(c),
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
                    self.text(0, 0, "MATCH PREPARATION", CYAN, GROUND, 20);
                    self.roster(c, 58);
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
                    // [designed:T70] Replace the actual menu labels, not the
                    // substitution counter. Static mockup values never select actions.
                    for (i, label) in [
                        c.editor.campaign.next_fixture.opponent_name.as_str(),
                        "Match stats",
                        "PLAY GAME",
                        "Change tactics",
                    ]
                    .iter()
                    .enumerate()
                    {
                        let selected = i == c.preparation_menu;
                        self.text(
                            2,
                            12 + i * 2,
                            label,
                            if selected { GROUND } else { TEXT },
                            if selected { TEXT } else { GROUND },
                            16,
                        );
                    }
                    self.text(
                        0,
                        24,
                        if c.preparation_menu == 2 {
                            "Enter: play match   Up/Down: select   Space: squad   Esc: back   E: exhibition"
                        } else {
                            "Up/Down: select PLAY GAME   Enter: choose   Space: squad   Esc: back"
                        },
                        TEXT,
                        GROUND,
                        80,
                    );
                }
                if c.screen == Screen::Preparing {
                    self.rect(90, 112, 540, 96, 0);
                    self.text(
                        12,
                        8,
                        if c.exhibition {
                            "Preparing exhibition..."
                        } else {
                            "Preparing campaign match..."
                        },
                        CYAN,
                        0,
                        56,
                    );
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
                    "Player changes are not available in this build yet.",
                    TEXT,
                    GROUND,
                    76,
                );
                self.text(2, 24, "Enter: PLAY GAME / continue match", TEXT, GROUND, 76);
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
                    &c.recorded_result_text().unwrap_or_else(|| {
                        if c.menu == 0 {
                            "Enter: start match   Up/Down: select   Space: squad   F7: save"
                        } else {
                            "Enter: open selected item   Up/Down: select   Space: squad   F7: save"
                        }
                        .into()
                    }),
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

#[cfg(test)]
mod refusal_tests {
    use super::*;
    use crate::controller::Key;

    #[test]
    fn refused_campaign_renders_complete_recovery_instruction() {
        let font = std::env::var_os("ONENIL_DESKTOP_FONT").expect("set verified replacement font");
        let mut controller = Controller::default();
        controller
            .editor
            .campaign
            .exchange(
                onenil_campaign::Location::First(0),
                onenil_campaign::Location::First(1),
            )
            .unwrap();
        controller.key(Key::Enter);
        controller.key(Key::Enter);
        assert_eq!(controller.screen, Screen::Preparation);
        assert!(!controller.prepare_requested);
        let expected = crate::campaign_match::REFUSAL;
        assert!(expected.len() <= 80);
        let mut actual = Renderer::new(Path::new(&font)).unwrap();
        actual.draw(&controller);
        let mut reference = Renderer::new(Path::new(&font)).unwrap();
        reference.text(0, 24, expected, 0xffff55, 0, 80);
        assert_eq!(
            &actual.pixels[384 * WIDTH..],
            &reference.pixels[384 * WIDTH..]
        );
        // Recovery words have visible ink, including the final period.
        for cell in 45..expected.len() {
            if expected.as_bytes()[cell] != b' ' {
                assert!((384..400)
                    .any(|y| (cell * 9..cell * 9 + 9)
                        .any(|x| actual.pixels[y * WIDTH + x] == 0xffff55)));
            }
        }
    }
}

#[cfg(test)]
mod readability_tests {
    use super::*;
    use crate::controller::Key;
    fn renderer() -> Renderer {
        Renderer::new(Path::new(
            &std::env::var_os("ONENIL_DESKTOP_FONT").expect("verified font"),
        ))
        .unwrap()
    }
    fn region(r: &Renderer, x: usize, y: usize, w: usize, h: usize) -> Vec<u32> {
        (y..y + h)
            .flat_map(|yy| r.pixels[yy * WIDTH + x..yy * WIDTH + x + w].iter().copied())
            .collect()
    }
    #[test]
    fn readability_glyphs_keep_native_columns_and_stay_in_their_cell() {
        let mut r = renderer();
        for ch in ['M', 'W', 'm', '0', '1', 'g', 'Á'] {
            r.pixels.fill(0x123456);
            r.cell(4, 3, ch, 0xffffff, 0);
            let (m, bitmap) = r.font.rasterize(ch, 16.0);
            let ascent = r.font.horizontal_line_metrics(16.0).unwrap().ascent.round() as i32;
            let mut expected = vec![0; 9 * 16];
            for gy in 0..m.height {
                for gx in 0..m.width {
                    let x = gx as i32 + m.xmin;
                    let y = ascent - m.height as i32 - m.ymin + gy as i32;
                    if (0..9).contains(&x)
                        && (0..16).contains(&y)
                        && bitmap[gy * m.width + gx] >= 128
                    {
                        expected[y as usize * 9 + x as usize] = 0xffffff;
                    }
                }
            }
            assert_eq!(region(&r, 36, 48, 9, 16), expected, "native glyph {ch}");
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if !(36..45).contains(&x) || !(48..64).contains(&y) {
                        assert_eq!(r.pixels[y * WIDTH + x], 0x123456, "glyph escapes {ch}");
                    }
                }
            }
            for (w, h) in [(1440, 900), (1280, 800)] {
                assert!(r
                    .scaled(w, h)
                    .iter()
                    .all(|p| [0, 0xffffff, 0x123456, 0x111111].contains(p)));
            }
        }
    }
    #[test]
    fn readability_preparation_has_one_play_action_and_aligned_roster() {
        let mut c = Controller::default();
        c.key(Key::Enter);
        let mut actual = renderer();
        actual.draw(&c);
        let mut template = renderer();
        template.sheet("preparation");
        assert_eq!(
            region(&actual, 9, 9 * 16, 17 * 9, 16),
            region(&template, 9, 9 * 16, 17 * 9, 16),
            "substitution count must not become a duplicate PLAY GAME"
        );
        template.text(2, 16, "PLAY GAME", GROUND, TEXT, 16);
        assert_eq!(
            region(&actual, 18, 16 * 16, 16 * 9, 16),
            region(&template, 18, 16 * 16, 16 * 9, 16),
            "highlight the real selected action"
        );
        let mut players = Controller::default();
        players.key(Key::Space);
        let mut player_raster = renderer();
        player_raster.draw(&players);
        assert_eq!(
            region(&actual, 57 * 9, 0, 18 * 9, 23 * 16),
            region(&player_raster, 57 * 9, 0, 18 * 9, 23 * 16),
            "one roster column layout on both screens"
        );
    }
    #[test]
    fn readability_start_hints_match_actual_enter_routes() {
        let mut c = Controller::default();
        let mut actual = renderer();
        let mut expected = renderer();
        for (screen, hint) in [
            (
                Screen::Main,
                "Enter: start match   Up/Down: select   Space: squad   F7: save",
            ),
            (
                Screen::Preparation,
                "Enter: play match   Up/Down: select   Space: squad   Esc: back   E: exhibition",
            ),
        ] {
            assert_eq!(c.screen, screen);
            actual.draw(&c);
            expected.text(0, 24, hint, TEXT, GROUND, 80);
            assert_eq!(
                region(&actual, 0, 384, WIDTH, 16),
                region(&expected, 0, 384, WIDTH, 16)
            );
            c.key(Key::Enter);
        }
        assert_eq!(c.screen, Screen::Preparing);
        assert!(c.prepare_requested);
    }
}

#[cfg(test)]
mod numbered_pitch_tests {
    use super::*;
    #[test]
    fn numbered_pitch_uses_original_digit_not_an_invented_figure() {
        // [designed] Synthetic raster probe, not a simulated match. Physical
        // scene1 deliberately carries selector2 to protect their distinction.
        let players: Vec<_>=(0..22).map(|i| serde_json::json!({
            "scene":i+1,"player":2581+i,"club":if i<11 {130} else {17},
            "focus":false,"position":if i==0 {serde_json::json!({"x":-4,"y":-15})} else {serde_json::json!({"x":7,"y":36})}
        })).collect();
        let view =
            serde_json::json!({"clock":0,"score":[0,0],"ball":{"x":7,"y":-36},"players":players});
        let row: Vec<_> = (0..22)
            .map(|i| [if i == 0 { 2 } else { i + 1 }, if i < 11 { 11 } else { 9 }])
            .collect();
        let timeline = serde_json::from_value(serde_json::json!({
            "club_names":["Blackburn R","P St Germain"],"initial":view,
            "frames":[{"view":view,"events":[]}],"setup_events":[],"native_return":"0AB9:EBE2",
            "draws":0,"clock_rows":0,"seed":0,"clubs":[130,17],"marker_rows":[row,row]
        }))
        .unwrap();
        let mut c = Controller::default();
        c.screen = Screen::Pitch;
        c.playback = Some(crate::timeline::Playback::new(timeline));
        let font = std::env::var_os("ONENIL_DESKTOP_FONT").expect("verified font");
        let mut actual = Renderer::new(Path::new(&font)).unwrap();
        actual.draw(&c);
        let mut expected = Renderer::new(Path::new(&font)).unwrap();
        expected.cell(0, 0, '0', 0x55ffff, 0x004000);
        let region: Vec<_> = (166..182)
            .flat_map(|y| {
                actual.pixels[y * WIDTH + 233..y * WIDTH + 242]
                    .iter()
                    .copied()
            })
            .collect();
        let mask: Vec<_> = (0..16)
            .flat_map(|y| expected.pixels[y * WIDTH..y * WIDTH + 9].iter().copied())
            .collect();
        assert_eq!(
            region, mask,
            "source selector2 must draw the original numbered digit0"
        );
    }
}
