# High-Assurance Approval Executor Enforcement Plan

Status: First opt-in executor enforcement slice implemented in [High-Assurance Approval Executor Enforcement Report](../concepts/HIGH_ASSURANCE_APPROVAL_EXECUTOR_ENFORCEMENT_REPORT.md) and accepted with non-blocking follow-ups in [High-Assurance Approval Executor Enforcement Review](../concepts/HIGH_ASSURANCE_APPROVAL_EXECUTOR_ENFORCEMENT_REVIEW.md).

## 1. Executive Summary

The high-assurance approval control model exists, and the first pure in-memory approval decision validation helper is implemented and reviewed.

The next question is how the local executor should use that helper before recording an approval decision. This plan defines a narrow, opt-in executor path that validates explicit high-assurance approval controls before `ApprovalGranted` or `ApprovalDenied` events are appended.

The default `LocalExecutor::decide_approval(...)` behavior should remain unchanged. The first executor integration should be additive, local, deterministic, and explicit.

The first implementation adds the explicit opt-in executor method. It does not add workflow schema fields, runtime config, CLI behavior, examples, RBAC, IdP integration, quorum approval, hosted behavior, write-capable adapters, provider mutations, side-effect execution, reasoning lineage, or release posture changes.

## 2. Goals

- Add a future opt-in executor path for high-assurance approval decisions.
- Validate selected high-assurance controls before any approval decision event is appended.
- Preserve existing default approval behavior.
- Preserve event-log source-of-truth semantics.
- Preserve approval projection rebuildability.
- Preserve existing approval grant, denial, resume, and fail-closed semantics after validation passes.
- Reject unsupported high-assurance requirements fail-closed.
- Keep the first path local and deterministic.
- Keep inputs explicit rather than workflow-declared.
- Keep errors stable and non-leaking.
- Prepare for later WorkReport disclosure of high-assurance approval posture.
- Keep write-capable adapters blocked until executor enforcement is implemented and reviewed.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- changing `LocalExecutor::decide_approval(...)`;
- automatic high-assurance enforcement for every approval;
- workflow-declared high-assurance controls;
- workflow schema fields;
- runtime config;
- CLI behavior;
- examples;
- write-capable adapters;
- provider mutations;
- runtime side-effect execution;
- approval evidence attachment;
- WorkReport disclosure implementation;
- RBAC, IdP, SSO, SCIM, teams, groups, or external directory integration;
- quorum or multi-party approval enforcement;
- role-bound approval authority;
- approval UI;
- hosted or distributed runtime behavior;
- background expiration timers;
- approval revocation events;
- safety-critical certification claims;
- Level 3/4 autonomy enablement;
- reasoning lineage;
- release posture changes.

## 4. Current Runtime Baseline

Current local approval behavior:

- approval-gated steps append `ApprovalRequested` before skill invocation;
- the run enters `WaitingForApproval`;
- `SkillInvocationRequested` is not emitted while approval is pending;
- `LocalExecutor::decide_approval(...)` rehydrates the run and validates that it is waiting for approval;
- approval decisions validate against the event-derived approval request, not just a projection;
- duplicate decisions are rejected;
- decisions after terminal states are rejected;
- a granted decision appends `ApprovalGranted`, evaluates resume policy, appends `RunResumed`, then resumes the gated step;
- a denied decision appends `ApprovalDenied` and fails closed;
- missing approval projections can be rebuilt from event-derived state.

Current high-assurance behavior:

- `validate_high_assurance_approval_decision(...)` can validate explicit controls and supplied stable references without mutating runtime state;
- the helper supports the reviewed local subset: requester/approver separation, minimum approvals equal to one, required stable references, decision-time expiration checks, unsupported revocation fail-closed, and fail-closed denial posture;
- the helper is not wired into the executor.

## 5. First Executor Boundary

The first executor boundary should be an additive method, tentatively:

```rust
LocalExecutor::decide_approval_with_high_assurance(...)
```

The method should accept:

- the existing `LocalApprovalDecisionRequest`;
- explicit high-assurance approval controls;
- explicit supplied high-assurance references;
- a current timestamp for deterministic expiration validation.

The method should:

1. rehydrate the run using the existing approval path;
2. validate terminal and waiting state exactly as the existing approval path does;
3. retrieve the event-backed approval request;
4. construct the same proposed `ApprovalDecision` shape the existing path would append;
5. call `validate_high_assurance_approval_decision(...)`;
6. append no approval events if high-assurance validation fails;
7. after validation passes, continue through the same grant or denial behavior as the default path.

The default `decide_approval(...)` should not call high-assurance validation in the first implementation. That preserves compatibility and avoids silently making model vocabulary mandatory for existing local users.

## 6. Request Shape

Use a small executor-adjacent input type, such as:

```rust
LocalHighAssuranceApprovalDecisionRequest
```

It should contain:

- `approval: LocalApprovalDecisionRequest`;
- explicit high-assurance approval controls;
- supplied stable high-assurance approval references;
- `current_time: Timestamp`.

