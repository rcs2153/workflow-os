# Provider-Neutral Trusted-Host Input Observer Plan

Status: Core model accepted and fixture-backed in-memory observer proof
implemented. Focused proof review is next.

This plan follows:

- [Trusted-Host Build-Input Discovery Proof Plan](trusted-host-build-input-discovery-proof-plan.md)
- [Trusted-Host Live-Execution Security Contract Core Model Plan](trusted-host-live-execution-security-contract-model-plan.md)
- [Trusted-Host Cargo Clippy Live-Execution Security Boundary Plan](trusted-host-cargo-clippy-live-execution-security-plan.md)
- [OpenShell Upstream API Attestation Contract Plan](openshell-upstream-api-attestation-contract-plan.md)

## 1. Executive Summary

Workflow OS now has an accepted process-free proof for one real Cargo
workspace. That proof binds the approved static metadata graph but correctly
leaves nine host obligations unavailable: toolchain executables, target
specification, SDK/sysroot, linker/archiver, native compiler tools, dynamic
libraries, registry/package sources, effective environment inputs, and runtime
base identity.

The next implementation should add a provider-neutral, non-authorizing Core
contract for observing those facts. The observer contract must bind every
result to one exact immutable operation and static discovery result, disclose
which component observed each fact, distinguish observed facts from declared
or unavailable facts, and return only bounded commitments and stable
references.

The first implementation is model-only. It must not inspect the host, run a
process, choose a provider, admit execution, persist results, or integrate
OpenShell. A later fixture-backed proof may exercise the contract without host
access. Live observation and provider integration require separate planning,
review, and approval.

## 2. Goals

- Define the smallest provider-neutral request/result contract for trusted-host
  input observation.
- Bind observation to one exact immutable run, step, operation, static
  discovery result, and security-requirement commitment.
- Represent all nine host obligations as one closed, complete set.
- Distinguish declared, independently observed, unavailable, unsupported, and
  uncertain facts.
- Record provenance, freshness, object-binding assurance, and continuity
  posture without storing host payloads.
- Make provider capability gaps explicit and fail closed.
- Compose with the accepted static discovery and live-security models.
- Preserve a future implementation boundary for OpenShell or another reviewed
  observation provider.

## 3. Strict Non-Goals

This plan does not authorize or implement:

- host inspection, filesystem traversal, environment reads, or executable
  resolution;
- Cargo, rustc, linker, archiver, build-script, procedural-macro, or shell
  execution;
- process creation, sandboxing, mounts, network controls, or cleanup;
- provider selection, OpenShell integration, or a provider client;
- runtime admission, dispatch, capability grants, approvals, or policy
  decisions;
- construction of a complete live `BuildTimeInputManifest`;
- persistence, workflow events, WorkReports, artifacts, schemas, SDKs, CLI
  behavior, examples, or onboarding changes;
- credentials, provider mutations, external writes, or release changes; or
- claims that the current trusted-host check is production safe.

## 4. Existing Foundations

The observer must compose existing sources of truth rather than replace them:

| Concern | Existing source of truth | Observer role |
| --- | --- | --- |
| Governed work identity | validated immutable run bundle | bind only |
| Static Cargo graph | `TrustedHostBuildInputDiscoveryResult` | consume exact result commitment |
| Host obligation taxonomy | `TrustedHostBuildInputHostObligationKind` | return one observation per closed kind |
| Live security requirements | `TrustedHostLiveExecutionSecurityRequirements` | bind only |
| Root and process controls | trusted-host live-security observations | do not duplicate |
| Provider runtime facts | future provider attestation | map only after independent review |
| Admission | future Core composition | no authority |

The observer does not upgrade a static result from incomplete to complete. A
later composition function may combine an accepted static result, accepted
host observations, and security observations into a complete manifest claim.

## 5. Trust Boundary

The contract describes evidence from a trusted observation boundary. It does
not make arbitrary caller assertions trustworthy.

The first model must identify:

- the observation producer kind and version;
- the observation channel assurance;
- the exact request and result commitments;
- the independently bound operation and static result;
- the observation interval or ordered revisions;
- the object-binding and continuity method used for each fact; and
- stable evidence references where an accepted external evidence store exists.

`Observed` means the identified observer produced a fact through its declared
channel. It does not mean Core has accepted the observer implementation. The
later provider-review boundary decides which producer identities and methods
are trusted for live use.

## 6. Candidate Core Model

The first implementation should add only the vocabulary required to validate
and canonically commit a request and result, likely including:

