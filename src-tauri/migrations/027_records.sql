CREATE TABLE IF NOT EXISTS club_records (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  competition_id INTEGER REFERENCES competitions(id),
  record_type TEXT NOT NULL,
  value REAL NOT NULL,
  season TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(club_id, competition_id, record_type, season)
);
CREATE TABLE IF NOT EXISTS competition_records (
  id INTEGER PRIMARY KEY,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  record_type TEXT NOT NULL,
  player_id INTEGER REFERENCES players(id),
  club_id INTEGER REFERENCES clubs(id),
  value REAL NOT NULL,
  season TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(competition_id, record_type, season)
);
CREATE INDEX IF NOT EXISTS idx_club_records_lookup ON club_records(club_id, record_type, value DESC);
CREATE INDEX IF NOT EXISTS idx_competition_records_lookup ON competition_records(competition_id, record_type, value DESC);
