//! [confirmed:archaeology T100/T102/T119/T123; native T118/T119] The desktop's
//! league quick match: the T119 admission, the native quick session driven to
//! the real halftime/full-time boundaries, and the witnessed week handoff.
//! [designed] Presentation facts only; the engine owns the match.
use onenil_campaign::Campaign;
use onenil_engine::quick_match;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Scorer {
    pub name: String,
    pub minute: u16,
    pub side: u8,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Board {
    /// Club ids in the admission's club order (`legacy.goal.clubs`).
    pub clubs: [u16; 2],
    /// Club names in the admission's club order (`legacy.goal.clubs`).
    pub club_names: [String; 2],
    /// Goals in the same club order.
    pub score: [u8; 2],
    /// The commentary panel's lines, oldest first.
    pub lines: Vec<String>,
    /// Scorers with minutes, as the board lists them.
    pub scorers: Vec<Scorer>,
    pub attendance: u32,
    pub stage: &'static str,
}

pub struct Match {
    session: onenil_engine::setup::Session,
    clubs: [u16; 2],
    club_names: [String; 2],
    player_clubs: BTreeMap<u16, u16>,
    rosters: [[u16; 20]; 2],
    names: BTreeMap<u16, String>,
    attendance: u32,
    goals: Vec<Scorer>,
    lines: Vec<String>,
    substitutions: Vec<Substitution>,
}

#[derive(Clone, Debug)]
struct Substitution {
    minute: u16,
    off: u16,
    on: u16,
}

impl Match {
    pub fn start(campaign: &Campaign) -> Result<Self, String> {
        let input = crate::campaign_match::admission(campaign)?;
        let request = serde_json::to_string(&input).map_err(|error| error.to_string())?;
        let setup = &input.preparation.setup.legacy;
        let clubs = [setup.goal.clubs[0], setup.goal.clubs[1]];
        let player_clubs = setup
            .players
            .iter()
            .map(|player| (player.id, u16::from(player.p8[0])))
            .collect();
        let names_meta = crate::campaign_match::league_metadata()?;
        let mut names = BTreeMap::new();
        for player in &campaign.players {
            names.insert(player.id, player.name.clone());
        }
        for (id, name) in names_meta["players"]
            .as_object()
            .ok_or("missing league player names")?
        {
            names.insert(
                id.parse::<u16>().map_err(|_| "invalid league player id")?,
                name.as_str()
                    .ok_or("invalid league player name")?
                    .to_owned(),
            );
        }
        let club_names = [
            names_meta["clubs"][clubs[0].to_string()]
                .as_str()
                .ok_or("missing league home club name")?
                .to_owned(),
            names_meta["clubs"][clubs[1].to_string()]
                .as_str()
                .ok_or("missing league away club name")?
                .to_owned(),
        ];
        let rosters = setup.goal.rosters;
        let session = quick_match::session(quick_match::Profile::League, &request)?;
        Ok(Self {
            session,
            clubs,
            club_names,
            player_clubs,
            rosters,
            names,
            attendance: input.preparation.setup.legacy_income_total,
            goals: vec![],
            lines: vec![],
            substitutions: vec![],
        })
    }

    pub fn score(&self) -> [u8; 2] {
        let snapshot = self.session.match_state.snapshot();
        let side = snapshot["score_side"]
            .as_array()
            .map(|values| {
                values
                    .iter()
                    .map(|value| value.as_u64().unwrap_or(0) as u8)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        [
            side.first().copied().unwrap_or(0),
            side.get(1).copied().unwrap_or(0),
        ]
    }

    /// [confirmed:T49/T102] The quick result's own halftime stop.
    pub fn to_half_time(&mut self) -> Result<Board, String> {
        loop {
            match self.session.match_state.phase() {
                "half-time" => break,
                "full-time" => return Err("the first half already reached full time".into()),
                _ => {}
            }
            self.step()?;
        }
        Ok(self.board("HALF TIME"))
    }

    pub fn from_half_time(&mut self) -> Result<Board, String> {
        if self.session.match_state.phase() != "half-time" {
            return Err("the match is not at its halftime boundary".into());
        }
        self.session.match_state.continue_without_changes()?;
        loop {
            match self.session.match_state.phase() {
                "full-time" => break,
                "half-time" => return Err("the second half returned to halftime".into()),
                _ => {}
            }
            self.step()?;
        }
        Ok(self.board("FULL TIME"))
    }

    pub fn substitutions(&self) -> usize {
        self.substitutions.len()
    }

    fn step(&mut self) -> Result<(), String> {
        let before = self.scene_players();
        let out = self.session.match_state.step()?;
        let minute = out["input"]["clock"]["elapsed"].as_u64().unwrap_or(0) as u16;
        let after = self.scene_players();
        let mut changed = vec![];
        for (scene, on) in &after {
            if let Some(off) = before.get(scene) {
                if off != on {
                    changed.push(Substitution {
                        minute,
                        off: *off,
                        on: *on,
                    });
                }
            }
        }
        // [confirmed:T123] The dispatcher's replacement-log order is the
        // original's substitution order; the log's actor slot is the out
        // player's roster slot and its secondary slot the in player's.
        let mut ordered = vec![];
        for event in out["events"].as_array().into_iter().flatten() {
            if event["kind"] != "replacement-log" {
                continue;
            }
            let args = event["arguments"].as_array().cloned().unwrap_or_default();
            if args.len() < 4 {
                continue;
            }
            let secondary = args[1].as_u64().unwrap_or(0) as usize;
            let actor = args[2].as_u64().unwrap_or(0) as usize;
            if actor == 0 || secondary == 0 || actor > 20 || secondary > 20 {
                continue;
            }
            let index = changed.iter().position(|sub| {
                self.rosters
                    .iter()
                    .any(|roster| roster[actor - 1] == sub.off && roster[secondary - 1] == sub.on)
            });
            if let Some(index) = index {
                ordered.push(changed.remove(index));
            }
        }
        self.substitutions.extend(ordered);
        self.substitutions.extend(changed);
        for event in out["events"].as_array().cloned().unwrap_or_default() {
            if event["kind"] != "goal" {
                continue;
            }
            let minute = event["event"]["minute"].as_u64().unwrap_or(0) as u16;
            let side = event["event"]["side_flag"].as_u64().unwrap_or(0) as u8;
            let player = self.session.match_state.snapshot()["scorer_logs"][usize::from(side)]
                .as_array()
                .and_then(|entries| {
                    entries
                        .iter()
                        .find(|entry| entry["minute_byte"].as_u64() == Some(u64::from(minute)))
                })
                .and_then(|entry| entry["player"].as_u64())
                .unwrap_or(0) as u16;
            let club = self.player_clubs.get(&player).copied().unwrap_or(0);
            let name = self
                .names
                .get(&player)
                .cloned()
                .unwrap_or_else(|| format!("player {player}"));
            self.lines.push(format!("**** GOAL **** by {name}"));
            let side_index = self.clubs.iter().position(|id| *id == club).unwrap_or(0) as u8;
            self.goals.push(Scorer {
                name,
                minute,
                side: side_index,
            });
        }
        Ok(())
    }

    fn scene_players(&self) -> BTreeMap<u8, u16> {
        let snapshot = self.session.match_state.snapshot();
        let mut players = BTreeMap::new();
        for row in snapshot["players"].as_array().into_iter().flatten() {
            let scene = row["scene_slot"].as_u64().unwrap_or(0) as u8;
            let player = row["player"].as_u64().unwrap_or(0) as u16;
            players.insert(scene, player);
        }
        players
    }

    fn board(&self, stage: &'static str) -> Board {
        let mut lines = self.lines.clone();
        let mut substitutions = self.substitutions.iter().collect::<Vec<_>>();
        substitutions.sort_by_key(|substitution| substitution.minute);
        for substitution in substitutions {
            // [confirmed:T49/T102] The board names the player who came on and
            // the one who went off; substitutions may belong to either club.
            let on = self
                .names
                .get(&substitution.on)
                .cloned()
                .unwrap_or_else(|| format!("player {}", substitution.on));
            let off = self
                .names
                .get(&substitution.off)
                .cloned()
                .unwrap_or_else(|| format!("player {}", substitution.off));
            lines.push(format!("{on} on for {off}"));
        }
        lines.push(format!("---- {stage} ----"));
        Board {
            clubs: self.clubs,
            club_names: self.club_names.clone(),
            score: self.score(),
            lines,
            scorers: self.goals.clone(),
            attendance: self.attendance,
            stage,
        }
    }

    pub fn complete(
        &self,
        campaign: &mut Campaign,
        ticket: &onenil_campaign::MatchTicket,
    ) -> Result<(), String> {
        let score = self.score();
        let own = self.clubs[0] == campaign.club.id;
        let goals = if own {
            [u16::from(score[0]), u16::from(score[1])]
        } else {
            [u16::from(score[1]), u16::from(score[0])]
        };
        let seed = onenil_campaign::league_campaign::LeagueSeed::pinned()?;
        let following = campaign.next_fixture.clone();
        campaign.record_league_week(
            ticket,
            goals,
            "0AB9:EBE2",
            seed.cash.total_in,
            seed.cash.total_out,
            following.competition_code,
            following.opponent,
            following.flags,
            &following.opponent_name,
        )
    }
}
