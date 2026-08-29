use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize, Clone)]
pub struct TrainingRow {
    pub day: i64,
    pub type_id: i64,
    pub type_name: String,
    pub category: String,
    pub intensity: i64,
}

#[derive(Serialize, Clone)]
pub struct StaffImpact { pub coach:i64, pub assistant:i64, pub scout:i64, pub youth:i64, pub physio:i64, pub training_bonus:i64, pub scouting_bonus:i64, pub injury_reduction:i64 }

#[derive(Serialize, Clone)]
pub struct ProgressRow {
    pub player_id: i64,
    pub name: String,
    pub position: String,
    pub ca: i64,
    pub pa: i64,
    pub age: i64,
    pub improvement: f64,
}

#[derive(Serialize, Clone)]
pub struct TacticalAutomation { pub id:i64, pub name:String, pub trigger_type:String, pub threshold:i64, pub formation:String, pub tempo:i64, pub pressing:i64, pub defensive_line:i64, pub width:i64, pub enabled:bool, pub training_level:i64 }

pub async fn get_automations(pool: &SqlitePool, club_id: i64) -> Result<Vec<TacticalAutomation>, String> {
    let rows: Vec<(i64,String,String,i64,String,i64,i64,i64,i64,i64,i64)> = sqlx::query_as("SELECT id,name,trigger_type,threshold,formation,tempo,pressing,defensive_line,width,enabled,training_level FROM tactical_automations WHERE club_id=? ORDER BY id").bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id,name,trigger_type,threshold,formation,tempo,pressing,defensive_line,width,enabled,training_level)| TacticalAutomation { id,name,trigger_type,threshold,formation,tempo,pressing,defensive_line,width,enabled:enabled != 0,training_level }).collect())
}

pub async fn set_automation(pool: &SqlitePool, club_id:i64, name:String, trigger_type:String, threshold:i64, formation:String, tempo:i64, pressing:i64, defensive_line:i64, width:i64, enabled:bool) -> Result<(), String> {
    if name.trim().is_empty() { return Err("El automatismo necesita un nombre".into()); }
    if !(0..=100).contains(&threshold) || !(0..=100).contains(&tempo) || !(0..=100).contains(&pressing) || !(0..=100).contains(&defensive_line) || !(0..=100).contains(&width) { return Err("Los valores tácticos deben estar entre 0 y 100".into()); }
    sqlx::query("INSERT INTO tactical_automations(club_id,name,trigger_type,threshold,formation,tempo,pressing,defensive_line,width,enabled,training_level) VALUES(?,?,?,?,?,?,?,?,?,?,0) ON CONFLICT(club_id,name) DO UPDATE SET trigger_type=excluded.trigger_type,threshold=excluded.threshold,formation=excluded.formation,tempo=excluded.tempo,pressing=excluded.pressing,defensive_line=excluded.defensive_line,width=excluded.width,enabled=excluded.enabled")
        .bind(club_id).bind(name).bind(trigger_type).bind(threshold).bind(formation).bind(tempo).bind(pressing).bind(defensive_line).bind(width).bind(enabled as i64).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_schedule(pool: &SqlitePool, club_id: i64) -> Result<Vec<TrainingRow>, String> {
    let rows = sqlx::query_as::<_, (i64, i64, String, String, i64)>(
        "SELECT ts.day_of_week, ts.training_type_id, tt.name, tt.category, ts.intensity FROM training_schedule ts JOIN training_types tt ON tt.id=ts.training_type_id WHERE ts.club_id=? ORDER BY ts.day_of_week"
    ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(day, type_id, type_name, category, intensity)| TrainingRow { day, type_id, type_name, category, intensity }).collect())
}

