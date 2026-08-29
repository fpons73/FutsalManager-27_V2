use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

use crate::commands::AppState;

#[derive(Serialize)]
pub struct NewGameResult {
    pub game_date: String,
    pub season: String,
    pub clubs: Vec<ClubRow>,
    pub competitions: Vec<CompRow>,
}

#[derive(Serialize, Clone)]
pub struct ClubRow {
    pub id: i64,
    pub name: String,
    pub short_name: String,
    pub nation: String,
    pub reputation: i64,
    pub primary_color: String,
    pub division: String,
    pub tier: Option<i64>,
}

#[derive(Serialize, Clone)]
pub struct CompRow {
    pub id: i64,
    pub name: String,
    pub nation: String,
    pub kind: String,
    pub competition_type: String,
    pub knockout_rounds: i64,
    pub champion_id: Option<i64>,
    pub champion_name: Option<String>,
}

#[derive(Serialize)]
pub struct GameStateRow {
    pub game_date: String,
    pub season: String,
    pub user_club_id: Option<i64>,
    pub user_club_name: Option<String>,
}

#[derive(Serialize)]
pub struct StandingRow {
    pub position: i64,
    pub club_id: i64,
    pub club_name: String,
    pub short_name: String,
    pub played: i64,
    pub won: i64,
    pub drawn: i64,
    pub lost: i64,
    pub goals_for: i64,
    pub goals_against: i64,
    pub goal_difference: i64,
    pub points: i64,
}

#[derive(sqlx::FromRow)]
struct FixtureRaw { id:i64, round:i64, date:String, home_id:i64, home_name:String, home_short:String, away_id:i64, away_name:String, away_short:String, home_score:i64, away_score:i64, status:String, competition_type:String, cup_winner_id:Option<i64>, extra:i64, penalties:i64, penalty_home:i64, penalty_away:i64 }

#[derive(Serialize)]
pub struct FixtureRow {
    pub id: i64,
    pub round: i64,
    pub date: String,
    pub home_id: i64,
    pub home_name: String,
    pub home_short: String,
    pub away_id: i64,
    pub away_name: String,
    pub away_short: String,
    pub home_score: i64,
    pub away_score: i64,
    pub status: String,
    pub competition_type: String,
    pub cup_winner_id: Option<i64>,
    pub went_to_extra_time: bool,
    pub went_to_penalties: bool,
    pub penalty_home_score: i64,
    pub penalty_away_score: i64,
}

#[derive(Serialize)]
pub struct PlayerRow {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub common_name: String,
    pub age: i64,
    pub nation: String,
    pub flag_path: Option<String>,
    pub second_flag_path: Option<String>,
    pub secondary_position: Option<String>,
    pub position: String,
    pub ca: i64,
    pub pa: i64,
    pub wage: f64,
    pub condition: i64,
    pub morale: i64,
    pub attrs: PlayerAttrsRow,
}

#[derive(Serialize)]
pub struct PlayerAttrsRow {
    pub passing: i64,
    pub finishing: i64,
    pub dribbling: i64,
    pub tackling: i64,
    pub vision: i64,
    pub anticipation: i64,
    pub positioning: i64,
    pub stamina: i64,
    pub acceleration: i64,
    pub pace: i64,
    pub composure: i64,
    pub technique: i64,
    pub reflexes: i64,
}

async fn get_pool(state: &State<'_, AppState>) -> Result<SqlitePool, String> {
    let guard = state.pool.lock().map_err(|e| e.to_string())?;
    guard.clone().ok_or_else(|| "No hay partida activa. Crea una nueva partida.".to_string())
}

