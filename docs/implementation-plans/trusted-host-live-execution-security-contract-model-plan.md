# Trusted-Host Live-Execution Security Contract Core Model Plan

Status: planning complete; focused maintainer and security review required.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Cargo Clippy Live-Execution Security Boundary Plan](trusted-host-cargo-clippy-live-execution-security-plan.md)
- [Cargo Clippy Live-Execution Security Plan Review](../concepts/TRUSTED_HOST_CARGO_CLIPPY_LIVE_EXECUTION_SECURITY_PLAN_REVIEW.md)
- [OpenShell Optional No-Write Execution Provider Plan](openshell-optional-no-write-execution-provider-plan.md)
- existing hosted-execution and local-check attestation models in
  `crates/workflow-core/src/hosted.rs` and
  `crates/workflow-core/src/local_check_attestation.rs`

## 1. Executive Summary

Implement the first post-review slice as a provider-neutral, non-authorizing
Core model only. The model will describe the security requirements a future
trusted-host live execution must satisfy and the independently observed facts
a future provider may return. It will validate coherence without starting a
process, selecting a provider, discovering real Cargo inputs, granting
authority, or changing executor behavior.

The repository already has useful provider-neutral hosted vocabulary:

- requested policy binding;
- provider and request identity;
- enforcement mode;
- filesystem, process, and network control posture;
- bounded observation counts and references;
- cleanup posture;
- environment attestation; and
- exact terminal receipt binding.

The new model must compose with those concepts rather than create another
terminal receipt hierarchy. It should add only the facts the existing hosted
attestation cannot currently express:

- build-time input completeness posture;
- typed input-manifest commitment;
- typed root-binding requirements and observations;
- process-tree terminality distinct from command exit;
- transient-write prevention and observation posture;
- explicit requirement-to-observation assessment; and
- failure-closed ambiguity when mandatory facts are unavailable, degraded,
  unsupported, or uncertain.

## 2. Approved Scope

The implementation may add:

- provider-neutral model types in Workflow OS Core;
- validated constructors and private fields;
- bounded stable identifiers, commitments, counts, and enums;
- deterministic requirement and observation fingerprints;
- one pure assessment function that compares observations with requirements;
- redaction-safe Debug, Serialize, and fail-closed Deserialize behavior;
- focused unit/integration tests;
- crate exports only where justified for model tests and future provider
  composition; and
- honest implementation documentation and an end-of-phase report.

The model is descriptive and evaluative. It is not executable authority.

## 3. Strict Non-Goals

Do not implement:

- a process runner, subprocess call, shell, or live Cargo invocation;
- a live smoke test;
- real build-time input discovery;
- filesystem mounting, sandboxing, network policy, or process termination;
- provider selection, provider client behavior, or OpenShell integration;
- a new hosted receipt hierarchy;
- workflow composition or local-check registration;
- executor dispatch, admission, or authority changes;
- public CLI, configuration, workflow schema, SDK, onboarding, or examples;
- persistence, events, WorkReport, or artifact integration;
- dependencies or feature flags;
- provider mutations, credentials, or writes;
- hosted expansion or release-posture changes.

## 4. Existing Model Reuse

Reuse these existing types where their semantics match:

- `SpecContentHash` for deterministic commitments;
- immutable run-bundle and hosted request fingerprints for future exact
  binding;
- `HostedExecutionProviderId` and `HostedExecutionProviderVersion` only for an
  observed provider identity where needed;
- `HostedExecutionEnforcementMode`;
- `HostedExecutionControlPosture`;
- `HostedExecutionCleanupPosture`;
- `HostedExecutionReference` and its closed kind taxonomy;
- `HostedExecutionObservationSummary`; and
- `HostedExecutionAttestation` and `HostedExecutionReceipt` as the future
  environment and terminal receipt surfaces.

