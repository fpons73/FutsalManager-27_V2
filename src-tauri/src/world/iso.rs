use sqlx::SqlitePool;
use std::path::Path;

pub async fn import_catalog(pool: &SqlitePool) -> Result<i64, String> {
    let path = Path::new("Países e ISOS.csv");
    if !path.exists() { return Ok(0); }
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut imported = 0;
    for (idx, line) in text.lines().enumerate() {
        if idx == 0 { continue; }
        let mut parts = line.trim_start_matches('\u{feff}').split(';');
        let Some(name) = parts.next().map(str::trim) else { continue };
        let Some(iso2) = parts.next().map(str::trim).filter(|v| v.len() == 2) else { continue };
        let Some(iso3) = parts.next().map(str::trim).filter(|v| v.len() == 3) else { continue };
        let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM nations WHERE iso2=? OR name=?").bind(iso2).bind(name).fetch_optional(pool).await.map_err(|e| e.to_string())?;
        if let Some((id,)) = existing {
            sqlx::query("UPDATE nations SET iso2=COALESCE(iso2,?), iso3=COALESCE(iso3,?), flag_path=COALESCE(flag_path,?) WHERE id=?").bind(iso2).bind(iso3).bind(format!("/flags/{}.png", iso2.to_lowercase())).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
        } else {
            let conf: (i64,) = sqlx::query_as("SELECT id FROM confederations WHERE short_name='UEFA' LIMIT 1").fetch_one(pool).await.map_err(|e| e.to_string())?;
            sqlx::query("INSERT INTO nations(name,confederation_id,reputation,futsal_level,iso2,iso3,flag_path) VALUES(?,?,500,50,?,?,?)").bind(name).bind(conf.0).bind(iso2).bind(iso3).bind(format!("/flags/{}.png", iso2.to_lowercase())).execute(pool).await.map_err(|e| e.to_string())?;
            imported += 1;
        }
    }
    Ok(imported)
}
