ALTER TABLE international_matches ADD COLUMN phase TEXT NOT NULL DEFAULT 'window';
ALTER TABLE international_matches ADD COLUMN round INTEGER NOT NULL DEFAULT 0;
ALTER TABLE international_matches ADD COLUMN group_code TEXT;
CREATE INDEX IF NOT EXISTS idx_int_matches_phase ON international_matches(competition_id, season, phase, round);
