# Evidence And Check Obligation-Set Aggregation Plan

Status: the corrected first implementation slice is complete and awaiting
phase-level maintainer review. It adds a crate-private local-check candidate
model and pure structural evaluator only. Declaration provenance remains
explicitly unresolved, and no authoritative aggregate posture, schema,
executor integration, or proportional-governance reassessment is implemented.

See the
[focused plan review](../concepts/EVIDENCE_CHECK_OBLIGATION_SET_AGGREGATION_PLAN_REVIEW.md).
The focused correction is accepted in the
[blocker-fix review](../concepts/EVIDENCE_CHECK_OBLIGATION_SET_AGGREGATION_PLAN_BLOCKER_FIX_REVIEW.md).
The implementation is documented in the
[Local Check Governance Structural Coverage Report](../concepts/LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_REPORT.md).
Phase-level review found a construction-time cross-bundle relabeling blocker in
the private obligation adapter; see the
[Local Check Governance Structural Coverage Review](../concepts/LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_REVIEW.md).
The focused fix now derives obligation identity from candidate binding plus the
exact requirement fingerprint and is documented in the
[Local Check Governance Structural Coverage Blocker Fix Report](../concepts/LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_BLOCKER_FIX_REPORT.md).
Focused re-review accepts the fix in the
[Local Check Governance Structural Coverage Blocker Fix Review](../concepts/LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_BLOCKER_FIX_REVIEW.md).

Related foundations:

- [DocsCheck Attestation Proportional-Governance Integration Plan](docs-check-attestation-proportional-governance-integration-plan.md)
- [DocsCheck Attestation Governance Contribution Review](../concepts/DOCS_CHECK_ATTESTATION_GOVERNANCE_CONTRIBUTION_REVIEW.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)

## 1. Executive Summary

Workflow OS can now produce one verified, requirement-scoped `DocsCheck`
governance contribution. It cannot safely turn that leaf into the aggregate
`evidence_and_checks` workload fact because the kernel does not yet own an
authoritative immutable set of every evidence and check obligation for the
step.

The next implementation should begin with model types only. Those types should
represent an explicitly supplied candidate obligation set and exact structural
coverage for the accepted local-check attestation family. The result is a
non-authoritative structural coverage candidate, not an aggregate workload
fact. It must not claim that today's workflow schemas or repository inference
provide an authoritative obligation source.

Runtime aggregation remains blocked until validated declarations are resolved
from workflow, skill, policy, profile, and steward constraints and frozen into
the immutable run bundle.

## 2. Goals

- Model an exact immutable local-check obligation-set candidate.
- Bind the set to workflow, run, bundle, step, and declaration-source identity.
- Distinguish required and optional obligations without weakening required
  coverage.
- Accept one contribution per expected obligation.
- Reject missing, duplicate, unexpected, mismatched, stale, or unsupported
  contributions.
- Evaluate structural coverage independent of input order.
- Preserve `Failed`, `RequiredUnavailable`, `OptionalUnavailable`, `Unknown`,
  and `Satisfied` semantics.
- Permit candidate `Satisfied` only after complete coverage of the supplied
  candidate set, without creating aggregate authority.
- Preserve execution disposition and disclosure obligation as independent axes.
- Define the declaration and immutable-bundle prerequisites for later runtime
  authority.

## 3. Non-Goals

This plan does not authorize:

- implementation during planning;
- proportional-governance reassessment or selector invocation;
- executor checkpoints or automatic checks;
- treating repository metadata or model inference as enforcement declarations;
- converting structural coverage into
  `GovernanceWorkloadEvidenceCheckPosture`;
- workflow, skill, policy, profile, project, or SDK schema changes in the first
  model-only phase;
- persistence, replay, events, evidence records, reports, or artifacts;
- CLI, UI, onboarding, or example behavior;
- provider calls, SideEffects, writes, hosted behavior, or release changes.

## 4. Source-Of-Truth Boundary

The authoritative obligation set must eventually be derived from validated
governance declarations and frozen into the immutable run bundle before
execution. Candidate sources are:

- workflow step evidence/check requirements;
- skill validation and output-proof requirements;
- policy-gate requirements;
- governance-profile minima; and
- future steward-mandated constraints.

Repository metadata, package scripts, CI configuration, and model analysis may
recommend declarations during onboarding. They are not authoritative until a
user, delegated maintainer, or future steward accepts them through a governed
authoring path and the kernel validates and freezes them.

