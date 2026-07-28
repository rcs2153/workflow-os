# Authoritative WorkReport Artifact Persistence Review

## 1. Executive Verdict

**Needs blocker fixes.**

The Core composition, exact-retry boundary, create-only persistence, CLI
inspection surface, and ordinary-run isolation are well designed and broadly
match the approved phase. Full validation is green.

The phase cannot be accepted yet because its authoritative approval
presentation still declares report artifacts and persistence outside the
approved scope. Granting that approval can now persist the authoritative
terminal `WorkReport`. The durable presentation proof would therefore attest
that the operator reviewed a scope statement contradicted by the approved
runtime behavior.

## 2. Scope Verification

The implementation stayed within the approved local authoritative artifact
lane.

It added:

- explicit Core composition over the existing `WorkReportArtifactStore`;
- persistence after authoritative terminal report generation;
- exact terminal-run reassessment and artifact reconciliation;
- bounded CLI and JSON artifact posture;
- metadata-only artifact inspection; and
- focused Core, executor, CLI, retry, conflict, and non-regression tests.

It did not add:

- artifacts to ordinary undeclared execution;
- provider or sandbox integration;
- new provider mutations or SideEffect families;
- report export or body rendering;
- hosted or shared storage;
- schema or example changes;
- post-terminal workflow events;
- snapshot mutation; or
- release posture changes.

## 3. Core Composition Assessment

`persist_authoritative_governance_report_artifact(...)` is an additive,
explicit boundary. It accepts an existing run and validated report, constructs
the existing artifact record, verifies run identity, derives workflow-authored
artifact requirements, persists required approval proof-marker projections,
evaluates the existing SideEffect, approval-linkage, high-assurance, and
proof-marker gates, and performs one create-only store write.

The helper does not execute or resume workflows, call providers, infer missing
evidence, append events, or mutate run state. Its result separates workflow
truth from artifact-obligation posture.

## 4. Deterministic Retry Assessment

The new `ExistingTerminal` route correctly avoids treating a terminal retry as
fresh execution. It:

1. rehydrates the durable run;
2. rejects non-terminal existing runs;
3. requires the immutable bundle binding;
4. validates the explicit request against that bundle;
5. reruns the closed local check;
6. recomputes the source-bound governance assessment;
7. requires exact equality with the durable binding; and
8. regenerates and reconciles the same report artifact.

Visible disclosure dependencies are rejected on this route, so retry cannot
redeliver disclosure. Tests prove the event history is unchanged across both
quiet completion and approval-resume retry.

Exactly equal duplicates return `AlreadyPersisted`. Conflicting content under
the same run/report identity fails with the stable
`work_report_artifact.authoritative.duplicate_conflict` code. A concurrent
equal-write test proves one create and one exact reconciliation.

## 5. Artifact Gate Assessment

The composition reuses the strict existing artifact gates:

- artifact/run identity;
- cited SideEffect existence;
- approval linkage for approval-requiring SideEffects;
- matching approved or denied decisions;
- workflow-authored high-assurance disclosure; and
- workflow-authored approval proof-marker projection.

Quiet execution does not bypass these gates. Artifact failure is returned as a
separate non-success operation after the terminal workflow state already
exists; it does not rewrite workflow status or append compensating events.

## 6. Approval Presentation Blocker

The approval presentation is now materially false.

`persist_authoritative_governance_approval_presentation(...)` stores a strict
non-goal that includes:

```text
No ... report artifacts ... .
```

The rendered handoff separately says:

```text
approval_does_not_allow: ... artifacts, persistence ... .
```

After that exact approval is granted, the authoritative approval path calls
the new artifact composition and requires a persisted or exactly reconciled
artifact before returning success.

This is not merely stale help text. Workflow OS persists a content hash and
proof marker for the approval presentation. The proof would claim that an
operator approved a scope excluding the behavior that the approval enables.
That violates the project's approval-presentation integrity invariant.

The blocker fix must:

- state that approval permits the governed terminal WorkReport artifact
  obligation for the exact immutable run;
- keep report export, publication, arbitrary artifacts, and unrelated
  persistence outside scope;
- include the local artifact and proof-marker projection stores in expected
  touched surfaces;
- update both persisted presentation content and rendered handoff text; and
- add a regression proving the presented scope and actual artifact behavior
  agree.

## 7. CLI And Compatibility Assessment

Human quiet success is appropriately conditional on:

- `QuietProceed` or exact `ExistingTerminal`;
- terminal `Completed` status;
- generated report posture; and
- persisted or exactly reconciled artifact posture.

All failures, denials, visible routes, and approval routes retain detailed
output. `inspect` exposes artifact metadata without rendering the report body.
Ordinary `run` behavior remains unchanged.

