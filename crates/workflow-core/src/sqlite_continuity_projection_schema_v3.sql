CREATE UNIQUE INDEX events_full_identity
ON events(event_id, run_id, sequence_number);

ALTER TABLE snapshots RENAME TO snapshots_v2;

CREATE TABLE snapshots (
  run_id TEXT PRIMARY KEY CHECK (length(run_id) BETWEEN 1 AND 128),
  last_sequence_number INTEGER NOT NULL CHECK (last_sequence_number > 0),
  last_event_id TEXT NOT NULL CHECK (length(last_event_id) BETWEEN 1 AND 128),
  snapshot_commitment TEXT NOT NULL CHECK (length(snapshot_commitment) BETWEEN 1 AND 256),
  payload TEXT NOT NULL CHECK (length(payload) >= 2),
  FOREIGN KEY (last_event_id, run_id, last_sequence_number)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT
);

CREATE UNIQUE INDEX continuity_windows_run_identity
ON continuity_windows(window_id, workflow_id, run_id);

CREATE TABLE continuity_projection_bindings (
  operation_id TEXT PRIMARY KEY CHECK (length(operation_id) BETWEEN 1 AND 128),
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  operation_kind TEXT NOT NULL CHECK (operation_kind IN ('register_yield','transition_wait','consume_directive','record_attempt_outcome','recover_ambiguous_attempt')),
  disposition TEXT NOT NULL CHECK (disposition IN ('applied','security_rejected')),
  workflow_id TEXT NOT NULL CHECK (length(workflow_id) BETWEEN 1 AND 128),
  run_id TEXT NOT NULL CHECK (length(run_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL CHECK (length(window_id) BETWEEN 1 AND 128),
  request_commitment TEXT NOT NULL CHECK (length(request_commitment) BETWEEN 1 AND 256),
  projection_commitment TEXT NOT NULL UNIQUE CHECK (length(projection_commitment) BETWEEN 1 AND 256),
  expected_event_id TEXT NOT NULL CHECK (length(expected_event_id) BETWEEN 1 AND 128),
  expected_sequence INTEGER NOT NULL CHECK (expected_sequence > 0),
  result_event_id TEXT NOT NULL UNIQUE CHECK (length(result_event_id) BETWEEN 1 AND 128),
  result_sequence INTEGER NOT NULL UNIQUE CHECK (result_sequence > 1),
  snapshot_commitment TEXT NOT NULL CHECK (length(snapshot_commitment) BETWEEN 1 AND 256),
  target_kind TEXT NOT NULL CHECK (target_kind IN ('yield','wait','directive_attempt','attempt_outcome','ambiguity_recovery','security_rejection')),
  target_id TEXT NOT NULL CHECK (length(target_id) BETWEEN 1 AND 128),
  target_revision INTEGER NOT NULL CHECK (target_revision > 0),
  binding_json TEXT NOT NULL CHECK (length(binding_json) BETWEEN 2 AND 16384),
  CHECK (result_sequence = expected_sequence + 1),
  UNIQUE (operation_id, operation_kind),
  UNIQUE (result_event_id, run_id, result_sequence),
  FOREIGN KEY (operation_id, operation_kind)
    REFERENCES continuity_operations(operation_id, operation_kind) ON DELETE RESTRICT,
  FOREIGN KEY (expected_event_id, run_id, expected_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT,
  FOREIGN KEY (result_event_id, run_id, result_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT,
  FOREIGN KEY (window_id, workflow_id, run_id)
    REFERENCES continuity_windows(window_id, workflow_id, run_id) ON DELETE RESTRICT
);

CREATE INDEX continuity_projection_bindings_run
ON continuity_projection_bindings(run_id, result_sequence);
