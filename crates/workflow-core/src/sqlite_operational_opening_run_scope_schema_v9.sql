PRAGMA defer_foreign_keys = ON;

ALTER TABLE operational_opening_operations RENAME TO operational_opening_operations_v8;
ALTER TABLE operational_opening_attempts RENAME TO operational_opening_attempts_v8;
ALTER TABLE operational_opening_projection_bindings RENAME TO operational_opening_projection_bindings_v8;

CREATE TABLE operational_opening_operations (
  operation_id TEXT PRIMARY KEY CHECK (length(operation_id) BETWEEN 1 AND 128),
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  version TEXT NOT NULL CHECK (version = 'v2'),
  request_commitment TEXT NOT NULL CHECK (length(request_commitment) BETWEEN 1 AND 256),
  workflow_id TEXT NOT NULL CHECK (length(workflow_id) BETWEEN 1 AND 128),
  run_id TEXT NOT NULL CHECK (length(run_id) BETWEEN 1 AND 128),
  step_id TEXT NOT NULL CHECK (length(step_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL UNIQUE CHECK (length(window_id) BETWEEN 1 AND 128),
  attempt_id TEXT NOT NULL UNIQUE CHECK (length(attempt_id) BETWEEN 1 AND 128),
  subject_actor_id TEXT NOT NULL CHECK (length(subject_actor_id) BETWEEN 1 AND 128),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  governance_commitment TEXT NOT NULL CHECK (length(governance_commitment) BETWEEN 1 AND 256),
  operation_binding TEXT NOT NULL CHECK (operation_binding = 'invoke_current_step_skill'),
  operation_binding_commitment TEXT NOT NULL CHECK (length(operation_binding_commitment) BETWEEN 1 AND 256),
  expected_snapshot_commitment TEXT NOT NULL CHECK (length(expected_snapshot_commitment) BETWEEN 1 AND 256),
  expected_event_id TEXT NOT NULL CHECK (length(expected_event_id) BETWEEN 1 AND 128),
  expected_sequence INTEGER NOT NULL CHECK (expected_sequence > 0),
  result_event_id TEXT NOT NULL UNIQUE CHECK (length(result_event_id) BETWEEN 1 AND 128),
  result_sequence INTEGER NOT NULL CHECK (result_sequence = expected_sequence + 1),
  attempt_number INTEGER NOT NULL CHECK (attempt_number = 1),
  window_revision INTEGER NOT NULL CHECK (window_revision = 1),
  committed_seconds INTEGER NOT NULL,
  committed_nanos INTEGER NOT NULL CHECK (committed_nanos BETWEEN 0 AND 999999999),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (run_id, result_sequence),
  UNIQUE (window_id, operation_binding_commitment),
  FOREIGN KEY (window_id, workflow_id, run_id)
    REFERENCES continuity_windows(window_id, workflow_id, run_id) ON DELETE RESTRICT,
  FOREIGN KEY (expected_event_id, run_id, expected_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT,
  FOREIGN KEY (result_event_id, run_id, result_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT
);

CREATE TABLE operational_opening_attempts (
  attempt_id TEXT PRIMARY KEY CHECK (length(attempt_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL UNIQUE REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  operation_id TEXT NOT NULL UNIQUE REFERENCES operational_opening_operations(operation_id) ON DELETE RESTRICT,
  attempt_number INTEGER NOT NULL CHECK (attempt_number = 1),
  subject_actor_id TEXT NOT NULL CHECK (length(subject_actor_id) BETWEEN 1 AND 128),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  operation_binding_commitment TEXT NOT NULL CHECK (length(operation_binding_commitment) BETWEEN 1 AND 256),
  state TEXT NOT NULL CHECK (state = 'started'),
  revision INTEGER NOT NULL CHECK (revision = 1),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (attempt_id, window_id, operation_id)
);

CREATE TABLE operational_opening_projection_bindings (
  operation_id TEXT PRIMARY KEY REFERENCES operational_opening_operations(operation_id) ON DELETE RESTRICT,
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  workflow_id TEXT NOT NULL CHECK (length(workflow_id) BETWEEN 1 AND 128),
  run_id TEXT NOT NULL CHECK (length(run_id) BETWEEN 1 AND 128),
  projection_commitment TEXT NOT NULL UNIQUE CHECK (length(projection_commitment) BETWEEN 1 AND 256),
  snapshot_commitment TEXT NOT NULL CHECK (length(snapshot_commitment) BETWEEN 1 AND 256),
  binding_json TEXT NOT NULL CHECK (length(binding_json) BETWEEN 2 AND 16384),
  FOREIGN KEY (receipt_id) REFERENCES operational_opening_operations(receipt_id) ON DELETE RESTRICT
);

INSERT INTO operational_opening_operations
SELECT * FROM operational_opening_operations_v8;

INSERT INTO operational_opening_attempts
SELECT * FROM operational_opening_attempts_v8;

INSERT INTO operational_opening_projection_bindings
SELECT * FROM operational_opening_projection_bindings_v8;

DROP TABLE operational_opening_projection_bindings_v8;
DROP TABLE operational_opening_attempts_v8;
DROP TABLE operational_opening_operations_v8;

CREATE UNIQUE INDEX operational_opening_one_scope
ON operational_opening_operations(workflow_id, run_id, step_id, operation_binding_commitment);

ALTER TABLE dispatch_reservations RENAME TO dispatch_reservations_v8;
ALTER TABLE dispatch_reservation_projection_bindings RENAME TO dispatch_reservation_projection_bindings_v8;

CREATE TABLE dispatch_reservations (
  attempt_id TEXT PRIMARY KEY CHECK (length(attempt_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL CHECK (length(window_id) BETWEEN 1 AND 128),
  run_id TEXT NOT NULL CHECK (length(run_id) BETWEEN 1 AND 128),
  operation_id TEXT NOT NULL UNIQUE CHECK (length(operation_id) BETWEEN 1 AND 128),
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  request_commitment TEXT NOT NULL CHECK (length(request_commitment) BETWEEN 1 AND 256),
  reservation_commitment TEXT NOT NULL UNIQUE CHECK (length(reservation_commitment) BETWEEN 1 AND 256),
  result_event_id TEXT NOT NULL UNIQUE CHECK (length(result_event_id) BETWEEN 1 AND 128),
  result_sequence INTEGER NOT NULL CHECK (result_sequence > 0),
  committed_seconds INTEGER NOT NULL,
  committed_nanos INTEGER NOT NULL CHECK (committed_nanos BETWEEN 0 AND 999999999),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (run_id, result_sequence),
  FOREIGN KEY (attempt_id, window_id) REFERENCES continuity_attempts(attempt_id, window_id) ON DELETE RESTRICT,
  FOREIGN KEY (result_event_id, run_id, result_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT
);

CREATE TABLE dispatch_reservation_projection_bindings (
  operation_id TEXT PRIMARY KEY REFERENCES dispatch_reservations(operation_id) ON DELETE RESTRICT,
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  projection_commitment TEXT NOT NULL UNIQUE CHECK (length(projection_commitment) BETWEEN 1 AND 256),
  snapshot_commitment TEXT NOT NULL CHECK (length(snapshot_commitment) BETWEEN 1 AND 256),
  binding_json TEXT NOT NULL CHECK (length(binding_json) BETWEEN 2 AND 16384),
  FOREIGN KEY (receipt_id) REFERENCES dispatch_reservations(receipt_id) ON DELETE RESTRICT
);

INSERT INTO dispatch_reservations
  (attempt_id, window_id, run_id, operation_id, receipt_id, request_commitment,
   reservation_commitment, result_event_id, result_sequence, committed_seconds,
   committed_nanos, record_json)
SELECT d.attempt_id, d.window_id, w.run_id, d.operation_id, d.receipt_id,
       d.request_commitment, d.reservation_commitment, d.result_event_id,
       d.result_sequence, d.committed_seconds, d.committed_nanos, d.record_json
FROM dispatch_reservations_v8 AS d
JOIN continuity_windows AS w ON w.window_id = d.window_id;

INSERT INTO dispatch_reservation_projection_bindings
SELECT * FROM dispatch_reservation_projection_bindings_v8;

DROP TABLE dispatch_reservation_projection_bindings_v8;
DROP TABLE dispatch_reservations_v8;

ALTER TABLE continuity_projection_bindings RENAME TO continuity_projection_bindings_v8;

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
  result_sequence INTEGER NOT NULL CHECK (result_sequence > 1),
  snapshot_commitment TEXT NOT NULL CHECK (length(snapshot_commitment) BETWEEN 1 AND 256),
  target_kind TEXT NOT NULL CHECK (target_kind IN ('yield','wait','directive_attempt','attempt_outcome','ambiguity_recovery','security_rejection')),
  target_id TEXT NOT NULL CHECK (length(target_id) BETWEEN 1 AND 128),
  target_revision INTEGER NOT NULL CHECK (target_revision > 0),
  binding_json TEXT NOT NULL CHECK (length(binding_json) BETWEEN 2 AND 16384),
  CHECK (result_sequence = expected_sequence + 1),
  UNIQUE (operation_id, operation_kind),
  UNIQUE (result_event_id, run_id, result_sequence),
  UNIQUE (run_id, result_sequence),
  FOREIGN KEY (operation_id, operation_kind)
    REFERENCES continuity_operations(operation_id, operation_kind) ON DELETE RESTRICT,
  FOREIGN KEY (expected_event_id, run_id, expected_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT,
  FOREIGN KEY (result_event_id, run_id, result_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT,
  FOREIGN KEY (window_id, workflow_id, run_id)
    REFERENCES continuity_windows(window_id, workflow_id, run_id) ON DELETE RESTRICT
);

INSERT INTO continuity_projection_bindings
SELECT * FROM continuity_projection_bindings_v8;

DROP TABLE continuity_projection_bindings_v8;

CREATE INDEX continuity_projection_bindings_run
ON continuity_projection_bindings(run_id, result_sequence);
