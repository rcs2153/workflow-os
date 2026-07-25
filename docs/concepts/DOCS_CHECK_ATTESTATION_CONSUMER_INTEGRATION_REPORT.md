# DocsCheck Attestation Consumer Integration Report

## 1. Executive Summary

Workflow OS now has one crate-private, in-memory consumer for independently
verified `DocsCheck` results. The consumer executes and verifies the check in
one call, then returns a typed satisfied or not-satisfied gate disposition.

The gate does not mutate workflow state or grant executor authority. It exposes
only a bounded proof fingerprint after satisfaction and does not expose,
persist, import, cache, serialize, or reuse the accepted proof object.

## 2. Scope Completed

- Added a crate-private `DocsCheckAttestationGateOutcome`.
- Added typed `Satisfied` and `NotSatisfied` dispositions.
- Added bounded reasons for unaccepted result status and freshness expiry.
- Added one same-call wrapper around the reviewed execution and verifier path.
- Rechecked requirement, immutable run, workflow, run, step, invocation,
  result, handler selection, freshness, assurance, and truncation context.
- Reevaluated maximum-age freshness using a distinct Core-owned consumption
  clock sample.
- Preserved failed and timed-out structured results without manufacturing
  proof.

## 3. Scope Explicitly Not Completed

- executor integration or workflow-state mutation;
- automatic or default local checks;
- handler discovery or default registration;
- accepted-proof persistence, import, replay, cache, or serialization;
- events, audit records, evidence attachment, reports, or artifacts;
- proportional-governance mapping or enforcement;
- schemas, CLI, UI, SDK, or example behavior;
- providers, SideEffects, external writes, network access, or hosted behavior;
- release-posture changes.

## 4. Gate API Summary

`execute_docs_check_attestation_gate(...)` accepts the same explicit validated
input as the reviewed runtime-composition helper. It does not accept a separate
proof or caller-asserted gate result.

The read-only outcome retains the bounded `LocalCheckResult`, a typed gate
disposition, and an optional proof fingerprint. The fingerprint is present only
for `Satisfied`; it remains a commitment and is not independent authenticity.

## 5. Satisfaction And Failure Semantics

An accepted current-invocation result satisfies the gate only after the
crate-private verifier returns accepted proof and the consumer rechecks exact
context. A failed or timed-out status outside the accepted requirement returns
`NotSatisfied(ResultStatusNotAccepted)` with no proof fingerprint.

Maximum-age proof that was valid during verification but stale at consumption
returns `NotSatisfied(FreshnessExpired)` with no proof fingerprint. Missing
proof for an accepted status, context mismatch, impossible clock ordering, or
underlying execution/verifier failure returns a stable non-leaking error and no
gate outcome.

`NoReuse` applies only to the exact invocation executed, verified, and consumed
inside the current wrapper call. This API exposes no accepted-proof accessor.

## 6. Privacy And Redaction

The gate stores no raw stdout, stderr, command transcript, executable path,
working directory, environment value, source content, credential, token, or
provider payload. Debug output exposes only result status, typed disposition,
and proof presence. Identities, results, and fingerprints remain redacted.

All new errors use stable codes and static bounded messages without IDs,
hashes, paths, commands, or payloads.

## 7. Test Coverage

Focused tests prove:

- a passed current invocation satisfies and runs the process once;
- failed and timed-out results are typed not-satisfied without proof;
- maximum-age freshness is reevaluated at consumption time;
- expired proof cannot satisfy the gate;
- regressing consumption time fails closed;
- proof fingerprints and identities do not leak through Debug or errors; and
- the existing composition path retains its original four-sample behavior.

## 8. Governed Phase

- workflow: `dg/implement`
- run: `run-1784938762432132000-2`
- approval: `approval/run-1784938762432132000-2/implementation-approved`
- presentation: `presentation/84e40a518fc9b70f`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code edits, tests,
  documentation, and validation ran outside the kernel

## 9. Validation

- focused crate runtime tests: passed, 11 tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

All commands passed. The workspace suite retained its explicit ignored posture
for opt-in live integration tests; no required test was skipped or failed.

## 10. Remaining Limitations

- no executor or proportional-governance consumer exists;
- no accepted-proof persistence, replay, or concurrent claim semantics exist;
- handler implementation provenance remains registered-unattested;
- only the bounded `DocsCheck` command family is supported; and
- the known dogfood phase-close presentation-record read cap is separate.

## 11. Recommended Next Phase

Phase-level maintainer review accepts this gate with non-blocking follow-ups in
[DocsCheck Attestation Consumer Integration Review](DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_REVIEW.md).
Plan one explicit proportional-governance reassessment consumer next. Do not
add an executor checkpoint until that mapping and its semantic-preservation
boundary are reviewed.