#[tauri::command]
pub async fn new_game(state: State<'_, AppState>, user_club_id: Option<i64>) -> Result<NewGameResult, String> {
    let existing_pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone()
    };
    if let Some(existing) = existing_pool {
        let cnt: i64 = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM clubs").fetch_one(&existing).await.map_err(|e| e.to_string()).map(|(c,)| c).unwrap_or(0);
        if cnt > 0 {
            if let Some(cid) = user_club_id {
                sqlx::query("UPDATE game_state SET user_club_id=? WHERE id=1").bind(cid).execute(&existing).await.map_err(|e| e.to_string())?;
            }
            let (game_date,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&existing).await.map_err(|e| e.to_string())?;
            let (season,): (String,) = sqlx::query_as("SELECT season FROM game_state WHERE id=1").fetch_one(&existing).await.map_err(|e| e.to_string())?;
            let clubs: Vec<ClubRow> = sqlx::query_as("SELECT c.id, c.name, c.short_name, n.name, c.reputation, c.primary_color, COALESCE((SELECT comp.name FROM league_standings ls JOIN competitions comp ON comp.id=ls.competition_id WHERE ls.club_id=c.id AND comp.tier IS NOT NULL ORDER BY comp.tier LIMIT 1),'Sin liga') AS division, (SELECT comp.tier FROM league_standings ls JOIN competitions comp ON comp.id=ls.competition_id WHERE ls.club_id=c.id AND comp.tier IS NOT NULL ORDER BY comp.tier LIMIT 1) AS tier FROM clubs c JOIN nations n ON n.id=c.nation_id ORDER BY n.name, tier, c.reputation DESC").fetch_all(&existing).await.map_err(|e| e.to_string())?.into_iter().map(|(id, name, short_name, nation, reputation, primary_color, division, tier): (i64, String, String, String, i64, String, String, Option<i64>)| ClubRow { id, name, short_name, nation, reputation, primary_color, division, tier }).collect();
            let comps: Vec<CompRow> = sqlx::query_as("SELECT comp.id, comp.name, COALESCE(n.name,'Internacional'), comp.kind, comp.competition_type, comp.knockout_rounds, h.club_id, cl.name FROM competitions comp LEFT JOIN nations n ON n.id=comp.nation_id LEFT JOIN competition_honours h ON h.competition_id=comp.id AND h.season=(SELECT season FROM game_state WHERE id=1) LEFT JOIN clubs cl ON cl.id=h.club_id ORDER BY comp.kind, comp.id").fetch_all(&existing).await.map_err(|e| e.to_string())?.into_iter().map(|(id, name, nation, kind, competition_type, knockout_rounds, champion_id, champion_name): (i64, String, String, String, String, i64, Option<i64>, Option<String>)| CompRow { id, name, nation, kind, competition_type, knockout_rounds, champion_id, champion_name }).collect();
            return Ok(NewGameResult { game_date, season, clubs, competitions: comps });
        }
    }
    {
        let mut guard = state.pool.lock().map_err(|e| e.to_string())?;
        *guard = None;
    }
    {
        let mut live = state.live_match.lock().map_err(|e| e.to_string())?;
        *live = None;
    }
    // Pequeña espera para que SQLite cierre ficheros en Windows
    tokio::time::sleep(std::time::Duration::from_millis(120)).await;
    let path = crate::db::db_path();
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("db-wal"));
    let _ = std::fs::remove_file(path.with_extension("db-shm"));
    let _ = std::fs::remove_file(path.with_extension("db-journal"));
    let pool = crate::db::init_pool(None).await.map_err(|e| e.to_string())?;
    crate::world::seed_world(&pool).await?;
    if let Some(cid) = user_club_id {
        sqlx::query("UPDATE game_state SET user_club_id=? WHERE id=1").bind(cid).execute(&pool).await.map_err(|e| e.to_string())?;
    }
    {
        let mut guard = state.pool.lock().map_err(|e| e.to_string())?;
        *guard = Some(pool.clone());
    }
    let (game_date,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let (season,): (String,) = sqlx::query_as("SELECT season FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;

    let clubs: Vec<ClubRow> = sqlx::query_as(
        "SELECT c.id, c.name, c.short_name, n.name, c.reputation, c.primary_color, COALESCE((SELECT comp.name FROM league_standings ls JOIN competitions comp ON comp.id=ls.competition_id WHERE ls.club_id=c.id AND comp.tier IS NOT NULL ORDER BY comp.tier LIMIT 1),'Sin liga') AS division, (SELECT comp.tier FROM league_standings ls JOIN competitions comp ON comp.id=ls.competition_id WHERE ls.club_id=c.id AND comp.tier IS NOT NULL ORDER BY comp.tier LIMIT 1) AS tier FROM clubs c JOIN nations n ON n.id=c.nation_id ORDER BY n.name, tier, c.reputation DESC"
    )
    .fetch_all(&pool).await.map_err(|e| e.to_string())?
    .into_iter().map(|(id, name, short_name, nation, reputation, primary_color, division, tier): (i64, String, String, String, i64, String, String, Option<i64>)| ClubRow { id, name, short_name, nation, reputation, primary_color, division, tier }).collect();

    let comps: Vec<CompRow> = sqlx::query_as(
        "SELECT comp.id, comp.name, COALESCE(n.name,'Internacional'), comp.kind, comp.competition_type, comp.knockout_rounds, h.club_id, cl.name FROM competitions comp LEFT JOIN nations n ON n.id=comp.nation_id LEFT JOIN competition_honours h ON h.competition_id=comp.id AND h.season=(SELECT season FROM game_state WHERE id=1) LEFT JOIN clubs cl ON cl.id=h.club_id ORDER BY comp.kind, comp.id"
    )
    .fetch_all(&pool).await.map_err(|e| e.to_string())?
    .into_iter().map(|(id, name, nation, kind, competition_type, knockout_rounds, champion_id, champion_name): (i64, String, String, String, String, i64, Option<i64>, Option<String>)| CompRow { id, name, nation, kind, competition_type, knockout_rounds, champion_id, champion_name }).collect();

    Ok(NewGameResult { game_date, season, clubs, competitions: comps })
}

#[tauri::command]
pub async fn get_game_state(state: State<'_, AppState>) -> Result<GameStateRow, String> {
    let pool = get_pool(&state).await?;
    let row: (String, String, Option<i64>) = sqlx::query_as("SELECT game_date, season, user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let club_name = if let Some(cid) = row.2 {
        sqlx::query_as::<_, (String,)>("SELECT name FROM clubs WHERE id=?").bind(cid).fetch_optional(&pool).await.map_err(|e| e.to_string())?.map(|(n,)| n)
    } else { None };
    Ok(GameStateRow { game_date: row.0, season: row.1, user_club_id: row.2, user_club_name: club_name })
}

#[tauri::command]
pub async fn advance_day_cmd(state: State<'_, AppState>) -> Result<crate::simulation::AdvanceResult, String> {
    let pool = get_pool(&state).await?;
    crate::simulation::advance_day(&pool).await
}

#[tauri::command]
pub async fn advance_week_cmd(state: State<'_, AppState>) -> Result<Vec<crate::simulation::AdvanceResult>, String> {
    let pool = get_pool(&state).await?;
    crate::simulation::advance_days(&pool, 7).await
}

#[tauri::command]
pub async fn get_standings(state: State<'_, AppState>, competition_id: i64) -> Result<Vec<StandingRow>, String> {
    let pool = get_pool(&state).await?;
    let rows = sqlx::query_as::<_, (i64, i64, String, String, i64, i64, i64, i64, i64, i64, i64, i64)>(
        "SELECT ls.position, ls.club_id, c.name, c.short_name, ls.played, ls.won, ls.drawn, ls.lost, ls.goals_for, ls.goals_against, ls.goal_difference, ls.points FROM league_standings ls JOIN clubs c ON c.id=ls.club_id WHERE ls.competition_id=? ORDER BY ls.position ASC, ls.points DESC"
    ).bind(competition_id).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(position, club_id, club_name, short_name, played, won, drawn, lost, goals_for, goals_against, goal_difference, points)| StandingRow { position, club_id, club_name, short_name, played, won, drawn, lost, goals_for, goals_against, goal_difference, points }).collect())
}

