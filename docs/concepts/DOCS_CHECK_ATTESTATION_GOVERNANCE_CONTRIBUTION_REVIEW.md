# DocsCheck Attestation Governance Contribution Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation safely converts one same-call `DocsCheck` attestation gate
outcome into one exact requirement-scoped governance contribution. It does not
claim aggregate evidence/check satisfaction, invoke proportional-governance
reassessment, or create executor authority.

## 2. Scope Verification

The phase stayed within the approved private in-memory boundary. It added one
private leaf posture, one requirement-scoped contribution, one bounded outcome,
one same-call wrapper, focused tests, and honest documentation.

It did not add aggregate assessment, executor integration, persistence, events,
evidence records, reports, artifacts, schemas, CLI behavior, providers,
SideEffects, writes, hosted behavior, or release changes.

## 3. Model Assessment

`GovernanceEvidenceCheckContributionPosture` is a dedicated private leaf type
with only `Satisfied`, `Failed`, and `RequiredUnavailable`. It deliberately does
not reuse `GovernanceWorkloadEvidenceCheckPosture`, so a leaf result cannot be
mistaken for the aggregate workload fact.

`DocsCheckGovernanceEvidenceCheckContribution` stores only a domain-separated
obligation fingerprint and leaf posture. The bounded outcome retains the
structured `LocalCheckResult`; it does not retain raw process output or accepted
proof.

## 4. Same-Call Consumption Assessment

`execute_docs_check_governance_contribution(...)` invokes the accepted gate in
the same call stack and consumes its private disposition immediately. Callers
cannot import a gate result, accepted proof, obligation fingerprint, aggregate
posture, or assessment input.

Process execution occurs once. Errors from execution, binding, observation,
verification, or gate consumption return before a contribution is constructed.

## 5. Mapping Assessment

The mapping is total for the current gate vocabulary:

- `Satisfied` becomes leaf `Satisfied`;
- `ResultStatusNotAccepted` becomes leaf `Failed`; and
- `FreshnessExpired` becomes leaf `RequiredUnavailable`.

There is no wildcard or default-to-satisfaction branch. A future gate reason
will require an explicit compile-time mapping decision.

## 6. Identity Assessment

The obligation fingerprint is domain-separated and length-framed. It binds the
stored immutable bundle ID, bundle version, integrity root, exact step ID, and
exact requirement fingerprint.

The step is resolved against the stored canonical workflow before execution,
and requirement-to-command compatibility is checked before process execution.
The contribution therefore cannot be produced for an unresolved step or a
requirement targeting a different command.

## 7. Aggregate-Safety Assessment

The implementation does not import or call the proportional-governance
selector and does not produce `GovernanceWorkloadAssessment` or aggregate
`evidence_and_checks` posture. One successful contribution therefore cannot
erase another failed, unavailable, missing, or unknown obligation.

Aggregate reassessment remains correctly blocked until a separately reviewed
authoritative obligation set and fail-closed complete-coverage aggregator
exist.

## 8. Privacy And Error Assessment

Debug output exposes result status and leaf posture while redacting result,
proof, and obligation identity. The contribution stores no command transcript,
path, working directory, environment value, source content, credential, token,
provider payload, stdout, or stderr.

The wrapper adds no new user-facing error payload. Existing runtime and gate
errors use stable codes and static non-leaking messages.

## 9. Test Quality Assessment

Focused tests cover satisfied, failed, timed-out, and stale mappings; equal-input
determinism; step and requirement identity substitution; single process
execution; and Debug redaction. Existing verifier, gate, immutable-bundle,
proportional-governance, executor, provider, and workspace suites remain green.

Two non-blocking test-depth gaps remain:

- add a direct wrapper-level assertion that changing the immutable bundle
  identity changes the obligation fingerprint; and
- add a direct wrapper-level assertion that a gate error returns no
  contribution.

The implementation already has both properties by construction and underlying
gate coverage, so these are hardening follow-ups rather than blockers.

## 10. Documentation Assessment

The plan and report accurately distinguish leaf contribution from aggregate
satisfaction and explicitly preserve execution and disclosure as independent
axes. They also keep automatic checks, executor defaults, persistence, schemas,
providers, SideEffects, and writes out of scope.

## 11. Validation

- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed in an isolated target directory;
- `npm run check:docs` - passed; and
- `git diff --check` - passed.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add the two direct wrapper-boundary regression tests listed above.
- Preserve the private leaf type until complete-coverage aggregation has its own
  reviewed model.
- Do not expose or persist contribution identity before replay and claim
  semantics are designed.

## 14. Governed Review

- workflow: `dg/review`
- run: `run-1784958997223313000-2`
- approval: `approval/run-1784958997223313000-2/review-scope-approved`
- presentation: `presentation/e252078c723b2315`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code inspection,
  documentation, and validation ran outside the kernel

## 15. Recommended Next Phase

Planning for the authoritative evidence/check obligation-set and fail-closed
complete-coverage aggregation model is documented in the
[Evidence And Check Obligation-Set Aggregation Plan](../implementation-plans/evidence-check-obligation-set-aggregation-plan.md).

Review that plan before connecting leaf contributions to the aggregate
proportional-governance selector or an executor checkpoint.
