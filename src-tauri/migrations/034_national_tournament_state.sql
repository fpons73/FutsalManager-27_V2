ALTER TABLE international_matches ADD COLUMN winner_nation_id INTEGER REFERENCES nations(id);
ALTER TABLE international_matches ADD COLUMN stage_status TEXT NOT NULL DEFAULT 'scheduled';
