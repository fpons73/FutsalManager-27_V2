CREATE TABLE IF NOT EXISTS player_attribute_history (
  id INTEGER PRIMARY KEY,
  season TEXT NOT NULL,
  player_id INTEGER NOT NULL REFERENCES players(id),
  club_id INTEGER REFERENCES clubs(id),
  current_ability INTEGER NOT NULL,
  potential_ability INTEGER NOT NULL,
  morale INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(season, player_id)
);
CREATE INDEX IF NOT EXISTS idx_player_attribute_history_player ON player_attribute_history(player_id, season);
