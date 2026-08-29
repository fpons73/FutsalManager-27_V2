CREATE TABLE IF NOT EXISTS club_staff_costs (
  club_id INTEGER PRIMARY KEY REFERENCES clubs(id) ON DELETE CASCADE,
  weekly_cost REAL NOT NULL DEFAULT 0,
  staff_count INTEGER NOT NULL DEFAULT 0,
  updated_at TEXT
);

INSERT OR IGNORE INTO club_staff_costs(club_id, weekly_cost, staff_count)
SELECT c.id, COALESCE(SUM(s.wage_weekly), 0), COUNT(s.id)
FROM clubs c LEFT JOIN staff s ON s.club_id=c.id
GROUP BY c.id;
