# Proportional-Governance Decision-Time Authority Receipt Plan

Status: accepted for model-only implementation in the
[focused maintainer plan review](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_PLAN_REVIEW.md).

## 1. Executive Summary

Workflow OS can now resume one explicit local approval only after proving both
the exact presented decision and fresh current facts from the registered
runtime-fact source. The successful result contains a payload-free
decision-time snapshot, but that snapshot is call-local and is not represented
as durable report-ready evidence.

The next boundary should create a dedicated, payload-free, evidence-only
decision-time authority receipt. The receipt should explain which immutable
run bundle, approval decision, source commitment, fact-set commitment, and
assessment authorized one approval resume. It must never authorize a later
operation or preserve raw facts.

This plan does not implement the model, persistence, report integration,
schemas, CLI/UI behavior, automatic approval, proportional-governance default
changes, providers, OpenShell, SideEffects, writes, hosted behavior, or release
changes.

## 2. Goals

- Represent one successful proof-enforced, fresh-fact approval-resume decision
  as bounded evidence.
- Bind the receipt to the exact run, approval, immutable bundle, proof marker,
  registered source, decision-time snapshot, and reproduced assessment.
- Derive trusted receipts only from an opaque Core-owned successful outcome.
- Make the receipt point-in-time, evidence-only, local, unsigned, and
  non-authorizing.
- Preserve commitments and references without raw facts or presentation text.
- Keep serialized receipt claims untrusted unless re-established through a
  trusted Core path.
- Prepare a later WorkReport citation target without prematurely changing the
  report model.
- Preserve deterministic reassessment and fail-closed behavior.
- Support quiet-success explainability: fewer prompts must not mean less proof.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- changing the accepted approval-resume wrapper;
- making a receipt reusable authority, permission, grant, lease, or token;
- constructing a trusted receipt from arbitrary public fields;
- treating serialization, hashing, or self-consistency as authentication;
- persisting raw runtime facts, source payloads, presentation content, prompts,
  command output, provider output, credentials, or tokens;
- adding receipt persistence, stores, workflow events, audit projection,
  report citations, report artifacts, or automatic report generation;
- workflow/project schema or SDK changes;
- CLI, UI, approval-card, or hosted rendering;
- automatic approval, model self-approval, or inferred approval authority;
- broad proportional-governance default changes;
- provider execution, OpenShell integration, SideEffect execution, external
  writes, hosted expansion, enterprise identity, or release changes.

## 4. Source-Of-Truth Boundaries

| Concept | Source of truth | Must not become |
| --- | --- | --- |
| Approval presentation | Durable `ApprovalPresentationRecord` and proof marker | Approval authority by itself |
| Approval decision | Exact workflow approval decision event | Reusable grant |
| Current runtime facts | Registered source observed at decision time | Persisted raw report payload |
| Runtime-fact snapshot | Core-validated same-call observation | Authority for a later call |
| Durable V3 assessment binding | Initial immutable provenance expectation | Proof that facts are still fresh |
| Decision-time authority receipt | Bounded evidence of one successful resume decision | Token, capability, lease, or permission |
| WorkReport citation | Reference to the future receipt | Copy of receipt or source payload |

The existing general `AuthorityReceipt` model is specialized to governed
context access and one exact WorkReport metadata read. Reusing its name while
changing its operation, inputs, and trust boundary would blur semantics. The
first implementation should add a dedicated proportional-governance decision
receipt or a carefully versioned operation-specific sibling, not broaden the
existing producer casually.

## 5. Candidate Core Model

The first implementation should be model-only and add the smallest justified
set, likely:

- `GovernanceDecisionAuthorityReceipt`;
- `GovernanceDecisionAuthorityReceiptId`;
- `GovernanceDecisionAuthorityReceiptVersion`;
- `GovernanceDecisionAuthorityReceiptOperationKind` with only
  `ApprovalResumeReassessmentV1`;
- `GovernanceDecisionAuthorityReceiptEffect` with only
  `EvidenceOnlyNotAuthorization`;
