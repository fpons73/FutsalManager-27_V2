CREATE TABLE IF NOT EXISTS tv_rights_contracts (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  broadcaster TEXT NOT NULL DEFAULT 'Futsal Sports Network',
  weekly_amount REAL NOT NULL DEFAULT 0,
  bonus_per_match REAL NOT NULL DEFAULT 0,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active',
  UNIQUE(club_id, status)
);

CREATE TABLE IF NOT EXISTS tv_rights_payments (
  id INTEGER PRIMARY KEY,
  contract_id INTEGER NOT NULL REFERENCES tv_rights_contracts(id) ON DELETE CASCADE,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  week_date TEXT NOT NULL,
  amount REAL NOT NULL,
  UNIQUE(contract_id, week_date)
);

INSERT OR IGNORE INTO tv_rights_contracts(club_id, broadcaster, weekly_amount, bonus_per_match, start_date, end_date)
SELECT c.id, 'Futsal Sports Network', MAX(250.0, c.reputation * 3.0), MAX(50.0, c.reputation * 0.25),
       COALESCE((SELECT game_date FROM game_state WHERE id=1), '2026-07-10'), '2027-06-30'
FROM clubs c;
