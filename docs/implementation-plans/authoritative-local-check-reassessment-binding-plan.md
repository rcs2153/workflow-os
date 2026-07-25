# Authoritative Local-Check Reassessment Binding Plan

Status: accepted after focused blocker correction and re-review. Complete
deterministic wrapper preflight precedes process execution, and one private
bound-assessment value makes fact binding inseparable from assessment
authority. See the original
[plan review](../concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_REVIEW.md),
[blocker fix report](../concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_BLOCKER_FIX_REPORT.md),
and
[focused re-review](../concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_BLOCKER_FIX_REVIEW.md).

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)
- [DocsCheck Attestation Proportional-Governance Integration Plan](docs-check-attestation-proportional-governance-integration-plan.md)
- [Evidence And Check Obligation-Set Aggregation Plan](evidence-check-obligation-set-aggregation-plan.md)
- [Authoritative Local-Check Aggregate Posture Conversion Plan](authoritative-local-check-aggregate-posture-conversion-plan.md)
- [Authoritative Local-Check Same-Call Composition Plan](authoritative-local-check-same-call-composition-plan.md)

## 1. Executive Summary

Workflow OS can now derive one authoritative aggregate local-check fact from:

- canonical stored declarations;
- exact immutable command and requirement identity;
- kernel-observed `DocsCheck` execution;
- same-call attestation and freshness gating; and
- exact structural coverage.

The proportional-governance reassessment path still accepts
`GovernanceWorkloadEvidenceCheckPosture` directly through
`StepGovernanceRuntimeFacts`. Copying only `fact.posture()` into that public
input would discard the aggregate fact fingerprint. Two different
authoritative check universes could then produce the same posture and become
indistinguishable to reassessment.

The next implementation should add one crate-private, explicit, unwired
composition boundary. It should preflight all deterministic wrapper context,
invoke the accepted authoritative local-check same-call helper itself, replace
the selected step's caller-optional evidence/check posture with the Core-
derived posture, run the existing immutable-bundle assessment, and return one
private bound-assessment value that commits to both the local-check fact and
the resulting assessment.

This phase must not alter the public runtime-fact model, invoke an executor,
activate quiet success, or persist the binding.

## 2. Product Rationale

The current product review is correct: the next challenge is reducing ceremony
for low-risk work while preserving the evidence trail that distinguishes
Workflow OS.

That requires a stronger invariant than:

```text
caller says checks are satisfied
  -> selector may proceed quietly
```

The safe path is:

```text
immutable declarations
  -> observed checks
  -> exact coverage
  -> authoritative fact plus fingerprint
  -> fact-bound reassessment
  -> later enforcement
```

This phase closes only the fact-to-reassessment identity gap. It does not make
the reassessment enforceable runtime authority.

## 3. Goals

- Add one crate-private authoritative local-check reassessment composition
  helper.
- Invoke the accepted same-call composition helper inside that call rather
  than accepting a detached posture or imported leaf contributions.
- Bind the exact stored bundle, workflow, run, and step.
- Require exactly one matching selected-step runtime-fact record.
- Reject a caller-supplied evidence/check posture for the selected step.
- Preserve all other runtime facts and explicit governance minima unchanged.
- Inject only the authoritative fact's posture for the selected step.
- Reuse the accepted immutable-bundle reassessment path.
- Return bounded local-check results plus one private bound-assessment value
  that owns the complete aggregate fact, immutable assessment set, and new
  binding fingerprint.
- Bind the new fingerprint to aggregate fact algorithm and identity, selected
  step, immutable bundle, and reassessment identity.
- Keep errors and `Debug` stable, bounded, and non-leaking.
- Preserve monotonic governance: a satisfied fact cannot weaken profile,
  policy, authority, sensitivity, SideEffect, prior-decision, runtime
  escalation, or steward minima.

## 4. Strict Non-Goals

The first implementation must not add:

- executor integration or a new executor checkpoint;
- default, automatic, background, or parallel local-check execution;
- runtime quiet-success activation or visible-disclosure presentation;
- mutation of a `WorkflowRun`, snapshot, event history, or durable binding;
- persistence, events, evidence records, WorkReports, or artifacts;
- public API or serialization for the new private binding;
- workflow, policy, project, SDK, or schema changes;
- CLI, UI, onboarding, or example behavior;
- additional check families;
- imported, cached, persisted, replayed, or caller-constructed check facts;
- provider calls, OpenShell, SideEffects, external writes, or network access;
- hosted or distributed behavior;
- reasoning lineage;
- enterprise administration; or
- release-posture changes.

