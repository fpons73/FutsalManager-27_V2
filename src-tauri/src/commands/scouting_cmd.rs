use serde::Serialize;
use tauri::State;
use crate::commands::AppState;

#[derive(Serialize)]
pub struct ScoutingPlayer {
  pub player_id: i64,
  pub name: String,
  pub club_name: String,
  pub nation: String,
  pub flag_path: Option<String>,
  pub second_flag_path: Option<String>,
  pub secondary_position: Option<String>,
  pub position: String,
  pub knowledge: i64,
  pub ca_min: i64,
  pub ca_max: i64,
  pub pa_min: i64,
  pub pa_max: i64,
  pub recommendation: String,
}

#[derive(Serialize)]
pub struct ScoutingAttribute {
  pub name: String,
  pub value_min: i64,
  pub value_max: i64,
}

#[derive(Serialize)]
pub struct ScoutingReport {
  pub player: ScoutingPlayer,
  pub strengths: Vec<String>,
  pub weaknesses: Vec<String>,
  pub attributes: Vec<ScoutingAttribute>,
}

#[derive(Serialize)]
pub struct ScoutingState {
  pub knowledge_level: i64,
  pub max_scouts: i64,
  pub active_assignments: i64,
  pub players: Vec<ScoutingPlayer>,
  pub nations: Vec<(i64, String)>,
}

async fn pool(state: &State<'_, AppState>) -> Result<sqlx::SqlitePool, String> {
  state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida activa".into())
}

