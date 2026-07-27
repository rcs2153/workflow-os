# Required Context Immutable-Run Binding And Time-Of-Use Plan

Status: Planning complete. The first immutable required-context execution
binding model is implemented and documented in
[Required Context Immutable Execution Binding Report](../concepts/REQUIRED_CONTEXT_IMMUTABLE_EXECUTION_BINDING_REPORT.md).
Time-of-use authority re-resolution, runtime consumption, target dereference,
persistence, events, schemas, CLI behavior, provider integration, sandbox
integration, and writes remain unimplemented.

Related foundations:

- [Required Context Contract Consumption Plan](required-context-contract-consumption-plan.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Governed Context Access Projection Plan](governed-context-access-projection-plan.md)
- [Immutable Run Bundle Boundary Plan](immutable-run-bundle-boundary-plan.md)
- [Capability Grant And Availability Plan](capability-grant-availability-plan.md)
- [Composable Harness Contract Plan](composable-harness-contract-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)

## 1. Executive Summary

Workflow OS can now declare exact typed context requirements, project
authorized payload-free references for one execution context, and determine
whether required context is satisfied. That result is a point-in-time
assessment. It is not a lease, capability grant, immutable run input, or
permission to dereference a target.

Before any runtime consumer may use required context, Workflow OS must prove
two independent properties:

1. the exact run, step, harness contract, and requirement set were bound before
   consumption to the accepted immutable run context; and
2. capability availability and authority were re-resolved from current typed
   facts at the exact time of use.

The governing invariant is:

```text
Bind what was accepted. Re-resolve what can change. Dereference only after
both checks pass at the same governed boundary.
```

The smallest safe implementation sequence begins with a payload-free
pre-consumption binding model only. A later pure helper may recompute current
capability resolutions, rebuild governed projections, and rerun required
context consumption. Runtime dereference remains a separate reviewed phase.

The first phase now implements that pre-consumption binding. Its constructor
accepts a validated stored immutable bundle, exact content-addressed contract,
actor, step, sensitivity ceiling, and timestamp. It derives workflow/run
identity, verifies the step against the canonical frozen workflow record, and
commits every field with fixed-width framed SHA-256 hashing. The binding
remains explicitly non-authoritative.

## 2. Goals

- Bind required-context consumption to one exact immutable run bundle.
- Bind the exact required-context contract identity, version, and content hash.
- Bind actor, workflow, run, step, harness, sensitivity, and binding time.
- Keep harness-contract requirements separate from current immutable-bundle
  definition taxonomy until that taxonomy is explicitly broadened.
- Re-resolve availability and grant authority at a caller-supplied time of use.
- Rebuild projections and consumption results from current typed sources rather
  than trusting a prior serialized result.
- Require exact stored-bundle, contract, execution-context, and projection
  equality.
- Preserve required-gap blocking and optional-gap disclosure.
- Produce stable non-leaking failures for missing, stale, changed, revoked,
  unavailable, substituted, or ambiguous inputs.
- Prepare one later read-only dereference boundary without implementing it.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- target, evidence, event, report, handoff, SideEffect, source, or artifact
  payload dereference;
- repository or source-content inspection;
- executor or runtime integration;
- persistence, events, audit projection, authority receipts, or report
  artifacts;
- workflow, harness, immutable-bundle, schema, or SDK shape changes;
- CLI, UI, examples, or hosted behavior;
- provider, connector, OpenShell, filesystem, network, process, or inference
  execution;
- environment-value or credential access;
- SideEffect execution or writes;
- implicit approval, inferred authority, or model judgment as enforcement;
- enterprise RBAC, IdP, DLP, or steward administration;
- reasoning lineage;
- release posture changes.

## 4. Current Boundary

The accepted required-context consumer retains:

- a content-addressed `RequiredContextContractBinding`;
- an independently declared actor, workflow, run, step, harness, and
  evaluation time;
- complete governed context projections;
- satisfied requirements;
- required and optional gaps; and
- a deterministic `Satisfied | Blocked` posture.

The accepted immutable run bundle retains:

- bundle identity, version, and root hash;
- workflow and run identity;
- canonical workflow, skill, and policy definition references;
- resolved execution-context commitment;
- selected local-check declaration-set references;
- handler and execution posture; and
- create-only run binding in the local bundle store.

The accepted capability resolver evaluates:

- exact actor, capability, resource, workflow, run, step, and optional harness;
- current capability availability records;
- current validated grants, lifecycle, expiry, revocation, prerequisites, and
  sensitivity; and
- one explicit evaluation timestamp.

No current model proves that a required-context contract was accepted as part
of an immutable run. No current helper reruns capability resolution at target
dereference time.

## 5. Source-Of-Truth Boundaries

| Concern | Source of truth | Must not be treated as |
| --- | --- | --- |
| Run identity and validated definitions | `StoredImmutableRunBundle` | Current mutable project files |
| Required context declaration | Exact `RequiredContextContractBinding` | A capability grant or payload |
| Pre-consumption commitment | Future required-context execution binding | Runtime freshness forever |
| Capability availability | Current bounded availability records | Authority or successful access |
| Capability authority | Fresh `CapabilityResolution` from current grants and availability | Approval, visibility, or prior resolution |
| Governed visibility | Fresh `GovernedContextProjection` | Target payload access |
| Contract satisfaction | Fresh `RequiredContextConsumptionResult` | Dereference lease |
| Payload access | Future separately governed consumer | Implied by any model above |

Mutable repository files, natural-language summaries, agent judgment, cached
projection results, and report citations are not authority sources.

## 6. Pre-Consumption Immutable Binding

The first implementation should add the smallest payload-free binding model,
likely:

- `RequiredContextExecutionBinding`;
- `RequiredContextExecutionBindingVersion`; and
- a Core-owned pure constructor from validated sources.

The binding should retain:

- `ImmutableRunBundleBinding`;
- workflow and run identity from the stored manifest;
- exact step identity;
- exact actor identity;
- exact harness contract identity and version;
- exact required-context contract content hash;
- requested sensitivity ceiling;
- binding algorithm/version;
- binding creation time; and
- a deterministic content hash over every field above.

The constructor should accept a validated `StoredImmutableRunBundle`, exact
required-context contract, actor, step, sensitivity, and timestamp. It should:

1. validate the stored bundle and manifest/record integrity;
2. derive bundle identity from the stored manifest;
3. require exact workflow and run equality;
4. prove the step exists in the immutable workflow definition;
5. require contract and execution harness equality;
6. compute the binding with fixed-width framing and a versioned domain
   separator; and
7. return only the validated payload-free binding.

The first model should not add harness contracts to
`ImmutableRunBundleDefinitionKind`. Current bundles freeze workflow, skill,
policy, and selected local-check declarations, but not a canonical harness
contract record. The separate binding commits the exact contract hash without
claiming the contract was already a bundle definition. Broadening bundle
taxonomy requires separate planning and compatibility review.

## 7. Binding Authority Posture

The execution binding proves only:

- which immutable run bundle was used;
- which exact requirement contract was selected;
- which actor, step, harness, sensitivity, and time were committed; and
- that later consumers can detect substitution.

It does not prove:

- capability availability at a later time;
- grant validity at a later time;
- policy, approval, evidence, or check prerequisites;
- target existence or payload integrity;
- successful dereference;
- sandbox containment; or
- provider execution.

The binding must not expose a method named `authorize`, `permit`, or
`dereference`. Its Debug and errors must redact identities, hashes, paths, and
caller-supplied values.

## 8. Time-Of-Use Re-Resolution

The second implementation phase should add a pure, non-dereferencing helper
that accepts:

- validated `StoredImmutableRunBundle`;
- validated `RequiredContextExecutionBinding`;
- exact `RequiredContextContractBinding`;
- exact current actor/workflow/run/step/harness context;
- one explicit time-of-use timestamp;
- current complete candidate context references;
- current complete capability availability records; and
- current complete candidate grants.

For every requirement, the helper should:

1. derive the exact capability and resource from the typed target and access
   level;
2. call the existing capability resolver at the time-of-use timestamp;
3. reject incomplete or duplicate availability/grant candidate sets according
   to an explicit completeness contract;
4. reconstruct governed projection candidates from current references and
   fresh resolutions;
