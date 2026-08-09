# Proportional-Governance Approval-Resume Runtime-Fact Source Consumer Plan

Implementation status: implemented and accepted in the explicit local,
opt-in boundary described here. The implementation report and maintainer review
record the final API, validation, and remaining limitations.

## 1. Executive Summary

The registered current runtime-fact source, explicit local executor consumer,
and durable initial source-snapshot commitment are implemented and reviewed.
Fresh execution and exact retry can therefore derive proportional-governance
facts from one trusted registered source while preserving the provenance of the
observation that established the run's durable assessment.

Approval resume remains the unresolved operation boundary. The current
assessment-bound approval helper accepts a caller-assembled runtime-fact vector.
This phase will add one explicit opt-in approval-resume consumer that resolves
fresh facts from the same registered source, validates them against the exact
immutable run bundle and durable V3 assessment binding, and only then permits a
granted approval to append decision or resume events.

The implemented consumer does not activate proportional governance by default,
reuse an old snapshot as authority, persist raw facts, change schemas or CLI
behavior, invoke providers, integrate OpenShell, execute SideEffects, or add
writes.

## 2. Goals

- Resolve decision-time facts from an explicitly registered source for the
  exact approval-bound immutable run bundle.
- Prove that the supplied source registration matches the durable initial
  source-registration commitment before invoking the source.
- Validate source identity, contract version, exact bundle identity, freshness,
  complete step coverage, and resulting assessment in one Core-owned call.
- Require the fresh assessment to reproduce the durable V3 assessment binding
  before any granted-decision, resume, policy, or skill event is appended.
- Preserve the durable initial snapshot commitment as provenance without
  replacing it with the decision-time observation.
- Return the accepted payload-free decision-time snapshot for later evidence
  or report composition without persisting it as reusable authority.
- Keep denied decisions available without requiring a runtime-fact source call.
- Preserve existing approval, presentation-proof, resolved-context,
  idempotency, and workflow semantics.
- Use stable non-leaking errors and redaction-safe Debug output.

## 3. Non-Goals

- Default or automatic activation.
- Replacing existing approval APIs.
- Treating the initial or decision-time snapshot as reusable authority.
- Persisting raw runtime facts or source payloads.
- Rebinding or overwriting the durable initial source-snapshot commitment.
- Weakening explicit workflow, policy, profile, authority, evidence/check,
  sensitivity, SideEffect, or steward requirements.
- Automatically granting an approval because reassessment succeeds.
- Requiring a source call before a human or delegated maintainer may deny work.
- Workflow or project schema changes.
- CLI or UI behavior.
- Provider execution, OpenShell integration, SideEffect execution, external
  writes, hosted expansion, or additional mutation families.
- Report citation or artifact persistence for the decision-time snapshot.
- Remote source authentication, attestation, enterprise identity, or reusable
  capability grants.
- Release posture changes.

## 4. Current Boundary

Current explicit source-backed execution is implemented by
`execute_with_current_runtime_facts_governance_assessment_binding(...)`. It:

1. persists or validates the immutable run bundle;
2. resolves the registered source exactly once;
3. validates source and observation freshness;
4. constructs assessment-binding V3 with an initial snapshot commitment;
5. persists that binding before run events; and
6. re-resolves current facts on exact retry.

Current approval reassessment is implemented by
`decide_approval_with_governance_reassessment(...)`. It validates the pending
approval and immutable run state before mutation, but its request contains
`LocalExecutionGovernanceAssessmentInputs`, including a caller-supplied
`runtime_facts` vector. That path can reproduce a durable assessment, but it
does not prove that the approval decision used the registered source committed
at run creation.

The existing `GovernanceRuntimeFactSnapshotBinding` already contains the
payload-free source-registration commitment, exact immutable bundle binding,
initial observation commitment, canonical fact-set commitment, freshness
inputs, bounded fact count, and assessment aggregate needed to close this gap.

## 5. Candidate API

Add the smallest additive request and result types, likely:

