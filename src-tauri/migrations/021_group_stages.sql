ALTER TABLE competitions ADD COLUMN group_count INTEGER NOT NULL DEFAULT 0;
ALTER TABLE competitions ADD COLUMN teams_per_group INTEGER NOT NULL DEFAULT 0;
ALTER TABLE competitions ADD COLUMN group_qualifiers INTEGER NOT NULL DEFAULT 0;

CREATE TABLE IF NOT EXISTS competition_groups (
  id INTEGER PRIMARY KEY,
  competition_id INTEGER NOT NULL REFERENCES competitions(id) ON DELETE CASCADE,
  season TEXT NOT NULL,
  group_code TEXT NOT NULL,
  UNIQUE(competition_id, season, group_code)
);

CREATE TABLE IF NOT EXISTS group_members (
  group_id INTEGER NOT NULL REFERENCES competition_groups(id) ON DELETE CASCADE,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  position INTEGER NOT NULL DEFAULT 0,
  played INTEGER NOT NULL DEFAULT 0,
  won INTEGER NOT NULL DEFAULT 0,
  drawn INTEGER NOT NULL DEFAULT 0,
  lost INTEGER NOT NULL DEFAULT 0,
  goals_for INTEGER NOT NULL DEFAULT 0,
  goals_against INTEGER NOT NULL DEFAULT 0,
  points INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY(group_id, club_id)
);

CREATE INDEX IF NOT EXISTS idx_group_members_club ON group_members(club_id);
