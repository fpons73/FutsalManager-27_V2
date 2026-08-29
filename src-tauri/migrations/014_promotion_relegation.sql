ALTER TABLE competitions ADD COLUMN promotion_places INTEGER NOT NULL DEFAULT 0;
ALTER TABLE competitions ADD COLUMN relegation_places INTEGER NOT NULL DEFAULT 0;
ALTER TABLE competitions ADD COLUMN playoff_places INTEGER NOT NULL DEFAULT 0;

CREATE TABLE IF NOT EXISTS season_movements (
  id INTEGER PRIMARY KEY,
  season TEXT NOT NULL,
  from_competition_id INTEGER NOT NULL REFERENCES competitions(id),
  to_competition_id INTEGER NOT NULL REFERENCES competitions(id),
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  movement_type TEXT NOT NULL,
  source_position INTEGER,
  created_at TEXT NOT NULL,
  UNIQUE(season, from_competition_id, to_competition_id, club_id, movement_type)
);
CREATE INDEX IF NOT EXISTS idx_season_movements_season ON season_movements(season);
