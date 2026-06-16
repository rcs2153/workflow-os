# Self-Governance Dogfood Docs Cleanup Plan

Status: Implemented. The stale current-state wording in the dogfood multi-step conversion plan has been updated to describe the current five-step workflow, with the old single-step shape preserved only as historical context.

## 1. Executive Summary

The self-governance dogfood workflow has been converted to a five-step sequential governed workflow and reviewed.

The conversion review accepted the phase with no blockers, but identified one non-blocking documentation issue: the implemented conversion plan still contains historical "Current Dogfood State" wording that describes the old single-step workflow.

This plan defines a tiny documentation-only cleanup phase. It does not implement runtime behavior, change dogfood specs, add tests, introduce local check execution, add report artifacts, or broaden Workflow OS governance semantics.

## 2. Goals

- Align the dogfood conversion plan with the current implemented five-step workflow.
- Preserve historical context without describing the old single-step workflow as current behavior.
- Keep living docs, roadmap status, and dogfood guidance consistent.
- Preserve the kernel-governed, Codex/human-executed dogfood boundary.
- Keep the cleanup small, reviewable, and documentation-only.

## 3. Non-Goals

This plan does not authorize:

- runtime code changes;
- dogfood workflow spec changes;
- skill or policy changes;
- new CLI behavior;
- new tests unless needed to protect docs links;
- real local check execution;
- default local check handler registration;
- arbitrary shell execution;
- command-output evidence attachment;
- automatic runtime report generation;
- automatic report artifact writing;
- report CLI rendering;
- typed handoff runtime behavior;
- Composable Harness Contract runtime behavior;
- reasoning lineage or claim graph implementation;
- side-effect boundary implementation;
- write behavior;
- workflow schema changes;
- examples outside documentation wording;
- hosted or distributed runtime behavior;
- release posture changes.

## 4. Cleanup Target

Primary target:

- `docs/implementation-plans/self-governance-dogfood-multi-step-conversion-plan.md`

The section currently titled "Current Dogfood State" should be updated so it no longer describes the old one-step `d` workflow as present tense current behavior.

Recommended approach:

- rename or rewrite the section to describe the current implemented five-step state;
- optionally retain the old single-step description as clearly labeled prior state if useful;
- ensure limitations describe the current placeholder/check/report boundaries rather than saying multi-step execution is not yet exercised.

## 5. Related Documentation To Check

Review and adjust only if needed:

- `README.md`
- `ROADMAP.md`
- `dogfood/workflow-os-self-governance/README.md`
- `dogfood/workflow-os-self-governance/tests/README.md`
- `docs/implementation-plans/self-governed-validation-check-plan.md`
- `docs/concepts/SELF_GOVERNANCE_DOGFOOD_MULTI_STEP_CONVERSION_REPORT.md`
- `docs/concepts/SELF_GOVERNANCE_DOGFOOD_MULTI_STEP_CONVERSION_REVIEW.md`

The cleanup should not rewrite accepted reports or reviews except to add a small forward note if a dangerous false claim is found. Accepted reports and reviews should remain historical records.

## 6. Required Documentation Posture

Docs must continue to state:

- the dogfood workflow is now sequential and multi-step;
- the workflow remains kernel-governed and Codex/human-executed;
- approval is scoped to the planning checkpoint;
- placeholder local skill behavior remains deterministic and bounded;
- real validation/check commands are not executed by default from the dogfood workflow;
- report generation and report artifacts are not automatic;
- typed handoffs are not produced by the dogfood workflow;
- command-output evidence is not attached;
- reasoning lineage is not implemented;
- side-effect boundary and writes remain unsupported.

Docs must not claim:

- production self-hosting;
- automatic Codex control by the kernel;
- automatic implementation execution;
- recursive agents, agent swarms, or nested harness runtime behavior;
- automatic local build/test execution;
- Level 3 or Level 4 autonomy.

## 7. Privacy And Redaction

This phase should not add raw payloads, paths beyond existing repo-relative documentation paths, command output, parser output, provider payloads, environment values, credentials, tokens, private keys, or secret-like examples.

Any new wording should remain bounded and non-secret.

## 8. Validation

Run:

- `npm run check:docs`

Run broader checks only if documentation changes unexpectedly touch code, generated files, or validation tooling.

## 9. Acceptance Criteria

- The implemented conversion plan no longer describes the old single-step dogfood workflow as the current state.
- Current dogfood docs consistently describe the five-step sequential workflow.
- Historical reports and reviews remain intact.
- No runtime code, specs, tests, CLI behavior, schemas, examples, report artifacts, reasoning lineage, side effects, writes, or release posture changes are introduced.
- Documentation checks pass.

## 10. Recommended Next Phase

Recommended next phase: self-governance dogfood docs cleanup.

After cleanup, choose the next substantive roadmap phase deliberately. Current candidates are:

- dogfood review hardening tests;
- local check handler default-registration planning;
- typed handoff integration planning;
- report-bearing dogfood execution planning.

Real command execution, default handler registration, command-output evidence attachment, side-effect boundary implementation, and writes should remain deferred until separately scoped and reviewed.
