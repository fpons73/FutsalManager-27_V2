CREATE TABLE IF NOT EXISTS scouting_centers (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL UNIQUE REFERENCES clubs(id),
  knowledge_level INTEGER NOT NULL DEFAULT 15,
  max_scouts INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS scout_assignments (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  nation_id INTEGER REFERENCES nations(id),
  target_club_id INTEGER REFERENCES clubs(id),
  target_name TEXT NOT NULL,
  knowledge_gain INTEGER NOT NULL DEFAULT 3,
  is_active INTEGER NOT NULL DEFAULT 1,
  UNIQUE(club_id, target_name)
);

CREATE TABLE IF NOT EXISTS player_knowledge (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  player_id INTEGER NOT NULL REFERENCES players(id),
  knowledge_percentage INTEGER NOT NULL DEFAULT 0,
  UNIQUE(club_id, player_id)
);

CREATE INDEX IF NOT EXISTS idx_player_knowledge_club ON player_knowledge(club_id, knowledge_percentage);
