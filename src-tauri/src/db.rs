use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::PathBuf;
use std::sync::OnceLock;

#[allow(dead_code)]
static POOL: OnceLock<SqlitePool> = OnceLock::new();

pub fn db_path() -> PathBuf {
    if let Some(dir) = dirs_next() {
        dir.join("futsal-manager-27.db")
    } else {
        PathBuf::from("futsal-manager-27.db")
    }
}

pub fn app_data_dir() -> PathBuf {
    dirs_next().unwrap_or_else(|| PathBuf::from("."))
}

fn dirs_next() -> Option<PathBuf> {
    if let Some(base) = std::env::var_os("APPDATA") {
        return Some(PathBuf::from(base).join("FutsalManager27"));
    }
    None
}

pub async fn init_pool(path: Option<PathBuf>) -> Result<SqlitePool, sqlx::Error> {
    let db_file = path.unwrap_or_else(db_path);
    if let Some(parent) = db_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let url = format!("sqlite:{}?mode=rwc", db_file.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    sqlx::query("PRAGMA journal_mode=WAL").execute(&pool).await?;
    sqlx::query("PRAGMA foreign_keys=ON").execute(&pool).await?;
    ensure_legacy_schema(&pool).await?;
    sqlx::migrate!("./migrations").run(&pool).await.map_err(|e| {
        eprintln!("migration error: {e}");
        sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    Ok(pool)
}

pub async fn init_memory_pool() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;
    sqlx::query("PRAGMA foreign_keys=ON").execute(&pool).await?;
    ensure_legacy_schema(&pool).await?;
    sqlx::migrate!("./migrations").run(&pool).await.map_err(|e| {
        sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    Ok(pool)
}

async fn ensure_legacy_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let (exists,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='competitions'").fetch_one(pool).await?;
    if exists == 0 { return Ok(()); }
    let (has_kind,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pragma_table_info('competitions') WHERE name='kind'").fetch_one(pool).await?;
    if has_kind == 0 {
        sqlx::query("ALTER TABLE competitions ADD COLUMN kind TEXT NOT NULL DEFAULT 'club'").execute(pool).await?;
    }
    Ok(())
}

pub fn set_pool(pool: SqlitePool) {
    let _ = POOL.set(pool);
}

pub fn get_pool() -> Option<SqlitePool> {
    POOL.get().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migration_creates_tables() {
        let pool = init_memory_pool().await.expect("pool");
        let (count,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE '_sqlx%'")
                .fetch_one(&pool)
                .await
                .expect("count");
        assert!(count >= 20, "expected >=20 tables, got {count}");
        let (fk,): (i64,) = sqlx::query_as("PRAGMA foreign_keys")
            .fetch_one(&pool)
            .await
            .expect("fk pragma");
        assert_eq!(fk, 1);
    }

    #[tokio::test]
    async fn debug_game_state_insert() {
        let pool = init_memory_pool().await.unwrap();
        sqlx::query("INSERT INTO game_state(id, game_date, season, game_speed) VALUES(1,'2026-07-10','2026/2027','normal')")
            .execute(&pool).await.unwrap();
        let (d,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&pool).await.unwrap();
        assert_eq!(d, "2026-07-10");
    }
}
