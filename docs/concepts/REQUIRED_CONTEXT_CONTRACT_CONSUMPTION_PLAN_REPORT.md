# Required Context Contract Consumption Plan Report

## 1. Executive Summary

Workflow OS now has a phase-ready plan for consuming typed required-context
contracts against governed context projections without turning declarations
into authority or projections into payload access.

The plan recommends a model-only first implementation: exact typed
requirements, immutable contract binding, a pure deterministic consumer,
required-gap blocking, optional-gap disclosure, and rejection of undeclared
projected context.

## 2. Scope Completed

- Inspected the accepted governed context-access model and review.
- Inspected the existing Composable Harness Contract context vocabulary.
- Defined the declaration-versus-authority invariant.
- Defined exact target, access-level, execution-context, sensitivity, and
  immutable-contract matching.
- Defined required and optional gap behavior.
- Defined least-privilege rejection of undeclared projected context.
- Defined deterministic source retention and wire recomputation.
- Defined privacy, validation, tests, sequencing, and open questions.
- Connected the boundary to proportional governance and optional future
  sandbox execution.

## 3. Scope Explicitly Not Completed

- No Rust model or helper implementation.
- No context or source payload access.
- No runtime consumption or executor integration.
- No persistence, events, audit records, or authority receipts.
- No workflow or harness schema changes.
- No CLI, SDK, UI, or example changes.
- No OpenShell integration.
- No provider execution, SideEffect execution, or writes.
- No hosted administration, enterprise identity, or release changes.

## 4. Key Architecture Decisions

- Existing name-only `HarnessContextRequirement` values are not silently
  reinterpreted as enforceable typed requirements.
- Typed requirements use exact stable targets and exact access levels.
- Contract declaration never grants authority.
- Required gaps block and cannot be repaired by approval alone.
- Optional gaps remain explicit.
- Extra projected context fails closed as undeclared overexposure.
- A consumption result remains payload-free and is not a dereference lease.

## 5. User Feedback Reconciliation

Fresh-pull evaluation confirms that Workflow OS is a coherent, honest local
governance kernel and that quiet-success/proportional-governance work is the
right product direction. The evaluator's Node 24 integration-check sharpness
and duplicate missing-manifest diagnostic were already corrected in the
fresh-pull UX/tooling fix phase and do not change current sequencing.

This plan preserves that product direction: required context is enforced
deterministically, while quiet versus visible operator disclosure remains a
separate proportional-governance decision.

## 6. OpenShell Architecture Posture

OpenShell remains a promising optional execution provider, not a replacement
for Workflow OS governance and not a fork target. A future integration should
receive only freshly authorized context and return structured containment and
attestation references. No integration is authorized by this phase.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Dogfood Governance

- workflow: `dg/d`
- run ID: `run-1785129688753786000-2`
- approval ID:
  `approval/run-1785129688753786000-2/planning-approved`
- presentation ID: `presentation/41f25eaf4b7cc9f9`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase posture: planning/docs only
- event summary: 39 events, 1 approval, 0 retries, 0 escalations, with
  proof-enforced presentation
- out-of-kernel work: documentation edits, repository inspection, and
  validation commands were performed by the delegated maintainer; the kernel
  coordinated governance and did not edit files or run git operations

## 9. Remaining Limitations

- The first typed contract shape remains unimplemented.
- The ownership boundary between harness-specific and reusable step-level
  contracts remains an implementation-review question.
- Immutable-run binding, freshness, audited dereference, and sandbox
  attestation remain later phases.

## 10. Recommended Next Phase

Perform a focused maintainer review of this plan, then implement the
required-context contract consumption core model and pure helper only if the
plan is accepted.
