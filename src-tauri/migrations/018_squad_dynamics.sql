CREATE TABLE IF NOT EXISTS club_dynamics (
  club_id INTEGER PRIMARY KEY REFERENCES clubs(id) ON DELETE CASCADE,
  chemistry INTEGER NOT NULL DEFAULT 60,
  cohesion INTEGER NOT NULL DEFAULT 60,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS player_roles (
  player_id INTEGER PRIMARY KEY REFERENCES players(id) ON DELETE CASCADE,
  squad_role TEXT NOT NULL DEFAULT 'rotation',
  leadership INTEGER NOT NULL DEFAULT 0,
  promised_minutes INTEGER NOT NULL DEFAULT 0,
  promised_role TEXT,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO club_dynamics(club_id)
SELECT id FROM clubs;

INSERT OR IGNORE INTO player_roles(player_id, squad_role)
SELECT c.player_id, COALESCE(c.contract_role, 'rotation') FROM contracts c;
