# ADR 0009: EvidenceReference Core Model

## Status

Proposed

## Context

ADR 0007 accepts Governed Work Pattern as Workflow OS architecture and product direction. That direction says governed enterprise work needs explicit context, evidence, policy gates, approvals, validation, audit, observability, side-effect boundaries, and structured reporting.

WorkReportContract, terminal WorkReport artifacts, and Reasoning Lineage / Claim Graph remain future scoped work. Before Workflow OS can implement work reports or reasoning lineage honestly, the kernel needs a safe, domain-neutral way to point at evidence used by decisions, approvals, validation, adapter reads, audit events, reports, and future provenance artifacts.

Workflow OS already has related records:

- workflow events for run state;
- audit events for operational history;
- adapter telemetry records for read-only integration attempts;
- validation diagnostics and contract checks;
- policy and approval decisions;
- CLI status and inspect output.

Those records are useful, but there is no single core model for saying: “this specific piece of evidence was cited by this decision, approval, validation result, adapter read, audit projection, work report section, or future claim.”

Without such a model, future work reports risk copying raw sensitive payloads, domain packs risk inventing incompatible evidence shapes, and reasoning lineage would lack a safe reference substrate.

## Decision

Workflow OS should add `EvidenceReference` as a core model in a future implementation.

This ADR proposes the model and implementation plan only. No code, schema, CLI behavior, runtime feature, persistence table, work report, reasoning lineage model, write support, or domain pack is implemented by this ADR. Implementation requires a separate implementation prompt or pull request.

`EvidenceReference` should be:

- **reference-first**: point to evidence by stable internal reference, provider reference, local file path, content hash, event ID, or summarized command/result reference;
- **redacted**: store summaries and redaction metadata, not raw sensitive payloads by default;
- **domain-neutral**: use terms that work across legal, finance, HR, security, procurement, support, operations, data/analytics, and software engineering;
- **safe by default**: treat provider metadata and personal data as sensitive unless explicitly classified as safe.

Raw sensitive payloads must not be stored by default.

## Candidate Core Model

The proposed minimum v1 `EvidenceReference` fields are:

| Field | Purpose |
| --- | --- |
| `id` | Stable evidence reference ID. |
| `kind` | Domain-neutral evidence kind. |
| `title` | Short human-readable label. |
| `uri_or_reference` | Non-secret pointer, internal object reference, local path, provider object reference, or content-addressed reference. |
| `source_component` | Component that created the reference, such as `validator`, `runtime`, `adapter`, `cli`, `operator`, `skill`, or `release_review`. |
| `scope` | Scope such as project, workflow, run, step, skill, adapter, audit, event, approval, validation, or external. |
| `workflow_id` | Workflow ID when run-scoped. |
| `workflow_version` | Workflow version when run-scoped. |
| `schema_version` | Schema version when run-scoped. |
| `spec_hash` | Spec content hash when run-scoped. |
| `run_id` | Workflow run ID when run-scoped. |
| `step_id` | Step ID when step-scoped. |
| `skill_id` | Skill ID when skill-scoped. |
| `skill_version` | Skill version when skill-scoped. |
| `adapter_id` | Adapter ID when adapter-scoped. |
| `adapter_kind` | Adapter kind when adapter-scoped. |
| `audit_event_id` | Audit event ID when audit-scoped. |
| `workflow_event_id` | Workflow event ID when event-scoped. |
| `approval_id` | Approval request or decision reference when approval-scoped. |
| `validation_result_id` | Validation result or diagnostic reference when validation-scoped. |
| `correlation_id` | Correlation ID when available. |
| `actor` | Human actor when available. |
| `system_actor` | System actor when available. |
| `created_at` | Creation timestamp. |
| `summary` | Optional redacted summary. |
| `content_hash` | Content hash when available and safe. |
| `provider_etag_or_version` | Provider ETag, version, revision, or equivalent when available and safe. |
| `redaction_metadata` | How sensitive fields were omitted, summarized, or redacted. |
| `sensitivity` | Sensitivity classification, defaulted conservatively when unknown. |
| `retention_hint` | Optional retention guidance such as short-lived, audit-retained, report-retained, or external-only. |
| `metadata` | Non-secret extension metadata for narrow needs. |

The future implementation should prefer typed fields for common relationships and keep `metadata` small, non-secret, and explicitly documented.

## Candidate `kind` Taxonomy

The proposed v1 taxonomy is:

- `local_file`
- `spec_file`
- `validation_result`
- `workflow_event`
- `audit_event`
- `adapter_invocation`
- `adapter_response_summary`
- `approval_decision`
- `policy_decision`
- `operator_note`
- `external_reference`
- `test_result`
- `command_output`
- `release_review`
- `live_smoke_evidence`

This taxonomy is a v1 starting point and may evolve through later ADRs or compatibility-reviewed changes.

## What Must Not Be Stored

`EvidenceReference` must not store these values by default:

- provider tokens;
- authorization headers;
- private keys;
- environment variable values;
- raw CI logs;
- raw Jira descriptions or comments;
- raw large GitHub file contents;
- raw provider payloads;
- unredacted personal data;
- secrets copied from specs, shells, logs, or screenshots.

