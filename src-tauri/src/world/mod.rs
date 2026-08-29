pub mod data;
pub mod prd;
pub mod iso;

use chrono::NaiveDate;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sqlx::SqlitePool;

use data::{BRAZIL_CLUBS, PORTUGAL_CLUBS, SPAIN_CLUBS};

const SEASON: &str = "2026/2027";
const GAME_DATE: &str = "2026-07-10";

pub async fn seed_world(pool: &SqlitePool) -> Result<(), String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let mut conf_ids: std::collections::HashMap<&str, i64> = Default::default();
    for (name, short, rep) in [("UEFA","UEFA",950),("CONMEBOL","CONMEBOL",900),("AFC","AFC",850),("CAF","CAF",750),("OFC","OFC",600),("CONCACAF","CONCACAF",650)] {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO confederations(name,short_name,reputation) VALUES(?,?,?) RETURNING id")
            .bind(name).bind(short).bind(rep)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        conf_ids.insert(short, id);
    }

    let mut nation_ids: std::collections::HashMap<String, i64> = Default::default();
    for (name, conf, rep, level) in prd::ALL_NATIONS {
        let cid = *conf_ids.get(conf).unwrap();
        let (id,): (i64,) = sqlx::query_as("INSERT INTO nations(name,confederation_id,reputation,futsal_level) VALUES(?,?,?,?) RETURNING id")
            .bind(*name).bind(cid).bind(*rep).bind(*level)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        nation_ids.insert(name.to_string(), id);
    }

    let mut city_ids: std::collections::HashMap<String, i64> = Default::default();
    let mut all_cities: Vec<(&str, String)> = Vec::new();
    for c in SPAIN_CLUBS { all_cities.push((c.city, "España".to_string())); }
    for c in BRAZIL_CLUBS { all_cities.push((c.city, "Brasil".to_string())); }
    for c in PORTUGAL_CLUBS { all_cities.push((c.city, "Portugal".to_string())); }
    let mut seen = std::collections::HashSet::new();
    for (city, nat) in all_cities {
        if seen.contains(city) { continue; }
        seen.insert(city.to_string());
        if let Some(&nid) = nation_ids.get(&nat) {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name,nation_id,population) VALUES(?,?,500000) RETURNING id")
                .bind(city).bind(nid)
                .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
            city_ids.insert(city.to_string(), id);
        }
    }
    for (nation, _, _, _) in prd::ALL_NATIONS {
        let cap_name = format!("{} Capital", nation);
        if !city_ids.contains_key(&cap_name) {
            if let Some(&nid) = nation_ids.get(*nation) {
                let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name,nation_id,population) VALUES(?,?,300000) RETURNING id")
                    .bind(&cap_name).bind(nid).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
                city_ids.insert(cap_name, id);
            }
        }
    }

    let mut comp_ids: Vec<i64> = Vec::new();
    for comp in prd::ALL_COMPS {
        let nid = comp.nation.and_then(|n| nation_ids.get(n).copied());
        let kind = if comp.nation.is_none() { "national_team" } else { "club" };
        let competition_type = if comp.tier.is_some() { "league" } else { "cup" };
        let knockout_rounds = if competition_type == "cup" { 4 } else { 0 };
        let (id,): (i64,) = sqlx::query_as("INSERT INTO competitions(name,nation_id,tier,total_teams,season,format,kind,competition_type,knockout_rounds,promotion_places,relegation_places,playoff_places) VALUES(?,?,?,?,?,?,?,?,?,?,?,?) RETURNING id")
            .bind(comp.name).bind(nid).bind(comp.tier).bind(comp.teams).bind(SEASON).bind(if comp.tier.is_some() { "Round Robin" } else { "Cup" }).bind(kind).bind(competition_type).bind(knockout_rounds).bind(if comp.tier.is_some() { 2 } else { 0 }).bind(if comp.tier.is_some() { 2 } else { 0 }).bind(0)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        comp_ids.push(id);
    }

    let mut rng = StdRng::from_entropy();
    let base_date = NaiveDate::parse_from_str(GAME_DATE, "%Y-%m-%d").unwrap();
    seed_free_agents(&mut tx, &nation_ids, base_date, &mut rng).await?;
    seed_free_staff(&mut tx, &nation_ids, &mut rng).await?;
    let base_date = NaiveDate::parse_from_str(GAME_DATE, "%Y-%m-%d").unwrap();

    // Para cada nación con ligas, cuántos clubes DISTINTOS hacen falta y sus divisiones (tier asc)
    let mut nation_divisions: std::collections::HashMap<String, Vec<i64>> = Default::default(); // nation -> [teams por tier asc]
    for comp in prd::ALL_COMPS {
        if let Some(nation) = comp.nation {
            if comp.tier.is_some() {
                nation_divisions.entry(nation.to_string()).or_default().push(comp.teams);
            }
        }
    }
    let nation_needed: std::collections::HashMap<String, i64> = nation_divisions.iter().map(|(n, ts)| (n.clone(), ts.iter().sum())).collect();

    struct OwnedClub { name: String, short: String, city: String, stadium: String, capacity: i64, rep: i64, c1: String, c2: String, nid: i64, nation: String }
    let mut clubs_to_create: Vec<OwnedClub> = Vec::new();
    let mut created_by_nation: std::collections::HashMap<String, usize> = Default::default();

    // España
    if let Some(&nid) = nation_ids.get("España") {
        for c in SPAIN_CLUBS { clubs_to_create.push(OwnedClub { name: c.name.into(), short: c.short.into(), city: c.city.into(), stadium: c.stadium.into(), capacity: c.capacity, rep: c.reputation, c1: c.color.into(), c2: c.color2.into(), nid, nation: "España".into() }); }
        created_by_nation.insert("España".into(), SPAIN_CLUBS.len());
    }
    // Brasil
    if let Some(&nid) = nation_ids.get("Brasil") {
        for c in BRAZIL_CLUBS { clubs_to_create.push(OwnedClub { name: c.name.into(), short: c.short.into(), city: c.city.into(), stadium: c.stadium.into(), capacity: c.capacity, rep: c.reputation, c1: c.color.into(), c2: c.color2.into(), nid, nation: "Brasil".into() }); }
        created_by_nation.insert("Brasil".into(), BRAZIL_CLUBS.len());
    }
    // Portugal
    if let Some(&nid) = nation_ids.get("Portugal") {
        for c in PORTUGAL_CLUBS { clubs_to_create.push(OwnedClub { name: c.name.into(), short: c.short.into(), city: c.city.into(), stadium: c.stadium.into(), capacity: c.capacity, rep: c.reputation, c1: c.color.into(), c2: c.color2.into(), nid, nation: "Portugal".into() }); }
        created_by_nation.insert("Portugal".into(), PORTUGAL_CLUBS.len());
    }
    // Rellenar hasta el número necesario por nación (divisiones inferiores) con clubes genéricos
    for (nation, &needed) in &nation_needed {
        let nid = match nation_ids.get(nation) { Some(v) => *v, None => continue };
        let have = *created_by_nation.get(nation.as_str()).unwrap_or(&0);
        let start_rep = if ["España","Brasil","Portugal"].contains(&nation.as_str()) { 700 } else { 720 };
        for i in have..(needed as usize) {
            // reputación descendente para que la 1ª división quede con los mejores
            let rep = (start_rep - ((i as i64) * 8)).max(480);
            let city_name = format!("{} Capital", nation);
            let first2 = nation.chars().take(3).collect::<String>().to_uppercase();
            clubs_to_create.push(OwnedClub {
                name: format!("{} Futsal {}", nation, i+1),
                short: format!("{}{}", first2, i+1),
                city: city_name.clone(),
                stadium: format!("{} Arena {}", nation, i+1),
                capacity: 1500 + (rep % 3000), rep, c1: "#0f4c3a".into(), c2: "#ffffff".into(), nid, nation: nation.clone(),
            });
        }
    }

    let mut club_ids: Vec<i64> = Vec::new();
    let mut club_nation: Vec<String> = Vec::new();
    let mut club_rep: Vec<i64> = Vec::new();

    for oc in clubs_to_create {
        let city_id = city_ids.get(&oc.city).copied().unwrap_or_else(|| *city_ids.values().next().unwrap());
        let (stadium_id,): (i64,) = sqlx::query_as("INSERT INTO stadiums(name,city_id,capacity,pitch_type) VALUES(?,?,?, 'parquet') RETURNING id")
            .bind(&oc.stadium).bind(city_id).bind(oc.capacity)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        let (club_id,): (i64,) = sqlx::query_as("INSERT INTO clubs(name,short_name,nation_id,city_id,stadium_id,reputation,primary_color,secondary_color) VALUES(?,?,?,?,?,?,?,?) RETURNING id")
            .bind(&oc.name).bind(&oc.short).bind(oc.nid).bind(city_id).bind(stadium_id).bind(oc.rep).bind(&oc.c1).bind(&oc.c2)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        club_ids.push(club_id);
        club_nation.push(oc.nation.clone());
        club_rep.push(oc.rep);
        let balance = (oc.rep as f64) * 1800.0 + rng.gen_range(50_000.0..250_000.0);
        let wage_budget = (oc.rep as f64) * 12.0 + 2000.0;
        sqlx::query("INSERT INTO club_finances(club_id,balance,transfer_budget,wage_budget,total_wages) VALUES(?,?,?,?,0)").bind(club_id).bind(balance).bind(balance*0.25).bind(wage_budget).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        let formations = ["3-1","4-0","2-2"];
        let f = formations[rng.gen_range(0..formations.len())];
        sqlx::query("INSERT INTO tactics(club_id,formation,tempo,pressing,defensive_line,width,playing_style,powerplay_enabled) VALUES(?,?,?,?,?,?,?,1)").bind(club_id).bind(f).bind(rng.gen_range(40..75) as i64).bind(rng.gen_range(40..80) as i64).bind(rng.gen_range(35..70) as i64).bind(rng.gen_range(40..70) as i64).bind(if rng.gen_bool(0.5) {"balanced"}else{"counter"}).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        let nat_for_names: &str = if ["España","Brasil","Portugal"].contains(&oc.nation.as_str()) { &oc.nation } else { "España" };
        generate_squad(&mut tx, club_id, oc.nid, nat_for_names, oc.rep, base_date, &mut rng).await?;
        for (day, type_id, intensity) in [(0,1,70),(1,2,75),(2,4,65),(3,3,75),(4,7,60)] {
            sqlx::query("INSERT OR IGNORE INTO training_schedule(club_id, day_of_week, training_type_id, intensity) VALUES(?,?,?,?)").bind(club_id).bind(day).bind(type_id).bind(intensity).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        }
        sqlx::query("INSERT OR IGNORE INTO youth_academy(club_id, level) VALUES(?,50)").bind(club_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        sqlx::query("INSERT OR IGNORE INTO scouting_centers(club_id, knowledge_level, max_scouts) VALUES(?, ?, ?)").bind(club_id).bind((oc.rep / 10).clamp(15, 80)).bind(if oc.rep >= 700 { 3 } else if oc.rep >= 600 { 2 } else { 1 }).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        seed_staff_for_club(&mut tx, club_id, oc.nid, &oc.nation, oc.rep, &mut rng).await?;
    }

    // Agrupar clubes por nación, ordenados por reputación desc
    let mut nation_clubs: std::collections::HashMap<String, Vec<(i64, i64)>> = Default::default();
    for (idx, cid) in club_ids.iter().enumerate() {
        nation_clubs.entry(club_nation[idx].clone()).or_default().push((*cid, club_rep[idx]));
    }
    for v in nation_clubs.values_mut() { v.sort_by(|a, b| b.1.cmp(&a.1)); }

    // Asignar a CADA división su propio tramo de clubes (tier asc -> slice consecutivo)
    let mut offset_by_nation: std::collections::HashMap<String, usize> = Default::default();
    let mut comps_tiered: Vec<&prd::CompDef> = prd::ALL_COMPS.iter().filter(|c| c.tier.is_some()).collect();
    comps_tiered.sort_by_key(|c| (c.nation.unwrap_or(""), c.tier.unwrap_or(0)));
    for comp in comps_tiered {
        let nation = match comp.nation { Some(n) => n, None => continue };
        let clubs_for_nation = match nation_clubs.get(nation) { Some(v) => v, None => continue };
        let offset = *offset_by_nation.get(nation).unwrap_or(&0);
        let take = (comp.teams as usize).min(clubs_for_nation.len().saturating_sub(offset));
        let selected: Vec<i64> = clubs_for_nation.iter().skip(offset).take(take).map(|(id, _)| *id).collect();
        offset_by_nation.insert(nation.to_string(), offset + take);
        if selected.is_empty() { continue; }
        let comp_id: i64 = sqlx::query_as::<_, (i64,)>("SELECT id FROM competitions WHERE name=? AND season=?").bind(comp.name).bind(SEASON).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?.0;
        for &club in &selected {
            sqlx::query("INSERT INTO league_standings(competition_id,season,club_id,position,played,won,drawn,lost,goals_for,goals_against,goal_difference,points,form_last_5) VALUES(?,?,?,?,0,0,0,0,0,0,0,0,'')").bind(comp_id).bind(SEASON).bind(club).bind(0).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        }
    }

    sqlx::query("INSERT INTO game_state(id, game_date, season, game_speed) VALUES(1, ?, ?, 'normal')").bind(GAME_DATE).bind(SEASON).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    let _ = iso::import_catalog(pool).await;
    crate::competition::generate_calendars(pool).await?;
    Ok(())
}

