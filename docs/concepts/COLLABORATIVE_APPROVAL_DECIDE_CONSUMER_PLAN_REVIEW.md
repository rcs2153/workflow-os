# Collaborative ApprovalDecide Consumer Plan Review

Status: plan accepted with mandatory implementation decisions

## 1. Executive Verdict

**Plan accepted; proceed to the transaction-scoped collaborative source
refactor only.**

The plan selects the correct first consequential collaborative identity
consumer and now defines a credible non-TOCTOU persistence boundary. Review
found and corrected one material ambiguity: the existing PostgreSQL
`record_approval_decision(...)` primitive is not itself an executor-integrated
runtime consumer. The accepted plan now requires both the atomic backend
operation and one explicit proof-enforced executor-facing callsite with crash-
safe rehydration that does not append a duplicate decision event.

No planning blocker remains for Phase 1. Receipt persistence and the actual
decision consumer remain separately reviewable later phases.

## 2. Scope Verification

The planning phase stayed within scope. It added no Rust implementation,
PostgreSQL migration, hosted endpoint, authentication protocol, identity
administration, provider write, workflow schema, CLI, SDK, example, or second
capability consumer.

The plan remains limited to one exact-project `ApprovalDecide` path. It does
not authorize OIDC, SCIM, dynamic groups, enterprise administration, hosted
authentication cutover, another provider mutation, nested harness execution,
or production identity claims.

## 3. Consumer Selection Assessment

`HostedProjectCapability::ApprovalDecide` is the correct first consumer:

- it is consequential enough to prove current authority matters;
- it already has exact project scope, pending approval state, presentation
  proof, event history, and PostgreSQL persistence foundations;
- it can fail before external side effects;
- it exercises revocation and authority revision semantics directly; and
- it does not require broadening into provider writes or general tool
  execution.

The plan correctly rejects a generic capability consumer and caller-selected
capability input.

## 4. Trusted Query Derivation Assessment

The query is appropriately fixed to organization, project, authenticated local
principal, and `ApprovalDecide`. Core derives organization and project from a
trusted exact project scope and derives principal from the authenticated hosted
composition context.

The request cannot supply identity facts, source evidence, a watermark,
trusted time, completeness, or a capability. This preserves the accepted rule
that serialized evidence and caller-assembled projection claims are not
authority.

Implementation must continue to verify that the authenticated principal is the
actor on the proposed decision and that the run's durable project binding
matches the trusted scope.

## 5. Atomicity And TOCTOU Assessment

The accepted transaction design is sound after review clarification:

1. acquire the same organization-scoped session advisory lock as authority
   revision activation;
2. start one serializable transaction;
3. lock the active watermark row `FOR UPDATE`;
4. obtain database time and resolve complete current authority;
5. lock and validate project binding, pending approval, and presentation;
6. compare the watermark again;
7. append the decision event, update the approval projection, and create the
   receipt atomically; and
8. commit before releasing the advisory lock.

The shared advisory lock is required because a row lock cannot protect a
watermark row that does not yet exist. Serializable isolation remains useful
for conflict detection, but it is not treated as a substitute for an explicit
serial order with revision activation.

Retry must reconstruct the query result and all approval state. A positive
private view from a failed attempt cannot be reused.

## 6. Runtime Callsite Assessment

The original plan was ambiguous about whether a backend method alone counted
as the consumer. Repository inspection showed that
`PostgresStateBackend::record_approval_decision(...)` is currently called only
by persistence tests. Existing executor approval paths append their own
decision events and do not compose this method.

The plan now requires an executor-facing
`decide_collaborative_project_approval_with_presentation(...)`-style boundary.
It must prepare and proof-mark the decision through existing validation, invoke
the atomic PostgreSQL operation exactly once, then rehydrate the committed
event before continuing. It must not call a path that appends the decision a
second time.

This correction is mandatory. Phase 3 may not claim a runtime consumer if it
delivers only a new storage helper.

## 7. Approval Gate Preservation Assessment

Collaborative identity authority is correctly treated as one prerequisite, not
the entire approval decision. The implementation must preserve:

- approval-presentation proof;
- policy and evidence requirements;
- immutable run context;
- active project/run binding;
- current required checks;
- proportional governance;
- selected high-assurance requester/approver separation;
- approval expiration and revocation posture;
- SideEffect disclosure/linkage; and
- terminal WorkReport obligations.

