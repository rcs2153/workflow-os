# DocsCheck Attestation Proportional-Governance Integration Plan

Status: The corrected requirement-scoped contribution wrapper is implemented
and phase-level review accepts it with non-blocking test-depth follow-ups. It is
documented in the
[DocsCheck Attestation Governance Contribution Report](../concepts/DOCS_CHECK_ATTESTATION_GOVERNANCE_CONTRIBUTION_REPORT.md)
and
[Review](../concepts/DOCS_CHECK_ATTESTATION_GOVERNANCE_CONTRIBUTION_REVIEW.md).

Focused review found that one leaf `DocsCheck` gate cannot replace the
aggregate evidence/check workload fact. The corrected plan stops the first
implementation at a requirement-scoped contribution and keeps aggregate
reassessment blocked until an authoritative complete obligation set exists.
Focused re-review accepts that correction with a dedicated private leaf-posture
type. See the
[focused plan review](../concepts/DOCS_CHECK_ATTESTATION_PROPORTIONAL_GOVERNANCE_INTEGRATION_PLAN_REVIEW.md).

Related foundations:

- [DocsCheck Attestation Consumer Integration Plan](docs-check-attestation-consumer-integration-plan.md)
- [DocsCheck Attestation Consumer Integration Review](../concepts/DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_REVIEW.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)

## 1. Executive Summary

Workflow OS can execute one bounded `DocsCheck`, independently verify its
payload-free attestation, and consume the accepted proof in a same-call gate.
The proportional-governance workload assessment accepts one aggregate
evidence/check posture.

One leaf check cannot establish that aggregate posture. The next bounded step
is a private same-call wrapper that maps the gate outcome to an exact
requirement-scoped contribution. It must not replace the aggregate workload
fact or invoke proportional-governance reassessment.

Aggregate reassessment requires a later authoritative set of every evidence
and check obligation for the exact immutable step. That model does not exist
today. This plan fails closed rather than allowing partial proof to stand in
for complete coverage.

## 2. Goals

- Give the accepted gate its first requirement-scoped governance contribution.
- Define a total deterministic leaf mapping.
- Bind each contribution to the exact immutable run, step, and requirement.
- Prevent one contribution from claiming aggregate satisfaction.
- Preserve independent execution and disclosure axes.
- Define complete-coverage prerequisites for later reassessment.
- Keep failures stable, bounded, and non-leaking.

## 3. Non-Goals

This plan does not authorize:

- implementation during planning;
- aggregate evidence/check satisfaction or proportional-governance
  reassessment;
- executor integration or workflow-state mutation;
- automatic, default, or background checks;
- handler discovery or default registration;
- proof import, cache, persistence, replay, or serialization;
- events, audit records, evidence attachment, reports, or artifacts;
- workflow, policy, or project schema changes;
- YAML configuration for proportional governance;
- CLI, UI, SDK, or example behavior;
- automatic approval or model self-approval;
- provider calls, SideEffects, external writes, or network access;
- hosted runners, enterprise administration, or release changes.

## 4. Current Foundation And Gap

Accepted foundations include immutable run bundles, deterministic workload
assessment, independent execution and disclosure axes, explicit minima,
immutable assessment fingerprints, and the same-call `DocsCheck` attestation
gate.

The gate establishes one exact requirement result. The workload assessment's
`evidence_and_checks` field is aggregate and may summarize several obligations.
No authoritative immutable model currently enumerates every independent
evidence/check obligation for a step.

## 5. Selected First Boundary

Add one crate-private same-call wrapper, tentatively:

```text
execute_docs_check_governance_contribution(input)
  -> Result<DocsCheckGovernanceContributionOutcome, WorkflowOsError>
```

The wrapper should execute the accepted attestation gate and consume its
private outcome immediately. It should return the bounded check result and one
private `DocsCheckGovernanceEvidenceCheckContribution` containing only:

- a domain-separated obligation fingerprint derived inside Core from the exact
  immutable run binding, step identity, and attestation requirement
  fingerprint; and
- one dedicated private `GovernanceEvidenceCheckContributionPosture` with only
  `Satisfied`, `Failed`, and `RequiredUnavailable` variants.

The contribution must not store
`GovernanceWorkloadEvidenceCheckPosture`. That existing type represents the
aggregate workload fact and should remain unavailable at this leaf boundary.

The wrapper must not accept a separately supplied gate outcome, proof,
obligation fingerprint, aggregate posture, or workload assessment input. It
must not call `assess_proportional_governance_workload(...)`.

## 6. Deterministic Leaf Mapping

| Gate outcome | Contribution posture | Meaning |
| --- | --- | --- |
| `Satisfied` | `Satisfied` | The exact current invocation produced accepted proof and passed same-call consumption. |
| `NotSatisfied(ResultStatusNotAccepted)` | `Failed` | The required deterministic check completed outside its accepted statuses. |
| `NotSatisfied(FreshnessExpired)` | `RequiredUnavailable` | Accepted proof cannot satisfy the requirement at consumption time. |

Freshness expiry is not check execution failure. Mapping it to `Failed` would
erase a useful distinction; mapping it to `OptionalUnavailable` or `Unknown`
would weaken a required gate.

