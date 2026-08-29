use serde::Serialize;
use tauri::State;
use crate::commands::AppState;

#[derive(Serialize, sqlx::FromRow)]
pub struct NationalPlayerRow { pub player_id:i64, pub player_name:String, pub nation_id:i64, pub nation_name:String, pub flag_path:Option<String>, pub club_name:Option<String>, pub position:String, pub selected:i64, pub callup_status:String }
#[derive(Serialize, sqlx::FromRow)]
pub struct NationalManagerRow { pub nation_id:i64, pub staff_id:i64, pub staff_name:String, pub role:String, pub tactical:i64, pub motivating:i64 }
#[derive(Serialize, sqlx::FromRow)]
pub struct InternationalFixtureRow { pub id:i64, pub date:String, pub home_nation_id:i64, pub home_nation:String, pub home_flag_path:Option<String>, pub away_nation_id:i64, pub away_nation:String, pub away_flag_path:Option<String>, pub status:String, pub home_score:i64, pub away_score:i64 }
#[derive(Serialize, sqlx::FromRow)]
pub struct NationalStandingRow { pub competition_id:i64, pub competition_name:String, pub nation_id:i64, pub nation_name:String, pub flag_path:Option<String>, pub position:i64, pub played:i64, pub won:i64, pub drawn:i64, pub lost:i64, pub goals_for:i64, pub goals_against:i64, pub points:i64 }
#[derive(Serialize, sqlx::FromRow)]
pub struct NationalHonourRow { pub season:String, pub competition_id:i64, pub competition_name:String, pub nation_id:i64, pub nation_name:String, pub flag_path:Option<String>, pub honour_type:String }

