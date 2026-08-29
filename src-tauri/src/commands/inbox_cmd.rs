use serde::Serialize;
use tauri::State;
use crate::commands::AppState;

#[derive(Serialize, Clone)]
pub struct InboxRow {
    pub id: i64,
    pub sender: String,
    pub subject: String,
    pub body: String,
    pub date: String,
    pub is_read: i64,
    pub is_important: i64,
}

#[tauri::command]
pub async fn get_inbox(state: State<'_, AppState>) -> Result<Vec<InboxRow>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    let rows: Vec<(i64, String, String, String, String, i64, i64)> = sqlx::query_as(
        "SELECT id, sender_type, subject, body, date_sent, is_read, is_important FROM inbox_messages WHERE club_id=? ORDER BY date_sent DESC, id DESC LIMIT 50"
    ).bind(uc).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    let mut out: Vec<InboxRow> = rows.into_iter().map(|(id, sender, subject, body, date, is_read, is_important)| InboxRow { id, sender, subject, body, date, is_read, is_important }).collect();
    let news: Vec<(i64,String,String,String,i64,i64)> = sqlx::query_as("SELECT id,news_type,headline,body,importance,is_read FROM world_news WHERE club_id=? ORDER BY date DESC,id DESC LIMIT 50").bind(uc).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    out.extend(news.into_iter().map(|(id,news_type,headline,body,importance,is_read)| InboxRow { id: -id, sender: news_type, subject: headline, body, date: String::new(), is_read, is_important: importance }));
    out.truncate(50);
    Ok(out)
}

#[tauri::command]
pub async fn mark_read(state: State<'_, AppState>, msg_id: i64) -> Result<(), String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    sqlx::query("UPDATE inbox_messages SET is_read=1 WHERE id=?").bind(msg_id).execute(&pool).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn mark_all_read(state: State<'_, AppState>) -> Result<(), String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    sqlx::query("UPDATE inbox_messages SET is_read=1 WHERE club_id=?").bind(uc).execute(&pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
