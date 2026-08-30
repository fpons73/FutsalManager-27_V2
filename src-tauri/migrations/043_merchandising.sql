CREATE TABLE IF NOT EXISTS club_merchandising (
  club_id INTEGER PRIMARY KEY REFERENCES clubs(id) ON DELETE CASCADE,
  price REAL NOT NULL DEFAULT 35,
  product_quality INTEGER NOT NULL DEFAULT 60,
  demand INTEGER NOT NULL DEFAULT 50,
  total_units INTEGER NOT NULL DEFAULT 0,
  total_revenue REAL NOT NULL DEFAULT 0,
  updated_at TEXT
);

CREATE TABLE IF NOT EXISTS merchandising_sales (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  week_date TEXT NOT NULL,
  units INTEGER NOT NULL,
  unit_price REAL NOT NULL,
  revenue REAL NOT NULL,
  UNIQUE(club_id, week_date)
);

INSERT OR IGNORE INTO club_merchandising(club_id)
SELECT id FROM clubs;
