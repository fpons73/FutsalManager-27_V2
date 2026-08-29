CREATE TABLE IF NOT EXISTS staff (
    id INTEGER PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    common_name TEXT,
    nation_id INTEGER REFERENCES nations(id),
    role TEXT NOT NULL CHECK(role IN ('coach','assistant','scout','physio','fitness_coach','goalkeeper_coach','technical_coach','analyst')),
    club_id INTEGER REFERENCES clubs(id),
    tactical INTEGER DEFAULT 10,
    man_management INTEGER DEFAULT 10,
    judging INTEGER DEFAULT 10,
    motivating INTEGER DEFAULT 10,
    working_youngsters INTEGER DEFAULT 10,
    physio_level INTEGER DEFAULT 10,
    wage_weekly REAL DEFAULT 500
);

ALTER TABLE clubs ADD COLUMN crest_path TEXT;
ALTER TABLE clubs ADD COLUMN coach_id INTEGER;
CREATE INDEX IF NOT EXISTS idx_staff_club ON staff(club_id);
