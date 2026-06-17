# Hook Disclosure Model Plan

Status: Implemented. This plan follows the accepted `BeforeSkillInvocation` hook status/failure semantics work and the unsupported-status hardening review. The bounded hook disclosure core model is implemented as model-only vocabulary and validation. It does not implement warning continuation, skipped-with-disclosure continuation, blocked runtime behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sink emission, hook persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

Workflow OS now has a conservative explicit hook path:

- `Passed` may continue execution.
- explicit `FailedClosed` blocks the scoped skill invocation and fails the run before `SkillInvocationRequested`.
- `Warning`, `SkippedWithDisclosure`, and `Blocked` remain unsupported in the executor path.

The next safe step is a model-only hook disclosure layer. Warning and skipped hook statuses cannot safely continue based on free-form text, agent memory, logs, or unbounded notes. They need bounded, validated, redaction-safe disclosure values that can later be cited by WorkReports, represented in hook events, and evaluated by policy.

This plan does not implement runtime behavior.

## 2. Goals

- Define a bounded domain model for hook disclosures.
- Prepare future `Warning` and `SkippedWithDisclosure` hook status support without enabling continuation.
- Preserve deterministic workflow execution.
- Preserve fail-closed unsupported-status behavior until policy/report/event semantics exist.
- Prevent unbounded prose from becoming durable governance state.
- Prevent raw payloads, command output, provider output, parser output, file contents, paths, tokens, or secret-like values from entering hook disclosures.
- Keep disclosure data reference-first and report-safe.
- Prepare focused model-only tests for validation, serde, Debug, and non-leakage.
- Preserve existing `Passed`, `FailedClosed`, and unsupported-status behavior.

## 3. Non-Goals

Do not implement in the disclosure model phase:

- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- automatic hook invocation;
- broad executor hook checkpoints;
- workflow-declared hook configuration;
- runtime hook configuration;
- hook optionality semantics;
- policy-controlled warning or skipped continuation;
- post-terminal workflow events;
- conversion of `BeforeReport` into workflow events;
- dedicated hook audit sink emission;
- hook audit store or persistence;
- hook observability metrics;
- WorkReport hook event citation targets;
- CLI hook commands or rendering;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
- external provider calls;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- report artifact writes;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented behavior:

- `LocalExecutionRequest` can carry one explicit `BeforeSkillInvocation` hook input.
- Policy is evaluated before the hook.
- Local skill handler lookup happens before hook append.
- `Passed` appends hook requested/evaluated events and continues to skill invocation.
- explicit `FailedClosed` appends hook requested/evaluated events and fails before skill invocation.
- `Warning`, `SkippedWithDisclosure`, and `Blocked` return `executor.hook.before_skill_invocation.unsupported_status`.
- Unsupported statuses append no hook events, skill events, retries, WorkReports, or report artifacts.

Current gap after this phase:

- There is now a typed hook disclosure core model with ID, kind, severity, bounded title, bounded summary, stable references, sensitivity, and redaction metadata.
- Warning/skipped statuses still cannot continue or broaden executor behavior.
- WorkReport disclosure behavior for hook warnings/skips is not defined.
- Policy does not yet know how to allow or deny warning/skipped continuation.
- Hook optionality is not modeled.

## 5. Candidate Model

The model-only implementation should add the smallest idiomatic set of Rust types needed to represent hook disclosures safely. Candidate names:

- `AgentHarnessHookDisclosure`
- `AgentHarnessHookDisclosureKind`
- `AgentHarnessHookDisclosureId`
- `AgentHarnessHookDisclosureSeverity`
- `AgentHarnessHookDisclosureText`
- `AgentHarnessHookDisclosureReference`

The implementation should choose the smallest shape that fits existing hook model patterns.

Recommended v1 fields:

