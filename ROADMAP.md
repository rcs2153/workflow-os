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

Adapters should not be built until release posture and local kernel contracts are settled.

Before any real adapter implementation:

- Adapter capability, policy, idempotency, audit, and redaction contracts must remain enforced.
- External writes must remain denied unless explicitly designed, policy-gated, audited, and idempotent.
- Adapter health, error classification, dry-run/plan behavior, and redacted response summaries must be tested.
- Docs must continue to state that adapters cannot mutate core workflow state directly.

## Later Read-Only Adapter Phases

Read-only adapters may be considered after the local kernel preview posture is stable:

- Read-only GitHub adapter later.
- Read-only Jira adapter later.
- Read-only CI adapter later.

Read-only adapter work must not imply write support, OAuth completeness, webhook ingestion, hosted operation, or production deployment readiness.

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

- GitHub adapters.
- Jira adapters.
- CI adapters.
- Production database backend.
- Distributed workers.
- SaaS control plane.
- UI product.
- Marketplace or package registry.
- High-autonomy external write behavior.
