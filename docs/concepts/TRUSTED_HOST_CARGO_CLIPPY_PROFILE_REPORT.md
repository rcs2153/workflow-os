# Trusted-Host Cargo Clippy Profile Implementation Report

## 1. Executive Summary

Workflow OS now models one closed Cargo clippy profile for exactly
`cargo clippy --workspace --all-targets -- -D warnings`. The implementation is
deliberately non-live: public API exposes only non-executing profile
vocabulary, while a crate-private authoritative handler can run only through
an injected process runner used by module tests.

The handler binds explicit repository, toolchain, platform-tool, dependency,
cache, build-output, and temporary inputs before runner entry. It synthesizes
an offline eight-variable environment, validates protected identity before and
after the injected call, and returns the existing payload-free local-check
result. It does not provide operating-system containment or authority to run a
real process.

## 2. Scope Completed

- Added `CargoClippyWorkspace` to the closed command/profile vocabulary.
- Added the exact canonical clippy command contract and
  `BuildOrCacheWrites` boundary with source-read, cache-write,
  build-output-write, and temporary-write effects.
- Added typed private cache, build-output, and temporary root roles with exact
  role/order validation and bidirectional overlap rejection.
- Added a crate-private handler implementing the existing authoritative
  local-check boundary.
- Added explicit Cargo, cargo-clippy, clippy-driver, rustc, rustdoc, and
  additional platform-tool identity.
- Added explicit dependency-input and protected-repository commitments.
- Added a sanitized offline request and existing payload-free result mapping.
- Added focused fail-closed tests without starting a process.

## 3. Scope Explicitly Not Completed

This phase does not add a standard-process constructor, production resolver,
live Cargo invocation, live smoke, workflow composition, durable runtime
replay, default registration, automatic checks, public CLI/config/schema/SDK
surface, examples, provider access, external writes, hosted execution,
OpenShell integration, or another profile.

No operating-system filesystem isolation, egress denial, process-group
termination proof, transient-write observation, or independent sandbox
attestation is claimed.

## 4. Model And Handler API Summary

`ExplicitLocalCheckProfileSelection::cargo_clippy_workspace()` selects the
profile vocabulary. The public generic profile resolver rejects that selection
with `local_check.profile.resolver_mismatch`; no matching public resolver
exists.

The crate-private handler requires an explicit canonical contract, explicit
identity definition, and injected `LocalCheckProcessRunner`. It implements
`AuthoritativeLocalCheckHandler` as `local/check-cargo-clippy@v0`, but it is not
registered or reachable through a production runtime path.

## 5. Identity And Integrity Boundary

Before runner entry, the handler validates and commits:

- the complete canonical command contract;
- a locked repository root containing `Cargo.toml` and `Cargo.lock`;
- the protected repository tree, including source, manifests, lockfile, and
  build scripts;
- exact Cargo, cargo-clippy, clippy-driver, rustc, rustdoc, and additional
  platform-tool files under one explicit toolchain root;
- explicitly enumerated dependency files under one dependency-cache root; and
- typed cache, build-output, and temporary roots plus their initial state.

It repeats source, toolchain, dependency, root-role, and execution-identity
validation after the injected call. Missing inputs, symbolic links, duplicate
identity inputs, root overlap, role substitution, or persistent protected
drift fail closed with stable non-leaking errors.

## 6. Environment And Side-Effect Boundary

The request contains only `CARGO_HOME`, `CARGO_NET_OFFLINE`,
`CARGO_TARGET_DIR`, `PATH`, `RUSTC`, `RUSTDOC`, `RUSTUP_HOME`, and `TMPDIR`.
`PATH` is synthesized from directories containing the explicitly committed
toolchain executables. No ambient home, proxy, credential, or caller-selected
environment is copied.

Cache, build-output, and temporary roots may change. The protected repository,
toolchain files, and enumerated dependency inputs may not. This detects
persistent final-state drift; it does not prevent writes or detect a transient
write restored before the post-call commitment.

`CARGO_NET_OFFLINE=true` constrains Cargo dependency behavior but does not
prevent build scripts, procedural macros, or child processes from opening
network connections. No egress-control claim is made.

## 7. Result, Privacy, And Replay Posture

Passed, failed, and timed-out injected outcomes map to the existing
`LocalCheckResult`. Raw stdout and stderr are not persisted as evidence. Debug
output redacts paths, roots, commitments, and runner details; errors expose
stable codes without copying protected values.

The stable profile and execution commitments are compatible with a future
immutable run/replay path, but no runtime composition or durable replay path
was added. A later phase must review that boundary separately.

## 8. Test Coverage

Focused tests cover:

- exact profile, command, side-effect, and authoritative skill vocabulary;
- exact offline environment and absence of ambient home/proxy variables;
- payload-free pass, failure, and timeout results;
- missing platform-tool identity before runner entry;
- root overlap and writable-root role substitution;
- source, manifest, lockfile, build-script, toolchain, platform-tool, and
  dependency drift after runner entry;
- accepted cache, build-output, and temporary mutations;
- redacted Debug and serialized result output; and
- rejection by the public generic production resolver.

Existing local-check integration tests continue to cover default registration,
contract validation, output privacy, profile collision, and existing profile
behavior.

## 9. Validation

- `cargo fmt --all --check`: passed.
- focused Cargo clippy module tests: passed, 8 tests.
- `cargo test -p workflow-core --test local_check`: passed, 82 tests with two
  existing opt-in live smokes ignored.
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; credential- and environment-dependent live
  tests remained ignored by their existing opt-in contracts.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed after the final report update.
- `git diff --check`: passed.

## 10. Governed Implementation Record

- Workflow ID: `dg/implement`
- Run ID: `run-1792803170034381000-2`
- Approval ID:
  `approval/run-1792803170034381000-2/implementation-approved`
- Presentation ID: `presentation/6ba68ba0d7d0c23a`
- Presentation content hash:
  `6ba68ba0d7d0c23a05fefdf07ae6ba61eb1092549e1c396572e2dd953b4b2c35`
- Approval outcome: granted under delegated-maintainer authority for the exact
  injected-only implementation scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof marker
- Approval presentation enforcement: `proof_enforced`
- Out-of-kernel work: source edits, tests, documentation, report writing, and
  git operations were performed by the delegated maintainer; the kernel
  governed scope and approval but did not perform those actions.

## 11. Remaining Known Limitations

- Build scripts and procedural macros may execute arbitrary repository or
  dependency code in a future live process.
- Additional platform tools cannot yet be proved complete for arbitrary
  repositories or host platforms.
- There is no OS-enforced filesystem or network containment.
- Complete process-group termination is not proved.
- Final-state commitments cannot detect restored transient writes.
- No accepted live sandbox attestation substitutes for those missing controls.
- No runtime path can execute or replay this profile.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the injected-runner-only
implementation. Confirm that no production resolver, standard runner, live
process, runtime composition, default registration, public surface, or
overstated containment claim was introduced.

Do not proceed to live execution, workflow composition, provider access,
OpenShell integration, or another profile before that review.
