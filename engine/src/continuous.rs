//! [confirmed] DD02/7B7B:F12-F23;T36 at archaeology1415668 supplies one initial read set.
//! [designed] Debug admission starts after setup;one image,then input-free steps to halftime.
use crate::{
    ai_selection, event_effects, extra_event, goal_effects, layout, neutral_input, placement,
    quick_reset, ratings, rearrangement, step_tail, substitution, terminal, Draw, Engine, Input,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct History {
    pub base_offset: u16,
    pub base_segment: u16,
    pub club: u16,
    pub selector: u8,
    pub value: u8,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    pub contest: Input,
    pub ratings: [ratings::Input; 2],
    pub players: Vec<rearrangement::Player>,
    pub scenes: Vec<terminal::Scene>,
    pub goal: goal_effects::State,
    pub extra: extra_event::Input,
    pub event: event_effects::State,
    pub reset: quick_reset::State,
    pub substitution: substitution::State,
    pub neutral: neutral_input::State,
    pub clock: step_tail::ClockState,
    pub history: History,
    pub config: [u8; 3],
    pub selectors: Vec<u8>,
    pub scorer_slots: [u8; 30],
    pub minute_limits: [u8; 2],
    pub replacement_word: u16,
}
pub struct Session {
    pub(crate) e: Engine,
    pub(crate) a: Admission,
    pub(crate) ended: bool,
}
fn word(e: &Engine, a: usize) -> u16 {
    u16::from_le_bytes([e.image[a], e.image[a + 1]])
}
fn put(e: &mut Engine, a: usize, v: u16) {
    e.image[a..a + 2].copy_from_slice(&v.to_le_bytes())
}
fn scene(i: u8) -> usize {
    0xCCEF + 25 * usize::from(i)
}
fn club(i: u16) -> usize {
    layout::CLUB + 62 * usize::from(i - 1)
}
fn row(e: &Engine, i: u16) -> goal_effects::QueueRow {
    let a = 0xA52A + 6 * usize::from(i);
    goal_effects::QueueRow {
        index: i,
        code: e.image[a],
        first_player: word(e, a + 1),
        second_player: word(e, a + 3),
        side: e.image[a + 5],
    }
}
fn queue(e: &Engine, n: usize) -> goal_effects::Queue {
    let head = word(e, 0x380);
    let mut i = head;
    let mut rows = vec![];
    for _ in 0..n {
        rows.push(row(e, i));
        i = if i == 1700 { 1 } else { i + 1 };
    }
    goal_effects::Queue { head, rows }
}
fn same<A: Serialize, B: Serialize>(a: &A, b: &B) -> bool {
    serde_json::to_value(a).unwrap() == serde_json::to_value(b).unwrap()
}
impl Session {
    pub fn new(a: Admission) -> Result<Self, String> {
        Self::new_with_initial(a, None)
    }
    pub(crate) fn new_with_initial(
        a: Admission,
        initial: Option<&crate::role_scores::Cells>,
    ) -> Result<Self, String> {
        for (side, r) in a.ratings.iter().enumerate() {
            ratings::validate(r)?;
            if usize::from(r.side) != side {
                return Err("admission rating side order differs".into());
            }
        }
        extra_event::validate(&a.extra)?;
        event_effects::validate_storage(&a.event)?;
        quick_reset::validate(&a.reset)?;
        step_tail::validate_clock(&a.clock)?;
        if a.goal.clubs.iter().any(|id| !(1..=176).contains(id))
            || a.goal
                .players
                .iter()
                .any(|p| p.id > 3920 || p.p8_position > 13)
            || a.goal.queue.head > 1700
            || a.goal.queue.rows.iter().any(|r| r.index > 1700)
            || a.goal.log_counts.iter().any(|n| *n > 11)
        {
            return Err("invalid goal admission domain".into());
        }
        if a.selectors.len() != 255
            || a.config[0] != a.history.selector
            || a.config[2] != a.neutral.speed
            || a.contest.k != 8
            || a.reset.kind != 45
            || a.substitution.replacement.is_some()
        {
            return Err("unsupported/incomplete first-half admission".into());
        }
        let ids = a.players.iter().map(|p| p.id).collect::<BTreeSet<_>>();
        if ids.len() != a.players.len()
            || ids.iter().any(|i| *i > 3920)
            || a.scenes.len() != 22
            || a.scenes
                .iter()
                .enumerate()
                .any(|(i, s)| usize::from(s.index) != i + 1 || !ids.contains(&s.word14))
        {
            return Err("incomplete/ambiguous admission players/scenes".into());
        }
        let goal_ids = a.goal.players.iter().map(|p| p.id).collect::<BTreeSet<_>>();
        let required_goal_ids = a
            .goal
            .rosters
            .iter()
            .flatten()
            .copied()
            .chain(a.goal.scenes.iter().map(|s| s.player))
            .collect::<BTreeSet<_>>();
        let head = a.goal.queue.head;
        let next = if head == 1700 { 1 } else { head + 1 };
        let after = if next == 1700 { 1 } else { next + 1 };
        if a.goal.scenes.len() != 22
            || a.goal
                .scenes
                .iter()
                .enumerate()
                .any(|(i, s)| usize::from(s.index) != i + 1 || !(1..=22).contains(&s.slot))
            || goal_ids.len() != a.goal.players.len()
            || goal_ids != required_goal_ids
            || !ids.is_superset(&goal_ids)
            || a.goal
                .queue
                .rows
                .iter()
                .map(|r| r.index)
                .collect::<Vec<_>>()
                != [head, next, after]
            || a.history.club != a.goal.clubs[0]
            || a.scorer_slots.iter().any(|s| !(1..=11).contains(s))
        {
            return Err("incomplete/contradictory goal,history or scorer admission".into());
        }
        let mut e = if let Some(initial) = initial {
            Engine::new_tracked(&a.contest, initial)?
        } else {
            Engine::new(&a.contest)
        };
        // [confirmed static:F07/F14,seg_0AB9_14.asm8700-870E@218490d] Setup fills all1700 numeric rows FF.
        // [designed] Initial-state bootstrap only;does not claim native setup/RNG execution.
        e.image[0xA530..0xA530 + 0x27D8].fill(255);
        for p in &a.players {
            let i = layout::P8 + 8 * usize::from(p.id);
            e.image[i..i + 8].copy_from_slice(&p.p8);
            let i = layout::P15 + 15 * usize::from(p.id);
            e.image[i..i + 15].copy_from_slice(&p.p15);
        }
        for s in &a.scenes {
            e.image[scene(s.index)..scene(s.index) + 25].copy_from_slice(&s.bytes());
        }
        goal_effects::load(&mut e, &a.goal);
        e.image[0xD09F] = a.goal.global_actor;
        extra_event::load(&mut e, &a.extra, false)?;
        event_effects::load(&mut e, &a.event, false)?;
        quick_reset::load(&mut e, &a.reset, false);
        let ai = ai_selection::State {
            context: a.substitution.context.clone(),
            replacement: None,
            mode: 0,
            side: 0,
            scene: a.substitution.context.side_offsets[0] + 1,
        };
        for (address, value) in ai_selection::cells(&ai)? {
            e.image[address] = value;
        }
        for r in &a.ratings {
            ratings::load_tables(&mut e, &r.tables);
            for p in &r.players {
                ratings::load_player(&mut e, p);
            }
            ratings::load_bonus(
                &mut e,
                &ratings::BonusInput {
                    club: r.club,
                    staff_ids: r.staff_ids,
                    staff: r.staff.clone(),
                },
            );
            e.image[club(r.club) + 0x37] = r.club_37;
        }
        step_tail::load_clock(&mut e, &a.clock);
        for (addr, v) in [
            (0xD074, a.neutral.quick_mode),
            (0xD075, a.neutral.mirror_mode),
            (0xD1CC, a.neutral.auto_switch),
            (0xD0B8, a.neutral.active),
            (0xD0B9, a.neutral.d0b9),
            (0xA2EC, a.neutral.speaker_gate),
        ] {
            e.image[addr] = v;
        }
        put(&mut e, 0xD0AE, a.neutral.remaining);
        put(&mut e, 0xD7CE, a.history.base_offset);
        put(&mut e, 0xD7D0, (layout::HISTORY >> 4) as u16);
        put(&mut e, 0xD7D2, 0);
        put(&mut e, 0xD7D4, (layout::CONFIG >> 4) as u16);
        e.image[layout::CONFIG..layout::CONFIG + 3].copy_from_slice(&a.config);
        for (i, v) in a.selectors.iter().enumerate() {
            e.image[0xCF72 + i] = *v;
        }
        e.image[0xCE0..0xCFE].copy_from_slice(&a.scorer_slots);
        e.image[0xD00..0xD02].copy_from_slice(&a.minute_limits);
        put(&mut e, 0xD32, a.replacement_word);
        let address = Self::history_address(&e);
        e.image[address] = a.history.value;
        let s = Self { e, a, ended: false };
        s.validate_admission()?;
        Ok(s)
    }
    fn history_address(e: &Engine) -> usize {
        layout::HISTORY
            + usize::from(
                word(e, 0xD7CE)
                    .wrapping_add(word(e, 0xD6E6).wrapping_mul(0x3A))
                    .wrapping_add(u16::from(e.image[layout::CONFIG]))
                    .wrapping_sub(0x3B),
            )
    }
    pub(crate) fn current_ids(&self, cached: bool) -> BTreeSet<u16> {
        let e = &self.e;
        let mut ids = BTreeSet::new();
        for side in 0..2 {
            let base = club(word(e, 0xD6E4 + 2 * side));
            for i in 0..20 {
                ids.insert(word(e, base + 2 * i));
            }
            if cached {
                ids.insert(word(e, 0xCF5E + 10 * side));
            }
        }
        for i in 1..=22 {
            ids.insert(word(e, scene(i) + 14));
        }
        ids
    }
    pub(crate) fn goal(&self) -> goal_effects::State {
        let mut t = self.a.goal.clone();
        t.queue = queue(&self.e, 3);
        let mut out = goal_effects::snapshot(&self.e, &t);
        let ids = self.current_ids(false);
        out.players.retain(|p| ids.contains(&p.id));
        out
    }
    // [derived] Roster/scene references may change after an actual substitution.
    pub(crate) fn extra(&self) -> extra_event::State {
        let e = &self.e;
        let mut state = self.a.extra.state.clone();
        let ids = (1..=22)
            .map(|i| word(e, scene(i) + 14))
            .collect::<BTreeSet<_>>();
        state.players = ids
            .into_iter()
            .map(|id| {
                let a = layout::P8 + 8 * usize::from(id);
                let b = layout::P15 + 15 * usize::from(id);
                let skills = [6u16, 14u16].map(|mode| {
                    let style = e.image
                        [(0x957D + 22 * usize::from(e.image[a + 7]) + usize::from(mode)) & 65535];
                    extra_event::Skill {
                        mode,
                        style_cell: style,
                        level_cell: e.image[(0x96B1
                            + 22 * usize::from(e.image[a + 5])
                            + usize::from(style))
                            & 65535],
                    }
                });
                extra_event::Player {
                    id,
                    p8_0: e.image[a],
                    p8_1: e.image[a + 1],
                    p8_3: e.image[a + 3],
                    p8_4: e.image[a + 4],
                    p8_5: e.image[a + 5],
                    p8_7: e.image[a + 7],
                    p15_6: e.image[b + 6],
                    p15_8: e.image[b + 8],
                    p15_10: e.image[b + 10],
                    p15_11: e.image[b + 11],
                    skills,
                }
            })
            .collect();
        extra_event::project(e, &state)
    }
    pub(crate) fn event(&self) -> event_effects::State {
        let mut t = self.a.event.clone();
        t.handoff = self.extra();
        t.queue = queue(&self.e, 4);
        let head = word(&self.e, 0xD072);
        t.selectors.head = head;
        t.selectors.cells = [
            event_effects::Cell {
                index: head,
                value: 0,
            },
            event_effects::Cell {
                index: if head == 255 { 1 } else { head + 1 },
                value: 0,
            },
        ];
        let mut out = event_effects::project(&self.e, &t);
        let ids = self.current_ids(false);
        out.players.retain(|p| ids.contains(&p.id));
        out
    }
    pub(crate) fn reset(&self) -> quick_reset::State {
        let mut out = quick_reset::project(&self.e, &self.a.reset);
        let ids = self.current_ids(false);
        out.players.retain(|p| ids.contains(&p.id));
        out
    }
    pub(crate) fn selection(&self) -> ai_selection::Context {
        let mut out = ai_selection::snapshot(&self.e, &self.a.substitution.context);
        let ids = self.current_ids(false);
        out.players.retain(|p| ids.contains(&p.id));
        out
    }
    pub(crate) fn clock(&self) -> step_tail::ClockState {
        let mut t = self.a.clock.clone();
        t.commentary.queue = queue(&self.e, 3);
        step_tail::clock_snapshot(&self.e, &t)
    }
    pub(crate) fn rating(&self, side: usize) -> ratings::Input {
        let e = &self.e;
        let mut r = self.a.ratings[side].clone();
        r.seed = e.word(crate::SEED);
        r.ratings = e.ratings_snapshot();
        r.club = word(e, if side == 0 { 0xD6E6 } else { 0xD6E4 });
        r.flag = if side == 0 { e.image[0xD29] } else { 0 };
        r.club_37 = e.image[club(r.club) + 0x37];
        for (i, id) in r.roster.iter_mut().enumerate() {
            *id = word(e, club(r.club) + 2 * i);
        }
        r.players = r
            .roster
            .iter()
            .copied()
            .filter(|id| *id != 0)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|id| {
                let a = layout::P8 + 8 * usize::from(id);
                let b = layout::P15 + 15 * usize::from(id);
                ratings::Player {
                    id,
                    p8_4: e.image[a + 4],
                    p8_6: e.image[a + 6],
                    p8_7: e.image[a + 7],
                    p15_6: e.image[b + 6],
                    p15_8: e.image[b + 8],
                    p15_11: e.image[b + 11],
                }
            })
            .collect();
        r.staff_ids = [word(e, club(r.club) + 40), word(e, club(r.club) + 42)];
        r.staff = r
            .staff_ids
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|id| ratings::Staff {
                id,
                byte2: e.image[layout::STAFF + usize::from(id.wrapping_shl(2).wrapping_add(2))],
            })
            .collect();
        r.tables
            .halving_group
            .copy_from_slice(&e.image[0x12BA..0x12BD]);
        for i in 0..2 {
            r.tables.halving_flags[i]
                .copy_from_slice(&e.image[0x129E + 14 * i..0x129E + 14 * (i + 1)]);
        }
        r.tables
            .position_class
            .copy_from_slice(&e.image[0x12BE..0x12CC]);
        r.tables
            .drop_divisors
            .copy_from_slice(&e.image[0x1286..0x1288]);
        r.tables
            .home_divisors
            .copy_from_slice(&e.image[0x12CC..0x12CE]);
        r
    }
    pub(crate) fn validate_admission(&self) -> Result<(), String> {
        let e = &self.e;
        fn difference(a: &Value, b: &Value) -> String {
            if let (Some(a), Some(b)) = (a.as_object(), b.as_object()) {
                if a.len() != b.len() {
                    return "/keys".into();
                }
                for (k, v) in a {
                    if b.get(k) != Some(v) {
                        return format!(
                            "/{}{}",
                            k,
                            b.get(k).map(|w| difference(v, w)).unwrap_or_default()
                        );
                    }
                }
            } else if let (Some(a), Some(b)) = (a.as_array(), b.as_array()) {
                if a.len() != b.len() {
                    return "/length".into();
                }
                for (i, (v, w)) in a.iter().zip(b).enumerate() {
                    if v != w {
                        return format!("/{}{}", i, difference(v, w));
                    }
                }
            }
            String::new()
        }
        for (name, actual, expected) in [
            ("goal", json!(self.goal()), json!(self.a.goal)),
            ("event", json!(self.event()), json!(self.a.event)),
            (
                "extra",
                json!(extra_event::project(e, &self.a.extra.state)),
                json!(self.a.extra.state),
            ),
            (
                "reset",
                json!(quick_reset::project(e, &self.a.reset)),
                json!(self.a.reset),
            ),
            ("clock", json!(self.clock()), json!(self.a.clock)),
            (
                "selection",
                json!(ai_selection::snapshot(e, &self.a.substitution.context)),
                json!(self.a.substitution.context),
            ),
        ] {
            if actual != expected {
                return Err(format!(
                    "contradictory admission image aliases: {}{}",
                    name,
                    difference(&actual, &expected)
                ));
            }
        }
        for i in 0..2 {
            if !same(&self.rating(i), &self.a.ratings[i]) {
                return Err("contradictory rating admission aliases".into());
            }
        }
        for p in &self.a.players {
            let i = layout::P8 + 8 * usize::from(p.id);
            let j = layout::P15 + 15 * usize::from(p.id);
            if e.image[i..i + 8] != p.p8 || e.image[j..j + 15] != p.p15 {
                return Err("contradictory full-player admission".into());
            }
        }
        for s in &self.a.scenes {
            if terminal::Scene::read(e, s.index) != *s {
                return Err("contradictory full-scene admission".into());
            }
        }
        Ok(())
    }
    pub fn snapshot(&self) -> Value {
        let e = &self.e;
        let g = self.goal();
        let c = self.clock();
        let players=(1..=22).map(|i|{let a=scene(i);let id=word(e,a+14);let p=layout::P15+15*usize::from(id);json!({"scene_slot":i,"player":id,"scene_word0":word(e,a),"scene_word8":word(e,a+8),"scene_byte19":e.image[a+19],"p15_word2":word(e,p+2),"p15_byte5":e.image[p+5],"p15_sbyte6":e.image[p+6] as i8,"p15_sbyte8":e.image[p+8] as i8,"p15_byte11":e.image[p+11]})}).collect::<Vec<_>>();
        let logs = (0..2)
            .map(|side| {
                g.logs[side]
                    .iter()
                    .take(usize::from(g.log_counts[side]))
                    .map(|v| json!({"player":v.player,"minute_byte":v.minute_byte}))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        json!({"clock":c.clock,"seed":e.word(crate::SEED),"score_side":[e.image[0xD0C5],e.image[0xD0C4]],"goal_bytes":[e.image[0xD086],e.image[0xD087]],"ratings":e.ratings_snapshot(),"clubs":g.clubs,"flags":{"DS:D074":e.image[0xD074],"DS:D09F":e.image[0xD09F],"DS:D0A0":e.image[0xD0A0],"DS:D0B7":e.image[0xD0B7],"DS:D1CA":e.image[0xD1CA],"DS:D6DD":e.image[0xD6DD]},"history_byte":e.image[Self::history_address(e)],"scorer_logs":logs,"players":players})
    }
    pub fn step(&mut self) -> Result<Value, String> {
        self.step_with_readset(None, None)
    }
    pub(crate) fn step_with_readset(
        &mut self,
        replacement: Option<&ai_selection::ReadSet>,
        ai_trace: Option<&mut Vec<Value>>,
    ) -> Result<Value, String> {
        if self.ended {
            return Err("first-half session has reached its actual clock boundary".into());
        }
        if self.e.image[0xD074] == 0 || self.e.image[0xD0AC] != 45 {
            return Err("unsupported watched/alternate phase in first-half session".into());
        }
        let input = self.snapshot();
        self.e.image[0xD0B8] = 1;
        self.e.put_word(0xD0B0, 0);
        let scores = [self.e.image[0xD0C5], self.e.image[0xD0C4]];
        self.e.put_word(crate::GOALS, 0);
        for side in 0..2 {
            let r = self.rating(side);
            self.e.rate_side(&r)?;
        }
        let mut draws = Vec::<Draw>::new();
        draws.extend(self.e.quick_contests(self.a.contest.k).draws);
        let g = self.goal();
        let e = &self.e;
        let p = placement::Input {
            seed: e.word(crate::SEED),
            generated: [e.image[0xD086], e.image[0xD087]],
            scores,
            elapsed: word(e, 0xD0B2),
            minute_limit: u16::from(e.image[0xD00 + usize::from(e.image[0xD0AC] == 15)]),
            side_offsets: g.side_offsets,
            actor_before: g.global_actor,
            scorer_slots: self.a.scorer_slots,
            rosters: g.rosters,
            players: self
                .a
                .players
                .iter()
                .map(|p| {
                    let a = layout::P15 + 15 * usize::from(p.id);
                    placement::Player {
                        id: p.id,
                        p15_sbyte8: e.image[a + 8] as i8,
                        p15_byte11: e.image[a + 11],
                    }
                })
                .collect(),
        };
        let placed = self.e.place_goals(&p)?;
        draws.extend(placed.draws.clone());
        let mut events = vec![];
        for request in &placed.callbacks {
            let g = self.goal();
            let out = self.e.apply_goal_effects(&g, request)?;
            events.push(json!({"kind":"goal","event":request.event,"commentary":out.commentary}));
        }
        let actual = step_tail::PlacementHandoff::from(&placed.output);
        let e = &self.e;
        let prefix = step_tail::PrefixInput {
            placement: actual.clone(),
            mode: e.image[0xD074],
            d6dd: e.image[0xD6DD],
            remaining: word(e, 0xD0AE),
            replacement_word: word(e, 0xD32),
            clubs: [word(e, 0xD6E6), word(e, 0xD6E4)],
            history: step_tail::History {
                base_offset: self.a.history.base_offset,
                base_segment: self.a.history.base_segment,
                selector: e.image[layout::CONFIG],
                value: e.image[Self::history_address(e)],
            },
            scenes: (1..=22)
                .map(|i| step_tail::Scene {
                    index: i,
                    player: word(e, scene(i) + 14),
                    word0: word(e, scene(i)),
                })
                .collect(),
            players: (1..=22)
                .map(|i| word(e, scene(i) + 14))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|id| {
                    let a = layout::P15 + 15 * usize::from(id);
                    step_tail::Player {
                        id,
                        byte8: e.image[a + 8],
                        byte11: e.image[a + 11],
                    }
                })
                .collect(),
        };
        self.e.finish_prefix(&prefix, &actual)?;
        if actual.goal_flag == 0 {
            if self.e.image[0xD6DD] != 0 {
                let current = self.extra();
                let out = self.e.decide_loaded(&current)?;
                draws.extend(out.draws);
                if out.dispatch.is_some() {
                    let current = self.event();
                    event_effects::validate(&current)?;
                    let out = self.e.logical_event_loaded(&current)?;
                    if !out.external_calls.is_empty() {
                        return Err("unresolved reached event-effect callee".into());
                    }
                    draws.extend(out.draws);
                    events.extend(
                        out.events
                            .into_iter()
                            .map(|e| json!({"kind":"numeric","event":e})),
                    );
                }
            }
            let sub = substitution::State {
                context: self.selection(),
                replacement: replacement.cloned(),
            };
            let out = if replacement.is_some() {
                substitution::run_with_commentary(&mut self.e, &sub)?
            } else {
                substitution::run(&mut self.e, &sub)?
            };
            if let Some(trace) = ai_trace {
                trace.extend(out.ai.clone());
            }
            draws.extend(out.draws);
            events.extend(out.presentation);
        } else {
            let current = self.reset();
            let out = self.e.reset_loaded(&current)?;
            draws.extend(out.draws);
        }
        let neutral = neutral_input::neutral_image(&mut self.e);
        events.extend(neutral.presentation);
        let output = self.snapshot();
        let current = self.clock();
        let clock = step_tail::clock_loaded(&mut self.e, &current);
        self.ended = clock.result != 0;
        Ok(
            json!({"input":input,"output":output,"clock_input":current,"clock":clock,"draws":draws,"events":events}),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn admission() -> Admission {
        serde_json::from_str(include_str!("../tests/fixtures/continuous/admission.json")).unwrap()
    }
    #[test]
    fn one_admission_two_steps_consume_current_image_and_rng() {
        let mut s = Session::new(admission()).unwrap();
        s.e.image[0x8000] = 0x7B;
        let first = s.step().unwrap();
        let second = s.step().unwrap();
        assert_eq!(first["clock"]["output"]["seed"], second["input"]["seed"]);
        assert_eq!(first["clock"]["output"]["clock"], second["input"]["clock"]);
        assert_ne!(first["input"]["seed"], second["input"]["seed"]);
        assert_eq!(s.e.image[0x8000], 0x7B);
    }
    #[test]
    fn current_player_mutation_changes_next_computation() {
        let a = admission();
        let mut ordinary = Session::new(a.clone()).unwrap();
        let mut changed = Session::new(a).unwrap();
        let id = word(&changed.e, club(word(&changed.e, 0xD6E6)));
        changed.e.image[layout::P8 + 8 * usize::from(id) + 4] ^= 127;
        let before = ordinary.step().unwrap();
        let after = changed.step().unwrap();
        assert_ne!(before["output"]["ratings"], after["output"]["ratings"]);
    }
    #[test]
    fn contradictory_player_alias_rejected() {
        let mut a = admission();
        a.players[1].p15[6] ^= 1;
        assert!(Session::new(a).is_err())
    }
    #[test]
    fn contradictory_global_alias_rejected() {
        let mut a = admission();
        a.goal.flags.d6dd ^= 1;
        assert!(Session::new(a).is_err())
    }
    #[test]
    fn missing_selector_and_duplicate_player_rejected() {
        let mut a = admission();
        a.selectors.pop();
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.players.push(a.players[0].clone());
        assert!(Session::new(a).is_err())
    }
    #[test]
    fn first_half_ends_on_actual_clock_result() {
        let mut s = Session::new(admission()).unwrap();
        let mut n = 0;
        loop {
            n += 1;
            if s.step().unwrap()["clock"]["result"] == 1 {
                break;
            }
            assert!(n < 100)
        }
        assert_eq!(n, 46);
        assert!(s.step().is_err())
    }
    #[test]
    fn invalid_admission_domains_are_diagnostics() {
        let mut a = admission();
        a.ratings[0].club = 0;
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.goal.players[0].id = u16::MAX;
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.clock.commentary.queue.head = 1701;
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.ratings[0].side = 1;
        assert!(Session::new(a).is_err());
    }
    #[test]
    fn missing_redundant_goal_fields_cannot_hide_behind_other_views() {
        let mut a = admission();
        a.goal.scenes.clear();
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.goal.queue.rows.pop();
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.goal.players.push(a.goal.players[0].clone());
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.history.club ^= 1;
        assert!(Session::new(a).is_err());
        let mut a = admission();
        a.scorer_slots[0] = 0;
        assert!(Session::new(a).is_err());
    }
}
