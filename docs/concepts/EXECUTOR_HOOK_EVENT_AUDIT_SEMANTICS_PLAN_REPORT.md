# Executor Hook Event And Audit Semantics Plan Report

Report date: 2026-06-17

## 1. Executive Summary

Created the follow-on planning document for executor hook workflow event and audit semantics after the accepted explicit `BeforeReport` executor hook integration.

The plan keeps the current `BeforeReport` checkpoint report-path-only and non-mutating. It defines the durable semantics questions that must be resolved before any future state-visible or pre-side-effect hook checkpoint is implemented, and recommends a small next code-bearing phase: hook workflow event vocabulary model, model-only.

## 2. Scope Completed

- Added [Executor Hook Event And Audit Semantics Plan](../implementation-plans/executor-hook-event-audit-semantics-plan.md).
- Documented current runtime and hook baselines.
- Defined source-of-truth boundaries for workflow events, snapshots, audit projections, hook invocation results, hook audit records, WorkReports, and EvidenceReference.
- Decided that the current post-terminal `BeforeReport` checkpoint should not become a workflow event yet.
- Defined candidate future hook workflow event names.
- Defined event ordering considerations for future pre-terminal checkpoints.
- Defined state transition posture for future hook events.
- Defined failure semantics posture for passed, warning, skipped, failed-closed, and blocked hooks.
- Defined audit sink emission sequencing.
- Defined idempotency and replay policy requirements.
- Defined relationships to policy, approvals, local checks, adapters, EvidenceReference, and WorkReports.
- Updated roadmap and related hook/report planning docs with links to the new plan.

## 3. Scope Explicitly Not Completed

- No runtime hook broadening.
- No automatic executor hook invocation.
- No additional executor checkpoints.
- No workflow event model implementation.
- No workflow event append behavior.
- No audit sink emission for hook records.
- No hook persistence.
- No hook audit store.
- No report artifact writes.
- No workflow schema fields.
- No workflow-declared hook configuration.
- No runtime hook config.
- No CLI hook commands.
- No automatic local check execution.
- No default local check handler registration.
- No command execution.
- No adapter invocation.
- No external provider calls.
- No `EvidenceReference` creation or attachment.
- No approval request or approval decision creation.
- No approval evidence attachment.
- No reasoning lineage.
- No side-effect boundary implementation.
- No write-capable adapters.
- No recursive agents.
- No agent swarms.
- No hosted or distributed runtime claims.
- No release posture change.

## 4. Planning Summary

The plan separates two futures:

- the current `BeforeReport` hook remains post-terminal, in-memory, and report-path-only;
- future pre-terminal hook checkpoints may need durable workflow event vocabulary before they can safely influence runtime behavior.

The plan rejects post-terminal metadata-only events for the current `BeforeReport` checkpoint because terminal states currently reject further mutating events and because report generation already has a separate error channel.

## 5. Event Semantics Summary

Candidate future hook event names are documented as:

- `HookInvocationRequested`;
- `HookInvocationEvaluated`;
- `HookInvocationFailedClosed`;
- `HookInvocationSkipped`;
- `HookInvocationBlocked`.

The plan recommends that the first implementation keep any accepted event vocabulary model-only and non-executor-integrated. It should not append events or change `LocalExecutor` behavior.

## 6. Audit Semantics Summary

The plan recommends no automatic audit sink emission until event/source semantics are accepted.

Future audit options are:

1. project hook workflow events through `AuditEvent::from_workflow_event(...)`;
2. add a dedicated hook audit sink method for `AgentHarnessHookAuditRecord`;
3. persist hook audit records in a hook-specific store and cite them from reports.

The plan preserves the boundary: no event source, no audit projection.

## 7. Idempotency And Replay Summary

The plan requires future state-visible hook checkpoints to define deterministic hook invocation identity, idempotency keys, duplicate run behavior, replay behavior, and whether hook execution is re-run or recovered from durable history.

The current explicit `BeforeReport` path may re-run in-memory validation on duplicate report-bearing calls only because it writes no events or records.

## 8. Privacy And Redaction Summary

Future hook event and audit semantics must remain reference-first and redaction-safe.

Forbidden payloads include raw prompts, raw spec contents, raw command output, raw command transcripts, provider payloads, CI logs, Jira or GitHub raw bodies, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, and unbounded summaries.

## 9. Test Coverage Plan Summary

Future model-only tests should cover hook event kind representation, valid minimal hook event payloads, invalid ID rejection, secret-like reference rejection, redaction metadata validation, redaction-safe debug output, serialization non-leakage, invalid serialized payload fail-closed behavior, terminal-state rejection for hook events, and proof that event vocabulary does not execute hooks, emit audit sink records, persist records, run local checks, execute commands, invoke adapters, or change existing tests.

## 10. Commands Run And Results

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 11. Remaining Known Limitations

- Hook workflow event vocabulary is not implemented.
- Hook workflow events are not appended by any executor path.
- Hook audit sink emission is not implemented.
- Hook audit records are not persisted.
- `BeforeReport` remains report-path-only and non-mutating.
- Pre-skill and other state-visible hook checkpoints remain deferred.
- Workflow schema and CLI hook surfaces remain deferred.

## 12. Recommended Next Phase

Recommended next phase: **executor hook event/audit semantics plan review**.

After review, the likely code-bearing implementation is **hook workflow event vocabulary model, model-only**. That phase must not add executor hook broadening, event append behavior, audit sink emission, persistence, CLI behavior, workflow schema fields, automatic local checks, command execution, adapter invocation, side-effect modeling, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.
