# Governed Obligation Plan Phase 4 Plan Review

## Verdict

Changes requested. The plan has the right authority boundary and correctly
rejects mutable-checkout rederivation, caller-authored proof, public contracts,
and a second evaluator. Three blocking architecture corrections are required
before runtime implementation can be authorized.

This review does not reject Phase 4. It requires the plan to use the complete
existing canonical input closure, reconcile existing governance-assessment
binding semantics, and state an achievable failure boundary around create-only
bundle persistence.

## Findings

### P0: The Proposed Sole Source Boundary Omits Required Context

The plan selects `ImmutableRunBundleBuildResult` as the sole canonical source
boundary and requires it to derive required-context obligations from the
immutable workflow/skill closure. Current Core does not support that claim.

`ImmutableRunBundleBuildResult` contains:

- the immutable manifest;
- canonical workflow, skill, and policy definition records; and
- canonical local-check declaration-set records.

`RequiredContextContractBinding` is a separate content-addressed Core contract.
It is supplied to current-authority, connector, local-check, and other protected
runtime paths; it is not stored in `WorkflowDefinition`, `SkillDefinition`, or
the immutable bundle's definition-record closure. The existing
`RequiredContextExecutionBinding` binds one such contract to an immutable run
bundle, workflow step, actor, sensitivity ceiling, and time, but the Phase 4
plan does not include or reconcile that binding.

The accepted parent plan explicitly requires required-context contract
commitments in the complete source closure and says that changing one must
invalidate a run-bound plan. Deriving from the bundle alone would either omit a
required contract or manufacture an absence claim that Core cannot prove.

Required correction:

1. Replace the “sole bundle source” claim with one explicit private canonical
   input-closure type assembled only from existing validated Core values.
2. Keep `StoredImmutableRunBundle` as the mandatory base and accept
   required-context inputs only through existing
   `RequiredContextContractBinding` plus `RequiredContextExecutionBinding`
   values that validate against that exact stored bundle and step.
3. Require one explicit per-step posture: an exact bound contract or a
   canonical existing explicit-none contract. Missing posture must be unknown
   or unsupported and fail closed; absence of an argument must never mean no
   required context.
4. Commit every accepted contract hash and execution-binding hash in the plan
   source closure.
5. Select a runtime consumer that already owns these values. Do not widen a
   generic executor API merely to inject them.
6. Add negative tests for omitted, substituted, duplicated, stale, wrong-step,
   wrong-actor, and wrong-bundle required-context bindings.

If no existing narrow runtime path owns the complete required-context closure,
the first Phase 4 implementation must stop at source-extractor/binding work.
Executor composition remains unauthorized until a later focused plan names a
real consumer.

### P0: Existing Governance Assessment Binding Is Not Reconciled

Core already has a durable, create-only `GovernanceAssessmentBinding` tied to
the exact immutable run bundle. Existing opt-in execution paths persist it,
project it into run state and events, and require current reassessment to
reproduce it on retry. Provider-write admission and authority receipts also
consume its commitment.

The Phase 4 plan proposes a new private plan binding and says the composition
will preserve proportional-governance assessment, but it does not define the
relationship between the two bindings or name the existing executor path it
will compose with. Even a non-persistent parallel binding can become a second
governance fingerprint if its semantics overlap or if a new wrapper bypasses
the established reassessment path.

Required correction:

1. Name the new value a projection binding, not a governance assessment
   binding, and state that it proves only static inventory identity.
2. It must embed or reference the existing `ImmutableRunBundleBinding`; it
   must not restate execution disposition, disclosure, completeness, current
   facts, authority, approval, evidence, or satisfaction.
3. A runtime slice that uses proportional governance must compose with the
   existing `execute_with_governance_assessment_binding` or
   `execute_with_current_runtime_facts_governance_assessment_binding` path, or
   factor a reviewed common internal stage from one of those paths. It must not
   create a parallel assessment lifecycle.
4. Continuation must independently reproduce both the existing durable
   governance assessment and the private plan projection. Agreement between
   them is not authority; disagreement fails closed.
