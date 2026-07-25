# Authoritative Local-Check Aggregate Posture Conversion Plan

Status: planning complete and accepted. The canonical stored-declaration
adapter and exact structural-coverage evaluator are implemented and accepted.
No aggregate posture conversion, proportional-governance reassessment, or
executor integration is implemented by this plan. See the
[focused plan review](../concepts/AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_PLAN_REVIEW.md).

Related foundations:

- [Evidence And Check Obligation-Set Aggregation Plan](evidence-check-obligation-set-aggregation-plan.md)
- [Canonical Local-Check Declaration And Immutable-Bundle Derivation Plan](canonical-local-check-declaration-immutable-bundle-derivation-plan.md)
- [Canonical Local-Check Declaration Structural-Coverage Adapter Report](../concepts/CANONICAL_LOCAL_CHECK_DECLARATION_STRUCTURAL_COVERAGE_ADAPTER_REPORT.md)
- [Canonical Local-Check Declaration Structural-Coverage Adapter Review](../concepts/CANONICAL_LOCAL_CHECK_DECLARATION_STRUCTURAL_COVERAGE_ADAPTER_REVIEW.md)
- [DocsCheck Attestation Governance Contribution Review](../concepts/DOCS_CHECK_ATTESTATION_GOVERNANCE_CONTRIBUTION_REVIEW.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)

## 1. Executive Summary

Workflow OS can now derive the exact local-check obligation set for one
workflow step from a validated stored immutable run bundle and can evaluate
exact structural coverage from requirement-scoped check contributions. The
remaining boundary is converting that accepted authoritative result into a
bounded aggregate fact that can eventually inform proportional governance.

The first implementation should add one crate-private pure conversion helper
and one crate-private provenance-bearing fact. It must accept only structural
coverage whose source posture is `CanonicalStoredBundle`. The fact should
retain the mapped `GovernanceWorkloadEvidenceCheckPosture`, authoritative
coverage counts, algorithm identity, the structural-coverage commitment, and a
new deterministic fact fingerprint.

The helper must not invoke proportional governance directly. It must not let a
caller supply or relabel the mapped posture. Runtime reassessment, executor
enforcement, automatic check execution, persistence, and quiet-success
presentation remain later reviewed phases.

## 2. Product Rationale

Fresh-pull evaluation confirms that Workflow OS now explains its governance
boundary well. The next product problem is reducing ceremony for low-risk work
without weakening the evidence trail.

Authoritative aggregate check posture is a prerequisite for that behavior:

- complete passing required checks may support quiet execution;
- unavailable optional checks may allow execution while requiring visible
  disclosure;
- unavailable required checks or deterministic failures must fail closed; and
- unresolved or caller-asserted coverage must never be treated as success.

This conversion does not itself make work quiet. It supplies one trustworthy
input to the existing proportional-governance selector, where explicit policy,
authority, sensitivity, SideEffect, profile, runtime-escalation, and steward
minimums remain independently authoritative.

## 3. Goals

- Convert accepted authoritative local-check structural coverage into the
  existing bounded evidence/check posture vocabulary.
- Require canonical stored-bundle provenance before conversion.
- Preserve exact structural-coverage identity and counts in a
  provenance-bearing fact.
- Derive posture inside Core rather than accepting a caller-selected enum.
- Bind the fact to a versioned deterministic payload-free fingerprint.
- Preserve authoritative empty declaration sets as a distinct valid case.
- Preserve optional-unavailable, required-unavailable, and failed semantics.
- Make future proportional-governance consumption possible without creating
  executor behavior in the first phase.
- Keep errors and `Debug` output bounded and non-leaking.

## 4. Non-Goals

This plan does not authorize:

- implementation during planning;
- automatic or default local-check execution;
- command inference, handler registration, or handler defaults;
- executor checkpoints or runtime governance enforcement;
- direct proportional-governance selector invocation;
- treating one leaf contribution as aggregate coverage;
- caller-supplied or repository-inferred obligation authority;
- evidence generation or proof authenticity claims;
- workflow, policy, profile, project, SDK, or public schema changes;
- CLI, UI, onboarding, or example behavior;
- events, persistence, reports, artifacts, or hosted behavior;
- providers, OpenShell integration, SideEffect execution, or writes;
- automatic approvals or release-posture changes.

## 5. Current Accepted Input Boundary

The conversion may consume only
`LocalCheckGovernanceStructuralCoverageCandidate` produced after:

1. `StoredImmutableRunBundle` validation;
2. canonical declaration reference and record resolution;
3. exact one-record-per-step completeness checks;
4. canonical obligation derivation;
5. same-boundary leaf contribution adaptation; and
6. fail-closed exact structural-coverage evaluation.

The conversion must reject
`LocalCheckGovernanceDeclarationSourcePosture::Unresolved`. It must not accept
an independently supplied posture, count set, fingerprint, manifest, record,
or declaration list.

