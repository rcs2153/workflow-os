# OpenShell Upstream API Attestation Contract Plan

Status: accepted by focused maintainer review; the version-pinned OpenShell
v0.0.101 evidence-sufficiency matrix is complete and provider wiring remains
blocked. No runtime implementation is authorized.

## 1. Executive Summary

Workflow OS should continue toward an optional NVIDIA OpenShell no-write
execution provider, not a fork and not a default runtime. The existing
provider-neutral hosted contract and disconnected OpenShell CLI compatibility
transport prove the architecture boundary, but the reviewed pinned CLI does
not supply every fact required for an authoritative execution receipt.

This plan defines the smallest upstream/API attestation contract that must be
proven before `OpenShellNoWriteClient` can be implemented. It does not wire a
live provider, invoke a sandbox, add access material, enable writes, expose a
workflow schema, or claim production readiness.

## 2. Architecture Decision

Keep the boundary:

```text
Workflow OS governed run
  -> immutable operation and authority
  -> policy, approval, evidence, and report obligations
  -> optional OpenShell execution provider
  -> sandbox-enforced filesystem, process, network, and inference controls
  -> payload-free attestation facts and stable references
  -> Workflow OS validation, event projection, evidence, and WorkReport
```

OpenShell owns execution containment and sandbox lifecycle. Workflow OS owns
whether execution may start, what facts are required, whether those facts are
sufficient, how ambiguity is reconciled, and what enters the governed record.
Neither system may treat a requested setting as proof of effective execution.

## 3. Goals

- Define an upstream-neutral fact contract for one fixed no-write operation.
- Separate requested configuration, committed control-plane state, and
  driver-observed runtime state.
- Bind exact policy input and effective policy to one sandbox revision.
- Require immutable runtime-image identity observed by the execution driver.
- Require complete structured observations for a bounded execution interval.
- Require machine-readable cleanup and reconciliation facts.
- Preserve conservative attempt posture and restart safety.
- Keep raw policy, log, command-output, and provider payloads outside Core.
- Define objective evidence-sufficiency gates for SDK, gateway API, CLI, or
  independently verified observation surfaces.

## 4. Non-Goals

This plan does not authorize:

- live OpenShell installation, gateway startup, sandbox creation, or commands;
- `OpenShellNoWriteClient` implementation or provider registration;
- automatic or default OpenShell selection;
- a Workflow OS-specific OpenShell distribution or fork;
- access-material, provider, or inference routing;
- provider writes or any external mutation family;
- arbitrary commands, interactive shells, or agent teams;
- workflow schema, SDK, CLI, or example changes;
- raw OCSF, policy, source, artifact, or command-output persistence in Core;
- hosted multi-tenancy, enterprise identity, or administration;
- cryptographic, hardware-rooted, or production security claims; or
- release-posture changes.

## 5. Current Foundations

Workflow OS already provides:

- immutable run bundles and exact hosted request fingerprints;
- durable hosted attempts with `NotStarted` and `MayHaveStarted` posture;
- provider identity, version, and configuration binding;
- provider-neutral environment, policy, control, observation, cleanup, and
  receipt vocabulary;
- an injected `OpenShellNoWriteClient` contract and scripted provider tests;
- a disconnected, version-pinned CLI compatibility transport with bounded
  subprocess handling and drift-detecting reconciliation; and
- EvidenceReference and WorkReport citation foundations.

The existing OpenShell provider remains intentionally unusable with the CLI
transport because the transport cannot honestly construct all required facts.

## 6. Upstream Baseline Reverification

OpenShell is alpha software and its public surfaces are moving. The existing
v0.0.101 compatibility seam is a reviewed historical pin, not proof that a
newer release is compatible or more authoritative.

Before implementation, select one upstream release or commit and record:

- repository commit and release identity;
- CLI, SDK, gateway, supervisor, and driver versions;
- release-asset digest and installation provenance;
- consumed protobuf/API schema revision;
- tested driver and platform;
- sandbox image reference and resolved digest; and
- upgrade and rollback boundaries.

Do not compare version numbers to infer compatibility. Re-run the complete
fact matrix against the selected release.

## 7. Required Attestation Facts

### 7.1 Invocation Identity

Required:

- Workflow OS execution ID, request fingerprint, and durable attempt identity;
- OpenShell sandbox ID and resource version;
- provider and gateway identity/version/configuration commitment;
- idempotent lookup or reconciliation key that survives caller restart; and
- timestamps or ordered revisions sufficient to bound observation order.

