# Authorized Execution Continuity Atomic State Report

## 1. Executive Summary

Workflow OS now has the first executable atomic-state boundary for authorized
execution continuity. The implementation adds a separately versioned support
contract, a private authoritative state and operation model, explicit
unsupported declarations for every current durable backend, and a test-only
in-memory reference store that proves the accepted transaction semantics.

This phase does not make continuation operational. It does not persist
continuity state in a production backend, append runtime events, invoke or
schedule an executor, approve a gate, resume a model turn, mutate a provider,
or expose CLI or workflow-schema behavior.

## 2. Scope Completed

- Added `AuthorizedExecutionContinuityStateContractVersion::V1` without
  changing `DurableStateContractVersion::V1`.
- Added exhaustive support declarations for yield registration, wait
  transition, directive consumption, attempt-outcome recording, and ambiguous
  attempt recovery.
- Added explicit unsupported declarations for the local filesystem, SQLite,
  and PostgreSQL backends.
- Added private authoritative window, yield, wait, directive, attempt,
  operation, receipt, revision, and cursor models.
- Added private non-cloneable, non-serializable authority-use,
  wake-assessment, and attempt-use capabilities.
- Added domain-separated canonical request and committed-operation
  commitments.
- Added a store-owned injected trusted-clock boundary; callers cannot supply
  operation timestamps or provenance through continuity requests.
- Added globally unique receipt identity across operation records.
- Added a private `AuthorizedExecutionContinuityStore` operation boundary.
- Added a test-only exact yielded-state fixture and in-memory transactional
  reference implementation.
- Added executable public-contract and private transaction-conformance tests.

## 3. Scope Explicitly Not Completed

This phase does not add:

- local filesystem, SQLite, or PostgreSQL continuity transactions;
- durable continuity schemas or migrations;
- runtime continuity events or snapshot projection;
- initial execution-window opening;
- executor, scheduler, polling, host-supervisor, or automatic-resume behavior;
- approval decisions, delegated approval, or policy inference;
- shell, git, browser, connector, provider, or external-write execution;
- workflow schema, SDK, CLI, example, or release-posture changes;
- nested harness execution or hosted/distributed operation;
- exactly-once external-effect claims or blind retry.

## 4. Capability Contract And Backend Posture

The public compatibility surface identifies five operation families:

- register yield;
- transition wait;
- consume directive;
- record attempt outcome;
- recover ambiguous attempt.

Contract construction requires exactly one declaration for every operation
and rejects missing, duplicate, or unknown wire values. The local filesystem,
SQLite, and PostgreSQL providers each declare every operation unsupported.
Reading PostgreSQL support posture does not open a database connection.

The contract is separate from the existing durable-state V1 contract. A
backend may not inherit continuity support from a generic operation list or
advertise support before its dedicated implementation passes the same
conformance boundary.

## 5. Authoritative State And Operation Boundary

The private reference model binds one revisioned execution window to the exact
workflow, run, step, subject, immutable run bundle, governance commitment,
authority commitment, event cursor, attempt budget, expiry, and trusted-time
watermark.

The transaction boundary supports:

- registering one yield against a started attempt and exact cursor;
- transitioning one exact wait using a bound wake-assessment capability;
- consuming one available directive and durably creating a started attempt;
- recording one normal attempt outcome using the exact private attempt-use
  capability; and
- recovering an orphaned started attempt only as ambiguous.

The fixture bootstrap begins from one exact yielded state. It is unavailable
through the production trait and is not evidence that a legal runtime window
was opened.

## 6. Replay, Concurrency, And Crash Posture

Every operation is identified by a domain-separated operation ID and a
canonical commitment over its semantic inputs. Operation history is checked
before current-state validation:

- exact committed replay returns the original bounded result;
- a reused operation ID with different content fails closed;
- exact directive-consumption replay never returns another attempt-use
  capability;
- competing directive consumers produce exactly one durable attempt;
- competing yield registrations produce exactly one durable yield generation;
- a reused operation identity with different content, or a reused receipt
  identity under a different operation, fails closed without changing state;
- injected failure before or during commit writes nothing for all five
  operations;
- injected failure after commit returns an error but an exact retry observes
  the committed result for all five operations;
- a started attempt without a same-call private capability can only be moved
  to `ambiguous_may_have_started` and `recovery_required`.

This proves one durable directive winner and explicit ambiguity. It does not
claim exactly-once external execution.

## 7. Wait, Time, And Attempt Semantics

This first slice only permits the `unsatisfied -> satisfied` wait transition.
It requires a private capability bound to the exact condition, condition
version, trigger class, source commitment, and source revision. Expiry,
supersession, and cancellation require later dedicated lifecycle authority and
cannot be inferred through this operation. A satisfied wait only removes one
prerequisite; it does not grant execution.

