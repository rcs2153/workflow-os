# Authorized Execution Continuity Atomic State Plan

Status: Accepted after focused maintainer/security review in [Authorized
Execution Continuity Atomic State Plan
Review](../concepts/AUTHORIZED_EXECUTION_CONTINUITY_ATOMIC_STATE_PLAN_REVIEW.md).
Planning only; no runtime continuity state operations, schema migrations, or
supervisor behavior are implemented by this document.

Related foundations:

- [Authorized Execution Continuity Plan](authorized-execution-continuity-plan.md)
- [Authorized Execution Continuity Core Model Review](../concepts/AUTHORIZED_EXECUTION_CONTINUITY_CORE_MODEL_REVIEW.md)
- [Authoritative Agent Continuation Context And Rehydration Plan](authoritative-agent-continuation-context-rehydration-plan.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Durable State Contract](../runtime/durable-state-contract.md)

## 1. Executive Summary

The accepted continuity model can describe a bounded execution window, an
executor yield, a genuine wait, and an attempt outcome. It cannot yet make
those values authoritative or restart-safe. The next phase must define atomic
durable operations that preserve lawful non-terminal work across process and
executor-turn boundaries without creating duplicate execution authority.

The first implementation should add a narrow continuity-state capability,
authoritative record vocabulary, and an executable conformance harness. It
must atomically:

- register one exact yield or typed wait against the current run cursor; and
- consume one current resume directive, allocate one attempt, and record that
  attempt as started before any executor boundary is crossed.

Attempt outcome reconciliation must also be atomic with its event projection.
An attempt that reached `started` but has no durable outcome is ambiguous and
must never be automatically retried.

This phase does not schedule an executor, approve a gate, invoke a handler,
change `WorkflowRunStatus`, mutate a provider, expose a CLI command, or claim
that any existing backend supports continuity transactions before passing the
dedicated conformance suite.

## 2. Goals

- Define one Core-owned atomic continuity-state contract.
- Preserve exact workflow, run, step, immutable-bundle, cursor, window,
  authority, wait, directive, and attempt identity.
- Make exact replay idempotent and conflicting replay fail closed.
- Allocate attempt numbers and enforce attempt budgets inside the transaction.
- Persist `started` before crossing the executor-consumer boundary.
- Record success, failure, or ambiguous outcome without permitting blind
  retry.
- Specify crash posture at every durable boundary.
- Require executable backend conformance before support may be advertised.
- Keep authoritative state separate from non-authoritative serialized models
  and host-delivery acknowledgements.
- Preserve bounded, payload-free, redaction-safe storage and errors.

## 3. Non-Goals

This plan does not authorize:

- a scheduler, polling daemon, hosted worker, queue, or automatic model turn;
- executor, shell, git, browser, connector, provider, or external-write calls;
- delegated approval, model self-approval, or policy inference;
- new `WorkflowRunStatus` wire variants;
- generic transactional callbacks exposed through `StateBackend`;
- public executable resume tokens or bearer credentials;
- filesystem emulation of cross-record atomicity across multiple files;
- workflow schema, SDK, CLI, example, or release-posture changes;
- provider mutation broadening or nested harness execution;
- exactly-once claims for external effects;
- blind retry, lease takeover, or automatic recovery from ambiguity.

## 4. Authoritative Records

The state contract should define private or Core-owned durable records rather
than treating public continuity models as authority.

### Run cursor

The authoritative cursor is the exact pair of positive event sequence number
and event ID from the durable event tail. Sequence alone is insufficient.
Immutable run identity and bundle binding must match the run being mutated.

### Execution window state

One revisioned record owns:

- window ID and model version;
- workflow, run, step, action, resource, subject, and sensitivity scope;
- immutable run bundle and governance/authority commitments;
- opening cursor and expiry;
- lifecycle posture;
- maximum attempts and next allocatable attempt number;
- active yield/wait generation;
- last trusted-time watermark;
- durable revision.