- `GovernanceDecisionAuthorityReceiptValidity` with only `PointInTimeOnly`;
- `GovernanceDecisionAuthorityReceiptSignaturePosture` with only
  `LocalUnsigned`;
- `UnverifiedGovernanceDecisionAuthorityReceipt` for deserialized claims; and
- one crate-private opaque construction proof produced only after the accepted
  proof-enforced current-fact approval wrapper succeeds.

Final names should follow repository conventions and make the operation scope
obvious. Do not add a generic extensible map or free-form receipt payload.

## 6. Required Receipt Identity And Bindings

The trusted receipt should bind at least:

- receipt version and deterministic receipt ID;
- workflow ID and run ID;
- approval reference ID and approval decision event ID;
- granted decision posture;
- approval-presentation proof-marker commitment or equivalent bounded proof
  reference;
- immutable run-bundle binding;
- durable V3 governance assessment binding commitment;
- registered source commitment;
- decision-time snapshot commitment;
- decision-time fact-set commitment and count;
- reproduced assessment aggregate fingerprint;
- receipt issuance time;
- freshness posture `fresh_at_issuance`;
- validity `point_in_time_only`;
- signature posture `local_unsigned`;
- effect `evidence_only_not_authorization`;
- reference-only redaction posture; and
- complete receipt commitment from which the receipt ID is derived.

The receipt must not copy source ID, snapshot ID, fact values, approval reason,
presentation content, project paths, command output, provider payloads, tokens,
or credentials.

## 7. Trusted Construction Boundary

The public model must not accept a definition struct that callers can fill with
claims and then treat as trusted. Core should create one opaque single-use
construction proof only after all of these are true in the same call:

1. the exact pending approval was validated;
2. durable presentation proof matched the run, approval, actor, decision, and
   content;
3. the immutable resume plan was frozen;
4. V3 source registration and initial provenance were validated;
5. fresh source facts reproduced the durable assessment;
6. the `ApprovalGranted` event carrying the proof marker was appended; and
7. the resulting event identity and decision-time snapshot are available.

The model constructor should consume that opaque proof. A denial must not emit
an authority receipt because it authorizes no resume. Failed or partial paths
must not emit a receipt.

## 8. Serialization And Trust

Trusted receipts should be serialize-only outside Core. Deserialization should
produce an explicitly unverified claim type, following the existing authority
receipt pattern.

Validating an unverified claim may prove structural consistency and commitment
integrity only. It must not restore trusted status, source freshness, or
execution authority. A future verifier would need separately scoped source and
event attestation.

Debug output should expose only bounded enum posture and presence/count fields.
Receipt IDs, run IDs, approval IDs, event IDs, commitments, and timestamps
should remain redacted.

## 9. Relationship To WorkReport And EvidenceReference

The first model phase should not modify `WorkReportCitationTarget` or create an
`EvidenceReference`. Later phases should proceed separately:

1. add the trusted receipt model and opaque producer proof;
2. review the model and trust boundary;
3. add explicit `AuthorityReceipt` citation vocabulary to WorkReport, using
   only the receipt ID and bounded posture;
4. add a pure citation derivation helper;
5. compose it into one explicit terminal report path; and
6. consider persistence or artifact integrity only after another plan/review.

Reports should cite the receipt, not the raw snapshot. EvidenceReference may
later point to a persisted receipt artifact, but report generation must not
implicitly fabricate evidence or persistence.

## 10. Quiet Success And Product Feedback

Fresh-pull evaluation confirms that Workflow OS now presents an honest local
kernel boundary and that unnecessary ceremony is the main product friction.
The receipt supports the right answer: low-risk work may become quieter while
the kernel preserves inspectable proof of why it proceeded.

Execution disposition and operator disclosure remain independent axes. A local
UI may display quiet-capture decisions live, but UI visibility is not a source
of governance truth. Policy-required visible disclosure must be durably
recorded even when no UI is open.

Configuration should eventually be mostly inferred from safe, validated repo
and workflow facts, with explicit user/steward minima and overrides. Inference
may propose facts or stricter posture, but enforcement must remain
deterministic. Changes to definitions, capabilities, authority, evidence,
sensitivity, SideEffects, or runtime facts must invalidate the previous
assessment and trigger reassessment. Pure probabilistic inference must not
downgrade an explicit requirement.

