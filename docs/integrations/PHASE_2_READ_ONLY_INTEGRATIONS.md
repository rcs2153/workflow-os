# Phase 2 Read-Only Integrations

Workflow OS `0.1.0-preview.1` remains the public local-kernel preview. Workflow OS `0.2.0-preview.1` is the narrow public read-only integration preview layered on that local kernel. Its purpose is to add real read-only integration capability without turning Workflow OS into a brittle GitHub, Jira, or CI automation tool.

Phase 2 read-only adapters are approved only for read-only preview use. They remain fixture-first in normal CI and opt-in for live providers.

This document is the integration boundary for current and future Phase 2 work.

## Phase 2 Goal

Phase 2 should:

- Add real read-only integration capability.
- Prove the generic adapter contract against real external systems.
- Preserve Workflow OS as a generic framework for governed AI workflows.
- Keep external write operations out of scope.
- Keep the runtime policy, audit, idempotency, and redaction model authoritative.

Phase 2 is not a production integration release. It is a careful adapter-contract proving phase that is public-previewed only for read-only evaluation.

## In-Scope Integrations

The first read-only integrations are:

- GitHub read-only.
- Jira read-only.
- CI read-only, with GitHub Actions first.

Read-only means retrieving external facts and references through adapters. It does not mean mutating repositories, issues, comments, workflow runs, checks, build jobs, or external statuses.

## Explicitly Out Of Scope

Phase 2 must not include:

- Creating branches.
- Opening pull requests.
- Posting pull request comments.
- Merging pull requests.
- Updating Jira issues.
- Adding Jira comments.
- Changing Jira status.
- Rerunning CI.
- Workflow dispatch.
- Webhooks or an event ingestion service.
- OAuth app implementation.
- Hosted service behavior.
- Distributed workers.
- Production database backend.
- Level 3 or Level 4 autonomy enablement.

These exclusions are product boundaries, not merely implementation backlog.

## Read-Only Integration Principles

All Phase 2 integrations must follow these rules:

- External reads must go through adapters.
- Adapters must not mutate workflow state directly.
- Adapters must not bypass policy.
- Adapter calls must produce contract-level adapter telemetry records. Controlled fixture-backed examples may map those records into local runtime-visible adapter telemetry records, but that mapping is not generic adapter execution.
- Adapter errors must be classified.
- Adapter requests must carry explicit policy-precheck provenance.
- Adapter responses must avoid storing raw sensitive payloads by default.
- Adapter request and response summaries must be redacted.
- Credentials must never be stored in specs.
- Credentials must be loaded only from environment variables or documented local secret references.
- Fixtures must exist for offline tests.
- Live integration tests must be opt-in and skipped by default.

Read-only adapters still cross a security boundary. Possessing a read credential is not enough to bypass Workflow OS policy, audit, redaction, or capability checks.

## Capability Model

Phase 2 introduces or reserves these read-only capabilities:

- `github.read`
- `jira.read`
- `ci.read`

These capabilities allow adapter implementations to request read-only access, subject to policy approval and credential availability. They are not blanket permissions.

The following capabilities remain denied, unsupported, or out of scope for Phase 2:

- `github.write`
- `jira.write`
- `ci.write`
- `ci.rerun`
- `adapter.write`

Unknown capabilities continue to fail closed.

## Credential Handling

Specs must not contain tokens, passwords, API keys, private keys, OAuth refresh tokens, or session material.

Phase 2 adapters may read credentials only from:

- documented environment variables
- documented local secret references

Credential loading must be adapter-local and auditable without writing raw credential values to workflow specs, runtime events, audit events, observability events, diagnostics, or logs.

## Test Posture

Phase 2 tests must distinguish mock, fixture, and live behavior.

- Unit tests use mocks and static fixtures.
- Contract tests validate adapter behavior against the generic adapter interfaces.
- Fixture tests must run offline and without credentials.
- Live integration tests must be opt-in through explicit environment variables.
- CI must not require live credentials.
- CI must not call live GitHub, Jira, or CI APIs by default.

Live tests may prove that an adapter can read from a real service, but they do not replace offline contract tests.

## Documentation Honesty

Phase 2 docs must be explicit about mode and scope:

- Fixture mode uses static files.
- Mock mode uses deterministic local test doubles.
- Live mode calls real read-only APIs and is opt-in.

Docs must not claim:

- write support
- production integration readiness
- OAuth app readiness
- webhook ingestion
- distributed execution
- hosted operation
- production database readiness
- Level 3/4 autonomy enablement

If a document describes a future write operation, it must mark it as future work outside Phase 2.

## Adapter Contract Implications

Future Phase 2 implementation prompts must preserve the existing adapter rules:

- Adapter requests carry correlation IDs.
- Side-effecting requests require idempotency keys, even though Phase 2 avoids writes.
- Read responses store redacted summaries and external references by default.
- Errors classify authentication failure, permission failure, rate limit, not found, validation failure, transient failure, and unknown failure.
- Policy checks happen before adapter calls.
- Adapter request helpers do not silently grant policy approval; runtime, approval, fixture/test, denied, and missing prechecks must be distinguishable.
- Contract-level adapter invocation and observability records are produced for adapter attempts and outcomes.

The first Phase 2 success criterion is not breadth. It is proving that a small number of real read-only adapters can obey the same generic kernel contract.
