# Trusted-Host Local Cargo Input Observer Source Report

## 1. Executive Summary

The reviewed fixed local Cargo input observer source is implemented privately
in the unpublished `workflow-local-host` crate. It converts explicit closed
local inputs into the existing Core-validated observation result through four
fixed read-only probes and one bounded repository-tree observation.

The implementation remains preflight evidence, not authority. It promotes no
fact to `Observed`, proves no admission-to-terminal execution continuity, and
is not composed into runtime execution.

## 2. Scope Completed

- Added canonical closed repository-root and Cargo-home inputs.
- Added an exact five-role executable set for Cargo, rustc, rustdoc,
  cargo-clippy, and clippy-driver.
- Added one explicit sanitized environment entry: `CARGO_HOME`.
- Added four fixed probes for rustc version identity, sysroot, target libdir,
  and locked offline Cargo metadata without dependencies.
- Added bounded repository-tree and executable commitments, pre/post identity
  checks, strict parsers, timeout handling, and process-group termination.
- Added process-free ordinary tests through an injected scripted probe runner.
- Added one ignored opt-in `aarch64-apple-darwin` live smoke using a disposable
  Cargo project.

## 3. Scope Explicitly Not Completed

This phase added no runtime composition, admission decision, Cargo clippy
execution, OpenShell integration, persistence, workflow or audit events,
schemas, CLI behavior, provider mutation, ambient environment access,
caller-selected commands or arguments, additional live platform, or release
posture change.

## 4. Typed Input And Source Boundary

`LocalCargoFixedProbeDefinition` canonicalizes a caller-supplied repository
root, Cargo home, and exact executable inventory before deriving a selection
commitment. Direct symlink inputs, missing executable roles, non-directories,
and non-regular executables fail closed.

The definition binds that closed selection to the existing observation
request's operation and security commitments. Debug output redacts roots,
paths, and the selection commitment. The fixed filesystem and process sources
remain private implementation details.

## 5. Fixed Probe Behavior

The standard runner can execute only these argument vectors:

- `rustc -vV`;
- `rustc --print sysroot`;
- `rustc --print target-libdir`; and
- `cargo metadata --locked --offline --format-version 1 --no-deps`.

It uses absolute executable paths, the exact canonical repository root as the
working directory, a cleared environment containing only the declared
`CARGO_HOME`, null stdin, and piped bounded stdout and stderr. It starts no
shell and accepts no caller-selected command, argument, or environment value
after definition construction.

## 6. Integrity, Timeout, And Parser Boundary

Each invoked executable is committed immediately before and after its probe.
The full executable set is also committed before and after collection, and
the repository tree is recomputed against the request commitment.

Probe streams are bounded to 256 KiB. The runner enforces a 15-second timeout
and terminates the Unix process group before joining output readers. Nonzero
status, any stderr, invalid UTF-8, empty output, malformed rustc identity,
non-absolute rustc paths, or malformed Cargo metadata fails closed with a
stable generic error.

Repository traversal rejects symlinks and non-regular entries and is bounded
to 20,000 files and 512 MiB. Individual executable commitments are bounded to
256 MiB.

## 7. Observation Posture

Toolchain, target, sysroot, native-tool, and package-source facts remain
`Uncertain`. Linker and environment facts remain `DeclaredOnly`. Dynamic
libraries and runtime base remain `Unsupported`. Root and fact integrity and
continuity remain uncertain.

The result therefore cannot authorize live execution admission. The
`IndependentlyReviewed` producer label continues to describe reviewed source
code, not independent machine attestation.

## 8. Privacy And Error Handling

Core receives commitments, counts, closed posture vocabulary, and bounded
references rather than raw paths, environment values, Cargo metadata, stdout,
stderr, source contents, registry locations, or credentials. Debug output for
the fixed definition redacts path-bearing fields.

Errors use stable generic codes and do not echo paths, probe output, Cargo
metadata, environment values, or executable contents.

## 9. Test Coverage

Process-free ordinary tests cover:

- complete exact-bound source construction through an injected runner;
- missing executable roles and symlinked executable rejection;
- strict success and failure parser shapes;
- repository and executable drift rejection; and
- Debug redaction.

The existing observer contract tests continue to cover exact request,
producer, revision, posture, fact, and evidence bindings. The ignored live
smoke proves the actual fixed process path on `aarch64-apple-darwin` without
making live execution part of the ordinary suite.

## 10. Commands And Results

- `cargo test -p workflow-local-host --lib`: passed; 18 passed and one ignored
  opt-in smoke.
- `cargo test -p workflow-local-host --lib opt_in_apple_silicon_fixed_probe_smoke -- --ignored --nocapture`:
  passed; one live smoke passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed. The first full run exposed that a
  target-specific `nix` dependency was outside the accepted fixture manifest
  shape; the dependency was made unconditional and the intentional repository
  snapshot count was updated before the passing full rerun.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Known Limitations

- The source supports only `aarch64-apple-darwin` for real probes.
- It is private and not composed into a runtime operation.
- No fact has same-call custody or admission-to-terminal continuity.
- Evidence references remain in memory and are not durable.
- Cargo clippy itself is not executed by this source.
- The fixed source does not authorize execution, provider mutation, or any
  external side effect.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the fixed source and opt-in
smoke. The review should verify typed selection closure, executable and root
identity, process-group timeout behavior, output and parser bounds, privacy,
ordinary process-free coverage, the live-smoke isolation, and every
non-`Observed` posture ceiling before any runtime composition is planned.

That focused review is complete and accepts the implementation with
non-blocking follow-ups. See the [review](TRUSTED_HOST_LOCAL_CARGO_INPUT_OBSERVER_SOURCE_REVIEW.md).

## 13. Governed Phase Evidence

- Workflow ID: `dg/implement`.
- Run ID: `run-1792844076534413000-2`.
- Approval ID:
  `approval/run-1792844076534413000-2/implementation-approved`.
- Approval presentation ID: `presentation/1e99d4c909d914df`.
- Approval presentation content hash:
  `1e99d4c909d914df086f3868aebb4f65e03b757550e98f92c4930df7bbae67e5`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact fixed-source implementation scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: focused process-free tests, the opt-in live smoke,
  workspace formatting, workspace clippy, workspace tests, documentation
  validation, and diff checks passed.
- Out-of-kernel work: Codex implemented the private source and tests, ran the
  live smoke, authored this report, and ran validation. The kernel governed
  scope and approval but did not inspect files, edit code or documentation,
  start probe processes, or execute validation commands.
