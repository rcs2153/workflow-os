# EvidenceReference MVP Implementation Plan

Status: Phase 1 implemented; later phases remain proposed.

This plan scopes the `EvidenceReference` MVP so implementation can happen in small, reviewable steps. Phase 1 now implements the core Rust type model, serialization/deserialization, redaction-safe Debug/Display behavior, bounded string/metadata behavior, scope-specific validation, and tests.

This plan still does not implement schemas, CLI behavior, runtime persistence, work reports, reasoning lineage, writes, generic runtime adapter execution, production evidence storage, or domain packs.

Phase 2 attachment planning is documented separately in [EvidenceReference Attachment Plan](evidence-reference-attachment-plan.md). Adapter telemetry evidence attachment is implemented for adapter invocation and runtime audit telemetry records. Persistence, CLI inspection, examples, work reports, reasoning lineage, writes, and domain packs remain deferred.

## Goals

- Add a core, domain-neutral `EvidenceReference` model.
- Keep evidence references reference-first, redacted, and safe by default.
- Let future approvals, validation results, adapter telemetry, audit events, and work reports cite evidence consistently.
- Avoid raw sensitive payload storage.
- Preserve current `0.2.0-preview.1` release posture and read-only/no-write boundaries.

## Non-Goals

- No work report implementation.
- No reasoning lineage implementation.
- No side-effect boundary implementation.
- No write-capable adapters.
- No new CLI command.
- No stable CLI JSON contract.
- No public schema changes unless separately approved.
- No provider fetch/replay.
- No DLP engine.
- No access-control system.
- No production evidence store.
- No domain packs.

## Phase 1: Core Type Model

Status: implemented.

Add the minimal core Rust type model.

Likely files:

- `crates/workflow-core/src/lib.rs`
- a new `crates/workflow-core/src/evidence.rs` or equivalent core module
- focused tests under `crates/workflow-core/tests/`
- docs updates under `docs/concepts/` and `docs/security/`

Scope:

- `EvidenceReference`
- `EvidenceReferenceId`
- `EvidenceKind`
- `EvidenceScope`
- `EvidenceSourceComponent`
- `EvidenceSensitivity`
- `EvidenceRetentionHint`
- redaction metadata integration or reuse
- optional run/step/skill/adapter/audit/event/approval/validation reference fields
- serialization/deserialization
- redaction-safe `Debug` and `Display`

Tests:

- serializes/deserializes the minimum model;
- rejects or redacts secret-like summary/metadata values where supported by existing helpers;
- Debug and Display do not leak sensitive fields;
- provider token, authorization header, private key, and environment value fixtures are not emitted;
- unknown sensitivity defaults conservatively;
- metadata is non-secret and bounded.

Acceptance criteria:

- Type model compiles and is documented.
- Tests prove safe serialization and display behavior.
- No persistence, CLI, schema, report, or runtime behavior is added in this phase.

## Phase 2: Attachment Points

Status: adapter telemetry attachment implemented; validation, approval, audit/policy, persistence, CLI, and example attachment remain proposed.

Define narrow attachment points without changing report behavior.

Detailed sequencing, validation-boundary rules, and target-specific acceptance criteria are documented in [EvidenceReference Attachment Plan](evidence-reference-attachment-plan.md).

Likely files:

- adapter telemetry types in `crates/workflow-core/src/`
- validation result or diagnostic types
- approval decision types
- audit projection types where appropriate
- related tests

Scope:

- allow adapter telemetry records to carry validated evidence references; implemented for adapter invocation and runtime audit telemetry records;
- allow validation results/diagnostics to be referenced;
- allow approval decisions to cite evidence references;
- allow audit events to cite evidence reference IDs where appropriate;
- reserve future attachment to terminal work report placeholders without implementing work reports.

Tests:

- adapter telemetry can cite evidence references without storing raw provider payloads;
- validation diagnostics can be referenced without copying command output;
- approval decisions can cite evidence reference IDs;
- audit projections remain low-level operational records and do not become evidence payload stores.

Acceptance criteria:

- Attachment points are explicit and optional.
- Existing behavior remains unchanged unless an evidence reference is supplied.
- No WorkReport implementation is added.

## Phase 3: Local Persistence

Status: proposed, not implemented.

Decide whether and how evidence references persist in the local backend.

Likely files if approved:

- local backend implementation under `crates/workflow-core/src/`
- runtime state backend docs
- corruption/doctor state tests if persistence is added

Decision points:

- persist evidence references as separate local artifacts or embed references in existing records;
- define storage path and file naming;
- define corruption behavior for missing, malformed, or dangling evidence reference files;
- define whether evidence references are event-log-derived, audit-derived, or separate local records;
- avoid raw payload storage.

Tests:

- healthy local evidence reference store reports healthy if implemented;
- malformed evidence reference is reported clearly;
- dangling links are reported clearly;
- persistence does not store raw CI logs, Jira bodies, GitHub large file content, tokens, or provider payloads.

Acceptance criteria:

- Persistence design is explicit before implementation.
- Corruption behavior is documented and tested if persistence is added.
- Local state remains inspectable and non-destructive by default.