#[tauri::command]
pub async fn get_fixtures(state: State<'_, AppState>, competition_id: i64) -> Result<Vec<FixtureRow>, String> {
    let pool = get_pool(&state).await?;
    let rows = sqlx::query_as::<_, FixtureRaw>(
        "SELECT m.id, m.round, m.date, m.home_club_id, hc.name, hc.short_name, m.away_club_id, ac.name, ac.short_name, m.home_score, m.away_score, m.status, comp.competition_type, ct.winner_club_id, COALESCE(ct.went_to_extra_time,0), COALESCE(ct.went_to_penalties,0), COALESCE(ct.penalty_home_score,0), COALESCE(ct.penalty_away_score,0) FROM matches m JOIN competitions comp ON comp.id=m.competition_id JOIN clubs hc ON hc.id=m.home_club_id JOIN clubs ac ON ac.id=m.away_club_id LEFT JOIN cup_ties ct ON ct.match_id=m.id WHERE m.competition_id=? ORDER BY m.round, m.date, m.id"
    ).bind(competition_id).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|r| FixtureRow { id:r.id, round:r.round, date:r.date, home_id:r.home_id, home_name:r.home_name, home_short:r.home_short, away_id:r.away_id, away_name:r.away_name, away_short:r.away_short, home_score:r.home_score, away_score:r.away_score, status:r.status, competition_type:r.competition_type, cup_winner_id:r.cup_winner_id, went_to_extra_time:r.extra != 0, went_to_penalties:r.penalties != 0, penalty_home_score:r.penalty_home, penalty_away_score:r.penalty_away }).collect())
}