A sandbox name or label alone is not sufficient.

### 7.2 Exact Policy Input

Required before creation:

- canonical digest of the exact policy bytes Workflow OS supplied;
- policy source and precedence posture, including global/default/provider
  composition that may replace or augment sandbox input;
- explicit prohibition of unreviewed environment-default policy selection;
- static versus dynamic control classification; and
- an accepted revision/loading outcome.

The adapter must not hash a local file and assume OpenShell consumed those
bytes. The upstream response must bind accepted input or expose the committed
base policy for exact comparison.

### 7.3 Effective Policy And Control State

Required after creation and after execution:

- full effective policy canonical digest and revision;
- policy source and configuration revision;
- enforcement mode for filesystem, process, network, and inference controls;
- observed control degradation, skipped paths, unsupported controls, and
  compatibility mode;
- policy-load completion and failure posture; and
- a stable snapshot or monotonic revision proof that detects intervening
  changes.

OpenShell supports dynamic network-policy updates, while filesystem and
process controls are static at creation. The prototype must disable or reject
unplanned updates and prove the accepted revision did not change.

### 7.4 Driver-Observed Runtime Image

Required:

- immutable image digest observed by the selected compute driver after
  resolution;
- driver identity and runtime object reference;
- distinction between requested tag/digest and observed digest; and
- fail-closed behavior for mutable, missing, or mismatched identities.

Gateway defaults or caller-supplied image references are requests, not runtime
evidence.

### 7.5 Fixed Operation Outcome

Required:

- provider-owned fixed operation identity;
- process start and terminal observations;
- exit status, timeout, signal, cancellation, and ambiguity posture;
- bounded start and finish times; and
- no caller-selected shell string, environment value, or writable path.

The CLI exit code alone is not sufficient because it does not bind the command
to the exact sandbox, policy snapshot, or complete observation interval.

### 7.6 Structured Observations

Required:

- complete machine-readable OCSF or equivalent records for the bounded
  operation interval;
- stable reference plus range/cursor or start/end watermark;
- explicit completeness posture and dropped-event count;
- bounded counts for network allows/denials, process starts/terminals, policy
  changes, degradation events, and security findings; and
- one deliberate denied-egress observation for the no-write proof.

OpenShell documents that its gRPC push channel can drop events under load,
while files inside the sandbox contain the complete record. Therefore the
push channel alone is insufficient. Complete JSONL must be exported or reduced
to stable evidence before cleanup, with an integrity commitment and no raw log
copy into Core.

### 7.7 Cleanup And Reconciliation

Required:

- deletion operation/reference bound to the exact sandbox resource version;
- terminal absence or deleted-state confirmation after teardown;
- cleanup completion time and credential-purge posture where applicable;
- deterministic failed versus ambiguous cleanup status;
- restart-safe lookup of possibly existing sandboxes; and
- no automatic retry when execution or cleanup is `MayHaveStarted`.

A successful delete command or transition to `Deleting` is not cleanup proof.

## 8. Candidate Upstream Surfaces

Evaluate in this order:

1. Documented SDK or gateway API with typed responses and stable versioning.
2. Gateway/driver APIs that expose committed resource and observed runtime
   state.
3. Machine-readable CLI output backed by the same typed API.
4. Complete sandbox-resident OCSF JSONL reduced before deletion.
5. A narrowly scoped gateway interceptor only for policy validation or
   post-commit notification.

Gateway interceptors may help require an approved initial policy or observe a
committed create response, but they are not sufficient attestation by
themselves. They cannot replace driver-observed image state, complete execution
observations, or teardown confirmation.

Human-formatted text, tool self-description, caller assertions, requested
configuration, and synthetic evidence are never accepted surfaces.

The completed
[OpenShell v0.0.101 Evidence-Sufficiency Matrix](openshell-v0-0-101-evidence-sufficiency-matrix.md)
finds authoritative sandbox identity and effective-policy revision/load facts,
but no sufficient upstream surface for restart-safe invocation identity,
driver-observed image identity, durable operation outcome, complete
interval-bound observations, exact cleanup proof, or typed capability
negotiation. The selected pin therefore remains blocked for provider wiring.

## 9. Evidence-Sufficiency Matrix

Each required fact must be classified before implementation:

| Classification | Meaning | Integration result |
| --- | --- | --- |
| Authoritative | Typed fact from the enforcing/observing component, exactly bound and complete | May satisfy the contract |
| Corroborating | Useful independent observation without full authority or completeness | May strengthen evidence only |
| Requested | Caller or control-plane intent not proven effective | Cannot satisfy attestation |
| Lossy | Human text, bounded push stream, incomplete logs, or missing interval | Cannot satisfy attestation |
| Unavailable | No trustworthy machine-readable fact | Provider wiring remains blocked |

No majority vote across weak facts may manufacture an authoritative fact.

## 10. Capability Negotiation

The selected upstream boundary must expose or allow a deterministic capability
probe for:

- policy input binding;
- effective-policy snapshots;
- observed image digest;
- hard filesystem/process/network controls;
- complete structured observations;
- execution outcome binding;
- cleanup confirmation; and
- restart reconciliation.

Unknown, degraded, unsupported, or version-mismatched capabilities fail closed
before sandbox creation. Capability probing must not create a sandbox or grant
authority.

## 11. Provider-Neutral Mapping

The upstream adapter may construct existing Workflow OS types only after every
field has an accepted source:

- `OpenShellSandboxSnapshot` from authoritative environment, observed image,
  effective policy, and control facts;
- `OpenShellFixedOperationOutcome` from exactly bound process and observation
  facts;
- `HostedExecutionAttestation` from the stable post-operation snapshot,
  complete observation summary, and cleanup proof; and
- `HostedExecutionReceipt` only after Core validates exact request,
  configuration, provider, attempt, and reference binding.

No adapter method may append workflow events, issue approvals, grant
capabilities, mutate snapshots, or determine workflow terminal state.

## 12. Failure And Retry Semantics

- Static configuration rejection before any provider call is `NotStarted`.
- Any uncertainty after create/exec/delete activity is `MayHaveStarted`.
- Missing or conflicting attestation facts are protocol, policy, or ambiguous
  failures, never successful degraded evidence for the first slice.
- Cleanup ambiguity requires reconciliation and blocks receipt completion.
- Reconciliation must query by exact durable identity and reject multiple,
  substituted, stale, or drifted resources.
- Workflow OS must not convert report/evidence failure into a false execution
  result, nor convert an execution result into fake evidence.

## 13. Privacy And Redaction

Core stores stable references, canonical commitments, bounded counts, typed
postures, and timestamps only. It must not store:

- raw policy YAML or effective policy payloads;
- raw OCSF JSONL or shorthand logs;
- command stdout/stderr;
- source contents or arbitrary artifacts;
- environment values, provider material, authorization headers, or private
  runtime configuration; or
- host paths, internal endpoints, or sandbox labels in errors and Debug output.

The external evidence store and retention/access policy remain separately
scoped. A reference is not evidence completeness unless its production and
retention posture are known.

## 14. Minimal Prototype After Contract Acceptance

Only after this plan is reviewed and the upstream matrix is complete:

1. Select and pin one upstream release, API schema, driver, and immutable
   sandbox image.
2. Implement one injected client for a fixed no-write repository check.
3. Use no access material and default-deny networking.
4. Bind exact policy input and stable effective policy before execution.
5. Verify driver-observed image and hard controls.
6. Execute one fixed operation with no caller shell text.
7. Trigger one expected denied-egress event.
8. Reduce the complete structured observation interval to bounded facts and
   stable references.
9. Delete and independently confirm terminal absence.
10. Return one exactly bound receipt; otherwise require reconciliation.

This prototype remains explicit, opt-in, local or single-tenant, and
non-production.

## 15. Test Plan

Future implementation tests must cover:

- capability negotiation and version mismatch before provider mutation;
- exact policy-byte and effective-policy binding;
- global/default/provider policy substitution;
- policy drift before, during, and after execution;
- requested versus driver-observed image mismatch;
- hard-control degradation and unavailable enforcement;
- complete observation interval and dropped-event rejection;
- denied-egress proof without raw log copying;
- process outcome, timeout, signal, and cancellation binding;
- cleanup completed, failed, ambiguous, and stale-resource outcomes;
- restart reconciliation and duplicate-resource rejection;
- attempt-posture correctness on every failure boundary;
- stable non-leaking errors and redaction-safe Debug/serialization;
- no events, approvals, authority, reports, or evidence fabricated by the
  adapter; and
- no provider writes, automatic selection, schema behavior, or fork.

## 16. Upstream Engagement And Fork Policy

Prefer an upstream issue, API proposal, or narrow extension when a required
fact is unavailable. A fork is not justified for convenience, CLI stability,
custom branding, release cadence, or Workflow OS-specific defaults.

