# Trusted-Host Cargo Clippy Live-Execution Security Boundary Plan

Status: planning complete; focused maintainer and security review required.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Trusted-Host Cargo Clippy Profile Plan](trusted-host-cargo-clippy-profile-plan.md)
- [Trusted-Host Cargo Clippy Profile Blocker-Fix Review](../concepts/TRUSTED_HOST_CARGO_CLIPPY_PROFILE_BLOCKER_FIX_REVIEW.md)
- [OpenShell Optional No-Write Execution Provider Plan](openshell-optional-no-write-execution-provider-plan.md)
- [OpenShell No-Write Provider Threat Model](../security/openshell-no-write-provider-threat-model.md)
- [Workflow OS Threat Model](../security/THREAT_MODEL.md)

## 1. Executive Summary

The injected-runner-only Cargo clippy profile proves a closed command,
sanitized environment, typed writable roots, protected-input commitments,
bounded results, and denial from ordinary executor and registration paths. It
does not prove that a real Cargo process can execute safely.

This plan defines the security boundary that must exist before Workflow OS may
consider one real invocation of:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

The boundary must close six risks that fixture-based injection cannot close:

1. complete discovery and attestation of build-time code and tools;
2. root object identity and time-of-check/time-of-use behavior;
3. full process-tree lifecycle and termination;
4. operating-system filesystem and network containment;
5. transient write visibility; and
6. independent evidence that requested controls were actually enforced.

The first implementation after this plan should model and validate the
provider-neutral live-execution security contract only. It must not start
Cargo, register the profile, expose public configuration, or select OpenShell.
A live smoke remains a later, separately approved phase after implementation
and focused security review close every blocker.

## 2. Decision

Do not add a standard host-process runner for Cargo clippy.

A future real invocation must cross an explicit containment-provider boundary
that returns independently observed security evidence. The provider may later
be implemented by OpenShell or another reviewed substrate. Core must define
the required facts without depending on provider-specific labels or trusting
the request echoed back as proof.

The term `trusted-host` describes the operator and machine trust posture. It
does not exempt the command from containment because Cargo may execute
repository and dependency build scripts, procedural macros, compiler wrappers,
linkers, SDK tools, and other build-time programs.

## 3. Goals

- Define a closed, provider-neutral security contract for one Cargo clippy
  invocation.
- Bind the operation to the exact immutable run bundle and accepted profile.
- Require complete, deterministic build-time input discovery before execution.
- Distinguish requested controls from provider-observed effective controls.
- Require object-identity and TOCTOU controls appropriate to each root role.
- Require full process-tree cancellation, timeout, and terminal reconciliation.
- Enforce filesystem and network policy outside the child process.
- Observe or prevent transient protected-state writes.
- Return bounded, payload-free, report-ready security evidence.
- Preserve current executor denial and registration exclusion until a later
  phase explicitly changes them.

## 4. Strict Non-Goals

This planning phase does not implement or authorize:

- a process runner or live Cargo invocation;
- a live smoke test;
- workflow composition or default registration;
- public CLI, config, schema, SDK, or onboarding exposure;
- a new local-check profile or arbitrary command execution;
- OpenShell installation, selection, integration, or fork;
- provider access, credentials, external SideEffects, or writes;
- hosted or distributed execution;
- source, manifest, lockfile, or generated-source mutation;
- dependency download or toolchain installation;
- report delivery or raw command-output evidence;
- cryptographic, hardware, or production-sandbox attestation claims;
- release-posture changes.

## 5. Threat Model

The future command runs repository and dependency-controlled build-time code
under a local operating-system identity. That code may attempt to:

- read undeclared host files or credentials;
- write protected repository or toolchain content and restore it before a
  final-state hash check;
- replace or redirect roots between validation and use;
- spawn descendants that outlive the parent process;
- open network connections independently of Cargo's offline setting;
- invoke undeclared executables or dynamically loaded libraries;
- alter generated output after the command appears complete;
- exhaust process, memory, disk, file-descriptor, or time budgets; or
- forge a provider receipt that merely repeats requested policy.

The boundary may trust the reviewed Workflow OS Core process, the immutable
run bundle, and an explicitly accepted containment provider implementation. It
must not trust build-time code, command output, mutable path names, ambient
environment, or provider self-description without independent observation.

## 6. Source-Of-Truth Boundaries

