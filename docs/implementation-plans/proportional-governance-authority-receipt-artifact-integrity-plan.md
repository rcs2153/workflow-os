# Proportional-Governance Authority-Receipt Artifact Integrity Plan

Status: implemented and accepted through the validation-only artifact-integrity
phase. The persisted
receipt-record model and transport-neutral store contract are documented in the
[implementation report](../concepts/PROPORTIONAL_GOVERNANCE_AUTHORITY_RECEIPT_RECORD_STORE_MODEL_REPORT.md)
and [maintainer review](../concepts/PROPORTIONAL_GOVERNANCE_AUTHORITY_RECEIPT_RECORD_STORE_MODEL_REVIEW.md).
The create-only local filesystem store is documented in the
[local-store report](../concepts/PROPORTIONAL_GOVERNANCE_AUTHORITY_RECEIPT_LOCAL_STORE_REPORT.md)
and [focused review](../concepts/PROPORTIONAL_GOVERNANCE_AUTHORITY_RECEIPT_LOCAL_STORE_REVIEW.md).
The explicit artifact-integrity helper is documented in the
[implementation report](../concepts/PROPORTIONAL_GOVERNANCE_AUTHORITY_RECEIPT_ARTIFACT_INTEGRITY_REPORT.md)
and [focused review](../concepts/PROPORTIONAL_GOVERNANCE_AUTHORITY_RECEIPT_ARTIFACT_INTEGRITY_REVIEW.md).
Combined receipt persistence and artifact writing remain separately scoped.

## 1. Executive Summary

Workflow OS can issue one trusted, payload-free decision-time governance
authority receipt after a successful proof-enforced approval reassessment and
can cite that receipt in an in-memory `WorkReport`. The receipt is deliberately
serialize-only: wire input becomes an explicitly unverified claim and cannot
regain trusted status through self-consistency.

The next durability boundary should preserve that distinction. A local
create-only store may accept a trusted in-memory receipt and retain its exact
serialized record, but reads must return a persisted receipt record that is
structurally verified and explicitly not reusable authority. A separate,
opt-in report-artifact integrity helper may then prove that every cited receipt
ID resolves to one matching persisted record for the same immutable run and
approval decision.

This boundary adds durable provenance, not durable permission.

## 2. Goals

- Preserve decision-time receipt provenance across local process boundaries.
- Accept only a trusted in-memory receipt at the write boundary.
- Keep persisted receipt reads explicitly unauthenticated and non-authorizing.
- Use create-only, exact-idempotent local persistence.
- Reject conflicting duplicate receipt identities.
- Validate report-artifact receipt citations against an explicit receipt store.
- Bind each resolved citation to the artifact's workflow and run identity.
- Bind the receipt to its approval reference and approval-decision event.
- Return bounded integrity counts and stable non-leaking errors.
- Reuse existing `WorkReportArtifactStore` and explicit artifact-gate patterns.
- Preserve existing executor, report, artifact, and provider semantics.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic receipt persistence;
- automatic report generation or artifact writing;
- changing existing executor return types or defaults;
- treating a persisted record as current or reusable authority;
- claim-to-trusted receipt conversion;
- cryptographic signing, notarization, or remote attestation;
- workflow events or audit projection for receipt persistence;
- CLI, UI, schema, SDK, or example changes;
- provider or OpenShell integration changes;
- SideEffect execution or new provider mutation families;
- filesystem, provider, or external writes beyond a future explicit local
  receipt-store operation;
- hosted or shared-state expansion;
- reasoning lineage; or
- release posture changes.

## 4. Current Baseline

The accepted baseline includes:

- `GovernanceDecisionAuthorityReceipt`, issued only from the successful
  proof-enforced approval-reassessment path;
- deterministic receipt identity and complete commitment;
- fixed `point_in_time_only`, `local_unsigned`,
  `evidence_only_not_authorization`, and `reference_only` postures;
- `UnverifiedGovernanceDecisionAuthorityReceipt` for structurally valid wire
  claims;
- trusted-receipt-only citation derivation;
- explicit in-memory terminal report composition;
- explicit executor-result propagation retaining decision, receipt, report,
  and report-error posture;
- validated `WorkReportArtifactRecord` and explicit artifact stores; and
- existing explicit SideEffect and approval referential-integrity gates.

The unresolved boundary is that a durable `WorkReportArtifactRecord` can carry
a syntactically valid receipt citation without an independently resolvable
receipt record.

## 5. Trust Boundary

The trusted in-memory receipt and the durable receipt record are intentionally
different trust classes.

The write path may accept:

```text
GovernanceDecisionAuthorityReceipt
```

The read path must return a type whose name and API communicate:

```text
persisted + structurally verified + locally stored
does not equal authenticated current authority
```

A candidate name is:

```rust
PersistedGovernanceDecisionAuthorityReceiptRecord
```

