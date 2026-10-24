# Trusted-Host Build-Input Discovery Proof Plan

Status: planning and focused review accepted; Core static-discovery model and
its blocker fix are accepted; the TOML parser dependency decision is accepted;
the fixture-only parser and graph resolver are implemented and reviewed with
blocker fixes required. Live Workflow OS workspace proof remains unimplemented
and blocked pending fix and focused re-review.

Implementation evidence is recorded in the [Trusted-Host Build-Input Discovery
Core Model Report](../concepts/TRUSTED_HOST_BUILD_INPUT_DISCOVERY_MODEL_REPORT.md).
The focused model verdict is recorded in the [Trusted-Host Build-Input
Discovery Core Model
Review](../concepts/TRUSTED_HOST_BUILD_INPUT_DISCOVERY_MODEL_REVIEW.md).
Blocker-fix evidence is recorded in the [Trusted-Host Build-Input Discovery
Core Model Blocker Fix
Report](../concepts/TRUSTED_HOST_BUILD_INPUT_DISCOVERY_MODEL_BLOCKER_FIX_REPORT.md).
Focused acceptance is recorded in the [Trusted-Host Build-Input Discovery Core
Model Blocker Fix
Review](../concepts/TRUSTED_HOST_BUILD_INPUT_DISCOVERY_MODEL_BLOCKER_FIX_REVIEW.md).
The parser dependency selection is recorded in the [Trusted-Host Build-Input
TOML Parser Dependency
Decision](trusted-host-build-input-toml-parser-dependency-decision.md).
Fixture implementation evidence is recorded in the [Trusted-Host Build-Input
Fixture Parser Report](../concepts/TRUSTED_HOST_BUILD_INPUT_FIXTURE_PARSER_REPORT.md).
The focused verdict is recorded in the [Trusted-Host Build-Input Fixture Parser
Review](../concepts/TRUSTED_HOST_BUILD_INPUT_FIXTURE_PARSER_REVIEW.md).

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Cargo Clippy Live-Execution Security Boundary Plan](trusted-host-cargo-clippy-live-execution-security-plan.md)
- [Cargo Clippy Live-Execution Security Plan Review](../concepts/TRUSTED_HOST_CARGO_CLIPPY_LIVE_EXECUTION_SECURITY_PLAN_REVIEW.md)
- [Trusted-Host Live-Execution Security Contract Core Model Plan](trusted-host-live-execution-security-contract-model-plan.md)
- [Trusted-Host Live-Execution Security Contract Model Blocker Fix Review](../concepts/TRUSTED_HOST_LIVE_EXECUTION_SECURITY_CONTRACT_MODEL_BLOCKER_FIX_REVIEW.md)

## 1. Executive Summary

Plan one deterministic, no-build-code-execution proof for discovering the
inputs that a future trusted-host Rust build may consume. The first
implementation should parse an explicitly supplied repository snapshot and a
closed set of Rust/Cargo metadata files, resolve the statically declared
workspace and locked package graph, classify executable build-time components,
and return a bounded payload-free discovery result.

The proof has two fixtures:

1. synthetic repositories that exercise supported and unsupported graph
   shapes; and
2. the Workflow OS workspace at an explicitly supplied repository commitment.

The proof does not run Cargo, `rustc`, a linker, build scripts, procedural
macros, package code, provider code, or shell commands. It cannot by itself
prove host toolchain, sysroot, linker, dynamic-library, environment-derived,
or runtime-image inputs. Those facts must remain explicit unsupported or
unavailable obligations until a separately reviewed observer can bind them.

The fixture-only parser and resolver described by this plan are now
implemented. This document still does not authorize live workspace discovery
or execution.

## 2. Goals

- Define the smallest truthful static discovery boundary for one Rust/Cargo
  workspace snapshot.
- Bind discovery to an explicit repository-tree commitment and discovery
  contract version.
- Parse workspace manifests, member manifests, lockfile, optional toolchain
  files, and optional Cargo configuration without invoking Cargo.
- Resolve the selected workspace package graph and classify locked dependency,
  build-script, and procedural-macro inputs.
- Produce deterministic bounded commitments and counts rather than raw paths
  or file contents.
- Distinguish complete static graph discovery from complete live build-input
  closure.
- Fail closed for unsupported, ambiguous, missing, changing, or unbounded
  input sources.
- Prove behavior against fixtures and the Workflow OS workspace.
- Preserve a future provider-neutral composition boundary.

## 3. Non-Goals

Do not implement in this phase:

- Cargo metadata, build, check, clippy, compiler, linker, archiver, build-script,
  procedural-macro, package-code, provider-code, or shell execution;
- provider selection, OpenShell integration, sandbox composition, or a live
  process;