Today's schemas do not provide a complete canonical declaration source. The
first model phase must represent this missing source honestly and remain
unwired. A caller-supplied list, count, boolean `complete` flag, or opaque hash
must not be treated as proof of completeness. Structural exactness proves only
coverage of the supplied candidate set.

## 5. Candidate Core Model

The smallest candidate private v1 model set is:

- `LocalCheckGovernanceObligationSetCandidate`
- `LocalCheckGovernanceObligationSetBinding`
- `LocalCheckGovernanceObligation`
- `LocalCheckGovernanceRequirementLevel`
- `LocalCheckGovernanceContribution`
- `LocalCheckGovernanceContributionPosture`
- `LocalCheckGovernanceStructuralCoverageCandidate`
- `LocalCheckGovernanceStructuralCoverageDisposition`

The first phase should keep these types crate-private and non-serialized. It
supports only the current local-check attestation obligation family and should
not introduce a generic obligation-kind enum. It should not reuse the aggregate
`GovernanceWorkloadEvidenceCheckPosture` inside obligations, contributions, or
structural coverage candidates.

## 6. Required Obligation-Set Binding

The candidate set binding should include or derive:

- immutable bundle ID, version, and integrity root;
- workflow ID and version;
- run ID;
- exact step ID;
- declaration-set fingerprint;
- obligation-set algorithm version; and
- deterministic obligation-set fingerprint.

In v1, the declaration-source posture is explicitly `Unresolved`. The model may
bind a candidate-set fingerprint for deterministic structural tests, but it
must not label a caller-supplied declaration fingerprint as authoritative.

A future authoritative binding must derive the declaration-set fingerprint
from canonical validated records stored in the bundle. It must not accept a
naked caller assertion in the runtime integration path.

## 7. Obligation Identity

Each local-check obligation identity should be domain-separated and bind:

- the obligation-set binding;
- requirement level;
- exact requirement or declaration fingerprint; and
- a stable source locator that contains no raw path or payload.

The current `DocsCheck` contribution fingerprint should be adapted inside the
same call stack by a private identity-checking adapter. The adapter must not
serialize, recreate, or accept an imported leaf contribution and must not treat
the adapted identity as aggregate authority.

Two obligations with the same command but different requirement, source, step,
or immutable bundle must remain distinct.

## 8. Requirement Levels

The v1 model should support:

- `Required`; and
- `Optional`.

Required obligations may structurally evaluate only to `Satisfied`, `Failed`,
or `RequiredUnavailable`. Optional obligations may additionally evaluate to
`OptionalUnavailable`, but an optional omission must never erase a required
failure or unavailable result.

If an optional check runs and fails its accepted criteria, it remains `Failed`.
Optionality permits absence; it does not make a failed executed check pass.

Unknown requirement level, unsupported obligation kind, or contradictory
metadata must fail validation.

## 9. Complete-Coverage Algorithm

Structural coverage evaluation should:

1. validate the obligation set and immutable binding;
2. canonicalize expected obligation identities;
3. canonicalize supplied contributions independent of input order;
4. reject duplicate expected identities;
5. reject duplicate contribution identities;
6. reject unexpected contributions;
7. reject contribution/set binding mismatches;
8. identify every missing expected obligation;
9. preserve the strictest leaf posture; and
10. return candidate `Satisfied` only when every obligation in the supplied
    candidate set is covered according to its requirement level.

The proposed precedence is:

```text
Failed
  > RequiredUnavailable
  > Unknown
  > OptionalUnavailable
  > Satisfied
```

Missing required coverage should produce `RequiredUnavailable`. Missing
optional coverage should produce `OptionalUnavailable`. A candidate set with
unresolved provenance may produce only structural coverage, never aggregate
workload posture.

An absent or unresolved authoritative set maps to `Unknown` only in a future
reviewed authoritative adapter. A future canonical authoritative empty set may
be vacuously satisfied because it proves no obligations were declared. An
absent, caller-asserted, or unresolved set must never be treated as that empty
authoritative set.

## 10. Coverage Result

The structural coverage candidate should include only bounded facts:

- candidate structural disposition;
- expected, satisfied, failed, required-unavailable, optional-unavailable, and
  missing counts;
- obligation-set fingerprint; and
- structural coverage fingerprint.

It should not expose raw obligation identifiers, commands, paths, source
content, process output, proof material, or provider payloads through Debug.

