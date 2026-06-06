# Roadmap

Workflow OS grows from the local-first kernel outward.

## Foundation

- Establish governance, contribution, security, release, and quality-gate standards.
- Set up the Rust workspace and TypeScript SDK workspace.
- Prepare documentation structure for concepts, specs, runtime, CLI, SDK, operations, security, and release.

## v0 Kernel

- Model canonical workflow specs in Rust.
- Define schema versioning and content hashing.
- Build validation for workflow definitions.
- Define durable state interfaces.
- Define append-only meaningful runtime events.
- Define policy, audit, and observability primitives.
- Build local-first CLI commands only after their contracts are documented.

## v0 Local Kernel Preview Release Hygiene

- Keep the public posture clear: v0 is a local kernel preview, not a production distributed runtime.
- Keep README, changelog, release readiness, known limitations, and example docs aligned.
- Keep CI green across Rust, TypeScript, docs, dependency audits, examples, and schema/SDK contracts.
- Apply release versions consistently across crates, packages, changelog, and release notes.
- Track schema/TypeScript synchronization explicitly until generated contracts exist.
- `YAML-001`: replace `serde_yaml` or isolate YAML parsing behind a maintained, bounded parser strategy before any production-readiness or malicious-spec hardening claim.
- Keep CLI JSON output marked as preview until a stable machine-output contract is designed.

## Adapter Readiness Criteria

Write-capable and production adapters should not be built until release posture and local kernel contracts are settled. Phase 2 read-only adapters are the narrow exception: they exist to prove the adapter contract against real systems without writes.

Before any real adapter implementation:

- Adapter capability, policy, idempotency, audit, and redaction contracts must remain enforced.
- External writes must remain denied unless explicitly designed, policy-gated, audited, and idempotent.
- Adapter health, error classification, dry-run/plan behavior, and redacted response summaries must be tested.
- Docs must continue to state that adapters cannot mutate core workflow state directly.

## Phase 2 Read-Only Integration Posture

Phase 2 is the read-only integration capability phase. It is documented in [docs/integrations/PHASE_2_READ_ONLY_INTEGRATIONS.md](docs/integrations/PHASE_2_READ_ONLY_INTEGRATIONS.md).

The `0.2.0-preview.1` public read-only integration preview includes initial Phase 2 read-only adapters:

- GitHub read-only adapter foundation.
- Jira read-only adapter foundation.
- GitHub Actions CI read-only adapter foundation.

GitHub Actions is the first CI target for read-only adapter proving. Other CI providers remain future work.

The `0.2.0-preview.1` posture approves a narrow public read-only integration preview after live smoke evidence was recorded and reviewed. That approval is limited to read-only provider access, fixture-first normal CI, and opt-in live tests.

Read-only adapter work must not imply write support, OAuth completeness, webhook ingestion, hosted operation, distributed workers, production database readiness, production integration readiness, broad live provider compatibility, or Level 3/4 autonomy enablement.

The following remain out of scope for Phase 2:

- Creating branches.
- Opening pull requests.
- Posting pull request comments.
- Updating Jira issues or comments.
- Changing Jira status.
- Rerunning CI.
- Workflow dispatch.
- Webhooks or an event ingestion service.
- OAuth app implementation.
- External writes of any kind.

## Governed Work Pattern Architecture

[Governed Work Pattern](docs/concepts/governed-work-pattern.md) is accepted as architecture direction by [ADR 0007](docs/adr/0007-governed-work-pattern.md). Acceptance does not implement runtime behavior or authorize schemas, CLI changes, writes, generic runtime adapter execution, or domain packs.

The first scoped MVP concept is [EvidenceReference](docs/concepts/evidence-reference.md), proposed in [ADR 0009](docs/adr/0009-evidence-reference-core-model.md) with a phased implementation plan in [docs/implementation-plans/evidence-reference-mvp.md](docs/implementation-plans/evidence-reference-mvp.md). EvidenceReference Phase 1 core type model is implemented and reviewed. Adapter telemetry evidence attachment, `Diagnostic` evidence attachment, and selected schema-version validation diagnostic call-site evidence are implemented and reviewed. Broader validation attachment, approval attachment, persistence, CLI, and example attachments remain future scoped work.