The input should not contain:

- raw provider payloads;
- raw command output;
- raw spec contents;
- raw parser payloads;
- raw source snippets;
- credentials;
- tokens;
- authorization headers;
- private keys;
- unbounded natural-language evidence.

The input should use existing model constructors and validation boundaries for controls and supplied references. It should not infer controls from policy effect strings, workflow schema fields, runtime config, or hidden state.

## 7. Event Ordering

High-assurance validation must happen before:

- `ApprovalGranted`;
- `ApprovalDenied`;
- `RunResumed`;
- any post-resume policy evaluation;
- any gated skill invocation event;
- any local handler invocation.

On validation failure:

- append no `ApprovalGranted`;
- append no `ApprovalDenied`;
- append no `RunResumed`;
- append no `RunFailed` solely for the rejected decision;
- do not mutate the run snapshot;
- do not save approval projection changes;
- return a structured `WorkflowOsError`.

On validation success:

- preserve existing grant semantics;
- preserve existing denial semantics;
- preserve existing resume policy evaluation;
- preserve existing projection rebuildability.

## 8. Grant And Denial Posture

The first executor path should validate both granted and denied decisions through the helper, but the test plan should distinguish the two.

Rationale:

- grants authorize continuation and must be protected;
- denials are fail-closed but still create durable approval history;
- validating both paths prevents a high-assurance approval packet from recording an unsupported or mismatched decision under a sensitive approval ID.

If implementation complexity suggests grant-only validation, that must be explicitly reviewed before code is written. The default recommendation is to validate both grant and denial before appending either decision event.

## 9. Control Source

The first executor path should accept high-assurance controls explicitly from the caller.

Do not add:

- workflow schema declarations;
- policy-effect-derived high-assurance controls;
- runtime config;
- governance profile defaults;
- enterprise stewardship configuration.

Rationale:

- explicit inputs avoid false governance from decorative YAML;
- the runtime boundary can be tested without schema churn;
- control source design can follow after executor semantics are proven.

## 10. Supplied Reference Policy

The executor method should pass supplied references through to the existing helper. It should not discover, create, or hydrate evidence payloads.

Allowed references should remain stable identifiers only, using the helper's supported target vocabulary:

- EvidenceReference;
- PolicyDecision event;
- SideEffect;
- ValidationReference;
- LocalCheckResult;
- WorkflowEvent;
- AuditEvent;
- WorkReport;
- AdapterTelemetry.

Rules:

- required references must be supplied explicitly;
- missing required references fail closed;
- duplicate supplied reference names fail closed;
- mismatched targets fail closed;
- supplied references must not copy payloads;
- the executor must not fabricate IDs;
- the executor must not create `EvidenceReference` values implicitly.

## 11. Expiration And Revocation

The executor path should preserve the helper's reviewed expiration behavior:

- `NotRequired` passes without expiration metadata;
- `RequiredOnRequest` requires request expiration metadata;
- `MustBeUnexpiredAtDecision` compares explicit `current_time` to the request expiration timestamp;
- `MustBeUnexpiredAtUse` remains unsupported until a protected-use checkpoint is designed.

Revocation enforcement remains deferred:

- `Unsupported` is accepted;
- event-backed revocation requirements fail closed as unsupported;
- report-only revocation behavior remains model vocabulary until disclosure is separately implemented.

No background timers, revocation events, or expiration events are added in this phase.

## 12. Error Handling

Validation failures should return `WorkflowOsError` with stable codes from the helper or a bounded executor wrapper code.

Errors must not include:

- actor IDs;
- approval IDs;
- reference names;
- reference IDs;
- side-effect IDs;
- file paths;
- source snippets;
- command output;
- provider payloads;
- parser payloads;
- environment variable values;
- credentials;
- tokens;
- private keys;
- secret-like values.

High-assurance rejection should not become a misleading user project diagnostic. It is a runtime approval decision failure, not workflow spec validation output.

## 13. State, Audit, And Observability

The first executor integration should not add new workflow event kinds.

On validation failure:

- no approval events are appended;
- no audit projection is emitted from a new event;
- no observability event is emitted from a new event;
- no approval projection is updated;
- no report artifact is written.

On validation success, existing approval events continue to project to audit and observability through the already implemented approval event path.

Future explicit high-assurance validation events or audit records should be planned separately after the opt-in executor method is reviewed.

## 14. WorkReport Disclosure

WorkReport disclosure remains future work.

The executor method should not generate or mutate reports. It should leave enough deterministic state for a later disclosure phase to cite:

- approval request;
- approval decision;
- high-assurance control IDs and versions supplied to the executor call, if model-safe disclosure is later added;
- supplied required reference kinds;
- validation success or failure posture, if a future event/disclosure model exists.

Do not create WorkReport sections, report artifacts, or EvidenceReference values in the first executor enforcement implementation.

## 15. Relationship To Writes And Side Effects

