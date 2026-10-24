# Trusted-Host Cargo Clippy Live-Execution Security Plan Review

## 1. Executive Verdict

**Plan accepted with mandatory implementation constraints; proceed to the
provider-neutral live-execution security contract core model only.**

The plan correctly rejects a standard host-process runner and treats trusted
host as an operator posture rather than a containment claim. It identifies the
material gaps between the accepted injected-runner profile and a real Cargo
process: incomplete build-time input closure, path-based rather than durable
object identity, absent process-tree control, absent filesystem and egress
containment, unobservable transient writes, and missing independent
effective-control evidence.

The first implementation is acceptable only as validated, redaction-safe,
non-authorizing model vocabulary. It must not create a runner, executable
resolver, provider selection, live smoke, workflow composition, registration,
or public configuration surface.

## 2. Scope Verification

The plan stays within planning scope. It does not authorize:

- a standard runner or real Cargo process;
- live smoke testing;
- workflow composition or registration;
- public CLI, configuration, schema, SDK, onboarding, or example behavior;
- OpenShell selection, installation, integration, or fork;
- provider credentials or external access;
- source or provider writes;
- dependency additions; or
- release-posture changes.

The proposed sequence keeps every later capability behind a separate governed
phase and focused review.

## 3. Threat-Model Assessment

The threat model is appropriate for Cargo clippy. It does not reduce the
operation to compiler diagnostics. It accounts for repository and dependency
build scripts, procedural macros, wrappers, linkers, SDK tools, dynamic
libraries, descendants, direct network access, protected-state mutation,
resource exhaustion, and forged or incomplete provider observations.

The plan also names its trust anchors: Workflow OS Core, the immutable run
bundle, and a future explicitly accepted containment provider. It does not
trust command output, path names, ambient environment, or a child process's
claim that it followed policy.

Mandatory implementation constraint: the model must represent unavailable,
degraded, unsupported, and uncertain control states explicitly and fail closed
for every mandatory control. A boolean `enforced` field is not rich enough.

## 4. Build-Time Input Closure Assessment

The required inventory is materially complete at the planning level. It covers
workspace and member manifests, lock and Cargo configuration, source and
included files, build scripts, procedural macros, clippy configuration,
toolchain executables, linkers, SDKs, sysroots, dynamic libraries, dependency
packages, environment posture, and base image or workspace identity.

The plan correctly distinguishes deterministic fixture inventory from proof
that real Cargo inputs are closed.

Mandatory implementation constraints:

- The first model must not claim that the inventory is complete merely because
  all supplied entries validate.
- Completeness posture and discovery method must be explicit facts.
- Unsupported or unknown material inputs must block later invocation.
- Content or path values must not be copied into errors, Debug, or serialized
  audit summaries.
- Input discovery remains a later implementation phase; the model phase must
  not invoke Cargo metadata, rustc, build scripts, or provider tooling.

## 5. Root Identity And TOCTOU Assessment

The plan correctly retains canonical-path validation as a useful admission
check while rejecting it as a live containment mechanism. Read-only
projection, invocation-scoped writable roots, protected handles or
provider-native immutable workspace identities, and admission-to-terminal
coherence are the right requirements.

The plan also avoids falsely requiring portable inode semantics. A future
implementation review must evaluate the selected platform and provider rather
than hiding portability gaps behind one identifier field.

Mandatory implementation constraint: the first model should describe root
binding requirements and observed binding facts separately. Constructing a
valid requirement must not imply that a root was actually pinned or protected.

## 6. Filesystem And Transient-Write Assessment

The access classes are appropriately closed: repository, toolchain, SDK, and
dependency inputs are read-only; cache, build, and temporary roots are typed
and invocation-scoped; every other host path is unavailable.

The plan correctly states that an empty environment cannot hide host files and
that pre/post commitments cannot detect a restored transient write. Requiring
read-only projection plus independent denial or mutation observations where
supported is the right security posture.

Mandatory implementation constraint: the model must not collapse `prevented`,
`observed_denied`, `not_observable`, and `not_supported` into one success state.
Any live boundary lacking a required prevention or observation fact must fail
closed.

## 7. Network Assessment

The zero-egress posture is correct. Cargo offline mode remains useful defense
in depth but is not confused with operating-system enforcement. Requested and
effective policy identity, enforce mode, zero allowed endpoints, bounded
denied observations, and revision stability are all appropriate requirements.

Mandatory implementation constraint: requiring at least one denied-egress
observation belongs to a deliberate live proof probe, not to every successful
production invocation. The core contract should require enforceable zero
egress and support denied-observation evidence without making an artificial
network attempt part of the normal clippy command.

## 8. Process-Tree And Resource Assessment

The plan correctly requires ownership of the complete process tree, prevention
of new descendants during shutdown, terminal confirmation, and explicit
reconciliation when termination is uncertain. Parent exit code alone is
properly rejected as success evidence.

The resource categories are appropriate. The future provider evaluation must
state which budgets are hard-enforced on each supported platform. Unsupported
mandatory limits cannot silently become advisory.

Mandatory implementation constraint: terminal process-tree observation and
cleanup are distinct facts. A process tree may be terminal while writable
state or a sandbox still remains, and the model must preserve that difference.

## 9. Effective-Control Attestation Assessment

The proposed receipt content is well bounded and correctly separates:

- Core request identity;
- provider and configuration identity;
- requested and effective policy;
- process and cleanup observations;
- security counts and stable references; and
- ambiguity and reconciliation posture.

The evidence-independence section is especially important. The plan rejects a
provider echo of the request as enforcement proof and keeps workflow status in
Core.

