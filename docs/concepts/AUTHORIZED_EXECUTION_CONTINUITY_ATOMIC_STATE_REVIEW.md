# Authorized Execution Continuity Atomic State Review

## 1. Executive Verdict

**Phase accepted; proceed to the first durable continuity backend.**

The implementation satisfies the approved atomic-state and reference-
conformance scope. It defines a separately versioned capability contract,
keeps authoritative state and bearer capabilities private, proves one-winner
and exact-replay semantics, fails closed around trusted time and ambiguous
attempts, and explicitly prevents every current durable backend from
overclaiming support.

This verdict does not accept operational continuation. No durable backend can
yet register a yield, transition a wait, consume a directive, record an
attempt outcome, or recover an ambiguous attempt. No supervisor can redispatch
an external executor.

## 2. Scope Verification

The phase stayed within the accepted reference-contract boundary. It added:

- one public, versioned continuity-support contract;
- private authoritative window, yield, wait, directive, attempt, operation,
  receipt, cursor, and revision state;
- private one-use authority, wake, and attempt capabilities;
- a test-only fixture and transactional reference store;
- explicit unsupported declarations for local filesystem, SQLite, and
  PostgreSQL; and
- focused public and private conformance tests plus phase documentation.

It did not add durable schemas, runtime events, run-status changes, executor or
supervisor behavior, approval automation, provider calls, external writes,
CLI behavior, workflow-schema fields, SDK changes, examples, nested execution,
or release-posture changes.

## 3. Contract And Compatibility Assessment

`AuthorizedExecutionContinuityStateContractVersion::V1` is independent of the
existing durable-state contract. The contract requires one declaration for
each supported operation family and rejects missing, duplicate, and unknown
wire values. Support inspection is side-effect free; in particular,
PostgreSQL support inspection does not connect to a database.

All current backends declare every continuity operation unsupported. This is
the correct compatibility posture. Generic backend transaction support cannot
silently imply continuity support.

## 4. Authoritative State Assessment

The reference implementation binds continuity state to exact workflow, run,
step, subject, immutable bundle, governance commitment, authority commitment,
event cursor, attempt budget, expiry, revision, and trusted-time watermark.
Serialized public orientation values cannot grant execution authority.

Authority-use, wake-assessment, and attempt-use capabilities are private,
non-cloneable, non-serializable, and redaction-safe. A caller cannot recreate
them from a persisted model or use an ordinary final response as proof that a
run completed.

## 5. Atomicity And Replay Assessment

The five operation families are explicit:

1. register yield;
2. transition wait;
3. consume directive;
4. record attempt outcome; and
5. recover ambiguous attempt.

Each operation uses a domain-separated operation identity and canonical
commitment over its semantic bindings. Operation history is checked before
current-state validation. Exact committed replay returns the original bounded
result; conflicting replay fails closed. Exact directive-consumption replay
does not return another attempt capability.

True concurrent consumers and yield registrants produce one durable winner.
Fault injection before and during commit leaves no write; failure after commit
is reconciled by exact replay for every operation family. Receipt identity is
globally unique across operations and cannot be rebound.

These properties prove at-most-one durable attempt start. They do not prove
exactly-once external execution.

## 6. Attempt, Yield, And Wait Assessment

Directive consumption atomically allocates and records a started attempt
before returning its private attempt-use capability. Yield registration uses
that capability to close the attempt as yielded and return the window to a
non-terminal yielded posture. A started attempt observed without its live
capability may only become ambiguous and recovery-required.

Wait satisfaction requires a private wake capability bound to the exact
window, active yield generation, condition, condition version, trigger, source
commitment, and source revision. Stale pre-wake authority is rejected after a
wait revision. Satisfying a wait removes one prerequisite only; it does not
authorize execution or consume a directive.

The first slice intentionally supports only `unsatisfied -> satisfied`.
Expiry, cancellation, and supersession need their own later authority paths.

## 7. Trusted-Time Assessment

Mutation time comes from a store-owned injected clock, not an operation
request. Source kind, provenance commitment, and observation are committed to
the operation and receipt. Exact replay reuses the committed observation
rather than consulting a newer clock value.

