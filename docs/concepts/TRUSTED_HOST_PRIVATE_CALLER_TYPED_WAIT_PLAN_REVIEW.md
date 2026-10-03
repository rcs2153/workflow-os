# Private Trusted-Host Caller And Typed-Wait Plan Review

## 1. Executive Verdict

**Plan accepted with in-review corrections and non-blocking follow-ups.**

The plan correctly refuses to put a scheduler, provider, or generic host loop
around the accepted operational entry helper before genuine wait semantics
exist. It identifies the material gap: the supervisor registers empty wait
sets, and durable waits do not yet bind the exact dependency expected to wake
them.

Two planning blockers were corrected during review. Executor wait
declarations are now explicitly untrusted liveness requests rather than
authority or replacement approval gates. `TimeWindow` is selected as the
first source-specific wake path, verified from the accepted trusted-time
source in the same call that transitions the wait. No blocker remains.

## 2. Scope Verification

The plan remains private, local, SQLite-first, and staged. It does not
authorize runtime implementation during planning, scheduler behavior,
provider execution, provider mutation, OpenShell, nested harnesses, public
configuration, CLI, SDK, hosted behavior, multi-host execution, enterprise
administration, or release changes.

The plan does discuss a future reviewed SQLite continuity schema version. It
does not change a schema in this phase.

## 3. Current-State Assessment

The source inspection is accurate:

- `TrustedHostAttemptExecutionResult::Yielded` carries only a reason;
- the supervisor constructs `RegisterYieldRequest` with an empty wait list;
- the private wait record stores condition/version, owner, generation,
  trigger, state, source commitment after transition, and revision;
- it does not store the exact dependency binding expected before transition;
  and
- `AwaitCondition` has no accepted host-facing opaque handoff or caller.

This is safe while no production path claims genuine automatic wake behavior.
It is not enough for caller integration.

## 4. Executor Declaration Assessment

An exact authorized executor may report that it cannot continue before a
bounded dependency changes. That report is not an authority decision.

The corrected plan requires Core to:

- validate the supported dependency type and shape;
- derive run, window, attempt, generation, cursor, and revision bindings;
- bind the request to the exact immutable invocation and executor path;
- preserve or increase current governance strictness; and
- freshly reassess authority before any resumed attempt.

The executor cannot satisfy its own wait, grant an approval, replace a policy
gate, or choose a weaker continuation route. A malicious or faulty executor
can at worst request an invalid or delaying wait, which Core rejects or records
as a bounded liveness posture; it cannot gain execution authority.

## 5. Dependency-Binding Assessment

The minimum proposed durable binding is sufficient when implemented as one
canonical contract across private state, SQLite columns, `record_json`,
operation commitments, receipts, and projections:

- condition identity/version;
- owner window and yield generation;
- condition kind and compatible trigger;
- exact required dependency commitment;
- optional deadline binding;
- state and revision.

The review requires a new explicit SQLite continuity schema version instead
of an unreviewed side table. Legacy waits lacking the binding cannot be treated
as automatically satisfiable.

## 6. First Wake Source Assessment

`TimeWindow` is the correct first source. It uses the already accepted trusted
time boundary and avoids prematurely coupling attempt-yield semantics to
approval, evidence, or check models.

Registration must bind:

- the exact deadline;
- trusted-time source kind;
- provenance commitment;
- epoch identity;
- registering observation or watermark; and
- owning window expiry.

The deadline must be future relative to the accepted observation and no later
than window expiry. Satisfaction requires a fresh trusted-time observation
whose source, provenance, and epoch match and whose time is monotonic and at
or beyond the deadline. No caller-authored timestamp, boolean, or capability
is sufficient.

## 7. Approval And Policy Separation Assessment

The plan now correctly prohibits using attempt-yield declarations to create or
replace approval gates. Workflow approval and policy decisions remain owned by
their existing runtime paths.

A future `HumanDecision` wait may reference an already current, exact approval
dependency only after a separate source-specific plan proves how approval ID,
decision event, presentation proof, authority, and cursor are reloaded. That
work is not authorized here.

## 8. Handoff And Reinvocation Assessment

The opaque wait handoff is correctly non-authoritative. It may orient a host to
current conditions and the closed next operation, but it cannot satisfy or
resume work.

Wake transition and operational reinvocation remain separate. After a
successful transition, the host calls the accepted entry helper again. That
helper rehydrates current posture, consumes fresh authority, and competes
through existing one-winner directive and reservation operations.

## 9. Caller Assessment

The proposed synchronous private coordinator is appropriately narrow only
after dependency binding, typed registration, and one verifier are accepted.
It performs one explicit requested operation and does not loop, poll, sleep,
schedule, create model turns, open network connections, or choose progress.

The review agrees with the plan's sequence: implementing the caller first
would turn incomplete wait vocabulary into false governance.

## 10. Replay, Concurrency, And Failure Assessment

The plan preserves exact replay and compare-and-set behavior for yield,
transition, directive consumption, and dispatch reservation. It also preserves
truthful state across a crash after wait transition and before reinvocation.

The implementation must add independent-connection races for wait transition
and reinvocation. It must not claim exactly-once external delivery or recover a
capability from a receipt.

## 11. Privacy Assessment

The plan keeps dependency payloads out of durable state. Safe categories,
bounded IDs where justified, commitments, revisions, cursors, trusted-time
facts, receipts, and lifecycle posture are sufficient.

Errors and Debug must not echo raw references, deadlines supplied in rejected
input, approval rationale, evidence/check data, prompts, command output,
provider payloads, paths, credentials, tokens, or capability internals.

## 12. Test Plan Assessment

The proposed tests cover registration, incompatible declarations, wrong-source
substitution, source unavailability, same-call verification, restart,
remaining waits, all-satisfied posture, competing wake callers, competing
reinvocation, privacy, legacy rows, and retained regression suites.

Add two explicit assertions during the first implementation:

- a caller-supplied timestamp beyond the deadline cannot satisfy the wait;
- a fresh trusted-time observation from the wrong epoch fails without changing
  the wait or window revision.

## 13. Blockers

None after the in-review corrections.

## 14. Non-Blocking Follow-Ups

- Decide whether durable wait identity stores a safe reference plus commitment
  or commitment plus source category only.
- Choose the smallest opaque handoff identity after wait-state hardening.
- Plan each non-time wake source separately.
- Keep caller integration blocked until typed registration and the first wake
  verifier pass focused review.

## 15. Recommended Next Phase

Implement exact authoritative wait dependency binding and the private
`TimeWindow` verifier only, including a reviewed SQLite continuity schema
version and legacy-row fail-closed posture. Also add the deferred direct
fresh-opening composition test.

Do not implement the private caller, scheduler, provider execution,
OpenShell, nested harnesses, automatic approval, public configuration, CLI,
SDK, hosted behavior, or release claims in that phase.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791022405486239000-2`
- approval: `approval/run-1791022405486239000-2/review-scope-approved`
- presentation: `presentation/5445d2c57b0b05d4`
- presentation hash:
  `5445d2c57b0b05d418cfd52fa9ad9bf4a6b38751e8d6a875887d06ce3bb7c657`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused plan review and blocker corrections only
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations, 0
  retries, and 0 escalations; approval-presentation proof enforced
- validation summary: documentation and diff checks passed
- out-of-kernel work: source inspection, security review, documentation,
  validation, and git actions were performed by the delegated trusted host;
  Workflow OS governed scope and approval but did not perform those actions
