//! [derived] Presentation adapter from tools/play_watched.py at a1599a4; no engine rules.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub scene: u64,
    pub player: u64,
    pub club: u64,
    pub focus: bool,
    pub position: Position,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct View {
    pub clock: u64,
    pub score: [u64; 2],
    pub ball: Position,
    pub players: Vec<Player>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: usize,
    pub native: Value,
    pub kind: String,
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Frame {
    pub view: View,
    pub events: Vec<Event>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Timeline {
    pub club_names: [String; 2],
    pub initial: View,
    pub frames: Vec<Frame>,
    pub setup_events: Vec<Event>,
    pub native_return: String,
    pub draws: usize,
    pub clock_rows: usize,
    pub seed: u64,
    pub clubs: Vec<u64>,
    /// [derived:F30] Initial, then one row per computed frame. Each pair is
    /// stored selector and original display attribute, in view-player order.
    pub marker_rows: Vec<Vec<[u16; 2]>>,
}
impl Timeline {
    pub fn validate_markers(&self) -> Result<(), String> {
        if self.marker_rows.len() != self.frames.len() + 1 {
            return Err("Match display data is incomplete.".into());
        }
        for (row, view) in self
            .marker_rows
            .iter()
            .zip(std::iter::once(&self.initial).chain(self.frames.iter().map(|f| &f.view)))
        {
            if row.len() != view.players.len()
                || row.len() != 22
                || row.iter().flatten().any(|v| *v > 255)
            {
                return Err("Match display data is invalid.".into());
            }
        }
        Ok(())
    }
}
/// [derived:F30,archaeology011a341,0AB9:0207/02D7] Mode4 numbers.
/// [designed:T72] G replaces the private custom keeper glyph1 independently.
pub fn numbered_marker(selector: u16) -> char {
    if matches!(selector, 1 | 12) {
        'G'
    } else {
        char::from((selector.wrapping_sub(1) % 11 + 47) as u8)
    }
}
fn marker_row(snapshot: &Value) -> Result<Vec<[u16; 2]>, String> {
    a(&snapshot["scenes"])?
        .iter()
        .map(|scene| {
            let selector = n(&scene["byte7"])?;
            let word = n(&scene["word4"])?;
            if selector > 255 || word > 65535 {
                return Err("Invalid computed marker fields.".into());
            }
            Ok([selector as u16, (word >> 8) as u16])
        })
        .collect()
}
fn n(v: &Value) -> Result<u64, String> {
    v.as_u64()
        .ok_or_else(|| format!("expected unsigned integer: {v}"))
}
fn a(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array().ok_or_else(|| "expected array".into())
}
fn coord(v: &Value) -> Result<Position, String> {
    let word = n(v)? as u16;
    Ok(Position {
        x: (word as u8 as i8) as i16,
        y: ((word >> 8) as u8 as i8) as i16,
    })
}
fn view(
    snapshot: &Value,
    goals: &Value,
    clock: u64,
    names: &Value,
    player_clubs: &BTreeMap<u64, u64>,
    clubs: &[u64],
) -> Result<View, String> {
    let players = a(&snapshot["scenes"])?
        .iter()
        .map(|s| {
            let player = n(&s["word14"])?;
            let club = *player_clubs.get(&player).ok_or("missing player club")?;
            if !names[player.to_string()].is_string() || !clubs.contains(&club) {
                return Err("missing player identity".into());
            }
            Ok(Player {
                scene: n(&s["index"])?,
                player,
                club,
                focus: s["index"] == snapshot["actor"],
                position: coord(&s["word0"])?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    if players.len() != 22 {
        return Err("expected 22 native positions".into());
    }
    if a(goals)?.len() != 2 {
        return Err("expected two native scores".into());
    }
    Ok(View {
        clock,
        score: [n(&goals[0])?, n(&goals[1])?],
        ball: coord(&snapshot["ball"]["word0"])?,
        players,
    })
}
fn describe(
    event: &Value,
    id: usize,
    names: &Value,
    messages: &[(String, String)],
) -> Result<Event, String> {
    let (kind, template) = messages
        .get(n(&event["code"])? as usize)
        .ok_or("unknown commentary code")?;
    let mut text = template.clone();
    for (marker, field) in [("{p}", "player"), ("{s}", "secondary")] {
        if text.contains(marker) {
            let player = n(&event[field])?;
            if player == 0 {
                return Err("zero commentary identity".into());
            }
            let name = names[player.to_string()]
                .as_str()
                .ok_or("missing commentary name")?;
            text = text.replace(marker, name);
        }
    }
    Ok(Event {
        id,
        native: event.clone(),
        kind: kind.clone(),
        text,
    })
}
pub fn compute(root: &Path) -> Result<Timeline, String> {
    let raw = std::fs::read(root.join("engine/tests/fixtures/prepared_match/admission.json"))
        .map_err(|e| e.to_string())?;
    if format!("{:x}", Sha256::digest(&raw))
        != "842f1453cae4ba9ab626d7e57316494cfe7e70f64ea80e746c3834de0e12989f"
    {
        return Err("prepared fixture digest differs".into());
    }
    let fixture: Value = serde_json::from_slice(&raw).map_err(|e| e.to_string())?;
    let metadata: Value = serde_json::from_slice(
        &std::fs::read(root.join("demo/metadata.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    compute_input(
        fixture["input"].clone(),
        &metadata["players"],
        None,
        [130, 134],
        ["Blackburn R".into(), "Sheffield Wed".into()],
    )
}
pub fn compute_campaign(campaign: &onenil_campaign::Campaign) -> Result<Timeline, String> {
    let input = crate::campaign_match::admission(campaign)?;
    let metadata = crate::campaign_match::metadata()?;
    let display_clubs = [campaign.club.id, campaign.next_fixture.opponent];
    let club_names = display_clubs.map(|id| {
        metadata["clubs"][id.to_string()]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| "missing campaign club name".to_owned())
    });
    let [first, second] = club_names;
    compute_input(
        serde_json::to_value(input).map_err(|error| error.to_string())?,
        &metadata["players"],
        Some(crate::campaign_match::profile(campaign)?),
        display_clubs.map(u64::from),
        [first?, second?],
    )
}
/// [designed:T76] Incomplete reached views. This type has no completed-return field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IntervalTimeline {
    pub boundary: String,
    pub control: u8,
    pub club_names: [String; 2],
    pub initial: View,
    pub frames: Vec<Frame>,
    pub setup_events: Vec<Event>,
    pub draws: usize,
    pub clock_rows: usize,
    pub seed: u64,
    pub clubs: Vec<u64>,
    pub marker_rows: Vec<Vec<[u16; 2]>>,
}
impl IntervalTimeline {
    pub fn validate(&self) -> Result<(), String> {
        if self.boundary != "0AB9:7E4A"
            || self.control != 1
            || self.frames.is_empty()
            || self.frames.last().unwrap().view.clock < 45
            || self.marker_rows.len() != self.frames.len() + 1
            || !self
                .frames
                .iter()
                .flat_map(|f| &f.events)
                .any(|e| e.kind == "half-time")
            || self
                .frames
                .iter()
                .flat_map(|f| &f.events)
                .any(|e| e.kind == "full-time")
        {
            return Err("invalid incomplete interval boundary".into());
        }
        for (row, view) in self
            .marker_rows
            .iter()
            .zip(std::iter::once(&self.initial).chain(self.frames.iter().map(|f| &f.view)))
        {
            if row.len() != 22
                || row.len() != view.players.len()
                || row.iter().flatten().any(|v| *v > 255)
            {
                return Err("invalid interval markers".into());
            }
        }
        Ok(())
    }
    pub fn is_prefix_of(&self, finished: &Timeline) -> bool {
        finished.native_return == "0AB9:EBE2"
            && finished
                .frames
                .iter()
                .flat_map(|f| &f.events)
                .any(|e| e.kind == "full-time")
            && finished.frames.len() > self.frames.len()
            && self.club_names == finished.club_names
            && self.initial == finished.initial
            && self.setup_events == finished.setup_events
            && self.seed == finished.seed
            && self.clubs == finished.clubs
            && finished.draws >= self.draws
            && finished.clock_rows >= self.clock_rows
            && finished.frames.starts_with(&self.frames)
            && finished.marker_rows.starts_with(&self.marker_rows)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Completion {
    pub timeline: Timeline,
    pub native_sha256: String,
}
struct TimelineContext {
    initial: Value,
    setup_draws: Vec<Value>,
    seed: u64,
    names: Value,
    messages: Vec<(String, String)>,
    clubs: Vec<u64>,
    player_clubs: BTreeMap<u64, u64>,
    display_clubs: [u64; 2],
    club_names: [String; 2],
}
struct ViewData {
    club_names: [String; 2],
    initial: View,
    frames: Vec<Frame>,
    setup_events: Vec<Event>,
    draws: usize,
    clock_rows: usize,
    seed: u64,
    clubs: Vec<u64>,
    marker_rows: Vec<Vec<[u16; 2]>>,
}
impl ViewData {
    fn complete(self) -> Timeline {
        let Self {
            club_names,
            initial,
            frames,
            setup_events,
            draws,
            clock_rows,
            seed,
            clubs,
            marker_rows,
        } = self;
        Timeline {
            club_names,
            initial,
            frames,
            setup_events,
            draws,
            clock_rows,
            seed,
            clubs,
            marker_rows,
            native_return: "0AB9:EBE2".into(),
        }
    }
    fn interval(self) -> IntervalTimeline {
        let Self {
            club_names,
            initial,
            frames,
            setup_events,
            draws,
            clock_rows,
            seed,
            clubs,
            marker_rows,
        } = self;
        IntervalTimeline {
            club_names,
            initial,
            frames,
            setup_events,
            draws,
            clock_rows,
            seed,
            clubs,
            marker_rows,
            boundary: "0AB9:7E4A".into(),
            control: 1,
        }
    }
}
fn prepare_input(
    input: Value,
    names: &Value,
    profile: Option<crate::campaign_match::Profile>,
    display_clubs: [u64; 2],
    club_names: [String; 2],
) -> Result<
    (
        onenil_engine::watched_compiled::prepared::Session,
        TimelineContext,
    ),
    String,
> {
    let setup = &input["preparation"]["setup"]["legacy"];
    let clubs = a(&setup["substitution"]["context"]["clubs"])?
        .iter()
        .map(|c| n(&c["id"]))
        .collect::<Result<Vec<_>, _>>()?;
    let player_clubs = a(&setup["players"])?
        .iter()
        .map(|p| Ok((n(&p["id"])?, n(&p["p8"][0])?)))
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    let messages: Vec<(String, String)> =
        serde_json::from_str(include_str!("../messages.json")).map_err(|e| e.to_string())?;
    let input = serde_json::from_value(input).map_err(|e| e.to_string())?;
    let fresh = profile.is_some();
    let session = match profile {
        Some(crate::campaign_match::Profile::Original) => {
            onenil_engine::watched_compiled::prepared::Session::new_fresh(input)?
        }
        Some(crate::campaign_match::Profile::Edited) => {
            onenil_engine::watched_compiled::prepared::Session::new_edited(input)?
        }
        None => onenil_engine::watched_compiled::prepared::Session::new(input)?,
    };
    let initial = session.snapshot()?;
    let observation = session.observation();
    let setup_draws = a(&observation["draws"])?.clone();
    let seed = n(&setup_draws.first().ok_or("missing setup draw")?["seed_before"])?;
    if !fresh && (seed != 64 || !a(&initial["scenes"])?.iter().any(|s| s["word14"] == 2590)) {
        return Err("native selection differs".into());
    }
    Ok((
        session,
        TimelineContext {
            initial,
            setup_draws,
            seed,
            names: names.clone(),
            messages,
            clubs,
            player_clubs,
            display_clubs,
            club_names,
        },
    ))
}
fn compute_input(
    input: Value,
    names: &Value,
    profile: Option<crate::campaign_match::Profile>,
    display_clubs: [u64; 2],
    club_names: [String; 2],
) -> Result<Timeline, String> {
    let (mut session, context) = prepare_input(input, names, profile, display_clubs, club_names)?;
    let output = session.finish().map_err(|e| {
        serde_json::to_string(&e).unwrap_or_else(|_| "native preparation failed".into())
    })?;
    context.complete(&output)
}
impl TimelineContext {
    fn complete(&self, output: &Value) -> Result<Timeline, String> {
        if output["return"] != "0AB9:EBE2" || !a(&output["draws"])?.starts_with(&self.setup_draws) {
            return Err("native return/draw prefix differs".into());
        }
        Ok(self.adapt(output)?.complete())
    }
    fn interval(&self, output: &Value) -> Result<IntervalTimeline, String> {
        if output["boundary"] != "0AB9:7E4A"
            || output["control"] != 1
            || output.get("return").is_some()
            || !a(&output["draws"])?.starts_with(&self.setup_draws)
        {
            return Err("native interval/draw prefix differs".into());
        }
        let interval = self.adapt(output)?.interval();
        interval.validate()?;
        Ok(interval)
    }
    fn adapt(&self, output: &Value) -> Result<ViewData, String> {
        let names = &self.names;
        let messages = &self.messages;
        let initial = &self.initial;
        let player_clubs = &self.player_clubs;
        let clubs = self.clubs.clone();
        let seed = self.seed;
        let display_clubs = self.display_clubs;
        let club_names = self.club_names.clone();
        // [confirmed:T67 original actions] Queue side0 is PSG in this fresh
        // fixture,whereas it is Blackburn in the older exhibition. Use the
        // computed native club/side identities;preparation array order is not ABI.
        let mut native_clubs = [None; 2];
        for event in a(&output["setup_events"])?
            .iter()
            .chain(a(&output["events"])?)
        {
            let side = usize::try_from(n(&event["side"])?).map_err(|_| "invalid event side")?;
            let slot = native_clubs.get_mut(side).ok_or("invalid event side")?;
            let club = n(&event["club"])?;
            if !display_clubs.contains(&club) || slot.is_some_and(|prior| prior != club) {
                return Err("native score-side club identity differs".into());
            }
            *slot = Some(club);
        }
        let [first, second] = display_clubs.map(|club| {
            native_clubs
                .iter()
                .position(|value| *value == Some(club))
                .ok_or("missing native scoring club")
        });
        let order = [first?, second?];
        let score = |value: &Value| -> Result<Value, String> {
            Ok(serde_json::json!([
                n(&value[order[0]])?,
                n(&value[order[1]])?
            ]))
        };
        let setup_events = a(&output["setup_events"])?
            .iter()
            .enumerate()
            .map(|(i, e)| describe(e, i, names, &messages))
            .collect::<Result<Vec<_>, _>>()?;
        let mut events: BTreeMap<u64, Vec<Event>> = BTreeMap::new();
        for (i, e) in a(&output["events"])?.iter().enumerate() {
            events.entry(n(&e["tick"])?).or_default().push(describe(
                e,
                i + setup_events.len(),
                names,
                &messages,
            )?);
        }
        let mut clocks = BTreeMap::new();
        for c in a(&output["caller_clocks"])? {
            clocks.insert(n(&c["tick"])?, n(&c["clock"]["elapsed"])?);
        }
        let ticks = a(&output["ticks"])?;
        let goals = a(&output["tick_live_goals"])?;
        if ticks.len() != goals.len() || ticks.is_empty() {
            return Err("incomplete native score observations".into());
        }
        let mut frames = Vec::new();
        let mut marker_rows = vec![marker_row(&initial)?];
        for (t, g) in ticks.iter().zip(goals) {
            let clock = clocks
                .get(&n(&t["tick"])?)
                .copied()
                .unwrap_or(n(&t["after"]["elapsed"])?);
            let actions = events.remove(&n(&t["tick"])?).unwrap_or_default();
            if actions
                .iter()
                .any(|e| e.native["minute"].as_u64().is_none_or(|m| m > clock))
            {
                return Err("event newer than observed clock".into());
            }
            marker_rows.push(marker_row(&t["after"])?);
            frames.push(Frame {
                view: view(&t["after"], &score(g)?, clock, names, &player_clubs, &clubs)?,
                events: actions,
            });
        }
        if !events.is_empty()
            || frames.last().unwrap().view.score
                != [
                    n(&output["live_goals"][order[0]])?,
                    n(&output["live_goals"][order[1]])?,
                ]
        {
            return Err("incomplete native timeline".into());
        }
        Ok(ViewData {
            club_names,
            initial: view(
                &initial,
                &score(&initial["scores"])?,
                n(&initial["elapsed"])?,
                names,
                &player_clubs,
                &clubs,
            )?,
            frames,
            setup_events,
            draws: a(&output["draws"])?.len(),
            clock_rows: clocks.len(),
            seed,
            clubs,
            marker_rows,
        })
    }
}

/// [designed:T76] One owned engine image; UI/IPC cannot set guest state.
pub struct LiveCampaign {
    session: onenil_engine::watched_compiled::prepared::Session,
    context: TimelineContext,
    at_interval: bool,
    ended: bool,
    replacement_allowed: bool,
}
impl LiveCampaign {
    pub fn new(campaign: &onenil_campaign::Campaign) -> Result<Self, String> {
        let profile = crate::campaign_match::profile(campaign)?;
        let mut input = crate::campaign_match::admission(campaign)?;
        let replacement_allowed = profile == crate::campaign_match::Profile::Original;
        if replacement_allowed {
            // [confirmed:T75] Explicit incoming facts; no expected output witness.
            let facts: Value = serde_json::from_str(include_str!(
                "../../../engine/tests/fixtures/halftime_replacement/incoming.json"
            ))
            .map_err(|e| e.to_string())?;
            input.support.extend(
                serde_json::from_value::<Vec<onenil_engine::watched_compiled::action::ByteFact>>(
                    facts["support"].clone(),
                )
                .map_err(|e| e.to_string())?,
            );
        }
        let metadata = crate::campaign_match::metadata()?;
        let clubs = [campaign.club.id, campaign.next_fixture.opponent];
        let names = clubs.map(|id| {
            metadata["clubs"][id.to_string()]
                .as_str()
                .map(str::to_owned)
                .ok_or("missing campaign club name")
        });
        let [first, second] = names;
        let (session, context) = prepare_input(
            serde_json::to_value(input).map_err(|e| e.to_string())?,
            &metadata["players"],
            Some(profile),
            clubs.map(u64::from),
            [first?.into(), second?.into()],
        )?;
        Ok(Self {
            session,
            context,
            at_interval: false,
            ended: false,
            replacement_allowed,
        })
    }
    pub fn pause(&mut self) -> Result<IntervalTimeline, String> {
        if self.at_interval || self.ended {
            return Err("live session cannot pause in this phase".into());
        }
        let result = (|| {
            self.session.pause_at_interval()?;
            self.context.interval(&self.session.interval_history()?)
        })();
        match result {
            Ok(prefix) => {
                self.at_interval = true;
                Ok(prefix)
            }
            Err(error) => {
                self.ended = true;
                Err(error)
            }
        }
    }
    pub fn finish(&mut self, replace: bool) -> Result<Completion, String> {
        if !self.at_interval || self.ended {
            return Err("live session has no pending interval decision".into());
        }
        if replace && !self.replacement_allowed {
            return Err("replacement is supported only in the verified original campaign".into());
        }
        self.ended = true;
        let output = if replace {
            let mut result = self.session.finish_with_interval_replacement(false)?;
            result["match"].take()
        } else {
            self.session.finish().map_err(|e| e.fault.message)?
        };
        let native_sha256 = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&output).map_err(|e| e.to_string())?)
        );
        Ok(Completion {
            timeline: self.context.complete(&output)?,
            native_sha256,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Ready,
    FirstHalf,
    HalfTime,
    SecondHalf,
    FullTime,
}
pub struct Playback {
    full_whistle_seen: bool,
    pub timeline: Timeline,
    pub phase: Phase,
    pub cursor: usize,
    pub events: Vec<Event>,
}
impl Playback {
    pub fn new(timeline: Timeline) -> Self {
        Self {
            timeline,
            full_whistle_seen: false,
            phase: Phase::Ready,
            cursor: 0,
            events: Vec::new(),
        }
    }
    pub fn markers(&self) -> &[[u16; 2]] {
        self.timeline
            .marker_rows
            .get(self.cursor)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
    pub fn view(&self) -> &View {
        if self.cursor == 0 {
            &self.timeline.initial
        } else {
            &self.timeline.frames[self.cursor - 1].view
        }
    }
    pub fn start_or_continue(&mut self) -> Result<(), String> {
        match self.phase {
            Phase::Ready => {
                self.events = self.timeline.setup_events.clone();
                self.phase = Phase::FirstHalf;
            }
            Phase::HalfTime => self.phase = Phase::SecondHalf,
            _ => return Err("no pending start/half-time decision".into()),
        }
        Ok(())
    }
    pub fn step(&mut self, count: usize) -> Result<(), String> {
        if !matches!(self.phase, Phase::FirstHalf | Phase::SecondHalf) {
            return Err("match is paused".into());
        }
        for _ in 0..count {
            let f = self
                .timeline
                .frames
                .get(self.cursor)
                .ok_or("timeline ended without full-time whistle")?;
            self.events.extend(f.events.clone());
            self.cursor += 1;
            if f.events.iter().any(|e| e.kind == "full-time") {
                self.full_whistle_seen = true;
            }
            if self.cursor == self.timeline.frames.len() {
                if !self.full_whistle_seen {
                    return Err("native playback ended without final whistle".into());
                }
                self.phase = Phase::FullTime;
                break;
            }
            if f.events.iter().any(|e| e.kind == "half-time") {
                self.phase = Phase::HalfTime;
                break;
            }
        }
        Ok(())
    }
    pub fn restart(&mut self) {
        self.cursor = 0;
        self.events.clear();
        self.full_whistle_seen = false;
        self.phase = Phase::Ready;
    }
}

#[cfg(test)]
mod marker_tests {
    use super::*;
    #[test]
    fn marker_projection_reads_stored_selector_and_computed_attribute() {
        let source = serde_json::json!({"scenes":[{"index":1,"byte7":20,"word4":0x0941}]});
        assert_eq!(marker_row(&source).unwrap(), vec![[20, 9]]);
        assert!(marker_row(&serde_json::json!({"scenes":[{"byte7":256,"word4":1}]})).is_err());
        assert!(marker_row(&serde_json::json!({"scenes":[{"byte7":1,"word4":65536}]})).is_err());
    }
    #[test]
    fn numbered_labels_agree_with_existing_original_native_rule() {
        let source = include_str!("../../../engine/tests/fixtures/scene_codes/cases.jsonl");
        let original: Value = source
            .lines()
            .map(|s| serde_json::from_str::<Value>(s).unwrap())
            .find(|v| v["stream"] == "parents")
            .unwrap();
        let mut input: onenil_engine::scene_codes::State =
            serde_json::from_value(original["input"].clone()).unwrap();
        input.mode = 4;
        for selector in [0, 1, 2, 3, 11, 12, 13, 22, 255] {
            input.scenes[2].byte7 = selector;
            let result = onenil_engine::scene_codes::refresh(&input).unwrap();
            assert!(result.draws.is_empty());
            let code = result.output.scenes[2].byte4;
            let expected = if code == 1 { 'G' } else { char::from(code) };
            assert_eq!(
                numbered_marker(u16::from(selector)),
                expected,
                "stored selector{selector}"
            );
        }
    }
}
