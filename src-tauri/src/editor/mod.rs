use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize)]
pub struct NationRow { pub id: i64, pub name: String, pub confederation: String, pub confederation_id: i64, pub reputation: i64, pub futsal_level: i64, pub flag_path: Option<String> }
#[derive(Serialize, Deserialize)]
pub struct ConfederationRow { pub id: i64, pub name: String, pub short_name: String, pub reputation: i64, pub crest_path: Option<String> }
#[derive(Serialize, Deserialize)]
pub struct ClubRow { pub id: i64, pub name: String, pub short_name: String, pub nation: String, pub nation_id: i64, pub city: String, pub city_id: Option<i64>, pub stadium: String, pub capacity: i64, pub reputation: i64, pub primary_color: String, pub secondary_color: String, pub crest_path: Option<String>, pub coach_id: Option<i64>, pub coach_name: Option<String>, pub staff_count: i64, pub squad_count: i64 }
#[derive(Serialize, Deserialize)]
pub struct PlayerRow { pub id: i64, pub first_name: String, pub last_name: String, pub common_name: String, pub nation: String, pub nation_id: i64, pub second_nation_id: Option<i64>, pub club: String, pub club_id: Option<i64>, pub position: String, pub secondary_position: Option<String>, pub ca: i64, pub pa: i64, pub age: i64, pub foot: String, pub photo_path: Option<String>, pub flag_path: Option<String>, pub second_flag_path: Option<String> }
#[derive(Serialize, Deserialize, Clone)]
pub struct StadiumRow { pub id:i64, pub name:String, pub city:String, pub city_id:Option<i64>, pub capacity:i64, pub pitch_type:String, pub photo_path:Option<String>, pub club_id:Option<i64>, pub club_name:Option<String> }

#[derive(Serialize, Deserialize, Clone)]
pub struct ContractEditorRow { pub id:i64, pub player_id:i64, pub player_name:String, pub club_id:i64, pub club_name:String, pub wage_weekly:f64, pub start_date:String, pub end_date:String, pub release_clause:Option<f64>, pub role:String, pub signing_bonus:f64, pub appearance_bonus:f64, pub clean_sheet_bonus:f64, pub loan_parent_id:Option<i64>, pub loan_until:Option<String>, pub is_active:i64 }

#[derive(Serialize, Deserialize)]
pub struct CompetitionRow { pub id: i64, pub name: String, pub nation: Option<String>, pub nation_id: Option<i64>, pub tier: Option<i64>, pub total_teams: Option<i64>, pub season: String, pub format: String, pub kind: String, pub group_count: i64, pub teams_per_group: i64, pub group_qualifiers: i64, pub knockout_two_legs: i64, pub tiebreak_rule: String }
#[derive(Serialize, Deserialize, Clone)]
pub struct StaffRow { pub id: i64, pub first_name: String, pub last_name: String, pub common_name: String, pub nation: String, pub nation_id: i64, pub flag_path: Option<String>, pub role: String, pub club_id: Option<i64>, pub club_name: Option<String>, pub tactical: i64, pub man_management: i64, pub judging: i64, pub motivating: i64, pub working_youngsters: i64, pub physio_level: i64, pub wage_weekly: f64, pub photo_path: Option<String> }

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAttributes {
    pub ca: i64,
    pub pa: i64,
    pub position: String,
    pub first_touch: i64, pub dribbling: i64, pub ball_control: i64, pub technique: i64,
    pub passing: i64, pub vision: i64, pub crossing: i64, pub long_shots: i64,
    pub finishing: i64, pub heading: i64, pub penalty_taking: i64,
    pub tackling: i64, pub marking: i64, pub interception: i64, pub blocking: i64,
    pub anticipation: i64, pub decisions: i64, pub positioning: i64, pub off_the_ball: i64, pub work_rate: i64,
    pub composure: i64, pub concentration: i64, pub determination: i64, pub bravery: i64,
    pub aggression: i64, pub leadership: i64, pub teamwork: i64, pub flair: i64,
    pub acceleration: i64, pub pace: i64, pub agility: i64, pub balance: i64, pub stamina: i64,
    pub strength: i64, pub jumping: i64,
    pub reflexes: i64, pub handling: i64, pub one_on_ones: i64, pub positioning_gk: i64,
    pub rushing_out: i64, pub throwing: i64, pub kicking: i64,
    pub professionalism: i64, pub consistency: i64, pub important_matches: i64, pub injury_proneness: i64,
}