5. Add tests proving the plan binding cannot substitute for, weaken, or satisfy
   a missing, stale, incomplete, denied, or changed governance assessment.

### P1: The Post-Persistence Failure Promise Is Not Achievable

The plan correctly derives and validates in memory before persistence, then
requires a read-back and deterministic rederivation from the create-only
immutable store. It also says any extraction or binding failure creates no
artifact.

Those statements conflict. Once the bundle is written, a storage corruption,
read-back mismatch, or post-write rederivation failure can leave a durable
immutable bundle even though no run event has been appended. Current local
bundle storage is create-only and is not transactionally coupled to the state
backend's event append. Silently deleting or rolling back that evidence would
also violate current storage posture.

Required correction:

1. Before persistence, all deterministic extraction, bounds, and plan-binding
   validation must complete in memory. Failure here creates no bundle or run
   state.
2. After persistence, read-back or rederivation failure must create no run
   events, approval, invocation, WorkReport, `SideEffect`, or external action,
   but the immutable bundle may remain as an unactivated/orphaned record.
3. Exact retry must either validate and reuse the identical bundle or fail
   closed. It must not overwrite, delete, or silently replace it.
4. The report and test contract must disclose this storage boundary rather
   than claiming atomicity Core does not provide.
5. Any future automatic orphan cleanup requires separate retention and audit
   design and is outside Phase 4.

## Required Plan Clarifications

The blocker correction must also make these narrower changes:

- Reuse one crate-private immutable-bundle cardinality constant or a validated
  manifest-bound accessor. Do not copy the literal `256` into a second module
  where the limits can drift.
- Require an exact bijection between manifest references and supplied canonical
  records by reusing immutable-store resolution and unreferenced-record
  validation. Do not implement another record matcher.
- Distinguish workflow-authored `ReportArtifactRequirements`, which are present
  in the canonical workflow record, from runtime `WorkReportContract` and
  artifact-generation inputs. The source extractor may inventory only the
  former unless an existing exact bound runtime contract is included in the
  canonical input closure.
- Name the exact first consumer or explicitly keep executor composition blocked
  after the extractor/binding slice. “A wrapper around an existing entry point”
  is not specific enough to prove it will preserve established retry,
  governance-assessment, required-context, and report semantics.

## Accepted Boundaries

The following parts of the plan are accepted and should be preserved:

- private, opt-in, non-serializing scope;
- no public schema, SDK, CLI, provider, dependency, Salsa, release, or
  publication behavior;
- no mutable-checkout rederivation after run start;
- no caller-authored obligation, decision, evidence, or satisfaction proof;
- no policy, approval, evidence, `SideEffect`, report, event, or completion
  authority in the plan;
- deterministic ordering, bounded cardinality, redaction-safe failures, and no
  truncation;
- failure before runtime events on any mismatch; and
- separately reviewed extraction/binding and executor-composition increments.

## Validation

- Reviewed merged planning commit
  `162977e8de5ff51ca3bdf637cabe4609aaeb7c08`.
- Inspected immutable bundle manifest, builder, canonical definition records,
  create-only store, required-context contract and execution binding,
  proportional-governance assessment binding, and existing opt-in executor
  paths.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## Governed Review

- workflow: `dg/review`
- run: `run-1793967429264863000-2`
- approval: `approval/run-1793967429264863000-2/review-scope-approved`
- presentation: `presentation/8fe532b96498d60a`
- approval outcome: granted with persisted presentation proof
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations
- presentation enforcement: `proof_enforced`

Workflow OS governed the review scope and approval. Source inspection,
documentation edits, validation, Git, PR, and merge work occur outside the
kernel and remain disclosed. No implementation, runtime execution, provider
action, release, publication, or independent third-party assurance is
simulated.

## Next Authorized Phase

One separately governed documentation-only blocker correction to the Phase 4
plan is authorized. It must define the complete canonical input closure,
reconcile the existing durable governance-assessment binding, correct the
post-persistence failure contract, share cardinality and record-resolution
invariants, and name an exact consumer or keep executor composition blocked.

Runtime implementation remains unauthorized. A second focused review is
required after the correction.

