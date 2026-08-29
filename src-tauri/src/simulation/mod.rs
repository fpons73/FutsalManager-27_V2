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
    let _ = crate::commands::national_cmd::generate_international_windows(pool).await;
    let _ = crate::commands::national_cmd::ensure_national_tournament_entries(pool).await;
    let _ = crate::commands::national_cmd::generate_national_tournament_matches(pool).await;
    let _ = crate::commands::national_cmd::progress_national_tournaments(pool).await;
    let _ = crate::commands::national_cmd::resolve_national_knockouts(pool).await;
    let (cur_date,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1")
        .fetch_one(pool).await.map_err(|e| e.to_string())?;

    let date = NaiveDate::parse_from_str(&cur_date, "%Y-%m-%d").map_err(|e| e.to_string())?;

    let matches: Vec<(i64, i64, i64, i64)> =
        sqlx::query_as("SELECT id, home_club_id, away_club_id, competition_id FROM matches WHERE date=? AND status='scheduled'")
            .bind(&cur_date).fetch_all(pool).await.map_err(|e| e.to_string())?;

    let international_matches: Vec<(i64,i64,i64,i64)> = sqlx::query_as("SELECT id,home_nation_id,away_nation_id,COALESCE(competition_id,0) FROM international_matches WHERE date=? AND status='scheduled'").bind(&cur_date).fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (id, home, away, _competition_id) in international_matches {
        let home_level: (i64,) = sqlx::query_as("SELECT futsal_level FROM nations WHERE id=?").bind(home).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let away_level: (i64,) = sqlx::query_as("SELECT futsal_level FROM nations WHERE id=?").bind(away).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let home_goals = ((home_level.0 as f64 / 35.0) + rand::random::<f64>() * 3.0).round() as i64;
        let away_goals = ((away_level.0 as f64 / 35.0) + rand::random::<f64>() * 3.0).round() as i64;
        sqlx::query("UPDATE international_matches SET status='finished',home_score=?,away_score=? WHERE id=?").bind(home_goals).bind(away_goals).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE national_tournament_entries SET played=played+1,won=won+?,drawn=drawn+?,lost=lost+?,goals_for=goals_for+?,goals_against=goals_against+?,points=points+? WHERE competition_id=(SELECT competition_id FROM international_matches WHERE id=?) AND season=(SELECT season FROM international_matches WHERE id=?) AND nation_id=?")
            .bind((home_goals>away_goals) as i64).bind((home_goals==away_goals) as i64).bind((home_goals<away_goals) as i64).bind(home_goals).bind(away_goals).bind(if home_goals>away_goals{3}else if home_goals==away_goals{1}else{0}).bind(id).bind(id).bind(home).execute(pool).await.ok();
        sqlx::query("UPDATE national_tournament_entries SET played=played+1,won=won+?,drawn=drawn+?,lost=lost+?,goals_for=goals_for+?,goals_against=goals_against+?,points=points+? WHERE competition_id=(SELECT competition_id FROM international_matches WHERE id=?) AND season=(SELECT season FROM international_matches WHERE id=?) AND nation_id=?")
            .bind((away_goals>home_goals) as i64).bind((home_goals==away_goals) as i64).bind((away_goals<home_goals) as i64).bind(away_goals).bind(home_goals).bind(if away_goals>home_goals{3}else if home_goals==away_goals{1}else{0}).bind(id).bind(id).bind(away).execute(pool).await.ok();
    }

    let mut results = Vec::new();
    for (mid, hid, aid, comp_id) in matches.iter().copied() {
        let snap = crate::engine::simulate_clubs(pool, hid, aid).await?;
        let home_goals = snap.score[0] as i64;
        let away_goals = snap.score[1] as i64;

        let home_shots = snap.shots[0] as i64;
        let away_shots = snap.shots[1] as i64;
        let home_fouls = snap.fouls[0] as i64;
        let away_fouls = snap.fouls[1] as i64;

        sqlx::query("UPDATE matches SET status='finished', home_score=?, away_score=?, home_shots=?, away_shots=?, home_fouls=?, away_fouls=?, home_possession=?, away_possession=? WHERE id=?")
            .bind(home_goals).bind(away_goals).bind(home_shots).bind(away_shots).bind(home_fouls).bind(away_fouls).bind(snap.possession[0] as i64).bind(snap.possession[1] as i64).bind(mid)
            .execute(pool).await.map_err(|e| e.to_string())?;

        persist_player_statistics(pool, mid, comp_id, &snap, hid, aid).await?;

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
        }
        let group_ids: Vec<(i64,)> = sqlx::query_as("SELECT gm.group_id FROM group_members gm JOIN competition_groups cg ON cg.id=gm.group_id WHERE cg.competition_id=? AND cg.season=(SELECT season FROM game_state WHERE id=1) AND gm.club_id IN (?,?) GROUP BY gm.group_id HAVING COUNT(DISTINCT gm.club_id)=2").bind(comp_id).bind(hid).bind(aid).fetch_all(pool).await.unwrap_or_default();
        for (group_id,) in group_ids {
            update_group_members(pool, group_id, hid, aid, home_goals, away_goals).await?;
        }
        if is_cup.0 != "league" {
            let tie: Option<(i64, i64, i64, i64, i64, i64)> = sqlx::query_as("SELECT id, leg, home_club_id, away_club_id, COALESCE(aggregate_home_score,0), COALESCE(aggregate_away_score,0) FROM cup_ties WHERE match_id=?").bind(mid).fetch_optional(pool).await.map_err(|e| e.to_string())?;
            let is_two_leg = if let Some((_, _, _, _, _, _)) = tie { let (v,): (i64,) = sqlx::query_as("SELECT knockout_two_legs FROM competitions WHERE id=?").bind(comp_id).fetch_one(pool).await.unwrap_or((0,)); v != 0 } else { false };
            if is_two_leg {
                let (tie_id, leg, first_home, first_away, prior_home, prior_away) = tie.unwrap();
                let (agg_home, agg_away) = if leg == 2 { (prior_home + away_goals, prior_away + home_goals) } else { (home_goals, away_goals) };
                sqlx::query("UPDATE cup_ties SET aggregate_home_score=?, aggregate_away_score=? WHERE id=?").bind(agg_home).bind(agg_away).bind(tie_id).execute(pool).await.map_err(|e| e.to_string())?;
                if leg == 2 {
                    let winner = if agg_home > agg_away { first_home } else if agg_away > agg_home { first_away } else if rand::random::<bool>() { first_home } else { first_away };
                    sqlx::query("UPDATE cup_ties SET winner_club_id=? WHERE id=?").bind(winner).bind(tie_id).execute(pool).await.map_err(|e| e.to_string())?;
                    sqlx::query("UPDATE cup_ties SET winner_club_id=? WHERE competition_id=? AND season=(SELECT season FROM matches WHERE id=?) AND round=(SELECT round FROM matches WHERE id=?) AND leg=1 AND ((home_club_id=? AND away_club_id=?) OR (home_club_id=? AND away_club_id=?))").bind(winner).bind(comp_id).bind(mid).bind(mid).bind(first_home).bind(first_away).bind(first_away).bind(first_home).execute(pool).await.map_err(|e| e.to_string())?;
                }
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
        }

        for (club_id, won) in [(hid, home_goals > away_goals), (aid, away_goals > home_goals)] {
            let delta = if won { 1 } else if home_goals == away_goals { 0 } else { -1 };
            sqlx::query("UPDATE player_states SET morale=MIN(100,MAX(0,morale+?)), happiness=MIN(100,MAX(0,happiness+?)) WHERE player_id IN (SELECT player_id FROM contracts WHERE club_id=? AND is_active=1)").bind(delta).bind(delta).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
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
        let _ = crate::commands::board_cmd::evaluate_board(pool).await;
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
    let _ = crate::competition::progress_group_competitions(pool).await?;

    Ok(AdvanceResult {
        from_date: cur_date,
        to_date: next_s,
        matches_played: results.len() as i64,
        results,
    })
}

fn snap_stats(snap: &crate::engine::MatchSnapshot) -> Vec<(u32,u32,bool,u32,u32,u32,u32,u32,u32,u32,u32,f64)> {
    let mut map = std::collections::HashMap::new();
    for p in &snap.players { map.insert(p.id, (p.id, p.team_id, p.on_pitch, if p.on_pitch { (snap.time_seconds/60).min(40) } else { 0 }, 0, 0, 0, 0, 0, 0, 0, 6.0)); }
    for e in &snap.events { if let Some(pid)=e.player_id { if let Some(s)=map.get_mut(&pid) { match e.kind.as_str() { "goal"|"double_penalty_goal"=>{s.4+=1;if let Some(a)=e.assist_player_id{if let Some(asst)=map.get_mut(&a){asst.10+=1;}}}, "shot_off"=>s.5+=1, "save"=>{s.5+=1;s.6+=1}, "foul"=>s.7+=1, "yellow_card"=>s.8+=1, "red_card"=>s.9+=1, _=>{} } } } }
    map.into_values().collect()
}

async fn player_id_is_goalkeeper(pool: &SqlitePool, player_id: i64) -> Result<bool, String> {
    let row: (i64,) = sqlx::query_as("SELECT por_natural FROM player_positions WHERE player_id=?").bind(player_id).fetch_optional(pool).await.map_err(|e| e.to_string())?.unwrap_or((0,));
    Ok(row.0 >= 18)
}

async fn persist_player_statistics(pool: &SqlitePool, match_id: i64, competition_id: i64, snap: &crate::engine::MatchSnapshot, home_id: i64, away_id: i64) -> Result<(), String> {
    let season: (String,) = sqlx::query_as("SELECT season FROM matches WHERE id=?").bind(match_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    for (player_id, team_id, started, minutes, goals, shots, shots_on, fouls, yellow, red, assists, rating) in snap_stats(snap) {
        let club_id = if team_id == 0 { home_id } else { away_id };
        sqlx::query("INSERT INTO match_player_stats(match_id,player_id,club_id,started,minutes_played,goals,assists,shots,shots_on_target,fouls_committed,yellow_cards,red_cards,rating) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?) ON CONFLICT(match_id,player_id) DO UPDATE SET minutes_played=excluded.minutes_played,goals=excluded.goals,assists=excluded.assists,shots=excluded.shots,shots_on_target=excluded.shots_on_target,fouls_committed=excluded.fouls_committed,rating=excluded.rating")
            .bind(match_id).bind(player_id as i64).bind(club_id).bind(started as i64).bind(minutes as i64).bind(goals as i64).bind(assists as i64).bind(shots as i64).bind(shots_on as i64).bind(fouls as i64).bind(yellow as i64).bind(red as i64).bind(rating).execute(pool).await.map_err(|e| e.to_string())?;
        let clean_sheets = if player_id_is_goalkeeper(pool, player_id as i64).await? && ((team_id == 0 && snap.score[1] == 0) || (team_id == 1 && snap.score[0] == 0)) { 1 } else { 0 };
        let saves = snap.events.iter().filter(|e| e.player_id == Some(player_id) && e.kind == "save").count() as i64;
        sqlx::query("INSERT INTO player_season_stats(season,competition_id,player_id,club_id,appearances,starts,minutes_played,goals,assists,shots,shots_on_target,fouls_committed,yellow_cards,red_cards,rating_total,clean_sheets,saves) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?) ON CONFLICT(season,competition_id,player_id) DO UPDATE SET appearances=appearances+excluded.appearances,starts=starts+excluded.starts,minutes_played=minutes_played+excluded.minutes_played,goals=goals+excluded.goals,assists=assists+excluded.assists,shots=shots+excluded.shots,shots_on_target=shots_on_target+excluded.shots_on_target,fouls_committed=fouls_committed+excluded.fouls_committed,yellow_cards=yellow_cards+excluded.yellow_cards,red_cards=red_cards+excluded.red_cards,rating_total=rating_total+excluded.rating_total,clean_sheets=clean_sheets+excluded.clean_sheets,saves=saves+excluded.saves")
            .bind(&season.0).bind(competition_id).bind(player_id as i64).bind(club_id).bind((minutes > 0) as i64).bind(started as i64).bind(minutes as i64).bind(goals as i64).bind(assists as i64).bind(shots as i64).bind(shots_on as i64).bind(fouls as i64).bind(yellow as i64).bind(red as i64).bind(rating).bind(clean_sheets).bind(saves).execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

async fn update_group_members(pool: &SqlitePool, group_id: i64, hid: i64, aid: i64, hg: i64, ag: i64) -> Result<(), String> {
    for (club_id, gf, ga, won, drawn, lost, points) in [(hid, hg, ag, (hg > ag) as i64, (hg == ag) as i64, (hg < ag) as i64, if hg > ag {3} else if hg == ag {1} else {0}), (aid, ag, hg, (ag > hg) as i64, (hg == ag) as i64, (ag < hg) as i64, if ag > hg {3} else if hg == ag {1} else {0})] {
        sqlx::query("UPDATE group_members SET played=played+1, won=won+?, drawn=drawn+?, lost=lost+?, goals_for=goals_for+?, goals_against=goals_against+?, points=points+? WHERE group_id=? AND club_id=?").bind(won).bind(drawn).bind(lost).bind(gf).bind(ga).bind(points).bind(group_id).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
    }
    sqlx::query("UPDATE group_members SET position=(SELECT COUNT(*)+1 FROM group_members other WHERE other.group_id=group_members.group_id AND (other.points>group_members.points OR (other.points=group_members.points AND (other.goals_for-other.goals_against)>(group_members.goals_for-group_members.goals_against)))) WHERE group_id=?").bind(group_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
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
    let comps: Vec<(i64, String)> = sqlx::query_as("SELECT id, tiebreak_rule FROM competitions").fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (cid, rule) in comps {
        let order = crate::competition::rules::order_clause(&rule);
        let rows: Vec<(i64, i64, i64, i64)> = sqlx::query_as(&format!("SELECT club_id, points, goal_difference, goals_for FROM league_standings WHERE competition_id={} ORDER BY {}", cid, order)).fetch_all(pool).await.map_err(|e| e.to_string())?;
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
