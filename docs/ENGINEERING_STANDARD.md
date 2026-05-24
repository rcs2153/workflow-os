# Workflow OS Engineering Standard

This document is the standing engineering standard for Workflow OS. Every future implementation, review, and Codex task must follow it unless a change is explicitly approved through a documented architecture decision record (ADR).

Workflow OS is intended to become an enterprise-grade open-source framework for defining, validating, testing, executing, governing, observing, and scaling AI-driven workflows. It must not be treated as a prototype, demo, hackathon artifact, or generic automation script.

## 1. Production-Grade Scope Control

All work must be minimal, scoped, and production-shaped.

Required practices:

- Make the smallest change that correctly satisfies the accepted requirement.
- Avoid unrelated refactors, formatting churn, renames, and opportunistic cleanup.
- Do not implement speculative features, unused abstractions, or future-facing hooks without a concrete requirement.
- Do not claim completion when behavior is stubbed, mocked, placeholder-only, or non-functional.
- Clearly identify any incomplete behavior, placeholder behavior, mock-only behavior, or deferred production requirement in the implementation report.
- Prefer boring, explicit, reviewable implementation over cleverness.

## 2. Product Boundary

Workflow OS Core is a local-first, declarative framework for defining, validating, testing, and executing governed AI workflows through pluggable skills and adapters.

Workflow OS Core is not:

- A replacement for Temporal.
- A replacement for Airflow.
- A replacement for GitHub Actions.
- A replacement for Zapier.
- A generic business process management engine.
- A chat agent framework.
- A one-off GitHub, Jira, or CI automation tool.
- An enterprise SaaS application.
- A UI product in v0.

Implementation work must preserve this boundary. Features that pull Core toward orchestration platforms, SaaS workflow builders, chat-agent products, or one-off integration bots must be rejected or deferred unless the product boundary is intentionally changed through an ADR.

## 3. Architecture

Rust owns the canonical implementation of Workflow OS Core.

Rust-owned responsibilities include:

- Core domain model.
- Workflow validator.
- Runtime kernel.
- Durable state model.
- CLI.
- Public schemas.
- Policy evaluation.
- Audit model.
- Observability model.

TypeScript may be used for SDK ergonomics, developer experience, and spec generation, but it must not become a parallel incompatible model. TypeScript-facing contracts must be generated from, validated against, or otherwise kept compatible with the canonical Rust model.

Runtime architecture requirements:

- Runtime workers must be designed as stateless workers operating over durable workflow state.
- Durable workflow state must be externalized through interfaces.
- Core logic must not depend on a single embedded database, queue, cloud provider, or vendor runtime.
- External integrations must happen only through adapters.
- Adapters must not mutate core state directly.
- Adapters must request actions through explicit core interfaces that enforce validation, policy, audit, idempotency, and state transition rules.
- Core invariants must be enforceable without requiring a specific external integration.

## 4. Safety and Governance

Workflow OS must be safe by default.

Autonomy requirements:

- Level 1 and Level 2 autonomy are the default.
- Level 3 and Level 4 autonomy must be explicitly policy-enabled.
- Level 3 and Level 4 autonomy must never be default behavior.
- Escalation from lower autonomy to higher autonomy must be explicit, policy-checked, and auditable.

Policy and side-effect requirements:

- Policy decisions must happen before side effects.
- Unknown, unsupported, ambiguous, or unsafe actions must fail closed.
- External writes must be capability-gated, policy-gated, auditable, and idempotent.
- Human approval must be required for sensitive or ambiguous actions.
- Sensitive actions must include enough decision context for later audit.
- A denied policy decision must not be bypassable by an adapter, SDK, CLI flag, retry, or worker restart.

## 5. Determinism and Runtime Correctness

Workflow execution must be reproducible, auditable, and restart-safe.

Required runtime invariants:

- Workflow definitions must be immutable once a run starts.
- A run must reference the exact workflow spec version and content hash it was created from.
- State transitions must be explicit and auditable.
- Meaningful state transitions must be append-only in the event log.
- Current state must be derivable from, or reconcilable against, the event log.
- Skill invocations must be idempotency-keyed.
- Runtime workers must be restart-safe.
- Duplicate trigger events must be deduplicated.
- No workflow may silently terminate in an unsafe, partial, or ambiguous state.
- Retry behavior must be explicit and observable.
- Failures must preserve enough context for diagnosis, replay, or safe manual resolution.

## 6. Contracts and Compatibility

Workflow OS must treat public contracts as durable product surface area.

Required practices:

