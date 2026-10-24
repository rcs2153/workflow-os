# Trusted-Host Local Cargo Observation And Execution Composition Plan Review

## 1. Executive Verdict

**Plan accepted with mandatory implementation constraints; proceed to the
private injected fail-closed composition contract only.**

The plan resolves the central security question correctly: placing observation
and execution in one function does not create custody. The accepted native
observer returns useful preflight evidence, but it does not retain repository
or executable objects, enforce filesystem or network policy, own a Cargo
process tree, or prove cleanup. Its current facts therefore cannot become
`Observed` and cannot unlock Cargo execution.

The first implementation may bind the complete attempt and prove deterministic
pre-start denial. It must not add a standard runner, live Cargo process, live
smoke, provider adapter, or production execution path.

## 2. Scope Verification

The plan remains planning-only. It does not authorize:

- admission or executor behavior changes;
- Cargo clippy execution or a standard host-process runner;
- promotion of current observer facts or roots to proven posture;
- OpenShell selection, wiring, installation, or fork;
- persistence, events, audit projection, report generation, or schemas;
- public CLI, SDK, configuration, onboarding, or examples;
- additional profiles or operating systems;
- provider writes, credentials, network access, or external SideEffects;
- hosted or distributed behavior; or
- release-posture changes.

## 3. Architectural Boundary Assessment

`workflow-local-host` is the correct owner for the private coordinator because
it owns host observation and would later own a local execution-provider
adapter. Core remains the owner of immutable bindings, observation contracts,
live-security assessment, local-check attestation, and report vocabulary.

The review found and corrected one ownership ambiguity. The live-security
assessment function is currently crate-private, so `workflow-local-host`
cannot call it. The plan now permits only a narrow
`trusted-host-application-spi`-gated pure Core assessment entry point. It does
not permit the local host to duplicate the security matrix or reinterpret
postures.

## 4. Same-Call Custody Assessment

The plan correctly separates:

- observation interval drift detection;
- pre-start identity revalidation;
- admission-to-terminal object continuity;
- provider-enforced controls;
- process-tree terminality; and
- cleanup completion.

The first two are not substitutes for the latter four. Repeating a hash or
path check next to process creation remains preflight evidence. Only an
accepted continuous owner may establish the required custody.

Mandatory implementation constraint: the composition result must never use
the term custody, continuity, or observed enforcement for the current native
source. Its accepted terminal state is pre-start denial.

## 5. Immutable Binding Assessment

The plan binds the correct identity layers: stored immutable run bundle,
workflow/run/step/skill, command contract, registered handler, Cargo operation
and security requirements, static discovery, repository tree, ordering
authority, observer producer and selection, roots, executables, attempt, and
invocation.

The complete composition fingerprint must be Core-derived or derived inside
the private reviewed boundary. A recomputable caller fingerprint is integrity
metadata, not provenance.

Mandatory implementation constraints:

- use one existing durable attempt/invocation identity rather than creating a
  parallel lifecycle;
- reject any cross-run, cross-step, cross-request, or cross-attempt binding;
- validate the complete supplied batch before any observer or runner entry;
  and
- keep handler posture honest as `RegisteredUnattested` until a separate
  assurance source exists.

## 6. Observer And Evidence Assessment

The plan binds the exact observation request, producer, version, selection,
result, completion revision, root evidence, nine obligation facts, and
in-memory evidence bundle. Changed inputs require reobservation, and the result
is not reusable authority.

The current posture ceilings are preserved. Toolchain, target, sysroot, native
tools, registry sources, and repository root remain uncertain; linker and
environment remain declared-only; dynamic libraries and runtime base remain
unsupported.

Mandatory implementation constraint: missing provider observations must be an
explicit composition-level pre-start denial. The implementation must not
construct a synthetic execution-provider observation from host-observer facts.

## 7. Security Assessment

The strict live-security model remains controlling. Requirements and observed
effective controls are distinct. The local observer cannot prove default-deny
filesystem, host-home exclusion, credential-store exclusion, zero egress,
process-tree ownership, resource limits, transient-write posture, or cleanup.

The plan correctly routes any future supplied observations through Core-owned
assessment and preserves `DeniedBeforeStart` versus
`ReconciliationRequired`.

Mandatory implementation constraints:

- no boolean or detached fingerprint may claim security satisfaction;
- unavailable or absent mandatory facts deny before process entry;
- possibly-started ambiguity cannot be retried silently; and
- a satisfied security assessment grants no workflow, policy, approval, or
  capability authority by itself.

## 8. Environment, Root, And Executable Assessment

