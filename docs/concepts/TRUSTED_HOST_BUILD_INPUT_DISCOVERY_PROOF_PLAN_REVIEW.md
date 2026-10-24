# Trusted-Host Build-Input Discovery Proof Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the Core static-discovery request/result model and
canonicalization only.**

The plan defines a truthful boundary between deterministic repository metadata
discovery and complete live build-input observation. It does not mistake Cargo
file parsing for proof of the host toolchain, sysroot, linker, dynamic
libraries, environment, provider, or runtime image. The first implementation
is appropriately limited to non-authorizing model vocabulary and canonical
commitments.

## 2. Scope Verification

The plan remains planning-only. It does not authorize Cargo, compiler, linker,
build-script, procedural-macro, package-code, provider-code, or shell
execution. It adds no provider, handler, executor path, persistence, events,
reports, artifacts, CLI behavior, configuration, schema, SDK, example,
dependency, credential, mutation, write, hosted expansion, or release change.

The proposed first implementation is smaller than the complete proof: model
types and canonicalization only. Parser, fixture discovery, workspace
discovery, host observation, provider suitability, and live execution each
remain separately reviewed phases.

## 3. Proof-Claim Assessment

The bounded proof claim is appropriately narrow. It promises deterministic
parsing of approved static metadata sources for one exact repository snapshot
and one closed discovery-contract version. It explicitly does not promise that
the resulting graph contains every input a live build can consume.

This distinction is the plan's most important security property. Static graph
completeness and complete live build-input closure must remain different types
or closed postures. Static discovery alone must never construct
`BuildTimeInputCompletenessPosture::Complete` for the strict live-execution
profile.

## 4. Exact Snapshot And Authority Assessment

The request must bind an opaque repository-tree commitment, discovery-contract
version, metadata selection, and feature/target/profile selection. These values
describe one discovery operation; they do not authorize filesystem access or
execution.

The first model must not accept an ambient current directory, raw host path,
Git branch name, mutable working-tree label, or caller assertion that a root is
immutable. A later parser boundary must receive a separately object-bound read
capability or fixture source and prove that observed metadata corresponds to
the request commitment.

The plan correctly leaves the object-binding mechanism open. That is not a
blocker to the model-only phase, but it is a blocker to any real repository
discovery implementation.

## 5. Allowed-Input Assessment

The closed source list is conservative: workspace and package manifests,
lockfile, optional repository-local toolchain and Cargo configuration,
explicit local path manifests, repository-object metadata, and limited file
metadata for root-containment checks.

The plan correctly excludes arbitrary source inspection as an inference
engine. Presence-only checks for implicit targets or `build.rs` require their
own reviewed semantics; unsupported constructs fail closed instead of being
approximated.

Repository-local configuration cannot prove the absence of user-, system-, or
environment-level Cargo configuration. The plan treats ambient configuration
as a later host obligation, which is required.

## 6. Cargo Graph Assessment

The planned graph sequence is deterministic in shape and correctly requires
all selected member and local-path manifests plus an unambiguous lock record
for external packages. Canonical ordering and domain separation are explicit.

`Cargo.lock` alone does not determine the activated feature, target, profile,
or build dependency graph. The request/result model must therefore carry an
opaque closed selection commitment and an explicit selection posture. The
later parser must either implement reviewed Cargo-compatible selection rules,
conservatively include every potential edge with an over-approximation marker,
or return unsupported. It must not silently choose host defaults.

## 7. Build-Script And Procedural-Macro Assessment

The plan correctly treats build scripts and procedural macros as executable
inputs while forbidding their execution. Static discovery may classify their
packages and static dependencies, but it cannot observe generated files,
`cargo:rerun-if-*` behavior, compiler probing, native library discovery,
environment reads, network reads, or dynamically selected outputs.

Those unknowns must participate in the result's unsupported/unavailable
obligations. A graph containing a build script or procedural macro can still be
a complete static graph under closed rules, but cannot by that fact become a
complete live-input manifest.

## 8. Toolchain And Host-Input Assessment

The plan correctly prevents version strings and installation paths from being
treated as object identity. Requested toolchain metadata is distinct from an
observed toolchain executable, target, sysroot, SDK, linker, archiver, native
tool, dynamic library set, registry/package object set, environment, and
runtime/base image.

The model should represent these as closed obligations and postures, not a bag
of caller-authored strings. The first implementation must default unresolved
host obligations to unsupported or unavailable.

## 9. Symlink And Dynamic-Ambiguity Assessment

Default symlink rejection is appropriate. Submodules, sparse-checkout
indirection, nested repositories, root escapes, generated-later inputs, and
metadata mutation remain unsupported unless later object binding proves their
identity and containment.

