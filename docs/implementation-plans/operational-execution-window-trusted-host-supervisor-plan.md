# Operational Execution Window And Trusted-Host Supervisor Plan

## 1. Executive Summary

Workflow OS can now preserve yielded authorized work durably, derive a current
continuation disposition, consume one resume directive atomically, start one
attempt, record its outcome, and project those operations into the runtime
event stream and snapshot. It cannot yet open the initial operational
execution window, and no trusted host consumes the resulting capability to
invoke work.

This plan closes that gap with one local, opt-in vertical slice. The first code
phase adds an atomic SQLite operation that opens one exact execution window and
starts its first attempt from current authority, immutable run context, and a
current runtime cursor. The second code phase adds an injected one-shot host
supervisor that invokes one existing local skill operation with the returned
private attempt capability and reports the bounded outcome back to Core.

The supervisor is an execution host, not a governance authority. Core alone
decides legality, current authority, waits, blockers, retry eligibility, and
terminal posture. This plan does not implement a scheduler, polling daemon,
model-turn creation, automatic approval, provider mutation, nested harness
execution, hosted execution, CLI behavior, or general-purpose orchestration.

## 2. Goals

- Open one operational execution window only after current source-backed
  authority, immutable run context, policy, gate readiness, and cursor binding
  have been validated.
- Allocate and durably start the first attempt in the same transaction as the
  window opening and runtime event/snapshot projection.
- Give an injected trusted host only the private, one-use attempt capability
  needed to invoke one exact local operation.
- Record success, retryable failure, terminal failure, yield, or ambiguous
  may-have-started posture without fabricating workflow completion.
- Reconcile every ambiguous commit acknowledgement before dispatch or
  redispatch.
- Demonstrate one turn-boundary yield and one lawful resume through the
  existing directive-consumption path.
- Preserve existing executor semantics and keep the slice local, opt-in, and
  unavailable on unsupported state backends.

## 3. Non-Goals

This phase does not add:

- a generic scheduler, daemon, queue, polling loop, or hosted supervisor;
- a claim that Workflow OS can create or force a model conversation turn;
- automatic approval, delegated self-approval, or evidence bypass;
- ambient authority, reconstructed bearer tokens, or reusable capabilities;
- provider writes, additional mutation families, live adapters, or OpenShell
  execution;
- nested harness execution, recursive agents, or agent swarms;
- workflow schema, runtime configuration, CLI, SDK, UI, or example changes;
- filesystem or PostgreSQL continuity support;
- persistence of prompts, transcripts, source, command output, or provider
  payloads;
- Reasoning Lineage, enterprise administration, or release-posture changes.

## 4. Existing Boundary

The accepted continuity implementation already provides:

- validated non-authoritative execution-window vocabulary;
- immutable run-bundle, governance, authority, actor, step, and cursor
  bindings;
- atomic yield registration, wait transition, resume-directive consumption,
  attempt outcome, and ambiguous-attempt recovery;
- semantic V2 target-owner, trusted-time, attempt-budget, and replay checks;
- SQLite semantic persistence and migration;
- atomic continuity operation, receipt, runtime event, and snapshot
  projection; and
- fresh-connection reconciliation of ambiguous projected commits.

The production store intentionally has no initial-window opening operation.
Test fixtures bootstrap yielded windows, but that bootstrap is not evidence
that runtime authority lawfully opened a window. `consume_directive` therefore
proves resume behavior only after a yielded state already exists.

## 5. Source-Of-Truth Boundaries

- **Current authority source:** proves the actor may attempt the exact action
  now. It does not schedule or invoke work.
- **Immutable run bundle:** binds workflow, step, policy, evidence, check, and
  spec identity. It is not refreshed from mutable project files during the
  operation.
- **Continuity state:** owns execution-window, attempt, yield, wait, directive,
  replay, and current-disposition authority.
- **Runtime events:** provide bounded ordered operational history. They cannot
  authorize an attempt.
- **Runtime snapshot:** is a deterministic event projection. It cannot replace
  current continuity or authority state.
- **Trusted-host supervisor:** consumes a private attempt capability and
  invokes an injected executor. It cannot decide legality or completion.
- **Injected executor:** performs one bounded operation and returns a typed
  result. It cannot mutate continuity state directly.

## 6. Initial Window Opening Contract

Add one private operation, tentatively named
`open_window_and_start_attempt_projected`. The exact repository-idiomatic name
may differ, but it must remain a single atomic capability boundary.

The request should contain only validated bounded values and commitments:

- operation and receipt IDs plus a domain-separated request commitment;
- workflow, run, step, window, and generated attempt IDs;
- expected runtime cursor and snapshot commitment;
- immutable run-bundle binding;
- exact current-authority capability and authority commitment;
- current gate-readiness and policy commitments;
- subject actor and governed action/resource scope commitments;
- sensitivity ceiling, expiry, trusted-time epoch, and maximum attempts; and
- the expected absence or exact prior assessment posture required by the
  accepted state model.

The transaction must:

