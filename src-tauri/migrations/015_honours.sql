CREATE TABLE IF NOT EXISTS competition_honours (
  id INTEGER PRIMARY KEY,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  season TEXT NOT NULL,
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  honour_type TEXT NOT NULL,
  created_at TEXT NOT NULL,
  UNIQUE(competition_id, season, honour_type)
);
CREATE INDEX IF NOT EXISTS idx_competition_honours_club ON competition_honours(club_id, season);
