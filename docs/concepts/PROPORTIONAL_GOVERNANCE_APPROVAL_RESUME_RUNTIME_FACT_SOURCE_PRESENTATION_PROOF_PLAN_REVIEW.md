# Proportional-Governance Approval-Resume Runtime-Fact Source Presentation-Proof Plan Review

## 1. Executive Verdict

Plan accepted; proceed to the explicit local proof-enforced runtime-fact source
approval-resume implementation.

The plan closes the remaining composition gap between two already accepted
approval invariants: proof that the exact approval scope was presented and
proof that current registered-source facts still authorize the resume.

## 2. Scope Verification

The plan remains within one additive local Core wrapper and any private
refactoring necessary to reuse accepted boundaries. It does not authorize
default activation, automatic approval, raw fact or presentation persistence,
schemas, CLI or UI behavior, report/artifact changes, providers, OpenShell,
SideEffects, writes, hosted expansion, enterprise identity, or release changes.

## 3. Architecture Assessment

The proposed architecture is correct:

- approval preparation remains the pending-state authority;
- durable presentation proof remains the decision-presentation authority;
- the existing source-backed grant-precondition path remains the resume-plan,
  immutable-state, and mutation authority; and
- the existing runtime-fact source reassessment remains the current-fact
  authority.

The plan does not create a second approval state machine or let a source grant
approval. The source supplies bounded facts; Workflow OS validates and governs
the decision.

## 4. Ordering Assessment

Validating presentation proof before source invocation is the strongest and
least surprising order. A missing or stale proof means no decision is
authorized, so Core should not access a source or disclose immutable run
context to it.

After proof succeeds, the accepted source-backed helper should preserve its
existing ordering: freeze the exact resume plan before source access, validate
the V3 registration commitment, invoke the source once, reassess, and mutate
only after equality succeeds.

Tests must prove both boundaries with source-call counts and exact event-vector
equality.

## 5. Grant And Denial Assessment

A grant requires proof plus fresh reassessment because it can resume execution.
A denial requires proof but no fresh facts because it cannot expand authority
or execute work. This distinction preserves fail-closed availability while
still proving what the decision maker saw.

The existing proof marker is sufficient for both outcomes. The implementation
must not introduce a new marker vocabulary or copy presentation content into
events.

## 6. Atomicity Assessment

The plan correctly requires all proof, resolved-context, bundle, registration,
source, freshness, and assessment checks to complete before new events. This is
the essential atomicity claim.

The implementation should avoid validating proof in one public helper and then
calling another public helper that repeats approval preparation. A narrow
private composition should carry the prepared decision and marker into the
existing grant-precondition boundary without introducing an intermediate
mutation window.

## 7. Privacy And Error Assessment

The proposed privacy boundary is conservative and consistent with current
Core behavior. Existing stable errors should be reused, nested requests should
remain redacted in Debug, and neither presentation payloads nor runtime facts
should enter events or returned serialization.

The implementation review must specifically inspect mixed-failure cases. When
proof and source inputs are both invalid, the proof error should win without a
source call and without leaking either input.

## 8. Compatibility Assessment

The path remains explicit and additive. Existing ordinary, proof-only,
source-only, caller-fact, and authoritative local-check APIs remain valid and
unchanged. Reusing the current source-backed result type is appropriate because
the new wrapper adds approval evidence rather than a new runtime-fact result.

Rejecting V1 and V2 durable bindings remains correct for this path. Those
formats cannot prove the registered source commitment required for fresh
decision-time authority.

## 9. Product Feedback Reconciliation

The fresh-pull review confirms that Workflow OS has a credible, honest
first-run product and that its next product challenge is reducing ceremony for
low-risk work while preserving evidence. This plan is infrastructure for that
goal, not another user-visible mode: it makes the approvals that remain
necessary trustworthy across presentation and current-fact boundaries.

The previously reported Node 24 integration-check opacity and duplicate
missing-manifest diagnostic have already been fixed on current `main`. Reopening
them would distract from the active proportional-governance lane.

## 10. Test Quality Assessment

The planned tests cover successful grant, source-call count, proof marker,
fresh result, every material proof failure, mixed-failure precedence,
resolved-context and registration preflight, source and reassessment failure,
source-free denial, exact event stability, compatibility, and non-leakage.

The matrix is phase-ready. No additional broad integration fixture is required
for this local helper beyond existing workspace regression coverage.

## 11. Planning Blockers

None.

## 12. Non-Blocking Follow-Ups

- Decide later whether decision-time source snapshots need bounded report or
  audit citations.
- Keep changed-assessment escalation separate from exact approval resume.
- Keep production time and source authentication out of this local wrapper.
- Do not use this phase to change approval defaults or proportional-governance
  routing.

## 13. Recommended Next Phase

Implement the explicit local proof-enforced current-runtime-fact source
approval-resume wrapper in one governed runtime-composition phase, then perform
a focused maintainer review.

## 14. Governed Review Record

- Dogfood workflow: `dg/d`.
- Run ID: `run-1786289982161301000-2`.
- Approval ID: `approval/run-1786289982161301000-2/planning-approved`.
- Approval presentation ID: `presentation/0848855afdd4da7f`.
- Approval outcome: granted by the delegated maintainer after the complete
  proof-enforced handoff was presented.
- Review posture: planning-only; no runtime behavior was added.
- Phase status: completed with 39 ordered events, one approval, zero retries,
  and zero escalations.
- Validation: `npm run check:docs` passed; `git diff --check` passed.