Any future gate reason requires explicit review. There must be no wildcard
fallback to satisfaction or optional posture.

## 7. Complete-Coverage Prerequisite

One contribution cannot produce aggregate `Satisfied`. Before aggregate
reassessment is authorized, a separately reviewed model must:

1. derive every expected obligation identity for the exact immutable step from
   an authoritative validated source;
2. reject missing, duplicate, unexpected, mismatched, ambiguous, or unsupported
   coverage;
3. combine contributions independent of input order;
4. return aggregate `Satisfied` only when every required obligation is present,
   current, and satisfied;
5. preserve `Failed` and `RequiredUnavailable` distinctions;
6. bind the aggregate to the immutable definition root; and
7. only then replace the aggregate workload fact and invoke the existing
   monotonic selector.

Current schemas do not declare the complete attestation-requirement set. The
first implementation must not invent that set, accept a free-form count, infer
exclusivity from one gate, or treat caller omission as completeness.

## 8. Execution And Disclosure Axes

The future aggregate integration will change a workload fact, not a UI mode.
The existing selector will continue to choose execution disposition separately
from quiet or visible disclosure.

Visible disclosure is an operator-presentation obligation, not execution
authority. A local UI may later display quiet-capture decisions live without
altering the kernel decision.

## 9. Configuration And Inference

This phase adds no YAML fields. The gate contributes one verified leaf fact; it
does not claim current onboarding or schemas enumerate every required check.

Future onboarding may infer workload posture from validated repository,
workflow, policy, capability, and runtime metadata. Inference must remain
deterministic, inspectable, and bounded by explicit minima. Model suggestions
cannot silently lower enforcement.

The future aggregate must use the immutable definition root and assessment
fingerprint for invalidation. The leaf obligation fingerprint adds exact scope;
it is not a mutable cache key.

## 10. Proof And Identity Boundary

The wrapper trusts only the private gate disposition returned in its own call
stack. The proof fingerprint remains a commitment, not independent
authenticity or reusable authority.

The wrapper must not create an `EvidenceReference`, expose accepted proof, or
copy proof into an assessment. Identity and substitution checks remain owned
by the gate. Future aggregate, persisted, or asynchronous consumers require
separate coverage, one-time claim, and replay semantics.

## 11. Failure Behavior

The wrapper should fail only for invalid internal boundaries, including an
unsupported future reason, impossible gate outcome, or obligation-fingerprint
construction failure. Gate execution and verification errors must propagate
without a contribution.

Errors must use stable codes and static messages without IDs, hashes, paths,
commands, output, source content, environment values, credentials, provider
payloads, or proof material.

## 12. Runtime And Compatibility

- Existing executor, reassessment, immutable-bundle, composition, and gate APIs
  remain unchanged.
- No state, snapshots, events, reports, or artifacts are created or mutated.
- No public API or serde contract is introduced.
- No aggregate assessment is produced.

This is a leaf-fact adapter, not runtime enforcement.

## 13. Privacy And Redaction

The wrapper may observe only typed gate posture and exact bounded gate input.
It must not retain raw process output, commands, paths, environment values,
source contents, credentials, tokens, or provider payloads.

Debug output should expose contribution posture and redact obligation identity.

## 14. Future Test Plan

First implementation tests should prove:

1. satisfied maps to a requirement-scoped `Satisfied` contribution;
2. unaccepted result maps to `Failed`;
3. freshness expiry maps to `RequiredUnavailable`;
4. proof fingerprint presence alone is never authority;
5. immutable run, step, or requirement substitution changes obligation identity;
6. imported gate outcomes and obligation identities are not accepted;
7. equal inputs produce equal contribution identities and postures;
8. gate errors produce no contribution;
9. Debug and errors do not leak identities, paths, commands, or payloads;
10. no aggregate assessment, state, events, reports, artifacts, files, or CLI
    output are produced; and
11. existing attestation, proportional-governance, immutable-bundle, executor,
    provider, and workspace tests remain green.

Future aggregate tests must prove that partial, duplicate, mismatched,
unexpected, or reordered coverage cannot produce false satisfaction.

## 15. Proposed Implementation Sequence

1. Perform focused re-review of this correction.
2. Add the crate-private contribution wrapper beside the gate.
3. Add focused mapping, identity, no-import, and privacy tests.
4. Run full repository validation.
5. Perform phase-level maintainer review.
6. Plan the authoritative obligation-set and aggregation model.
7. Only after complete-coverage aggregation is accepted may a later phase plan
   proportional-governance reassessment or an executor checkpoint.

## 16. Open Questions

- What authoritative declaration or runtime binding should enumerate every
  expected evidence/check obligation for one immutable step?
- Should a later aggregate update an immutable-bundle assessment set or bind a
  distinct step-local reassessment record?
- What one-time claim semantics are required before contributions cross a
  process or persistence boundary?

## 17. Final Recommendation

The private requirement-scoped contribution wrapper is implemented. Perform
phase-level review next. Do not implement aggregate
reassessment, an executor checkpoint, automatic checks, persistence, events,
evidence records, reports, artifacts, schemas, CLI behavior, providers,
SideEffects, writes, hosted behavior, or release changes.
