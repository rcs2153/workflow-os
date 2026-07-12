# Immutable Run Bundle Boundary Plan Report

Report date: 2026-07-12

## 1. Executive Summary

The immutable run-bundle boundary is planned as the next load-bearing runtime
hardening phase after approval/resume context integrity. The plan separates
durable declarative inspection from executable replay and defines a manifest,
content-addressed canonical definition records, bounded execution posture, and
explicit unattested-handler posture.

## 2. Planning Scope Completed

- Defined source-of-truth boundaries among events, snapshots, bundles, Git,
  approvals, authority receipts, and WorkReports.
- Defined candidate manifest and definition-reference types.
- Defined required run identity and execution-posture fields.
- Defined canonical validated definition records instead of raw YAML archives.
- Defined deterministic root hashing and local create-only persistence order.
- Defined backward-readable legacy run posture.
- Defined inspection-only initial semantics and explicit replay limitations.
- Defined privacy, error, test, and phased implementation requirements.

## 3. Scope Explicitly Not Completed

No Rust model, persistence, executor integration, replay, handler/check
attestation, provider mutation, automatic resume, CLI, schema, SDK, example,
hosted runtime, RBAC, signing, reasoning lineage, or release behavior was added.

## 4. Key Decision

A bundle cannot be only another aggregate hash. The plan requires a payload-free
manifest that references content-addressed canonical validated definition
records. This lets Workflow OS inspect exact historical declarative inputs after
working-tree changes while avoiding raw-source archival.

The first bundle remains non-executable because handler implementations,
external inputs, credentials, provider state, and SideEffect reconciliation are
not attested or preserved.

## 5. Relationship To External Feedback

External dogfood testing correctly identified immutable run bundles, real check
attestation, actor enforcement, artifact capture, and machine-readable reporting
as important maturity gaps. This plan addresses the first dependency and keeps
the others ordered:

1. immutable declarative run bundle;
2. handler/check attestation;
3. scoped actor and capability authority;
4. governed resume and broader mutation only after those boundaries hold.

## 6. Recommended Implementation Sequence

Begin with core manifest/reference model types, validation, deterministic
explicit-label hashing, serde, redaction-safe Debug, and tests only. Review that
model before canonical-record storage or executor integration.

## 7. Governed Planning Evidence

- Workflow: `dg/d`.
- Run: `run-1783875199591477000-2`.
- Approval: `approval/run-1783875199591477000-2/planning-approved`.
- Presentation: `presentation/4713911c1b242bf3`.
- Approval outcome: granted through the proof-enforced presentation path.
- Kernel boundary: planning governance only; Codex inspected the repository and
  authored docs outside the kernel.

## 8. Validation

- `npm run check:docs` required.
- `git diff --check` required.
- Scope audit must confirm no runtime code, schema, CLI, example, provider, or
  release changes.

## 9. Recommended Next Phase

Perform a focused maintainer review of the immutable run-bundle boundary plan.
If accepted, implement the core manifest and reference model only.
