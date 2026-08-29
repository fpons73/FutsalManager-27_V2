use chrono::{Datelike, NaiveDate, Weekday};
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize)]
pub struct AdvanceResult {
    pub from_date: String,
    pub to_date: String,
    pub matches_played: i64,
    pub results: Vec<String>,
}

pub async fn advance_day(pool: &SqlitePool) -> Result<AdvanceResult, String> {
    let (cur_date,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1")
        .fetch_one(pool).await.map_err(|e| e.to_string())?;

    let date = NaiveDate::parse_from_str(&cur_date, "%Y-%m-%d").map_err(|e| e.to_string())?;

    let matches: Vec<(i64, i64, i64, i64)> =
        sqlx::query_as("SELECT id, home_club_id, away_club_id, competition_id FROM matches WHERE date=? AND status='scheduled'")
            .bind(&cur_date).fetch_all(pool).await.map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for (mid, hid, aid, comp_id) in matches.iter().copied() {
        let snap = crate::engine::simulate_clubs(pool, hid, aid).await?;
        let home_goals = snap.score[0] as i64;
        let away_goals = snap.score[1] as i64;

        let home_shots = snap.shots[0] as i64;
        let away_shots = snap.shots[1] as i64;
        let home_fouls = snap.fouls[0] as i64;
        let away_fouls = snap.fouls[1] as i64;

        sqlx::query("UPDATE matches SET status='finished', home_score=?, away_score=?, home_shots=?, away_shots=?, home_fouls=?, away_fouls=? WHERE id=?")
            .bind(home_goals).bind(away_goals).bind(home_shots).bind(away_shots).bind(home_fouls).bind(away_fouls).bind(mid)
            .execute(pool).await.map_err(|e| e.to_string())?;

        for ev in &snap.events {
            if ev.kind == "goal" || ev.kind == "double_penalty_goal" || ev.kind == "foul" || ev.kind == "double_penalty" {
                let club_for_event = if ev.team_id == 0 { hid } else { aid };
                sqlx::query("INSERT INTO match_events(match_id, minute, second, event_type, player_id, club_id, description, x, y) VALUES(?,?,?,?,?,?,?,?,?)")
                    .bind(mid).bind(ev.minute as i64).bind(ev.second as i64).bind(&ev.kind).bind(ev.player_id.map(|v| v as i64)).bind(club_for_event).bind(&ev.description).bind(ev.x as f64).bind(ev.y as f64)
                    .execute(pool).await.map_err(|e| e.to_string())?;
            }
        }

        let is_cup: (String,) = sqlx::query_as("SELECT competition_type FROM competitions WHERE id=?").bind(comp_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if is_cup.0 == "league" {
            update_standings(pool, comp_id, hid, aid, home_goals, away_goals).await?;
        } else {
            let winner = if home_goals > away_goals { hid } else if away_goals > home_goals { aid } else {
                // Desempate reglamentario: prórroga y, si persiste el empate, penaltis.
                let extra_home = rand::random::<bool>();
                let extra_away = rand::random::<bool>();
                if extra_home != extra_away { if extra_home { hid } else { aid } }
                else if rand::random::<bool>() { hid } else { aid }
            };
            if home_goals == away_goals {
                let extra_home = rand::random::<bool>();
                let extra_away = rand::random::<bool>();
                let (pen_home, pen_away) = if extra_home != extra_away { (extra_home as i64, extra_away as i64) } else { (rand::random::<u8>() as i64 % 5, rand::random::<u8>() as i64 % 5) };
                let resolved_winner = if pen_home > pen_away { hid } else if pen_away > pen_home { aid } else if rand::random::<bool>() { hid } else { aid };
                sqlx::query("UPDATE cup_ties SET winner_club_id=?, went_to_extra_time=1, went_to_penalties=?, penalty_home_score=?, penalty_away_score=? WHERE match_id=?")
                    .bind(resolved_winner).bind((pen_home == pen_away) as i64).bind(pen_home).bind(pen_away).bind(mid).execute(pool).await.map_err(|e| e.to_string())?;
            } else {
                sqlx::query("UPDATE cup_ties SET winner_club_id=? WHERE match_id=?").bind(winner).bind(mid).execute(pool).await.map_err(|e| e.to_string())?;
            }
            crate::competition::generate_calendars(pool).await?;
        }

        let home_name: (String,) = sqlx::query_as("SELECT short_name FROM clubs WHERE id=?").bind(hid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let away_name: (String,) = sqlx::query_as("SELECT short_name FROM clubs WHERE id=?").bind(aid).fetch_one(pool).await.map_err(|e| e.to_string())?;
        results.push(format!("{} {}-{} {}", home_name.0, home_goals, away_goals, away_name.0));
    }

    // Ticket income for home clubs
    for (_, hid, _, _) in matches.iter().copied() {
        let (cap,): (Option<i64>,) = sqlx::query_as("SELECT capacity FROM stadiums WHERE id=(SELECT stadium_id FROM clubs WHERE id=?)").bind(hid).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((Some(3000),));
        let cap = cap.unwrap_or(3000) as f64;
        let attendance = (cap * 0.65 + rand::random::<f64>() * cap * 0.25) as i64;
        let income = attendance as f64 * 12.0;
        let _ = crate::finance::add_ticket_income(pool, hid, income).await;
    }

    // Weekly processing on Mondays
    let next = date + chrono::Duration::days(1);
    let next_s = next.format("%Y-%m-%d").to_string();
    sqlx::query("UPDATE game_state SET game_date=? WHERE id=1").bind(&next_s).execute(pool).await.map_err(|e| e.to_string())?;

    if next.weekday() == Weekday::Mon {
        let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
        if let Some(uc) = user_club {
            let _ = crate::training::process_training_week(pool, uc).await;
        }
        let _ = crate::finance::process_weekly_finances(pool).await;
        // Recover player conditions slightly each week
        sqlx::query("UPDATE player_states SET condition_val = MIN(100, condition_val + 5) WHERE condition_val < 100").execute(pool).await.ok();
        sqlx::query("UPDATE player_states SET match_fitness = MIN(100, match_fitness + 3) WHERE match_fitness < 100").execute(pool).await.ok();
    }

    // El ojeo progresa al pasar los días: cada asignación descubre mejor sus jugadores.
    sqlx::query("UPDATE scouting_centers SET knowledge_level=MIN(100, knowledge_level+1) WHERE club_id=(SELECT user_club_id FROM game_state WHERE id=1)").execute(pool).await.ok();
    sqlx::query("UPDATE player_knowledge SET knowledge_percentage=MIN(100, knowledge_percentage + (SELECT knowledge_gain FROM scout_assignments sa WHERE sa.club_id=player_knowledge.club_id AND sa.is_active=1 AND (sa.nation_id=(SELECT nation_id FROM players WHERE id=player_knowledge.player_id) OR sa.target_club_id=(SELECT club_id FROM contracts WHERE player_id=player_knowledge.player_id AND is_active=1 LIMIT 1)) LIMIT 1) WHERE club_id=(SELECT user_club_id FROM game_state WHERE id=1)").execute(pool).await.ok();

    // Desarrollo diario de la cantera.
    if let Ok((Some(uc),)) = sqlx::query_as::<_, (Option<i64>,)>("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(pool).await {
        let _ = crate::youth::develop(pool, uc).await;
    }

    // Random incoming offers
    {
        let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
        if let Some(uc) = user_club {
            let _ = crate::transfer::generate_incoming_offers(pool, uc).await;
        }
    }

    // Devolver automáticamente los jugadores cedidos cuando vence su préstamo.
    let expired_loans: Vec<(i64, i64, i64)> = sqlx::query_as("SELECT loan.id, loan.player_id, loan.loan_parent_id FROM contracts loan WHERE loan.loan_parent_id IS NOT NULL AND loan.is_active=1 AND loan.loan_until <= ?").bind(&next_s).fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (loan_id, player_id, parent_id) in expired_loans {
        sqlx::query("UPDATE contracts SET is_active=0 WHERE id=?").bind(loan_id).execute(pool).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE contracts SET is_active=1 WHERE id=? AND player_id=?").bind(parent_id).bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    }

    // Auto-recover injuries past return date
    sqlx::query("UPDATE injuries SET is_active=0 WHERE is_active=1 AND expected_return_date <= ?").bind(&next_s).execute(pool).await.ok();

    recompute_positions(pool).await?;

    Ok(AdvanceResult {
        from_date: cur_date,
        to_date: next_s,
        matches_played: results.len() as i64,
        results,
    })
}

async fn update_standings(pool: &SqlitePool, comp_id: i64, hid: i64, aid: i64, hg: i64, ag: i64) -> Result<(), String> {
    let (hw, hl, hd) = if hg > ag { (1, 0, 0) } else if hg < ag { (0, 1, 0) } else { (0, 0, 1) };
    let (aw, al, ad) = if ag > hg { (1, 0, 0) } else if ag < hg { (0, 1, 0) } else { (0, 0, 1) };
    let hpts = hw * 3 + hd;
    let apts = aw * 3 + ad;

    sqlx::query(
        "UPDATE league_standings SET played=played+1, won=won+?, drawn=drawn+?, lost=lost+?, goals_for=goals_for+?, goals_against=goals_against+?, goal_difference=goals_for-goals_against, points=points+? WHERE competition_id=? AND club_id=?"
    )
    .bind(hw).bind(hd).bind(hl).bind(hg).bind(ag).bind(hpts).bind(comp_id).bind(hid)
    .execute(pool).await.map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE league_standings SET played=played+1, won=won+?, drawn=drawn+?, lost=lost+?, goals_for=goals_for+?, goals_against=goals_against+?, goal_difference=goals_for-goals_against, points=points+? WHERE competition_id=? AND club_id=?"
    )
    .bind(aw).bind(ad).bind(al).bind(ag).bind(hg).bind(apts).bind(comp_id).bind(aid)
    .execute(pool).await.map_err(|e| e.to_string())?;

    Ok(())
}

async fn recompute_positions(pool: &SqlitePool) -> Result<(), String> {
    let comps: Vec<(i64,)> = sqlx::query_as("SELECT id FROM competitions").fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (cid,) in comps {
        let rows: Vec<(i64, i64, i64, i64)> = sqlx::query_as(
            "SELECT club_id, points, goal_difference, goals_for FROM league_standings WHERE competition_id=? ORDER BY points DESC, goal_difference DESC, goals_for DESC, club_id ASC"
        ).bind(cid).fetch_all(pool).await.map_err(|e| e.to_string())?;
        for (pos, (club_id, _, _, _)) in rows.iter().enumerate() {
            sqlx::query("UPDATE league_standings SET position=? WHERE competition_id=? AND club_id=?")
                .bind((pos + 1) as i64).bind(cid).bind(club_id)
                .execute(pool).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub async fn advance_days(pool: &SqlitePool, n: i64) -> Result<Vec<AdvanceResult>, String> {
    let mut out = Vec::new();
    for _ in 0..n {
        out.push(advance_day(pool).await?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::world;

    #[tokio::test]
    async fn advance_simulates_and_advances() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();

        let (d0,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&pool).await.unwrap();
        assert_eq!(d0, "2026-07-10");

        let r1 = advance_day(&pool).await.unwrap();
        assert_eq!(r1.from_date, "2026-07-10");
        assert_eq!(r1.to_date, "2026-07-11");
        assert_eq!(r1.matches_played, 0);

        let mut played = 0;
        for _ in 0..50 {
            let r = advance_day(&pool).await.unwrap();
            played += r.matches_played;
            if played > 0 { break; }
        }
        assert!(played > 0, "debe haber jornadas dentro de 50 días desde julio");

        let (finished,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE status='finished'").fetch_one(&pool).await.unwrap();
        assert!(finished > 0);
        let (with_goals,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM match_events WHERE event_type='goal' OR event_type='double_penalty_goal'").fetch_one(&pool).await.unwrap();
        assert!(with_goals > 0);

        let (top_pos,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_standings WHERE position>0").fetch_one(&pool).await.unwrap();
        assert!(top_pos >= 46, "al menos 46 filas en clasificación, got {}", top_pos);
    }

    #[tokio::test]
    async fn full_season_simulation_finishes_662() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        let mut total = 0;
        for _ in 0..600 {
            let r = advance_day(&pool).await.unwrap();
            total += r.matches_played;
            let (pending,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE status='scheduled'").fetch_one(&pool).await.unwrap();
            if pending == 0 { break; }
        }
        assert!(total >= 662, "al menos 662 partidos, got {}", total);
        let (pending,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE status='scheduled'").fetch_one(&pool).await.unwrap();
        assert_eq!(pending, 0);
        let (played,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_standings WHERE played>0").fetch_one(&pool).await.unwrap();
        assert!(played >= 46, "al menos 46 clubes con partidos, got {}", played);
    }
}