#[derive(sqlx::FromRow)]
struct SquadRaw {
    id: i64,
    first_name: String,
    last_name: String,
    common_name: String,
    date_of_birth: String,
    nation: String,
    flag_path: Option<String>,
    second_flag_path: Option<String>,
    secondary_position: Option<String>,
    position: String,
    ca: i64,
    pa: i64,
    wage: f64,
    condition: i64,
    morale: i64,
    passing: i64,
    finishing: i64,
    dribbling: i64,
    tackling: i64,
    vision: i64,
    anticipation: i64,
    positioning: i64,
    stamina: i64,
    acceleration: i64,
    pace: i64,
    composure: i64,
    technique: i64,
    reflexes: i64,
}

#[tauri::command]
pub async fn get_squad(state: State<'_, AppState>, club_id: i64) -> Result<Vec<PlayerRow>, String> {
    let pool = get_pool(&state).await?;
    let mut rows = sqlx::query_as::<_, SquadRaw>(
        "SELECT p.id as id, p.first_name as first_name, p.last_name as last_name, p.common_name as common_name, p.date_of_birth as date_of_birth, n.name as nation, n.flag_path as flag_path, n2.flag_path as second_flag_path, p.secondary_position as secondary_position, COALESCE(pp2.pos, 'UNI') as position, ps.current_ability as ca, ps.potential_ability as pa, c.wage_weekly as wage, ps.condition_val as condition, ps.morale as morale, pa.passing as passing, pa.finishing as finishing, pa.dribbling as dribbling, pa.tackling as tackling, pa.vision as vision, pa.anticipation as anticipation, pa.positioning as positioning, pa.stamina as stamina, pa.acceleration as acceleration, pa.pace as pace, pa.composure as composure, pa.technique as technique, pa.reflexes as reflexes FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN nations n ON n.id=p.nation_id JOIN player_states ps ON ps.player_id=p.id JOIN player_attributes pa ON pa.player_id=p.id LEFT JOIN nations n2 ON n2.id=p.second_nation_id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp2 ON pp2.player_id=p.id ORDER BY ps.current_ability DESC"
    ).bind(club_id).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    if rows.is_empty() {
        rows = sqlx::query_as::<_, SquadRaw>(
            "SELECT p.id as id, p.first_name as first_name, p.last_name as last_name, p.common_name as common_name, p.date_of_birth as date_of_birth, n.name as nation, n.flag_path as flag_path, n2.flag_path as second_flag_path, p.secondary_position as secondary_position, COALESCE(pp2.pos, 'UNI') as position, ps.current_ability as ca, ps.potential_ability as pa, COALESCE(c.wage_weekly,500) as wage, ps.condition_val as condition, ps.morale as morale, pa.passing as passing, pa.finishing as finishing, pa.dribbling as dribbling, pa.tackling as tackling, pa.vision as vision, pa.anticipation as anticipation, pa.positioning as positioning, pa.stamina as stamina, pa.acceleration as acceleration, pa.pace as pace, pa.composure as composure, pa.technique as technique, pa.reflexes as reflexes FROM players p JOIN nations n ON n.id=p.nation_id JOIN player_states ps ON ps.player_id=p.id JOIN player_attributes pa ON pa.player_id=p.id LEFT JOIN contracts c ON c.player_id=p.id AND c.is_active=1 LEFT JOIN nations n2 ON n2.id=p.second_nation_id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp2 ON pp2.player_id=p.id WHERE p.id IN (SELECT player_id FROM contracts WHERE club_id=? LIMIT 12) ORDER BY ps.current_ability DESC"
        ).bind(club_id).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    }

    let today: chrono::NaiveDate = sqlx::query_as::<_, (String,)>("SELECT game_date FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string()).and_then(|(d,)| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").map_err(|e| e.to_string()))?;

    Ok(rows.into_iter().map(|r| {
        let age = {
            let dob_d = chrono::NaiveDate::parse_from_str(&r.date_of_birth, "%Y-%m-%d").unwrap_or(today);
            ((today - dob_d).num_days() / 365) as i64
        };
        PlayerRow { id: r.id, first_name: r.first_name, last_name: r.last_name, common_name: r.common_name, age, nation: r.nation, flag_path: r.flag_path, second_flag_path: r.second_flag_path, secondary_position: r.secondary_position, position: r.position, ca: r.ca, pa: r.pa, wage: r.wage, condition: r.condition, morale: r.morale, attrs: PlayerAttrsRow { passing: r.passing, finishing: r.finishing, dribbling: r.dribbling, tackling: r.tackling, vision: r.vision, anticipation: r.anticipation, positioning: r.positioning, stamina: r.stamina, acceleration: r.acceleration, pace: r.pace, composure: r.composure, technique: r.technique, reflexes: r.reflexes } }
    }).collect())
}

