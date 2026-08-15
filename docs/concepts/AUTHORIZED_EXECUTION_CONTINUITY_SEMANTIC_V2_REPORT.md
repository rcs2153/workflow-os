# Authorized Execution Continuity Semantic V2 Report

## 1. Executive Summary

Workflow OS now has the shared semantic V2 baseline required before a durable
authorized-execution continuity backend can be implemented. The amendment
distinguishes a lawful executor yield from a genuine wait, makes delegated
authority consume-by-value, records security rejections as replayable committed
dispositions, and gives the kernel a conservative continuation classification:
`resume_now`, `await_condition`, `blocked`, or `terminal`.

This phase does not schedule an agent, create another model turn, persist
continuity state in SQLite, approve a gate, execute a tool, mutate a provider,
or expose workflow, CLI, or SDK configuration. A future host supervisor must
honor the kernel's authoritative disposition; Core does not claim to control a
host scheduler by itself.

## 2. Scope Completed

- Added an additive V2 support-contract type without widening the existing
  public exhaustive V1 version enum.
- Added the explicit `local_live_state_only` support scope.
- Added committed success and committed security-rejection dispositions.
- Added epoch-bound trusted-time observations, durable watermarks, quarantine,
  and private live-instance eligibility.
- Changed directive authority to a non-cloneable capability consumed with the
  request by value.
- Added capability-free read-only operation reconciliation.
- Bound waits to exact condition ID and condition version.
- Added kernel-owned continuation disposition for runnable, genuine-wait,
  blocked, and terminal posture.
- Hardened exact replay and reconciliation against commitment, target, result
  shape, trusted-time, and legal-transition corruption.
- Preserved every production backend as V1 with all continuity operations
  unsupported.

## 3. Scope Explicitly Not Completed

This phase does not add:

- SQLite schema, upgrade, transaction, or restart implementation;
- local-filesystem or PostgreSQL continuity support;
- runtime event or audit projection;
- execution-window opening from the executor;
- scheduler, supervisor, polling, or agent redispatch behavior;
- automatic gate approval or evidence satisfaction;
- reusable delegated authority or model self-approval;
- provider writes, another mutation family, or external execution;
- workflow schema, public runtime configuration, CLI commands, or examples;
- nested harness execution, hosted orchestration, or distributed leases.

## 4. Contract And Compatibility Summary

The existing `AuthorizedExecutionContinuityStateContractVersion` remains the
same V1-only exhaustive public enum. The existing contract constructor,
provider trait, and serialized V1 shape remain readable and usable. This avoids
breaking downstream exhaustive matches.

Semantic V2 is additive through
`AuthorizedExecutionContinuityStateContractV2`, its new non-exhaustive version
type, and the new non-exhaustive support-scope type. V2 accepts only a complete
all-supported declaration under `local_live_state_only`; mixed or incomplete
support fails closed. Operation entries are canonicalized into stable order.

Malformed contract, entry, field-name, and enum-value input maps to fixed,
bounded errors that do not echo attacker-controlled text.

## 5. Trusted Time And Security Rejections

Absent-operation mutations use one store-owned trusted-time observation bound
to a fixed source, provenance commitment, and epoch. Regression, incompatible
provenance, epoch mismatch, and expiry become committed rejection dispositions
with exact prior and resulting trusted-time and window security snapshots.

Replay and reconciliation recompute the trusted-time, receipt, operation, and
rejection commitments. They also validate the legal transition for each
rejection kind. Regression, provenance mismatch, and epoch mismatch must
quarantine the instance without changing the window. Expiry must advance the
watermark and expire the exact window. The committed rejection must also match
the current authoritative trusted-time source, provenance, epoch, and exact
window expiry. A self-consistent but illegal rewritten transition or divergent
authoritative row is rejected as corrupt state.

## 6. Authority, Reconciliation, And Attempts

`ConsumeDirective` owns a private, non-cloneable authority-use capability and
consumes the request by value. Crossing the operation boundary burns that
capability regardless of commit, rollback, or ambiguity. Exact replay returns
the committed disposition but never returns another attempt capability.

The reconciliation reader requires the exact operation ID, request commitment,
and receipt ID. It returns a boxed committed disposition, confirmed absence, or
unreadable state without accepting or reissuing execution authority.

A started attempt is not classified as resumable merely because its window is
`executing`. Without proof of a live attempt lease or capability, the read-only
continuation disposition is `blocked`; ambiguity recovery must resolve a lost
executor before another attempt can be authorized.

