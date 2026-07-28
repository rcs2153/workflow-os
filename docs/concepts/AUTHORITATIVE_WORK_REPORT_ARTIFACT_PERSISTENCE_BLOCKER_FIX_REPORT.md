# Authoritative WorkReport Artifact Persistence Blocker Fix Report

## 1. Executive Summary

The authoritative WorkReport artifact persistence approval-integrity blocker
is fixed.

The exact approval presentation now states that approval may resume only the
immutable run through the closed project-validation profile and, if terminal
report generation succeeds, persist that run's exact governed local
`WorkReport` artifact. Arbitrary artifacts, report export or publication,
hosted persistence, provider writes, and scope expansion remain prohibited.

The new JSON artifact fields also match the approved implementation plan.

## 2. Blockers Fixed

The maintainer review identified two blockers:

1. persisted and rendered approval scope excluded artifacts and persistence
   even though approval completion could persist an authoritative report
   artifact; and
2. JSON emitted `artifact_posture` and `artifact_error_code` instead of the
   planned `report_artifact_posture` and `report_artifact_error_code`.

Both are fixed before publication.

## 3. Implementation Approach

The existing authoritative approval presentation now:

- includes the exact governed terminal WorkReport artifact in approved scope;
- includes approval-presentation and proof-marker projection state plus the
  exact artifact in expected touched surfaces;
- distinguishes that artifact from arbitrary artifacts;
- preserves the prohibition on report export, publication, hosted
  persistence, provider writes, and broader runtime authority; and
- renders the same boundary in the human approval handoff.

No approval authority, runtime route, artifact gate, or store behavior changed.

The three authoritative JSON renderers now emit:

- `report_artifact_posture`; and
- `report_artifact_error_code`.

The unplanned shorter aliases are absent.

## 4. Validation Boundary

Approval still authorizes only:

- the exact waiting immutable run;
- fresh project validation and proof enforcement;
- its existing workflow execution; and
- its exact governed terminal WorkReport artifact when valid terminal report
  generation succeeds.

It does not authorize new commands, provider writes, arbitrary artifacts,
report export or publication, hosted persistence, schema changes, automatic
approval, or scope expansion.

## 5. Privacy And Redaction

The fix changes bounded fixed presentation text and JSON field names only.

It adds no report body, command output, provider payload, source content,
environment value, credential, token, local path, approval reason, or
redaction-reason output.

## 6. Test Coverage

Focused tests prove:

- persisted approval scope names the exact terminal artifact obligation;
- rendered `approval_allows` matches that scope;
- rendered `approval_does_not_allow` retains the narrower prohibitions;
- the stale contradictory handoff text is absent;
- visible terminal completion persists an artifact;
- denied terminal completion persists its generated artifact before the
  bounded denial result;
- JSON exposes the exact planned field names; and
- the unplanned JSON aliases are absent.

Existing quiet, approval-resume, retry, conflict, ordinary-run, report,
artifact, approval, SideEffect, executor, CLI, and integration coverage
remains applicable.

## 7. Validation

All required validation passed:

- focused authoritative CLI tests: 9 passed;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations` under Node 20;
- `npm run check:integrations` under Node 24;
- `npm run check:docs`; and
- `git diff --check`.

## 8. Remaining Limitations

- Report body inspection and export remain unimplemented.
- Artifact storage remains local preview storage without encryption,
  retention, or shared access control.
- Corrupt-artifact CLI inspection has store-level fail-closed coverage but
  remains a useful explicit CLI regression.
- Provider and sandbox integrations remain outside this phase.
- Ordinary undeclared runs remain artifact-free.

## 9. Recommended Next Phase

Run a focused blocker-fix maintainer review. If accepted, publish the complete
authoritative artifact persistence phase and return to the roadmap's runtime
authority and capability sequence.

## 10. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1785209563321222000-2`
- approval: `approval/run-1785209563321222000-2/fix-approved`
- presentation: `presentation/27ad82a4b3cdafa0`
- approval outcome: granted by delegated maintainer through
  presentation-proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- skipped checks: opt-in live provider and adapter smoke tests remained skipped
  by their existing environment-gated contracts
- report posture: this repository report records the phase; no separate
  runtime WorkReport artifact was generated
- out-of-kernel work: code and test edits, validation commands, documentation,
  git, and pull-request operations
- kernel boundary: the kernel governed scope and approval; it did not edit
  files, run validation, or perform git and pull-request actions