pub async fn set_schedule(pool: &SqlitePool, club_id: i64, schedule: Vec<(i64, i64, i64)>) -> Result<(), String> {
    sqlx::query("DELETE FROM training_schedule WHERE club_id=?").bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
    for (day, type_id, intensity) in schedule {
        sqlx::query("INSERT INTO training_schedule(club_id, day_of_week, training_type_id, intensity) VALUES(?,?,?,?)")
            .bind(club_id).bind(day).bind(type_id).bind(intensity).execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn ensure_default_schedule(pool: &SqlitePool, club_id: i64) -> Result<(), String> {
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM training_schedule WHERE club_id=?").bind(club_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if cnt == 0 {
        let defaults = vec![(0,1,70),(1,2,75),(2,4,65),(3,3,75),(4,7,60)];
        set_schedule(pool, club_id, defaults).await?;
    }
    Ok(())
}

pub async fn process_training_week(pool: &SqlitePool, club_id: i64) -> Result<Vec<String>, String> {
    // La cohesión crece con una semana completada; la química se aproxima a la media moral.
    sqlx::query("UPDATE club_dynamics SET cohesion=MIN(100, cohesion+1), chemistry=MIN(100, MAX(0, chemistry + CASE WHEN (SELECT AVG(morale) FROM player_states ps JOIN contracts c ON c.player_id=ps.player_id WHERE c.club_id=? AND c.is_active=1) > chemistry THEN 1 ELSE -1 END), updated_at=CURRENT_TIMESTAMP WHERE club_id=?").bind(club_id).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
    ensure_default_schedule(pool, club_id).await?;
    let schedule = get_schedule(pool, club_id).await?;
    if schedule.is_empty() { return Ok(vec![]); }

    let (today_s,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let today = chrono::NaiveDate::parse_from_str(&today_s, "%Y-%m-%d").map_err(|e| e.to_string())?;

    let players = sqlx::query_as::<_, (i64, String, i64, i64, String)>(
        "SELECT p.id, p.common_name, ps.current_ability, ps.potential_ability, p.date_of_birth FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_states ps ON ps.player_id=p.id"
    ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;

    let mut logs = Vec::new();
    for (pid, name, ca, pa, dob) in players {
        let dob_d = chrono::NaiveDate::parse_from_str(&dob, "%Y-%m-%d").unwrap_or(today);
        let age = ((today - dob_d).num_days()/365) as i64;
        let (prof,): (i64,) = sqlx::query_as("SELECT professionalism FROM player_attributes WHERE player_id=?").bind(pid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let staff: (i64,i64,i64,i64,i64,i64) = sqlx::query_as("SELECT COALESCE(MAX(tactical),0),COALESCE(MAX(working_youngsters),0),COALESCE(MAX(physio_level),0),COALESCE(MAX(man_management),0),COALESCE(MAX(judging),0),COALESCE(MAX(motivating),0) FROM staff WHERE club_id=?").bind(club_id).fetch_one(pool).await.map_err(|e| e.to_string())?;

        if ca >= pa { continue; }

        let age_factor = match age {
            15..=18 => 1.6,
            19..=23 => 1.3,
            24..=27 => 0.9,
            28..=31 => 0.4,
            _ => 0.1,
        };
        let gap = pa - ca;
        let pot_factor = if gap > 30 { 1.4 } else if gap > 15 { 1.15 } else { 0.7 };
        let prof_factor = prof as f64 / 12.0;
        let avg_intensity: f64 = schedule.iter().map(|s| s.intensity as f64).sum::<f64>() / schedule.len() as f64 / 80.0;

        let staff_factor = 0.75 + (staff.0.max(staff.1) as f64 / 20.0) * 0.5;
        let improvement = 0.12 * age_factor * pot_factor * prof_factor * avg_intensity * staff_factor;
        if improvement < 0.02 { continue; }

        let new_ca = ((ca as f64 + improvement).round() as i64).min(pa).min(200);
        if new_ca > ca {
            sqlx::query("UPDATE player_states SET current_ability=? WHERE player_id=?").bind(new_ca).bind(pid).execute(pool).await.map_err(|e| e.to_string())?;
            if new_ca - ca >= 1 {
                logs.push(format!("{} +{} CA ({}→{})", name, new_ca - ca, ca, new_ca));
            }
            // improve random attribute
            let attrs = ["passing","finishing","dribbling","vision","stamina","acceleration"];
            let attr = attrs[rand::random::<usize>() % attrs.len()];
            let q = format!("UPDATE player_attributes SET {} = MIN(20, {}+1) WHERE player_id=?", attr, attr);
            sqlx::query(&q).bind(pid).execute(pool).await.map_err(|e| e.to_string())?;
        }

        let injury_risk = (0.008 * (1.25 - staff.2 as f64 / 40.0)).clamp(0.001, 0.012);
        if rand::random::<f64>() < injury_risk {
            let types = ["Tobillo","Rodilla","Isquiotibial","Gemelo","Hombro"];
            let t = types[rand::random::<usize>() % types.len()];
            let days = rand::random::<u64>() % 21 + 7;
            let ret = today + chrono::Duration::days(days as i64);
            sqlx::query("INSERT INTO injuries(player_id, injury_type, severity, expected_return_date, injury_date, is_active) VALUES(?,?,?,?,?,1)")
                .bind(pid).bind(t).bind(40).bind(ret.format("%Y-%m-%d").to_string()).bind(today_s.clone()).execute(pool).await.map_err(|e| e.to_string())?;
            sqlx::query("INSERT INTO inbox_messages(club_id, sender_type, subject, body, date_sent, is_important) VALUES(?,'staff','Lesión: '||?, ?, ?,1)")
                .bind(club_id).bind(&name).bind(format!("{} se ha lesionado ({}) - {} días", name, t, days)).bind(&today_s).execute(pool).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(logs)
}

pub async fn get_progress(pool: &SqlitePool, club_id: i64) -> Result<Vec<ProgressRow>, String> {
    let (today_s,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let today = chrono::NaiveDate::parse_from_str(&today_s, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let rows = sqlx::query_as::<_, (i64, String, String, i64, i64, String)>(
        "SELECT p.id, p.common_name, COALESCE(pp.pos,'UNI'), ps.current_ability, ps.potential_ability, p.date_of_birth FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_states ps ON ps.player_id=p.id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp ON pp.player_id=p.id ORDER BY ps.current_ability DESC"
    ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name, pos, ca, pa, dob)| {
        let age = chrono::NaiveDate::parse_from_str(&dob, "%Y-%m-%d").map(|d| ((today - d).num_days()/365) as i64).unwrap_or(25);
        let gap = pa - ca;
        let improvement = if age < 23 { gap as f64 * 0.08 } else if age < 28 { gap as f64 * 0.03 } else { 0.0 };
        ProgressRow { player_id: id, name, position: pos, ca, pa, age, improvement }
    }).collect())
}
