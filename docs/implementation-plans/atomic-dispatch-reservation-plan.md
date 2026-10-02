# Atomic Dispatch Reservation Plan

> Implementation status: the private SQLite atomic dispatch-reservation slice
> is implemented. See [Atomic Dispatch Reservation Implementation
> Report](../concepts/ATOMIC_DISPATCH_RESERVATION_IMPLEMENTATION_REPORT.md).
> Repeated scheduling, provider execution, OpenShell, nested harnesses,
> automatic approval, public runtime configuration, CLI, SDK, and schema
> exposure remain unimplemented.

## 1. Executive Summary

The accepted one-shot trusted-host supervisor validates that an exact started
attempt is currently dispatchable before entering an injected executor. That
validation is a read. Two concurrent in-crate callers can both observe the
same attempt as dispatchable before either records a result.

The next implementation phase must replace that read-only boundary with one
private, SQLite-backed, atomic dispatch reservation. Core will admit exactly
one claimant for one exact started attempt and return one non-cloneable,
non-serializable capability to that first successful caller. Every competing,
stale, replayed, or ambiguous claimant must be rejected before executor entry.

This is not a scheduler. It does not add repeated dispatch, model turns,
provider execution, OpenShell, nested harnesses, automatic approval, public
runtime configuration, CLI, SDK, schema exposure, hosted behavior, or another
provider mutation family.

This document defined the implementation boundary. The bounded private slice
is now implemented without broadening the non-goals below.

## 2. Problem Statement

The current private supervisor consumes an owned attempt capability and checks
durable dispatchability immediately before executor entry. Ownership prevents
ordinary sequential reuse by a conforming caller, and a stale call after a
recorded result is rejected. It does not establish a one-winner durable claim
when two internal callers reconstruct equivalent authority concurrently.

The unsafe interleaving is:

1. claimant A validates the attempt as `started`;
2. claimant B validates the same attempt as `started`;
3. A enters the executor;
4. B enters the executor;
5. one result write wins and the other eventually fails.

Durable result protection is too late because external work may already have
run twice. Core must decide who may cross the executor boundary inside an
atomic state transaction.

## 3. Goals

- Atomically admit one claimant for one exact started attempt.
- Reject every competing claimant before executor entry.
- Bind admission to the complete immutable invocation, selected executor,
  window, attempt, actor, authority, governance, cursor, revision, and trusted
  time posture already accepted by the supervisor boundary.
- Return one private in-memory dispatch capability only to the first caller
  that receives an unambiguous successful commit acknowledgement.
- Persist a payload-free reservation receipt and commitment for audit and
  reconciliation.
- Preserve the accepted five-operation continuity result, yield, and recovery
  semantics.
- Fail closed when reservation commit acknowledgement is ambiguous.
- Keep exact replay idempotent without recreating execution authority.
- Preserve deterministic, non-leaking errors and Debug behavior.
- Prove one-winner behavior under real concurrent SQLite claimants and commit
  fault injection.

## 4. Non-Goals

- No scheduler, daemon, queue, polling loop, or repeated supervisor loop.
- No model-turn creation or conversational auto-resume.
- No automatic approval, evidence bypass, check bypass, or delegated-authority
  broadening.
- No provider execution, provider mutation, OpenShell, or live adapter.
- No nested harness execution, recursive agents, or agent swarms.
- No public runtime configuration, CLI, SDK, workflow schema, or example.
- No filesystem or PostgreSQL implementation in the first slice.
- No hosted or distributed execution claim.
- No lease renewal, reservation stealing, automatic timeout recovery, or
  reassignment after host loss.
- No binary, process, hardware, or remote attestation.
- No new public continuity operation or change to workflow terminal semantics.
- No prompts, transcripts, source contents, command output, environment
  values, credentials, authorization headers, or provider payloads.

## 5. Required Invariants

### 5.1 One Winner

For a given attempt, at most one reservation operation may commit an admitted
posture. A second claimant must receive a stable rejection before its executor
callback can run.

### 5.2 Authority Is Not Reconstructable

The durable reservation record is evidence that admission occurred. It is not
an execution credential. It must not contain enough material to reconstruct
the private dispatch capability.

