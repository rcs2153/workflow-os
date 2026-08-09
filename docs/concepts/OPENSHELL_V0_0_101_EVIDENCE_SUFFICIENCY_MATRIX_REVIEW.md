# OpenShell v0.0.101 Evidence-Sufficiency Matrix Review

## 1. Executive Verdict

Matrix accepted; proceed to a focused upstream API proposal.

The matrix accurately evaluates NVIDIA OpenShell v0.0.101 at source commit
`8ddd98c3dff62619a3963f99ba1e055b67650e72` against the reviewed Workflow OS
attestation contract. It identifies useful authoritative facts without
promoting requested state, partial logs, or successful commands into a false
execution receipt.

This verdict accepts the matrix, not provider readiness. Live
`OpenShellNoWriteClient` wiring remains blocked.

## 2. Scope Verification

The phase stayed within its approved source-assessment scope.

It:

- selected an exact upstream tag, commit, and schema boundary;
- mapped every required fact to an enforcing or observing component;
- evaluated binding, completeness, drift, reconciliation, and privacy;
- recorded accepted facts and blocking gaps;
- proposed focused upstream additions;
- preserved the optional-provider and no-fork decisions; and
- updated roadmap and plan status honestly.

It did not:

- install, start, or execute OpenShell;
- create a sandbox or use access material;
- wire a provider or modify Workflow OS runtime code;
- add schemas, SDK behavior, CLI behavior, examples, or defaults;
- enable external writes or another provider mutation;
- fork OpenShell; or
- claim live, production, or cryptographic proof.

## 3. Version And Source Assessment

The selected pin is sufficiently precise for a source matrix:

- tag `v0.0.101`;
- commit `8ddd98c3dff62619a3963f99ba1e055b67650e72`;
- public gateway, sandbox, compute-driver, and metadata protobuf schemas; and
- the matching gateway policy implementation.

The matrix correctly avoids treating an indexed release page as authoritative
version selection. It also states that source provenance does not prove an
installed binary, release asset, driver artifact, platform, or sandbox image.
Those identities remain obligations for a separately approved live proof.

No source-provenance blocker was found.

## 4. Architecture Boundary Assessment

The matrix preserves the correct product boundary:

```text
Workflow OS governs authorization, evidence acceptance, audit, and reporting.
OpenShell contains execution and reports facts about its enforcement boundary.
```

OpenShell should be an optional typed execution provider, not a generic skill
handler, default runtime, or governance source of truth. The matrix does not
confuse stronger containment with authority to execute.

The no-fork decision remains correct. Missing API facts justify upstream
engagement, not immediate ownership of OpenShell's container, VM, Kubernetes,
network, process, credential, release, and vulnerability surfaces.

## 5. Classification Assessment

The classifications are consistently conservative.

The matrix correctly accepts these facts within narrow boundaries:

- gateway-generated sandbox identity, workspace, timestamps, and resource
  version;
- gateway and driver version self-report;
- committed structured policy and effective composed policy;
- deterministic semantic policy hash, source, version, configuration revision,
  and failure mode;
- sandbox-reported policy load status;
- driver-observed runtime object identity; and
- final exit code for one connected exec stream.

It also correctly refuses to claim that any one of those narrow facts is a
complete operation receipt. An authoritative exit frame, for example, does
not provide durable operation identity, timeout/signal posture, restart
reconciliation, or complete observations.

No weak facts were combined to manufacture authority.

## 6. Invocation And Reconciliation Assessment

The matrix correctly marks restart-safe invocation binding unavailable.

`CreateSandboxRequest` has no client request or idempotency key. Labels,
annotations, names, and later list/get queries can find candidate resources,
but they do not prove that one resource is the exact result of one ambiguous
Workflow OS create attempt.

This is a blocker even for a no-write operation because sandbox creation and
deletion are provider mutations. Retrying after an ambiguous response could
create a duplicate sandbox or attach a receipt to a substituted resource.

The proposed upstream create acceptance record is proportionate and should be
included in the next proposal.

## 7. Policy Assessment

The matrix's strongest positive finding is the policy boundary.

OpenShell v0.0.101 exposes a typed effective policy, deterministic semantic
hash, policy version, source, global version, configuration revision, failure
mode, revision history, and sandbox-reported load status. Those surfaces are
meaningfully stronger than the reviewed CLI compatibility output.

The matrix also correctly identifies that OpenShell does not preserve or
return exact source YAML bytes. Exact YAML-byte binding is therefore
unavailable, but this does not necessarily mean trustworthy policy-input
binding is impossible.

Non-blocking follow-up: review a provider-neutral contract clarification that
allows an authoritative canonical structured-policy commitment when a provider
parses and persists a typed policy. Any clarification must still require:

- provider-returned committed canonical input;
- gateway-generated provenance, not caller assertion alone;
- full effective composed policy and relationship to input;
- accepted and loaded revision;
- operation binding; and
- pre/post no-drift proof.

Do not make that contract amendment implicitly inside the OpenShell adapter.

## 8. Runtime Image And Control Assessment

The driver-observed image classification is correct.

The public and driver schemas carry the requested image reference. Driver
status carries platform object identity and lifecycle conditions, but not the
resolved immutable image digest actually running. A digest-shaped request is
still intent, not runtime evidence.

The matrix also correctly rejects requested filesystem, process, and network
policy as proof of applied hard controls. Individual OCSF events and driver
conditions can corroborate enforcement, but v0.0.101 has no complete typed
applied-control and degradation snapshot across supported drivers.

