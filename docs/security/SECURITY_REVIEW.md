# Security Review

This review records the v0 security posture after the local-first kernel hardening pass.

## Scope

Reviewed:

- Rust core primitives and runtime contracts.
- Project loader and semantic validator.
- Event-sourced run model.
- Local filesystem state backend.
- Local executor, approvals, retries, cancellation, escalation, policy, audit, and observability.
- CLI local commands.
- TypeScript SDK spec generation boundary.
- Phase 2 GitHub, Jira, and GitHub Actions read-only adapter boundaries.
- Vertical-slice example.
- OSS and CI security posture.

Not reviewed as implemented behavior because it does not exist in v0:

- write-capable GitHub, Jira, CI, or HTTP adapters
- distributed workers
- production database backends
- hosted SaaS
- UI
- enterprise RBAC or IdP integration
- real secret providers

## Findings

### S1: No Open Critical Security Blockers For Local v0 Kernel

The local v0 kernel has no known critical blocker for continued local development and contributor use, assuming it is not represented as production deployment software.

### S2: YAML Parser Dependency Accepted For 0.1.0-preview.1 Only

Maintainer decision for `0.1.0-preview.1`: accept `serde_yaml` for the public local kernel preview, with explicit risk tracking.

Rationale:

- YAML is the required human-authored spec format for v0.
- The current parser is already wired through schema-version checks, source-aware diagnostics, secret rejection, typed deserialization, and canonical content hashing.
- Replacing the parser immediately would have broad compatibility and diagnostic blast radius for a release-posture fix.
- The v0 preview treats specs as trusted local project files reviewed in Git, not untrusted remote input or attacker-supplied network payloads.

Risk:

- `serde_yaml` is deprecated and depends on `unsafe-libyaml`.
- `cargo audit` currently passes, but lack of active maintenance remains a public-preview risk.
- The project must not claim hardened malicious-spec parsing, production-grade parser isolation, or safe handling of arbitrary remote YAML.

Tracked follow-up: `ROADMAP.md` tracks `YAML-001`, replacing or isolating the YAML parser before any production-readiness claim or malicious-input hardening claim.

### S2: Local State Is Not A Production Security Boundary

The local filesystem backend does not provide encryption at rest, tamper evidence, replication, distributed locks, access controls, or automated repair. This is acceptable for v0 local development but must not be described as production storage.

### S2: Local Skill Handlers Are Trusted Code

The v0 `SkillHandler` trait does not sandbox handler implementations. Handler code could perform side effects outside the runtime if a contributor writes it that way. Documentation must continue to state that real external behavior belongs behind adapters and policy/audit/idempotency boundaries.

### S3: Full Runtime Contract Validation Is Deferred

The local executor checks required output fields, but full nested type validation and field-level runtime redaction enforcement are not implemented. This is documented as a v0 limitation.

## Controls Confirmed

- `#![deny(unsafe_code)]` is present in `workflow-core`.
- Workspace lints deny unsafe Rust, `unwrap`, `expect`, `panic`, `todo`, and `unimplemented`.
- Secret-like spec keys and values are rejected by loader/parser paths.
- `RedactedValue` avoids accidental `Display`, `Debug`, and serialization disclosure.
- Audit/log paths use non-secret summaries and references.
- Unknown policy actions and capabilities fail closed.
- `external.write`, unsupported adapter invocation, and Level 3/4 execution are denied by default.
- Approval gates are enforced before local skill invocation.
- Duplicate event IDs and duplicate sequence numbers are rejected by the local backend.
- Rehydration rejects invalid event sequences and terminal-state mutation.
- CI includes Rust, TypeScript, docs, lockfile, and dependency/security checks.

## Privacy Considerations

Workflow OS should not store full sensitive payloads in specs, diagnostics, logs, audit records, event payload summaries, or observability records by default. Local examples intentionally avoid secrets and external services.

Users running local workflows are responsible for choosing non-sensitive input/output summaries until real secret-provider and adapter boundaries exist.

## Required Follow-Ups Before Production Readiness

- Replace or justify the YAML parser strategy.
- Resolve `YAML-001`: replace `serde_yaml` or isolate YAML parsing behind a maintained, bounded parser strategy before production-readiness or hardened malicious-spec parsing claims.
- Add production backend threat model and contract tests when a backend exists.
- Add adapter-specific security reviews before any real external integration.
- Add secret provider design and tests before enabling `secret.read`.
- Add stronger audit persistence guarantees for production modes. v0 now durably records policy decisions, including denied starts before `RunCreated`, in the local policy audit ledger; this is still a local development backend, not a production audit service.
- Add tamper-evidence or integrity strategy for production event logs.
- Add sandboxing or process isolation strategy if untrusted skill handlers become supported.

## Assessment

The repository is security-coherent for a local-first v0 kernel. Remaining risks are explicit and should block production deployment claims, not local kernel development.
