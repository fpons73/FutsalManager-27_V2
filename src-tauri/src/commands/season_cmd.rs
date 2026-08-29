use tauri::State;
use crate::commands::AppState;
use serde::Serialize;

#[derive(Serialize)]
pub struct MovementRow { pub season: String, pub from_competition_id: i64, pub from_competition: String, pub to_competition_id: i64, pub to_competition: String, pub club_id: i64, pub club_name: String, pub movement_type: String, pub source_position: i64 }
#[derive(Serialize)]
pub struct HonourRow { pub season: String, pub competition_id: i64, pub competition: String, pub club_id: i64, pub club_name: String, pub honour_type: String }
#[derive(Serialize, sqlx::FromRow)]
pub struct PlayerHistoryRow { pub season: String, pub player_id: i64, pub player_name: String, pub club_id: Option<i64>, pub current_ability: i64, pub potential_ability: i64, pub morale: i64 }
#[derive(Serialize, sqlx::FromRow)]
pub struct RecordRow { pub record_type: String, pub value: f64, pub season: String, pub player_id: Option<i64>, pub player_name: Option<String>, pub club_id: Option<i64>, pub club_name: Option<String> }

#[tauri::command]
pub async fn check_season_finished(state: State<'_, AppState>) -> Result<bool, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    crate::season::is_season_finished(&pool).await
}

#[tauri::command]
pub async fn get_season_movements(state: State<'_, AppState>, season: Option<String>) -> Result<Vec<MovementRow>, String> {
    let pool = { let g = state.pool.lock().map_err(|e| e.to_string())?; g.clone().ok_or("No hay partida")? };
    let rows = if let Some(s) = season { sqlx::query_as::<_, (String,i64,String,i64,String,i64,String,String,i64)>("SELECT sm.season, sm.from_competition_id, fc.name, sm.to_competition_id, tc.name, sm.club_id, c.name, sm.movement_type, sm.source_position FROM season_movements sm JOIN competitions fc ON fc.id=sm.from_competition_id JOIN competitions tc ON tc.id=sm.to_competition_id JOIN clubs c ON c.id=sm.club_id WHERE sm.season=? ORDER BY sm.created_at, sm.movement_type, sm.source_position").bind(s).fetch_all(&pool).await.map_err(|e| e.to_string())? } else { sqlx::query_as::<_, (String,i64,String,i64,String,i64,String,String,i64)>("SELECT sm.season, sm.from_competition_id, fc.name, sm.to_competition_id, tc.name, sm.club_id, c.name, sm.movement_type, sm.source_position FROM season_movements sm JOIN competitions fc ON fc.id=sm.from_competition_id JOIN competitions tc ON tc.id=sm.to_competition_id JOIN clubs c ON c.id=sm.club_id ORDER BY sm.created_at DESC, sm.movement_type, sm.source_position").fetch_all(&pool).await.map_err(|e| e.to_string())? };
    Ok(rows.into_iter().map(|(season, from_competition_id, from_competition, to_competition_id, to_competition, club_id, club_name, movement_type, source_position)| MovementRow { season, from_competition_id, from_competition, to_competition_id, to_competition, club_id, club_name, movement_type, source_position }).collect())
}

#[tauri::command]
pub async fn get_honours(state: State<'_, AppState>, club_id: Option<i64>) -> Result<Vec<HonourRow>, String> {
    let pool = { let g = state.pool.lock().map_err(|e| e.to_string())?; g.clone().ok_or("No hay partida")? };
    let query = "SELECT h.season, h.competition_id, c.name, h.club_id, cl.name, h.honour_type FROM competition_honours h JOIN competitions c ON c.id=h.competition_id JOIN clubs cl ON cl.id=h.club_id WHERE (? IS NULL OR h.club_id=?) ORDER BY h.season DESC, h.id DESC";
    let rows: Vec<(String,i64,String,i64,String,String)> = sqlx::query_as(query).bind(club_id).bind(club_id).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(season, competition_id, competition, club_id, club_name, honour_type)| HonourRow { season, competition_id, competition, club_id, club_name, honour_type }).collect())
}

#[tauri::command]
pub async fn get_player_history(state: State<'_, AppState>, player_id: i64) -> Result<Vec<PlayerHistoryRow>, String> {
    let pool = { let g = state.pool.lock().map_err(|e| e.to_string())?; g.clone().ok_or("No hay partida")? };
    let rows: Vec<PlayerHistoryRow> = sqlx::query_as("SELECT h.season,h.player_id,p.common_name,h.club_id,h.current_ability,h.potential_ability,h.morale FROM player_attribute_history h JOIN players p ON p.id=h.player_id WHERE h.player_id=? ORDER BY h.season").bind(player_id).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub async fn get_competition_records(state: State<'_, AppState>, competition_id: i64) -> Result<Vec<RecordRow>, String> {
    let pool = { let g = state.pool.lock().map_err(|e| e.to_string())?; g.clone().ok_or("No hay partida")? };
    let rows: Vec<RecordRow> = sqlx::query_as("SELECT r.record_type,r.value,r.season,r.player_id,p.common_name,r.club_id,c.name FROM competition_records r LEFT JOIN players p ON p.id=r.player_id LEFT JOIN clubs c ON c.id=r.club_id WHERE r.competition_id=? ORDER BY r.record_type,r.value DESC").bind(competition_id).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub async fn get_club_records(state: State<'_, AppState>, club_id: i64) -> Result<Vec<RecordRow>, String> {
    let pool = { let g = state.pool.lock().map_err(|e| e.to_string())?; g.clone().ok_or("No hay partida")? };
    let rows: Vec<RecordRow> = sqlx::query_as("SELECT r.record_type,r.value,r.season,NULL,NULL,r.club_id,c.name FROM club_records r LEFT JOIN clubs c ON c.id=r.club_id WHERE r.club_id=? ORDER BY r.record_type,r.value DESC").bind(club_id).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub async fn rollover_season_cmd(state: State<'_, AppState>) -> Result<String, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    crate::season::rollover_season(&pool).await
}