| Concern | Required source of truth | Not sufficient |
| --- | --- | --- |
| Workflow and profile identity | Stored immutable run bundle and closed profile resolver | Command text or caller claim |
| Permission to invoke | Current Workflow OS authority, policy, checks, and approval posture | Handler availability |
| Input identity | Complete Core-validated manifest plus content and object commitments | `Cargo.lock` alone |
| Requested containment | Core-owned security policy binding | Provider defaults |
| Effective containment | Provider-observed policy and independent control observations | Requested policy echoed in a receipt |
| Process outcome | Bound terminal process-tree observation | Parent exit code alone |
| Protected-state integrity | OS containment plus independent mutation observations | Pre/post final-state hash alone |
| Network posture | OS/provider egress enforcement and observations | `CARGO_NET_OFFLINE=true` |
| Workflow status | Core event projection | Provider lifecycle state |
| Evidence and report posture | Stable Workflow OS references | Raw stdout, stderr, or sandbox logs |

## 7. Exact Operation Boundary

The only operation considered by this plan is the existing closed profile:

```text
executable: <committed cargo executable>
arguments: clippy --workspace --all-targets -- -D warnings
shell: none
working directory: <immutable admitted repository root>
```

No caller-selected package, feature, target, environment, output path,
compiler wrapper, tool override, or additional argument is accepted. Any
future variation is a separate profile and review.

## 8. Complete Build-Time Input Discovery

Before invocation, Core must derive a closed input manifest from committed
repository and dependency facts. At minimum it must cover:

- workspace and member manifests;
- the exact lockfile and Cargo configuration;
- Rust source, build scripts, procedural-macro crates, and included files;
- target-specific configuration and clippy configuration;
- Cargo, rustc, rustdoc, cargo-clippy, clippy-driver, linker, archiver, and
  other resolved toolchain executables;
- target SDKs, sysroots, platform tools, dynamically loaded libraries, and
  configuration files materially used by the invocation;
- every selected dependency package and its build script or procedural macro;
- explicit environment values and locale/timezone posture; and
- immutable container, image, or sandbox base identity when applicable.

Discovery must be deterministic and fail closed when the implementation cannot
prove closure. A supplied fixture inventory is not evidence that real Cargo's
inventory is complete.

The first model slice should represent:

- input kind and role;
- canonical stable identity;
- content commitment;
- object-identity commitment when the platform supports it;
- origin and discovery method;
- required/optional posture; and
- bounded unavailable or unsupported reason code.

It must not store source contents, command output, environment secrets, or raw
provider payloads.

## 9. Root Object Identity And TOCTOU

Canonical-path equality remains useful validation but is not sufficient for a
live process. The future provider contract must define how repository,
toolchain, dependency-cache, build-output, and temporary roots remain bound
from admission through process termination and evidence collection.

Required posture:

- roots are canonical, disjoint, typed, and unlinked at admission;
- read-only roots are mounted or projected read-only by the containment layer;
- writable roots are newly created for the invocation or bound to an exact
  invocation identity;
- the provider prevents mount, link, rename, or path-redirection changes that
  would substitute a root during execution;
- object identity is retained through protected handles, mount identities,
  provider-native immutable workspace identities, or an equivalently reviewed
  mechanism;
- admission and terminal observations bind the same objects; and
- any unverifiable identity transition fails closed as possibly started.

The implementation plan review must choose supported platform semantics. It
must not imply portable inode identity where the selected platform or provider
cannot guarantee it.

## 10. Filesystem Containment

The provider must enforce a default-deny filesystem posture outside explicit
roots.

Required access classes:

- admitted repository input: read-only;
- admitted toolchain and SDK inputs: read-only and executable only where
  required;
- admitted dependency inputs: read-only;
- Cargo cache: explicit bounded write role only if required by the accepted
  implementation;
- build output: writable, invocation-scoped;
- temporary root: writable, invocation-scoped;
- all other host paths: unavailable.

The process must not see the user's home directory, operating-system credential
stores, SSH material, Git credentials, provider tokens, unrelated repositories,
or ambient temporary paths. Empty environment construction does not satisfy
this requirement.

## 11. Network And Egress Containment

The primary command requires no network access. The provider must enforce
default-deny egress outside the process, including descendants.

Required evidence:

- requested network policy identity;
- effective enforced network policy identity;
- enforcement mode, with audit-only and best-effort rejected;
- allowed endpoint count of zero;
- bounded denied-egress count and stable observation references; and
- policy revision stability from process start through terminal collection.

`CARGO_NET_OFFLINE=true` remains part of the deterministic command environment
but is defense in depth only. It is not accepted as egress enforcement.

## 12. Process-Tree Lifecycle

The provider must own the complete process tree, not only the initial Cargo
PID.

Required behavior:

- start the command in a provider-owned process group, job object, cgroup, or
  equivalent reviewed boundary;
- bind process start to the exact invocation identity;
- apply wall-clock, CPU, memory, process-count, file-descriptor, and disk
  budgets where supported;
- on timeout or cancellation, stop new process creation and terminate every
  descendant;
- wait for terminal process-tree confirmation before reporting completion;
- treat uncertain termination as reconciliation-required, never success; and
- prove no descendant remains able to mutate writable roots after evidence
  collection begins.