- filesystem mounts, object binding, network policy, process-tree control,
  resource enforcement, cleanup, or transient-write observation;
- automatic handler registration, executor integration, runtime authority, or
  live-check admission;
- package downloads, registry access, source checkout, vendoring, or dependency
  cache mutation;
- persistence, events, WorkReports, artifacts, schemas, SDKs, CLI behavior,
  onboarding, examples, provider mutations, credentials, or writes;
- new dependencies or feature flags without a separately reviewed need; or
- hosted expansion or release-posture changes.

## 4. Proof Claim

The first implementation may prove only this bounded claim:

> For one immutable repository snapshot and one closed discovery-contract
> version, Workflow OS deterministically parsed every approved static metadata
> source present inside the supplied repository root, resolved the selected
> workspace members and lockfile package records within supported syntax and
> hard bounds, classified declared build scripts and procedural macros, and
> committed the resulting payload-free graph without executing build code.

It must not claim that these are all inputs a live build will consume. A
future complete live-build claim additionally requires independently observed
and object-bound toolchain, sysroot, linker/archiver, dynamic library,
environment, filesystem, network, process-tree, and provider/runtime facts.

The existing `BuildTimeInputCompletenessPosture::Complete` is therefore not a
valid output of static discovery alone for the strict live-execution profile.
The discovery result should carry a narrower static-graph completeness value;
later reviewed composition may combine that value with host observations to
construct a complete `BuildTimeInputManifest`.

## 5. Discovery Inputs

The caller must provide explicit, already available inputs:

- an object-bound or test-fixture repository root;
- the expected repository-tree commitment;
- the selected root `Cargo.toml` relative location;
- the expected `Cargo.lock` relative location;
- the discovery-contract version;
- the selected workspace package set or closed default selection rule; and
- hard collection and byte limits.

The implementation must not discover authority from ambient current-working
directory state. It may read only files reached through the validated root and
the approved relative-location rules.

## 6. Allowed Static Metadata Sources

The first proof may read only:

- root and member `Cargo.toml` files;
- the selected `Cargo.lock`;
- `rust-toolchain` or `rust-toolchain.toml`, when present;
- `.cargo/config` or `.cargo/config.toml`, when present;
- explicitly declared local path dependency manifests;
- repository object metadata needed to verify the supplied snapshot
  commitment; and
- file metadata needed to reject symlink, type, size, or root-escape
  ambiguity.

It must not read arbitrary Rust source to infer dynamic behavior. Source-tree
content may be represented by a precomputed repository-tree commitment or
bounded object commitments, but not copied into the result.

If Cargo semantics require a source file to decide whether a package has an
implicit target or implicit `build.rs`, the first implementation must either
use a closed, separately reviewed presence-only rule or mark the package
unsupported. It must not silently approximate Cargo target discovery.

## 7. Parsing Boundary

Use structured parsing with a pinned, reviewed syntax boundary. The parser must
not evaluate commands, environment interpolation, shell expressions, plugin
code, or Cargo configuration aliases.

The workspace has no existing TOML parser. The accepted dependency decision
selects exact `toml` version `0.8.23` with default features disabled and only
the `parse` feature enabled. The separately governed parser implementation must
add and lock that dependency, inspect its resolved dependency and license
surface, and fail the phase for any unexpected supply-chain expansion. It must
not enable formatting, insertion-order preservation, or unbounded parsing.

The parser must reject or classify as unsupported:

- unknown metadata constructs that affect graph resolution;
- malformed or duplicate semantic declarations;
- workspace inheritance that cannot be resolved from approved files;
- target-specific dependency expressions requiring unavailable target facts;
- registry replacement or source configuration that cannot be bound;
- environment-relative, home-relative, or root-escaping paths;
- unsupported Cargo config keys that can change compilation inputs; and
- metadata changes detected between initial binding and final commitment.

## 8. Workspace And Package Graph

The discovery algorithm should:

1. bind and parse the root manifest;
2. expand workspace members and excludes using deterministic repository-local
   matching only;
3. reject symlinked member resolution unless the object-binding rule proves
   the target remains inside the committed root;
4. parse every selected member and local path dependency manifest;
5. parse the lockfile and require one unambiguous locked record for every
   external package edge;
6. canonicalize packages by source kind, package name, version, checksum or
   repository commitment, and package role without exposing those raw values;
7. classify normal, build, development, target-specific, build-script, and
   procedural-macro roles; and
8. commit the canonical graph with domain separation and stable ordering.

Optional, target-specific, feature-gated, and development dependencies must
not be silently discarded. The first proof should either receive a closed
feature/target/profile selection commitment or conservatively include every
potential edge and disclose that over-approximation. If neither is truthful,
the result is unsupported.

