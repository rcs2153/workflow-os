# Trusted-Host Local Cargo Input Observer Provider Plan

Status: planning complete; focused maintainer/security review required before
implementation.

## 1. Executive Summary

The first live trusted-host input observation producer should be one explicit,
read-only local Cargo observer owned by the unpublished `workflow-local-host`
boundary.

The selected producer is not an execution sandbox and is not an authority
source. It observes what the local host can prove about one exact
`CargoClippyWorkspace` operation request, returns the complete closed
`TrustedHostInputObservationResult`, and leaves every unsupported, unavailable,
or uncertain obligation explicit. It does not admit or execute the operation.

OpenShell remains the preferred future optional containment provider, but it
is not selected for this slice. The accepted OpenShell v0.0.101 matrix and the
current upstream discussion still lack the durable operation, applied-state,
complete-observation, cleanup, reconciliation, and capability surfaces needed
for trustworthy provider wiring. Those gaps must not be bypassed by parsing
human output or relabeling requested state as observed fact.

## 2. Decision

Select a **native local Cargo host observer** for the first live observation
producer.

The implementation belongs in `workflow-local-host`, behind a narrow internal
interface that consumes an exact validated Core request and returns Core model
values. Core remains the source of the request/result contract and all
validation. The local-host crate owns read-only operating-system interaction
and fixed process probes.

Do not select:

- OpenShell v0.0.101, because its current authoritative API is insufficient;
- a Workflow OS fork of OpenShell, because Workflow OS should not own that
  runtime and vulnerability surface;
- an arbitrary shell observer, because caller command text is not a bounded
  producer contract; or
- caller-supplied facts, because they cannot establish live observation.

## 3. Goals

- Prove one real, opt-in local observation path using the accepted request and
  result models.
- Bind the producer to an exact implementation and configuration commitment.
- Observe only through fixed read-only file operations and fixed bounded Cargo
  and Rust toolchain probes.
- Return exactly one fact for each of the nine closed host obligations plus one
  repository-root observation.
- Use `Observed` only when object identity and continuity are actually proven.
- Preserve `DeclaredOnly`, `Unavailable`, `Unsupported`, and `Uncertain` when
  proof is incomplete.
- Fail closed on request, producer, revision, root, object, process-result, or
  evidence-reference substitution.
- Keep raw paths, environment names and values, command output, source content,
  tool output, and provider payloads outside Core values.
- Demonstrate honest live observation without claiming admission readiness.

## 4. Non-Goals

The first implementation must not add:

- runtime admission or execution authorization;
- Cargo clippy execution;
- default or automatic observer selection;
- workflow, executor, scheduler, or local-check composition;
- OpenShell installation, sandbox execution, or provider wiring;
- arbitrary command execution or caller-selected probes;
- network access, registry refresh, or dependency download;
- persistence, event emission, audit projection, or report generation;
- schemas, SDK fields, public CLI behavior, or examples;
- provider writes, SideEffect execution, credentials, or external mutations;
- nested harnesses, reasoning lineage, or enterprise administration;
- cryptographic, hardware-backed, or remote attestation claims; or
- release-posture changes.

## 5. Selected Producer Boundary

Candidate internal identity:

```text
producer kind: HostObserver
producer id: observer/local-cargo-host
contract: CargoHostInputsV2
owner: workflow-local-host
```

The exact Rust names should follow repository conventions after review. The
producer version must bind:

- observer implementation commitment;
- fixed probe-set version;
- supported platform family;
- selected Cargo profile version;
- parser and normalization version; and
- privacy-reduction configuration commitment.

`IndependentlyReviewed` means the producer implementation and probe contract
received focused review. It does not mean the local machine, kernel, files, or
processes are independently attested. Documentation and future assessment must
preserve that distinction.

## 6. Crate And Dependency Direction

`workflow-core` owns:

- `TrustedHostInputObservationRequest`;
- `TrustedHostInputObservationResult`;
- producer, fact, posture, method, and root vocabulary;
- deterministic validation and canonical commitments; and
- no host or process access.

`workflow-local-host` may depend on Core and own:

- the local observer implementation;
- a fixed read-only filesystem interface;
- a fixed process-probe interface;
- platform capability detection;
- bounded parsing and payload reduction; and
- the opt-in live smoke boundary.

