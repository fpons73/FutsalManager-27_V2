CREATE TABLE IF NOT EXISTS national_tournament_honours (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  season TEXT NOT NULL,
  nation_id INTEGER NOT NULL REFERENCES nations(id),
  honour_type TEXT NOT NULL DEFAULT 'champion',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(competition_id, season, honour_type)
);
CREATE TABLE IF NOT EXISTS national_knockout_ties (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  season TEXT NOT NULL,
  phase TEXT NOT NULL,
  round INTEGER NOT NULL,
  match_id INTEGER NOT NULL UNIQUE REFERENCES international_matches(id),
  home_nation_id INTEGER NOT NULL REFERENCES nations(id),
  away_nation_id INTEGER NOT NULL REFERENCES nations(id),
  winner_nation_id INTEGER REFERENCES nations(id),
  UNIQUE(competition_id, season, phase, round, home_nation_id, away_nation_id)
);
CREATE INDEX IF NOT EXISTS idx_national_honours_nation ON national_tournament_honours(nation_id, season);
