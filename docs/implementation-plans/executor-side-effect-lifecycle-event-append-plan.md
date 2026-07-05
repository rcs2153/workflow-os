# Executor SideEffect Lifecycle Event Append Plan

Status: Planning only. This plan follows the accepted store-backed SideEffect lifecycle transition helper and blocker fix review. It does not implement executor append behavior, provider writes, runtime side-effect execution, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS can now persist validated store-backed `SideEffectRecord` transitions for `Proposed -> Attempted`, `Attempted -> Completed`, and `Attempted -> Failed`.

The executor already supports explicit local append behavior for `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` workflow events. It intentionally rejects `SideEffectAttempted`, `SideEffectCompleted`, and `SideEffectFailed` inputs.

The next question is how an executor may append attempted/completed/failed lifecycle events once the corresponding store-backed transition has already succeeded. This plan defines that boundary. It does not authorize provider mutation or runtime side-effect execution.

## 2. Goals

- Append attempted/completed/failed SideEffect workflow events only after validated store-backed lifecycle transition.
- Preserve `WorkflowRunEvent` ordering and replay determinism.
- Preserve `SideEffectRecordStore` as the durable lifecycle record boundary.
- Preserve current workflow pass/fail semantics.
- Avoid provider calls and write execution.
- Avoid raw provider payloads, command output, parser payloads, and secrets.
- Require explicit caller input and stable identifiers.
- Reuse existing `SideEffectWorkflowEvent` payloads returned by store-backed transition helpers.
- Keep the first implementation local, opt-in, and reviewable.
- Prepare for future provider write orchestration without implementing it.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- provider calls;
- live GitHub pull request comments;
- Jira writes;
- CI reruns or workflow dispatch;
- runtime side-effect execution;
- write-capable adapter execution;
- automatic side-effect attempts or completions;
- automatic report artifact writes;
- CLI commands, rendering, or export;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- credential loading or secret-provider integration;
- rollback or compensation behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented:

- SideEffect workflow event vocabulary for proposed, denied, skipped, attempted, completed, and failed;
- local executor append path for explicit proposed/denied/skipped SideEffect events;
- generic audit projection for SideEffect workflow events;
- `SideEffectRecordStore` persistence and discovery;
- pure lifecycle transition helpers;
- store-backed lifecycle transition helpers;
- store update lifecycle guard allowing only `Proposed -> Attempted`, `Attempted -> Completed`, and `Attempted -> Failed`;
- WorkReport/report artifact SideEffect citation and referential integrity helpers.

Not implemented:

- executor attempted/completed/failed append path;
- provider calls;
- runtime side-effect execution;
- automatic SideEffect record transition from executor paths;
- automatic report artifact writes from default executor paths.

## 5. Recommended First Implementation Target

Recommended target: **explicit local executor append of attempted/completed/failed events from already-transitioned store-backed results**.

The future implementation should not ask the executor to construct the lifecycle transition itself from raw provider context. Instead, it should accept an explicit input produced by the store-backed transition helper, or a narrow equivalent that requires:

- stable `SideEffectId`;
- expected run/workflow identity;
- target step and skill identity;
- lifecycle state;
- validated reference-only `SideEffectWorkflowEvent` payload;
- proof that the store transition already succeeded in the same explicit operation boundary.

This keeps store mutation and workflow event append coupled by explicit API without making the executor a provider-write orchestrator.

## 6. Candidate API Shape

The smallest future shape should extend the existing explicit side-effect input model rather than inventing runtime config.

Possible options:

1. Extend `LocalExecutionSideEffectEventInput` to allow attempted/completed/failed only when the input carries store-backed transition proof.
2. Add a separate `LocalExecutionSideEffectLifecycleEventInput` for attempted/completed/failed.
3. Add an executor-adjacent helper that accepts a store-backed transition result and returns a validated `LocalExecutionSideEffectEventInput`.

Recommendation: use a separate helper or input type first. The existing `LocalExecutionSideEffectEventInput` is already accepted by the executor and currently rejects attempted/completed/failed. Adding a separate explicit path makes the new trust boundary visible and easier to test.

## 7. Store And Event Ordering Policy

The future implementation should preserve this ordering:

1. Load prior `SideEffectRecord` from `SideEffectRecordStore`.
2. Validate and persist the lifecycle transition through the store-backed helper.
3. Append the returned reference-only workflow event payload to the active run.
4. Allow existing generic audit projection to derive bounded audit context.

Do not append attempted/completed/failed events before the store transition succeeds.

If store transition succeeds but event append fails, the implementation must return a structured error and disclose that the record/event pair is out of sync. A later reconciliation phase should decide whether and how to repair that state. The first implementation should prefer fail-closed reporting over silent recovery.

## 8. Runtime Placement

The first implementation should attach to an explicit executor-adjacent path, not the default skill invocation loop.

Preferred placement:

- helper or method that accepts an active local run context plus explicit lifecycle transition input;
- local only;
- no hidden global state;
- no provider credentials;
- no runtime config;
- no automatic invocation from ordinary `execute(...)` unless separately reviewed.

Avoid first:

- default `LocalExecutor::execute(...)` automatic attempted/completed/failed append;
- automatic provider result classification;
- CLI mutation command;
- workflow-declared configuration.

## 9. Identity Validation

Before appending an attempted/completed/failed event, validate:

- run ID matches active run;
- workflow ID matches active run;
- workflow version matches active run;
- schema version matches active run;
- spec hash matches active run;
- step ID matches the targeted step;
- skill ID and version match the targeted skill;
- event lifecycle state matches the intended append kind;
- side-effect ID matches the transitioned record;
- idempotency key is present and bounded;
- event payload is validated by existing constructors.

