ALTER TABLE live_sandbox_grants ADD COLUMN issuance_claim_hash TEXT
  CHECK (issuance_claim_hash IS NULL OR length(issuance_claim_hash) = 64);

CREATE TABLE installation_issuance_claims (
  attempt_id TEXT NOT NULL PRIMARY KEY CHECK (length(attempt_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL CHECK (length(window_id) BETWEEN 1 AND 128),
  operation_id TEXT NOT NULL UNIQUE CHECK (length(operation_id) BETWEEN 1 AND 128),
  payload TEXT NOT NULL CHECK (length(payload) BETWEEN 2 AND 4096),
  payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64),
  FOREIGN KEY (attempt_id, window_id) REFERENCES continuity_attempts(attempt_id, window_id) ON DELETE RESTRICT
);
