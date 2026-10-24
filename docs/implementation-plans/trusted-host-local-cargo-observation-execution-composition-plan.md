# Trusted-Host Local Cargo Observation And Execution Composition Plan

Status: observation-binding blocker fixed; focused blocker-fix review required.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Trusted-Host Local Cargo Input Observer Provider Plan](trusted-host-local-cargo-input-observer-provider-plan.md)
- [Trusted-Host Local Cargo Input Observer Source Review](../concepts/TRUSTED_HOST_LOCAL_CARGO_INPUT_OBSERVER_SOURCE_REVIEW.md)
- [Trusted-Host Cargo Clippy Live-Execution Security Boundary Plan](trusted-host-cargo-clippy-live-execution-security-plan.md)
- [Trusted-Host Cargo Clippy Live-Execution Security Plan Review](../concepts/TRUSTED_HOST_CARGO_CLIPPY_LIVE_EXECUTION_SECURITY_PLAN_REVIEW.md)
- [Immutable Local-Check Execution Binding Core Model Review](../concepts/IMMUTABLE_LOCAL_CHECK_EXECUTION_BINDING_CORE_MODEL_REVIEW.md)

## 1. Executive Summary

The accepted fixed local Cargo observer should compose with the existing Cargo
clippy local-check boundary through one private, fail-closed same-call
coordinator. That coordinator must bind the immutable run, command, selected
handler, observer request and result, local roots, executable set, sanitized
environment, live-security requirements and observations, invocation, bounded
result, attestation, and report references.

Same-call adjacency is not custody. The current observer terminates after its
fixed probes and returns preflight facts whose integrity and continuity remain
`Uncertain`, `DeclaredOnly`, or `Unsupported`. The existing Cargo clippy
handler is injected-runner-only and is deliberately denied by ordinary
executor and registration paths. Therefore the first composition
implementation must not run Cargo. It should prove the complete private
binding and return a stable pre-start denial while mandatory live-security
facts remain unsatisfied.

A later execution provider may satisfy the same composition contract only if
it retains accepted authority and object continuity from admission through
terminal process-tree observation and cleanup. OpenShell remains a candidate,
not a selected provider.

## 2. Decision

Plan one private `workflow-local-host` composition boundary for the exact
`CargoClippyWorkspace` profile. The boundary owns the full attempt from
observation start through either:

- a proven pre-start denial; or
- future provider-owned execution, terminal observation, attestation, and
  bounded result construction.

The current native observer is not a continuous execution owner. Its result
cannot be upgraded to `Observed`, and it cannot unlock the injected Cargo
clippy runner. A successful preflight result followed by missing or
insufficient live-security observations returns `DeniedBeforeStart` without
entering the process runner.

## 3. Goals

- Define one source of truth for same-call observation and execution identity.
- Bind the exact stored immutable run bundle and local-check execution binding.
- Bind one canonical Cargo clippy command and one private handler selection.
- Bind the observer request, producer, selection, result, and evidence bundle.
- Bind repository, toolchain, dependency, build-output, and temporary roots.
- Bind the exact five executable roles and sanitized environment commitment.
- Require accepted live-security requirements and observations before process
  entry.
- Preserve pre-start denial, possibly-started ambiguity, terminal outcome, and
  cleanup as distinct states.
- Bind accepted local-check attestation and terminal report citations without
  copying payloads.
- Keep errors, Debug output, evidence, and reports bounded and non-leaking.

## 4. Strict Non-Goals

This plan does not implement or authorize:

- admission changes or Cargo execution composition;
- a standard host-process Cargo runner or live Cargo invocation;
- promotion of any current observer fact to `Observed`;
- OpenShell selection, installation, integration, or fork;
- provider persistence, workflow events, audit projection, or schemas;
- public CLI, SDK, configuration, onboarding, or example behavior;
- automatic local-check execution or default registration;
- additional commands, profiles, platforms, or caller-selected probes;
- provider writes, external SideEffects, credentials, or network access;
- hosted or distributed execution, nested harnesses, or enterprise controls;
- release-posture changes.

## 5. Exact First Operation

The only operation is the existing closed profile:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

It uses the existing `CargoClippyWorkspace` command kind and canonical command
contract. No package, feature, target, argument, environment variable,
executable, root, timeout, output bound, or network option is caller-tunable.
Any variation is a different reviewed profile.

