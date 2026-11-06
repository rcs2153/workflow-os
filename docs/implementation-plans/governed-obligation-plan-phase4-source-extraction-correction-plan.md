# Governed Obligation Plan Phase 4 Source-Extraction Correction Plan

## Status

Planned. Implementation is not yet authorized.

This correction narrows the merged Phase 4 canonical-input closure to an
immutable **commitment closure**. It may prove exactly which required-context
contract and execution-binding commitments were supplied for each immutable
workflow step. It may not prove that the actor, sensitivity ceiling, or binding
time was selected by the future runtime consumer.

Executor composition remains blocked until a separately reviewed runtime path
owns those expectations and compares them at the protected time of use.

## Why The Claim Must Narrow

`RequiredContextExecutionBinding::new` commits a caller-selected actor,
maximum sensitivity, and binding time to an exact immutable bundle, workflow,
run, step, and required-context contract. Its constructor rejects an unknown
sensitivity, a missing step, and a time before bundle creation. Its content
hash proves internal integrity.

It does not prove that the selected actor, sensitivity, or time is the value a
later consumer expected. `GovernedObligationCanonicalInputClosure::new`
currently checks the binding's internal integrity and exact immutable
identities, but it has no independent consumer-owned comparison source.

Inspection found narrow runtime paths that construct bindings inside their own
admission flow, including trusted-host local application and GitHub repository
read paths. Those paths do not own the complete generic multi-step Phase 4
source closure plus the existing durable governance-assessment lifecycle.
Selecting one as a generic consumer would overstate its authority and silently
narrow the supported workflow shape.

The correction therefore takes the review's second accepted option: preserve
the extractor as a non-authorizing commitment model and keep runtime
composition blocked. It does not invent a parallel expected-actor structure,
infer authority from unrelated bundle fields, or treat a binding hash as
semantic authorization.

## Corrected Model Boundary

The implementation should rename the private closure to
`GovernedObligationCanonicalCommitmentClosure`, or otherwise make the same
commitment-only posture unambiguous in its type and documentation.

Construction may prove only:

- the stored immutable bundle passes existing manifest-to-record bijection
  and canonical-record validation;
- the closure contains exactly one required-context contract and execution
  binding for every immutable workflow step;
- each execution binding validates internally;
- bundle, workflow, run, step, contract identity, contract version, and
  contract content hash match exactly;
- every binding time is at or after immutable bundle creation;
- no step is missing or duplicated;
- deterministic sorting is independent of input order; and
- all shared cardinality and allocation bounds are satisfied before complete
  plan construction.

The commitment closure must not claim:

- that the actor is the actor expected by an executor;
- that the sensitivity ceiling is the executor's authorized ceiling;
- that a later binding time is current or fresh;
- that current authority, approval, evidence, checks, or required context are
  satisfied;
- that the plan authorizes execution or a `SideEffect`; or
- that a changed but internally valid binding is invalid merely because it
  differs from another possible commitment.

An alternate actor, known sensitivity, or non-pre-bundle binding time is a
different valid commitment. It must change the required-context closure and
projection commitments deterministically. It becomes rejectable only when a
future exact consumer supplies its own authoritative expectation through an
existing reviewed boundary.

## Stable Fail-Closed Errors

The private implementation must preserve bounded, payload-free failures. The
correction may refine private suffixes, but tests must pin these semantic
classes:

- `source_closure_incomplete` for missing per-step posture;
- `required_context_binding_mismatched` for wrong bundle, workflow, run, step,
  contract identity, contract version, contract hash, or duplicate step;
- existing required-context execution-binding errors for unknown sensitivity,
  missing immutable step, pre-bundle time, and content-hash mismatch; and
- `cardinality_exceeded` for any checked source, pair, or obligation bound.

Errors and `Debug` output must not expose actor IDs, step IDs, contract IDs,
content hashes, timestamps, or payloads.

## Required Test Matrix

