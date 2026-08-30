use serde::Serialize;
use tauri::State;
use crate::commands::AppState;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SponsorshipOfferRow {
    pub id: i64,
    pub sponsor_name: String,
    pub weekly_amount: f64,
    pub signing_bonus: f64,
    pub target_type: String,
    pub target_value: i64,
    pub duration_weeks: i64,
    pub expires_date: String,
}

fn active_pool(state: &State<'_, AppState>) -> Result<sqlx::SqlitePool, String> {
    state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or_else(|| "No hay partida".into())
}

async fn user_club(pool: &sqlx::SqlitePool) -> Result<i64, String> {
    let (club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    club.ok_or_else(|| "No hay club seleccionado".into())
}

#[tauri::command]
pub async fn get_finance(state: State<'_, AppState>) -> Result<crate::finance::FinanceRow, String> {
    let pool = active_pool(&state)?;
    crate::finance::get_finance(&pool, user_club(&pool).await?).await
}

#[tauri::command]
pub async fn get_sponsorship_offers(state: State<'_, AppState>) -> Result<Vec<SponsorshipOfferRow>, String> {
    let pool = active_pool(&state)?;
    let club = user_club(&pool).await?;
    sqlx::query_as("SELECT id,sponsor_name,weekly_amount,signing_bonus,target_type,target_value,duration_weeks,expires_date FROM sponsorship_offers WHERE club_id=? AND status='available' ORDER BY weekly_amount DESC")
        .bind(club).fetch_all(&pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn accept_sponsorship_offer(state: State<'_, AppState>, offer_id: i64) -> Result<String, String> {
    let pool = active_pool(&state)?;
    let club = user_club(&pool).await?;
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let offer: Option<(String,f64,f64,String,i64,i64,String)> = sqlx::query_as("SELECT sponsor_name,weekly_amount,signing_bonus,target_type,target_value,duration_weeks,expires_date FROM sponsorship_offers WHERE id=? AND club_id=? AND status='available'").bind(offer_id).bind(club).fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;
    let Some((name, weekly, bonus, target_type, target, weeks, expires)) = offer else { return Err("La oferta ya no está disponible".into()); };
    let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
    if today > expires { return Err("La oferta ha caducado".into()); }
    sqlx::query("UPDATE sponsorship_contracts SET status='replaced' WHERE club_id=? AND status='active'").bind(club).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    let end = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d").map_err(|e| e.to_string())? + chrono::Duration::weeks(weeks);
    sqlx::query("INSERT INTO sponsorship_contracts(club_id,sponsor_name,weekly_amount,signing_bonus,target_type,target_value,start_date,end_date,status) VALUES(?,?,?,?,?,?,?,?, 'active')").bind(club).bind(&name).bind(weekly).bind(bonus).bind(&target_type).bind(target).bind(&today).bind(end.format("%Y-%m-%d").to_string()).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE sponsorship_offers SET status='accepted' WHERE id=?").bind(offer_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE sponsorship_offers SET status='expired' WHERE club_id=? AND status='available' AND id<>?").bind(club).bind(offer_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE club_finances SET balance=balance+? WHERE club_id=?").bind(bonus).bind(club).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(format!("Contrato firmado con {name}"))
}

#[cfg(test)]
mod tests {
    use crate::{db, world};

    #[tokio::test]
    async fn accepts_sponsorship_offer_once() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        let (club_id,): (i64,) = sqlx::query_as("SELECT id FROM clubs ORDER BY id LIMIT 1").fetch_one(&pool).await.unwrap();
        let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&pool).await.unwrap();
        sqlx::query("INSERT INTO sponsorship_offers(club_id,sponsor_name,weekly_amount,signing_bonus,target_type,target_value,duration_weeks,expires_date) VALUES(?,?,?,?,?,?,?,date(?,'+30 day'))")
            .bind(club_id).bind("Test Sponsor").bind(999.0).bind(2500.0).bind("league_position").bind(3).bind(52).bind(&today).execute(&pool).await.unwrap();
        let (offer_id,): (i64,) = sqlx::query_as("SELECT id FROM sponsorship_offers WHERE club_id=? AND sponsor_name='Test Sponsor'").bind(club_id).fetch_one(&pool).await.unwrap();
        let mut tx = pool.begin().await.unwrap();
        let (name, weekly, bonus, target_type, target, weeks, expires): (String,f64,f64,String,i64,i64,String) = sqlx::query_as("SELECT sponsor_name,weekly_amount,signing_bonus,target_type,target_value,duration_weeks,expires_date FROM sponsorship_offers WHERE id=? AND club_id=? AND status='available'").bind(offer_id).bind(club_id).fetch_one(&mut *tx).await.unwrap();
        assert!(today <= expires);
        sqlx::query("UPDATE sponsorship_contracts SET status='replaced' WHERE club_id=? AND status='active'").bind(club_id).execute(&mut *tx).await.unwrap();
        let end = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap() + chrono::Duration::weeks(weeks);
        sqlx::query("INSERT INTO sponsorship_contracts(club_id,sponsor_name,weekly_amount,signing_bonus,target_type,target_value,start_date,end_date,status) VALUES(?,?,?,?,?,?,?,?, 'active')").bind(club_id).bind(&name).bind(weekly).bind(bonus).bind(&target_type).bind(target).bind(&today).bind(end.format("%Y-%m-%d").to_string()).execute(&mut *tx).await.unwrap();
        sqlx::query("UPDATE sponsorship_offers SET status='accepted' WHERE id=?").bind(offer_id).execute(&mut *tx).await.unwrap();
        sqlx::query("UPDATE club_finances SET balance=balance+? WHERE club_id=?").bind(bonus).bind(club_id).execute(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();
        let (accepted,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sponsorship_offers WHERE id=? AND status='accepted'").bind(offer_id).fetch_one(&pool).await.unwrap();
        let (contracts,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sponsorship_contracts WHERE club_id=? AND sponsor_name='Test Sponsor'").bind(club_id).fetch_one(&pool).await.unwrap();
        assert_eq!(accepted, 1);
        assert_eq!(contracts, 1);
    }
}

#[tauri::command]
pub async fn get_injuries(state: State<'_, AppState>) -> Result<Vec<(i64, String, String, String, String)>, String> {
    let pool = active_pool(&state)?;
    let uc = user_club(&pool).await?;
    sqlx::query_as("SELECT i.id, p.common_name, i.injury_type, i.expected_return_date, i.injury_date FROM injuries i JOIN players p ON p.id=i.player_id JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 WHERE i.is_active=1 ORDER BY i.injury_date DESC")
        .bind(uc).fetch_all(&pool).await.map_err(|e| e.to_string())
}
