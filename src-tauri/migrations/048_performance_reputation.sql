CREATE TABLE IF NOT EXISTS match_reputation_impacts (
  id INTEGER PRIMARY KEY,
  match_id INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  result TEXT NOT NULL CHECK(result IN ('win','draw','loss')),
  reputation_delta INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  UNIQUE(match_id, club_id)
);

CREATE INDEX IF NOT EXISTS idx_reputation_impacts_club ON match_reputation_impacts(club_id, created_at);
