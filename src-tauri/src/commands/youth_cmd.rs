use tauri::State;
use crate::commands::AppState;

async fn pool(state: &State<'_, AppState>) -> Result<sqlx::SqlitePool, String> {
    state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or_else(|| "No hay partida activa".into())
}
async fn club(db: &sqlx::SqlitePool) -> Result<i64, String> {
    sqlx::query_as::<_,(Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(db).await.map_err(|e| e.to_string())?.0.ok_or_else(|| "No hay club seleccionado".into())
}
#[tauri::command]
pub async fn get_youth(state: State<'_, AppState>) -> Result<Vec<crate::youth::YouthPlayerRow>, String> { let db=pool(&state).await?; let c=club(&db).await?; crate::youth::list(&db,c).await }
#[tauri::command]
pub async fn promote_youth(state: State<'_, AppState>, youth_id: i64) -> Result<String, String> { let db=pool(&state).await?; let c=club(&db).await?; crate::youth::promote(&db,c,youth_id).await }