The implementation's JSON names differ from the approved plan. The plan names
the new fields `report_artifact_posture` and `report_artifact_error_code`; the
implementation emits `artifact_posture` and `artifact_error_code`. Because
these are new preview fields, the blocker fix should align them with the
documented contract before publication and add assertions for the exact keys.

## 8. Privacy And Redaction Assessment

The new Core `Debug` implementations expose posture and counts only. CLI output
contains stable IDs, statuses, error codes, and inspect commands without report
body content.

The implementation does not copy or print:

- provider payloads;
- command or CI output;
- source or spec contents;
- parser payloads;
- environment values;
- credentials, authorization headers, tokens, or private keys;
- local-check stdout or stderr; or
- redaction reasons.

Artifact stores remain sensitive local preview storage. Encryption, retention,
shared access control, and regulated-data suitability are not claimed.

## 9. Test Quality Assessment

Existing tests provide strong coverage for:

- quiet terminal persistence;
- proof-enforced two-stage approval completion;
- no artifact before terminal approval completion;
- exact quiet and approval-resume retry;
- unchanged event history;
- concurrent equal duplicate reconciliation;
- conflicting duplicate rejection;
- bounded quiet, verbose, and JSON output;
- metadata-only inspection;
- ordinary-run non-persistence; and
- full existing executor, report, approval, SideEffect, adapter, and runtime
  behavior.

The blocker fix should add or strengthen:

- approval-presentation scope agreement with artifact persistence;
- exact documented JSON field names;
- visible terminal artifact persistence;
- denied terminal artifact persistence when a valid report exists;
- failed report generation writes no artifact; and
- corrupt artifact inspection fails closed without content leakage.

These are targeted gaps against the phase's own test plan. They do not require
new runtime concepts.

## 10. Documentation Assessment

The plan, implementation report, and roadmap accurately describe:

- local authoritative-only artifact persistence;
- deterministic terminal retry;
- strict artifact gates;
- ordinary-run isolation;
- metadata-only inspection;
- no provider or sandbox expansion; and
- no hosted storage, export, schemas, examples, or release changes.

The implementation report must not be marked accepted until the presentation
blocker is fixed and reviewed.

## 11. Blockers

1. **Approval presentation contradicts approved runtime behavior.**
   Persisted and rendered approval scope excludes artifacts and persistence,
   while approval completion now requires a local authoritative report
   artifact.
2. **New JSON field names do not match the approved plan.**
   Align the preview contract before publication rather than documenting a
   second vocabulary.

## 12. Non-Blocking Follow-Ups

- Add explicit visible and denied-route artifact assertions.
- Add corrupt-artifact inspect coverage at the CLI composition boundary.
- Continue treating report body rendering, export, hosted storage, retention,
  encryption, and shared access control as separate future phases.
- Do not begin OpenShell or broader provider mutation work from this phase.

## 13. Recommended Next Phase

Run a focused **authoritative artifact approval-presentation blocker fix**.

Keep the fix limited to truthful presentation scope, planned JSON field names,
and the focused missing regressions. Re-run the complete validation set and a
focused blocker-fix review before accepting the artifact phase.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785209236256795000-2`
- approval:
  `approval/run-1785209236256795000-2/review-scope-approved`
- presentation: `presentation/ed0e0f87bff86159`
- approval outcome: granted by delegated maintainer through
  presentation-proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- implementation validation:
  `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `npm run check`, `npm run check:integrations`
  under Node 20 and Node 24, `npm run check:docs`, and
  `git diff --check` passed
- skipped checks: opt-in live adapter and provider smoke tests remained
  skipped by their existing environment-gated contracts
- report posture: implementation report and maintainer review are repository
  documents; no separate runtime WorkReport artifact was generated for the
  review
- out-of-kernel work: code and diff inspection, review authoring, validation,
  git, and pull-request operations
- kernel boundary: the kernel governed scope and approval; it did not inspect
  code, edit files, execute validation, or perform git and pull-request actions

## 15. Fix-Forward Status

The focused blocker fix is implemented and awaiting its own maintainer review.
It updates persisted and rendered approval scope to authorize only the exact
governed terminal WorkReport artifact for the immutable run, retains explicit
prohibitions on arbitrary artifacts and broader persistence, aligns the new
JSON keys with the approved plan, and adds focused route and presentation
regressions.

This note does not erase the original blocker finding or change this review's
verdict. The separate
[blocker-fix review](AUTHORITATIVE_WORK_REPORT_ARTIFACT_PERSISTENCE_BLOCKER_FIX_REVIEW.md)
subsequently accepted the focused correction and the complete phase without
remaining blockers.
