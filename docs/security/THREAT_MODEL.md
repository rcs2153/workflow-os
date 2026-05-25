# Threat Model

This threat model covers the v0 local-first Workflow OS kernel and the development-branch Phase 2 GitHub/Jira/GitHub Actions read-only adapter boundary. The `0.1.0-preview.1` local kernel release does not include real provider adapters in its release contract. Phase 2 read-only adapters are for internal review and are not a public read-only integration preview until a follow-up maintainer review approves that posture.

This threat model does not cover hosted SaaS, distributed workers, production database backends, production integrations, write-capable external adapters, OAuth, webhook ingestion, or UI because those are not implemented in v0.

## Assets

- Workflow specs and project metadata.
- Workflow run event logs.
- Run snapshots and approval projections.
- Audit and observability records.
- GitHub, Jira, and GitHub Actions read-only adapter request summaries, response summaries, health records, and invocation records.
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
- GitHub read-only adapter user.
- Jira read-only adapter user.
- GitHub Actions read-only adapter user.
- Malicious or mistaken spec author.
- Local attacker with filesystem access.

## Trust Boundaries

- YAML specs enter through the project loader.
- Runtime actions enter through the CLI or Rust APIs.
- Skill handlers execute inside the local runtime process.
- State is persisted through `StateBackend`.
- Audit and observability leave the runtime through sink interfaces.
- Adapters must remain outside the core state boundary.
- The GitHub read-only adapter crosses a network and credential boundary but must not mutate GitHub state.
- The Jira read-only adapter crosses a network and credential boundary but must not mutate Jira state.
- The GitHub Actions read-only adapter crosses a network and credential boundary but must not mutate workflow runs, jobs, checks, logs, or artifacts.

v0 YAML specs are trusted local project files, expected to be authored and reviewed in Git by project contributors. They are not treated as untrusted remote input, webhook payloads, uploaded SaaS content, or adversarial documents. The parser posture for `0.1.0-preview.1` is preview-only: `serde_yaml` is accepted with documented risk, and Workflow OS must not claim hardened malicious-spec parsing.

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

### GitHub Read-Only Credential Leakage

Risk: a GitHub token appears in specs, diagnostics, health output, audit records, observability records, debug output, or logs.

Controls:

- GitHub credentials are loaded from environment variables, not specs
- health checks report credential presence only
- token values are wrapped in `RedactedValue`
- fixture tests assert token values do not appear in debug, audit, observability, or health output
- adapter summaries store references and normalized metadata, not raw large provider payloads by default

Limit: live GitHub calls are opt-in and run in the local process. There is no secret provider integration, OAuth app, token rotation workflow, or hosted credential isolation in v0.

### GitHub Read-Only Scope Drift

Risk: the adapter grows write behavior or makes Workflow OS look like a GitHub automation tool.

Controls:

- write-capable GitHub capabilities fail closed in Phase 2
- no branch creation, commits, pull request creation, comments, labels, merges, check reruns, workflow dispatch, or webhook receiver are implemented
- docs and tests distinguish fixture, mock, and live read-only behavior
- adapter responses are normalized into generic adapter records

Limit: future write-capable GitHub work would require a new scoped design, policy model review, audit review, and threat-model update.

### Jira Read-Only Credential Leakage

Risk: a Jira token appears in specs, diagnostics, health output, audit records, observability records, debug output, or logs.

Controls:

- Jira credentials are loaded from environment variables, not specs
- health checks report credential presence only
- token values are wrapped in `RedactedValue`
- fixture tests assert token values do not appear in debug, audit, observability, or health output
- issue descriptions and comment bodies are represented as reference-only summaries
- adapter summaries store references and normalized metadata, not raw issue payloads by default

Limit: live Jira calls are opt-in and run in the local process. There is no secret provider integration, OAuth app, token rotation workflow, or hosted credential isolation in v0.

### Jira Read-Only Scope Drift

Risk: the adapter grows write behavior or makes Workflow OS look like a ticketing automation product.

Controls:

- write-capable Jira capabilities fail closed in Phase 2
- no issue creation, issue updates, comments, status transitions, assignment changes, label changes, link creation, or webhook receiver are implemented
- docs and tests distinguish fixture, mock, and live read-only behavior
- adapter responses are normalized into generic adapter records

Limit: future write-capable Jira work would require a new scoped design, policy model review, audit review, and threat-model update.

### CI Read-Only Credential And Log Leakage

Risk: a GitHub Actions token or sensitive CI log content appears in specs, diagnostics, health output, audit records, observability records, debug output, or logs.

Controls:

- GitHub Actions credentials are loaded from environment variables, not specs
- health checks report credential presence only
- token values are wrapped in `RedactedValue`
- fixture tests assert token values do not appear in debug, audit, observability, or health output
- log references are preferred over raw logs
- explicit log excerpts are bounded and redacted before adapter summaries are produced
- audit records store references and summaries rather than full logs by default

Limit: log redaction is a preview safety layer, not a full data-loss-prevention system. Live GitHub Actions calls are opt-in and run in the local process. There is no secret provider integration, OAuth app, token rotation workflow, hosted credential isolation, or enterprise log-classification engine in v0.

### CI Read-Only Scope Drift

Risk: the adapter grows rerun, dispatch, cancellation, check mutation, or artifact mutation behavior and makes Workflow OS look like a CI automation replacement.

Controls:

- `ci.write`, `ci.rerun`, and `adapter.write` fail closed in Phase 2
- no workflow rerun, failed-job rerun, workflow cancellation, workflow dispatch, artifact upload, log deletion, check mutation, or webhook receiver is implemented
- docs and tests distinguish fixture, mock, and live read-only behavior
- adapter responses are normalized into generic adapter records

Limit: future write-capable CI work would require a new scoped design, policy model review, audit review, and threat-model update.

### Malicious Skill Handler

Risk: local skill handler code performs hidden network, filesystem, shell, or secret access.

Controls:

- v0 handler docs define handlers as deterministic local test/development code
- external behavior belongs behind adapters
- capability and policy model defines future side-effect boundaries

Limit: Rust trait implementations are trusted local code in v0. There is no sandboxing of handler code.

### Malicious YAML Spec

Risk: a malicious or fuzzed YAML document exploits parser behavior, resource usage, or the deprecated `serde_yaml` / `unsafe-libyaml` dependency.

Controls:

- v0 specs are trusted local project files, not remote uploads or webhook payloads
- loader validates schema version before typed parsing
- loader rejects secret-like spec keys and values
- malformed YAML returns structured diagnostics
- CI runs parser behavior tests and example validation
- `cargo audit` is part of the release checks

Limit: `serde_yaml` is deprecated and depends on `unsafe-libyaml`. v0 does not sandbox YAML parsing, impose a hardened parser resource policy, or claim malicious-spec hardening. `YAML-001` tracks replacement or isolation before production-readiness or adversarial-input claims.

## Deferred Threats

Deferred until corresponding features exist:

- distributed worker compromise
- production database compromise
- adapter credential theft beyond the Phase 2 GitHub/Jira/GitHub Actions read-only local environment variable posture
- OAuth authorization flaws
- webhook spoofing
- hardened malicious YAML parsing
- SaaS tenant isolation
- UI session security
- marketplace/package supply-chain execution

Any implementation of these areas requires a threat-model update and likely an ADR.
