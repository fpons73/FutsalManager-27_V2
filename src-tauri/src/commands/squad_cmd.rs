use serde::Serialize;
use tauri::State;
use crate::commands::AppState;

#[derive(Serialize)]
pub struct SquadDynamics { pub chemistry:i64, pub cohesion:i64, pub captain_id:Option<i64>, pub vice_captain_id:Option<i64> }
#[derive(Serialize)]
pub struct PromiseRow { pub id:i64, pub player_id:i64, pub promise_type:String, pub target_value:i64, pub status:String }
#[derive(Serialize)]
pub struct DialogueRow { pub id:i64, pub player_id:i64, pub topic:String, pub response:String, pub morale_delta:i64, pub created_at:String }

async fn pool(state:&State<'_,AppState>)->Result<sqlx::SqlitePool,String>{state.pool.lock().map_err(|e|e.to_string())?.clone().ok_or("No hay partida".into())}
async fn club(pool:&sqlx::SqlitePool)->Result<i64,String>{sqlx::query_as::<_,(Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e|e.to_string())?.0.ok_or("Sin club".into())}

#[tauri::command]
pub async fn get_squad_dynamics(state:State<'_,AppState>)->Result<SquadDynamics,String>{let p=pool(&state).await?;let c=club(&p).await?;let (chem,coh): (i64,i64)=sqlx::query_as("SELECT chemistry,cohesion FROM club_dynamics WHERE club_id=?").bind(c).fetch_one(&p).await.map_err(|e|e.to_string())?;let row: (Option<i64>,Option<i64>)=sqlx::query_as("SELECT captain_player_id,vice_captain_player_id FROM club_leadership WHERE club_id=?").bind(c).fetch_one(&p).await.map_err(|e|e.to_string())?;Ok(SquadDynamics{chemistry:chem,cohesion:coh,captain_id:row.0,vice_captain_id:row.1})}

#[tauri::command]
pub async fn get_player_promises(state:State<'_,AppState>)->Result<Vec<PromiseRow>,String>{let p=pool(&state).await?;let c=club(&p).await?;let rows:Vec<(i64,i64,String,i64,String)>=sqlx::query_as("SELECT id,player_id,promise_type,target_value,status FROM player_promises WHERE club_id=? ORDER BY id DESC").bind(c).fetch_all(&p).await.map_err(|e|e.to_string())?;Ok(rows.into_iter().map(|(id,player_id,promise_type,target_value,status)|PromiseRow{id,player_id,promise_type,target_value,status}).collect())}

#[tauri::command]
pub async fn create_player_promise(state:State<'_,AppState>,player_id:i64,promise_type:String,target_value:i64)->Result<(),String>{let p=pool(&state).await?;let c=club(&p).await?;if !(0..=100).contains(&target_value){return Err("Objetivo no válido".into())}let valid: (i64,)=sqlx::query_as("SELECT COUNT(*) FROM contracts WHERE player_id=? AND club_id=? AND is_active=1").bind(player_id).bind(c).fetch_one(&p).await.map_err(|e|e.to_string())?;if valid.0==0{return Err("El jugador no pertenece a la plantilla activa".into())}let active: (i64,)=sqlx::query_as("SELECT COUNT(*) FROM player_promises WHERE player_id=? AND club_id=? AND status='active'").bind(player_id).bind(c).fetch_one(&p).await.map_err(|e|e.to_string())?;if active.0>=3{return Err("Este jugador ya tiene el máximo de promesas activas".into())}sqlx::query("INSERT INTO player_promises(player_id,club_id,promise_type,target_value) VALUES(?,?,?,?)").bind(player_id).bind(c).bind(promise_type).bind(target_value).execute(&p).await.map_err(|e|e.to_string())?;Ok(())}

#[tauri::command]
pub async fn talk_to_player(state:State<'_,AppState>,player_id:i64,topic:String,response:String)->Result<String,String>{let p=pool(&state).await?;let c=club(&p).await?;let valid: (i64,)=sqlx::query_as("SELECT COUNT(*) FROM contracts WHERE player_id=? AND club_id=? AND is_active=1").bind(player_id).bind(c).fetch_one(&p).await.map_err(|e|e.to_string())?;if valid.0==0{return Err("El jugador no pertenece a la plantilla activa".into())}let delta=if response=="support"{5}else if response=="demand"{-3}else{1};sqlx::query("UPDATE player_states SET morale=MIN(100,MAX(0,morale+?)),happiness=MIN(100,MAX(0,happiness+?)) WHERE player_id=?").bind(delta).bind(delta).bind(player_id).execute(&p).await.map_err(|e|e.to_string())?;sqlx::query("INSERT INTO player_dialogues(player_id,club_id,topic,response,morale_delta) VALUES(?,?,?,?,?)").bind(player_id).bind(c).bind(topic).bind(response).bind(delta).execute(&p).await.map_err(|e|e.to_string())?;Ok(format!("Conversación registrada: impacto de {} en moral",delta))}

#[tauri::command]
pub async fn get_player_dialogues(state:State<'_,AppState>)->Result<Vec<DialogueRow>,String>{let p=pool(&state).await?;let c=club(&p).await?;let rows:Vec<(i64,i64,String,String,i64,String)>=sqlx::query_as("SELECT id,player_id,topic,response,morale_delta,created_at FROM player_dialogues WHERE club_id=? ORDER BY id DESC LIMIT 50").bind(c).fetch_all(&p).await.map_err(|e|e.to_string())?;Ok(rows.into_iter().map(|(id,player_id,topic,response,morale_delta,created_at)|DialogueRow{id,player_id,topic,response,morale_delta,created_at}).collect())}
