# ADR 0011: Side-Effect Boundary Core Model

## Status

Proposed

## Status Change Criteria

This ADR should not move from `Proposed` to `Accepted` until:

- maintainers review the side-effect lifecycle vocabulary;
- maintainers confirm that authority and lifecycle state remain separate;
- privacy and redaction implications are reviewed;
- the relationship to policy decisions, approval decisions, audit events, EvidenceReference, WorkReport, adapter telemetry, and idempotency is accepted;
- a model-only implementation plan is reviewed and scoped;
- acceptance preserves the Workflow OS product boundary;
- acceptance does not itself authorize writes, write-capable adapters, generic runtime adapter execution, schemas, CLI behavior, persistence changes, report artifact changes, hosted runtime behavior, or release posture changes.

## Context

Workflow OS now has the core primitives needed to discuss side effects seriously:

- local deterministic workflow execution;
- governed multi-step local execution;
- append-only workflow events;
- policy decisions before meaningful actions;
- approval requests and decisions;
- idempotency keys for execution boundaries;
- read-only adapter contracts and telemetry;
- EvidenceReference model and selected attachment points;
- WorkReport and report artifact foundations;
- local-check result references and report citations.

The kernel still intentionally denies external writes. Phase 2 read-only adapters can inspect provider state, but they cannot create branches, open pull requests, update Jira, rerun CI, dispatch workflows, or mutate external systems.

That denial is correct until Workflow OS has a domain-neutral way to represent mutation intent, authority, attempt, outcome, denial, skipping, failure, idempotency, auditability, evidence, and reportability.

Without a first-class side-effect boundary, future writes could be represented only as adapter telemetry, prose in a WorkReport, or provider-specific return values. That would be too weak: write authority must be explicit, policy-gated, approval-aware, idempotent, auditable, and citeable.

## Decision

Workflow OS should add a domain-neutral side-effect boundary core model in a future model-only implementation phase.

The side-effect boundary should represent mutation lifecycle and authority without enabling mutation by itself.

Core concepts should remain domain-neutral and should not encode GitHub, Jira, CI, local filesystem, or other provider-specific write semantics directly. Provider-specific details belong in adapters, skills, examples, or future domain packs.

The model should support, at minimum:

- stable side-effect identity;
- target reference;
- requested capability;
- lifecycle state;
- actor or system actor;
- workflow and run identity;
- step and skill identity when available;
- policy decision references;
- approval decision references;
- idempotency binding;
- adapter telemetry or outcome references;
- evidence references;
- audit event references;
- WorkReport citation compatibility;
- sensitivity and redaction metadata.

The first implementation should be model-only: types, validation, serialization/deserialization, redaction-safe Debug behavior, and tests. It must not attempt provider writes.

## Authority Versus Lifecycle

Authority and lifecycle state must remain separate.

Recommended direction:

- authority answers whether the side effect is allowed to proceed under policy, approval, capability, actor, and runtime conditions;
- lifecycle state answers where the side effect is in its mutation timeline.

`approved` should not be the primary lifecycle state in v1. Approval is authority context that may permit an attempted side effect. A side effect can be authorized and still be skipped, fail before attempt, fail during attempt, or complete.

This avoids treating approval as equivalent to execution.

## Candidate Lifecycle Vocabulary

The proposed minimal lifecycle vocabulary is:

- `proposed`: a side effect has been suggested, planned, or requested but not attempted;
- `attempted`: the runtime or adapter attempted the side effect;
- `completed`: the side effect completed successfully;
- `denied`: policy, approval, capability, kill switch, validation, or safety checks blocked the side effect;
- `skipped`: the workflow intentionally did not attempt the side effect;
- `failed`: the side effect was attempted and did not complete successfully.

`rolled_back` should remain future vocabulary only. Workflow OS must not imply rollback or compensation support unless a specific adapter operation can prove that behavior honestly.

## Authority And Policy Rules

Side-effect authority must be explicit and fail closed.

Required rules:

- credentials never imply permission;
- unknown capabilities fail closed;
- missing policy context fails closed;
- external writes remain denied until separately implemented;
- denied policy cannot be bypassed by adapter precheck, retry, CLI flag, worker restart, report generation, or SDK call;
- sensitive or ambiguous side effects require human approval;
- kill switch denies new side-effect proposals and attempts;
- adapters may request side effects only through explicit core interfaces;
- adapters must not mutate core workflow state directly.

## Idempotency Rules

Side-effecting operations should require an idempotency key before any attempt.

Duplicate idempotency keys must not re-attempt mutation. They should return or cite a prior non-secret side-effect record or outcome reference.

Idempotency results should reference records, external object IDs, provider operation references, or adapter telemetry IDs. They must not store raw provider payloads, raw command output, credentials, or secret-like values.

## Source-Of-Truth Boundaries

