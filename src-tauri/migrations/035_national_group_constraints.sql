CREATE UNIQUE INDEX IF NOT EXISTS idx_unique_international_match ON international_matches(competition_id, season, date, home_nation_id, away_nation_id, phase, round, group_code);
