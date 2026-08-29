use serde::Serialize;
use sqlx::SqlitePool;
use std::path::PathBuf;
use tauri::State;

use crate::commands::AppState;

#[derive(Serialize, Clone)]
pub struct SaveSlot {
    pub id: i64,
    pub name: String,
    pub is_autosave: bool,
    pub updated_at: String,
}

fn pool(state: &State<'_, AppState>) -> Result<SqlitePool, String> {
    state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or_else(|| "No hay partida activa".to_string())
}

fn slot_path(id: i64) -> PathBuf {
    crate::db::app_data_dir().join(format!("save-slot-{id}.db"))
}

async fn copy_database(pool: &SqlitePool, destination: &PathBuf) -> Result<(), String> {
    sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)").execute(pool).await.map_err(|e| e.to_string())?;
    if let Some(parent) = destination.parent() { std::fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
    let current = crate::db::db_path();
    let temp = destination.with_extension("db.tmp");
    std::fs::copy(&current, &temp).map_err(|e| e.to_string())?;
    std::fs::rename(&temp, destination).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn list_save_slots(state: State<'_, AppState>) -> Result<Vec<SaveSlot>, String> {
    let pool = pool(&state)?;
    let rows: Vec<(i64, String, String, i64)> = sqlx::query_as("SELECT id, name, updated_at, is_autosave FROM save_slots ORDER BY is_autosave DESC, updated_at DESC").fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name, updated_at, is_autosave)| SaveSlot { id, name, updated_at, is_autosave: is_autosave != 0 }).collect())
}

#[tauri::command]
pub async fn save_game(state: State<'_, AppState>, slot_id: i64, name: String) -> Result<SaveSlot, String> {
    let pool = pool(&state)?;
    if slot_id <= 0 { return Err("El slot debe ser mayor que cero".into()); }
    copy_database(&pool, &slot_path(slot_id)).await?;
    sqlx::query("INSERT INTO save_slots(id,name,file_path,is_autosave,updated_at) VALUES(?,?,?,0,CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET name=excluded.name,file_path=excluded.file_path,is_autosave=0,updated_at=CURRENT_TIMESTAMP")
        .bind(slot_id).bind(&name).bind(slot_path(slot_id).to_string_lossy().to_string()).execute(&pool).await.map_err(|e| e.to_string())?;
    let row: (i64, String, String, i64) = sqlx::query_as("SELECT id,name,updated_at,is_autosave FROM save_slots WHERE id=?").bind(slot_id).fetch_one(&pool).await.map_err(|e| e.to_string())?;
    Ok(SaveSlot { id:row.0, name:row.1, updated_at:row.2, is_autosave:row.3 != 0 })
}

#[tauri::command]
pub async fn load_game(state: State<'_, AppState>, slot_id: i64) -> Result<String, String> {
    let source = slot_path(slot_id);
    if !source.exists() { return Err("El slot no existe o está vacío".into()); }
    let current = crate::db::db_path();
    let backup = current.with_extension("db.preload.bak");
    let active = {
        let mut guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.take()
    };
    if let Some(active) = active { active.close().await; }
    if current.exists() { std::fs::copy(&current, &backup).map_err(|e| e.to_string())?; }
    let temp = current.with_extension("db.load.tmp");
    std::fs::copy(&source, &temp).map_err(|e| e.to_string())?;
    if let Err(e) = std::fs::rename(&temp, &current) {
        let _ = std::fs::remove_file(&temp);
        return Err(e.to_string());
    }
    let reopened = crate::db::init_pool(Some(current)).await.map_err(|e| e.to_string())?;
    *state.pool.lock().map_err(|e| e.to_string())? = Some(reopened);
    *state.live_match.lock().map_err(|e| e.to_string())? = None;
    *state.live_match_info.lock().map_err(|e| e.to_string())? = None;
    Ok(format!("Partida cargada desde el slot {slot_id}"))
}

#[tauri::command]
pub async fn backup_game(state: State<'_, AppState>, destination: String) -> Result<String, String> {
    let pool = pool(&state)?;
    let path = PathBuf::from(destination);
    if path.as_os_str().is_empty() { return Err("La ruta de backup está vacía".into()); }
    copy_database(&pool, &path).await?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn autosave_game(state: State<'_, AppState>) -> Result<SaveSlot, String> {
    let pool = pool(&state)?;
    let path = slot_path(0);
    copy_database(&pool, &path).await?;
    sqlx::query("INSERT INTO save_slots(id,name,file_path,is_autosave,updated_at) VALUES(0,'Autoguardado',?,1,CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET updated_at=CURRENT_TIMESTAMP,file_path=excluded.file_path")
        .bind(path.to_string_lossy().to_string()).execute(&pool).await.map_err(|e| e.to_string())?;
    let row: (i64, String, String, i64) = sqlx::query_as("SELECT id,name,updated_at,is_autosave FROM save_slots WHERE id=0").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    Ok(SaveSlot { id:row.0, name:row.1, updated_at:row.2, is_autosave: true })
}
