//! [confirmed] Navigation: research manual W007/W013–W017/W031/W034–W040.
//! [designed] Unsupported actions are diagnosed; they do not invent campaign behavior.
use crate::{
    editor::{Editor, Location, Panel},
    timeline::{
        Completion, IntervalPlayback, IntervalTimeline, Phase, Playback, PlaybackView, Timeline,
    },
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Main,
    SavedGames,
    Players,
    Options,
    Instructions,
    Preparation,
    Preparing,
    Pitch,
    History,
    Bench,
    MatchSquad,
    HalfTime,
    FullTime,
    Quit,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    Enter,
    Escape,
    Space,
    Insert,
    F7,
    F9,
    F10,
    Y,
    N,
    Exhibition,
}
/// [designed:T78] Read-only interval bench selection state. The highlight and
/// pending pick live here; the roster itself stays in the engine-owned snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SquadSelection {
    pub highlight: usize,
    pub picked: Option<usize>,
}
/// The witnessed fresh exchange (research T74/T75; archaeology T80 policy).
const WITNESSED: (usize, u16, usize, u16) = (1, 2582, 11, 2592);
pub struct Controller {
    pub editor: Editor,
    pub screen: Screen,
    pub caller: Screen,
    pub menu: usize,
    pub preparation_menu: usize,
    pub option: usize,
    pub instruction: usize,
    pub message: String,
    pub prepare_requested: bool,
    pub job_id: u64,
    pub playback: Option<Playback>,
    pub interval_playback: Option<IntervalPlayback>,
    pub resume_requested: bool,
    pub replace_requested: bool,
    pub squad: Option<SquadSelection>,
    pending_replacement: Option<crate::timeline::ExecutedReplacement>,
    awaiting_continuation: bool,
    request_campaign: Option<onenil_campaign::Campaign>,
    pub save_requested: bool,
    pub load_requested: bool,
    pub load_entries: Vec<String>,
    pub load_index: usize,
    pub game_name: String,
    pub close_requested: bool,
    pub speed: u8,
    pub history_top: usize,
    pub exhibition: bool,
    match_ticket: Option<onenil_campaign::MatchTicket>,
    remembered_first: usize,
}
impl Default for Controller {
    fn default() -> Self {
        Self {
            editor: Editor::default(),
            screen: Screen::Main,
            caller: Screen::Main,
            menu: 0,
            preparation_menu: 2,
            option: 0,
            instruction: 0,
            message: String::new(),
            prepare_requested: false,
            job_id: 0,
            playback: None,
            interval_playback: None,
            resume_requested: false,
            replace_requested: false,
            squad: None,
            pending_replacement: None,
            awaiting_continuation: false,
            request_campaign: None,
            save_requested: false,
            load_requested: false,
            load_entries: Vec::new(),
            load_index: 0,
            game_name: "SAVE1".into(),
            close_requested: false,
            speed: 23,
            history_top: 0,
            exhibition: false,
            match_ticket: None,
            remembered_first: 0,
        }
    }
}
impl Controller {
    pub fn match_view(&self) -> Option<PlaybackView<'_>> {
        self.interval_playback
            .as_ref()
            .map(IntervalPlayback::view_ref)
            .or_else(|| self.playback.as_ref().map(Playback::view_ref))
    }
    pub fn is_resuming(&self) -> bool {
        self.awaiting_continuation
    }
    pub fn interval_halftime(&self) -> bool {
        self.interval_playback
            .as_ref()
            .is_some_and(|p| p.phase == Phase::HalfTime)
    }
    pub fn owns_live_request(&self, id: u64) -> bool {
        id == self.job_id
            && !self.exhibition
            && self.match_ticket.is_some()
            && self.request_campaign.is_some()
            && matches!(
                self.screen,
                Screen::Preparing
                    | Screen::Pitch
                    | Screen::History
                    | Screen::Bench
                    | Screen::HalfTime
            )
    }
    fn request_unchanged(&self) -> bool {
        self.request_campaign.as_ref() == Some(&self.editor.campaign)
    }
    fn fixture_matches(&self, clubs: &[u64], names: &[String; 2]) -> bool {
        clubs.len() == 2
            && clubs.contains(&u64::from(self.editor.campaign.club.id))
            && clubs.contains(&u64::from(self.editor.campaign.next_fixture.opponent))
            && *names
                == [
                    self.editor.campaign.club.name.clone(),
                    self.editor.campaign.next_fixture.opponent_name.clone(),
                ]
    }
    pub fn accept_interval(&mut self, id: u64, prefix: IntervalTimeline) -> bool {
        if !self.owns_live_request(id)
            || self.screen != Screen::Preparing
            || self.awaiting_continuation
            || self.interval_playback.is_some()
            || self.playback.is_some()
        {
            return false;
        }
        if !self.request_unchanged() || !self.fixture_matches(&prefix.clubs, &prefix.club_names) {
            return self.accept_live_error(id, "interval request/fixture differs".into());
        }
        match IntervalPlayback::new(prefix) {
            Ok(playback) => {
                self.interval_playback = Some(playback);
                self.prepare_requested = false;
                self.screen = Screen::Pitch;
                true
            }
            Err(error) => self.accept_live_error(id, error),
        }
    }
    pub fn accept_continuation(&mut self, id: u64, completion: Completion) -> bool {
        if !self.owns_live_request(id)
            || self.screen != Screen::Preparing
            || !self.awaiting_continuation
        {
            return false;
        }
        if !self.request_unchanged()
            || !self.fixture_matches(&completion.timeline.clubs, &completion.timeline.club_names)
            || completion.timeline.validate_markers().is_err()
            || completion.native_sha256.len() != 64
            || !completion
                .native_sha256
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
        {
            return self.accept_live_error(id, "completed request/fixture differs".into());
        }
        let Some(prefix) = self.interval_playback.take() else {
            return self.accept_live_error(id, "missing owned interval".into());
        };
        // [designed:T78] The completion must carry the executed exchange exactly
        // when this controller requested one; stale or duplicate decisions refuse.
        if completion.replacement.is_some() != self.replace_requested {
            return self.accept_live_error(
                id,
                "completed continuation differs from the owned decision".into(),
            );
        }
        match prefix.attach(completion.timeline) {
            Ok(playback) => {
                self.playback = Some(playback);
                self.pending_replacement = completion.replacement;
                self.awaiting_continuation = false;
                self.resume_requested = false;
                self.replace_requested = false;
                self.squad = None;
                self.screen = Screen::Pitch;
                true
            }
            Err(error) => self.accept_live_error(id, error),
        }
    }
    pub fn accept_live_error(&mut self, id: u64, error: String) -> bool {
        if !self.owns_live_request(id) {
            return false;
        }
        eprintln!("live match stopped: {error}");
        self.clear_match();
        self.screen = Screen::Preparation;
        self.message = "Match stopped. Campaign unchanged; retry PLAY GAME.".into();
        true
    }
    /// [confirmed:W027,archaeology011a341] History contains only reached actions.
    pub fn history_lines(&self) -> Vec<String> {
        self.match_view()
            .map(|p| crate::history::lines(p.events))
            .unwrap_or_default()
    }
    pub fn visible_history(&self) -> Vec<String> {
        let lines = self.history_lines();
        let top = self
            .history_top
            .min(lines.len().saturating_sub(crate::history::PAGE_LINES));
        lines
            .into_iter()
            .skip(top)
            .take(crate::history::PAGE_LINES)
            .collect()
    }
    fn players(&mut self) {
        self.caller = self.screen;
        self.screen = Screen::Players;
    }
    fn unsupported(&mut self) {
        self.message = "This action is outside the desktop feasibility slice.".into();
    }
    /// [designed:T78] Confirmation sends the witnessed exchange to the owned
    /// worker; every other pair refuses here with readable feedback, before any
    /// engine contact. The snapshot stays read-only either way.
    fn confirm_replacement(&mut self, out_slot: usize, in_slot: usize) {
        let Some(playback) = self.interval_playback.as_ref() else {
            self.message = "The owned interval is gone.".into();
            return;
        };
        let roster = &playback.timeline.snapshot.roster;
        let (w_out_slot, w_out, w_in_slot, w_in) = WITNESSED;
        let ids = (out_slot, in_slot) == (w_out_slot, w_in_slot)
            && roster.get(out_slot) == Some(&(u64::from(w_out)))
            && roster.get(in_slot) == Some(&(u64::from(w_in)));
        if !ids {
            self.message =
                "Only the witnessed Pearce/Gale exchange is admitted in this build.".into();
            return;
        }
        self.squad = None;
        self.replace_requested = true;
        self.awaiting_continuation = true;
        self.screen = Screen::Preparing;
    }
    /// [designed] Supported prototype actions; original keeper/age guards preserved.
    pub fn option_available(&self, option: usize) -> bool {
        let Some(p) = self.editor.card().and_then(|id| self.editor.player(id)) else {
            return false;
        };
        match option {
            0 => p.club_position != 0,
            1 => p.age >= 18,
            4 | 7 => true,
            _ => false,
        }
    }
    pub fn accept_error(&mut self, id: u64, error: String) -> bool {
        if id != self.job_id
            || self.screen != Screen::Preparing
            || self.interval_playback.is_some()
            || self.awaiting_continuation
        {
            return false;
        }
        self.match_ticket = None;
        self.message = error;
        self.screen = Screen::Preparation;
        self.prepare_requested = false;
        true
    }
    pub fn accept_timeline(&mut self, id: u64, timeline: Timeline) -> bool {
        if id != self.job_id
            || self.screen != Screen::Preparing
            || self.interval_playback.is_some()
            || self.awaiting_continuation
        {
            return false;
        }
        if let Err(error) = timeline.validate_markers() {
            return self.accept_error(id, error);
        }
        if !self.exhibition
            && (self.match_ticket.is_none()
                || timeline.native_return != "0AB9:EBE2"
                || timeline.frames.is_empty()
                || timeline.clubs.len() != 2
                || !timeline
                    .clubs
                    .contains(&u64::from(self.editor.campaign.club.id))
                || !timeline
                    .clubs
                    .contains(&u64::from(self.editor.campaign.next_fixture.opponent))
                || timeline.club_names
                    != [
                        self.editor.campaign.club.name.clone(),
                        self.editor.campaign.next_fixture.opponent_name.clone(),
                    ])
        {
            return self.accept_error(
                id,
                "Match output refused: fixture or completion differs.".into(),
            );
        }
        let mut playback = Playback::new(timeline);
        if let Err(e) = playback.start_or_continue() {
            return self.accept_error(id, e);
        }
        self.playback = Some(playback);
        self.prepare_requested = false;
        self.screen = Screen::Pitch;
        true
    }
    /// [designed:T65] Keep the computed receipt visible without claiming week progression.
    pub fn recorded_result_text(&self) -> Option<String> {
        self.editor.campaign.played_fixture.as_ref().map(|result| {
            format!(
                "Result recorded: {} {}-{} {}. F7 saves.",
                self.editor.campaign.club.name,
                result.score[0],
                result.score[1],
                result.fixture.opponent_name
            )
        })
    }
    fn record_completion(&mut self) -> Result<(), String> {
        let ticket = self.match_ticket.take().ok_or("missing match request")?;
        let replacement = self.pending_replacement.take();
        let playback = self.playback.as_ref().ok_or("missing match playback")?;
        if playback.phase != Phase::FullTime || playback.cursor != playback.timeline.frames.len() {
            return Err("match has not completed".into());
        }
        let score = playback.view().score.map(u16::try_from);
        let [first, second] = score;
        let native_return = playback.timeline.native_return.as_str();
        match replacement {
            // [designed:T78] One campaign transaction: result ticket plus the
            // researched persisted roster exchange (archaeology T80 policy).
            Some(replacement) => self.editor.campaign.record_match_with_replacement(
                &ticket,
                [
                    first.map_err(|_| "score outside native storage")?,
                    second.map_err(|_| "score outside native storage")?,
                ],
                native_return,
                replacement.out_slot,
                replacement.in_slot,
                u16::try_from(replacement.out_id)
                    .map_err(|_| "player id outside native storage")?,
                u16::try_from(replacement.in_id).map_err(|_| "player id outside native storage")?,
            ),
            None => self.editor.campaign.record_match(
                &ticket,
                [
                    first.map_err(|_| "score outside native storage")?,
                    second.map_err(|_| "score outside native storage")?,
                ],
                native_return,
            ),
        }
    }
    pub(crate) fn clear_match(&mut self) {
        self.match_ticket = None;
        self.playback = None;
        self.interval_playback = None;
        self.resume_requested = false;
        self.replace_requested = false;
        self.squad = None;
        self.pending_replacement = None;
        self.awaiting_continuation = false;
        self.request_campaign = None;
        self.prepare_requested = false;
        self.exhibition = false;
    }
    pub fn tick(&mut self) {
        if self.screen != Screen::Pitch {
            return;
        }
        let phase = if let Some(p) = self.interval_playback.as_mut() {
            p.step(usize::from(self.speed.saturating_sub(20)).max(1))
                .map(|_| p.phase)
        } else if let Some(p) = self.playback.as_mut() {
            p.step(usize::from(self.speed.saturating_sub(20)).max(1))
                .map(|_| p.phase)
        } else {
            return;
        };
        match phase {
            Err(error) => {
                self.message = error;
                self.screen = Screen::Bench;
            }
            Ok(Phase::HalfTime) => self.screen = Screen::HalfTime,
            Ok(Phase::FullTime) => self.screen = Screen::FullTime,
            _ => (),
        }
    }
    pub fn key(&mut self, key: Key) {
        self.message.clear();
        match self.screen {
            // [designed] Clamp at list edges; W006 establishes interior movement only.
            Screen::SavedGames => match key {
                Key::Up => self.load_index = self.load_index.saturating_sub(1),
                Key::Down => {
                    self.load_index =
                        (self.load_index + 1).min(self.load_entries.len().saturating_sub(1))
                }
                Key::Enter => self.load_requested = true,
                // [confirmed] T43/r0003 step14: Escape keeps the selected save open.
                Key::Escape => (),
                // [designed] Native shortcut cancellation, not a DOS key claim.
                Key::F9 => {
                    self.screen = Screen::Main;
                    self.load_entries.clear();
                }
                _ => (),
            },
            Screen::Main => match key {
                Key::Up => self.menu = (self.menu + 16) % 17,
                Key::Down => self.menu = (self.menu + 1) % 17,
                Key::Space => {
                    // [confirmed:T69 UI main-return, retained by T66] The
                    // shortcut selects Players; Escape keeps that main item.
                    self.menu = 1;
                    self.players();
                }
                Key::Enter if self.menu == 0 => {
                    self.exhibition = false;
                    self.screen = Screen::Preparation;
                }
                Key::Exhibition => {
                    self.exhibition = true;
                    self.screen = Screen::Preparation;
                    self.preparation_menu = 2;
                }
                Key::Enter if self.menu == 1 => self.players(),
                Key::F7 => self.save_requested = true,
                Key::F9 => {
                    self.load_requested = true;
                }
                Key::F10 => self.screen = Screen::Quit,
                _ => self.unsupported(),
            },
            Screen::Players => match key {
                Key::Up | Key::Down => {
                    let (i, reserve) = match self.editor.selected {
                        Location::First(i) => (i, false),
                        Location::Reserve(i) => (i, true),
                    };
                    let next = if key == Key::Down {
                        (i + 1).min(19)
                    } else {
                        i.saturating_sub(1)
                    };
                    self.editor
                        .select(if reserve {
                            Location::Reserve(next)
                        } else {
                            Location::First(next)
                        })
                        .unwrap();
                    if !reserve {
                        self.remembered_first = next;
                    }
                }
                Key::Left | Key::Right => {
                    let at = match self.editor.selected {
                        Location::First(i) => {
                            self.remembered_first = i;
                            Location::Reserve(0)
                        }
                        Location::Reserve(_) => Location::First(self.remembered_first),
                    };
                    self.editor.select(at).unwrap();
                }
                Key::Insert => {
                    self.editor.panel = if self.editor.panel == Panel::Skills {
                        Panel::Details
                    } else {
                        Panel::Skills
                    }
                }
                Key::Enter => {
                    let result = if self.editor.carried_player().is_some() {
                        self.editor.place()
                    } else {
                        self.editor.pick()
                    };
                    if let Err(e) = result {
                        self.message = e;
                    }
                }
                Key::Escape => {
                    if self.editor.carried_player().is_some() {
                        self.editor.cancel();
                    } else {
                        self.screen = self.caller;
                    }
                }
                Key::Space
                    if self.editor.carried_player().is_none() && self.editor.card().is_some() =>
                {
                    self.editor.panel = Panel::Details;
                    self.screen = Screen::Options
                }
                _ => self.unsupported(),
            },
            // [confirmed] B271 disables the fifth Mark selection in management (0D91=0).
            Screen::Instructions => match key {
                Key::Up => self.instruction = self.instruction.saturating_sub(1),
                Key::Down => self.instruction = (self.instruction + 1).min(3),
                Key::Escape => self.screen = Screen::Options,
                Key::Insert => {
                    self.editor.panel = Panel::Skills;
                    self.screen = Screen::Options;
                }
                Key::Enter if self.instruction == 0 => {
                    if let Some(id) = self.editor.card() {
                        if let Err(e) = self.editor.campaign.cycle_goalkeeper_positioning(id) {
                            self.message = e;
                        }
                    }
                }
                _ => self.message = "This instruction action is unavailable in this slice.".into(),
            },
            Screen::Options => match key {
                Key::Insert => {
                    self.editor.panel = if self.editor.panel == Panel::Details {
                        Panel::Skills
                    } else {
                        Panel::Details
                    }
                }
                Key::Up => self.option = self.option.saturating_sub(1),
                Key::Down => self.option = (self.option + 1).min(7),
                Key::Escape => self.screen = Screen::Players,
                Key::Enter if self.option == 7 => self.screen = Screen::Players,
                Key::Enter if self.option == 0 || self.option == 1 => {
                    if let Some(id) = self.editor.card() {
                        let result = if self.option == 0 {
                            self.editor.campaign.reposition(id)
                        } else {
                            self.editor.campaign.restore_club_position(id)
                        };
                        if let Err(e) = result {
                            self.message = e;
                        }
                    }
                }
                Key::Enter if self.option == 4 => {
                    self.editor.panel = Panel::Details;
                    self.screen = Screen::Instructions;
                }
                Key::Enter => {
                    self.message = "This player action is unavailable in this slice.".into()
                }
                _ => self.unsupported(),
            },
            Screen::Preparation | Screen::Bench => match key {
                Key::Space if self.screen == Screen::Bench && self.interval_halftime() => {
                    // [designed:T78] The original W028 bench: SPACE opens the squad
                    // view over the owned interval snapshot. Read-only until confirm.
                    self.squad = Some(SquadSelection {
                        highlight: WITNESSED.0,
                        picked: None,
                    });
                    self.screen = Screen::MatchSquad;
                }
                Key::Up => self.preparation_menu = self.preparation_menu.saturating_sub(1),
                Key::Down => self.preparation_menu = (self.preparation_menu + 1).min(3),
                Key::Space if self.screen == Screen::Preparation && !self.exhibition => {
                    self.players()
                }
                Key::Exhibition if self.screen == Screen::Preparation => {
                    self.exhibition = true;
                    self.preparation_menu = 2;
                }
                Key::Enter if self.preparation_menu == 2 => {
                    if self.screen == Screen::Preparation {
                        if !self.exhibition {
                            if self.editor.carried_player().is_some() {
                                self.message =
                                    "Finish or cancel the pending player move first.".into();
                                return;
                            }
                            if let Err(error) =
                                crate::campaign_match::validate(&self.editor.campaign)
                            {
                                self.message = error;
                                return;
                            }
                        }
                        self.match_ticket = if self.exhibition {
                            None
                        } else {
                            match self.editor.campaign.match_ticket() {
                                Ok(ticket) => Some(ticket),
                                Err(error) => {
                                    self.message = error;
                                    return;
                                }
                            }
                        };
                        self.playback = None;
                        self.interval_playback = None;
                        self.resume_requested = false;
                        self.awaiting_continuation = false;
                        self.request_campaign = if self.exhibition {
                            None
                        } else {
                            Some(self.editor.campaign.clone())
                        };
                        self.job_id += 1;
                        self.prepare_requested = true;
                        self.screen = Screen::Preparing;
                    } else {
                        if self
                            .interval_playback
                            .as_ref()
                            .is_some_and(|p| p.phase == Phase::HalfTime)
                        {
                            self.resume_requested = true;
                            self.awaiting_continuation = true;
                            self.screen = Screen::Preparing;
                            return;
                        }
                        if let Some(p) = self.playback.as_mut() {
                            if p.phase == Phase::HalfTime {
                                let _ = p.start_or_continue();
                            }
                        }
                        self.screen = Screen::Pitch;
                    }
                }
                Key::Escape if self.screen == Screen::Preparation => {
                    self.exhibition = false;
                    self.screen = Screen::Main;
                }
                _ => self.unsupported(),
            },
            Screen::Preparing => {
                if key == Key::Escape {
                    self.job_id += 1;
                    self.match_ticket = None;
                    self.request_campaign = None;
                    self.interval_playback = None;
                    self.playback = None;
                    self.resume_requested = false;
                    self.awaiting_continuation = false;
                    self.prepare_requested = false;
                    self.screen = Screen::Preparation;
                }
            }
            Screen::Pitch => match key {
                Key::Space => {
                    self.history_top = self
                        .history_lines()
                        .len()
                        .saturating_sub(crate::history::PAGE_LINES);
                    self.screen = Screen::History;
                }
                Key::Escape | Key::Enter => {
                    self.preparation_menu = 2;
                    self.screen = Screen::Bench;
                }
                Key::Right => self.speed = self.speed.saturating_add(1).min(24),
                Key::Left => self.speed = self.speed.saturating_sub(1).max(1),
                _ => self.unsupported(),
            },
            Screen::History => {
                let end = self
                    .history_lines()
                    .len()
                    .saturating_sub(crate::history::PAGE_LINES);
                self.history_top = self.history_top.min(end);
                match key {
                    Key::Up => self.history_top = self.history_top.saturating_sub(1),
                    Key::Down => self.history_top = self.history_top.saturating_add(1).min(end),
                    Key::PageUp => {
                        self.history_top =
                            self.history_top.saturating_sub(crate::history::PAGE_LINES)
                    }
                    Key::PageDown => {
                        self.history_top = self
                            .history_top
                            .saturating_add(crate::history::PAGE_LINES)
                            .min(end)
                    }
                    Key::Home => self.history_top = 0,
                    Key::End => self.history_top = end,
                    Key::Space | Key::Escape => self.screen = Screen::Pitch,
                    _ => self.unsupported(),
                }
            }
            // [designed:T78] The interval squad view: W028 pick/put-here keys
            // over the read-only snapshot; only the witnessed exchange is admitted.
            Screen::MatchSquad => {
                let Some(squad) = self.squad.as_mut() else {
                    self.screen = Screen::Bench;
                    return;
                };
                match key {
                    Key::Up => squad.highlight = squad.highlight.saturating_sub(1),
                    Key::Down => squad.highlight = (squad.highlight + 1).min(19),
                    Key::Left | Key::Right => {
                        let row = if squad.highlight < 11 {
                            squad.highlight.min(8)
                        } else {
                            squad.highlight - 11
                        };
                        squad.highlight = if squad.highlight < 11 { 11 + row } else { row };
                    }
                    Key::Enter if squad.picked.is_none() => {
                        if squad.highlight >= 11 {
                            self.message = "Pick a first-team player first.".into();
                            return;
                        }
                        squad.picked = Some(squad.highlight);
                    }
                    Key::Enter if squad.highlight < 11 => {
                        self.message = "Choose a bench player to put here.".into();
                    }
                    Key::Enter => {
                        let out_slot = squad.picked.unwrap_or_default();
                        let in_slot = squad.highlight;
                        self.confirm_replacement(out_slot, in_slot);
                    }
                    Key::Escape => {
                        if squad.picked.is_some() {
                            squad.picked = None;
                        } else {
                            self.squad = None;
                            self.screen = Screen::Bench;
                        }
                    }
                    _ => self.unsupported(),
                }
            }
            Screen::HalfTime => {
                self.preparation_menu = 2;
                self.screen = Screen::Bench;
            }
            Screen::FullTime => {
                self.screen = Screen::Main;
                self.message = if self.exhibition {
                    "Exhibition complete. No campaign result was posted.".into()
                } else {
                    match self.record_completion() {
                        Ok(()) => self.recorded_result_text().unwrap(),
                        Err(_) => {
                            "Result not recorded: missing, changed or unfinished match request."
                                .into()
                        }
                    }
                };
                self.exhibition = false;
            }
            Screen::Quit => match key {
                Key::Y => self.close_requested = true,
                Key::N | Key::Escape => self.screen = Screen::Main,
                _ => (),
            },
        }
    }
}
