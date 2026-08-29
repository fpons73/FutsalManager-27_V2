ALTER TABLE contracts ADD COLUMN loan_parent_id INTEGER REFERENCES contracts(id);
ALTER TABLE contracts ADD COLUMN loan_until TEXT;
CREATE INDEX IF NOT EXISTS idx_contracts_loan_until ON contracts(loan_until);
