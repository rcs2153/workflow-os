# Project-Scoped Approval Routing Core Model Report

## 1. Executive Summary

Workflow OS now has a deterministic, payload-free Core boundary for resolving
which project actor may receive a pending approval. Immutable workflow metadata
selects a candidate; deployment-owned exact-project capability remains the
authority source. The resolver never creates or widens authority.

This phase is model-only. It does not persist routes, expose a hosted inbox,
send notifications, decide approvals, or change workflow execution.

## 2. Scope Completed

- added `ProjectApprovalRouteId` as a validated content-derived identity;
- added bounded routing reason, route status, and notification posture enums;
- added private, read-only `ProjectApprovalRoute` records;
- added explicit `ProjectApprovalRouteInput` values;
- added pure `resolve_project_approval_route` behavior;
- validated exact project scope, active run binding, pending approval subject,
  bounded authority view, and route posture;
- intersected maintainer or escalation-contact metadata with exact-project
  `ApprovalDecide` capability;
- represented missing metadata and unavailable authority explicitly;
- rejected ambiguous duplicate authority state;
- added validated serde reconstruction and redaction-safe `Debug` behavior;
- exported the model and resolver from `workflow-core`;
- added focused security and compatibility tests.

## 3. Scope Explicitly Not Completed

This phase did not add route persistence, approval enumeration, a hosted inbox,
email, chat, paging, webhooks, notification queues, delivery claims, dynamic
identity, invitations, groups, RBAC, IdP integration, workflow schema changes,
approval automation, workflow execution changes, provider writes, CLI behavior,
or release-posture changes.

## 4. Model And Resolver Summary

The resolver consumes an exact `HostedProjectScope`, active project-bound run
binding, validated pending `ApprovalRequest`, immutable `OwnershipMetadata`, one
closed routing reason, and an explicit deployment principal view.

For ordinary approvals it selects `maintainer`; for an explicitly
escalation-related approval route it selects `escalation_contact`. A selected
actor becomes the recipient only when one matching principal binding belongs to
the exact organization and grants `ApprovalDecide` for the exact project.

## 5. Authority Boundary

The implemented invariant is:

```text
routing candidate = immutable workflow metadata
decision authority = deployment-owned exact-project capability
effective recipient = candidate intersection authority
```

`owning_team` text is ignored. Missing authority produces an unresolved route,
not a fallback recipient. Duplicate matching authority bindings fail closed as
ambiguous state.

## 6. Privacy And Redaction

Routes contain stable references only. They do not copy approval reasons,
workflow payloads, source contents, evidence payloads, provider payloads,
command output, credentials, paths, or contact details. `Debug` redacts route,
scope, run, approval, workflow, and recipient identities. Validation and serde
errors use stable messages without input values.

## 7. Test Coverage

Focused tests cover:

- successful maintainer routing with exact-project authority;
- metadata without authority remaining unresolved;
- owning-team text being ignored;
- wrong-project and wrong-organization grants remaining unresolved;
- explicit missing-metadata status;
- escalation-contact routing vocabulary;
- ambiguous duplicate authority rejection;
- inactive run binding and decided approval rejection;
- deterministic route identities and serde round trips;
- tampered serialized route rejection;
- `Debug` non-leakage.

## 8. Validation Commands

Completed during implementation:

- `cargo test -p workflow-core --test project_approval_routing`: passed, 8 tests.
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 9. Remaining Limitations

- route records are not persisted;
- pending approvals cannot yet be enumerated through a project-scoped state
  query;
- no hosted approval inbox consumes the resolver;
- no route is enforced during approval decision handling;
- no external notification delivery exists;
- escalation routing accepts an explicit exact run-bound `EscalationRecord`;
  automatic event-log discovery or executor integration is not implemented;
- principals remain deployment-provisioned configuration.

## 10. Recommended Next Phase

The focused blocker-fix review accepted the Core model. Proceed to
project-scoped route persistence planning, including durable source
authentication and create-only conflict semantics. Do not begin hosted inbox
consumption until persistence is separately implemented and reviewed.

## 11. Governed Phase Record

- dogfood workflow: `dg/implement`;
- run ID: `run-1786670136342705000-2`;
- approval ID:
  `approval/run-1786670136342705000-2/implementation-approved`;
- approval outcome: granted with persisted presentation proof
  `presentation/bb5288884c485db0`;
- phase status: `Completed`;
- validation summary: all required Rust, documentation, and diff checks passed;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: approval event marker present and matched to the persisted
  presentation record.

Repository inspection, code and documentation edits, tests, validation, and git
operations are executor work performed outside the kernel. The kernel governed
the approved scope and recorded the approval; it did not execute those actions.
