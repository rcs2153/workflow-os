CREATE TABLE live_comment_observations (
  operation_id TEXT NOT NULL PRIMARY KEY REFERENCES dispatch_reservations(operation_id) ON DELETE RESTRICT,
  payload TEXT NOT NULL CHECK (length(payload) BETWEEN 2 AND 2048),
  payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64)
);
