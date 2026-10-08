CREATE TABLE immutable_run_bundles (
    run_id TEXT PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    root_hash TEXT NOT NULL,
    manifest_payload TEXT NOT NULL,
    definition_records_payload TEXT NOT NULL,
    local_check_records_payload TEXT NOT NULL
);
