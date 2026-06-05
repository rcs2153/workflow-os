# Adapter Contracts

Workflow OS adapters are future integration boundaries. They are the only place external systems may be read from or written to, and they must never mutate core workflow state directly.

v0 defines generic adapter contracts. The `0.1.0-preview.1` local kernel release does not include real provider adapters. The `0.2.0-preview.1` public read-only integration preview adds GitHub, Jira, and GitHub Actions read-only adapters that implement the read side of this contract. Generic HTTP, event-source, OAuth, webhook, and write-capable adapters are still not implemented.

The integration phase is documented in [PHASE_2_READ_ONLY_INTEGRATIONS.md](PHASE_2_READ_ONLY_INTEGRATIONS.md). Phase 2 is limited to real read-only adapter capability, explicitly excludes external writes, and is public-previewed only with the accepted live-smoke limitations documented in the release notes.

## Concepts

- `AdapterId`: stable adapter identifier.
- `AdapterKind`: symbolic adapter kind. v0 reserves `github`, `jira`, `ci`, `local`, and `generic-http`.
- `AdapterCapability`: capability required by an adapter operation.
- `AdapterAction`: named operation plus whether it may create side effects.
- `AdapterOperationMode`: explicit mode: fixture, mock, local, live read-only, or future live write-capable.
- `AdapterRequest`: request envelope carrying adapter identity, adapter kind, action, primary capability, operation mode, actor, correlation ID, run identity where relevant, idempotency key where relevant, redaction policy, timeout policy, non-secret metadata, and explicit policy pre-check provenance.
- `AdapterResponse`: audit-safe response carrying adapter ID, action, success or failure status, normalized summary, external references, redaction metadata, response size metadata, correlation ID, latency, and warnings.
- `AdapterError`: classified adapter failure.
- `AdapterEvent`: non-secret event emitted by an event-source adapter.
- `AdapterHealth`: health-check result that reports configured/unconfigured, reachability where testable, credential presence without credential values, last checked timestamp, and warnings.
- `AdapterInvocationRecord`: audit-safe record of an adapter attempt.
- `AdapterObservabilityRecord`: observability record derived from an adapter invocation.
- `EvidenceReference`: reference-first evidence pointer that adapter telemetry may attach after validation.

Run-scoped adapter requests must include workflow run ID, workflow ID, workflow version, schema version, and spec content hash. Non-run-scoped adapter requests must still include actor, correlation ID, capability, timeout policy, redaction policy, and policy pre-check.

## Traits

The Rust core defines traits for:

- read-only operations
- write-capable operations
- event-source operations
- health checks
- capability discovery
- dry-run or plan mode

Implementations must live behind these traits. They must not call core state mutation APIs directly.

## Side-Effect Preconditions

External writes require all of the following before an adapter operation can run:

- declared `external_write` capability
- correlation ID
- idempotency key
- policy allow or approval-granted pre-check
- redaction strategy

Unknown capabilities fail closed.

Phase 2 denies write-capable adapter operations even when they carry idempotency keys and policy pre-checks. The write-capable contract exists so future phases have a precise surface to harden, not because writes are enabled.

## Phase 2 Read-Only Capabilities

Phase 2 defines these provider-specific read-only capabilities:

- `github.read`
- `jira.read`
- `ci.read`

The following capabilities remain unsupported or denied in Phase 2:

- `github.write`
- `jira.write`
- `ci.write`
- `ci.rerun`
- `adapter.write`

Read-only capabilities still require policy evaluation, contract-level adapter telemetry records, classified errors, redacted summaries, and credential handling outside specs.

Read-only adapter actions must declare one of the read capabilities on the request and on the action. Missing capabilities fail closed.

Adapter request helpers must not silently mark requests as policy-approved. A request must carry one of:

- runtime policy authorization
- runtime approval-decision authorization
- fixture/test authorization
- explicit denial
- missing/not-evaluated state

Fixture/test authorization is allowed only for deterministic fixture and test paths. Runtime adapter invocation paths must use runtime policy or approval-decision provenance.

## Error Classification

Adapter errors must classify failures as one of:

- authentication failure
- permission failure
- not found
- rate limited
- timeout
- validation failure
- malformed response
- transient network failure
- unsupported operation
- policy denied
- unknown

Adapters must not return raw provider error payloads when those payloads may contain tokens, credentials, private object bodies, or sensitive policy details.

## Health Checks

Adapter health checks must not expose secrets. A health result may state that credentials are present or absent, but must not include credential values, token prefixes, headers, private keys, or secret provider payloads.

Reachability is optional because fixture, mock, and local modes may not have a network endpoint to test. Live read-only adapters should report reachability when a safe read-only check exists.

## Audit And Redaction

Adapter responses should store external references and summaries, not raw payloads. Sensitive-looking summaries are redacted by the contract helpers. Future adapters must preserve this rule for logs, audit events, and runtime event payloads.

Every adapter invocation must produce an audit-safe invocation record and an observability record. Those records must include adapter ID, adapter kind, action, operation mode, capability, actor, correlation ID, latency, status, error classification for failures, redaction metadata, and run identity when run-scoped.

The adapter pre-check provenance must be auditable. Operators should be able to distinguish a runtime policy decision from fixture/test authorization.

Adapter invocation and runtime audit telemetry records can carry validated `EvidenceReference` values for adapter invocation and adapter response summary evidence. Attachment validates internally and fails closed for invalid evidence; invalid evidence must not be silently dropped or partially attached. Evidence attachments must preserve redaction metadata and sensitivity, and must not store raw provider payloads, raw CI logs, raw Jira descriptions/comments, raw large GitHub file contents, tokens, authorization headers, or private keys.

## Phase 2 Telemetry Posture

Phase 2 adapter telemetry starts as **contract-level adapter telemetry**. The GitHub, Jira, and CI adapters return `AdapterInvocationRecord` and `AdapterObservabilityRecord` values, and the adapter contract tests assert those records are produced and redacted.

For the controlled fixture-backed GitHub, Jira, and CI reference examples, the local executor maps those records into clearly named runtime-visible adapter telemetry records:

- `AdapterRuntimeAuditRecord`
- `AdapterRuntimeObservabilityRecord`

The local filesystem backend persists those mapped records for the workflow run, and `workflow-os inspect` shows a concise redacted summary. The mapping preserves adapter kind, action, capability, operation mode, policy-precheck provenance, run identity where available, step and skill references, actor, correlation ID, status, error classification, latency, redaction metadata, and response references or summaries.

Adapter invocation and runtime audit telemetry records may also carry validated evidence references for adapter invocation and response summary evidence. This does not add validation-result attachment, approval attachment, generic evidence persistence, CLI evidence rendering, work reports, or reasoning lineage.

This mapping is intentionally narrow. It is not a generic adapter execution framework, not live adapter execution by default, not production telemetry export, and not SIEM or OpenTelemetry integration. Future runtime adapter execution work must still define a broader adapter invocation path before arbitrary workflow specs can execute adapters.

## Non-Goals

v0 adapter contracts do not include:

- OAuth flows
- API clients
- network calls
- webhook receivers
- distributed queues
- production secret providers

Those features require future scoped work after the kernel remains correct under local execution.
