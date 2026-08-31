use rand::rngs::StdRng;
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FutsalRules {
    pub half_seconds: u32,
    pub half_time_seconds: u32,
    pub total_seconds: u32,
    pub extra_time_seconds: u32,
    pub fouls_for_double: u8,
    pub timeouts_per_half: u8,
    pub kick_in_seconds: u8,
}

impl Default for FutsalRules {
    fn default() -> Self {
        Self {
            half_seconds: 20 * 60,
            half_time_seconds: 10 * 60,
            total_seconds: 40 * 60,
            extra_time_seconds: 5 * 60,
            fouls_for_double: 6,
            timeouts_per_half: 1,
            kick_in_seconds: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Role {
    POR,
    CIE,
    ALA,
    PIV,
    UNI,
}

impl Role {
    pub fn from_str(s: &str) -> Self {
        match s {
            "POR" => Role::POR,
            "CIE" => Role::CIE,
            "ALA" => Role::ALA,
            "PIV" => Role::PIV,
            _ => Role::UNI,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerAttrs {
    pub passing: f32,
    pub finishing: f32,
    pub dribbling: f32,
    pub tackling: f32,
    pub vision: f32,
    pub anticipation: f32,
    pub positioning: f32,
    pub stamina: f32,
    pub acceleration: f32,
    pub pace: f32,
    pub composure: f32,
    pub technique: f32,
    pub reflexes: f32,
}

impl PlayerAttrs {
    pub fn average(ca: i64, role: Role) -> Self {
        let base = (ca as f32 / 2.0).clamp(0.0, 100.0);
        let mut rng = StdRng::from_entropy();
        let jitter = |rng: &mut StdRng| rng.gen_range(-6.0..6.0);
        let mut mk = |bonus: f32| (base + bonus + jitter(&mut rng)).clamp(0.0, 100.0);
        match role {
            Role::POR => Self {
                passing: mk(-5.0), finishing: mk(-15.0), dribbling: mk(-10.0),
                tackling: mk(-10.0), vision: mk(0.0), anticipation: mk(5.0),
                positioning: mk(10.0), stamina: mk(0.0), acceleration: mk(0.0),
                pace: mk(0.0), composure: mk(5.0), technique: mk(0.0), reflexes: mk(20.0),
            },
            Role::CIE => Self {
                passing: mk(5.0), finishing: mk(-7.5), dribbling: mk(0.0),
                tackling: mk(15.0), vision: mk(5.0), anticipation: mk(10.0),
                positioning: mk(15.0), stamina: mk(5.0), acceleration: mk(2.5),
                pace: mk(2.5), composure: mk(2.5), technique: mk(2.5), reflexes: mk(-20.0),
            },
            Role::ALA => Self {
                passing: mk(5.0), finishing: mk(2.5), dribbling: mk(12.5),
                tackling: mk(-5.0), vision: mk(5.0), anticipation: mk(2.5),
                positioning: mk(0.0), stamina: mk(5.0), acceleration: mk(10.0),
                pace: mk(10.0), composure: mk(2.5), technique: mk(7.5), reflexes: mk(-20.0),
            },
            Role::PIV => Self {
                passing: mk(0.0), finishing: mk(15.0), dribbling: mk(2.5),
                tackling: mk(-10.0), vision: mk(0.0), anticipation: mk(2.5),
                positioning: mk(2.5), stamina: mk(2.5), acceleration: mk(0.0),
                pace: mk(0.0), composure: mk(7.5), technique: mk(7.5), reflexes: mk(-20.0),
            },
            Role::UNI => Self {
                passing: mk(5.0), finishing: mk(2.5), dribbling: mk(5.0),
                tackling: mk(2.5), vision: mk(5.0), anticipation: mk(5.0),
                positioning: mk(5.0), stamina: mk(5.0), acceleration: mk(5.0),
                pace: mk(5.0), composure: mk(2.5), technique: mk(5.0), reflexes: mk(-10.0),
            },
        }
    }
    pub fn from_ints(passing: i64, finishing: i64, dribbling: i64, tackling: i64, vision: i64, anticipation: i64, positioning: i64, stamina: i64, acceleration: i64, pace: i64, composure: i64, technique: i64, reflexes: i64) -> Self {
        Self {
            passing: passing as f32, finishing: finishing as f32, dribbling: dribbling as f32,
            tackling: tackling as f32, vision: vision as f32, anticipation: anticipation as f32,
            positioning: positioning as f32, stamina: stamina as f32, acceleration: acceleration as f32,
            pace: pace as f32, composure: composure as f32, technique: technique as f32, reflexes: reflexes as f32,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EnginePlayer {
    pub id: u32,
    pub team_id: u32,
    pub shirt: u8,
    pub role: Role,
    pub attrs: PlayerAttrs,
    pub x: f32,
    pub y: f32,
    pub stamina_now: f32,
    pub on_pitch: bool,
    pub is_gk: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchEvent {
    pub minute: u32,
    pub second: u32,
    pub kind: String,
    pub team_id: u32,
    pub player_id: Option<u32>,
    pub assist_player_id: Option<u32>,
    pub description: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerSnapshot {
    pub id: u32,
    pub team_id: u32,
    pub shirt: u8,
    pub x: f32,
    pub y: f32,
    pub stamina: f32,
    pub role: String,
    pub on_pitch: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchSnapshot {
    pub state: String,
    pub half: u8,
    pub time_seconds: u32,
    pub score: [u8; 2],
    pub went_to_extra_time: bool,
    pub went_to_penalties: bool,
    pub penalty_score: [u8; 2],
    pub fouls: [u8; 2],
    pub shots: [u32; 2],
    pub yellow_cards: [u8; 2],
    pub red_cards: [u8; 2],
    pub powerplay: [bool; 2],
    pub timeouts_used: [u8; 2],
    pub possession: [u8; 2],
    pub players: Vec<PlayerSnapshot>,
    pub ball: (f32, f32),
    pub ball_holder: Option<u32>,
    pub events: Vec<MatchEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MatchState {
    PreMatch,
    FirstHalf,
    HalfTime,
    SecondHalf,
    Finished,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct EngineAutomation {
    pub trigger_type: u8,
    pub threshold: f32,
    pub tactics: EngineTactics,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct EngineTactics {
    pub formation: u8,
    pub tempo: f32,
    pub pressing: f32,
    pub defensive_line: f32,
    pub width: f32,
}

impl Default for EngineTactics {
    fn default() -> Self {
        Self { formation: 0, tempo: 50.0, pressing: 50.0, defensive_line: 50.0, width: 50.0 }
    }
}

pub struct MatchEngine {
    pub teams: [(u32, String, String); 2],
    pub players: Vec<EnginePlayer>,
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_holder: Option<u32>,
    pub last_passer: Option<u32>,
    pub possession: [u32; 2],
    pub rules: FutsalRules,
    pub time: u32,
    pub half: u8,
    pub state: MatchState,
    pub score: [u8; 2],
    pub went_to_extra_time: bool,
    pub went_to_penalties: bool,
    pub penalty_score: [u8; 2],
    pub fouls: [u8; 2],
    pub shots: [u32; 2],
    pub shots_on: [u32; 2],
    pub yellow_cards: [u8; 2],
    pub red_cards: [u8; 2],
    pub events: Vec<MatchEvent>,
    pub rng: StdRng,
    pub powerplay: [bool; 2],
    pub bench: [Vec<u32>; 2],
    pub tactics: [EngineTactics; 2],
    pub allow_powerplay: [bool; 2],
    pub automation: [Option<EngineAutomation>; 2],
    pub automation_applied: [bool; 2],
    reactive_last_time: [u32; 2],
    pub timeouts_used: [u8; 2],
    pub timeout_until: u32,
    on_pitch_ids: [Vec<u32>; 2],
}

impl MatchEngine {
    pub fn new(team_names: [(u32, String, String); 2], rosters: [Vec<(u32, u8, Role, PlayerAttrs)>; 2]) -> Self {
        let mut players = Vec::new();
        let mut bench: [Vec<u32>; 2] = [Vec::new(), Vec::new()];
        let mut on_pitch: [Vec<u32>; 2] = [Vec::new(), Vec::new()];

        for (ti, roster) in rosters.iter().enumerate() {
            for (idx, (pid, shirt, role, attrs)) in roster.iter().enumerate() {
                let is_gk = *role == Role::POR;
                let ep = EnginePlayer {
                    id: *pid,
                    team_id: ti as u32,
                    shirt: *shirt,
                    role: *role,
                    attrs: attrs.clone(),
                    x: 0.0,
                    y: 0.0,
                    stamina_now: 100.0,
                    on_pitch: idx < 5,
                    is_gk,
                };
                if idx < 5 {
                    on_pitch[ti].push(*pid);
                } else {
                    bench[ti].push(*pid);
                }
                players.push(ep);
            }
        }

        let mut eng = Self {
            teams: team_names,
            players,
            ball_x: 20.0,
            ball_y: 10.0,
            ball_holder: None,
            last_passer: None,
            possession: [0, 0],
            rules: FutsalRules::default(),
            time: 0,
            half: 1,
            state: MatchState::PreMatch,
            score: [0, 0],
            went_to_extra_time: false,
            went_to_penalties: false,
            penalty_score: [0, 0],
            fouls: [0, 0],
            shots: [0, 0],
            shots_on: [0, 0],
            yellow_cards: [0, 0],
            red_cards: [0, 0],
            events: Vec::new(),
            rng: StdRng::from_entropy(),
            powerplay: [false, false],
            bench,
            tactics: [EngineTactics::default(), EngineTactics::default()],
            allow_powerplay: [true, true],
            automation: [None, None],
            automation_applied: [false, false],
            reactive_last_time: [0, 0],
            timeouts_used: [0, 0],
            timeout_until: 0,
            on_pitch_ids: on_pitch,
        };
        eng.reset_positions();
        eng.ball_holder = eng.on_pitch_ids[0].first().copied();
        eng
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.rng = StdRng::seed_from_u64(seed);
        self
    }

    pub fn set_tactics(&mut self, team: usize, t: EngineTactics) {
        if team < 2 {
            self.tactics[team] = t;
            if self.state == MatchState::PreMatch {
                self.reset_positions();
            }
        }
    }

    pub fn set_allow_powerplay(&mut self, team: usize, enabled: bool) {
        if team < 2 { self.allow_powerplay[team] = enabled; }
    }

    pub fn set_automation(&mut self, team: usize, automation: Option<EngineAutomation>) {
        if team < 2 { self.automation[team] = automation; }
    }

    fn apply_automation_if_needed(&mut self) {
        for team in 0..2 {
            if self.automation_applied[team] { continue; }
            let Some(auto) = self.automation[team] else { continue; };
            let other = 1 - team;
            let late = self.time as f32 >= self.rules.total_seconds as f32 * 0.75;
            let losing = self.score[team] < self.score[other];
            let margin = (self.score[other] as f32 - self.score[team] as f32) * 20.0;
            let triggered = match auto.trigger_type {
                1 => late && losing && margin >= auto.threshold,
                2 => late && !losing,
                _ => late && losing,
            };
            if triggered {
                self.tactics[team] = auto.tactics;
                self.automation_applied[team] = true;
                self.reset_positions();
                self.events.push(MatchEvent { minute:self.time/60, second:self.time%60,kind: "tactical_automation".into(), team_id:team as u32, player_id:None, assist_player_id:None, description:"Automatismo táctico activado".into(), x:20.0, y:10.0 });
            }
        }
    }

    /// Toma decisiones de banquillo para un equipo no controlado por el usuario.
    /// Se evalúa por intervalos para evitar cambios nerviosos y conserva la táctica
    /// manual/configurada cuando no existe una situación clara.
    pub fn apply_reactive_ai(&mut self, team: usize) {
        if team > 1 || self.state == MatchState::Finished || self.time < 300 || self.time.saturating_sub(self.reactive_last_time[team]) < 120 { return; }
        let other = 1 - team;
        let losing = self.score[team] < self.score[other];
        let winning = self.score[team] > self.score[other];
        let late = self.time >= self.rules.total_seconds * 2 / 3;
        let average_stamina = {
            let values: Vec<f32> = self.players.iter().filter(|p| p.team_id == team as u32 && p.on_pitch).map(|p| p.stamina_now).collect();
            if values.is_empty() { 100.0 } else { values.iter().sum::<f32>() / values.len() as f32 }
        };
        let overloaded = self.fouls[team] >= self.rules.fouls_for_double.saturating_sub(1);
        let desired = if late && losing {
            EngineTactics { formation: 3, tempo: 88.0, pressing: 82.0, defensive_line: 72.0, width: 68.0 }
        } else if late && winning {
            EngineTactics { formation: 2, tempo: 34.0, pressing: 42.0, defensive_line: 35.0, width: 45.0 }
        } else if overloaded {
            EngineTactics { formation: self.tactics[team].formation, tempo: self.tactics[team].tempo.min(55.0), pressing: self.tactics[team].pressing.min(38.0), defensive_line: self.tactics[team].defensive_line.min(48.0), width: self.tactics[team].width }
        } else if average_stamina < 62.0 {
            EngineTactics { formation: self.tactics[team].formation, tempo: self.tactics[team].tempo.min(48.0), pressing: self.tactics[team].pressing.min(50.0), defensive_line: self.tactics[team].defensive_line, width: self.tactics[team].width }
        } else { return; };
        let current = self.tactics[team];
        let changed = current.formation != desired.formation || (current.tempo - desired.tempo).abs() > 8.0 || (current.pressing - desired.pressing).abs() > 8.0 || (current.defensive_line - desired.defensive_line).abs() > 8.0;
        self.reactive_last_time[team] = self.time;
        if !changed { return; }
        self.tactics[team] = desired;
        self.reset_positions();
        self.events.push(MatchEvent { minute: self.time / 60, second: self.time % 60, kind: "reactive_tactical_change".into(), team_id: team as u32, player_id: None, assist_player_id: None, description: if late && losing { "IA: arriesga con presión y ataque".into() } else if late && winning { "IA: protege la ventaja".into() } else if overloaded { "IA: modera la presión por faltas".into() } else { "IA: reduce el ritmo por fatiga".into() }, x: 20.0, y: 10.0 });
    }

    pub fn update_live_tactics(&mut self, team: usize, tactics: EngineTactics) -> Result<(), String> {
        if team > 1 { return Err("Equipo inválido".into()); }
        self.tactics[team] = tactics;
        self.events.push(MatchEvent { minute:self.time/60, second:self.time%60, kind:"tactical_change".into(), team_id:team as u32, player_id:None, assist_player_id:None, description:"Ajustes tácticos modificados".into(), x:20.0, y:10.0 });
        Ok(())
    }

    pub fn manual_substitution(&mut self, team: usize, out_id: u32, in_id: u32) -> Result<(), String> {
        if team > 1 { return Err("Equipo inválido".into()); }
        if !self.on_pitch_ids[team].contains(&out_id) { return Err("El jugador que sale no está en pista".into()); }
        let bench_pos = self.bench[team].iter().position(|&id| id == in_id).ok_or("El jugador que entra no está en el banquillo")?;
        self.bench[team].remove(bench_pos);
        self.bench[team].push(out_id);
        let slot = self.on_pitch_ids[team].iter().position(|&id| id == out_id).unwrap();
        self.on_pitch_ids[team][slot] = in_id;
        for p in &mut self.players {
            if p.id == out_id { p.on_pitch = false; }
            if p.id == in_id { p.on_pitch = true; p.stamina_now = 95.0; let (x,y) = tactical_target(p.role, p.team_id, false, self.tactics[team]); p.x=x; p.y=y; }
        }
        self.events.push(MatchEvent { minute:self.time/60, second:self.time%60, kind:"manual_substitution".into(), team_id:team as u32, player_id:Some(in_id), assist_player_id:None, description:format!("Cambio manual: entra {} por {}", in_id, out_id), x:self.ball_x, y:self.ball_y });
        Ok(())
    }

    pub fn call_timeout(&mut self, team: usize) -> Result<(), String> {
        if team > 1 { return Err("Equipo inválido".into()); }
        if self.timeouts_used[team] >= self.rules.timeouts_per_half { return Err("No quedan tiempos muertos en esta parte".into()); }
        self.timeouts_used[team] += 1;
        self.timeout_until = self.time + 60;
        self.events.push(MatchEvent { minute:self.time/60, second:self.time%60, kind:"timeout".into(), team_id:team as u32, player_id:None, assist_player_id:None, description:"Tiempo muerto solicitado".into(), x:20.0, y:10.0 });
        Ok(())
    }

    fn reset_positions(&mut self) {
        for p in &mut self.players {
            if !p.on_pitch { continue; }
            let t = self.tactics[p.team_id as usize];
            let (x, y) = tactical_target(p.role, p.team_id, false, t);
            p.x = x;
            p.y = y;
        }
    }

    pub fn start(&mut self) {
        self.state = MatchState::FirstHalf;
        self.time = 0;
        self.half = 1;
        self.events.push(MatchEvent {
            minute: 0, second: 0, kind: "kickoff".into(), team_id: 0,
            player_id: self.ball_holder, assist_player_id: None, description: "Inicio del partido".into(), x: 20.0, y: 10.0,
        });
    }

    fn holder_team(&self) -> Option<u32> {
        if let Some(pid) = self.ball_holder {
            self.players.iter().find(|p| p.id == pid).map(|p| p.team_id)
        } else { None }
    }

    pub fn tick(&mut self) -> Vec<MatchEvent> {
        if self.state == MatchState::Finished || self.state == MatchState::PreMatch || self.state == MatchState::HalfTime {
            return Vec::new();
        }
        if self.timeout_until > self.time { self.time += 1; return Vec::new(); }

        let mut new_events = Vec::new();
        self.time += 1;

        if self.time == self.rules.half_seconds && self.half == 1 {
            self.state = MatchState::HalfTime;
            self.fouls = [0, 0];
            self.powerplay = [false, false];
            self.timeouts_used = [0, 0];
            new_events.push(MatchEvent {
                minute: 20, second: 0, kind: "halftime".into(), team_id: 0, player_id: None, assist_player_id: None,
                description: "Descanso".into(), x: 20.0, y: 10.0,
            });
            self.events.extend(new_events.clone());
            return new_events;
        }
        if self.state == MatchState::HalfTime {
            self.state = MatchState::SecondHalf;
            self.half = 2;
            self.time = self.rules.half_seconds + 1;
            self.reset_positions();
            self.ball_holder = self.on_pitch_ids[1].first().copied();
        }

        if self.time >= self.rules.total_seconds {
            self.state = MatchState::Finished;
            new_events.push(MatchEvent {
                minute: 40, second: 0, kind: "finished".into(), team_id: 0, player_id: None, assist_player_id: None,
                description: format!("Final {}-{}", self.score[0], self.score[1]),
                x: 20.0, y: 10.0,
            });
            self.events.extend(new_events.clone());
            return new_events;
        }

        self.apply_automation_if_needed();

        let losing_powerplay = self.time > self.rules.total_seconds - 180;
        if losing_powerplay {
            for t in 0..2 {
                let other = 1 - t;
                if self.score[t] < self.score[other] && self.allow_powerplay[t] {
                    self.powerplay[t] = true;
                }
            }
        }

        self.update_movement(1.0);
        self.update_stamina(1.0);

        if self.time % 8 == 0 {
            self.maybe_substitute();
        }

        if self.time % 2 == 0 {
            let period = 90.0 - (self.tactics[0].tempo + self.tactics[1].tempo) / 2.0 * 0.4;
            let period = period.max(20.0) as u32;
            if self.time % period == 0 {
                if let Some(ev) = self.resolve_action() {
                    new_events.push(ev.clone());
                    self.events.push(ev);
                }
            }
        }

        new_events
    }

    fn update_movement(&mut self, dt: f32) {
        let possessing = self.holder_team();
        for p in &mut self.players {
            if !p.on_pitch { continue; }
            let attacking = Some(p.team_id) == possessing;
            let t = self.tactics[p.team_id as usize];
            let (tx, ty) = tactical_target(p.role, p.team_id, attacking, t);
            let dx = tx - p.x;
            let dy = ty - p.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > 0.3 {
                let speed = (p.attrs.pace * 0.04 + p.attrs.acceleration * 0.03).clamp(0.4, 2.2);
                let fatigue = (p.stamina_now / 100.0).clamp(0.5, 1.0);
                p.x += (dx / dist) * speed * fatigue * dt;
                p.y += (dy / dist) * speed * fatigue * dt;
                p.x = p.x.clamp(0.5, 39.5);
                p.y = p.y.clamp(0.5, 19.5);
            }
        }
        if let Some(pid) = self.ball_holder {
            if let Some(pl) = self.players.iter().find(|p| p.id == pid) {
                self.ball_x = pl.x;
                self.ball_y = pl.y;
            }
        }
    }

    fn update_stamina(&mut self, dt: f32) {
        for p in &mut self.players {
            if !p.on_pitch { continue; }
            let drain = if Some(p.id) == self.ball_holder { 0.10 } else { 0.06 };
            p.stamina_now -= drain * dt * (1.5 - p.attrs.stamina / 100.0);
            if Some(p.id) != self.ball_holder {
                p.stamina_now += 0.015 * dt;
            }
            p.stamina_now = p.stamina_now.clamp(0.0, 100.0);
        }
    }

    fn maybe_substitute(&mut self) {
        let mut to_swap: Vec<(u32, u32)> = Vec::new();
        for t in 0..2 {
            for &pid in &self.on_pitch_ids[t].clone() {
                if let Some(pl) = self.players.iter().find(|p| p.id == pid) {
                    if should_substitute(pl.stamina_now, self.time) && !self.bench[t].is_empty() {
                        let bench_pid = self.bench[t][0];
                        to_swap.push((pid, bench_pid));
                        break;
                    }
                }
            }
        }
        for (out_id, in_id) in to_swap {
            let team = self.players.iter().find(|p| p.id == out_id).map(|p| p.team_id).unwrap_or(0) as usize;
            if let Some(pos) = self.bench[team].iter().position(|&x| x == in_id) {
                self.bench[team].remove(pos);
            }
            self.bench[team].push(out_id);
            if let Some(idx) = self.on_pitch_ids[team].iter().position(|&x| x == out_id) {
                self.on_pitch_ids[team][idx] = in_id;
            }
            for p in &mut self.players {
                if p.id == out_id { p.on_pitch = false; }
                if p.id == in_id { p.on_pitch = true; p.stamina_now = 95.0; let (tx, ty) = tactical_target(p.role, p.team_id, false, self.tactics[team]); p.x = tx; p.y = ty; }
            }
            self.events.push(MatchEvent {
                minute: self.time / 60, second: self.time % 60, kind: "substitution".into(),
                team_id: team as u32, player_id: Some(in_id), assist_player_id: None,
                description: format!("Cambio: entra {} por {}", in_id, out_id),
                x: self.ball_x, y: self.ball_y,
            });
        }
    }

    fn resolve_action(&mut self) -> Option<MatchEvent> {
        let holder = self.ball_holder?;
        let holder_idx = self.players.iter().position(|p| p.id == holder)?;
        let holder_team = self.players[holder_idx].team_id;
        let holder_attrs = self.players[holder_idx].attrs.clone();
        let holder_x = self.players[holder_idx].x;
        let holder_y = self.players[holder_idx].y;

        let opp_team = 1 - holder_team;
        let is_powerplay = self.powerplay[holder_team as usize];

        let goal_x = if holder_team == 0 { 40.0 } else { 0.0 };
        let dist_to_goal = ((holder_x - goal_x).abs().powi(2) + (holder_y - 10.0).powi(2)).sqrt();
        let angle = ((10.0 - holder_y).abs() / dist_to_goal.max(1.0)).asin().to_degrees().abs();

        let do_shoot = dist_to_goal < 12.0 && self.rng.gen_bool( ((0.30 + (holder_attrs.finishing * 0.005)).min(0.95)) as f64 );
        if do_shoot {
            self.shots[holder_team as usize] += 1;
            let prob = calculate_goal_probability(&holder_attrs, dist_to_goal, angle, is_powerplay);
            let noise: f32 = self.rng.gen_range(0.85..1.15);
            let effective = (prob * noise).clamp(0.0, 0.95);
            let roll: f32 = self.rng.gen();
            let gk = self.players.iter().filter(|p| p.team_id == opp_team && p.role == Role::POR && p.on_pitch).next();
            let gk_mod = gk.map(|g| 1.0 - g.attrs.reflexes / 100.0).unwrap_or(1.0);
            let final_prob = effective * gk_mod;

            if roll < final_prob * 0.55 {
                self.score[holder_team as usize] += 1;
                self.shots_on[holder_team as usize] += 1;
                self.ball_holder = self.on_pitch_ids[opp_team as usize].first().copied();
                self.ball_x = 20.0; self.ball_y = 10.0;
                return Some(MatchEvent {
                    minute: self.time / 60, second: self.time % 60, kind: "goal".into(),
                    team_id: holder_team, player_id: Some(holder), assist_player_id: self.last_passer.filter(|p| *p != holder),
                    description: match self.last_passer { Some(p) if p != holder => format!("GOOOOL de {}! Asistencia de {}", holder, p), _ => format!("GOOOOL de {}!", holder) },
                    x: holder_x, y: holder_y,
                });
            } else if roll < final_prob + 0.25 {
                self.shots_on[holder_team as usize] += 1;
                self.ball_holder = self.players.iter().find(|p| p.team_id == opp_team && p.role == Role::POR && p.on_pitch).map(|p| p.id);
                return Some(MatchEvent {
                    minute: self.time / 60, second: self.time % 60, kind: "save".into(),
                    team_id: opp_team, player_id: gk.map(|g| g.id), assist_player_id: None,
                    description: "Parada del portero".into(), x: holder_x, y: holder_y,
                });
            } else {
                let recov = if self.rng.gen_bool(0.5) { opp_team } else { holder_team };
                self.ball_holder = self.on_pitch_ids[recov as usize].choose(&mut self.rng).copied();
                return Some(MatchEvent {
                    minute: self.time / 60, second: self.time % 60, kind: "shot_off".into(),
                    team_id: holder_team, player_id: Some(holder), assist_player_id: None,
                    description: "Tiro fuera".into(), x: holder_x, y: holder_y,
                });
            }
        }

        let teammates: Vec<u32> = self.on_pitch_ids[holder_team as usize].iter().copied().filter(|&id| id != holder).collect();
        if teammates.is_empty() { return None; }

        let target = *teammates.choose(&mut self.rng).unwrap();
        let defender = self.on_pitch_ids[opp_team as usize].choose(&mut self.rng).copied();
        let def_attrs = defender.and_then(|did| self.players.iter().find(|p| p.id == did).map(|p| p.attrs.clone()));
        // presión del equipo defensor: más presión -> mayor intercepción
        let press_mod = 0.9 + (self.tactics[opp_team as usize].pressing / 100.0) * 0.2;

        let (result, _prob) = resolve_duel(&holder_attrs, &def_attrs.unwrap_or_else(|| holder_attrs.clone()), "pass", &mut self.rng, press_mod);

        match result {
            DuelResult::Success => {
                self.last_passer = Some(holder);
                self.ball_holder = Some(target);
                if let Some(tp) = self.players.iter().find(|p| p.id == target) {
                    self.ball_x = tp.x; self.ball_y = tp.y;
                }
                self.possession[holder_team as usize] += 1;
                None
            }
            DuelResult::Foul => {
                self.fouls[opp_team as usize] += 1;
                let is_sixth = self.fouls[opp_team as usize] >= self.rules.fouls_for_double;
                if is_sixth {
                    let dp_prob = 0.72;
                    let roll: f32 = self.rng.gen();
                    if roll < dp_prob * (holder_attrs.composure / 100.0) {
                        self.score[holder_team as usize] += 1;
                        self.events.push(MatchEvent {
                            minute: self.time / 60, second: self.time % 60, kind: "double_penalty_goal".into(),
                            team_id: holder_team, player_id: Some(holder), assist_player_id: None,
                            description: "Gol de doble penalti!".into(), x: 30.0, y: 10.0,
                        });
                    }
                    self.fouls[opp_team as usize] = 0;
                    self.ball_holder = self.on_pitch_ids[opp_team as usize].first().copied();
                    return Some(MatchEvent {
                        minute: self.time / 60, second: self.time % 60, kind: "double_penalty".into(),
                        team_id: holder_team, player_id: Some(holder), assist_player_id: None,
                        description: "Doble penalti por 6ª falta".into(), x: 30.0, y: 10.0,
                    });
                }
                self.ball_holder = Some(holder);
                Some(MatchEvent {
                    minute: self.time / 60, second: self.time % 60, kind: "foul".into(),
                    team_id: opp_team, player_id: defender, assist_player_id: None,
                    description: format!("Falta de {} ({}ª del equipo)", defender.unwrap_or(0), self.fouls[opp_team as usize]),
                    x: holder_x, y: holder_y,
                })
            }
            DuelResult::Failure => {
                if let Some(did) = defender {
                    self.ball_holder = Some(did);
                    self.possession[opp_team as usize] += 1;
                    return Some(MatchEvent {
                        minute: self.time / 60, second: self.time % 60, kind: "interception".into(),
                        team_id: opp_team, player_id: Some(did), assist_player_id: None,
                        description: "Intercepción".into(), x: holder_x, y: holder_y,
                    });
                }
                None
            }
        }
    }

    pub fn simulate_full(&mut self) -> MatchSnapshot {
        self.simulate_until_finished();
        self.snapshot()
    }

    pub fn simulate_full_reactive(&mut self) -> MatchSnapshot {
        self.start();
        while self.state != MatchState::Finished {
            if self.state == MatchState::HalfTime {
                self.state = MatchState::SecondHalf;
                self.half = 2;
                self.time = self.rules.half_seconds + 1;
                self.reset_positions();
                continue;
            }
            self.tick();
            self.apply_reactive_ai(0);
            self.apply_reactive_ai(1);
        }
        self.snapshot()
    }

    pub fn simulate_full_knockout(&mut self) -> MatchSnapshot {
        self.simulate_full_knockout_if(true)
    }

    pub fn simulate_full_knockout_if(&mut self, resolve_tie: bool) -> MatchSnapshot {
        self.simulate_until_finished();
        if resolve_tie {
            self.resolve_knockout_tie_forced();
            self.events.push(MatchEvent {
                minute: 45, second: 0, kind: "finished".into(), team_id: 0, player_id: None,
                assist_player_id: None, description: format!("Final {}-{}", self.score[0], self.score[1]),
                x: 20.0, y: 10.0,
            });
        }
        self.snapshot()
    }

    fn simulate_until_finished(&mut self) {
        self.start();
        while self.state != MatchState::Finished {
            if self.state == MatchState::HalfTime {
                self.state = MatchState::SecondHalf;
                self.half = 2;
                self.time = self.rules.half_seconds + 1;
                self.reset_positions();
                self.ball_holder = self.on_pitch_ids[1].first().copied();
                continue;
            }
            self.tick();
        }
    }

    pub fn player_stats(&self) -> Vec<(u32, u32, bool, u32, u32, u32, u32, u32, u32, u32, u32, f64)> {
        let mut stats: std::collections::HashMap<u32, (u32, u32, bool, u32, u32, u32, u32, u32, u32, u32, u32, f64)> = std::collections::HashMap::new();
        for player in &self.players {
            stats.insert(player.id, (player.id, player.team_id, false, 0, 0, 0, 0, 0, 0, 0, 0, 6.0));
        }
        for player in &self.players {
            if player.on_pitch {
                if let Some(s) = stats.get_mut(&player.id) { s.2 = true; }
            }
        }
        for event in &self.events {
            if let Some(pid) = event.player_id {
                if let Some(s) = stats.get_mut(&pid) {
                    match event.kind.as_str() {
                        "goal" | "double_penalty_goal" => s.4 += 1,
                        "shot_off" => s.5 += 1,
                        "save" => { s.5 += 1; s.6 += 1; }
                        "foul" => s.7 += 1,
                        "yellow_card" => s.8 += 1,
                        "red_card" => s.9 += 1,
                        _ => {}
                    }
                }
            }
        }
        let minutes = (self.time / 60).min(40);
        for s in stats.values_mut() { s.3 = if s.2 { minutes } else { 0 }; }
        stats.into_values().collect()
    }

    pub fn resolve_knockout_tie(&mut self) {
        if self.score[0] != self.score[1] { return; }
        self.resolve_knockout_tie_forced();
    }

    pub fn resolve_knockout_tie_forced(&mut self) {
        if self.went_to_extra_time { return; }
        self.went_to_extra_time = true;
        self.events.push(MatchEvent { minute: 40, second: 0, kind: "extra_time".into(), team_id: 0, player_id: None, assist_player_id: None, description: "Comienza la prórroga".into(), x: 20.0, y: 10.0 });
        for _ in 0..self.rules.extra_time_seconds {
            if self.rng.gen_bool(0.002) { self.score[(self.rng.gen::<bool>()) as usize] += 1; }
        }
        if self.score[0] == self.score[1] {
            self.went_to_penalties = true;
            for i in 0..5 {
                for team in 0..2 {
                    if self.rng.gen_bool(if i < 3 { 0.72 } else { 0.68 }) { self.penalty_score[team] += 1; }
                }
            }
            while self.penalty_score[0] == self.penalty_score[1] {
                if self.rng.gen_bool(0.72) { self.penalty_score[0] += 1; }
                if self.rng.gen_bool(0.72) { self.penalty_score[1] += 1; }
            }
            self.events.push(MatchEvent { minute: 45, second: 0, kind: "penalties".into(), team_id: 0, player_id: None, assist_player_id: None, description: format!("Penaltis {}-{}", self.penalty_score[0], self.penalty_score[1]), x: 20.0, y: 10.0 });
        }
    }

    pub fn snapshot(&self) -> MatchSnapshot {
        let players = self.players.iter().map(|p| PlayerSnapshot {
            id: p.id, team_id: p.team_id, shirt: p.shirt, x: p.x, y: p.y,
            stamina: p.stamina_now, role: format!("{:?}", p.role), on_pitch: p.on_pitch,
        }).collect();
        let total_poss = (self.possession[0] + self.possession[1]).max(1) as f32;
        let poss_pct = [
            ((self.possession[0] as f32 / total_poss) * 100.0) as u8,
            ((self.possession[1] as f32 / total_poss) * 100.0) as u8,
        ];
        MatchSnapshot {
            state: format!("{:?}", self.state),
            half: self.half,
            time_seconds: self.time,
            score: self.score,
            went_to_extra_time: self.went_to_extra_time,
            went_to_penalties: self.went_to_penalties,
            penalty_score: self.penalty_score,
            fouls: self.fouls,
            shots: self.shots,
            yellow_cards: self.yellow_cards,
            red_cards: self.red_cards,
            powerplay: self.powerplay,
            timeouts_used: self.timeouts_used,
            possession: poss_pct,
            players,
            ball: (self.ball_x, self.ball_y),
            ball_holder: self.ball_holder,
            events: self.events.clone(),
        }
    }
}

fn tactical_target(role: Role, team_id: u32, attacking: bool, t: EngineTactics) -> (f32, f32) {
    let left = team_id == 0;
    // profundidad (0-100): desplaza el bloque adelante/atrás
    let depth = ((t.defensive_line - 50.0) / 50.0).clamp(-1.0, 1.0);
    // amplitud (0-100): separa los laterales
    let width = ((t.width - 50.0) / 50.0).clamp(-1.0, 1.0);
    // formaciones: 0=3-1, 1=4-0, 2=2-2, 3=5-0
    let base = match role {
        Role::POR => if left { (2.5, 10.0) } else { (37.5, 10.0) },
        Role::CIE => if left { (9.0, 10.0) } else { (31.0, 10.0) },
        Role::ALA => {
            let advance = if t.formation == 1 { 2.0 } else if t.formation == 3 { 4.0 } else { 0.0 };
            let spread = width * 3.0;
            if left {
                let x = if attacking { 21.0 + advance } else { 12.0 };
                (x, 5.5 - spread)
            } else {
                let x = if attacking { 19.0 - advance } else { 28.0 };
                (x, 14.5 + spread)
            }
        }
        Role::PIV => {
            // 5-0: pívot muy adelantado; 2-2: más retrasado
            let advance = match t.formation { 3 => 5.0, 2 => -1.0, _ => 0.0 };
            let x = if left { 31.0 + advance } else { 9.0 - advance };
            (x, 10.0)
        }
        Role::UNI => {
            let advance = if t.formation == 3 { 6.0 } else { 0.0 };
            let x = if left { 16.0 + advance } else { 24.0 - advance };
            (x, 10.0)
        }
    };
    // aplicar profundidad (más arriba = atacar más) en la coordenada X según dirección
    let mut bx = base.0;
    let by = base.1;
    if left {
        bx = (bx + depth * 3.0).clamp(1.0, 39.0);
    } else {
        bx = (bx - depth * 3.0).clamp(1.0, 39.0);
    }
    (bx, by)
}

#[derive(Debug)]
enum DuelResult { Success, Failure, Foul }

fn resolve_duel(attacker: &PlayerAttrs, defender: &PlayerAttrs, action: &str, rng: &mut StdRng, def_bonus: f32) -> (DuelResult, f32) {
    let atk = match action {
        "pass" => attacker.passing * 0.5 + attacker.vision * 0.3 + attacker.technique * 0.2,
        "dribble" => attacker.dribbling * 0.5 + attacker.acceleration * 0.3 + attacker.technique * 0.2,
        _ => attacker.passing,
    };
    let def = (defender.tackling * 0.4 + defender.anticipation * 0.3 + defender.positioning * 0.3) * def_bonus;
    let noise: f32 = rng.gen_range(0.85..1.15);
    let prob = ((atk / (atk + def).max(1.0)) * noise).clamp(0.05, 0.95);
    let roll: f32 = rng.gen();
    let res = if roll < prob * 0.92 {
        DuelResult::Success
    } else if roll < prob + 0.12 {
        if rng.gen_bool(0.22) { DuelResult::Foul } else { DuelResult::Failure }
    } else {
        if rng.gen_bool(0.18) { DuelResult::Foul } else { DuelResult::Failure }
    };
    (res, prob)
}

fn calculate_goal_probability(shooter: &PlayerAttrs, distance: f32, angle: f32, is_powerplay: bool) -> f32 {
    let base = if distance < 3.0 { 0.68 } else if distance < 6.0 { 0.38 } else if distance < 10.0 { 0.18 } else { 0.05 };
    let angle_mod = (angle / 90.0).clamp(0.2, 1.0);
    let skill = (shooter.finishing / 100.0) * 0.5 + (shooter.composure / 100.0) * 0.3 + (shooter.technique / 100.0) * 0.2;
    let pp = if is_powerplay { 1.25 } else { 1.0 };
    (base * angle_mod * (0.5 + skill) * pp).clamp(0.02, 0.85)
}

fn should_substitute(stamina: f32, time: u32) -> bool {
    if stamina < 38.0 { return true; }
    if time > 300 && stamina < 58.0 { return true; }
    if time % 240 == 0 && stamina < 68.0 { return true; }
    false
}

pub async fn simulate_clubs(pool: &sqlx::SqlitePool, home_club: i64, away_club: i64) -> Result<MatchSnapshot, String> {
    simulate_clubs_with_knockout(pool, home_club, away_club, false).await
}

pub async fn simulate_clubs_with_knockout(pool: &sqlx::SqlitePool, home_club: i64, away_club: i64, knockout: bool) -> Result<MatchSnapshot, String> {
    simulate_clubs_with_context(pool, home_club, away_club, knockout, None).await
}

pub async fn simulate_clubs_with_aggregate(pool: &sqlx::SqlitePool, home_club: i64, away_club: i64, aggregate_before: (i64, i64)) -> Result<MatchSnapshot, String> {
    simulate_clubs_with_context(pool, home_club, away_club, true, Some(aggregate_before)).await
}

async fn simulate_clubs_with_context(pool: &sqlx::SqlitePool, home_club: i64, away_club: i64, knockout: bool, aggregate_before: Option<(i64, i64)>) -> Result<MatchSnapshot, String> {
    let home_row: Option<(String, String)> = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(home_club).fetch_optional(pool).await.map_err(|e| e.to_string())?;
    let away_row: Option<(String, String)> = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(away_club).fetch_optional(pool).await.map_err(|e| e.to_string())?;
    let (hn, hc) = home_row.ok_or("home club no encontrado")?;
    let (an, ac) = away_row.ok_or("away club no encontrado")?;

    async fn load_roster(pool: &sqlx::SqlitePool, club_id: i64, team_id: u32) -> Result<Vec<(u32, u8, Role, PlayerAttrs)>, String> {
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(
            "SELECT p.id, pa.passing, pa.finishing, pa.dribbling, pa.tackling, pa.vision, pa.anticipation, pa.positioning, pa.stamina, pa.acceleration, pa.pace, pa.composure, pa.technique, pa.reflexes, pp.ala_natural FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_attributes pa ON pa.player_id=p.id JOIN player_positions pp ON pp.player_id=p.id LIMIT 12"
        ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for (pid, passing, finishing, dribbling, tackling, vision, anticipation, positioning, stamina, acceleration, pace, composure, technique, reflexes, _ala) in rows {
            let role = if out.len() < 2 { Role::POR } else if out.len() < 4 { Role::CIE } else if out.len() < 8 { Role::ALA } else if out.len() < 10 { Role::PIV } else { Role::UNI };
            let attrs = PlayerAttrs::from_ints(passing, finishing, dribbling, tackling, vision, anticipation, positioning, stamina, acceleration, pace, composure, technique, reflexes);
            out.push((pid as u32, (out.len() + 1) as u8, role, attrs));
        }
        if out.len() < 10 { return Err(format!("club {club_id} solo tiene {} jugadores activos", out.len())); }
        let _ = team_id;
        Ok(out)
    }

    let r1 = load_roster(pool, home_club, 0).await?;
    let r2 = load_roster(pool, away_club, 1).await?;
    let mut eng = MatchEngine::new(
        [(0, hn, hc), (1, an, ac)],
        [r1, r2],
    );        if knockout {
        eng.start();
        while eng.state != crate::engine::MatchState::Finished {
            if eng.state == crate::engine::MatchState::HalfTime {
                eng.state = crate::engine::MatchState::SecondHalf;
                eng.half = 2;
                eng.time = eng.rules.half_seconds + 1;
                eng.reset_positions();
                continue;
            }
            eng.tick();
            eng.apply_reactive_ai(0);
            eng.apply_reactive_ai(1);
        }
        let resolve_tie = aggregate_before.map(|(prior_home, prior_away)| {
            prior_home + eng.score[1] as i64 == prior_away + eng.score[0] as i64
        }).unwrap_or(eng.score[0] == eng.score[1]);
        if resolve_tie {
            eng.resolve_knockout_tie_forced();
            eng.events.push(MatchEvent {
                minute: 45, second: 0, kind: "finished".into(), team_id: 0, player_id: None,
                assist_player_id: None, description: format!("Final {}-{}", eng.score[0], eng.score[1]),
                x: 20.0, y: 10.0,
            });
        }
        Ok(eng.snapshot())
    } else {
        Ok(eng.simulate_full_reactive())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_attrs(ca: i64, role: Role) -> PlayerAttrs { PlayerAttrs::average(ca, role) }

    #[test]
    fn duel_attacker_stronger_wins_more() {
        let strong = mk_attrs(170, Role::ALA);
        let weak = mk_attrs(70, Role::CIE);
        let mut rng = StdRng::seed_from_u64(42);
        let mut wins = 0;
        for _ in 0..200 {
            let (r, _) = resolve_duel(&strong, &weak, "pass", &mut rng, 1.0);
            if matches!(r, DuelResult::Success) { wins += 1; }
        }
        assert!(wins > 120, "fuerte debe ganar >60%, gano {wins}/200");
    }

    #[test]
    fn automation_triggers_when_losing_late() {
        let roster: Vec<(u32,u8,Role,PlayerAttrs)> = (1..=12).map(|i| (i, i as u8, Role::ALA, mk_attrs(120, Role::ALA))).collect();
        let mut eng = MatchEngine::new([(0,"A".into(),"#f00".into()),(1,"B".into(),"#00f".into())], [roster.clone(), roster]).with_seed(7);
        eng.set_automation(0, Some(EngineAutomation { trigger_type: 0, threshold: 0.0, tactics: EngineTactics { formation: 3, tempo: 90.0, pressing: 85.0, defensive_line: 80.0, width: 70.0 } }));
        eng.start();
        eng.score = [0, 2];
        eng.time = 1900;
        eng.tick();
        assert!(eng.automation_applied[0]);
        assert_eq!(eng.tactics[0].formation, 3);
        assert!(eng.events.iter().any(|e| e.kind == "tactical_automation"));
    }

    #[test]
    fn reactive_ai_changes_plan_when_trailing_late() {
        let roster: Vec<(u32,u8,Role,PlayerAttrs)> = (1..=12).map(|i| (i, i as u8, Role::ALA, mk_attrs(120, Role::ALA))).collect();
        let mut eng = MatchEngine::new([(0,"A".into(),"#f00".into()),(1,"B".into(),"#00f".into())], [roster.clone(), roster]).with_seed(9);
        eng.start();
        eng.score = [0, 2];
        eng.time = 1700;
        eng.apply_reactive_ai(0);
        assert_eq!(eng.tactics[0].formation, 3);
        assert!(eng.tactics[0].pressing > 75.0);
        assert!(eng.events.iter().any(|e| e.kind == "reactive_tactical_change" && e.team_id == 0));
        let before = eng.events.len();
        eng.apply_reactive_ai(0);
        assert_eq!(eng.events.len(), before, "la IA no debe cambiar de plan continuamente");
    }

    #[test]
    fn goal_prob_distance() {
        let shooter = mk_attrs(150, Role::PIV);
        let p_close = calculate_goal_probability(&shooter, 2.0, 45.0, false);
        let p_far = calculate_goal_probability(&shooter, 14.0, 45.0, false);
        assert!(p_close > p_far, "cerca ({p_close}) > lejos ({p_far})");
        assert!(p_close > 0.3);
        assert!(p_far < 0.1);
    }

    #[test]
    fn knockout_tie_resolves_with_extra_time_or_penalties() {
        let roster: Vec<(u32,u8,Role,PlayerAttrs)> = (1..=12)
            .map(|i| (i, i as u8, if i <= 2 { Role::POR } else { Role::ALA }, mk_attrs(110, Role::ALA)))
            .collect();
        let mut eng = MatchEngine::new(
            [(0, "A".into(), "#f00".into()), (1, "B".into(), "#00f".into())],
            [roster.clone(), roster],
        ).with_seed(7);
        let snap = eng.simulate_full_knockout_if(true);
        assert!(snap.went_to_extra_time);
        assert!(snap.score[0] != snap.score[1] || snap.went_to_penalties);
        if snap.went_to_penalties {
            assert_ne!(snap.penalty_score[0], snap.penalty_score[1]);
            assert!(snap.events.iter().any(|event| event.kind == "penalties"));
        }
    }

    #[test]
    fn full_match_produces_valid_score() {
        let t1: Vec<(u32,u8,Role,PlayerAttrs)> = (1..=12).map(|i| {
            let role = if i<=2 { Role::POR } else if i<=4 { Role::CIE } else if i<=8 { Role::ALA } else if i<=10 { Role::PIV } else { Role::UNI };
            (i, i as u8, role, mk_attrs(120, role))
        }).collect();
        let t2: Vec<(u32,u8,Role,PlayerAttrs)> = (101..=112).map(|i| {
            let role = if i<=102 { Role::POR } else if i<=104 { Role::CIE } else if i<=108 { Role::ALA } else if i<=110 { Role::PIV } else { Role::UNI };
            (i, (i-100) as u8, role, mk_attrs(115, role))
        }).collect();
        let mut eng = MatchEngine::new(
            [(0, "A".into(), "#f00".into()), (1, "B".into(), "#00f".into())],
            [t1, t2],
        ).with_seed(12345);
        let snap = eng.simulate_full();
        assert_eq!(snap.state, "Finished");
        assert_eq!(snap.time_seconds, 2400);
        assert!(snap.score[0] + snap.score[1] <= 15, "goles totales razonables: {:?}", snap.score);
        assert!(snap.events.iter().any(|e| e.kind=="goal" || e.kind=="double_penalty_goal") || snap.score==[0,0]);
    }

    #[test]
    fn stamina_drains_over_match() {
        let t1: Vec<(u32,u8,Role,PlayerAttrs)> = (1..=12).map(|i| {
            let role = if i<=2 { Role::POR } else { Role::ALA };
            (i, i as u8, role, mk_attrs(130, role))
        }).collect();
        let t2: Vec<(u32,u8,Role,PlayerAttrs)> = (101..=112).map(|i| (i, (i-100) as u8, Role::ALA, mk_attrs(130, Role::ALA))).collect();
        let mut eng = MatchEngine::new(
            [(0,"A".into(),"#f00".into()), (1,"B".into(),"#00f".into())],
            [t1,t2],
        ).with_seed(99);
        eng.start();
        for _ in 0..600 { eng.tick(); }
        let low = eng.players.iter().filter(|p| p.on_pitch && p.stamina_now < 85.0).count();
        assert!(low > 0, "algún jugador debe haber perdido stamina tras 10 min");
    }

    #[tokio::test]
    async fn simulate_from_db() {
        let pool = crate::db::init_memory_pool().await.unwrap();
        crate::world::seed_world(&pool).await.unwrap();
        let (hid,): (i64,) = sqlx::query_as("SELECT id FROM clubs WHERE short_name='BAR'").fetch_one(&pool).await.unwrap();
        let (aid,): (i64,) = sqlx::query_as("SELECT id FROM clubs WHERE short_name='INT'").fetch_one(&pool).await.unwrap();
        let snap = crate::engine::simulate_clubs(&pool, hid, aid).await.unwrap();
        assert_eq!(snap.state, "Finished");
        assert!(snap.score[0] + snap.score[1] < 20);
        assert!(snap.events.len() > 2);
    }
}
