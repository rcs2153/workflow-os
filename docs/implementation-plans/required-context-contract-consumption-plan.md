# Required Context Contract Consumption Plan

Status: Core model and pure consumption helper implemented and accepted after
the independent execution-context binding blocker was fixed. Consumption now
adds and retains an explicit actor, workflow, run, step, harness, and
evaluation-time context that every projection must match. Runtime context
dereference, executor integration, persistence, schema fields, CLI behavior,
provider integration, and sandbox integration remain unimplemented.

Related foundations:

- [Governed Context Access Projection Plan](governed-context-access-projection-plan.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Composable Harness Contract Plan](composable-harness-contract-plan.md)
- [Typed Handoff Plan](typed-handoff-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Governed Context Access Projection Review](../concepts/GOVERNED_CONTEXT_ACCESS_PROJECTION_REVIEW.md)

## 1. Executive Summary

Workflow OS can project authorized, payload-free context references for one
exact actor, workflow, run, step, and optional harness. It does not yet consume
a contract that says which references are required for that execution scope.

The next model boundary should compare an immutable typed required-context
declaration with one fresh governed context projection. It should determine
whether every required reference is present at the exact requested access
level, whether optional context is available, and whether the projection
contains undeclared context.

The core invariant is:

```text
A contract declares what context is needed. It does not grant authority.
A projection proves bounded reference visibility. It does not provide payloads.
Only an exact, fresh, least-privilege match may satisfy the contract.
```

The first implementation now provides that pure model and helper with no
runtime consumption or target dereference. It is documented in
[Required Context Contract Consumption Report](../concepts/REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_REPORT.md).
The execution-binding correction is documented in
[Required Context Contract Consumption Blocker Fix Report](../concepts/REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_BLOCKER_FIX_REPORT.md).
Focused re-review accepts the correction in
[Required Context Contract Consumption Blocker Fix Review](../concepts/REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_BLOCKER_FIX_REVIEW.md).
Immutable-run binding and time-of-use authority re-resolution are now planned
in
[Required Context Immutable-Run Binding And Time-Of-Use Plan](required-context-immutable-run-time-of-use-plan.md).
That plan starts with a separate immutable execution-binding model and keeps
runtime dereference deferred. The first binding model is now implemented in
[Required Context Immutable Execution Binding Report](../concepts/REQUIRED_CONTEXT_IMMUTABLE_EXECUTION_BINDING_REPORT.md).
It commits the validated stored bundle root and exact contract/execution scope
without granting current authority or permitting target dereference.

## 2. Goals

- Define a typed, domain-neutral required-context declaration.
- Bind the declaration to an immutable contract identity and content hash.
- Bind consumption to one exact actor, workflow, run, step, optional harness,
  evaluation time, and sensitivity ceiling.
- Require exact target and access-level matching.
- Distinguish required from optional context.
- Fail closed when required context is unavailable, unauthorized, unknown,
  excessive in sensitivity, or absent from the evaluated candidate set.
- Reject undeclared projected context rather than permit ambient access.
- Preserve explicit optional-context gaps without fabricating references.
- Retain deterministic ordering, validated serde, and redaction-safe Debug.
- Prepare for later immutable-run binding, time-of-use authorization, audited
  dereference, and optional sandbox execution.

## 3. Non-Goals

This planning phase does not authorize:

- target, evidence, event, report, handoff, SideEffect, source, or artifact
  payload dereference;
- repository or source inspection;
- prompts, transcripts, memory stores, vector databases, or RAG systems;
- tool loading, skill invocation, command execution, or local checks;
- connectors, providers, OpenShell, filesystem mounts, network access, process
  execution, or credential injection;
- SideEffect execution or writes;
- runtime consumption, executor integration, or nested harness execution;
- persistence, events, audit records, authority receipts, or report artifacts;
- workflow or harness schema changes;
- SDK, CLI, UI, examples, or hosted behavior;
- probabilistic inference as an enforcement source of truth;
- enterprise RBAC, IdP, DLP, or steward administration;
- reasoning-lineage claims or edges;
- release posture changes.

## 4. Current Foundation

The existing `HarnessContextRequirement` model stores only a validated name and
a required/optional flag. That vocabulary is useful for early contract shape,
but it is not sufficient to enforce target identity, access level,
sensitivity, immutable binding, or authority.

The governed context projection now provides:

- nine fixed typed stable-reference target kinds;
- reference-only and bounded-metadata access levels;
- exact capability and canonical resource mapping;
- actor, workflow, run, step, optional harness, evaluation-time, and
  sensitivity binding;
- complete supplied candidate retention;
- authorized entries and bounded gaps; and
- deterministic wire recomputation.