Public `AuthorizedExecutionWindow` values may orient callers, but the store
must derive and reload authoritative state rather than trusting a serialized
window as permission.

### Yield and wait state

One yielded generation records the exact owning window, attempt, cursor, and
bounded yield posture. Genuine waits are separate versioned records bound to
the same generation. Ordinary turn boundaries create a yield, not a fabricated
approval, evidence, or external-event wait.

### Resume directive state

A directive is a derived, non-bearer identity over the current window
revision, yield generation, wait versions, run cursor, and fresh authority
assessment commitment. It is not accepted from serialized host input as proof
of authority.

### Attempt state

An attempt represents one bounded executor entry or turn. Every consumed
directive creates one revisioned attempt record with the legal lifecycle:

```text
prepared -> started -> yielded
                    -> succeeded
                    -> retryable_failure
                    -> terminal_failure
                    -> ambiguous_may_have_started
```

`prepared` exists only inside the atomic consume operation. `started` must be
durable before executor entry. `started` is intrinsically a may-have-started
posture whenever read outside the same live consumer call: it blocks another
consumer or retry without requiring an unsafe liveness inference. The original
same-call consumer may still record the terminal posture only by presenting
the private, non-cloneable, non-serializable attempt-use capability returned by
directive consumption. A yielded attempt cannot also receive another terminal
outcome. A recovery caller without that capability may only record
`ambiguous_may_have_started`.

### Events and snapshots

Continuity records and their corresponding durable events are committed in
one transaction. Events provide ordered audit/replay projection; continuity
records hold revisioned coordination state. Snapshots remain derived caches
and cannot authorize work when stale.

## 5. State Machines

### Authoritative window lifecycle

This persistence lifecycle is private coordination state and does not add
variants to the public model-only `AuthorizedExecutionWindowStatus` enum.

```text
assessment_required -> executing -> yielded -> executing
          |               |           |
          +-------------> closed <-----+
          +-------------> recovery_required
          +-------------> expired
          +-------------> revoked
          +-------------> superseded
```

Terminal run state, cancellation, expiry, revocation, supersession, stricter
policy, lost authority, or immutable-binding mismatch wins every race.
Operational initial-window opening and the `assessment_required -> executing`
transition are deferred to the later runtime integration phase. The first
conformance implementation receives no production API for opening a window.

### Yield and wait lifecycle

- One yield generation may be registered for one exact attempt and cursor.
- Yield registration requires `window=executing` and `attempt=started`, then
  atomically transitions the attempt to `yielded` and the window to `yielded`.
- Exact replay returns the original registration result.
- Same idempotency identity with different content is a conflict.
- A wait begins unsatisfied and may become satisfied, expired, superseded, or
  canceled only through an authoritative transition.
- Satisfying one wait never implies the other prerequisites are satisfied.
- A wait transition only makes the window eligible for fresh reassessment; it
  never creates or consumes a resume directive.

### Directive lifecycle

```text
available -> consumed
available -> invalidated
available -> expired
```

Exactly one consumer can move `available` to `consumed`. Consumption allocates
the next attempt, records `started`, and moves the private window from
`yielded` to `executing` in the same transaction. A directive cannot return to
`available`.

### Attempt lifecycle

- Attempt numbers are allocated monotonically by the store.
- Caller-supplied numbers are rejected.
- Budget exhaustion is checked in the same transaction as allocation.
- `started` may transition once to `yielded`, `succeeded`,
  `retryable_failure`, `terminal_failure`, or
  `ambiguous_may_have_started`; every one is terminal for that attempt.
- Every terminal attempt outcome requires fresh authority before another
  attempt; no outcome is automatically retryable.

### Outcome transition table

| Attempt terminal posture | Resulting window posture | Fresh reassessment | Reconciliation |
| --- | --- | --- | --- |
| `yielded` | `yielded` | required before consume | no, unless another invariant failed |
| `succeeded` | `closed` | a later runtime phase must open a new window for more work | no |
| `retryable_failure` | `closed` | a new window and fresh authorization are required | no automatic retry |
| `terminal_failure` | `closed` | ineligible | no retry through this window |
| `ambiguous_may_have_started` | `recovery_required` | ineligible | mandatory |

