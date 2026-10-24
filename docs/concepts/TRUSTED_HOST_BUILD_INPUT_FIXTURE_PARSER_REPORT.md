# Trusted-Host Build-Input Fixture Parser Report

## 1. Executive Summary

Workflow OS now has a fixture-only, process-free parser and static graph
resolver for a deliberately narrow Cargo metadata subset. The implementation
reads only approved metadata beneath an explicit fixture root, applies hard
file/byte/graph bounds, rejects unsupported or ambiguous Cargo surfaces, and
returns the existing non-authorizing trusted-host discovery request and result.

The result now separately commits the canonical resolved package nodes and
dependency edges. This prevents a count-equivalent graph substitution from
retaining the same outer result commitment.

This phase does not inspect the live Workflow OS workspace, run Cargo or build
code, observe host inputs, admit trusted-host execution, persist discovery,
expose schemas or CLI behavior, select a provider, or authorize writes.

## 2. Scope Completed

- Added exact `toml` `0.8.23` with default features disabled and `parse` only.
- Added explicit fixture input, hard-limit, discovery, and result-wrapper APIs.
- Parsed root/member Cargo manifests, Cargo lockfile v3/v4, optional toolchain
  metadata, and empty Cargo configuration.
- Resolved exact workspace members, local path dependencies, and reachable
  locked package records under conservative all-potential-edge selection.
- Classified explicitly declared build scripts and local procedural-macro
  crates without executing either.
- Added a canonical semantic graph commitment over resolved nodes, checksums,
  dependency-role edges, feature expressions, and executable-build metadata.
- Rejected malformed, unknown, changing, symlinked, root-escaping, nested,
  inconsistent, ambiguous, unsupported, and over-limit inputs with stable
  non-leaking errors.
- Added deterministic fixed vectors and adversarial tests.

## 3. Scope Explicitly Not Completed

- Live Workflow OS workspace discovery or proof.
- Cargo metadata, build, check, clippy, compiler, linker, archiver, build
  script, procedural macro, package-code, provider-code, or shell execution.
- Arbitrary source-code parsing or implicit target inference.
- Target-specific dependencies, workspace inheritance, registry replacement,
  Git dependencies, non-empty Cargo configuration, workspace globs, symlinks,
  nested repositories, or external package build-code classification.
- Toolchain, sysroot, target, linker, native-tool, dynamic-library,
  environment, registry-source, runtime-base, filesystem, network,
  process-tree, resource, or transient-write observation.
- Runtime admission, handler registration, persistence, workflow events,
  WorkReports, artifacts, schemas, SDKs, CLI behavior, onboarding, examples,
  provider mutations, credentials, authority, or release posture changes.

## 4. API Summary

The public fixture-only boundary is:

- `TrustedHostBuildInputFixtureDiscoveryLimits` and its definition;
- `TrustedHostBuildInputFixtureDiscoveryInput<'a>`;
- `TrustedHostBuildInputFixtureDiscovery`; and
- `discover_trusted_host_build_inputs_from_fixture(...)`.

The caller supplies the fixture root, repository-tree commitment, selection
commitment, and explicit hard limits. The helper does not consult ambient
current-working-directory authority, state backends, providers, or runtime
configuration.

## 5. Parser And Filesystem Boundary

The parser accepts only a reviewed Cargo subset and uses strict typed TOML
decoding. Unknown fields fail closed. The reader canonicalizes the supplied
root, rejects a symlink root and symlink path components, permits only normal
relative metadata paths, verifies canonical descendants remain under the root,
and checks file metadata around bounded reads.

Supported static files are root/member/local-path `Cargo.toml`, one
`Cargo.lock`, at most one toolchain file, and at most one empty Cargo config.
The implementation reads no Rust source content. Presence of undeclared
`build.rs` is rejected because implicit build-script inference is outside the
approved subset.

## 6. Graph And Completeness Semantics

The resolver commits stable, sorted package keys, locked dependency edges,
manifest dependency-role edges, checksums, feature expressions, explicit build
scripts, and local procedural-macro declarations. It requires each local
manifest package and each declared dependency to map unambiguously into the
lock graph.

An all-local supported fixture may report `Complete` for the narrow static
graph. A reachable external locked package reports `Incomplete` with
`SelectionAmbiguous`, because a lockfile alone cannot prove whether that
external package contains executable build scripts or procedural macros.
Every unresolved live host fact remains an explicit unavailable host
obligation. Static completeness never means live build-input completeness.

## 7. Validation And Error Boundary

Hard limits cover file count, per-file bytes, aggregate bytes, workspace
members, local path dependencies, locked packages, graph edges, feature
expressions, and traversal depth. Zero or implementation-exceeding limits are
rejected.

