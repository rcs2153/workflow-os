CREATE TABLE live_sandbox_targets (
  target_key TEXT PRIMARY KEY CHECK (length(target_key) BETWEEN 1 AND 256),
  payload TEXT NOT NULL CHECK (length(payload) BETWEEN 2 AND 4096),
  revoked INTEGER NOT NULL DEFAULT 0 CHECK (revoked IN (0, 1))
);
