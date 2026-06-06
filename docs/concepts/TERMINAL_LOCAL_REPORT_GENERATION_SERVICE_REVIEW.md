# Terminal Local Report Generation Service Review

Review date: 2026-06-05

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a narrow in-memory terminal local report generation helper that constructs validated `WorkReport` values from explicit inputs. It stays within the approved scope, supports current runtime terminal statuses, preserves workflow semantics, uses existing report/citation constructors, avoids runtime mutation and persistence, and keeps automatic runtime generation, CLI rendering, artifacts, schemas, examples, reasoning lineage, side-effect modeling, writes, approval evidence attachment, runtime config, and release posture changes out of scope.

No blockers were found before the next planning phase.

## 2. Scope Verification

The phase stayed within approved in-memory helper/service scope.

Verified in scope:

- `TerminalLocalWorkReportInput<'a>` accepts explicit report/run context and reference collections.
- `generate_terminal_local_work_report(...)` returns an in-memory `WorkReport`.
- The helper maps completed, failed, and canceled runtime terminal statuses.
- The helper populates all required v1 report sections.
- Citations are built from supplied stable IDs and references only.
- Optional unavailable references become explicit none/not-available section text.
- Existing constructors perform model validation and redaction checks.
- Tests and docs were updated.
- The end-of-phase report exists.

No accidental implementation was found for:

- automatic runtime report generation;
- runtime result exposure;
- report artifacts;
- persistence;
- CLI behavior;
- example updates;
- workflow spec schema changes;
- reasoning lineage implementation;
- side-effect boundary;
- writes;
- approval evidence attachment;
- release posture changes;
- runtime config creation.

## 3. Helper/Service API Assessment

The API is narrow and testable.

`TerminalLocalWorkReportInput<'a>` explicitly accepts:

- report identity;
- report contract identity/version;
- a borrowed `WorkflowRun`;
- generation timestamp and actor;
- optional correlation ID;
- sensitivity and redaction metadata;
- stable reference collections;
- bounded disclosure, limitation, risk, and handoff-note text.

The API avoids hidden global state and does not invent runtime config. It returns `Result<WorkReport, WorkflowOsError>` and lets existing constructor validation produce structured errors. The helper itself is exposed from `workflow-core`; that is acceptable for this phase, but compatibility posture should be reviewed before runtime result exposure or schema work.

## 4. Terminal Status Behavior

Verified:

- completed terminal input produces a valid `WorkReport`;
- failed terminal input produces a valid `WorkReport`;
- canceled terminal input produces a valid `WorkReport`;
- running/non-terminal input is rejected with `work_report_generation.status.not_terminal`;
- current runtime `Escalated` is not treated as terminal;
- `WorkReportStatus::Escalated` and `WorkReportStatus::Blocked` remain model vocabulary only;
- runtime terminal semantics were not changed.

This matches the runtime event/state-machine docs, which define `Completed`, `Failed`, and `Canceled` as terminal and reject post-terminal events.

## 5. Required Section Assessment

Generated reports include all required v1 sections:

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

Unavailable data becomes explicit section text, such as no stable approval references or no validation diagnostic references supplied. Sections do not silently disappear because the helper always passes a complete v1 section set through `WorkReport::new`. Domain-specific sections are not required.

## 6. Citation Construction Assessment

Citation construction is reference-first and bounded.

Verified:

- citations use stable IDs only;
- `EvidenceReferenceId` values are cited without recreating `EvidenceReference` values;
- `ValidationReferenceId` values are cited by stable reference where supplied;
- adapter telemetry is cited through `WorkReportStableReference`;
- policy and approval citations are produced only from supplied stable identifiers;
- audit and workflow events are cited by `EventId`;
- unavailable IDs become explicit none/not-available section text;
- citation failure cannot fabricate evidence because citation targets are created only from supplied typed identifiers and then validated.

Non-blocking follow-up: the plan discussed `missing=true` citations; the implementation uses section text for unavailable references. That is acceptable for this phase, but runtime result exposure planning should decide when to prefer explicit missing-citation records.

## 7. Workflow Semantics Assessment

The helper preserves workflow semantics.

Verified:

- report generation failure returns an error from the helper and does not change workflow pass/fail status;
- the helper borrows `WorkflowRun` and does not mutate it;
- `WorkflowRunSnapshot` is not mutated;
- event history is not mutated;
- no post-terminal events are appended;
- tests prove generation can occur without a `StateBackend` write;
- tests prove no filesystem artifacts are created;
- the helper emits no CLI output.

This is the correct boundary before runtime result exposure.

## 8. Privacy/Redaction Assessment

The helper preserves the WorkReport privacy posture.

Verified:

- it uses `WorkReport` constructors;
- it uses `WorkReportSection` constructors;
- it uses `WorkReportCitation` constructors;
- raw provider payloads are not copied;
- raw spec contents are not copied;
- raw command output is not copied;
- raw parser payloads are not copied;
- environment variable values are not copied;
- credentials, secrets, and tokens are not copied;
- secret-like handoff notes, limitations, risks, and incomplete-work disclosures are rejected through existing report note validation;
- Debug output is redaction-safe;
- serialization tests show forbidden raw payload markers are not present;
- errors use stable codes and do not leak secret-like values.

Non-blocking follow-up: before runtime result exposure, decide whether bounded free-text report inputs need a stronger provenance or payload-classification rule. The current implementation correctly relies on existing WorkReport validation, but it cannot prove arbitrary non-secret-looking text is not copied raw payload text.

## 9. Contract Enforcement Assessment

The helper accepts explicit contract identity and version. It does not create schema fields, runtime config, or workflow-declared contract enforcement.

Verified:

- full workflow-declared contract enforcement remains deferred;
- schema changes remain deferred;
- missing required sections are not possible through the helper path because all v1 sections are populated;
- model constructors remain the enforcement point;
- the helper does not instantiate or enforce a full `WorkReportContract` instance beyond identity/version alignment and `WorkReport` model validation.

Non-blocking follow-up: runtime result exposure planning should decide whether the helper should accept a `WorkReportContract` instance, a default contract helper, or continue accepting contract identity/version explicitly.

## 10. Test Quality Assessment

The tests are focused and meaningful.

Covered:

- completed terminal report;
- failed terminal report;
- canceled terminal report;
- non-terminal rejection;
- all required sections;
- constructor-gated report validation;
- `EvidenceReferenceId` citations;
- validation diagnostic/reference citations;
- adapter telemetry references;
- missing unavailable stable references;
- side effects as none/skipped/unsupported;
- workflow status preservation;
- no post-terminal events;
- no workflow run, snapshot, or event-history mutation;
- no `StateBackend` write requirement;
- no filesystem artifacts;
- no raw provider/spec/command/parser payload copying;
- secret-like input handling;
- Debug non-leakage;
- serialization non-leakage;
- existing WorkReport, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter, runtime, CLI, and example tests.

No shallow or fake tests were found. Some tests assert absence of known forbidden markers rather than proving all possible raw payloads are impossible; that is acceptable for this phase because the helper accepts bounded operator-supplied text by design.

## 11. Documentation Review

Docs now state:

- in-memory terminal local report generation helper/service is implemented;
- automatic runtime report generation is not implemented;
- runtime result exposure is not implemented;
- report artifacts are not implemented;
- persistence is not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- workflow spec schema changes are not implemented;
- reasoning lineage is not implemented;
- side-effect boundary is not implemented;
- writes remain unsupported.

The end-of-phase report also documents remaining limitations and recommends review before runtime exposure.

## 12. Blockers

No blockers found.

## 13. Non-Blocking Follow-Ups

- Decide whether unavailable references should be represented as section text, `missing=true` citations, or both before runtime result exposure.
- Decide whether the helper should remain a public exported core API or become internal/experimental before broader compatibility expectations attach.
- Decide whether future runtime exposure should pass a full `WorkReportContract` instance instead of only contract identity/version.
- Consider stricter provenance or source typing for bounded handoff/limitation/risk text before reports are exposed to runtime callers.
- Add direct tests for audit event and workflow event citations in generated report sections if those become important for runtime exposure.

## 14. Recommended Next Phase

Recommended next phase: runtime result exposure planning.

The helper is safe and useful as an in-memory constructor, but it is not yet wired into runtime results. The next planning phase should decide if and how generated reports are returned from runtime APIs while preserving current workflow semantics, avoiding post-terminal events, avoiding persistence/artifacts/CLI exposure, and keeping report generation failure behavior explicit.

## Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