5. rebuild exact step-scoped projections;
6. rerun `consume_required_context`;
7. require exact execution-binding, bundle, contract, and context equality; and
8. return a payload-free time-of-use decision.

The helper must never accept a prior projection or prior consumption result as
the current authority source.

## 9. Candidate Time-Of-Use Result

Candidate vocabulary:

- `RequiredContextTimeOfUseInput`;
- `RequiredContextTimeOfUseResult`;
- `RequiredContextTimeOfUsePosture` with `Ready | Blocked`;
- `RequiredContextTimeOfUseReason`; and
- optional bounded per-requirement resolution references.

`Ready` means the exact requirements are currently satisfied under the exact
immutable binding and current authority facts. It still does not mean payload
dereference occurred.

`Blocked` should cover stable reasons including:

- immutable bundle missing, changed, or mismatched;
- contract changed or mismatched;
- actor, workflow, run, step, or harness mismatch;
- availability missing, unknown, unsupported, or changed;
- grant missing, expired, revoked, sensitivity-incompatible, or changed;
- independent policy, approval, evidence, or check evaluation still required;
- current reference missing or duplicated;
- required context unavailable; and
- unsupported access posture.

Optional requirement gaps remain explicit and non-blocking unless another
governance source raises the posture.

## 10. Freshness Policy

Freshness must be evaluated at the consuming boundary, not inferred from
successful construction.

The first time-of-use helper should prefer same-call re-resolution:

```text
current facts -> fresh capability resolutions -> fresh projections ->
fresh required-context consumption -> Ready or Blocked
```

It should not introduce a reusable time-to-live lease. If a later consumer
cannot perform re-resolution and dereference in one bounded call, it needs a
separately reviewed freshness and one-time-use model.

Availability observation time must not be future-dated. Grant expiry and
revocation must be evaluated at the supplied time of use. Any maximum-age
policy for availability records must be explicit, bounded, deterministic, and
owned by Core; absence of a required freshness policy must fail closed.

## 11. Completeness And Ambient Authority

The helper must define what makes supplied grants, availability records, and
candidate references complete for the requested target set. A caller-selected
subset cannot prove that no conflicting, revoked, or more specific record
exists.

The first implementation should either:

- accept a validated complete-set record from a Core-owned resolver; or
- remain explicitly `assessed_not_authoritative` until such a record exists.

It must not claim global store completeness from an arbitrary slice. This is
the same boundary already preserved by governed context projection.

No extra target, candidate, projection, grant-derived authority, or context
reference may enter the result. Ambient context remains a fail-closed error.

## 12. Policy, Approval, Evidence, And Check Prerequisites

The capability resolver already distinguishes a matched grant from unresolved
independent prerequisites. Time-of-use required-context readiness must preserve
that distinction.

The first helper should not accept booleans such as `policy_passed` or
`approved`. Later integration should consume exact validated references or
accepted records from the owning policy, approval, evidence, and check
boundaries.

Until those records are composed:

- any required independent prerequisite blocks `Ready`;
- approval cannot manufacture missing context;
- a WorkReport citation cannot prove authority;
- an evidence reference cannot prove its target is readable; and
- quiet-success posture cannot weaken required authority.

## 13. Runtime Integration Boundary

Only after the binding and time-of-use helper are implemented and reviewed
should one explicit opt-in read-only runtime consumer be planned.

That future path should:

1. load the exact stored immutable run bundle;
2. load or derive the exact pre-consumption binding;
3. obtain current complete authority and availability facts;
4. perform same-call time-of-use re-resolution;
5. stop when posture is blocked;
6. request only the exact declared target through a separately governed
   read-only handler;
7. record bounded outcome and evidence references without copying payloads; and
8. leave workflow semantics unchanged if report-only projection fails.

No default runtime context injection, ambient workspace mounting, provider
write, or sandbox execution is authorized by this plan.

## 14. Optional Sandbox Provider Relationship

An optional sandbox such as OpenShell may later enforce filesystem, network,
process, inference, and credential boundaries for an execution handler. It
does not replace Workflow OS authority, policy, approval, evidence, or
required-context decisions.