Mandatory implementation constraints:

- Requirement types and observation types must be separate.
- A receipt must bind an exact request and durable attempt.
- Unknown enum or wire values must fail closed.
- Stable references must be bounded and validated.
- No raw log, command output, source content, path, environment value,
  credential, or provider payload may be stored.
- The model must not introduce cryptographic or hardware-attestation claims.

## 10. Failure And Ambiguity Assessment

The pre-start versus possibly-started distinction is correct. Incomplete input
closure or unavailable containment denies before process start. Lost transport,
policy mismatch, uncertain termination, incomplete evidence, and cleanup
uncertainty after possible start require reconciliation and prohibit blind
retry.

The plan also correctly prevents security-control failures from becoming
misleading project diagnostics.

Mandatory implementation constraint: the first model should validate the
state combinations that are impossible or unsafe, such as terminal success
with uncertain process termination, required control degradation, or unknown
cleanup posture.

## 11. Workflow And Authority Assessment

The proposed runtime order preserves Workflow OS as the governing authority:
immutable run validation, current authority, policy and proportional routing,
approval when required, closed-profile resolution, durable attempt creation,
provider invocation, and Core-owned terminal projection.

The provider cannot grant authority, append events, mutate snapshots, or decide
workflow status. This is consistent with the project's architecture.

No WorkReport or artifact integration should be added in the first model
phase. Existing evidence vocabulary may be referenced only where required to
define a stable future boundary.

## 12. OpenShell Assessment

The plan's OpenShell posture is correct. OpenShell is a candidate containment
provider, not the architecture itself and not a selected dependency. The
future suitability evaluation must compare pinned real interfaces with every
mandatory contract fact.

The plan does not erase existing OpenShell work. It appropriately refuses to
treat requested labels or separately sampled best-effort CLI state as atomic
proof of effective controls.

No fork is justified by this plan.

## 13. Model Assessment

The candidate concepts are justified, but the implementation should choose the
smallest set after inspecting existing hosted-execution and OpenShell models.
It should extend or compose existing provider-neutral vocabulary where that
preserves one source of truth.

Avoid a parallel receipt hierarchy. The first model should add only the facts
that current contracts cannot express, with constructors enforcing bounds and
coherence.

The implementation report must map each new type to a specific accepted
requirement and explain why an existing type was insufficient.

## 14. Sequence Assessment

The staged sequence is accepted:

1. provider-neutral security contract model;
2. focused model review;
3. deterministic input-discovery proof;
4. provider suitability evaluation;
5. injected provider conformance proof;
6. focused pre-live security review;
7. separately approved live smoke;
8. live-proof review; and
9. later composition and registration planning.

This sequence prevents the project from selecting a sandbox first and then
weakening its governance requirements to match the available API.

## 15. Test-Plan Assessment

The proposed tests cover the important substitution, degradation, ambiguity,
descendant, egress, write, resource, restart, non-leakage, and non-exposure
cases.

The model-only implementation must at minimum test:

- all required control states and bounds;
- requested versus observed control separation;
- valid and invalid state combinations;
- exact request and attempt binding;
- incomplete input closure;
- unavailable/degraded mandatory controls;
- ambiguity and reconciliation states;
- stable-reference bounds;
- serde failure-closed behavior;
- Debug and error non-leakage; and
- proof that constructing the model does not expose an executable path.

Process, filesystem, network, and descendant behavior tests remain later
provider-conformance work and must not be simulated as model proof.

## 16. Blockers

No blocker prevents the provider-neutral model-only phase.

The following remain blockers to any real process, smoke, composition, or
registration:

- complete real-Cargo build-time input discovery and attestation;
- accepted root object and TOCTOU semantics for supported platforms;
- OS/provider-enforced filesystem containment;
- OS/provider-enforced zero-egress policy;
- full process-tree lifecycle and resource enforcement;
- accepted transient-write prevention and observation posture;
- independently observed effective-control evidence;
- restart-safe ambiguity and cleanup reconciliation; and
- focused security acceptance of the complete provider boundary.

## 17. Non-Blocking Follow-Ups

- Clarify in the model plan that a denied-egress probe is required for the
  live proof, not every normal invocation.
- Prefer composition with existing hosted/OpenShell receipt vocabulary over a
  parallel type family.
- Keep platform support explicit; do not make cross-platform claims from one
  local proof.
- Retain the distinction between terminal process tree and completed cleanup.

## 18. Validation

- `npm run check:docs`: required at review phase close.
- `git diff --check`: required at review phase close.
- Runtime tests are not required because this review changes documentation
  only and does not authorize implementation behavior.

## 19. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792810246769925000-2`.
- Approval ID:
  `approval/run-1792810246769925000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/c9feda86d966ed90`.
- Approval-presentation content hash:
  `c9feda86d966ed90b5a2684760bd7045a0141fa8614ad0703308cfc5a7dd8697`.
- Approval outcome: granted under standing delegated-maintainer authority
  through the proof-enforced path for the exact review-only scope.
- Validation: documentation and diff checks are required at phase close.
- Out-of-kernel work: the delegated maintainer inspected the plan and related
  security/runtime boundaries, authored this review and roadmap update, and
  ran validation. The kernel governed scope and approval but did not edit
  files, run checks, or perform git and PR operations.

## 20. Recommended Next Phase

Plan the provider-neutral trusted-host live-execution security contract core
model, then review that implementation plan before code.

The next planning phase must remain model-only. It must not add a runner, live
Cargo, smoke, input discovery, provider selection, OpenShell integration,
workflow composition, registration, public configuration, dependencies,
writes, or release changes.
