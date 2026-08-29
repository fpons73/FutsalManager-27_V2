CREATE TABLE IF NOT EXISTS world_news (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  news_type TEXT NOT NULL,
  headline TEXT NOT NULL,
  body TEXT NOT NULL,
  date TEXT NOT NULL,
  importance INTEGER NOT NULL DEFAULT 0,
  is_read INTEGER NOT NULL DEFAULT 0,
  UNIQUE(club_id, news_type, headline, date)
);
CREATE INDEX IF NOT EXISTS idx_world_news_club_date ON world_news(club_id, date DESC, id DESC);
