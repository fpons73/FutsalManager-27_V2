use chrono::NaiveDate;
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize, Clone)]
pub struct YouthPlayerRow {
    pub id: i64, pub team_id: i64, pub team_name: String, pub age_group: i64,
    pub name: String, pub nation: String, pub flag_path: Option<String>,
    pub second_flag_path: Option<String>, pub position: String, pub secondary_position: Option<String>,
    pub ca: i64, pub pa: i64, pub development: f64, pub can_promote: bool,
}

pub async fn ensure_teams(pool: &SqlitePool, club_id: i64) -> Result<(), String> {
    let groups = [12_i64, 14, 16, 18, 20];
    for age in groups {
        sqlx::query("INSERT OR IGNORE INTO youth_teams(club_id,age_group,name) VALUES(?,?,?)")
            .bind(club_id).bind(age).bind(format!("U{age}")).execute(pool).await.map_err(|e| e.to_string())?;
    }
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM youth_players yp JOIN youth_teams yt ON yt.id=yp.youth_team_id WHERE yt.club_id=?").bind(club_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if count == 0 { generate_intake(pool, club_id).await?; }
    Ok(())
}

async fn generate_intake(pool: &SqlitePool, club_id: i64) -> Result<(), String> {
    let (date,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let groups = [12_i64, 14, 16, 18, 20];
    let names = [("Alex","Garcia"),("Leo","Santos"),("Hugo","Costa"),("Dani","Silva"),("Iker","Lopez"),("Marc","Martins")];
    for (idx, age) in groups.iter().enumerate() {
        let (team_id,): (i64,) = sqlx::query_as("SELECT id FROM youth_teams WHERE club_id=? AND age_group=?").bind(club_id).bind(age).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let (nation_id,): (i64,) = sqlx::query_as("SELECT nation_id FROM clubs WHERE id=?").bind(club_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
        for slot in 0..4 {
            let (first,last) = names[(idx + slot) % names.len()];
            let dob = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| e.to_string())? - chrono::Duration::days(*age * 365 + (slot as i64 * 40));
            let pos = ["POR","CIE","ALA","PIV"][(idx+slot)%4];
            let secondary = ["ALA","CIE","PIV","UNI"][(idx+slot+1)%4];
            let ca = 20 + (*age / 2) + slot as i64;
            let pa = (ca + 35 + (slot as i64 * 4)).min(180);
            sqlx::query("INSERT INTO youth_players(youth_team_id,first_name,last_name,common_name,date_of_birth,nation_id,position,secondary_position,current_ability,potential_ability,created_date) VALUES(?,?,?,?,?,?,?,?,?,?,?)")
                .bind(team_id).bind(first).bind(last).bind(format!("{first} {last}"))
                .bind(dob.format("%Y-%m-%d").to_string()).bind(nation_id).bind(pos).bind(secondary).bind(ca).bind(pa).bind(&date)
                .execute(pool).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub async fn list(pool: &SqlitePool, club_id: i64) -> Result<Vec<YouthPlayerRow>, String> {
    ensure_teams(pool, club_id).await?;
    let rows = sqlx::query_as::<_, (i64,i64,String,i64,String,String,Option<String>,Option<String>,String,Option<String>,i64,i64,f64)>("SELECT yp.id,yt.id,yt.name,yt.age_group,yp.common_name,n.name,n.flag_path,n2.flag_path,yp.position,yp.secondary_position,yp.current_ability,yp.potential_ability,yp.development FROM youth_players yp JOIN youth_teams yt ON yt.id=yp.youth_team_id JOIN nations n ON n.id=yp.nation_id LEFT JOIN nations n2 ON n2.id=yp.second_nation_id WHERE yt.club_id=? AND yp.promoted_to_first_team=0 ORDER BY yt.age_group, yp.current_ability DESC").bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id,team_id,team_name,age_group,name,nation,flag_path,second_flag_path,position,secondary_position,ca,pa,development)| YouthPlayerRow { id,team_id,team_name,age_group,name,nation,flag_path,second_flag_path,position,secondary_position,ca,pa,development,can_promote: age_group >= 18 && ca >= 45 }).collect())
}

pub fn facility_youth_gain(level: i64, age: i64) -> i64 {
    let base = if age <= 16 { 2 } else { 1 };
    base + (level.clamp(1, 5) - 1) / 2
}

pub async fn develop(pool: &SqlitePool, club_id: i64) -> Result<(), String> {
    let (level,): (i64,) = sqlx::query_as("SELECT COALESCE(youth_level,1) FROM club_facilities WHERE club_id=?")
        .bind(club_id).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((1,));
    let rows: Vec<(i64,i64,i64,i64)> = sqlx::query_as("SELECT yp.id,yp.current_ability,yp.potential_ability,yt.age_group FROM youth_players yp JOIN youth_teams yt ON yt.id=yp.youth_team_id WHERE yt.club_id=? AND yp.promoted_to_first_team=0").bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (id,_ca,_pa,age) in rows {
        let gain = facility_youth_gain(level, age);
        sqlx::query("UPDATE youth_players SET current_ability=MIN(potential_ability,current_ability+?), development=development+? WHERE id=?").bind(gain).bind(gain as f64).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn academy_level_increases_daily_development() {
        assert_eq!(facility_youth_gain(1, 16), 2);
        assert_eq!(facility_youth_gain(3, 16), 3);
        assert_eq!(facility_youth_gain(5, 18), 3);
    }
}

pub async fn promote(pool: &SqlitePool, club_id: i64, youth_id: i64) -> Result<String, String> {
    let row: (String,String,i64,i64,i64) = sqlx::query_as("SELECT yp.first_name,yp.last_name,yp.current_ability,yp.potential_ability,yt.age_group FROM youth_players yp JOIN youth_teams yt ON yt.id=yp.youth_team_id WHERE yp.id=? AND yt.club_id=? AND yp.promoted_to_first_team=0").bind(youth_id).bind(club_id).fetch_one(pool).await.map_err(|_| "Juvenil no encontrado".to_string())?;
    if row.4 < 18 || row.2 < 45 { return Err("El jugador debe tener al menos 18 años y CA 45".into()); }
    let (nation_id,): (i64,) = sqlx::query_as("SELECT nation_id FROM youth_players WHERE id=?").bind(youth_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (date,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let _dob: String = sqlx::query_as::<_,(String,)>("SELECT date_of_birth FROM youth_players WHERE id=?").bind(youth_id).fetch_one(pool).await.map_err(|e| e.to_string())?.0;
    let (pid,): (i64,) = sqlx::query_as("INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,height_cm,weight_kg) SELECT first_name,last_name,common_name,date_of_birth,nation_id,175,72 FROM youth_players WHERE id=? RETURNING id").bind(youth_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO player_positions(player_id,uni_natural) VALUES(?,20)").bind(pid).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO player_states(player_id,current_ability,potential_ability) VALUES(?,?,?)").bind(pid).bind(row.2).bind(row.3).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO player_attributes(player_id) VALUES(?)").bind(pid).execute(pool).await.map_err(|e| e.to_string())?;
    let end = NaiveDate::parse_from_str(&date,"%Y-%m-%d").map_err(|e| e.to_string())? + chrono::Duration::days(365*3);
    let (cid,): (i64,) = sqlx::query_as("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1) RETURNING id").bind(pid).bind(club_id).bind(400.0).bind(&date).bind(end.format("%Y-%m-%d").to_string()).fetch_one(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE youth_players SET promoted_to_first_team=1 WHERE id=?").bind(youth_id).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO youth_promotions(youth_player_id,club_id,promotion_date,contract_id) VALUES(?,?,?,?)").bind(youth_id).bind(club_id).bind(date).bind(cid).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(format!("{} {} promocionado al primer equipo", row.0,row.1))
}
