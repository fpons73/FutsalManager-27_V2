CREATE TABLE IF NOT EXISTS sponsorship_offers (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  sponsor_name TEXT NOT NULL,
  weekly_amount REAL NOT NULL,
  signing_bonus REAL NOT NULL DEFAULT 0,
  target_type TEXT NOT NULL DEFAULT 'league_position',
  target_value INTEGER NOT NULL DEFAULT 8,
  duration_weeks INTEGER NOT NULL DEFAULT 52,
  expires_date TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'available'
);

CREATE TABLE IF NOT EXISTS sponsorship_contracts (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  sponsor_name TEXT NOT NULL,
  weekly_amount REAL NOT NULL,
  signing_bonus REAL NOT NULL DEFAULT 0,
  target_type TEXT NOT NULL DEFAULT 'league_position',
  target_value INTEGER NOT NULL DEFAULT 8,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active'
);

CREATE TABLE IF NOT EXISTS sponsorship_payments (
  id INTEGER PRIMARY KEY,
  contract_id INTEGER NOT NULL REFERENCES sponsorship_contracts(id) ON DELETE CASCADE,
  club_id INTEGER NOT NULL REFERENCES clubs(id) ON DELETE CASCADE,
  week_date TEXT NOT NULL,
  amount REAL NOT NULL,
  target_met INTEGER NOT NULL DEFAULT 0,
  UNIQUE(contract_id, week_date)
);

INSERT OR IGNORE INTO sponsorship_contracts(club_id,sponsor_name,weekly_amount,signing_bonus,target_type,target_value,start_date,end_date)
SELECT c.id, 'Velocity Sportswear', MAX(180.0,c.reputation*1.8), MAX(500.0,c.reputation*4.0), 'league_position', 8,
       COALESCE((SELECT game_date FROM game_state WHERE id=1),'2026-07-10'), '2027-06-30'
FROM clubs c;

INSERT INTO sponsorship_offers(club_id,sponsor_name,weekly_amount,signing_bonus,target_type,target_value,duration_weeks,expires_date)
SELECT c.id, 'CourtVision Energy', MAX(220.0,c.reputation*2.1), MAX(750.0,c.reputation*5.0), 'league_position', 6, 52,
       date(COALESCE((SELECT game_date FROM game_state WHERE id=1),'2026-07-10'), '+30 day')
FROM clubs c
WHERE NOT EXISTS (SELECT 1 FROM sponsorship_offers o WHERE o.club_id=c.id AND o.status='available');
