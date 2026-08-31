CREATE TABLE IF NOT EXISTS club_ticketing (
  club_id INTEGER PRIMARY KEY REFERENCES clubs(id) ON DELETE CASCADE,
  ticket_price REAL NOT NULL DEFAULT 12 CHECK(ticket_price >= 1 AND ticket_price <= 250),
  demand INTEGER NOT NULL DEFAULT 50 CHECK(demand >= 0 AND demand <= 100),
  last_attendance INTEGER NOT NULL DEFAULT 0,
  updated_at TEXT
);

CREATE TABLE IF NOT EXISTS match_ticket_sales (
  id INTEGER PRIMARY KEY,
  match_id INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  attendance INTEGER NOT NULL,
  ticket_price REAL NOT NULL,
  revenue REAL NOT NULL,
  created_at TEXT NOT NULL,
  UNIQUE(match_id, club_id)
);

INSERT OR IGNORE INTO club_ticketing(club_id)
SELECT id FROM clubs;
