CREATE TABLE IF NOT EXISTS precontracts (
  id INTEGER PRIMARY KEY,
  player_id INTEGER NOT NULL REFERENCES players(id),
  from_club_id INTEGER NOT NULL REFERENCES clubs(id),
  to_club_id INTEGER NOT NULL REFERENCES clubs(id),
  agreed_date TEXT NOT NULL,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  wage_weekly REAL NOT NULL,
  signing_bonus REAL NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'pending',
  UNIQUE(player_id, to_club_id, status)
);
CREATE INDEX IF NOT EXISTS idx_precontracts_start ON precontracts(start_date, status);
