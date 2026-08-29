CREATE TABLE IF NOT EXISTS club_leadership (
  club_id INTEGER PRIMARY KEY REFERENCES clubs(id) ON DELETE CASCADE,
  captain_player_id INTEGER REFERENCES players(id) ON DELETE SET NULL,
  vice_captain_player_id INTEGER REFERENCES players(id) ON DELETE SET NULL,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS player_relationships (
  player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
  other_player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
  compatibility INTEGER NOT NULL DEFAULT 50,
  PRIMARY KEY(player_id, other_player_id),
  CHECK(player_id <> other_player_id)
);

CREATE TABLE IF NOT EXISTS player_promises (
  id INTEGER PRIMARY KEY,
  player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  promise_type TEXT NOT NULL,
  target_value INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'active',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  resolved_at TEXT
);

INSERT OR IGNORE INTO club_leadership(club_id)
SELECT id FROM clubs;
