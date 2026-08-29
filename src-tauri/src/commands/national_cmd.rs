use serde::Serialize;
use tauri::State;
use crate::commands::AppState;

#[derive(Serialize, sqlx::FromRow)]
pub struct NationalPlayerRow { pub player_id:i64, pub player_name:String, pub nation_id:i64, pub nation_name:String, pub flag_path:Option<String>, pub club_name:Option<String>, pub position:String, pub selected:i64, pub callup_status:String }

#[tauri::command]
pub async fn get_national_players(state: State<'_, AppState>, nation_id:i64) -> Result<Vec<NationalPlayerRow>, String> {
    let pool = { let g=state.pool.lock().map_err(|e|e.to_string())?; g.clone().ok_or("No hay partida")? };
    let rows = sqlx::query_as::<_, NationalPlayerRow>("SELECT p.id,p.common_name,p.nation_id,n.name,n.flag_path,c.name,COALESCE(pp.pos,'UNI'),COALESCE(ns.selected,0),COALESCE(ns.callup_status,'eligible') FROM players p JOIN nations n ON n.id=p.nation_id LEFT JOIN contracts ct ON ct.player_id=p.id AND ct.is_active=1 LEFT JOIN clubs c ON c.id=ct.club_id LEFT JOIN national_team_squads ns ON ns.player_id=p.id AND ns.nation_id=? LEFT JOIN (SELECT player_id,CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END pos FROM player_positions) pp ON pp.player_id=p.id WHERE p.is_retired=0 AND (p.nation_id=? OR p.second_nation_id=?) ORDER BY selected DESC,p.common_name").bind(nation_id).bind(nation_id).bind(nation_id).fetch_all(&pool).await.map_err(|e|e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub async fn set_national_callup(state: State<'_, AppState>, nation_id:i64, player_id:i64, selected:bool) -> Result<(), String> {
    let pool = { let g=state.pool.lock().map_err(|e|e.to_string())?; g.clone().ok_or("No hay partida")? };
    let eligible: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM players WHERE id=? AND (nation_id=? OR second_nation_id=?) AND is_retired=0").bind(player_id).bind(nation_id).bind(nation_id).fetch_one(&pool).await.map_err(|e|e.to_string())?;
    if eligible.0 == 0 { return Err("Jugador no elegible para esta selección".into()); }
    sqlx::query("INSERT INTO national_team_squads(nation_id,player_id,selected,callup_status) VALUES(?,?,?,?) ON CONFLICT(nation_id,player_id) DO UPDATE SET selected=excluded.selected,callup_status=excluded.callup_status").bind(nation_id).bind(player_id).bind(selected as i64).bind(if selected {"called_up"} else {"eligible"}).execute(&pool).await.map_err(|e|e.to_string())?;
    Ok(())
}