#[tauri::command]
pub async fn get_competitions(state: State<'_, AppState>) -> Result<Vec<CompRow>, String> {
    let pool = get_pool(&state).await?;
    let rows: Vec<(i64, String, String, String, String, i64, Option<i64>, Option<String>)> = sqlx::query_as("SELECT comp.id, comp.name, COALESCE(n.name,'Internacional'), comp.kind, comp.competition_type, comp.knockout_rounds, h.club_id, cl.name FROM competitions comp LEFT JOIN nations n ON n.id=comp.nation_id LEFT JOIN competition_honours h ON h.competition_id=comp.id AND h.season=(SELECT season FROM game_state WHERE id=1) LEFT JOIN clubs cl ON cl.id=h.club_id ORDER BY comp.kind, comp.id").fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name, nation, kind, competition_type, knockout_rounds, champion_id, champion_name)| CompRow { id, name, nation, kind, competition_type, knockout_rounds, champion_id, champion_name }).collect())
}

#[tauri::command]
pub async fn get_next_fixture(state: State<'_, AppState>, club_id: i64) -> Result<Option<FixtureRow>, String> {
    let pool = get_pool(&state).await?;
    let row = sqlx::query_as::<_, FixtureRaw>(
        "SELECT m.id, m.round, m.date, m.home_club_id, hc.name, hc.short_name, m.away_club_id, ac.name, ac.short_name, m.home_score, m.away_score, m.status, comp.competition_type, ct.winner_club_id FROM matches m JOIN competitions comp ON comp.id=m.competition_id JOIN clubs hc ON hc.id=m.home_club_id JOIN clubs ac ON ac.id=m.away_club_id LEFT JOIN cup_ties ct ON ct.match_id=m.id WHERE (m.home_club_id=? OR m.away_club_id=?) AND m.status='scheduled' ORDER BY m.date, m.round LIMIT 1"
    ).bind(club_id).bind(club_id).fetch_optional(&pool).await.map_err(|e| e.to_string())?;
    Ok(row.map(|r| FixtureRow { id:r.id, round:r.round, date:r.date, home_id:r.home_id, home_name:r.home_name, home_short:r.home_short, away_id:r.away_id, away_name:r.away_name, away_short:r.away_short, home_score:r.home_score, away_score:r.away_score, status:r.status, competition_type:r.competition_type, cup_winner_id:r.cup_winner_id, went_to_extra_time:r.extra != 0, went_to_penalties:r.penalties != 0, penalty_home_score:r.penalty_home, penalty_away_score:r.penalty_away }))
}
