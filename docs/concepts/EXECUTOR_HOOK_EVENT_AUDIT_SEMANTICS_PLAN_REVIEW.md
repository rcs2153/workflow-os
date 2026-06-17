# Executor Hook Event And Audit Semantics Plan Review

Review date: 2026-06-17

## 1. Executive Verdict

Plan accepted; proceed to hook workflow event vocabulary model, model-only.

The plan correctly follows the accepted explicit `BeforeReport` executor hook integration and defines the next durable semantics boundary without authorizing runtime mutation. It preserves the current `BeforeReport` checkpoint as report-path-only and non-mutating, rejects post-terminal metadata-only workflow events for now, and recommends a small model-only hook workflow event vocabulary phase before any executor path appends hook events or emits audit sink records.

## 2. Scope Verification

The plan stayed within planning-only scope.

It did not authorize:

- runtime hook broadening;
- automatic executor hook invocation;
- additional executor checkpoints;
- workflow event implementation;
- workflow event append behavior;
- audit sink emission for hook records;
- hook persistence;
- hook audit store;
- report artifact writes;
- workflow schema fields;
- workflow-declared hook configuration;
- runtime hook configuration;
- CLI hook commands;
- automatic local check execution;
- default local check handler registration;
- command execution;
- adapter invocation;
- external provider calls;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- reasoning lineage;
- side-effect boundary implementation;
- write-capable adapters;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 3. Baseline Assessment

The plan accurately captures the current runtime and hook baseline.

Current runtime behavior remains centered on `WorkflowRunEvent` append, snapshot projection, policy-before-skill behavior, approval events, retry/escalation handling, terminal completion/failure/cancellation events, and explicit report-bearing result construction.

Current hook behavior is accurately scoped:

- `execute_runtime_agent_harness_hook(...)` is explicit and in-memory;
- `LocalExecutor::execute_with_report(...)` can run one supplied `BeforeReport` hook after a terminal run exists;
- successful hook invocation IDs can be cited in WorkReports;
- hook failure returns as report-generation-side error;
- no hook event, audit sink record, persistence, artifact, schema, CLI, local check, adapter, side effect, or write behavior exists.

## 4. Source-Of-Truth Boundary Assessment

The source-of-truth boundaries are sound.

The plan preserves:

- `WorkflowRunEvent` as runtime state source of truth;
- `WorkflowRunSnapshot` as rebuildable projection;
- `AuditEvent` as runtime event projection;
- `PolicyAuditRecord` as explicit policy audit shape;
- `AgentHarnessHookInvocationResult` as in-memory validation result;
- `AgentHarnessHookAuditRecord` as model-only hook audit record;
- `WorkReport` as governed handoff artifact;
- `EvidenceReference` as citation pointer.

The review agrees with the plan's central boundary: hook execution must not become hidden workflow state. If a hook affects runtime behavior, it needs an accepted workflow event or separately accepted audit/store boundary.

## 5. Post-Terminal BeforeReport Assessment

The plan is correct to keep the implemented `BeforeReport` checkpoint out of the workflow event log for now.

Reasons:

- terminal states currently reject further mutating events;
- post-terminal metadata-only events would change replay and projection semantics;
- current report generation already has a separate report error channel;
- the implemented `BeforeReport` path is intentionally in-memory and report-path-only;
- adding a durable post-terminal event would require separate transition, audit, artifact, and compatibility design.

If durable `BeforeReport` history becomes necessary, the plan correctly suggests considering a hook audit store or report artifact linkage before allowing post-terminal workflow events.

## 6. Event Vocabulary Assessment

The candidate hook event vocabulary is reasonable as future model vocabulary:

- `HookInvocationRequested`;
- `HookInvocationEvaluated`;
- `HookInvocationFailedClosed`;
- `HookInvocationSkipped`;
- `HookInvocationBlocked`.

The plan properly frames these as candidates rather than implemented or final event names. The proposed minimal payload fields are bounded and reference-first: hook invocation ID, contract identity, hook kind, status, optional step/phase/correlation context, stable references or counts, redaction metadata, and sensitivity.

Non-blocking implementation note: the next model-only phase should choose the smallest event set needed for tests and should avoid over-modeling all five names if two or three names are enough to prove the boundary.

## 7. Event Ordering Assessment

The ordering section identifies the right high-risk checkpoints.

The plan correctly defers:

- `BeforeReport` workflow events because they would be post-terminal;
- `BeforeSkillInvocation` because it is side-effect-adjacent;
- `AfterSkillSuccess` and `AfterSkillFailure` because they affect continuation, retry, and escalation;
- approval-adjacent hooks because they must not fabricate or bypass approval context.

The plan also correctly requires exact ordering before any event-producing executor implementation.

## 8. State Transition Assessment

The state transition posture is conservative and appropriate.

The review agrees that initial hook event vocabulary, if implemented, should be model-only and should not add new runtime statuses. Future state-preserving hook events should be accepted only from reviewed non-terminal states, and terminal states should continue to reject hook events unless a separate post-terminal metadata-event model is accepted.

