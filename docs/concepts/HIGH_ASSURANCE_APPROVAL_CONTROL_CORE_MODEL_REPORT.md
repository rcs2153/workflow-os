# High-Assurance Approval Control Core Model Report

## 1. Executive Summary

The high-assurance approval control core model is implemented as model-only vocabulary and validation.

The model defines domain-neutral approval-control posture for sensitive or irreversible future actions: protected action classes, requester/approver separation posture, approval count, required reference packets, expiration/revocation posture, denial behavior, report disclosure requirements, sensitivity, and redaction metadata.

This phase does not enforce high-assurance approvals at runtime. It does not add write-capable adapters, provider mutations, runtime side-effect execution, RBAC, IdP integration, quorum approval, schemas, CLI behavior, hosted behavior, examples, or release posture changes.

## 2. Scope Completed

Implemented:

- `HighAssuranceApprovalControl`;
- `HighAssuranceApprovalControlDefinition`;
- `HighAssuranceApprovalControlId`;
- `HighAssuranceApprovalControlVersion`;
- `HighAssuranceProtectedActionKind`;
- `HighAssuranceRequesterApproverRule`;
- `HighAssuranceApprovalExpirationPolicy`;
- `HighAssuranceApprovalRevocationPolicy`;
- `HighAssuranceApprovalDenialBehavior`;
- `HighAssuranceApprovalReportDisclosure`;
- `HighAssuranceApprovalRequiredReference`;
- `HighAssuranceApprovalRequiredReferenceTarget`;
- `workflow-core` exports for the model types;
- deterministic validation for required fields, duplicate declarations, bounded identifiers, and redaction metadata;
- stable non-leaking validation error codes;
- serde support with fail-closed deserialization through constructors;
- redaction-safe `Debug` behavior;
- focused tests;
- documentation status updates.

## 3. Scope Explicitly Not Completed

Not implemented:

- runtime high-assurance approval enforcement;
- changes to existing approval request or decision execution;
- requester/approver identity enforcement;
- human-vs-agent authority enforcement;
- role-based approval authority;
- external identity provider integration;
- quorum or multi-party approval enforcement;
- approval revocation enforcement;
- background expiration timers;
- evidence sufficiency enforcement;
- automatic report disclosure population;
- approval evidence attachment;
- write-capable adapters;
- provider mutation;
- runtime side-effect execution;
- side-effect attempt/completion/failure execution semantics;
- workflow schema fields;
- CLI behavior;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Model Types Added

The implementation adds a single validated control type, `HighAssuranceApprovalControl`, constructed from `HighAssuranceApprovalControlDefinition`.

Identity is modeled by:

- `HighAssuranceApprovalControlId`;
- `HighAssuranceApprovalControlVersion`;
- `SchemaVersion`.

Approval posture is modeled by:

- `HighAssuranceProtectedActionKind`;
- `HighAssuranceRequesterApproverRule`;
- `minimum_approvals`;
- `HighAssuranceApprovalRequiredReference`;
- `HighAssuranceApprovalRequiredReferenceTarget`;
- `HighAssuranceApprovalExpirationPolicy`;
- `HighAssuranceApprovalRevocationPolicy`;
- `HighAssuranceApprovalDenialBehavior`;
- `HighAssuranceApprovalReportDisclosure`.

Privacy posture is modeled by:

- `WorkReportSensitivity`;
- `WorkReportRedactionPolicy`;
- validated `RedactionMetadata`.

Future write and side-effect terms are vocabulary only. Their presence in the model does not authorize writes or side-effect execution.

## 5. Validation Boundary Summary

Validation ensures:

- control ID is present, bounded, character-limited, and not secret-like;
- control version is present, bounded, character-limited, and not secret-like;
- schema version is not secret-like;
- protected action list is non-empty and unique;
- minimum approval count is nonzero;
- required reference list is non-empty;
- required reference names are valid and unique;
- report disclosure list is non-empty and unique;
- redaction metadata field names and reasons are present, bounded, and not secret-like;
- deserialized controls pass the same constructor validation as in-memory controls.

Validation errors use stable `high_assurance_approval.*` codes and do not include raw IDs, raw reference values, redaction values, secret-like values, provider payloads, command output, parser payloads, spec snippets, paths, credentials, tokens, authorization headers, or private keys.

## 6. Privacy And Redaction Summary

The model is reference-only.

It stores stable references to existing governance records, such as EvidenceReference IDs, policy decision events, SideEffect IDs, validation references, local check result IDs, workflow events, audit events, WorkReport IDs, and adapter telemetry references.

It does not store:

- raw provider payloads;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw command output;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

`Debug` output redacts IDs, reference values, and redaction metadata. Serialization can carry valid stable references because this is a model type, but invalid or secret-like redaction metadata fails closed during construction and deserialization.

## 7. Test Coverage Summary

Added focused tests for:

- valid minimal high-assurance approval control;
- invalid control ID;
- invalid version;
- secret-like schema version rejection;
- empty protected action rejection;
- duplicate protected action rejection;
- zero approval count rejection;
- empty required references rejection;
- duplicate required reference names rejection;
- all required reference target vocabulary;
- future write vocabulary remaining model-only;
- requester/approver separation rule vocabulary;
- expiration and revocation policy vocabulary;
- non-empty and unique report disclosures;
- all report disclosure vocabulary;
- serde round trip for valid controls;
- invalid serialized controls failing closed;
- secret-like redaction metadata rejection without leakage;
- redaction-safe `Debug`;
- serialization avoiding forbidden raw payload field markers.

Existing approval, SideEffect, WorkReport, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests remain covered by the workspace test suite.

## 8. Commands Run And Results

Commands run:

- `cargo fmt --all` - passed
- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test -p workflow-core --test high_assurance_approval` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed

## 9. Remaining Known Limitations

- The model is not wired into runtime approval enforcement.
- Existing approval requests and decisions do not enforce requester/approver separation.
- Existing approval flows do not enforce evidence sufficiency, role authority, quorum, revocation, or expiration-at-use semantics.
- Existing report generation does not automatically disclose high-assurance approval requirements.
- Existing executor paths do not consume high-assurance approval controls.
- Write-capable adapters remain unsupported.
- Runtime side-effect execution remains unsupported.

## 10. Recommended Next Phase

Recommended next phase: **High-assurance approval control core model review**.

After review, the next implementation work should connect already-built primitives into explicit runtime paths. High-assurance runtime enforcement should remain scoped and opt-in until approval identity, evidence sufficiency, side-effect linkage, report disclosure, and failure behavior are reviewed together.
