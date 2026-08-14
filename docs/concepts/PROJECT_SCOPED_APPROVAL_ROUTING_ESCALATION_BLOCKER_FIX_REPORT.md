# Project-Scoped Approval Routing Escalation Blocker Fix Report

## 1. Executive Summary

The escalation-subject blocker in the project approval route resolver is fixed.
Escalation-contact routing can no longer be selected for an ordinary pending
approval. It requires an exact run-bound `EscalationRecord` whose contact
matches the immutable workflow escalation contact.

The fix remains Core-only and payload-free. It does not persist routes, inspect
event history automatically, expose a hosted inbox, deliver notifications, or
change approval or workflow execution semantics.

## 2. Blocker Fixed

The initial resolver accepted a caller-selected
`WorkflowEscalationContact` reason without requiring proof of an actual runtime
escalation. This could route an ordinary approval to the escalation contact
while still returning `Routed`.

The input now carries an optional explicit escalation subject. Ordinary
maintainer routes reject any escalation subject. Escalation-contact routes
reject missing subjects and require exact run and immutable contact matches.

## 3. Implementation Approach

- added `Option<&EscalationRecord>` to `ProjectApprovalRouteInput`;
- added an optional stable `escalation_id` reference to the payload-free route;
- require no escalation subject for `WorkflowMaintainer`;
- require one valid escalation subject for `WorkflowEscalationContact`;
- require `escalation.run_id == approval.run_id`;
- require `escalation.contact == ownership.escalation_contact`;
- bind the escalation reference into the deterministic route identity;
- validate serde posture so an escalation route cannot deserialize without its
  stable escalation reference;
- keep `Debug` output redacted.

## 4. Validation Boundary

Stable error codes distinguish missing, unexpected, mismatched, and invalid
escalation subjects without including run, approval, actor, escalation, project,
path, reason, or payload values. The resolver still validates pending approval
subject, active exact-project run binding, bounded authority view, and unique
exact-project `ApprovalDecide` authority.

## 5. Privacy And Redaction

The route stores only the escalation identifier, not failure messages, reasons,
suggested actions, attempts, skill metadata, or other escalation payloads.
`Debug` redacts the escalation reference. Generic serde errors do not echo
invalid values.

## 6. Test Coverage

Focused tests now prove:

- ordinary approval escalation selection fails without a subject;
- ordinary maintainer routing rejects unexpected escalation proof;
- wrong-run escalation proof fails closed;
- mismatched immutable contact fails closed;
- exact run/contact escalation proof routes successfully;
- escalation routes round-trip through serde;
- missing serialized escalation proof fails closed;
- escalation references do not leak through `Debug`;
- the original maintainer, scope, ambiguity, tamper, and privacy tests remain
  green.

## 7. Validation Commands

Completed:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test -p workflow-core --test project_approval_routing`: passed, 10 tests;
- `cargo test --workspace`: passed; opt-in live integration tests remained
  ignored as designed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 8. Remaining Limitations

- the caller must supply the escalation record explicitly;
- the resolver does not discover or authenticate escalation records from event
  history;
- route persistence and decision-time route enforcement are not implemented;
- the hosted approval inbox and external notification delivery are not
  implemented;
- dynamic identity and enterprise administration remain deferred.

## 9. Recommended Next Phase

Focused project approval routing blocker-fix review. If accepted, proceed to
create-only project-scoped route persistence before hosted inbox consumption.

## 10. Governed Phase Record

- dogfood workflow: `dg/blocker`;
- run ID: `run-1786675529082542000-2`;
- approval ID: `approval/run-1786675529082542000-2/fix-approved`;
- approval outcome: granted with persisted presentation proof
  `presentation/cbc665c0fb0a6b6d`;
- terminal status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: approval event marker present and matched to the persisted
  presentation record.

Repository inspection, code and documentation edits, tests, validation, and git
operations are executor work performed outside the kernel. The kernel governed
the approved blocker-fix scope and recorded the approval; it did not perform
those actions.
