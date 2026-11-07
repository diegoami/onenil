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
        false,
        ["Blackburn R".into(), "Sheffield Wed".into()],
    )
}
pub fn compute_campaign(campaign: &onenil_campaign::Campaign) -> Result<Timeline, String> {
    let input = crate::campaign_match::admission(campaign)?;
    let metadata = crate::campaign_match::metadata()?;
    let ratings = &input.preparation.setup.legacy.ratings;
    let club_names = [ratings[0].club, ratings[1].club].map(|id| {
        metadata["clubs"][id.to_string()]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| "missing campaign club name".to_owned())
    });
    let [first, second] = club_names;
    compute_input(
        serde_json::to_value(input).map_err(|error| error.to_string())?,
        &metadata["players"],
        true,
        [first?, second?],
    )
}
fn compute_input(
    input: Value,
    names: &Value,
    fresh: bool,
    club_names: [String; 2],
) -> Result<Timeline, String> {
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
    let mut session = if fresh {
        onenil_engine::watched_compiled::prepared::Session::new_fresh(input)?
    } else {
        onenil_engine::watched_compiled::prepared::Session::new(input)?
    };
    let initial = session.snapshot()?;
    let observation = session.observation();
    let setup_draws = a(&observation["draws"])?;
    let seed = n(&setup_draws.first().ok_or("missing setup draw")?["seed_before"])?;
    if !fresh && (seed != 64 || !a(&initial["scenes"])?.iter().any(|s| s["word14"] == 2590)) {
        return Err("native selection differs".into());
    }
    let output = session.finish().map_err(|e| {
        serde_json::to_string(&e).unwrap_or_else(|_| "native preparation failed".into())
    })?;
    if output["return"] != "0AB9:EBE2" || !a(&output["draws"])?.starts_with(setup_draws) {
        return Err("native return/draw prefix differs".into());
    }
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
        frames.push(Frame {
            view: view(&t["after"], g, clock, names, &player_clubs, &clubs)?,
            events: actions,
        });
    }
    if !events.is_empty()
        || frames.last().unwrap().view.score
            != [n(&output["live_goals"][0])?, n(&output["live_goals"][1])?]
    {
        return Err("incomplete native timeline".into());
    }
    Ok(Timeline {
        club_names,
        initial: view(
            &initial,
            &initial["scores"],
            n(&initial["elapsed"])?,
            names,
            &player_clubs,
            &clubs,
        )?,
        frames,
        setup_events,
        native_return: "0AB9:EBE2".into(),
        draws: a(&output["draws"])?.len(),
        clock_rows: clocks.len(),
        seed,
        clubs,
    })
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