## Phase 4: CLI Inspection

Status: proposed, not implemented.

Decide whether `workflow-os inspect` should show evidence summaries.

Likely files if approved:

- `crates/workflow-cli/src/main.rs`
- CLI docs under `docs/cli/`
- operations docs
- CLI tests

Scope:

- show concise redacted evidence summaries;
- preserve experimental CLI JSON posture unless separately versioned;
- avoid new CLI commands unless separately approved;
- keep display bounded and useful.

Tests:

- human-readable inspect output shows evidence ID, kind, title, sensitivity, and redaction summary;
- inspect output does not show secrets or raw provider payloads;
- JSON output remains experimental and documented;
- missing referenced evidence is shown as missing, not silently ignored.

Acceptance criteria:

- Operators can understand what evidence was cited without seeing sensitive payloads.
- CLI output remains conservative and bounded.

## Phase 5: Example Integration

Status: proposed, not implemented.

Add evidence references to existing examples after the core model and attachment points exist.

Likely files:

- `examples/vertical-slice-approval/`
- `examples/github-read-only-review-context/`
- `examples/jira-read-only-intake-quality/`
- `examples/ci-read-only-failure-summary/`
- example README files
- integration gate scripts/tests

Scope:

- vertical slice cites validation, approval, and local run evidence;
- GitHub example cites fixture-backed adapter response summaries;
- Jira example cites fixture-backed issue metadata/comment summary references;
- CI example cites workflow run/job/failure summary and redacted log excerpt references;
- no live credentials required for normal CI.

Tests:

- all examples validate;
- all fixture-backed examples run;
- evidence references are produced or attached where declared;
- evidence output is redacted;
- no token appears in output;
- no raw CI log appears in output;
- no raw Jira description/comment body appears in output;
- no raw large GitHub file content appears in output.

Acceptance criteria:

- Examples demonstrate evidence citation without overclaiming production behavior.
- Fixture-first and no-write posture remains intact.

## Phase 6: Review Gate

Status: proposed, not completed.

Run a maintainer review before WorkReportContract implementation.

Review must verify:

- model is domain-neutral;
- redaction and sensitivity behavior is safe;
- source-of-truth boundaries are documented;
- local persistence, if added, is diagnosable;
- CLI display, if added, is conservative;
- examples do not imply production evidence storage;
- WorkReportContract can cite evidence references without redesigning the model.

Acceptance criteria:

- Maintainers approve proceeding to WorkReportContract planning or implementation.
- Any remaining gaps are explicitly documented.

## Files Likely To Change

Future implementation may touch:

- `crates/workflow-core/src/lib.rs`
- `crates/workflow-core/src/evidence.rs`
- `crates/workflow-core/src/adapter_telemetry.rs` or equivalent adapter modules
- validation/diagnostic types
- approval/audit/event types
- local backend types if persistence is approved
- `crates/workflow-cli/src/main.rs` if inspect display is approved
- tests under `crates/workflow-core/tests/` and `crates/workflow-cli/tests/`
- `docs/concepts/evidence-reference.md`
- `docs/runtime/state-backends.md`
- `docs/runtime/event-model.md`
- `docs/operations/` docs if persistence/inspection is added
- read-only example READMEs

Exact files should be chosen during the implementation prompt after reading the current code.

## Required Tests

Minimum test set:

- evidence reference serialization/deserialization;
- redaction-safe Debug/Display;
- secret-like values do not appear in serialized display paths;
- sensitivity defaults conservatively;
- kind taxonomy round-trips;
- attachment to adapter telemetry;
- attachment to validation result or diagnostic reference;
- attachment to approval decision reference;
- audit event citation without raw payload storage;
- persistence health/corruption tests if persistence is implemented;
- inspect display redaction tests if CLI display is implemented;
- example tests if examples are updated.

## Required Docs

Future implementation should update:

- ADR 0009 status only if maintainers accept it;
- `docs/concepts/evidence-reference.md`;
- runtime/audit/adapter docs for attachment behavior;
- security redaction docs for evidence reference display/serialization behavior;
- operations docs if evidence references are persisted or inspected;
- example READMEs if examples emit evidence references.

## Security And Privacy Review Points

Before implementation is accepted, review:

- whether summaries can leak sensitive provider content;
- whether metadata is sufficiently bounded and non-secret;
- whether provider object references reveal private project names or personal data;
- whether evidence references can outlive a user's permission to view the source object;
- whether local persistence creates retention risk;
- whether debug/display/JSON output is safe;
- whether evidence references duplicate audit logs or provider payloads.

## Final Acceptance Criteria

The MVP is acceptable only when:

- `EvidenceReference` is implemented as a small core model;
- raw sensitive payloads are not stored by default;
- redaction-safe display behavior is tested;
- source-of-truth boundaries are documented;
- attachment points are explicit and optional;
- local persistence and CLI display are either implemented with tests or explicitly deferred;
- examples remain fixture-first and read-only;
- no writes, work reports, reasoning lineage, generic live adapter execution, domain packs, or release-posture changes are introduced.
