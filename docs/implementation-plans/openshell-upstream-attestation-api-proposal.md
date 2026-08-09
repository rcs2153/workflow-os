# OpenShell Upstream Attestation API Proposal

Status: planning complete and focused review accepted; no upstream submission
or runtime implementation is authorized.

## 1. Executive Summary

Workflow OS should propose a small set of general-purpose NVIDIA OpenShell API
additions that let callers prove what sandbox resource was created, which
policy and image were applied, what exact operation ran, which complete
security observations cover that operation, and whether the exact sandbox was
deleted.

The proposal should remain useful to any governance, compliance, CI, or
security integration. It must not introduce Workflow OS-specific concepts into
OpenShell or ask OpenShell to become the system of record for workflow policy,
approval, evidence, or reports.

The required upstream additions are:

1. idempotent sandbox creation and exact request lookup;
2. canonical policy-input and applied-environment snapshots;
3. driver-observed immutable image identity and applied-control posture;
4. a durable sandbox exec-operation resource;
5. operation-bound complete-observation export manifests;
6. resource-version-bound deletion operations and terminal cleanup receipts;
   and
7. typed attestation capability negotiation.

No live provider wiring should begin until an accepted upstream release or
reviewed commit exposes sufficient authoritative facts and the complete matrix
is rerun.

## 2. Problem Statement

OpenShell v0.0.101 already provides valuable typed facts, especially
gateway-owned sandbox identity and effective-policy revision/load state. It
does not provide a complete, restart-safe execution attestation.

A caller can currently request a sandbox and operation, observe an exit code,
read partial or sandbox-resident logs, request deletion, and inspect later
state. Those observations are useful, but they do not exactly answer:

- whether an ambiguous create response produced one exact sandbox;
- which immutable image the compute driver actually ran;
- which control revision was applied for one operation;
- whether the exact operation started, timed out, was signaled, or completed;
- whether an observation set completely covers that operation;
- whether events were dropped;
- whether deletion targeted the exact observed resource version; or
- whether cleanup reached terminal absence after caller restart.

Workflow OS cannot infer those facts without weakening its governance and
evidence boundary. OpenShell is the enforcing or observing component for them
and should expose them authoritatively.

## 3. Design Principles

The upstream proposal should follow these principles:

- **General-purpose:** use sandbox, operation, policy, observation, and
  deletion vocabulary rather than Workflow OS domain types.
- **Enforcer-owned facts:** facts come from the gateway, driver, supervisor,
  or platform component that enforces or observes them.
- **Exact binding:** every fact carries stable IDs, revisions, and commitments
  sufficient to join it to one resource and operation.
- **Restart safety:** every provider mutation accepts an idempotency key and
  supports durable lookup.
- **Explicit ambiguity:** unknown or may-have-started outcomes are typed rather
  than collapsed into generic failure.
- **Payload-minimizing:** callers can prove completeness through manifests,
  commitments, counts, and references without ingesting raw logs.
- **Backward-compatible where practical:** new optional request fields,
  response fields, and RPCs should not alter current callers silently.
- **No false cryptography claim:** SHA-256 commitments provide integrity and
  identity within the trust boundary; they are not signatures or hardware
  attestations.

## 4. Non-Goals

This proposal does not ask OpenShell to implement:

- Workflow OS workflow, approval, policy-gate, EvidenceReference, or WorkReport
  models;
- a Workflow OS-specific runtime distribution or fork;
- agent orchestration, recursive agents, or agent teams;
- provider writes, repository mutation, or a new credential system;
- hosted Workflow OS behavior or enterprise administration;
- cryptographic signing, transparency logs, TPM/TEE attestation, or remote
  attestation;
- indefinite raw-log retention;
- a universal policy language shared with Workflow OS; or
- production-readiness claims for either project.

This planning phase also does not submit an upstream issue or pull request,
install OpenShell, start a gateway or sandbox, or change Workflow OS Rust.

## 5. Proposed Resource Model

The proposal should add five durable resources or equivalent typed records:

```text
SandboxCreationRecord
  -> SandboxAppliedStateSnapshot
  -> SandboxExecOperation
  -> ObservationExportManifest
  -> SandboxDeletionOperation
```

The exact upstream names should follow OpenShell conventions. The semantic
relationships are more important than these candidate names.

Every resource should include:

- stable gateway-generated ID;
- caller-supplied idempotency key where it represents a mutation;
- sandbox ID and resource version where applicable;
- creation and update timestamps;
- terminal or ambiguity posture;
- schema/API version; and
- bounded machine-readable errors that do not expose credentials or raw
  payloads.

## 6. Idempotent Sandbox Creation

### 6.1 Request Addition

Add an optional idempotency field to `CreateSandboxRequest`:

```proto
message CreateSandboxRequest {
  SandboxSpec spec = 1;
  string name = 2;
  map<string, string> labels = 3;
  map<string, string> annotations = 4;
  string workspace = 5;

  // Caller-generated opaque key, unique within workspace and operation kind.
  string idempotency_key = 6;

  // SHA-256 commitment to the caller's canonical create request.
  string request_commitment = 7;
}
```

Candidate constraints:

- bounded ASCII or UUID-shaped value;
- no secret material or semantic payload;
- unique within workspace for a documented retention period;
- same key plus same commitment returns the original result;
- same key plus different commitment fails with a typed conflict;
- persistence occurs before or atomically with sandbox creation intent; and
- lookup remains available after caller disconnect and process restart.

### 6.2 Creation Record

Return or expose a durable record:

```proto
message SandboxCreationRecord {
  string creation_id = 1;
  string idempotency_key = 2;
  string request_commitment = 3;
  string sandbox_id = 4;
  uint64 sandbox_resource_version = 5;
  int64 accepted_at_ms = 6;
  MutationPosture posture = 7;
}
```

`MutationPosture` should distinguish at least:

- accepted and resource assigned;
- terminal failure proven before resource mutation;
- mutation may have started;
- terminal success; and
- reconciled after ambiguity.

Add lookup by `creation_id` and idempotency key. Name and labels remain useful
metadata but are not reconciliation keys.

## 7. Canonical Policy Commitment

### 7.1 Input Commitment

OpenShell should commit the canonical structured `SandboxPolicy` it accepts,
not the caller's original YAML serialization.

The response or policy revision should include:

```proto
message PolicyInputCommitment {
  string canonicalization = 1;
  string policy_schema_version = 2;
  string canonical_policy_sha256 = 3;
  string source_class = 4;
  map<string, string> gateway_provenance = 5;
}
```

Requirements:

- `canonicalization` identifies the exact deterministic algorithm and version;
- the gateway computes the commitment from the typed policy it persists;
- provenance keys required for attestation are gateway-generated;
- caller annotations may be returned separately but remain untrusted;
- the committed canonical policy can be returned for exact comparison when
  authorized; and
- schema/canonicalization changes create a new compatibility boundary.

This proposal deliberately does not require retention of YAML comments,
ordering, whitespace, or original bytes.

### 7.2 Applied Policy Relationship

Extend the effective configuration response or add a snapshot that binds:

- input policy commitment;
- final effective policy commitment after global/provider composition;
- policy source and all composition-layer revision IDs;
- policy version and configuration revision;
- applied/loaded version and time;
- validation failure mode;
- static versus dynamic controls; and
- drift posture.

The relationship must be machine-readable. A final effective hash without its
input and composition lineage is insufficient for policy-substitution review.

## 8. Applied Environment And Control Snapshot

Add a typed snapshot emitted from gateway and driver observations after the
sandbox is ready:

```proto
message SandboxAppliedStateSnapshot {
  string snapshot_id = 1;
  string sandbox_id = 2;
  uint64 sandbox_resource_version = 3;
  string runtime_object_id = 4;
  string requested_image = 5;
  string observed_image_digest = 6;
  string driver_name = 7;
  string driver_version = 8;
  string effective_policy_commitment = 9;
  uint32 active_policy_version = 10;
  uint64 config_revision = 11;
  repeated AppliedControl controls = 12;
  int64 observed_at_ms = 13;
}
```