- `LocalCurrentRuntimeFactsGovernanceApprovalDecisionRequest`
  - existing `LocalApprovalDecisionRequest`;
  - governance profile;
  - explicit `GovernanceRuntimeFactSourceRegistration`;
  - Core-selected `evaluated_at`;
  - optional expected aggregate fingerprint.
- `LocalCurrentRuntimeFactsGovernanceApprovalDecisionResult`
  - resulting `WorkflowRun`;
  - unchanged durable `GovernanceAssessmentBinding`;
  - optional decision-time `GovernanceRuntimeFactSnapshot`.
- `decide_approval_with_current_runtime_facts_governance_reassessment(...)`
  - existing executor;
  - existing local immutable-bundle store;
  - injected `GovernanceRuntimeFactSource`;
  - explicit request.

Names should follow the implementation's final local conventions. The public
surface must remain explicit and opt-in. It must not discover a source,
registration, store, profile, or evaluation time from hidden global state.

The result carries a snapshot only for a granted decision that performed
source-backed reassessment. A denied decision returns no snapshot because no
source invocation is required.

## 6. Grant Ordering And Atomicity

For a granted decision, Core must perform this ordering:

1. call the existing approval preparation path so pending approval identity,
   decision validity, and terminal posture are validated without mutation;
2. reconstruct and freeze the exact resolved resume plan so workflow identity
   and approved execution-context commitments are validated before any source
   access;
3. load the exact immutable bundle binding from the durable run;
4. read the exact stored bundle and durable assessment binding;
5. require snapshot/store binding equality and V3 source-snapshot commitment;
6. compare the supplied registration commitment with the durable initial
   registration commitment before source invocation;
7. invoke the registered source exactly once for the stored bundle at the
   Core-selected decision time;
8. validate identity, contract version, bundle binding, freshness, and exact
   step coverage through the existing same-call assessment helper;
9. validate the optional expected aggregate fingerprint;
10. require the fresh assessment and source registration to reproduce the
   durable V3 assessment core through the existing reassessment validator; and
11. append and execute through the existing approval application semantics
    using the already-frozen resume plan.

Steps 1 through 10 must complete before `ApprovalGranted`, `RunResumed`, policy,
step-scheduling, or skill-invocation events are appended. Any failure leaves the
run waiting for approval with byte-for-byte equal event history and no skill
invocation.

The decision-time snapshot must not replace the initial durable snapshot
commitment. Persisting a second snapshot record or event is out of scope.

## 7. Denial Semantics

Fail closed must not make denial dependent on a healthy source. A denied
decision does not resume work and therefore does not need fresh execution facts.

The additive helper should prepare and apply a valid denial through the existing
approval boundary without invoking the source. It must still enforce current
resolved-context and approval identity rules already owned by the executor.

Tests must prove zero source calls for denial. Documentation must not claim that
a denied decision has a decision-time source snapshot.

## 8. Registration And Freshness Semantics

The caller selects one explicit registration and injects its matching source.
Before invoking the source, Core must compare
`registration.registration_commitment()` with the durable V3
`source_registration_commitment()`. Mismatch fails with a stable Core-owned
error and zero source calls.

The existing same-call source helper remains authoritative for:

- source ID and contract-version agreement;
- exact immutable-bundle agreement;
- stricter-of-source-and-Core maximum age;
- future-dated observation rejection;
- exact step coverage and canonical ordering; and
- assessment derivation.

The evaluation time is explicit input selected by Core's embedding caller in
this local phase. It is not read from hidden ambient time. Production time
authority remains separately scoped.

## 9. Assessment Compatibility

The grant path accepts only a durable V3 assessment binding with a valid
runtime-fact snapshot commitment. V1 and V2 bindings remain readable and valid
for their existing paths, but this source-backed approval consumer must reject
them as unsupported rather than silently downgrading to caller-supplied facts.

Fresh source facts may have a different snapshot ID, observation time, or
canonical fact-set commitment only when they preserve:

- the same registered source commitment;
- the exact immutable run bundle;
- the same assessment algorithm and aggregate fingerprint;
- the same bounded step count;
- the same execution disposition;
- the same disclosure requirement; and
- the same completeness posture.

Any stricter or otherwise changed assessment fails closed in this first
consumer. Runtime escalation routing is a later separately reviewed composition;
approval resume must not silently rewrite the approved scope or durable run
assessment.

## 10. Interaction With Existing Approval Proof

The first implementation should compose the base approval-decision path only.
It must preserve existing resolved-context integrity and must not weaken
presentation-proof requirements on callers that already use proof-enforced
approval APIs.

A follow-up may add a proof-enforced wrapper after this source consumer is
reviewed. That wrapper should reuse the same private grant reassessment helper
rather than duplicate source invocation or validation. The first phase must not
create a second approval state machine.

## 11. Error And Privacy Posture

- Reuse stable source errors where the existing source helper owns the failure.
- Add executor-owned errors only for missing V3 commitment, registration
  mismatch, durable binding mismatch, or reassessment mismatch.
- Do not include project paths, source IDs, contract versions, snapshot IDs,
  bundle identities, fact values, timestamps, fingerprints, approval reasons,
  provider output, command output, tokens, or credentials in errors.
- Replace source-local error messages before they cross Core.
- Redact approval request, source registration, evaluation time, bundle,
  binding, and snapshot identity from Debug output.
- Serialize no raw runtime facts through the new result.

## 12. Test Plan

Add focused tests proving:

1. a granted approval resolves the source exactly once and completes;
2. the decision-time source receives the exact stored immutable bundle;
3. source reassessment completes before approval or resume events;
4. the result returns the accepted payload-free decision-time snapshot;
5. the durable initial snapshot commitment remains unchanged;
6. a new snapshot with equivalent assessment is accepted;
7. changed assessment fails before new events or skill invocation;
8. changed source registration fails before source invocation;
9. stale, future-dated, wrong-source, wrong-version, wrong-bundle, missing,
   duplicate, or extra facts fail closed through stable errors;
10. missing, corrupt, V1, or V2 durable bindings fail before source invocation;
11. snapshot/store binding mismatch fails before source invocation;
12. source failure is replaced by a stable non-leaking error;
13. optional expected-fingerprint mismatch fails before approval mutation;
14. denied approval invokes no source and leaves no decision-time snapshot;
15. duplicate approval decisions preserve existing rejection behavior;
16. existing presentation-proof and resolved-context tests remain green;
17. errors and Debug output contain no secret-like source or approval values;
18. no raw fact vector appears in events, durable binding, or serialization;
19. existing source-backed fresh/retry execution remains unchanged; and
20. workspace tests pass.

## 13. Documentation Updates

Update:

- `ROADMAP.md`;
- this implementation plan;
- a focused implementation report;
- a focused maintainer review;
- proportional-governance concept documentation only if the implementation
  introduces a user-relevant boundary clarification.

Docs must state that the consumer is explicit, local, and opt-in; denial does
not require source health; the initial commitment remains provenance rather
than authority; default enforcement, schemas, CLI, providers, OpenShell,
SideEffects, writes, hosted expansion, and report citation remain unimplemented.

## 14. Validation

Run:

- focused runtime-fact source tests;
- focused local executor approval tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

## 15. Acceptance Criteria

- One explicit local approval-resume API consumes the registered source.
- Granted decisions resolve fresh facts exactly once.
- Registration mismatch fails before source invocation.
- Reassessment failure occurs before approval/resume mutation.
- The initial durable source commitment is preserved unchanged.
- Denied decisions remain available without source invocation.
- No raw facts, payloads, or secrets are persisted or leaked.
- Existing approval, retry, report, evidence, SideEffect, provider, OpenShell,
  hosted, and default executor behavior remains unchanged.

## 16. Recommended Follow-Up

After focused maintainer review, add a proof-enforced approval wrapper that
reuses the accepted private source-reassessment boundary. Do not broaden
proportional-governance defaults or provider mutation families first.
