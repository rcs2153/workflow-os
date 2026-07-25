# Authoritative Local-Check Reassessment Binding Plan Review

## 1. Executive Verdict

**Needs two focused planning blocker fixes.**

The plan chooses the correct product boundary: invoke accepted same-call local-
check composition inside a private reassessment wrapper and bind the complete
fact identity to reassessment identity. Two details must be corrected before
implementation:

1. all deterministic wrapper context must be preflighted before any local
   process starts; and
2. the outcome must not expose an unbound assessment set as independently
   reusable authority.

## 2. Scope Verification

The plan stays within planning scope and does not authorize:

- Rust implementation;
- executor wiring or automatic checks;
- runtime quiet-success activation;
- public APIs, schemas, CLI, UI, onboarding, or examples;
- persistence, events, evidence, reports, or artifacts;
- providers, OpenShell, SideEffects, or writes;
- hosted or distributed behavior; or
- release changes.

## 3. Source-Of-Truth Assessment

The selected sources are correct:

- immutable definitions and identity come from the validated stored bundle;
- the local-check universe and optionality come from canonical stored
  declarations;
- current check observations come through the accepted handler, attestation,
  and same-call gate;
- aggregate check posture and provenance come from the accepted aggregate
  fact;
- other runtime axes remain explicit exact runtime facts; and
- deterministic reassessment remains owned by the existing immutable-bundle
  helper.

The plan correctly refuses detached posture, imported leaf contributions, and
caller-selected local-check optionality.

## 4. Same-Call Composition Assessment

Invoking
`compose_authoritative_docs_check_evidence_check_fact(...)` inside the wrapper
is stronger than accepting a detached aggregate fact. It preserves the current
no-import boundary and gives the wrapper the exact results and fact produced
from the current call.

The plan is honest that this is local same-call composition, not distributed
authenticity, persistence, replay protection, or durable freshness.

## 5. Runtime-Fact Injection Assessment

Rejecting a caller-supplied selected-step evidence/check posture is correct.
Silently overwriting or reconciling it would hide ambiguous authority.

The private replacement helper may change only the selected step's
evidence/check axis. Authority, SideEffect, prior decision, runtime escalation,
and steward minima must remain byte-for-byte semantically unchanged. Existing
public `StepGovernanceRuntimeFacts` behavior remains compatible.

## 6. Binding Identity Assessment

The proposed v1 binding includes the correct identity layers:

- immutable bundle, workflow, run, and step;
- local-check fact algorithm and fingerprint;
- candidate and structural-coverage commitments;
- selected workload-assessment algorithm and input fingerprint; and
- complete assessment-set algorithm and aggregate fingerprint.

This directly prevents equal posture with different fact identity from
collapsing into the same authority boundary. Fixed-width framing, a known
vector, and direct input-invalidation tests are required.

## 7. Monotonicity Assessment

The local-check fact changes only the evidence/check axis. Existing selector
semantics continue to combine all axes monotonically:

- satisfied checks cannot grant authority;
- satisfied checks cannot approve SideEffects;
- satisfied checks cannot lower sensitivity, policy, profile, prior-decision,
  escalation, or steward minima; and
- required-unavailable or failed checks remain denied under the current
  mapping.

The plan does not equate reassessment with enforcement or presentation.

## 8. Planning Blocker: Full Preflight Ordering

The implementation sequence currently invokes local-check composition before
it clearly completes validation of runtime-fact shape, selected-step
ambiguity, and static reassessment context.

That can start a local process before discovering a deterministic mismatch
that was already knowable.

The corrected plan must require a pure preflight stage before composition that
validates:

- exact stored bundle and selected step;
- canonical selected-step declaration source;
- exact runtime-fact count and step membership;
- no duplicate, extra, or missing runtime-fact records;
- selected-step evidence/check posture absence; and
- resolvable immutable workflow, skill, and policy context needed by
  reassessment.

Only after preflight succeeds may the accepted local-check composition start a
process. Runtime observation and final reassessment necessarily remain after
that boundary.

## 9. Planning Blocker: Bound Outcome Authority

The candidate outcome currently returns:

- the aggregate fact;
- the raw `ImmutableBundleGovernanceAssessmentSet`; and
- a separate binding fingerprint.

That shape allows a future crate-internal caller to ignore the binding and
consume the assessment set through existing paths. The plan's purpose is to
make fact identity inseparable from the assessment it influenced.

The corrected plan should introduce one private bound-assessment value that
owns the fact identity, assessment identity, and binding fingerprint. The
outer outcome may also retain bounded check results, but it must not expose the
raw assessment set as independently reusable authority.

Read-only posture accessors may support tests and later review. Any future
executor consumer must accept the bound value or a separately reviewed durable
projection derived from it.

## 10. Privacy And Test Assessment

The privacy policy is appropriate: static errors, redacted `Debug`, bounded
typed posture, no raw commands, output, paths, source, environment, provider,
credential, or report payloads.

The test plan is strong after adding direct proof that:

- every deterministic wrapper mismatch fails before clock or process use; and
- no accessor or conversion returns an unbound assessment set as authority.

Compile-time privacy should carry most of the second property; focused module
tests should exercise only the bound outcome's intended accessors.

## 11. Planning Blockers

1. Reorder and specify complete deterministic wrapper preflight before local
   process execution.
2. Replace the raw assessment-set-plus-fingerprint outcome with one private
   bound-assessment authority value.

## 12. Non-Blocking Follow-Ups

- Keep the first implementation selected-step only.
- Preserve explicit runtime facts for other steps without claiming they are
  authoritative local-check facts.
- Add broader exact per-step check-fact composition only after this path is
  implemented and reviewed.
- Keep executor, persistence, events, and runtime quiet-success behavior for a
  separate consumer phase.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1785019761861428000-2`
- approval:
  `approval/run-1785019761861428000-2/review-scope-approved`
- presentation: `presentation/bddeb258bd473439`
- approval outcome: granted by delegated maintainer through proof enforcement
- review status: completed with two planning blockers
- validation: `npm run check:docs` and `git diff --check`
- out-of-kernel work: source inspection, architecture review, review authoring,
  and documentation validation
- missing coverage: the kernel coordinated governance only; it did not perform
  architecture analysis or generate a WorkReport artifact

## 14. Recommended Next Phase

Run one focused planning blocker-fix phase for the two corrections above, then
perform focused re-review.

Do not authorize Rust implementation until both boundaries are accepted.

## 15. Fix-Forward Status

Both planning blockers are corrected in the
[Authoritative Local-Check Reassessment Binding Plan Blocker Fix Report](AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_BLOCKER_FIX_REPORT.md).

The original findings remain preserved above. Focused re-review accepted both
corrections in the
[Authoritative Local-Check Reassessment Binding Plan Blocker Fix Review](AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_BLOCKER_FIX_REVIEW.md).