async fn seed_free_agents(tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, nations: &std::collections::HashMap<String,i64>, base_date: NaiveDate, rng: &mut impl Rng) -> Result<(), String> {
    let pool_nations: Vec<(&String,&i64)> = nations.iter().collect();
    for idx in 0..24 {
        let (nation_name, &nation_id) = pool_nations[idx % pool_nations.len()];
        let role = ["POR","CIE","ALA","PIV","UNI"][idx % 5];
        let age = 18 + (idx % 14) as i64;
        let dob = base_date - chrono::Duration::days(age * 365);
        let ca = 55 + rng.gen_range(0..35) as i64;
        let pa = (ca + rng.gen_range(10..45) as i64).min(180);
        let first = data::pick_first(nation_name, rng); let last = data::pick_last(nation_name, rng);
        let secondary = ["POR","CIE","ALA","PIV","UNI"][(idx + 2) % 5];
        let (pid,): (i64,) = sqlx::query_as("INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,second_nation_id,secondary_position,height_cm,weight_kg) VALUES(?,?,?,?,?,?,?,?,?) RETURNING id").bind(first).bind(&last).bind(format!("{} {}",first,last)).bind(dob.format("%Y-%m-%d").to_string()).bind(nation_id).bind(if idx % 4 == 0 { Some(nations.values().copied().nth((idx+1)%nations.len()).unwrap()) } else { None }).bind(secondary).bind(170).bind(72).fetch_one(&mut **tx).await.map_err(|e|e.to_string())?;
        let (por,cie,ala,piv,uni) = match role { "POR"=>(20,5,1,1,3), "CIE"=>(1,20,12,8,10), "ALA"=>(1,10,20,10,14), "PIV"=>(1,6,10,20,12), _=>(3,10,14,14,20) };
        sqlx::query("INSERT INTO player_positions(player_id,por_natural,cie_natural,ala_natural,piv_natural,uni_natural) VALUES(?,?,?,?,?,?)").bind(pid).bind(por).bind(cie).bind(ala).bind(piv).bind(uni).execute(&mut **tx).await.map_err(|e|e.to_string())?;
        sqlx::query("INSERT INTO player_states(player_id,current_ability,potential_ability) VALUES(?,?,?)").bind(pid).bind(ca).bind(pa).execute(&mut **tx).await.map_err(|e|e.to_string())?;
        sqlx::query("INSERT INTO player_attributes(player_id) VALUES(?)").bind(pid).execute(&mut **tx).await.map_err(|e|e.to_string())?;
    }
    Ok(())
}

