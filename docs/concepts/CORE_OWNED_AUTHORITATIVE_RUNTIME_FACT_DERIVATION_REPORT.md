# Core-Owned Authoritative Runtime-Fact Derivation Report

## 1. Executive Summary

The closed authoritative project-validation path now accepts a fact-free
request. Workflow Core constructs one neutral fact for one immutable workflow
step, binds current authority from the validated project activation, binds
evidence/check posture from the same-call local-check result, derives
side-effect posture from the immutable skill declaration, and selects the
proportional-governance route.

The CLI no longer constructs `StepGovernanceRuntimeFacts` or predicts whether
visible disclosure will be required.

## 2. Scope Completed

- Added a fact-free Core request for the closed authoritative profile.
- Added an immutable one-step workflow-shape preflight.
- Added Core-owned report and approval-report request paths.
- Reused existing immutable activation, same-call check, assessment, approval,
  presentation-proof, report, and disclosure receipt boundaries.
- Made visible delivery capability available to Core without invoking it on
  quiet, approval-required, or denied routes.
- Migrated the authoritative CLI run and aggregate-approval paths.
- Preserved existing explicit-fact APIs.

## 3. Scope Explicitly Not Completed

This phase did not add multi-step authoritative aggregation, scaffold default
activation, inferred repository commands, new local-check profiles, automatic
approval, providers, OpenShell, credential handling, SideEffect execution,
external writes, hosted behavior, schemas, SDK changes, examples, or release
posture changes.

## 4. Runtime Boundary

The closed request carries execution identity, immutable-bundle inputs,
selected step identity, fixed profile, project activation, and an optional
expected fingerprint. It cannot carry authority, evidence/check, or
side-effect facts.

Core rejects a workflow unless the immutable definition contains exactly one
step and that step is the selected step. Rejection occurs before a local-check
process, run event, or immutable bundle persistence.

## 5. Fact And Route Derivation

Core creates one neutral fact. The existing derivation then supplies:

- authority from immutable project activation;
- evidence/check posture from the accepted same-call check result;
- action class, sensitivity, policy minima, and side-effect posture from
  immutable definitions; and
- the final quiet, visible, approval-required, or denied route from the
  complete proportional-governance assessment.

An available visible-delivery handler is invoked only for visible proceed.

## 6. Approval And Report Behavior

Aggregate approval decisions repeat immutable request matching, one-step shape
validation, current-authority binding, and the same-call check before validating
presentation proof or mutating approval state.

Authored workflow-step approvals remain a separate existing gate. Aggregate
approval does not satisfy a later authored step approval.

## 7. Privacy And Failure Posture

New Debug implementations redact execution and selected-step identity. Shape,
fact, check, and delivery failures use stable bounded errors without workflow
payloads, paths, command output, credentials, policy bodies, or disclosure
content.

## 8. Test Coverage

Focused tests cover:

- one-step quiet execution;
- one-step visible execution and exact single delivery;
- missing visible-delivery capability failing closed before run events;
- supplied-but-unused visible capability on quiet and approval routes;
- approval reassessment using the same immutable shape and check;
- multi-step rejection before check or durable run state;
- failed same-call check rejection;
- CLI quiet, visible, approval-required, denied, retry, JSON, and missing-profile
  behavior; and
- compatibility of the existing explicit-fact suite.

## 9. Validation

Completed during implementation:

- `cargo check -p workflow-core`
- `cargo check -p workflow-cli`
- focused Core-owned authoritative executor tests: 6 passed
- focused authoritative CLI tests: 9 passed
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run check:integrations` under the repository's validated Node 20 runtime
- `git diff --check`

A disposable Git repository was scaffolded with
`init-repo-governance --authoritative-governance`, validated, and run through
both the aggregate governance approval and the separate authored-step approval.
The run completed with 20 ordered events, two approval-presentation proof
markers, and one persisted WorkReport artifact. No provider or external-write
path was used.

The implementation phase was governed by `dg/runtime-composition`:

- run ID: `run-1785468966766957000-2`;
- approval ID:
  `approval/run-1785468966766957000-2/composition-approved`;
- approval outcome: granted with persisted presentation proof;
- terminal status: `Completed`;
- event summary: 39 events, one approval, no retries, and no escalations; and
- report posture: repo edits, shell and validation commands, the disposable
  proof, and subsequent git/PR work remained outside kernel execution and are
  disclosed here rather than represented as kernel-executed work.

## 10. Remaining Limitations

- The closed authoritative profile supports exactly one immutable step.
- Mutation capability without a concrete SideEffect fact remains incomplete and
  fails closed.
- Authored step approvals remain distinct from aggregate governance approval.
- Scaffold default activation remains deferred.
- OpenShell is not integrated; any future use should be an optional execution
  provider behind a provider-neutral request and receipt boundary.

## 11. Recommended Next Phase

Proceed to authoritative governance scaffold default activation. Preserve an
explicit opt-out and do not broaden provider or mutation behavior.