pub async fn list_nations(pool: &SqlitePool) -> Result<Vec<NationRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, i64, String, i64, i64, Option<String>)>("SELECT n.id, n.name, n.confederation_id, c.short_name, n.reputation, n.futsal_level, n.flag_path FROM nations n JOIN confederations c ON c.id=n.confederation_id ORDER BY n.name")
        .fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name, confederation_id, confederation, reputation, futsal_level, flag_path)| NationRow { id, name, confederation, confederation_id, reputation, futsal_level, flag_path }).collect())
}
pub async fn list_confederations(pool: &SqlitePool) -> Result<Vec<ConfederationRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, String, i64, Option<String>)>("SELECT id, name, short_name, reputation, crest_path FROM confederations ORDER BY name").fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name, short_name, reputation, crest_path)| ConfederationRow { id, name, short_name, reputation, crest_path }).collect())
}
pub async fn update_confederation(pool: &SqlitePool, id: i64, name: String, short_name: String, reputation: i64) -> Result<(), String> {
    sqlx::query("UPDATE confederations SET name=?, short_name=?, reputation=? WHERE id=?").bind(name).bind(short_name).bind(reputation).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn list_clubs(pool: &SqlitePool) -> Result<Vec<ClubRow>, String> {
    #[derive(sqlx::FromRow)]
    #[allow(dead_code)]
    struct Cr {
        id: i64, name: String, short_name: String, nation: String, nation_id: i64,
        city: Option<String>, city_id: Option<i64>, stadium: Option<String>, capacity: Option<i64>,
        reputation: i64, primary_color: String, secondary_color: String,
        crest_path: Option<String>, coach_id: Option<i64>, coach_name: Option<String>,
        staff_count: i64, squad_count: i64,
    }
    let rows = sqlx::query_as::<_, Cr>(
        "SELECT c.id, c.name, c.short_name, n.name AS nation, c.nation_id, ci.name AS city, c.city_id, s.name AS stadium, s.capacity, c.reputation, c.primary_color, c.secondary_color, c.crest_path, c.coach_id, coach.common_name AS coach_name,
                (SELECT COUNT(*) FROM staff st WHERE st.club_id=c.id) AS staff_count,
                (SELECT COUNT(*) FROM contracts ct WHERE ct.club_id=c.id AND ct.is_active=1) AS squad_count
         FROM clubs c JOIN nations n ON n.id=c.nation_id LEFT JOIN cities ci ON ci.id=c.city_id LEFT JOIN stadiums s ON s.id=c.stadium_id LEFT JOIN staff coach ON coach.id=c.coach_id ORDER BY c.reputation DESC, c.name"
    ).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|r| ClubRow { id: r.id, name: r.name, short_name: r.short_name, nation: r.nation, nation_id: r.nation_id, city: r.city.unwrap_or_default(), city_id: r.city_id, stadium: r.stadium.unwrap_or_default(), capacity: r.capacity.unwrap_or(2000), reputation: r.reputation, primary_color: r.primary_color, secondary_color: r.secondary_color, crest_path: r.crest_path, coach_id: r.coach_id, coach_name: r.coach_name, staff_count: r.staff_count, squad_count: r.squad_count }).collect())
}
pub async fn list_players(pool: &SqlitePool, limit: i64) -> Result<Vec<PlayerRow>, String> {
    let lim = limit.clamp(20, 2000);
    let rows = sqlx::query_as::<_, (i64, String, String, String, String, i64, Option<i64>, Option<String>, Option<i64>, Option<String>, Option<String>, i64, i64, String, Option<String>, Option<String>)>(
        "SELECT p.id, p.first_name, p.last_name, p.common_name, n.name, p.nation_id, p.second_nation_id, cl.name, c.club_id, COALESCE(pp.pos,'UNI'), p.secondary_position, ps.current_ability, ps.potential_ability, p.preferred_foot, p.photo_path, n.flag_path, n2.flag_path FROM players p JOIN nations n ON n.id=p.nation_id JOIN player_states ps ON ps.player_id=p.id LEFT JOIN contracts c ON c.player_id=p.id AND c.is_active=1 LEFT JOIN clubs cl ON cl.id=c.club_id LEFT JOIN nations n2 ON n2.id=p.second_nation_id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp ON pp.player_id=p.id ORDER BY ps.current_ability DESC LIMIT ?"
    ).bind(lim).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, first_name, last_name, common_name, nation, nation_id, second_nation_id, club, club_id, position, secondary_position, ca, pa, foot, photo_path, flag_path)| {
        PlayerRow { id, first_name, last_name, common_name, nation, nation_id, second_nation_id, club: club.unwrap_or_default(), club_id, position: position.unwrap_or_else(|| "UNI".into()), secondary_position, ca, pa, age: 0, foot, photo_path, flag_path, second_flag_path: None }
    }).collect())
}
pub async fn list_players_by_club(pool: &SqlitePool, club_id: i64) -> Result<Vec<PlayerRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, String, String, String, i64, Option<i64>, Option<String>, Option<i64>, Option<String>, Option<String>, i64, i64, String, Option<String>, Option<String>)>(
        "SELECT p.id, p.first_name, p.last_name, p.common_name, n.name, p.nation_id, p.second_nation_id, cl.name, c.club_id, COALESCE(pp.pos,'UNI'), p.secondary_position, ps.current_ability, ps.potential_ability, p.preferred_foot, p.photo_path, n.flag_path, n2.flag_path FROM players p JOIN nations n ON n.id=p.nation_id JOIN player_states ps ON ps.player_id=p.id JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 LEFT JOIN clubs cl ON cl.id=c.club_id LEFT JOIN nations n2 ON n2.id=p.second_nation_id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp ON pp.player_id=p.id ORDER BY ps.current_ability DESC"
    ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, first_name, last_name, common_name, nation, nation_id, second_nation_id, club, club_id, position, secondary_position, ca, pa, foot, photo_path, flag_path)| {
        PlayerRow { id, first_name, last_name, common_name, nation, nation_id, second_nation_id, club: club.unwrap_or_default(), club_id, position: position.unwrap_or_else(|| "UNI".into()), secondary_position, ca, pa, age: 0, foot, photo_path, flag_path, second_flag_path: None }
    }).collect())
}
pub async fn assign_player(pool: &SqlitePool, player_id: i64, club_id: i64) -> Result<(), String> {
    let has: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts WHERE player_id=? AND is_active=1").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (ca,): (i64,) = sqlx::query_as("SELECT current_ability FROM player_states WHERE player_id=?").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if has.0 > 0 {
        sqlx::query("UPDATE contracts SET club_id=?, wage_weekly=? WHERE player_id=? AND is_active=1").bind(club_id).bind(ca as f64*18.0).bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    } else {
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(player_id).bind(club_id).bind(ca as f64*18.0).bind("2026-07-10").bind("2029-07-10").execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}
pub async fn release_player(pool: &SqlitePool, player_id: i64) -> Result<(), String> {
    sqlx::query("UPDATE contracts SET is_active=0 WHERE player_id=? AND is_active=1").bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn list_contracts(pool: &SqlitePool) -> Result<Vec<ContractEditorRow>, String> {
    let rows = sqlx::query_as::<_,(i64,i64,String,i64,String,f64,String,String,Option<f64>,String,f64,f64,f64,Option<i64>,Option<String>,i64)>("SELECT c.id,c.player_id,p.common_name,c.club_id,cl.name,c.wage_weekly,c.start_date,c.end_date,c.release_clause,c.contract_role,c.signing_bonus,c.appearance_bonus,c.clean_sheet_bonus,c.loan_parent_id,c.loan_until,c.is_active FROM contracts c JOIN players p ON p.id=c.player_id JOIN clubs cl ON cl.id=c.club_id ORDER BY c.is_active DESC,c.end_date") .fetch_all(pool).await.map_err(|e|e.to_string())?;
    Ok(rows.into_iter().map(|(id,player_id,player_name,club_id,club_name,wage_weekly,start_date,end_date,release_clause,role,signing_bonus,appearance_bonus,clean_sheet_bonus,loan_parent_id,loan_until,is_active)| ContractEditorRow { id,player_id,player_name,club_id,club_name,wage_weekly,start_date,end_date,release_clause,role,signing_bonus,appearance_bonus,clean_sheet_bonus,loan_parent_id,loan_until,is_active }).collect())
}
pub async fn get_finance(pool:&SqlitePool, club_id:i64)->Result<crate::finance::FinanceRow,String>{crate::finance::get_finance(pool,club_id).await}
pub async fn update_finance(pool:&SqlitePool, club_id:i64, balance:f64, transfer_budget:f64, wage_budget:f64, sponsorship:f64, ticket_income:f64, prize_money:f64)->Result<(),String>{if [balance,transfer_budget,wage_budget,sponsorship,ticket_income,prize_money].iter().any(|v|!v.is_finite()||*v<0.0){return Err("Valores económicos no válidos".into())}sqlx::query("UPDATE club_finances SET balance=?,transfer_budget=?,wage_budget=?,sponsorship_income=?,ticket_income=?,prize_money=? WHERE club_id=?").bind(balance).bind(transfer_budget).bind(wage_budget).bind(sponsorship).bind(ticket_income).bind(prize_money).bind(club_id).execute(pool).await.map_err(|e|e.to_string())?;Ok(())}
pub async fn update_contract(pool: &SqlitePool, id:i64, club_id:i64, wage:f64, start_date:String, end_date:String, release_clause:Option<f64>, role:String, signing_bonus:f64, appearance_bonus:f64, clean_sheet_bonus:f64, is_active:i64) -> Result<(),String> {
    if wage < 0.0 || signing_bonus < 0.0 || appearance_bonus < 0.0 || clean_sheet_bonus < 0.0 || end_date < start_date { return Err("Datos de contrato no válidos".into()); }
    sqlx::query("UPDATE contracts SET club_id=?,wage_weekly=?,start_date=?,end_date=?,release_clause=?,contract_role=?,signing_bonus=?,appearance_bonus=?,clean_sheet_bonus=?,is_active=? WHERE id=?").bind(club_id).bind(wage).bind(start_date).bind(end_date).bind(release_clause).bind(role).bind(signing_bonus).bind(appearance_bonus).bind(clean_sheet_bonus).bind(is_active).bind(id).execute(pool).await.map_err(|e|e.to_string())?;
    Ok(())
}
pub async fn list_competitions(pool: &SqlitePool) -> Result<Vec<CompetitionRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, Option<i64>, Option<String>, Option<i64>, Option<i64>, String, String, String, i64, i64, i64, i64, String)>(
        "SELECT comp.id, comp.name, comp.nation_id, n.name, comp.tier, comp.total_teams, comp.season, comp.format, comp.kind, comp.group_count, comp.teams_per_group, comp.group_qualifiers, comp.knockout_two_legs, comp.tiebreak_rule FROM competitions comp LEFT JOIN nations n ON n.id=comp.nation_id ORDER BY comp.kind, comp.tier NULLS LAST, comp.name"
    ).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name, nation_id, nation, tier, total_teams, season, format, kind, group_count, teams_per_group, group_qualifiers, knockout_two_legs, tiebreak_rule)| CompetitionRow { id, name, nation, nation_id, tier, total_teams, season, format, kind, group_count, teams_per_group, group_qualifiers, knockout_two_legs, tiebreak_rule }).collect())
}
pub async fn list_stadiums(pool: &SqlitePool) -> Result<Vec<StadiumRow>, String> {
    let rows=sqlx::query_as::<_,(i64,String,String,Option<i64>,i64,String,Option<String>,Option<i64>,Option<String>)>("SELECT s.id,s.name,COALESCE(ci.name,'-'),s.city_id,s.capacity,s.pitch_type,s.photo_path,c.id,c.name FROM stadiums s LEFT JOIN cities ci ON ci.id=s.city_id LEFT JOIN clubs c ON c.stadium_id=s.id ORDER BY s.name").fetch_all(pool).await.map_err(|e|e.to_string())?;
    Ok(rows.into_iter().map(|(id,name,city,city_id,capacity,pitch_type,photo_path,club_id,club_name)| StadiumRow{id,name,city,city_id,capacity,pitch_type,photo_path,club_id,club_name}).collect())
}
pub async fn create_stadium(pool:&SqlitePool,name:String,city_id:Option<i64>,capacity:i64,pitch_type:String)->Result<i64,String>{ if name.trim().is_empty()||capacity<1{return Err("Pabellón no válido".into())} let (id,):(i64,)=sqlx::query_as("INSERT INTO stadiums(name,city_id,capacity,pitch_type) VALUES(?,?,?,?) RETURNING id").bind(name).bind(city_id).bind(capacity).bind(pitch_type).fetch_one(pool).await.map_err(|e|e.to_string())?;Ok(id) }
pub async fn update_stadium(pool:&SqlitePool,id:i64,name:String,city_id:Option<i64>,capacity:i64,pitch_type:String)->Result<(),String>{if name.trim().is_empty()||capacity<1{return Err("Pabellón no válido".into())}sqlx::query("UPDATE stadiums SET name=?,city_id=?,capacity=?,pitch_type=? WHERE id=?").bind(name).bind(city_id).bind(capacity).bind(pitch_type).bind(id).execute(pool).await.map_err(|e|e.to_string())?;Ok(())}
pub async fn delete_stadium(pool:&SqlitePool,id:i64)->Result<(),String>{let (n,):(i64,)=sqlx::query_as("SELECT COUNT(*) FROM clubs WHERE stadium_id=?").bind(id).fetch_one(pool).await.map_err(|e|e.to_string())?;if n>0{return Err("No se puede borrar un pabellón asignado a un club".into())}sqlx::query("DELETE FROM stadiums WHERE id=?").bind(id).execute(pool).await.map_err(|e|e.to_string())?;Ok(())}

