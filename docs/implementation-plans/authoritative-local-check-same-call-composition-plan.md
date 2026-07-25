# Authoritative Local-Check Same-Call Composition Plan

Status: implemented and accepted in
[phase-level maintainer review](../concepts/AUTHORITATIVE_LOCAL_CHECK_SAME_CALL_COMPOSITION_REVIEW.md).
The crate-private helper now:

- derives canonical obligations from a validated stored immutable run bundle;
- preflights the complete supplied batch before any process starts;
- verifies exact requirement and command-contract identity against canonical
  declaration records;
- derives required or optional posture from Core-owned declarations;
- executes accepted `DocsCheck` inputs through the existing same-call gate;
- evaluates exact structural coverage; and
- returns bounded results plus the provenance-bearing aggregate evidence/check
  fact.

The helper remains private and unwired. Proportional-governance reassessment,
executor checkpoints, automatic check execution, and default runtime
enforcement remain unimplemented. See the
[plan review](../concepts/AUTHORITATIVE_LOCAL_CHECK_SAME_CALL_COMPOSITION_PLAN_REVIEW.md)
and
[implementation report](../concepts/AUTHORITATIVE_LOCAL_CHECK_SAME_CALL_COMPOSITION_REPORT.md).
The first fact-to-reassessment binding is now planned in the
[Authoritative Local-Check Reassessment Binding Plan](authoritative-local-check-reassessment-binding-plan.md).

Related foundations:

- [Independent Local Check Attestation Plan](independent-local-check-attestation-plan.md)
- [DocsCheck Attestation Runtime Composition Plan](docs-check-attestation-runtime-composition-plan.md)
- [DocsCheck Attestation Proportional-Governance Integration Plan](docs-check-attestation-proportional-governance-integration-plan.md)
- [Evidence And Check Obligation-Set Aggregation Plan](evidence-check-obligation-set-aggregation-plan.md)
- [Canonical Local-Check Declaration And Immutable-Bundle Derivation Plan](canonical-local-check-declaration-immutable-bundle-derivation-plan.md)
- [Authoritative Local-Check Aggregate Posture Conversion Plan](authoritative-local-check-aggregate-posture-conversion-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)

## 1. Executive Summary

Workflow OS currently has every private transformation needed to derive an
authoritative local-check evidence/check fact, but callers must still assemble
those transformations:

```text
stored immutable declarations
  -> authoritative obligation candidate
  -> same-call DocsCheck contribution
  -> caller-supplied requirement-level adaptation
  -> exact structural coverage
  -> authoritative aggregate fact
```

That assembly boundary is too weak for later governance use. In particular,
the current structural adapter accepts a caller-supplied requirement level,
even though required versus optional posture is already owned by the canonical
stored declaration.

The next implementation should add one crate-private Core-owned composition
helper. It should preflight an explicit batch of private `DocsCheck` execution
inputs against one validated `StoredImmutableRunBundle`, execute only accepted
inputs in canonical obligation order through the existing same-call
contribution wrapper, preserve bounded structured check results, evaluate exact
coverage, and return the accepted provenance-bearing aggregate fact.

The helper must not invoke proportional governance. It must not persist or
expose an aggregate posture enum detached from the fact fingerprint. It is the
last private composition boundary before separately reviewed reassessment
binding.

## 2. Product Rationale

Proportional governance and quiet success require trustworthy current facts.
Low-risk work should not pause merely because Workflow OS lacks a composed
path from declared checks to aggregate posture. Conversely, a caller must not
be able to label one successful check as complete evidence, choose optionality,
or pass an unbound `Satisfied` enum into reassessment.

This phase moves existing runtime foundations into one inspectable path:

- the immutable run bundle says which checks exist and which are required;
- Core executes and consumes eligible `DocsCheck` gates;
- exact structural coverage accounts for every declaration;
- the aggregate fact preserves provenance and exact counts; and
- later reassessment can bind the fact fingerprint.

The result reduces caller ceremony without weakening governance.

## 3. Goals

- Add one crate-private same-call composition helper.
- Derive the authoritative obligation set from the validated stored immutable
  run bundle.
- Preflight every supplied check execution input before any process starts.
- Reject duplicate, unexpected, mismatched, cross-bundle, cross-run, and
  cross-step inputs before execution.
- Derive required or optional posture from canonical declarations, never from
  a caller argument.