The current scoped report foundation has advanced through the `WorkReportContract` core model, `WorkReport` core model, in-memory terminal local report generation helper, and in-memory runtime result exposure helper. These phases are documented in [docs/implementation-plans/work-report-contract-plan.md](docs/implementation-plans/work-report-contract-plan.md), [docs/implementation-plans/terminal-local-report-generation-plan.md](docs/implementation-plans/terminal-local-report-generation-plan.md), and [docs/implementation-plans/runtime-result-report-exposure-plan.md](docs/implementation-plans/runtime-result-report-exposure-plan.md). Automatic runtime report generation, executor-integrated automatic result exposure, report artifacts, persistence, CLI rendering, schema changes, and examples remain later phases and require separate accepted implementation work.

Side-effect boundary modeling must be accepted before policy-gated writes, generic runtime adapter execution, or domain packs.

Remaining candidate decisions:

- remaining EvidenceReference attachment boundaries, including approval evidence and broader validation evidence
- review of the in-memory runtime result exposure helper
- whether generated report exposure should return report-generation errors separately from workflow results
- whether unavailable report references should remain section text or become explicit missing-citation records
- how governed work reports relate to audit events
- how much report structure the runtime should enforce
- how side-effect boundaries should be represented before write-capable adapters
- how future Reasoning Lineage or Claim Graph concepts should relate to governed work

This milestone must not introduce domain packs, write-capable adapters, or new runtime primitives until a scoped ADR or implementation plan is accepted.

## Reasoning Lineage / Claim Graph Architecture

The [Governed Work Pattern](docs/concepts/governed-work-pattern.md) is accepted as architecture direction, and [Reasoning Lineage / Claim Graph](docs/concepts/reasoning-lineage.md) remains captured as proposed architecture direction in [ADR 0008](docs/adr/0008-reasoning-lineage-claim-graph.md). Reasoning Lineage is a follow-on provenance direction after Governed Work Pattern, and neither direction is implemented as runtime behavior.

Revisit Reasoning Lineage after the EvidenceReference and WorkReportContract foundations are scoped. Revisit these directions together before policy-gated writes, generic runtime adapter execution, or broader domain packs. Implementation of either direction requires a separate accepted ADR or scoped implementation plan.

This milestone should treat reasoning lineage as supporting structure for governed work, not as the primary workflow runtime. Workflow OS must remain a declarative workflow kernel with durable state, policy gates, approvals, auditability, observability, and adapter boundaries.

Candidate decisions:

- how to represent claim or finding nodes
- how to represent derivation edges between claims, evidence, validations, decisions, and reports
- how additive corrections should work without rewriting history
- whether confidence metadata belongs in core, skills, domain packs, or reports
- how actor attribution should attach to generated, reviewed, corrected, or approved claims
- how reference resolution and context binding should connect claims to evidence
- how reasoning lineage should link to evidence references, work reports, audit events, adapter invocation records, validation results, and approval decisions
- what belongs in core versus skills versus domain packs

This milestone must not interrupt Phase 2 live-smoke/public-preview readiness. Implementation of either concept requires a separate accepted ADR or scoped implementation plan.

## Later Production Backend Phase

Production backends are deferred until after local kernel preview release hygiene and adapter readiness criteria are settled.

Future backend work should include:

- Production database contract tests.
- Migration and compatibility strategy for persisted state.
- Backup and restore guidance.
- Corruption detection and repair procedures.
- Locking/fencing semantics.
- Audit persistence and export posture.
- Threat model updates.

## Deferred Until Kernel Correctness And Release Posture

- GitHub write adapters.
- Jira write adapters.
- CI write adapters and additional CI providers.
- Production database backend.
- Distributed workers.
- SaaS control plane.
- UI product.
- Marketplace or package registry.
- High-autonomy external write behavior.
