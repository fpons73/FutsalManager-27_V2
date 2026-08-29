use chrono::NaiveDate;
use sqlx::SqlitePool;

pub fn build_round_robin(team_ids: &[i64]) -> Vec<Vec<(i64, i64)>> {
    let n = team_ids.len();
    let mut teams = team_ids.to_vec();
    let is_odd = n % 2 == 1;
    if is_odd {
        teams.push(-1);
    }
    let m = teams.len();
    let rounds = m - 1;
    let mut result: Vec<Vec<(i64, i64)>> = Vec::with_capacity(rounds * 2);
    let mut current = teams.clone();
    for _ in 0..rounds {
        let mut round: Vec<(i64, i64)> = Vec::new();
        for i in 0..(m / 2) {
            let a = current[i];
            let b = current[m - 1 - i];
            if a != -1 && b != -1 {
                round.push((a, b));
            }
        }
        result.push(round);
        let last = current.pop().unwrap();
        current.insert(1, last);
    }
    let first_leg = result.clone();
    let mut second_leg: Vec<Vec<(i64, i64)>> = Vec::new();
    for round in first_leg.iter() {
        second_leg.push(round.iter().map(|(a, b)| (*b, *a)).collect());
    }
    result.extend(second_leg);
    result
}

async fn generate_next_cup_round(pool: &SqlitePool, comp_id: i64, season: &str) -> Result<(), String> {
    let (round,): (Option<i64>,) = sqlx::query_as("SELECT MAX(round) FROM cup_ties WHERE competition_id=? AND season=?").bind(comp_id).bind(season).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let next_round = round.unwrap_or(1) + 1;
    let (max_round,): (i64,) = sqlx::query_as("SELECT knockout_rounds FROM competitions WHERE id=?").bind(comp_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if next_round > max_round { return Ok(()); }
    let winners: Vec<(i64,)> = sqlx::query_as("SELECT winner_club_id FROM cup_ties WHERE competition_id=? AND season=? AND round=? AND winner_club_id IS NOT NULL ORDER BY id").bind(comp_id).bind(season).bind(next_round - 1).fetch_all(pool).await.map_err(|e| e.to_string())?;
    if winners.len() < 2 || winners.len() % 2 != 0 { return Ok(()); }
    let existing_next: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cup_ties WHERE competition_id=? AND season=? AND round=?").bind(comp_id).bind(season).bind(next_round).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if existing_next.0 > 0 { return Ok(()); }
    let start = NaiveDate::from_ymd_opt(season.split('/').next().and_then(|s| s.parse().ok()).unwrap_or(2026), 9, 1).unwrap();
    for (idx, pair) in winners.chunks(2).enumerate() {
        let date = (start + chrono::Duration::days((next_round * 7 + idx as i64) * 7)).format("%Y-%m-%d").to_string();
        let (mid,): (i64,) = sqlx::query_as("INSERT INTO matches(competition_id,season,round,date,home_club_id,away_club_id,stadium_id,status) VALUES(?,?,?,?,?,?,(SELECT stadium_id FROM clubs WHERE id=?),'scheduled') RETURNING id").bind(comp_id).bind(season).bind(next_round).bind(&date).bind(pair[0].0).bind(pair[1].0).bind(pair[0].0).fetch_one(pool).await.map_err(|e| e.to_string())?;
        sqlx::query("INSERT INTO cup_ties(competition_id,season,round,leg,home_club_id,away_club_id,match_id) VALUES(?,?,?,1,?,?,?)").bind(comp_id).bind(season).bind(next_round).bind(pair[0].0).bind(pair[1].0).bind(mid).execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn generate_calendars(pool: &SqlitePool) -> Result<(), String> {
    // Las competiciones de copa se generan como eliminatorias de partido único.
    let cups: Vec<(i64,String,i64,String)> = sqlx::query_as("SELECT id,season,total_teams,name FROM competitions WHERE competition_type='cup' AND knockout_rounds>0 AND kind='club'").fetch_all(pool).await.map_err(|e|e.to_string())?;
    for (comp_id, season, _, name) in cups {
        let is_supercup = name.to_lowercase().contains("supercopa") || name.to_lowercase().contains("supercup");
        let existing:(i64,) = sqlx::query_as("SELECT COUNT(*) FROM cup_ties WHERE competition_id=? AND season=?").bind(comp_id).bind(&season).fetch_one(pool).await.map_err(|e|e.to_string())?;
        if existing.0 > 0 {
            let _ = generate_next_cup_round(pool, comp_id, &season).await?;
            continue;
        }
        let teams: Vec<(i64,)> = if is_supercup {
            let nation: Option<(i64,)> = sqlx::query_as("SELECT nation_id FROM competitions WHERE id=?").bind(comp_id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
            let nation_id = nation.map(|(id,)| id);
            let champions: Vec<(i64,)> = sqlx::query_as("SELECT club_id FROM competition_honours WHERE honour_type IN ('league','cup') AND season < ? AND competition_id IN (SELECT id FROM competitions WHERE nation_id=?) ORDER BY season DESC, CASE honour_type WHEN 'league' THEN 0 ELSE 1 END LIMIT 2").bind(&season).bind(nation_id).fetch_all(pool).await.unwrap_or_default();
            if champions.len() >= 2 { champions } else {
                sqlx::query_as("SELECT club_id FROM league_standings WHERE competition_id IN (SELECT id FROM competitions WHERE nation_id=(SELECT nation_id FROM competitions WHERE id=?) AND tier=1) AND season=? ORDER BY position LIMIT 4").bind(comp_id).bind(&season).fetch_all(pool).await.map_err(|e|e.to_string())?
            }
        } else {
            sqlx::query_as("SELECT club_id FROM (SELECT club_id, MIN(position) AS seed FROM league_standings WHERE competition_id IN (SELECT id FROM competitions WHERE nation_id=(SELECT nation_id FROM competitions WHERE id=?) AND tier IS NOT NULL) GROUP BY club_id ORDER BY seed ASC, club_id LIMIT (SELECT total_teams FROM competitions WHERE id=?))").bind(comp_id).bind(comp_id).fetch_all(pool).await.map_err(|e|e.to_string())?
        };
        let start=NaiveDate::from_ymd_opt(season.split('/').next().and_then(|s|s.parse().ok()).unwrap_or(2026),9,1).unwrap();
        let mut unique_teams: Vec<(i64,)> = Vec::new();
        for (id,) in teams { if !unique_teams.iter().any(|(existing,)| *existing == id) { unique_teams.push((id,)); } }
        for (idx,pair) in unique_teams.chunks(2).enumerate() {
            if pair.len() < 2 { continue; }
            let date = (start + chrono::Duration::days(idx as i64 * 7)).format("%Y-%m-%d").to_string();
            let (mid,): (i64,) = sqlx::query_as("INSERT INTO matches(competition_id,season,round,date,home_club_id,away_club_id,stadium_id,status) VALUES(?,?,?,?,?,?,(SELECT stadium_id FROM clubs WHERE id=?),'scheduled') RETURNING id")
                .bind(comp_id).bind(&season).bind(1_i64).bind(&date).bind(pair[0].0).bind(pair[1].0).bind(pair[0].0).fetch_one(pool).await.map_err(|e|e.to_string())?;
            sqlx::query("INSERT INTO cup_ties(competition_id,season,round,leg,home_club_id,away_club_id,match_id) VALUES(?,?,1,1,?,?,?)")
                .bind(comp_id).bind(&season).bind(pair[0].0).bind(pair[1].0).bind(mid).execute(pool).await.map_err(|e|e.to_string())?;
        }
    }


    let comps: Vec<(i64, String)> =
        sqlx::query_as("SELECT id, season FROM competitions WHERE competition_type='league' AND kind='club'")
            .fetch_all(pool).await.map_err(|e| e.to_string())?;

    for (comp_id, season) in comps {
        let season_year: i32 = season.split('/').next().and_then(|s| s.parse().ok()).unwrap_or(2026);
        let start = NaiveDate::from_ymd_opt(season_year, 8, 22).unwrap_or(NaiveDate::from_ymd_opt(2026, 8, 22).unwrap());
        let mut club_rows: Vec<(i64,)> =
            sqlx::query_as("SELECT club_id FROM league_standings WHERE competition_id=? AND season=? ORDER BY club_id")
                .bind(comp_id).bind(&season).fetch_all(pool).await.map_err(|e| e.to_string())?;
        if club_rows.is_empty() {
            club_rows = sqlx::query_as("SELECT club_id FROM league_standings WHERE competition_id=? ORDER BY club_id").bind(comp_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
            if club_rows.is_empty() { continue; }
        }
        let team_ids: Vec<i64> = club_rows.into_iter().map(|(id,)| id).collect();

        let rounds = build_round_robin(&team_ids);

        let existing: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE competition_id=? AND season=?")
            .bind(comp_id).bind(&season).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if existing.0 > 0 { continue; }
        if team_ids.len() < 2 { continue; }

        for (idx, round) in rounds.iter().enumerate() {
            let date = start + chrono::Duration::days(idx as i64 * 7);
            let date_s = date.format("%Y-%m-%d").to_string();
            let round_no = (idx + 1) as i64;
            for (home, away) in round {
                let stadium: Option<(i64,)> =
                    sqlx::query_as("SELECT stadium_id FROM clubs WHERE id=?")
                        .bind(home).fetch_optional(pool).await.map_err(|e| e.to_string())?;
                let sid = stadium.and_then(|(s,)| Some(s));
                sqlx::query("INSERT INTO matches(competition_id,season,round,date,home_club_id,away_club_id,stadium_id,status) VALUES(?,?,?,?,?,?,?, 'scheduled')")
                    .bind(comp_id).bind(&season).bind(round_no).bind(&date_s).bind(home).bind(away).bind(sid)
                    .execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::world;

    #[tokio::test]
    async fn calendar_counts_and_balance() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        generate_calendars(&pool).await.unwrap();

        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches").fetch_one(&pool).await.unwrap();
        assert!(total >= 662, "al menos 662 partidos, got {}", total);

        for (comp_name, n) in [("Primera División de Fútbol Sala", 16), ("Liga Nacional de Futsal (LNF)", 16), ("Liga Placard", 14)] {
            let (comp_id,): (i64,) = sqlx::query_as("SELECT id FROM competitions WHERE name=?").bind(comp_name).fetch_one(&pool).await.unwrap();
            let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE competition_id=?").bind(comp_id).fetch_one(&pool).await.unwrap();
            let expected = (n as i64) * ((n as i64) - 1);
            assert_eq!(cnt, expected, "{comp_name} partidos");

            let clubs: Vec<(i64,)> = sqlx::query_as("SELECT club_id FROM league_standings WHERE competition_id=?").bind(comp_id).fetch_all(&pool).await.unwrap();
            for (cid,) in clubs {
                let (played,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE competition_id=? AND (home_club_id=? OR away_club_id=?)")
                    .bind(comp_id).bind(cid).bind(cid).fetch_one(&pool).await.unwrap();
                assert_eq!(played, (n as i64 - 1) * 2, "club {cid} debe jugar {} partidos", (n - 1) * 2);
            }
        }

        let (distinct_rounds,): (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT round) FROM matches WHERE competition_id=(SELECT id FROM competitions WHERE name='Primera División de Fútbol Sala')")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(distinct_rounds, 30);
    }

    #[test]
    fn builder_pure_logic() {
        let teams: Vec<i64> = (1..=4).collect();
        let rounds = build_round_robin(&teams);
        assert_eq!(rounds.len(), 6);
        for r in &rounds { assert_eq!(r.len(), 2); }
        let all: Vec<(i64,i64)> = rounds.iter().flatten().copied().collect();
        for &t in &teams {
            let cnt = all.iter().filter(|(a,b)| *a==t || *b==t).count();
            assert_eq!(cnt, 6);
        }
    }
}
