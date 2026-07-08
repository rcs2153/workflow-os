# Workflow Catalog Store Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The workflow catalog store helper stayed within the approved local persistence
boundary. It provides a small file-backed helper for validated catalog,
stewardship, and archive metadata records without introducing command
integration, runtime workflow registration, provider behavior, schemas, hosted
behavior, or write-capable adapters.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented scope:

- local `LocalWorkflowCatalogStore` helper;
- explicit caller-supplied catalog root;
- write/read/list support for workflow catalog records;
- write/read/list support for workflow stewardship records;
- write/read/list support for workflow archive records;
- bounded health summary;
- deterministic listing;
- duplicate-write rejection;
- safe encoded file names;
- validation and non-leaking error behavior;
- focused tests;
- documentation and phase report.

No accidental scope expansion was found for:

- command integration;
- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- automatic archive cleanup;
- persisted approval consumption;
- promotion command catalog writes;
- archive command catalog writes;
- catalog conflict detection;
- draft deletion;
- workflow schema changes;
- examples;
- provider calls;
- command execution or local check execution;
- hosted or distributed behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately narrow and testable.

`LocalWorkflowCatalogStore::new(root)` accepts an explicit root and does not read
hidden global state. The implemented methods map directly to the model records:

- `write_catalog_record_if_absent`
- `read_catalog_record`
- `list_catalog_records`
- `write_stewardship_record_if_absent`
- `read_stewardship_record`
- `list_stewardship_records_for_workflow`
- `write_archive_record_if_absent`
- `read_archive_record`
- `list_archive_records`
- `health_check`

The helper does not synthesize records from loader state, does not mutate
workflow files, and does not register workflows with the executor. That is the
right boundary for this phase.

## 4. Storage Layout Assessment

The storage layout matches the planned local catalog boundary:

```text
.workflow-os/catalog/
  workflows/
    <encoded-catalog-record-id>.json
  stewardship/
    <encoded-stewardship-decision-id>.json
  archives/
    <encoded-archive-record-id>.json
```

Record ids are encoded as hex file names, which avoids slash/path traversal
issues while keeping the canonical id inside the serialized record. Reads and
lists fail closed when the stored record identity does not match the file
address.

`index.json` remains deferred. That is acceptable because deterministic listing
is derived from record files and command-level indexing/conflict behavior was
not in scope.

## 5. Validation And Atomicity Assessment

Validation behavior is deterministic and conservative.

The catalog write path validates catalog records before writing. Read paths
deserialize through the validated model boundary and reject corrupt, invalid, or
identity-mismatched records. Duplicate writes are rejected rather than
overwriting existing records.

The write path creates parent directories, writes pretty JSON to a temporary
file, syncs the file, and publishes with a create-new hard-link operation. That
is a reasonable local atomicity boundary for this helper phase. Parent directory
fsync and richer crash-recovery behavior can remain future hardening.

Stable store errors are used for missing records, read failures, invalid
records, duplicate writes, identity mismatch, serialization failures, sync
failures, and publish failures.

## 6. Privacy And Redaction Assessment

The helper preserves the privacy boundary.

The store does not persist:

- raw workflow YAML;
- draft YAML;
- source file contents;
- package script bodies beyond model fields;
- dependency values or lockfile contents;
- CI logs;
- command output;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- existing agent instruction bodies.

Errors are stable and non-leaking. Tests cover missing record errors, corrupt
JSON, invalid serialized records, and identity mismatches without exposing
record ids, private paths, raw payload snippets, or secret-like values.

`LocalWorkflowCatalogStore` implements redaction-safe `Debug` output by
redacting the root path.

## 7. Test Quality Assessment

The tests are focused and appropriate for the phase.

Covered behavior includes:

- writing and reading catalog, stewardship, and archive records;
- duplicate catalog write rejection;
- safe encoded file names for ids containing `/`;
- deterministic catalog listing;
- stewardship listing filtered by workflow id;
- stable non-leaking missing-record errors;
- invalid serialized record failure without payload leakage;
- corrupt JSON failure without content leakage;
- identity mismatch failure between file name and record id;
- debug output redacting the catalog root path;
- existing workspace tests through full validation.

Non-blocking test follow-ups:

- add a deterministic ordering test for multiple archive records;
- add a deterministic ordering test for multiple stewardship records after
  filtering;
- add a health-check failure-path test for invalid records in stewardship or
  archive directories;
- add a focused test for duplicate stewardship and archive writes;
- consider a small temp-file cleanup/publish-failure test if it can be kept
  deterministic across local filesystems.

These are hardening items, not blockers.

## 8. Documentation Review

Documentation is honest about what exists and what remains future work.

The phase report states that the workflow catalog store helper is implemented,
local, explicit, and file-backed. It also states that command integration,
runtime workflow registration, automatic workflow generation, automatic
promotion, archive cleanup, persisted approval consumption, catalog conflict
detection, schemas, examples, providers, hosted behavior, write-capable
adapters, and release posture changes are not implemented.

The roadmap positions the store as a local foundation for future catalog
indexing, stewardship, promotion, archive, and conflict work without claiming a
hosted catalog or community workflow registry.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add deterministic archive and stewardship multi-record ordering tests.
- Add health-check failure-path coverage for invalid non-catalog records.
- Add duplicate-write tests for stewardship and archive records.
- Consider parent-directory sync or documented crash-recovery hardening before
  using the helper for higher-value promotion/archive workflows.
- Plan an in-memory catalog indexing/conflict helper before command wiring.
- Keep command integration, runtime registration, schemas, examples, hosted
  catalog behavior, and provider/write behavior behind separately reviewed
  phases.

## 11. Recommended Next Phase

Recommended next phase: workflow catalog indexing and conflict helper planning.

The store is now sufficient as a validated local persistence substrate. The
next useful step is to plan how callers derive an in-memory catalog index,
detect conflicting active workflows, identify stale drafts, and surface
stewardship/archive state without wiring commands or runtime registration too
early.

## 12. Validation

Validation run for this review:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

Governed review run:

- workflow: `dg/review`
- run id: `run-1783490232640154000-2`
- approval id: `approval/run-1783490232640154000-2/review-scope-approved`
- approval outcome: granted
- terminal status: `Completed`
- events: `39`
- event summary:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6`
- out-of-kernel work disclosed: Codex reviewed the implementation, wrote this
  review artifact, ran validation commands, and will perform git/PR packaging
  outside the kernel.