Executor-integrated high-assurance approval enforcement is a prerequisite for write-capable adapters, not authorization to build them.

Future write-capable adapter work must still compose:

- policy effect enforcement;
- side-effect proposal, denial, attempted, completed, and failed lifecycle semantics;
- side-effect approval linkage;
- high-assurance approval enforcement where required;
- idempotency;
- audit and observability;
- evidence references;
- WorkReport disclosure.

Credentials, adapter capability, or approval presence alone must never imply authority to mutate an external system.

## 16. Test Plan

Future implementation tests should cover:

- existing `decide_approval(...)` grant behavior remains unchanged;
- existing `decide_approval(...)` denial behavior remains unchanged;
- high-assurance grant with `SameActorAllowed` succeeds;
- high-assurance grant with `MustDiffer` and different actors succeeds;
- high-assurance grant with same actor fails before `ApprovalGranted`;
- `HumanApproverMustDiffer` rejects same actor without claiming IdP-backed human proof;
- missing requester identity fails closed for separation-required controls;
- high-assurance denial decision validates before `ApprovalDenied`;
- unsupported minimum approval count fails before any decision event;
- required EvidenceReference target present succeeds;
- required policy decision target present succeeds;
- required SideEffect target present succeeds;
- required validation/local-check/workflow/audit/report/adapter-telemetry targets present succeed;
- missing required reference fails before any decision event;
- mismatched required reference fails before any decision event;
- duplicate supplied reference names fail before any decision event;
- expiration required on request succeeds when metadata is present;
- expiration required on request fails when metadata is absent;
- unexpired-at-decision succeeds before expiration;
- expired-at-decision fails before any decision event;
- use-time expiration fails closed as unsupported;
- revocation requirements fail closed as unsupported;
- terminal run decisions remain rejected before high-assurance validation;
- non-waiting run decisions remain rejected before high-assurance validation;
- unknown approval ID remains rejected before high-assurance validation;
- projection mismatch remains rejected;
- validation failure appends no events;
- validation failure mutates no state;
- validation failure does not save approval projection changes;
- validation failure does not invoke local handlers;
- validation failure does not create report artifacts;
- errors and `Debug` output do not leak actors, IDs, references, paths, payloads, tokens, or secret-like values;
- existing approval, policy, SideEffect, WorkReport, adapter, validation, runtime, and CLI tests still pass.

## 17. Proposed Implementation Sequence

1. Add the explicit executor-adjacent high-assurance approval decision request type.
2. Factor the existing approval decision append/resume/fail behavior behind a private helper if needed, preserving public behavior.
3. Add `LocalExecutor::decide_approval_with_high_assurance(...)`.
4. Rehydrate and validate state using the same path as `decide_approval(...)`.
5. Construct the proposed `ApprovalDecision`.
6. Call `validate_high_assurance_approval_decision(...)` before appending events.
7. On failure, return the structured error with no runtime mutation.
8. On success, continue through the existing approval decision path.
9. Add focused executor tests and regression tests.
10. Review before any WorkReport disclosure, workflow-declared controls, runtime config, schemas, write-capable adapters, or provider mutations.

## 18. Deferred Work

Deferred until separately planned and reviewed:

- automatic enforcement for all approvals;
- workflow-declared high-assurance controls;
- policy-effect-derived high-assurance controls;
- governance profile control sources;
- enterprise steward/admin control sources;
- WorkReport high-assurance disclosure;
- explicit high-assurance validation events;
- dedicated high-assurance audit records;
- CLI approval flags for high-assurance controls;
- examples;
- RBAC/IdP authority;
- quorum approval;
- revocation events;
- background expiration timers;
- protected-use checkpoints;
- write-capable adapters;
- provider mutations;
- runtime side-effect execution;
- hosted/distributed runtime;
- reasoning lineage.

## 19. Open Questions

- Should the public input type wrap `LocalApprovalDecisionRequest`, or should the high-assurance fields be separate method parameters?
- Should denied decisions be fully validated in the first executor integration, or should the implementation validate grants only and document denial as fail-closed default behavior?
- Should high-assurance validation failures eventually append a rejected-validation event, or remain pure rejected method calls?
- How should later WorkReport disclosure cite high-assurance validation without storing raw control payloads?
- Should governance strictness profiles become the first control source after explicit inputs?
- Should the first CLI exposure wait until workflow-declared control sources exist?
- What is the smallest protected-use checkpoint needed for `MustBeUnexpiredAtUse`?

## 20. Final Recommendation

Next phase: **WorkReport high-assurance approval disclosure planning**.

The implementation adds a narrow explicit executor method that calls the reviewed validation helper before appending approval decision events. It keeps `decide_approval(...)` unchanged, preserves existing event-sourced approval semantics, appends no events on validation failure, and avoids schema, CLI, report disclosure, side-effect execution, write-capable adapter, hosted runtime, RBAC/IdP, quorum, reasoning lineage, or release posture changes. The executor boundary is now ready for a separate reporting-disclosure plan.
