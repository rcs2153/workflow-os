# DocsCheck Attestation Consumer Integration Plan Review

## 1. Executive Verdict

Needs planning blocker fixes.

The same-call gate boundary is appropriate, but the plan leaves consumption
freshness semantics unresolved and would return accepted proof from a boundary
that claims `NoReuse` posture.

## 2. Scope Assessment

The plan stays within a narrow crate-private, in-memory consumer. It does not
authorize executor integration, automatic checks, default registration,
persistence, events, evidence, reports, artifacts, schemas, CLI behavior,
providers, SideEffects, writes, hosted behavior, or release changes.

## 3. Consumer Boundary Assessment

Wrapping execution, verification, and consumption in one call is the correct
first boundary. It prevents importing caller-created proof and gives failed or
timed-out structured results an explicit gate meaning without changing workflow
state.

## 4. Proof Trust Assessment

The plan correctly trusts only the private accepted type returned by the
crate-private verifier in the same Core call. It correctly rejects fingerprints
as independent authenticity and preserves exact requirement and immutable-run
identity.

The proposed outcome should not expose the accepted proof itself. The accepted
type is cloneable and read-only; returning it would permit later in-memory reuse
even though the first boundary claims `NoReuse`. The gate may expose a bounded
proof fingerprint and satisfaction posture without returning reusable proof.

## 5. Freshness Assessment

The plan requires a consumption-time clock sample, which is correct. It leaves
freshness expiration as an open choice between a typed not-satisfied result and
an error. That choice affects API shape, tests, and future proportional-
governance mapping and must be resolved before implementation.

Expiration should be a typed `NotSatisfied(FreshnessExpired)` result because the
requirement was not met, while impossible or regressing time remains a stable
error. `NoReuse` should mean satisfaction exists only in the current wrapper
result and accepted proof is not returned for subsequent consumption.

## 6. Result Model Assessment

`DocsCheckAttestationGateReason` is described as the payload of
`NotSatisfied`, but its candidate values include `accepted proof`. Success must
not be represented as a not-satisfied reason. The model should instead use:

- `Satisfied`;
- `NotSatisfied(ResultStatusNotAccepted)`; and
- `NotSatisfied(FreshnessExpired)`.

An accepted status without accepted proof is an internal invariant failure from
the composition boundary, not an ordinary gate reason.

## 7. Proportional-Governance Assessment

The plan correctly keeps execution disposition and disclosure independent. A
later reviewed adapter can map satisfaction to `Satisfied`, deterministic check
failure to `Failed`, and expiry/unavailability to `RequiredUnavailable` without
allowing inference to weaken explicit minima.

## 8. Privacy And Compatibility Assessment

The proposed payload-free, non-serialized, crate-private result is appropriate.
No existing executor, handler registry, workflow state, event, report, artifact,
or CLI surface changes.

## 9. Test Plan Assessment

The planned identity, ordering, freshness, no-proof, no-mutation, and privacy
coverage is strong. The test for accepted-status output without proof should
exercise an existing invariant path or be omitted; implementation should not
add an artificial production seam solely to manufacture an impossible state.

## 10. Planning Blockers

1. Resolve freshness expiration as typed not-satisfaction and reserve errors
   for invalid or impossible clock posture.
2. Remove accepted proof from the not-satisfied reason vocabulary.
3. Do not expose the accepted proof from the first gate outcome; expose only
   bounded satisfaction metadata or its proof commitment.
4. Clarify that accepted-status-without-proof remains an internal composition
   failure and does not require an artificial test seam.

## 11. Non-Blocking Follow-Ups

- Later persisted reuse needs one-time claim and concurrency semantics.
- Handler implementation provenance remains registered-unattested.
- Proportional-governance and executor consumers remain separate phases.

## 12. Governed Review

- workflow: `dg/review`
- run: `run-1784929317326191000-2`
- approval: `approval/run-1784929317326191000-2/review-scope-approved`
- presentation: `presentation/885b75b858e288fc`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- kernel boundary: governance coordination only; review, documentation, and
  validation ran outside the kernel

## 13. Recommended Next Phase

Perform a focused planning blocker fix, then re-review the corrected plan before
implementation.

## 14. Fix-Forward Status

The focused plan correction is documented in
[DocsCheck Attestation Consumer Integration Plan Blocker Fix Report](DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_PLAN_BLOCKER_FIX_REPORT.md).
This does not erase the original blocker finding. Focused re-review is required
before implementation.