Core must not depend on `workflow-local-host`. The local observer must not
receive workflow mutation, approval, event, persistence, or admission APIs.

## 7. Exact Input Boundary

The observer accepts only:

- one validated `TrustedHostInputObservationRequest`;
- one exact Cargo clippy profile observation definition;
- explicit canonical repository, toolchain, dependency-cache, build-output,
  and temporary roots already selected by the local host;
- an explicit sanitized environment definition;
- explicit toolchain and dependency inventory candidates; and
- one Core-issued ordering-authority revision source.

The profile observation definition is not free caller configuration. Its
canonical commitment must equal the applicable request-bound operation,
selection, and security commitments. Every root and inventory commitment must
be derivable from the exact static-discovery result or the bound profile
definition. An extra root, executable, dependency, target, or environment
definition that is not covered by those commitments is request substitution
and fails closed.

Candidate inventories are claims to inspect, not proof. The observer must
independently resolve, inspect, and classify them. It must reject hidden global
configuration and must not read ambient process environment except through a
fixed capability probe that reports only bounded posture.

## 8. Fixed Probe Set

The first producer may use only reviewed fixed probes, such as:

- read-only metadata and content commitments for explicit files and roots;
- exact executable resolution for Cargo, rustc, rustdoc, clippy-driver, and
  cargo-clippy;
- `rustc -vV` through a fixed no-input process contract;
- `rustc --print sysroot` through a fixed no-input process contract;
- `rustc --print target-libdir` through a fixed no-input process contract;
- `cargo metadata --locked --offline --format-version 1` with cleared ambient
  environment and the exact explicit Cargo home; and
- platform-specific read-only object metadata only after separate capability
  review.

The implementation must not accept caller command text, shell fragments,
extra arguments, inherited environment, network fallback, or best-effort
parsing of human diagnostics.

Every process probe requires:

- exact executable commitment before and after invocation;
- exact argument vector;
- cleared environment plus a closed explicit allowlist;
- repository-root working directory where required;
- bounded stdout and stderr;
- timeout and process-group termination posture;
- success-only structured parsing;
- rejection of unexpected stderr where the probe contract requires silence;
  and
- stable non-leaking errors.

## 9. Capability And Evidence Matrix

The first implementation must preserve this conservative matrix.

| Obligation | Candidate source | First-slice posture ceiling | Evidence requirement | Reason for ceiling |
| --- | --- | --- | --- | --- |
| Toolchain executable | Exact Cargo/rustc/rustdoc/clippy executable content commitments before and after probes | `Uncertain` in the standalone slice | Canonical set commitment, count, producer/version, pre/post identity method, stable observation reference, and later same-call custody | Explicit files can be inspected, but standalone preflight does not prove admission-to-terminal continuity |
| Target specification | Fixed rustc host/target output plus exact profile target selection | `Uncertain` unless the complete effective target specification is returned canonically | Commitment to canonical complete target object, not target name alone | Stable rustc output does not always expose the complete built-in target specification |
| SDK or sysroot | Fixed rustc sysroot and target-libdir resolution plus bounded object inventory | `Uncertain` | Complete selected sysroot object-set commitment and pre/post continuity | Resolving a sysroot path does not prove every object consumed by the future build |
| Linker or archiver | Explicit profile configuration and independently resolved executable identity | `DeclaredOnly` or `Uncertain` | Actual selected binary set and continuity evidence | Cargo/rustc may select platform tools only during compilation |
| Native compiler tools | Candidate inventory plus Cargo metadata | `Uncertain` | Complete actual tool set, including build-script and procedural-macro use | Preflight cannot prove absence of dynamically selected native tools |
| Dynamic libraries | No accepted preflight-only source | `Unsupported` | Runtime-loader observation bound to the exact operation | Static executable inspection is not the actual loaded-library set |
| Registry and package sources | Locked offline Cargo metadata plus content commitments for selected local source objects | `Uncertain` initially | Complete registry, Git, and path dependency object-set commitment with provenance | Cache presence and lock metadata do not alone prove the exact future-consumed source set |
| Effective environment inputs | Exact sanitized environment that the local host plans to pass | `DeclaredOnly` | Same-call process-owner evidence that no other environment reaches execution | A preflight observer cannot prove the future process receives only the declared environment |
| Runtime base | Local OS/platform metadata | `Unsupported` initially | Immutable base identity from an enforcing or observing runtime boundary | OS name, version, and kernel label are not immutable runtime-base proof |
| Repository root | Exact static-discovery commitment plus full read-only root commitment before and after observation | `Uncertain` for future execution continuity | Accepted root method, exact object commitment, and continuity through admission/execution | Standalone observation cannot prove the root remains unchanged until a later execution |