- `TrustedHostInputObserverContractVersion`;
- `TrustedHostInputObservationRequest`;
- `TrustedHostInputObservationRequestDefinition`;
- `TrustedHostInputObservationResult`;
- `TrustedHostInputObservationResultDefinition`;
- `TrustedHostInputObservationProducer`;
- `TrustedHostInputObservationFact`;
- `TrustedHostInputObservationPosture`;
- `TrustedHostInputObservationMethod`;
- `TrustedHostInputObjectBindingPosture`;
- `TrustedHostInputContinuityPosture`; and
- `TrustedHostInputObservationAssessment` or one equivalent pure validation
  result if needed.

The result must compose the existing `TrustedHostRootObservation` for the
repository input root. It must not add another repository-root identity or
continuity type merely for observer convenience.

Names may be refined to follow existing Core conventions. Fields remain
private, constructors validate all invariants, deserialization uses strict
validated wire definitions, and public exports are limited to future
provider-neutral composition needs.

## 7. Exact Request Binding

An observation request must bind validated commitments for:

- immutable run bundle;
- workflow, run, and step definition;
- closed operation/profile contract;
- static discovery request and result;
- repository selection and root identity claim;
- trusted-host live-security requirements;
- target and feature selection;
- durable attempt identity when one exists; and
- observer contract version.

Raw paths, command strings, environment values, provider labels, and current
mutable repository files are not identity.

The request fingerprint must use a versioned canonical domain and fixed-width
framing. Field order, set ordering, and integer encoding must be deterministic
and covered by pinned vectors.

The repository selection/root value in the request is only a claim to be
observed. It is not independent evidence and cannot satisfy repository object
binding by itself.

## 8. Closed Observation Set

Every valid result contains exactly one fact for each closed host obligation:

1. toolchain executable;
2. target specification;
3. SDK or sysroot;
4. linker or archiver;
5. native compiler tools;
6. dynamic libraries;
7. registry and package sources;
8. effective environment inputs; and
9. runtime base.

Missing and duplicate obligations fail validation. Unsupported or unavailable
facts remain explicit entries; they are not omitted.

Each fact should contain only:

- obligation kind;
- closed observation posture;
- deterministic fact-set commitment;
- bounded item count;
- closed observation method;
- object-binding posture;
- continuity posture;
- producer identity/version binding;
- optional stable evidence reference of an accepted kind; and
- bounded stable reason code when not proven.

The closed nine-obligation set is necessary but not sufficient for a valid
result. The result also requires exactly one repository-root observation as
defined below.

## 9. Repository-Root Observation

Every result must compose exactly one existing `TrustedHostRootObservation`
whose role is `TrustedHostRootRole::RepositoryInput`. That observation must
bind:

- the exact repository commitment accepted by static discovery;
- `TrustedHostRootAccess::ReadOnly`;
- object-binding posture;
- continuity posture across the relevant observation/admission interval;
- one closed `TrustedHostRootObservationMethod`;
- `invocation_scoped = false`; and
- one valid telemetry reference.

A future satisfied assessment requires proven object binding and proven
continuity through an accepted method. `Unavailable`, `Unsupported`,
`Degraded`, or `Uncertain` remains representable but cannot satisfy the later
live-execution requirement.

The observer result fingerprint must canonically commit the complete
repository-root observation. Static discovery result substitution, repository
commitment substitution, duplicate repository roots, a non-repository role,
writable access, invocation-scoped repository creation, or incoherent
method/posture combinations fail closed.

The model does not choose how a live implementation proves the root. Protected
handles, immutable projections, and provider-native stable identities remain
the accepted vocabulary. Git-specific observation, if later selected, belongs
behind the provider boundary and requires separate review.

## 10. Observation Posture

The model must distinguish:

- `Observed`: the producer reports an independently observed, object-bound fact;
- `DeclaredOnly`: configuration or metadata requested the fact but no
  independent observation proved it;
- `Unavailable`: the channel could not obtain the fact;
- `Unsupported`: the observer contract or provider cannot observe the fact;
  and
- `Uncertain`: observation may have occurred but integrity, continuity, or
  completeness cannot be established.

Only `Observed` with the required object-binding and continuity posture may
satisfy a mandatory obligation in a later assessment. `DeclaredOnly` never
does.

## 11. Per-Obligation Requirements

### Toolchain executable

Bind the actual compiler and required component object set, not a requested
channel string or version label. The commitment must cover every resolved
toolchain executable used by the closed operation.

### Target specification

