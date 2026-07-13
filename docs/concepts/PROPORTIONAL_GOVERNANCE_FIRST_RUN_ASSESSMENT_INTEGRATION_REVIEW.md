# Proportional Governance First-Run Assessment Integration Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implementation adds a bounded, read-only onboarding projection of the
accepted proportional-governance assessment model. It does not enforce,
persist, authorize, or execute the projected decision. The corrected execution
and disclosure axes remain separate and explicit.

## 2. Scope Verification

The phase stayed within its approved scope:

- `workflow-os first-run --verbose` exposes bounded assessment detail;
- preview JSON exposes the same machine-readable posture;
- default human output remains concise;
- existing constructors and deterministic helpers are reused;
- tests and documentation were added.

No runtime enforcement, automatic approval, persistence, schema change,
provider mutation, UI server, source inspection, model authority, fabricated
authority, SideEffect execution, or release-posture change was introduced.

## 3. Product-Model Assessment

The implementation reflects the corrected product interpretation:

- execution answers whether work may proceed, requires approval, or is denied;
- disclosure answers whether presentation may remain quiet or must be visible;
- visible disclosure does not itself block execution;
- a future local UI may display quiet records without changing governance;
- explicit workflow, policy, profile, and future steward minima remain
  authoritative.

This is a better boundary than treating visible disclosure as an execution
mode.

## 4. Derivation Assessment

The integration derives input from the already-loaded, validated project
bundle. For each workflow step, it resolves the declared skill and relevant
policy set through the accepted core helper.

It does not manufacture runtime facts. Authority and executed evidence/check
posture are passed as unavailable and therefore remain explicit unknown facts.
No natural-language classifier, source-code scan, provider call, command
execution, or new YAML field participates in the decision.

## 5. Invalidation Assessment

The projected algorithm identifier and input fingerprint expose a stable basis
for reassessment. The accepted core derivation includes the relevant workflow,
step, skill, and policy definition roots. Current first-run behavior recomputes
the assessment on every invocation, so changed validated definitions cannot
silently reuse a persisted result.

The fingerprint is not treated as an authority grant, approval, or replay
token.

## 6. Output And UX Assessment

The default human first-run output remains quiet and does not add assessment
density. Operators can opt into verbose output, while automation can consume
preview JSON.

Both surfaces label the result review-only, assessed-not-enforced, and not
persisted. Verbose and JSON output expose separate execution/disclosure fields,
completeness, unknown-fact categories, algorithm identity, and fingerprint.

No UI is implemented. The output shape is compatible with a future UI that
projects disclosure independently from execution.

## 7. Privacy And Error Assessment

The output is bounded to validated identifiers, typed labels, unknown-fact
categories, algorithm identity, and a canonical hash. It does not copy raw
workflow descriptions, policy bodies, repository source, command output,
provider payloads, environment values, credentials, or user-supplied authority.

The new path uses existing stable errors. Validated identifiers and hashes are
safe to expose under the repository's current preview contract.

## 8. Runtime-Semantics Assessment

No executor, run, approval, state, event, artifact, or provider path reads the
new projection. First-run still creates no runtime state. Assessment failure
cannot be converted into a workflow decision because no workflow execution is
started.

Existing workflow semantics therefore remain unchanged.

## 9. Test Quality Assessment

Focused CLI tests prove:

- default output remains quiet;
- verbose output includes the scaffold workflow assessment;
- execution and disclosure are separate;
- unknown authority and evidence/check facts remain explicit;
- completeness is conservative;
- algorithm and fingerprint are present;
- JSON labels the result assessed-not-enforced and not persisted;
- first-run creates no state.

The full workspace suite passed. Core tests already cover identical-input
determinism, all decision-relevant fingerprint invalidation families,
presentation non-authority, and explicit-minimum monotonicity.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add one CLI-level multi-workflow/multi-step ordering test when the first-run
  output contract is next expanded.
- Add one CLI-level changed-definition reassessment test when decisions are
  cached or persisted; recomputation currently makes stale reuse impossible.
- Keep runtime integration deferred until immutable run inputs, explicit
  authority, and executed evidence/check facts are available at the decision
  boundary.
- Treat any future local UI as a presentation projection, not governance
  authority.

## 12. Validation

Passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

The focused first-run CLI filter passed 18 tests. The full suite passed with
only explicitly opt-in live integration tests ignored by default.

## 13. Governed Review Evidence

- Workflow: `dg/review`
- Run ID: `run-1783941269535727000-2`
- Approval ID:
  `approval/run-1783941269535727000-2/review-scope-approved`
- Approval presentation: `presentation/94a8d84f1ec11fca`
- Approval outcome: granted with persisted presentation proof
- Review status: completed

Codex performed repository inspection and validation outside the kernel. The
kernel governed phase scope and approval; it did not execute checks or edit
files.

## 14. Recommended Next Phase

Plan the smallest runtime reassessment boundary that can consume immutable run
inputs plus explicit authority and executed evidence/check facts. Do not wire
the first-run recommendation directly into executor authority, do not persist
it as an approval, and do not broaden provider mutations first.