Canonical stored provenance is necessary but not sufficient on its own. The
coverage result must also retain the canonical candidate-set and
structural-coverage fingerprints already derived by Core.

## 6. Supported Obligation Universe

The first conversion covers only the current authoritative local-check
attestation family. Today, this is the only evidence/check obligation family
with:

- typed step declarations;
- canonical resolution;
- immutable-bundle publication;
- requirement-scoped contributions; and
- exact structural coverage.

The resulting fact means "aggregate posture for the complete canonical
local-check obligation universe supported by v1." It does not claim that an
evidence artifact was generated, that arbitrary evidence quality was assessed,
or that future obligation families were satisfied.

Adding another authoritative evidence/check family must require a new reviewed
aggregation algorithm. It must not silently reuse the v1 local-check-only
conversion and continue returning `Satisfied`.

## 7. Candidate Private Model

The smallest justified first model is:

- `AuthoritativeLocalCheckEvidenceCheckFact`
- `AuthoritativeLocalCheckEvidenceCheckFactAlgorithm`
- `convert_authoritative_local_check_coverage`

Names may be refined to match local conventions. The types should remain
crate-private and non-serialized in the first phase.

The fact should contain only:

- algorithm identity;
- mapped `GovernanceWorkloadEvidenceCheckPosture`;
- expected, satisfied, failed, required-unavailable,
  optional-unavailable, and missing counts;
- canonical candidate-set fingerprint;
- structural-coverage fingerprint; and
- deterministic aggregate-fact fingerprint.

The fact should expose read-only accessors. `Debug` must disclose only the
algorithm, mapped posture, and bounded counts while redacting fingerprints.

## 8. Deterministic Mapping

The conversion mapping is exact:

| Authoritative structural disposition | Aggregate workload posture |
| --- | --- |
| `Satisfied` | `Satisfied` |
| `OptionalUnavailable` | `OptionalUnavailable` |
| `RequiredUnavailable` | `RequiredUnavailable` |
| `Failed` | `Failed` |

The helper does not manufacture `Unknown`. Missing, legacy, unresolved,
ambiguous, incomplete, duplicate, or mismatched authoritative sources fail
before conversion. A later consumer may use `Unknown` when no accepted
authoritative fact is available, but it must do so explicitly and must not
invoke this converter with invented coverage.

Optionality permits absence; it does not excuse failure. An optional check that
runs and fails remains aggregate `Failed`, matching the accepted structural
precedence.

## 9. Authoritative Empty Sets

A canonical stored declaration record with zero declarations proves that the
step has no v1 local-check obligations. Exact structural evaluation therefore
produces:

- expected count zero;
- all outcome counts zero;
- missing count zero; and
- `Satisfied`.

The converter may preserve that result as aggregate `Satisfied`.

A legacy bundle with no canonical declaration source, an unresolved
caller-supplied empty candidate, or a missing declaration record is not the
same condition. Those cases must fail before conversion and must never receive
vacuous satisfaction.

## 10. Fact Fingerprint

The fact fingerprint should use a new versioned domain separator and the
repository's fixed-width length framing. It should bind:

- conversion algorithm;
- mapped posture;
- every bounded count;
- canonical candidate-set fingerprint; and
- structural-coverage fingerprint.

Identical accepted input must produce the same fingerprint. Changes to source
posture, obligation identity, coverage, contribution posture, counts, or
algorithm must change it.

The first implementation should include a stable known vector and a
delimiter-collision regression. Fingerprints must remain redacted from
`Debug`, errors, and ordinary display.

## 11. Validation And Failure Behavior

The conversion must fail closed when:

- source posture is not `CanonicalStoredBundle`;
- counts are internally inconsistent;
- expected count does not equal the sum of terminal outcome counts;
- missing count exceeds unavailable coverage;
- satisfied coverage is paired with non-zero unavailable, failed, or missing
  counts;
- the structural disposition contradicts the strictest count posture; or
- an unsupported algorithm or state is encountered.

The current structural evaluator already constructs consistent results. These
checks are defense in depth at the authority-changing conversion boundary.

Errors should use stable
`local_check_attestation.aggregate_posture.*` codes and static messages. They
must not include workflow, run, bundle, step, command, path, fingerprint,
provider, output, credential, or payload values.

## 12. Proportional-Governance Relationship

The accepted workload selector currently maps evidence/check posture as
follows:

| Aggregate posture | Existing selector effect |
| --- | --- |
| `Satisfied` | quiet requirement |
| `OptionalUnavailable` | proceed with visible disclosure |
| `Unknown` | approval required |
| `RequiredUnavailable` | denied |
| `Failed` | denied |

This phase does not call that selector. It only produces an authoritative fact
that a later runtime composition may consume.

