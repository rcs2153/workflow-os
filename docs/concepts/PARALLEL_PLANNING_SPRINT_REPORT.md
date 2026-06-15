# Parallel Planning Sprint Report

## 1. Executive Summary

The parallel planning sprint split the next roadmap discovery into four independent tracks:

- typed handoffs;
- report/audit/missing-citation semantics;
- DocsCheck default-registration posture;
- side-effect boundary pre-ADR discovery.

The sprint produced three new planning documents and confirmed that the existing DocsCheck default-registration plan remains directionally correct, with the next implementation slice narrowed to an explicit opt-in constructor rather than ambient default registration.

## 2. Scope Completed

Completed:

- typed handoff planning;
- report/audit/missing-citation semantics planning;
- side-effect boundary ADR planning;
- DocsCheck default-registration discovery;
- roadmap cleanup for the completed Composable Harness Contract review;
- next-phase recommendation.

## 3. Scope Explicitly Not Completed

This sprint did not implement:

- typed handoff model;
- report/audit semantics tests;
- DocsCheck constructor changes;
- side-effect boundary model;
- nested harness execution;
- runtime scheduling;
- automatic report generation;
- automatic artifact writing;
- CLI behavior;
- workflow schema changes;
- reasoning lineage;
- write-capable adapters;
- domain packs;
- release posture changes.

## 4. Planning Outputs

New planning documents:

- [Typed Handoff Plan](../implementation-plans/typed-handoff-plan.md);
- [Report, Audit, And Missing-Citation Semantics Plan](../implementation-plans/report-audit-missing-citation-semantics-plan.md);
- [Side-Effect Boundary ADR Plan](../implementation-plans/side-effect-boundary-adr-plan.md).

Existing plan confirmed:

- [DocsCheck Default-Registration Plan](../implementation-plans/docs-check-default-registration-plan.md).

## 5. Track Summaries

Typed handoffs:

- Recommended next implementation: model-only typed handoff core model.
- Keep handoffs reference-first and bounded.
- Do not implement nested execution or runtime scheduling.

Report/audit/missing-citation semantics:

- Reports remain derived governed handoff artifacts, not audit events.
- Report-generation errors remain separate from workflow execution errors.
- Absent optional references remain section text until required citation slots exist.

DocsCheck default registration:

- Do not make `LocalSkillRegistry::new()` register DocsCheck.
- Recommended next implementation: explicit config-driven opt-in constructor/helper.
- Keep CLI exposure, arbitrary shell execution, and automatic check execution out of scope.

Side-effect boundary:

- Requires a narrow ADR before any write implementation.
- Recommended direction separates authority from lifecycle state.
- No writes, provider mutations, schemas, CLI, or generic adapter execution are authorized.

## 6. Integrated Recommendation

Recommended next implementation phase:

1. Typed handoff core model only.

Recommended parallel follow-up planning:

1. Side-effect Boundary Core Model ADR.
2. Report/audit/missing-citation semantics hardening.
3. DocsCheck explicit constructor implementation prompt.

The typed handoff model is the best next code slice because it builds directly on the just-reviewed Composable Harness Contract model and does not require runtime behavior, schemas, writes, CLI, or persistence.

## 7. Commands Run

- `npm run check:docs`

## 8. Remaining Known Limitations

- Typed handoffs are planned but not implemented.
- Side-effect boundary remains planning-only.
- DocsCheck remains explicit and non-default.
- Missing citation semantics are documented as a plan but not hardened by tests yet.
- No nested harness execution exists.

## 9. Recommended Next Phase

Typed handoff core model implementation.

The implementation should be model-only, validated, serde-compatible, redaction-safe, and explicitly non-runtime.
