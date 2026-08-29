ALTER TABLE stadiums ADD COLUMN photo_path TEXT;
CREATE INDEX IF NOT EXISTS idx_stadiums_city ON stadiums(city_id);