`observed_image_digest` must come from the compute platform or driver after
resolution. It must not echo `requested_image`.

`AppliedControl` should identify:

- control family: filesystem, process, network, inference, or platform;
- requested mode;
- effective mode;
- hard, best-effort, degraded, skipped, unsupported, or unavailable posture;
- bounded machine-readable reason code;
- enforcing component; and
- observation time or revision.

Driver-specific details may remain extensible, but required security posture
must not be hidden only inside opaque `driver_config` or human condition text.

## 9. Durable Sandbox Exec Operation

### 9.1 Operation Creation

Add a durable exec operation rather than relying only on a live stream:

```proto
message CreateSandboxExecOperationRequest {
  string sandbox_id = 1;
  string idempotency_key = 2;
  string request_commitment = 3;
  repeated string command = 4;
  string workdir = 5;
  map<string, string> environment = 6;
  uint32 timeout_seconds = 7;
  string required_applied_snapshot_id = 8;
}
```

The general OpenShell API may continue to support arbitrary commands. Workflow
OS will use this resource only through a fixed provider-owned operation and
will commit the exact canonical request.

The operation must reject a stale or mismatched applied snapshot before
process start.

### 9.2 Operation Resource

```proto
message SandboxExecOperation {
  string operation_id = 1;
  string idempotency_key = 2;
  string request_commitment = 3;
  string sandbox_id = 4;
  uint64 sandbox_resource_version = 5;
  string applied_snapshot_id = 6;
  ExecOperationPosture posture = 7;
  int64 accepted_at_ms = 8;
  int64 started_at_ms = 9;
  int64 finished_at_ms = 10;
  optional int32 exit_code = 11;
  optional int32 signal = 12;
  string terminal_reason = 13;
  string observation_manifest_id = 14;
}
```

Required terminal postures:

- pending;
- started;
- succeeded;
- failed;
- timed out;
- signaled;
- canceled before start;
- canceled after start;
- may have started; and
- terminal outcome unavailable.

Lookup by operation ID and idempotency key must survive caller and gateway
restart according to documented persistence posture.

Stdout and stderr may remain streamed or referenced separately. They must not
be required to establish the typed outcome.

## 10. Complete Observation Export Manifest

### 10.1 Manifest

OpenShell should finalize a manifest after an operation reaches a terminal or
ambiguous posture:

```proto
message ObservationExportManifest {
  string manifest_id = 1;
  string sandbox_id = 2;
  string operation_id = 3;
  string applied_snapshot_id = 4;
  string format = 5;
  string schema_version = 6;
  string start_cursor = 7;
  string end_cursor = 8;
  uint64 event_count = 9;
  uint64 dropped_event_count = 10;
  string content_sha256 = 11;
  string stable_reference = 12;
  ObservationCompleteness completeness = 13;
  repeated ObservationCount bounded_counts = 14;
  int64 finalized_at_ms = 15;
}
```

The manifest must be generated by the supervisor or observation subsystem,
not reconstructed by the caller after downloading a file.

### 10.2 Completeness

`ObservationCompleteness` should distinguish:

- complete and finalized;
- incomplete with known drops;
- incomplete due to unavailable interval;
- incomplete due to export failure;
- may be incomplete; and
- unsupported.

A complete manifest requires:

- operation-bound start and end watermarks;
- final flush acknowledgement;
- zero unexplained drops;
- count and digest over the exported record set; and
- stable retrieval or retention semantics.

### 10.3 Payload-Free Reduction

`bounded_counts` should allow callers to accept useful facts without loading
raw OCSF into their core state. Candidate dimensions include:

- process starts and terminals;
- network allows and denials;
- policy/configuration changes;
- degradation or skipped-control findings;
- security findings by severity; and
- lifecycle events.

Raw records may be retrieved by authorized tools through the stable reference,
but references and manifests must not contain credentials, raw URLs with
secrets, environment values, or command output.

