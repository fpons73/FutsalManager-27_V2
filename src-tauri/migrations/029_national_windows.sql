CREATE TABLE IF NOT EXISTS international_windows (
  id INTEGER PRIMARY KEY,
  season TEXT NOT NULL,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  window_type TEXT NOT NULL DEFAULT 'friendly',
  UNIQUE(season, start_date, end_date)
);

CREATE TABLE IF NOT EXISTS national_fixtures (
  id INTEGER PRIMARY KEY,
  international_match_id INTEGER NOT NULL REFERENCES international_matches(id) ON DELETE CASCADE,
  calendar_match_id INTEGER REFERENCES matches(id) ON DELETE SET NULL,
  window_id INTEGER REFERENCES international_windows(id),
  UNIQUE(international_match_id)
);

CREATE INDEX IF NOT EXISTS idx_international_matches_date ON international_matches(date);