- Execute accepted `DocsCheck` inputs through the existing private
  contribution wrapper.
- Use canonical obligation order so caller input order cannot change execution
  or fact identity.
- Preserve each bounded `LocalCheckResult` under its existing output-capture
  policy for the immediate caller without copying output into the aggregate
  fact.
- Represent omitted required and optional declarations through the existing
  unavailable coverage postures.
- Evaluate exact structural coverage and convert it to the accepted
  authoritative aggregate fact in the same Core-owned call.
- Return the fact object, including its fingerprint, rather than a detached
  aggregate posture enum.
- Keep errors stable and non-leaking.

## 4. Non-Goals

The first implementation must not add:

- proportional-governance selector invocation or reassessment;
- mutation of `StepGovernanceRuntimeFacts`;
- executor integration or a new executor checkpoint;
- automatic or default check discovery or execution;
- background execution, parallel execution, or check scheduling;
- support for check families beyond the accepted local `DocsCheck` path;
- imported, cached, persisted, replayed, or serialized contributions;
- fact, result, or coverage persistence;
- workflow events, audit events, EvidenceReference creation, WorkReport
  generation, or report artifacts;
- schema, SDK, CLI, UI, or example changes;
- handler installation, hidden handler selection, network access, providers,
  OpenShell integration, SideEffects, or writes;
- hosted or distributed execution;
- reasoning lineage; or
- release posture changes.

## 5. Current Source-Of-Truth Boundaries

| Concern | Current source of truth |
| --- | --- |
| Run, workflow, and immutable definition identity | `StoredImmutableRunBundle` |
| Step-local declared obligation membership | Canonical stored local-check declaration-set record |
| Required versus optional level | Canonical declaration |
| Command, handler, and effective policy commitments | Immutable local-check execution binding |
| Process result | `LocalCheckResult` produced by the registered local handler |
| Independent accepted proof | Private verifier and same-call gate |
| Requirement-scoped posture | Private `DocsCheckGovernanceEvidenceCheckContribution` |
| Exact complete coverage | Private structural coverage evaluator |
| Aggregate evidence/check posture and identity | Private authoritative aggregate fact |

The composition helper may connect these sources. It must not replace or
duplicate them.

## 6. Proposed Private API

Use the smallest private API that preserves results and aggregate authority.
Names remain tentative:

```text
AuthoritativeDocsCheckCompositionInput<'a> {
  stored_immutable_run_bundle: &'a StoredImmutableRunBundle,
  step_id: &'a StepId,
  executions: &'a [DocsCheckAttestationExecutionInput<'a>],
}

AuthoritativeDocsCheckCompositionOutcome {
  results: Vec<LocalCheckResult>,
  fact: AuthoritativeLocalCheckEvidenceCheckFact,
}

compose_authoritative_docs_check_evidence_check_fact(input)
  -> Result<AuthoritativeDocsCheckCompositionOutcome, WorkflowOsError>
```

The input remains crate-private and borrowed. It accepts the existing
execution input so observation time, handler invocation, attestation, gate
consumption, and leaf contribution remain owned by the accepted runtime path.

The outcome should expose read-only crate-private accessors. It should not
expose adapted leaf contributions or structural coverage as reusable authority.

## 7. Preflight Before Execution

All deterministic input checks must complete before the first process starts.
Preflight should:

1. validate the stored immutable bundle through the existing accepted adapter;
2. derive the canonical candidate for the requested step;
3. verify the requested step belongs to the stored workflow;
4. map each supplied execution input to exactly one canonical obligation;
5. require exact stored-bundle, workflow, run, and step equality;
6. require exact attestation-requirement identity;
7. reject duplicate execution inputs for one obligation;
8. reject inputs for undeclared obligations;
9. reject unsupported command/check families;
10. reject inputs whose requirement or command contract does not match the
    canonical declaration; and
11. derive a canonical execution sequence from obligation identity.

Preflight failure returns before process execution. It must not leave partial
results or an aggregate fact.

## 8. Canonical Execution Order

Caller ordering must not control execution order or aggregate identity.
Accepted inputs should be ordered by the canonical obligation fingerprint
already derived from the stored bundle.

V1 remains sequential. Parallel checks introduce cancellation, partial
observation, resource contention, and deterministic ordering questions and
require separate planning.

The canonical empty declaration set executes no process and may produce
authoritative `Satisfied` posture because the stored immutable bundle proves
that the step declared no local-check obligations. Missing or legacy
declaration sources must fail before this distinction is made.

