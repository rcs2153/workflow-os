ALTER TABLE continuity_waits ADD COLUMN dependency_kind TEXT
  CHECK (dependency_kind IS NULL OR dependency_kind = 'time_window');
ALTER TABLE continuity_waits ADD COLUMN dependency_commitment TEXT
  CHECK (dependency_commitment IS NULL OR length(dependency_commitment) BETWEEN 1 AND 256);
ALTER TABLE continuity_waits ADD COLUMN deadline_seconds INTEGER;
ALTER TABLE continuity_waits ADD COLUMN deadline_nanos INTEGER
  CHECK (deadline_nanos IS NULL OR deadline_nanos BETWEEN 0 AND 999999999);
ALTER TABLE continuity_waits ADD COLUMN required_time_source_kind TEXT
  CHECK (required_time_source_kind IS NULL OR required_time_source_kind = 'core_injected_clock_v1');
ALTER TABLE continuity_waits ADD COLUMN required_time_provenance_commitment TEXT
  CHECK (required_time_provenance_commitment IS NULL OR length(required_time_provenance_commitment) BETWEEN 1 AND 256);
ALTER TABLE continuity_waits ADD COLUMN required_time_epoch_id TEXT
  CHECK (required_time_epoch_id IS NULL OR length(required_time_epoch_id) BETWEEN 1 AND 128);
