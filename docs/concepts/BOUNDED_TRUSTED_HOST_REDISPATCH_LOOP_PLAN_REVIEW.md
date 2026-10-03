# Bounded Trusted-Host Redispatch Loop Plan Review

## 1. Executive Verdict

**Needs blocker fixes.**

The plan correctly composes the accepted execution-window, directive,
one-shot supervisor, and atomic dispatch-reservation boundaries. It remains
private, local, injected, and appropriately excludes scheduling, provider
execution, OpenShell, nested harnesses, public configuration, and hosted
behavior.

Two issues must be corrected before implementation. The proposed normal
budget-exhaustion return can recreate the false-stall condition while Core
still requires immediate lawful continuation. The proposed iteration-input
provider also has more authority-shaped input surface than the accepted
operation binding permits: the exact `SkillInput` is already committed and
must not be resupplied or varied per iteration.

## 2. Scope Verification

The planning phase stayed within its approved documentation-only boundary. It
did not add runtime code, a scheduler, model-turn creation, provider execution
or mutation, OpenShell, nested harnesses, automatic approval, public runtime
configuration, CLI, SDK, workflow schema, hosted behavior, or release changes.

## 3. Existing Runtime Boundary Assessment

The plan is grounded in the current private runtime contracts:

- `supervise_one_local_skill_attempt` validates the exact workflow, run, step,
  invocation, and executor commitments before reservation or executor entry;
- `trusted_host_invocation_commitment` commits every `SkillInput` identity and
  value together with the executor binding;
- directive consumption validates the exact window revision, cursor, active
  yield generation, expected waits, authority commitment, and immutable
  window binding in one SQLite transaction;
- dispatch reservation provides one-winner admission before executor entry;
  and
- the supervisor persists an accepted outcome or yield before deriving the
  next authoritative continuation disposition.

These primitives are sufficient for a bounded composition helper after the
two plan blockers below are resolved.

## 4. Atomic Directive And Reservation Assessment

The plan correctly requires fresh durable rehydration for every iteration and
prohibits a cached host `ResumeNow` decision from authorizing work. The current
consume transaction already fails closed on stale cursor, revision, yield,
wait, or authority posture. A private composition helper may therefore derive
the current disposition and construct a consume request, provided the consume
transaction remains the authoritative compare-and-set boundary.

The implementation need not combine a read and mutation into an impossible
single host-side decision. It must treat the read as advisory and the
transactional consume result as authoritative. Replay or a losing concurrent
caller returns no fresh authority and invokes no executor.

## 5. Invocation-Binding Assessment

The accepted opening contract commits the complete invocation:

- workflow, workflow version, schema version, and spec hash;
- run, step, skill, skill version, and correlation identity;
- every input name and value; and
- the executor binding commitment.

The loop therefore must not ask an iteration provider to supply a new
`SkillInput` or executor binding. A matching replacement would merely
reconstruct already-authorized work; a non-matching replacement fails closed.
Leaving this in the provider contract obscures the authority boundary and
needlessly exposes invocation values to another host component.

The corrected plan should carry one exact immutable invocation template and
one executor binding from the accepted initial operation across the loop. An
injected provider may supply only fresh bounded non-authorizing identities
needed for operation, receipt, attempt, or yield persistence. Core must still
validate those identities and construct every capability.

## 6. Budget And Liveness Assessment

The positive finite executor-entry budget is necessary process-safety posture.
The proposed `ResumeRequiredBudgetExhausted` normal return is not sufficient.

At that point durable Core state still says `ResumeNow`: lawful work remains,
there is no typed wait, no block, and no terminal transition. Returning control
without a durable and actionable redispatch obligation allows the host or
agent turn to end exactly where the roadmap says it must not. That would turn
a process budget into a false workflow stall.

The corrected plan must define a bounded preemption handoff that cannot be
mistaken for phase completion. At minimum it must:

1. preserve `ResumeNow` as the authoritative disposition;
2. return a typed non-terminal continuation token or equivalent private
   obligation that contains no reusable execution authority;
3. require the calling trusted host to immediately schedule or invoke a fresh
   bounded loop call while the execution window remains lawful;