## 7. Wait And Continuation Semantics

Wait identity is the exact pair of condition ID and positive condition version.
An unsatisfied active wait produces `await_condition`. A terminal wait state,
assessment requirement, recovery requirement, executing attempt without live
ownership, ineligible instance, stale or incompatible trusted time, or expired
window produces `blocked`. Closed, expired, revoked, and superseded windows are
terminal. A yielded window with no unsatisfied or terminal wait is
`resume_now`.

This distinction is the answer to the false-stall problem: an executor turn
ending is not workflow completion and does not fabricate a human approval wait.
The next operational slice must persist these semantics and eventually expose
them to an injected supervisor that can redispatch lawful work.

## 8. Replay And Integrity Summary

Exact replay validates:

- trusted-time source, provenance, epoch, observation, and commitment;
- receipt identity, kind, committed time, and operation commitment;
- legal security-rejection state deltas;
- operation-specific committed success shapes;
- exact window, yield, directive, wait, and attempt targets; and
- the stored request and result commitments.

Yield success must record yielded attempt and window states. Wait transition
must record `satisfied`. Directive consumption must record consumed directive,
started attempt, and executing window states. Ordinary outcomes must record an
allowed terminal attempt state and closed window. Recovery must record
`ambiguous_may_have_started` and `recovery_required`.

## 9. Privacy And Error Posture

The implementation stores only validated bounded identifiers, hashes,
revisions, enums, timestamps, and security snapshots. It stores no prompt,
transcript, source content, command output, provider payload, environment
value, credential, authorization header, private key, or reusable bearer
authority.

Private capabilities remain non-serializable and redact their bindings in
`Debug`. Public malformed-wire errors are fixed and do not echo secret-like
field names, enum values, or payload values.

## 10. Test Coverage

Focused tests cover:

- legacy V1 construction, serialization, and canonical ordering;
- additive V2 round trip, complete support, scope, and mixed-support rejection;
- non-leaking unknown fields and secret-like enum values;
- one-winner directive consumption and yield registration;
- consume-by-value authority and capability-free exact replay;
- read-only reconciliation without authority reissuance;
- exact wait identity and wake binding;
- committed regression, provenance, epoch, and expiry rejection;
- legal rejection-transition replay validation;
- result-shape and exact-target replay validation;
- trusted-time and receipt corruption;
- restart trust-root mismatch and instance quarantine;
- runnable, genuine-wait, blocked, executing, expired, and terminal
  continuation classification; and
- every operation's before-, during-, and after-commit fault posture.

## 11. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1786820346813404000-2`
- approval: `approval/run-1786820346813404000-2/implementation-approved`
- presentation: `presentation/220aa735e476276f`
- approval outcome: granted under delegated-maintainer authority after the
  complete approval handoff
- phase status: completed
- out-of-kernel work: source inspection, edits, independent review, tests,
  documentation, and command execution were performed by the external executor
  under the governed scope

The kernel did not edit files, run checks automatically, commit, push, open a
pull request, schedule an agent, or claim production backend support.

## 12. Validation

- continuity semantic unit tests: 21 passed, 0 failed;
- public V1/V2 contract tests: 6 passed, 0 failed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed using a clean target directory under
  `/private/tmp` to avoid per-binary macOS provenance latency in the checkout;
- `npm run check`: passed;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed;
- independent focused security review: two reviewers returned `ACCEPT` with no
  remaining security or correctness blockers after the authoritative-row
  binding regression was added.

## 13. Remaining Known Limitations

- No production backend implements V2 continuity operations.
- No durable continuity state survives restart.
- No supervisor consumes `resume_now` to create another executor turn.
- No live attempt lease exists; an `executing` window therefore blocks
  conservative read-only redispatch until outcome or ambiguity recovery.
- No runtime path opens an authoritative execution window.
- No external rollback-resistant epoch anchor exists.
- The reference store remains a conformance oracle, not operational state.
- Existing backends remain V1/unsupported.

## 14. Recommended Next Phase

Perform the focused maintainer/security acceptance review of this semantic V2
baseline. Only after acceptance should Workflow OS implement the explicit
SQLite V2 schema, atomic upgrade, transactions, restart/replay behavior, and
backend-parametric conformance harness. Runtime scheduler integration remains
later and must not precede durable state.
