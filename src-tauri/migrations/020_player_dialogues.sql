CREATE TABLE IF NOT EXISTS player_dialogues (
  id INTEGER PRIMARY KEY,
  player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  topic TEXT NOT NULL,
  response TEXT NOT NULL,
  morale_delta INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_player_dialogues_club ON player_dialogues(club_id, created_at);
