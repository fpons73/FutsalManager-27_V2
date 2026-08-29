CREATE TABLE IF NOT EXISTS board_meetings (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  date TEXT NOT NULL,
  topic TEXT NOT NULL,
  decision TEXT NOT NULL,
  confidence_delta INTEGER NOT NULL DEFAULT 0,
  patience_delta INTEGER NOT NULL DEFAULT 0,
  notes TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_board_meetings_club_date ON board_meetings(club_id,date DESC,id DESC);