Directive consumption requires every wait in the active yield generation to
be satisfied, the exact authority binding to match, the current cursor and
revision to match, and attempt budget to remain. Yield registration atomically
closes the started attempt as yielded and returns the window to yielded state.
Normal attempt outcomes close the window. Recovery without the private
attempt-use capability can record ambiguity only.

Each absent-operation mutation obtains a private Core-supplied trusted-time
observation from the store-owned injected clock and validates it against the
expected source/provenance, stored watermark, and expiry. The source kind,
provenance commitment, and observed instant are bound into the committed
receipt and operation commitment. Callers cannot submit time through operation
requests. Exact replay reuses and verifies the committed observation without
consulting the current clock. Clock unavailability, incompatible provenance,
time regression, and equality at expiry fail closed without writes.

## 8. Privacy And Error Posture

The implementation stores bounded validated identifiers, hashes, revisions,
enums, timestamps, and stable references only. It stores no prompt,
transcript, source content, command output, provider payload, environment
value, credential, authorization header, private key, or bearer authority.

Capabilities are private, non-cloneable, non-serializable, and use redacted
`Debug`. Stable errors use bounded codes and generic messages. Secret-like
identifiers are rejected without echoing their values. Public contract
deserialization fails closed for unknown fields and operation variants.

## 9. Test Coverage

Focused coverage proves:

- complete versioned contract validation and serde round trip;
- fail-closed invalid contract wire shapes;
- explicit unsupported posture for all current backends;
- PostgreSQL support inspection without connection;
- one directive consumer and capability-free exact replay;
- true concurrent directive consumption with one winner;
- true concurrent yield registration with one winner;
- attempt-budget exhaustion and restart rehydration;
- exact wake capability before consumption;
- exact active-yield-generation and window binding for wake authority;
- stale pre-wake authority rejection after wait revision changes;
- one-use normal outcome recording;
- capability-free ambiguity recovery only, with exact run/window/cursor
  binding;
- all-five-operation pre-commit and during-commit rollback plus post-commit
  exact replay;
- conflicting replay and cross-operation receipt reuse rejection;
- yield registration and wait-order canonicalization;
- trusted-time rollback and expiry rejection without writes;
- secret-like identifier rejection and capability Debug redaction.

All existing workspace tests remain part of the required phase validation.

## 10. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1786803272941084000-2`
- approval: `approval/run-1786803272941084000-2/implementation-approved`
- presentation: `presentation/56e0fa96a30face2`
- presentation hash:
  `56e0fa96a30face221ff4b97e330408e5ad4b679f806b3a7557612b80949c18e`
- approval outcome: granted by delegated maintainer after complete handoff
- approval reason: `implement-accepted-atomic-continuity-state-contract`
- phase status: completed and inspected through phase close
- event summary: 39 events, including one approval request, one proof-enforced
  approval grant, six scheduled steps, six successful skill invocations, no
  retries, and no escalations
- approval-presentation enforcement: proof-enforced
- out-of-kernel work: source inspection, edits, tests, documentation, and
  command execution were performed by the external executor under the
  governed scope

The kernel did not edit files, execute checks automatically, commit, push,
open or merge a pull request, invoke a provider, or simulate backend support.

## 11. Validation

- focused private conformance tests: 14 passed, 0 failed;
- public support-contract tests: 3 passed, 0 failed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed, including the continuity contract,
  concurrency, replay, fault-injection, and existing workspace suites;
- `npm run check`: passed, including docs, dogfood-helper, integration-helper,
  TypeScript, and contract checks;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed through `npm run check`;
- `git diff --check`: passed;
- independent focused security review: accepted after two blocker-fix passes;
  trusted-time ownership, active-generation wake binding, receipt uniqueness,
  replay integrity, and all-operation fault posture were verified.

## 12. Remaining Known Limitations

- No production backend implements the continuity operations.
- No continuity event vocabulary or event/state atomic projection exists.
- No supervisor observes a yielded window or starts another executor turn.
- No runtime path opens an authoritative execution window.
- Current-authority freshness between same-call assessment and transaction
  consumption remains a minimized but documented residual boundary.
- SQLite trusted-time and schema details remain unresolved.
- The reference store is a conformance oracle, not operational runtime state.
- The conformance suite remains coupled to the private reference store and
  must become backend-parametric before SQLite may advertise support.

## 13. Recommended Next Phase

Perform a focused maintainer/security review of the atomic-state contract and
reference conformance implementation. If accepted, implement SQLite continuity
schema and atomic transactions as the first durable backend and run the same
conformance and crash suite against it. Keep the local filesystem backend
unsupported.
