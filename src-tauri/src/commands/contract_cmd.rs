use tauri::State;
use crate::commands::AppState;
use crate::transfer::ContractRow;

fn pool(state: &State<'_, AppState>) -> Result<sqlx::SqlitePool, String> {
    state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or_else(|| "No hay partida activa".into())
}

#[tauri::command]
pub async fn get_contracts(state: State<'_, AppState>, club_id: i64) -> Result<Vec<ContractRow>, String> {
    crate::transfer::get_contracts(&pool(&state)?, club_id).await
}

#[tauri::command]
pub async fn renew_contract(state: State<'_, AppState>, club_id: i64, player_id: i64, years: i64, wage: f64, release_clause: Option<f64>, role: String, signing_bonus: f64, appearance_bonus: f64, clean_sheet_bonus: f64) -> Result<String, String> {
    crate::transfer::renew_contract(&pool(&state)?, club_id, player_id, years, wage, release_clause, role, signing_bonus, appearance_bonus, clean_sheet_bonus).await
}
