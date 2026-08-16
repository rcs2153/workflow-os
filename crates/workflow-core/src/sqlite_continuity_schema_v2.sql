CREATE TABLE continuity_trusted_time (
  singleton_id INTEGER PRIMARY KEY CHECK (singleton_id = 1),
  source_kind TEXT NOT NULL CHECK (source_kind = 'core_injected_clock_v1'),
  provenance_commitment TEXT NOT NULL CHECK (length(provenance_commitment) BETWEEN 1 AND 256),
  epoch_id TEXT NOT NULL CHECK (length(epoch_id) BETWEEN 1 AND 128),
  observed_seconds INTEGER,
  observed_nanos INTEGER CHECK (observed_nanos BETWEEN 0 AND 999999999),
  posture TEXT NOT NULL CHECK (posture IN ('unobserved','healthy','quarantined')),
  eligibility TEXT NOT NULL CHECK (eligibility IN ('live_state_eligible','restore_unverified','quarantined')),
  revision INTEGER NOT NULL CHECK (revision > 0),
  CHECK ((posture = 'unobserved' AND observed_seconds IS NULL AND observed_nanos IS NULL)
      OR (posture <> 'unobserved' AND observed_seconds IS NOT NULL AND observed_nanos IS NOT NULL)),
  CHECK ((posture = 'quarantined' AND eligibility = 'quarantined')
      OR (posture <> 'quarantined' AND eligibility <> 'quarantined'))
);

CREATE TABLE continuity_windows (
  window_id TEXT PRIMARY KEY CHECK (length(window_id) BETWEEN 1 AND 128),
  workflow_id TEXT NOT NULL CHECK (length(workflow_id) BETWEEN 1 AND 128),
  run_id TEXT NOT NULL CHECK (length(run_id) BETWEEN 1 AND 128),
  step_id TEXT NOT NULL CHECK (length(step_id) BETWEEN 1 AND 128),
  window_binding_commitment TEXT NOT NULL CHECK (length(window_binding_commitment) BETWEEN 1 AND 256),
  subject_actor_id TEXT NOT NULL CHECK (length(subject_actor_id) BETWEEN 1 AND 128),
  immutable_bundle_commitment TEXT NOT NULL CHECK (length(immutable_bundle_commitment) BETWEEN 1 AND 256),
  governance_commitment TEXT NOT NULL CHECK (length(governance_commitment) BETWEEN 1 AND 256),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  state TEXT NOT NULL CHECK (state IN ('assessment_required','executing','yielded','closed','recovery_required','expired','revoked','superseded')),
  maximum_attempts INTEGER NOT NULL CHECK (maximum_attempts > 0),
  next_attempt_number INTEGER NOT NULL CHECK (next_attempt_number > 0),
  expires_seconds INTEGER NOT NULL,
  expires_nanos INTEGER NOT NULL CHECK (expires_nanos BETWEEN 0 AND 999999999),
  watermark_seconds INTEGER NOT NULL,
  watermark_nanos INTEGER NOT NULL CHECK (watermark_nanos BETWEEN 0 AND 999999999),
  trusted_time_epoch_id TEXT NOT NULL CHECK (length(trusted_time_epoch_id) BETWEEN 1 AND 128),
  revision INTEGER NOT NULL CHECK (revision > 0),
  active_yield_generation_id TEXT,
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (window_id, window_binding_commitment),
  FOREIGN KEY (active_yield_generation_id, window_id)
    REFERENCES continuity_yields(generation_id, window_id)
    ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED
);

CREATE UNIQUE INDEX continuity_one_active_window
ON continuity_windows(workflow_id, run_id, step_id, window_binding_commitment)
WHERE state IN ('executing','yielded','recovery_required');

