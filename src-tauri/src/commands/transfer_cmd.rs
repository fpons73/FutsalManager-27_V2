use tauri::State;
use crate::commands::AppState;

#[tauri::command]
pub async fn get_precontracts(state: State<'_, AppState>) -> Result<Vec<crate::transfer::PrecontractRow>, String> {
    let pool={state.pool.lock().map_err(|e|e.to_string())?.clone().ok_or("No hay partida")?};
    let club: i64=sqlx::query_as::<_,(Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e|e.to_string())?.0.ok_or("Sin club")?;
    crate::transfer::get_precontracts(&pool,club).await
}

#[tauri::command]
pub async fn make_precontract(state: State<'_, AppState>, player_id:i64, wage:f64, signing_bonus:f64, years:i64) -> Result<String,String> {
    let pool={state.pool.lock().map_err(|e|e.to_string())?.clone().ok_or("No hay partida")?};
    let club: i64=sqlx::query_as::<_,(Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e|e.to_string())?.0.ok_or("Sin club")?;
    crate::transfer::make_precontract(&pool,player_id,club,wage,signing_bonus,years).await
}

#[tauri::command]
pub async fn get_loans(state: State<'_, AppState>) -> Result<Vec<crate::transfer::LoanRow>, String> {
    let pool={state.pool.lock().map_err(|e|e.to_string())?.clone().ok_or("No hay partida")?};
    let club: i64=sqlx::query_as::<_,(Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e|e.to_string())?.0.ok_or("Sin club")?;
    crate::transfer::get_loans(&pool,club).await
}

#[tauri::command]
pub async fn offer_loan(state: State<'_, AppState>, player_id:i64, to_club:i64, months:i64, wage:f64) -> Result<String,String> {
    let pool={state.pool.lock().map_err(|e|e.to_string())?.clone().ok_or("No hay partida")?};
    let from: i64=sqlx::query_as::<_,(Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e|e.to_string())?.0.ok_or("Sin club")?;
    crate::transfer::offer_loan(&pool,player_id,from,to_club,months,wage).await
}

#[tauri::command]
pub async fn get_free_agents(state: State<'_, AppState>) -> Result<Vec<crate::transfer::MarketPlayer>, String> {
    let pool = { state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")? };
    let club: Option<i64> = sqlx::query_as::<_, (Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?.0;
    crate::transfer::get_free_agents(&pool, club.unwrap_or(0)).await
}

#[tauri::command]
pub async fn sign_free_agent(state: State<'_, AppState>, player_id: i64, wage: f64, years: i64) -> Result<String, String> {
    let pool = { state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")? };
    let club: Option<i64> = sqlx::query_as::<_, (Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?.0;
    crate::transfer::sign_free_agent(&pool, player_id, club.ok_or("Sin club")?, wage, years).await
}

#[tauri::command]
pub async fn get_market(state: State<'_, AppState>) -> Result<Vec<crate::transfer::MarketPlayer>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club usuario")?;
    crate::transfer::get_market(&pool, uc).await
}

#[tauri::command]
pub async fn get_offers(state: State<'_, AppState>) -> Result<Vec<crate::transfer::OfferRow>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    crate::transfer::get_offers(&pool, uc).await
}

#[tauri::command]
pub async fn make_offer(state: State<'_, AppState>, player_id: i64, fee: f64) -> Result<String, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    crate::transfer::make_offer(&pool, player_id, uc, fee).await
}

#[tauri::command]
pub async fn respond_offer(state: State<'_, AppState>, offer_id: i64, accept: bool) -> Result<String, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    crate::transfer::respond_offer(&pool, offer_id, accept).await
}
