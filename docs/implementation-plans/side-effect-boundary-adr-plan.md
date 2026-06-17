# Side-Effect Boundary ADR Plan

Status: ADR accepted. [ADR 0011: Side-Effect Boundary Core Model](../adr/0011-side-effect-boundary.md) is accepted as architecture direction, and the model-only SideEffect core model is implemented in [SideEffect Core Model Report](../concepts/SIDE_EFFECT_CORE_MODEL_REPORT.md). This plan and ADR do not implement writes, write-capable adapters, generic runtime adapter execution, schemas, CLI behavior, persistence changes, report artifact changes, domain packs, hosted runtime behavior, or release posture changes.

## 1. Executive Summary

Workflow OS needs a first-class side-effect boundary before write-capable integrations are considered.

The kernel already has policy, audit, idempotency, redaction, event sourcing, approvals, read-only adapter contracts, EvidenceReference, and WorkReport foundations. It does not yet have a domain-neutral record for a proposed, authorized, attempted, completed, denied, skipped, or failed mutation.

The next step should be a narrow ADR for a model-only side-effect boundary, not implementation of writes.

## 2. Goals

- Define why a side-effect boundary is required before writes.
- Keep write authority explicit and fail-closed.
- Separate authority from lifecycle state.
- Preserve adapter boundaries.
- Define idempotency and audit/report/evidence interactions.
- Prevent WorkReport prose or adapter telemetry from becoming the only record of mutation intent.

## 3. Non-Goals

This plan does not authorize:

- implementing writes;
- enabling GitHub, Jira, CI, local, or adapter writes;
- branch creation, PR creation, comments, issue updates, CI reruns, dispatch, or status writes;
- generic live adapter execution;
- schemas;
- CLI commands or rendering;
- persistence changes;
- report artifact changes;
- domain packs;
- rollback or compensation claims;
- production SIEM, DLP, access control, OAuth, webhooks, distributed workers, or hosted behavior.

## 4. Current Baseline

Implemented:

- conservative policy with denied unknowns and denied external writes;
- append-only workflow events;
- policy audit records;
- approval waits and decisions;
- idempotency store contract;
- read-only adapter posture;
- EvidenceReference model and selected attachment;
- WorkReport side-effects section vocabulary.

Not implemented:

- side-effect boundary model;
- side-effect event kinds or projections;
- write-capable adapter execution;
- generic runtime adapter execution;
- approval evidence attachment for writes;
- side-effect persistence, CLI, schemas, or examples.

## 5. Candidate Model Concepts

A future ADR should evaluate a domain-neutral model such as:

- `SideEffectRecord`;
- `SideEffectId`;
- `SideEffectTargetReference`;
- `SideEffectCapability`;
- `SideEffectAuthority`;
- `SideEffectLifecycleState`;
- `SideEffectIdempotencyBinding`;
- `SideEffectOutcomeReference`;
- `SideEffectEvidenceReference`;
- `SideEffectPolicyReference`;
- `SideEffectApprovalReference`.

Candidate lifecycle vocabulary:

- proposed;
- attempted;
- completed;
- denied;
- skipped;
- failed;
- rolled back, only as future vocabulary when adapter-specific compensation is proven.

The ADR should decide whether `approved` is lifecycle state or authority. Recommended direction: keep authority and lifecycle state separate.

## 6. Authority And Policy Rules

- Credentials must never imply permission.
- Unknown capabilities fail closed.
- Missing policy context fails closed.
- Denied policy cannot be bypassed by adapter precheck, retry, CLI flag, restart, or report generation.
- Sensitive or ambiguous side effects require human approval.
- Kill switch denies new side-effect proposals or attempts.

## 7. Idempotency Rules

Side-effecting operations should require an idempotency key before attempt.

Duplicate idempotency keys should return a prior non-secret result reference and must not re-attempt mutation.

Idempotency results must reference records or external object IDs, not raw payloads.

## 8. Audit, Evidence, And Report Interactions

- Workflow events remain run-state source of truth.
- Audit events remain governance projections.
- Side-effect records should be citeable from audit and reports.
- EvidenceReference should cite decision inputs and adapter summaries by reference only.
- WorkReport side-effect sections should cite side-effect records, policy decisions, approval decisions, audit events, adapter telemetry, and evidence references.
- Denied, skipped, and failed side effects should be explicit, not absent.

## 9. Privacy And Redaction

Side-effect records must not store:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment values;
- credentials, authorization headers, private keys, or token-like values.

Debug, Display, serialization, and errors must be redaction-safe.

## 10. Test Requirements For A Future Model Phase

Future tests should cover:

- unknown side-effect action/capability fails closed;
- external writes remain denied until separately implemented;
- missing actor, workflow identity, authority, or idempotency key fails validation;
- lifecycle states serialize and deserialize safely;
- denied policy cannot be bypassed;
- duplicate idempotency key returns prior result reference without re-attempt;
- redaction-safe Debug/Display/serialization;
- secret-like targets, summaries, metadata, and provider payloads are rejected or redacted;
- WorkReport citations can reference side-effect records without copying payloads;
- rollback vocabulary cannot imply support without adapter-specific proof.

## 11. ADR Questions

- Should side-effect records be workflow events, separate records, or both?
- What is the minimal lifecycle vocabulary?
- How should authority bind policy decisions and approvals?
- How should idempotency bind to side-effect outcomes?
- Should WorkReport citation target vocabulary add side-effect records before writes exist?
- Which evidence attachment rules are required before write adapters?
- What must remain in core versus adapter-specific domains?

## 12. Final Recommendation

Create a narrow Side-Effect Boundary Core Model ADR before any write implementation.

The ADR is accepted in [ADR 0011](../adr/0011-side-effect-boundary.md), and the model-only implementation is documented in [SideEffect Core Model Report](../concepts/SIDE_EFFECT_CORE_MODEL_REPORT.md). The recommended next phase is SideEffect core model review. Future phases must not enable write-capable adapters, generic adapter execution, schemas, CLI, examples, or provider mutations unless separately scoped and reviewed.
