# DocsCheck Attestation Proportional-Governance Integration Plan Report

## 1. Executive Summary

Planning now defines the first bounded translation from the accepted
`DocsCheck` attestation gate into proportional-governance reassessment. The
planned integration is crate-private, pure, in-memory, and review-only.

No runtime enforcement or check execution was implemented in this phase.

## 2. Scope Completed

- Defined the selected pure adapter boundary.
- Defined total gate-disposition mapping.
- Preserved the existing workload assessment as strictness owner.
- Preserved independent execution and disclosure axes.
- Defined immutable-definition invalidation and inference posture.
- Defined failure, privacy, compatibility, and test requirements.
- Proposed a small implementation sequence.

## 3. Scope Explicitly Not Completed

- mapping implementation;
- executor integration or workflow mutation;
- automatic or default checks;
- proof import, persistence, cache, replay, or serialization;
- events, evidence records, reports, or artifacts;
- schema, YAML, CLI, UI, SDK, or example changes;
- providers, SideEffects, writes, hosted behavior, or release changes.

## 4. Selected Mapping

- `Satisfied` maps to `GovernanceWorkloadEvidenceCheckPosture::Satisfied`.
- `NotSatisfied(ResultStatusNotAccepted)` maps to `Failed`.
- `NotSatisfied(FreshnessExpired)` maps to `RequiredUnavailable`.

The distinction between failed execution and unavailable current proof remains
explicit. Both fail closed under the existing selector.

## 5. Semantic Boundary

The future adapter will clone one complete validated assessment input, replace
only its evidence/check fact, and invoke the existing deterministic assessment.
All explicit minima and prior monotonic posture remain intact.

Visible disclosure remains presentation, not separate execution authority. No
new YAML configuration or probabilistic enforcement source is planned.

## 6. Privacy And Safety

The planned helper consumes no raw output or payloads and creates no evidence
record. It trusts only the same-call crate-private gate disposition. The proof
fingerprint remains a commitment and is not reusable authority.

## 7. Governed Phase

- workflow: `dg/d`
- run: `run-1784954548484179000-2`
- approval: `approval/run-1784954548484179000-2/planning-approved`
- presentation: `presentation/e2d49734f75c9468`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; planning, documentation, and
  validation ran outside the kernel

## 8. Validation

- `npm run check:docs` - passed.
- `git diff --check` - passed.
- governed phase status - `Completed` with 39 events, one approval, zero
  retries, and zero escalations.

The repo edits, documentation authoring, shell commands, and validation ran
outside the kernel. The kernel coordinated governance only. Phase close hit the
known 250-record approval-presentation reader cap and reported
`proof_record_read_error`; the approval itself was granted through persisted
presentation-proof enforcement.

## 9. Remaining Limitations

- no gate-to-assessment implementation exists;
- no executor checkpoint consumes independently verified check posture;
- no persisted or asynchronous proof claim semantics exist;
- handler implementation provenance remains registered-unattested; and
- automatic local checks remain unsupported.

## 10. Recommended Next Phase

Focused review found a planning blocker: one leaf DocsCheck outcome cannot
safely replace an aggregate evidence/check fact without complete obligation
coverage. The focused correction now stops at a requirement-scoped
contribution and keeps aggregate reassessment blocked. Perform focused
re-review before implementing that leaf wrapper.
