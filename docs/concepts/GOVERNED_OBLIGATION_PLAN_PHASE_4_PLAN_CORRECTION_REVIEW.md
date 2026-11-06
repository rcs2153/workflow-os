# Governed Obligation Plan Phase 4 Corrected Plan Review

## Verdict

Accepted for the first private implementation slice only. The corrected plan
resolves every P0 and P1 finding from the first focused review without
introducing a second authority, persistence model, or runtime consumer.

This acceptance authorizes only the crate-private canonical-input closure,
source extractor, and static projection binding. Executor composition remains
blocked until a later focused plan names one exact consumer that owns the full
required-context closure and existing durable governance-reassessment
lifecycle.

## Accepted Corrections

### Complete Canonical Input Closure

The plan no longer treats the immutable run bundle as the complete source
closure. One module-owned input now requires:

- one store-validated `StoredImmutableRunBundle`;
- exactly one `RequiredContextContractBinding` for every workflow step,
  including canonical explicit-none posture; and
- one matching `RequiredContextExecutionBinding` validated against the exact
  bundle, step, actor, sensitivity posture, and contract.

Missing input cannot mean no required context. Omitted, substituted, duplicate,
stale, wrong-step, wrong-actor, and wrong-bundle values fail closed. The plan
also prohibits mutable-checkout reload, independently assembled project data,
and caller-authored obligations or decisions.

### Subordinate Projection Binding

`GovernedObligationPlanProjectionBinding` is now limited to static inventory
identity. It may bind the exact bundle, run, required-context closure,
algorithm, source closure, inventory, and count, but cannot carry execution
disposition, disclosure, completeness, current facts, authority, approval,
evidence, or satisfaction.

The existing durable `GovernanceAssessmentBinding` remains the sole accepted
proportional-governance assessment binding. A future runtime path must
reproduce both values independently and fail closed on disagreement. The
projection cannot replace, satisfy, or weaken governance reassessment.

### Truthful Persistence Boundary

The plan now distinguishes two failure classes:

- deterministic extraction and binding failure before persistence creates no
  immutable bundle or runtime state; and
- read-back or rederivation failure after create-only persistence may leave an
  exact unactivated immutable bundle, but creates no event, approval,
  invocation, WorkReport, `SideEffect`, or external action.

Exact retry may validate and reuse an identical bundle or fail closed. It may
not overwrite, delete, replace, or silently clean up the durable record.

### Shared Invariants And Consumer Gate

The plan requires reuse of one crate-private immutable-bundle cardinality
bound and the existing immutable-store record-resolution and bijection checks.
It no longer permits a second record matcher or copied numeric limit.

Workflow-authored `ReportArtifactRequirements` are correctly distinguished
from runtime `WorkReportContract` values and report artifacts. The first
extractor may inventory only the authored workflow requirement.

No executor wrapper is authorized. A later plan must name a path based on
`execute_with_governance_assessment_binding`,
`execute_with_current_runtime_facts_governance_assessment_binding`, or a
focused-review-accepted common internal stage. If no path already owns the
complete required-context and governance-reassessment closure, Phase 4 must
stop rather than inject parallel inputs.

## Implementation Conditions

The accepted first implementation must preserve two explicit conditions:

1. The current private source kind named `WorkReportContract` must not be used
   to label workflow-authored `ReportArtifactRequirements`. The private
   vocabulary must be renamed or otherwise corrected so source identity states
   what the canonical workflow record actually contains.
2. `RequiredContextExecutionBinding` is currently supplied to protected paths,
   not part of `StoredImmutableRunBundle`. The first slice may validate and
   commit exact supplied bindings in memory, but must not claim durable
   continuation ownership. Any later continuation composition remains blocked
   until the consumer amendment proves where the exact bindings are durably
   obtained and how they are reproduced without caller reconstruction.

These conditions narrow the accepted implementation. They do not authorize a
new store, public contract, or executor API.

## Validation

- Reviewed the merged correction at `ad7b5433cba0bab79cc41b38f271faafe0ea66e2`.
- Rechecked the original P0/P1 findings against the corrected plan.
- Inspected `StoredImmutableRunBundle`, immutable-store record validation,
  `RequiredContextContractBinding`, `RequiredContextExecutionBinding`,
  `GovernanceAssessmentBinding`, the two existing governance-assessment
  executor entry points, `ReportArtifactRequirements`, `WorkReportContract`,
  and the current private obligation vocabulary.
- `npm run check:docs`: required before commit.
- `git diff --check`: required before commit.
- Runtime tests were not run because this review changes documentation only
  and authorizes no runtime behavior.

## Governed Review

- workflow: `dg/review`
- run: `run-1793969436793892000-2`
- approval: `approval/run-1793969436793892000-2/review-scope-approved`
- presentation: `presentation/bb93b49c11042327`
- approval outcome: granted with persisted presentation proof
- phase status: `Completed`
- presentation enforcement: `proof_enforced`

Workflow OS governed the review scope and approval. Source inspection,
documentation edits, validation, Git, PR, and merge work occur outside the
kernel and remain disclosed. No runtime implementation, provider action,
release, publication, or independent third-party assurance is simulated.

## Next Authorized Phase

One separately governed private implementation of the canonical-input closure,
source extractor, corrected private source vocabulary, and projection binding
is authorized. It must include focused completeness, cardinality, commitment,
redaction, and negative-binding tests.

Executor composition, persistence changes, public schema/SDK/CLI, providers,
dependencies, Salsa, release behavior, and publication remain unauthorized.
Focused maintainer/security review of the exact implementation and tests is
required before any consumer amendment.