The result remains valid when facts are non-proven because the closed model
requires explicit posture and reason. In the standalone first slice, no fact
may be `Observed`: the reused continuity posture means admission-to-terminal
continuity, which preflight alone cannot prove. The implementation still adds
value by replacing unknown claims with live bounded facts and exact disclosed
gaps. It is not a satisfied live-execution assessment. A future assessment
must reject it for every mandatory obligation until accepted same-call custody
or an immutable provider projection can establish `Observed` with proven
binding and continuity.

## 10. OpenShell Comparison

OpenShell remains a different and complementary producer kind:

- local Cargo observer: `HostObserver`, read-only preflight facts, no
  containment;
- future OpenShell adapter: `ExecutionProvider`, same-call containment and
  execution-provider facts.

As of 2026-10-24, the accepted upstream discussion has no OpenShell maintainer
answer. Two public third-party comments do not provide the required upstream
contract. The accepted v0.0.101 evidence matrix therefore remains controlling.

OpenShell provider wiring remains blocked on:

- restart-safe create and operation identity;
- driver-observed immutable image identity;
- complete applied-control and degradation state;
- durable terminal operation outcome;
- complete operation-bound observations;
- exact resource-version-bound cleanup and reconciliation; and
- typed capability negotiation.

Do not copy OpenShell CLI output into the local observer or use it as a
substitute for those missing provider facts.

## 11. Repository Root And TOCTOU

The local observer must compose the existing `TrustedHostRootObservation`.
It must not create a second root model.

The first standalone observer may prove only the observation interval. It must
not set root continuity to `Proven` for later execution unless an accepted
same-call owner retains the exact root identity through admission and
execution. Root replacement, symlink substitution, tree drift, or inability to
repeat the exact commitment maps to a stable failure or non-proof posture.

The same rule applies to every fact. A pre/post hash around the observation
probe is not admission-to-terminal continuity and must remain `Uncertain`.

## 12. Ordering And Freshness

The producer must use the exact ordering-authority commitment in the request.
It obtains the next observation revisions from one injected monotonic revision
source in that same authority domain. It must not use wall-clock time as
ordering authority.

The implementation must prove:

- static discovery completed before observation start;
- observation completion follows observation start;
- the result binds the exact request and producer configuration; and
- changed repository, profile, toolchain, target, inventory, environment,
  producer, or security requirements require reobservation.

The result is evidence, not reusable authority.

## 13. Evidence And Privacy

The producer may return only:

- commitments;
- closed postures and reasons;
- bounded item counts;
- producer identity/version;
- ordered revisions;
- accepted root posture; and
- stable payload-free references.

The first implementation should return an in-memory observation evidence
bundle alongside the Core result so every reference resolves within the exact
call. Persistent evidence storage is deferred. The bundle should contain only
canonical commitments and bounded typed metadata, never raw probe output.

Do not store or expose:

- absolute host paths;
- file or source contents;
- executable bytes;
- environment names or values;
- Cargo metadata JSON;
- stdout or stderr;
- registry URLs or package source locations;
- usernames, home directories, endpoints, credentials, or tokens; or
- upstream/provider error bodies.

Debug, serialization, and errors must remain non-leaking.

## 14. Failure And Retry Semantics

Validation failure before host interaction is a stable construction error.
After any process probe starts, ambiguity must not be retried silently. The
producer either reconciles the exact fixed probe through retained local
process outcome or returns `Uncertain`/a stable observation error.

Unknown platform capability, unsupported command shape, oversized output,
unexpected stderr, timeout, identity drift, incomplete inventory, parse
ambiguity, or evidence-reference mismatch fails closed. The observer never
converts failure into approval denial, workflow failure, or runtime status.

## 15. Implementation Sequence

1. Perform a focused maintainer/security review of this plan and matrix.
2. Add one private provider interface and in-memory evidence-bundle contract,
   preserving Core/local-host dependency direction.