Clock unavailability, incompatible provenance, regression, and equality at
expiry fail closed without mutation. The reference clock now stores its
observation and provenance in one coherent mutex-protected state, eliminating
the torn-observation concern raised during security review.

SQLite still needs an explicit monotonic trusted-time design before it can
implement this contract.

## 8. Privacy And Error Assessment

The state and wire models contain bounded identifiers, commitments, enums,
revisions, timestamps, and stable references only. They contain no prompts,
transcripts, source contents, parser payloads, command output, provider
payloads, environment values, credentials, authorization headers, private
keys, or serializable bearer authority.

Validation and state errors use stable codes and generic messages. Secret-like
identifiers are rejected without echoing their values. Capability and state
`Debug` output is redacted, and unknown serialized variants fail closed.

## 9. Test Quality Assessment

Focused private coverage proves:

- concurrent one-winner directive consumption and yield registration;
- exact replay and conflicting replay behavior;
- one-use capability behavior and capability-free ambiguity recovery;
- attempt-budget exhaustion and restart rehydration;
- exact wake binding and stale-wake rejection;
- trusted-time failure, regression, and expiry behavior;
- receipt uniqueness and commitment integrity; and
- before-, during-, and after-commit fault posture for all five operations.

Public tests prove complete contract shape, validated serde, explicit
unsupported posture for all current backends, and side-effect-free PostgreSQL
support inspection. The full workspace, Clippy, formatting, Node/SDK/docs,
integration-contract, and diff checks pass.

The remaining test-architecture weakness is non-blocking for this phase: the
transaction conformance suite is coupled to the private reference store. It
must be extracted into a backend-parametric harness before SQLite advertises
support.

## 10. Documentation Assessment

The roadmap, accepted plan, continuation-context plan, and implementation
report consistently state that:

- atomic reference semantics are implemented;
- all production backends remain unsupported;
- runtime events and automatic continuation are not implemented;
- an executor turn ending is not workflow completion;
- a future host supervisor may dispatch from authoritative directives; and
- provider and nested-runtime broadening remain outside this phase.

No current capability is overstated.

## 11. Blockers

No blocker remains for accepting this reference implementation.

The following are blockers for operational continuity and must not be treated
as non-blocking product polish:

- no durable backend implements the contract;
- no atomic continuity event/state projection exists;
- no runtime path opens an authoritative execution window; and
- no host supervisor consumes durable directives and redispatches an external
  executor.

## 12. Non-Blocking Follow-Ups

- Extract a backend-parametric conformance harness before backend support is
  advertised.
- Resolve SQLite monotonic trusted-time semantics.
- Preserve the local filesystem backend as unsupported.
- Keep exact external-effect reconciliation separate from continuity-state
  idempotency.

## 13. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786814439804417000-2`;
- approval: `approval/run-1786814439804417000-2/review-scope-approved`;
- presentation: `presentation/3ef72135b2dcb444`;
- presentation hash:
  `3ef72135b2dcb4447906f46fe60cc09d3b824a53578ba80304884991bce29606`;
- approval outcome: granted by delegated maintainer after the complete handoff;
- phase status: completed and inspected through phase close;
- event summary: 39 events, including one approval request, one proof-enforced
  approval grant, six scheduled steps, six successful skill invocations, no
  retries, and no escalations; and
- out-of-kernel work: source inspection, review authoring, validation, and git
  operations were performed by the external executor under the governed
  scope. The kernel did not edit files, execute checks, commit, push, open or
  merge a pull request, or simulate durable backend support.

## 14. Validation

- focused private conformance tests: 14 passed;
- public support-contract tests: 3 passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check`: passed;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed;
- independent focused security review: accepted after blocker fixes.

## 15. Recommended Next Phase

Implement the **SQLite authorized-execution continuity backend and reusable
conformance harness**.

That phase should add the first durable schema and atomic transactions for the
five accepted operation families, use explicit trusted-time and crash
semantics, and earn backend support only by passing the extracted conformance
suite. It must not yet add runtime events, an executor, a scheduler, automatic
approval, provider writes, CLI behavior, workflow-schema exposure, or nested
execution.

After that backend is accepted, implement atomic event/state projection and
then one local injected-supervisor vertical slice. That is the shortest safe
path from durable lawful work to automatic executor redispatch without
confusing an agent turn boundary with workflow completion.
