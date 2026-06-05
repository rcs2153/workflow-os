# EvidenceReference Proposal Review

Review date: 2026-06-04

Reviewed materials:

- `docs/adr/0009-evidence-reference-core-model.md`
- `docs/concepts/evidence-reference.md`
- `docs/implementation-plans/evidence-reference-mvp.md`
- ADR 0007 Governed Work Pattern and acceptance review
- ADR 0008 Reasoning Lineage / Claim Graph
- runtime event, state backend, adapter, security, redaction, user-guide, and release posture documentation

This review is architecture and scoping only. It does not implement code, schemas, CLI behavior, runtime features, writes, domain packs, work reports, reasoning lineage, generic runtime adapter execution, or release posture changes.

## 1. Executive Verdict

**Ready for implementation with revisions.**

`EvidenceReference` is the right next core MVP concept after Governed Work Pattern acceptance. It belongs in core because evidence citation is cross-domain governance infrastructure, not an adapter convenience, domain-pack feature, or report-only concern.

The proposal is strong enough to generate a scoped implementation prompt, provided that implementation starts with a small core type model and redaction tests only. Attachment points may follow once the core model is stable. Local persistence, CLI inspection, and example integration should remain separately reviewed follow-up phases.

## 2. Architecture Fit

EvidenceReference fits Workflow OS as a governed enterprise work kernel.

It supports the kernel by giving decisions, approvals, validation, audit projections, adapter telemetry, future work reports, and future reasoning lineage a shared way to cite evidence without becoming the evidence store itself.

Boundary assessment:

| Boundary | Assessment |
| --- | --- |
| Governed enterprise work kernel | Fits. It strengthens evidence, audit, approval, and reporting foundations. |
| BPM engine | Safe if kept as a reference model, not a process-routing or case-management layer. |
| Knowledge graph product | Safe if it remains citation metadata, not queryable enterprise graph storage. |
| Chat memory system | Safe. The proposal is actor/run/evidence oriented, not transcript-memory oriented. |
| Domain-specific automation system | Safe. The model is domain-neutral and avoids GitHub/Jira/CI names in core fields. |

The strongest architecture fit is the source-of-truth boundary: EvidenceReference records that evidence was cited; it does not replace workflow events, audit events, provider systems, or future reports.

## 3. Core-Model Decision

| Question | Decision |
| --- | --- |
| Does EvidenceReference belong in core? | Yes. It is cross-domain governance infrastructure. |
| Should it be implemented before WorkReportContract? | Yes. Reports should cite evidence references, not invent evidence shape. |
| Should it be implemented before Reasoning Lineage? | Yes. Lineage needs evidence links, but EvidenceReference should not wait for lineage. |
| Should it be implemented before side-effect boundary and writes? | Yes. Write-capable adapters need evidence-backed approvals and side-effect decisions. |

EvidenceReference should be implemented before WorkReportContract, terminal work reports, reasoning lineage, side-effect boundary modeling, and policy-gated writes.

## 4. Field Model Review

Recommended v1 field disposition:

| Field | Disposition | Review note |
| --- | --- | --- |
| `id` | Required | Stable ID is necessary for citations. |
| `kind` | Required | Needed for typed interpretation and validation. |
| `title` | Required | Needed for human review; must be non-secret and bounded. |
| `uri_or_reference` | Required with naming review | Concept is right. Implementation should consider `reference` or typed `reference_uri` plus `reference_id` to avoid a vague union. |
| `source_component` | Required | Important for provenance and review. |
| `scope` | Required | Needed to distinguish project, run, step, adapter, audit, validation, and external references. |
| `workflow_id` | Optional | Required only when run-scoped. |
| `workflow_version` | Optional | Required only when run-scoped. |
| `schema_version` | Optional | Required only when run-scoped. |
| `spec_hash` | Optional | Required only when run-scoped. |
| `run_id` | Optional | Required only when run-scoped. |
| `step_id` | Optional | Required only when step-scoped. |
| `skill_id` | Optional | Required only when skill-scoped. |
| `skill_version` | Optional | Required only when skill-scoped. |
| `adapter_id` | Optional | Required only when adapter-scoped. |
| `adapter_kind` | Optional | Required only when adapter-scoped. |
| `audit_event_id` | Optional | Required only when audit-scoped. |
| `workflow_event_id` | Optional | Required only when event-scoped. |
| `approval_id` | Optional | Required only when approval-scoped. |
| `validation_result_id` | Optional | Required only when validation-scoped. |
| `correlation_id` | Optional | Include whenever available. |
| `actor` | Optional | Use when human actor is known. |
| `system_actor` | Optional | Use when system actor is known. |
| `created_at` | Required | Required for auditability and freshness review. |
| `summary` | Optional | Must be redacted, bounded, and safe for Display/Debug. |
| `content_hash` | Optional | Useful for local files and command/test outputs; only when safe. |
| `provider_etag_or_version` | Optional | Useful for provider freshness, but can disclose metadata; treat as sensitive unless safe. |
| `redaction_metadata` | Required | Required even when it says reference-only/no payload stored. |
| `sensitivity` | Required or defaulted | Must default conservatively if not specified. |
| `retention_hint` | Optional | Useful but should not imply enforcement until retention behavior exists. |
| `metadata` | Optional with strict constraints | Keep bounded, non-secret, and small. Avoid turning it into a payload bag. |

