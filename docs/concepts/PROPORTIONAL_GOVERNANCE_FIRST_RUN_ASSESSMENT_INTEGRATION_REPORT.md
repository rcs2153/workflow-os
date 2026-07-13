# Proportional Governance First-Run Assessment Integration Report

## 1. Executive Summary

Workflow OS now derives a review-only proportional-governance assessment for
each validated workflow step during `workflow-os first-run`. The full bounded
assessment appears in verbose human output and preview JSON. Default human
output remains concise.

The integration implements the corrected product boundary from external
dogfood feedback: execution disposition and disclosure obligation are separate
axes. A local UI may eventually display quiet decisions without changing their
governance. This phase does not implement that UI or any enforcement.

## 2. Scope Completed

- Reused the accepted deterministic workflow-step derivation helper.
- Reused the accepted proportional-governance workload assessor.
- Derived one assessment for every already-loaded, validated workflow step.
- Exposed execution, disclosure, completeness, unknown facts, algorithm
  identity, and input fingerprint in verbose and JSON first-run output.
- Labeled every result `review_only`, `assessed_not_enforced`, and
  `not_persisted`.
- Kept the default human first-run summary free of assessment detail.

## 3. Scope Explicitly Not Completed

This phase does not add runtime enforcement, automatic approval, persistence,
workflow or policy schema fields, provider mutation, a UI server, source-code
inspection, probabilistic inference, model authority, fabricated authority,
enterprise stewardship, or weaker explicit minima.

## 4. Input And Derivation Boundary

The first-run path passes the validated project bundle, workflow id, step id,
and active governance profile to the existing derivation helper. The helper
resolves the step, skill, and relevant policies and derives bounded static
facts from validated declarations.

Runtime-only authority and evidence/check results are not available during
first-run and remain explicit unknowns. The integration does not guess them.
Side-effect posture is derived only where the accepted helper can establish it
from declared capability vocabulary.

## 5. Execution And Disclosure Axes

Each projected assessment reports:

- execution: `proceed`, `require_approval`, or `denied`;
- disclosure: `quiet` or `visible`.

`visible` is not treated as a blocking execution mode. Presentation surfaces
may later render either quiet or required-visible records, but presentation
preference cannot weaken a disclosure obligation.

## 6. Invalidation Posture

Each assessment includes the accepted algorithm identifier and payload-free
input fingerprint. The fingerprint is bound to the relevant validated
definition root and decision inputs, providing the build-cache-style basis for
future reassessment when declarations change.

This phase does not cache, persist, or reuse assessments. It recomputes them
from the current validated project during each first-run invocation.

## 7. Privacy And Safety

The output contains stable identifiers, enum labels, bounded unknown-fact
categories, algorithm identity, and a SHA-256 fingerprint. It does not copy
workflow descriptions, policy text, source contents, provider payloads,
command output, credentials, environment values, or caller-supplied authority.

Incomplete safety-relevant facts are disclosed rather than inferred
optimistically. Explicit workflow, policy, profile, and future steward minima
remain authoritative.

## 8. Test Coverage

Focused CLI coverage proves:

- concise first-run output does not emit proportional-governance detail;
- verbose output emits one assessment for the scaffold workflow step;
- execution and disclosure are separate fields;
- incomplete authority and evidence/check facts are explicit;
- algorithm identity and fingerprint are present;
- JSON labels results review-only, assessed-not-enforced, and not persisted;
- first-run creates no runtime state.

## 9. Validation

Phase validation passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

The focused first-run CLI suite passed 18 tests. The full workspace suite
passed with only the repository's explicitly opt-in live integration tests
ignored by default. Strict clippy emitted no warnings, documentation validation
passed, formatting was clean, and `git diff --check` reported no whitespace
errors.

## 10. Governed Build Evidence

- Workflow: `dg/implement`
- Run ID: `run-1783939078394630000-2`
- Approval ID: `approval/run-1783939078394630000-2/implementation-approved`
- Approval presentation: `presentation/627b800fb61c0d15`
- Approval outcome: granted with persisted presentation proof
- Governed run status: completed

Codex performed repository edits and validation outside the kernel. The kernel
governed phase scope and approval; it did not execute commands or edit files.

## 11. Remaining Limitations

- Assessments are not runtime-enforced.
- Assessments are not persisted or attached to run records.
- Authority and executed evidence/check results remain unknown during
  first-run.
- No operator UI or notification surface exists.
- No workflow schema exposes proportional-governance overrides.
- No automatic approval or model self-approval is introduced.

## 12. Recommended Next Phase

Perform a focused maintainer review of the read-only first-run integration.
After acceptance, plan the smallest runtime reassessment boundary only after
immutable run inputs and authority facts can be supplied without guessing.