The correction implementation is not complete without explicit tests for the
following cases.

### Identity And Closure Rejection

1. missing per-step contract/binding pair;
2. duplicate pair for one step;
3. wrong immutable bundle binding;
4. wrong workflow identity;
5. wrong run identity;
6. wrong or nonexistent step;
7. substituted contract identity;
8. substituted contract version;
9. substituted contract content hash;
10. binding content-hash mismatch;
11. unknown sensitivity rejected by the execution-binding constructor; and
12. binding time before bundle creation rejected.

### Commitment-Only Semantics

13. a different actor remains a valid alternate commitment and changes both
    the required-context closure commitment and projection commitment;
14. a different known sensitivity remains a valid alternate commitment and
    changes those commitments;
15. a later non-pre-bundle time remains a valid alternate commitment and
    changes those commitments;
16. no extractor result is represented as authority, satisfaction, evidence,
    approval, or execution disposition; and
17. redacted formatting remains payload-safe for every alternate commitment.

### Determinism And Bounds

18. two or more steps produce identical sorted output and commitments under
    every insertion order;
19. the exact required-context pair maximum is accepted;
20. pair maximum plus one is rejected before complete allocation;
21. the exact canonical-source maximum is accepted;
22. source maximum plus one is rejected before complete plan construction;
23. the exact obligation maximum is accepted;
24. obligation maximum plus one is rejected independently of source count;
25. checked arithmetic overflow fails closed; and
26. repeated extraction from the same stored inputs is byte-for-byte stable.

Fixtures for exact maxima must derive their limits from the shared
immutable-bundle bound or reviewed accessors. Tests must not copy the literal
limit into the obligation module.

## Implementation Sequence

1. Amend private type and API documentation to state commitment-only posture.
2. Preserve existing identity, bijection, ordering, redaction, and shared-bound
   checks.
3. Add the complete rejection, alternate-commitment, determinism, and
   independent cardinality matrix.
4. Update the Phase 4 implementation report so it no longer claims
   consumer-owned actor, sensitivity, or freshness validation.
5. Run focused tests, warnings-denied Clippy, the full `workflow-core` suite,
   docs checks, and diff checks.
6. Perform one focused maintainer/security review of the exact correction.

No executor call site may consume the plan or projection during this sequence.

## Runtime Consumer Gate

Executor composition remains unauthorized after this correction.

A future separately governed plan may reopen composition only after naming one
exact existing runtime consumer that owns, without caller-authored parallel
assertions:

- the exact stored immutable run bundle;
- every per-step required-context contract and execution binding;
- the expected actor for the protected operation;
- the authorized maximum sensitivity;
- a trusted current-time and freshness policy;
- the existing durable `GovernanceAssessmentBinding` and reassessment path;
- approval, evidence/check, `SideEffect`, event, retry, wait, replay, report,
  and completion semantics for that exact path.

The consumer must construct or compare the commitment closure inside its own
authority boundary. A generic wrapper may not accept expected actor,
sensitivity, or time as new caller-provided proof. Any mismatch must fail
before invocation, external action, report persistence, or terminal success.

## Non-Goals

This correction does not add:

- executor composition or a runtime wrapper;
- persistence changes or a second durable binding;
- public schema, SDK, CLI, or serialization;
- provider behavior or external writes;
- a new current-authority or required-context resolver;
- dependencies, Salsa, caching, or incremental recomputation;
- release or publication behavior; or
- a claim of independent assurance.

## Acceptance

The correction plan is satisfied only when:

- the implementation truthfully exposes commitment identity rather than
  consumer-owned authority;
- all 26 required cases above are explicit and passing;
- exact maxima and maximum-plus-one failures are independently proven for
  pairs, sources, and obligations;
- no executor path imports or consumes the private projection;
- all failures and formatting remain redaction-safe;
- the focused and full required validation commands pass; and
- focused review accepts the corrected boundary before any later composition
  planning begins.