Errors use stable `trusted_host.build_input_fixture_discovery.*` codes and
generic messages. TOML parser details, file paths, package values, lockfile
contents, and secret-like fixture strings are not copied into errors.

## 8. Privacy And Redaction

The input `Debug` implementation redacts the fixture root and caller-supplied
commitments. The output contains only existing payload-free request/result
models, bounded counts, closed posture enums, and commitments. Their `Debug`
implementations redact commitments, and serialization does not include source
paths, TOML contents, feature text, target paths, or build-script paths.

The parser reads no provider payload, credential, environment value, command
output, CI transcript, source body, or runtime state.

## 9. Test Coverage

Focused tests cover:

- a deterministic all-local virtual workspace;
- exact semantic-graph and outer-result fixed vectors;
- normal, build, and development local dependency roles;
- explicit build-script and procedural-macro classification;
- external locked dependency incompleteness;
- malformed and unknown TOML non-leakage;
- root escape, symlink, and nested-repository rejection;
- non-empty Cargo config, target-specific dependency, and workspace
  inheritance rejection;
- implicit target and implicit build-script rejection;
- file limits and inconsistent lockfile rejection;
- input `Debug` and output Debug/serialization non-leakage; and
- result commitment sensitivity to resolved semantic graph substitution.

The existing trusted-host discovery model and workspace test suites remain
part of the required regression matrix.

## 10. Commands Run And Results

- `cargo test -p workflow-core trusted_host_build_input`: passed, 35 tests.
- `cargo test --workspace`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo fmt --all --check`: passed.
- `cargo metadata --locked --format-version 1`: passed.
- `npm run check`: passed under the repository Node 20 toolchain.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `npm audit --audit-level=moderate`: passed with zero vulnerabilities.
- The CI-pinned RustSec audit at advisory database revision
  `309ad29d8fe448bf986019e05d47b9e0e29a2218` passed with one allowed yanked
  dependency warning and no vulnerability findings.
- A current-feed `cargo audit` probe reported seven advisories in pre-existing
  PostgreSQL and TLS dependency paths. None are introduced by the TOML parser
  subtree. The repository intentionally pins the last known-valid RustSec
  snapshot in CI while the upstream feed is under review; this current-feed
  difference remains explicit follow-up rather than a parser-phase claim.
- `cargo tree -p toml@0.8.23`: confirmed the reviewed parser-only dependency
  subtree.
- License inspection confirmed `toml`, `serde_spanned`, `toml_datetime`, and
  `toml_edit` are `MIT OR Apache-2.0`; `winnow` is `MIT`.
- `git diff --check`: passed.

## 11. Remaining Known Limitations

- The parser implements a strict Cargo subset, not complete Cargo semantics.
- External locked packages cannot be classified for build scripts or
  procedural macros without additional bound metadata, so they remain
  incomplete.
- Repository-tree and selection commitments are caller-supplied; this phase
  does not independently prove Git object identity.
- File reads are root-contained and checked for observed metadata change, but
  this fixture proof is not an immutable object-store reader.
- Source-file existence and contents are not inspected.
- Live host inputs and execution controls remain unavailable obligations.
- The live RustSec feed currently reports advisories in pre-existing
  PostgreSQL and TLS dependency paths even though the repository's pinned CI
  snapshot passes. Dependency remediation or CI snapshot advancement requires
  a separately governed supply-chain phase.
- Final dogfood phase-close inspection could not rehydrate the implementation
  run from the configured state directory. The kernel returned
  `runtime.events.empty`; no matching run record remained in repository or
  system temporary state. The run is not reconstructed or claimed closed.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the fixture parser and static
graph resolver. Review the dependency surface, strict TOML subset, path and
TOCTOU posture, hard bounds, package/edge semantics, semantic graph commitment,
external-package incompleteness, privacy behavior, and adversarial tests before
authorizing any live Workflow OS workspace proof.

## 13. Governed Phase Record

- Workflow ID: `dg/implement`.
- Run ID: `run-1792827489674551000-2`.
- Approval ID:
  `approval/run-1792827489674551000-2/implementation-approved`.
- Approval-presentation ID: `presentation/f7bbbdbe6059c597`.
- Approval-presentation content hash:
  `f7bbbdbe6059c59765d4e0c52aa866873e862f2fd0beac5a7ca434c52a53ac`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact fixture-only implementation scope.
- Phase status: the implementation run was observed completed before coding,
  but final phase-close rehydration failed with `runtime.events.empty` after
  validation. No terminal event count is claimed.
- Event summary: unavailable because the configured state directory contained
  no event stream for the recorded run ID at phase close.
- Out-of-kernel work: the delegated maintainer edited code and docs, inspected
  dependency resolution, and ran tests and validation. The kernel governed
  scope and approval but did not edit files, parse fixtures, run validation,
  or perform git or pull-request operations.