The exact name should follow repository conventions during implementation.
It must not expose a conversion into `GovernanceDecisionAuthorityReceipt`.
Its validation proves deterministic identity and commitment consistency only.

Local store provenance is useful evidence, but a mutable local filesystem is
not a cryptographic issuer. Future signed or hosted receipt verification must
use a separately designed authenticated envelope rather than widening this
type's claims.

## 6. Candidate Persistence Contract

The smallest transport-neutral contract should support exact writes and reads:

```rust
pub trait GovernanceDecisionAuthorityReceiptRecordStore {
    fn write_governance_decision_authority_receipt(
        &self,
        receipt: &GovernanceDecisionAuthorityReceipt,
    ) -> Result<GovernanceDecisionAuthorityReceiptWriteOutcome, WorkflowOsError>;

    fn read_governance_decision_authority_receipt(
        &self,
        receipt_id: &GovernanceDecisionAuthorityReceiptId,
    ) -> Result<Option<PersistedGovernanceDecisionAuthorityReceiptRecord>, WorkflowOsError>;
}
```

The exact API may be adjusted to existing store conventions, but it must:

- accept no public unverified claim at the write boundary;
- expose no generic trust-restoration callback;
- read one exact receipt by stable ID;
- validate stored data before returning it; and
- remain independent of `StateBackend` and `WorkReportArtifactStore` traits.

## 7. Create-Only Semantics

The future local store should use receipt ID as the content-addressed identity.

- First valid write creates one record.
- An exact duplicate is idempotent and returns an explicit already-present
  outcome.
- Different serialized content under the same receipt ID fails closed.
- Corrupt existing content fails closed and is not repaired automatically.
- A failed or partial write must not become visible as a complete receipt.
- Reads must validate the deterministic receipt commitment and ID.
- Listing or discovery is not required for the first slice.

Receipt persistence must occur only after trusted issuance. Persistence failure
must not rewrite the already-completed approval or workflow result.

## 8. Candidate Artifact Integrity Contract

The first artifact boundary should be an explicit validation helper, not a
change to `WorkReportArtifactStore::write_work_report_artifact`:

```rust
pub struct WorkReportArtifactAuthorityReceiptIntegrityInput<'a> {
    pub artifact: &'a WorkReportArtifactRecord,
}

pub fn validate_work_report_artifact_authority_receipt_integrity(
    store: &impl GovernanceDecisionAuthorityReceiptRecordStore,
    input: WorkReportArtifactAuthorityReceiptIntegrityInput<'_>,
) -> Result<WorkReportArtifactAuthorityReceiptIntegrityResult, WorkflowOsError>;
```

The first implementation should validate only. A combined receipt-persist and
artifact-write helper requires a later reviewed phase.

## 9. Integrity Rules

The helper should:

1. Validate the artifact.
2. Extract only
   `WorkReportCitationTarget::GovernanceDecisionAuthorityReceipt` citations.
3. De-duplicate receipt IDs deterministically.
4. Read each exact record through the explicit receipt store.
5. Require the persisted record's receipt ID and commitment to validate.
6. Require its workflow ID and run ID to match artifact metadata.
7. Require the referenced approval ID and approval-decision event ID to remain
   internally committed by the receipt.
8. Return bounded counts for cited, resolved, missing, and duplicate IDs.

V1 must fail closed on missing, corrupt, or mismatched records. Authority
receipt citations assert governed decision provenance, so a permissive
missing-record mode would create a durable artifact with a knowingly dangling
governance reference. If a later product use case needs a missing citation, it
must use the report model's explicit missing-citation posture rather than a
successful artifact-integrity result.

A successful result means only:

```text
Each required receipt citation resolves to a structurally valid persisted
record whose immutable workflow/run identity matches this report artifact.
```

It does not mean the receipt is fresh, signed, externally authenticated, or
authorization for a later action.

## 10. Ordering And Atomicity

For a later explicit combined path, the required order is:

1. Produce the trusted receipt from the accepted proof-enforced decision path.
2. Generate and validate the in-memory WorkReport using that trusted receipt.
3. Construct and validate the `WorkReportArtifactRecord`.
4. Persist or exactly reconcile the receipt record.
5. Validate artifact receipt referential integrity against the store.
6. Run all other caller-selected artifact gates.
7. Write or exactly reconcile the report artifact.

No artifact may be written after receipt integrity fails. If receipt persistence
succeeds and a later artifact gate fails, the receipt may remain as truthful
evidence of the completed decision-time operation; it is not partial workflow
authority and does not require compensating deletion.

## 11. Relationship To Existing Artifact Gates

Authority-receipt integrity should follow the same explicit composition style
as SideEffect and approval-linkage gates:

