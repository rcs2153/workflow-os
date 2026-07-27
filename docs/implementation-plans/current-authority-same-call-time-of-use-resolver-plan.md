# Current Authority Same-Call Time-Of-Use Resolver Plan

Status: Implemented and accepted as a private test-only composition proof.

Related foundations:

- [Required Context Immutable-Run Binding And Time-Of-Use Plan](required-context-immutable-run-time-of-use-plan.md)
- [Required Context Current Authority Fact-Set Plan](required-context-current-authority-fact-set-plan.md)
- [Current Authority In-Memory Source Plan](current-authority-in-memory-source-plan.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Governed Context Access Projection Plan](governed-context-access-projection-plan.md)
- [Required Context Contract Consumption Plan](required-context-contract-consumption-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)

## 1. Executive Summary

Workflow OS now has:

- an immutable required-context execution binding;
- a public payload-free current-authority fact-set commitment;
- a private test-only Core-owned source that proves exact-query completeness;
- pure capability resolution;
- pure step-scoped context projection; and
- pure required-context consumption.

Those parts are not yet composed at one time-of-use boundary.

The first resolver should prove that composition without creating a public
authority shortcut. It should remain private and test-only, invoke the
Core-owned source and all accepted pure helpers in one call, and return a
bounded payload-free `Ready | Blocked` assessment.

The governing invariant is:

```text
Caller-owned fact commitments cannot confer readiness.
Core-owned complete current facts must be resolved, projected, and consumed
inside one non-reusable call.
```

`Ready` remains pre-dereference vocabulary. It does not read target data,
execute a tool, create a sandbox, authorize a provider, persist a lease, or
permit a write.

## 2. Why This Phase Is Next

The private in-memory source implementation and focused review are complete.
The review explicitly recommends a pure same-call resolver and warns that
source and resolver grant matching must not drift.

Fresh-pull product feedback also supports this sequencing. Workflow OS already
explains governance clearly; the next product challenge is reducing low-risk
ceremony without weakening evidence or authority. Proportional governance and
quiet success cannot safely become broader defaults while current authority is
still a caller assertion or a stale prior resolution.

This phase therefore strengthens a load-bearing runtime prerequisite rather
than adding another user-facing mode.

## 3. Goals

- Compose current fact acquisition and required-context evaluation in one
  private Core-owned call.
- Derive the exact query set from the exact required-context contract.
- Require exact immutable execution-binding and contract equality.
- Resolve capability availability, lifecycle, expiry, revocation, scope,
  specificity, sensitivity, and independent prerequisites at one explicit
  evaluation time.
- Rebuild governed context projections from current references and fresh
  capability resolutions.
- Rerun required-context consumption from those fresh projections.
- Return a payload-free bounded readiness assessment.
- Keep required gaps blocking and optional gaps explicit.
- Fail closed for incomplete, stale, substituted, ambiguous, or unresolved
  authority.
- Share one canonical grant-to-execution matching predicate between source
  selection and resolution expectations.
- Preserve stable non-leaking errors, Debug output, and deterministic hashes.

## 4. Non-Goals

This plan does not authorize:

- target, evidence, event, report, handoff, SideEffect, source, or artifact
  payload dereference;
- repository or source-content inspection;
- executor, runtime, retry, approval-resume, or report integration;
- a public production authority source;
- public `Ready` APIs from caller-constructed `CurrentAuthorityFactSet`;
- reusable authority leases, caching, TTLs, or persisted readiness;
- persistence, events, audit projection, reports, receipts, or artifacts;
- policy, approval, evidence, or check prerequisite evaluation;
- workflow, harness, immutable-bundle, schema, SDK, or compatibility changes;
- CLI, UI, examples, hosted behavior, or automatic defaults;
- provider, connector, OpenShell, filesystem, network, process, or inference
  execution;
- credential or environment-value access;
- SideEffect execution or writes;
- enterprise RBAC, IdP, DLP, or steward administration;
- reasoning lineage; or
- release posture changes.

## 5. Current Trust Boundary