Do not widen those existing types merely to make the first implementation
convenient. Add a narrowly composed security contract and assessment beside
them, then let a later composition plan decide whether and how an accepted
assessment is attached to `HostedExecutionAttestation` or the terminal
receipt.

The model phase must not change existing hosted receipt validation or provider
behavior.

## 5. Requirement And Observation Separation

Use separate top-level concepts:

- `TrustedHostLiveExecutionSecurityRequirements`: what Core requires before a
  future invocation may be accepted; and
- `TrustedHostLiveExecutionSecurityObservations`: payload-free facts observed
  for one future provider attempt.

Construction of requirements must not imply that controls were enforced.
Construction of observations must not imply that requirements were satisfied.
Only a pure assessment function may compare them.

The assessment returns a closed result such as:

- `Satisfied`;
- `DeniedBeforeStart`; or
- `ReconciliationRequired`.

Names may be refined during implementation to match repository conventions,
but success, safe pre-start denial, and possibly-started uncertainty must
remain distinct.

## 6. Exact Binding Model

Both requirements and observations must bind to one exact future operation
without introducing executable behavior.

The binding should include validated commitments for:

- immutable run bundle;
- workflow, run, and step identity where existing bounded types are available;
- closed local-check profile or command-contract fingerprint;
- future durable attempt identity or attempt fingerprint;
- provider-neutral request fingerprint; and
- security requirements fingerprint.

The first slice may use commitments instead of importing executor-specific
objects where that keeps the module provider-neutral. It must not accept raw
command text, paths, environment values, or caller-defined labels as identity.

Observation/request substitution and attempt substitution must fail
deterministically.

## 7. Build-Time Input Model

Add a payload-free `BuildTimeInputManifest` or equivalent containing:

- deterministic manifest fingerprint;
- bounded total input count;
- closed `BuildTimeInputKind` counts or entries;
- `BuildTimeInputDiscoveryMethod`;
- `BuildTimeInputCompletenessPosture`; and
- optional bounded stable observation reference if an existing hosted
  reference kind is semantically correct.

Candidate input kinds:

- repository content;
- manifest or configuration;
- build script;
- procedural macro;
- dependency package;
- toolchain executable;
- linker or archiver;
- SDK or sysroot;
- dynamic library; and
- runtime/base image.

Do not store file names, paths, source contents, package payloads, environment
values, or raw inventories. If per-input references are necessary for future
substitution proof, use bounded stable IDs plus content/object commitments and
limit collection size.

Completeness posture must distinguish at least:

- `Complete`;
- `Incomplete`;
- `Unavailable`;
- `Unsupported`; and
- `Uncertain`.

Only `Complete` may satisfy a mandatory input-closure requirement. The model
does not prove that a discovery implementation deserves to report `Complete`;
that remains a later reviewed boundary.

## 8. Root Binding Model

Represent requirements and observations separately for typed root roles:

- repository input;
- toolchain input;
- dependency input;
- cache write;
- build-output write; and
- temporary write.

Requirement facts should include:

- required access class;
- required object-binding assurance;
- required lifecycle span; and
- whether invocation-scoped creation is mandatory.

Observation facts should include:

- root role;
- observed access class;
- object-binding posture;
- admission-to-terminal continuity posture;
- provider/platform observation method; and
- bounded observation commitment or stable reference.

Object-binding posture must distinguish enforced/proven, degraded,
unavailable, unsupported, and uncertain. It must not use a path string or inode
number as a portable public contract.

Duplicate roles, missing required roles, extra conflicting roles, role
substitution, and writable/read-only mismatch must fail closed.

## 9. Filesystem Containment Model

The requirement model should express:

- default-deny access outside declared roots;
- read-only protected roots;
- typed writable roots;
- host-home and credential-store exclusion;
- ambient temporary-path exclusion; and
- mandatory enforcement, not audit-only posture.

The observation model should compose the existing
`HostedExecutionControlPosture` with bounded facts for:

- unexpected read attempts;
- unexpected write attempts;
- denied filesystem actions;
- policy revision stability; and
- stable telemetry reference.

Counts must be bounded. The model must not store denied paths or payloads.

## 10. Network Containment Model

The requirements for the clippy profile are:

- enforce mode;
- default-deny egress;
- zero allowed endpoints;
- policy stability for the process lifetime; and
- denied-action observation support for a deliberate live proof probe.

Normal successful execution does not require an artificial network attempt.
Represent the probe requirement independently from the normal zero-egress
requirement so later live-proof tests can require a denied observation without
changing production command semantics.

Observations should reuse existing network counts and references where
possible. Allowed network count greater than zero cannot satisfy this profile.

## 11. Process-Tree Model

Requirements should represent:

- provider-owned process tree;
- descendant creation containment;
- timeout and cancellation propagation;
- terminal-tree confirmation;
- prevention of post-terminal mutation;
- explicit CPU, memory, process-count, file-descriptor, disk, and wall-clock
  budget posture; and
- mandatory versus optional budget enforcement.

Observations must distinguish:

- command exit;
- terminal process tree;
- uncertain descendants;
- cancellation or timeout initiated;
- termination completed; and
- cleanup completed.

Terminal process tree and cleanup are separate. A zero parent exit code with an
uncertain descendant cannot satisfy the requirements.

## 12. Transient-Write Model

Represent both prevention and observation:

- protected roots enforced read-only;
- protected-write attempts denied;
- unexpected write observation available/unavailable;
- transient-write observation method;
- writable outputs quiescent after process-tree termination; and
- retained artifact commitment posture.

Do not collapse:

- prevented;
- observed and denied;
- not observed;
- not observable;
- unsupported; and
- uncertain.

When a requirement demands prevention or observation, unavailable,
unsupported, degraded, or uncertain posture fails closed.

## 13. Effective-Control And Evidence Model

The observations should bind:

- requested policy hash;
- effective policy hash and revision;
- enforcement mode;
- filesystem, process, and network control posture;
- root-binding and input-manifest commitments;
- observation and cleanup references;
- bounded security counts;
- process terminality; and
- cleanup posture.

Where possible, compose existing `HostedExecutionAttestation` rather than
duplicate runtime image, effective policy, general control posture,
observation summary, and cleanup fields.

The security contract assessment may accept an existing hosted attestation plus
the new build-input/root/process-tree/transient-write observations. It must not
weaken existing `satisfies_hard_requirements()` behavior.

## 14. Control-State Vocabulary

Use one closed provider-neutral posture enum where semantics genuinely align,
preferably the existing `HostedExecutionControlPosture`:

- `Enforced`;
- `Degraded`;
- `Skipped`;
- `Unavailable`; and
- `Unsupported`.

Add `Uncertain` only where existing ambiguity vocabulary cannot represent an
observation whose truth cannot be established. Do not silently map uncertainty
to `Unavailable` or `Degraded` if retry/reconciliation semantics differ.

The plan review must decide whether to extend an existing enum or introduce a
narrow observation-certainty type. Compatibility and unknown-wire-value
failure behavior must be explicit.

## 15. Pure Assessment Function

Add one pure function, tentatively:

```text
assess_trusted_host_live_execution_security(
  requirements,
  observations,
) -> Result<TrustedHostLiveExecutionSecurityAssessment, WorkflowOsError>
```

The function must:

- verify exact request, attempt, requirement, and manifest binding;
- reject missing, duplicate, substituted, or conflicting root roles;
- require complete input posture;
- compare requested and effective policy identity;
- require enforced filesystem, process, and network controls;
- require zero allowed network events for this profile;
- require terminal process tree and compatible cleanup posture;
- enforce transient-write requirements;
- reject impossible or unsafe state combinations;
- distinguish pre-start denial from possibly-started reconciliation; and
- return only bounded stable reason codes.

It must not call a provider, inspect the filesystem, run a process, append an
event, change workflow status, or grant authority.

