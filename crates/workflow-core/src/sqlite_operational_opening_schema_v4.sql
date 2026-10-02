DROP TABLE continuity_attempts;

CREATE TABLE continuity_attempts (
  attempt_id TEXT PRIMARY KEY CHECK (length(attempt_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  attempt_number INTEGER NOT NULL CHECK (attempt_number > 0),
  subject_actor_id TEXT NOT NULL CHECK (length(subject_actor_id) BETWEEN 1 AND 128),
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  consume_operation_id TEXT NOT NULL UNIQUE CHECK (length(consume_operation_id) BETWEEN 1 AND 128),
  consume_operation_kind TEXT NOT NULL DEFAULT 'consume_directive' CHECK (consume_operation_kind IN ('consume_directive', 'operational_opening')),
  consume_operation_disposition TEXT NOT NULL DEFAULT 'committed_success' CHECK (consume_operation_disposition = 'committed_success'),
  state TEXT NOT NULL CHECK (state IN ('started','yielded','succeeded','retryable_failure','terminal_failure','ambiguous_may_have_started')),
  revision INTEGER NOT NULL CHECK (revision > 0),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (window_id, attempt_number),
  UNIQUE (attempt_id, window_id),
  UNIQUE (attempt_id, window_id, consume_operation_id)
);

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
  result_sequence INTEGER NOT NULL UNIQUE CHECK (result_sequence = expected_sequence + 1),
  attempt_number INTEGER NOT NULL CHECK (attempt_number = 1),
  window_revision INTEGER NOT NULL CHECK (window_revision = 1),
  committed_seconds INTEGER NOT NULL,
  committed_nanos INTEGER NOT NULL CHECK (committed_nanos BETWEEN 0 AND 999999999),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (window_id, operation_binding_commitment),
  FOREIGN KEY (window_id, workflow_id, run_id)
    REFERENCES continuity_windows(window_id, workflow_id, run_id) ON DELETE RESTRICT,
  FOREIGN KEY (expected_event_id, run_id, expected_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT,
  FOREIGN KEY (result_event_id, run_id, result_sequence)
    REFERENCES events(event_id, run_id, sequence_number) ON DELETE RESTRICT
);

CREATE UNIQUE INDEX operational_opening_one_scope
ON operational_opening_operations(workflow_id, run_id, step_id, operation_binding_commitment);

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