3. Implement the local Cargo observer with injected filesystem, process, and
   revision sources; prove all matrix ceilings with deterministic fixtures.
4. Add one opt-in live smoke for the exact local platform and closed probe set.
5. Perform focused review before any runtime composition.
6. Separately plan same-call composition with the existing Cargo clippy local
   host path only after the observer proof is accepted.
7. Revisit OpenShell only after authoritative upstream surfaces change and a
   new evidence-sufficiency review accepts them.

Do not combine observer implementation, live smoke, runtime admission, Cargo
execution, and OpenShell integration into one phase.

## 16. Test Plan

The future implementation must test:

- exact producer kind, assurance, version, implementation, and configuration;
- exact request, repository, ordering-authority, profile, and inventory
  binding;
- all nine obligations and exactly one repository-root observation;
- canonical result and evidence ordering;
- deterministic repeat for unchanged inputs;
- producer, request, revision, root, toolchain, target, environment, and
  evidence-reference substitution rejection;
- exact matrix posture ceilings for every obligation;
- incomplete and duplicate fact rejection through Core constructors;
- pre/post executable and root drift;
- symlink and canonical-root replacement;
- missing executable, non-file, permission, timeout, nonzero, unexpected
  stderr, oversized output, and parse failures;
- no inherited environment and no network fallback;
- no caller-selected executable, argument, command, or extra probe;
- stable non-leaking errors;
- Debug and serialization non-leakage;
- no persistence, event, audit, report, admission, or workflow mutation;
- ordinary tests remain process-free through injected sources;
- the opt-in live smoke remains excluded from default CI; and
- existing observer, static discovery, local-check, local-host, executor,
  hosted-provider, persistence, and workspace tests remain unchanged.

## 17. Acceptance Criteria

The implementation is acceptable only if:

- one explicit local-host producer returns a Core-validated complete result;
- every fact respects the matrix posture ceiling;
- no unsupported fact is promoted to `Observed`;
- live inputs are bound to the exact request and producer version;
- ordinary tests start no process and read no ambient host state;
- one separately opted-in smoke proves the real fixed probe path;
- no result authorizes admission or execution;
- no raw payload or sensitive host detail enters Core values; and
- documentation states plainly that the first result is expected to remain
  insufficient for live execution admission.

## 18. Validation For This Planning Phase

- Inspect the accepted observer request/result model and fixture proof.
- Inspect trusted-host live-security and Cargo clippy profile boundaries.
- Inspect the OpenShell v0.0.101 evidence matrix, current-upstream
  re-verification, optional-provider report, and upstream discussion state.
- Run `npm run check:docs`.
- Run `git diff --check`.

## 19. Open Questions For Review

- Is `workflow-local-host` the correct owner, or should operating-system
  observation live in a new unpublished adapter crate?
- Should the first implementation support one exact platform only, or expose a
  platform capability snapshot and return `Unsupported` elsewhere?
- Can an in-memory evidence bundle provide sufficiently exact reference
  resolution without adding persistence?
- Does the existing `IndependentlyReviewed` assurance label need wording
  clarification so it cannot be mistaken for independent machine attestation?
- Confirm that no fact may be `Observed` before same-call execution
  composition under the existing admission-to-terminal continuity semantics.
- Should process-probe ambiguity return a result with `Uncertain` facts or one
  stable whole-observation error?

## 20. Final Recommendation

Perform a focused maintainer/security review of this plan.

If accepted, begin with the private local-host observer and in-memory evidence
bundle contracts, then implement injected-source behavior before one opt-in
live smoke. Do not begin runtime admission, Cargo execution composition,
OpenShell wiring, persistence, schemas, CLI exposure, or writes.

## 21. Governed Planning Evidence

- Workflow ID: `dg/d`.
- Run ID: `run-1792841264205450000-2`.
- Approval ID: `approval/run-1792841264205450000-2/planning-approved`.
- Approval presentation ID: `presentation/071339777fa3f48b`.
- Approval presentation content hash:
  `071339777fa3f48b4ac5d032d9af9c29cda163fa0a35c135df48c7f78f7e496a`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact planning-only scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected accepted observer,
  live-security, Cargo-profile, OpenShell provider, evidence-matrix, and
  upstream discussion records and authored this plan. The kernel governed
  scope and approval but did not inspect sources, edit files, query GitHub, or
  run validation.