### 5.3 Replay Does Not Reissue Authority

An exact replay of a committed reservation operation may return a bounded
receipt or already-admitted posture. It must not return another dispatch
capability. Idempotency protects durable state; it does not authorize another
executor call.

### 5.4 Ambiguity Does Not Issue Authority

If commit acknowledgement is lost, fresh-connection reconciliation may prove
that the reservation committed. Even then, the current call must return a
non-authorizing admitted-but-unavailable result. It must not manufacture a new
capability after the original delivery boundary became ambiguous.

### 5.5 Exact Binding

Reservation must commit the exact invocation and executor binding already
authorized at operational-window opening or directive consumption. A change
to any bound identity, value commitment, cursor, revision, actor, authority,
governance fact, trusted-time fact, or operation binding must reject before
admission.

### 5.6 No False Completion

Reservation success means only that one executor entry is admitted. It does
not mean the attempt, execution window, workflow step, or workflow run
completed.

## 6. Candidate Private Core Model

The first implementation should add the smallest crate-private model set:

- `DispatchReservationOperationId`
- `DispatchReservationReceiptId`
- `DispatchReservationRequest`
- `DispatchReservationCommitment`
- `DispatchReservationRecord`
- `DispatchReservationProjection`
- `DispatchReservationOutcome`
- `ReservedAttemptDispatchCapability`

The names may follow adjacent Core conventions, but the semantics must remain
closed.

`DispatchReservationOutcome` should distinguish at least:

- `Admitted { capability, receipt }` for the sole unambiguous first success;
- `AlreadyAdmitted { receipt }` for exact replay without authority;
- `Rejected` for stale, conflicting, expired, revoked, or non-current input;
- `CommittedButCapabilityUnavailable { receipt }` for an ambiguous commit
  proven durable on reconciliation; and
- a stable storage/reconciliation error for unreadable or corrupt state.

Only `Admitted` may carry `ReservedAttemptDispatchCapability`.

## 7. Capability Contract

`ReservedAttemptDispatchCapability` must be:

- crate-private;
- owned and consumed by one supervisor call;
- non-`Clone`;
- non-serializable and non-deserializable;
- Debug-redacted;
- bound to the reservation operation and receipt;
- bound to the exact window and attempt;
- bound to expected window and attempt revisions;
- bound to the complete invocation commitment;
- bound to the executor-selection commitment;
- bound to authority and governance commitments; and
- bound to the admitted trusted-time observation and epoch.

The capability is an in-process proof of successful first-delivery admission,
not a durable bearer credential.

## 8. Durable Reservation Model

The SQLite implementation should use a dedicated private reservation table or
equivalent projection keyed by attempt identity with a uniqueness constraint
that enforces one admitted reservation per attempt.

The durable record should contain only bounded identities, versions,
commitments, revisions, trusted-time facts, disposition, and timestamps. It
must not contain invocation values, skill inputs, prompts, transcripts,
command output, provider payloads, or credentials.

The attempt may remain in the accepted `started` state while the presence of
the exact committed reservation derives `dispatch_admitted`. This avoids
changing the closed attempt outcome vocabulary merely to express executor
admission. Result and yield operations must require the exact reservation
binding in addition to their current attempt capability.

The schema migration must be additive and explicit. The first implementation
is SQLite-only and must update the managed manifest digest and migration tests
without pretending filesystem or PostgreSQL parity.

### 8.1 Append-Only Admission Event

Dispatch admission is a meaningful runtime transition and must be represented
in the run event ledger. The reservation transaction must therefore append one
payload-free `AuthorizedExecutionAttemptDispatchAdmitted` runtime event and
project its binding into the run snapshot in the same transaction that inserts
the unique reservation.

Its bounded projection payload must contain:

- reservation operation ID and receipt ID;
- window ID and attempt ID;
- window revision and attempt revision;
- reservation request commitment;
- invocation commitment and executor-selection commitment;
- authority, governance, and trusted-time commitments;
- expected input cursor and committed result cursor; and
- the immutable reservation commitment.