Identity mismatch must fail closed with stable, non-leaking errors.

## 10. Lifecycle Semantics

Allowed in the future implementation:

- `SideEffectAttempted`: only after `Proposed -> Attempted` store transition succeeds.
- `SideEffectCompleted`: only after `Attempted -> Completed` store transition succeeds.
- `SideEffectFailed`: only after `Attempted -> Failed` store transition succeeds.

Still unsupported:

- direct `Proposed -> Completed` append;
- same-state replacement or replay-as-success without explicit replay semantics;
- completed/failed append without an attempted prior record;
- denied/skipped conversion to attempted/completed/failed;
- lifecycle mutation after terminal workflow state.

## 11. Idempotency And Replay

The future implementation must define deterministic idempotency keys for attempted/completed/failed append operations.

Recommended key ingredients:

- run ID;
- step ID;
- skill ID/version;
- side-effect ID;
- lifecycle state;
- append checkpoint name.

Duplicate executor calls must not append duplicate lifecycle events. Because store same-state replacement is currently rejected, replay-as-success requires separate design before same-state updates are allowed.

## 12. Failure Behavior

Conservative failure policy:

- Missing prior store record fails before append.
- Store read failure fails before append.
- Store transition failure fails before append.
- Event construction failure fails before append.
- Event append failure returns structured error and does not fabricate success.
- No provider outcome is inferred from event append success or failure.
- Report generation and artifact writing remain separate.

Errors must not leak side-effect IDs, target references, provider payloads, command output, parser payloads, paths, snippets, credentials, tokens, authorization headers, private keys, or secret-like values.

## 13. Relationship To Provider Writes

This plan is still pre-write.

Attempted/completed/failed lifecycle events are necessary for future write-capable adapters, but they are not provider writes themselves. A future provider adapter path must separately define:

- dry-run/preflight posture;
- credential boundary;
- provider request/response boundary;
- idempotency behavior;
- provider retry classification;
- result classification;
- rollback/compensation posture;
- high-assurance approval requirements;
- audit and report artifact closure.

## 14. Relationship To Reports And Artifacts

WorkReports and report artifacts should continue to cite SideEffect IDs and event IDs by stable reference.

The future implementation should not:

- automatically generate reports;
- automatically write artifacts;
- bypass existing SideEffect referential integrity checks;
- bypass approval-side-effect linkage gates;
- copy provider payloads into report sections.

Report artifact integrity should eventually be able to validate both the store record and the matching workflow event when required.

## 15. Test Plan

Future implementation tests should cover:

- attempted append requires successful store-backed `Proposed -> Attempted` transition;
- completed append requires successful store-backed `Attempted -> Completed` transition;
- failed append requires successful store-backed `Attempted -> Failed` transition;
- missing prior record fails before event append;
- store read failure fails before event append;
- store transition failure appends no event;
- event append failure does not fabricate success;
- identity mismatch fails without leaking IDs or target references;
- lifecycle mismatch fails without partial append;
- duplicate execution does not append duplicate attempted/completed/failed events;
- terminal run state rejects lifecycle append;
- generic audit projection remains bounded;
- no provider calls occur;
- no runtime side-effect execution occurs;
- no report artifacts are written;
- no CLI output or schema behavior is introduced;
- raw provider/spec/command/parser payload markers are not copied;
- existing executor, state, SideEffect, WorkReport, report artifact, policy, adapter, hook, local check, CLI, and docs tests still pass.

## 16. Proposed Implementation Sequence

1. Add explicit lifecycle-event append helper/input type for attempted/completed/failed.
2. Require the helper to consume store-backed transition output or equivalent validated store transition result.
3. Validate run, workflow, step, skill, side-effect, lifecycle, and idempotency identity.
4. Append the returned reference-only `SideEffectWorkflowEvent` payload through the existing event append pipeline.
5. Add focused tests for attempted/completed/failed success and fail-closed boundaries.
6. Review before provider write orchestration.

## 17. Deferred Work

- Provider mutation.
- Runtime side-effect execution.
- Provider result classification.
- Replay-as-success or reconciliation for same-state store updates.
- Automatic executor append from default `execute(...)`.
- Report artifact auto-write.
- CLI behavior.
- Workflow schema fields.
- Examples.
- Hosted/distributed runtime.
- Reasoning lineage.
- Recursive agents or agent swarms.
- Level 3/4 autonomy.

## 18. Open Questions

- Should event append and store transition be one atomic helper, or should a caller perform the store transition first and pass the result to the executor?
- Should a store transition success followed by event append failure require a later repair queue?
- Should the first implementation support all attempted/completed/failed states together, or start with attempted only?
- Should completed/failed append remain impossible until a provider result classifier exists?
- Should WorkReport artifact integrity eventually require both record and event citations for attempted/completed/failed states?
- Should explicit lifecycle append live on `LocalExecutor`, a helper service, or a future write-adapter orchestration layer?

## 19. Final Recommendation

Next implementation phase: **executor attempted/completed/failed SideEffect event append helper, local and explicit only**.

The implementation should compose the merged store-backed lifecycle transition helper into the executor event append boundary without provider calls, default runtime behavior, CLI changes, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 20. Dogfood Governance

- Workflow: `dg/d`
- Run: `run-1783269126605130000-2`
- Approval: `approval/run-1783269126605130000-2/planning-approved`
- Approval outcome: granted by delegated maintainer.
- Approved scope: planning docs and roadmap status only.
- Required validation: `npm run check:docs`.
- Phase close status: completed.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations.
- Event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `StepScheduled`.
- Out-of-kernel work: repository documentation edits, roadmap updates, and docs validation were performed by the agent outside the kernel under the governed planning phase boundary.