Success does not imply `RunCompleted`. Terminal run state, cancellation, or
revocation overrides every row. A started attempt read outside its original
same-call consumer is exposed as recovery-required even before a separate
event projection exists.

## 6. Transactional API Shape

The first implementation should introduce a separate capability trait, such
as `AuthorizedExecutionContinuityStore`, instead of broadening the aggregate
`StateBackend` contract or exposing a generic transaction closure.

Each operation uses a domain-separated operation ID and a canonical commitment
over every semantic input, expected revision, generated identity, trusted-time
observation, and requested terminal posture. The transaction checks an existing
operation record first: an exact committed ID and commitment returns the
original bounded result even if later state advanced; the same ID with a
different commitment is a conflict; an absent ID proceeds to locked/current
state validation. Storage-level retries reuse the exact IDs, event/receipt
identities, timestamp observation, and commitment.

### Register yield at cursor

Candidate operation:

```text
register_authorized_execution_yield(request)
  -> Registered | ExactReplay
```

The request contains validated IDs and commitments only: run/workflow/step,
window ID and expected revision, exact event cursor, attempt ID, immutable
bundle binding, governance commitment, the private attempt-use capability,
yield disposition, typed wait inputs, and idempotency commitment. It contains
no prompt, transcript, command output, provider payload, source contents, or
serializable authority token.

Inside one transaction the backend must:

1. verify that the transaction family is supported;
2. lock or compare the exact run cursor and window revision;
3. reject terminal run, closed/expired/revoked/superseded window, stale attempt,
   or incompatible immutable binding;
4. require the exact attempt to be `started` and the window to be `executing`;
5. transition the attempt to `yielded`;
6. insert one yield generation and zero or more validated wait records;
7. transition the window to `yielded`;
8. append the matching payload-free continuity receipt/event projection; and
9. return a bounded result identifying the committed generation and revision.

### Transition wait at cursor

Candidate operation:

```text
transition_authorized_execution_wait(request)
  -> Transitioned | ExactReplay
```

The request binds condition ID and expected version, window and expected
revision, exact cursor, target posture, exact wake-trigger class, trusted-time
observation, and domain-separated operation commitment. For satisfaction, it
also carries either an opaque, non-serializable crate-private Core
wake-assessment capability or the identity/version of an exact durable wake
record that the transaction reloads. Inside one transaction the backend
validates the current wait and window, validates the capability binding or
reloads and verifies the durable wake source, persists its commitment and
source revision, transitions the wait to satisfied, expired, superseded, or
canceled, updates the window revision and trusted-time watermark, and appends
the matching receipt/event projection. The store does not attest policy-source
correctness. Exact replay is idempotent; a mismatched trigger, source,
revision, cursor, or commitment fails closed. A satisfied wait permits only
fresh Core reassessment.

### Consume directive at cursor

Candidate operation:

```text
consume_authorized_execution_directive(request)
  -> Consumed { attempt } | ExactReplay
```

The request identifies the run, window, expected window revision, yield
generation, exact cursor, expected wait versions, an opaque crate-private
same-call authority-use input, its persisted assessment commitment, and the
operation commitment. The backend reloads authoritative records; it does not
trust a caller-supplied directive body.

Inside one transaction the backend must:

1. verify run, window, yield, wait, cursor, expiry, and current lifecycle;
2. require all genuine waits to be currently satisfied;
3. validate that the opaque same-call Core input is exactly bound to the
   stored identities and persist its commitment; the store does not attest
   that policy evaluation was fresh or correct;
4. reject terminal/canceled/revoked/superseded state and exhausted budget;
5. atomically consume the directive;
6. allocate the next attempt identity and number;
7. persist the attempt as `started`;
8. transition the private window from `yielded` to `executing`; and
9. append the matching payload-free attempt-started/resume projection.