async fn get_pool(state: &State<'_, AppState>) -> Result<sqlx::SqlitePool, String> { state.pool.lock().map_err(|e|e.to_string())?.clone().ok_or("No hay partida".into()) }

#[tauri::command]
pub async fn get_national_players(state: State<'_, AppState>, nation_id:i64) -> Result<Vec<NationalPlayerRow>, String> { let pool=get_pool(&state).await?; sqlx::query_as::<_,NationalPlayerRow>("SELECT p.id,p.common_name,?,n.name,n.flag_path,c.name,COALESCE(pp.pos,'UNI'),COALESCE(ns.selected,0),COALESCE(ns.callup_status,'eligible') FROM players p JOIN nations n ON n.id=? LEFT JOIN contracts ct ON ct.player_id=p.id AND ct.is_active=1 LEFT JOIN clubs c ON c.id=ct.club_id LEFT JOIN national_team_squads ns ON ns.player_id=p.id AND ns.nation_id=? LEFT JOIN (SELECT player_id,CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END pos FROM player_positions) pp ON pp.player_id=p.id WHERE p.is_retired=0 AND (p.nation_id=? OR p.second_nation_id=?) ORDER BY selected DESC,p.common_name").bind(nation_id).bind(nation_id).bind(nation_id).bind(nation_id).bind(nation_id).fetch_all(&pool).await.map_err(|e|e.to_string()) }

#[tauri::command]
pub async fn set_national_callup(state: State<'_, AppState>, nation_id:i64, player_id:i64, selected:bool) -> Result<(), String> { let pool=get_pool(&state).await?; let eligible:(i64,)=sqlx::query_as("SELECT COUNT(*) FROM players WHERE id=? AND (nation_id=? OR second_nation_id=?) AND is_retired=0").bind(player_id).bind(nation_id).bind(nation_id).fetch_one(&pool).await.map_err(|e|e.to_string())?; if eligible.0==0{return Err("Jugador no elegible para esta selección".into())} let (selected_count,):(i64,)=sqlx::query_as("SELECT COUNT(*) FROM national_team_squads WHERE nation_id=? AND selected=1 AND player_id<>?").bind(nation_id).bind(player_id).fetch_one(&pool).await.map_err(|e|e.to_string())?; if selected&&selected_count>=14{return Err("Una convocatoria no puede superar los 14 jugadores".into())} sqlx::query("INSERT INTO national_team_squads(nation_id,player_id,selected,callup_status) VALUES(?,?,?,?) ON CONFLICT(nation_id,player_id) DO UPDATE SET selected=excluded.selected,callup_status=excluded.callup_status").bind(nation_id).bind(player_id).bind(selected as i64).bind(if selected{"called_up"}else{"eligible"}).execute(&pool).await.map_err(|e|e.to_string())?; Ok(()) }

#[tauri::command]
pub async fn get_free_national_staff(state: State<'_, AppState>, nation_id:i64)->Result<Vec<(i64,String,String,i64,i64)>,String>{let pool=get_pool(&state).await?;crate::commands::free_staff_for_nation(&pool,nation_id).await}
#[tauri::command]
pub async fn get_national_manager(state: State<'_, AppState>, nation_id:i64)->Result<Option<NationalManagerRow>,String>{let pool=get_pool(&state).await?;sqlx::query_as::<_,NationalManagerRow>("SELECT nts.nation_id,s.id,s.common_name,nts.role,COALESCE(s.tactical,0),COALESCE(s.motivating,0) FROM national_team_staff nts JOIN staff s ON s.id=nts.staff_id WHERE nts.nation_id=? AND nts.active=1").bind(nation_id).fetch_optional(&pool).await.map_err(|e|e.to_string())}
#[tauri::command]
pub async fn assign_national_manager(state: State<'_, AppState>, nation_id:i64, staff_id:i64)->Result<(),String>{let pool=get_pool(&state).await?;let valid:(i64,)=sqlx::query_as("SELECT COUNT(*) FROM staff WHERE id=? AND (club_id IS NULL OR club_id=0)").bind(staff_id).fetch_one(&pool).await.map_err(|e|e.to_string())?;if valid.0==0{return Err("El seleccionador debe estar libre".into())}sqlx::query("INSERT INTO national_team_staff(nation_id,staff_id,role,active) VALUES(?,?, 'manager',1) ON CONFLICT(nation_id,role) DO UPDATE SET staff_id=excluded.staff_id,active=1").bind(nation_id).bind(staff_id).execute(&pool).await.map_err(|e|e.to_string())?;Ok(())}
#[tauri::command]
pub async fn get_national_standings(state: State<'_, AppState>, competition_id:i64) -> Result<Vec<NationalStandingRow>, String> {
    let pool = get_pool(&state).await?;
    let mut rows = sqlx::query_as::<_, NationalStandingRow>("SELECT e.competition_id,c.name,e.nation_id,n.name,n.flag_path,e.position,e.played,e.won,e.drawn,e.lost,e.goals_for,e.goals_against,e.points FROM national_tournament_entries e JOIN competitions c ON c.id=e.competition_id JOIN nations n ON n.id=e.nation_id WHERE e.competition_id=? AND e.season=(SELECT season FROM game_state WHERE id=1) ORDER BY e.points DESC,(e.goals_for-e.goals_against) DESC,e.goals_for DESC,e.nation_id").bind(competition_id).fetch_all(&pool).await.map_err(|e|e.to_string())?;
    for (idx, row) in rows.iter().enumerate() { sqlx::query("UPDATE national_tournament_entries SET position=? WHERE competition_id=? AND season=(SELECT season FROM game_state WHERE id=1) AND nation_id=?").bind((idx+1) as i64).bind(competition_id).bind(row.nation_id).execute(&pool).await.map_err(|e|e.to_string())?; }
    rows.iter_mut().enumerate().for_each(|(idx,row)| row.position=(idx+1) as i64);
    Ok(rows)
}

#[tauri::command]
pub async fn get_national_honours(state: State<'_, AppState>, nation_id:i64)->Result<Vec<NationalHonourRow>,String>{let pool=get_pool(&state).await?;sqlx::query_as::<_,NationalHonourRow>("SELECT h.season,h.competition_id,c.name,h.nation_id,n.name,n.flag_path,h.honour_type FROM national_tournament_honours h JOIN competitions c ON c.id=h.competition_id JOIN nations n ON n.id=h.nation_id WHERE h.nation_id=? ORDER BY h.season DESC").bind(nation_id).fetch_all(&pool).await.map_err(|e|e.to_string())}

#[tauri::command]
pub async fn get_international_fixtures(state: State<'_, AppState>, nation_id:i64)->Result<Vec<InternationalFixtureRow>,String>{let pool=get_pool(&state).await?;sqlx::query_as::<_,InternationalFixtureRow>("SELECT im.id,im.date,im.home_nation_id,hn.name,hn.flag_path,im.away_nation_id,an.name,an.flag_path,im.status,im.home_score,im.away_score FROM international_matches im JOIN nations hn ON hn.id=im.home_nation_id JOIN nations an ON an.id=im.away_nation_id WHERE im.home_nation_id=? OR im.away_nation_id=? ORDER BY im.date").bind(nation_id).bind(nation_id).fetch_all(&pool).await.map_err(|e|e.to_string())}

pub async fn generate_international_windows(pool:&sqlx::SqlitePool)->Result<(),String>{let(season,):(String,)=sqlx::query_as("SELECT season FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e|e.to_string())?;let year:i32=season.split('/').next().and_then(|v|v.parse().ok()).unwrap_or(2026);for(start,end,kind)in[(format!("{}-09-07",year),format!("{}-09-15",year),"friendly"),(format!("{}-10-05",year),format!("{}-10-13",year),"qualifier"),(format!("{}-11-09",year),format!("{}-11-17",year),"friendly"),(format!("{}-03-22",year+1),format!("{}-03-30",year+1),"qualifier")]{sqlx::query("INSERT OR IGNORE INTO international_windows(season,start_date,end_date,window_type) VALUES(?,?,?,?)").bind(&season).bind(start).bind(end).bind(kind).execute(pool).await.map_err(|e|e.to_string())?;}let(count,):(i64,)=sqlx::query_as("SELECT COUNT(*) FROM international_matches WHERE season=?").bind(&season).fetch_one(pool).await.map_err(|e|e.to_string())?;    if count==0{let nations:Vec<(i64,)>=sqlx::query_as("SELECT id FROM nations WHERE futsal_level>=60 ORDER BY futsal_level DESC,id LIMIT 16").fetch_all(pool).await.map_err(|e|e.to_string())?;let date=format!("{}-09-10",year);let comp:Option<(i64,)>=sqlx::query_as("SELECT id FROM competitions WHERE kind='national_team' ORDER BY id LIMIT 1").fetch_optional(pool).await.map_err(|e|e.to_string())?;for pair in nations.chunks(2){if pair.len()==2{let(mid,):(i64,)=sqlx::query_as("INSERT INTO international_matches(competition_id,season,date,home_nation_id,away_nation_id,status,phase,round) VALUES(?,?,?,?,?,'scheduled','window',0) RETURNING id").bind(comp.map(|v|v.0)).bind(&season).bind(&date).bind(pair[0].0).bind(pair[1].0).fetch_one(pool).await.map_err(|e|e.to_string())?;let(wid,):(i64,)=sqlx::query_as("SELECT id FROM international_windows WHERE season=? AND start_date<=? AND end_date>=? LIMIT 1").bind(&season).bind(&date).bind(&date).fetch_one(pool).await.map_err(|e|e.to_string())?;sqlx::query("INSERT INTO national_fixtures(international_match_id,window_id) VALUES(?,?)").bind(mid).bind(wid).execute(pool).await.map_err(|e|e.to_string())?;}}}    Ok(())}

pub async fn generate_national_tournament_matches(pool: &sqlx::SqlitePool) -> Result<(), String> {
    let (season,): (String,) = sqlx::query_as("SELECT season FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let comps: Vec<(i64, i64)> = sqlx::query_as("SELECT id,total_teams FROM competitions WHERE kind='national_team'").fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (comp, total) in comps {
        let (existing,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM international_matches WHERE competition_id=? AND season=? AND phase='group'").bind(comp).bind(&season).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if existing > 0 { continue; }
        let entries: Vec<(i64,)> = sqlx::query_as("SELECT nation_id FROM national_tournament_entries WHERE competition_id=? AND season=? ORDER BY nation_id LIMIT ?").bind(comp).bind(&season).bind(total).fetch_all(pool).await.map_err(|e| e.to_string())?;
        let date = format!("{}-01-10", season.split('/').next().and_then(|v|v.parse::<i32>().ok()).unwrap_or(2026)+1);
        for pair in entries.chunks(2) { if pair.len()==2 { sqlx::query("INSERT INTO international_matches(competition_id,season,date,home_nation_id,away_nation_id,status,phase,round,group_code) VALUES(?,?,?,?,?,'scheduled','group',1,'A')").bind(comp).bind(&season).bind(&date).bind(pair[0].0).bind(pair[1].0).execute(pool).await.map_err(|e| e.to_string())?; } }
    }
    Ok(())
}

pub async fn ensure_national_tournament_entries(pool: &sqlx::SqlitePool) -> Result<(), String> {
    let (season,): (String,) = sqlx::query_as("SELECT season FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let competitions: Vec<(i64, i64)> = sqlx::query_as("SELECT id,total_teams FROM competitions WHERE kind='national_team'").fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (competition_id, total) in competitions {
        let (existing,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM national_tournament_entries WHERE competition_id=? AND season=?").bind(competition_id).bind(&season).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if existing > 0 { continue; }
        let nations: Vec<(i64,)> = sqlx::query_as("SELECT id FROM nations ORDER BY futsal_level DESC,id LIMIT ?").bind(total).fetch_all(pool).await.map_err(|e| e.to_string())?;
        for (index, (nation_id,)) in nations.into_iter().enumerate() {
            sqlx::query("INSERT INTO national_tournament_entries(competition_id,season,nation_id,position) VALUES(?,?,?,?)").bind(competition_id).bind(&season).bind(nation_id).bind((index+1) as i64).execute(pool).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
 use super::*;
 use crate::{db,world};
 #[tokio::test]
 async fn international_windows_are_idempotent(){let pool=db::init_memory_pool().await.unwrap();world::seed_world(&pool).await.unwrap();generate_international_windows(&pool).await.unwrap();generate_international_windows(&pool).await.unwrap();let(w,):(i64,)=sqlx::query_as("SELECT COUNT(*) FROM international_windows").fetch_one(&pool).await.unwrap();let(m,):(i64,)=sqlx::query_as("SELECT COUNT(*) FROM international_matches").fetch_one(&pool).await.unwrap();assert_eq!(w,4);assert_eq!(m,8);}
 #[tokio::test]
 async fn international_match_updates_table() {
  let pool=db::init_memory_pool().await.unwrap();
  world::seed_world(&pool).await.unwrap();
  ensure_national_tournament_entries(&pool).await.unwrap();
  let (comp,): (i64,) = sqlx::query_as("SELECT id FROM competitions WHERE kind='national_team' LIMIT 1").fetch_one(&pool).await.unwrap();
  let (home,): (i64,) = sqlx::query_as("SELECT nation_id FROM national_tournament_entries WHERE competition_id=? LIMIT 1").bind(comp).fetch_one(&pool).await.unwrap();
  let (away,): (i64,) = sqlx::query_as("SELECT nation_id FROM national_tournament_entries WHERE competition_id=? AND nation_id<>? LIMIT 1").bind(comp).bind(home).fetch_one(&pool).await.unwrap();
  sqlx::query("INSERT INTO international_matches(competition_id,season,date,home_nation_id,away_nation_id,status,phase,round) VALUES(?,?,?, ?,?,'scheduled','group',1)").bind(comp).bind("2026/2027").bind("2026-07-10").bind(home).bind(away).execute(&pool).await.unwrap();
  let (before,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM international_matches WHERE status='finished'").fetch_one(&pool).await.unwrap();
  assert_eq!(before, 0);
  let (entries,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM national_tournament_entries WHERE competition_id=?").bind(comp).fetch_one(&pool).await.unwrap();
  assert!(entries >= 2);
 }

 #[tokio::test]
 async fn callup_limit_and_dual_nationality(){let pool=db::init_memory_pool().await.unwrap();world::seed_world(&pool).await.unwrap();let(a, b):(i64,i64)=sqlx::query_as("SELECT id, second_nation_id FROM players WHERE second_nation_id IS NOT NULL LIMIT 1").fetch_one(&pool).await.unwrap();let nation:(i64,)=sqlx::query_as("SELECT second_nation_id FROM players WHERE id=?").bind(a).fetch_one(&pool).await.unwrap();let result=sqlx::query("INSERT INTO national_team_squads(nation_id,player_id,selected,callup_status) VALUES(?,?,1,'called_up')").bind(nation.0).bind(a).execute(&pool).await;assert!(result.is_ok());for i in 0..13{let(pid,):(i64,)=sqlx::query_as("SELECT id FROM players WHERE id<>? LIMIT 1 OFFSET ?").bind(a).bind(i).fetch_one(&pool).await.unwrap();let _=sqlx::query("INSERT OR IGNORE INTO national_team_squads(nation_id,player_id,selected,callup_status) VALUES(?,?,1,'called_up')").bind(nation.0).bind(pid).execute(&pool).await;}let(count,):(i64,)=sqlx::query_as("SELECT COUNT(*) FROM national_team_squads WHERE nation_id=? AND selected=1").bind(nation.0).fetch_one(&pool).await.unwrap();assert!(count>=1);let _=b;}
}
