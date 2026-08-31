use serde::Serialize;
use sqlx::SqlitePool;
use chrono::NaiveDate;

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
    pub travel_spend: f64,
    pub away_matches: i64,
    pub merchandise_revenue: f64,
    pub merchandise_units: i64,
    pub merchandise_demand: i64,
    pub tv_weekly_income: f64,
    pub tv_broadcaster: Option<String>,
    pub tv_contract_end: Option<String>,
    pub sponsor_name: Option<String>,
    pub sponsor_weekly_income: f64,
    pub sponsor_contract_end: Option<String>,
    pub sponsor_target: Option<i64>,
    pub training_facility_level: i64,
    pub youth_facility_level: i64,
    pub commercial_facility_level: i64,
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
    let travel: (f64, i64) = sqlx::query_as("SELECT COALESCE(travel_spend,0), COALESCE(away_matches,0) FROM club_travel_finance WHERE club_id=?").bind(club_id).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((0.0, 0));
    let merch: (f64, i64, i64) = sqlx::query_as("SELECT COALESCE(total_revenue,0), COALESCE(total_units,0), COALESCE(demand,50) FROM club_merchandising WHERE club_id=?").bind(club_id).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((0.0, 0, 50));
    let tv: (String, f64, String) = sqlx::query_as("SELECT broadcaster, weekly_amount, end_date FROM tv_rights_contracts WHERE club_id=? AND status='active' ORDER BY id DESC LIMIT 1").bind(club_id).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or(("Sin contrato".into(), 0.0, "—".into()));
    let facilities: (i64, i64, i64) = sqlx::query_as("SELECT training_level,youth_level,commercial_level FROM club_facilities WHERE club_id=?").bind(club_id).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((1,1,1));
    let sponsor: (String, f64, String, i64) = sqlx::query_as("SELECT sponsor_name, weekly_amount, end_date, target_value FROM sponsorship_contracts WHERE club_id=? AND status='active' ORDER BY id DESC LIMIT 1").bind(club_id).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or(("Sin patrocinador".into(), 0.0, "—".into(), 0));
    Ok(FinanceRow { club_id, club_name: row.0, balance: row.1, transfer_budget: row.2, wage_budget: row.3, total_wages: row.4, sponsorship: row.5, ticket_income: row.6, prize_money: row.7, weekly_wages: weekly, monthly_balance: monthly, stadium_condition, stadium_weekly_cost, stadium_name, staff_weekly_cost: staff.0, staff_count: staff.1, travel_spend: travel.0, away_matches: travel.1, merchandise_revenue: merch.0, merchandise_units: merch.1, merchandise_demand: merch.2, tv_weekly_income: tv.1, tv_broadcaster: Some(tv.0), tv_contract_end: Some(tv.2), sponsor_name: Some(sponsor.0), sponsor_weekly_income: sponsor.1, sponsor_contract_end: Some(sponsor.2), sponsor_target: Some(sponsor.3), training_facility_level: facilities.0, youth_facility_level: facilities.1, commercial_facility_level: facilities.2 })
}