- disclosure ID;
- disclosure kind;
- severity;
- bounded title or label;
- bounded redacted summary;
- optional stable references;
- redaction metadata;
- sensitivity;
- created-by actor or system actor if already represented locally;
- created-at timestamp if consistent with existing hook model conventions.

The model must not store raw provider payloads, raw command output, raw logs, raw spec contents, raw parser payloads, raw environment values, credentials, authorization headers, private keys, token-like values, or unbounded agent prose.

## 6. Disclosure Kinds

Recommended minimal v1 kinds:

- `Warning`
- `Skipped`
- `PolicyNote`
- `ValidationNote`
- `OperatorNote`

`Warning` and `Skipped` are required to support future hook status semantics.

`PolicyNote`, `ValidationNote`, and `OperatorNote` are vocabulary only if included. They must not imply policy decisions, diagnostics, approvals, or local check results were created.

Avoid domain-specific kinds in the core model.

## 7. Severity

Recommended severity vocabulary:

- `Info`
- `Warning`
- `NeedsAttention`

Do not add `Error` or `Critical` unless a runtime mapping exists. Failed hook behavior is already represented by `FailedClosed`, not by a disclosure severity.

Severity must be descriptive only in the model phase. It must not change workflow status, policy decisions, retry behavior, approval behavior, or report artifact behavior.

## 8. Stable References

Hook disclosures should be reference-first.

Allowed reference targets may include stable identifiers already modeled in Workflow OS:

- `EvidenceReferenceId`
- `AgentHarnessHookInvocationId`
- workflow event ID
- audit event ID
- validation diagnostic/reference ID
- local check result reference ID
- typed handoff ID
- adapter telemetry reference where stable
- policy decision/reference ID where stable
- approval decision/reference ID where stable

If no stable reference exists, the disclosure may contain bounded redacted text, but it must not fabricate IDs or imply evidence exists.

## 9. Validation Rules

Validation should ensure:

- disclosure ID is valid;
- disclosure kind is valid;
- severity is valid;
- title/label is bounded and secret-aware;
- summary is bounded and secret-aware;
- references are valid, bounded, deduplicated, and secret-aware;
- redaction metadata is validated through the report-safe or hook-safe boundary used elsewhere;
- sensitivity is valid;
- no duplicate reference names or IDs;
- no raw payload marker fields;
- no token-like, credential-like, private-key-like, authorization-header-like, or environment-value-like strings;
- serialized values fail closed or sanitize according to an explicit policy.

Validation errors must use stable codes and must not include raw titles, summaries, references, paths, tokens, payloads, snippets, command output, provider output, parser output, or secret-like values.

## 10. Debug And Serialization

`Debug` output must be redaction-safe:

- do not print title text;
- do not print summary text;
- do not print raw references;
- show kind/severity and counts only where possible.

Serialization must not silently carry secret-like disclosure text or raw payload markers. Invalid serialized disclosures should fail deserialization or validation with stable non-leaking errors.

The implementation should follow existing `WorkReport`, `EvidenceReference`, and hook audit record redaction patterns.

## 11. Relationship To Hook Statuses

The disclosure model alone must not broaden runtime statuses.

Future status behavior should remain:

- `Passed`: no disclosure required.
- `FailedClosed`: may later include a disclosure only if the failure reason is bounded and non-secret.
- `Warning`: may not continue until a valid warning disclosure, policy allow decision, event semantics, and WorkReport disclosure behavior exist.
- `SkippedWithDisclosure`: may not continue until a valid skip disclosure, optionality model, policy allow decision, event semantics, and WorkReport disclosure behavior exist.
- `Blocked`: remains deferred until runtime blocked/escalation semantics are accepted.

## 12. Relationship To WorkReports

WorkReports should eventually cite or summarize hook disclosures without copying raw hook context.

Future report behavior should:

- cite stable hook invocation IDs and/or disclosure IDs if modeled;
- include bounded disclosure summaries only after validation;
- distinguish warnings from skipped hooks;
- disclose skipped required hooks as incomplete/deferred or policy-exceptional work;
- avoid treating a warning as evidence, approval, validation success, or local check success.