The consumer must bridge these foundations without treating a free-form context
name as a target, a declaration as a grant, or a projection as payload access.

## 5. Compatibility Boundary

The implementation does not silently reinterpret existing name-only
`HarnessContextRequirement` values as enforceable typed requirements. Doing so
would create false governance because names have no stable target or access
semantics.

The model-only phase should add a typed companion contract or typed consumption
definition in Core. A later separately reviewed compatibility/schema phase may
decide how authored harness contracts declare the typed fields.

Until then:

- existing harness contracts remain model vocabulary;
- no existing serialized shape changes;
- no name-to-target convention is inferred;
- no target ID is fabricated from a requirement name; and
- no workflow becomes runtime-enforced by this plan.

## 6. Candidate Core Model

The implementation adds:

- `RequiredContextRequirement`;
- `RequiredContextRequirementId`;
- `RequiredContextObligation` with `Required | Optional`;
- `RequiredContextContractBinding`;
- `RequiredContextConsumptionInput`;
- `RequiredContextConsumptionResult`;
- `RequiredContextConsumptionPosture` with `Satisfied | Blocked`;
- `RequiredContextSatisfaction`;
- `RequiredContextGap`.

Each requirement should carry:

- a bounded stable requirement ID;
- an exact `GovernedContextReferenceTarget`;
- an exact `GovernedContextAccessLevel`;
- required or optional obligation;
- a maximum sensitivity; and
- validated redaction metadata where needed.

The contract binding should carry:

- harness contract ID and version when the requirement belongs to a harness;
- an immutable contract content hash;
- deterministic requirement ordering; and
- no raw contract or target payload.

The consumption input should bind the contract and projection to the exact
execution context. The result should retain enough source information to
recompute its posture during validation and deserialization.

## 7. Exact Matching Rules

The first consumer should require:

- exact target equality;
- exact access-level equality;
- exact actor, workflow, run, step, optional harness, and evaluation-time
  equality;
- exact contract ID, version, and content-hash binding;
- sensitivity within both requirement and projection ceilings;
- one and only one projection candidate per declared target;
- one authorized projection entry for every satisfied requirement; and
- deterministic ordering by requirement ID and target.

`BoundedMetadata` must not satisfy a `ReferenceOnly` requirement implicitly.
Exposing more metadata than declared is not least privilege. A later explicit
access-lattice design may broaden this rule only through a separate review.

The projection candidate target set should equal the contract target set.
Extra projected candidates or entries are undeclared context and must fail
closed. Missing candidates must not be hidden by an empty projection.

## 8. Required And Optional Semantics

A required requirement is satisfied only when the matching projection entry is
authorized and available at the exact requested access level.

A required gap must block consumption. Approval cannot turn missing,
unauthorized, unavailable, unknown, or sensitivity-incompatible context into a
satisfied requirement.

An optional requirement may remain unsatisfied, but its bounded reason must be
retained and disclosed. Optional does not mean ambient context may be added.

The consumer must not:

- create a placeholder target;
- create a fake citation or evidence reference;
- infer authority from availability;
- infer availability from authority;
- infer authority from approval;
- downgrade required to optional; or
- omit a declared requirement from serialized results.

## 9. Pure Consumption Algorithm

The first helper should:

1. Validate contract identity, version, content hash, and requirements.
2. Validate the governed projection.
3. Require exact execution-context equality.
4. Sort and reject duplicate requirements.
5. Require exact equality between declared targets and projection candidates.
6. Match each requirement to one candidate and any derived entry or gap.
7. Require exact access-level and sensitivity compatibility.
8. Mark authorized available entries as satisfied.
9. Retain bounded optional gaps.
10. mark any required gap as blocked.
11. Reject extra, missing, substituted, or reordered requirements, candidates,
    entries, or gaps.
12. Return a deterministic payload-free result.

Validation and deserialization should recompute the exact result from retained
source requirements and the retained projection.

## 10. Immutable Binding

The typed declaration must be content-addressed. Contract ID and version alone
do not prove which requirements were reviewed.

The first model should reuse `SpecContentHash` or another existing canonical
content-hash primitive rather than introduce an unstructured digest. The
consumer should retain that binding and reject substitution.

Runtime use remains deferred. Before runtime consumption, the declaration and
projection must also be bound to the immutable run bundle or an equivalent
reviewed immutable source. Reloading a current mutable contract after approval
would recreate the approval/resume time-of-check/time-of-use risk already
identified elsewhere in the kernel.

## 11. Freshness And Time Of Use

A satisfied consumption result is not a lease and is not sufficient to
dereference a target.