The first callsite requires presentation proof. It must not claim high-
assurance posture unless the selected request actually composes the accepted
high-assurance controls.

## 8. Denial Semantics Assessment

The review accepts the conservative rule that both grant and denial require
current `ApprovalDecide` authority. A denial is not passive observation; it
changes durable project workflow state and normally fails the run closed.

This does not prevent later policy from defining a narrower cancel or emergency
stop capability. No such capability is introduced here.

## 9. Receipt Assessment

A dedicated create-only receipt table is preferable to generic records storage
for this security boundary. It permits exact lookup by approval and decision
event, explicit uniqueness, additive migration, focused recovery checks, and a
clear corruption boundary.

The planned receipt remains payload-free and non-authorizing. Its commitments
are sufficient to show which query, source revision, grant, snapshot, decision,
and database observation produced the decision without preserving provider
subjects, identity assertions, grant payloads, presentation text, or reasons.

The receipt cannot be accepted as a permit for resumption or another action.

## 10. Crash And Recovery Assessment

Atomic decision persistence and workflow resumption are distinct recovery
steps. A crash after the decision transaction commits but before resumption
must leave:

- one durable decision event;
- one updated approval projection;
- one authority receipt; and
- no duplicate decision on retry.

The executor-facing path must rehydrate this committed state and continue from
it. It must not attempt to replay the decision through an existing append-first
helper. Backup/restore rehearsal must prove receipt and event equivalence.

## 11. Privacy And Error Assessment

The plan's privacy posture is appropriate:

- private authority views are call-local and non-serializable;
- public evidence and receipts contain bounded commitments only;
- Debug output is redacted;
- fact counts remain exact-query-minimized;
- deserialization verifies commitments and emits generic errors; and
- stable errors do not echo identifiers, subjects, reasons, payloads, paths,
  tokens, or database content.

No raw provider assertion or external subject should enter the approval event
or authority receipt.

## 12. Test Plan Assessment

The planned coverage is strong and includes positive authority, complete
negative authority, every lifecycle failure, revision and revocation races,
concurrent duplicate decisions, retry freshness, gate preservation, non-
mutation on failure, restart/replay, corruption, and non-leakage.

Two requirements are especially important:

- concurrency tests must use transaction barriers or database hooks rather
  than timing sleeps; and
- Phase 3 tests must prove the executor-facing callsite creates exactly one
  decision event across crash/retry boundaries.

Local environment skips are acceptable only when disclosed. CI must provide
live PostgreSQL conformance and recovery evidence before implementation review.

## 13. Blockers

None for Phase 1 after the runtime-callsite and lock-domain clarifications.

Phase 2 and Phase 3 remain blocked on their preceding phase reviews by design;
this is sequencing, not an unresolved planning defect.

## 14. Non-Blocking Follow-Ups

- Keep source activation and advisory-lock helpers internal to trusted
  composition; do not expose identity administration.
- Measure lock contention before considering narrower locking or indexed exact
  reads.
- Preserve the distinction between historical project approval routes and
  current collaborative identity authority.
- Add operator recovery tooling only after the consumer path is accepted.
- Do not broaden into another capability until the full consumer is reviewed.

## 15. Recommended Next Phase

Implement **Phase 1: Transaction-Scoped Source Refactor** only.

That phase should extract a private source-read helper that runs inside a
caller-owned PostgreSQL transaction, preserve the existing public non-
authorizing read unchanged, and prove equivalent evidence plus redaction and
error behavior. It should not add the receipt table or approval consumer yet.

This gives the later atomic decision transaction a reviewed primitive without
combining source refactoring, storage migration, executor integration, and
security-sensitive decision behavior in one code change.

## 16. Validation And Governed Evidence

- `npm run check:docs`: required for review completion.
- `git diff --check`: required for review completion.
- Focused comparison included the engineering standard, accepted source plan
  and review, PostgreSQL source and approval transaction, project route
  persistence, executor approval paths, and collaborative threat model.
- Dogfood workflow: `dg/review`.
- Run: `run-1792651156581223000-2`.
- Approval: `approval/run-1792651156581223000-2/review-scope-approved`.
- Approval presentation: `presentation/c0dfb6712bd7d9bb`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Out-of-kernel work: repository inspection, documentation edits, validation,
  git, and later PR operations are performed by the delegated maintainer;
  Workflow OS governs scope and records the review trail.
