# Trusted-Host Cargo Format Check Profile Implementation Report

## 1. Executive Summary

Workflow OS now has a third closed trusted-host local-check profile for the
exact command `cargo fmt --all --check`. The profile is explicit, unpublished,
local, and one-step only. It binds an exact Cargo, cargo-fmt, and rustfmt
executable chain; commits protected repository final state; records the normal
local-check lifecycle; composes one terminal WorkReport and private artifact;
and replays terminal state without rerunning the formatter or duplicating
events.

The profile does not provide an operating-system sandbox. It detects persistent
protected-tree mutation by comparing complete pre/post commitments, including
ignored and untracked files, but cannot detect a transient write restored
before the post-execution commitment.

## 2. Scope Completed

- Added `ExplicitLocalCheckProfileId::CargoFmtCheck` and the canonical profile
  selector.
- Added a dedicated internal Cargo format handler for the fixed Core-owned
  command and timeout.
- Added explicit Cargo, cargo-fmt, rustfmt, toolchain-root, and repository-root
  resolution.
- Added a synthesized environment containing only the exact formatter-chain
  directories and exact `RUSTFMT` path.
- Added formatter-chain content identity and composite execution identity.
- Added protected repository final-state commitments with exact Core-owned
  exclusions.
- Added one explicit one-step local-host route through immutable admission,
  result recording, terminal report generation, artifact persistence, and
  exact replay.

## 3. Scope Explicitly Not Completed

This phase does not add clippy, tests, build commands, arbitrary Cargo
arguments, another workflow topology, source-writing format mode, dependency or
toolchain installation, network access, provider access, external writes,
public CLI behavior, schemas, SDK behavior, onboarding changes, hosted parity,
parallelism, branching, loops, nested harnesses, or release changes.

## 4. Profile And Handler API Summary

The caller selects `cargo_fmt_check` and supplies exact borrowed paths for:

- Cargo;
- cargo-fmt;
- rustfmt;
- the containing toolchain root; and
- the repository root.

The resolver accepts no caller-authored command text, arguments, environment,
output directory, network posture, or exclusions. The canonical skill is
`local/check-cargo-fmt@v0` and the command contract remains
`cargo fmt --all --check`.

## 5. Toolchain Identity Boundary

All three formatter executables must be regular executable files with exact
names inside the explicit canonical toolchain root. Their content hashes form
the formatter-chain commitment. Replacement of any executable after resolution
fails before process entry or before result acceptance with stable non-leaking
errors.

The repository commitment includes Cargo/rustfmt configuration under the
repository root. The handler does not search ambient `PATH`, consult caller
environment, install components, or use network fallback.

## 6. Filesystem Integrity Boundary

The handler commits all repository directories and files except the exact
Core-owned exclusions `.git`, `.workflow-os`, `dogfood/.workflow-os`, and
`target`. Ignored and untracked files remain protected. Symbolic links are
rejected. Addition, deletion, content change, and file-type change fail before
the process result can become authoritative.

This is final-state equivalence, not write prevention. A process that writes
and restores identical bytes between commitments is not observable. No report,
event, or error claims sandbox-enforced immutability.

## 7. Runtime, Report, Artifact, And Replay

The unpublished local-host binary accepts the closed profile only through
explicit arguments. A valid one-step workflow records one local-check result,
terminalizes, generates the existing WorkReport, and persists one private
artifact. Exact terminal replay returns the existing artifact posture without
rerunning Cargo or appending events.

## 8. Privacy And Error Posture

Debug output redacts executable paths, repository/toolchain roots, commitments,
and process-runner details. Integrity and resolution failures use stable codes
without copying paths, source names or contents, environment values, command
output, or toolchain details. Work reports cite bounded local-check references
and do not copy formatter output.

## 9. Test Coverage

Focused tests cover:

- canonical profile, skill, command, fixed arguments, and environment;
- explicit execute-only behavior;
- formatter replacement before process entry;
- source content mutation, addition, deletion, type change, and ignored/
  untracked addition;
- protected symlink rejection;
- persistent mutation before result acceptance;
- real bundled stable Cargo/cargo-fmt/rustfmt execution in an isolated repo;
- complete one-step local-host execution, report artifact persistence, and
  exact terminal replay; and
- all existing core, executor, adapter, report, artifact, SQLite, hosted, and
  runtime behavior through the full workspace suite.

## 10. Commands Run And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test local_check`: passed, 77 tests; two
  opt-in live smokes ignored by default.
- opt-in real Cargo format smoke: passed against the bundled stable formatter
  chain.
- `cargo test -p workflow-local-host --test foreground_admission`: passed, 28
  tests.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `git diff --check`: passed.

The dedicated Cargo format live smoke was explicitly enabled and passed. Other
environment- or credential-dependent live provider tests remained ignored by
their existing opt-in contracts; no provider operation was required or
simulated for this phase.

## 11. Remaining Known Limitations

- No process sandbox or kernel-enforced filesystem write denial exists.
- Transient restored writes are not detectable.
- Executable-chain identity does not claim independent third-party attestation.
- The toolchain must already exist and operate without declared writes.
- The profile is unpublished and is not yet part of the accepted two-step
  workflow.

## 12. Governed Implementation Record

- Workflow ID: `dg/implement`
- Run ID: `run-1792673289805744000-2`
- Approval ID:
  `approval/run-1792673289805744000-2/implementation-approved`
- Presentation ID: `presentation/ad244b22439de15c`
- Approval outcome: granted under delegated-maintainer authority for the exact
  bounded implementation scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof marker
- Out-of-kernel work: source edits, process execution, tests, report writing,
  and git operations were performed by the delegated maintainer; the kernel
  governed scope and approval but did not perform those actions.

## 13. Recommended Next Phase

Perform a focused maintainer/security review of this implementation. Do not add
clippy, tests, generic command execution, provider access, external writes,
public surfaces, hosted expansion, or another topology before that review.
