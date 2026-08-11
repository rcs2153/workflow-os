# Proportional-Governance Authority-Receipt Artifact-Write Composition Plan

Status: accepted for a narrow implementation phase after planning review. This
document plans behavior only; it does not implement receipt persistence or
artifact writing from an executor path.

## 1. Executive Summary

Workflow OS can issue a trusted, payload-free decision-time governance
authority receipt, compose it into an in-memory `WorkReport`, persist it through
a create-only receipt store, and validate report-artifact receipt citations
against that store. These reviewed primitives remain separate.

The next implementation should add one explicit executor-adjacent composition
helper. It should consume the existing receipt-bearing decision/report result,
construct a report artifact, persist or exactly reconcile the trusted receipt,
validate receipt referential integrity, run the existing caller-selected
artifact gates, and write or exactly reconcile the artifact.

The helper must preserve partial-failure truth. A completed approval and
workflow result stay completed if receipt persistence, integrity validation, an
artifact gate, or artifact storage later fails. A persisted receipt may remain
as truthful decision evidence even when no artifact is written.

This plan does not change executor defaults, make persistence automatic, add a
cross-store transaction, or authorize provider execution.

## 2. Goals

- Compose already-reviewed receipt, report, artifact, and store boundaries.
- Accept only the trusted receipt emitted by the proof-enforced decision path.
- Keep the path explicit, local, caller-supplied, and executor-adjacent.
- Construct and validate the artifact before durable writes begin.
- Persist receipts with create-only, exact-idempotent semantics.
- Validate every cited receipt before artifact storage.
- Reuse existing SideEffect, approval-linkage, and high-assurance artifact gates.
- Reconcile exact artifact duplicates without weakening conflicting-duplicate
  behavior.
- Preserve the complete decision, receipt, report, artifact, persistence, and
  error posture in one bounded result.
- Keep errors and Debug output stable and non-leaking.

## 3. Non-Goals

Do not implement in this planning phase:

- Rust runtime code or tests;
- automatic receipt persistence or artifact generation;
- changes to `LocalExecutor::execute(...)` or other default executor methods;
- one transaction spanning receipt and artifact stores;
- deletion or compensation of truthful persisted receipts;
- conversion of persisted records into trusted authority;
- reusable, ambient, or future-action authority;
- workflow events, audit projection, or observability emission;
- CLI/UI behavior, schemas, SDK changes, or examples;
- provider calls, provider mutations, or OpenShell integration;
- SideEffect execution or new mutation families;
- hosted or shared-state expansion;
- reasoning lineage; or
- release posture changes.

## 4. Accepted Baseline

The accepted baseline includes:

- proof-enforced decision-time issuance of
  `GovernanceDecisionAuthorityReceipt`;
- `LocalCurrentRuntimeFactsGovernanceAuthorityReceiptDecisionResult`;
- `compose_governance_authority_receipt_decision_report(...)` and
  `LocalGovernanceAuthorityReceiptReportResult`;
- trusted-receipt-only WorkReport citation derivation;
- `WorkReportArtifactRecord::new(...)`;
- `GovernanceDecisionAuthorityReceiptRecordStore` with trusted write input;
- `GovernanceDecisionAuthorityReceiptWriteOutcome::{Written, AlreadyExists}`;
- create-only local receipt storage;
- `validate_work_report_artifact_authority_receipt_integrity(...)`;
- explicit SideEffect, approval-linkage, high-assurance, provider-candidate,
  and approval-proof-marker artifact gates;
- explicit `WorkReportArtifactStore` implementations; and
- exact artifact duplicate reconciliation in the authoritative artifact path.

No accepted helper currently composes the receipt-bearing decision result into
both durable receipt evidence and a governed report artifact.

## 5. Recommended Boundary

Add one additive helper in the executor-adjacent module, tentatively:

```rust
persist_governance_authority_receipt_report_artifact(...)
```

The exact name should follow implementation conventions. The helper should
consume an owned `LocalGovernanceAuthorityReceiptReportResult` so callers cannot
inject a public unverified receipt claim or a prebuilt authority citation.

Store dependencies should remain explicit function parameters:

- `GovernanceDecisionAuthorityReceiptRecordStore`;
- `WorkReportArtifactStore`; and
- `SideEffectRecordStore`.

The input should carry only explicit artifact gate policy and any already
reviewed gate dependencies. It must not discover stores, configuration,
workflow definitions, events, projections, providers, or hidden global state.