## 9. Build Scripts And Procedural Macros

Build scripts and procedural macros are executable build inputs, not ordinary
metadata.

Static discovery must:

- identify packages that declare or implicitly contain a build script under a
  reviewed presence rule;
- identify procedural-macro crates from package target declarations and locked
  dependency roles;
- include their package/source commitments and transitive static dependency
  closure in the graph;
- record bounded counts by closed input kind; and
- mark their runtime-discovered outputs and environment reads as unobserved.

The proof must never execute them. Any claim about `cargo:rerun-if-*`, generated
files, compiler probing, native-library discovery, environment access, or
network access remains unavailable without an independent execution observer.

## 10. Toolchain And Host Inputs

Repository-local toolchain metadata may identify a requested channel, version,
components, and target only through a commitment. It does not prove the actual
toolchain executable or sysroot.

Static discovery must report separate obligations for:

- toolchain executable and component object bindings;
- target specification;
- sysroot and SDK;
- linker and archiver;
- native compiler tools;
- dynamically loaded libraries;
- registry/index and package-source objects;
- environment values that can affect Cargo or `rustc`; and
- runtime/base image or host identity.

For the first implementation these obligations are `Unavailable` or
`Unsupported`, never fabricated from version strings or paths. The Workflow OS
workspace fixture is expected to prove its repository graph while honestly
remaining incomplete for these host inputs.

## 11. Symlinks, Submodules, And Generated Inputs

- Symlinks must be rejected by default. A future implementation may accept
  them only with root-contained target binding and cycle detection.
- Git submodules, sparse checkouts, worktree indirection, and nested repositories
  are unsupported in the first proof unless the supplied repository commitment
  includes and independently binds their object identity.
- Generated sources and vendored outputs are ordinary repository content only
  when present in the committed snapshot. A declaration that they will be
  generated later is not an input commitment.
- Files outside the committed repository root are unsupported.
- Any metadata mutation during discovery invalidates the result.

## 12. Candidate Core Types

The first implementation should remain a small provider-neutral module, likely
with concepts such as:

- `TrustedHostBuildInputDiscoveryRequest`;
- `TrustedHostBuildInputDiscoveryContractVersion`;
- `TrustedHostBuildInputDiscoveryResult`;
- `TrustedHostStaticGraphCompleteness`;
- `TrustedHostBuildInputGraphCommitment`;
- `TrustedHostBuildInputKindCount` or reuse of the existing compatible type;
- `TrustedHostBuildInputUnsupportedReason`; and
- `TrustedHostBuildInputObservationReference` if an existing bounded reference
  cannot express the fixture/workspace proof.

Names should follow implementation review and existing conventions. Private
fields, validated constructors, closed enums, deterministic canonicalization,
fail-closed serde, and redaction-safe Debug are mandatory.

Do not add a second terminal receipt or make discovery results executable
authority. The discovery operation returns facts for later composition only.

## 13. Deterministic Commitment

The graph commitment must use a versioned, domain-separated canonical encoding
that includes:

- discovery-contract version;
- repository-tree commitment;
- selected workspace/package/feature/target/profile commitment;
- canonical metadata-file object commitments;
- canonical package nodes and dependency edges;
- build-script and procedural-macro classifications;
- static completeness posture;
- unsupported and unavailable obligation codes; and
- bounded kind counts and total counts.

Do not serialize a caller-supplied derived fingerprint as authoritative input.
Deserialization must reconstruct and revalidate authoritative fields or omit
public deserialization until a real consumer requires it.

## 14. Bounds And Failure Behavior

Hard limits are required for:

- metadata files;
- bytes per file and total bytes;
- workspace members;
- local path dependencies;
- locked packages;
- package edges;
- feature and target expressions;
- symlink depth and graph traversal depth; and
- unsupported-reason and observation-reference collections.

Limit overflow fails closed with stable value-independent errors. Missing
files, inconsistent lock data, root escape, graph cycles outside supported
Cargo semantics, ambiguous package identity, unsupported source replacement,
and snapshot drift cannot produce `Complete` static posture.

No error may include a raw path, manifest fragment, package-source URL,
environment value, repository content, credential, or secret-like value.

## 15. Privacy And Redaction

The result may store only closed vocabulary, bounded counts, stable opaque
references, and cryptographic commitments. It must not store:

- raw paths or file names;
- manifest or lockfile contents;
- source or dependency contents;
- package-source URLs;
- commands or output;
- environment names or values;
- toolchain installation locations;
- registry credentials or network payloads; or
- provider/sandbox logs.

Debug output must redact commitments and references. Serialization must remain
payload free. Errors must disclose only stable codes and bounded generic
context.

## 16. Proof Fixtures

The implementation proof should include:

### Synthetic supported fixture