## 16. Coherence Rules

At minimum, reject:

- satisfied assessment with incomplete or uncertain input closure;
- satisfied assessment with any mandatory control not enforced;
- satisfied assessment with audit-only enforcement;
- satisfied assessment with allowed network events;
- satisfied assessment with policy drift;
- satisfied assessment with uncertain descendants;
- satisfied assessment with non-terminal process tree;
- satisfied assessment with ambiguous or failed required cleanup;
- satisfied assessment with required transient-write posture unavailable;
- pre-start denial when observations indicate process start;
- safe retry posture after a possibly-started observation;
- mismatched request, attempt, manifest, requirements, or provider identity;
- duplicate root roles or unknown required control kinds; and
- observation references of the wrong closed reference kind.

## 17. Authority Boundary

All new types must state in documentation and behavior:

- constructing requirements does not authorize execution;
- constructing observations does not prove satisfaction;
- a satisfied assessment is evidence about one exact attempt, not reusable
  authority;
- the model does not register a handler or provider;
- executor and local-check profile denial remain unchanged; and
- workflow policy, capability, proportional-governance, approval, and durable
  attempt creation remain prerequisites outside this model.

No API named `authorize`, `grant`, `execute`, `run`, or `dispatch` should be
added in this phase.

## 18. Validation And Error Design

Use repository-established constructor validation, private fields, stable
error codes, and `try_from` serde validation.

Candidate stable code families:

- `trusted_host.live_security.requirements.*`;
- `trusted_host.live_security.input_manifest.*`;
- `trusted_host.live_security.root_binding.*`;
- `trusted_host.live_security.observation.*`; and
- `trusted_host.live_security.assessment.*`.

Errors must not include IDs, fingerprints, paths, source contents, command
output, environment values, provider payloads, or secret-like inputs.

Deserialize through validated wire types. Unknown enum values and invalid state
combinations fail closed with generic non-leaking messages.

## 19. Privacy And Redaction

The model stores only bounded identities, commitments, enums, counts,
timestamps if required, and stable references.

It must not store:

- raw source or dependency content;
- file or host paths;
- command output;
- environment values;
- credentials, tokens, headers, private keys, or provider payloads;
- raw filesystem or network observations; or
- sandbox logs.

Custom Debug must redact identifiers, fingerprints, references, and any field
whose future value could reveal infrastructure topology. Serialization is an
explicit contract and must contain no hidden payload escape hatch.

## 20. Module And Export Posture

Prefer one focused Core module such as
`trusted_host_live_execution_security.rs`. The implementation review may place
the types beside `hosted.rs` if doing so materially reduces duplication and
keeps ownership clear.

Export only the model concepts needed by focused tests and future provider
composition. Keep assessment helpers crate-private if no external contract
requires them yet.

Do not add feature flags, persistence migrations, schemas, SDK models, or CLI
surface.

## 21. Test Plan

Add focused tests for:

1. valid minimal requirements;
2. valid complete observations;
3. satisfied exact assessment;
4. construction does not imply satisfaction or authority;
5. incomplete, unavailable, unsupported, and uncertain input closure;
6. every build-time input kind is representable;
7. manifest count and collection bounds;
8. missing, duplicate, extra, and substituted root roles;
9. read-only/writable root mismatch;
10. requested/effective policy mismatch;
11. audit-only and degraded mandatory controls;
12. non-zero allowed network events;
13. denied-egress probe requirement separated from normal invocation;
14. non-terminal process tree despite zero parent exit;
15. uncertain descendants;
16. timeout/cancellation without confirmed termination;
17. terminal process tree with incomplete cleanup remains distinct;
18. transient-write prevention unavailable or uncertain;
19. observation and cleanup reference-kind validation;
20. request, attempt, manifest, and requirements substitution;
21. pre-start denial versus reconciliation-required mapping;
22. impossible state combinations fail closed;
23. deterministic fingerprints;
24. serde round trip for valid values;
25. invalid serialized values fail closed;
26. Debug and errors do not leak secret-like values;
27. serialization has no raw payload/path/output fields;
28. existing hosted attestation and receipt tests remain unchanged;
29. existing injected Cargo clippy tests remain unchanged;
30. executor denial and registration exclusion remain unchanged; and
31. `cargo test --workspace` passes.