If a future sandbox consumer is approved, Workflow OS should pass:

- exact immutable bundle and required-context binding references;
- effective sandbox policy identity and hash;
- only the already-authorized context references needed by the step; and
- no ambient credentials or undeclared mounts.

The sandbox should return stable sandbox, policy, outcome, denial, artifact,
and log references. Workflow OS should cite those records rather than copy raw
logs. No OpenShell integration or fork is implemented by this plan.

## 15. Privacy And Error Handling

Models and errors must not store or expose:

- raw target payloads;
- source or repository contents;
- command output or parser payloads;
- provider responses or logs;
- environment values;
- credentials, authorization headers, private keys, or tokens;
- unrestricted paths or mount lists;
- raw policy, approval, evidence, or check payloads; or
- caller-supplied values in validation errors.

Debug output should expose bounded posture, counts, and enum reasons while
redacting identities and hashes. Deserialization errors must use stable
non-leaking messages.

## 16. Proposed Implementation Sequence

1. Implement `RequiredContextExecutionBinding` model and pure constructor from
   a validated stored immutable bundle and exact contract.
2. Add focused binding integrity, canonical hash, serde, and privacy tests.
3. Perform a phase-level maintainer review.
4. Define the Core-owned complete current authority-fact set in the
   [Required Context Current Authority Fact-Set Plan](required-context-current-authority-fact-set-plan.md).
5. Implement and review the private Core-owned test source defined in the
   [Current Authority In-Memory Source Plan](current-authority-in-memory-source-plan.md).
6. Implement a pure same-call time-of-use re-resolution helper.
7. Add expiry, revocation, availability, prerequisite, substitution,
   completeness, and non-leakage tests.
8. Perform a focused review.
9. Plan one opt-in read-only runtime consumer.
10. Plan optional sandbox execution separately.

The immutable execution-binding and current-authority fact-set models and
focused reviews are complete. The private in-memory test source now proves
source-owned exact-query completeness over its complete fixture inventory.
Focused source review accepts the boundary in the
[Current Authority In-Memory Source Review](../concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_REVIEW.md).
Pure same-call time-of-use resolver planning is next.

## 17. Test Plan

Future tests should prove:

- exact stored bundle and contract produce a valid binding;
- workflow, run, step, actor, harness, sensitivity, or contract substitution
  fails closed;
- changed bundle root or contract hash invalidates the binding;
- serialized binding tampering fails closed;
- canonical hashes are deterministic and collision-resistant;
- Debug and errors do not leak identities, hashes, paths, or secret-like data;
- fresh active grant and availability can satisfy a requirement;
- expired or revoked grant blocks;
- missing, unknown, or unsupported availability blocks;
- unresolved policy, approval, evidence, or check prerequisite blocks;
- stale or future-dated availability blocks under the selected policy;
- required gaps block and optional gaps remain explicit;
- extra or missing targets fail closed;
- prior projections and prior consumption results are not accepted as current
  authority inputs;
- no payload is read, copied, or dereferenced;
- existing immutable-run, capability, context projection, required-context,
  approval, report, SideEffect, provider, and runtime tests still pass; and
- docs remain honest about model-only and read-only boundaries.

## 18. Open Questions

- Should the first binding constructor be crate-private to preserve Core-owned
  provenance?
- What complete-set model should prove current grant and availability candidate
  completeness?
- Should harness contracts eventually become canonical immutable-bundle
  definition records?
- Is same-call re-resolution sufficient for the first read-only consumer, or
  is a one-time claim required?
- Which current availability sources can provide trustworthy observation time?
- How should accepted policy, approval, evidence, and check records compose
  without boolean shortcuts?
- What is the first useful read-only target kind to dereference?
- What minimum sandbox attestation is required before an optional execution
  provider can consume projected context?

## 19. Final Recommendation

The immutable execution-binding core model is implemented and accepted. The
next implementation prompt should be:

**Required-context current authority fact-set core model only.**

Do not implement authoritative time-of-use readiness, dereference, executor
consumption, persistence, events, schemas, CLI behavior, providers, OpenShell
integration, process execution, SideEffect execution, writes, hosted
administration, reasoning lineage, or release changes.