Bind the effective target specification consumed by the operation, including
whether it was built-in or externally supplied. A target name alone is
insufficient.

### SDK or sysroot

Bind the effective SDK/sysroot object set and its continuity through the
observation interval. Host installation paths remain private.

### Linker or archiver

Bind the exact resolved linker, archiver, and equivalent platform tools. A
configuration declaration without resolution is `DeclaredOnly`.

### Native compiler tools

Bind any selected C/C++ compiler, assembler, package-config helper, or other
native build tool required by the closed graph. Absence must be observed, not
assumed from repository metadata.

### Dynamic libraries

Bind the dynamically loaded library set relevant to toolchain and build-tool
execution. The first live observer must define whether this is preflight,
runtime, or both; the model records the method explicitly.

### Registry and package sources

Bind immutable source objects for every selected external dependency. Registry
checksums, Git commits, and local path roots need distinct reviewed methods but
one provider-neutral commitment surface.

### Effective environment inputs

Use a closed allowlist of environment influence classes and commit names and
values without exposing either. Unknown relevant inputs or inability to prove
the allowlist complete yields `Uncertain` or `Unavailable`.

### Runtime base

Bind the host/sandbox/container base that can affect tool and library
resolution. A provider image tag or host OS label is not an immutable object
binding.

## 12. Object Binding And TOCTOU

Canonical paths and preflight hashes are not sufficient for live authority.
Each observed fact must declare how its object identity was retained across
the observation interval, using a closed method such as:

- immutable content-addressed object;
- protected handle or mount identity;
- provider-native immutable projection;
- independently verified pre/post object identity; or
- unavailable/unsupported.

The contract must separately represent:

- identity binding at observation;
- continuity from observation through future execution admission; and
- whether the same object can later be proven at terminal reconciliation.

The model must not imply portable inode semantics. Substitution, drift, stale
facts, or unverifiable transitions fail closed.

## 13. Freshness And Reobservation

Observation is valid only for the exact request and bounded interval. The
result should include ordered revisions or bounded timestamps sufficient to
show:

- static discovery completed before host observation;
- observation completed before any future admission decision;
- the result has not exceeded its closed freshness policy; and
- a changed request, repository object, toolchain selection, target, provider
  configuration, or security requirement requires reobservation.

No result is reusable authority. Cache reuse is allowed only when the cache
key binds every semantic request input and the underlying objects remain
immutably bound. Cache hit remains an observation method, not a bypass.

## 14. Provider-Neutral Observer Boundary

A future implementation may expose an internal interface conceptually like:

```text
observe_trusted_host_inputs(request)
  -> Result<TrustedHostInputObservationResult, WorkflowOsError>
```

The interface must:

- accept validated explicit input only;
- return one complete closed result or a stable non-leaking error;
- have no access to workflow mutation APIs;
- append no events and grant no authority;
- avoid implicit global configuration;
- avoid retries after possible host interaction unless exact reconciliation is
  defined; and
- expose provider capability gaps before any future process starts.

The first model phase does not add this callable interface. The fixture proof
may use a pure constructor-backed fake observer, not host inspection.

## 15. Assessment And Admission Separation

Three steps remain distinct:

1. static discovery states what repository metadata can prove;
2. host observation states what an accepted channel observed; and
3. a later Core assessment decides whether the combined evidence satisfies the
   exact live-security requirements.

Even a complete observation result does not authorize execution. Admission
still requires immutable run-bundle validation, current authority and
capability resolution, policy and proportional-governance routing, any
required approval with presentation proof, durable attempt creation, complete
static/host input assessment, and containment capability assessment.

## 16. Failure And Ambiguity Semantics

Use stable errors and closed reason codes. At minimum fail closed for:

- request, result, static-discovery, operation, attempt, or requirement
  substitution;
- missing, duplicate, substituted, writable, invocation-scoped, or
  role-incompatible repository-root observations;
- missing or duplicate obligation facts;
- count/commitment inconsistency;
- unsupported observation method for a proven claim;
- observed claim without object binding;
- stale observation or continuity gap;
- producer identity/version mismatch;
- unknown wire fields or enum values;
- oversized facts, counts, references, or reason collections; and
- secret-like or raw-payload input at a public boundary.

Before any future process start, failure is a pre-start denial. If a later
provider may have interacted with the host, uncertainty must use the existing
possibly-started/reconciliation posture and must not be retried blindly.

## 17. Privacy And Redaction

The public model stores commitments, counts, closed postures, bounded stable
identifiers, and accepted evidence references only. It must not store or emit:

- host paths or user names;
- executable names when they reveal private configuration;
- raw manifests, source, lockfiles, registry data, or package payloads;
- environment names or values;
- command text, stdout, stderr, or provider logs;
- dynamic-library names/paths;
- endpoints, credentials, tokens, authorization headers, or private keys; or
- raw provider responses.

Debug output summarizes shape and posture only. Validation and deserialization
errors never echo rejected values or unknown field names.

## 18. Relationship To OpenShell

OpenShell remains an optional future execution and observation provider, not a
dependency of Core and not selected by this plan.

A later adapter may map authoritative, version-pinned OpenShell facts into the
observer result only if upstream exposes every mandatory fact with exact
request, policy, object, lifecycle, and evidence binding. Repeated requested
configuration, best-effort CLI output, or unbound sandbox labels are not
observation proof.

If OpenShell cannot expose a mandatory fact, the obligation remains
`Unsupported` or `Unavailable`; Workflow OS must not weaken the contract. A
fork remains out of scope unless a separate ADR demonstrates that upstream
blocks required governance hooks and that Workflow OS can own the added
security surface.

## 19. Test Plan

The model-only implementation should prove:

- valid request/result construction and stable fingerprints;
- exact representation of all nine obligations;
- rejection of missing and duplicate obligations;
- request/result/static-discovery/attempt substitution rejection;
- required repository-root observation presence and role uniqueness;
- exact static repository commitment binding;
- rejection of writable, invocation-scoped, or wrong-role repository roots;
- rejection of incoherent root methods and object-binding/continuity postures;
- unavailable repository binding remains representable but cannot support a
  future satisfied assessment;
- reuse of existing `TrustedHostRootObservation` semantics without a parallel
  root-binding model;
- declared-only facts cannot satisfy observation posture;
- observed facts require compatible object binding and continuity;
- unavailable, unsupported, and uncertain facts remain explicit;
- stale or mismatched producer facts fail closed;
- deterministic ordering and pinned canonical vectors;
- strict unknown-field and unknown-enum rejection;
- serde round trip for valid values;
- Debug, serialization, and error non-leakage;
- bounded count and collection enforcement;
- no process, filesystem, environment, network, provider, persistence, event,
  CLI, or authority API is introduced; and
- existing static-discovery, live-security, hosted-execution, executor, and
  workspace tests remain unchanged.

The later fixture proof should construct the same semantic result twice from
an explicit synthetic fact source and prove deterministic commitments without
reading the real host.

## 20. Proposed Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Core request/result/fact model and canonical commitments only.
3. Focused model review and blocker repair.
4. Fixture-backed in-memory observer proof with no host or process access.
5. Focused proof review.
6. Separate planning for one live provider implementation and provider
   capability/evidence matrix.
7. Only after provider review, plan static-plus-host assessment and runtime
   admission composition.

Do not combine model, fixture proof, live observation, provider integration,
assessment, and admission into one phase.

## 21. Open Questions

- Should the first contract support only Cargo/Rust host obligations while
  keeping names provider-neutral, or define a cross-language taxonomy now?
- Which producer identity type should be reused without implying that every
  hosted execution provider is an accepted observer?
- Which existing hosted reference kind, if any, correctly identifies
  observation evidence?
- What freshness representation avoids wall-clock authority while still
  supporting ordered observation and admission?
- How should registry packages, Git dependencies, and local path dependencies
  express different object-binding methods?
- Can environment influence be represented by a closed versioned class set
  without storing variable names?
- Should dynamic-library completeness be preflight-only or require runtime
  loader observation for the first live profile?
- Which future provider method can produce the required
  `TrustedHostRootObservation` without coupling Core to Git?

## 22. Final Recommendation

Implement the fixture-backed in-memory observer proof already sequenced in
this plan. Use explicit synthetic facts and the accepted Core constructors to
prove complete, deterministic result construction without host access.

Do not implement a live observer, inspect a host, execute a process, select or
integrate a provider, admit runtime execution, persist observations, add
schemas or CLI behavior, enable writes, or change release posture.

## 23. Blocker-Fix Record

- Workflow ID: `dg/blocker`.
- Run ID: `run-1792833600231944000-2`.
- Approval ID: `approval/run-1792833600231944000-2/fix-approved`.
- Approval-presentation ID: `presentation/6372bd71ea78af0f`.
- Approval-presentation content hash:
  `6372bd71ea78af0f11fe3b39e9c166f56bb97d44834e32d5428521fa8eef3a78`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact documentation-only blocker-fix scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and its
  event marker is present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.