## 5. Current Source-Of-Truth Boundaries

| Concern | Source of truth |
| --- | --- |
| Run and immutable definitions | Validated `StoredImmutableRunBundle` |
| Selected step and local-check universe | Canonical stored declaration-set record |
| Required versus optional check level | Canonical declaration |
| Command and handler commitments | Immutable local-check execution binding |
| Current check result | Existing registered local handler |
| Accepted check proof | Existing same-call attestation gate |
| Complete check posture and identity | `AuthoritativeLocalCheckEvidenceCheckFact` |
| Other current runtime facts | Exact `StepGovernanceRuntimeFacts` records |
| Deterministic workload assessment | Existing immutable-bundle reassessment |
| New fact-to-assessment commitment | Private binding defined by this plan |

The new helper connects these sources. It must not replace or duplicate them.

## 6. Why A Detached Fact Argument Is Insufficient

The aggregate fact is crate-private and constructible only through reviewed
Core paths, but accepting it independently still permits accidental pairing
with the wrong reassessment call.

The stronger first boundary should accept the existing same-call composition
input and invoke
`compose_authoritative_docs_check_evidence_check_fact(...)` internally. The
caller supplies executions and other runtime facts, not an aggregate posture,
leaf contribution, structural coverage candidate, or aggregate fact.

The helper should retain and return the resulting fact. It should never
reconstruct a fact or accept one from persistence.

## 7. Candidate Private API

Use the smallest private API, with names still tentative:

```text
AuthoritativeLocalCheckReassessmentInput<'a> {
  local_check: AuthoritativeDocsCheckCompositionInput<'a>,
  profile: GovernanceStrictnessProfile,
  runtime_facts: &'a [StepGovernanceRuntimeFacts],
}

AuthoritativeLocalCheckBoundAssessment {
  local_check_fact: AuthoritativeLocalCheckEvidenceCheckFact,
  assessment_set: ImmutableBundleGovernanceAssessmentSet,
  binding_fingerprint: SpecContentHash,
}

AuthoritativeLocalCheckReassessmentOutcome {
  results: Vec<LocalCheckResult>,
  bound_assessment: AuthoritativeLocalCheckBoundAssessment,
}

compose_authoritative_local_check_reassessment(input)
  -> Result<AuthoritativeLocalCheckReassessmentOutcome, WorkflowOsError>
```

Both values remain crate-private, non-serializable, and read-only. The outcome
must not expose the raw `ImmutableBundleGovernanceAssessmentSet` or local-check
fact as independently reusable authority. It may expose bounded posture/count
inspection and the bound fingerprint. A future runtime consumer must accept
the bound value or a separately reviewed durable projection derived from it.

`Debug` should expose only bounded posture/count metadata and redact results,
identities, and fingerprints.

## 8. Runtime-Fact Injection Boundary

`StepGovernanceRuntimeFacts` currently accepts an optional caller-selected
evidence/check posture. Existing public APIs and compatibility tests must
remain unchanged.

The new private helper should:

1. validate the exact runtime-fact set through the existing reassessment path;
2. locate the record matching the local-check composition step;
3. require its `evidence_and_checks` field to be absent;
4. clone the records through one private Core-owned replacement helper;
5. set only that selected step's evidence/check posture to
   `local_check_fact.posture()`; and
6. leave authority, SideEffect, prior decision, runtime escalation, and steward
   minima unchanged.

A caller-supplied selected-step evidence/check posture is ambiguous and must
fail closed. The helper must not choose between caller and authoritative
values.

Runtime facts for other steps remain explicit under the current reassessment
contract. This first phase does not claim authoritative check coverage for
other steps.

## 9. Complete Deterministic Preflight Before Process Execution

Before invoking local-check composition or starting any process, a pure
preflight stage must require:

