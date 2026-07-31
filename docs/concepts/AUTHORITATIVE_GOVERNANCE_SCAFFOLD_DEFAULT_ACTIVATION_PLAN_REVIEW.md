# Authoritative Governance Scaffold Default Activation Plan Review

## 1. Executive Verdict

Needs prerequisite runtime-composition fixes; do not activate by default.

The product proposal remains sound, but the initial review overstated runtime
readiness. The exercised scaffold is single-step, while the project-level
runtime path can accept workflows for which the CLI supplies unobserved
evidence/check facts and predicts visible disclosure before the authoritative
check result exists.

## 2. Scope Assessment

The plan is appropriately narrow:

- it changes only newly generated `init-repo-governance` scaffolds;
- existing projects are not migrated or rewritten;
- the scaffold command remains non-executing;
- the exact accepted project-validation contract is reused;
- the existing positive flag remains compatible;
- an explicit negative flag preserves the legacy scaffold; and
- contradictory flags fail before writes.

No OpenShell, provider, SideEffect, write, hosted, schema, SDK, example, or
release work is authorized.

## 3. Evidence Assessment

The prior opt-in path now has enough evidence for broader onboarding treatment:

- its incomplete scaffold-to-runtime contract blocker was fixed;
- repeated disposable external-repository execution passed;
- current authority is derived inside Core from validated immutable project
  activation rather than a runtime flag;
- the fixed local check is source-read-only and network-disabled;
- quiet, visible, approval-required, and denied routes are implemented;
- approval resume rechecks current context;
- terminal WorkReport artifact persistence is implemented;
- quiet-success output is accepted; and
- fresh-pull evaluation identifies low-risk ceremony, not contract clarity, as
  the next product problem.

This evidence supports preserving the opt-in and preparing a future
new-scaffold default. It does not yet support making that path ordinary because
the runtime boundary remains partly caller-classified outside the exact
single-step scaffold exercised by the evaluation.

The correction does not invalidate the external-repository proof. It narrows
what that proof establishes: the exact generated one-step opt-in path works.
It does not establish safe project-wide activation for other workflow shapes.

## 4. Governance Assessment

Default activation does not mean default approval or silent execution.
Proportional governance continues to select execution and disclosure posture
from authoritative inputs. Authored workflow approvals remain distinct.

The negative option is important for compatibility, but it must be explicit.
The ordinary path should carry Workflow OS's evidence-preserving governance
opinion rather than requiring users to discover an expert flag.

## 5. Compatibility Assessment

Keeping `--authoritative-governance` avoids needless script breakage. Adding
`--no-authoritative-governance` provides an explicit legacy escape hatch.
Rejecting both flags before writes prevents ambiguous intent.

Because the repository is still in preview, changing newly generated scaffold
defaults is acceptable when it is documented and regression tested. Existing
manifests and runs remain untouched.

## 6. Privacy And Failure Assessment

The plan adds no caller-controlled payload surface. The fixed declaration and
requirement contain bounded identifiers and posture values only.

The runtime remains fail closed on declaration drift, immutable-run mismatch,
missing or changed check requirements, failed checks, missing authority,
approval mismatch, and artifact integrity failure.

## 7. Test Assessment

The proposed matrix covers default, affirmative, negative, contradictory,
dry-run, preservation, runtime, approval separation, quiet success, artifact
persistence, and compatibility behavior. The disposable-repository proof is
required because this is an onboarding-default change.

## 8. Blockers

1. The CLI marks non-selected steps' evidence/check posture as `Satisfied`
   without same-call evidence.
2. The CLI predicts visible disclosure from optimistic facts instead of
   allowing Core to conditionally consume visible-delivery capability after
   the actual assessment.
3. The closed project-validation route is not explicitly constrained to the
   one-step workflow shape that has been exercised and reviewed.

## 9. Non-Blocking Follow-Ups

- Measure explicit opt-out use before considering removal of the positive
  compatibility flag.
- Continue expanding safe repo-specific recommendations separately; do not
  infer executable commands in this phase.
- Treat OpenShell as a later optional execution provider after the missing
  provider-neutral sandbox contracts are implemented and reviewed.
- Add governance-friction metrics only after a bounded telemetry contract
  exists.

## 10. Recommended Next Phase

Implement the
[Core-Owned Authoritative Runtime-Fact Derivation Plan](../implementation-plans/core-owned-authoritative-runtime-fact-derivation-plan.md),
then perform a focused review. Return to default scaffold activation only after
that review confirms there are no caller-classified runtime facts or
caller-predicted routes in the closed path.

## 11. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785465361469797000-2`
- approval: `approval/run-1785465361469797000-2/planning-approved`
- presentation: `presentation/6cc40e398a0406c8`
- approval outcome: granted under delegated-maintainer authority
- approval proof: persisted before decision
- planning boundary: documentation and implementation design only

## 12. Fix-Forward Record

The readiness correction was governed separately:

- workflow: `dg/d`
- run: `run-1785468401630252000-2`
- approval: `approval/run-1785468401630252000-2/planning-approved`
- presentation: `presentation/caeca7fe4c85adb9`
- approval outcome: granted under delegated-maintainer authority
- correction: default activation deferred behind Core-owned fact derivation