The plan correctly binds five typed roots and the exact Cargo, cargo-clippy,
clippy-driver, rustc, and rustdoc executable roles. It also correctly notes
that observer and execution environments differ by purpose and must have
separate commitments.

Mandatory implementation constraint: private host paths may be used by the
local owner but must not cross into Core evidence, errors, Debug output,
serialization, reports, or stable references.

## 9. Invocation, Result, And Attestation Assessment

The plan preserves the injected Cargo clippy handler as proof code rather than
turning it into a production runner. Future provider-backed execution must own
the complete process tree and effective controls before the existing bounded
result conversion can be reused.

Attestation must bind the exact immutable execution binding, observation,
security assessment, provider invocation, terminal outcome, cleanup posture,
and bounded result. A WorkReport may cite stable references later, but cannot
claim a check ran after pre-start denial.

No report or artifact integration belongs in the first implementation.

## 10. Failure And Privacy Assessment

The failure matrix is sound. Invalid identity and observer failures happen
before process entry. Insufficient evidence produces a proven pre-start
denial. Uncertainty after possible start requires reconciliation. Terminality
and cleanup remain distinct.

The privacy boundary is appropriately payload-free. Raw source, manifests,
lockfiles, dependency material, paths, environment values, command output,
provider payloads, credentials, and free-form provider errors remain excluded.

Mandatory implementation constraint: errors must use stable codes and static
messages and must not convert security failures into compiler or project
diagnostics.

## 11. First Implementation Assessment

The proposed first slice is appropriately narrow:

1. validate and fingerprint all immutable inputs;
2. invoke the observer through injected sources;
3. validate result and evidence resolution;
4. use Core-owned security assessment for any injected provider observation;
5. deny explicitly when provider observations are absent or insufficient; and
6. prove with a runner spy that no process starts.

This is useful despite not executing Cargo. It closes the semantic gap between
preflight observation and execution eligibility and creates a testable boundary
where a future provider must supply real proof rather than being trusted by
convention.

## 12. Test-Plan Assessment

The plan covers binding substitution, stale revisions, cross-request evidence,
posture ceilings, environment separation, security degradation, no-start
proof, ambiguity coherence, privacy, and non-exposure.

The implementation should additionally prove:

- absence of provider observations does not construct provider identity or
  cleanup references;
- Core's assessment SPI is the only path to a satisfied assessment;
- a later invalid input prevents all earlier observer and runner activity;
- current native observation can never select the future-provider branch; and
- the first slice exposes no production constructor or registration path.

## 13. Sequence Assessment

The sequence is accepted:

1. private injected composition and pre-start denial;
2. focused implementation review;
3. provider suitability re-evaluation;
4. injected provider conformance proof;
5. focused pre-live security review;
6. separately approved live proof; and
7. only then workflow composition or registration.

This preserves the project's safety ordering without mistaking model progress
for execution readiness.

## 14. Blockers

None for the private injected fail-closed composition contract.

The following remain blockers to any real Cargo process:

- accepted continuous root and executable custody;
- complete real-Cargo input closure;
- enforced filesystem and zero-egress policy;
- complete process-tree and resource ownership;
- transient-write prevention and observation;
- independently observed effective controls;
- terminal cleanup and restart-safe reconciliation; and
- focused acceptance of one provider implementation.

## 15. Non-Blocking Follow-Ups

- Reuse one existing attempt/invocation identity rather than adding another
  lifecycle model.
- Keep the Core assessment SPI unpublished and feature-gated to the local-host
  application boundary.
- Re-evaluate OpenShell only from pinned authoritative interfaces.
- Decide the report citation for pre-start denial during a later report phase,
  not in the first implementation.

## 16. Validation

- `npm run check:docs`: required at phase close.
- `git diff --check`: required at phase close.
- Runtime checks are not required because this review changes documentation
  only and does not authorize execution.

## 17. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792846494281320000-2`.
- Approval ID:
  `approval/run-1792846494281320000-2/review-scope-approved`.
- Approval presentation ID: `presentation/28ba9b29a65c6357`.
- Approval presentation content hash:
  `28ba9b29a65c6357b74a66634aa7fefc21e20ca1be0a036cef3b9198b171f94f`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval-presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected the plan and related
  source, authored this review and the narrow plan correction, and ran
  validation. The kernel governed scope and approval but did not inspect
  files, edit documents, run checks, or perform git and PR operations.

## 18. Recommended Next Phase

Implement the private injected local Cargo observation/execution composition
contract and fail-closed pre-start assessment only.

Do not add a standard runner, run Cargo, add a live smoke, select or wire
OpenShell, change admission, persist outcomes, emit events, add schemas or CLI
behavior, enable writes, broaden authority, or change release posture.