- the composition and reassessment use the same stored bundle;
- the selected step belongs to the immutable workflow;
- canonical declarations exist for the selected step;
- runtime facts contain exactly one record for every immutable workflow step;
- the selected step has exactly one runtime-fact record;
- selected-step evidence/check posture is absent before Core injection; and
- the existing reassessment resolves every workflow, skill, policy, and
  runtime-fact identity exactly.

The preflight should prepare the validated runtime-fact replacement context but
must not manufacture a final check posture or assessment. The implementation
may factor existing pure immutable-bundle resolution so this validation is not
duplicated semantically.

Only after wrapper preflight succeeds may the same-call local-check helper run.
That helper then performs its own complete requirement and command preflight
before process execution.

After composition, the wrapper must require the produced fact's candidate-set
fingerprint to match the candidate derived for the exact preflighted bundle and
step before reassessment.

## 10. Binding Fingerprint

Add one private, versioned binding algorithm and fingerprint.

The fingerprint should commit, with fixed-width framing, to:

- a new domain separator;
- immutable bundle ID, version, and root;
- workflow and run identity;
- selected step identity;
- local-check fact algorithm identifier;
- local-check fact fingerprint;
- local-check candidate-set fingerprint;
- local-check structural-coverage fingerprint;
- resulting workload assessment algorithm;
- selected step workload-assessment input fingerprint;
- complete assessment-set algorithm; and
- complete assessment-set aggregate fingerprint.

Binding the complete assessment set ensures facts for other steps and other
governance axes remain visible in the resulting authority boundary. Binding
the selected assessment prevents the local-check fact from being detached from
the decision it influenced.

The helper must return one bound-assessment object, not a raw assessment set
plus a separable fingerprint. The bound value owns both identities. No
accessor may return the raw set as independently reusable authority.

## 11. Invalidation Semantics

The build-cache analogy becomes explicit:

- equal immutable inputs, check observations, runtime facts, profile, and
  algorithms produce an equal binding fingerprint;
- any decision-relevant local-check fact change changes the binding;
- a different candidate or structural coverage identity changes the binding
  even when aggregate posture is unchanged;
- a changed non-check governance axis changes the selected assessment or
  assessment-set fingerprint and therefore changes the binding;
- workflow, run, step, or bundle substitution fails or changes the binding;
- absence of a current fact returns an error, not `Unknown`; and
- no prior binding may be silently reused.

The implementation must include a stable known vector and direct invalidation
tests for every current fingerprint input.

## 12. Monotonic Governance Semantics

The local-check fact changes one workload axis only.

The existing selector remains authoritative for the combined result:

- `Satisfied` may support proceed with quiet capture;
- `OptionalUnavailable` may require visible disclosure;
- `RequiredUnavailable` and `Failed` require denial under the current mapping;
- stricter profile, policy, authority, sensitivity, SideEffect, prior-decision,
  runtime-escalation, or steward requirements remain strict; and
- no check fact can grant authority, approve a SideEffect, lower sensitivity,
  or override a denial.

This phase computes a review-only result. It does not enforce or present it.

## 13. Freshness And Replay Posture

The new helper invokes local-check composition and reassessment synchronously
in one call. It does not serialize, persist, import, or replay a fact.

This provides the strongest current local same-call boundary but is not:

- distributed authenticity;
- durable one-time claim enforcement;
- proof that a process result remains current after the call; or
- permission to reuse the binding on retry or approval resume.

Future runtime integration must reassess from current observations and compare
the new binding with the durable expected binding at each time-of-use boundary.

## 14. Failure And Partial-Execution Semantics

All deterministic preflight in the local-check helper still completes before
the first process starts.

If wrapper preflight fails, no clock or process is used. If local-check
execution fails, no reassessment or bound value is returned. If reassessment
or binding construction fails after checks completed, no bound value is
returned. The local non-source-writing checks cannot be rolled back, and the
implementation must not claim otherwise.

No workflow state, event, evidence record, report, artifact, or external
provider is mutated by this helper.

## 15. Privacy And Redaction

Errors must use stable codes and static messages. They must not include:

- workflow, run, step, requirement, result, invocation, or handler IDs;
- fingerprints;
- command text or arguments;
- paths or working directories;
- stdout, stderr, check output, or CI logs;
- source or spec contents;
- environment values;
- provider payloads;
- credentials, authorization headers, private keys, or tokens; or
- natural-language report content.