Required revision for implementation prompt: define scope-specific validation rules. For example, adapter-scoped references must include adapter fields, run-scoped references must include immutable run identity, and validation-scoped references must include a validation diagnostic/result reference.

## 5. Kind Taxonomy Review

The proposed taxonomy is broad but acceptable for v1 if treated as an enum with compatibility review.

| Kind | Review |
| --- | --- |
| `local_file` | Safe and useful. |
| `spec_file` | Safe and useful; should prefer path plus content hash. |
| `validation_result` | Core v1 candidate. |
| `workflow_event` | Core v1 candidate. |
| `audit_event` | Core v1 candidate. |
| `adapter_invocation` | Core v1 candidate. |
| `adapter_response_summary` | Core v1 candidate; must not contain raw payload. |
| `approval_decision` | Core v1 candidate. |
| `policy_decision` | Core v1 candidate. |
| `operator_note` | Useful, but needs non-secret guidance. |
| `external_reference` | Necessary escape hatch, but high-risk; must be constrained and documented. |
| `test_result` | Useful for validation/release evidence. |
| `command_output` | Useful, but high-risk; only summaries/hashes, not raw logs by default. |
| `release_review` | Acceptable for repository governance evidence. |
| `live_smoke_evidence` | Acceptable for preview/release evidence; should remain non-provider-specific. |

Verdict: domain-neutral and safe for v1 with constraints. The confusion risk is `external_reference`, `command_output`, and `operator_note`; all three need strong redaction and bounded-summary rules.

## 6. Privacy And Redaction Review

The privacy posture is directionally correct.

Positive findings:

- Reference-over-copy is explicit.
- Summary-over-payload is explicit.
- Redaction metadata is required.
- Sensitivity classification is required or conservatively defaulted.
- Debug/Display non-leakage is called out.
- Provider metadata is treated as sensitive unless explicitly safe.
- Access-control implications are acknowledged.
- The forbidden-storage list covers tokens, authorization headers, private keys, environment values, raw CI logs, raw Jira bodies, raw GitHub large file contents, raw provider payloads, personal data, and secrets copied from specs/shells/logs/screenshots.

Missing or needs sharpening before implementation:

- Define a maximum summary size or bounded string policy.
- Define whether `uri_or_reference` may contain private repo names, issue keys, or tenant URLs, and how sensitivity applies to those identifiers.
- Require tests for `metadata` non-leakage, not just summary non-leakage.
- Treat `content_hash` and provider ETags as potentially sensitive metadata when they identify private artifacts.
- Make clear that redaction-safe Debug/Display must apply to nested metadata too.

These are implementation-prompt requirements, not ADR blockers.

## 7. Source-Of-Truth Boundary Review

The ADR correctly separates source-of-truth boundaries:

| Boundary | Review |
| --- | --- |
| EvidenceReference | Source of truth for existence of a cited pointer, not payload truth. |
| Provider systems | Remain source of truth for provider data. |
| Workflow events | Remain source of truth for run state. |
| Audit events | Remain source of truth for operational history. |
| Work reports | Future handoff artifacts that cite evidence references. |
| Reasoning lineage | Future derivation/provenance truth, not implemented now. |

This is one of the strongest parts of the proposal. It prevents EvidenceReference from becoming an accidental event store, audit replacement, provider cache, or reasoning graph.

## 8. Relationship To Existing Systems

| System | Review |
| --- | --- |
| Adapter telemetry | Good fit. Adapter response summaries and invocation records should produce/cite evidence references without storing raw provider payloads. |
| Validation diagnostics/results | Good fit. Validation output should be citeable by reports and approvals. |
| Approval decisions | Good fit. Approval packets and decisions need evidence references. |
| Policy decisions | Good fit, but v1 should only cite evidence when policy evaluation actually depends on evidence. |
| CLI inspect/status | Defer display until core model and persistence/attachment semantics are proven. |
| Local state backend | Defer persistence design. Do not place evidence references in the event log without a separate state-boundary review. |
| Future work reports | EvidenceReference should precede and support WorkReportContract. |
| Future reasoning lineage | EvidenceReference should be the citation substrate, not the graph itself. |

