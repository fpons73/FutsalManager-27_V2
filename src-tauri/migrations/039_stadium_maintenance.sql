CREATE TABLE IF NOT EXISTS stadium_operations (
  stadium_id INTEGER PRIMARY KEY REFERENCES stadiums(id) ON DELETE CASCADE,
  condition INTEGER NOT NULL DEFAULT 85,
  weekly_cost REAL NOT NULL DEFAULT 2500,
  last_maintenance TEXT
);
INSERT OR IGNORE INTO stadium_operations(stadium_id)
SELECT id FROM stadiums;
