# External Action Connector Same-Call Admission Report

## 1. Executive Summary

The first Core-private same-call external connector admission slice is
implemented.

Workflow OS can now combine an immutable external-action authorization
projection, durable transition lifecycle, fresh registered current authority,
and exact fixture-injected approval, resource, window, revocation, policy, and
evidence facts before one injected fixture connector may run. The connector
receives only a borrowed, non-serializable use capability for the admitted
operation and attempt. It receives no credential, provider payload, reusable
authority token, or raw evidence.

This is a dark fixture boundary. It does not expose a public connector API,
perform network access, call GitHub, mutate a provider, add workflow schema or
CLI surfaces, or claim scheduler continuity.

## 2. Scope Completed

- Added one crate-private external-action connector admission boundary.
- Reused the registered current-authority same-call use boundary rather than
  introducing a second permission model.
- Added exact projected-grant verification against the requested capability,
  repository resource, actor, workflow, run, step, harness contract,
  sensitivity, availability, lifecycle, expiry, and current grant inventory.
- Required exact immutable run, workflow, actor, fixture execution-window, policy,
  governance-assessment, revocation-source, and resource binding.
- Required a granted approval decision with the exact presentation identity,
  content hash, requested-action scope commitment, strict-non-goals commitment,
  actor, and persisted proof marker.
- Required the exact projected evidence predicates and rejected stale or
  future-dated observations.
- Derived one deterministic operation and a distinct cursor-bound attempt, then persisted reservation,
  admission, attempted, and observed-outcome lifecycle records around the
  fixture call.
- Prevented terminal replay from invoking the fixture again.
- Preserved ambiguous outcomes as recovery-required state that blocks blind
  redispatch.
- Recorded known success, known failure, and ambiguous fixture outcomes without
  fabricating provider evidence.

## 3. Same-Call Authority Boundary

Serialized authorization projections and lifecycle records remain
non-authorizing claims. Admission opens the existing registered current-
authority boundary and receives a borrowed use capability only when the
required-context assessment is ready.

Inside that same call, Core locates the exact projected grant and recomputes
capability resolution from the complete current grant and availability
inventories. The selected grant must be the projected grant. A revoked,
expired, missing, shadowed, unavailable, or scope-mismatched grant never reaches
the connector.

The fixture connector receives a private borrowed capability containing only
the exact operation and a payload-free authority commitment. The capability is
not cloneable, serializable, persistable, or publicly exported.

## 4. Approval, Evidence, And Resource Integrity

Admission fails closed unless the approval presentation and decision prove the
exact projected scope. The proof marker must use presentation-required
enforcement and request-match validation, and its presentation identity,
content hash, and validation time must match the decision.

Current resource facts must equal the immutable projection, including the exact
repository, base ref and SHA, head ref and SHA. Current evidence must equal the
canonical projected predicate set and remain within each predicate's bounded
freshness window.

These checks occur before lifecycle initialization or connector entry.

## 5. Lifecycle And Replay Posture

The boundary composes the accepted durable transition lifecycle:

1. initialize the exact operation idempotently;
2. return an existing terminal result without connector re-entry;
3. return recovery-required for reserved, admitted, attempted, waiting, or
   ambiguous posture;
4. reserve one deterministic attempt;
5. admit that attempt under the fresh authority commitment;
6. mark the provider attempt before connector entry; and
7. record the bounded observed outcome after connector return.

A known failed outcome remains eligible for the lifecycle's separately
governed retry semantics. A retry receives a distinct local attempt identity
while preserving the stable provider operation/idempotency identity. An
ambiguous outcome is never retried blindly.

## 6. Scope Explicitly Not Completed

This phase does not implement:

- credentials, tokens, provider access material, or connector account lookup;
- network access, live GitHub behavior, or any provider mutation;
- a public connector, adapter, workflow, SDK, schema, or CLI surface;
- platform-permission discovery;
- provider-side reconciliation or successor pull-request identity;
- automatic retry, scheduler wakeup, or hosted supervision;
- source-backed reload of approval, policy, execution-window, revocation,
  resource, or evidence facts beyond the registered capability-authority
  source used by this fixture slice;
- SideEffect, WorkReport, or public telemetry projection;
- additional transition or provider families;
- nested harness execution or agent teams; or
- a reusable bearer capability or parallel authorization system.

## 7. Focused Coverage

Focused tests prove:

- one successful fixture call receives current authority and terminal replay
  does not invoke it again;
- a revoked projected grant blocks before lifecycle or connector entry;
- exact but stale evidence blocks before connector entry;
- a changed head binding blocks before connector entry;
- missing approval-presentation proof blocks before connector entry;
- known failure is durably recorded and retry uses a distinct attempt; and
- ambiguity is durable and replay returns recovery-required without a second
  connector call.

## 8. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1793725526885035000-2`
- approval: `approval/run-1793725526885035000-2/implementation-approved`
- approval presentation: `presentation/de59fc77fd7bc6f6`
- approval outcome: granted by delegated maintainer
- phase status: implementation and validation complete; focused review pending
- out-of-kernel work: local code and documentation edits performed by Codex
  inside the approved scope; the kernel governed phase boundaries and approval

## 9. Validation

- focused all-feature admission tests: 6 passed;
- full all-feature `workflow-core` library suite: 738 passed;
- strict all-target, all-feature `workflow-core` Clippy: passed;
- `cargo fmt --all -- --check`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.