## 9. Same-Call Execution And Contribution

For each preflighted input, the helper should call the existing
`execute_docs_check_governance_contribution(...)` path exactly once.

That path already owns:

- command and requirement compatibility;
- immutable execution binding;
- registered handler selection;
- process execution;
- kernel-owned observation time;
- result construction;
- attestation candidate construction;
- independent verification;
- freshness evaluation at gate consumption; and
- total leaf-posture mapping.

The composition helper must not reconstruct proof, accept imported gate
outcomes, or manufacture leaf contributions.

If one execution returns an error, the helper returns an error and no aggregate
fact. Earlier non-writing local checks may already have run. The first
implementation does not claim transactional rollback. Canonical declarations
already require disabled network and classified non-source-writing SideEffect
posture, so this bounded partial-execution risk is explicit and does not
authorize mutations.

## 10. Core-Derived Leaf Adaptation

The authoritative path must not call the existing adapter with a caller-
supplied `LocalCheckGovernanceRequirementLevel`.

Instead, a focused private adapter should:

1. locate the contribution's obligation in the authoritative candidate;
2. read required or optional level from that canonical obligation;
3. map the existing leaf posture using that level; and
4. bind the adapted contribution to the candidate-set fingerprint.

The unresolved structural-coverage tests may continue using their explicit
test-oriented adapter. The authoritative composition path must make optionality
unselectable by its caller.

## 11. Exact Coverage And Aggregate Conversion

After execution:

1. adapt each produced leaf contribution through the Core-derived
   authoritative adapter;
2. evaluate exact structural coverage against the canonical candidate;
3. account for omitted required declarations as `RequiredUnavailable`;
4. account for omitted optional declarations as `OptionalUnavailable`;
5. preserve executed failures as `Failed`, including optional failures;
6. reject duplicate or unexpected adapted contributions;
7. convert only canonical stored-bundle coverage; and
8. return the existing `AuthoritativeLocalCheckEvidenceCheckFact`.

The strict aggregate precedence remains:

```text
Failed
  > RequiredUnavailable
  > OptionalUnavailable
  > Satisfied
```

The aggregate fact must retain exact counts plus candidate, coverage, and fact
fingerprints. The helper must not return only
`GovernanceWorkloadEvidenceCheckPosture`.

## 12. Freshness Boundary

The existing gate owns observation and consumption time. A stale accepted
proof becomes a `RequiredUnavailable` leaf contribution before aggregation.

V1 should execute and aggregate contributions in one synchronous call and
must not serialize, import, cache, persist, or replay them. This gives a
bounded same-call freshness property but is not a distributed authenticity or
long-term freshness claim.

Any future asynchronous, parallel, persisted, or resumed consumer must add
explicit observation-time commitments, one-time claim or replay semantics, and
fresh reassessment. It must not reuse this private in-memory contract as proof
that an old contribution remains current.

## 13. Relationship To Proportional Governance

This phase stops before reassessment.

The next separately reviewed phase should replace the caller-selected
evidence/check input for one explicit assessment path with a structure that
contains:

- the aggregate posture derived from the accepted fact;
- the aggregate fact fingerprint;
- the immutable bundle binding;
- exact step identity; and
- algorithm identity.

Reassessment must bind the complete fact fingerprint into its own aggregate
fingerprint. It must not copy only `fact.posture()`.

Policy, profile, authority, sensitivity, SideEffect, steward minimum, prior
decision, and runtime-escalation posture remain independent inputs. A satisfied
local-check fact cannot lower any stricter requirement.

## 14. Quiet-Success Boundary

The composition helper does not decide quiet versus visible presentation and
does not decide proceed, approval, or denial.

A later selector may use:

- `Satisfied` as one input supporting quiet capture;
- `OptionalUnavailable` as one input supporting visible disclosure;
- `RequiredUnavailable` or `Failed` as one input requiring denial under the
  current evidence/check mapping.

Those outcomes remain subject to every other governance axis. Evidence,
events, audit, disclosure, and report obligations are not removed by quiet
execution.

## 15. Failure And Privacy Posture

Errors must use stable codes and static messages. They must not include:

- workflow, run, step, requirement, result, invocation, or handler IDs;
- fingerprints;
- command text or arguments;
- paths or working directories;
- stdout, stderr, raw check output, or CI logs;
- source or spec contents;
- environment values;
- provider payloads;
- credentials, authorization headers, private keys, or token-like values; or
- natural-language report content.

