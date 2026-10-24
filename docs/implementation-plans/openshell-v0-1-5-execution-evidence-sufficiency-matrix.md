# OpenShell v0.1.5 Execution-Evidence Sufficiency Matrix

Status: matrix complete; provider wiring remains blocked for the selected
Docker-on-Apple-Silicon evaluation boundary; focused maintainer/security review
required.

Related foundations:

- [Trusted-Host Cargo Clippy Live-Execution Security Boundary Plan](trusted-host-cargo-clippy-live-execution-security-plan.md)
- [Provider Suitability Re-evaluation](trusted-host-local-cargo-execution-provider-suitability-reevaluation.md)
- [OpenShell v0.0.101 Evidence-Sufficiency Matrix](openshell-v0-0-101-evidence-sufficiency-matrix.md)
- [OpenShell Discussion #2661](https://github.com/NVIDIA/OpenShell/discussions/2661)

## 1. Executive Verdict

OpenShell `v0.1.5` is a materially stronger optional execution-provider
candidate than `v0.0.101`, but the selected stable Docker boundary does not
expose every authoritative fact required to run trusted-host local Cargo.

The exact candidate assessed is:

- OpenShell release `v0.1.5`;
- source commit `bdb956f2cd65a635f1294fccf1d138d263bf650f`;
- OpenShell Docker compute driver;
- local Apple Silicon macOS host using the Docker execution substrate; and
- one fixed no-network Cargo invocation after a future compatibility and live
  proof phase.

Stable `v0.1.5` adds durable mutation request IDs, typed deletion outcomes,
selected resource-version checks, typed resolved-workload and fence fields,
durable desired-state operations, endpoint status reports, and stronger
provisioning ownership. Those additions close or narrow several old API gaps.

Provider wiring remains blocked because:

1. admitted exec retries do not replay terminal exit status or stream output;
2. a lost gateway owner can leave a driver operation permanently unresolved;
3. OCSF collection is explicitly best-effort and lacks an operation-bound
   completeness manifest;
4. the Docker driver does not populate the public `resolved_identity` or
   `fence_evidence` fields in its production status projection;
5. Docker capability negotiation does not report the complete mandatory
   filesystem, process-tree, egress, protected-write, degradation, and
   observation capabilities; and
6. deletion lacks one immutable terminal receipt proving downstream platform
   reclamation, descendant termination, and credential purge.

Result: **blocked for compatibility transport and provider wiring**. Do not
implement an `OpenShellNoWriteClient` transport or run a live Cargo smoke from
this matrix. Retain the optional-provider and no-fork posture.

## 2. Scope And Non-Scope

This matrix:

- pins one stable release, driver, and candidate host boundary;
- maps required security facts to exact typed and implementation surfaces;
- distinguishes API vocabulary from selected-driver production behavior;
- classifies fact authority, binding, completeness, and failure posture;
- defines Workflow OS fail-closed outcomes for uncertainty; and
- identifies the remaining upstream gaps.

This matrix does not:

- install, build, start, or execute OpenShell;
- invoke Cargo or add a local process runner;
- add Rust, a compatibility transport, provider wiring, or a live smoke;
- change admission, persistence, events, audit, reports, schemas, SDKs, CLI,
  examples, authority, writes, or release posture;
- verify release assets, installed binaries, Docker daemon configuration, or
  runtime images; or
- claim production sandbox, cryptographic, hardware, or supply-chain
  attestation.

## 3. Exact Source Boundary

Primary official stable sources:

- [OpenShell v0.1.5 source](https://github.com/NVIDIA/OpenShell/tree/bdb956f2cd65a635f1294fccf1d138d263bf650f)
- [gateway API](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/proto/openshell.proto)
- [compute-driver API](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/proto/compute_driver.proto)
- [Docker driver](https://github.com/NVIDIA/OpenShell/tree/bdb956f2cd65a635f1294fccf1d138d263bf650f/crates/openshell-driver-docker)
- [API errors and retries](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/docs/sdk/api-errors.mdx)
- [sandbox lifecycle](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/docs/how-it-works/sandboxes/overview.mdx)
- [OCSF JSON export](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/docs/observability/ocsf-json-export.mdx)
- [gateway OCSF configuration](https://github.com/NVIDIA/OpenShell/blob/bdb956f2cd65a635f1294fccf1d138d263bf650f/docs/how-it-works/gateways/configuration.mdx)

Relevant public protobuf contracts are unchanged at inspected upstream `main`
commit `c4688aa8715ea75d6c39fe468361dbbc16c691fa`. Stable `v0.1.5` remains the
only compatibility candidate assessed here.

## 4. Classification Rules

| Classification | Meaning |
| --- | --- |
| Authoritative | Emitted by the enforcing or observing component, exact and complete for the stated fact |
| Corroborating | Useful independent observation without enough authority, binding, or completeness |
| Requested | Caller or control-plane intent, not proof of applied runtime state |
| Lossy | Structured or bounded observation with known gaps or drop posture |
| Unavailable | No trustworthy selected-boundary fact exists |

An authoritative classification is fact-specific. A durable admission fence
can be authoritative for at-most-once launch while remaining insufficient for
terminal execution outcome.

## 5. Release, Gateway, Driver, And Capability Identity

| Required fact | Stable surface | Selected Docker behavior | Classification | Result |
| --- | --- | --- | --- | --- |
| Source release and schema | Exact tag, commit, and public protobuf source | Exact for reviewed source only | Authoritative for source provenance | Accept as source pin |
| Running gateway version | Gateway info response | Typed self-report; no independent binary digest in this phase | Authoritative self-report, corroborating supply-chain identity | Require separate binary verification later |
| Driver identity | Driver extension metadata, name, and version | Docker reports `openshell/docker` and implementation version | Authoritative self-report | Accept narrowly |
| Extension protocol | `PeerMetadata` and supported capability strings | Docker reports compute contract metadata | Authoritative for negotiated protocol metadata | Accept narrowly |
| Resource limits | `ResourceCapabilities` | Docker reports CPU and memory limit support; GPU follows CDI configuration | Authoritative for reported portable resource forms | Accept only reported fields |
| Full security capabilities | No complete typed set | Docker does not negotiate filesystem, process-tree, egress, protected-write, observation-completeness, or cleanup-receipt capabilities | Unavailable | Blocking |
| Provider configuration commitment | Workflow OS provider-neutral identity | Not implemented in this phase | Authoritative for future Workflow OS input only | Preserve separately |

## 6. Invocation And Sandbox Identity

| Required fact | Stable surface | Binding and completeness | Classification | Result |
| --- | --- | --- | --- | --- |
| Workflow OS request identity | `CreateSandboxRequest.request_id` | Nonzero UUID gives durable at-most-once admission and success replay for 24 hours when identity, scope, method, and payload match | Authoritative for bounded admission replay | Candidate acceptance with expiry/replay rules |
| Exact request commitment | Mutation replay validates same request identity and payload | Binds upstream request payload, not the future Workflow OS immutable run bundle unless adapter constructs and commits both exactly | Authoritative upstream; requested until adapter-bound | Require exact adapter binding later |
| Stable sandbox identity | Gateway sandbox ID and metadata | Gateway-owned stable resource ID | Authoritative | Accept |
| Workspace and creation identity | Gateway metadata | Typed and resource-bound | Authoritative | Accept |
| Resource version | Sandbox object resource version | Strong observation and selected-mutation precondition; not accepted by every lifecycle request | Authoritative for observed object version | Accept narrowly |
| Driver runtime identity | Docker create response runtime identity | Docker advertises `supports_sandbox_authentication=false`; runtime identity is not the selected binding mechanism | Unavailable for selected capability | Do not require as Docker proof |
| Restart-safe create replay after 24 hours | No durable replay beyond documented retention | Expired replay cannot prove the old create outcome by request ID alone | Unavailable | Blocking for delayed reconciliation; constrain or retain uncertainty |

## 7. Policy And Effective-Control State

| Required fact | Stable surface | Binding and completeness | Classification | Result |
| --- | --- | --- | --- | --- |
| Structured requested policy | Sandbox spec policy | Typed desired input | Requested | Never treat as enforcement proof |
| Effective policy payload and hash | Gateway-composed policy, hash, version, source, global version, config revision | Typed gateway desired state | Authoritative for gateway composition | Accept |
| Policy load result | Sandbox policy status and revision result | Authenticated supervisor reports active revision and bounded result | Authoritative for reported application | Accept with producer boundary |
| Durable desired-state operation | `ConfigUpdateOperation` | Binds sandbox, component, target revision, state, outcome, timestamps, sanitized error | Authoritative for that desired-state application | Accept |
| Filesystem enforcement snapshot | No complete public selected-driver receipt | Docker implementation may enforce internal boundaries, but no complete typed operation-bound applied-state result is projected | Unavailable | Blocking |
| Process-tree enforcement snapshot | No complete public receipt | No typed terminal tree/cgroup/job state bound to one exec | Unavailable | Blocking |
| Egress enforcement snapshot | Effective policy plus endpoint status and OCSF observations | Endpoint status is revision- and session-bound but does not prove all operation traffic or interval completeness | Corroborating | Blocking alone |
| Degraded/skipped controls | Config outcomes and conditions expose selected degradation | No exhaustive selected-driver security posture across all mandatory controls | Corroborating | Blocking |

## 8. Driver-Observed Workload And Fence Evidence

| Required fact | Stable surface | Selected Docker behavior | Classification | Result |
| --- | --- | --- | --- | --- |
| Requested image | Sandbox template image | Desired provisioning input | Requested | Reject as observation |
| Resolved immutable image/rootfs/config digest | `DriverSandboxStatus.resolved_identity.resource_digest` | Docker resolves an image ID internally for workload identity, but production status construction does not populate `resolved_identity` | Unavailable through selected public status | Blocking |
| Resolved UID/GID/groups | `resolved_identity` | Docker computes the identity internally but does not project it in production status | Unavailable through selected public status | Blocking |
| Generation-bound outer fence | `DriverSandboxStatus.fence_evidence` | Typed field exists, but Docker production status construction does not populate it | Unavailable | Blocking |
| Platform object identity | Status name and instance ID | Docker reports container name and ID in observed summaries | Authoritative for observed object identity | Accept as context only |
| Drift detection | Repeated selected status | Missing identity/fence projection prevents exact required comparison | Unavailable | Blocking |

The distinction is decisive: protobuf vocabulary is not evidence. The selected
Docker crate resolves workload identity while preparing its boundary, but its
production `DriverSandboxStatus` construction uses default values for
`resolved_identity` and `fence_evidence`. Tests of internal identity resolution
do not make those facts available to a Workflow OS adapter.

## 9. Fixed Operation Identity And Terminal Outcome

| Required fact | Stable surface | Binding and completeness | Classification | Result |
| --- | --- | --- | --- | --- |
| Fixed command request | Future Workflow OS adapter plus `ExecSandboxRequest.command` | Workflow OS can close arguments; upstream accepts typed argv | Authoritative for future adapter input only | Preserve |
| Durable exec admission | `ExecSandboxRequest.request_id` | Duplicate request never relaunches or reattaches | Authoritative for launch fence | Accept narrowly |
| Sandbox binding | Workspace and canonical sandbox name in exec request | Routed to selected sandbox; stream events do not carry a durable operation resource | Requested plus transport binding | Insufficient alone |
| Live stdout/stderr | Exec stream frames | Raw payload, connected-stream only | Authoritative for received chunks, forbidden from Core | Never use as governance evidence |
| Live terminal exit code | Final exec stream frame | Exact only while the caller receives the terminal frame | Authoritative for connected stream | Accept only before durable projection succeeds |
| Restart terminal reconciliation | `REQUEST_STREAM_UNAVAILABLE` after admitted completion | Officially does not replay output, exit code, stream, or stdin | Unavailable | Blocking |
| Timeout/signal/cancel reason | Execution timeout plus transport status | No durable operation resource with complete terminal reason | Unavailable | Blocking |
| Start/finish time | No durable exec operation resource | OCSF timestamps are not exact operation lifecycle receipts | Unavailable | Blocking |

## 10. Driver Operation Ownership And Recovery

| Required fact | Stable surface | Binding and completeness | Classification | Result |
| --- | --- | --- | --- | --- |
| Pending driver work | `SandboxProvisioning.driver_operation_pending` | Persisted while an owned create/start call is outstanding | Authoritative | Accept |
| Operation identity | `driver_operation_id` | Unique per claimed driver operation and retained to reject stale callbacks | Authoritative | Accept |
| Owner-loss resolution | Official lifecycle posture | Gateway crash before recording response can leave pending state indefinitely; no resolution API | Unavailable | Blocking |
| Safe retry during pending work | Gateway blocks start, stop, delete, and cleanup completion | Strong fail-closed behavior | Authoritative | Accept as safety, not completion |
| Terminal reconciliation after owner loss | None | Cannot establish whether exact backend operation finished | Unavailable | Blocking |

## 11. Structured Security Observations

| Required fact | Stable surface | Binding and completeness | Classification | Result |
| --- | --- | --- | --- | --- |
| Individual OCSF record | Supervisor/gateway JSON output | Complete structured object for each record actually retained | Authoritative for that record only | Accept as source material outside Core |
| Gateway JSONL loss metrics | Queue/write/drop counters and recovery-discarded bytes | Bounded metrics; discarded bytes do not prove lost-record count | Corroborating | Insufficient |
| Gateway OTLP gap records | Drop count and first-loss time for selected loss paths | Shutdown loss has no gap record; collector partial rejection needs collector metrics | Lossy | Blocking |
| Supervisor JSON stderr | Bounded 1,024-line queue | Drops new lines under pressure | Lossy | Blocking |
| Supervisor JSONL files | Local rotating files | Best-effort, finite retention, external collection required before teardown | Lossy | Blocking |
| Endpoint status | Policy hash, provider revision, supervisor session, monotonic sequence, complete configured endpoint inventory | Strong for latest configured endpoint status; not operation interval coverage | Authoritative for reported endpoint snapshot | Corroborating for Cargo operation |
| Operation watermarks/manifest | None | No exact start/end cursor, flush acknowledgement, event/drop count, digest, and coverage manifest | Unavailable | Blocking |
| Denied-egress observation | Endpoint/OCSF denial | Strong individual denial fact, not proof that every attempted egress was observed | Corroborating | Insufficient alone |
| Protected-write attempts | No complete operation-bound public record | Cannot prove transient protected-write coverage | Unavailable | Blocking |

Raw OCSF records, command I/O, paths, endpoint paths, policy bodies, provider
errors, environment values, and credentials must remain outside Workflow OS
Core. A future adapter may project only bounded commitments, counts, postures,
and stable evidence references after completeness is proven.

## 12. Cleanup And Reconciliation

| Required fact | Stable surface | Binding and completeness | Classification | Result |
| --- | --- | --- | --- | --- |
| Delete admission | `request_id` on delete | Durable bounded at-most-once mutation admission | Authoritative | Accept narrowly |
| Exact target | `DeleteSandboxResponse.sandbox_id` | Identifies targeted immutable sandbox; same-name replacement is excluded | Authoritative after response/replay | Accept |
| Typed outcome | `DeletionOutcome` | Distinguishes completed, accepted, and already absent | Authoritative for gateway outcome | Accept narrowly |
| Asynchronous progress | Observe targeted sandbox ID until disappearance | Stronger than name polling | Corroborating terminal platform cleanup | Insufficient alone |
| Platform reclamation | Driver deletion observation and later absence | No immutable public terminal cleanup receipt | Corroborating | Blocking |
| Descendant termination | Docs state deletion stops processes | Human contract, not per-operation typed proof | Requested/Corroborating | Blocking |
| Credential purge | Docs state deletion purges credentials | No per-operation typed fact | Requested | Blocking |
| Cleanup completion/degradation | Provisioning cleanup fields cover selected timeout recovery | Not a universal exact deletion receipt for the execution generation | Corroborating | Blocking |

## 13. Fail-Closed Workflow OS Mapping

| OpenShell posture | Required Workflow OS outcome |
| --- | --- |
| Required Docker identity or fence field empty | `DeniedBeforeStart` with missing authoritative provider observation |
| Unsupported mandatory capability | `DeniedBeforeStart` with unsupported effective control |
| Request outcome uncertain before confirmed admission | Preserve exact request ID and reconcile; do not submit a new ID |
| `REQUEST_STREAM_UNAVAILABLE` | `ReconciliationRequired`; never infer exit status or success |
| `driver_operation_pending` after owner loss | `ReconciliationRequired`; do not retry, delete, or report completion |
| OCSF/gap/drop/completeness uncertainty after start | `ReconciliationRequired`; do not issue complete evidence posture |
| Delete `ACCEPTED` | Cleanup pending; run remains non-terminal for provider lifecycle |
| Delete `COMPLETED` without exact platform/credential receipt | Cleanup unproven; preserve reconciliation requirement |
| Provider policy/control degradation | Deny before start when known pre-start; reconcile if discovered after start |
| Exact terminal facts and cleanup unavailable | No successful WorkReport or terminal governed artifact may claim completion |

## 14. Privacy And Error Posture

The future provider boundary may carry:

- stable provider, request, operation, sandbox, generation, and evidence IDs;
- version and resource commitments;
- typed capability and control postures;
- bounded counts and timestamps;
- sanitized stable reason codes; and
- governed evidence references.

It must reject or redact raw command output, OCSF payloads, policy bodies,
source or manifest content, environment values, credentials, authorization
material, host paths, endpoint paths, provider error bodies, and token-like
values. Debug and error paths must not expose those values.

## 15. Blocking Gaps And Decision

Provider wiring is blocked on these selected-boundary facts:

1. Docker projection of non-empty resolved workload identity and generation-
   bound fence evidence;
2. durable exact exec terminal outcome lookup after stream loss;
3. resolution of pending driver work after gateway-owner loss;
4. operation-bound complete security observation manifest;
5. complete typed mandatory security capability negotiation;
6. complete applied filesystem, process-tree, egress, protected-write, and
   degradation posture; and
7. exact terminal cleanup receipt including platform reclamation, descendant
   termination, and credential purge.

Because mandatory API and selected-driver facts are unavailable, this matrix
does **not** authorize a compatibility client or live smoke. A smoke cannot
repair a missing contract by scraping logs or inferring state.

## 16. Upstream And Fork Posture

OpenShell remains the preferred optional provider candidate. The existing
[Discussion #2661](https://github.com/NVIDIA/OpenShell/discussions/2661) has no
recorded upstream maintainer acceptance as of this review; third-party comments
must not be treated as an upstream commitment.

Do not fork OpenShell. The gaps are general execution-provider attestation and
reconciliation needs. A fork would transfer a large security/runtime surface
to Workflow OS before upstream incompatibility or contribution failure has
been demonstrated.

## 17. Recommended Next Phase

Perform a focused maintainer/security review of this matrix.

If accepted, update the upstream gap record or discussion draft with the exact
`v0.1.5` delta and selected Docker findings. Do not implement a compatibility
transport or live smoke unless a later stable upstream release closes the
mandatory API and Docker projection gaps and a new matrix accepts it.

## 18. Governed Phase Record

- Workflow ID: `dg/d`.
- Run ID: `run-1792850704615021000-2`.
- Approval ID:
  `approval/run-1792850704615021000-2/planning-approved`.
- Approval presentation ID: `presentation/3344bb808ef230f4`.
- Approval presentation content hash:
  `3344bb808ef230f429fec226ecd6e3d24a0ea8641bc3afd308f55debcf3cfa12`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact source-analysis scope.
- Out-of-kernel work: the delegated maintainer inspected official stable
  source and documentation, built this matrix, updated roadmap/docs, and ran
  validation. The kernel governed scope and approval but did not inspect
  upstream source, edit files, or run validation.

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
- Rust formatting, linting, and tests: not run because this phase changed
  documentation only and no Rust or runtime behavior.
