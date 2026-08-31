use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize, Clone)]
pub struct ContractRow {
    pub player_id: i64,
    pub player_name: String,
    pub position: String,
    pub wage: f64,
    pub end_date: String,
    pub release_clause: Option<f64>,
    pub role: String,
    pub signing_bonus: f64,
    pub appearance_bonus: f64,
    pub clean_sheet_bonus: f64,
    pub renewal_status: String,
    pub loan_parent_id: Option<i64>,
    pub loan_until: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct MarketPlayer {
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub nation: String,
    pub position: String,
    pub secondary_position: Option<String>,
    pub flag_path: Option<String>,
    pub second_flag_path: Option<String>,
    pub ca: i64,
    pub pa: i64,
    pub club_id: i64,
    pub club_name: String,
    pub club_short: String,
    pub value: f64,
    pub wage: f64,
    pub contract_end: String,
    pub knowledge: i64,
}

#[derive(Serialize, Clone)]
pub struct LoanRow {
    pub player_id: i64,
    pub player_name: String,
    pub from_club: String,
    pub to_club: String,
    pub loan_until: String,
    pub wage: f64,
}

#[derive(Serialize, Clone)]
pub struct OfferRow {
    pub id: i64,
    pub player_id: i64,
    pub player_name: String,
    pub from_club: String,
    pub from_club_id: i64,
    pub to_club: String,
    pub to_club_id: i64,
    pub fee: f64,
    pub status: String,
    pub date: String,
}

pub fn calculate_player_value(ca: i64, pa: i64, age: i64, contract_years: i64) -> f64 {
    let base = (ca as f64).powf(1.8) * 8.0;
    let age_mod = match age {
        15..=20 => 1.4,
        21..=27 => 1.0,
        28..=32 => 0.75,
        _ => 0.45,
    };
    let gap = pa - ca;
    let pot_mod = if gap > 25 { 1.35 } else if gap > 10 { 1.15 } else { 1.0 };
    let contract_mod = if contract_years < 1 { 0.6 } else if contract_years < 2 { 0.8 } else { 1.0 };
    (base * age_mod * pot_mod * contract_mod).round()
}

pub async fn get_loans(pool: &SqlitePool, club_id: i64) -> Result<Vec<LoanRow>, String> {
    let rows = sqlx::query_as::<_, (i64,String,String,String,String,f64)>("SELECT c.player_id,p.common_name,origin.name,dest.name,c.loan_until,c.wage_weekly FROM contracts c JOIN players p ON p.id=c.player_id JOIN contracts parent ON parent.id=c.loan_parent_id JOIN clubs origin ON origin.id=parent.club_id JOIN clubs dest ON dest.id=c.club_id WHERE c.loan_parent_id IS NOT NULL AND (c.club_id=? OR parent.club_id=?) AND c.is_active=1").bind(club_id).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(player_id,player_name,from_club,to_club,loan_until,wage)| LoanRow { player_id,player_name,from_club,to_club,loan_until,wage }).collect())
}

pub async fn offer_loan(pool: &SqlitePool, player_id: i64, from_club: i64, to_club: i64, months: i64, wage: f64) -> Result<String, String> {
    if !(1..=24).contains(&months) || wage < 0.0 { return Err("Duración o salario de cesión no válidos".into()); }
    let parent: (i64,String) = sqlx::query_as("SELECT id,end_date FROM contracts WHERE player_id=? AND club_id=? AND is_active=1").bind(player_id).bind(from_club).fetch_one(pool).await.map_err(|_| "El jugador no pertenece al club de origen".to_string())?;
    if sqlx::query_as::<_,(i64,)>("SELECT COUNT(*) FROM contracts WHERE player_id=? AND is_active=1").bind(player_id).fetch_one(pool).await.map_err(|e|e.to_string())?.0 > 1 { return Err("El jugador ya está cedido".into()); }
    let (date,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e|e.to_string())?;
    let start=chrono::NaiveDate::parse_from_str(&date,"%Y-%m-%d").map_err(|e|e.to_string())?;
    let requested=start+chrono::Duration::days(months*30);
    let parent_end=chrono::NaiveDate::parse_from_str(&parent.1,"%Y-%m-%d").map_err(|e|e.to_string())?;
    let end=if requested<parent_end { requested } else { parent_end };
    sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active,loan_parent_id,loan_until) VALUES(?,?,?,?,?,1,?,?)").bind(player_id).bind(to_club).bind(wage).bind(&date).bind(end.format("%Y-%m-%d").to_string()).bind(parent.0).bind(end.format("%Y-%m-%d").to_string()).execute(pool).await.map_err(|e|e.to_string())?;
    Ok(format!("Cesión acordada hasta {}",end.format("%Y-%m-%d")))
}

