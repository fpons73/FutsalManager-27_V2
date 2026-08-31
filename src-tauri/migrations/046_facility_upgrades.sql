CREATE TABLE IF NOT EXISTS club_facilities (
  club_id INTEGER PRIMARY KEY REFERENCES clubs(id) ON DELETE CASCADE,
  training_level INTEGER NOT NULL DEFAULT 1,
  youth_level INTEGER NOT NULL DEFAULT 1,
  commercial_level INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT
);

INSERT OR IGNORE INTO club_facilities(club_id,updated_at)
SELECT id, COALESCE((SELECT game_date FROM game_state WHERE id=1),'2026-07-10') FROM clubs;