pub async fn list_stadiums_legacy(pool: &SqlitePool) -> Result<Vec<(i64, String, String, i64)>, String> {
    let rows: Vec<(i64, String, String, i64)> = sqlx::query_as("SELECT s.id, s.name, COALESCE(ci.name,'-'), s.capacity FROM stadiums s LEFT JOIN cities ci ON ci.id=s.city_id ORDER BY s.capacity DESC").fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}

pub async fn create_nation(pool: &SqlitePool, name: String, confederation_id: i64, reputation: i64, futsal_level: i64) -> Result<i64, String> {
    let (id,): (i64,) = sqlx::query_as("INSERT INTO nations(name, confederation_id, reputation, futsal_level) VALUES(?,?,?,?) RETURNING id").bind(name).bind(confederation_id).bind(reputation).bind(futsal_level).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(id)
}
pub async fn update_nation(pool: &SqlitePool, id: i64, name: String, confederation_id: i64, reputation: i64, futsal_level: i64) -> Result<(), String> {
    sqlx::query("UPDATE nations SET name=?, confederation_id=?, reputation=?, futsal_level=? WHERE id=?").bind(name).bind(confederation_id).bind(reputation).bind(futsal_level).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn delete_nation(pool: &SqlitePool, id: i64) -> Result<(), String> {
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs WHERE nation_id=?").bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if cnt > 0 { return Err(format!("No se puede borrar: {} clubes dependen de esta nación", cnt)); }
    sqlx::query("DELETE FROM nations WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn create_club(pool: &SqlitePool, name: String, short: String, nation_id: i64, city: String, stadium: String, capacity: i64, rep: i64, c1: String, c2: String) -> Result<i64, String> {
    let city_id = if city.is_empty() { None } else {
        let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM cities WHERE name=? AND nation_id=?").bind(&city).bind(nation_id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
        if let Some((id,)) = existing { Some(id) } else {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name, nation_id, population) VALUES(?,?,500000) RETURNING id").bind(&city).bind(nation_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
            Some(id)
        }
    };
    let stadium_id = if stadium.is_empty() { None } else {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO stadiums(name, city_id, capacity) VALUES(?,?,?) RETURNING id").bind(&stadium).bind(city_id).bind(capacity).fetch_one(pool).await.map_err(|e| e.to_string())?;
        Some(id)
    };
    let (id,): (i64,) = sqlx::query_as("INSERT INTO clubs(name, short_name, nation_id, city_id, stadium_id, reputation, primary_color, secondary_color) VALUES(?,?,?,?,?,?,?,?) RETURNING id")
        .bind(name).bind(short).bind(nation_id).bind(city_id).bind(stadium_id).bind(rep).bind(c1).bind(c2).fetch_one(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO club_finances(club_id, balance, transfer_budget, wage_budget) VALUES(?,?,?,?)").bind(id).bind(rep as f64*1800.0).bind(rep as f64*450.0).bind(rep as f64*12.0+2000.0).execute(pool).await.ok();
    sqlx::query("INSERT INTO tactics(club_id, formation) VALUES(?, '3-1')").bind(id).execute(pool).await.ok();
    for (day, tid, intensity) in [(0,1,70),(1,2,75),(2,4,65),(3,3,75),(4,7,60)] {
        sqlx::query("INSERT OR IGNORE INTO training_schedule(club_id, day_of_week, training_type_id, intensity) VALUES(?,?,?,?)").bind(id).bind(day).bind(tid).bind(intensity).execute(pool).await.ok();
    }
    // Crear 12 jugadores ficticios para el club
    let base_date = chrono::NaiveDate::from_ymd_opt(2026,7,10).unwrap();
    for idx in 0..12 {
        let role = match idx { 0|1 => "POR", 2|3 => "CIE", 4..=7 => "ALA", 8|9 => "PIV", _ => "UNI" };
        let age = if idx<2 { 26 } else { 24 };
        let dob = base_date - chrono::Duration::days(age*365);
        let first = "Nuevo"; let last = format!("Jugador{}", idx+1);
        let ca = 70 + (rep/20) as i64; let pa = ca + 15;
        let (pid,): (i64,) = sqlx::query_as("INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,height_cm,weight_kg) VALUES(?,?,?,?,?,?,?) RETURNING id")
            .bind(first).bind(last.clone()).bind(format!("{} {}", first, last)).bind(dob.format("%Y-%m-%d").to_string()).bind(nation_id).bind(180).bind(75).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let (por,cie,ala,piv,uni) = match role { "POR"=>(20,2,1,1,3), "CIE"=>(1,20,12,8,10), "ALA"=>(1,10,20,10,14), "PIV"=>(1,6,10,20,12), _=>(3,10,14,14,20) };
        sqlx::query("INSERT INTO player_positions(player_id,por_natural,cie_natural,ala_natural,piv_natural,uni_natural) VALUES(?,?,?,?,?,?)").bind(pid).bind(por).bind(cie).bind(ala).bind(piv).bind(uni).execute(pool).await.ok();
        sqlx::query("INSERT INTO player_states(player_id,current_ability,potential_ability) VALUES(?,?,?)").bind(pid).bind(ca).bind(pa).execute(pool).await.ok();
        sqlx::query("INSERT INTO player_attributes(player_id,first_touch,dribbling,ball_control,technique,passing,vision,crossing,long_shots,finishing,heading,penalty_taking,tackling,marking,interception,blocking,anticipation,decisions,positioning,off_the_ball,work_rate,composure,concentration,determination,bravery,aggression,leadership,teamwork,flair,acceleration,pace,agility,balance,stamina,strength,jumping,reflexes,handling,one_on_ones,positioning_gk,rushing_out,throwing,kicking,professionalism,consistency,important_matches,injury_proneness) VALUES(?,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50)").bind(pid).execute(pool).await.ok();
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(pid).bind(id).bind(ca as f64*18.0).bind("2026-07-10").bind("2029-07-10").execute(pool).await.ok();
    }
    sqlx::query("UPDATE club_finances SET total_wages=(SELECT SUM(wage_weekly) FROM contracts WHERE club_id=? AND is_active=1) WHERE club_id=?").bind(id).bind(id).execute(pool).await.ok();
    Ok(id)
}
pub async fn update_club(pool: &SqlitePool, id: i64, name: String, short: String, nation_id: i64, city: String, stadium: String, capacity: i64, rep: i64, c1: String, c2: String) -> Result<(), String> {
    let city_id = if city.is_empty() { None } else {
        let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM cities WHERE name=? AND nation_id=?").bind(&city).bind(nation_id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
        if let Some((id,)) = existing { Some(id) } else {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name, nation_id, population) VALUES(?,?,500000) RETURNING id").bind(&city).bind(nation_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
            Some(id)
        }
    };
    let cur_stadium: Option<(Option<i64>,)> = sqlx::query_as("SELECT stadium_id FROM clubs WHERE id=?").bind(id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
    let existing_stadium = cur_stadium.and_then(|(s,)| s);
    let stadium_id = if stadium.is_empty() { existing_stadium } else {
        if let Some(es) = existing_stadium {
            sqlx::query("UPDATE stadiums SET name=?, capacity=? WHERE id=?").bind(&stadium).bind(capacity).bind(es).execute(pool).await.map_err(|e| e.to_string())?;
            Some(es)
        } else {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO stadiums(name, city_id, capacity) VALUES(?,?,?) RETURNING id").bind(&stadium).bind(city_id).bind(capacity).fetch_one(pool).await.map_err(|e| e.to_string())?;
            Some(id)
        }
    };
    sqlx::query("UPDATE clubs SET name=?, short_name=?, nation_id=?, city_id=?, stadium_id=?, reputation=?, primary_color=?, secondary_color=? WHERE id=?")
        .bind(name).bind(short).bind(nation_id).bind(city_id).bind(stadium_id).bind(rep).bind(c1).bind(c2).bind(id)
        .execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn update_player(pool: &SqlitePool, id: i64, first: String, last: String, nation_id: i64, second_nation_id: Option<i64>, secondary_position: Option<String>, club_id: Option<i64>, ca: i64, pa: i64, pos: String) -> Result<(), String> {
    sqlx::query("UPDATE players SET first_name=?, last_name=?, common_name=?, nation_id=?, second_nation_id=?, secondary_position=? WHERE id=?").bind(&first).bind(&last).bind(format!("{} {}", first, last)).bind(nation_id).bind(second_nation_id).bind(secondary_position).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE player_states SET current_ability=?, potential_ability=? WHERE player_id=?").bind(ca).bind(pa).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    let (por,cie,ala,piv,uni) = match pos.as_str() { "POR"=>(20,2,1,1,3), "CIE"=>(1,20,12,8,10), "ALA"=>(1,10,20,10,14), "PIV"=>(1,6,10,20,12), _=>(3,10,14,14,20) };
    sqlx::query("UPDATE player_positions SET por_natural=?, cie_natural=?, ala_natural=?, piv_natural=?, uni_natural=? WHERE player_id=?").bind(por).bind(cie).bind(ala).bind(piv).bind(uni).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    if let Some(cid) = club_id {
        let has: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts WHERE player_id=? AND is_active=1").bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if has.0 == 0 {
            sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(id).bind(cid).bind(ca as f64*18.0).bind("2026-07-10").bind("2029-07-10").execute(pool).await.map_err(|e| e.to_string())?;
        } else {
            sqlx::query("UPDATE contracts SET club_id=?, wage_weekly=? WHERE player_id=? AND is_active=1").bind(cid).bind(ca as f64*18.0).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
pub async fn update_competition(pool: &SqlitePool, id: i64, name: String, nation_id: Option<i64>, tier: Option<i64>, teams: i64, season: String, group_count: i64, teams_per_group: i64, group_qualifiers: i64, knockout_two_legs: i64, tiebreak_rule: String) -> Result<(), String> {
    let kind = if nation_id.is_none() { "national_team" } else { "club" };
    if group_count < 0 || teams_per_group < 0 || group_qualifiers < 0 || knockout_two_legs < 0 || teams < 2 || !crate::competition::rules::valid_tiebreak(&tiebreak_rule) { return Err("Formato de competición no válido".into()); }
    sqlx::query("UPDATE competitions SET name=?, nation_id=?, tier=?, total_teams=?, season=?, kind=?, group_count=?, teams_per_group=?, group_qualifiers=?, knockout_two_legs=?, tiebreak_rule=? WHERE id=?").bind(name).bind(nation_id).bind(tier).bind(teams).bind(season).bind(kind).bind(group_count).bind(teams_per_group).bind(group_qualifiers).bind(knockout_two_legs).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn delete_club(pool: &SqlitePool, id: i64) -> Result<(), String> {
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE home_club_id=? OR away_club_id=?").bind(id).bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if cnt>0 { return Err(format!("No se puede borrar: {} partidos referencian al club", cnt)); }
    sqlx::query("UPDATE contracts SET is_active=0 WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM club_finances WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM tactics WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM training_schedule WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM league_standings WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM clubs WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn create_player(pool: &SqlitePool, first: String, last: String, nation_id: i64, club_id: Option<i64>, ca: i64, pa: i64, pos: String) -> Result<i64, String> {
    let dob = chrono::NaiveDate::from_ymd_opt(2000,6,15).unwrap().format("%Y-%m-%d").to_string();
    let (pid,): (i64,) = sqlx::query_as("INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,height_cm,weight_kg) VALUES(?,?,?,?,?,?,?) RETURNING id")
        .bind(&first).bind(&last).bind(format!("{} {}", first, last)).bind(dob).bind(nation_id).bind(180).bind(75).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (por,cie,ala,piv,uni) = match pos.as_str() { "POR"=>(20,2,1,1,3), "CIE"=>(1,20,12,8,10), "ALA"=>(1,10,20,10,14), "PIV"=>(1,6,10,20,12), _=>(3,10,14,14,20) };
    sqlx::query("INSERT INTO player_positions(player_id,por_natural,cie_natural,ala_natural,piv_natural,uni_natural) VALUES(?,?,?,?,?,?)").bind(pid).bind(por).bind(cie).bind(ala).bind(piv).bind(uni).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO player_states(player_id,current_ability,potential_ability) VALUES(?,?,?)").bind(pid).bind(ca).bind(pa).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO player_attributes(player_id,first_touch,dribbling,ball_control,technique,passing,vision,crossing,long_shots,finishing,heading,penalty_taking,tackling,marking,interception,blocking,anticipation,decisions,positioning,off_the_ball,work_rate,composure,concentration,determination,bravery,aggression,leadership,teamwork,flair,acceleration,pace,agility,balance,stamina,strength,jumping,reflexes,handling,one_on_ones,positioning_gk,rushing_out,throwing,kicking,professionalism,consistency,important_matches,injury_proneness) VALUES(?,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50,50)").bind(pid).execute(pool).await.map_err(|e| e.to_string())?;
    if let Some(cid) = club_id {
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(pid).bind(cid).bind(ca as f64*18.0).bind("2026-07-10").bind("2029-07-10").execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(pid)
}
pub async fn delete_player(pool: &SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("UPDATE contracts SET is_active=0 WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM player_attributes WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM player_states WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM player_positions WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM injuries WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM players WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn create_competition(pool: &SqlitePool, name: String, nation_id: Option<i64>, tier: Option<i64>, teams: i64, season: String) -> Result<i64, String> {
    let kind = if nation_id.is_none() { "national_team" } else { "club" };
    let (id,): (i64,) = sqlx::query_as("INSERT INTO competitions(name, nation_id, tier, total_teams, season, kind) VALUES(?,?,?,?,?,?) RETURNING id").bind(name).bind(nation_id).bind(tier).bind(teams).bind(season).bind(kind).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(id)
}
pub async fn delete_competition(pool: &SqlitePool, id: i64) -> Result<(), String> {
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE competition_id=?").bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if cnt>0 { return Err(format!("No se puede borrar: {} partidos existen", cnt)); }
    sqlx::query("DELETE FROM league_standings WHERE competition_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM competitions WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_staff(pool: &SqlitePool, club_id: Option<i64>) -> Result<Vec<StaffRow>, String> {
    #[derive(sqlx::FromRow)]
    #[allow(dead_code)]
    struct Sr { id: i64, first_name: String, last_name: String, common_name: String, nation: String, nation_id: i64, flag_path: Option<String>, role: String, club_id: Option<i64>, club_name: Option<String>, tactical: i64, man_management: i64, judging: i64, motivating: i64, working_youngsters: i64, physio_level: i64, wage_weekly: f64, photo_path: Option<String> }
    let rows = sqlx::query_as::<_, Sr>(
        "SELECT st.id, st.first_name, st.last_name, COALESCE(st.common_name, st.first_name || ' ' || st.last_name) AS common_name, n.name AS nation, st.nation_id, n.flag_path, st.role, st.club_id, cl.name AS club_name, st.tactical, st.man_management, st.judging, st.motivating, st.working_youngsters, st.physio_level, st.wage_weekly, st.photo_path FROM staff st JOIN nations n ON n.id=st.nation_id LEFT JOIN clubs cl ON cl.id=st.club_id WHERE (? IS NULL OR st.club_id=?) ORDER BY st.role, st.last_name"
    ).bind(club_id).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|r| StaffRow { id: r.id, first_name: r.first_name, last_name: r.last_name, common_name: r.common_name, nation: r.nation, nation_id: r.nation_id, flag_path: r.flag_path, role: r.role, club_id: r.club_id, club_name: r.club_name, tactical: r.tactical, man_management: r.man_management, judging: r.judging, motivating: r.motivating, working_youngsters: r.working_youngsters, physio_level: r.physio_level, wage_weekly: r.wage_weekly, photo_path: r.photo_path }).collect())
}
pub async fn list_coaches(pool: &SqlitePool) -> Result<Vec<StaffRow>, String> {
    #[derive(sqlx::FromRow)]
    #[allow(dead_code)]
    struct Sr { id: i64, first_name: String, last_name: String, common_name: String, nation: String, nation_id: i64, flag_path: Option<String>, role: String, club_id: Option<i64>, club_name: Option<String>, tactical: i64, man_management: i64, judging: i64, motivating: i64, working_youngsters: i64, physio_level: i64, wage_weekly: f64, photo_path: Option<String> }
    let rows = sqlx::query_as::<_, Sr>(
        "SELECT st.id, st.first_name, st.last_name, COALESCE(st.common_name, st.first_name || ' ' || st.last_name) AS common_name, n.name AS nation, st.nation_id, n.flag_path, st.role, st.club_id, cl.name AS club_name, st.tactical, st.man_management, st.judging, st.motivating, st.working_youngsters, st.physio_level, st.wage_weekly, st.photo_path FROM staff st JOIN nations n ON n.id=st.nation_id LEFT JOIN clubs cl ON cl.id=st.club_id WHERE st.role='coach' ORDER BY st.last_name"
    ).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|r| StaffRow { id: r.id, first_name: r.first_name, last_name: r.last_name, common_name: r.common_name, nation: r.nation, nation_id: r.nation_id, flag_path: r.flag_path, role: r.role, club_id: r.club_id, club_name: r.club_name, tactical: r.tactical, man_management: r.man_management, judging: r.judging, motivating: r.motivating, working_youngsters: r.working_youngsters, physio_level: r.physio_level, wage_weekly: r.wage_weekly, photo_path: r.photo_path }).collect())
}
pub async fn create_staff(pool: &SqlitePool, first: String, last: String, nation_id: i64, role: String, club_id: Option<i64>, tactical: i64, man_management: i64, judging: i64, motivating: i64, working_youngsters: i64, physio_level: i64, wage_weekly: f64) -> Result<i64, String> {
    if !["coach","assistant","scout","physio","fitness_coach","goalkeeper_coach","technical_coach","analyst"].contains(&role.as_str()) { return Err("Rol de staff no válido".into()); }
    for value in [tactical, man_management, judging, motivating, working_youngsters, physio_level] { if !(1..=20).contains(&value) { return Err("Los atributos deben estar entre 1 y 20".into()); } }
    if wage_weekly < 0.0 { return Err("El salario no puede ser negativo".into()); }
    let (id,): (i64,) = sqlx::query_as("INSERT INTO staff(first_name,last_name,common_name,nation_id,role,club_id,tactical,man_management,judging,motivating,working_youngsters,physio_level,wage_weekly) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?) RETURNING id")
        .bind(&first).bind(&last).bind(format!("{} {}", first, last)).bind(nation_id).bind(&role).bind(club_id).bind(tactical).bind(man_management).bind(judging).bind(motivating).bind(working_youngsters).bind(physio_level).bind(wage_weekly)
        .fetch_one(pool).await.map_err(|e| e.to_string())?;
    if role == "coach" {
        if let Some(cid) = club_id { sqlx::query("UPDATE clubs SET coach_id=? WHERE id=?").bind(id).bind(cid).execute(pool).await.ok(); }
    }
    Ok(id)
}
pub async fn update_staff(pool: &SqlitePool, id: i64, first: String, last: String, nation_id: i64, role: String, club_id: Option<i64>, tactical: i64, man_management: i64, judging: i64, motivating: i64, working_youngsters: i64, physio_level: i64, wage_weekly: f64) -> Result<(), String> {
    sqlx::query("UPDATE staff SET first_name=?, last_name=?, common_name=?, nation_id=?, role=?, club_id=?, tactical=?, man_management=?, judging=?, motivating=?, working_youngsters=?, physio_level=?, wage_weekly=? WHERE id=?")
        .bind(&first).bind(&last).bind(format!("{} {}", first, last)).bind(nation_id).bind(&role).bind(club_id).bind(tactical).bind(man_management).bind(judging).bind(motivating).bind(working_youngsters).bind(physio_level).bind(wage_weekly).bind(id)
        .execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn delete_staff(pool: &SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("UPDATE clubs SET coach_id=NULL WHERE coach_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM staff WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn set_coach(pool: &SqlitePool, club_id: i64, staff_id: Option<i64>) -> Result<(), String> {
    sqlx::query("UPDATE clubs SET coach_id=? WHERE id=?").bind(staff_id).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}

fn save_media(data_b64: &str, ext: &str, prefix: &str) -> Result<String, String> {
    let bytes = base64_decode(data_b64).ok_or("Imagen no válida (base64)")?;
    let dir = crate::db::app_data_dir().join("media");
    let _ = std::fs::create_dir_all(&dir);
    let safe_ext = if ["png","jpg","jpeg","webp","gif","svg"].contains(&ext.to_lowercase().as_str()) { ext.to_lowercase() } else { "png".into() };
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(0);
    let filename = format!("{}_{}.{}", prefix, nanos, safe_ext);
    let path = dir.join(&filename);
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    Ok(path.display().to_string())
}

pub async fn set_crest(pool: &SqlitePool, club_id: i64, data_b64: &str, ext: &str) -> Result<String, String> {
    let path = save_media(data_b64, ext, &format!("club_{}", club_id))?;
    sqlx::query("UPDATE clubs SET crest_path=? WHERE id=?").bind(&path).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(path)
}
pub async fn set_player_photo(pool: &SqlitePool, player_id: i64, data_b64: &str, ext: &str) -> Result<String, String> {
    let path = save_media(data_b64, ext, &format!("player_{}", player_id))?;
    sqlx::query("UPDATE players SET photo_path=? WHERE id=?").bind(&path).bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(path)
}
pub async fn set_staff_photo(pool: &SqlitePool, staff_id: i64, data_b64: &str, ext: &str) -> Result<String, String> {
    let path = save_media(data_b64, ext, &format!("staff_{}", staff_id))?;
    sqlx::query("UPDATE staff SET photo_path=? WHERE id=?").bind(&path).bind(staff_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(path)
}
pub async fn set_nation_flag(pool: &SqlitePool, nation_id: i64, data_b64: &str, ext: &str) -> Result<String, String> {
    let path = save_media(data_b64, ext, &format!("flag_{}", nation_id))?;
    sqlx::query("UPDATE nations SET flag_path=? WHERE id=?").bind(&path).bind(nation_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(path)
}
pub async fn set_crest_confed(pool: &SqlitePool, confed_id: i64, data_b64: &str, ext: &str) -> Result<String, String> {
    let path = save_media(data_b64, ext, &format!("confed_{}", confed_id))?;
    sqlx::query("UPDATE confederations SET crest_path=? WHERE id=?").bind(&path).bind(confed_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(path)
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct AttrRaw { first_touch: i64, dribbling: i64, ball_control: i64, technique: i64, passing: i64, vision: i64, crossing: i64, long_shots: i64, finishing: i64, heading: i64, penalty_taking: i64, tackling: i64, marking: i64, interception: i64, blocking: i64, anticipation: i64, decisions: i64, positioning: i64, off_the_ball: i64, work_rate: i64, composure: i64, concentration: i64, determination: i64, bravery: i64, aggression: i64, leadership: i64, teamwork: i64, flair: i64, acceleration: i64, pace: i64, agility: i64, balance: i64, stamina: i64, strength: i64, jumping: i64, reflexes: i64, handling: i64, one_on_ones: i64, positioning_gk: i64, rushing_out: i64, throwing: i64, kicking: i64, professionalism: i64, consistency: i64, important_matches: i64, injury_proneness: i64 }

pub async fn get_player_attributes(pool: &SqlitePool, player_id: i64) -> Result<PlayerAttributes, String> {
    let pos: i64 = sqlx::query_scalar("SELECT CASE WHEN por_natural>=18 THEN 0 WHEN cie_natural>=18 THEN 1 WHEN piv_natural>=18 THEN 3 WHEN ala_natural>=18 THEN 2 ELSE 4 END FROM player_positions WHERE player_id=?")
        .bind(player_id).fetch_one(pool).await.unwrap_or(4);
    let position = ["POR","CIE","ALA","PIV","UNI"][pos.max(0) as usize].to_string();
    let ca: i64 = sqlx::query_scalar("SELECT current_ability FROM player_states WHERE player_id=?").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let pa: i64 = sqlx::query_scalar("SELECT potential_ability FROM player_states WHERE player_id=?").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let row = sqlx::query_as::<_, AttrRaw>(
        "SELECT first_touch, dribbling, ball_control, technique, passing, vision, crossing, long_shots, finishing, heading, penalty_taking, tackling, marking, interception, blocking, anticipation, decisions, positioning, off_the_ball, work_rate, composure, concentration, determination, bravery, aggression, leadership, teamwork, flair, acceleration, pace, agility, balance, stamina, strength, jumping, reflexes, handling, one_on_ones, positioning_gk, rushing_out, throwing, kicking, professionalism, consistency, important_matches, injury_proneness FROM player_attributes WHERE player_id=?"
    ).bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(PlayerAttributes { ca, pa, position, first_touch: row.first_touch, dribbling: row.dribbling, ball_control: row.ball_control, technique: row.technique, passing: row.passing, vision: row.vision, crossing: row.crossing, long_shots: row.long_shots, finishing: row.finishing, heading: row.heading, penalty_taking: row.penalty_taking, tackling: row.tackling, marking: row.marking, interception: row.interception, blocking: row.blocking, anticipation: row.anticipation, decisions: row.decisions, positioning: row.positioning, off_the_ball: row.off_the_ball, work_rate: row.work_rate, composure: row.composure, concentration: row.concentration, determination: row.determination, bravery: row.bravery, aggression: row.aggression, leadership: row.leadership, teamwork: row.teamwork, flair: row.flair, acceleration: row.acceleration, pace: row.pace, agility: row.agility, balance: row.balance, stamina: row.stamina, strength: row.strength, jumping: row.jumping, reflexes: row.reflexes, handling: row.handling, one_on_ones: row.one_on_ones, positioning_gk: row.positioning_gk, rushing_out: row.rushing_out, throwing: row.throwing, kicking: row.kicking, professionalism: row.professionalism, consistency: row.consistency, important_matches: row.important_matches, injury_proneness: row.injury_proneness })
}

pub async fn update_player_attributes(pool: &SqlitePool, player_id: i64, attrs: &PlayerAttributes) -> Result<(), String> {
    let (por,cie,ala,piv,uni) = match attrs.position.as_str() { "POR"=>(20,2,1,1,3), "CIE"=>(1,20,12,8,10), "ALA"=>(1,10,20,10,14), "PIV"=>(1,6,10,20,12), _=>(3,10,14,14,20) };
    sqlx::query("UPDATE player_positions SET por_natural=?, cie_natural=?, ala_natural=?, piv_natural=?, uni_natural=? WHERE player_id=?")
        .bind(por).bind(cie).bind(ala).bind(piv).bind(uni).bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE player_states SET current_ability=?, potential_ability=? WHERE player_id=?").bind(attrs.ca).bind(attrs.pa).bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query(
        "UPDATE player_attributes SET first_touch=?, dribbling=?, ball_control=?, technique=?, passing=?, vision=?, crossing=?, long_shots=?, finishing=?, heading=?, penalty_taking=?, tackling=?, marking=?, interception=?, blocking=?, anticipation=?, decisions=?, positioning=?, off_the_ball=?, work_rate=?, composure=?, concentration=?, determination=?, bravery=?, aggression=?, leadership=?, teamwork=?, flair=?, acceleration=?, pace=?, agility=?, balance=?, stamina=?, strength=?, jumping=?, reflexes=?, handling=?, one_on_ones=?, positioning_gk=?, rushing_out=?, throwing=?, kicking=?, professionalism=?, consistency=?, important_matches=?, injury_proneness=? WHERE player_id=?"
    )
    .bind(attrs.first_touch).bind(attrs.dribbling).bind(attrs.ball_control).bind(attrs.technique).bind(attrs.passing).bind(attrs.vision).bind(attrs.crossing).bind(attrs.long_shots).bind(attrs.finishing).bind(attrs.heading).bind(attrs.penalty_taking).bind(attrs.tackling).bind(attrs.marking).bind(attrs.interception).bind(attrs.blocking).bind(attrs.anticipation).bind(attrs.decisions).bind(attrs.positioning).bind(attrs.off_the_ball).bind(attrs.work_rate).bind(attrs.composure).bind(attrs.concentration).bind(attrs.determination).bind(attrs.bravery).bind(attrs.aggression).bind(attrs.leadership).bind(attrs.teamwork).bind(attrs.flair).bind(attrs.acceleration).bind(attrs.pace).bind(attrs.agility).bind(attrs.balance).bind(attrs.stamina).bind(attrs.strength).bind(attrs.jumping).bind(attrs.reflexes).bind(attrs.handling).bind(attrs.one_on_ones).bind(attrs.positioning_gk).bind(attrs.rushing_out).bind(attrs.throwing).bind(attrs.kicking).bind(attrs.professionalism).bind(attrs.consistency).bind(attrs.important_matches).bind(attrs.injury_proneness).bind(player_id)
    .execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}

fn base64_decode(s: &str) -> Option<Vec<u8>> {
    use base64::Engine as _;
    let s = s.trim();
    if let Some(i) = s.find(',') { // data URL
        base64::engine::general_purpose::STANDARD.decode(&s[i+1..]).ok()
    } else {
        base64::engine::general_purpose::STANDARD.decode(s).ok()
    }
}
