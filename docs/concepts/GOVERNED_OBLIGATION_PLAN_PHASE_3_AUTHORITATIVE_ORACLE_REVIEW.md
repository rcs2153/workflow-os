# Governed Obligation Plan Phase 3 Authoritative Oracle Review

## Verdict

Accepted for the private Phase 3 evidence boundary. No blocking correctness or
security finding remains in the authoritative-oracle correction.

This verdict accepts deterministic explanation and differential validation. It
does not accept runtime composition, public contracts, persistence, provider
behavior, or a new source of execution authority.

## What Changed Since The Requested-Changes Review

The correction removes the module-local canonical-outcome vocabulary that
previously allowed the test to author both sides of the comparison. The
observed side now carries exact existing Core result types:

- `PolicyDecision` returned by `ConservativePolicyEngine::evaluate`;
- `ProportionalGovernanceWorkloadAssessment` returned by
  `assess_proportional_governance_workload`; and
- `WorkflowOsError` returned when the accepted proportional-governance selector
  rejects an unsupported explicit requirement.

The fixtures invoke those existing Core paths. They do not manually assign an
outcome label such as `ApprovalRequired` or `LocalCheckRequired`.

## Review Findings

No blocking findings.

The private matcher recognizes only three exact pairings:

1. a mandatory approval obligation with approval-decision evidence and a
   policy decision that is allowed but requires approval;
2. a mandatory local-check obligation with local-check evidence and an
   assessment that denies execution for the existing
   `EvidenceOrCheckRequirement` reason; and
3. an unsupported local-check obligation with the existing Unsupported error
   and stable `governance.proportional.requirement.unsupported` code.

Every other kind, posture, evidence class, decision type, decision reason, or
error fails closed. This is a bounded reconciliation assertion, not a second
policy selector: it cannot allow work, satisfy evidence, grant approval, mark
completion, or alter runtime state.

The omitted-gate fixture is now credible for Phase 3. It independently proves
that Core still requires approval and still denies unavailable required
evidence/checks, while the complete expected inventory detects removal of the
local-check obligation. Duplicate expectations, duplicate decisions,
unexpected decisions, and mismatched decision types have direct negative
coverage.

The explanation remains bound to the accepted source-closure and inventory
commitments. Runtime `SideEffect` observations remain facts outside the static
obligation inventory. Debug output remains redaction-safe. The entire slice is
crate-private, non-serializing, non-persistent, non-authorizing, and unused by
executor control flow.

## Residual Boundaries

The accepted evidence is intentionally narrower than Phase 4.

- The observation wrapper itself does not prove provenance. `PolicyDecision`
  is a public data type and an obligation ID is associated by the private test
  fixture. Any runtime composition must obtain decisions from authoritative
  Core execution for the exact immutable run bundle; it must not accept an
  arbitrary caller-supplied decision plus obligation ID as proof.
- The local-check fixture exercises the canonical proportional-governance
  denial path. It does not execute a local-check handler or prove check-result
  evidence. Phase 4 must preserve the existing check/evidence verifier rather
  than treating this assessment as check satisfaction.
- There is still no canonical source extractor or reviewed runtime cardinality
  limit. Runtime composition must add a bounded extractor from existing
  canonical run inputs and fail closed before consumption when the complete
  source closure cannot be established.
- Phase 3 does not establish independent third-party assurance. This is a
  focused owner-conducted maintainer/security review.

These limits are not Phase 3 defects because this slice is differential review
evidence only. They become hard acceptance conditions for any runtime use.

## Validation

- Reviewed merged commit `2b6f0bcd72e7a9f60d7fa1c4a6257b922dc5d272`.
- Reviewed the exact three-file correction and its relationship to the
  accepted reconciliation plan and requested-changes review.
- `cargo test -p workflow-core --lib governed_obligation_plan`: 21 passed;
  zero failed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- PR #767 exact-head CI: nine of nine checks passed on
  `7fbffe6ca46b3163fe58c0fc054ebcac59230e09` before squash merge.

## Governed Review

- workflow: `dg/review`
- run: `run-1793965653120018000-2`
- approval: `approval/run-1793965653120018000-2/review-scope-approved`
- presentation: `presentation/67899c09b4c0825e`
- approval outcome: granted with persisted presentation proof

Workflow OS governed the review scope and approval. Source inspection, test
commands, documentation edits, Git, PR, and merge work occur outside the
kernel and remain disclosed. No runtime execution, provider action, release,
or publication behavior is simulated.

## Next Authorized Increment

Phase 3 is accepted. The next authorized increment is one separately governed
private Phase 4 plan for the smallest opt-in runtime composition.

That plan must bind the derivation algorithm and complete canonical source
closure to an immutable run bundle, define bounded cardinality, obtain
observations from existing authoritative runtime paths, preserve all current
policy, approval, evidence/check, `SideEffect`, event, report, and completion
checks, and fail closed before consumption on any mismatch. It must not expose
a public schema, SDK, CLI, persistence format, provider behavior, incremental
dependency, Salsa integration, release behavior, or publication path.
