ALTER TABLE nations ADD COLUMN iso2 TEXT;
ALTER TABLE nations ADD COLUMN iso3 TEXT;
ALTER TABLE players ADD COLUMN secondary_position TEXT;
CREATE INDEX IF NOT EXISTS idx_nations_iso2 ON nations(iso2);
CREATE INDEX IF NOT EXISTS idx_nations_iso3 ON nations(iso3);