The winning result includes a private, non-cloneable, non-serializable
attempt-use capability bound to the exact attempt, subject, window revision,
cursor, authority commitment, and consume-operation identity. No executor or
host callback occurs inside this transaction.

### Reconcile attempt outcome

Candidate operation:

```text
record_authorized_execution_attempt_outcome(request)
  -> Recorded | ExactReplay
```

The normal operation requires the exact private attempt-use capability, checks
the `started` attempt and window revision, records one succeeded,
retryable-failure, or terminal-failure posture, applies the outcome transition
table, updates the trusted-time watermark, and appends the matching
event/receipt projection atomically. Yield is recorded only through the yield
registration operation using the same private capability. Conflicting outcome
replay fails closed. A host crash after executor entry but before this commit
leaves `started`, which is itself recovery-required to every other reader and
never inferred as success or safe retry.

A separate recovery operation may atomically change an orphaned `started`
attempt only to `ambiguous_may_have_started`. It cannot record success,
retryable failure, terminal failure, or yield. The recovery operation has its
own domain-separated identity and commitment and must preserve the same exact
run, cursor, window, attempt, trusted-time, receipt/event, and replay rules.

## 7. Conflict And Error Semantics

The API must distinguish, using stable non-leaking codes:

- invalid input;
- unsupported transaction capability;
- exact idempotent replay;
- conflicting replay;
- stale event cursor;
- stale window or wait revision;
- run terminal or canceled;
- window closed, expired, revoked, or superseded;
- unsatisfied wait or prerequisite;
- authority assessment stale or unavailable;
- wake assessment stale, unavailable, or binding-mismatched;
- attempt budget exhausted;
- directive already consumed;
- attempt outcome already recorded;
- retryable storage conflict;
- backend unavailable;
- incompatible schema or corrupt state;
- recovery required after ambiguous start.

Errors must not echo IDs, repository paths, refs, payloads, tokens, evidence,
approval reasons, command output, or provider data.

## 8. Atomicity And Crash Matrix

| Boundary | Required durable posture |
| --- | --- |
| Before yield transaction | no continuity changes |
| During failed yield transaction | no yield, wait, window, or event changes |
| After yield commit | yield/waits, window revision, and event all visible |
| Before directive consume | directive remains available |
| During failed consume | no consumed marker, attempt, window, or event changes |
| After consume commit, before executor entry | directive consumed; attempt is `started`; no retry |
| Executor returns, before outcome commit | attempt remains `started`; restart classifies ambiguity |
| Outcome transaction fails | previous durable posture remains; operator reconciliation required |
| After outcome commit | one terminal attempt outcome and event are visible |
| Host response delivery fails | durable workflow posture is unchanged and re-readable |

The implementation must not claim exactly-once external execution. It provides
exactly-one durable directive winner and explicit ambiguity after consumer
entry.

## 9. Authority Composition

The state store coordinates durable mutation; it does not decide policy,
attest freshness, or grant authority. Before directive consumption, the
integrated Core boundary
must resolve current run state, immutable binding, policy, required context,
approval/evidence/check readiness, capability status, expiry, and revocation
from trusted sources. That private same-call Core boundary produces an opaque,
non-serializable authority-use input. The transaction validates its exact
identity binding and persists the resulting assessment commitment; reference
conformance tests binding, not policy-source correctness.

External authority sources may still change after assessment. The first
implementation must document this residual TOCTOU boundary and minimize it by
placing fresh resolution immediately before the atomic consume call. Host or
executor identity cannot substitute for the authorized subject. Serialized
orientation, final responses, and delivery acknowledgements never grant
authority.

## 10. Versioned Capability And Backend Matrix

Continuity uses a dedicated
`AuthorizedExecutionContinuityStateContractVersion::V1` and explicit
per-operation support declarations. It does not silently expand
`DurableStateContractVersion::V1` or add new members to its existing transaction
set. If later unified under a general durable-state contract, that requires a
new contract version with explicit compatibility review.

