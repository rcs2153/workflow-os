# Hook Disclosure Core Model Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The hook disclosure core model stays within the approved model-only scope. It gives future warning and skipped hook semantics a bounded, validated, redaction-safe disclosure object without changing current runtime behavior.

Proceed to **WorkReport hook disclosure citation planning** after the follow-ups are acknowledged.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented scope:

- model-only `AgentHarnessHookDisclosure` expansion;
- stable disclosure ID;
- disclosure kind and severity vocabulary;
- stable disclosure references;
- bounded title and summary;
- redaction metadata and sensitivity;
- validation, serde, redaction-safe `Debug`, tests, docs, and report.

No accidental implementation was found for:

- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- automatic hook invocation;
- workflow-declared hook configuration;
- runtime hook configuration;
- policy-controlled continuation;
- hook optionality;
- dedicated hook audit sink emission;
- hook persistence;
- CLI hook commands or rendering;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
- approvals or approval evidence attachment;
- EvidenceReference creation or attachment;
- report artifact writes;
- reasoning lineage;
- side-effect boundary behavior;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime behavior;
- release posture changes.

Existing executor tests still prove `Warning`, `SkippedWithDisclosure`, and `Blocked` remain unsupported continuation statuses.

## 3. Model Assessment

The implemented model is appropriately domain-neutral and minimal for the planned boundary.

Implemented concepts:

- `AgentHarnessHookDisclosureId`;
- `AgentHarnessHookDisclosureKind`;
- `AgentHarnessHookDisclosureSeverity`;
- `AgentHarnessHookDisclosureReference`;
- `AgentHarnessHookDisclosureDefinition`;
- expanded `AgentHarnessHookDisclosure`.

The model captures:

- stable disclosure identity;
- kind;
- severity;
- bounded title;
- bounded summary;
- stable references;
- redaction metadata;
- sensitivity.

The implementation intentionally does not add created-by or created-at fields. That is acceptable for this phase because hook invocation results and audit records already carry actor and timestamp context. If disclosures later become independently persisted or cited outside their invocation context, created-by/created-at should be reconsidered.

## 4. Kind And Severity Assessment

The minimal v1 disclosure kinds are implemented:

- `Warning`;
- `Skipped`;
- `PolicyNote`;
- `ValidationNote`;
- `OperatorNote`.

The minimal v1 severities are implemented:

- `Info`;
- `Warning`;
- `NeedsAttention`.

No `Error` or `Critical` severity was added, which preserves the existing design where failed hook behavior is represented by `FailedClosed`, not by disclosure severity.

## 5. Reference Assessment

The disclosure reference model wraps existing stable `AgentHarnessHookReference` values. This is a conservative choice that reuses existing validation and redaction behavior.

Supported reference families through the existing hook reference vocabulary include:

- evidence reference IDs;
- local check result IDs;
- typed handoff IDs;
- validation reference IDs;
- workflow event IDs;
- audit event IDs;
- policy IDs;
- policy decision event IDs;
- approval decision reference IDs.

The implementation does not yet support direct disclosure references to `AgentHarnessHookInvocationId` or adapter telemetry references. That is not a blocker because the phase only needed a safe reference-first model, but it should be considered before WorkReport disclosure citation integration.

## 6. Validation Assessment

Validation is deterministic and redaction-safe.

Verified behavior:

- disclosure IDs are validated through hook invocation identifier rules;
- title and summary are required, bounded, and secret-aware;
- references are validated;
- duplicate references are rejected;
- redaction metadata is validated through the hook invocation redaction boundary;
- invalid serialized disclosures are reconstructed through the validated constructor path;
- validation errors use stable codes;
- validation errors do not include raw secret-like values.

No issue was found with validation changing runtime execution semantics.

## 7. Runtime Semantics Assessment

Runtime behavior remains unchanged.

Verified:

- `Passed` remains the only continuing hook status.
- explicit `FailedClosed` remains the only implemented fail-closed hook result path.
- `Warning`, `SkippedWithDisclosure`, and `Blocked` still fail closed as unsupported statuses.
- unsupported statuses append no hook events, skill events, retries, WorkReports, or report artifacts.
- no executor checkpoint broadening was added.
- no workflow event append behavior was added by this phase.

This is the right boundary. The model exists so future continuation work can be planned safely, not so continuation happens implicitly.

## 8. Privacy And Redaction Assessment

The privacy posture is acceptable for this phase.

Verified:

- raw provider payloads are not modeled;
- raw command output is not modeled;
- raw CI logs are not modeled;
- raw Jira/GitHub bodies or file contents are not modeled;
- raw spec contents are not modeled;
- raw parser payloads are not modeled;
- environment values are not modeled;
- credentials, authorization headers, private keys, and token-like values are rejected by validation;
- `Debug` output redacts disclosure ID, title, summary, reference values, and redaction metadata;
- deserialization failures do not leak the tested secret-like values.