Workflow events remain the source of truth for run state.

Audit events remain governance and operational projections.

Adapter telemetry remains the record of adapter invocation and redacted adapter outcome summaries.

EvidenceReference remains the citation substrate for evidence pointers, not a raw evidence store.

WorkReport remains a governed handoff artifact that explains work performed, decisions made, checks run, approvals, side effects, risks, limitations, and incomplete work.

A future side-effect record should be the source of truth for side-effect intent and lifecycle status. It should be citeable from audit events, WorkReports, evidence references, and adapter telemetry, but it should not replace workflow events or audit events.

## Audit, Evidence, And Report Interactions

Side-effect records should be citeable by:

- audit events;
- WorkReport citations;
- EvidenceReference values;
- adapter telemetry records;
- policy decision records;
- approval decision records;
- future reasoning lineage or claim graph nodes, if separately accepted.

WorkReport side-effect sections should cite side-effect records and disclose proposed, denied, skipped, attempted, completed, and failed side effects explicitly. Absence of a side-effect record must not be used to imply safety, completion, or non-occurrence.

EvidenceReference should cite decision inputs and redacted outcome summaries by reference only. Side-effect modeling must not turn EvidenceReference into raw provider payload storage.

## Privacy And Redaction

Side-effect records must not store:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue bodies or comments;
- raw GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded summaries;
- secret-like target identifiers or metadata.

Debug, Display, serialization, deserialization errors, validation errors, audit projection, and report citation behavior must be redaction-safe and bounded.

## Non-Goals

This ADR does not authorize:

- implementing writes;
- enabling GitHub, Jira, CI, local filesystem, or provider mutation;
- branch creation;
- pull request creation;
- pull request comments;
- issue updates;
- CI reruns;
- workflow dispatch;
- generic live adapter execution;
- runtime adapter write routing;
- schema fields;
- CLI commands or rendering;
- example updates;
- persistence changes;
- automatic report artifact changes;
- domain packs;
- hosted or distributed runtime behavior;
- production SIEM, DLP, access control, OAuth, or webhook behavior;
- rollback or compensation claims;
- Level 3/4 autonomy enablement;
- release posture changes.

## Consequences

Positive consequences:

- Write-capable adapter work gets a safer prerequisite boundary.
- Workflow OS can distinguish proposal, authorization, attempt, completion, denial, skipping, and failure.
- WorkReports can cite side effects without becoming the source of truth for mutation history.
- Audit and evidence models can cite mutation intent and outcomes by stable reference.
- Idempotency semantics can be modeled before provider writes exist.
- The kernel avoids smuggling write behavior through adapter telemetry or report prose.

Tradeoffs:

- The model adds another governed record type that must be integrated carefully with workflow events, audit events, adapter telemetry, evidence references, and reports.
- The side-effect lifecycle could become noisy if every harmless internal operation is modeled as a side effect.
- Future persistence, schema, CLI, and adapter integration phases must preserve compatibility once the model is public.
- Rollback and compensation must stay out of v1 unless adapter-specific proof exists.

## Alternatives Considered

1. Treat side effects as adapter telemetry only.
   Rejected because telemetry records invocation behavior, not durable authority, lifecycle, idempotency, and governance semantics.

2. Treat side effects as WorkReport prose only.
   Rejected because reports are handoff artifacts, not mutation source-of-truth records.

3. Treat approval decisions as side-effect lifecycle states.
   Rejected because approval is authority context. Approval does not mean an operation was attempted or completed.

4. Implement provider writes first and model side effects afterward.
   Rejected because it would make the riskiest behavior arrive before the governance boundary.

5. Make the first model GitHub-specific.
   Rejected because core must remain domain-neutral across software engineering, legal, finance, HR, security, procurement, support, operations, and data/analytics.

## Implementation Timing

The next implementation phase should be SideEffect core model only.

That phase should add domain-neutral Rust model types, deterministic validation, serde support, redaction-safe Debug behavior, and focused tests. It should not add writes, adapters, schemas, CLI behavior, persistence changes, examples, automatic report artifact behavior, or release posture changes.

Recommended first implementation target:

- `SideEffectRecord`;
- `SideEffectId`;
- `SideEffectLifecycleState`;
- `SideEffectTargetReference`;
- `SideEffectCapability`;
- `SideEffectAuthority`;
- `SideEffectIdempotencyBinding`;
- references to policy, approval, audit, adapter telemetry, evidence, and report citation targets where stable identifiers already exist.

## Explicit Implementation Statement

No runtime feature is implemented by this ADR. No side-effect model code, schema field, CLI behavior, persistence table, adapter write path, provider mutation, example, hosted runtime behavior, or release posture change is added. Side-effect boundary modeling requires a separately scoped implementation phase after this ADR is reviewed and accepted.
