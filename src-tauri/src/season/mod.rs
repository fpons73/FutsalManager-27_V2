use chrono::{Datelike, NaiveDate};
use sqlx::SqlitePool;

pub async fn is_season_finished(pool: &SqlitePool) -> Result<bool, String> {
    let (pending,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE status='scheduled'").fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(pending == 0)
}

async fn generate_playoffs(pool: &SqlitePool, season: &str) -> Result<i64, String> {
    let comps: Vec<(i64, i64, i64)> = sqlx::query_as("SELECT id, playoff_places, total_teams FROM competitions WHERE competition_type='league' AND playoff_places > 0")
        .fetch_all(pool).await.map_err(|e| e.to_string())?;
    let mut created = 0;
    for (comp_id, places, _) in comps {
        let existing: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cup_ties WHERE competition_id=? AND season=? AND round=900").bind(comp_id).bind(season).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if existing.0 > 0 { continue; }
        let teams: Vec<(i64,)> = sqlx::query_as("SELECT club_id FROM league_standings WHERE competition_id=? AND season=? ORDER BY position LIMIT ?").bind(comp_id).bind(season).bind((places * 2).max(2)).fetch_all(pool).await.map_err(|e| e.to_string())?;
        for pair in teams.chunks(2) {
            if pair.len() < 2 { continue; }
            let (mid,): (i64,) = sqlx::query_as("INSERT INTO matches(competition_id,season,round,date,home_club_id,away_club_id,stadium_id,status) VALUES(?,?,?,?,?,?,(SELECT stadium_id FROM clubs WHERE id=?),'scheduled') RETURNING id")
                .bind(comp_id).bind(season).bind(900_i64).bind(format!("{}-06-01", season.split('/').next().unwrap_or("2027"))).bind(pair[0].0).bind(pair[1].0).bind(pair[0].0).fetch_one(pool).await.map_err(|e| e.to_string())?;
            sqlx::query("INSERT INTO cup_ties(competition_id,season,round,leg,home_club_id,away_club_id,match_id) VALUES(?,?,900,1,?,?,?)").bind(comp_id).bind(season).bind(pair[0].0).bind(pair[1].0).bind(mid).execute(pool).await.map_err(|e| e.to_string())?;
            created += 1;
        }
    }
    Ok(created)
}

async fn resolve_playoff_promotions(pool: &SqlitePool, season: &str) -> Result<i64, String> {
    let comps: Vec<(i64, i64)> = sqlx::query_as("SELECT id, playoff_places FROM competitions WHERE competition_type='league' AND playoff_places > 0").fetch_all(pool).await.map_err(|e| e.to_string())?;
    let mut promoted = 0;
    for (comp_id, places) in comps {
        let winners: Vec<(i64,)> = sqlx::query_as("SELECT winner_club_id FROM cup_ties WHERE competition_id=? AND season=? AND round=900 AND winner_club_id IS NOT NULL ORDER BY id LIMIT ?").bind(comp_id).bind(season).bind(places).fetch_all(pool).await.map_err(|e| e.to_string())?;
        for (club_id,) in winners {
            let target: Option<(i64,)> = sqlx::query_as("SELECT id FROM competitions c WHERE c.nation_id=(SELECT nation_id FROM competitions WHERE id=?) AND c.tier=(SELECT tier-1 FROM competitions WHERE id=?)").bind(comp_id).bind(comp_id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
            if let Some((target_id,)) = target {
                let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM season_movements WHERE season=? AND from_competition_id=? AND to_competition_id=? AND club_id=? AND movement_type='promotion'").bind(season).bind(comp_id).bind(target_id).bind(club_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
                if exists.0 == 0 { sqlx::query("INSERT INTO season_movements(season,from_competition_id,to_competition_id,club_id,movement_type,source_position,created_at) VALUES(?,?,?,?,?,?,?)").bind(season).bind(comp_id).bind(target_id).bind(club_id).bind("promotion").bind(0_i64).bind(season).execute(pool).await.map_err(|e| e.to_string())?; promoted += 1; }
            }
        }
    }
    Ok(promoted)
}

async fn apply_promotion_relegation(pool: &SqlitePool, season: &str) -> Result<i64, String> {
    // Recalcula las inscripciones de la temporada siguiente: cada división
    // conserva sus equipos salvo que una pareja intercambie plazas.
    let pairs: Vec<(i64, i64)> = sqlx::query_as("SELECT upper.id, lower.id FROM competitions upper JOIN competitions lower ON upper.nation_id=lower.nation_id AND upper.tier IS NOT NULL AND lower.tier IS NOT NULL AND lower.tier=upper.tier+1")
        .fetch_all(pool).await.map_err(|e| e.to_string())?;
    let mut moved = 0_i64;
    for (upper_id, lower_id) in pairs {
        let (up_places, down_places): (i64, i64) = sqlx::query_as("SELECT promotion_places, relegation_places FROM competitions WHERE id=?")
            .bind(lower_id).fetch_one(pool).await.unwrap_or((2, 2));
        let (upper_down,): (i64,) = sqlx::query_as("SELECT relegation_places FROM competitions WHERE id=?").bind(upper_id).fetch_one(pool).await.unwrap_or((down_places,));
        let down = upper_down.max(0);
        let up = up_places.max(0);
        let relegated: Vec<(i64, i64)> = sqlx::query_as("SELECT club_id, position FROM league_standings WHERE competition_id=? AND season=? ORDER BY position DESC LIMIT ?")
            .bind(upper_id).bind(season).bind(down).fetch_all(pool).await.map_err(|e| e.to_string())?;
        let playoff_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cup_ties WHERE competition_id=? AND season=? AND round=900").bind(lower_id).bind(season).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let playoff_pending: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cup_ties WHERE competition_id=? AND season=? AND round=900 AND winner_club_id IS NULL").bind(lower_id).bind(season).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let promoted: Vec<(i64, i64)> = if playoff_pending.0 > 0 {
            Vec::new()
        } else if playoff_count.0 > 0 {
            sqlx::query_as("SELECT winner_club_id, 0 FROM cup_ties WHERE competition_id=? AND season=? AND round=900 AND winner_club_id IS NOT NULL ORDER BY id LIMIT ?")
                .bind(lower_id).bind(season).bind(up).fetch_all(pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query_as("SELECT club_id, position FROM league_standings WHERE competition_id=? AND season=? ORDER BY position ASC LIMIT ?")
                .bind(lower_id).bind(season).bind(up).fetch_all(pool).await.map_err(|e| e.to_string())?
        };
        for (club_id, position) in relegated {
            sqlx::query("INSERT OR IGNORE INTO season_movements(season,from_competition_id,to_competition_id,club_id,movement_type,source_position,created_at) VALUES(?,?,?,?,?,?,?)")
                .bind(season).bind(upper_id).bind(lower_id).bind(club_id).bind("relegation").bind(position).bind(season).execute(pool).await.map_err(|e| e.to_string())?;
            moved += 1;
        }
        for (club_id, position) in promoted {
            sqlx::query("INSERT OR IGNORE INTO season_movements(season,from_competition_id,to_competition_id,club_id,movement_type,source_position,created_at) VALUES(?,?,?,?,?,?,?)")
                .bind(season).bind(lower_id).bind(upper_id).bind(club_id).bind("promotion").bind(position).bind(season).execute(pool).await.map_err(|e| e.to_string())?;
            moved += 1;
        }
    }
    Ok(moved)
}

async fn record_honours(pool: &SqlitePool, season: &str) -> Result<i64, String> {
    let comps: Vec<(i64, String, String)> = sqlx::query_as("SELECT id, name, competition_type FROM competitions WHERE competition_type IN ('league','cup')").fetch_all(pool).await.map_err(|e| e.to_string())?;
    let mut count = 0;
    for (cid, name, kind) in comps {
        let winner: Option<(i64,)> = if kind == "league" {
            sqlx::query_as("SELECT club_id FROM league_standings WHERE competition_id=? AND season=? ORDER BY position LIMIT 1").bind(cid).bind(season).fetch_optional(pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query_as("SELECT winner_club_id FROM cup_ties WHERE competition_id=? AND season=? AND winner_club_id IS NOT NULL ORDER BY round DESC, id DESC LIMIT 1").bind(cid).bind(season).fetch_optional(pool).await.map_err(|e| e.to_string())?
        };
        if let Some((club_id,)) = winner {
            let honour = if name.to_lowercase().contains("supercopa") { "supercup" } else if kind == "league" { "league" } else { "cup" };
            sqlx::query("INSERT OR IGNORE INTO competition_honours(competition_id,season,club_id,honour_type,created_at) VALUES(?,?,?,?,?)").bind(cid).bind(season).bind(club_id).bind(honour).bind(season).execute(pool).await.map_err(|e| e.to_string())?;
            count += 1;
        }
    }
    Ok(count)
}

pub async fn rollover_season(pool: &SqlitePool) -> Result<String, String> {
    let (season,): (String,) = sqlx::query_as("SELECT season FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let parts: Vec<&str> = season.split('/').collect();
    let next_season = if parts.len()==2 {
        let a: i32 = parts[0].parse().unwrap_or(2026);
        let b: i32 = parts[1].parse().unwrap_or(2027);
        format!("{}/{}", a+1, b+1)
    } else { "2027/2028".into() };

    let honours = record_honours(pool, &season).await?;

    // Prize money for top 3
    let comps: Vec<(i64,)> = sqlx::query_as("SELECT id FROM competitions").fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (cid,) in &comps {
        let top: Vec<(i64, i64)> = sqlx::query_as("SELECT club_id, position FROM league_standings WHERE competition_id=? AND season=? ORDER BY position LIMIT 3").bind(cid).bind(&season).fetch_all(pool).await.map_err(|e| e.to_string())?;
        for (club_id, pos) in top {
            let prize = match pos { 1 => 150000.0, 2 => 80000.0, 3 => 40000.0, _=>0.0 };
            sqlx::query("UPDATE club_finances SET balance=balance+?, prize_money=prize_money+?, transfer_budget=transfer_budget+? WHERE club_id=?").bind(prize).bind(prize).bind(prize*0.5).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
        }
    }

    // Age handling & retirements
    let (today_s,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    let today = NaiveDate::parse_from_str(&today_s, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let new_date = NaiveDate::from_ymd_opt(today.year()+1, 7, 10).unwrap_or(today);

    let players = sqlx::query_as::<_, (i64, String, i64)>("SELECT p.id, p.date_of_birth, ps.current_ability FROM players p JOIN player_states ps ON ps.player_id=p.id WHERE p.is_retired=0").fetch_all(pool).await.map_err(|e| e.to_string())?;
    let mut retired = 0;
    for (pid, dob, ca) in players {
        let dob_d = NaiveDate::parse_from_str(&dob, "%Y-%m-%d").unwrap_or(today);
        let age = ((today - dob_d).num_days()/365) as i64;
        let retire = age >= 36 || (age >= 33 && ca < 70 && rand::random::<f64>() < 0.4) || (age >= 34 && rand::random::<f64>() < 0.25);
        if retire {
            sqlx::query("UPDATE players SET is_retired=1 WHERE id=?").bind(pid).execute(pool).await.map_err(|e| e.to_string())?;
            sqlx::query("UPDATE contracts SET is_active=0 WHERE player_id=? AND is_active=1").bind(pid).execute(pool).await.map_err(|e| e.to_string())?;
            // free youth regeneration
            let club: Option<(i64,)> = sqlx::query_as("SELECT club_id FROM contracts WHERE player_id=? ORDER BY id DESC LIMIT 1").bind(pid).fetch_optional(pool).await.map_err(|e| e.to_string())?;
            if let Some((cid,)) = club {
                let new_age = 17;
                let new_dob = new_date - chrono::Duration::days(new_age*365 + (rand::random::<u64>()%365) as i64);
                let first = ["Leo","Dani","Hugo","Iker","Marc","Bruno","Tiago","Alex","Joao","Rafa"][rand::random::<usize>()%10];
                let last = ["Silva","Santos","Garcia","Lopez","Fernandez","Costa","Pereira","Rodrigues","Martins","Gomez"][rand::random::<usize>()%10];
                let (nid,): (i64,) = sqlx::query_as("SELECT nation_id FROM clubs WHERE id=?").bind(cid).fetch_one(pool).await.map_err(|e| e.to_string())?;
                let ca_new = 45 + (rand::random::<i64>()%25);
                let pa_new = ca_new + 20 + (rand::random::<i64>()%40);
                let (new_pid,): (i64,) = sqlx::query_as("INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,height_cm,weight_kg) VALUES(?,?,?,?,?,?,?) RETURNING id")
                    .bind(first).bind(last).bind(format!("{} {}", first, last)).bind(new_dob.format("%Y-%m-%d").to_string()).bind(nid).bind(170+ (rand::random::<i64>()%15)).bind(70+ (rand::random::<i64>()%15))
                    .fetch_one(pool).await.map_err(|e| e.to_string())?;
                sqlx::query("INSERT INTO player_positions(player_id, por_natural, cie_natural, ala_natural, piv_natural, uni_natural) VALUES(?,8,8,8,8,12)").bind(new_pid).execute(pool).await.map_err(|e| e.to_string())?;
                sqlx::query("INSERT INTO player_states(player_id, current_ability, potential_ability) VALUES(?,?,?)").bind(new_pid).bind(ca_new).bind(pa_new.min(200)).execute(pool).await.map_err(|e| e.to_string())?;
                sqlx::query("INSERT INTO player_attributes(player_id) VALUES(?)").bind(new_pid).execute(pool).await.map_err(|e| e.to_string())?;
                sqlx::query("INSERT INTO contracts(player_id, club_id, wage_weekly, start_date, end_date, is_active) VALUES(?,?,?,?,?,1)")
                    .bind(new_pid).bind(cid).bind(300.0).bind(new_date.format("%Y-%m-%d").to_string()).bind((new_date+chrono::Duration::days(365*2)).format("%Y-%m-%d").to_string())
                    .execute(pool).await.map_err(|e| e.to_string())?;
            }
            retired += 1;
        }
    }

    // Expire old contracts without renewal (free agents remain without club - simplified)
    sqlx::query("UPDATE contracts SET is_active=0 WHERE end_date < ? AND is_active=1").bind(new_date.format("%Y-%m-%d").to_string()).execute(pool).await.map_err(|e| e.to_string())?;

    let playoff_matches = generate_playoffs(pool, &season).await?;
    let playoff_promotions = resolve_playoff_promotions(pool, &season).await?;
    let movements = apply_promotion_relegation(pool, &season).await?;

    // Mantener la inscripción de cada liga para la siguiente temporada y aplicar
    // los intercambios registrados: los descendidos ocupan las plazas de los
    // ascendidos y viceversa. Las copas se regeneran después con estas plantillas.
    let comps2: Vec<(i64,)> = sqlx::query_as("SELECT id FROM competitions").fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (cid,) in comps2 {
        let mut clubs: Vec<i64> = sqlx::query_as::<_, (i64,)>("SELECT club_id FROM league_standings WHERE competition_id=? AND season=? ORDER BY position").bind(cid).bind(&season).fetch_all(pool).await.map_err(|e| e.to_string())?.into_iter().map(|(id,)| id).collect();
        let incoming: Vec<i64> = sqlx::query_as::<_, (i64,)>("SELECT club_id FROM season_movements WHERE season=? AND to_competition_id=? ORDER BY movement_type, source_position").bind(&season).bind(cid).fetch_all(pool).await.map_err(|e| e.to_string())?.into_iter().map(|(id,)| id).collect();
        let team_limit: usize = sqlx::query_as::<_, (i64,)>("SELECT COALESCE(total_teams, 0) FROM competitions WHERE id=?").bind(cid).fetch_one(pool).await.map_err(|e| e.to_string())?.0.max(0) as usize;
        let outgoing: std::collections::HashSet<i64> = sqlx::query_as::<_, (i64,)>("SELECT club_id FROM season_movements WHERE season=? AND from_competition_id=?").bind(&season).bind(cid).fetch_all(pool).await.map_err(|e| e.to_string())?.into_iter().map(|(id,)| id).collect();
        clubs.retain(|id| !outgoing.contains(id));
        for id in incoming { if !clubs.contains(&id) { clubs.push(id); } }
        sqlx::query("UPDATE competitions SET season=? WHERE id=?").bind(&next_season).bind(cid).execute(pool).await.map_err(|e| e.to_string())?;
        for club_id in clubs.into_iter().take(team_limit) {
            sqlx::query("INSERT INTO league_standings(competition_id, season, club_id, position, played, won, drawn, lost, goals_for, goals_against, goal_difference, points) VALUES(?,?,?,?,0,0,0,0,0,0,0,0)").bind(cid).bind(&next_season).bind(club_id).bind(0).execute(pool).await.map_err(|e| e.to_string())?;
        }
    }

    let _ = crate::competition::progress_group_competitions(pool).await?;
    crate::competition::generate_calendars(pool).await?;

    sqlx::query("UPDATE game_state SET season=?, game_date=? WHERE id=1").bind(&next_season).bind(new_date.format("%Y-%m-%d").to_string()).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM injuries WHERE is_active=1 AND expected_return_date < ?").bind(new_date.format("%Y-%m-%d").to_string()).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE suspensions SET is_active=0 WHERE is_active=1").execute(pool).await.map_err(|e| e.to_string())?;

    let msg = format!("Temporada {} finalizada. {} campeones registrados, {} jugadores retirados, {} movimientos de división, {} ascensos por playoff y {} partidos de playoff generados. Nueva temporada {} comienza el {}.", season, honours, retired, movements, playoff_promotions, playoff_matches, next_season, new_date.format("%Y-%m-%d"));
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(pool).await.map_err(|e| e.to_string())?;
    if let Some(cid) = user_club {
        sqlx::query("INSERT INTO inbox_messages(club_id, sender_type, subject, body, date_sent, is_important) VALUES(?,'board','Fin de temporada',?, ?,1)")
            .bind(cid).bind(&msg).bind(new_date.format("%Y-%m-%d").to_string()).execute(pool).await.map_err(|e| e.to_string())?;
    }

    Ok(msg)
}