1. reconcile the run and current cursor from durable state;
2. verify the run is non-terminal and the target step is currently eligible;
3. validate immutable-bundle, actor, scope, authority, gate, policy, expiry,
   trusted-time, and attempt-budget bindings;
4. reject an existing conflicting active window or attempt;
5. create the window directly in `executing` posture;
6. allocate attempt number one and create the attempt in `started` posture;
7. persist the operation and receipt;
8. append one bounded runtime projection event and derive the snapshot; and
9. return a private non-serializable attempt-use capability only after durable
   commit is known.

An intermediate `assessment_required` record may be retained only if Core can
commit and advance it without exposing a schedulable gap. The first production
API must not leave a window in an externally dispatchable half-open posture.

Exact replay returns the original result without another event or attempt.
Same operation identity with different content fails closed. A commit
acknowledgement failure returns ambiguity and no attempt capability; the host
must reconcile on a fresh connection before deciding whether dispatch is
legal.

## 7. Opening Event And Snapshot Projection

Add one bounded projection event for the accepted opening operation. It should
disclose only stable IDs, operation kind, result posture, attempt number,
window revision, cursor, and commitments already allowed by the continuity
projection privacy boundary.

The event must not include:

- prompts, transcripts, source text, spec bodies, or command output;
- authority tokens or reconstructable capabilities;
- raw evidence, approval, policy, or check payloads; or
- filesystem paths, environment values, credentials, or provider data.

The snapshot projection may disclose that one window/attempt is executing, but
continuity state remains authoritative. A snapshot indicating execution while
the continuity records are absent or conflicting is corruption.

## 8. Trusted-Host Supervisor Contract

Add a small injected interface, tentatively:

```text
TrustedHostAttemptExecutor::execute(AttemptExecutionContext)
    -> AttemptExecutionResult
```

The context should expose only:

- the private attempt-use capability;
- immutable workflow/run/step identity;
- one approved local action identifier;
- bounded, already-validated invocation input references;
- redaction and sensitivity posture; and
- correlation and trace references needed for existing local execution.

The result should be one of:

- succeeded;
- retryable failure;
- terminal failure;
- yielded with a bounded yield reason and optional typed waits; or
- ambiguous may have started.

The supervisor must:

1. request opening or consume a current resume directive through Core;
2. refuse dispatch unless it receives the private attempt capability from the
   successful atomic operation;
3. invoke the injected executor once;
4. report the exact result through the accepted projected outcome or yield
   operation;
5. reconcile any ambiguous persistence response before another action; and
6. return Core's current disposition after durable reconciliation.

The supervisor must not loop automatically in the first slice. One call owns
one dispatch and one durable result. A later scheduler may repeatedly call the
one-shot boundary only after separate review.

## 9. First Local Vertical Slice

The first proof should use an injected test executor around the existing local
skill invocation boundary. It should exercise one deterministic local handler
without a `StateBackend` write from the handler itself and without provider
access.

The end-to-end proof should show:

1. a validated non-terminal run and eligible current step;
2. current source-backed authority and immutable context;
3. atomic window opening plus first-attempt start;
4. one local injected invocation;
5. a turn-boundary yield while lawful work remains;
6. a derived `ResumeNow` directive;
7. atomic directive consumption plus second-attempt start;
8. one resumed injected invocation;
9. durable success or bounded failure; and
10. a current disposition derived from reconciled state.

Only an existing real terminal runtime event may make the workflow terminal.
Supervisor success, assistant final text, delivery acknowledgement, or absence
of another callback must not append `RunCompleted` or equivalent state.

## 10. Yield, Wait, And Resume Semantics

- An ordinary executor turn ending records `turn_boundary`; it is not a wait.
- Genuine dependency waits must use typed conditions and remain
  `AwaitCondition` until every exact current condition is satisfied.
- A yielded runnable window may produce `ResumeNow`, but that disposition is
  not itself bearer authority.
- Resume requires a fresh source-backed authority capability and one current
  directive bound to the exact cursor, yield generation, waits, and window
  revision.
- A consumed directive is one-use. A second consumer cannot dispatch.
- Satisfied waits trigger fresh assessment; they do not revive stale
  authority or stale policy.
- Expiry, revocation, supersession, cancellation, terminal run state, stricter
  policy, evidence/check invalidation, or immutable-context mismatch wins the
  race and blocks dispatch.

## 11. Failure And Reconciliation

- **Before opening commit:** no window, attempt, event, snapshot advancement,
  receipt, or capability exists.
- **Ambiguous opening commit:** return no capability; reconcile the complete
  projected operation on a fresh SQLite connection.
- **Committed opening, host crash before invocation:** the `started` attempt is
  ambiguous and must enter recovery; it is never silently retried.
- **Invocation returns but outcome commit is ambiguous:** reconcile before
  another dispatch.
- **Supervisor delivery failure before executor entry:** preserve a bounded
  delivery failure distinct from workflow failure; do not claim the attempt
  did not start unless that fact is provable.
- **Missing or unsupported backend:** fail before any state or event mutation.
- **Corrupt partial projection:** fail closed with the existing stable
  continuity corruption posture.
- **No scheduler available:** preserve current non-terminal disposition for
  inspection; do not fabricate a wait or terminal result.

