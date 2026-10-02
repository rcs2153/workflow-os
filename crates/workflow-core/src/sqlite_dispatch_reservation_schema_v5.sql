CREATE TABLE dispatch_reservations (
  attempt_id TEXT PRIMARY KEY CHECK (length(attempt_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL CHECK (length(window_id) BETWEEN 1 AND 128),
  operation_id TEXT NOT NULL UNIQUE CHECK (length(operation_id) BETWEEN 1 AND 128),
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  request_commitment TEXT NOT NULL CHECK (length(request_commitment) BETWEEN 1 AND 256),
  reservation_commitment TEXT NOT NULL UNIQUE CHECK (length(reservation_commitment) BETWEEN 1 AND 256),
  result_event_id TEXT NOT NULL UNIQUE CHECK (length(result_event_id) BETWEEN 1 AND 128),
  result_sequence INTEGER NOT NULL UNIQUE CHECK (result_sequence > 0),
  committed_seconds INTEGER NOT NULL,
  committed_nanos INTEGER NOT NULL CHECK (committed_nanos BETWEEN 0 AND 999999999),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  FOREIGN KEY (attempt_id, window_id) REFERENCES continuity_attempts(attempt_id, window_id) ON DELETE RESTRICT,
  FOREIGN KEY (result_event_id) REFERENCES events(event_id) ON DELETE RESTRICT
);

CREATE TABLE dispatch_reservation_projection_bindings (
  operation_id TEXT PRIMARY KEY REFERENCES dispatch_reservations(operation_id) ON DELETE RESTRICT,
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  projection_commitment TEXT NOT NULL UNIQUE CHECK (length(projection_commitment) BETWEEN 1 AND 256),
  snapshot_commitment TEXT NOT NULL CHECK (length(snapshot_commitment) BETWEEN 1 AND 256),
  binding_json TEXT NOT NULL CHECK (length(binding_json) BETWEEN 2 AND 16384),
  FOREIGN KEY (receipt_id) REFERENCES dispatch_reservations(receipt_id) ON DELETE RESTRICT
);