If a future implementation needs to retain a payload, that payload storage must be separately designed, capability-gated, policy-reviewed, redacted, tested, and documented. It must not be smuggled into `EvidenceReference`.

## Relationship To Existing Concepts

- **Workflow events**: may be cited by `EvidenceReference`, but remain the source of truth for run state.
- **Audit events**: may cite or be cited by `EvidenceReference`, but remain the source of truth for operational history.
- **Adapter telemetry**: should be able to produce evidence references from redacted adapter invocation records, response summaries, provider object references, and live smoke evidence.
- **Validation diagnostics/results**: may become evidence references for workflow qualification, report sections, or approval packets.
- **Approval decisions**: may cite evidence references used by an approver or generated for the approval packet.
- **Policy decisions**: may cite evidence references when policy evaluation depends on specific input or validation evidence.
- **Local state**: may persist evidence references later, but this ADR does not choose storage mechanics.
- **CLI inspect/status**: may display redacted evidence summaries later, but this ADR does not add CLI behavior.
- **Work reports**: should cite evidence references instead of copying raw evidence payloads. Core work-report models and explicit local helper/artifact APIs now exist through later scoped phases.
- **Future Reasoning Lineage / Claim Graph**: may link claims, findings, corrections, and confidence metadata to evidence references.

## Source-Of-Truth Boundaries

`EvidenceReference` is the source of truth for the existence of a cited evidence pointer. It is not the source of truth for the full underlying evidence payload.

Provider systems remain the source of truth for provider data. Local files remain the source of truth for local file contents. The workflow event stream remains the source of truth for run state. Audit events remain the source of truth for low-level operational history. Work reports cite `EvidenceReference` values and explain how evidence was used; they do not become the source of truth for underlying provider or local payloads.

An evidence reference may become stale, inaccessible, superseded, redacted, deleted from the provider, or no longer visible to the current user. Future access-control and retention work must account for that.

## Privacy And Redaction

The privacy posture is:

- prefer reference over copy;
- prefer summary over payload;
- require redaction metadata;
- require sensitivity classification or conservative defaulting;
- treat provider metadata as sensitive unless explicitly safe;
- ensure Debug, Display, logs, audit projections, and CLI output do not leak sensitive content;
- avoid raw provider payloads by default;
- acknowledge that an evidence reference may point to evidence the current user can no longer read.

`EvidenceReference` is not enterprise DLP. It is a structured reference model that must cooperate with redaction, sensitivity, retention, and future access-control decisions.

## Implementation Timing

Implement `EvidenceReference` before:

- `WorkReportContract`;
- terminal `WorkReport` artifacts;
- Reasoning Lineage / Claim Graph;
- side-effect boundary modeling;
- policy-gated writes.

The first implementation should be small and reviewable. It should introduce the core type model and tests before persistence, CLI display, reports, or domain templates.

## Non-Goals

This ADR does not:

- implement `EvidenceReference`;
- add schemas;
- add CLI behavior;
- add work reports;
- add reasoning lineage;
- add writes;
- store raw evidence payloads;
- implement DLP;
- implement access control;
- implement provider fetch or replay;
- implement a production evidence store;
- add generic live adapter execution;
- add domain packs.

## Consequences

Benefits:

- Gives Workflow OS a safe, shared evidence vocabulary before reports or lineage.
- Reduces pressure to copy raw provider payloads into audit events, reports, examples, or future domain packs.
- Gives approvals, adapter telemetry, validation results, and future work reports a common citation model.
- Keeps the model domain-neutral across enterprise workflows.
- Creates a clearer prerequisite for write-capable adapter design.

Tradeoffs:

- Adds a new core concept that must be kept small and precise.
- Requires careful privacy, redaction, and display behavior.
- Creates compatibility surface once serialized or persisted.
- Requires future decisions about local persistence and CLI presentation.

Risks:

- `metadata` could become a dumping ground if not constrained.
- Teams may overread evidence references as proof that underlying provider content is still available or trustworthy.
- Poor summaries could leak sensitive content.
- Storing too much evidence could duplicate audit logs or provider payloads.
- Storing too little could make reports and approvals hard to review.

## Alternatives Considered

1. **Store evidence directly in audit events.**
   Rejected because audit events are low-level operational history. Evidence used by decisions, reports, and future claims needs to be reusable without turning audit logs into payload storage.

2. **Make evidence domain-specific.**
   Rejected because legal clauses, finance exceptions, Jira issues, CI failures, support cases, and local validation results need one domain-neutral reference layer.

3. **Wait until work reports exist.**
   Rejected because work reports should cite evidence references rather than define evidence from scratch.

4. **Put evidence only in domain packs.**
   Rejected because evidence citation is cross-domain governance infrastructure, not a domain template feature.

5. **Use free-form metadata only.**
   Rejected because free-form metadata would make redaction, source-of-truth boundaries, search, validation, and future report/lineage links inconsistent.

## Explicit Implementation Statement

No runtime feature is implemented by this ADR. `EvidenceReference` is proposed as future core model work only.
