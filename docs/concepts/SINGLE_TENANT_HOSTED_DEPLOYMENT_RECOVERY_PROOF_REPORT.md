# Single-Tenant Hosted Deployment And Recovery Proof Report

Report date: 2026-07-29

## 1. Executive Summary

The single-tenant hosted alpha now proves one authenticated, server-owned,
no-write governed run from API creation through stateless worker execution,
authoritative terminal events, and durable report metadata.

A successfully receipted no-write run creates its validated `WorkReport`
artifact inside the same fenced `PostgreSQL` transaction as the receipt,
attempt, work item, terminal events, snapshot, and lease release. The isolated
compose rehearsal verifies API restart, worker completion and restart,
database-interruption readiness failure, recovery, and terminal report
readability. Existing live `PostgreSQL` conformance remains the source of
lease-takeover, stale-fence, schema-mismatch, backup/restore,
projection-rebuild, and immutable-bundle recovery proof.

This completes the single-tenant no-write evaluation milestone. It does not
claim production readiness, provider mutation, credential resolution,
multi-tenancy, enterprise identity, HA, PITR, or OpenShell integration.

## 2. Scope Completed

- Added a server-owned hosted recovery project with one validated no-write
  workflow, skill, policy, and test definition.
- Updated the compose topology to use the recovery fixture and one shared
  evaluation image for API and worker.
- Added an isolated hosted recovery CI job.
- Extended the rehearsal to:
  - start `PostgreSQL` and API without a worker;
  - create one authenticated idempotent governed run;
  - restart the API and re-read the queued run;
  - start the worker and wait for authoritative completion;
  - inspect the terminal event trail and report metadata;
  - restart the worker;
  - interrupt `PostgreSQL` and require readiness failure;
  - restore `PostgreSQL` and re-read the same run and report.
- Added `HostedTerminalReportArtifact` derivation for exactly bound,
  successfully receipted no-write runs.
- Added atomic `PostgreSQL` report persistence and exact replay validation.
- Added authenticated latest-terminal-report metadata retrieval for one run.

## 3. Scope Explicitly Not Completed

- No provider write or additional provider family.
- No credential or access-material resolver.
- No shell, filesystem, process, network, browser, or model execution.
- No OpenShell adapter or fork.
- No report generation for failed, canceled, rejected-before-start,
  ambiguous, or reconciliation-required hosted outcomes.
- No full report-body API.
- No multi-tenancy, enterprise identity, hosted UI, Kubernetes requirement,
  HA, PITR, capacity, SLO, or production disaster-recovery claim.
- No schema, CLI, example, release, or production-readiness change.

## 4. Terminal Report Boundary

`HostedTerminalReportArtifact::derive` accepts only an exact completed
`HostedTerminalResultProjection` and its exact no-write work item. It requires:

- a completed provider receipt;
- a completed projected run;
- exactly `SkillInvocationSucceeded` and `RunCompleted` terminal events;
- exact run, workflow, schema, workflow-version, bundle, and root-hash
  identity;
- no approved `SideEffect`;
- no access-material references;
- read-only capabilities;
- an exactly bound provider receipt.

The report cites the terminal workflow event IDs and bounded hosted
receipt/environment/telemetry references. It copies no provider payload.

## 5. Atomicity And Replay

The completed-receipt transaction now commits:

- terminal workflow events;
- rehydrated run snapshot;
- completed work item;
- terminal invocation attempt;
- exact execution receipt;
- validated terminal report artifact;
- fenced lease release.

An exact replay requires the same receipt, work item, events, and report
artifact. Missing or substituted report state fails closed with a stable
non-leaking conflict.

## 6. Deployment And Recovery Proof

The compose rehearsal proves the deployed process boundary:

- the API owns no authoritative run memory;
- a queued run remains readable after API restart;
- a separately started worker completes the exact queued run;
- worker restart does not change terminal state;
- readiness becomes unhealthy while `PostgreSQL` is unavailable;
- the same run and exact report metadata remain readable after database
  process recovery.

The live `PostgreSQL` conformance/recovery job separately proves expired-lease
takeover, monotonic fencing, stale-fence rejection, schema checksum closure,
logical backup/restore, projection rebuild, and immutable-bundle readability.

## 7. Privacy And Security Posture

- The rehearsal uses CI-only credentials and does not print them.
- The API remains host-bound in the evaluation topology.
- The report stores stable references, not provider output.
- Report construction uses validated `WorkReport` constructors.
- Debug output for the hosted report wrapper is redacted.
- Errors use stable platform-owned codes and bounded summaries.
- The provider remains inert and rejects writes, credentials, and non-read
  capabilities.

## 8. Test Coverage

Focused Rust tests cover:

- successful hosted report derivation;
- terminal workflow-event and hosted telemetry citations;
- rejection of ambiguous terminal posture;
- atomic report persistence with completed receipt projection;
- exact replay with the same report;
- rejection of report substitution;
- authenticated report route posture;
- compatibility with existing hosted failure and reconciliation paths.

The hosted CI rehearsal covers the deployed API-to-worker-to-report path.
The live `PostgreSQL` job covers database transaction and recovery behavior.

## 9. Validation

The phase validation completed successfully:

- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check`: passed under the repository Node 20 toolchain
- `npm run check:integrations`: passed
- `npm run check:docs`: passed
- hosted fixture validation: passed with expected experimental lifecycle
  warnings
- `bash -n scripts/rehearse-hosted-alpha.sh`: passed
- `git diff --check`: passed

Docker is not installed in the desktop development environment. The compose
rehearsal therefore runs as a dedicated mandatory Linux CI check. Live
`PostgreSQL` assertions likewise remain CI-gated when no local test database
URL is configured.

## 10. Remaining Limitations

1. Authentication is one rotatable deployment token, not production identity.
2. API and worker still share one runtime database role.
3. The no-write provider is deterministic and inert.
4. Non-success hosted outcomes do not yet produce terminal reports.
5. The alpha has no credential resolver or mutation authority.
6. Recovery proof is not HA, PITR, RTO, RPO, capacity, or SLO evidence.
7. Report retrieval exposes metadata only.

## 11. Recommended Next Phase

Return to the accepted **runtime proportional-governance composition** lane,
then continue scoped runtime authority and capability projection.

Use the hosted alpha as a real runtime consumer of those accepted governance
decisions. Do not add another provider mutation family or fork OpenShell
first. A future OpenShell integration should be an optional execution-provider
adapter after its policy, receipt, event-loss, idempotency, and recovery
surfaces are reviewed.

## 12. Governed Phase Record

- dogfood workflow: `dg/implement`
- run ID: `run-1785388452917428000-2`
- approval ID:
  `approval/run-1785388452917428000-2/implementation-approved`
- approval presentation: `presentation/7293775e37ddffa4`
- approval outcome: granted with persisted presentation proof
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations

Repository edits, shell commands, tests, documentation, and git/PR actions are
performed by Codex outside the kernel. The kernel governs the phase; it does
not execute those actions.