Both gaps block provider readiness.

## 9. Operation Outcome Assessment

The exec-stream assessment is correct.

`ExecSandboxRequest` identifies a sandbox and carries command inputs, while
`ExecSandboxEvent` returns stdout, stderr, and a final exit code. It does not
create a durable operation resource or expose:

- an operation ID or idempotency key;
- canonical request digest;
- policy revision binding;
- start and finish times;
- terminal timeout, signal, or cancellation posture; or
- restart lookup after transport ambiguity.

Workflow OS can own a fixed operation request, but it cannot infer upstream
execution identity or terminal semantics. The proposed durable operation
resource is required before integration.

## 10. Observation Assessment

The matrix correctly distinguishes complete event records from a complete
observation interval.

OpenShell documents complete OCSF JSON objects in sandbox-resident JSONL, but
v0.0.101 does not expose an operation-bound export manifest with start/end
watermarks, flush posture, event and drop counts, and an integrity commitment.
The documented `sandbox download` path is workspace-confined while OCSF logs
are under `/var/log`; even another transfer mechanism would not by itself add
interval completeness or integrity.

Gateway-pushed logs are explicitly bounded and lack the required cursor and
drop posture. They are lossy and cannot satisfy attestation.

An individual denied-egress OCSF record is strong corroboration of that denial,
but without exact operation binding and interval completeness it is not the
complete denied-egress proof required by the prototype.

The proposed observation export manifest is the right upstream request.

## 11. Cleanup Assessment

The cleanup classification is correct.

A typed `deleted` response, deletion timestamp, deleting phase, or driver watch
event each proves a narrower fact. The public delete request does not accept an
expected resource version, and the public API does not return a durable
deletion operation bound to the exact resource version with terminal gateway
and driver absence, completion time, and credential-purge posture.

Polling a name to absence can corroborate cleanup but can race with name reuse
or replacement. It is not an exact deletion receipt.

Resource-version-bound delete and restart-safe deletion lookup remain blockers.

## 12. Capability Negotiation Assessment

The matrix correctly rejects version-string inference as capability
negotiation.

The pin reports driver name, version, and default image, but not typed
capabilities for policy binding, observed image identity, hard controls,
complete observations, operation outcome, cleanup, or reconciliation. Driver
and platform behavior differs, so an adapter cannot safely infer those facts
from a known version table alone.

The upstream proposal should request a typed capability snapshot. Unknown or
degraded required capabilities must fail before sandbox creation.

## 13. Privacy And Error Assessment

The matrix preserves the payload-free Workflow OS Core boundary.

Acceptable projected facts remain stable IDs, commitments, versions, bounded
counts, timestamps, typed postures, and stable evidence references. Raw
policy, OCSF, stdout, stderr, environment, provider material, paths, endpoints,
and upstream errors remain outside Core.

The proposed observation manifest should support provider-side reduction so
Workflow OS does not need raw security logs merely to determine completeness.

No privacy blocker was found in the matrix.

## 14. Matrix Blockers

No blocker was found in the matrix itself.

The matrix correctly identifies these provider-readiness blockers:

1. restart-safe invocation identity and exact create reconciliation;
2. driver-observed immutable image identity;
3. durable operation identity and complete terminal semantics;
4. complete applied-control and degradation posture;
5. operation-bound complete observation export;
6. resource-version-bound deletion and durable cleanup receipt;
7. restart reconciliation for ambiguous provider mutations; and
8. typed capability negotiation.

Those blockers must not be reclassified as non-blocking merely to enable a
prototype.

## 15. Non-Blocking Follow-Ups

- Review the provider-neutral canonical structured-policy commitment
  clarification before changing Rust contracts.
- Include release-asset, binary, driver, platform, and image provenance in the
  future live-smoke plan.
- Ask upstream whether the proposed operation and observation resources can be
  one cohesive attestation API rather than unrelated endpoints.
- Define compatibility and rollback expectations for every accepted upstream
  schema revision.
- Keep the current CLI transport as a disconnected compatibility probe.

## 16. Fork Decision

Do not fork OpenShell.

The matrix has not shown upstream refusal, absence of an extension path, or a
narrow maintainable patch. The reviewed fork threshold is not met.

Reconsideration requires a new ADR only if a security-critical fact remains
unavailable after upstream engagement, no trustworthy independent observer can
provide it, the required patch is narrow, and Workflow OS explicitly accepts
the resulting maintenance and vulnerability burden.

## 17. Recommended Next Phase

Prepare a focused OpenShell upstream API proposal.

The proposal should cover idempotent sandbox creation, canonical policy and
applied-state commitment, driver-observed image identity, a durable fixed
operation resource, complete observation export manifests, exact cleanup
receipts, restart reconciliation, and typed capabilities.

Do not implement provider wiring, install or execute OpenShell, fork the
runtime, add access material, or broaden provider mutations in that phase.

## 18. Governed Review Evidence

- workflow ID: `dg/review`;
- run ID: `run-1786268018662791000-2`;
- approval ID:
  `approval/run-1786268018662791000-2/review-scope-approved`;
- approval presentation ID: `presentation/27b70e9369e5f0cd`;
- approval outcome: granted by delegated maintainer;
- out-of-kernel work: source-backed document review, documentation edits,
  local documentation checks, Git commit/push, and GitHub PR/merge actions;
- runtime/provider activity: none.

Required phase-close validation:

- `npm run check:docs`;
- `git diff --check`;
- governed event-trail inspection; and
- confirmation that only documentation changed.
