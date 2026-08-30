CREATE TABLE IF NOT EXISTS club_travel_finance (
  club_id INTEGER PRIMARY KEY REFERENCES clubs(id) ON DELETE CASCADE,
  travel_spend REAL NOT NULL DEFAULT 0,
  away_matches INTEGER NOT NULL DEFAULT 0,
  updated_at TEXT
);

CREATE TABLE IF NOT EXISTS match_travel_costs (
  match_id INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  origin_city_id INTEGER,
  destination_city_id INTEGER,
  distance_km REAL NOT NULL DEFAULT 0,
  cost REAL NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  PRIMARY KEY(match_id, club_id)
);

INSERT OR IGNORE INTO club_travel_finance(club_id)
SELECT id FROM clubs;