Tests must not spawn Cargo, a shell, a provider, or a sandbox.

## 22. Documentation Updates

The implementation phase should update:

- this plan's status;
- the Cargo clippy live-execution security plan if a concise implementation
  status link is useful;
- `ROADMAP.md`; and
- a new end-of-phase implementation report.

Docs must state that the model exists but no runner, live execution, input
discovery, provider selection, OpenShell integration, composition,
registration, public configuration, or execution authority exists.

## 23. End-Of-Phase Report

Create:

```text
docs/concepts/TRUSTED_HOST_LIVE_EXECUTION_SECURITY_CONTRACT_MODEL_REPORT.md
```

The report must include:

1. executive summary;
2. scope completed;
3. scope explicitly not completed;
4. model types added and existing types reused;
5. requirement/observation separation;
6. coherence and authority boundaries;
7. validation and stable-error summary;
8. redaction/privacy summary;
9. test coverage;
10. commands and results;
11. remaining live-process blockers; and
12. recommended next phase.

## 24. Implementation Validation

Run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused model tests;
- existing hosted-execution attestation tests;
- existing Cargo clippy profile tests;
- executor-denial and registration-exclusion regressions;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

## 25. Acceptance Criteria

- Requirements and observations are separate validated concepts.
- Existing hosted attestation vocabulary is reused where semantically correct.
- The new model adds no second terminal receipt hierarchy.
- Input completeness, root binding, process-tree terminality, transient-write,
  and ambiguity are explicitly representable.
- A pure assessment rejects every unsafe or incoherent combination.
- Exact request and attempt substitution fail closed.
- A satisfied assessment grants no execution authority.
- Debug, errors, serialization, and deserialization are redaction-safe.
- No runner, process, provider, input discovery, composition, registration,
  persistence, schema, SDK, CLI, dependency, write, or release change exists.
- Existing hosted and injected clippy behavior remains unchanged.
- An end-of-phase report exists.

## 26. Recommended Implementation Sequence

1. Add the focused provider-neutral model module.
2. Reuse or compose existing hosted types.
3. Add validated requirement, observation, and assessment constructors.
4. Add coherence validation and stable errors.
5. Add serde and custom Debug behavior.
6. Add focused tests, including authority non-exposure regressions.
7. Run the full validation matrix.
8. Create the implementation report.
9. Perform focused maintainer/security review before any input-discovery work.

## 27. Final Recommendation

Review this model plan. If accepted, implement the core model as one bounded
non-authorizing phase.

Do not run Cargo, add a runner, implement real input discovery, select a
provider, integrate OpenShell, compose a workflow, register the clippy profile,
add public configuration, persist the model, add dependencies, enable writes,
or change release posture.

## 28. Governed Planning Record

- Workflow ID: `dg/d`.
- Run ID: `run-1792811067484934000-2`.
- Approval ID:
  `approval/run-1792811067484934000-2/planning-approved`.
- Approval-presentation ID: `presentation/5c39e6351939a401`.
- Approval-presentation content hash:
  `5c39e6351939a4016db5b7f337fa8fc32e65bf3c95c89693a99f8bd84f1862e9`.
- Approval outcome: granted under standing delegated-maintainer authority
  through the proof-enforced path for the exact planning-only scope.
- Validation: documentation and diff checks are required at phase close.
- Out-of-kernel work: the delegated maintainer inspected existing Core models,
  authored this plan and roadmap update, and ran validation. The kernel governed
  scope and approval but did not edit files, run checks, or perform git and PR
  operations.
