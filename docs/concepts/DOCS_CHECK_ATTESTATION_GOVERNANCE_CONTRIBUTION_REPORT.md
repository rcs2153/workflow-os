# DocsCheck Attestation Governance Contribution Report

## 1. Executive Summary

Workflow OS now has one crate-private same-call wrapper that executes the
accepted `DocsCheck` attestation gate and returns an exact requirement-scoped
governance contribution.

The wrapper does not create aggregate evidence/check satisfaction, invoke
proportional-governance reassessment, or grant executor authority.

## 2. Scope Completed

- Added a dedicated private leaf contribution posture.
- Added an exact requirement-scoped contribution model.
- Added a bounded contribution outcome retaining the structured check result.
- Added a same-call wrapper around the accepted gate.
- Added domain-separated obligation fingerprinting.
- Added redaction-safe Debug behavior.
- Added focused mapping, determinism, identity-substitution, and privacy tests.

## 3. Scope Explicitly Not Completed

- aggregate evidence/check satisfaction;
- proportional-governance reassessment;
- executor integration or workflow mutation;
- automatic or default checks;
- persistence, events, evidence records, runtime reports, or artifacts;
- schemas, CLI, UI, SDK, or examples;
- providers, SideEffects, writes, hosted behavior, or release changes.

## 4. API Summary

`execute_docs_check_governance_contribution(...)` accepts the same private
validated input as the attestation gate. It runs and consumes that gate inside
one call and returns:

- the bounded `LocalCheckResult`; and
- one `DocsCheckGovernanceEvidenceCheckContribution`.

The contribution exposes a redacted obligation fingerprint and a dedicated
private posture. It does not use or expose the aggregate
`GovernanceWorkloadEvidenceCheckPosture` type.

## 5. Mapping Summary

- gate `Satisfied` maps to leaf `Satisfied`;
- unaccepted result status maps to leaf `Failed`; and
- expired required proof maps to leaf `RequiredUnavailable`.

Gate execution or verification errors return no contribution.

## 6. Identity Boundary

The obligation fingerprint is domain-separated and binds:

- immutable bundle identity, version, and integrity root;
- exact step identity; and
- exact attestation requirement fingerprint.

The wrapper accepts no imported gate outcome, proof, obligation fingerprint,
aggregate posture, or assessment input.

## 7. Aggregate Safety

The wrapper invokes no proportional-governance selector and returns no workload
assessment. One contribution therefore cannot erase another failed,
unavailable, or unknown evidence/check obligation.

Aggregate reassessment remains blocked until an authoritative exact obligation
set and fail-closed complete-coverage aggregator are separately planned,
implemented, and reviewed.

## 8. Privacy And Redaction

No raw stdout, stderr, command transcript, executable path, working directory,
environment value, source content, credential, token, or provider payload is
stored. Debug output exposes only result status and leaf posture. Result and
obligation identity remain redacted.

## 9. Test Coverage

Focused tests prove:

- passed gate maps to satisfied contribution;
- failed and timed-out checks map to failed contribution;
- stale proof maps to required-unavailable contribution;
- equal context yields equal obligation identity;
- step or requirement substitution changes identity;
- the process runs once; and
- Debug output does not expose identity or proof values.

## 10. Governed Phase

- workflow: `dg/implement`
- run: `run-1784955753638979000-2`
- approval: `approval/run-1784955753638979000-2/implementation-approved`
- presentation: `presentation/29bb5ebd054bdcb3`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code edits, tests,
  documentation, and validation ran outside the kernel

## 11. Validation

- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed in an isolated target directory;
- `npm run check:docs` - passed; and
- `git diff --check` - passed.

The isolated test wrapper attempted to assign Cargo's exit code to zsh's
read-only `status` parameter after the suite completed. That wrapper command
therefore exited nonzero after Cargo had finished. The complete Cargo log ends
after the final successful doc-test result and contains no failed test result or
Cargo test error; the wrapper defect is not a test failure.

## 12. Remaining Limitations

- no authoritative aggregate obligation set exists;
- no complete-coverage aggregator exists;
- no proportional-governance reassessment consumes contributions;
- no executor checkpoint exists;
- contributions are not persisted or reusable; and
- handler implementation provenance remains registered-unattested.

## 13. Recommended Next Phase

Phase-level maintainer review is complete and accepted with non-blocking
test-depth follow-ups in the
[DocsCheck Attestation Governance Contribution Review](DOCS_CHECK_ATTESTATION_GOVERNANCE_CONTRIBUTION_REVIEW.md).

Next, plan the authoritative evidence/check obligation-set and
complete-coverage aggregation model before any proportional-governance
reassessment or executor checkpoint.
