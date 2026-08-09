# Proportional-Governance Runtime-Fact Snapshot Commitment Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation closes the identified process-boundary provenance gap while
preserving the more important rule that current facts must be resolved again
for each retry or future decision-time operation.

## 2. Scope Verification

The phase stayed within the approved local model, persistence, and retry scope.
It did not add approval-resume consumption, authority reuse, defaults,
disposition enforcement, checks, provider execution, OpenShell, SideEffects,
writes, schemas, CLI behavior, hosted behavior, or mutation expansion.

## 3. Model Assessment

The V1 snapshot binding is domain-neutral, payload-free, bounded, versioned,
and self-validating. It commits every input needed to prove which accepted
source observation established the assessment without serializing the fact
vector itself.

Assessment-binding V3 is an appropriate compatibility boundary. V1 and V2
retain their existing shapes and semantics, while V3 requires exactly one
runtime-fact snapshot binding and rejects mixed source forms.

## 4. Durability Assessment

Nesting the source commitment inside the existing create-only assessment
binding preserves atomicity across local storage and the
`GovernanceAssessmentBound` event. A separate store or event would add partial
write states without adding useful authority.

The outer binding validates exact immutable-bundle and assessment-aggregate
agreement with the nested record. Corrupt nested commitment data fails during
store deserialization before source observation, new events, or execution.

## 5. Retry Assessment

Retry correctly resolves a fresh snapshot. It does not require the new snapshot
ID, observation time, or fact-set commitment to equal the initial observation;
instead it requires the same trusted source registration and exact bundle to
produce the same accepted assessment. This permits honest freshness while
preserving governance consistency.

The durable initial commitment is returned unchanged and is never rebound by a
retry. Changed facts or registration fail closed without appending events or
re-executing a skill.

## 6. Privacy And Error Assessment

Serialization contains bounded metadata and commitments, not raw facts or
source payloads. Debug output redacts identities, bundles, commitments, and
timestamps. Unknown versions, tampered commitments, source failures, and
executor mismatches use fixed messages that do not echo caller values.

## 7. Compatibility Assessment

Existing V1 plain assessment bindings and V2 authoritative local-check bindings
remain valid. Existing executor entry points and default behavior are unchanged.
The new source-backed path alone creates V3.

## 8. Test Quality Assessment

Tests cover model construction, serde, unknown versions, corruption, payload
absence, Debug safety, V3 persistence, initial snapshot linkage, equivalent
retry, changed assessment, changed source registration, stable event history,
and no duplicate execution. Workspace validation remains the regression gate.

## 9. Blockers

None for the durable source-snapshot commitment phase.

## 10. Non-Blocking Follow-Ups

- Add report citation vocabulary for the durable commitment only when a report
  consumer is separately scoped.
- Preserve current-source resolution at every operation boundary.
- Keep source authentication and remote attestation separate from this local
  registration contract.

## 11. Product Feedback Reconciliation

Fresh-pull feedback confirms that the kernel's honesty and first-run path are
credible, while low-risk ceremony remains the next product problem. This phase
supports quiet-success work by making current-fact provenance durable without
adding prompts or defaults. It does not itself decide when quiet execution is
appropriate. Previously reported Node 24 integration-check opacity and the
duplicate missing-manifest diagnostic have already been fixed and are not
reopened here.

## 12. Recommended Next Phase

Approval-resume current-runtime-fact source consumption, separately governed
and reviewed. Provider mutations and default activation should remain later.

## 13. Governed Review Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786281020359413000-2`
- Approval ID: `approval/run-1786281020359413000-2/composition-approved`
- Presentation ID: `presentation/179d2d66df12050c`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Approval-presentation enforcement: proof enforced with event marker present
- Out-of-kernel work: implementation, tests, documentation, validation, and
  git/PR operations
- Validation reviewed: focused tests, full fmt, workspace clippy, workspace
  tests, docs check, and diff check passed