#[derive(Serialize, Clone)]
pub struct PrecontractRow {
    pub id: i64,
    pub player_id: i64,
    pub player_name: String,
    pub from_club: String,
    pub to_club: String,
    pub start_date: String,
    pub end_date: String,
    pub wage_weekly: f64,
    pub signing_bonus: f64,
    pub status: String,
}

pub async fn get_precontracts(pool: &SqlitePool, club_id: i64) -> Result<Vec<PrecontractRow>, String> {
    let rows = sqlx::query_as::<_, (i64,i64,String,String,String,String,String,f64,f64,String)>(
        "SELECT pc.id,pc.player_id,p.common_name,fc.name,tc.name,pc.start_date,pc.end_date,pc.wage_weekly,pc.signing_bonus,pc.status FROM precontracts pc JOIN players p ON p.id=pc.player_id JOIN clubs fc ON fc.id=pc.from_club_id JOIN clubs tc ON tc.id=pc.to_club_id WHERE pc.from_club_id=? OR pc.to_club_id=? ORDER BY pc.start_date,pc.id"
    ).bind(club_id).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id,player_id,player_name,from_club,to_club,start_date,end_date,wage_weekly,signing_bonus,status)| PrecontractRow { id,player_id,player_name,from_club,to_club,start_date,end_date,wage_weekly,signing_bonus,status }).collect())
}

pub async fn make_precontract(pool: &SqlitePool, player_id: i64, to_club: i64, wage: f64, signing_bonus: f64, years: i64) -> Result<String, String> {
    if !wage.is_finite() || wage <= 0.0 || !signing_bonus.is_finite() || signing_bonus < 0.0 || !(1..=5).contains(&years) { return Err("Condiciones de precontrato no válidas".into()); }
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let (from_club, end_date): (i64,String) = sqlx::query_as("SELECT club_id,end_date FROM contracts WHERE player_id=? AND is_active=1 AND loan_parent_id IS NULL")
        .bind(player_id).fetch_one(&mut *tx).await.map_err(|_| "El jugador no tiene un contrato activo válido".to_string())?;
    if from_club == to_club { return Err("El jugador ya pertenece a tu club".into()); }
    let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
    let current_end = chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let start = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d").map_err(|e| e.to_string())?;
    if current_end <= start { return Err("El contrato ya ha finalizado".into()); }
    sqlx::query("INSERT INTO precontracts(player_id,from_club_id,to_club_id,agreed_date,start_date,end_date,wage_weekly,signing_bonus) VALUES(?,?,?,?,?,?,?,?)")
        .bind(player_id).bind(from_club).bind(to_club).bind(&today).bind(current_end.format("%Y-%m-%d").to_string()).bind((current_end + chrono::Duration::days(365*years)).format("%Y-%m-%d").to_string()).bind(wage).bind(signing_bonus).execute(&mut *tx).await.map_err(|e| if e.to_string().contains("UNIQUE") { "Ya existe un precontrato para este jugador y club".into() } else { e.to_string() })?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(format!("Precontrato acordado; comenzará el {}", current_end.format("%Y-%m-%d")))
}

