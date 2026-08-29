CREATE TABLE IF NOT EXISTS player_season_stats (
  id INTEGER PRIMARY KEY,
  season TEXT NOT NULL,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  player_id INTEGER NOT NULL REFERENCES players(id),
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  appearances INTEGER NOT NULL DEFAULT 0,
  starts INTEGER NOT NULL DEFAULT 0,
  minutes_played INTEGER NOT NULL DEFAULT 0,
  goals INTEGER NOT NULL DEFAULT 0,
  assists INTEGER NOT NULL DEFAULT 0,
  shots INTEGER NOT NULL DEFAULT 0,
  shots_on_target INTEGER NOT NULL DEFAULT 0,
  fouls_committed INTEGER NOT NULL DEFAULT 0,
  yellow_cards INTEGER NOT NULL DEFAULT 0,
  red_cards INTEGER NOT NULL DEFAULT 0,
  rating_total REAL NOT NULL DEFAULT 0,
  UNIQUE(season, competition_id, player_id)
);
CREATE INDEX IF NOT EXISTS idx_player_season_stats_ranking ON player_season_stats(season, competition_id, goals DESC, rating_total DESC);
CREATE UNIQUE INDEX IF NOT EXISTS idx_match_player_stats_unique ON match_player_stats(match_id, player_id);
