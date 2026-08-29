ALTER TABLE contracts ADD COLUMN contract_role TEXT NOT NULL DEFAULT 'rotation';
ALTER TABLE contracts ADD COLUMN signing_bonus REAL NOT NULL DEFAULT 0;
ALTER TABLE contracts ADD COLUMN appearance_bonus REAL NOT NULL DEFAULT 0;
ALTER TABLE contracts ADD COLUMN clean_sheet_bonus REAL NOT NULL DEFAULT 0;
ALTER TABLE contracts ADD COLUMN renewal_status TEXT NOT NULL DEFAULT 'none';
