# Trusted-Host Local Cargo Execution-Provider Suitability Re-evaluation

Status: decision complete; provider wiring remains blocked pending a refreshed
version-pinned evidence matrix and focused review.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Trusted-Host Cargo Clippy Live-Execution Security Boundary Plan](trusted-host-cargo-clippy-live-execution-security-plan.md)
- [OpenShell v0.0.101 Evidence-Sufficiency Matrix](openshell-v0-0-101-evidence-sufficiency-matrix.md)
- [OpenShell Upstream Attestation Discussion](https://github.com/NVIDIA/OpenShell/discussions/2661)
- [Trusted-Host Local Cargo Observation Binding Blocker-Fix Review](../concepts/TRUSTED_HOST_LOCAL_CARGO_OBSERVATION_BINDING_BLOCKER_FIX_REVIEW.md)

## 1. Executive Decision

NVIDIA OpenShell remains the preferred optional containment-provider candidate
for trusted-host local Cargo execution, and a fork remains unjustified.

OpenShell `v0.1.5` materially changes the prior assessment. The stable release
adds durable request IDs, resource-version checks for selected mutations,
typed deletion outcomes, driver-observed resolved workload identity and fence
evidence, resource capability negotiation, durable configuration operations,
and stronger lifecycle reconciliation state. These are real upstream advances,
not documentation-only promises.

They do not yet satisfy the complete Workflow OS live-execution contract.
OpenShell still documents exec result replay as unavailable after an admitted
stream is lost, documents an unresolved owner-loss state for a pending driver
operation, and describes OCSF collection as best-effort rather than a complete
operation-bound observation record. Cleanup is stronger but still requires
post-delete observation and does not expose a single immutable terminal
cleanup receipt covering platform reclamation and credential purge.

Decision: **do not wire OpenShell or a native Cargo runner yet**. Replace the
stale `v0.0.101` readiness basis with one exact `v0.1.5` evidence-sufficiency
matrix and focused maintainer/security review. Only if that review closes the
mandatory fact contract should Workflow OS authorize a compatibility transport
spike and separately governed live sandbox smoke.

## 2. Decision Scope

This phase:

- re-evaluated the native trusted-host boundary and optional OpenShell boundary;
- resolved the latest stable OpenShell release and current upstream `main`;
- inspected exact stable protobuf and official documentation surfaces;
- reassessed each previously blocking fact family;
- selected the next bounded evidence phase; and
- preserved the no-fork and no-provider-wiring boundaries.

This phase did not:

- change Rust, runtime, provider, executor, admission, persistence, event, audit,
  schema, SDK, CLI, example, authority, or release behavior;
- install, build, start, or execute OpenShell;
- start Cargo or add a host-process runner;
- perform a live sandbox smoke;
- create or mutate provider credentials or resources;
- enable external writes; or
- claim production sandbox or supply-chain attestation.

## 3. Exact Source Boundary

The official upstream boundaries inspected on 2026-10-24 were:

| Boundary | Exact ref |
| --- | --- |
| Latest stable release | `v0.1.5` |
| Stable release commit | `bdb956f2cd65a635f1294fccf1d138d263bf650f` |
| Official upstream `main` | `c4688aa8715ea75d6c39fe468361dbbc16c691fa` |
| Existing design discussion | [NVIDIA/OpenShell #2661](https://github.com/NVIDIA/OpenShell/discussions/2661) |

Primary stable sources:

- [OpenShell v0.1.5 source](https://github.com/NVIDIA/OpenShell/tree/bdb956f2cd65a635f1294fccf1d138d263bf650f)
- [public gateway API](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/proto/openshell.proto)
- [compute-driver API](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/proto/compute_driver.proto)
- [API retry migration guidance](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/docs/upgrade/0-1-0.mdx)
- [sandbox lifecycle guidance](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/docs/how-it-works/sandboxes/overview.mdx)
- [OCSF JSON export guidance](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/docs/observability/ocsf-json-export.mdx)

The relevant public protobuf contracts are unchanged between `v0.1.5` and the
inspected `main` commit. Current `main` adds observability documentation but
does not remove the best-effort completeness boundary. Stable `v0.1.5`, not
unreleased `main`, is the only candidate compatibility target from this phase.

No release artifact, installed binary, selected driver binary, platform
configuration, or sandbox image was verified. Source inspection is not live
execution evidence.

## 4. Native Trusted-Host Boundary

The native `workflow-local-host` observer is suitable only for process-free
host-input observation. It can bind the selected repository and executable
inputs and produce conservative preflight evidence. It cannot prove:

- filesystem or egress containment during execution;
- process-tree custody from admission through terminal state;
- complete transient-write observation;
- independently observed effective controls;
- restart-safe operation reconciliation; or
- terminal cleanup of a possibly started process tree.

The accepted composition therefore remains correct: absence of accepted
provider observations yields `DeniedBeforeStart` with
`ObservationUnavailable`. A native observer must not be promoted into an
execution provider by adjacency or by adding a direct host-process runner.

## 5. Stable OpenShell Advances

OpenShell `v0.1.5` closes or materially narrows these former gaps:

| Fact family | Stable `v0.1.5` advance | Re-evaluation |
| --- | --- | --- |
| Create identity | `CreateSandboxRequest.request_id` provides durable at-most-once admission and replay for 24 hours | Former create-idempotency blocker materially closed, subject to exact adapter binding and expiry handling |
| Mutation concurrency | Selected sandbox mutations accept `expected_resource_version` | Stronger same-resource mutation safety, but not universal terminal proof |
| Runtime identity | `DriverSandboxStatus.resolved_identity` includes an immutable image/rootfs/config digest | Former image-observation blocker materially closed for drivers that populate and preserve the field |
| Outer fence | `DriverFenceEvidence` binds a generation, evidence digest, and resource claims | Strong candidate for provider-observed generation and fence evidence |
| Capability negotiation | Driver extension metadata and typed resource capabilities are machine-readable | Partial closure; not yet a complete cross-driver security-control capability set |
| Lifecycle ownership | Provisioning tracks pending driver work and a unique `driver_operation_id` | Stronger stale-callback and in-flight ownership protection |
| Delete identity | Typed `DeletionOutcome` plus the immutable targeted sandbox ID distinguish accepted, completed, and absent outcomes | Stronger exact-target reconciliation than the prior boolean response |
| Desired-state application | Durable `ConfigUpdateOperation` records component, target revision, state, outcome, timestamps, and sanitized error | Strong authoritative evidence for configuration application, not command execution |
| Network posture | Complete endpoint status reports bind policy hash, provider environment revision, supervisor session, and monotonic sequence | Stronger bounded network-policy evidence for configured endpoints |

These facts must be assessed as emitted by the actual selected driver and
platform. Schema presence alone does not prove implementation conformance.

## 6. Remaining Mandatory Blockers

### 6.1 Exact Exec Outcome And Restart Reconciliation

`ExecSandboxRequest.request_id` prevents duplicate relaunch, but official
migration guidance states that an admitted exec retry cannot replay stdout,
stderr, exit code, or the stream and returns `REQUEST_STREAM_UNAVAILABLE`.
Workflow OS requires a durable terminal result or an exact
reconciliation-required outcome after caller or gateway interruption. A launch
fence without recoverable terminal state is insufficient.

### 6.2 Driver Operation Owner Loss

The sandbox lifecycle documentation states that a gateway crash before a
driver response is recorded can leave `driver_operation_pending` set
indefinitely, with no API to resolve the lost ownership. Automatic cleanup,
retry, and deletion remain blocked for that record. This is an honest and safe
upstream posture, but it does not satisfy Workflow OS terminal reconciliation.

### 6.3 Complete Operation-Bound Observations

OCSF records are structured and individually useful, but the stable docs call
collection best-effort. Supervisor console output uses a bounded queue and may
drop new lines; file retention is bounded; collection depends on an external
collector; and there is no operation-bound manifest proving start/end
watermarks, flush completion, event/drop counts, integrity commitment, and
coverage for the exact Cargo operation.

The endpoint-status protocol is stronger than generic log export, but it covers
configured endpoint results rather than all filesystem, process, network,
resource, and protected-write observations for one command interval.

### 6.4 Terminal Cleanup Receipt

Typed delete outcomes and targeted sandbox identity are a substantial
improvement. `ACCEPTED` is explicitly asynchronous, and `COMPLETED` may still
precede downstream platform garbage collection. The API does not yet provide
one immutable receipt that binds the exact sandbox generation, platform
reclamation, descendant termination, credential purge, completion time, and
any cleanup degradation.

### 6.5 Complete Effective-Control Posture

Resolved workload identity, fence evidence, policy hashes, load status,
configuration operations, endpoint status, and resource capabilities provide
stronger components. The next matrix must still prove that one selected
gateway/driver/platform combination reports every mandatory filesystem,
process-tree, egress, resource-budget, transient-write, degradation, and
skipped-control fact. Unknown or driver-optional facts must fail closed.

### 6.6 Live Platform Proof

No source contract can prove that a release artifact and selected driver
actually emit and enforce the required facts. Even a matrix with no schema
blocker requires a version-pinned, platform-specific, no-write live smoke with
independent negative tests before provider wiring.

## 7. Provider Suitability Decision

| Candidate | Current suitability | Decision |
| --- | --- | --- |
| Direct native Cargo process | Cannot prove containment, custody, or restart reconciliation | Reject |
| Native observer plus direct process | Observation remains preflight-only and non-authorizing | Reject |
| OpenShell `v0.0.101` | Accepted matrix remains blocked by broad API gaps | Superseded as the next evaluation target |
| OpenShell `v0.1.5` | Materially stronger stable contract; several blockers narrowed or closed | Advance to exact refreshed evidence matrix, not wiring |
| OpenShell current `main` | Relevant protobufs match `v0.1.5`; unreleased changes cannot be a compatibility target | Observe only |
| Workflow OS OpenShell fork | Transfers a large security/runtime maintenance surface without necessity | Reject |

OpenShell is now sufficiently promising to justify renewed exact evaluation.
It is not yet sufficiently proven to authorize execution-provider wiring.

## 8. No-Fork Decision

Do not fork OpenShell.

The remaining needs are general sandbox execution, reconciliation,
observability, and cleanup guarantees. They are not Workflow OS-specific
orchestration semantics. Forking would make Workflow OS responsible for
container, VM, Kubernetes, filesystem, process, network, credential, release,
platform, and vulnerability-management surfaces while fragmenting upstream
security work.

Reconsider a fork only if a version-pinned integration proves an essential
authoritative fact cannot be represented upstream, upstream contribution paths
have been exhausted, and the maintenance/security cost has explicit project
ownership. None of those conditions is met.

## 9. Privacy And Evidence Boundary

A future adapter may project only stable identifiers, versioned commitments,
typed control postures, bounded counts, timestamps, sanitized reason codes,
and governed evidence references into Workflow OS Core. It must not project:

- stdout, stderr, stdin, shell history, or raw OCSF records;
- source, manifest, lockfile, or provider payload contents;
- policy bodies, environment values, credentials, tokens, or headers;
- host paths, endpoint paths, or upstream error bodies; or
- provider self-description as proof of effective enforcement.

Provider uncertainty must remain explicit. Missing, stale, degraded,
best-effort, or unreconciled facts cannot be relabeled as observed success.

## 10. Required Next Phase

Create an **OpenShell `v0.1.5` execution-evidence sufficiency matrix** and
perform focused maintainer/security review.

The matrix must:

1. map every accepted live-execution requirement to its exact enforcing or
   observing component;
2. cite the exact `v0.1.5` typed surface and implementation support for the
   selected driver/platform;
3. classify each fact as authoritative, corroborating, requested, lossy, or
   unavailable;
4. distinguish source-contract support from tested runtime behavior;
5. define exact fail-closed handling for exec stream loss, driver owner loss,
   observation gaps, and asynchronous cleanup;
6. define the smallest compatibility client surface only if all mandatory API
   facts exist; and
7. preserve the requirement for a separate version-pinned live smoke before
   any provider wiring.

Do not implement Rust, install OpenShell, invoke Cargo, add a compatibility
transport, run a sandbox, change admission, or authorize provider wiring in
that phase.

## 11. Validation And Governed Record

Validation required for this decision phase:

- official upstream source refs resolved to exact commits;
- stable and current public contracts compared;
- `npm run check:docs`;
- `git diff --check`; and
- governed phase closure with event and approval-presentation summary.

Governed phase identity:

- workflow ID: `dg/d`;
- run ID: `run-1792849595465611000-2`;
- approval ID:
  `approval/run-1792849595465611000-2/planning-approved`;
- approval presentation ID: `presentation/deec46b358150ccc`;
- approval presentation content hash:
  `deec46b358150cccffd1a1708cb9d37692623acc6981552251c2b205733d9426`;
- approval outcome: granted under standing delegated-maintainer authority for
  the exact decision-only scope; and
- out-of-kernel work: official upstream source resolution and inspection,
  decision analysis, and documentation edits. The kernel governed scope and
  approval but did not inspect upstream source, edit files, or run validation.

Phase-close result:

- phase status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0;
- event kinds: 1 `RunCreated`, 1 `RunValidated`, 1 `RunStarted`, 6
  `StepScheduled`, 8 `PolicyDecisionRecorded`, 1 `ApprovalRequested`, 1
  `ApprovalGranted`, 1 `RunResumed`, 6 `SkillInvocationRequested`, 6
  `SkillInvocationStarted`, 6 `SkillInvocationSucceeded`, and 1
  `RunCompleted`; and
- approval-presentation enforcement: proof-enforced with one matching
  presentation record and event marker.

Validation result:

- `npm run check:docs`: passed under the repository-pinned Node 20 runtime;
- `git diff --check`: passed; and
- Rust formatting, linting, and tests: not run because this bounded phase
  changed documentation only and no Rust or runtime behavior.