## 6. Candidate Input And Result Model

A candidate input is:

```rust
pub struct LocalGovernanceAuthorityReceiptArtifactWriteInput<'a> {
    pub report_result: LocalGovernanceAuthorityReceiptReportResult,
    pub require_all_side_effect_citations: bool,
    pub require_approval_references_for_requires_approval: bool,
    pub require_decision_for_approved_or_denied: bool,
    pub high_assurance_disclosure_policy:
        WorkReportArtifactHighAssuranceDisclosurePolicy,
    pub provider_integration: ReportArtifactWriteProviderIntegration<'a>,
}
```

The first implementation may constrain `provider_integration` to `None` if
including the existing validation-only selector would widen the review surface.
It must not call a provider either way.

The result should retain:

- the complete approval decision and terminal run;
- the trusted receipt when issued;
- the in-memory report when generated;
- report-generation error posture;
- the constructed artifact when valid;
- receipt write outcome or bounded receipt-persistence error;
- receipt-integrity result or bounded integrity error;
- existing artifact-gate result posture;
- artifact persistence posture: written, exactly already present, not attempted,
  or failed; and
- one bounded next-action/retry posture where durable outcome is ambiguous.

Debug output must expose only terminal status, presence flags, bounded counts,
posture enums, and error codes.

## 7. Required Ordering

The implementation order is fixed:

1. Consume the accepted receipt-bearing decision/report result.
2. Preserve denied or report-failed results without persistence attempts.
3. Construct and validate `WorkReportArtifactRecord` from the validated report.
4. Require artifact workflow/run identity to match the terminal run and receipt.
5. Persist or exactly reconcile the trusted receipt.
6. Validate artifact authority-receipt referential integrity through the
   explicit receipt store.
7. Run every caller-selected existing artifact gate.
8. Write or exactly reconcile the report artifact.
9. Return the complete bounded outcome without mutating workflow truth.

No artifact write may occur before steps 1 through 7 succeed.

## 8. Applicability Rules

- A denied approval has no trusted receipt and no authority-receipt report; the
  helper returns not-applicable posture and performs no writes.
- A granted result without a receipt is invalid for this specialized path and
  fails closed without writing.
- A report-generation error is preserved; no receipt or artifact write occurs.
- An invalid artifact or identity mismatch fails before durable writes.
- Only the trusted receipt already present in the Core-owned result may be
  passed to the receipt store.
- The helper must not accept `UnverifiedGovernanceDecisionAuthorityReceipt` or
  `PersistedGovernanceDecisionAuthorityReceiptRecord` as authority input.

## 9. Receipt Persistence And Reconciliation

Receipt persistence uses the reviewed store contract:

- `Written` means the receipt was newly persisted.
- `AlreadyExists` means exact create-only reconciliation succeeded.
- conflicting or corrupt duplicates fail closed.
- a persistence error prevents receipt-integrity validation and artifact write.

The helper must not delete a persisted receipt after a later failure. The
receipt truthfully records the decision-time operation and is not a partially
granted capability.

## 10. Receipt Integrity Gate

After receipt persistence succeeds or exactly reconciles, call
`validate_work_report_artifact_authority_receipt_integrity(...)` against the
same explicit store.

The integrity result proves only that all authority-receipt citations resolve
to structurally valid, non-authorizing records matching artifact workflow/run
identity. It does not prove freshness, signature, issuer authentication,
operator observation, or authority for another action.

Missing, corrupt, unreadable, or mismatched records fail before artifact write.

## 11. Existing Artifact Gates

Do not duplicate existing gate logic. After receipt integrity passes, delegate
to the accepted governed artifact-write boundary:

- generic explicit integration for SideEffect integrity, approval linkage, and
  high-assurance disclosure; or
- the existing proof-marker governed write boundary when separately supplied
  projection inputs are included in a later reviewed extension.

The first implementation should compose only the smallest already-required gate
set. Approval-proof-marker persistence and workflow-derived policy discovery
must not be pulled in implicitly.

Receipt integrity is additive. It does not imply SideEffect completion,
provider success, approval proof-marker presence, or policy satisfaction.

## 12. Artifact Persistence And Reconciliation

`WorkReportArtifactStore::write_work_report_artifact(...)` remains create-only.
If it returns the existing duplicate error, the helper should read the exact
artifact by run/report identity:

- an exactly equal artifact becomes `AlreadyPersisted` success;
- conflicting content fails closed;
- absent or unreadable duplicate state becomes bounded ambiguous failure;
- retry must be blocked when durable outcome cannot be proven.

Do not weaken existing store semantics or add an upsert path.

## 13. Partial-Failure Truth Table

| Failure point | Approval/run truth | Receipt posture | Artifact posture |
| --- | --- | --- | --- |
| denied/no receipt | preserved | not written | not attempted |
| report generation | preserved | not written | not attempted |
| artifact construction/identity | preserved | not written | not attempted |
| receipt persistence | preserved | failed/unknown | not attempted |
| receipt integrity | preserved | persisted or reconciled | not written |
| later artifact gate | preserved | persisted or reconciled | not written |
| artifact store write | preserved | persisted or reconciled | failed or ambiguous |
| exact artifact duplicate | preserved | persisted or reconciled | already persisted |

No post-decision persistence failure may become a workflow failure, revoke an
approval, append compensating events, or hide the successful decision.

## 14. Error And Privacy Posture

Use stable composition error families, for example:

- `executor.governance_authority_receipt_artifact.report_unavailable`;
- `executor.governance_authority_receipt_artifact.receipt_unavailable`;
- `executor.governance_authority_receipt_artifact.receipt_persistence_failed`;
- `executor.governance_authority_receipt_artifact.receipt_integrity_failed`;
- `executor.governance_authority_receipt_artifact.artifact_write_failed`; and
- `executor.governance_authority_receipt_artifact.artifact_reconciliation_failed`.

Reuse narrower existing error codes inside bounded result accessors where
appropriate. Messages and Debug output must not include receipt, report, run,
approval, event, SideEffect, or artifact IDs; commitments; paths; source or
spec contents; provider payloads; command output; environment values; tokens;
credentials; or serialized records.

## 15. Future Test Plan

The implementation phase should prove:

1. granted receipt-bearing report result persists the receipt and artifact;
2. exact receipt duplicate reconciles and still permits artifact validation;
3. exact artifact duplicate reconciles successfully;
4. denied/no-receipt posture writes neither store;
5. report failure writes neither store;
6. invalid artifact identity writes neither store;
7. receipt-store failure prevents artifact write and preserves decision truth;
8. receipt-integrity failure prevents artifact write;
9. each selected existing artifact gate still fails before artifact write;
10. artifact-store failure retains persisted receipt and terminal run truth;
11. conflicting or unreadable artifact duplicates fail closed;
12. the helper consumes the trusted Core result and accepts no unverified claim;
13. no workflow, snapshot, or event history mutation occurs;
14. no provider, OpenShell, CLI, or hidden state access occurs;
15. Debug and errors do not leak identifiers, paths, payloads, or secret-like
    values; and
16. existing receipt, WorkReport, artifact, executor, SideEffect, approval,
    provider, OpenShell, state, and hosted tests remain unchanged.

## 16. Implementation Sequence

1. Add the bounded input, result, and persistence-posture model.
2. Add the explicit helper consuming
   `LocalGovernanceAuthorityReceiptReportResult`.
3. Construct and validate the artifact before writes.
4. Persist/reconcile the receipt and run the receipt-integrity gate.
5. Delegate to the smallest existing governed artifact-write helper.
6. Reconcile exact artifact duplicates through the store read API.
7. Add focused success, failure-ordering, idempotency, mutation, and privacy tests.
8. Run full repository validation.
9. Perform a focused maintainer review before broader executor or provider use.

## 17. Deferred Work

Deferred until separately planned and reviewed:

- automatic invocation from approval or executor defaults;
- full workflow-declared artifact contract enforcement;
- proof-marker projection persistence inside this helper;
- one atomic transaction across stores;
- receipt/artifact event or audit projection;
- CLI/UI inspection or export;
- schemas, SDKs, and examples;
- remote, signed, or hosted receipt provenance;
- provider or OpenShell execution;
- SideEffect execution or new provider mutation families;
- reusable authority; and
- release posture changes.

## 18. Final Recommendation

Proceed with the explicit executor-adjacent receipt-persist and artifact-write
composition helper only. Keep it local, opt-in, caller-supplied, and
non-authorizing. The implementation must preserve completed workflow and
approval truth across every later persistence failure and must not broaden
provider or default executor behavior.
