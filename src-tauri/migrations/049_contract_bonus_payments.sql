CREATE TABLE IF NOT EXISTS contract_bonus_payments (
  id INTEGER PRIMARY KEY,
  contract_id INTEGER NOT NULL REFERENCES contracts(id),
  match_id INTEGER NOT NULL REFERENCES matches(id),
  bonus_type TEXT NOT NULL,
  amount REAL NOT NULL,
  created_at TEXT NOT NULL,
  UNIQUE(contract_id, match_id, bonus_type)
);
CREATE INDEX IF NOT EXISTS idx_contract_bonus_match ON contract_bonus_payments(match_id);
