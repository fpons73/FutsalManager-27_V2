use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize, Clone)]
pub struct FinanceRow {
    pub club_id: i64,
    pub club_name: String,
    pub balance: f64,
    pub transfer_budget: f64,
    pub wage_budget: f64,
    pub total_wages: f64,
    pub sponsorship: f64,
    pub ticket_income: f64,
    pub prize_money: f64,
    pub weekly_wages: f64,
    pub monthly_balance: f64,
    pub stadium_condition: i64,
    pub stadium_weekly_cost: f64,
    pub stadium_name: Option<String>,
    pub staff_weekly_cost: f64,
    pub staff_count: i64,
}

pub async fn get_finance(pool: &SqlitePool, club_id: i64) -> Result<FinanceRow, String> {
    let row: (String, f64, f64, f64, f64, f64, f64, f64) = sqlx::query_as(
        "SELECT c.name, f.balance, f.transfer_budget, f.wage_budget, f.total_wages, f.sponsorship_income, f.ticket_income, f.prize_money FROM clubs c JOIN club_finances f ON f.club_id=c.id WHERE c.id=?"
    ).bind(club_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let weekly = row.4 / 52.0 * 52.0;
    let monthly = row.5 / 12.0 + row.6 / 30.0 * 4.0 - row.4 / 4.33;
    let stadium: Option<(String, i64, f64)> = sqlx::query_as("SELECT s.name, so.condition, so.weekly_cost FROM clubs c JOIN stadiums s ON s.id=c.stadium_id LEFT JOIN stadium_operations so ON so.stadium_id=s.id WHERE c.id=?").bind(club_id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
    let (stadium_name, stadium_condition, stadium_weekly_cost) = stadium.map(|(n,c,cost)|(Some(n),c,cost)).unwrap_or((None,100,0.0));
    let staff: (f64, i64) = sqlx::query_as("SELECT COALESCE(weekly_cost,0), COALESCE(staff_count,0) FROM club_staff_costs WHERE club_id=?").bind(club_id).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((0.0, 0));
    Ok(FinanceRow { club_id, club_name: row.0, balance: row.1, transfer_budget: row.2, wage_budget: row.3, total_wages: row.4, sponsorship: row.5, ticket_income: row.6, prize_money: row.7, weekly_wages: weekly, monthly_balance: monthly, stadium_condition, stadium_weekly_cost, stadium_name, staff_weekly_cost: staff.0, staff_count: staff.1 })
}

pub async fn process_stadium_operations(pool: &SqlitePool) -> Result<(), String> { let rows:Vec<(i64,i64,f64)>=sqlx::query_as("SELECT so.stadium_id,so.condition,so.weekly_cost FROM stadium_operations so JOIN clubs c ON c.stadium_id=so.stadium_id").fetch_all(pool).await.map_err(|e|e.to_string())?;let(date,):(String,)=sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e|e.to_string())?;for(stadium,condition,cost)in rows{let club:Option<(i64,)>=sqlx::query_as("SELECT id FROM clubs WHERE stadium_id=?").bind(stadium).fetch_optional(pool).await.map_err(|e|e.to_string())?;if let Some((club_id,))=club{sqlx::query("UPDATE club_finances SET balance=balance-? WHERE club_id=?").bind(cost).bind(club_id).execute(pool).await.map_err(|e|e.to_string())?;sqlx::query("UPDATE stadium_operations SET condition=MAX(0,condition-1),last_maintenance=? WHERE stadium_id=?").bind(&date).bind(stadium).execute(pool).await.map_err(|e|e.to_string())?;if condition<=30{let _=sqlx::query("INSERT OR IGNORE INTO inbox_messages(club_id,sender_type,subject,body,date_sent,is_important) VALUES(?,'board','Mantenimiento urgente','El pabellón necesita mantenimiento para evitar pérdida de ingresos.',?,1)").bind(club_id).bind(&date).execute(pool).await;}}}Ok(())}

pub async fn process_weekly_finances(pool: &SqlitePool) -> Result<(), String> {
    let clubs: Vec<(i64,)> = sqlx::query_as("SELECT club_id FROM club_finances").fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (cid,) in clubs {
        let (wages,): (f64,) = sqlx::query_as("SELECT total_wages FROM club_finances WHERE club_id=?").bind(cid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let staff: (f64, i64) = sqlx::query_as("SELECT COALESCE(SUM(wage_weekly),0), COUNT(*) FROM staff WHERE club_id=?").bind(cid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let today: (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
        sqlx::query("INSERT INTO club_staff_costs(club_id,weekly_cost,staff_count,updated_at) VALUES(?,?,?,?,?) ON CONFLICT(club_id) DO UPDATE SET weekly_cost=excluded.weekly_cost,staff_count=excluded.staff_count,updated_at=excluded.updated_at")
            .bind(cid).bind(staff.0).bind(staff.1).bind(&today.0).execute(pool).await.map_err(|e| e.to_string())?;
        let weekly_cost = wages + staff.0;
        sqlx::query("UPDATE club_finances SET balance=balance-?, sponsorship_income=sponsorship_income+? WHERE club_id=?")
            .bind(weekly_cost).bind(15000.0 + (cid as f64 * 200.0)).bind(cid).execute(pool).await.map_err(|e| e.to_string())?;

        // Low balance warning
        let (bal,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(cid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if bal < 0.0 {
            let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
            let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM inbox_messages WHERE club_id=? AND subject='Alerta financiera' AND date_sent=?").bind(cid).bind(&today).fetch_one(pool).await.map_err(|e| e.to_string())?;
            if exists.0 == 0 {
                sqlx::query("INSERT INTO inbox_messages(club_id, sender_type, subject, body, date_sent, is_important) VALUES(?,'board','Alerta financiera','Balance negativo: €'||?, ?,1)")
                    .bind(cid).bind(bal.round()).bind(&today).execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

pub async fn add_ticket_income(pool: &SqlitePool, club_id: i64, amount: f64) -> Result<(), String> {
    sqlx::query("UPDATE club_finances SET balance=balance+?, ticket_income=ticket_income+? WHERE club_id=?").bind(amount).bind(amount).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn check_transfer_budget(pool: &SqlitePool, club_id: i64, fee: f64) -> Result<bool, String> {
    let (budget,): (f64,) = sqlx::query_as("SELECT transfer_budget FROM club_finances WHERE club_id=?").bind(club_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(fee <= budget)
}
