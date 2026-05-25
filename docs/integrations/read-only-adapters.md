# Read-Only Adapters

Read-only adapters are the Phase 2 development-branch integration surface for Workflow OS. They allow the kernel to retrieve external facts and references from real systems without mutating those systems.

The GitHub, Jira, and GitHub Actions read-only adapters are implemented first for internal review. They are not part of the `0.1.0-preview.1` local kernel release contract, and they are not a public read-only integration preview until a follow-up maintainer review approves that posture. This document defines the contract all read-only adapters must obey.

## Scope

Read-only adapters may:

- Retrieve external metadata.
- Retrieve external object summaries.
- Retrieve external status or run information.
- Return external references such as URLs, IDs, issue keys, pull request numbers, commit SHAs, workflow run IDs, or job IDs.
- Return bounded, redacted excerpts only when explicitly requested.
- Classify provider errors.
- Emit audit-safe adapter invocation records.
- Emit observability records.

Read-only adapters must not:

- Create branches.
- Open pull requests.
- Post comments.
- Update issues.
- Change statuses.
- Rerun CI.
- Dispatch workflows.
- Mutate external systems through side effects.

## Request Requirements

Every read-only adapter request must include:

- adapter ID
- adapter kind
- action
- read capability
- operation mode
- actor or system actor
- correlation ID
- timeout policy
- redaction policy
- non-secret request metadata
- policy pre-check

Run-scoped requests must additionally include:

- workflow run ID
- workflow ID
- workflow version
- schema version
- spec content hash

Read-only operations usually do not require idempotency keys because they do not create external side effects. If an adapter uses an idempotency strategy other than `not_required_for_read_only`, the request must carry an idempotency key.

## Operation Modes

Adapters must label their execution mode:

- `fixture`: static offline fixture behavior.
- `mock`: deterministic mock behavior for tests.
- `local`: local non-network behavior.
- `live-read-only`: real read-only calls to an external service.
- `live-write-capable`: future write-capable mode; denied in Phase 2.

CI must not require `live-read-only` mode. Live tests must be opt-in through explicit environment variables.

## Policy And Capability Enforcement

Read-only adapter actions must require a declared read capability. Missing, unknown, or write-only capabilities fail closed.

Write-capable actions fail closed in Phase 2 even if the request includes an idempotency key or policy allow result. This is deliberate: Phase 2 proves safe reads only.

Adapters must not treat credentials as permission. Runtime policy must allow the adapter invocation before the adapter call occurs.

## Response Requirements

Adapter responses must include:

- adapter ID
- action
- success or failure status
- normalized non-secret result summary
- external references
- redaction metadata
- response size metadata
- correlation ID
- duration or latency metadata
- warnings

Responses should store references and summaries rather than raw provider payloads. Large or sensitive provider responses must remain outside audit and observability records by default.

## Audit And Observability

Each adapter invocation must produce:

- an audit-safe adapter invocation record
- an observability record
- latency measurement
- success or failure status
- error classification for failures
- redaction metadata
- run identity when run-scoped

These records are the operator-facing evidence that an external read was attempted and how it completed.

## Credentials

Credentials must never be stored in specs.

Future read-only adapters may load credentials only from documented environment variables or documented local secret references. Health checks may report whether credentials are present, but must never expose credential values, token prefixes, headers, private keys, or secret payloads.

## Testing

Read-only adapter tests must include:

- unit tests using mocks
- fixture tests that run offline
- contract tests against the generic adapter traits
- opt-in live tests that are skipped by default
- redaction tests
- policy-denial tests
- health output tests proving credentials are not exposed

Live tests do not replace offline contract tests.