The disclosure model phase should not modify report generation behavior unless separately scoped.

## 13. Relationship To Audit And Events

Hook disclosures should eventually be visible in durable hook event/audit semantics, but this plan does not implement event append behavior.

Future event/audit work should decide:

- whether disclosures are embedded in `HookInvocationEvaluated`;
- whether disclosures are cited by ID from hook events;
- whether hook audit records include disclosure references or bounded disclosure snapshots;
- how replay treats disclosures;
- how duplicate run rehydration preserves disclosure meaning.

Until that is accepted, disclosures remain model-only.

## 14. Optionality Requirements

`SkippedWithDisclosure` requires hook optionality before continuation.

The disclosure model should not decide:

- which hooks are optional;
- who declares optionality;
- whether policy can override optionality;
- whether a skipped hook counts as incomplete work;
- whether skipped hooks affect terminal status.

Those decisions belong in a later hook optionality and skipped-continuation plan.

## 15. Privacy And Redaction

Hook disclosures must not store or output:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub issue/comment/file bodies;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded agent notes;
- unbounded model critique;
- raw file paths unless explicitly represented as validated references.

File paths should be treated conservatively and should not be copied into disclosure summaries by default.

## 16. Test Plan

Future model-only implementation tests should cover:

- valid minimal warning disclosure;
- valid minimal skipped disclosure;
- all v1 disclosure kinds are representable;
- all v1 severities are representable;
- invalid disclosure ID rejected;
- invalid/empty title rejected;
- unbounded title rejected;
- secret-like title rejected;
- invalid/empty summary rejected where required;
- unbounded summary rejected;
- secret-like summary rejected;
- valid stable references accepted;
- duplicate references rejected;
- secret-like references rejected;
- raw provider/spec/command/parser payload markers rejected;
- redaction metadata validation;
- sensitivity validation;
- serde round trip for valid disclosure;
- invalid serialized disclosure fails closed;
- deserialization errors do not leak secret-like values;
- `Debug` output does not leak title, summary, references, or secret-like values;
- serialization does not leak forbidden raw payload fields;
- existing hook, WorkReport, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests still pass.

## 17. Proposed Implementation Sequence

1. Implement hook disclosure core model only.
2. Add focused validation, serde, and redaction tests.
3. Update docs and create an implementation report.
4. Review the model.
5. Plan WorkReport citation/disclosure integration for supplied hook disclosure references.
6. Plan policy-controlled warning continuation.
7. Plan optionality-controlled skipped continuation.

Do not implement warning/skipped continuation before the model is reviewed.

## 18. Open Questions

- Should disclosures have stable IDs in the first model, or should they remain embedded bounded values until event persistence exists?
- Should warning/skipped disclosures share one type or use separate newtypes?
- Should disclosures be cited by WorkReports directly, or only through hook invocation IDs?
- Should disclosure references allow workflow event IDs before hook event IDs are durable enough?
- Should severity be restricted to `Info` and `Warning` until policy semantics exist?
- How should disclosure text relate to future reasoning lineage claims?
- Should skipped required hooks be represented as incomplete work, risk, or separate governance disclosure?

## 19. Final Recommendation

Recommended next phase: **WorkReport hook disclosure citation target vocabulary, model-only**.

The implementation added bounded, validated, redaction-safe disclosure types and focused tests. The model review accepted the phase with non-blocking follow-ups, and WorkReport hook disclosure citation planning is documented in [WorkReport Hook Disclosure Citation Plan](work-report-hook-disclosure-citation-plan.md). The implementation did not add warning continuation, skipped-with-disclosure continuation, blocked runtime behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, policy-controlled continuation, hook optionality, dedicated audit sink emission, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, reasoning lineage, side effects, writes, hosted behavior, or release posture changes.