Reconsider a fork only if all are true:

1. a security-critical authoritative fact is unavailable;
2. upstream declines or cannot support a stable extension;
3. no independent trustworthy observer can supply the fact;
4. the required patch is narrow and maintainable; and
5. Workflow OS explicitly accepts ownership of the resulting security,
   compatibility, vulnerability, and release burden through a new ADR.

Even then, a maintained adapter-side extension or upstream contribution is
preferred to a runtime distribution fork.

## 17. Implementation Sequence

1. Review this plan.
2. Produce a version-pinned upstream evidence-sufficiency matrix using official
   API schemas and fixtures, without live sandbox execution.
3. Define any missing provider-neutral contract amendments, model-only.
4. Review the matrix and model changes.
5. Implement a disconnected typed client with recorded fixtures.
6. Review the disconnected client and retry/reconciliation semantics.
7. Plan and run one opt-in live no-write smoke proof.
8. Review the complete vertical slice before any default selection or broader
   operation.

## 18. Open Questions

- Which OpenShell release and API schema should become the next reviewed pin?
- Can the driver report the immutable image digest it actually started?
- Can accepted base-policy bytes and fully composed effective policy be bound
  through one revisioned API surface?
- Can a complete OCSF interval be exported with integrity and drop posture
  before sandbox deletion?
- What API proves terminal deletion rather than only accepting a delete call?
- Is there an idempotency or client-request field suitable for exact restart
  reconciliation?
- Which controls can be asserted as hard requirements across Docker, Podman,
  MicroVM, and Kubernetes drivers?
- Can gateway interceptors help bind policy without becoming a competing
  governance source of truth?

## 19. Final Recommendation

Proceed next to a focused maintainer review of this plan. If accepted, build a
version-pinned upstream evidence-sufficiency matrix before changing Rust code
or running a live sandbox.

The focused
[OpenShell Upstream API Attestation Contract Plan Review](../concepts/OPENSHELL_UPSTREAM_API_ATTESTATION_CONTRACT_PLAN_REVIEW.md)
accepts the plan and authorizes only that version-pinned matrix as the next
phase.

Continue to avoid live provider wiring, access material, writes, automatic
selection, schemas, examples, a fork, and production claims until every
required attestation fact is authoritative and exactly bound.

## 20. References

- [NVIDIA OpenShell: Manage Sandboxes](https://docs.nvidia.com/openshell/latest/sandboxes/manage-sandboxes)
- [NVIDIA OpenShell: Customize Sandbox Policies](https://docs.nvidia.com/openshell/latest/sandboxes/policies)
- [NVIDIA OpenShell: Sandbox Logging](https://docs.nvidia.com/openshell/observability/logging)
- [NVIDIA OpenShell: OCSF JSON Export](https://docs.nvidia.com/openshell/observability/ocsf-json-export)
- [NVIDIA OpenShell: Gateway Interceptors](https://docs.nvidia.com/openshell/latest/extensibility/gateway-interceptors)
- [NVIDIA OpenShell repository](https://github.com/NVIDIA/OpenShell)

These links describe candidate surfaces, not accepted Workflow OS attestation.

## 21. Governed Planning Evidence

- Workflow: `dg/d`.
- Run ID: `run-1786265842597878000-2`.
- Approval ID:
  `approval/run-1786265842597878000-2/planning-approved`.
- Approval presentation ID: `presentation/f9551085e64749c6`.
- Approval presentation hash:
  `f9551085e64749c6262d149cd9335696e93b95057ef01ad6fc9334358ca0ff8b`.
- Approval outcome: granted by delegated maintainer.
- Phase status: `Completed`.
- Event summary: 39 events comprising one run creation, validation, start,
  resume, and completion; six scheduled and successful skill invocations;
  eight policy decisions; one approval request; and one approval grant. There
  were no retries or escalations.
- Presentation posture: the persisted proof record matched the run and granted
  approval; current inspect output did not expose a proof-use event marker.
- Validation summary: `npm run check:docs` and `git diff --check` passed. No
  Rust source changed, so Rust validation was not required for this planning
  phase.
- Out-of-kernel work: Codex inspected Workflow OS contracts and plans, reviewed
  current official OpenShell documentation, authored this plan, and ran
  documentation validation. The kernel governed scope and approval but did not
  browse upstream sources, edit files, run checks, or perform git/PR actions.
