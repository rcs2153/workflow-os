CREATE TABLE installation_send_preparations (
  operation_id TEXT PRIMARY KEY REFERENCES dispatch_reservations(operation_id) ON DELETE RESTRICT,
  payload TEXT NOT NULL CHECK (length(payload) BETWEEN 2 AND 4096),
  payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64)
);