Exit code alone cannot establish terminal success.

## 13. Transient Write Posture

Pre/post commitments detect persistent mutation but cannot detect a protected
write that is restored before the final hash. A live boundary must therefore
prevent protected writes through read-only projection and, where supported,
collect independent mutation-denial or filesystem-event observations.

Required result posture:

- protected roots were enforced read-only;
- writable roots match the typed policy;
- unexpected write attempts are counted and referenced;
- policy degradation or observation gaps fail closed;
- writable-root outputs are collected only after process-tree termination; and
- retained artifacts are separately bounded and integrity committed.

Do not claim complete transient-write observation on platforms that provide
only final-state comparison.

## 14. Effective-Control Attestation

The live-execution result must distinguish requested policy from observed
effective policy.

The provider-neutral receipt should bind:

- immutable run, step, profile, request, attempt, and invocation identities;
- input-manifest commitment;
- containment-provider and configuration identities;
- immutable workspace or image identity;
- requested and effective policy digests and revisions;
- filesystem, process, network, resource, and cleanup enforcement states;
- start, terminal, timeout, cancellation, and cleanup observations;
- bounded denied-action and security-finding counts;
- stable log, artifact, and observation references;
- ambiguity/reconciliation posture; and
- redaction and sensitivity metadata.

The receipt must not contain raw logs, source contents, paths, environment
values, credentials, authorization material, or provider payloads. Unknown
wire values and missing required facts fail closed.

## 15. Evidence Independence

The implementation must document which component observes each fact and why
that observer is independent enough for the claim.

At least these distinctions are required:

- Core proves the accepted request and immutable identities;
- the containment provider proves effective enforcement and lifecycle;
- the runner reports command outcome;
- Core validates coherence and projects workflow status; and
- evidence/report layers cite validated receipts without recreating them.

One child process asserting that it obeyed policy is not independent evidence.
One provider field that repeats a requested digest is not proof that the policy
was loaded or remained active.

## 16. Failure And Ambiguity Semantics

Before process start, incomplete input closure or unavailable mandatory
containment returns a stable pre-start denial.

After the provider may have started the process, uncertainty must return a
possibly-started or reconciliation-required result. It must not retry blindly.
This applies to start timeout, lost transport, policy-observation mismatch,
uncertain termination, incomplete cleanup, and incomplete evidence collection.

Security-control failure must not be converted into a compiler diagnostic or a
misleading user-project validation error. Errors use stable codes and do not
include paths, source snippets, command output, environment values, provider
payloads, or secret-like content.

## 17. Workflow Semantics

The live boundary must remain downstream of:

- immutable run-bundle validation;
- current authority and capability resolution;
- policy and proportional-governance routing;
- any required approval with presentation proof;
- exact closed-profile resolution; and
- durable attempt creation.

The provider cannot append workflow events, decide policy, grant approval,
mutate snapshots, or decide terminal workflow status. Core remains responsible
for those transitions.

Report and artifact behavior remains deferred. A future composition phase may
cite the validated receipt and bounded local-check result after the execution
boundary is accepted.

## 18. Relationship To OpenShell

OpenShell is an architectural candidate because its intended boundary covers
process, filesystem, network, lifecycle, and security observations. This plan
does not select it.

Any future provider evaluation must compare actual, version-pinned OpenShell
interfaces against every mandatory fact in this plan. Requested policy labels,
separate best-effort CLI observations, or incomplete cleanup signals are not
enough. If OpenShell cannot expose a required enforcement or attestation fact,
the milestone remains blocked or another provider is evaluated.

Do not fork OpenShell for this profile. Fork reconsideration remains governed
by the criteria in the accepted OpenShell plan and requires a separate ADR.

## 19. Candidate Core Model

The first implementation should add only the smallest internal or
provider-neutral vocabulary necessary to validate a future request and receipt,
such as:

- `TrustedHostLiveExecutionSecurityContract`;
- `BuildTimeInputManifest`;
- `BuildTimeInputReference`;
- `RootObjectBinding`;
- `ExecutionContainmentRequirement`;
- `ProcessTreePolicy`;
- `FilesystemContainmentPolicy`;
- `NetworkContainmentPolicy`;
- `TransientWritePolicy`; and
- `LiveExecutionSecurityAttestation`.

Names should follow existing repository conventions after implementation-time
inspection. Types should remain private or provider-neutral unless a public
contract is explicitly justified. No model type authorizes execution by
itself.

## 20. Proposed Implementation Sequence

1. **Core security contract model only.** Add validated, redaction-safe
   provider-neutral requirements and attestation vocabulary. No runner.
2. **Focused model review.** Verify the model cannot confuse requested and
   effective controls or make incomplete evidence look authoritative.
