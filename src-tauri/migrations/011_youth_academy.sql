CREATE TABLE IF NOT EXISTS youth_teams (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  age_group INTEGER NOT NULL CHECK(age_group IN (12,14,16,18,20)),
  name TEXT NOT NULL,
  coach_id INTEGER REFERENCES staff(id),
  UNIQUE(club_id, age_group)
);
CREATE TABLE IF NOT EXISTS youth_players (
  id INTEGER PRIMARY KEY,
  youth_team_id INTEGER NOT NULL REFERENCES youth_teams(id),
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  common_name TEXT NOT NULL,
  date_of_birth TEXT NOT NULL,
  nation_id INTEGER NOT NULL REFERENCES nations(id),
  second_nation_id INTEGER REFERENCES nations(id),
  position TEXT NOT NULL,
  secondary_position TEXT,
  current_ability INTEGER NOT NULL DEFAULT 20,
  potential_ability INTEGER NOT NULL DEFAULT 80,
  development REAL NOT NULL DEFAULT 0,
  promoted_to_first_team INTEGER NOT NULL DEFAULT 0,
  created_date TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS youth_promotions (
  id INTEGER PRIMARY KEY,
  youth_player_id INTEGER NOT NULL REFERENCES youth_players(id),
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  promotion_date TEXT NOT NULL,
  contract_id INTEGER REFERENCES contracts(id)
);
CREATE INDEX IF NOT EXISTS idx_youth_teams_club ON youth_teams(club_id);
CREATE INDEX IF NOT EXISTS idx_youth_players_team ON youth_players(youth_team_id);