That later composition must bind the fact fingerprint into the reassessment
input or durable assessment identity. Passing only the mapped enum would lose
provenance and permit caller assertion. The existing caller-supplied
`StepGovernanceRuntimeFacts.evidence_and_checks` field must not become the
automatic runtime authority path for this fact without a separately reviewed
binding change.

## 13. Quiet-Success Boundary

Quiet success remains a composed decision, not a property of a check result.

Even an authoritative `Satisfied` fact may not produce quiet execution when:

- policy requires approval or visible disclosure;
- authority is unavailable or approval-bound;
- sensitivity is elevated or restricted;
- a SideEffect requires stronger governance;
- the active profile or steward minimum is stricter;
- runtime escalation is present; or
- another safety-relevant fact is unknown.

Evidence, audit, disclosure, and report obligations remain intact for quiet
work. The conversion must not suppress records or presentation obligations.

## 14. Privacy And Security

The converter may inspect only bounded typed posture, counts, and
fingerprints. It must not read or retain:

- command text or arguments;
- working directories or file paths;
- raw check output or CI logs;
- source or spec contents;
- provider payloads;
- environment values;
- credentials, authorization headers, private keys, or token-like values; or
- natural-language summaries.

The fact is a provenance commitment, not proof authenticity. It inherits the
accepted limits of the local-check verifier and stored immutable bundle.

## 15. Test Plan

The first implementation should prove:

1. canonical satisfied coverage maps to `Satisfied`;
2. canonical optional-unavailable coverage maps to
   `OptionalUnavailable`;
3. canonical required-unavailable coverage maps to
   `RequiredUnavailable`;
4. canonical failure maps to `Failed`;
5. optional executed failure remains `Failed`;
6. canonical empty coverage maps to `Satisfied`;
7. unresolved populated coverage is rejected;
8. unresolved empty coverage is rejected;
9. mapped posture cannot be caller-selected;
10. fact counts match accepted coverage;
11. contradictory disposition/count state fails closed if a focused private
    fixture can construct it safely;
12. identical accepted input is deterministic;
13. any posture, count, candidate, coverage, or algorithm change invalidates
    the fact fingerprint;
14. a stable known fingerprint vector is preserved;
15. fixed-width framing prevents delimiter collisions;
16. `Debug` redacts fingerprints and identities;
17. errors use stable codes and do not leak bounded test secrets;
18. existing unresolved structural-coverage tests pass;
19. authoritative adapter tests pass;
20. DocsCheck contribution and attestation tests pass;
21. proportional-governance assessment tests pass; and
22. `cargo test --workspace` passes.

## 16. First Implementation Scope

The next implementation prompt should add only:

- the private algorithm enum or constant;
- the private provenance-bearing fact;
- the pure authoritative conversion helper;
- stable non-leaking validation errors;
- focused tests;
- narrow documentation updates; and
- an end-of-phase report.

It should not modify executor requests, runtime facts, public exports, schemas,
CLI output, state stores, events, reports, or artifacts.

## 17. Later Runtime Sequence

After implementation and maintainer review:

1. plan exact same-call composition from stored declarations, executed check
   contributions, structural coverage, and aggregate fact;
2. bind the aggregate fact fingerprint into proportional-governance
   reassessment rather than passing only a caller-selected enum;
3. add one explicit opt-in executor checkpoint;
4. prove retry and approval-resume invalidation against the immutable bundle
   and fresh check facts;
5. preserve quiet execution for complete low-risk work while retaining
   records;
6. review before any default executor behavior; and
7. keep additional check or evidence families separately governed.

## 18. Review Criteria

Maintainer review should reject the phase if:

- unresolved coverage can convert;
- the mapped posture is caller supplied;
- canonical empty and missing source are conflated;
- one leaf contribution can bypass complete structural coverage;
- fact identity omits source or structural-coverage commitments;
- future obligation families could be silently treated as satisfied;
- the helper invokes proportional governance or changes runtime behavior;
- errors or `Debug` leak identities or payloads; or
- docs imply automatic checks, quiet-success enforcement, or evidence
  generation.

## 19. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785007002646463000-2`
- approval:
  `approval/run-1785007002646463000-2/planning-approved`
- presentation: `presentation/f21b03952a913e08`
- approval outcome: granted by delegated maintainer through proof enforcement
- event summary: 39 events, one approval, zero retries, zero escalations
- validation: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: accepted-contract inspection, planning authoring,
  documentation validation, and diff inspection
- missing coverage: the kernel coordinated governance only; it did not author
  files, run implementation checks, generate a WorkReport artifact, or
  simulate runtime conversion behavior

## 20. Final Recommendation

Proceed next to the private authoritative aggregate-fact model and conversion
helper only.

Do not begin executor integration, default check execution, schema exposure,
provider work, or broader quiet-success enforcement in the same phase.
