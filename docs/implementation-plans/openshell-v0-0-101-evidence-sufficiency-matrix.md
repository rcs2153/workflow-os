# OpenShell v0.0.101 Evidence-Sufficiency Matrix

Status: accepted by focused maintainer review; provider wiring remains
blocked.

## 1. Executive Verdict

NVIDIA OpenShell remains the right optional execution-containment boundary for
Workflow OS, but OpenShell v0.0.101 does not expose every authoritative fact
required by the reviewed `OpenShellNoWriteClient` attestation contract.

The selected upstream pin is:

- release tag: `v0.0.101`;
- source commit: `8ddd98c3dff62619a3963f99ba1e055b67650e72`; and
- evaluated API definitions: the protobuf schemas and implementation at that
  exact commit.

The pin provides strong machine-readable surfaces for gateway and driver
version identity, gateway-owned sandbox identity, effective policy content,
policy hash and revision, policy source, configuration revision, and sandbox
reported policy-load status. Those facts are useful and several are
authoritative within their stated boundary.

The pin does not provide an exact create idempotency key, an upstream-bound
Workflow OS request identity, a driver-observed immutable image digest, a
durable operation resource with complete terminal semantics, an
operation-bound complete-observation manifest, an exact resource-version-bound
deletion receipt, or typed security capability negotiation. Downloadable OCSF
JSONL and successful CLI commands do not fill those gaps.

Result: **blocked for provider wiring**. Do not implement or connect a live
`OpenShellNoWriteClient` against this pin. The next phase should review this
matrix and then convert the missing facts into a focused upstream API proposal.
No fork is justified.

## 2. Scope

This assessment:

- selects one exact upstream release and commit;
- maps each required attestation fact to the enforcing or observing component;
- identifies the exact machine-readable surface;
- evaluates binding, completeness, drift, and privacy posture;
- assigns the reviewed evidence-sufficiency classification; and
- records which upstream additions are required before integration.

This assessment does not:

- install or execute OpenShell;
- start a gateway, driver, supervisor, sandbox, or operation;
- add provider access material or network access;
- wire an execution provider or change Workflow OS Rust code;
- change workflow schemas, SDKs, CLI behavior, examples, or defaults;
- authorize provider writes, arbitrary commands, or production use;
- fork OpenShell or create a Workflow OS runtime distribution; or
- claim that source inspection replaces a future live smoke proof.

## 3. Source And Version Provenance

The version was selected from the official upstream Git tag namespace, not
from an indexed release page or version-number inference. The evaluated source
is the exact commit referenced by tag `v0.0.101`.

Primary sources:

- [OpenShell source at the selected commit](https://github.com/NVIDIA/OpenShell/tree/8ddd98c3dff62619a3963f99ba1e055b67650e72)
- [public gateway API schema](https://github.com/NVIDIA/OpenShell/blob/8ddd98c3dff62619a3963f99ba1e055b67650e72/proto/openshell.proto)
- [sandbox policy API schema](https://github.com/NVIDIA/OpenShell/blob/8ddd98c3dff62619a3963f99ba1e055b67650e72/proto/sandbox.proto)
- [compute-driver API schema](https://github.com/NVIDIA/OpenShell/blob/8ddd98c3dff62619a3963f99ba1e055b67650e72/proto/compute_driver.proto)
- [gateway object metadata schema](https://github.com/NVIDIA/OpenShell/blob/8ddd98c3dff62619a3963f99ba1e055b67650e72/proto/datamodel.proto)
- [policy implementation](https://github.com/NVIDIA/OpenShell/blob/8ddd98c3dff62619a3963f99ba1e055b67650e72/crates/openshell-server/src/grpc/policy.rs)
- [OCSF JSON export documentation](https://docs.nvidia.com/openshell/latest/observability/ocsf-json-export)
- [sandbox file-transfer documentation](https://docs.nvidia.com/openshell/latest/sandboxes/manage-sandboxes)

This source assessment does not establish release-asset integrity, installed
binary identity, tested platform, tested driver, or resolved sandbox-image
identity. A future live proof must pin and verify those independently.

## 4. Classification Rules

The matrix uses the classifications accepted by the upstream attestation
contract review:

| Classification | Meaning |
| --- | --- |
| Authoritative | Typed fact emitted by the enforcing or observing component, exactly bound and complete for the stated fact |
| Corroborating | Useful independent observation without sufficient authority, binding, or completeness |
| Requested | Caller or control-plane intent that does not prove effective runtime state |
| Lossy | Bounded, textual, partial, or interval-incomplete observation |
| Unavailable | No trustworthy machine-readable fact exists at this pin |

An authoritative classification is intentionally narrow. For example, an
exit code may be authoritative for one exec stream's final frame while still
being insufficient as a complete governed operation receipt.

## 5. Evidence-Sufficiency Matrix

### 5.1 Release, Gateway, And Driver Identity

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Source release and schema identity | Git release/tag infrastructure | exact tag and commit plus pinned protobuf source | Exact for reviewed source; does not prove the installed binary | Authoritative for source provenance | Accept as source pin only |
| Gateway implementation version | Running gateway | `GetGatewayInfoResponse.gateway_version` | Typed self-report from the running gateway; requires external binary digest verification | Authoritative for gateway self-report; corroborating for supply-chain identity | Require both version and independently verified binary digest |
| Selected driver name and version | Driver at gateway startup, projected by gateway | `GetGatewayInfoResponse.compute_drivers[].capabilities` | Typed startup snapshot, human-readable version string, no binary digest | Authoritative for reported driver identity; corroborating for installed artifact identity | Useful but insufficient alone |
| Provider configuration commitment | Workflow OS | Existing provider-neutral configuration identity | Workflow OS can commit its own reviewed configuration but cannot infer upstream effective state | Authoritative for Workflow OS input only | Preserve separately from provider facts |

### 5.2 Invocation And Sandbox Identity

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Workflow OS execution ID and request fingerprint accepted by OpenShell | Gateway create path | labels and annotations on `CreateSandboxRequest` | Caller can submit metadata, but upstream does not assign semantics or return an acceptance commitment specifically for the governed request | Requested | Cannot satisfy provider-side invocation binding |
| Stable sandbox ID | Gateway persistence | `Sandbox.metadata.id` | Gateway-generated stable object ID returned on create/get | Authoritative | Accept after create |
| Sandbox resource version | Gateway persistence | `Sandbox.metadata.resource_version` | Monotonic optimistic-concurrency version on gateway-owned object | Authoritative for observed gateway object version | Accept for observations; delete does not bind it |
| Sandbox creation time and workspace | Gateway persistence | `ObjectMeta.created_at_ms` and `workspace` | Typed and bound to sandbox resource | Authoritative | Accept |
| Restart-safe create idempotency key | Gateway create path | none | `CreateSandboxRequest` has no client request ID or idempotency key; name/label lookup can collide or be substituted | Unavailable | Blocking gap |
| Exact create reconciliation by immutable request identity | Gateway | list/get by name, labels, metadata | Can help find candidates but cannot prove one exact create attempt after an ambiguous response | Corroborating | Blocking gap |

### 5.3 Exact Policy Input

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Structured policy accepted at create | Gateway | `CreateSandboxRequest.spec.policy`, echoed in `Sandbox.spec.policy` | Typed semantic policy accepted into gateway desired state | Authoritative for committed structured input | Accept if canonical protobuf commitment is the contract |
| Exact original policy-file bytes | Gateway | none | YAML bytes, comments, ordering, and original serialization are not retained or returned | Unavailable | Existing exact-byte requirement cannot be met |
| Canonical semantic policy digest | Gateway policy service | deterministic SHA-256 over selected protobuf fields; exposed as `policy_hash` | Deterministic for OpenShell's structured policy semantics; not a digest of caller YAML bytes | Authoritative for hashed policy payload semantics | Candidate contract amendment: bind canonical structured policy, not file bytes |
| Policy revision provenance | Gateway policy store | `SandboxPolicyRevision.provenance` | Immutable metadata supplied with the policy revision, but caller-supplied values remain assertions unless the gateway defines and enforces keys | Requested unless upstream-generated | Do not treat caller provenance as proof |
| Global/provider policy precedence | Gateway policy composition | `policy_source`, `global_policy_version`, returned effective policy, provider composition in gateway implementation | Typed source and final composed payload are observable; provider-layer provenance is not individually enumerated in the response | Authoritative for final effective payload and source class; partial for composition lineage | Accept final state; retain composition-lineage gap |

### 5.4 Effective Policy And Control State

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Effective policy payload | Gateway composition service | `GetSandboxConfigResponse.policy` | Typed effective policy after global/provider composition | Authoritative for gateway-composed desired policy | Accept |
| Effective policy hash and version | Gateway policy store | `policy_hash`, `version`, `global_policy_version` | Typed, deterministic and monotonic within the sandbox/global scope | Authoritative | Accept |
| Effective configuration revision | Gateway | `config_revision` | Fingerprint changes with policy/settings inputs | Authoritative for gateway effective configuration | Accept and compare before/after |
| Policy source and failure mode | Gateway | `policy_source`, `policy_validation_failure_mode` | Typed source plus explicit fail-closed/retain-last-valid posture | Authoritative | Accept |
| Sandbox loaded policy version | Sandbox runtime reporting through gateway | `GetSandboxPolicyStatusResponse.active_version`, revision status and `loaded_at_ms` | Runtime reports load outcome; gateway persists the report | Authoritative for reported load acknowledgement | Accept with trust-boundary disclosure |
| Atomic operation-to-policy snapshot binding | Gateway/sandbox runtime | separate config/status and exec calls | Pre/post revision equality detects visible drift but cannot make the interval atomic or bind one exec operation to one revision | Corroborating | Blocking gap |
| Hard filesystem enforcement result | Driver/sandbox runtime | requested policy plus lifecycle/config OCSF events and driver-specific behavior | No typed complete applied-control snapshot across drivers | Unavailable as a complete attestation fact | Blocking gap |
| Hard process enforcement result | Driver/sandbox runtime | requested process policy and process/security observations | No typed capability or applied-control receipt | Unavailable as a complete attestation fact | Blocking gap |
| Hard network enforcement result | Sandbox network supervisor | effective network policy and network OCSF events | Individual allow/deny events can be observed; no operation-bound complete enforcement snapshot | Corroborating | Insufficient alone |
| Degradation, skipped controls, compatibility mode | Driver/sandbox runtime | conditions, messages, selected OCSF config/finding events | Driver-specific, not exhaustive, not typed as a complete security posture | Lossy/Corroborating | Blocking gap |

### 5.5 Driver-Observed Runtime Image

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Requested image reference | Caller/gateway | `SandboxTemplate.image` and `DriverSandboxTemplate.image` | Provisioning input only, even when digest-pinned | Requested | Cannot be relabeled as observed runtime evidence |
| Driver-observed immutable image digest | Compute driver | none in `DriverSandboxStatus`, driver conditions, or capability response | Driver status exposes platform instance identity and conditions, not the resolved image ID/digest actually running | Unavailable | Blocking gap |
| Runtime platform object identity | Compute driver | `DriverSandboxStatus.instance_id`, `sandbox_name` | Driver-observed runtime object reference | Authoritative for object identity | Accept as corroborating context for image proof |
| Image mismatch or mutable-tag detection after start | Compute driver | none | Gateway can require a digest-shaped request but cannot compare it with an observed runtime digest | Unavailable | Blocking gap |

### 5.6 Fixed Operation Identity And Outcome

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Fixed operation request | Workflow OS adapter | provider-owned request construction | Workflow OS can prevent caller-selected commands, but upstream does not commit an operation ID or request digest | Authoritative for adapter input only | Preserve, but insufficient for provider attestation |
| OpenShell operation identity | Gateway/sandbox supervisor | none in `ExecSandboxRequest` or `ExecSandboxEvent` | No operation resource, idempotency key, request echo, or digest | Unavailable | Blocking gap |
| Exact sandbox binding | Gateway exec relay | `ExecSandboxRequest.sandbox_id` | Request is routed to a sandbox ID, but response events do not echo sandbox or operation identity | Requested plus transport binding | Insufficient as durable receipt |
| Process start observation | Sandbox process supervisor/OCSF | process-activity OCSF event | Useful process observation, but no exact operation ID and no complete interval manifest | Corroborating | Insufficient alone |
| Terminal exit code | Sandbox exec stream | final `ExecSandboxExit.exit_code` | Authoritative final frame for that live stream | Authoritative for stream exit code only | Accept as one fact, not complete outcome |
| Timeout, signal, cancellation, and ambiguity posture | Sandbox exec stream | request timeout plus transport errors | No typed terminal reason, signal, cancellation state, or durable ambiguity record | Unavailable | Blocking gap |
| Operation start and finish times | Operation service | none in exec request/event | OCSF times can corroborate process events but are not bound to an upstream operation resource | Unavailable as exact operation facts | Blocking gap |
| Restart reconciliation after ambiguous exec | Gateway/sandbox runtime | no operation lookup resource | A disconnected caller cannot determine whether the exact operation ran or finished | Unavailable | Blocking gap |

### 5.7 Structured Security Observations

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Complete OCSF event records | Sandbox supervisor | opt-in JSONL files under `/var/log/openshell-ocsf.YYYY-MM-DD.log` | Documentation states each emitted event is a complete OCSF JSON object; file rotates daily and retains three files | Authoritative for each record present in the file | Useful source, not interval completeness proof |
| Machine-readable export | CLI over SSH/SFTP | `sandbox download` | Can copy a workspace-confined sandbox path; OCSF logs live under `/var/log`, outside the documented downloadable workspace boundary | Unavailable through documented safe download path | Blocking gap |
| Observation interval start/end watermarks | Sandbox supervisor/gateway | none | No operation-bound cursor, start/end watermark, flush acknowledgement, or manifest | Unavailable | Blocking gap |
| Event count, dropped count, and completeness posture | Sandbox supervisor/gateway | none for JSONL; `GetSandboxLogs.buffer_total` for bounded gateway buffer | Gateway log fetch is bounded and not a completeness receipt; JSONL has no export manifest or drop accounting | Unavailable | Blocking gap |
| Integrity commitment for exported observations | Sandbox supervisor/gateway | none | No provider-generated digest or signed/typed manifest binds file, sandbox, operation, range, and count | Unavailable | Blocking gap |
| Deliberate denied-egress fact | Network supervisor OCSF event | network activity event with denied disposition | Strong evidence for an individual denial, but exact operation and interval completeness are absent | Corroborating | Cannot satisfy the complete denied-egress proof alone |
| Gateway pushed logs | Sandbox log pusher and gateway | `PushSandboxLogs`, `GetSandboxLogs` | Default bounded buffer, line limit, no cursor, no dropped count, no interval manifest | Lossy | Explicitly reject as attestation source |

Raw OCSF records, stdout, stderr, command input, policy payloads, and provider
errors must not enter Workflow OS Core. A future adapter may reduce a complete
provider-side manifest into stable references, counts, hashes, and postures
only after the upstream completeness gap is closed.

### 5.8 Cleanup And Reconciliation

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Delete request accepted | Gateway/driver | public and driver `DeleteSandboxResponse.deleted` | Typed boolean that a resource was deleted by the request; public request uses name/workspace | Authoritative for request result only | Insufficient for terminal cleanup |
| Deletion initiation | Gateway metadata/driver | `deletion_timestamp_ms`, deleting state/phase | Indicates deletion began | Corroborating | Not terminal proof |
| Driver deletion event | Compute driver watch | `WatchSandboxesDeletedEvent.sandbox_id` | Driver-observed removal event bound to sandbox ID | Authoritative for observed driver removal event | Useful, but not exposed as a durable public deletion receipt |
| Delete bound to exact resource version | Gateway | none on public `DeleteSandboxRequest` | Public delete lacks `expected_resource_version`; a replaced same-name object can be targeted | Unavailable | Blocking gap |
| Terminal absence confirmation | Gateway/driver | later get/list absence plus driver watch | Polling absence can corroborate cleanup but lacks an exact immutable deletion receipt and can race with replacement | Corroborating | Blocking gap |
| Cleanup completion time and credential purge posture | Gateway/driver | docs state delete purges credentials; no typed response fact | Human documentation is not a per-operation receipt | Unavailable | Blocking gap |
| Restart-safe cleanup reconciliation | Gateway | get/list by name and workspace; stable ID not accepted by public get/delete | Cannot reliably reconcile the exact resource after an ambiguous delete response | Unavailable | Blocking gap |

### 5.9 Capability Negotiation

| Required fact | Enforcing or observing component | v0.0.101 surface | Binding and completeness | Classification | Integration result |
| --- | --- | --- | --- | --- | --- |
| Driver and gateway version | Gateway/driver | `GetGatewayInfo`, `GetCapabilities` | Typed human-readable identity | Authoritative for reported versions | Accept |
| Policy-input binding capability | Gateway | no capability flag | Must be inferred from schema/version inspection | Unavailable as negotiated capability | Blocking gap |
| Observed-image capability | Driver | no capability flag and no observed digest | Not supported at this pin | Unavailable | Blocking gap |
| Filesystem/process/network hard-control capabilities | Driver/sandbox runtime | no typed security capability set | Driver behavior differs and opaque `driver_config` cannot substitute for capability negotiation | Unavailable | Blocking gap |
| Complete-observation capability | Sandbox supervisor/gateway | no typed flag or export-manifest API | Enabling JSONL is a setting, not proof of complete export | Unavailable | Blocking gap |
| Cleanup and reconciliation capabilities | Gateway/driver | no typed flags | Must not be inferred from version strings | Unavailable | Blocking gap |

## 6. Accepted Facts And Blocking Gaps

### Accepted At This Pin

The following facts are strong enough to use within their narrow boundaries:

- exact upstream source tag, commit, and schema provenance;
- running gateway and driver version self-report;
- gateway-generated sandbox ID, creation time, workspace, and resource version;
- gateway-committed structured sandbox policy;
- canonical semantic policy hash, effective policy payload, policy version,
  source, global version, configuration revision, and failure mode;
- sandbox-reported policy load status and active version;
- driver-observed runtime object ID and lifecycle conditions; and
- final exit code for one connected exec stream.

### Blocking Gaps

Provider wiring remains blocked on:

1. restart-safe create idempotency and exact Workflow OS request binding;
2. exact operation identity and durable terminal outcome semantics;
3. driver-observed immutable image digest;
4. complete typed hard-control and degradation posture;
5. operation-bound complete OCSF export with watermarks, counts, drop posture,
   and integrity commitment;
6. resource-version-bound deletion and durable terminal cleanup receipt;
7. restart reconciliation for ambiguous create, execute, and delete activity;
   and
8. typed capability negotiation for every required security fact.

The exact original YAML-byte requirement also cannot be met. The next review
should decide whether the provider-neutral contract may safely use a canonical
structured-policy commitment plus gateway-generated provenance. That change
must not weaken effective-policy, load-status, or no-drift requirements.

## 7. Required Upstream Additions

A focused upstream proposal should request the smallest composable additions:

1. **Idempotent invocation identity**
   - client request/idempotency key on sandbox creation;
   - durable lookup by that key;
   - gateway-generated acceptance record binding request key, sandbox ID,
     resource version, and creation time.
2. **Policy commitment and applied-state receipt**
   - gateway-generated provenance for accepted canonical policy input;
   - explicit input/effective policy relationship;
   - immutable effective revision and applied control/degradation snapshot;
   - an operation-bindable loaded-policy reference.
3. **Driver-observed environment identity**
   - resolved immutable image digest;
   - driver runtime object UID and implementation identity;
   - typed mismatch and unsupported posture.
4. **Durable fixed-operation resource**
   - operation ID and caller request key;
   - canonical request digest;
   - sandbox and policy revision binding;
   - start/finish timestamps;
   - exit, timeout, signal, cancellation, and ambiguous outcome postures;
   - restart-safe lookup.
5. **Complete observation export manifest**
   - sandbox and operation identity;
   - start/end cursor or watermarks;
   - flush/finalization acknowledgement;
   - event count and dropped-event count;
   - content digest and stable export reference;
   - bounded class/disposition counts suitable for payload-free reduction.
6. **Exact cleanup receipt**
   - expected resource version on delete;
   - immutable deletion operation ID;
   - terminal driver and gateway absence confirmation;
   - completion timestamp and credential-purge posture;
   - restart-safe lookup after ambiguous responses.
7. **Typed capability snapshot**
   - explicit flags and versions for every fact above;
   - per-driver hard-control and degradation capabilities;
   - no inference from version strings.

These additions should be upstream-neutral in semantics even if the initial
proposal is made to OpenShell.

## 8. Provider-Neutral Contract Implications

The existing Workflow OS contract should not be weakened to match this pin.
One bounded clarification may be appropriate: replace exact source-file bytes
with an authoritative canonical structured-policy commitment when the
provider parses and persists a typed policy rather than retaining source
bytes. Such a change is acceptable only when the provider returns the
committed canonical input, effective composed policy, relationship between
them, applied revision, and load outcome.

No Rust contract amendment is authorized by this matrix. A focused review
must decide whether that clarification is sound before implementation.

`OpenShellNoWriteClient` remains intentionally disconnected. The current CLI
compatibility transport remains a drift-detecting compatibility probe, not an
attesting execution provider.

## 9. Fork Decision

Do not fork OpenShell.

This matrix identifies upstream gaps, but it does not establish that upstream
will reject the required hooks, that no trustworthy observer can provide
them, or that a narrow maintainable patch exists. A fork would prematurely
transfer container, VM, Kubernetes, network, process, credential, platform,
release, and vulnerability responsibility to Workflow OS.

The preferred path is:

1. focused maintainer review of this matrix;
2. upstream API proposal and engagement;
3. re-run the matrix against an accepted upstream release or reviewed commit;
4. only then implement the provider-neutral client and one fixed no-write live
   smoke proof.

## 10. Privacy And Security Posture

The matrix was produced from public source and documentation only. No
credential, provider material, sandbox, gateway, image, network, or external
mutation was used.

Future integration may project only payload-free facts such as stable IDs,
hashes, versions, bounded counts, timestamps, typed outcomes, and stable
evidence references. It must not project:

- raw policy or configuration payloads;
- stdout, stderr, command input, or environment values;
- raw OCSF or gateway logs;
- provider credentials or inference material;
- host or sandbox paths;
- internal endpoints; or
- unbounded upstream errors.

Errors and Debug output must use stable Workflow OS codes and must not copy
OpenShell payloads or human-readable driver condition messages.

## 11. Validation And Governed Evidence

Governed phase:

- workflow ID: `dg/d`;
- run ID: `run-1786267001100148000-2`;
- approval ID: `approval/run-1786267001100148000-2/planning-approved`;
- approval presentation ID: `presentation/3448aca4243e0d47`;
- approval outcome: granted by delegated maintainer;
- runtime status: completed before documentation edits;
- out-of-kernel work: read-only inspection of the pinned public upstream
  source and official documentation, plus documentation edits in this repo.

Validation required for phase close:

- `npm run check:docs`;
- `git diff --check`;
- inspection of the governed event trail; and
- confirmation that no runtime or live-provider changes entered the diff.

## 12. Remaining Limitations

- This is a source/schema assessment, not a live behavior proof.
- Release asset, binary, platform, driver, and image identities were not
  independently verified in a running environment.
- Gateway authorization, transport security, and hostile-provider behavior
  were not tested.
- Upstream may add, remove, or alter surfaces after the selected commit.
- A future live proof is still required even if upstream closes every API gap.

## 13. Recommended Next Phase

The focused maintainer review in
[OpenShell v0.0.101 Evidence-Sufficiency Matrix Review](../concepts/OPENSHELL_V0_0_101_EVIDENCE_SUFFICIENCY_MATRIX_REVIEW.md)
accepts the matrix. Prepare an upstream API proposal for the missing
invocation, image, operation, observation, cleanup, reconciliation, and
capability facts. Review the provider-neutral canonical structured-policy
commitment clarification separately before changing Rust contracts.

Do not implement provider wiring, install OpenShell, run a sandbox, fork the
runtime, or broaden provider mutations in that phase.

The focused
[OpenShell Upstream Attestation API Proposal](openshell-upstream-attestation-api-proposal.md)
now defines that provider-neutral resource and capability boundary. It remains
internal planning and has not been submitted upstream.