`CurrentAuthorityFactSet::new` is public model vocabulary. Its completeness
posture is a validated claim, not trusted source proof. Any caller can construct
the model from a supplied record slice.

The accepted `InMemoryCurrentAuthoritySource` is different:

- it is private to `workflow-core`;
- it is compiled only for tests;
- it owns a complete bounded inventory;
- callers cannot supply query hashes, source snapshot hashes, or completeness;
- it derives the exact contract query internally; and
- it returns exact matching grants and one availability observation per query.

The first resolver must preserve that distinction. A public helper accepting
only `&CurrentAuthorityFactSet` would erase it and allow caller-claimed
completeness to become readiness.

## 6. First Implementation Boundary

The first implementation should add a private test-only same-call helper,
co-located with the private in-memory source.

Candidate private vocabulary:

- `CurrentAuthorityTimeOfUseInput<'a>`;
- `CurrentAuthorityTimeOfUseAssessment`;
- `CurrentAuthorityTimeOfUsePosture`;
- `CurrentAuthorityTimeOfUseReason`; and
- a private source-owned reference inventory or query result.

These names are not a public compatibility commitment.

The helper should accept:

- validated `RequiredContextExecutionBinding`;
- exact `RequiredContextContractBinding`;
- exact time-of-use timestamp;
- the private complete current-authority source;
- a private complete inventory of `GovernedContextReference` values for the
  exact contract targets; and
- required redaction metadata for fresh projections and consumption.

It must derive actor, workflow, run, step, harness, contract, and sensitivity
from the validated binding and contract rather than accept duplicative caller
strings.

## 7. Reference Completeness

The current private authority source owns grants and availability observations,
but not context references. Projection requires one validated
`GovernedContextReference` for each target.

The first resolver proof should therefore add a private complete reference
inventory at the same test-only boundary. The inventory should:

- own all candidate references before query;
- canonicalize by typed target;
- reject duplicate targets;
- commit the full inventory with a deterministic hash;
- select references only from the exact derived query set;
- require exactly one reference for every exact contract target;
- retain unavailable or unknown reference posture as current facts; and
- never retain or dereference target payloads.

A caller-supplied exact slice must not be accepted as complete. The private
inventory, like the private grant source, establishes the completeness boundary.

## 8. Exact Same-Call Algorithm

The helper should execute this sequence without accepting serialized
intermediates:

```text
binding + contract + private complete inventories
  -> validate exact binding and contract
  -> derive canonical exact query set
  -> query Core-owned grant/availability source
  -> query Core-owned reference inventory
  -> resolve each query at evaluated_at
  -> construct fresh projection candidates
  -> project each required access level
  -> consume exact required-context contract
  -> derive Ready or Blocked assessment
```

For each requirement:

1. derive capability and resource from typed target and access level;
2. locate the exact source-owned availability observation;
3. invoke `resolve_capability_authority`;
4. preserve `Authorized`, `RequiresIndependentEvaluation`, or
   `NotAuthorized` without reinterpretation;
5. construct a projection candidate only from the exact current reference and
   fresh resolution;
6. group candidates by requested access level;
7. call `project_step_scoped_context` for each declared access level; and
8. call `consume_required_context` with the exact independently derived
   execution context.

The helper must never accept a prior `CapabilityResolution`,
`GovernedContextProjection`, or `RequiredContextConsumptionResult`.

## 9. Binding And Substitution Checks

Before source query, the helper must require:

- binding contract hash equals the exact contract hash;
- binding harness identity and version equal the exact contract identity and
  version;
- binding workflow, run, step, actor, and harness identities are used for
  every capability resolution and projection;
- time of use is not earlier than binding time;
- source observation time does not follow time of use;
- every authority and reference query derives from the exact contract; and
- the returned fact set binds the exact execution-binding hash and query-set
  hash.

Changed contract, binding, actor, workflow, run, step, harness, sensitivity, or
time must fail closed before any `Ready` posture is possible.

## 10. Canonical Matching

The private source already filters grants by actor, workflow, optional run,
optional step, and optional harness scope. The capability resolver independently
evaluates the same dimensions plus lifecycle, expiry, revocation, sensitivity,
and specificity.

