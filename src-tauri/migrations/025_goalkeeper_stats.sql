ALTER TABLE player_season_stats ADD COLUMN clean_sheets INTEGER NOT NULL DEFAULT 0;
ALTER TABLE player_season_stats ADD COLUMN saves INTEGER NOT NULL DEFAULT 0;
CREATE INDEX IF NOT EXISTS idx_player_season_stats_gk ON player_season_stats(clean_sheets DESC, saves DESC);
