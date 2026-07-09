# Approval Gate Presentation Core Model Report

## 1. Executive Summary

The first approval gate presentation proof slice is implemented as a
model/helper-only boundary.

Workflow OS can now model a bounded `ApprovalPresentationRecord`, compute and
validate a deterministic `ApprovalPresentationContentHash`, and validate a
presentation record against a supplied runtime `ApprovalRequest` identity.

This closes the first model gap but does not yet enforce presentation proof in
runtime approval decisions.

## 2. Scope Completed

- Added `ApprovalPresentationRecord`.
- Added `ApprovalPresentationId`.
- Added `ApprovalPresentationContentHash`.
- Added `ApprovalPresentationChannel`.
- Added `ApprovalPresentationSensitivity`.
- Added `ApprovalPresentationRecordDefinition`.
- Added `ApprovalPresentationValidationInput`.
- Added `compute_approval_presentation_content_hash(...)`.
- Added `validate_approval_presentation_for_request(...)`.
- Added focused approval-presentation model tests.
- Updated approval-presentation planning and gap docs.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- runtime approval-presentation enforcement;
- changes to default approval behavior;
- automatic approval;
- hidden approval;
- approval UI or hosted approval cards;
- local presentation record persistence;
- CLI mutation behavior;
- workflow schema fields;
- examples;
- high-assurance approval integration;
- provider writes;
- side effects;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Model Types Added

`ApprovalPresentationRecord` captures:

- presentation ID;
- run ID;
- approval ID;
- workflow ID;
- optional workflow version;
- optional schema version;
- optional step ID;
- requested action;
- work summary;
- approved scope;
- strict non-goals;
- expected touched surfaces;
- validation expectations;
- why-now context;
- next action;
- presented-at timestamp;
- presented-by actor;
- presentation channel;
- content hash;
- redaction metadata;
- sensitivity.

`ApprovalPresentationContentHash` is a lowercase SHA-256 hash over canonical
approval-presentation fields. It is deterministic and changes when bounded scope
content changes.

## 5. Validation Boundary Summary

Validation rejects:

- empty or malformed presentation IDs;
- empty, malformed, too-long, or secret-like approval IDs;
- missing or unbounded requested action, work summary, approved scope, why-now
  context, and next action;
- empty, too-large, duplicated, or secret-like non-goal/surface/check lists;
- invalid custom presentation channel text;
- invalid content hash shape;
- content hash mismatch;
- invalid or secret-like redaction metadata;
- request identity mismatch for run ID, approval ID, workflow ID, workflow
  version, schema version, or step ID.

All validation errors use stable codes and avoid echoing rejected raw values.

## 6. Redaction And Privacy Summary

The model stores bounded summaries and stable identifiers only. It does not
store raw chats, screenshots, provider payloads, command output, CI logs, source
contents, spec contents, environment variables, credentials, authorization
headers, private keys, or token-like values.

`Debug` output redacts IDs, work summaries, approved scope, actors, and redaction
metadata values. Serialization preserves the validated bounded model shape and
deserialization re-runs validation, including secret-like value checks and hash
matching.

## 7. Test Coverage Summary

Tests cover:

- valid minimal approval-presentation record;
- channel and sensitivity vocabulary;
- deterministic content hashes;
- changed scope changes content hash;
- mismatched content hash rejection;
- request identity validation success;
- approval ID mismatch rejection without value leakage;
- duplicate collection entry rejection;
- secret-like presentation text rejection without leakage;
- redaction metadata validation and Debug safety;
- serde round trip;
- invalid serialized record fail-closed behavior without secret leakage.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test approval_presentation` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- Approval decisions do not yet require presentation proof.
- Presentation records are not persisted.
- No runtime event or audit projection exists for presentation proof.
- No CLI approval card rendering exists.
- No high-assurance approval integration exists.
- No WorkReport citation/disclosure integration exists.

## 10. Recommended Next Phase

Recommended next phase: approval gate presentation core model review.

The review should verify model scope, validation, redaction safety, serde
behavior, tests, and preservation of default runtime approval behavior before
any persistence or executor enforcement phase begins.