CREATE TABLE continuity_attempts (
  attempt_id TEXT PRIMARY KEY CHECK (length(attempt_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  attempt_number INTEGER NOT NULL CHECK (attempt_number > 0),
  subject_actor_id TEXT NOT NULL CHECK (length(subject_actor_id) BETWEEN 1 AND 128),
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  consume_operation_id TEXT NOT NULL UNIQUE CHECK (length(consume_operation_id) BETWEEN 1 AND 128),
  consume_operation_kind TEXT NOT NULL DEFAULT 'consume_directive' CHECK (consume_operation_kind = 'consume_directive'),
  consume_operation_disposition TEXT NOT NULL DEFAULT 'committed_success' CHECK (consume_operation_disposition = 'committed_success'),
  state TEXT NOT NULL CHECK (state IN ('started','yielded','succeeded','retryable_failure','terminal_failure','ambiguous_may_have_started')),
  revision INTEGER NOT NULL CHECK (revision > 0),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (window_id, attempt_number),
  UNIQUE (attempt_id, window_id),
  UNIQUE (attempt_id, window_id, consume_operation_id),
  FOREIGN KEY (consume_operation_id, consume_operation_kind, consume_operation_disposition)
    REFERENCES continuity_operations(operation_id, operation_kind, disposition)
    ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE continuity_yields (
  generation_id TEXT PRIMARY KEY CHECK (length(generation_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  attempt_id TEXT NOT NULL UNIQUE,
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  reason TEXT NOT NULL CHECK (reason IN ('turn_boundary','context_budget','host_preemption','voluntary_checkpoint','transient_executor_failure')),
  registered_seconds INTEGER NOT NULL,
  registered_nanos INTEGER NOT NULL CHECK (registered_nanos BETWEEN 0 AND 999999999),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (generation_id, window_id),
  FOREIGN KEY (attempt_id, window_id)
    REFERENCES continuity_attempts(attempt_id, window_id) ON DELETE RESTRICT
);

CREATE TABLE continuity_waits (
  condition_id TEXT NOT NULL CHECK (length(condition_id) BETWEEN 1 AND 128),
  condition_version INTEGER NOT NULL CHECK (condition_version > 0),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  generation_id TEXT NOT NULL,
  wake_trigger TEXT NOT NULL CHECK (wake_trigger IN ('approval_decision_recorded','evidence_accepted','check_accepted','external_event_recorded','capability_availability_changed','deadline_reached','authority_source_changed','conflict_resolved')),
  state TEXT NOT NULL CHECK (state IN ('unsatisfied','satisfied','expired','superseded','canceled')),
  source_commitment TEXT CHECK (source_commitment IS NULL OR length(source_commitment) BETWEEN 1 AND 256),
  source_revision INTEGER CHECK (source_revision IS NULL OR source_revision > 0),
  revision INTEGER NOT NULL CHECK (revision > 0),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  PRIMARY KEY (condition_id, condition_version),
  UNIQUE (condition_id),
  UNIQUE (condition_id, condition_version, window_id),
  UNIQUE (condition_id, condition_version, window_id, generation_id),
  FOREIGN KEY (generation_id, window_id)
    REFERENCES continuity_yields(generation_id, window_id) ON DELETE RESTRICT
);

CREATE TABLE continuity_directives (
  directive_id TEXT PRIMARY KEY CHECK (length(directive_id) BETWEEN 1 AND 128),
  window_id TEXT NOT NULL REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  generation_id TEXT NOT NULL,
  cursor_sequence INTEGER NOT NULL CHECK (cursor_sequence > 0),
  cursor_event_id TEXT NOT NULL CHECK (length(cursor_event_id) BETWEEN 1 AND 128),
  authority_commitment TEXT NOT NULL CHECK (length(authority_commitment) BETWEEN 1 AND 256),
  state TEXT NOT NULL CHECK (state IN ('available','consumed','invalidated','expired')),
  revision INTEGER NOT NULL CHECK (revision > 0),
  record_json TEXT NOT NULL CHECK (length(record_json) BETWEEN 2 AND 16384),
  UNIQUE (directive_id, window_id, generation_id),
  FOREIGN KEY (generation_id, window_id)
    REFERENCES continuity_yields(generation_id, window_id) ON DELETE RESTRICT
);

CREATE TABLE continuity_operations (
  operation_id TEXT PRIMARY KEY CHECK (length(operation_id) BETWEEN 1 AND 128),
  receipt_id TEXT NOT NULL UNIQUE CHECK (length(receipt_id) BETWEEN 1 AND 128),
  operation_kind TEXT NOT NULL CHECK (operation_kind IN ('register_yield','transition_wait','consume_directive','record_attempt_outcome','recover_ambiguous_attempt')),
  request_commitment TEXT NOT NULL CHECK (length(request_commitment) BETWEEN 1 AND 256),
  request_json TEXT NOT NULL CHECK (length(request_json) BETWEEN 2 AND 16384),
  operation_commitment TEXT NOT NULL CHECK (length(operation_commitment) BETWEEN 1 AND 256),
  disposition TEXT NOT NULL CHECK (disposition IN ('committed_success','committed_security_rejection')),
  request_window_id TEXT NOT NULL CHECK (length(request_window_id) BETWEEN 1 AND 128),
  request_yield_generation_id TEXT CHECK (request_yield_generation_id IS NULL OR length(request_yield_generation_id) BETWEEN 1 AND 128),
  request_wait_condition_id TEXT CHECK (request_wait_condition_id IS NULL OR length(request_wait_condition_id) BETWEEN 1 AND 128),
  request_wait_condition_version INTEGER CHECK (request_wait_condition_version IS NULL OR request_wait_condition_version > 0),
  request_attempt_id TEXT CHECK (request_attempt_id IS NULL OR length(request_attempt_id) BETWEEN 1 AND 128),
  success_yield_generation_id TEXT CHECK (success_yield_generation_id IS NULL OR length(success_yield_generation_id) BETWEEN 1 AND 128),
  success_wait_condition_id TEXT CHECK (success_wait_condition_id IS NULL OR length(success_wait_condition_id) BETWEEN 1 AND 128),
  success_wait_condition_version INTEGER CHECK (success_wait_condition_version IS NULL OR success_wait_condition_version > 0),
  success_attempt_id TEXT CHECK (success_attempt_id IS NULL OR length(success_attempt_id) BETWEEN 1 AND 128),
  success_consume_operation_id TEXT CHECK (success_consume_operation_id IS NULL OR length(success_consume_operation_id) BETWEEN 1 AND 128),
  result_commitment TEXT CHECK (result_commitment IS NULL OR length(result_commitment) BETWEEN 1 AND 256),
  rejection_commitment TEXT CHECK (rejection_commitment IS NULL OR length(rejection_commitment) BETWEEN 1 AND 256),
  rejection_kind TEXT CHECK ((disposition = 'committed_success' AND rejection_kind IS NULL) OR
                              (disposition = 'committed_security_rejection' AND rejection_kind IN ('time_regressed','time_untrusted','time_epoch_mismatch','time_expired'))),
  trusted_time_source_kind TEXT NOT NULL CHECK (trusted_time_source_kind = 'core_injected_clock_v1'),
  trusted_time_provenance_commitment TEXT NOT NULL CHECK (length(trusted_time_provenance_commitment) BETWEEN 1 AND 256),
  trusted_time_epoch_id TEXT NOT NULL CHECK (length(trusted_time_epoch_id) BETWEEN 1 AND 128),
  observed_seconds INTEGER NOT NULL,
  observed_nanos INTEGER NOT NULL CHECK (observed_nanos BETWEEN 0 AND 999999999),
  trusted_time_commitment TEXT NOT NULL CHECK (length(trusted_time_commitment) BETWEEN 1 AND 256),
  result_json TEXT CHECK (result_json IS NULL OR length(result_json) BETWEEN 2 AND 16384),
  rejection_json TEXT CHECK (rejection_json IS NULL OR length(rejection_json) BETWEEN 2 AND 16384),
  committed_seconds INTEGER NOT NULL,
  committed_nanos INTEGER NOT NULL CHECK (committed_nanos BETWEEN 0 AND 999999999),
  UNIQUE (operation_id, operation_kind),
  UNIQUE (operation_id, operation_kind, disposition),
  FOREIGN KEY (request_window_id)
    REFERENCES continuity_windows(window_id) ON DELETE RESTRICT,
  FOREIGN KEY (success_yield_generation_id, request_window_id)
    REFERENCES continuity_yields(generation_id, window_id) ON DELETE RESTRICT,
  FOREIGN KEY (success_wait_condition_id, success_wait_condition_version, request_window_id)
    REFERENCES continuity_waits(condition_id, condition_version, window_id) ON DELETE RESTRICT,
  FOREIGN KEY (success_attempt_id, request_window_id)
    REFERENCES continuity_attempts(attempt_id, window_id) ON DELETE RESTRICT,
  FOREIGN KEY (success_attempt_id, request_window_id, success_consume_operation_id)
    REFERENCES continuity_attempts(attempt_id, window_id, consume_operation_id)
    ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED,
  CHECK (committed_seconds = observed_seconds AND committed_nanos = observed_nanos),
  CHECK ((operation_kind = 'register_yield'
          AND request_yield_generation_id IS NOT NULL
          AND request_wait_condition_id IS NULL
          AND request_wait_condition_version IS NULL
          AND request_attempt_id IS NULL)
      OR (operation_kind = 'transition_wait'
          AND request_yield_generation_id IS NULL
          AND request_wait_condition_id IS NOT NULL
          AND request_wait_condition_version IS NOT NULL
          AND request_attempt_id IS NULL)
      OR (operation_kind IN ('consume_directive','record_attempt_outcome','recover_ambiguous_attempt')
          AND request_yield_generation_id IS NULL
          AND request_wait_condition_id IS NULL
          AND request_wait_condition_version IS NULL
          AND request_attempt_id IS NOT NULL)),
  CHECK ((disposition = 'committed_success'
          AND result_commitment IS NOT NULL
          AND result_json IS NOT NULL
          AND rejection_kind IS NULL
          AND rejection_commitment IS NULL
          AND rejection_json IS NULL
          AND ((operation_kind = 'register_yield'
                AND success_yield_generation_id IS NOT NULL
                AND success_yield_generation_id = request_yield_generation_id
                AND success_wait_condition_id IS NULL
                AND success_wait_condition_version IS NULL
                AND success_attempt_id IS NULL
                AND success_consume_operation_id IS NULL)
            OR (operation_kind = 'transition_wait'
                AND success_yield_generation_id IS NULL
                AND success_wait_condition_id IS NOT NULL
                AND success_wait_condition_version IS NOT NULL
                AND success_wait_condition_id = request_wait_condition_id
                AND success_wait_condition_version = request_wait_condition_version
                AND success_attempt_id IS NULL
                AND success_consume_operation_id IS NULL)
            OR (operation_kind = 'consume_directive'
                AND success_yield_generation_id IS NULL
                AND success_wait_condition_id IS NULL
                AND success_wait_condition_version IS NULL
                AND success_attempt_id IS NOT NULL
                AND success_attempt_id = request_attempt_id
                AND success_consume_operation_id IS NOT NULL
                AND success_consume_operation_id = operation_id)
            OR (operation_kind IN ('record_attempt_outcome','recover_ambiguous_attempt')
                AND success_yield_generation_id IS NULL
                AND success_wait_condition_id IS NULL
                AND success_wait_condition_version IS NULL
                AND success_attempt_id IS NOT NULL
                AND success_attempt_id = request_attempt_id
                AND success_consume_operation_id IS NULL)))
      OR (disposition = 'committed_security_rejection'
          AND success_yield_generation_id IS NULL
          AND success_wait_condition_id IS NULL
          AND success_wait_condition_version IS NULL
          AND success_attempt_id IS NULL
          AND success_consume_operation_id IS NULL
          AND result_commitment IS NULL
          AND result_json IS NULL
          AND rejection_kind IS NOT NULL
          AND rejection_commitment IS NOT NULL
          AND rejection_json IS NOT NULL))
);