Key implementation guidance: adapter telemetry and validation attachments are the best first attachment points after the core type model.

## 9. Implementation Plan Review

| Phase | Verdict | Review note |
| --- | --- | --- |
| 1. Core type model | Ready | This should be the first implementation prompt. Keep it to types, serialization, redaction-safe display/debug, and tests. |
| 2. Attachment points | Ready with split | Do after Phase 1. Start with adapter telemetry and validation diagnostics; approval/audit citations can follow if the code boundaries are clean. |
| 3. Local persistence | Defer | Too consequential for the first implementation. Needs storage, corruption, retention, and source-of-truth review. |
| 4. CLI inspection | Defer | Should wait for persistence or stable attachment points. Keep CLI JSON experimental. |
| 5. Example integration | Defer | Useful after attachment/persistence decisions. Do not update examples until there is something real to inspect. |
| 6. Review gate | Ready | Required before WorkReportContract implementation. |

Recommended implementation sequence:

1. Implement Phase 1 only.
2. Run a focused review.
3. Implement adapter telemetry and validation attachment points.
4. Decide persistence and inspect behavior separately.
5. Update examples only after operator-visible behavior exists.

## 10. Test Plan Review

The implementation plan includes the right test categories:

- serialization/deserialization;
- redaction-safe Debug/Display;
- no token leakage;
- no raw provider payload storage;
- kind taxonomy parsing/round-trip;
- required/optional field behavior;
- attachment to adapter telemetry;
- attachment to validation results;
- attachment to approval decisions;
- evidence references in examples;
- local persistence if implemented;
- docs/examples remaining honest.

Required additions for the implementation prompt:

- scope-specific validation tests;
- bounded summary and metadata tests;
- tests that `uri_or_reference`/reference fields do not leak known secret patterns;
- tests that sensitivity defaults conservatively;
- tests that unknown or unsupported kind/scope combinations fail clearly or deserialize according to the compatibility policy chosen.

## 11. Product-Boundary Risks

| Risk | Assessment | Mitigation |
| --- | --- | --- |
| Evidence model becomes a document store | Real risk | Keep reference-only and forbid raw payloads by default. |
| Raw sensitive payloads leak into state | Real risk | Redaction-safe serialization/debug/display and non-leakage tests are mandatory. |
| Evidence model duplicates audit events | Moderate risk | Keep audit events as operational history and EvidenceReference as citation pointers. |
| Evidence references become domain-specific too early | Moderate risk | Keep provider-specific details in adapters and metadata constraints. |
| Evidence references imply access control that does not exist | Real risk | Document stale/inaccessible references and defer access control honestly. |
| Evidence references become hidden production evidence store | Real risk | Defer persistence and clearly deny production evidence-store posture. |
| Evidence references become reasoning lineage prematurely | Moderate risk | Keep claims/findings/edges out of EvidenceReference. |

## 12. Required Revisions

No ADR rewrite is required before generating an implementation prompt, but the implementation prompt must include these revisions:

1. Limit the first implementation to Phase 1 core type model.
2. Treat `uri_or_reference` naming as an implementation design point; prefer a clearer typed reference shape if the Rust model supports it cleanly.
3. Define scope-specific validation rules before coding.
4. Make `redaction_metadata`, `sensitivity`, and `created_at` required in the model.
5. Default sensitivity conservatively when omitted at construction boundaries.
6. Bound `summary`, `title`, and `metadata` size.
7. Prove Debug, Display, serialization, and test failure output do not leak nested metadata.
8. Defer local persistence, CLI inspection, and example changes until after the core model review.

## 13. Final Recommendation

**Approve implementation prompt generation.**

The implementation prompt should cover only the Phase 1 core type model plus tests and docs updates needed for that phase. It must not implement persistence, CLI inspection, example integration, work reports, reasoning lineage, writes, generic runtime adapter execution, domain packs, production evidence storage, or release posture changes.

Still do not build:

- schemas;
- CLI commands or stable CLI JSON changes;
- runtime persistence behavior;
- WorkReportContract or WorkReport artifacts;
- Reasoning Lineage / Claim Graph;
- side-effect boundary model;
- write-capable adapters;
- generic live adapter execution;
- domain packs;
- production backends;
- production evidence store;
- DLP or access-control systems.
