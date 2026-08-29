use serde::Serialize;
use tauri::State;

use crate::commands::{AppState, LiveMatchInfo};
use crate::engine::{EngineTactics, MatchEngine, MatchSnapshot, PlayerAttrs, Role};

#[derive(Serialize)]
pub struct PreMatch {
    pub home_name: String,
    pub home_color: String,
    pub away_name: String,
    pub away_color: String,
    pub tactics: TacticsRow,
    pub squad: Vec<PreMatchPlayer>,
}

#[derive(Serialize)]
pub struct TacticsRow {
    pub formation: String,
    pub tempo: i64,
    pub pressing: i64,
    pub defensive_line: i64,
    pub width: i64,
    pub powerplay_enabled: bool,
}

#[derive(Serialize)]
pub struct PreMatchPlayer {
    pub id: i64,
    pub name: String,
    pub position: String,
    pub ca: i64,
}

#[derive(Serialize)]
pub struct MatchSummaryTeam {
    pub id: i64,
    pub name: String,
    pub short_name: String,
    pub score: i64,
    pub shots: i64,
    pub shots_on_target: i64,
    pub fouls: i64,
    pub possession: i64,
}

#[derive(Serialize)]
pub struct MatchSummaryEvent {
    pub minute: i64,
    pub second: i64,
    pub kind: String,
    pub team_id: i64,
    pub player_id: Option<i64>,
    pub player_name: Option<String>,
    pub description: String,
}

#[derive(sqlx::FromRow)]
struct MatchSummaryRaw {
    match_id: i64,
    date: String,
    round: i64,
    competition: String,
    home_id: i64,
    home_name: String,
    home_short: String,
    home_score: i64,
    home_shots: i64,
    home_sot: i64,
    home_fouls: i64,
    home_pos: i64,
    away_id: i64,
    away_name: String,
    away_short: String,
    away_score: i64,
    away_shots: i64,
    away_sot: i64,
    away_fouls: i64,
    away_pos: i64,
}

#[derive(Serialize)]
pub struct MatchSummary {
    pub match_id: i64,
    pub date: String,
    pub round: i64,
    pub competition: String,
    pub went_to_extra_time: bool,
    pub went_to_penalties: bool,
    pub penalty_home_score: i64,
    pub penalty_away_score: i64,
    pub home: MatchSummaryTeam,
    pub away: MatchSummaryTeam,
    pub events: Vec<MatchSummaryEvent>,
}