## 6. Composition Owner And Dependency Direction

`workflow-core` remains the source of truth for:

- immutable run and local-check execution bindings;
- trusted-host observation request and result models;
- live-execution security requirements, observations, and assessment;
- local-check result and attestation validation; and
- report citation vocabulary.

`workflow-local-host` should own the private coordinator because it already
owns the native observer and any future local execution-provider adapter. Core
must not depend on `workflow-local-host`, inspect host paths, or start
processes.

Core must remain the only owner of security-requirement and observation
assessment. The current assessment function is crate-private, so the first
implementation may add one narrow `trusted-host-application-spi`-gated Core
entry point for the unpublished local-host caller. The local-host coordinator
must not duplicate the matrix, reinterpret posture enums, or infer
satisfaction from fingerprints. The SPI is a pure assessment boundary and
does not expose process execution or workflow authority.

The coordinator receives no workflow mutation, policy-decision, approval,
event-append, state-store, or report-store authority. Its output is evidence
for a separately reviewed caller, never authority to advance a workflow.

## 7. Candidate Private Boundary

Candidate internal concepts, with final names chosen at implementation time:

- `LocalCargoObservationExecutionRequest`;
- `LocalCargoObservationExecutionCoordinator`;
- `LocalCargoObservationExecutionAttempt`;
- `LocalCargoObservationExecutionOutcome`;
- `LocalCargoPreStartDenial`; and
- `LocalCargoExecutionEvidenceBundle`.

The request should borrow validated Core values and private local-host
definitions. It must not accept raw command text, arbitrary environment,
untyped paths, caller-created observation facts, detached fingerprints, or a
boolean claiming security satisfaction.

## 8. Immutable Identity Binding

Before host interaction, the coordinator must bind:

- immutable run-bundle identity, version, root, workflow, run, and step;
- resolved skill identity and version;
- canonical command ID, kind, contract, and policy fingerprint;
- registered handler selection and its honest assurance posture;
- Cargo operation-contract and security-requirements commitments;
- static-discovery and repository-tree commitments;
- ordering-authority commitment;
- observer selection, producer identity, version, and configuration;
- exact root-role and executable-role commitments; and
- one durable attempt and invocation identity supplied by the accepted caller.

The complete composition fingerprint must be domain-separated and derived
inside the private boundary. Caller-provided detached fingerprints are not
proof of provenance.

## 9. Observation Timing And Freshness

Observation runs after immutable binding and before any execution-provider
entry. Its start and completion revisions use the request's exact ordering
authority. Wall-clock time is not ordering authority.

The result and evidence bundle must resolve exactly the request, producer,
root, all nine obligation kinds, and completion revision. Changed run,
command, policy, repository, executable, dependency, environment, root,
producer, or security input requires a new observation and a new attempt.

Observation freshness expires at any detected substitution or when the
coordinator relinquishes the same-call attempt. A result is evidence, not
reusable permission.

## 10. Preflight Evidence Versus Same-Call Custody

The native observer's pre/post pathname commitments detect ordinary drift
during its probe interval. They do not retain repository or executable
objects, enforce read-only projection, own a future Cargo process tree, deny
egress, observe dynamic libraries, or prove cleanup.

Accordingly:

- no current fact may be promoted to `Observed`;
- no current root may claim proven admission-to-terminal continuity;
- successful observation cannot satisfy the strict live-execution assessment;
- repeating hashes immediately before process start is still preflight, not
  custody; and
- the injected Cargo clippy handler must remain unreachable in the first
  composition implementation.

Same-call custody requires one accepted provider or process owner to retain
the relevant identities and controls continuously from admission through
terminal collection. That capability does not exist today.

## 11. Admission And Security Assessment

The coordinator must construct or receive the exact existing
`TrustedHostLiveExecutionSecurityRequirements` bound to the operation and
attempt. It may accept live-security observations only from a separately
reviewed provider-owned source with Core-valid provenance.

Before process entry it asks Core to evaluate the existing closed assessment
through the narrow trusted-host application SPI. Any missing, unsupported,
uncertain, degraded, mismatched, stale, or substituted mandatory fact yields a
stable pre-start denial. Missing provider observations are represented as an
explicit composition-level pre-start denial; the coordinator must not invent
an execution-provider observation merely to call the assessment function. It
must not reinterpret an observer posture, requested control, or provider echo
as effective control.

