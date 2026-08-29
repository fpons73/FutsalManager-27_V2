ALTER TABLE competitions ADD COLUMN national_format TEXT NOT NULL DEFAULT 'groups_knockout';
ALTER TABLE competitions ADD COLUMN national_status TEXT NOT NULL DEFAULT 'scheduled';
ALTER TABLE competitions ADD COLUMN national_champion_id INTEGER REFERENCES nations(id);
