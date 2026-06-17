# Hook Event Audit Projection Plan Report

## 1. Executive Summary

Hook event audit projection planning is complete. The accepted model-only hook workflow event vocabulary now has a conservative follow-on plan that decides how future hook workflow events should relate to audit before any executor path appends hook events.

The plan recommends a projection-only implementation next: update generic `AuditEvent::from_workflow_event(...)` behavior for already-modeled hook workflow event vocabulary, with focused tests and no executor integration.

Fix-forward note: the projection-only implementation has since been completed. Generic `AuditEvent::from_workflow_event(...)` now projects modeled hook workflow events as bounded audit events. Executor event append behavior, dedicated hook audit sink emission, hook persistence, observability metrics, CLI behavior, schemas, side effects, writes, recursive agents, agent swarms, and release posture changes remain unimplemented.

Follow-on note: the projection-only implementation has since been reviewed, and executor hook event append planning is documented in [Executor Hook Event Append Plan](../implementation-plans/executor-hook-event-append-plan.md). Executor hook event append behavior remains unimplemented.

## 2. Scope Completed

- Created [Hook Event Audit Projection Plan](../implementation-plans/hook-event-audit-projection-plan.md).
- Documented current audit, hook, executor, and report boundaries.
- Compared generic audit projection, dedicated hook audit sink methods, hook audit store, and report-only citation.
- Recommended generic hook workflow event audit projection as the next implementation target.
- Defined hook audit projection rules.
- Defined action/capability posture.
- Defined relationship to `AgentHarnessHookAuditRecord`.
- Defined observability non-goals.
- Defined executor integration boundary.
- Defined privacy and redaction requirements.
- Defined future test plan.
- Updated roadmap and related planning/concept docs.

## 3. Scope Explicitly Not Completed

- No audit projection code changes.
- No audit sink emission for hook records.
- No dedicated hook audit sink method.
- No hook audit store.
- No hook persistence.
- No executor hook broadening.
- No automatic executor hook invocation.
- No workflow event append behavior in `LocalExecutor`.
- No additional executor checkpoints.
- No workflow schema fields.
- No runtime hook configuration.
- No CLI behavior.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No `EvidenceReference` creation or attachment.
- No approval attachment.
- No report artifact writes.
- No reasoning lineage.
- No side-effect boundary implementation.
- No writes.
- No recursive agents or agent swarms.
- No hosted/distributed runtime claims.
- No release posture changes.

## 4. Recommendation Summary

The plan recommends **hook workflow event audit projection implementation, projection-only**.

The next implementation should update generic audit projection for:

- `HookInvocationRequested`;
- `HookInvocationEvaluated`.

It should not append hook workflow events from executor paths or emit dedicated hook audit records.

## 5. Boundary Summary

The plan preserves the current boundaries:

- `WorkflowRunEvent` remains the runtime source of truth.
- `AuditEvent` remains a projection of accepted workflow events.
- `AgentHarnessHookAuditRecord` remains model-only vocabulary until sink/store semantics are accepted.
- `WorkReport` continues to cite supplied hook invocation IDs.
- The explicit `BeforeReport` checkpoint remains report-path-only, in-memory-only, and non-mutating.

## 6. Privacy Summary

Hook audit projection must remain reference-first and redaction-safe. It must not copy raw prompts, raw command output, command transcripts, raw local check output, provider payloads, CI logs, Jira/GitHub raw bodies, parser payloads, raw spec contents, environment values, credentials, authorization headers, private keys, token-like values, or unbounded summaries.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Known Limitations

- Hook audit projection is implemented only as generic projection from already-modeled hook workflow events.
- Executor hook event append behavior remains unimplemented.
- Dedicated hook audit sink behavior remains unimplemented.
- Hook audit persistence remains unimplemented.
- Hook observability metrics remain unimplemented.
- Hook event WorkReport citation targets remain unimplemented.
- Pre-terminal executor hook checkpoint integration remains unimplemented.

## 9. Recommended Next Phase

Recommended next phase: **executor hook event append planning**.

That planning phase is now documented in [Executor Hook Event Append Plan](../implementation-plans/executor-hook-event-append-plan.md). It must not add executor event append behavior, dedicated hook audit sink emission, hook persistence, observability metrics, CLI behavior, schemas, automatic local checks, command execution, adapter invocation, side effects, writes, or release posture changes.