Before any later payload access, the runtime must separately:

- re-resolve grant lifecycle and capability availability;
- re-evaluate policy, approval, evidence, and check prerequisites;
- verify the immutable run and step context;
- verify target identity and current availability;
- reapply sensitivity and redaction rules;
- prevent unprojected ambient workspace access; and
- emit an audited access record or reviewed authority receipt if required.

Freshness policy, authority receipts, and audited dereference belong to later
phases.

## 12. Relationship To Proportional Governance

Required-context enforcement and operator interruption are separate concerns.

- A missing required requirement blocks execution. Quiet or visible disclosure
  cannot downgrade that result.
- An optional gap may contribute bounded facts to proportional governance.
- The disclosure axis may choose quiet or visible presentation for a satisfied
  result or optional gap.
- A blocking approval may authorize a separately approvable action, but it
  cannot manufacture missing context authority or evidence.
- Runtime change signals must trigger reassessment; a cached accepted result
  must not survive a relevant contract, projection, authority, or sensitivity
  change.

This preserves quiet success for eligible work without weakening the context
contract.

## 13. Relationship To OpenShell And Other Execution Providers

An optional sandbox execution provider such as OpenShell is complementary to
this boundary. Workflow OS should decide what governed context is authorized;
the sandbox should enforce filesystem, process, network, inference, and
credential containment for execution.

A future sandbox integration should receive only materialized context derived
from a fresh satisfied contract, never ambient repository access by default.
It should return structured sandbox identity, effective policy revision,
result status, denial references, log references, artifact references, and
bounded attestation data.

Sandbox containment does not grant context authority, and sandbox availability
must not weaken a blocked contract. No OpenShell integration is implemented or
authorized by this plan.

## 14. Privacy And Redaction

The model must not store:

- target payloads or source contents;
- report, event, evidence, issue, comment, or handoff bodies;
- prompts, transcripts, chain-of-thought, or model memory;
- provider payloads, command output, CI logs, or parser payloads;
- paths, URLs, environment values, credentials, or token material;
- arbitrary metadata maps; or
- unbounded requirement names, reasons, or summaries.

Debug output must redact identities, targets, hashes, authority details,
sensitivity, and redaction metadata. Errors and deserialization failures must
use stable codes and must not echo rejected caller values.

## 15. Validation And Test Plan

Future focused tests should cover:

- valid all-required consumption;
- valid required plus unavailable optional context;
- every implemented target kind;
- exact access-level matching;
- required missing, unavailable, unknown, unauthorized, sensitivity-exceeded,
  and independent-prerequisite gaps;
- optional gap retention;
- extra undeclared candidate or entry rejection;
- omitted candidate or requirement rejection;
- duplicate requirement and target rejection;
- wrong actor, workflow, run, step, or harness;
- wrong contract ID, version, or content hash;
- stale evaluation context;
- projection substitution and reordering;
- deterministic ordering and serialization;
- serde round trip and invalid wire failure;
- Debug, error, and serialization non-leakage;
- no raw payload fields;
- declaration does not create authority;
- approval does not satisfy missing context; and
- existing harness, capability, context projection, EvidenceReference,
  WorkReport, handoff, policy, approval, SideEffect, and runtime regressions.

## 16. Proposed Implementation Sequence

1. Implement typed required-context requirement, binding, result, and gap model
   types.
2. Implement a pure exact-match consumption helper.
3. Add focused validation, serde, deterministic-order, and privacy tests.
4. Perform a phase-level maintainer review.
5. Implement the separately planned immutable execution-binding model.
6. Implement and review time-of-use re-resolution separately.
7. Only after review, consider one read-only dereference boundary.
8. Only after separate planning, consider optional sandbox provider execution.

Implementation should start with model types and a pure helper only.

## 17. Open Questions

- Should the first typed contract be harness-specific or a reusable
  workflow-step contract with optional harness identity?
- Should exact stable targets be authored directly, resolved from typed input
  bindings, or both through distinct reviewed models?
- Which immutable bundle should own the typed requirement content hash?
- Should a required gap produce a dedicated blocked posture or reuse a broader
  validation result?
- When should optional gaps become visible disclosure rather than quiet
  evidence?
- What freshness window is acceptable before future dereference?
- What minimum attestation must an optional sandbox provider return?

## 18. Final Recommendation

The next implementation phase should be:

**Required-context contract consumption core model and pure helper only.**

Do not implement payload dereference, repository reads, runtime consumption,
schemas, SDKs, CLI behavior, providers, OpenShell, persistence, events,
authority receipts, SideEffect execution, writes, hosted administration,
enterprise identity, reasoning lineage, or release changes.