4. prohibit a final assistant response or normal phase close while that
   obligation is outstanding; and
5. define the fail-closed behavior when no trusted redispatch mechanism is
   available.

If the current local host has no mechanism capable of honoring that contract,
the first implementation must remain an internal loop with a hard safety
ceiling that returns a structured liveness error, not a successful normal
outcome. It must not claim to solve uninterrupted continuation until a caller
can discharge the obligation.

## 7. Stop-Semantics Assessment

The plan handles the accepted workflow dispositions correctly:

- `AwaitCondition` stops without polling or fabricating approval;
- `Blocked` stops without hidden retry;
- `Terminal` stops without another directive or executor call; and
- ambiguity, corruption, security rejection, or reconciliation failure remain
  structured errors.

Successful executor output correctly does not imply step or workflow
completion. The only defect is treating a still-actionable `ResumeNow` budget
boundary as an ordinary successful stop.

## 8. Concurrency, Replay, And Recovery Assessment

The plan correctly relies on durable one-winner directive consumption and
dispatch reservation rather than process-local locking. Required concurrent
tests cover independent SQLite connections, one executor entry, losing callers
without authority, replay without execution, and stale-state rejection.

The plan also correctly refuses automatic retry after ambiguous executor entry
or ambiguous commit acknowledgement. Fresh redispatch is lawful only after
accepted recovery establishes a new authoritative `ResumeNow` posture.

## 9. Privacy And Error Assessment

The plan preserves payload-free events and bounded errors. It prohibits
prompts, transcripts, source content, command output, provider payloads,
environment values, credentials, tokens, and authorization material.

Removing `SkillInput` from the iteration provider further narrows exposure and
keeps raw invocation values confined to the already-accepted supervisor
boundary. Loop outcomes and Debug implementations must expose only disposition,
counts, bounded stop posture, and redacted output presence.

## 10. Test Plan Assessment

The planned tests are strong for multi-iteration execution, typed stops,
concurrency, replay, ambiguity, restart, event ordering, privacy, and no false
completion. The blocker fix must add explicit tests proving:

- the iteration provider cannot supply or alter `SkillInput` or executor
  binding;
- budget exhaustion cannot be returned as successful completion while
  `ResumeNow` remains actionable;
- an outstanding host redispatch obligation prevents normal phase close;
- a fresh loop call obtains fresh authority rather than reusing a returned
  capability; and
- absence or failure of the required redispatch mechanism fails closed without
  fabricating wait, block, approval, or terminal state.

## 11. Blockers

1. **Budget exhaustion can recreate a false stall.** The plan returns a normal
   host posture while Core still says `ResumeNow`, but does not define a durable
   or enforceable immediate redispatch obligation.
2. **The iteration provider can resupply authority-bound invocation data.**
   `SkillInput` and executor binding are already fixed by the operation
   commitment and must be immutable loop inputs, not per-iteration provider
   output.

## 12. Non-Blocking Follow-Ups

- Filesystem and PostgreSQL parity remain deferred.
- Multi-host leases, fairness, heartbeats, failover, and reservation recovery
  remain future host-runtime work.
- A later public host integration may expose bounded liveness posture only
  after the private contract is implemented and reviewed.
- Provider execution, OpenShell, and nested harnesses remain blocked.

## 13. Recommended Next Phase

Run a focused **bounded trusted-host redispatch-loop planning blocker fix**.
Correct only the invocation-provider boundary and budget-exhaustion liveness
contract, update the test plan, and then perform a focused blocker-fix review.
Do not implement the loop until that review accepts the corrected plan.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1790952495208918000-2`
- approval: `approval/run-1790952495208918000-2/review-scope-approved`
- presentation: `presentation/28e141bb2f408344`
- presentation hash:
  `28e141bb2f408344f42678fb6df1eb954cdd23c6456cbc6a5cad1f443585730f`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- reviewed boundary: planning artifacts and current private Core commitment,
  continuity, supervisor, reservation, event, and SQLite surfaces
- review verdict: needs blocker fixes
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: source inspection, security analysis, review authoring,
  validation, and git actions are performed by the delegated maintainer;
  Workflow OS governs scope and approval but does not inspect code, edit files,
  or run shell commands
