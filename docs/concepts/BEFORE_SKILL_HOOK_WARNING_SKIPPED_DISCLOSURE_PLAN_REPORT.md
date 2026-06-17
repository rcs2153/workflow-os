# BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan Report

## 1. Executive Summary

BeforeSkillInvocation warning and skipped disclosure semantics planning is complete. The plan concludes that `Warning` and `SkippedWithDisclosure` should remain unsupported in the executor path until bounded disclosure, hook optionality, policy-controlled continuation, WorkReport disclosure, replay, and audit semantics are designed and reviewed.

The plan recommends the next implementation phase be test-only hardening for unsupported warning/skipped/blocked statuses. It does not implement runtime continuation, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, reasoning lineage, hosted behavior, or release posture changes.

Fix-forward note: unsupported-status hardening tests are now implemented and documented in [BeforeSkillInvocation Unsupported Status Hardening Report](BEFORE_SKILL_HOOK_UNSUPPORTED_STATUS_HARDENING_REPORT.md). Warning/skipped continuation remains unimplemented.

## 2. Scope Completed

- Created [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](../implementation-plans/before-skill-hook-warning-skipped-disclosure-plan.md).
- Defined current status baseline after failed-closed implementation.
- Defined why warning continuation remains unsafe without disclosure and policy semantics.
- Defined why skipped-with-disclosure requires explicit hook optionality.
- Defined disclosure validation requirements.
- Defined policy, event, WorkReport, audit, retry, replay, and error-handling boundaries.
- Recommended the next implementation phase.

## 3. Scope Explicitly Not Completed

- No runtime code changes.
- No warning continuation.
- No skipped-with-disclosure continuation.
- No blocked runtime behavior.
- No automatic hook invocation.
- No workflow-declared hook configuration.
- No runtime hook configuration.
- No post-terminal workflow events.
- No dedicated hook audit sink/store.
- No hook observability metrics.
- No WorkReport hook event citation targets.
- No CLI behavior.
- No workflow schema fields.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No approvals.
- No `EvidenceReference` creation or attachment.
- No report artifact writes.
- No reasoning lineage.
- No side-effect boundary implementation.
- No writes.
- No recursive agents or agent swarms.
- No hosted/distributed runtime claims.
- No release posture changes.

## 4. Recommendation Summary

Recommended next implementation phase at planning close: **BeforeSkillInvocation warning/skipped unsupported-status hardening tests**. That implementation is now complete and ready for review.

The implementation should prove that:

- `Warning` appends no hook or skill events;
- `SkippedWithDisclosure` appends no hook or skill events;
- `Blocked` remains rejected or deferred;
- unsupported status errors use stable non-leaking codes/messages;
- disclosure-like values do not leak;
- existing `Passed` and `FailedClosed` behavior remains unchanged.

## 5. Boundary Summary

The plan preserves these boundaries:

- `Passed` remains the only continuing hook status.
- `FailedClosed` remains blocking-only.
- Warning/skipped continuation remains policy- and disclosure-gated future work.
- Skipped continuation requires a future optionality model.
- Hooks remain downstream of policy and do not replace approvals.
- Hooks do not create evidence, local check results, approvals, policy decisions, WorkReports, side effects, or writes.

## 6. Privacy And Redaction Summary

Future warning/skipped disclosures must be bounded, redaction-safe, and free of raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw spec contents, raw parser payloads, environment values, credentials, authorization headers, private keys, token-like values, and evidence payloads.

Unsupported-status errors must use stable codes and generic messages. They must not include hook IDs, references, phase IDs, paths, disclosures, payloads, snippets, tokens, or secret-like values.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Known Limitations

- Warning continuation is not implemented.
- Skipped-with-disclosure continuation is not implemented.
- Blocked runtime status support is not implemented.
- Hook optionality is not modeled.
- Hook disclosure model implementation is deferred.
- Policy-controlled warning/skipped continuation is deferred.
- WorkReport hook event citation targets remain deferred.
- Dedicated hook audit sink emission remains deferred.
- Automatic hook configuration remains deferred.

## 9. Recommended Next Phase

Recommended next phase: **BeforeSkillInvocation unsupported status hardening review**.

That review should verify the phase remained narrow and test-focused. It must not accept warning/skipped continuation, blocked runtime behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sinks, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, reasoning lineage, hosted behavior, or release posture changes.
