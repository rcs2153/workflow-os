# EvidenceReference Adapter Telemetry Attachment Fix

Fix date: 2026-06-04

## Blocker

The adapter telemetry attachment review found that `AdapterInvocationRecord.evidence_references` and `AdapterRuntimeAuditRecord.evidence_references` were public `Vec<EvidenceReference>` fields. A caller could push invalid or unsanitized evidence directly into those vectors and bypass the validated attachment APIs.

## Fix Implemented

The attached evidence collections are now private on:

- `AdapterInvocationRecord`
- `AdapterRuntimeAuditRecord`

Both records expose read-only `evidence_references()` accessors that return slices. Evidence can be added only through validated APIs:

- `attach_evidence_reference`
- `attach_evidence_references`
- `with_evidence_references`

The records continue to serialize evidence under the `evidence_references` field. Deserialization validates and sanitizes attached evidence references before constructing a record, so invalid evidence-bearing payloads fail instead of silently entering adapter telemetry.

`AdapterRuntimeAuditRecord::from_invocation` still preserves invocation evidence references. Because adapter invocation evidence is now private and deserialization validates attached evidence, normal construction paths cannot bypass the attachment boundary.

## Scope Preserved

This fix does not add:

- local persistence changes;
- CLI inspection changes;
- example integration;
- validation diagnostic/result attachment;
- approval request/decision attachment;
- work reports;
- reasoning lineage;
- side-effect boundary modeling;
- writes;
- domain packs;
- schema exposure;
- generic live adapter execution;
- production evidence storage;
- DLP or access-control systems;
- release posture changes.

## Validation

Validation performed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run check:integrations`

## Remaining Non-Blocking Follow-Ups

- Add fuller parent/child identity assertions before persistence.
- Decide whether a validated evidence collection newtype is useful before schema exposure.
- Keep observability evidence attachment deferred unless a concrete operator need appears.
- Proceed to validation diagnostics/results attachment planning only after this fix remains green in the full gate.