It must not contain invocation values, skill input, prompts, transcripts,
command output, provider payloads, credentials, paths, or other raw content.
The event is runtime audit/projection vocabulary only. It is not a sixth
caller-invoked continuity operation, a workflow-spec field, a CLI command, an
SDK surface, or an execution credential.

The run snapshot should retain only the latest exact dispatch-admission
projection binding needed for integrity checks and inspection. Rehydration
must preserve workflow status and must not interpret admission as attempt,
step, or run completion.

## 9. Atomic Reservation Transaction

One immediate SQLite transaction should:

1. verify the exact managed schema and healthy trusted-time state;
2. load the exact execution window and attempt;
3. require the window to be executing and the attempt to be started;
4. verify workflow, run, step, actor, immutable bundle, governance, authority,
   operation, invocation, executor, cursor, revision, expiry, and epoch
   commitments;
5. reject an existing reservation for another operation or commitment;
6. insert the reservation record and payload-free projection under the unique
   attempt constraint;
7. append the deterministic dispatch-admission event and project the snapshot;
8. validate the reservation, event, and snapshot commitments as one exact
   binding;
9. commit; and
10. return the private capability only after an unambiguous commit success.

No executor callback may occur before step 8.

## 10. Replay And Conflict Behavior

- Same operation ID and same request commitment after a confirmed committed
  reservation: return `AlreadyAdmitted` with the original receipt and event
  binding, no duplicate event, and no capability.
- Same operation ID with a different commitment: security rejection.
- Different operation ID for the same attempt: already-admitted rejection.
- Stale attempt/window revision or cursor: invalid-state rejection.
- Expired, revoked, superseded, yielded, closed, or terminal posture:
  rejection.
- Missing or mismatched opening/directive origin: security rejection.
- Corrupt partial reservation/projection: recovery-required error.

Every read or replay path must cross-check the reservation relation, admission
event binding, and projected run snapshot. A missing, duplicated, conflicting,
or partially projected admission fails closed.

Errors must use stable codes and must not echo IDs or caller values.

## 11. Ambiguous Commit Reconciliation

The request and expected receipt must be fully committed before mutation. On a
write error that may follow commit, Core must open a fresh SQLite connection
and reconcile the exact operation ID and request commitment.

Reconciliation outcomes:

- confirmed absence: return a bounded failure; no capability;
- exact committed admission: return
  `CommittedButCapabilityUnavailable` with the committed receipt and event
  binding; no capability;
- committed security rejection: return the bounded rejection;
- conflicting, unreadable, or partial state: return recovery-required; no
  capability.

The caller must never infer that retrying executor entry is safe from a durable
receipt alone.

## 12. Supervisor Integration

The supervisor should no longer call
`attempt_dispatch_is_current(...)` immediately before executor entry.

Instead:

1. a private reservation helper consumes the opening or resumed attempt
   capability and returns `DispatchReservationOutcome`;
2. only `Admitted` is converted into supervisor input;
3. the supervisor consumes `ReservedAttemptDispatchCapability` by value;
4. the supervisor validates the exact invocation and executor binding again;
5. the executor is invoked once; and
6. result, yield, and ambiguity persistence require the reservation binding.

The existing request models must gain private reservation-binding inputs:

- `dispatch_reservation_receipt_id`;
- `dispatch_reservation_commitment`; and
- `dispatch_admission_cursor`.

`RecordAttemptOutcomeRequest`, `RegisterYieldRequest`, and
`RecoverAmbiguousAttemptRequest` must verify those values against the exact
reservation relation and admission event inside their existing mutation
transactions. The old attempt-use capability alone is no longer sufficient
for those three mutations after reservation is implemented.

The reservation module must own the only constructor for
`ReservedAttemptDispatchCapability`. Its fields remain private to that module.
Other modules may consume the opaque value through the supervisor API but may
not construct, clone, serialize, deserialize, or reconstruct it from a receipt
or event.

This phase must not add a loop around the supervisor. One explicit caller may
reserve and invoke one attempt.

## 13. Trusted Time And Recovery Posture

Reservation must reuse the accepted injected trusted-time source, provenance,
epoch, monotonic watermark, and expiry checks. It must not read ambient wall
clock time.

