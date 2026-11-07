CREATE TABLE live_sandbox_grants (
  grant_id TEXT PRIMARY KEY CHECK (length(grant_id) BETWEEN 1 AND 128),
  attempt_id TEXT NOT NULL REFERENCES continuity_attempts(attempt_id) ON DELETE RESTRICT,
  binding TEXT NOT NULL CHECK (length(binding) = 64),
  payload TEXT NOT NULL CHECK (length(payload) BETWEEN 2 AND 32768),
  revoked INTEGER NOT NULL DEFAULT 0 CHECK (revoked IN (0, 1)),
  UNIQUE (grant_id, attempt_id)
);

CREATE INDEX live_sandbox_grants_attempt ON live_sandbox_grants(attempt_id);

CREATE TABLE live_sandbox_grant_uses (
  grant_id TEXT PRIMARY KEY,
  attempt_id TEXT NOT NULL UNIQUE REFERENCES dispatch_reservations(attempt_id)
    DEFERRABLE INITIALLY DEFERRED,
  FOREIGN KEY (grant_id, attempt_id) REFERENCES live_sandbox_grants(grant_id, attempt_id)
    ON DELETE RESTRICT
);