3. **Deterministic input-discovery proof.** Implement closed inventory
   discovery against fixtures and the Workflow OS workspace without executing
   build-time code.
4. **Provider suitability evaluation.** Compare pinned OpenShell and any other
   candidate against the complete contract. Do not integrate a provider that
   cannot satisfy mandatory facts.
5. **Injected provider conformance proof.** Exercise lifecycle, ambiguity,
   policy drift, process-tree, filesystem, network, write, and cleanup outcomes
   through a fake provider.
6. **Focused maintainer/security review.** Accept or block the complete
   pre-live boundary.
7. **Separately approved opt-in live smoke.** Only after blockers close, run
   the exact command in a pinned reviewed containment environment.
8. **Live proof review.** Inspect effective controls, denied egress, terminal
   process tree, cleanup, artifacts, and non-leakage.
9. **Workflow composition and registration planning.** Only after accepted
   live proof; defaults remain unchanged.

Every step is its own governed phase. Failure at one step does not authorize
skipping to a later step.

## 21. Test Plan

Future model and conformance tests must cover:

- valid complete security contract and attestation;
- missing or duplicate input references;
- build-script, procedural-macro, tool, SDK, dependency, root, or image
  substitution;
- requested/effective policy mismatch;
- audit-only, best-effort, unavailable, or degraded mandatory controls;
- root object replacement before and during invocation;
- undeclared filesystem reads and protected-write attempts;
- transient protected-write attempt observation;
- direct and descendant egress attempts;
- child and grandchild survival after timeout/cancellation;
- resource-budget exhaustion;
- policy revision drift;
- ambiguous start, result, termination, evidence collection, and cleanup;
- restart reconciliation without duplicate execution;
- bounded payload-free results and stable errors;
- Debug, serialization, and deserialization non-leakage;
- no executor or registration exposure before acceptance; and
- unchanged existing Cargo format, docs-check, and injected clippy behavior.

A later live smoke must additionally prove the exact pinned provider, image,
toolchain, workspace revision, command, zero allowed egress, at least one
controlled denied-egress observation, terminal process tree, cleanup, and
reportable evidence references.

## 22. Validation Requirements

Model and implementation phases should run, as applicable:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused security-contract and provider-conformance tests;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

The current planning phase requires only the documentation and diff checks.
It does not run or authorize Cargo clippy through Workflow OS.

## 23. Acceptance Criteria For Any Live Execution

No real Cargo clippy process may be authorized until focused review confirms:

- complete build-time input closure or explicit fail-closed unavailability;
- stable root object binding through terminal collection;
- OS/provider-enforced filesystem and network containment;
- full process-tree lifecycle and resource controls;
- protected writes prevented and independently observed where required;
- requested and effective policies are separately bound and coherent;
- ambiguity and restart behavior cannot duplicate execution;
- bounded, redaction-safe, independently validated evidence exists;
- current executor denial and registration exclusion are deliberately replaced
  only by an explicit opt-in path; and
- no production, portability, or sandbox guarantee exceeds the proof.

## 24. Open Questions

- Which supported operating systems can satisfy the same mandatory boundary?
- What mechanism provides root object stability on each supported platform?
- Can real Cargo input discovery be complete without first executing metadata
  or build-time code, and what operation safely closes any gap?
- Which linker, SDK, sysroot, dynamic-library, and subprocess facts are needed
  for reproducible identity?
- Must the first live proof use an immutable materialized workspace rather than
  the developer checkout?
- Which provider can enforce zero egress and prove policy stability across the
  full process lifetime?
- What observation source proves all descendants are terminal?
- Which transient-write observations are portable enough to require?
- How should cleanup uncertainty affect local-check and workflow terminal
  semantics?
- What evidence must survive provider teardown, and in which existing store?

## 25. Final Recommendation

Proceed next to a focused maintainer and security review of this plan. If
accepted, implement only the provider-neutral security contract model.

Do not implement a standard runner, run Cargo, add a smoke, compose a workflow,
register the profile, expose public configuration, select OpenShell, add a
dependency, broaden provider access, or change release posture.

## 26. Governed Planning Record

- Workflow ID: `dg/d`.
- Run ID: `run-1792809418373175000-2`.
- Approval ID:
  `approval/run-1792809418373175000-2/planning-approved`.
- Approval-presentation ID: `presentation/2bc3145744d33d61`.
- Approval-presentation content hash:
  `2bc3145744d33d6191984707be44d829d336199930e76744781bff9588146cfb`.
- Approval outcome: granted through the proof-enforced path for the exact
  planning-only scope.
- Validation: documentation and diff checks are required at phase close.
- Out-of-kernel work: the delegated maintainer inspected repository documents,
  authored this plan and the roadmap update, and ran validation. The kernel
  governed scope and approval but did not edit files, run checks, or perform
  git and PR operations.