| Backend | Initial continuity posture | Required proof before support |
| --- | --- | --- |
| In-memory reference store | test-only supported | full deterministic concurrency, replay, crash, and recovery conformance |
| Local filesystem preview | unsupported | no multi-file atomicity emulation; a separately designed single-record transaction format would be required |
| Embedded SQLite | unsupported initially | schema migration plus real transactional implementation and dedicated conformance |
| Shared PostgreSQL | unsupported initially | explicit transaction implementation, constraints/indexes, serializable-conflict tests, and dedicated conformance |

In implementation step 1, every backend must explicitly implement the new
continuity capability provider and report all continuity operations
unsupported. PostgreSQL or another backend must not inherit support by
iterating a generic `all()` list. Unknown or newly added continuity operations
default to unsupported until implementation and conformance are present.

## 11. Schema And Compatibility Posture

The first model/conformance implementation should add separately versioned
continuity-operation support vocabulary without changing the existing durable
state V1 transaction set, run-status variants, or workflow schema wire shapes.
Backend schemas remain unchanged until a backend-specific implementation phase
is approved.

Future SQLite/PostgreSQL schemas require:

- unique run/window/generation/attempt identities;
- revision columns and compare-and-set predicates;
- one active window constraint for an exact run/step/scope generation;
- one yield per attempt;
- one directive winner per window/cursor/yield generation;
- monotonic attempt-number allocation;
- unique event projection linkage;
- migration checksums, interruption recovery, rollback posture, and old-reader
  behavior.

Contract-version changes must fail closed for old adapters. Backup/restore and
rehydration must preserve the same current disposition.

## 12. Event And Projection Boundary

This phase defines the atomic relation to future continuity events but should
not add runtime event variants until the state contract and conformance model
are accepted. The first implementation may use private test projections or
payload-free operation receipts to prove atomic write-set semantics.

When event vocabulary is added in the next phase, every continuity state
transition and its event must share one transaction. Rehydration derives the
same disposition after restart. Snapshot repair may follow the event and
continuity records but cannot independently open a window, satisfy a wait,
consume a directive, or authorize an attempt.

## 13. Trusted Time

Expiry is evaluated by the authoritative operation boundary, not by an
untrusted host timestamp. Every authoritative window stores a trusted-time
watermark updated in each mutation. PostgreSQL should use database time.
SQLite and the in-memory reference implementation should use a Core-owned
injected clock whose observation is captured in the operation commitment.
Every operation requires `observed_now >= watermark`; equality at expiry
closes eligibility. Clock rollback, unavailable time, or incompatible time
provenance fails closed and cannot extend a window.

## 14. Privacy, Redaction, And Retention

Continuity storage is limited to validated IDs, hashes, revisions, enums,
bounded timestamps, and bounded stable references. Stable references are still
sensitive metadata and must not contain paths, prompts, transcripts, source
contents, command output, provider payloads, environment values, credentials,
authorization material, private keys, or token-like values.

Requirements include:

- bounded collection counts and serialized byte limits before decoding;
- redaction-safe `Debug`, serde errors, backend errors, conflict errors, and
  host-delivery errors;
- no SQL values or filesystem paths in user-facing errors;
- explicit sensitivity and retention posture;
- access control inherited from the state backend;
- payload-free audit references rather than duplicated evidence;
- deletion/retention behavior documented before hosted use.

## 15. Supervisor Boundary

Core may return a typed current disposition after durable reconciliation. The
host owns executor selection, process lifecycle, wake delivery, and invocation.
The host cannot mark a workflow complete, create authority, satisfy evidence,
or turn a delivery acknowledgement into a continuation claim.

A final assistant response while a run remains runnable is not a terminal
workflow event. The later supervisor slice must either schedule the next
executor, register a genuine wait, or preserve a yielded runnable posture.

## 16. Test Plan

The dedicated conformance suite must cover:

- two concurrent yield registrations at one cursor;
- exact replay and same-key/different-content conflict;
- yield racing cursor advance, completion, cancellation, window close,
  expiry, revocation, and supersession;