The assessment grants no workflow authority. A satisfied future assessment is
only one prerequisite for the caller's current policy, approval, capability,
and runtime checks.

## 12. Environment, Root, And Executable Coherence

The observer and future executor must bind one exact:

- canonical repository input root;
- toolchain root;
- dependency/cache root;
- invocation-scoped build-output root;
- invocation-scoped temporary root;
- Cargo, cargo-clippy, clippy-driver, rustc, and rustdoc executable set; and
- closed environment commitment.

The observer's current probe environment contains only explicit `CARGO_HOME`.
The Cargo clippy execution environment additionally needs the reviewed closed
set for offline mode, target output, tool paths, toolchain root, and temporary
root. The composition fingerprint must bind both purpose-specific environment
definitions and reject accidental equality assumptions between them.

No absolute path or environment value crosses into Core evidence or reports.

## 13. Invocation And Result Boundary

A future provider-backed invocation may start only after all pre-start
bindings and security requirements are satisfied. The provider must own the
complete process tree and return the accepted effective-control, terminal,
cleanup, and ambiguity observations.

The existing `CargoClippyWorkspaceLocalHandler` remains an injected proof and
must not be converted into the production runner. Its canonical command,
bounded process-output conversion, protected-input checks, and result model
may be reused only behind an accepted provider adapter.

The bounded `LocalCheckResult` must bind the exact attempt, invocation,
command, security assessment, and provider terminal outcome. Raw stdout,
stderr, logs, paths, or provider payloads are not result fields.

## 14. Attestation And Report Binding

After a future terminal provider outcome, Core may verify the existing
independent local-check attestation against:

- the immutable local-check execution binding;
- exact observation result and evidence-bundle commitment;
- live-security requirement, observation, and assessment commitments;
- provider identity and invocation identity;
- bounded local-check result reference; and
- terminal and cleanup posture.

WorkReport integration should cite stable existing identifiers and evidence
references. It must not recreate observation facts, copy command output, or
claim execution when the outcome was a pre-start denial.

The first implementation should return the complete private composition
outcome in memory. Persistence, event projection, audit records, and report
artifact generation remain later phases.

## 15. Failure And Ambiguity Matrix

| Boundary | Required outcome |
| --- | --- |
| Invalid immutable or command binding | Stable validation failure; no host interaction |
| Observer request, producer, selection, root, executable, parser, or evidence mismatch | Stable pre-start failure; no Cargo process |
| Complete observer result with mandatory facts below required live posture | `DeniedBeforeStart`; no Cargo process |
| Missing or untrusted live-security observations | `DeniedBeforeStart`; no Cargo process |
| Provider may have started but start acknowledgement is uncertain | Reconciliation required; no retry |
| Process-tree termination uncertain | Reconciliation required; never success |
| Effective policy drift or degraded mandatory control | Reconciliation required when possibly started |
| Cleanup or evidence collection incomplete | Reconciliation required; no success attestation |
| Terminal provider outcome with valid bounded result | Eligible for Core attestation verification, not automatic workflow advancement |

No failure may be converted into a compiler diagnostic, approval denial, or
misleading user-project validation error.

## 16. Privacy And Redaction

The composition boundary may expose only typed identities, commitments,
closed postures and reasons, bounded counts, and stable references. It must not
store or emit:

- source, manifest, lockfile, or dependency contents;
- absolute paths or usernames;
- environment names or values beyond existing closed public vocabulary;
- stdout, stderr, Cargo metadata, provider logs, or payloads;
- endpoints, credentials, tokens, authorization headers, or private keys; or
- free-form provider errors.

Debug, serialization, deserialization, and stable errors must remain
non-leaking.

## 17. First Implementation Scope

If this plan is accepted, implement only a private, injected composition
contract that:

1. validates and fingerprints all immutable inputs before host interaction;
2. invokes the accepted observer through an injected source;
3. validates the result and exact evidence bundle;
4. routes any supplied injected live-security observations through the
   Core-owned assessment SPI, while treating absent provider observations as
   an explicit pre-start denial;
5. returns `DeniedBeforeStart` without entering the Cargo runner when the
   current observer evidence is used; and
