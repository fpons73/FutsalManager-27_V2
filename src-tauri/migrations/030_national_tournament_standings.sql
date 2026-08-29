CREATE TABLE IF NOT EXISTS national_tournament_entries (
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  season TEXT NOT NULL,
  nation_id INTEGER NOT NULL REFERENCES nations(id),
  group_code TEXT,
  position INTEGER NOT NULL DEFAULT 0,
  played INTEGER NOT NULL DEFAULT 0,
  won INTEGER NOT NULL DEFAULT 0,
  drawn INTEGER NOT NULL DEFAULT 0,
  lost INTEGER NOT NULL DEFAULT 0,
  goals_for INTEGER NOT NULL DEFAULT 0,
  goals_against INTEGER NOT NULL DEFAULT 0,
  points INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY(competition_id, season, nation_id)
);
CREATE INDEX IF NOT EXISTS idx_national_entries_comp ON national_tournament_entries(competition_id, season, position);
