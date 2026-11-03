# External Action Transition Lifecycle Report

## 1. Executive Summary

The durable local lifecycle slice for projected external connector actions is
implemented.

Workflow OS can now derive one immutable operation for an exact transition in
an `ExternalActionAuthorizationProjection`, initialize an append-only local
history, and record reservation, admission, attempt, provider observation,
ambiguity reconciliation, genuine waits, denial, consumption, and revocation.
Every append is bound to an optimistic sequence-and-commitment cursor. Exact
replay returns the prior result without adding another event. Stale state,
illegal transitions, changed bindings, and corrupt records fail closed.

This slice does not authorize or execute an external action. It contains no
credential, provider payload, raw evidence, connector transport, GitHub call,
workflow schema, CLI mutation, scheduler, or automatic resumption. A future
Core-private same-call boundary must reload current authority and evidence
before any fixture connector may run.

## 2. Scope Completed

- Added `ExternalActionTransitionOperation`, deterministically derived from an
  immutable authorization projection and one listed transition.
- Added a payload-free lifecycle vocabulary for projected, reserved, admitted,
  attempted, observed, reconciled, waiting, denied, consumed, and revoked
  posture.
- Added typed observed outcomes for success, known failure, and ambiguity.
- Added typed refusal classes for authority, evidence, platform permission,
  connector availability, rate limit, external-state conflict, and scope.
- Added a local create-only operation definition and append-only event store.
- Added commitment-chained events with optimistic sequence-and-head cursors.
- Added exact-idempotent initialization and exact append replay.
- Added retry after a known or reconciled failure while blocking redispatch
  after an ambiguous outcome.
- Composed genuine wait records with the existing authorized-execution
  continuity model and restored the exact suspended lifecycle posture only
  after a satisfied wait.
- Added fail-closed rehydration for malformed, reordered, substituted, or
  structurally illegal history.
- Added validated operation deserialization so persisted operation claims
  cannot bypass deterministic commitment checks.
- Added redaction-safe operation diagnostics and bounded error messages.

## 3. Durable State Boundary

The immutable authorization projection remains separate from mutable lifecycle
state. The projection states what may become lawful. The lifecycle records what
happened to one exact projected transition.

The local store writes:

- one create-only operation definition;
- one create-only projected event; and
- monotonically numbered, commitment-chained event records.

The event files are the source of lifecycle truth. There is no caller-authored
mutable head pointer or `remaining transitions` field. A rehydrated snapshot is
derived from the validated history.

The store uses the shared local-state writer guard and create-only file commits.
An append must present the exact current sequence and event commitment. A retry
of the exact immediately preceding append returns `ExactReplay`; a different
command against that stale cursor is rejected.

## 4. Outcome And Retry Semantics

An admitted operation is recorded as attempted before an outcome may be
observed.

- known success may be consumed;
- known failure may reserve a new attempt;
- ambiguity blocks consumption and redispatch;
- reconciliation must resolve ambiguity to success or failure;
- reconciled success may be consumed; and
- reconciled failure may reserve a new attempt.

The lifecycle does not claim a provider mutation occurred merely because local
state advanced. Outcome commitments are bounded references for later connector
and reconciliation composition, not provider evidence by themselves.

## 5. Wait And Continuation Semantics

A durable wait must be an existing `AuthorizedExecutionWaitCondition` bound to
the same run, execution window, operation action reference, and active attempt.
The lifecycle records the exact suspended status.

A satisfied wait restores that status. Expired or superseded waits close the
operation as denied under the current lifecycle. This phase records resumable
state but does not claim that a scheduler observes or resumes it automatically.

## 6. Authority Boundary

The lifecycle is non-authorizing.

Serialized operation definitions, events, snapshots, reservations, and outcome
records cannot admit a connector call. They do not prove that approval,
delegation, capability, evidence, policy, expiry, revocation, or execution-window
facts remain current.

The next phase must add a Core-private same-call verifier that reloads those
facts, reserves the exact transition atomically, and passes a non-serializable,
single-use capability to an injected fixture connector. No public bearer token
or second authorization system should be introduced.

## 7. Scope Explicitly Not Completed

This phase does not implement:

- connector admission or execution;
- current-authority or current-evidence reassessment;
- a same-call use capability;
- credentials or platform permission resolution;
- GitHub API calls or any provider mutation;
- provider lookup or live reconciliation;
- successor PR persistence;
- event-to-SideEffect or WorkReport projection;
- workflow YAML, schemas, SDK, or CLI mutation surfaces;
- automatic retry, scheduler wakeup, supervisor, or hosted behavior;
- branch deletion or broader provider action families;
- Governed Obligation Plan runtime work; or
- Salsa or another incremental computation dependency.

## 8. Focused Validation

Focused unit coverage proves:

- create-only exact-idempotent initialization;
- a complete reservation-to-consumption lifecycle;
- exact replay without duplicate events;
- stale cursor rejection;
- terminal revocation;
- ambiguity blocking redispatch until reconciliation;
- exact continuity wait binding and status restoration; and
- tampered durable event rejection.

Validation completed:

- `cargo fmt --all -- --check` - passed;
- focused lifecycle unit suite - 6 passed after review corrections;
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings`
  - passed;
- serial `cargo test -p workflow-core --all-features --lib` - 637 passed after
  review corrections;
- `npm run check:docs` - passed; and
- `git diff --check` - passed.

## 9. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1793681905012513000-2`
- approval: `approval/run-1793681905012513000-2/composition-approved`
- approval presentation: `presentation/79f6c06b03cf5874`
- approval outcome: granted by delegated maintainer
- phase status: completed after validation
- event summary: recorded by the governed phase close, with one approval and no
  retries or escalations
- out-of-kernel work: local code and documentation edits performed by Codex
  inside the approved scope; the kernel governed phase boundaries and approval