The result proves exactness only relative to the supplied candidate set. It is
not a validated aggregate workload fact and is not authority to reassess,
execute, approve, persist, or write.

## 11. Relationship To Proportional Governance

The first model must expose no conversion from structural coverage to
`GovernanceWorkloadEvidenceCheckPosture`. `Unknown` remains future aggregate
vocabulary for absent authoritative source resolution; it is not a v1
structural candidate disposition.

Only a later reviewed authoritative adapter may combine a structural coverage
candidate with a canonical obligation-set binding, map it to
`GovernanceWorkloadEvidenceCheckPosture`, and replace the prior aggregate fact
in a proportional-governance reassessment.

That integration must retain existing monotonic rules: runtime facts may
escalate but may not lower workflow, policy, profile, authority, SideEffect,
or steward minima.

Visible disclosure remains a presentation obligation independent of execution
disposition. A local UI may display quiet-capture decisions without changing
their governance mode.

## 12. Configuration And Onboarding

Workflow OS should get most users close to a useful configuration by deriving
recommendations from safe repository metadata and existing workflow/policy
facts. The kernel should present concrete proposed obligations such as build,
test, lint, typecheck, security review, or evidence capture.

Those recommendations must become enforcement only after governed acceptance
and canonical declaration. Pure inference cannot decide completeness, lower a
declared minimum, or silently add/remove required checks during an active run.

Relevant declaration changes must invalidate the obligation-set fingerprint
and require a new immutable run bundle or a separately governed migration.

## 13. Failure And Privacy Posture

Validation and aggregation errors must use stable codes and static messages.
They must not include obligation IDs, hashes, paths, commands, output, source
content, environment values, credentials, provider payloads, or proof material.

Debug output should expose counts and posture only. No model in this phase
should store raw check output or evidence payloads.

## 14. Test Plan

Future model tests should prove:

1. a valid exact obligation set is deterministic;
2. empty required identity or source binding fails closed;
3. duplicate expected obligations are rejected;
4. complete candidate coverage structurally evaluates to `Satisfied` without
   aggregate authority;
5. one failed required contribution structurally evaluates to `Failed`;
6. stale or missing required coverage evaluates to `RequiredUnavailable`;
7. missing optional coverage evaluates to `OptionalUnavailable`;
8. unexpected, duplicate, mismatched, and cross-bundle contributions fail;
9. input ordering does not change result or fingerprint;
10. obligation, declaration, step, or bundle substitution changes identity;
11. unresolved source provenance cannot produce aggregate authority or convert
    to `GovernanceWorkloadEvidenceCheckPosture`;
12. one successful contribution cannot mask another failure or omission;
13. Debug and errors do not leak identifiers or payloads;
14. no state, events, files, reports, artifacts, CLI output, or writes occur;
    and
15. existing attestation, proportional-governance, immutable-bundle, executor,
    provider, and workspace tests remain green.

## 15. Proposed Implementation Sequence

1. Implement crate-private local-check-attestation-only obligation, candidate
   set, contribution, and structural-coverage types.
2. Implement pure deterministic structural evaluation over an explicitly
   supplied candidate set, while keeping runtime construction and aggregate
   conversion unwired.
3. Add focused privacy and complete-coverage tests.
4. Perform phase-level review.
5. Plan canonical evidence/check declaration fields and immutable-bundle
   derivation as a separate schema and resolution phase.
6. Implement and review authoritative derivation from stored canonical records.
7. Only then plan aggregate proportional-governance reassessment.
8. Only after reassessment review may an executor checkpoint be considered.

## 16. Open Questions

- Which declarations belong on workflow steps versus skills, policies, and
  governance profiles?
- Should future evidence obligations and executable checks share one envelope
  or use typed sub-sets under one aggregate?
- How should steward-required obligations compose without allowing runtime
  administrators to mutate active run definitions?
- Which existing first-run recommendations can become safe proposed
  declarations?
- Should optional evidence absence affect disclosure without affecting
  execution disposition?
- What migration posture is required when declaration schemas evolve?

## 17. Final Recommendation

The next implementation prompt should add the crate-private
local-check-attestation candidate model and pure structural evaluator only. It
must remain unwired, label declaration provenance unresolved, and expose no
conversion to aggregate workload posture.

Do not build schema exposure, onboarding mutation, executor integration,
automatic checks, proportional-governance reassessment, persistence, providers,
SideEffects, or writes in that phase.
