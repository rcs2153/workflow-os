CREATE TABLE installation_observation_journal (
  operation_id TEXT NOT NULL REFERENCES installation_send_preparations(operation_id) ON DELETE RESTRICT,
  evidence_id TEXT NOT NULL CHECK (length(evidence_id) = 64),
  payload TEXT NOT NULL CHECK (length(payload) BETWEEN 2 AND 4096),
  payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64),
  PRIMARY KEY (operation_id, evidence_id)
);
