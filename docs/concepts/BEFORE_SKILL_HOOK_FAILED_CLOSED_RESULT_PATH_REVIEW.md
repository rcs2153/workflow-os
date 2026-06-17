# BeforeSkillInvocation Failed-Closed Result Path Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation delivers the intended narrow `BeforeSkillInvocation` failed-closed result path: explicit failed-closed hook inputs can produce validated in-memory hook results, append requested/evaluated workflow events, and fail the run before `SkillInvocationRequested`. The phase stays within the accepted boundary and does not broaden hook execution into automatic configuration, warning/skipped continuation, blocked runtime behavior, local check execution, adapter execution, approvals, evidence creation, side effects, writes, CLI behavior, schemas, hosted behavior, or release posture changes.

## 2. Scope Verification

The phase stayed within approved scope.

Verified scope completed:

- explicit failed-closed in-memory hook invocation helper;
- explicit failed-closed runtime hook helper;
- explicit `BeforeSkillInvocation` executor status input;
- failed-closed requested/evaluated workflow events;
- run failure before `SkillInvocationRequested`;
- stable non-leaking failed-closed executor error;
- replay/idempotency tests;
- documentation and implementation report.

No accidental implementation found for:

- automatic hook invocation;
- workflow-declared hook configuration;
- runtime hook configuration;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- post-terminal workflow events;
- dedicated hook audit sink emission;
- hook persistence;
- hook observability metrics;
- CLI behavior;
- workflow schemas;
- local check execution;
- command execution;
- adapter invocation;
- approvals or approval evidence attachment;
- `EvidenceReference` creation or attachment;
- WorkReport hook event citation targets;
- report artifact writes;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents, agent swarms, hosted behavior, or release posture changes.

## 3. Helper And Model Assessment

The helper approach is minimal and idiomatic for the existing model.

The implementation adds:

- `invoke_agent_harness_hook_failed_closed(...)`;
- `execute_runtime_agent_harness_hook_failed_closed(...)`;
- private status-parameterized helper functions for reuse.

The helpers reuse the existing validation boundary for hook contracts, hook kind matching, workflow/run/schema/spec identity, target context, references, disclosures, redaction metadata, required references, and side-effect rejection. Unsupported statuses remain rejected at the helper layer.

The implementation does not introduce a new generic status constructor that could accidentally authorize warning, skipped, or blocked runtime behavior.

## 4. Executor Behavior Assessment

The executor behavior matches the accepted plan:

- `Passed` still appends requested/evaluated hook events and continues execution.
- `FailedClosed` appends requested/evaluated hook events with `FailedClosed`, then fails the run before skill invocation.
- `Warning`, `SkippedWithDisclosure`, and `Blocked` remain unsupported and append no hook or skill events.
- Policy decisions still precede hook events.
- Missing handlers and policy denial remain covered by earlier tests and append no hook events.
- Failed-closed execution does not invoke the local skill handler.
- Failed-closed execution does not append skill invocation, attempt, success, failure, or retry events.

The stable failure code is:

```text
executor.hook.before_skill_invocation.failed_closed
```

The failure message is generic and non-leaking.

## 5. Event Ordering And Idempotency Assessment

The failed-closed path preserves deterministic event ordering:

1. policy decision;
2. `HookInvocationRequested`;
3. `HookInvocationEvaluated(FailedClosed)`;
4. `RunFailed`.

Requested and evaluated event payloads are constructed before either event is appended, which preserves the safe-result-before-append posture for validation failures. Backend append failures still remain governed by existing backend error behavior, which is acceptable for this phase.

Replay/idempotency coverage verifies duplicate execution with the same run ID rehydrates the existing failed run and does not duplicate hook events.

## 6. Validation Boundary Assessment

Validation remains properly layered:

- invalid hook identity fails before hook events;
- side-effect requests fail before hook events;
- unsupported statuses fail before hook events;
- failed-closed hook result construction must pass the same model validation as passed hook result construction;
- the runtime helper produces a model-only audit record without emitting or persisting a dedicated hook audit sink record.

No fake evidence, approval decisions, policy decisions, local check results, WorkReports, report artifacts, or side effects are created.

## 7. Privacy And Redaction Assessment

The implementation remains redaction-safe.

Verified:

- failed-closed errors use stable codes and generic messages;
- error assertions check that scoped hook references and checkpoint identifiers do not leak;
- helper tests cover side-effect rejection without leaking workflow/context values;
- no raw provider payloads, command output, CI logs, Jira/GitHub bodies, spec contents, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, evidence payloads, local check output, approval content, or WorkReport content are copied.

Earlier hook model and event tests continue to cover Debug/serialization redaction for hook inputs, results, audit records, and workflow event payloads.

## 8. Test Quality Assessment

The new tests are focused and behavior-oriented.

New coverage includes:

- valid failed-closed hook helper result;
- failed-closed helper side-effect rejection;
- failed-closed runtime helper result plus audit record;
- failed-closed runtime helper side-effect rejection;
- failed-closed executor event order;
- failed-closed run failure before skill invocation;
- no local skill handler call;
- no report artifact creation;
- duplicate failed-closed run replay without duplicate hook events;
- unsupported warning status appends no hook or skill events.

Existing coverage continues to protect:

- no-hook executor behavior;
- passed hook behavior;
- later-step targeting;
- missing handler behavior;
- policy denial ordering;
- hook input debug redaction;
- runtime event transition constraints;
- audit projection;
- WorkReport, EvidenceReference, local check, adapter, and CLI regressions.

Non-blocking gap: there is no direct failure-injection test for a backend append failure between requested and evaluated events. That behavior remains under existing backend error semantics and does not block this phase.

## 9. Documentation Review

Documentation has been updated honestly.

Verified docs state:

- the first explicit failed-closed result path is implemented;
- `Passed` remains the only continuing hook status;
- explicit `FailedClosed` fails before `SkillInvocationRequested`;
- warning/skipped/blocked broadening is not implemented;
- automatic hook invocation is not implemented;
- workflow-declared hook configuration is not implemented;
- runtime hook configuration is not implemented;
- dedicated hook audit sink emission is not implemented;
- local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, persistence, CLI behavior, schemas, hosted behavior, and release posture changes remain unimplemented.

The planning report keeps historical planning limitations while adding fix-forward language to prevent stale current-state claims.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a focused backend failure-injection test if the executor gains a convenient append-failure seam for hook events.
- Plan warning and skipped-with-disclosure semantics before allowing any non-passed status to continue execution.
- Decide whether failed-closed hook events should become WorkReport citation targets after hook event citation planning.
- Keep `Blocked` deferred until a runtime blocked/escalation status model is explicitly accepted.

## 12. Recommended Next Phase

Recommended next phase: **BeforeSkillInvocation warning/skipped disclosure semantics planning**.

Reason: failed-closed blocking semantics are now implemented and reviewed. The next risky boundary is not more execution code; it is deciding whether warning or skipped statuses can ever continue, what durable disclosure they require, how policy controls them, and how WorkReports should present them without creating fake evidence or model-opinion governance.

Fix-forward note: that planning is now documented in [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](../implementation-plans/before-skill-hook-warning-skipped-disclosure-plan.md). The plan keeps warning/skipped continuation deferred and recommends unsupported-status hardening tests as the next implementation phase.

## 13. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