pub async fn process_precontracts(pool: &SqlitePool) -> Result<(), String> {
    let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let rows = sqlx::query_as::<_, (i64,i64,i64,i64,String,f64,f64)>("SELECT id,player_id,from_club_id,to_club_id,end_date,wage_weekly,signing_bonus FROM precontracts WHERE status='pending' AND start_date<=?").bind(&today).fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (id, player_id, from_club, to_club, end_date, wage, signing_bonus) in rows {
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE contracts SET is_active=0 WHERE player_id=? AND is_active=1").bind(player_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        let end = chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap());
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active,signing_bonus) VALUES(?,?,?,?,?,1,?)").bind(player_id).bind(to_club).bind(wage).bind(&today).bind(end.format("%Y-%m-%d").to_string()).bind(signing_bonus).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE precontracts SET status='completed' WHERE id=?").bind(id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE club_finances SET total_wages=(SELECT COALESCE(SUM(wage_weekly),0) FROM contracts WHERE club_id=? AND is_active=1),balance=balance-? WHERE club_id=?").bind(to_club).bind(signing_bonus).bind(to_club).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE club_finances SET total_wages=(SELECT COALESCE(SUM(wage_weekly),0) FROM contracts WHERE club_id=? AND is_active=1) WHERE club_id=?").bind(from_club).bind(from_club).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn get_free_agents(pool: &SqlitePool, _user_club: i64) -> Result<Vec<MarketPlayer>, String> {
    let rows = sqlx::query_as::<_, (i64, String, String, Option<String>, Option<String>, String, i64, i64, f64)>("SELECT p.id,p.common_name,n.name,n.flag_path,n2.flag_path,COALESCE(pos.position,'UNI'),ps.current_ability,ps.potential_ability,COALESCE(last.wage_weekly,500) FROM players p JOIN player_states ps ON ps.player_id=p.id JOIN nations n ON n.id=p.nation_id LEFT JOIN nations n2 ON n2.id=p.second_nation_id LEFT JOIN (SELECT player_id,MAX(id) id FROM contracts GROUP BY player_id) latest ON latest.player_id=p.id LEFT JOIN contracts last ON last.id=latest.id LEFT JOIN (SELECT player_id,CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END position FROM player_positions) pos ON pos.player_id=p.id WHERE p.id NOT IN (SELECT player_id FROM contracts WHERE is_active=1) AND p.is_retired=0 ORDER BY ps.current_ability DESC LIMIT 50").fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(| (id,name,nation,flag_path,second_flag_path,position,ca,pa,wage)| MarketPlayer { id,name,age:25,nation:nation.clone(),position,secondary_position:None,flag_path,second_flag_path,ca,pa,club_id:0,club_name:"Agente libre".into(),club_short:"LIBRE".into(),value:0.0,wage,contract_end:"Libre".into(),knowledge:100 }).collect())
}

pub async fn sign_free_agent(pool: &SqlitePool, player_id: i64, club_id: i64, wage: f64, years: i64) -> Result<String, String> {
    if wage <= 0.0 || !(1..=5).contains(&years) { return Err("Condiciones no válidas".into()); }
    let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts WHERE player_id=? AND is_active=1").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if exists.0 > 0 { return Err("El jugador ya tiene contrato".into()); }
    let (date,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let start = chrono::NaiveDate::parse_from_str(&date,"%Y-%m-%d").map_err(|e| e.to_string())?;
    let end = start + chrono::Duration::days(365*years);
    sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(player_id).bind(club_id).bind(wage).bind(&date).bind(end.format("%Y-%m-%d").to_string()).execute(pool).await.map_err(|e| e.to_string())?;
    Ok("Jugador libre incorporado".into())
}

pub async fn get_market(pool: &SqlitePool, user_club: i64) -> Result<Vec<MarketPlayer>, String> {
    let rows = sqlx::query_as::<_, (i64, String, String, Option<String>, Option<String>, String, i64, i64, i64, String, String, f64, String)>(
        "SELECT p.id, p.common_name, n.name, n.flag_path, n2.flag_path, COALESCE(pp.pos,'UNI'), ps.current_ability, ps.potential_ability, c.club_id, cl.name, cl.short_name, c.wage_weekly, c.end_date FROM players p JOIN contracts c ON c.player_id=p.id AND c.is_active=1 JOIN player_states ps ON ps.player_id=p.id JOIN nations n ON n.id=p.nation_id LEFT JOIN nations n2 ON n2.id=p.second_nation_id JOIN clubs cl ON cl.id=c.club_id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp ON pp.player_id=p.id WHERE c.club_id != ? ORDER BY ps.current_ability DESC LIMIT 40"
    ).bind(user_club).fetch_all(pool).await.map_err(|e| e.to_string())?;

    let today: chrono::NaiveDate = sqlx::query_as::<_, (String,)>("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string()).and_then(|(d,)| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").map_err(|e| e.to_string()))?;

    let mut out = Vec::new();
    for (id, name, nation, flag_path, second_flag_path, pos, ca, pa, club_id, club_name, club_short, wage, end) in rows {
        let dob: String = sqlx::query_as::<_, (String,)>("SELECT date_of_birth FROM players WHERE id=?").bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?.0;
        let age = chrono::NaiveDate::parse_from_str(&dob, "%Y-%m-%d").map(|d| ((today - d).num_days()/365) as i64).unwrap_or(25);
        let end_date = chrono::NaiveDate::parse_from_str(&end, "%Y-%m-%d").unwrap_or(today);
        let years = ((end_date - today).num_days() as f64 / 365.0).ceil() as i64;
        let value = calculate_player_value(ca, pa, age, years.max(0));
        let knowledge: i64 = sqlx::query_as("SELECT COALESCE(knowledge_percentage,0) FROM player_knowledge WHERE club_id=? AND player_id=?").bind(user_club).bind(id).fetch_optional(pool).await.map_err(|e| e.to_string())?.map(|(k,): (i64,)| k).unwrap_or(0);
        if knowledge >= 20 || rand::random::<f64>() < 0.5 {
            let (display_ca, display_pa) = if knowledge >= 80 { (ca, pa) } else if knowledge >= 50 { ((ca / 10) * 10, (pa / 10) * 10) } else { (0, 0) };
            out.push(MarketPlayer { id, name, age, nation:nation.clone(), position: pos, secondary_position: None, flag_path, second_flag_path, ca: display_ca, pa: display_pa, club_id, club_name, club_short, value: if knowledge >= 50 { value } else { 0.0 }, wage, contract_end: end, knowledge });
        }
        if out.len() >= 20 { break; }
    }
    Ok(out)
}