## 24. Governed Planning Record

- Workflow ID: `dg/d`.
- Run ID: `run-1792833273051217000-2`.
- Approval ID:
  `approval/run-1792833273051217000-2/planning-approved`.
- Approval-presentation ID: `presentation/e536f4597a081aef`.
- Approval-presentation content hash:
  `e536f4597a081aefc4ff7b87af3d2f3b365d8c7423f8b36a0851e21ade1e875c`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact planning-only scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and its
  event marker is present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected accepted security,
  static-discovery, immutable-binding, hosted-provider, and OpenShell plans;
  authored this plan and roadmap update; and ran documentation validation. The
  kernel governed scope and approval but did not edit files, run validation,
  or perform git or pull-request operations.

## 25. Implementation Record

The model-only implementation is documented in [Trusted-Host Input Observer
Core Model Report](../concepts/TRUSTED_HOST_INPUT_OBSERVER_CORE_MODEL_REPORT.md).
It adds no callable observer, host inspection, process execution, provider
integration, runtime admission, persistence, events, schemas, CLI behavior,
writes, or authority.

## 26. Core Model Blocker-Fix Record

The seven focused model-review blockers are fixed. The request now binds an
exact expected producer identity/version and static-discovery completion
revision. Results bind homogeneous per-fact provenance, enforce exact
non-proof reasons and posture/method/producer coherence, reject stale
observations, and use non-echoing deserializers for every nested enum reachable
through the observer wire boundary.

The fix remains model-only and non-authorizing. See [Trusted-Host Input
Observer Core Model Blocker Fix
Report](../concepts/TRUSTED_HOST_INPUT_OBSERVER_CORE_MODEL_BLOCKER_FIX_REPORT.md).
Focused blocker-fix review is next; no fixture-backed or live observer work is
authorized yet.

## 27. Blocker-Fix Review Record

Focused re-review accepted producer binding, homogeneous per-fact provenance,
closed non-proof reasons, posture/method/producer coherence, nested wire
privacy, and pinned version-two commitments. One blocker remains: the request
stores a static-discovery completion revision while the result documents its
start/completion revisions as producer local. Comparing those unrelated
counters does not establish a shared temporal order.

The next fix must bind one explicit ordered-revision authority/domain shared by
discovery and observation, or use an equivalent request-bound non-clock
freshness challenge. It must remain model-only. See [Trusted-Host Input
Observer Core Model Blocker Fix
Review](../concepts/TRUSTED_HOST_INPUT_OBSERVER_CORE_MODEL_BLOCKER_FIX_REVIEW.md).

## 28. Freshness-Authority Fix Record

The request and result now carry the same payload-free ordering-authority
commitment. Static-discovery completion, observation start, and observation
completion revisions are explicitly defined in that shared domain.
Result-to-request validation rejects authority substitution before accepting
the numeric freshness relation, and both canonical commitments include the
authority binding.

The change remains model-only and introduces no clock, observer, process,
provider, persistence, runtime admission, schema, CLI, write, or authority
behavior. See [Trusted-Host Input Observer Freshness Authority Fix
Report](../concepts/TRUSTED_HOST_INPUT_OBSERVER_FRESHNESS_AUTHORITY_FIX_REPORT.md).

## 29. Freshness-Authority Fix Review Record

Focused maintainer/security review accepts the shared ordering-authority
semantics with no remaining Core-model blocker. Request and result bind the
same authority, both canonical commitments include it, substitution and stale
observation fail closed, and Debug and error paths remain non-leaking.

The model remains non-authorizing. A later provider review must determine
which producer and ordering-authority identities may support live admission.
The next phase is the already-planned fixture-backed in-memory observer proof,
with no host inspection or process access. See [Trusted-Host Input Observer
Freshness Authority Fix
Review](../concepts/TRUSTED_HOST_INPUT_OBSERVER_FRESHNESS_AUTHORITY_FIX_REVIEW.md).

## 30. Fixture Proof Record

The process-free fixture proof is implemented. One explicit synthetic source
now constructs an exact-request-bound result through the accepted Core
constructor and validator. Repeated construction is deterministic; stale,
incomplete, substituted, or non-fixture input fails closed.

No host access, process, provider, persistence, admission, event, schema, CLI,
write, or authority behavior was added. Focused proof review is next. See
[Trusted-Host Input Observer Fixture Proof
Report](../concepts/TRUSTED_HOST_INPUT_OBSERVER_FIXTURE_PROOF_REPORT.md).