- Public schemas must be versioned.
- Spec files must declare schema version.
- Public CLI behavior must be documented.
- Public SDK contracts must be documented.
- Backward compatibility must be preserved unless explicitly broken through a documented major-version change.
- Experimental features must be clearly marked in code, documentation, CLI output, and schema where applicable.
- Breaking changes must include migration notes.
- Compatibility expectations must be tested where feasible.

## 7. Security

Security is a core requirement, not a later hardening pass.

Required practices:

- Rust crates must deny unsafe code unless unsafe usage is explicitly justified in an ADR.
- Secrets must never be stored in workflow specs.
- Secrets must be referenced through environment variables or secret provider references.
- Logs and audit events must redact sensitive fields.
- Audit records should store references and summaries, not full sensitive payloads by default.
- All dependency additions must be justified.
- Dependency and supply-chain risk must be considered before adding new packages, crates, build tools, or generated artifacts.
- File system, network, shell, and external write capabilities must be explicit and policy-governed.
- Error messages must avoid leaking secrets, tokens, credentials, private payloads, or sensitive policy details.

## 8. Observability and Auditability

Workflow OS must be observable by design.

Structured logs must include correlation IDs where relevant.

Audit events must include, where relevant:

- Actor or system actor.
- Timestamp.
- Workflow ID.
- Run ID.
- Event ID.
- Decision context.
- Input references.
- Output references.
- Policy context.
- Idempotency key.

Runtime events must support:

- Metrics.
- Tracing.
- Latency tracking.
- Retry counts.
- Failure counts.
- Escalation counts.
- Approval wait time.
- Stuck workflow detection.

Observability and audit features must be designed as core runtime behavior, not adapter-specific logging.

## 9. Testing

Tests must prove behavior, not merely construction.

Required test coverage includes, as applicable:

- Expected behavior.
- Edge cases.
- Failure modes.
- Regressions.
- Permission boundaries.
- Idempotency.
- Serialization.
- Validation.
- State transitions.
- Policy enforcement.
- Restart safety.
- Duplicate event handling.
- Audit event emission.

Testing requirements:

- Tests must not merely assert object construction.
- Placeholder tests do not count as tests.
- Mock-only behavior must not be presented as production behavior.
- Test names must describe the behavior being protected.
- Tests that intentionally rely on mocks must identify the production boundary that remains unverified.
- Runtime correctness tests must verify state and event-log behavior, not just return values.

## 10. Documentation

Documentation is part of the product contract.

Required documentation:

- Every public concept must be documented.
- Every CLI command must be documented.
- Every spec field must be documented.
- Every runtime invariant must be documented.
- Operational runbooks must exist for production-shaped behavior.
- Experimental features must be clearly documented as experimental.
- Known limitations must be explicit.
- Examples must not imply production support for behavior that is stubbed, mocked, or not implemented.

## 11. Open-Source Quality

Workflow OS must not be considered open-source-ready until the repository includes:

- License.
- Code of conduct.
- Contributing guide.
- Security policy.
- Vulnerability disclosure process.
- Changelog.
- Issue templates.
- Pull request template.
- ADR process.
- Semantic versioning policy.
- Release process.

These documents must be accurate, maintained, and aligned with the actual maturity of the project.

## 12. Implementation Reports

Every Codex task must return a structured implementation report.

The report must include:

- Files changed.
- Behavior added or changed.
- Contracts added or changed.
- Tests added or changed.
- Docs added or changed.
- Validation performed.
- Security and privacy considerations.
- Assumptions made.
- Risks and follow-ups.
- Explicit statement of any incomplete or placeholder work.

Required format:

```markdown
## Implementation Report

### Files Changed
- ...

### Behavior Added or Changed
- ...

### Contracts Added or Changed
- ...

### Tests Added or Changed
- ...

### Docs Added or Changed
- ...

### Validation Performed
- ...

### Security and Privacy Considerations
- ...

### Assumptions Made
- ...

### Risks and Follow-Ups
- ...

### Incomplete or Placeholder Work
- ...
```

If a section does not apply, the report must say so explicitly. Silent omission is not acceptable.

## Standing Review Checklist

Before any change is considered complete, reviewers and implementers must confirm:

- The change preserves the Workflow OS Core product boundary.
- The change is minimal and scoped.
- No speculative feature was added.
- Public contracts are documented and versioned where applicable.
- Runtime state behavior is explicit, auditable, and restart-safe where applicable.
- Policy is evaluated before side effects where applicable.
- Unsafe or ambiguous behavior fails closed.
- Security and privacy implications were considered.
- Tests cover meaningful behavior and failure modes.
- Documentation matches actual behavior.
- Placeholder or incomplete work is disclosed.