async fn seed_free_staff(tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, nations: &std::collections::HashMap<String,i64>, rng: &mut impl Rng) -> Result<(), String> {
    let names: Vec<(&String,&i64)> = nations.iter().collect();
    let roles = ["coach","assistant","scout","physio","fitness_coach","goalkeeper_coach","technical_coach","analyst"];
    for idx in 0..16 {
        let (nation,&nation_id)=names[idx % names.len()]; let first=data::pick_first(nation,rng); let last=data::pick_last(nation,rng); let base=(8+rng.gen_range(0..9)).clamp(1,20);
        sqlx::query("INSERT INTO staff(first_name,last_name,common_name,nation_id,role,club_id,tactical,man_management,judging,motivating,working_youngsters,physio_level,wage_weekly) VALUES(?,?,?,?,?,NULL,?,?,?,?,?,?,?)").bind(first).bind(&last).bind(format!("{} {}",first,last)).bind(nation_id).bind(roles[idx%roles.len()]).bind(base).bind(base).bind(base).bind(base).bind(base).bind(base).bind(400.0+base as f64*80.0).execute(&mut **tx).await.map_err(|e|e.to_string())?;
    }
    Ok(())
}

async fn seed_staff_for_club(tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, club_id: i64, nation_id: i64, nation: &str, reputation: i64, rng: &mut impl Rng) -> Result<(), String> {
    let roles = ["coach", "assistant", "scout", "physio"];
    for (idx, role) in roles.iter().enumerate() {
        let first = data::pick_first(nation, rng);
        let last = data::pick_last(nation, rng);
        let base = (reputation / 70 + rng.gen_range(-2..=3)).clamp(5, 20);
        let tactical = (base + if *role == "coach" { 4 } else { 0 }).clamp(1, 20);
        let management = (base + rng.gen_range(-2..=2)).clamp(1, 20);
        let judging = (base + if *role == "scout" { 5 } else { 0 }).clamp(1, 20);
        let motivating = (base + if *role == "assistant" { 4 } else { 0 }).clamp(1, 20);
        let youngsters = (base + if idx == 0 { 2 } else { 0 }).clamp(1, 20);
        let physio = (base + if *role == "physio" { 6 } else { 0 }).clamp(1, 20);
        let wage = (reputation as f64 * (if *role == "coach" { 1.4 } else { 0.9 }) + rng.gen_range(100.0..500.0)).round();
        sqlx::query("INSERT INTO staff(first_name,last_name,common_name,nation_id,role,club_id,tactical,man_management,judging,motivating,working_youngsters,physio_level,wage_weekly) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?)")
            .bind(first).bind(&last).bind(format!("{} {}", first, last)).bind(nation_id).bind(role).bind(club_id).bind(tactical).bind(management).bind(judging).bind(motivating).bind(youngsters).bind(physio).bind(wage)
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

async fn generate_squad(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    club_id: i64,
    nation_id: i64,
    nation_name: &str,
    reputation: i64,
    base_date: NaiveDate,
    rng: &mut impl Rng,
) -> Result<(), String> {
    let roles: &[&str] = &["POR","POR","CIE","CIE","ALA","ALA","ALA","ALA","PIV","PIV","UNI","UNI"];
    for (idx, role) in roles.iter().enumerate() {
        let age: i64 = if idx < 2 { rng.gen_range(22..36) } else if idx < 8 { rng.gen_range(19..32) } else { rng.gen_range(20..34) };
        let dob = base_date - chrono::Duration::days(age * 365 + rng.gen_range(0..365) as i64);
        let dob_s = dob.format("%Y-%m-%d").to_string();
        let first = data::pick_first(nation_name, rng);
        let last = data::pick_last(nation_name, rng);
        let foot = ["right","left","both"][rng.gen_range(0..3)];
        let height = rng.gen_range(168..195);
        let weight = rng.gen_range(65..92);

        let (ca, pa) = gen_ca_pa(reputation, age, rng);
        let attrs = gen_attributes(role, ca, rng);

        let (pid,): (i64,) = sqlx::query_as(
            "INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,preferred_foot,height_cm,weight_kg) VALUES(?,?,?,?,?,?,?,?) RETURNING id"
        )
        .bind(first).bind(last).bind(format!("{first} {last}")).bind(&dob_s).bind(nation_id).bind(foot).bind(height).bind(weight)
        .fetch_one(&mut **tx).await.map_err(|e| e.to_string())?;

        let (por, cie, ala, piv, uni) = match *role {
            "POR" => (20, 2, 1, 1, 3),
            "CIE" => (1, 20, 12, 8, 10),
            "ALA" => (1, 10, 20, 10, 14),
            "PIV" => (1, 6, 10, 20, 12),
            _ => (3, 10, 14, 14, 20),
        };
        sqlx::query("INSERT INTO player_positions(player_id,por_natural,cie_natural,ala_natural,piv_natural,uni_natural) VALUES(?,?,?,?,?,?)")
            .bind(pid).bind(por).bind(cie).bind(ala).bind(piv).bind(uni)
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;

        sqlx::query("INSERT INTO player_states(player_id,current_ability,potential_ability,condition_val,match_fitness,morale,sharpness,happiness) VALUES(?,?,?,?,?,?,?,?)")
            .bind(pid).bind(ca).bind(pa).bind(100).bind(rng.gen_range(85..100)).bind(rng.gen_range(60..90)).bind(rng.gen_range(40..80)).bind(rng.gen_range(60..90))
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO player_attributes(player_id,first_touch,dribbling,ball_control,technique,passing,vision,crossing,long_shots,finishing,heading,penalty_taking,tackling,marking,interception,blocking,anticipation,decisions,positioning,off_the_ball,work_rate,composure,concentration,determination,bravery,aggression,leadership,teamwork,flair,acceleration,pace,agility,balance,stamina,strength,jumping,reflexes,handling,one_on_ones,positioning_gk,rushing_out,throwing,kicking,professionalism,consistency,important_matches,injury_proneness) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"
        )
        .bind(pid)
        .bind(attrs[0]).bind(attrs[1]).bind(attrs[2]).bind(attrs[3]).bind(attrs[4]).bind(attrs[5]).bind(attrs[6]).bind(attrs[7]).bind(attrs[8]).bind(attrs[9]).bind(attrs[10]).bind(attrs[11]).bind(attrs[12]).bind(attrs[13]).bind(attrs[14]).bind(attrs[15]).bind(attrs[16]).bind(attrs[17]).bind(attrs[18]).bind(attrs[19]).bind(attrs[20]).bind(attrs[21]).bind(attrs[22]).bind(attrs[23]).bind(attrs[24]).bind(attrs[25]).bind(attrs[26]).bind(attrs[27]).bind(attrs[28]).bind(attrs[29]).bind(attrs[30]).bind(attrs[31]).bind(attrs[32]).bind(attrs[33]).bind(attrs[34]).bind(attrs[35]).bind(attrs[36]).bind(attrs[37]).bind(attrs[38]).bind(attrs[39]).bind(attrs[40]).bind(attrs[41]).bind(attrs[42]).bind(attrs[43]).bind(attrs[44]).bind(attrs[45])
        .execute(&mut **tx).await.map_err(|e| e.to_string())?;

        let wage = (ca as f64 * 18.0 + rng.gen_range(0.0..400.0)).round();
        let years = rng.gen_range(1..4);
        let end = base_date + chrono::Duration::days(years * 365);
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)")
            .bind(pid).bind(club_id).bind(wage).bind(GAME_DATE).bind(end.format("%Y-%m-%d").to_string())
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;

        let total: f64 = sqlx::query_scalar::<_, Option<f64>>("SELECT SUM(wage_weekly) FROM contracts WHERE club_id=? AND is_active=1")
            .bind(club_id).fetch_one(&mut **tx).await.map_err(|e| e.to_string())?.unwrap_or(0.0);
        sqlx::query("UPDATE club_finances SET total_wages=? WHERE club_id=?").bind(total).bind(club_id)
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn gen_ca_pa(rep: i64, age: i64, rng: &mut impl Rng) -> (i64, i64) {
    let base = 62 + (rep - 520) * 45 / 420;
    let mut ca = (base as f64 + rng.gen_range(-8.0..10.0)).round() as i64;
    ca = ca.clamp(45, 185);
    if age < 21 { ca = (ca as f64 * 0.72) as i64; }
    else if age < 24 { ca = (ca as f64 * 0.86) as i64; }
    else if age > 32 { ca = (ca as f64 * 0.92) as i64; }
    ca = ca.clamp(40, 190);
    let gap = if age <= 20 { rng.gen_range(18..50) } else if age <= 24 { rng.gen_range(8..32) } else if age <= 28 { rng.gen_range(2..16) } else { rng.gen_range(0..6) };
    let mut pa = ca + gap;
    pa = pa.clamp(ca, 200);
    (ca, pa)
}

fn gen_attributes(role: &str, ca: i64, rng: &mut impl Rng) -> Vec<i64> {
    let base = (ca as f64 / 2.0).clamp(0.0, 100.0);
    let mut v = Vec::with_capacity(46);
    let bonuses: Vec<f64> = match role {
        "POR" => vec![0.,0.,0.,0., 0.,0.,0.,0., -15.,-10.,0., -15.,-15.,-10.,-10., -5.,-5.,-5.,-10.,-5., -5.,0.,0.,0.,0.,0.,0., -10., -5.,-5.,-5.,-5.,0.,-5.,0., 25.,25.,25.,25.,25.,25.,25., 0.,0.,0.,0.],
        "CIE" => vec![0.,-5.,0.,0., 5.,5.,-5.,-5., -10.,0.,0., 20.,20.,20.,20., 15.,10.,15.,0.,10., 5.,5.,5.,5.,5.,10.,5., -5., 0.,0.,5.,5.,10.,10.,0., -20.,-20.,-20.,-20.,-20.,-20.,-20., 0.,0.,0.,0.],
        "ALA" => vec![5.,15.,10.,10., 5.,5.,5.,0., 0.,0.,0., -5.,-5.,0.,-5., 0.,0.,0.,5.,5., 0.,0.,0.,0.,0.,0.,0., 10., 15.,15.,15.,10.,5.,0.,0., -20.,-20.,-20.,-20.,-20.,-20.,-20., 0.,0.,0.,0.],
        "PIV" => vec![5.,0.,10.,5., 0.,0.,-5.,0., 20.,10.,5., -10.,-10.,-5.,0., 0.,5.,0.,10.,5., 5.,5.,0.,0.,5.,0.,0., 0., -5.,-5.,0.,5.,5.,15.,5., -20.,-20.,-20.,-20.,-20.,-20.,-20., 0.,0.,0.,0.],
        _     => vec![0.,5.,5.,5., 5.,5.,0.,0., 5.,0.,0., 0.,0.,0.,0., 5.,5.,5.,5.,5., 0.,0.,0.,0.,0.,0.,0., 5., 5.,5.,5.,5.,5.,5.,0., -10.,-10.,-10.,-10.,-10.,-10.,-10., 0.,0.,0.,0.],
    };
    for i in 0..46 {
        let b = bonuses.get(i).copied().unwrap_or(0.0);
        let mut val = base + b + rng.gen_range(-12.5..12.5);
        val = val.clamp(0.0, 100.0);
        v.push(val.round() as i64);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[tokio::test]
    async fn seed_creates_expected_counts() {
        let pool = db::init_memory_pool().await.unwrap();
        seed_world(&pool).await.unwrap();
        let (clubs,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs").fetch_one(&pool).await.unwrap();
        let (players,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM players").fetch_one(&pool).await.unwrap();
        let (comps,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM competitions").fetch_one(&pool).await.unwrap();
        let (stadiums,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stadiums").fetch_one(&pool).await.unwrap();
        assert!(clubs >= 46, "al menos 46 clubes, got {}", clubs);
        assert_eq!(players, clubs * 12 + 24, "12 jugadores por club más 24 agentes libres");
        assert_eq!(comps, 43, "43 competiciones del PRD (ligas, copas, 2ª división y selecciones)");
        assert_eq!(stadiums, clubs);
        let (standings,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_standings").fetch_one(&pool).await.unwrap();
        assert!(standings >= 46, "al menos 46 standings");
        let (fin,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM club_finances").fetch_one(&pool).await.unwrap();
        assert_eq!(fin, clubs);
        let (contracts,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts").fetch_one(&pool).await.unwrap();
        assert_eq!(contracts, clubs * 12, "solo los jugadores de club empiezan contratados");
        let (staff,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM staff").fetch_one(&pool).await.unwrap();
        assert_eq!(staff, clubs * 4 + 16, "cada club recibe cuatro perfiles y hay staff libre");
        let (matches,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches").fetch_one(&pool).await.unwrap();
        assert!(matches >= 662, "al menos 662 partidos, got {}", matches);
        let (d0,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&pool).await.unwrap();
        assert_eq!(d0, "2026-07-10");

        // Verificar que las divisiones españolas tienen clubes distintos (pirámide completa)
        let (primera,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_standings WHERE competition_id=(SELECT id FROM competitions WHERE name='Primera División de Fútbol Sala')").fetch_one(&pool).await.unwrap();
        let (segunda,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_standings WHERE competition_id=(SELECT id FROM competitions WHERE name='Segunda División de Fútbol Sala')").fetch_one(&pool).await.unwrap();
        assert_eq!(primera, 16, "Primera española 16 equipos");
        assert_eq!(segunda, 16, "Segunda española 16 equipos");
        let (overlap,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM league_standings a JOIN league_standings b ON a.club_id=b.club_id WHERE a.competition_id=(SELECT id FROM competitions WHERE name='Primera División de Fútbol Sala') AND b.competition_id=(SELECT id FROM competitions WHERE name='Segunda División de Fútbol Sala')"
        ).fetch_one(&pool).await.unwrap();
        assert_eq!(overlap, 0, "Primera y Segunda no comparten clubes");
        // Segunda B total (6 grupos)
        let (sb,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_standings WHERE competition_id IN (SELECT id FROM competitions WHERE name LIKE 'Segunda División B - Grupo%')").fetch_one(&pool).await.unwrap();
        assert_eq!(sb, 60, "Segunda B española 6 grupos x 10 = 60 equipos");
        let (esp_clubs,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs c JOIN nations n ON n.id=c.nation_id WHERE n.name='España'").fetch_one(&pool).await.unwrap();
        assert_eq!(esp_clubs, 92, "España 92 clubes (16+16+60)");
    }
}

