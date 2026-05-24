# Threat Model

This threat model covers the v0 local-first Workflow OS kernel. It does not cover hosted SaaS, distributed workers, production database backends, real external adapters, OAuth, webhook ingestion, or UI because those are not implemented in v0.

## Assets

- Workflow specs and project metadata.
- Workflow run event logs.
- Run snapshots and approval projections.
- Audit and observability records.
- Idempotency records.
- Local state backend files.
- CLI output and diagnostics.
- Spec content hashes and workflow identity.

Secrets are not valid spec assets. They must not be stored in specs or audit payloads.

## Actors

- Local contributor running CLI commands.
- Local approver submitting manual approval decisions.
- Local runtime system actor.
- Future adapter implementer.
- Malicious or mistaken spec author.
- Local attacker with filesystem access.

## Trust Boundaries

- YAML specs enter through the project loader.
- Runtime actions enter through the CLI or Rust APIs.
- Skill handlers execute inside the local runtime process.
- State is persisted through `StateBackend`.
- Audit and observability leave the runtime through sink interfaces.
- Future adapters must remain outside the core state boundary.

## Primary Threats

### Secret Leakage

Risk: a spec author places tokens, passwords, keys, or credentials in specs, diagnostics, logs, audit events, or examples.

Controls:

- loader rejects secret-like keys and values
- `RedactedValue` prevents routine display of sensitive values
- audit/log paths store references and summaries
- docs prohibit secrets in specs
- tests cover redaction behavior

### Policy Bypass

Risk: runtime invokes a skill, adapter, external write, resume, or approval-sensitive action without policy evaluation.

Controls:

- conservative policy engine runs before meaningful local runtime actions
- unknown actions and capabilities fail closed
- adapter invocation and `external.write` are denied in v0
- policy decisions are recorded as runtime/audit events
- tests cover policy denial paths

### Unsafe Autonomy Escalation

Risk: Level 3/4 workflows execute by default.

Controls:

- validator requires Level 3/4 declarations to be experimental and disabled by default
- policy engine denies Level 3/4 execution by default
- docs state Level 3/4 are not default behavior

### State Tampering Or Corruption

Risk: local event files are modified, deleted, duplicated, or partially restored.

Controls:

- event sequence numbers must be contiguous
- duplicate event IDs and duplicate sequence numbers are rejected
- rehydration fails deterministically on invalid sequences
- corrupt JSON returns structured errors
- snapshots are projections and are not the source of truth

Limit: v0 local state does not provide cryptographic tamper evidence, encryption at rest, replication, or automated repair.

### Approval Bypass

Risk: approval-gated steps execute before approval.

Controls:

- executor emits `ApprovalRequested` and stops before skill invocation
- resume requires an approval decision event
- approval denial fails closed
- terminal states reject later approval mutation
- tests cover pause, resume, denial, duplication, and rehydration

### Duplicate Side Effects

Risk: repeated execution creates duplicate side effects.

Controls:

- skill invocations are idempotency-keyed
- local idempotency store is first-write-wins
- explicit run ID reuse rehydrates existing durable events
- future side-effecting adapters must carry idempotency keys

Limit: v0 has no real external side effects; future adapters need additional contract tests.

### Malicious Skill Handler

Risk: local skill handler code performs hidden network, filesystem, shell, or secret access.

Controls:

- v0 handler docs define handlers as deterministic local test/development code
- external behavior belongs behind adapters
- capability and policy model defines future side-effect boundaries

Limit: Rust trait implementations are trusted local code in v0. There is no sandboxing of handler code.

## Deferred Threats

Deferred until corresponding features exist:

- distributed worker compromise
- production database compromise
- adapter credential theft
- OAuth authorization flaws
- webhook spoofing
- SaaS tenant isolation
- UI session security
- marketplace/package supply-chain execution

Any implementation of these areas requires a threat-model update and likely an ADR.