## 12. Concurrency And Idempotency

- At most one active operational window may own the exact run/step/action
  scope in this slice.
- Competing openers at the same cursor produce one durable winner.
- Operation IDs and request commitments provide exact replay and conflict
  detection.
- Attempt numbers are store-allocated monotonically.
- A private attempt capability cannot be cloned, serialized, reconstructed, or
  reused for another attempt.
- Generic event append and opening projection must contend for the same next
  cursor; no event may be skipped or overwritten.
- Supervisor retries are delivery retries only. They never imply safe
  operation retry after `started`.

## 13. Backend And Compatibility Posture

SQLite semantic V2 with accepted atomic runtime projection is the only backend
eligible for this proof. Filesystem and PostgreSQL return the established
unsupported capability error before mutation. Existing executor constructors
and behavior remain unchanged unless the new supervisor path is explicitly
selected.

No public workflow spec or wire schema is added. New private SQLite schema or
migration work is allowed only if the opening operation cannot be represented
honestly in the accepted continuity tables. Any migration must preserve old
reader refusal, backup/restore, interruption, and rollback tests.

## 14. Privacy And Redaction

The opening request, persisted records, supervisor context, events, snapshots,
errors, Debug output, and reconciliation results may contain bounded IDs,
enums, revisions, counts, timestamps, hashes, and sensitivity/redaction
posture. They must not contain raw:

- prompts, model transcripts, hidden reasoning, source, or spec contents;
- command, CI, parser, tool, provider, or sandbox payloads;
- environment values, credentials, authorization headers, private keys, or
  token-like values;
- approval reasons, evidence bodies, arbitrary handoff prose, or operator
  notes; or
- filesystem paths or reconstructable execution authority.

Stable errors must identify the failed boundary without echoing caller input.

## 15. Test Plan

Future implementation tests must cover:

- valid atomic opening and first-attempt start;
- unmet evidence/check, non-presentable gate, stale policy, missing authority,
  actor mismatch, scope mismatch, immutable-bundle mismatch, expiry,
  revocation, terminal run, and stale cursor rejection;
- exact replay and same-key/different-content conflict;
- two concurrent openers with exactly one winner;
- generic-event/opening cursor contention;
- before/during/after opening-commit fault injection and fresh-connection
  reconciliation;
- no attempt capability on ambiguous or rejected opening;
- crash after `started` and before invocation producing recovery posture;
- injected success, retryable failure, terminal failure, yield, and ambiguous
  outcomes;
- turn-boundary yield remaining runnable rather than waiting;
- typed wait remaining non-runnable until exact satisfaction and fresh
  reassessment;
- one lawful resume through existing directive consumption;
- duplicate, stale, expired, revoked, and superseded resume rejection;
- only one executor invocation per supervisor call;
- supervisor callback cannot append terminal workflow state directly;
- final assistant response cannot terminate the run;
- missing supervisor preserves inspectable non-terminal posture;
- SQLite support and filesystem/PostgreSQL zero-write unsupported behavior;
- non-leaking Debug, serde, persistence, projection, and error paths; and
- all existing executor, approval, continuity, report, side-effect, adapter,
  hosted, migration, and runtime tests remaining green.

## 16. Candidate Implementation Sequence

1. Add the private opening request/result model, commitment function, stable
   errors, store capability method, and unsupported backend declarations.
2. Add reference conformance for atomic opening, replay, concurrency, trusted
   time, attempt allocation, and fault posture.
3. Implement SQLite opening plus runtime event/snapshot projection and
   fresh-connection reconciliation.
4. Perform focused maintainer/security review of operational opening before
   adding host invocation.
5. Add the one-shot injected trusted-host supervisor interface and one local
   skill invocation adapter.
6. Prove first dispatch, turn-boundary yield, `ResumeNow`, one directive-bound
   resume, bounded outcome, and no false completion.
7. Perform focused end-to-end maintainer/security review.
8. Only after acceptance, consider repeated scheduling, additional local
   operations, CLI orientation, OpenShell execution, provider mutations, or
   nested harnesses as separately planned phases.

## 17. Open Questions

- Should initial opening create a separately persisted assessment record, or
  is its commitment inside the atomic operation sufficient for the first
  proof?
- Which existing source-backed authority producer is narrowest and strongest
  enough for the first local operation?
- Should host delivery failure before callback entry have its own attempt
  outcome, or always use conservative ambiguous recovery?
- Can the existing runtime event projection encode opening without a SQLite
  schema revision, or is a new private operation row variant required?
- Should the first injected operation be a no-side-effect test skill or one
  existing deterministic read-only local check handler?
- What bounded orientation context is necessary for a future model executor
  without storing prompts or transcripts?

## 18. Final Recommendation

Implement **operational execution-window opening first**, including its store
contract, reference conformance, SQLite transaction, event/snapshot
projection, reconciliation, and focused security review. Then implement the
one-shot injected local trusted-host supervisor over that accepted capability.

Do not combine opening and host dispatch in one unreviewed code phase. The
host must never become the component that decides or reconstructs execution
authority.