pub async fn get_contracts(pool: &SqlitePool, club_id: i64) -> Result<Vec<ContractRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, String, f64, String, Option<f64>, String, f64, f64, f64, String, Option<i64>, Option<String>)>(
        "SELECT p.id, p.common_name, COALESCE(pos.position,'UNI'), c.wage_weekly, c.end_date, c.release_clause, c.contract_role, c.signing_bonus, c.appearance_bonus, c.clean_sheet_bonus, c.renewal_status, c.loan_parent_id, c.loan_until FROM contracts c JOIN players p ON p.id=c.player_id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END AS position FROM player_positions) pos ON pos.player_id=p.id WHERE c.club_id=? AND c.is_active=1 ORDER BY c.end_date ASC"
    ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(player_id, player_name, position, wage, end_date, release_clause, role, signing_bonus, appearance_bonus, clean_sheet_bonus, renewal_status, loan_parent_id, loan_until)| ContractRow { player_id, player_name, position, wage, end_date, release_clause, role, signing_bonus, appearance_bonus, clean_sheet_bonus, renewal_status, loan_parent_id, loan_until }).collect())
}

pub async fn renew_contract(pool: &SqlitePool, club_id: i64, player_id: i64, years: i64, wage: f64, release_clause: Option<f64>, role: String, signing_bonus: f64, appearance_bonus: f64, clean_sheet_bonus: f64) -> Result<String, String> {
    if !(1..=5).contains(&years) || !wage.is_finite() || wage <= 0.0 || [signing_bonus, appearance_bonus, clean_sheet_bonus].iter().any(|v| !v.is_finite() || *v < 0.0) || release_clause.is_some_and(|v| !v.is_finite() || v < 0.0) { return Err("Parámetros de contrato no válidos".into()); }
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let (contract_id, current_end): (i64, String) = sqlx::query_as("SELECT id, end_date FROM contracts WHERE player_id=? AND club_id=? AND is_active=1")
        .bind(player_id).bind(club_id).fetch_one(&mut *tx).await.map_err(|_| "El jugador no pertenece a tu club".to_string())?;
    let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
    let start = chrono::NaiveDate::parse_from_str(&current_end, "%Y-%m-%d").unwrap_or_else(|_| chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap());
    let end = start + chrono::Duration::days(365 * years);
    sqlx::query("UPDATE contracts SET wage_weekly=?, end_date=?, release_clause=?, contract_role=?, signing_bonus=?, appearance_bonus=?, clean_sheet_bonus=?, renewal_status='accepted' WHERE id=?")
        .bind(wage).bind(end.format("%Y-%m-%d").to_string()).bind(release_clause).bind(role).bind(signing_bonus).bind(appearance_bonus).bind(clean_sheet_bonus).bind(contract_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE club_finances SET total_wages=(SELECT COALESCE(SUM(wage_weekly),0) FROM contracts WHERE club_id=? AND is_active=1) WHERE club_id=?")
        .bind(club_id).bind(club_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(format!("Contrato renovado hasta {}", end.format("%Y-%m-%d")))
}

pub async fn get_offers(pool: &SqlitePool, club_id: i64) -> Result<Vec<OfferRow>, String> {
    let rows = sqlx::query_as::<_, (i64, i64, String, i64, String, i64, String, f64, String, String)>(
        "SELECT o.id, o.player_id, p.common_name, o.from_club_id, cf.name, o.to_club_id, ct.name, o.offered_fee, o.status, o.date_offered FROM transfer_offers o JOIN players p ON p.id=o.player_id JOIN clubs cf ON cf.id=o.from_club_id JOIN clubs ct ON ct.id=o.to_club_id WHERE o.from_club_id=? OR o.to_club_id=? ORDER BY o.id DESC LIMIT 30"
    ).bind(club_id).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, pid, pname, from_id, from_name, to_id, to_name, fee, status, date)| OfferRow { id, player_id: pid, player_name: pname, from_club: from_name, from_club_id: from_id, to_club: to_name, to_club_id: to_id, fee, status, date }).collect())
}

