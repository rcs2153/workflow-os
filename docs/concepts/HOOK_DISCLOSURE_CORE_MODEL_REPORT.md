# Hook Disclosure Core Model Report

## 1. Executive Summary

The hook disclosure core model is implemented as a model-only primitive for future warning and skipped hook semantics.

The implementation upgrades hook disclosures from a bounded kind/text value into a validated, redaction-safe model with stable identity, kind, severity, bounded title, bounded summary, stable references, sensitivity, and redaction metadata. This gives future `Warning` and `SkippedWithDisclosure` paths a governed disclosure object to validate and cite before any continuation behavior is considered.

This phase does not broaden runtime hook status behavior.

## 2. Scope Completed

- Added `AgentHarnessHookDisclosureId`.
- Added `AgentHarnessHookDisclosureSeverity`.
- Added `AgentHarnessHookDisclosureReference`.
- Added `AgentHarnessHookDisclosureDefinition`.
- Expanded `AgentHarnessHookDisclosure` to include stable ID, kind, severity, title, summary, references, redaction metadata, and sensitivity.
- Added validation for disclosure IDs, title, summary, references, duplicate references, redaction metadata, and secret-like values.
- Added redaction-safe `Debug` behavior for disclosures and disclosure references.
- Added serde round-trip and fail-closed deserialization behavior for disclosures.
- Updated hook invocation tests to use the new disclosure model.
- Updated roadmap and concept documentation to state that the model is implemented.

## 3. Scope Explicitly Not Completed

This phase did not implement:

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
- approval request or approval decision creation;
- approval evidence attachment;
- EvidenceReference creation or attachment;
- report artifact writes;
- reasoning lineage;
- side-effect boundary behavior;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime behavior;
- release posture changes.

## 4. Model Types Added

- `AgentHarnessHookDisclosureId`: stable bounded disclosure identifier.
- `AgentHarnessHookDisclosureKind`: model vocabulary for `Warning`, `Skipped`, `PolicyNote`, `ValidationNote`, and `OperatorNote`.
- `AgentHarnessHookDisclosureSeverity`: model vocabulary for `Info`, `Warning`, and `NeedsAttention`.
- `AgentHarnessHookDisclosureReference`: validated wrapper around stable hook references.
- `AgentHarnessHookDisclosureDefinition`: constructor input for validated disclosure creation.
- `AgentHarnessHookDisclosure`: bounded, redaction-safe disclosure value.

The existing hook invocation result and hook audit record models continue to carry disclosure values, but runtime execution semantics remain unchanged.

## 5. Validation Boundary Summary

Validation ensures:

- disclosure IDs are valid and non-secret-like;
- titles are present, bounded, and non-secret-like;
- summaries are present, bounded, and non-secret-like;
- references are valid stable hook references;
- duplicate disclosure references are rejected;
- redaction metadata is bounded and secret-aware through the existing hook invocation redaction boundary;
- invalid serialized disclosures fail closed through the validated constructor path.

Validation errors use stable non-leaking codes and do not include raw titles, summaries, references, token-like values, raw payload markers, command output, provider output, parser output, or secret-like values.

## 6. Redaction And Privacy Summary

The model does not store raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw GitHub file contents, raw spec contents, raw parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, or unbounded agent prose.

`Debug` output redacts the disclosure ID, title, summary, redaction metadata, and raw reference values. Serialization may carry validated bounded title and summary text, but construction and deserialization reject secret-like values and forbidden raw payload markers.

## 7. Test Coverage Summary

Focused tests cover:

- valid disclosure construction and accessors;
- all v1 disclosure kinds;
- all v1 severities;
- duplicate disclosure references rejected;
- secret-like disclosure summaries rejected;
- secret-like titles rejected;
- secret-like references rejected;
- secret-like redaction metadata rejected;
- redaction-safe `Debug`;
- serde round trip for valid disclosure;
- invalid serialized disclosure fail-closed behavior;
- disclosure use inside hook invocation results and hook audit records;
- existing hook invocation, failed-closed, runtime helper, audit record, and citation behavior.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test agent_harness_hook_invocation` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- `Warning`, `SkippedWithDisclosure`, and `Blocked` remain unsupported as runtime continuation statuses.
- Hook optionality is not modeled.
- Policy does not yet decide warning or skipped continuation.
- WorkReports do not yet cite hook disclosure IDs directly.
- Hook events and audit projections do not yet include disclosure-specific event semantics beyond carrying bounded disclosure values in existing in-memory records.
- Disclosures are model values only and are not persisted.

## 10. Recommended Next Phase

Recommended next phase: **hook disclosure core model review**.

The model should be reviewed before any phase considers WorkReport disclosure citation, policy-controlled warning continuation, or optionality-controlled skipped continuation.
