# Bounded Trusted-Host Redispatch Loop Plan Blocker Fix Review

## 1. Executive Verdict

**Blockers fixed; proceed to the private local SQLite bounded redispatch-loop
implementation.**

The corrected plan no longer permits a successful host return while Core says
`ResumeNow`, and the injected provider can no longer resupply or alter exact
invocation data already fixed by the accepted operation commitment.

## 2. Scope Verification

The fix changed planning documentation only. It did not add runtime code,
scheduling, model-turn creation, automatic approval, provider execution or
mutation, OpenShell, nested harnesses, public configuration, CLI, SDK,
workflow schemas, hosted behavior, or release changes.

## 3. Original Blockers

The original plan had two blockers:

1. a caller-selected budget could return `ResumeRequiredBudgetExhausted` while
   durable Core state still required immediate continuation; and
2. a per-iteration provider could resupply `SkillInput` even though the
   accepted operation binding already commits every invocation field and
   executor binding.

Both could obscure the kernel's authority boundary. The first could recreate
the false-stall behavior this lane is intended to eliminate. The second added
an unnecessary host substitution surface.

## 4. Liveness Fix Assessment

The plan now uses the durable execution-window maximum-attempt posture as the
finite bound. Each resumed iteration consumes one attempt allocation in the
accepted directive-consumption transaction. The loop has no caller-selected
budget and no normal resume-required stop.

The accepted `continuation_disposition` currently derives `ResumeNow` from a
yielded window whose waits are satisfied. It does not itself inspect the next
attempt counter. The accepted `allocate_attempt` operation separately rejects
zero, overflow, or `next_attempt_number > maximum_attempts`.

The plan correctly requires the new private helper to cross-check these two
facts before another dispatch. If disposition is `ResumeNow` but no attempt
allocation is lawful, the helper returns a structured integrity/liveness error
rather than completion, wait, block, approval, or success. This is the
smallest fail-closed composition compatible with the current contracts.

## 5. Immutable Invocation Fix Assessment

`trusted_host_invocation_commitment` commits workflow, workflow version,
schema version, spec hash, run, step, skill, skill version, correlation ID,
every input name and value, and executor binding. The supervisor recomputes
that commitment before reservation or executor entry.

The corrected plan therefore carries one immutable `SkillInput` and executor
binding from the accepted opening operation. The renamed identity provider may
supply only fresh bounded non-authorizing operation, receipt, attempt, and
yield-generation identities. It cannot receive or return invocation data,
capabilities, durable state, or commitments.

This is appropriately narrow and makes invocation substitution structurally
unavailable rather than merely rejected after provider output.

## 6. Directive And Reservation Assessment

The corrected plan preserves the accepted transactional boundaries:

- a fresh disposition is rehydrated for every iteration;
- the read is advisory and transactional directive consumption remains the
  authoritative compare-and-set operation;
- every fresh attempt commits one atomic dispatch reservation before executor
  entry;
- losing callers and exact replay receive no authority;
- ambiguous commits reconcile without issuing authority; and
- outcome, yield, and recovery persistence remains bound to the exact
  reservation.

No new atomic primitive is required merely to prevent a stale host read from
authorizing work. The consume transaction already validates exact cursor,
revision, active generation, waits, actor, authority, and window binding.

## 7. Stop And Error Semantics

- `AwaitCondition` returns without polling or fabricated approval.
- `Blocked` returns without retry.
- `Terminal` returns without another directive or executor call.
- `ResumeNow` continues inside the helper.
- attempt-limit inconsistency, ambiguity, corruption, stale state, security
  rejection, projection drift, and failed reconciliation remain structured
  errors.

Successful executor output still does not imply workflow completion.

## 8. Privacy Assessment

The corrected provider boundary reduces exposure by keeping invocation values
inside the existing supervisor input. Durable state remains limited to
accepted identifiers, commitments, cursors, revisions, trusted-time facts,
and payload-free events.

The plan continues to prohibit prompts, transcripts, source content, command
output, provider payloads, environment values, credentials, authorization
material, tokens, and raw local skill output in errors, Debug, audit, or
report structures.

## 9. Test Quality Assessment

The revised test plan now proves both blocker fixes:

- no normal return while Core says `ResumeNow`;
- authoritative attempt limits bound executor entries;
- inconsistent `ResumeNow` plus exhausted attempts fails closed;
- the identity provider cannot supply or replace invocation input or executor
  binding; and
- no fake completion, wait, block, or approval is emitted at the finite bound.

It retains required multi-iteration, typed-stop, concurrency, replay,
ambiguity, restart, event-ordering, privacy, and regression coverage. This is
sufficient to begin the private implementation.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- The implementation must choose the smallest private remaining-attempt read
  without exporting new public model state.
- Process preemption requires a separately reviewed durable continuation
  handoff before it can interrupt a `ResumeNow` loop.
- Filesystem, PostgreSQL, multi-host leases, fairness, and failover remain
  deferred.
- Provider execution, OpenShell, nested harnesses, and public host integration
  remain blocked.

## 12. Recommended Next Phase

Implement only the **private local SQLite bounded trusted-host redispatch
loop** described by the corrected plan. Keep invocation immutable, obtain fresh
directive and reservation authority for every resumed attempt, continue while
Core says `ResumeNow`, and stop only on accepted dispositions or structured
errors. Require a focused maintainer/security implementation review before any
provider, sandbox, nested-harness, public-config, CLI, SDK, or hosted work.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1790952894437008000-2`
- approval: `approval/run-1790952894437008000-2/review-scope-approved`
- presentation: `presentation/c810b74a88270dfb`
- presentation hash:
  `c810b74a88270dfbf322afc0aa65035825a41c28409cb8e94c605ed21e3934fa`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- review verdict: blockers fixed
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced
- validation summary: `npm run check:docs` and `git diff --check` passed;
  corrected claims were verified against current private Core contracts
- out-of-kernel work: source inspection, security review, documentation
  authoring, validation, and git actions are performed by the delegated
  maintainer; Workflow OS governs scope and approval but does not inspect code,
  edit files, or run shell commands