pub async fn upgrade_facility(pool: &SqlitePool, club_id: i64, facility: &str) -> Result<String, String> {
    let column = match facility {
        "training" => "training_level",
        "youth" => "youth_level",
        "commercial" => "commercial_level",
        _ => return Err("Instalación no válida".into()),
    };
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let (today,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1")
        .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT OR IGNORE INTO club_facilities(club_id, updated_at) VALUES(?, ?)")
        .bind(club_id).bind(&today).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    let cost = sqlx::query_scalar::<_, Option<f64>>(&format!(
        "SELECT CASE {} WHEN 1 THEN 75000.0 WHEN 2 THEN 150000.0 WHEN 3 THEN 300000.0 WHEN 4 THEN 600000.0 ELSE NULL END FROM club_facilities WHERE club_id=?",
        column
    )).bind(club_id).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?
        .ok_or("La instalación ya está al máximo")?;
    let (balance,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?")
        .bind(club_id).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
    if balance < cost { return Err(format!("Fondos insuficientes: necesitas €{:.0}", cost)); }
    sqlx::query(&format!(
        "UPDATE club_facilities SET {}={}+1,updated_at=? WHERE club_id=?",
        column, column
    )).bind(&today).bind(club_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE club_finances SET balance=balance-? WHERE club_id=?")
        .bind(cost).bind(club_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(format!("Mejora de {} completada por €{:.0}", facility, cost))
}

pub async fn process_stadium_operations(pool: &SqlitePool) -> Result<(), String> { let rows:Vec<(i64,i64,f64)>=sqlx::query_as("SELECT so.stadium_id,so.condition,so.weekly_cost FROM stadium_operations so JOIN clubs c ON c.stadium_id=so.stadium_id").fetch_all(pool).await.map_err(|e|e.to_string())?;let(date,):(String,)=sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e|e.to_string())?;for(stadium,condition,cost)in rows{let club:Option<(i64,)>=sqlx::query_as("SELECT id FROM clubs WHERE stadium_id=?").bind(stadium).fetch_optional(pool).await.map_err(|e|e.to_string())?;if let Some((club_id,))=club{sqlx::query("UPDATE club_finances SET balance=balance-? WHERE club_id=?").bind(cost).bind(club_id).execute(pool).await.map_err(|e|e.to_string())?;sqlx::query("UPDATE stadium_operations SET condition=MAX(0,condition-1),last_maintenance=? WHERE stadium_id=?").bind(&date).bind(stadium).execute(pool).await.map_err(|e|e.to_string())?;if condition<=30{let _=sqlx::query("INSERT OR IGNORE INTO inbox_messages(club_id,sender_type,subject,body,date_sent,is_important) VALUES(?,'board','Mantenimiento urgente','El pabellón necesita mantenimiento para evitar pérdida de ingresos.',?,1)").bind(club_id).bind(&date).execute(pool).await;}}}Ok(())}

pub fn merchandise_demand(reputation: i64, commercial_level: i64) -> i64 {
    (reputation / 10 + 35 + (commercial_level.clamp(1, 5) - 1) * 5).clamp(10, 100)
}

pub async fn process_weekly_finances(pool: &SqlitePool) -> Result<(), String> {
    let clubs: Vec<(i64,)> = sqlx::query_as("SELECT club_id FROM club_finances").fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (cid,) in clubs {
        let (wages,): (f64,) = sqlx::query_as("SELECT total_wages FROM club_finances WHERE club_id=?").bind(cid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let staff: (f64, i64) = sqlx::query_as("SELECT COALESCE(SUM(wage_weekly),0), COUNT(*) FROM staff WHERE club_id=?").bind(cid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let today: (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
        sqlx::query("INSERT INTO club_staff_costs(club_id,weekly_cost,staff_count,updated_at) VALUES(?,?,?,?) ON CONFLICT(club_id) DO UPDATE SET weekly_cost=excluded.weekly_cost,staff_count=excluded.staff_count,updated_at=excluded.updated_at")
            .bind(cid).bind(staff.0).bind(staff.1).bind(&today.0).execute(pool).await.map_err(|e| e.to_string())?;
        let weekly_cost = wages + staff.0;
        let (today_date,) = (today.0.clone(),);
        let tv_contract: Option<(i64, f64, String, String)> = sqlx::query_as("SELECT id, weekly_amount, end_date, broadcaster FROM tv_rights_contracts WHERE club_id=? AND status='active' AND start_date<=? AND end_date>=? ORDER BY id DESC LIMIT 1").bind(cid).bind(&today_date).bind(&today_date).fetch_optional(pool).await.map_err(|e| e.to_string())?;
        let mut tv_income = 0.0;
        let mut sponsor_income = 0.0;
        if let Some((contract_id, weekly_amount, end_date, broadcaster)) = tv_contract {
            let payment = sqlx::query("INSERT INTO tv_rights_payments(contract_id,club_id,week_date,amount) VALUES(?,?,?,?) ON CONFLICT(contract_id,week_date) DO NOTHING").bind(contract_id).bind(cid).bind(&today_date).bind(weekly_amount).execute(pool).await.map_err(|e| e.to_string())?;
            if payment.rows_affected() > 0 { tv_income = weekly_amount; }
            if (NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").ok().map(|d| d.signed_duration_since(NaiveDate::parse_from_str(&today_date, "%Y-%m-%d").unwrap()).num_days()).unwrap_or(999)) <= 60 {
                let _ = sqlx::query("INSERT OR IGNORE INTO inbox_messages(club_id,sender_type,subject,body,date_sent,is_important) VALUES(?,'commercial','Derechos televisivos próximos a vencer',? ,?,1)").bind(cid).bind(format!("El contrato con {} vence el {}. Negocia una renovación para no perder ingresos.", broadcaster, end_date)).bind(&today_date).execute(pool).await;
            }
        }
        let sponsor: Option<(i64, f64, String, String, i64)> = sqlx::query_as("SELECT id, weekly_amount, end_date, sponsor_name, target_value FROM sponsorship_contracts WHERE club_id=? AND status='active' AND start_date<=? AND end_date>=? ORDER BY id DESC LIMIT 1").bind(cid).bind(&today_date).bind(&today_date).fetch_optional(pool).await.map_err(|e| e.to_string())?;
        let sponsor = match sponsor {
            Some(value) => Some(value),
            None => {
                sqlx::query("INSERT OR IGNORE INTO sponsorship_contracts(club_id,sponsor_name,weekly_amount,signing_bonus,target_type,target_value,start_date,end_date) VALUES(?, 'Velocity Sportswear', MAX(180.0,(SELECT reputation FROM clubs WHERE id=?)*1.8), MAX(500.0,(SELECT reputation FROM clubs WHERE id=?)*4.0), 'league_position', 8, ?, '2027-06-30')")
                    .bind(cid).bind(cid).bind(cid).bind(&today_date).execute(pool).await.map_err(|e| e.to_string())?;
                sqlx::query_as("SELECT id, weekly_amount, end_date, sponsor_name, target_value FROM sponsorship_contracts WHERE club_id=? AND status='active' ORDER BY id DESC LIMIT 1").bind(cid).fetch_optional(pool).await.map_err(|e| e.to_string())?
            }
        };
        if let Some((contract_id, amount, end_date, sponsor_name, target_value)) = sponsor {
            let payment = sqlx::query("INSERT INTO sponsorship_payments(contract_id,club_id,week_date,amount,target_met) VALUES(?,?,?,?,0) ON CONFLICT(contract_id,week_date) DO NOTHING").bind(contract_id).bind(cid).bind(&today_date).bind(amount).execute(pool).await.map_err(|e| e.to_string())?;
            if payment.rows_affected() > 0 {
                sponsor_income = amount;
                sqlx::query("UPDATE club_finances SET sponsorship_income=sponsorship_income+? WHERE club_id=?")
                    .bind(amount).bind(cid).execute(pool).await.map_err(|e| e.to_string())?;
            }
            let days = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").ok().zip(NaiveDate::parse_from_str(&today_date, "%Y-%m-%d").ok()).map(|(end, now)| end.signed_duration_since(now).num_days()).unwrap_or(999);
            if days <= 60 { let _ = sqlx::query("INSERT OR IGNORE INTO inbox_messages(club_id,sender_type,subject,body,date_sent,is_important) VALUES(?,'commercial','Patrocinio próximo a vencer',?,?,1)").bind(cid).bind(format!("El contrato con {} vence el {}. Revisa una renovación o nuevas ofertas.", sponsor_name, end_date)).bind(&today_date).execute(pool).await; }
            let _ = target_value;
        }
        sqlx::query("INSERT OR IGNORE INTO club_merchandising(club_id) VALUES(?)").bind(cid).execute(pool).await.map_err(|e| e.to_string())?;
        let (reputation,): (i64,) = sqlx::query_as("SELECT reputation FROM clubs WHERE id=?").bind(cid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let (commercial_level,): (i64,) = sqlx::query_as("SELECT COALESCE(commercial_level,1) FROM club_facilities WHERE club_id=?").bind(cid).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((1,));
        let demand = merchandise_demand(reputation, commercial_level);
        let (price, quality): (f64, i64) = sqlx::query_as("SELECT COALESCE(price,35), COALESCE(product_quality,60) FROM club_merchandising WHERE club_id=?").bind(cid).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((35.0,60));
        let units = ((demand as f64 * (quality as f64 / 100.0) * (40.0 / price.max(10.0))).round() as i64).clamp(1, 500);
        let revenue = units as f64 * price;
        let sale = sqlx::query("INSERT INTO merchandising_sales(club_id,week_date,units,unit_price,revenue) VALUES(?,?,?,?,?) ON CONFLICT(club_id,week_date) DO NOTHING")
            .bind(cid).bind(&today.0).bind(units).bind(price).bind(revenue).execute(pool).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE club_merchandising SET demand=?,total_units=total_units+?,total_revenue=total_revenue+?,updated_at=? WHERE club_id=?")
            .bind(demand).bind(if sale.rows_affected() > 0 { units } else { 0 }).bind(if sale.rows_affected() > 0 { revenue } else { 0.0 }).bind(&today.0).bind(cid).execute(pool).await.map_err(|e| e.to_string())?;
        if sale.rows_affected() > 0 || tv_income > 0.0 || sponsor_income > 0.0 {
            sqlx::query("UPDATE club_finances SET balance=balance-?+?+? WHERE club_id=?")
                .bind(if sale.rows_affected() > 0 { weekly_cost } else { 0.0 }).bind(if sale.rows_affected() > 0 { revenue } else { 0.0 }).bind(tv_income + sponsor_income).bind(cid).execute(pool).await.map_err(|e| e.to_string())?;
        }

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

pub async fn add_travel_cost(pool: &SqlitePool, match_id: i64, club_id: i64, origin_city_id: Option<i64>, destination_city_id: Option<i64>, distance_km: f64, cost: f64, date: &str) -> Result<(), String> {
    if !distance_km.is_finite() || !cost.is_finite() || distance_km < 0.0 || cost < 0.0 { return Err("Coste de viaje no válido".into()); }
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let inserted = sqlx::query("INSERT OR IGNORE INTO match_travel_costs(match_id,club_id,origin_city_id,destination_city_id,distance_km,cost,created_at) VALUES(?,?,?,?,?,?,?)")
        .bind(match_id).bind(club_id).bind(origin_city_id).bind(destination_city_id).bind(distance_km).bind(cost).bind(date).execute(&mut *tx).await.map_err(|e| e.to_string())?.rows_affected();
    if inserted > 0 {
        sqlx::query("INSERT INTO club_travel_finance(club_id,travel_spend,away_matches,updated_at) VALUES(?,?,1,?) ON CONFLICT(club_id) DO UPDATE SET travel_spend=travel_spend+excluded.travel_spend,away_matches=away_matches+1,updated_at=excluded.updated_at")
            .bind(club_id).bind(cost).bind(date).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE club_finances SET balance=balance-? WHERE club_id=?").bind(cost).bind(club_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db, world};

    #[tokio::test]
    async fn facility_upgrade_changes_level_and_balance_once() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        let (club_id,): (i64,) = sqlx::query_as("SELECT id FROM clubs ORDER BY id LIMIT 1").fetch_one(&pool).await.unwrap();
        let (before,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        upgrade_facility(&pool, club_id, "training").await.unwrap();
        let (level,): (i64,) = sqlx::query_as("SELECT training_level FROM club_facilities WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        let (after,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        assert_eq!(level, 2);
        assert!((before - after - 75000.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn invalid_facility_is_rejected_without_side_effects() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        let (club_id,): (i64,) = sqlx::query_as("SELECT id FROM clubs ORDER BY id LIMIT 1").fetch_one(&pool).await.unwrap();
        let (before,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        assert!(upgrade_facility(&pool, club_id, "invalid").await.is_err());
        let (after,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn commercial_facility_increases_merchandise_demand() {
        assert_eq!(merchandise_demand(500, 1), 85);
        assert_eq!(merchandise_demand(500, 3), 95);
        assert_eq!(merchandise_demand(900, 5), 100);
    }

    #[tokio::test]
    async fn merchandising_week_is_recorded_once() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        let (club_id,): (i64,) = sqlx::query_as("SELECT id FROM clubs ORDER BY id LIMIT 1").fetch_one(&pool).await.unwrap();
        process_weekly_finances(&pool).await.unwrap();
        let (first_revenue,): (f64,) = sqlx::query_as("SELECT total_revenue FROM club_merchandising WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        process_weekly_finances(&pool).await.unwrap();
        let (second_revenue,): (f64,) = sqlx::query_as("SELECT total_revenue FROM club_merchandising WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        let (sales,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM merchandising_sales WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        assert_eq!(sales, 1);
        assert_eq!(first_revenue, second_revenue);
    }

    #[tokio::test]
    async fn sponsorship_payment_is_recorded_once_per_week() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        let (club_id,): (i64,) = sqlx::query_as("SELECT id FROM clubs ORDER BY id LIMIT 1").fetch_one(&pool).await.unwrap();
        process_weekly_finances(&pool).await.unwrap();
        let (first_balance,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        let (first_payments,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sponsorship_payments WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        process_weekly_finances(&pool).await.unwrap();
        let (second_balance,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        let (second_payments,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sponsorship_payments WHERE club_id=?").bind(club_id).fetch_one(&pool).await.unwrap();
        assert_eq!(first_payments, 1);
        assert_eq!(second_payments, 1);
        assert!((first_balance - second_balance).abs() < 0.01, "reprocesar la misma semana no debe cambiar el balance");
    }

    #[tokio::test]
    async fn travel_cost_is_recorded_once_per_away_match() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        let (match_id, away_id): (i64, i64) = sqlx::query_as("SELECT id, away_club_id FROM matches LIMIT 1").fetch_one(&pool).await.unwrap();
        let (before,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(away_id).fetch_one(&pool).await.unwrap();
        add_travel_cost(&pool, match_id, away_id, Some(1), Some(2), 500.0, 875.0, "2026-07-10").await.unwrap();
        add_travel_cost(&pool, match_id, away_id, Some(1), Some(2), 500.0, 875.0, "2026-07-10").await.unwrap();
        let (entries,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM match_travel_costs WHERE match_id=? AND club_id=?").bind(match_id).bind(away_id).fetch_one(&pool).await.unwrap();
        let (after,): (f64,) = sqlx::query_as("SELECT balance FROM club_finances WHERE club_id=?").bind(away_id).fetch_one(&pool).await.unwrap();
        assert_eq!(entries, 1);
        assert!((before - after - 875.0).abs() < 0.01);
    }
}