The implementation should extract one crate-private matching predicate for the
shared actor and execution-scope dimensions and use it in both paths. It must
not duplicate or weaken the resolver's grant-selection semantics.

Source filtering may retain all scope-matching candidates. The resolver remains
the source of truth for terminal authority posture and selected-grant
specificity.

## 11. Independent Prerequisites

Policy, approval, evidence, and check references on a matching grant remain
independent obligations.

The first resolver has no trusted source for proving those obligations.
Therefore:

- `CapabilityResolutionPosture::RequiresIndependentEvaluation` must never
  produce a projected authorized entry;
- it blocks the overall assessment when the affected requirement is required;
- it remains an explicit non-blocking gap when the affected requirement is
  optional, preserving the accepted contract obligation semantics;
- the resolver must not accept caller booleans claiming those prerequisites
  passed;
- it must not look up or infer decisions from IDs alone; and
- it must not downgrade an unresolved prerequisite to an optional context gap.

Later phases may add independently trusted prerequisite fact sources. They
require separate planning and review.

## 12. Readiness Semantics

Candidate posture:

- `Ready`: every required requirement is satisfied by a current available
  reference and fresh `Authorized` resolution; no independent prerequisite is
  unresolved.
- `Blocked`: at least one required requirement is unavailable, unknown,
  unauthorized, incomplete, ambiguous, stale, substituted, or requires
  independent evaluation.

Optional requirement gaps remain explicit in the assessment and may coexist
with `Ready`. They must not silently disappear.

`Ready` means only:

```text
The exact payload-free required-context contract is currently satisfied under
the exact immutable execution binding and current complete private facts.
```

It does not mean the referenced target exists beyond its current source
observation, that payload integrity was checked, or that dereference is allowed.

## 13. Stable Reasons

The assessment should retain bounded stable reasons including:

- `ready`;
- `binding_mismatch`;
- `contract_mismatch`;
- `time_invalid`;
- `source_incomplete`;
- `reference_missing`;
- `reference_duplicate`;
- `reference_unavailable`;
- `reference_unknown`;
- `capability_not_authorized`;
- `independent_policy_required`;
- `independent_approval_required`;
- `independent_evidence_required`;
- `independent_check_required`;
- `required_context_gap`; and
- `optional_context_gap`.

Errors describe malformed or inconsistent invocation. A valid negative
authority decision should return `Blocked`, not an exception.

## 14. Determinism And Commitment

The result should commit:

- resolver version;
- execution-binding hash;
- contract content hash;
- query-set hash;
- authority source inventory hash;
- reference source inventory hash;
- authority fact-set hash;
- evaluation timestamp;
- canonical per-requirement posture and reasons;
- required and optional gap counts; and
- overall posture.

Use fixed-width framed hashing and a versioned domain separator. Add one fixed
v1 known vector and ambiguous-framing regression.

The commitment remains in memory and test-only. No persistence or schema is
authorized.

## 15. Privacy And Redaction

The resolver may retain only typed stable references and payload-free authority
records already allowed by accepted models.

It must not retain:

- target payloads or source contents;
- provider responses;
- command output or logs;
- environment values or credentials;
- approval prose;
- policy inputs;
- evidence payloads;
- local-check output;
- sandbox data; or
- paths.

Debug output must redact identities, hashes, timestamps, resources, and
references. Stable errors must not include caller values or secret-like test
markers.

## 16. Error And Failure Posture

Malformed or inconsistent invocation returns stable
`current_authority.time_of_use.*` errors.

Valid current facts that do not authorize access return a validated `Blocked`
assessment with bounded reasons.

Source unavailable, incomplete, stale, or ambiguous conditions fail closed.
No partial projection or partial consumption result may escape on error.

## 17. Test Plan

Focused tests should prove:

1. complete current facts with one active exact grant produce `Ready`;
2. multiple access levels are projected and consumed in canonical order;
3. required gaps block;
4. optional gaps remain explicit without blocking otherwise ready work;
5. missing or duplicate reference inventory fails closed;
6. unavailable and unknown references block required context;
7. missing, disconnected, unsupported, or unknown capability availability
   blocks;