The binding stores commitments and typed posture only. `Debug` must redact
results, identities, and fingerprints.

## 16. Test Plan

The implementation should prove:

1. successful canonical composition produces a fact-bound reassessment;
2. the selected assessment consumes the fact's posture;
3. the private bound value retains the fact and assessment identities;
4. selected-step caller-supplied evidence/check posture is rejected;
5. other runtime facts and minima remain unchanged;
6. a stricter non-check axis cannot be weakened by `Satisfied`;
7. required unavailable and failed facts remain denied;
8. optional unavailable retains visible disclosure behavior;
9. every deterministic bundle, workflow, run, step, immutable-definition, or
   runtime-fact mismatch fails before clock or process use;
10. candidate or coverage substitution fails closed or invalidates binding;
11. duplicate, missing, extra, or mismatched runtime facts fail closed;
12. a local-check execution error returns no reassessment or binding;
13. a reassessment error returns no binding;
14. equal complete inputs produce equal binding identity;
15. every binding-fingerprint input independently invalidates identity;
16. ambiguous delimiter inputs cannot collide;
17. a stable known vector pins v1 identity;
18. output preserves bounded local-check results and bound assessment posture
    without exposing a raw assessment set as independent authority;
19. `Debug` and errors do not leak identities, fingerprints, or payloads;
20. no executor, persistence, event, report, artifact, provider, SideEffect, or
    write path is invoked;
21. existing local-check, proportional-governance, immutable-bundle, executor,
    provider, and workspace tests remain green; and
22. `cargo test --workspace` passes.

## 17. Implementation Sequence

1. Add the private input, outcome, bound-assessment value, and versioned
   binding algorithm.
2. Add pure complete wrapper preflight over bundle, step, immutable
   definitions, and runtime-fact shape.
3. Add the private selected-step evidence/check replacement helper.
4. Invoke accepted local-check composition only after wrapper preflight.
5. Verify produced candidate identity against the preflighted exact step.
6. Invoke existing immutable-bundle reassessment with Core-injected posture.
7. Resolve the selected step assessment.
8. Compute the fact-to-assessment binding fingerprint.
9. Construct one inseparable private bound-assessment value.
10. Add focused semantic, preflight, invalidation, privacy, and authority-
    surface tests.
11. Update roadmap and phase documentation.
12. Run phase-level maintainer review.

Only after acceptance should Workflow OS plan one opt-in executor consumer.

## 18. Review Rejection Criteria

Maintainer review should reject implementation if:

- it accepts a detached posture, imported fact, leaf contribution, or coverage
  candidate;
- caller-selected evidence/check posture survives for the selected step;
- fact fingerprint is omitted from the new binding;
- equal posture with different fact identity produces the same binding;
- any deterministic wrapper mismatch can be discovered only after a process
  starts;
- the outcome or an accessor returns a raw assessment set as reusable
  authority independently of the fact binding;
- another governance axis can be overwritten or weakened;
- current public runtime-fact behavior changes;
- the helper invokes an executor or persists authority;
- the helper claims durable freshness or replay protection;
- errors or `Debug` leak identities or payloads; or
- documentation implies automatic checks or active quiet success.

## 19. Open Questions

- Should the bound value retain the complete assessment set or only the
  selected assessment? Recommendation: privately retain the complete set and
  bind its aggregate fingerprint so other steps remain visible, but do not
  expose the raw set as independent authority.
- Should the fact be accepted by reference? Recommendation: no for the first
  path; invoke same-call composition internally and return the produced fact.
- Should selected-step caller posture be ignored or rejected? Recommendation:
  reject it to expose ambiguous authority rather than silently overwrite it.
- Should runtime facts for other steps require authoritative check facts now?
  Recommendation: no. This is one explicit selected-step path; broader exact
  fact universes require separate composition.
- Should the first binding be serializable? Recommendation: no. Keep it private
  and in memory until a durable consumer is designed.

## 20. Final Recommendation

Implement only the accepted private same-call fact-to-reassessment binding and
focused tests. Do not combine it with executor wiring, automatic checks,
runtime quiet-success activation, persistence, events, schemas, providers,
OpenShell, SideEffects, writes, hosted behavior, or release changes.