- stores remain separate;
- policies are caller-visible;
- validation is deterministic;
- no referenced record is copied into the report artifact;
- the helper does not discover or add citations;
- failure occurs before artifact write; and
- existing artifact-store behavior remains unchanged unless the caller opts in.

The new gate must compose with existing artifact requirements without implying
that receipt evidence proves SideEffect completion, provider mutation, or
approval for any later operation.

## 12. Privacy And Redaction

The durable record must retain only the receipt's existing typed references and
commitments. It must not add:

- raw runtime facts;
- policy, approval, evidence, or report bodies;
- provider, command, CI, parser, or sandbox output;
- source or spec contents;
- paths;
- environment values;
- credentials, authorization headers, tokens, or private keys; or
- arbitrary metadata.

Debug output should expose only version, fixed postures, bounded counts, and
presence state. Errors must use stable codes and must not echo receipt IDs,
paths, serialized records, or secret-like values.

## 13. Error Model

Candidate stable error families:

- `governance_decision_authority_receipt_store.record.invalid`;
- `governance_decision_authority_receipt_store.duplicate.conflict`;
- `governance_decision_authority_receipt_store.read.failed`;
- `work_report_artifact.authority_receipt.missing`;
- `work_report_artifact.authority_receipt.identity_mismatch`; and
- `work_report_artifact.authority_receipt.store_failed`.

Implementation should reuse existing error taxonomy where a matching stable
family already exists. Error messages must remain bounded and non-leaking.

## 14. First Implementation Sequence

Recommended small phases:

1. Add the persisted-record model, transport-neutral store trait, and in-memory
   test store.
2. Add create-only local filesystem persistence with exact duplicate and
   conflict behavior.
3. Review the persistence trust boundary.
4. Add the explicit validation-only artifact referential-integrity helper.
5. Review the integrity helper.
6. Only then plan an explicit executor-adjacent receipt-persist/artifact-write
   composition path.

Phases 1 through 5 are complete. `GovernanceDecisionAuthorityReceiptRecordStore` accepts
only a trusted in-memory receipt for writes. Reads return
`PersistedGovernanceDecisionAuthorityReceiptRecord`, whose verification posture
is explicitly `unverified_serialized_claim` and whose effect remains
`evidence_only_not_authorization`. A focused in-memory implementation exists in
tests, and `LocalGovernanceDecisionAuthorityReceiptRecordStore` now implements
the production local create-only boundary. It publishes atomically through a
temporary file and hard link, uses encoded receipt identities as safe file
names, reconciles exact serialized duplicates, and refuses corrupt or
conflicting existing content without repair.

The explicit validation-only artifact referential-integrity helper now resolves
de-duplicated receipt citations through the caller-supplied store, requires
matching workflow/run identity, and returns bounded counts. Missing, corrupt,
or mismatched records fail closed. It does not combine receipt persistence with
artifact writing or change executor defaults.

The focused maintainer review accepted the helper without blockers. The next
prompt should cover phase 6 planning only: an explicit executor-adjacent
receipt-persist and artifact-write composition path with truthful ordering and
partial-failure semantics.

## 15. Test Plan

Future tests should cover:

- trusted receipt accepted by the write boundary;
- public unverified claim cannot be written through the trusted API;
- persisted read remains explicitly non-authorizing;
- deterministic first write and exact duplicate reconciliation;
- conflicting duplicate fails closed;
- corrupt stored record fails closed without leakage;
- missing receipt citation fails closed;
- matching workflow/run identity succeeds;
- mismatched workflow or run identity fails closed;
- duplicate citations are de-duplicated deterministically;
- artifact validation occurs before store access;
- no report or receipt payload is copied into another artifact;
- Debug and serde non-leakage;
- receipt-store failure writes no report artifact;
- existing WorkReport, artifact, approval, SideEffect, executor, state, and
  provider tests remain unchanged; and
- local filesystem and SQLite migration scope remain unchanged until separately
  approved.

## 16. Open Questions

- Should the first persisted-record type retain the complete serialized receipt
  record or a narrower immutable integrity projection?
- Should exact duplicate reconciliation return an enum or preserve existing
  store convention with success/error only?
- Should a later report-generation path use explicit missing-citation
  vocabulary when a receipt was not produced, while keeping persisted receipt
  integrity strict-only?
- When should local receipt records enter state export/migration inventory?
- What authenticated envelope is required before shared or hosted stores may
  claim issuer provenance?
- Should a future artifact contain a receipt-record commitment in addition to
  the stable receipt ID?

## 17. Final Recommendation

Proceed next with planning for one explicit executor-adjacent receipt-persist
and artifact-write composition path. Preserve completed decision truth, define
strict gate ordering and partial-failure semantics, and keep existing executor
defaults unchanged.

Do not add automatic persistence, executor defaults, artifact writes,
providers, OpenShell behavior, SideEffect execution, schemas, CLI/UI surfaces,
hosted expansion, or reusable authority.