The plan appropriately rejects making `HookInvocationFailedClosed` or `HookInvocationBlocked` directly alter run status without a paired reviewed transition such as `RunFailed`, `RunPaused`, or escalation.

## 9. Failure Semantics Assessment

The failure semantics policy is clear enough to guide future implementation.

The plan distinguishes:

- `Passed`;
- `Warning`;
- `SkippedWithDisclosure`;
- `FailedClosed`;
- `Blocked`.

It correctly states that warning-only behavior before side-effect-adjacent checkpoints is unsafe unless policy, audit, and report disclosure are implemented. It also preserves the current `BeforeReport` hook failure as a report-generation-side error that must not change workflow result status.

## 10. Audit Semantics Assessment

The audit sink emission policy is correct.

The plan rejects automatic hook audit sink emission until event/source semantics are accepted. It identifies the right future options:

1. project hook workflow events through `AuditEvent::from_workflow_event(...)`;
2. add a dedicated hook audit sink method for `AgentHarnessHookAuditRecord`;
3. persist hook audit records in a hook-specific store and cite them from reports.

The recommended sequencing is conservative and reviewable: model-only event vocabulary, review, event projection/audit sink planning, optional audit sink model changes, and only then executor integration for a pre-terminal checkpoint.

## 11. Idempotency And Replay Assessment

The idempotency and replay section addresses the key risk created by durable hook events.

The plan correctly requires future state-visible hooks to define deterministic hook invocation identity, idempotency key strategy, duplicate run behavior, replay behavior, and whether hook execution is re-run or recovered from durable history.

It also accurately states that current `BeforeReport` can re-run in-memory validation on duplicate report-bearing calls only because it writes no event or durable record. That behavior must not leak into future state-visible hook checkpoints.

## 12. Policy, Approval, Local Check, And Adapter Boundary Assessment

The plan preserves key governance boundaries.

Hooks must not:

- replace deterministic policy;
- request, grant, or deny approvals;
- infer approval from model self-review;
- run local checks;
- register handlers;
- execute shell commands;
- invoke adapters;
- call providers;
- create command-output evidence;
- treat missing checks as passing.

This keeps the hook layer as governed checkpoint semantics rather than a backdoor execution or authority layer.

## 13. Evidence And WorkReport Assessment

The plan keeps EvidenceReference and WorkReport boundaries honest.

Future hook events or audit records may be cited by WorkReports only by stable reference. They must not create `EvidenceReference` values implicitly, copy evidence payloads, copy hook disclosures or raw context into report summaries, fabricate missing hook evidence, or treat hook success as validation proof.

This is aligned with the current report and evidence posture.

## 14. Privacy And Redaction Assessment

The privacy posture is strong.

The plan forbids raw prompts, raw spec contents, raw command output, command transcripts, provider payloads, CI logs, Jira/GitHub raw bodies, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, and unbounded summaries.

The next implementation should enforce these through constructors, serde validation, redaction-safe `Debug`, and stable non-leaking error codes.

## 15. Test Plan Assessment

The future test plan is adequate for the recommended model-only implementation.

It covers:

- hook event kind representation;
- valid minimal payloads;
- invalid ID rejection;
- secret-like reference rejection;
- redaction metadata validation;
- redaction-safe debug output;
- serialization non-leakage;
- invalid serialized payload fail-closed behavior;
- terminal-state rejection;
- no hook execution;
- no audit sink emission;
- no persistence;
- no local check, command, or adapter execution;
- existing runtime and model test preservation.

Non-blocking additions for the implementation prompt:

- prove existing `WorkflowRunEventKindName` ordering/serde remains backward-compatible;
- prove existing `LocalExecutor::execute(...)` and `execute_with_report(...)` do not append hook events;
- prove `AuditEvent::from_workflow_event(...)` either ignores model-only hook events or remains unchanged if hook events are not integrated into `WorkflowRunEvent` yet.

## 16. Documentation Review

The roadmap and related docs now state that executor hook event/audit semantics planning is documented, while hook workflow events, audit sink emission, persistence, CLI behavior, schema fields, broader automatic executor hook invocation, automatic local checks, side effects, writes, recursive agents, agent swarms, hosted behavior, and release posture changes remain unimplemented.

The plan does not overclaim current runtime support.

## 17. Planning Blockers

No planning blockers.

## 18. Non-Blocking Follow-Ups

- The next implementation prompt should choose the smallest hook event vocabulary needed, rather than implementing every candidate if not needed.
- The next implementation should explicitly preserve existing executor behavior and tests.
- A later phase should decide whether hook audit records are projected from hook events, stored separately, or both.
- A later phase should decide whether `BeforeReport` ever needs durable post-terminal representation.

## 19. Recommended Next Phase

Recommended next phase: **hook workflow event vocabulary model, model-only**.

That phase should add the smallest accepted model surface for future hook workflow events and transition names. It must not wire hooks into `LocalExecutor`, append events from executor paths, emit audit sink records, persist hook records, expose CLI behavior, add workflow schema fields, run local checks, execute commands, invoke adapters, model side effects, add writes, enable recursive agents or agent swarms, claim hosted behavior, or change release posture.

## 20. Validation

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.
