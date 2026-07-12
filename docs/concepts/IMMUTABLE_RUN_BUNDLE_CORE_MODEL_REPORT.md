# Immutable Run Bundle Core Model Report

Report date: 2026-07-12

## 1. Executive Summary

The immutable run-bundle core manifest/reference model is implemented. It
provides validated, domain-neutral vocabulary for binding immutable run identity
to content-addressed workflow, resolved skill, and referenced policy records,
plus bounded execution and handler posture.

The model computes a versioned field-based root hash without relying on Debug or
storage serialization. It does not persist bundles, integrate with the
executor, execute replay, or attest handlers.

## 2. Scope Completed

- Added validated bundle ID and version types.
- Added typed workflow, skill, and policy definition references.
- Added execution-input preservation posture.
- Added honest handler posture including mock and unattested states.
- Added conservative sensitivity and bounded redaction posture.
- Added private manifest fields, validated construction, read-only accessors,
  validated deserialization, deterministic canonical ordering, and root hashing.
- Exported the model from `workflow-core`.

## 3. Validation Boundary

Validation requires exactly one workflow reference, unique definition
references, step binding only for skill references, unique bounded checkpoint
references, bounded collections, and validated identifier types. The
constructor canonicalizes workflow-first, declared skill order, sorted policy
order, and sorted handler identity order.

Deserialization reconstructs and validates the manifest and rejects a supplied
root hash that differs from the computed value.

## 4. Root Hash

The `workflow-os/immutable-run-bundle/v1` root uses labeled, length-delimited
fields for bundle/run/workflow identity, workflow and resolved-context hashes,
definition references, execution posture, handler posture, creation identity,
sensitivity, and redaction posture. It does not use Debug formatting, map
iteration, platform paths, raw definitions, or storage encoding.

## 5. Privacy And Debug Posture

The model stores references and hashes only. It has no fields for raw YAML,
source contents, prompts, transcripts, command output, provider payloads,
environment values, or credentials. Manifest Debug shows counts and posture but
redacts bundle/run identity and the root hash. Validation and deserialization
errors use fixed non-echoing text.

## 6. Tests

Focused tests cover valid construction, deterministic roots, content-change
sensitivity, serde round trip, tampered root rejection, exactly-one-workflow
validation, duplicate references, skill-step posture, duplicate checkpoints,
canonical definition ordering, workflow-reference identity alignment,
validated execution-posture deserialization, duplicate and mismatched handler
references, safe nested-record Debug, non-echoing invalid IDs, explicit handler
posture, and forbidden raw-payload field absence.

## 7. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run: `run-1783875690054673000-2`.
- Approval:
  `approval/run-1783875690054673000-2/implementation-approved`.
- Presentation: `presentation/53bc48d334153fd5`.
- Approval outcome: granted through the proof-enforced presentation path.
- Event summary: 39 events, one approval, zero retries, zero escalations.
- Out-of-kernel work: Codex authored code, tests, and docs and ran validation;
  the kernel governed scope and approval only.

## 8. Explicitly Not Implemented

No canonical definition records, storage, persistence, executor/run-identity
integration, replay, resume, handler/check attestation, provider mutation, CLI,
schemas, examples, hosted runtime, RBAC, signing, or release changes were added.

## 9. Validation Commands

All required validation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

## 10. Recommended Next Phase

The focused maintainer review accepted the phase with non-blocking follow-ups.
Implement canonical validated definition-record types next, before any
persistence or executor integration.