## 11. Error And Privacy Posture

- Use stable error codes with static, non-leaking messages.
- Reject malformed, missing, duplicate, or inconsistent receipt bindings.
- Fail closed if event identity, proof marker, snapshot, assessment, or bundle
  commitments disagree.
- Do not create a receipt on denial, source failure, reassessment mismatch,
  proof failure, or post-decision inconsistency.
- Do not leak receipt fields through Debug or deserialization errors.
- Do not serialize raw facts or approval-presentation content.

## 12. Test Plan

The future model phase should prove:

1. one valid successful grant proof creates a deterministic trusted receipt;
2. the same exact inputs produce the same commitment and ID;
3. changed run, approval, event, proof marker, bundle, source, fact set,
   snapshot, assessment, or issuance time changes the commitment;
4. a denial cannot produce the construction proof or receipt;
5. failed approval or reassessment paths cannot produce a receipt;
6. arbitrary public fields cannot construct a trusted receipt;
7. trusted receipts are serialize-only outside Core;
8. deserialization yields an unverified claim;
9. unverified validation does not produce trusted authority;
10. unknown versions, operations, effects, validity, signature, or redaction
    values fail closed;
11. commitment and ID mismatch fail closed;
12. Debug and errors do not leak IDs, commitments, timestamps, or secret-like
    values;
13. serialized receipts contain no raw fact, presentation, command, provider,
    credential, or token fields;
14. existing `AuthorityReceipt`, WorkReport, approval proof, runtime-fact, and
    executor tests remain green; and
15. `cargo test --workspace` passes.

## 13. Proposed Implementation Sequence

1. Implement the dedicated decision receipt model and opaque Core construction
   proof only.
2. Perform a focused maintainer review.
3. Add WorkReport citation vocabulary in a separate phase.
4. Add pure receipt-to-citation derivation.
5. Compose citation into one explicit report path.
6. Plan persistence and artifact referential integrity separately.
7. Revisit quiet-success defaults only after evidence completeness is proven.

## 14. Open Questions

- Should the dedicated model share a top-level receipt envelope with the
  existing context-access `AuthorityReceipt`, or remain a sibling until more
  operations establish a stable common core?
- Is the approval event ID sufficient as the operation outcome reference, or
  should the completed resume event also be committed?
- Should one receipt bind the granted decision only or the complete terminal
  resumed execution outcome?
- Which sensitivity should apply when source facts and approval presentation
  have different classifications?
- Should a future WorkReport cite the receipt ID only, or both receipt and
  approval decision event?
- What trusted verifier, if any, could promote an unverified serialized claim
  without introducing cryptographic or hosted scope?

## 15. Acceptance Criteria

- The plan preserves receipts as evidence, never authorization.
- Trusted construction is impossible without one exact successful
  proof-enforced fresh-fact approval resume.
- Raw facts and presentation content remain absent.
- WorkReport citation and persistence remain later phases.
- Existing approval and proportional-governance defaults remain unchanged.
- The first implementation is model-only and reviewable.

## 16. Recommended Next Phase

Implement the `GovernanceDecisionAuthorityReceipt` core model and its opaque
successful-outcome construction proof only. Do not add WorkReport citation,
persistence, schemas, providers, OpenShell, SideEffects, writes, hosted
behavior, or default changes in that phase.

## 17. Governed Planning Evidence

- Dogfood workflow: `dg/d`
- Run ID: `run-1786294417254823000-2`
- Approval ID: `approval/run-1786294417254823000-2/planning-approved`
- Presentation ID: `presentation/cc02de09950dc09e`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof
- Approved scope: planning, roadmap, and focused plan review only
- Phase status: `Completed` with 39 events, 1 approval, 0 retries, and 0
  escalations
- Validation: `npm run check:docs` and `git diff --check` passed
- Out-of-kernel work: source inspection, planning, documentation, validation,
  and git/PR work
