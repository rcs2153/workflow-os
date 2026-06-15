# DocsCheck Local Handler Plan Report

Report date: 2026-06-15

## 1. Executive Summary

The first non-dogfood local check handler phase is planned.

The plan selects `DocsCheck` as the next candidate, backed by the canonical `npm run check:docs` template. It keeps implementation deferred until review, because `DocsCheck` invokes Node/npm and needs explicit decisions around environment, cache/write posture, output capture, registration, runtime boundaries, and tests.

## 2. Governance Run

This planning phase was governed by the self-governance dogfood workflow before documentation changes.

- State directory: `/tmp/workflow-os-docs-check-handler-plan`
- Run ID: `run-1781502840484281000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781502840484281000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Completed

- Created [DocsCheck Local Handler Plan](../implementation-plans/docs-check-local-handler-plan.md).
- Positioned `DocsCheck` as the first non-dogfood local check candidate.
- Defined command authority rules.
- Defined Node/npm environment and cache posture questions.
- Defined bounded output and redaction policy.
- Defined runtime/event/report/evidence boundaries.
- Defined handler registration posture.
- Defined implementation test plan.
- Updated roadmap and related planning docs.

## 4. Scope Explicitly Not Completed

- No `DocsCheck` handler implementation.
- No production local check handler registration.
- No default handler registration.
- No CLI exposure.
- No workflow schema fields.
- No automatic check execution.
- No report artifact writing.
- No evidence attachment.
- No side-effect boundary implementation.
- No source writes.
- No broader command handler families.
- No release posture change.

## 5. Plan Summary

The plan recommends a future explicit `DocsCheck` handler that:

- accepts only `LocalCheckCommandKind::DocsCheck`;
- executes only the canonical `npm run check:docs` template;
- uses executable plus argument vector, not a shell;
- starts from sanitized environment values;
- treats npm cache/write behavior as explicit policy, not ambient behavior;
- derives output from `LocalCheckResult`;
- remains non-default and explicitly registered for tests or reviewed internal use only.

## 6. Risk Summary

Primary risks identified:

- ambient npm credentials or registry configuration leaking into execution;
- cache writes being mistaken for source-write support;
- docs check output copying raw docs or parser payloads into stable model surfaces;
- default registration creating premature production handler expectations;
- CLI exposure creating a public contract before the handler is reviewed.

## 7. Recommended Next Phase

Recommended next phase: **DocsCheck local handler plan review**.

The review should decide whether the plan is ready to generate an implementation prompt for a non-default, explicit, in-memory/test-scoped `DocsCheck` handler.

## 8. Validation

Validation commands for this planning phase:

- `npm run check:docs`
  - Passed.
