# Terminal Local Report Generation Service Report

Report date: 2026-06-05

## 1. Executive Summary

The terminal local report generation service phase is implemented as an in-memory helper only.

The helper constructs a validated `WorkReport` from explicit terminal run/report inputs. It supports current runtime terminal statuses `completed`, `failed`, and `canceled`, cites stable IDs where supplied, populates all required v1 report sections, and uses existing `WorkReport` and `WorkReportCitation` constructors so validation and redaction boundaries remain active.

No automatic runtime report generation, persistence, CLI rendering, examples, schemas, report artifacts, reasoning lineage, side-effect boundary, writes, approval evidence attachment, or release posture changes were introduced.

## 2. Scope Completed

- Added `TerminalLocalWorkReportInput`.
- Added `generate_terminal_local_work_report(...)`.
- Derived report identity from an already-terminal borrowed `WorkflowRun`.
- Mapped runtime terminal statuses:
  - `WorkflowRunStatus::Completed` to `WorkReportStatus::Completed`;
  - `WorkflowRunStatus::Failed` to `WorkReportStatus::Failed`;
  - `WorkflowRunStatus::Canceled` to `WorkReportStatus::Canceled`.
- Rejected non-terminal runtime statuses, including current runtime `Escalated`.
- Populated all required v1 `WorkReportSectionKind` sections.
- Created citations for supplied stable IDs:
  - `EvidenceReferenceId`;
  - workflow event `EventId`;
  - audit event `EventId`;
  - adapter telemetry `WorkReportStableReference`;
  - `ValidationReferenceId`;
  - policy decision event `EventId`;
  - `ApprovalReferenceId`.
- Used explicit none/not-available section text where optional stable references were not supplied.
- Added tests for terminal status behavior, citation construction, section population, workflow non-mutation, no backend dependency, no filesystem artifacts, and redaction safety.

## 3. Scope Explicitly Not Completed

- Automatic runtime report generation is not implemented.
- Report artifacts are not implemented.
- Persistence is not implemented.
- CLI rendering is not implemented.
- Examples are not updated.
- Workflow spec schema changes are not implemented.
- Reasoning lineage is not implemented.
- Side-effect boundary modeling is not implemented.
- Write behavior is not implemented.
- Approval evidence attachment is not implemented.
- Runtime configuration is not added.
- Release posture is unchanged.

## 4. Helper/Service API Summary

The new helper is:

- `TerminalLocalWorkReportInput<'a>`;
- `generate_terminal_local_work_report(input) -> Result<WorkReport, WorkflowOsError>`.

The input borrows a `WorkflowRun` and accepts explicit report identity, contract identity/version, generation timestamp, generating actor, optional correlation ID, sensitivity, redaction metadata, stable citation references, and bounded disclosure/note strings.

The helper returns an in-memory `WorkReport`. It does not read hidden global state, mutate the borrowed run, append events, write files, persist records, or expose CLI output.

## 5. Terminal Status Behavior

Supported runtime statuses:

- completed;
- failed;
- canceled.

Current runtime `Escalated` is not treated as terminal by `WorkflowRunStatus::is_terminal()` and is rejected by the helper. `WorkReportStatus::Escalated` and `WorkReportStatus::Blocked` remain report model vocabulary only until runtime terminal semantics are separately designed.

## 6. Citation Construction Summary

The helper builds citations only from supplied stable identifiers and references. It does not fabricate IDs and does not create `EvidenceReference` values implicitly.

When stable references are unavailable, the helper uses explicit section text such as “No stable approval references were supplied” or “No validation diagnostic references were supplied.” Missing-citation modeling remains available in the core model, but this phase does not fabricate missing citation targets.

## 7. Section Population Summary

All v1 sections are present:

- work performed;
- evidence considered;
- decisions made;
- policy gates evaluated;
- approvals;
- validation and quality checks;
- side effects;
- incomplete or deferred work;
- known limitations;
- risks;
- operator handoff notes.

The side-effects section always states the current no-write posture as none/skipped/unsupported. Disclosure, limitation, risk, and handoff-note collections use supplied bounded text or explicit default “none supplied” text.

## 8. Workflow Semantics Summary

Report generation is independent from workflow execution semantics.

The helper:

- does not change terminal run status;
- does not append post-terminal events;
- does not mutate `WorkflowRun`, `WorkflowRunSnapshot`, event history, state backend, telemetry stores, or runtime state;
- returns structured construction errors without converting them into user project diagnostics;
- does not retroactively fail or alter a workflow result.

## 9. Redaction/Privacy Summary

The helper preserves the WorkReport privacy posture:

- uses existing `WorkReport`, `WorkReportSection`, `WorkReportCitation`, and note constructors;
- keeps summaries bounded;
- rejects secret-like supplied disclosure/note text through existing model validation;
- requires validated report redaction metadata;
- never copies raw provider payloads, raw spec contents, raw command output, raw parser payloads, environment values, tokens, credentials, authorization headers, or private keys;
- keeps Debug output redaction-safe through existing report Debug implementations.

## 10. Test Coverage Summary

Added tests cover:

- completed, failed, and canceled terminal runs producing valid in-memory reports;
- non-terminal runtime status rejection;
- all required v1 sections;
- `EvidenceReferenceId` citation without recreating evidence references;
- validation diagnostic citation by stable reference;
- adapter telemetry citation by stable reference;
- explicit not-available text when optional stable references are absent;
- side effects section as none/skipped/unsupported;
- workflow run status, snapshot, and event history non-mutation;
- use without `StateBackend` writes;
- no filesystem artifacts;
- no raw provider/spec/command/parser payload copying;
- secret-like disclosure/note rejection;
- Debug and serialization non-leakage.

Existing WorkReport, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter telemetry, runtime, CLI, and example tests still pass.

## 11. Commands Run And Results

- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo test --workspace` — passed.
- `npm run check:docs` — passed.

## 12. Remaining Known Limitations

- The helper is not automatically invoked by `LocalExecutor` or any runtime path.
- Generated reports are not returned from runtime result types.
- Reports are not persisted, written as artifacts, exported, or rendered by CLI.
- The helper accepts contract identity/version explicitly; full contract-instance enforcement is deferred.
- Escalated and blocked report statuses remain model vocabulary only for generation because current runtime terminal status support is completed/failed/canceled.
- Missing citation targets are represented through section text rather than fabricated citation targets.
- Approval/policy citations are supported only when stable IDs are supplied; approval evidence attachment remains unimplemented.

## 13. Recommended Next Phase

Recommended next phase: terminal local report generation service review.

The helper should be reviewed before any runtime result exposure, artifact planning, persistence, CLI rendering, examples, schema changes, reasoning lineage, side-effect modeling, or writes are considered.