- a bounded workspace with multiple members;
- normal and build dependencies;
- a declared build script;
- a procedural-macro dependency;
- target/feature selection fixed by an explicit commitment; and
- a stable expected graph commitment and counts.

### Synthetic unsupported fixtures

- missing or inconsistent lockfile;
- root-escaping local path dependency;
- symlinked member or dependency;
- unsupported source replacement/configuration;
- ambiguous target/feature selection;
- implicit build-script ambiguity;
- changed metadata during observation;
- oversized files, graph, or collections; and
- malformed secret-like values proving non-leaking errors.

### Workflow OS workspace fixture

- root manifest plus four current workspace members;
- current lockfile package closure;
- current absence of repo-local toolchain, Cargo config, and build-script files
  recorded as observed absence, not implied global absence;
- transitive procedural-macro packages classified from the locked graph; and
- host toolchain/sysroot/linker/dynamic-library obligations reported as
  unavailable, preventing a complete live-input claim.

The workspace test should avoid a brittle whole-repository golden value where
ordinary dependency updates would create unexplained churn. Protect canonical
encoding with a small fixed-vector fixture and assert workspace invariants with
reviewable counts/postures plus deterministic repeatability.

## 17. Test Plan

Future implementation tests must cover:

- deterministic repeat discovery for identical snapshots;
- fixed-vector canonical graph commitment;
- repository-commitment and selection substitution rejection;
- workspace member and local path resolution;
- lockfile package-edge consistency;
- build-script and procedural-macro classification without execution;
- explicit target/feature/profile selection behavior;
- missing, malformed, changing, and oversized metadata;
- symlink, submodule, root-escape, and cycle handling;
- unsupported Cargo config and source replacement;
- unavailable toolchain/sysroot/linker/dynamic-library obligations;
- payload-free serde and redaction-safe Debug;
- secret-like error non-leakage;
- fixture bounds and denial behavior;
- Workflow OS workspace repeatability and honest incompleteness; and
- proof that no child process, network client, provider, state backend, event
  store, filesystem writer, or executor is required.

## 18. Candidate Implementation Sequence

1. Focused maintainer/security review of this plan. Completed and accepted.
2. Core static-discovery request/result model and canonicalization only.
   Completed.
3. Focused model review and blocker repair. Completed and accepted.
4. Fixture-only structured metadata parser and graph resolver. Next.
5. Focused parser/security review.
6. Workflow OS workspace proof using the reviewed parser.
7. Focused proof review.
8. Only then plan a provider-neutral host-input observer for toolchain,
   sysroot, linker, dynamic-library, and object-binding facts.
9. Provider suitability and live execution remain later phases.

Each implementation phase must remain independently reviewable. Do not combine
the model, parser, workspace proof, host observer, and provider integration into
one change.

## 19. Open Questions

- Which existing TOML capability, if any, can parse supported Cargo metadata
  without adding an unjustified dependency?
- Should the first package selection conservatively include all target/feature
  edges or require one explicit selection commitment?
- Can implicit target and `build.rs` presence be supported safely without
  source-content parsing?
- What repository object-binding API can prove no metadata changed during the
  read without coupling Core to Git?
- Should package names and versions be omitted entirely from the public model
  and represented only in the graph commitment?
- Which existing hosted reference kind can identify discovery evidence without
  widening unrelated semantics?
- What exact static posture is required before a later host observer may
  compose a live `BuildTimeInputManifest` as complete?
- How should registry source checksums, Git dependency commits, and local path
  object commitments be normalized across platforms?

## 20. Final Recommendation

Implement a focused **fixture-parser blocker fix**, then re-review it.

Fix path containment for every accepted path-bearing Cargo field, aggregate
feature-expression bounds, dependency feature/default/optional commitment and
canonical ordering, and optional metadata symlink ambiguity. Do not combine
the fix with Cargo execution, a provider adapter, live workspace proof, a
handler, host observation, persistence, schemas, writes, or release changes.

## 21. Governed Planning Record

- Workflow ID: `dg/d`.
- Run ID: `run-1792818106697352000-2`.
- Approval ID:
  `approval/run-1792818106697352000-2/planning-approved`.
- Approval-presentation ID: `presentation/290d72217824a9f1`.
- Approval-presentation content hash:
  `290d72217824a9f181349ab3ac23a3840babaa3539bb98aa7f84e29bc319b931`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact planning-only scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and its
  event marker is present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected the accepted security
  plans, model reports and reviews, current model vocabulary, workspace Cargo
  metadata, lockfile shape, and repository file inventory; authored this plan
  and roadmap update; and ran documentation validation. The kernel governed
  scope and approval but did not edit files, run validation, or perform git or
  pull-request operations.