Serialization intentionally includes validated bounded title and summary text. That is acceptable because the constructor and deserializer reject secret-like values and raw payload markers, and because this model is not persisted or exposed through CLI behavior in this phase.

## 9. Serde And Compatibility Assessment

Serde behavior is appropriate:

- valid disclosures serialize and deserialize;
- invalid serialized disclosure summaries fail closed;
- invalid serialized disclosure values route through the constructor validation boundary;
- field names are clear and suitable for future schema planning.

This phase changes the constructor shape and serialized disclosure shape from the prior bounded kind/text value. That is acceptable because the API remains pre-release and the phase explicitly upgrades the model before any persistence or schema exposure.

No workflow spec schema changes were introduced.

## 10. Relationship To WorkReports

The model is compatible with future WorkReport integration.

Verified:

- disclosures carry stable IDs that can become citation targets later;
- disclosures can carry stable references without creating evidence;
- disclosures do not create `EvidenceReference` values implicitly;
- report generation behavior was not changed;
- WorkReports still cite hook invocation IDs, not hook disclosures directly.

The next planning phase should decide whether WorkReports cite disclosure IDs directly, cite hook invocation IDs with bounded disclosure summaries, or both.

## 11. Relationship To Events And Audit

The model remains consistent with existing hook event and audit boundaries.

Verified:

- existing hook invocation results and hook audit records can carry disclosure values;
- no dedicated audit sink emission was added;
- no persistent hook audit store was added;
- no hook workflow event semantics were broadened;
- replay behavior remains unchanged.

Future event/audit planning should decide whether disclosure IDs become durable event fields or remain embedded bounded snapshots inside hook invocation records.

## 12. Test Quality Assessment

Tests are solid for the model-only boundary and runtime non-regression.

Covered:

- valid disclosure construction and accessors;
- v1 kinds representable;
- v1 severities representable;
- duplicate disclosure references rejected;
- secret-like summary rejected;
- secret-like title rejected;
- secret-like reference rejected;
- secret-like redaction metadata rejected;
- `Debug` non-leakage;
- serde round trip;
- invalid serialized disclosure fail-closed behavior;
- disclosure use in hook invocation and audit records;
- existing unsupported warning/skipped/blocked executor behavior;
- existing WorkReport, EvidenceReference, Diagnostic, validation, adapter, local-check, and runtime tests through full workspace test run.

Shallow or missing tests:

- invalid disclosure ID is not tested directly for the disclosure model;
- empty title and empty summary are not tested directly;
- unbounded title and unbounded summary are not tested directly;
- standalone `AgentHarnessHookDisclosureReference` deserialization is not tested directly;
- direct coverage for every supported reference variant inside disclosures is not exhaustive.

These are useful follow-ups but not blockers because the underlying shared validators and broader hook/reference tests already cover the main safety boundary.

## 13. Documentation Review

Docs correctly state:

- bounded hook disclosure core model is implemented;
- model is model-only;
- warning/skipped/blocked continuation is not implemented;
- automatic hook invocation is not implemented;
- workflow-declared hook configuration is not implemented;
- runtime hook configuration is not implemented;
- persistence is not implemented;
- CLI hook behavior is not implemented;
- schemas are not changed;
- local check execution, command execution, adapter invocation, approvals, evidence attachment, reasoning lineage, side effects, writes, hosted behavior, recursive agents, agent swarms, and release posture changes remain unsupported.

The implementation report is present at [HOOK_DISCLOSURE_CORE_MODEL_REPORT.md](HOOK_DISCLOSURE_CORE_MODEL_REPORT.md).

## 14. Blockers

No blockers.

## 15. Non-Blocking Follow-Ups

- Add direct tests for invalid disclosure ID, empty title, empty summary, unbounded title, and unbounded summary.
- Add direct tests for standalone disclosure reference deserialization if the type remains publicly deserializable.
- Decide whether `AgentHarnessHookDisclosureReference` should support direct `AgentHarnessHookInvocationId` references.
- Decide whether adapter telemetry references should be added before WorkReport disclosure citation integration.
- Revisit created-by/created-at only if disclosures become independently persisted or cited outside hook invocation context.

## 16. Recommended Next Phase

Recommended next phase: **WorkReport hook disclosure citation planning**.

Reason: the disclosure model is now safe enough to plan how final work reports should cite or summarize hook disclosures without copying raw hook context. Policy-controlled warning continuation and skipped-hook optionality should wait until report citation and event/audit semantics for disclosures are clearer.

## 17. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