## 11. Exact Deletion And Cleanup Receipt

### 11.1 Delete Request

Extend public deletion:

```proto
message DeleteSandboxRequest {
  string name = 1;
  string workspace = 2;
  string sandbox_id = 3;
  uint64 expected_resource_version = 4;
  string idempotency_key = 5;
}
```

The gateway must reject name/ID or resource-version mismatch before deleting a
different resource.

### 11.2 Deletion Operation

```proto
message SandboxDeletionOperation {
  string deletion_id = 1;
  string idempotency_key = 2;
  string sandbox_id = 3;
  uint64 deleted_resource_version = 4;
  DeletionPosture posture = 5;
  int64 accepted_at_ms = 6;
  int64 completed_at_ms = 7;
  bool gateway_absent = 8;
  bool driver_absent = 9;
  string driver_observation_id = 10;
  CleanupPosture cleanup = 11;
}
```

`CleanupPosture` should include bounded facts for:

- compute resource teardown;
- supervisor/process termination;
- attached temporary secret or credential purge where applicable;
- observation finalization; and
- unresolved or ambiguous cleanup.

Deletion lookup must remain available after the sandbox resource disappears.
A later `GetSandbox` not-found response is corroboration, not the deletion
receipt itself.

## 12. Typed Capability Negotiation

Extend gateway information with a versioned capability snapshot:

```proto
message AttestationCapabilities {
  string schema_version = 1;
  Capability policy_input_commitment = 2;
  Capability applied_policy_snapshot = 3;
  Capability observed_image_digest = 4;
  Capability applied_control_posture = 5;
  Capability durable_exec_operation = 6;
  Capability complete_observation_manifest = 7;
  Capability exact_cleanup_receipt = 8;
  Capability restart_reconciliation = 9;
}

message Capability {
  CapabilityPosture posture = 1;
  string version = 2;
  repeated string supported_drivers = 3;
  string bounded_reason_code = 4;
}
```

Postures should distinguish supported, degraded, unsupported, unavailable, and
unknown. Capability output must reflect the active gateway/driver/platform
combination rather than a compile-time marketing claim.

Unknown or degraded required capabilities allow callers to fail before
sandbox creation.

## 13. Authorization And Privacy

The new records expose sensitive security posture even when they exclude raw
payloads. OpenShell should apply its existing gateway authorization model and
document scopes for:

- creation and deletion records;
- applied-state snapshots;
- operation status;
- observation manifests; and
- raw observation retrieval.

Supervisor-to-gateway facts must remain sandbox-bound. User callers must not
be able to submit driver-observed image identity, applied-control posture,
event-drop counts, or cleanup completion as assertions.

Error fields should use bounded reason codes. Human diagnostics may exist, but
integrations must not need to persist them. No API should echo credential
values, authorization headers, environment secrets, raw provider payloads, or
unbounded command output.

## 14. Failure And Ambiguity Semantics

Every provider mutation should make these states distinguishable:

- request rejected before provider activity;
- request durably accepted;
- activity started;
- terminal success;
- terminal failure;
- may have started;
- cleanup pending;
- cleanup ambiguous; and
- reconciled after caller restart.

Transport failure must not be translated automatically into not-started.
Idempotency-key reuse with a different commitment must fail closed.

The API should document retention periods for creation, operation, observation,
and deletion records. A caller cannot rely on restart reconciliation beyond
that declared boundary.

## 15. Backward Compatibility And Rollout

A possible incremental upstream sequence is:

1. add optional create idempotency and durable creation lookup;
2. expose canonical policy and applied environment snapshots;
3. expose driver-observed image and typed controls;
4. add durable exec operations;
5. add observation manifests;
6. add exact deletion operations; and
7. expose one combined capability snapshot.

Until all required capabilities are present, Workflow OS should keep provider
wiring disconnected. Partial upstream delivery is still useful to other
callers and can be reclassified in later matrices.

Each released schema must have:

- explicit version;
- compatibility policy;
- migration and retention notes;
- driver/platform support matrix;
- fixture coverage; and
- rollback behavior.

## 16. Minimum Upstream Acceptance Criteria

The upstream boundary is sufficient for a new Workflow OS compatibility
assessment only when it can prove:

1. one create request maps idempotently to one sandbox ID and resource version;
2. canonical input and effective loaded policy are exactly related;
3. the selected driver reports the immutable image digest actually running;
4. hard controls and degradation are typed for that driver/platform;
5. one durable operation binds request, sandbox, policy snapshot, and outcome;
6. one complete observation manifest binds an interval with zero unexplained
   drops and includes the deliberate denied-egress event;
7. deletion targets the exact resource version and returns durable terminal
   cleanup posture; and
8. all required capabilities are negotiated before create.

Meeting those criteria authorizes another matrix and live-smoke plan. It does
not automatically authorize production provider wiring.

## 17. Proposed Upstream Engagement Package

After focused review, prepare a concise upstream package containing:

- problem statement centered on trustworthy sandbox attestation;
- v0.0.101 gap matrix with exact source references;
- this provider-neutral semantic proposal;
- one no-write CI/governance use case;
- candidate protobuf sketches clearly labeled non-final;
- privacy and backward-compatibility expectations; and
- an offer to split the work into independently useful upstream issues.

Do not open multiple broad issues without maintainer guidance. Begin with one
discussion or issue asking whether the resource model aligns with upstream
direction and which surface the maintainers prefer.

No external submission is authorized by this document.

## 18. Fork Threshold

This proposal does not change the no-fork decision.

A fork remains unjustified unless:

1. one security-critical authoritative fact remains unavailable;
2. upstream explicitly declines the needed hook or compatible alternative;
3. no trustworthy independent observer can provide it;
4. the patch is narrow and sustainably maintainable; and
5. a new Workflow OS ADR accepts lifecycle, platform, release, and CVE burden.

Even then, an upstream extension point or small maintained observer should be
preferred over a full runtime distribution.

## 19. Test And Verification Plan

If upstream adds the surfaces, a future Workflow OS phase should verify:

- same idempotency key and commitment returns one resource;
- same key with another commitment fails closed;
- ambiguous create is reconciled after client restart;
- canonical input and effective policy commitments match reviewed fixtures;
- global/provider substitution is visible;
- stale applied snapshots block operation start;
- mutable or mismatched observed images fail;
- degraded or unsupported controls fail capability negotiation;
- operation timeout, signal, cancellation, and transport ambiguity remain
  distinct;
- observation manifests detect missing, duplicated, or dropped events;
- deliberate denied egress appears inside the exact operation interval;
- deletion rejects resource-version mismatch;
- deletion can be reconciled after caller restart;
- raw payloads and secret-like values do not enter Workflow OS errors, Debug,
  audit, evidence, or reports; and
- an upstream upgrade fails closed until its capability/schema matrix is
  reviewed.

## 20. Governed Planning Evidence

- workflow ID: `dg/d`;
- run ID: `run-1786268568824732000-2`;
- approval ID: `approval/run-1786268568824732000-2/planning-approved`;
- approval presentation ID: `presentation/f657e0177eead788`;
- approval outcome: granted by delegated maintainer;
- out-of-kernel work: source-backed proposal design, documentation edits,
  local documentation validation, Git commit/push, and GitHub PR/merge actions;
- provider/runtime activity: none.

## 21. Recommended Next Phase

The focused maintainer review is accepted. One bounded
[OpenShell Trustworthy Sandbox Attestation Discussion Draft](openshell-upstream-attestation-discussion-draft.md)
is prepared for human review.
Do not submit it automatically. Do not install or execute OpenShell, implement
provider wiring, change Workflow OS Rust contracts, fork OpenShell, add access
material, or broaden provider mutations in that phase.

See the
[OpenShell Upstream Attestation API Proposal Review](../concepts/OPENSHELL_UPSTREAM_ATTESTATION_API_PROPOSAL_REVIEW.md).