async fn user_club(db: &sqlx::SqlitePool) -> Result<i64, String> {
  sqlx::query_as::<_, (i64,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(db).await.map(|(id,)| id).map_err(|e| e.to_string())
}

fn ranges(value: i64, knowledge: i64) -> (i64, i64) {
  let spread = if knowledge >= 80 { 2 } else if knowledge >= 50 { 8 } else { 18 };
  ((value - spread).max(1), (value + spread).min(100))
}

fn recommendation(knowledge: i64, ca: i64, pa: i64) -> String {
  if knowledge >= 80 && pa - ca >= 15 { "Seguir de cerca".into() } else if knowledge >= 50 { "Seguir".into() } else { "Información insuficiente".into() }
}

#[tauri::command]
pub async fn get_scouting(state: State<'_, AppState>) -> Result<ScoutingState, String> {
  let db = pool(&state).await?; let club_id = user_club(&db).await?;
  let (knowledge_level, max_scouts): (i64, i64) = sqlx::query_as("SELECT knowledge_level + COALESCE((SELECT MAX(judging) / 5 FROM staff WHERE club_id=? AND role='scout'),0), max_scouts FROM scouting_centers WHERE club_id=?").bind(club_id).fetch_one(&db).await.map_err(|e| e.to_string())?;
  let (active_assignments,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM scout_assignments WHERE club_id=? AND is_active=1").bind(club_id).fetch_one(&db).await.map_err(|e| e.to_string())?;
  let rows = sqlx::query_as::<_, (i64,String,String,String,Option<String>,Option<String>,Option<String>,String,i64,i64,i64)>("SELECT p.id,p.common_name,c.name,n.name,n.flag_path,n2.flag_path,p.secondary_position,COALESCE(pos.position,'UNI'),pk.knowledge_percentage,ps.current_ability,ps.potential_ability FROM player_knowledge pk JOIN players p ON p.id=pk.player_id JOIN contracts ct ON ct.player_id=p.id AND ct.is_active=1 JOIN clubs c ON c.id=ct.club_id JOIN nations n ON n.id=p.nation_id LEFT JOIN nations n2 ON n2.id=p.second_nation_id JOIN player_states ps ON ps.player_id=p.id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END AS position FROM player_positions) pos ON pos.player_id=p.id WHERE pk.club_id=? ORDER BY pk.knowledge_percentage DESC, ps.current_ability DESC LIMIT 100").bind(club_id).fetch_all(&db).await.map_err(|e| e.to_string())?;
  let players = rows.into_iter().map(|(player_id,name,club_name,nation,flag_path,second_flag_path,secondary_position,position,knowledge,ca,pa)| { let (ca_min,ca_max)=if knowledge>=80{(ca,ca)}else{let(s,e)=ranges(ca,knowledge);(s,e)}; let (pa_min,pa_max)=if knowledge>=80{(pa,pa)}else{let(s,e)=ranges(pa,knowledge);(s,e)}; ScoutingPlayer { player_id,name,club_name,nation,flag_path,second_flag_path,secondary_position,position,knowledge,ca_min,ca_max,pa_min,pa_max,recommendation:recommendation(knowledge,ca,pa) } }).collect();
  let nations = sqlx::query_as::<_, (i64,String)>("SELECT id,name FROM nations ORDER BY name").fetch_all(&db).await.map_err(|e| e.to_string())?;
  Ok(ScoutingState { knowledge_level, max_scouts, active_assignments, players, nations })
}

#[tauri::command]
pub async fn get_scouting_report(state: State<'_, AppState>, player_id: i64) -> Result<ScoutingReport, String> {
  let db = pool(&state).await?; let club_id = user_club(&db).await?;
  let row = sqlx::query_as::<_, (String,String,String,String,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64)>("SELECT p.common_name,c.name,n.name,COALESCE(pos.position,'UNI'),pk.knowledge_percentage,ps.current_ability,ps.potential_ability,pa.passing,pa.finishing,pa.dribbling,pa.tackling,pa.vision,pa.stamina,pa.acceleration FROM player_knowledge pk JOIN players p ON p.id=pk.player_id JOIN contracts ct ON ct.player_id=p.id AND ct.is_active=1 JOIN clubs c ON c.id=ct.club_id JOIN nations n ON n.id=p.nation_id LEFT JOIN nations n2 ON n2.id=p.second_nation_id JOIN player_states ps ON ps.player_id=p.id JOIN player_attributes pa ON pa.player_id=p.id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END AS position FROM player_positions) pos ON pos.player_id=p.id WHERE pk.club_id=? AND p.id=?").bind(club_id).bind(player_id).fetch_optional(&db).await.map_err(|e| e.to_string())?.ok_or("Jugador aún no descubierto")?;
  let (name,club_name,nation,position,knowledge,ca,pa,passing,finishing,dribbling,tackling,vision,stamina,acceleration)=row;
  let flag_path: Option<String> = sqlx::query_as("SELECT n.flag_path FROM players p JOIN nations n ON n.id=p.nation_id WHERE p.id=?").bind(player_id).fetch_optional(&db).await.map_err(|e| e.to_string())?.and_then(|(v,): (Option<String>,)| v);
  let second_flag_path: Option<String> = sqlx::query_as("SELECT n.flag_path FROM players p LEFT JOIN nations n ON n.id=p.second_nation_id WHERE p.id=?").bind(player_id).fetch_optional(&db).await.map_err(|e| e.to_string())?.and_then(|(v,): (Option<String>,)| v);
  let secondary_position: Option<String> = sqlx::query_as("SELECT secondary_position FROM players WHERE id=?").bind(player_id).fetch_optional(&db).await.map_err(|e| e.to_string())?.and_then(|(v,): (Option<String>,)| v);
  let player=ScoutingPlayer { player_id,name,club_name,nation,flag_path,second_flag_path,secondary_position,position,knowledge,ca_min:if knowledge>=80{ca}else{ranges(ca,knowledge).0},ca_max:if knowledge>=80{ca}else{ranges(ca,knowledge).1},pa_min:if knowledge>=80{pa}else{ranges(pa,knowledge).0},pa_max:if knowledge>=80{pa}else{ranges(pa,knowledge).1},recommendation:recommendation(knowledge,ca,pa) };
  let vals=[("Pase",passing),("Finalización",finishing),("Regate",dribbling),("Defensa",tackling),("Visión",vision),("Resistencia",stamina),("Aceleración",acceleration)];
  let mut sorted=vals.to_vec(); sorted.sort_by_key(|(_,v)| -*v);
  let strengths=sorted.iter().take(3).map(|(n,_)|n.to_string()).collect(); let weaknesses=sorted.iter().rev().take(2).map(|(n,_)|n.to_string()).collect();
  let attributes=vals.iter().map(|(name,value)|{let(vmin,vmax)=if knowledge>=80{(*value,*value)}else{ranges(*value,knowledge)};ScoutingAttribute{name:(*name).into(),value_min:vmin,value_max:vmax}}).collect();
  Ok(ScoutingReport { player,strengths,weaknesses,attributes })
}

#[tauri::command]
pub async fn scout_nation(state: State<'_, AppState>, nation_id: i64) -> Result<String, String> {
  let db = pool(&state).await?; let club_id = user_club(&db).await?;
  let (max,): (i64,) = sqlx::query_as("SELECT max_scouts FROM scouting_centers WHERE club_id=?").bind(club_id).fetch_one(&db).await.map_err(|e| e.to_string())?;
  let (active,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM scout_assignments WHERE club_id=? AND is_active=1").bind(club_id).fetch_one(&db).await.map_err(|e| e.to_string())?;
  if active >= max { return Err("No hay scouts disponibles".into()); }
  let (name,): (String,) = sqlx::query_as("SELECT name FROM nations WHERE id=?").bind(nation_id).fetch_one(&db).await.map_err(|e| e.to_string())?;
  sqlx::query("INSERT OR IGNORE INTO scout_assignments(club_id,nation_id,target_name,knowledge_gain) VALUES(?,?,?,(3 + COALESCE((SELECT MAX(judging) / 10 FROM staff WHERE club_id=? AND role='scout'),0)))").bind(club_id).bind(nation_id).bind(&name).bind(club_id).execute(&db).await.map_err(|e| e.to_string())?;
  sqlx::query("INSERT OR IGNORE INTO player_knowledge(club_id,player_id,knowledge_percentage) SELECT ?,p.id,10 FROM players p WHERE p.nation_id=?").bind(club_id).bind(nation_id).execute(&db).await.map_err(|e| e.to_string())?;
  Ok(format!("Scout asignado a {name}"))
}