- two concurrent directive consumers with exactly one durable attempt;
- a non-winning or reconstructed caller cannot record an ordinary attempt
  outcome without the private attempt-use capability;
- recovery without that capability can record only ambiguity;
- wait satisfaction racing another wake, supersession, expiry, and revocation;
- stale or mismatched wake capability/source rejection;
- exact and conflicting wait-transition replay;
- attempt-budget allocation under contention;
- current-authority or policy change before assessment, before consume, after
  consume, and before executor entry;
- fault injection before, during, and after every transaction commit;
- restart recovery from every attempt posture;
- `started` without outcome becoming ambiguous;
- `started` being recovery-required to every reader other than the original
  same-call consumer;
- conflicting and exact outcome replay;
- unsupported filesystem and pre-support SQLite/PostgreSQL paths producing
  zero writes;
- SQLite conformance after its later capability implementation;
- PostgreSQL serializable-conflict retry without duplicate event, directive,
  or attempt;
- migration interruption, rollback, old-reader, backup, and restore behavior;
- oversized input, malicious identifiers, database decode, logging, and error
  non-leakage;
- deterministic rehydration after process restart;
- final response or host delivery never appending `RunCompleted`;
- trusted-time equality at expiry, rollback, and concurrent observations;
- all existing state, executor, approval, continuation, capability, report,
  adapter, and hosted tests remaining unchanged.

## 17. Candidate Implementation Sequence

1. Add the separately versioned continuity capability contract, explicit
   unsupported declarations for every existing backend, request/result models,
   authoritative record state machines, domain-separated replay commitments,
   stable errors, and a separate `AuthorizedExecutionContinuityStore` trait.
2. Add a test-only fixture bootstrap API unavailable through the production
   trait. Every consume-path fixture begins from an exact `yielded` window with
   one prior terminal `yielded` attempt, one active yield generation, exact
   wait records, run cursor, record revisions, operation history, and
   trusted-time watermark. The bootstrap is solely for conformance and is not
   evidence that a legal runtime window opened. Operational initial-window
   opening and `assessment_required -> executing` remain deferred.
3. Add a test-only in-memory reference store and dedicated executable
   conformance harness covering register, consume/start, outcome, replay,
   wait transition, trusted time, contention, unsupported posture, and crash
   recovery.
4. Review the contract and conformance implementation before any backend
   advertises support.
5. Implement SQLite schema and transactions as the first local durable backend;
   keep local filesystem unsupported.
6. Run the same conformance and crash suite against SQLite, then review.
7. Implement PostgreSQL schema and serializable transactions only after the
   local proof is accepted.
8. Add runtime event vocabulary and projection only after an accepted backend
   can commit records and events atomically.
9. Add one explicit injected-supervisor vertical slice after event integration.

Implementation must not stop at declarations. The first accepted milestone is
the capability contract plus executable reference conformance; the first
durable milestone is SQLite passing that same suite.

## 18. Open Questions

- Should the reference conformance store live in production code behind a
  private constructor or only in integration tests?
- Which exact private event receipts best prove atomicity before public event
  vocabulary exists?
- How should SQLite obtain trusted monotonic time across machine clock rollback?
- Is one active window constraint per run/step/action scope sufficient, or is a
  separately versioned scope generation required?
- Which authority assessment commitment source is first eligible for the local
  supervisor proof?

## 19. Final Recommendation

Proceed with the **Authorized Execution Continuity atomic state contract and
reference conformance implementation only**.

The implementation should add the separately versioned capability contract,
separate capability trait, authoritative record/request/result model, wait
transition and attempt lifecycle, domain-separated replay semantics, explicit
unsupported declarations for all existing backends, a test-only fixture
bootstrap, and an executable in-memory conformance suite. It should not yet
alter SQLite/PostgreSQL schemas, add runtime events, invoke an executor,
schedule a host, approve a gate, mutate a provider, expose CLI behavior, or
broaden nested harness execution.
