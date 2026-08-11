# Proportional-Governance Authority-Receipt Artifact-Write Composition Report

## 1. Executive Summary

Workflow OS now provides one explicit executor-adjacent helper that composes an
already trusted governance decision authority receipt and terminal WorkReport
into durable receipt evidence and a governed local report artifact. The helper
preserves decision and run truth across every later persistence failure and
does not make persistence automatic.

## 2. Scope Completed

- Added `LocalGovernanceAuthorityReceiptArtifactWriteInput`.
- Added bounded result, parts, and persistence-posture types.
- Added `persist_governance_authority_receipt_report_artifact`.
- Constructed and validated the report artifact before any durable write.
- Persisted or exactly reconciled the trusted receipt through the reviewed
  receipt-store contract.
- Validated receipt referential integrity before selected artifact gates.
- Reused the existing SideEffect, approval-linkage, and high-assurance artifact
  write boundary.
- Reconciled exact artifact duplicates while failing closed on conflicts or
  unreadable duplicate state.
- Added stable, non-leaking composition errors and bounded Debug output.
- Exported the additive API through `workflow-core`.

## 3. Scope Explicitly Not Completed

This phase did not add automatic persistence, default executor invocation,
cross-store transactions, workflow or audit events, provider execution,
OpenShell integration, SideEffect execution, CLI/UI behavior, schemas, SDKs,
examples, hosted expansion, reusable authority, or release posture changes.

## 4. API And Ordering Boundary

The caller supplies the trusted Core-owned receipt-bearing report result plus
explicit receipt, artifact, and SideEffect stores. The helper consumes that
owned result, preserves denied or report-failed posture without writes,
constructs the artifact, validates run/workflow identity, persists the trusted
receipt, validates persisted receipt integrity, runs selected existing artifact
gates, and only then attempts artifact storage.

The API does not accept an unverified serialized receipt claim and does not
discover stores, providers, runtime configuration, or hidden global state.

## 5. Partial Failure And Reconciliation

A completed run and granted approval remain completed even when receipt
persistence, receipt integrity, an artifact gate, or artifact persistence later
fails. A successfully persisted receipt remains truthful decision evidence and
is never deleted or promoted into reusable authority.

Receipt duplicates use the reviewed exact-idempotent store contract. Artifact
duplicates are read back and compared: exact content becomes
`AlreadyPersisted`; conflicting content fails closed; unreadable or otherwise
ambiguous durable outcome sets `retry_blocked`.

## 6. Privacy And Error Posture

Composition errors use stable
`executor.governance_authority_receipt_artifact.*` codes. Messages and Debug
output omit receipt, report, run, approval, event, SideEffect, and artifact IDs;
commitments; paths; raw payloads; command output; environment values; and
secret-like values. The result exposes bounded posture and presence only.

## 7. Test Coverage

Ten focused tests cover successful persistence, exact receipt and artifact
reconciliation, denied and report-failed no-write behavior, receipt persistence
failure ordering, receipt-integrity failure after truthful persistence, existing
artifact-gate ordering, ambiguous artifact writes, conflicting duplicates,
unreadable duplicate state, run/event preservation, and error non-leakage.

## 8. Commands Run And Results

- Focused authority-receipt artifact-composition tests: passed, 10 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Full `local_executor` probe: 310 passed, 46 failed, 1 ignored because the
  isolated test PATH did not expose the required `npm` and built
  `workflow-os` executables; after prebuilding the CLI and supplying the
  configured Node runtime, the complete workspace suite passed, including 356
  passing `local_executor` tests and 1 opt-in test ignored.

## 9. Governed Phase Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1786430887198980000-2`
- Approval ID: `approval/run-1786430887198980000-2/implementation-approved`
- Presentation ID: `presentation/cb78d1b81c04fe53`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: Rust implementation, tests, documentation, validation,
  reporting, and git/PR operations

## 10. Remaining Limitations

- Persistence remains explicit and caller initiated.
- Receipt and artifact writes are not one atomic transaction.
- Ambiguous artifact outcomes require operator reconciliation.
- Local unsigned receipt records do not authenticate remote issuer provenance.
- The helper does not enforce every workflow-declared artifact contract.
- No provider action or external SideEffect is authorized by this API.

## 11. Recommended Next Phase

The focused maintainer review is accepted. Return to the roadmap's active
runtime-composition lane. Do not broaden provider mutation families or default
executor persistence from this helper.
