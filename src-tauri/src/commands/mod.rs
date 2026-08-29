pub mod contract_cmd;
pub mod editor_cmd;
pub mod finance_cmd;
pub mod game;
pub mod inbox_cmd;
pub mod match_live;
pub mod season_cmd;
pub mod scouting_cmd;
pub mod save_cmd;
pub mod squad_cmd;
pub mod national_cmd;
pub mod board_cmd;
pub mod training_cmd;
pub mod youth_cmd;
pub mod transfer_cmd;

pub async fn free_staff_for_nation(pool: &SqlitePool, nation_id: i64) -> Result<Vec<(i64, String, String, i64, i64)>, String> {
  sqlx::query_as("SELECT id, common_name, role, COALESCE(tactical,0), COALESCE(motivating,0) FROM staff WHERE club_id IS NULL AND nation_id=? ORDER BY judging DESC, id LIMIT 50")
    .bind(nation_id).fetch_all(pool).await.map_err(|e| e.to_string())
}

use std::sync::Mutex;
use serde::Serialize;
use sqlx::SqlitePool;
use crate::engine::MatchEngine;

#[derive(Clone, serde::Serialize)]
pub struct LiveMatchInfo {
  pub match_id: i64,
  pub home_club_id: i64,
  pub away_club_id: i64,
}

pub struct AppState {
  pub pool: Mutex<Option<SqlitePool>>,
  pub live_match: Mutex<Option<MatchEngine>>,
  pub live_match_info: Mutex<Option<LiveMatchInfo>>,
}

impl Default for AppState {
  fn default() -> Self { Self { pool: Mutex::new(None), live_match: Mutex::new(None), live_match_info: Mutex::new(None) } }
}

#[derive(Serialize)]
pub struct AppInfo {
  name: String,
  version: String,
}

#[derive(Serialize)]
pub struct DbStatus {
  ok: bool,
  tables: i64,
  message: String,
}

#[tauri::command]
pub fn ping() -> String {
  "pong".to_string()
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
  AppInfo {
    name: "Futsal Manager 27".to_string(),
    version: "0.1.0".to_string(),
  }
}

#[tauri::command]
pub async fn test_db() -> Result<DbStatus, String> {
  let pool = crate::db::init_memory_pool()
    .await
    .map_err(|e| e.to_string())?;
  let (count,): (i64,) =
    sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE '_sqlx%'")
      .fetch_one(&pool)
      .await
      .map_err(|e| e.to_string())?;
  Ok(DbStatus {
    ok: true,
    tables: count,
    message: format!("{count} tablas creadas, WAL + FK activos"),
  })
}

#[derive(Serialize)]
pub struct WorldSeedResult {
  clubs: i64,
  players: i64,
  competitions: i64,
  message: String,
}

#[tauri::command]
pub async fn seed_world_cmd() -> Result<WorldSeedResult, String> {
  let pool = crate::db::init_memory_pool().await.map_err(|e| e.to_string())?;
  crate::world::seed_world(&pool).await?;
  let (clubs,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs").fetch_one(&pool).await.map_err(|e| e.to_string())?;
  let (players,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM players").fetch_one(&pool).await.map_err(|e| e.to_string())?;
  let (comps,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM competitions").fetch_one(&pool).await.map_err(|e| e.to_string())?;
  Ok(WorldSeedResult { clubs, players, competitions: comps, message: format!("Mundo generado: {clubs} clubes, {players} jugadores") })
}