The plan should be interpreted strictly: observed absence means absence from
the exact approved repository snapshot and source list, not global absence
from the host or build environment.

## 10. Model-Boundary Assessment

The candidate model is appropriately provider neutral and non-authorizing.
Implementation may refine names, but it must preserve these distinct facts:

- exact request binding;
- closed discovery-contract version;
- repository and selection commitments;
- static graph completeness;
- canonical graph commitment;
- bounded kind counts;
- unsupported/unavailable obligation codes; and
- stable optional observation reference.

Reuse existing `BuildTimeInputKind` vocabulary only where semantics are exact.
Do not widen the existing live manifest or expose a new public assessment
function merely for convenience. Derived commitments should be recomputed from
authoritative fields and omitted from untrusted input.

## 11. Bounds And Failure Assessment

The plan identifies the necessary count, byte, depth, and collection bounds.
Exact numeric limits may be selected during implementation using existing Core
conventions, then reviewed as part of the model phase.

Stable value-independent error families are required for invalid binding,
invalid selection, duplicate or missing kinds, impossible count totals,
unsupported obligations, oversized collections, malformed serde, and
canonicalization failure. No rejected value may be echoed.

## 12. Privacy Assessment

The payload-free posture is sound. Public values may contain closed enums,
bounded counts, opaque stable references, and commitments only. Raw paths,
file names, manifest fragments, package names or versions, source URLs,
environment names or values, tool locations, contents, commands, outputs,
credentials, and provider logs remain excluded.

Debug must redact commitments and references. Serialization must not create a
second raw inventory. Deserialization must fail closed without echoing unknown
or secret-like values.

## 13. Fixture And Test Assessment

The synthetic supported fixture, unsupported fixture matrix, and Workflow OS
workspace proof form an appropriate later proof suite. The fixed-vector
canonical fixture should protect encoding stability; the live workspace test
should assert deterministic invariants and honest incompleteness rather than a
brittle whole-workspace golden commitment.

The first model-only tests should cover:

- valid request and result construction;
- exact binding and substitution rejection;
- deterministic order-independent canonicalization;
- fixed-vector result commitment;
- static completeness distinct from live completeness;
- required unsupported host obligations;
- kind-count and total-count coherence;
- bounds and duplicate rejection;
- serde round trip through validated constructors;
- invalid serde failure without leakage;
- redaction-safe Debug; and
- compile-time or dependency-boundary proof that no executor/provider/process
  API is required.

Parser, filesystem, mutation-detection, symlink, workspace, and Cargo-semantic
tests belong to later phases and must not be simulated in the model phase.

## 14. Planning Blockers

No blocker remains for the model-only implementation.

Before fixture parser implementation, a focused plan or accepted implementation
decision must resolve:

- structured TOML parsing and dependency posture;
- exact feature/target/profile selection semantics;
- implicit target and build-script presence semantics;
- repository object binding and mutation detection;
- local path, registry, Git source, replacement-source, and checksum
  normalization; and
- the exact rule for composing static completeness with later host
  observations.

These are intentionally later blockers and must not be hidden inside the model
implementation.

## 15. Non-Blocking Follow-Ups

- Select concrete collection and byte limits during model implementation using
  existing Core limits as precedent.
- Decide whether an existing hosted observation-reference kind has exact
  semantics before adding a new one.
- Keep package identity details inside canonical commitments unless a later
  diagnostic use case justifies bounded public vocabulary.
- Prefer a closed selection-posture enum over a boolean complete flag.

## 16. Recommended Next Phase

Implement the **Core static-discovery request/result model and canonicalization
only**.

The implementation should add validated non-authorizing types, deterministic
domain-separated commitments, static completeness and host-obligation
vocabulary, stable errors, serde and redaction behavior, and focused model
tests. It must not read repository files, parse TOML, start processes, select a
provider, register a handler, persist state, expose schemas, or enable writes.

## 17. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

No implementation or runtime validation was required for this documentation
review.

## 18. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792818900864067000-2`.
- Approval ID:
  `approval/run-1792818900864067000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/a24bc5453154551b`.
- Approval-presentation content hash:
  `a24bc5453154551bc321cdce6076d3d97675c14bccc1d40533e76e9c348f9688`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and its
  event marker is present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected the accepted plan,
  its security foundations, current Core vocabulary, workspace Cargo metadata,
  lockfile shape, and repository inventory; authored this review and roadmap
  update; and ran documentation validation. The kernel governed scope and
  approval but did not edit files, run checks, or perform git or pull-request
  operations.
