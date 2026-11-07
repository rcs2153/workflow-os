CREATE TABLE live_sandbox_check_proofs (
  run_id TEXT NOT NULL CHECK (length(run_id) BETWEEN 1 AND 128),
  result_id TEXT NOT NULL CHECK (length(result_id) BETWEEN 1 AND 128),
  event_id TEXT NOT NULL REFERENCES events(event_id) ON DELETE RESTRICT,
  event_commitment TEXT NOT NULL CHECK (length(event_commitment) = 64),
  payload TEXT NOT NULL CHECK (length(payload) BETWEEN 2 AND 16384),
  PRIMARY KEY (run_id, result_id)
);