pub async fn make_offer(pool: &SqlitePool, player_id: i64, from_club: i64, offered_fee: f64) -> Result<String, String> {
    let (current_club,): (i64,) = sqlx::query_as("SELECT club_id FROM contracts WHERE player_id=? AND is_active=1").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if current_club == from_club { return Err("El jugador ya es de tu club".into()); }

    let (ca,): (i64,) = sqlx::query_as("SELECT current_ability FROM player_states WHERE player_id=?").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (pa,): (i64,) = sqlx::query_as("SELECT potential_ability FROM player_states WHERE player_id=?").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (dob,): (String,) = sqlx::query_as("SELECT date_of_birth FROM players WHERE id=?").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (end,): (String,) = sqlx::query_as("SELECT end_date FROM contracts WHERE player_id=? AND is_active=1").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (today_s,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let today = chrono::NaiveDate::parse_from_str(&today_s, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let dob_d = chrono::NaiveDate::parse_from_str(&dob, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let age = ((today - dob_d).num_days()/365) as i64;
    let end_d = chrono::NaiveDate::parse_from_str(&end, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let years = ((end_d - today).num_days() as f64 /365.0).ceil() as i64;
    let value = calculate_player_value(ca, pa, age, years.max(0));

    let (balance,): (f64,) = sqlx::query_as("SELECT transfer_budget FROM club_finances WHERE club_id=?").bind(from_club).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if offered_fee > balance { return Err(format!("Presupuesto insuficiente: €{:.0} disponible, oferta €{:.0}", balance, offered_fee)); }

    let decision = if offered_fee >= value * 0.85 { "accepted" } else if offered_fee >= value * 0.6 { "pending" } else { "rejected" };

    let date = today.format("%Y-%m-%d").to_string();
    sqlx::query("INSERT INTO transfer_offers(player_id, from_club_id, to_club_id, offered_fee, status, date_offered) VALUES(?,?,?,?,?,?)")
        .bind(player_id).bind(current_club).bind(from_club).bind(offered_fee).bind(decision).bind(&date)
        .execute(pool).await.map_err(|e| e.to_string())?;

    if decision == "accepted" {
        execute_transfer(pool, player_id, current_club, from_club, offered_fee, &date).await?;
        return Ok(format!("¡Oferta aceptada! {} fichado por €{:.0}", get_player_name(pool, player_id).await?, offered_fee));
    } else if decision == "rejected" {
        return Ok(format!("Oferta rechazada. Valor estimado: €{:.0}", value));
    }
    Ok(format!("Oferta enviada (€{:.0}). El club pide €{:.0}. Negociando...", offered_fee, value))
}

async fn get_player_name(pool: &SqlitePool, pid: i64) -> Result<String, String> {
    let (n,): (String,) = sqlx::query_as("SELECT common_name FROM players WHERE id=?").bind(pid).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(n)
}

pub async fn execute_transfer(pool: &SqlitePool, player_id: i64, from_club: i64, to_club: i64, fee: f64, date: &str) -> Result<(), String> {
    sqlx::query("UPDATE contracts SET is_active=0 WHERE player_id=? AND is_active=1").bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    let (wage,): (f64,) = sqlx::query_as("SELECT wage_weekly FROM contracts WHERE player_id=? ORDER BY id DESC LIMIT 1").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let new_wage = wage * 1.1;
    let end = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|e| e.to_string())? + chrono::Duration::days(365*3);
    sqlx::query("INSERT INTO contracts(player_id, club_id, wage_weekly, start_date, end_date, is_active) VALUES(?,?,?,?,?,1)")
        .bind(player_id).bind(to_club).bind(new_wage).bind(date).bind(end.format("%Y-%m-%d").to_string())
        .execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO transfer_history(player_id, from_club_id, to_club_id, transfer_date, transfer_fee) VALUES(?,?,?,?,?)")
        .bind(player_id).bind(from_club).bind(to_club).bind(date).bind(fee)
        .execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE club_finances SET balance=balance-?, transfer_budget=transfer_budget-? WHERE club_id=?").bind(fee).bind(fee).bind(to_club).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE club_finances SET balance=balance+? WHERE club_id=?").bind(fee).bind(from_club).execute(pool).await.map_err(|e| e.to_string())?;
    let pname = get_player_name(pool, player_id).await?;
    let from_name: (String,) = sqlx::query_as("SELECT name FROM clubs WHERE id=?").bind(from_club).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let to_name: (String,) = sqlx::query_as("SELECT name FROM clubs WHERE id=?").bind(to_club).fetch_one(pool).await.map_err(|e| e.to_string())?;
    for (cid, subject, body) in [
        (to_club, format!("Fichaje: {}", pname), format!("{} se une desde {} por €{:.0}. Contrato 3 años.", pname, from_name.0, fee)),
        (from_club, format!("Venta: {}", pname), format!("{} vendido a {} por €{:.0}", pname, to_name.0, fee)),
    ] {
        sqlx::query("INSERT INTO inbox_messages(club_id, sender_type, subject, body, date_sent) VALUES(?, 'board', ?, ?, ?)")
            .bind(cid).bind(subject).bind(body).bind(date).execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn respond_offer(pool: &SqlitePool, offer_id: i64, accept: bool) -> Result<String, String> {
    let row: Option<(i64, i64, i64, f64, String)> = sqlx::query_as("SELECT player_id, from_club_id, to_club_id, offered_fee, status FROM transfer_offers WHERE id=?").bind(offer_id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
    let (pid, from_c, to_c, fee, status) = row.ok_or("Oferta no encontrada")?;
    if status != "pending" { return Err(format!("Oferta ya {}", status)); }
    let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    if accept {
        sqlx::query("UPDATE transfer_offers SET status='accepted' WHERE id=?").bind(offer_id).execute(pool).await.map_err(|e| e.to_string())?;
        execute_transfer(pool, pid, from_c, to_c, fee, &today).await?;
        Ok("Transferencia completada".into())
    } else {
        sqlx::query("UPDATE transfer_offers SET status='rejected' WHERE id=?").bind(offer_id).execute(pool).await.map_err(|e| e.to_string())?;
        Ok("Oferta rechazada".into())
    }
}

pub async fn generate_incoming_offers(pool: &SqlitePool, user_club: i64) -> Result<(), String> {
    if rand::random::<f64>() > 0.15 { return Ok(()); }
    let candidates = sqlx::query_as::<_, (i64,)>("SELECT player_id FROM contracts WHERE club_id=? AND is_active=1 ORDER BY RANDOM() LIMIT 3").bind(user_club).fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (pid,) in candidates {
        if rand::random::<f64>() < 0.3 {
            let other: Option<(i64,)> = sqlx::query_as("SELECT id FROM clubs WHERE id != ? ORDER BY RANDOM() LIMIT 1").bind(user_club).fetch_optional(pool).await.map_err(|e| e.to_string())?;
            if let Some((other_id,)) = other {
                let (ca,): (i64,) = sqlx::query_as("SELECT current_ability FROM player_states WHERE player_id=?").bind(pid).fetch_one(pool).await.map_err(|e| e.to_string())?;
                let fee = (ca as f64 * 120.0 + rand::random::<f64>()*20000.0).round();
                let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
                let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM transfer_offers WHERE player_id=? AND status='pending'").bind(pid).fetch_one(pool).await.map_err(|e| e.to_string())?;
                if exists.0 == 0 {
                    sqlx::query("INSERT INTO transfer_offers(player_id, from_club_id, to_club_id, offered_fee, status, date_offered) VALUES(?,?,?,?,'pending',?)")
                        .bind(pid).bind(user_club).bind(other_id).bind(fee).bind(&today).execute(pool).await.map_err(|e| e.to_string())?;
                    let pname = get_player_name(pool, pid).await?;
                    let oname: (String,) = sqlx::query_as("SELECT name FROM clubs WHERE id=?").bind(other_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
                    sqlx::query("INSERT INTO inbox_messages(club_id, sender_type, subject, body, date_sent, is_important) VALUES(?,'board',?, ?, ?,1)")
                        .bind(user_club).bind(format!("Oferta por {}", pname)).bind(format!("{} ofrece €{:.0} por {}. Responde en Mercado.", oname.0, fee, pname)).bind(&today).execute(pool).await.map_err(|e| e.to_string())?;
                }
            }
        }
    }
    Ok(())
}