`Debug` for the outcome should expose result count, bounded result statuses,
aggregate posture, and proof presence only. It should redact result bodies and
all identities and fingerprints.

## 16. Test Plan

The implementation should prove:

1. canonical populated declarations plus matching successful executions
   produce `Satisfied`;
2. failed execution produces aggregate `Failed`;
3. timed-out or otherwise unavailable required proof produces
   `RequiredUnavailable`;
4. omitted required execution produces `RequiredUnavailable`;
5. omitted optional execution produces `OptionalUnavailable`;
6. an executed optional failure remains `Failed`;
7. canonical empty declarations execute nothing and produce `Satisfied`;
8. legacy or missing declaration sources fail before execution;
9. unknown step fails before execution;
10. cross-bundle, cross-workflow, cross-run, and cross-step inputs fail before
    execution;
11. unexpected and duplicate execution inputs fail before execution;
12. requirement or command mismatch fails before execution;
13. caller input ordering does not change canonical execution order, results,
    coverage, or fact identity;
14. each accepted input executes exactly once;
15. the authoritative path derives requirement level and cannot accept a
    caller-selected level;
16. one successful check cannot mask another failed, missing, or unavailable
    obligation;
17. one execution error returns no aggregate fact and does not execute later
    inputs;
18. the output preserves bounded structured `LocalCheckResult` values;
19. the returned aggregate fact preserves the accepted known identity and
    invalidation behavior;
20. no proportional-governance selector is invoked;
21. the helper creates no Workflow OS state, events, evidence records, reports,
    artifacts, provider calls, SideEffects, or writes, and the accepted command
    contract retains disabled-network and non-source-writing posture;
22. `Debug` and errors do not leak identities or payloads;
23. existing attestation, gate, contribution, structural-coverage, aggregate-
    fact, immutable-bundle, proportional-governance, executor, provider, and
    workspace tests remain green; and
24. `cargo test --workspace` passes.

## 17. Implementation Sequence

1. Add the crate-private composition input and outcome.
2. Add pure batch preflight against the authoritative candidate.
3. Add canonical execution ordering.
4. Add the Core-derived authoritative leaf adapter.
5. Execute the accepted contribution wrapper once per preflighted input.
6. Evaluate structural coverage and convert the aggregate fact.
7. Add focused tests and privacy regressions.
8. Update roadmap and phase documentation.
9. Run phase-level maintainer review.
10. Only after acceptance, plan aggregate-fact binding into one explicit
    proportional-governance reassessment path.

## 18. Review Rejection Criteria

Maintainer review should reject the implementation if:

- any process can start before complete batch preflight;
- caller order controls execution;
- caller-supplied optionality enters the authoritative path;
- imported, cached, persisted, or reconstructed contributions are accepted;
- one leaf can claim aggregate satisfaction;
- missing required or optional declarations disappear;
- an optional executed failure is weakened;
- the helper returns only a posture enum;
- aggregate fact identity is not preserved;
- proportional governance or executor behavior is invoked;
- result preservation copies raw output into governance facts;
- errors or `Debug` leak identities or payloads; or
- documentation implies default, automatic, persisted, or externally enforced
  checks.

## 19. Open Questions

- Should the first outcome preserve results in canonical declaration order or
  in actual execution order? The recommendation is canonical order because v1
  is sequential and both are then identical.
- Should a check execution error return previously completed bounded results?
  The recommendation is no for v1: return no aggregate outcome and disclose
  that non-writing checks may already have run.
- Should an explicit skip input exist? The recommendation is no for v1:
  omission already maps deterministically through canonical required or
  optional level.
- Should the helper accept raw private contributions instead of execution
  inputs? The recommendation is no: executing through the existing wrapper in
  the same call provides the strongest current freshness and no-import
  boundary.

## 20. Final Recommendation

Proceed to planning one private reassessment binding that consumes the
authoritative aggregate fact and its fingerprint.

That planning is now documented in the
[Authoritative Local-Check Reassessment Binding Plan](authoritative-local-check-reassessment-binding-plan.md)
and awaits phase-level maintainer review.

Do not combine that first binding with executor integration, automatic checks,
schema or CLI exposure, providers, OpenShell, SideEffects, writes, hosted
behavior, or default runtime changes.
