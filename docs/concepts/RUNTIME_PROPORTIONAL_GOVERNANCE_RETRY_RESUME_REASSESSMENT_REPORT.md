# Runtime Proportional-Governance Retry/Resume Reassessment Report

## 1. Executive Summary

The explicit opt-in proportional-governance executor path now reassesses exact
retries and approval decisions against the durable immutable run bundle and
assessment binding. Current typed runtime facts must reproduce the accepted
binding before a run is rehydrated or any approval, policy, resume, or skill
event is appended.

Existing executor and approval APIs remain unchanged. This phase hardens
binding integrity; it does not enforce the assessment's approval or denial
disposition.

## 2. Scope Completed

- Exact assessment-bound retries re-read the stored immutable bundle.
- Retry requests must still match the exact bundle identity and execution
  posture established at run creation.
- Current typed facts are reassessed with the accepted deterministic helper.
- Optional expected aggregate fingerprints are revalidated.
- Snapshot, durable, and reassessed bindings must be exactly equal.
- Added `decide_approval_with_governance_reassessment` as an explicit opt-in
  approval boundary.
- Reassessment completes before approval decision, policy, resume, or skill
  events are appended.
- Added stable non-leaking failures for missing and changed bindings.

## 3. Runtime Semantics

An exact retry returns the already durable run and does not invoke a skill or
append duplicate events. Changed retry facts or an expected fingerprint
mismatch fail before new events.

The approval helper first performs the existing pending-approval validation,
then reopens and reassesses the immutable governance context. Only an exact
match reaches existing approval application and resolved-context validation.
Changed facts leave the run waiting for approval with its event history
unchanged.

## 4. Error And Privacy Boundary

Errors use stable codes and bounded messages. They do not include workflow,
run, step, bundle, fingerprint, path, definition, runtime-fact, provider,
command, environment, credential, or payload values. The helper stores no new
raw material and does not redesign persistence.

## 5. Tests Added

Focused behavior tests prove:

- exact retry reassesses and rehydrates without duplicate invocation;
- changed retry facts fail before new events;
- changed expected retry fingerprint fails before new events;
- matching approval-resume facts complete the run;
- changed approval-resume facts append no approval or resume events and invoke
  no skill;
- existing pre-run expected-fingerprint and event-order behavior remains.

## 6. Scope Explicitly Not Added

No default executor change, assessment-disposition enforcement, inferred or
trusted fact freshness, schema, CLI, UI, provider call, provider write,
automatic approval, enterprise control, hosted behavior, or unrelated runtime
refactor was added.

## 7. Validation

- Focused governance-assessment executor tests: passed.
- Full formatting, lint, workspace tests, documentation checks, and diff checks
  are recorded at phase close.

## 8. Known Limitations

- Runtime facts are caller-supplied typed facts without independent freshness
  attestations.
- Assessment dispositions remain recorded, not enforced.
- Only the explicit opt-in executor and approval helper use this boundary.
- Schema and operator surfaces do not expose this path.

## 9. Recommended Next Phase

Focused maintainer review accepts the complete opt-in local assessment-binding
path with non-blocking follow-ups. Continue with the next
roadmap-authoritative bounded runtime phase; do not add disposition enforcement
or default behavior without separate planning and review.

## 10. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1784504893339571000-2`
- Approval ID: `approval/run-1784504893339571000-2/composition-approved`
- Presentation ID: `presentation/5e4da7cabb6e8115`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: Rust implementation and tests, documentation updates,
  validation commands, diff inspection, and phase-report drafting
