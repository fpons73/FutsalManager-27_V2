CREATE TABLE IF NOT EXISTS national_team_staff (
  id INTEGER PRIMARY KEY,
  nation_id INTEGER NOT NULL REFERENCES nations(id),
  staff_id INTEGER NOT NULL REFERENCES staff(id),
  role TEXT NOT NULL DEFAULT 'manager',
  active INTEGER NOT NULL DEFAULT 1,
  UNIQUE(nation_id, role)
);
CREATE TABLE IF NOT EXISTS national_team_squads (
  id INTEGER PRIMARY KEY,
  nation_id INTEGER NOT NULL REFERENCES nations(id),
  player_id INTEGER NOT NULL REFERENCES players(id),
  callup_status TEXT NOT NULL DEFAULT 'eligible',
  selected INTEGER NOT NULL DEFAULT 0,
  UNIQUE(nation_id, player_id)
);
CREATE TABLE IF NOT EXISTS international_matches (
  id INTEGER PRIMARY KEY,
  competition_id INTEGER REFERENCES competitions(id),
  season TEXT NOT NULL,
  date TEXT NOT NULL,
  home_nation_id INTEGER NOT NULL REFERENCES nations(id),
  away_nation_id INTEGER NOT NULL REFERENCES nations(id),
  status TEXT NOT NULL DEFAULT 'scheduled',
  home_score INTEGER NOT NULL DEFAULT 0,
  away_score INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_national_squads_nation ON national_team_squads(nation_id, selected);
