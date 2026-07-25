# DocsCheck Attestation Consumer Integration Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups. Proceed to planning one explicit
proportional-governance reassessment consumer.

## 2. Scope Verification

The phase stayed within the approved crate-private, in-memory consumer scope.
It added no executor integration, automatic or default check execution,
persistence, events, audit projection, evidence attachment, reports beyond the
phase report, artifacts, schemas, CLI behavior, providers, SideEffects, writes,
hosted behavior, or release changes.

## 3. Gate Model Assessment

The outcome is appropriately small and crate-private. It preserves the bounded
`LocalCheckResult`, exposes a typed satisfied or not-satisfied disposition, and
retains only a proof fingerprint after satisfaction. It does not serialize or
expose `AcceptedLocalCheckAttestation`.

Success is not represented as a failure reason. Failed and timed-out statuses
outside the accepted requirement become
`NotSatisfied(ResultStatusNotAccepted)`. Maximum-age expiry becomes
`NotSatisfied(FreshnessExpired)`. These are valid decision outcomes rather than
internal model failures.

## 4. Proof And Context Assessment

The gate calls the reviewed composition and verifier inside the same stack and
accepts no separate proof input. Before satisfaction it rechecks requirement
fingerprint, minimum assurance, immutable run binding, workflow, run, step,
invocation, result identity and status, handler-selection commitment,
freshness policy, and truncation posture.

The gate outcome exposes no accepted-proof accessor. `NoReuse` therefore means
the exact process observation verified and consumed in this wrapper call. The
fingerprint remains a commitment, not independent authenticity or reusable
authority.

## 5. Freshness And Time Assessment

The consumer samples time after verification through the injected Core-owned
clock. Maximum-age policy is reevaluated against observed completion time.
Expiry returns no proof commitment. Consumption before verification or
observation completion fails with a stable error and cannot produce a partial
satisfied outcome.

The freshness comparison preserves the verifier boundary: age equal to the
configured maximum remains valid, while age greater than the maximum expires.

## 6. Runtime Semantics Assessment

The helper borrows explicit inputs, executes the process exactly once, and does
not mutate `WorkflowRun`, snapshots, event history, state backends, reports, or
artifacts. It grants no workflow or executor authority. Existing explicit
composition behavior remains available and unchanged except for a more
accurate borrowed-input signature.

## 7. Privacy And Error Assessment

Debug output includes only result status, typed disposition, and proof
presence. It redacts result and fingerprint values. New gate errors use stable
codes and static messages without IDs, hashes, paths, command details, process
output, source content, environment values, credentials, or provider payloads.

No raw payload storage or new serialization surface was introduced.

## 8. Test Quality Assessment

Focused tests prove current-invocation satisfaction, one process execution,
typed failed/timed-out not-satisfaction, consumption-time expiry, invalid clock
ordering, proof absence on non-satisfaction, and Debug/error non-leakage.

The underlying composition and verifier suites continue to cover exact stored
manifest identity, canonical step/skill resolution, requirement mismatch,
bundle and handler substitution, result mismatch, truncation, duration,
freshness boundary, and accepted-status verifier failure. Full workspace tests
passed.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Future runtime consumers should call the gate, not treat the lower-level
  proof-returning composition helper as execution authority.
- Add consumer-level substitution regression tests when the first real
  proportional-governance or executor consumer creates an independently
  meaningful integration seam.
- Persisted or concurrent proof use still requires one-time claim and replay
  semantics and is not authorized by this phase.
- Handler implementation provenance remains registered-unattested.

## 11. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed; opt-in live tests retained their explicit
  ignored posture.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 12. Governed Review

- workflow: `dg/review`
- run: `run-1784954347954765000-2`
- approval: `approval/run-1784954347954765000-2/review-scope-approved`
- presentation: `presentation/f5883a1b527a18a3`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; review, documentation, and
  validation ran outside the kernel

## 13. Recommended Next Phase

Plan one explicit adapter from `DocsCheckAttestationGateOutcome` into bounded
proportional-governance evidence/check posture and deterministic reassessment.
Keep executor integration, automatic checks, persistence, events, evidence,
reports, artifacts, schemas, CLI, providers, SideEffects, writes, hosted
behavior, and release changes out of that planning phase.

That planning is now documented in the
[DocsCheck Attestation Proportional-Governance Integration Plan](../implementation-plans/docs-check-attestation-proportional-governance-integration-plan.md).