8. no matching grant blocks;
9. revoked grant blocks;
10. expired grant blocks at the explicit time of use;
11. sensitivity above the grant ceiling blocks;
12. narrower run, step, and harness scopes outrank broader candidates through
    the existing resolver;
13. unrelated actors and execution scopes cannot authorize;
14. policy prerequisite returns blocked independent-policy reason;
15. approval prerequisite returns blocked independent-approval reason;
16. evidence prerequisite returns blocked independent-evidence reason;
17. check prerequisite returns blocked independent-check reason;
18. changed contract fails closed;
19. changed execution binding fails closed;
20. earlier evaluation time fails closed;
21. caller-constructed public fact sets are not accepted by the private helper;
22. prior resolutions, projections, and consumption results are not inputs;
23. source and resolver share canonical scope matching;
24. output ordering and hashes are input-order independent;
25. fixed v1 hash vector remains stable;
26. framing prevents ambiguous commitments;
27. Debug and errors do not leak protected values;
28. no payload fields or dereference path exist;
29. existing capability, context, required-context, immutable-run,
    proportional-governance, approval, evidence, checks, SideEffect, provider,
    adapter, report, and runtime tests pass; and
30. documentation remains honest about private test-only readiness.

## 18. Compatibility Posture

The first resolver is private test infrastructure and has no public
compatibility guarantee.

No wire shape, schema, SDK, CLI output, persistence record, or external API is
introduced.

A production resolver requires a production source abstraction with
authenticated source identity, snapshot or high-watermark semantics,
freshness, concurrency, retry, operational failure behavior, and trusted
prerequisite facts.

## 19. Relationship To Proportional Governance

The resolver does not invoke proportional governance.

It prepares one trustworthy current-authority fact that a later runtime
assessment may consume. Quiet success must never be selected from stale or
caller-asserted authority. Unknown or incomplete authority remains monotonic:
it can raise friction or deny work, never reduce an explicit minimum.

## 20. Relationship To Execution Providers

An optional OpenShell adapter remains a sensible future execution boundary:
Workflow OS governs authority, obligations, evidence, and reports; OpenShell
may enforce filesystem, network, process, inference, and credential
containment.

This resolver does not integrate OpenShell or any provider. A future adapter
must consume a reviewed governed execution commitment and return bounded
enforcement evidence; it must not establish Workflow OS authority.

## 21. Proposed Implementation Sequence

1. Add a private test-only complete reference inventory.
2. Extract or reuse one canonical grant/execution-scope matcher.
3. Add the private same-call resolver input and result vocabulary.
4. Validate exact execution binding, contract, source, reference, and time
   equality.
5. Query the private sources in the same call.
6. Invoke existing capability resolution for each exact query.
7. Rebuild projections by access level.
8. Rerun required-context consumption.
9. Derive and commit bounded `Ready | Blocked` assessment.
10. Add focused determinism, substitution, prerequisite, privacy, and
    non-dereference tests.
11. Run full workspace validation.
12. Perform focused maintainer review.
13. Only after acceptance, plan one opt-in read-only runtime consumer.

## 22. Open Questions

- Should a future production source combine authority and reference inventory,
  or expose separately authenticated snapshots joined by one high watermark?
- Which accepted policy record can prove policy prerequisites at exact
  time of use?
- Must every approval prerequisite require persisted approval-presentation
  proof?
- What evidence acceptance observation is stronger than reference existence?
- Which local-check attestations qualify as independent current facts?
- Does the first production consumer need one-time-use or replay-prevention
  semantics?
- At what later boundary should source freshness become source-specific rather
  than same-call only?

## 23. Final Recommendation

The private test-only same-call resolver is implemented and accepted in the
[Current Authority Same-Call Time-Of-Use Resolver Review](../concepts/CURRENT_AUTHORITY_SAME_CALL_TIME_OF_USE_RESOLVER_REVIEW.md).
Plan the production current-authority source boundary next.

Do not expose public readiness, dereference targets, integrate the executor,
persist results, add schemas, invoke providers or OpenShell, execute
SideEffects, or enable writes.
