ALTER TABLE competitions ADD COLUMN competition_type TEXT NOT NULL DEFAULT 'league';
ALTER TABLE competitions ADD COLUMN knockout_rounds INTEGER NOT NULL DEFAULT 0;
CREATE TABLE IF NOT EXISTS cup_ties (
  id INTEGER PRIMARY KEY,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  season TEXT NOT NULL,
  round INTEGER NOT NULL,
  leg INTEGER NOT NULL DEFAULT 1,
  home_club_id INTEGER NOT NULL REFERENCES clubs(id),
  away_club_id INTEGER NOT NULL REFERENCES clubs(id),
  match_id INTEGER REFERENCES matches(id),
  winner_club_id INTEGER,
  went_to_extra_time INTEGER NOT NULL DEFAULT 0,
  went_to_penalties INTEGER NOT NULL DEFAULT 0,
  penalty_home_score INTEGER NOT NULL DEFAULT 0,
  penalty_away_score INTEGER NOT NULL DEFAULT 0,
  UNIQUE(competition_id, season, round, leg, home_club_id, away_club_id)
);
CREATE INDEX IF NOT EXISTS idx_cup_ties_comp ON cup_ties(competition_id, season, round);
