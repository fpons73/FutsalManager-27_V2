use tauri::State;
use crate::commands::AppState;

async fn db(state: &State<'_, AppState>) -> Result<sqlx::SqlitePool, String> {
    state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida".into())
}
async fn user_club(pool: &sqlx::SqlitePool) -> Result<i64, String> {
    sqlx::query_as::<_,(Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?.0.ok_or("Sin club".into())
}

#[tauri::command]
pub async fn get_tactical_automations(state: State<'_, AppState>) -> Result<Vec<crate::training::TacticalAutomation>, String> { let pool=db(&state).await?; let c=user_club(&pool).await?; crate::training::get_automations(&pool,c).await }

#[tauri::command]
pub async fn set_tactical_automation(state: State<'_, AppState>, name:String, trigger_type:String, threshold:i64, formation:String, tempo:i64, pressing:i64, defensive_line:i64, width:i64, enabled:bool) -> Result<String, String> { let pool=db(&state).await?; let c=user_club(&pool).await?; crate::training::set_automation(&pool,c,name,trigger_type,threshold,formation,tempo,pressing,defensive_line,width,enabled).await?; Ok("Automatismo guardado".into()) }

#[tauri::command]
pub async fn get_training_schedule(state: State<'_, AppState>) -> Result<Vec<crate::training::TrainingRow>, String> { let pool=db(&state).await?; let c=user_club(&pool).await?; crate::training::ensure_default_schedule(&pool,c).await?; crate::training::get_schedule(&pool,c).await }
#[tauri::command]
pub async fn set_training_schedule(state: State<'_, AppState>, schedule: Vec<(i64,i64,i64)>) -> Result<String, String> { let pool=db(&state).await?; let c=user_club(&pool).await?; crate::training::set_schedule(&pool,c,schedule).await?; Ok("Calendario actualizado".into()) }
#[tauri::command]
pub async fn get_training_progress(state: State<'_, AppState>) -> Result<Vec<crate::training::ProgressRow>, String> { let pool=db(&state).await?; let c=user_club(&pool).await?; crate::training::get_progress(&pool,c).await }
#[tauri::command]
pub async fn get_training_types(state: State<'_, AppState>) -> Result<Vec<(i64,String,String,i64)>, String> { let pool=db(&state).await?; sqlx::query_as("SELECT id,name,category,intensity FROM training_types ORDER BY id").fetch_all(&pool).await.map_err(|e| e.to_string()) }

#[tauri::command]
pub async fn get_staff_impact(state: State<'_, AppState>) -> Result<crate::training::StaffImpact, String> {
    let pool=db(&state).await?; let c=user_club(&pool).await?;
    let r: (i64,i64,i64,i64,i64,i64,i64) = sqlx::query_as("SELECT COALESCE(MAX(CASE WHEN role='coach' THEN tactical END),0),COALESCE(MAX(CASE WHEN role='assistant' THEN man_management END),0),COALESCE(MAX(CASE WHEN role='scout' THEN judging END),0),COALESCE(MAX(CASE WHEN role IN ('coach','assistant','technical_coach','goalkeeper_coach') THEN working_youngsters END),0),COALESCE(MAX(CASE WHEN role='physio' THEN physio_level END),0),COALESCE(MAX(CASE WHEN role='fitness_coach' THEN physio_level END),0),COALESCE(MAX(CASE WHEN role IN ('technical_coach','goalkeeper_coach') THEN tactical END),0) FROM staff WHERE club_id=?").bind(c).fetch_one(&pool).await.map_err(|e| e.to_string())?;
    Ok(crate::training::StaffImpact { coach:r.0, assistant:r.1, scout:r.2, youth:r.3, physio:r.4, training_bonus: (((crate::training::staff_training_factor(r.0, r.1, r.6, r.5) - 1.0) * 100.0).round() as i64).max(0), scouting_bonus: (r.2/10), injury_reduction: ((1.0 - crate::training::staff_injury_risk_factor(r.4, r.5)) * 100.0).round() as i64 })
}
