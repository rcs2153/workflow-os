# Explicit Local Trusted-Host Production Caller Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

The plan preserves the accepted Core authority boundary and correctly avoids a
general scheduler, but four decisions must be made before implementation:
cancellation ownership is not operationally reachable, the production entropy
source is unspecified, the finite wake budget is deferred, and the proposed
reconstruction hint is ambiguous policy advice rather than authoritative
posture.

## 2. Scope Verification

The plan stays within the approved planning-only scope. It does not authorize:

- caller implementation;
- a daemon, detached thread, queue, worker pool, or run discovery;
- public API, CLI, SDK, schema, or runtime configuration;
- automatic approval or model turns;
- provider reads or mutations;
- OpenShell or nested harness execution;
- hosted or distributed scheduling; or
- release posture changes.

The proposed caller remains crate-private, synchronous, local, SQLite-only,
and `TimeWindow`-only.

## 3. Architectural Boundary Assessment

The composition boundary is sound. The caller owns host-process mechanics;
Core retains wait satisfaction, current disposition, authority, directive
consumption, dispatch reservation, workflow state, and completion.

The caller accepts an exact operational locator, executor, and `SkillInput`
and reuses `run_trusted_host_local_timer`. It does not accept caller-authored
authority or claimed eligibility. Keeping caller implementation separate from
an adoption site avoids silently turning a private helper into automatic
scheduling.

One wording correction is required in the blocker fix: an operational locator
is already selected, not necessarily “already authorized.” Current authority
is proven only inside the accepted same-call Core boundaries.

## 4. Ownership And Cancellation Assessment

The synchronous ownership model is appropriate, but the cancellation API is
not implementable as written.

The plan says the caller creates the cancellation pair internally while the
caller blocks until completion. A handle returned by the function would arrive
too late to cancel that call. The alternatives are left as an unspecified
“scoped callback or owner object,” which could accidentally introduce a
threading or lifetime surface the plan otherwise excludes.

The blocker fix must select one exact boundary. The smallest compatible choice
is:

- a private factory creates the existing cancellation receiver/handle pair;
- the owner retains the handle;
- the caller input consumes the receiver; and
- the caller remains synchronous and creates no thread.

Cancellation is not execution authority, so accepting the private receiver
does not weaken Core governance. The caller must not accept an arbitrary trait
implementation or reconstruct cancellation from durable state.

## 5. Identity Generation Assessment

The required properties are correct: domain separation, independence, bounded
length, no secret or timestamp-only derivation, deterministic test injection,
and no retry after replay conflict.

The production source remains unspecified. “Already available in the
workspace, or a narrowly justified dependency” is not phase-ready for a
security-sensitive identity boundary. Transitive `getrandom` presence in the
lockfile is not a supported direct dependency contract.

The blocker fix must choose the source and exact encoding. A conservative
choice is a direct, pinned `getrandom` dependency used to obtain 128 random
bits per identifier, encoded as lowercase hexadecimal after a short
operation-family prefix. Every required identifier should receive independent
random bytes. Production construction must be private; tests may inject a
deterministic byte source. Entropy failure must map to one fixed error and no
partial identity set may escape.

## 6. Wake-Budget Assessment

The plan correctly forbids arbitrary or unbounded caller input, but defers the
actual value to implementation review. That postpones part of the liveness and
ownership contract until after code exists.

The blocker fix should select a fixed budget of **two wakes** for the first
caller. Two is the smallest value that proves repeated lawful waiting rather
than only schedule-once behavior. Exhaustion remains non-terminal and returns
the accepted `WakeBudgetExhausted` posture. Any later increase requires a
separate reviewed change.

## 7. Restart And Recovery Assessment

The restart posture is correct. A reopened backend plus the durable locator and
exact immutable execution inputs are revalidated through Core. Timer state,
cancellation, identity-source state, and remaining budget are not restored.
No event or receipt reconstructs authority.

The future test must prove a restarted caller receives a new cancellation pair
and fresh identities while preserving exact executor and `SkillInput` binding.

## 8. Operator Outcome Assessment

Disposition, scheduled-wake count, executor-entry count, and a closed stop
reason are bounded and useful.

The proposed `whether explicit reconstruction may be considered` field is not
acceptable. It compresses policy, current authority, state freshness, and host
judgment into a boolean that could become an implied resume authorization.
Remove it. The direct owner may use the stop reason as descriptive posture and
must reenter Core for every later decision.

Structured errors should remain errors rather than a `Failed` stop reason.
This preserves the distinction between a lawful host stop and a failed or
ambiguous operation.

## 9. Failure And Privacy Assessment

The plan preserves fail-closed behavior and does not convert host failure into
workflow failure or completion. Automatic retry is correctly prohibited after
identity conflict, corruption, ambiguity, trusted-time rejection, or binding
mismatch.

The privacy boundary is appropriate: no prompts, source content, commands,
credentials, approval reasons, evidence bodies, skill outputs, provider
payloads, paths, or reusable authority belong in caller outcomes, Debug, or
errors.

The blocker fix should explicitly require all-or-nothing identity-set
construction so entropy failure cannot expose a partially usable set.

## 10. Test Plan Assessment

The proposed tests cover the important runtime properties. Add focused tests
for:

- cancellation-handle reachability while the synchronous caller is blocked;
- all-or-nothing identity-set construction on injected entropy failure;
- independent random material for every operation family;
- the fixed two-wake budget being impossible to override;
- absence of any reconstruction/advisory boolean in the outcome; and
- no background thread created by the caller.

The implementation review should also verify that the new production entropy
dependency is direct, pinned through the workspace lockfile, and used nowhere
to derive authority.

## 11. Documentation Assessment

The plan and roadmap accurately state that no caller exists and that public or
automatic scheduling remains unsupported. The planning report clearly
discloses non-scope and governed-phase evidence.

The blocker fix must replace the inaccurate “already-authorized” shorthand
with “already-selected” or equivalent source-of-truth language.

## 12. Blockers

1. Select a reachable cancellation ownership API for a synchronous call.
2. Select an exact production entropy source, encoding, and all-or-nothing
   identity-set construction rule.
3. Fix the first caller's wake budget at two.
4. Remove the ambiguous reconstruction-advice boolean from the outcome.
5. Correct “already-authorized” wording so only Core claims current authority.

## 13. Non-Blocking Follow-Ups

- Later operator metrics and notifications need a separate public adoption
  plan.
- Detached owner-loss semantics remain deferred.
- Other wait-source families remain deferred.
- A later internal adoption site must be separately planned and reviewed.

## 14. Recommended Next Phase

Perform one documentation-only planning blocker fix that resolves all five
items above. Then conduct a focused re-review before implementation.

Do not implement the caller, entropy source, scheduler, public surface,
provider mutation, OpenShell, nested harness, hosted scheduling, or release
change in the blocker-fix phase.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791362274491289000-2`
- approval: `approval/run-1791362274491289000-2/review-scope-approved`
- presentation: `presentation/cc9710269023b9c3`
- presentation hash:
  `cc9710269023b9c340c360d7adc3a4da61857bbb39c7dff24819db7852c2c0cb`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: focused plan review only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced with one matching record and event
  marker
- validation summary: `npm run check:docs` and `git diff --check` passed; Rust
  checks were not required because the phase changed documentation only
- out-of-kernel work: plan and source inspection, review authoring,
  documentation validation, and later git and pull-request actions