If a process exits after reservation commit but before executor entry, Core
cannot prove whether external work began. The first slice must leave the
attempt admitted and fail closed. Automatic lease expiry, reassignment, or
reservation stealing would risk duplicate work and is deferred to a separate
recovery design.

## 14. Privacy And Redaction

- Persist commitments and bounded references, not payloads.
- Redact all reservation and capability Debug output.
- Reject secret-like caller-authored identifiers before storage.
- Keep serialization errors generic and value-free.
- Do not include file paths, invocation values, prompts, transcripts, command
  output, environment values, credentials, or provider payloads in errors,
  events, projections, or receipts.
- Treat the reservation record as sensitive operational metadata even though
  it is payload-free.

## 15. Test Plan

Future implementation tests must prove:

1. one exact started opening attempt can be reserved;
2. one exact resumed attempt can be reserved;
3. reservation returns one non-reusable private capability;
4. sequential second claim is rejected before executor entry;
5. two concurrent claimants produce exactly one admitted capability;
6. the losing claimant never enters the executor;
7. exact operation replay returns a receipt without a capability;
8. conflicting replay is rejected;
9. every invocation field substitution is rejected;
10. executor identity substitution is rejected;
11. actor, authority, governance, cursor, revision, epoch, and expiry
    substitutions are rejected;
12. yielded and terminal attempts cannot be reserved;
13. before-commit fault reconciles to confirmed absence without authority;
14. after-commit fault reconciles to committed-but-unavailable without
    authority;
15. partial or corrupt projection fails closed;
16. one admission appends one ordered payload-free admission event;
17. a losing claimant and exact replay append no duplicate admission event;
18. ambiguous reconciliation returns the committed event binding without
    authority;
19. reservation, admission event, and snapshot disagreement fails closed;
20. rehydration preserves workflow status and the exact admission projection;
21. supervisor entry requires and consumes the reserved capability;
22. result, yield, and ambiguity persistence require the receipt, commitment,
    and admission cursor binding;
23. supervisor success does not complete the workflow run;
24. capability construction is localized to the reservation module;
25. Debug, errors, serialization, and persistence do not leak raw values;
26. managed SQLite migration and manifest integrity remain deterministic;
27. existing opening, continuity, supervisor, executor, report, adapter, and
    runtime tests remain green; and
28. `cargo test --workspace` passes in CI.

## 16. Proposed Implementation Sequence

1. Add private reservation identities, request, outcome, receipt, projection,
   and capability types.
2. Add the additive SQLite reservation schema and explicit migration.
3. Add the payload-free admission event, snapshot projection, and integrity
   codec.
4. Implement atomic one-winner reservation, event projection, and exact
   reconciliation in one transaction.
5. Replace supervisor read-only dispatch validation with required reservation
   capability consumption.
6. Bind existing result, yield, and ambiguity persistence to the reservation
   receipt, commitment, and admission cursor.
7. Add concurrency, replay, event-integrity, substitution, fault, and privacy
   tests.
8. Update roadmap and create an end-of-phase report.
9. Perform a focused maintainer/security review before any repeated dispatch
   or scheduler work.

## 17. Open Questions

- Should the first implementation represent admitted posture solely through a
  reservation relation, or also add a private attempt-state discriminator?
- Should result persistence settle the reservation in the same transaction or
  leave it immutable and derive settlement from the attempt outcome?
- What bounded operator recovery is appropriate for an admitted capability
  lost before executor entry?
- Should a future reservation lease ever be reclaimable, and what evidence
  would prove no external work began?
- What conformance interface will eventually support PostgreSQL without
  broadening this SQLite-only slice?

These questions must not be answered by weakening the one-winner or
no-authority-on-ambiguity invariants.

## 18. Final Recommendation

After focused plan review, implement the **private SQLite atomic dispatch
reservation slice** exactly as sequenced above.

Do not begin a repeated supervisor loop, scheduler, provider execution,
OpenShell integration, nested harness runtime, automatic approval, or public
configuration first. Core must prove that one and only one caller can cross
the executor boundary for an exact attempt.
