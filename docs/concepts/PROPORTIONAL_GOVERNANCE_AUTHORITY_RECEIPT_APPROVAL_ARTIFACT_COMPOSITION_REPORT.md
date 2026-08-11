# Proportional-Governance Authority-Receipt Approval-Artifact Composition Report

## 1. Executive Summary

Workflow OS now provides one explicit Core-owned call that applies a
proof-enforced approval decision with fresh registered runtime facts, derives
the trusted decision-time authority receipt, generates the terminal
receipt-citing WorkReport, persists the receipt, validates referential
integrity and existing artifact gates, and writes the report artifact.

The path is local, opt-in, and dependency-injected. Existing executor and CLI
defaults remain unchanged.

## 2. Scope Completed

- Added `LocalGovernanceAuthorityReceiptArtifactDecisionInput`.
- Added
  `decide_approval_with_governance_authority_receipt_report_artifact`.
- Reused the accepted proof-enforced approval-resume, trusted receipt,
  receipt-citing report, receipt-store, integrity, artifact-gate, and
  artifact-store boundaries without duplicating their semantics.
- Kept pre-decision failures as `Err` and post-decision failures inside the
  existing bounded artifact-write result.
- Exported the additive API through `workflow-core`.
- Added focused grant, denial, missing-proof, report-failure, ordering,
  no-mutation, and privacy regression coverage.

## 3. Scope Explicitly Not Completed

This phase did not add default executor or CLI consumption, automatic
approval, automatic persistence for existing paths, cross-store transactions,
provider or OpenShell execution, SideEffect execution, new mutations, reusable
authority, schemas, SDK changes, examples, hosted expansion, or release
changes.

## 4. Runtime Composition Boundary

The helper accepts the exact existing approval-presentation decision request
plus explicit report and artifact-gate inputs. It receives every executor,
source, and store dependency explicitly. It first applies the accepted
proof-enforced fresh-fact approval path, then passes only the Core-produced
trusted result into the existing report and persistence helpers.

No public serialized receipt claim, caller-built citation, detached posture,
or inferred dependency can enter the trusted path.

## 5. Decision And Failure Semantics

Missing or stale presentation proof, immutable-context mismatch, source
failure, stale facts, or changed assessment fails before approval mutation.
Denial remains source-free at decision time and writes no receipt or artifact.
Once a grant completes, report or persistence failure cannot change the
terminal workflow or approval result.

Report failure writes neither store. Receipt failure blocks artifact writing.
Receipt-integrity and selected existing artifact gates run before artifact
storage. Existing exact-duplicate and ambiguous-outcome posture remains
unchanged.

## 6. Privacy And Redaction

The new input Debug representation redacts approval-decision and report inputs.
The result retains the existing bounded Debug contract. Error and Debug output
omit IDs, commitments, paths, report contents, runtime facts, command output,
environment values, credentials, and secret-like values.

## 7. Test Coverage

Four direct end-to-end tests cover successful grant closure, denial without
decision-time source access or writes, missing presentation proof before source
or mutation, and report failure after a truthful terminal grant. The existing
ten receipt-artifact tests continue to cover exact duplicates, conflicts,
integrity failure, gate ordering, ambiguous outcomes, and non-leakage.

## 8. Commands Run And Results

- Focused end-to-end composition tests: passed, 4 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Governed Phase Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1786433276572547000-2`
- Approval ID:
  `approval/run-1786433276572547000-2/implementation-approved`
- Presentation ID: `presentation/969fd015a372ca6b`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: Rust implementation, tests, documentation, validation,
  reporting, and git/PR operations

## 10. Remaining Limitations

- The API remains explicit and caller initiated.
- Receipt and artifact stores do not share one transaction.
- Ambiguous artifact outcomes require operator reconciliation.
- The local unsigned receipt does not authenticate a remote issuer.
- No default runtime or product-facing consumer uses this API.
- No provider action or external SideEffect is authorized.

## 11. Recommended Next Phase

Return to the active runtime roadmap after focused maintainer review. Keep the
helper opt-in and require a separately reviewed consumer before changing any
default or external-effect behavior.