6. proves through a runner spy that no process can start on every incomplete,
   substituted, stale, or unsupported path.

The implementation must not include a standard runner, real Cargo process,
provider adapter, live smoke, persistence, events, or report generation.

## 18. Test Plan

Future focused tests must cover:

- exact immutable run, workflow, run, step, skill, command, and policy binding;
- exact observer request, producer, selection, result, and evidence binding;
- exact five root roles and five executable roles;
- distinct observer and execution environment commitments;
- canonical composition fingerprint and deterministic unchanged replay;
- substitution of every identity and commitment;
- stale ordering revisions and cross-request evidence;
- current observer result always denying before runner entry;
- no promotion to `Observed` or proven root continuity;
- mandatory live-security fact absence, degradation, or mismatch;
- runner spy proving zero starts on every preflight denial;
- possibly-started and reconciliation-required state coherence through
  injected future-provider fixtures only;
- bounded result and attestation binding fixtures without host execution;
- stable non-leaking errors and redacted Debug;
- serialization non-leakage where any Core value is serialized;
- no persistence, event, report, workflow mutation, or public registration;
- ordinary tests remain process-free; and
- all existing observer, local-check, attestation, immutable-run, executor,
  provider, persistence, and workspace tests remain unchanged.

## 19. Proposed Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Private injected composition contract and fail-closed pre-start assessment.
3. Focused implementation review.
4. Provider suitability re-evaluation against the accepted live-security
   matrix, including OpenShell only if authoritative interfaces changed.
5. Injected provider conformance proof for one selected provider boundary.
6. Focused pre-live security review.
7. Separately approved opt-in live Cargo clippy proof.
8. Live-proof review before any workflow composition or registration.

Do not combine these phases.

## 20. Open Questions

- Which provider can retain repository, executable, process-tree, policy, and
  cleanup custody for the full attempt?
- Can the provider expose complete effective-control observations without raw
  payloads or request echoes?
- Does the first accepted provider require an immutable materialized workspace
  rather than the developer checkout?
- Which existing invocation identity should bind the provider operation and
  local-check attestation without introducing a second attempt model?
- Which current report citation target best represents a pre-start security
  denial without claiming a check ran?
- What durable reconciliation surface is required before a live provider may
  cross the possible-start boundary?

## 21. Final Recommendation

Proceed to focused maintainer and security review of this plan.

If accepted, implement only the private injected composition contract and its
fail-closed pre-start assessment. Do not run Cargo, add a standard runner,
select OpenShell, wire admission, persist outcomes, emit events, add schemas or
CLI behavior, enable writes, broaden authority, or change release posture.

## 22. Governed Planning Record

- Workflow ID: `dg/d`.
- Run ID: `run-1792846202820998000-2`.
- Approval ID:
  `approval/run-1792846202820998000-2/planning-approved`.
- Approval presentation ID: `presentation/3de3c756a5ef2158`.
- Approval presentation content hash:
  `3de3c756a5ef2158a4829a995cd83ca60706493072543bc5937bf0b85728cdf2`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact planning-only scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval-presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected accepted source,
  security, immutable-binding, local-check, and attestation boundaries;
  authored this plan and roadmap update; and ran documentation validation. The
  kernel governed scope and approval but did not inspect files, edit documents,
  run validation, or perform git and PR operations.

## 23. Implementation Status

The first implementation is complete as a private, in-memory, process-free
composition boundary. It binds the immutable local-check execution contract,
observer request and evidence, live-security requirements, and attempt before
routing security posture through the Core-owned assessment SPI.

Missing execution-provider observations produce the required
`DeniedBeforeStart` result. The coordinator has no runner, process, provider,
state-store, event, report-store, or workflow-mutation capability in its input
surface, so the current slice cannot enter Cargo even if a caller supplies a
different assessment fixture. Focused maintainer and security review is next.

Focused review found that the first implementation nevertheless accepted an
optional provider-observation value without committing the exact value into
the outcome identity. The blocker fix removes that input from the pre-start-
only local coordinator. It now always passes explicit observation absence to
the Core SPI. The future SPI may still assess provider-owned observations in a
separately reviewed provider contract, but this slice cannot accept or collapse
uncommitted provider claims. Focused blocker-fix review is next.