async fn match_info(pool: &sqlx::SqlitePool, match_id: i64) -> Result<(i64, i64, String, String, String, String), String> {
    let (home, away): (i64, i64) = sqlx::query_as("SELECT home_club_id, away_club_id FROM matches WHERE id=?")
        .bind(match_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (hn, hc): (String, String) = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(home).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (an, ac): (String, String) = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(away).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok((home, away, hn, hc, an, ac))
}

async fn load_roster_raw(pool: &sqlx::SqlitePool, club_id: i64) -> Result<Vec<(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>, String> {
    let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(
        "SELECT p.id, pa.passing, pa.finishing, pa.dribbling, pa.tackling, pa.vision, pa.anticipation, pa.positioning, pa.stamina, pa.acceleration, pa.pace, pa.composure, pa.technique FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_attributes pa ON pa.player_id=p.id LIMIT 12"
    ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}

fn attrs_from(r: &(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)) -> PlayerAttrs {
    PlayerAttrs::from_ints(r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.9, r.10, r.11, r.12, 50)
}

async fn natural_role(pool: &sqlx::SqlitePool, pid: i64) -> Role {
    let (por, cie, ala, piv, uni): (i64, i64, i64, i64, i64) = sqlx::query_as(
        "SELECT por_natural, cie_natural, ala_natural, piv_natural, uni_natural FROM player_positions WHERE player_id=?"
    ).bind(pid).fetch_one(pool).await.unwrap_or((0, 0, 0, 0, 0));
    let best = [(por, Role::POR), (cie, Role::CIE), (ala, Role::ALA), (piv, Role::PIV), (uni, Role::UNI)]
        .into_iter().max_by_key(|(v, _)| *v).map(|(_, r)| r).unwrap_or(Role::UNI);
    best
}

fn role_for_index(idx: usize) -> Role {
    if idx < 2 { Role::POR } else if idx < 4 { Role::CIE } else if idx < 8 { Role::ALA } else if idx < 10 { Role::PIV } else { Role::UNI }
}

fn formation_code(f: &str) -> u8 {
    match f { "4-0" => 1, "2-2" => 2, "5-0" => 3, _ => 0 }
}

#[tauri::command]
pub async fn get_last_match_summary(state: State<'_, AppState>, club_id: i64) -> Result<Option<MatchSummary>, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let row = sqlx::query_as::<_, MatchSummaryRaw>(
        "SELECT m.id AS match_id, m.date, m.round, comp.name AS competition, m.home_club_id AS home_id, hc.name AS home_name, hc.short_name AS home_short, m.home_score, m.home_shots, m.home_shots_on_target AS home_sot, m.home_fouls, COALESCE(m.home_possession,0) AS home_pos, m.away_club_id AS away_id, ac.name AS away_name, ac.short_name AS away_short, m.away_score, m.away_shots, m.away_shots_on_target AS away_sot, m.away_fouls, COALESCE(m.away_possession,0) AS away_pos FROM matches m JOIN competitions comp ON comp.id=m.competition_id JOIN clubs hc ON hc.id=m.home_club_id JOIN clubs ac ON ac.id=m.away_club_id WHERE m.status='finished' AND (m.home_club_id=? OR m.away_club_id=?) ORDER BY m.date DESC, m.id DESC LIMIT 1"
    ).bind(club_id).bind(club_id).fetch_optional(&pool).await.map_err(|e| e.to_string())?;
    let Some(raw) = row else { return Ok(None); };
    let MatchSummaryRaw { match_id:id, date, round, competition, home_id, home_name, home_short, home_score, home_shots, home_sot, home_fouls, home_pos, away_id, away_name, away_short, away_score, away_shots, away_sot, away_fouls, away_pos } = raw;
    let (extra, penalties, ph, pa): (i64, i64, i64, i64) = sqlx::query_as("SELECT COALESCE(went_to_extra_time,0), COALESCE(went_to_penalties,0), COALESCE(penalty_home_score,0), COALESCE(penalty_away_score,0) FROM cup_ties WHERE match_id=?").bind(id).fetch_optional(&pool).await.map_err(|e| e.to_string())?.unwrap_or((0,0,0,0));
    let events = sqlx::query_as::<_, (i64, i64, String, i64, Option<i64>, Option<String>, String)>(
        "SELECT e.minute, e.second, e.event_type, e.club_id, e.player_id, CASE WHEN e.player_id IS NULL THEN NULL ELSE p.common_name END, COALESCE(e.description,'') FROM match_events e LEFT JOIN players p ON p.id=e.player_id WHERE e.match_id=? ORDER BY e.minute, e.second, e.id"
    ).bind(id).fetch_all(&pool).await.map_err(|e| e.to_string())?.into_iter().map(|(minute, second, kind, team_id, player_id, player_name, description)| MatchSummaryEvent { minute, second, kind, team_id, player_id, player_name, description }).collect();
    Ok(Some(MatchSummary { match_id:id, date, round, competition, went_to_extra_time: extra != 0, went_to_penalties: penalties != 0, penalty_home_score: ph, penalty_away_score: pa, home: MatchSummaryTeam { id:home_id, name:home_name, short_name:home_short, score:home_score, shots:home_shots, shots_on_target:home_sot, fouls:home_fouls, possession:home_pos }, away: MatchSummaryTeam { id:away_id, name:away_name, short_name:away_short, score:away_score, shots:away_shots, shots_on_target:away_sot, fouls:away_fouls, possession:away_pos }, events }))
}

#[tauri::command]
pub async fn get_pre_match(state: State<'_, AppState>, match_id: i64) -> Result<PreMatch, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let (home, _away, hn, hc, an, ac) = match_info(&pool, match_id).await?;

    let tactics: (String, i64, i64, i64, i64, i64) = sqlx::query_as(
        "SELECT formation, tempo, pressing, defensive_line, width, powerplay_enabled FROM tactics WHERE club_id=?"
    ).bind(home).fetch_one(&pool).await.unwrap_or(("3-1".into(), 50, 50, 50, 50, 1));
    let tac = TacticsRow { formation: tactics.0.clone(), tempo: tactics.1, pressing: tactics.2, defensive_line: tactics.3, width: tactics.4, powerplay_enabled: tactics.5 == 1 };

    let rows = sqlx::query_as::<_, (i64, String, String, i64)>(
        "SELECT p.id, p.common_name, COALESCE(pp.pos,'UNI'), ps.current_ability FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_states ps ON ps.player_id=p.id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp ON pp.player_id=p.id ORDER BY ps.current_ability DESC"
    ).bind(home).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    let squad = rows.into_iter().map(|(id, name, position, ca)| PreMatchPlayer { id, name, position, ca }).collect();

    Ok(PreMatch { home_name: hn, home_color: hc, away_name: an, away_color: ac, tactics: tac, squad })
}

#[tauri::command]
pub async fn start_live_match_tactics(
    state: State<'_, AppState>,
    match_id: i64,
    formation: String,
    tempo: i64,
    pressing: i64,
    defensive_line: i64,
    width: i64,
    powerplay_enabled: bool,
    lineup: Vec<i64>,
) -> Result<MatchSnapshot, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let (home, away, hn, hc, an, ac) = match_info(&pool, match_id).await?;

    let rows_home = load_roster_raw(&pool, home).await?;
    let rows_away = load_roster_raw(&pool, away).await?;

    // Equipo local: colocar primero el quintero elegido (con rol natural)
    let mut r1: Vec<(u32, u8, Role, PlayerAttrs)> = Vec::new();
    for pid in &lineup {
        let role = natural_role(&pool, *pid).await;
        if let Some(r) = rows_home.iter().find(|r| r.0 == *pid) {
            r1.push((r.0 as u32, (r1.len() + 1) as u8, role, attrs_from(r)));
        }
    }
    // resto de la plantilla (lesión/banquillo) en su rol natural
    for r in &rows_home {
        if !lineup.contains(&r.0) {
            let role = natural_role(&pool, r.0).await;
            r1.push((r.0 as u32, (r1.len() + 1) as u8, role, attrs_from(r)));
        }
    }

    // Equipo rival: rol por índice (descanso por el quinto inicial)
    let mut r2: Vec<(u32, u8, Role, PlayerAttrs)> = Vec::new();
    for r in &rows_away {
        let role = role_for_index(r2.len());
        r2.push((r.0 as u32, (r2.len() + 1) as u8, role, attrs_from(r)));
    }

    let mut eng = MatchEngine::new([(0, hn, hc), (1, an, ac)], [r1, r2]);
    // Tacticas del equipo local
    let t = EngineTactics {
        formation: formation_code(&formation),
        tempo: tempo as f32,
        pressing: pressing as f32,
        defensive_line: defensive_line as f32,
        width: width as f32,
    };
    eng.set_tactics(0, t);
    eng.set_allow_powerplay(0, powerplay_enabled);
    // Tacticas del rival desde BD (o default)
    let away_tac: Option<(String, i64, i64, i64, i64, i64)> = sqlx::query_as(
        "SELECT formation, tempo, pressing, defensive_line, width, powerplay_enabled FROM tactics WHERE club_id=?"
    ).bind(away).fetch_optional(&pool).await.map_err(|e| e.to_string())?;
    if let Some((af, at, ap, ad, aw, _)) = away_tac {
        eng.set_tactics(1, EngineTactics { formation: formation_code(&af), tempo: at as f32, pressing: ap as f32, defensive_line: ad as f32, width: aw as f32 });
    }
    eng.start();
    let snap = eng.snapshot();
    {
        let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
        *guard = Some(eng);
        let mut info = state.live_match_info.lock().map_err(|e| e.to_string())?;
        *info = Some(LiveMatchInfo { match_id, home_club_id: home, away_club_id: away });
    }
    Ok(snap)
}

#[tauri::command]
pub async fn start_live_match(state: State<'_, AppState>, match_id: i64) -> Result<MatchSnapshot, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let (home, away, hn, hc, an, ac) = match_info(&pool, match_id).await?;
    let rows_home = load_roster_raw(&pool, home).await?;
    let rows_away = load_roster_raw(&pool, away).await?;
    let mut r1: Vec<(u32, u8, Role, PlayerAttrs)> = Vec::new();
    for r in &rows_home { let role = role_for_index(r1.len()); r1.push((r.0 as u32, (r1.len() + 1) as u8, role, attrs_from(r))); }
    let mut r2: Vec<(u32, u8, Role, PlayerAttrs)> = Vec::new();
    for r in &rows_away { let role = role_for_index(r2.len()); r2.push((r.0 as u32, (r2.len() + 1) as u8, role, attrs_from(r))); }
    let mut eng = MatchEngine::new([(0, hn, hc), (1, an, ac)], [r1, r2]);
    eng.start();
    let snap = eng.snapshot();
    {
        let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
        *guard = Some(eng);
        let mut info = state.live_match_info.lock().map_err(|e| e.to_string())?;
        *info = Some(LiveMatchInfo { match_id, home_club_id: home, away_club_id: away });
    }
    Ok(snap)
}

#[tauri::command]
async fn persist_finished_match(pool: &sqlx::SqlitePool, info: &LiveMatchInfo, snapshot: &MatchSnapshot) -> Result<(), String> {
    let (status,): (String,) = sqlx::query_as("SELECT status FROM matches WHERE id=?").bind(info.match_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if status == "finished" { return Ok(()); }
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE matches SET status='finished', home_score=?, away_score=?, home_shots=?, away_shots=?, home_fouls=?, away_fouls=?, home_possession=?, away_possession=? WHERE id=?")
        .bind(snapshot.score[0] as i64).bind(snapshot.score[1] as i64)
        .bind(snapshot.shots[0] as i64).bind(snapshot.shots[1] as i64)
        .bind(snapshot.fouls[0] as i64).bind(snapshot.fouls[1] as i64)
        .bind(snapshot.possession[0] as i64).bind(snapshot.possession[1] as i64)
        .bind(info.match_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    for event in &snapshot.events {
        sqlx::query("INSERT INTO match_events (match_id, minute, second, event_type, player_id, club_id, description, x, y) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(info.match_id).bind(event.minute as i64).bind(event.second as i64).bind(&event.kind)
            .bind(event.player_id.map(|id| id as i64)).bind(if event.team_id == 0 { info.home_club_id } else { info.away_club_id })
            .bind(&event.description).bind(event.x).bind(event.y).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    sqlx::query("UPDATE league_standings SET played=played+1, won=won+CASE WHEN club_id=? AND ? > ? OR club_id=? AND ? > ? THEN 1 ELSE 0 END, drawn=drawn+CASE WHEN ? = ? THEN 1 ELSE 0 END, lost=lost+CASE WHEN club_id=? AND ? < ? OR club_id=? AND ? < ? THEN 1 ELSE 0 END, goals_for=goals_for+CASE WHEN club_id=? THEN ? ELSE ? END, goals_against=goals_against+CASE WHEN club_id=? THEN ? ELSE ? END, goal_difference=goal_difference+CASE WHEN club_id=? THEN ?-? ELSE ?-? END, points=points+CASE WHEN club_id=? AND ? > ? THEN 3 WHEN club_id=? AND ? = ? THEN 1 WHEN club_id=? AND ? > ? THEN 3 ELSE 0 END WHERE competition_id=(SELECT competition_id FROM matches WHERE id=?) AND club_id IN (?, ?)")
        .bind(info.home_club_id).bind(snapshot.score[0] as i64).bind(snapshot.score[1] as i64).bind(info.away_club_id).bind(snapshot.score[1] as i64).bind(snapshot.score[0] as i64)
        .bind(snapshot.score[0] as i64).bind(snapshot.score[1] as i64)
        .bind(info.home_club_id).bind(snapshot.score[0] as i64).bind(snapshot.score[1] as i64).bind(info.away_club_id).bind(snapshot.score[1] as i64).bind(snapshot.score[0] as i64)
        .bind(info.home_club_id).bind(snapshot.score[0] as i64).bind(snapshot.score[1] as i64).bind(info.away_club_id).bind(snapshot.score[1] as i64).bind(snapshot.score[0] as i64)
        .bind(info.home_club_id).bind(snapshot.score[0] as i64).bind(snapshot.score[1] as i64).bind(info.away_club_id).bind(snapshot.score[1] as i64).bind(snapshot.score[0] as i64)
        .bind(info.match_id).bind(info.home_club_id).bind(info.away_club_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn live_tactics(state: State<'_, AppState>, formation: u8, tempo: f32, pressing: f32, defensive_line: f32, width: f32) -> Result<MatchSnapshot, String> {
    let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
    let eng = guard.as_mut().ok_or("No hay partido en vivo")?;
    eng.update_live_tactics(0, EngineTactics { formation, tempo:tempo.clamp(0.0,100.0), pressing:pressing.clamp(0.0,100.0), defensive_line:defensive_line.clamp(0.0,100.0), width:width.clamp(0.0,100.0) })?;
    Ok(eng.snapshot())
}

#[tauri::command]
pub async fn live_substitute(state: State<'_, AppState>, team: usize, out_id: u32, in_id: u32) -> Result<MatchSnapshot, String> {
    let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
    let eng = guard.as_mut().ok_or("No hay partido en vivo")?;
    eng.manual_substitution(team, out_id, in_id)?;
    Ok(eng.snapshot())
}

#[tauri::command]
pub async fn live_timeout(state: State<'_, AppState>, team: usize) -> Result<MatchSnapshot, String> {
    let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
    let eng = guard.as_mut().ok_or("No hay partido en vivo")?;
    eng.call_timeout(team)?;
    Ok(eng.snapshot())
}

#[tauri::command]
pub async fn tick_live(state: State<'_, AppState>, ticks: Option<u32>) -> Result<MatchSnapshot, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let n = ticks.unwrap_or(1) as usize;
    let (snapshot, finished) = {
        let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
        let eng = guard.as_mut().ok_or("No hay partido en vivo")?;
        for _ in 0..n {
            eng.tick();
            if eng.state == crate::engine::MatchState::Finished { break; }
        }
        (eng.snapshot(), eng.state == crate::engine::MatchState::Finished)
    };
    if finished {
        let info = state.live_match_info.lock().map_err(|e| e.to_string())?.clone().ok_or("Falta información del partido")?;
        persist_finished_match(&pool, &info, &snapshot).await?;
    }
    Ok(snapshot)
}

#[tauri::command]
pub async fn get_live_snapshot(state: State<'_, AppState>) -> Result<MatchSnapshot, String> {
    let guard = state.live_match.lock().map_err(|e| e.to_string())?;
    let eng = guard.as_ref().ok_or("No hay partido en vivo")?;
    Ok(eng.snapshot())
}
