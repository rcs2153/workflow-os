# Workflow Catalog Store Helper Report

## 1. Executive Summary

This phase implements the first local workflow catalog persistence slice: a
small file-backed helper for validated workflow catalog, stewardship, and
archive metadata records.

The helper is model-backed and in-memory-callable. It writes only explicit
catalog files under a caller-supplied catalog root such as
`.workflow-os/catalog/`. It does not integrate authoring commands, register
workflows with the runtime, change workflow schemas, create examples, call
providers, execute commands, enable hosted behavior, add write-capable adapters,
or change release posture.

## 2. Scope Completed

- Added `LocalWorkflowCatalogStore`.
- Added `WorkflowCatalogStoreHealth`.
- Added file-backed write/read/list methods for `WorkflowCatalogRecord`.
- Added file-backed write/read/list methods for `WorkflowStewardshipRecord`.
- Added file-backed write/read/list methods for `WorkflowArchiveRecord`.
- Added deterministic record-id sorted listing.
- Added safe encoded file names derived from validated canonical ids.
- Added duplicate-write rejection.
- Added validated read/deserialization behavior.
- Added bounded store health summary.
- Exported the helper types from `workflow-core`.
- Added focused store tests.
- Updated roadmap and planning docs.

## 3. Scope Explicitly Not Completed

This phase did not implement:

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

## 4. Helper API Summary

`LocalWorkflowCatalogStore::new(root)` creates a store rooted at a caller-supplied
catalog directory.

Implemented operations:

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

The helper uses existing validated model types. It does not synthesize workflow
records from loader state and does not mutate workflow files.

## 5. Storage Layout

The helper supports the planned local layout:

```text
.workflow-os/catalog/
  workflows/
    <encoded-catalog-record-id>.json
  stewardship/
    <encoded-stewardship-decision-id>.json
  archives/
    <encoded-archive-record-id>.json
```

Record ids are encoded into safe hex file names. The canonical id remains inside
the serialized record. Reads and lists fail closed when the stored record id does
not match the encoded storage address.

`index.json` remains deferred. Deterministic listing is currently derived from
record files.

## 6. Validation And Atomicity Boundary

The store validates catalog records through the existing model validation before
write, and reads records through validated serde constructors.

Writes create parent directories, serialize pretty JSON, write to a temporary
file, sync that file, and publish with an atomic create-new link operation where
the local filesystem supports it. Duplicate records are rejected and do not
overwrite existing files.

Invalid JSON, invalid serialized records, missing records, duplicate writes, and
identity mismatches return stable `workflow_catalog_store.*` error codes.

## 7. Redaction And Privacy Summary

The store does not persist raw workflow YAML, draft YAML, source contents,
package scripts, dependency values, lockfile contents, CI logs, command output,
provider payloads, parser payloads, absolute private paths, environment
variables, credentials, authorization headers, private keys, token-like values,
or existing agent instruction bodies.

Store errors do not echo record ids, paths, raw serialized payloads, corrupt file
contents, or secret-like values. `LocalWorkflowCatalogStore` debug output redacts
the root path.

## 8. Test Coverage Summary

Added focused tests covering:

- writing and reading catalog, stewardship, and archive records;
- duplicate write rejection;
- safe encoded file names for ids containing `/`;
- deterministic catalog listing;
- stewardship listing filtered by workflow id;
- stable non-leaking missing-record errors;
- invalid serialized record failure without payload leakage;
- corrupt JSON failure without content leakage;
- identity mismatch failure between file name and record id;
- debug output redacting the catalog root path.

Focused command run:

- `cargo test -p workflow-core --test workflow_catalog_store`
  - Passed.

## 9. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase implementation ...`
  - Completed with run `run-1783488846450236000-2`.
  - Approval requested:
    `approval/run-1783488846450236000-2/implementation-approved`.
- `npm run dogfood:benchmark -- approve run-1783488846450236000-2 approval/run-1783488846450236000-2/implementation-approved --actor user/delegated-maintainer --reason approved-workflow-catalog-store-helper-implementation`
  - Granted; run completed.
- `cargo test -p workflow-core --test workflow_catalog_store`
  - Passed.

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `npm run dogfood:benchmark -- phase-close run-1783488846450236000-2 --phase implementation`
  - Passed.
  - Workflow: `dg/implement`.
  - Status: `Completed`.
  - Events: `39`.
  - Event summary:
    `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6`.

Out-of-kernel work disclosed: Codex edited Rust source, tests, docs, and
roadmap text; ran validation commands; and will perform git/PR packaging outside
the kernel. The kernel coordinated the phase boundary and approval checkpoint.

## 10. Remaining Known Limitations

- No authoring command writes catalog records.
- No persisted stewardship decision is consumed by promotion.
- No archive command writes archive metadata.
- No catalog conflict helper exists.
- No catalog index file exists.
- No schema exposure exists.
- No hosted or team catalog backend exists.
- Git commit policy for catalog files remains user/team-defined.

## 11. Recommended Next Phase

Recommended next phase: workflow catalog store helper review.

The store helper should be reviewed before adding in-memory indexing, conflict
detection, steward-review persistence, promotion integration, or archive
metadata command wiring.
