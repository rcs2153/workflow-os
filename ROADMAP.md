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

Write-capable and production adapters should not be built until release posture and local kernel contracts are settled. Phase 2 development-branch read-only adapters are the narrow exception: they exist to prove the adapter contract against real systems without writes.

Before any real adapter implementation:

- Adapter capability, policy, idempotency, audit, and redaction contracts must remain enforced.
- External writes must remain denied unless explicitly designed, policy-gated, audited, and idempotent.
- Adapter health, error classification, dry-run/plan behavior, and redacted response summaries must be tested.
- Docs must continue to state that adapters cannot mutate core workflow state directly.

## Phase 2 Read-Only Integration Posture

Phase 2 is the read-only integration capability phase. It is documented in [docs/integrations/PHASE_2_READ_ONLY_INTEGRATIONS.md](docs/integrations/PHASE_2_READ_ONLY_INTEGRATIONS.md).

The development branch includes initial Phase 2 read-only adapters for internal review:

- GitHub read-only adapter foundation.
- Jira read-only adapter foundation.
- GitHub Actions CI read-only adapter foundation.

GitHub Actions is the first CI target for read-only adapter proving. Other CI providers remain future work.

This is not yet a public read-only integration preview. Public preview posture requires the maintainer review blockers to remain fixed, live smoke procedures to be available, and a follow-up review to approve the release posture.

Read-only adapter work must not imply write support, OAuth completeness, webhook ingestion, hosted operation, distributed workers, production database readiness, or Level 3/4 autonomy enablement.

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
