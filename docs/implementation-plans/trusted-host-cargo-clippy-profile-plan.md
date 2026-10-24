# Trusted-Host Cargo Clippy Profile Plan

Status: injected-runner-only implementation complete; focused
maintainer/security implementation review required.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Broader Local Check Handler Plan](broader-local-check-handler-plan.md)
- [Local Check Side-Effect Boundary Plan](local-check-side-effect-boundary-plan.md)
- [Trusted-Host Cargo Format Check Profile Plan](trusted-host-cargo-fmt-profile-plan.md)
- [Trusted-Host Cargo Format Workflow Composition Plan](trusted-host-cargo-fmt-workflow-composition-plan.md)
- [Trusted-Host Terminal WorkReport Artifact Plan](trusted-host-terminal-work-report-artifact-plan.md)
- [OpenShell Optional No-Write Execution Provider Report](../concepts/OPENSHELL_OPTIONAL_NO_WRITE_EXECUTION_PROVIDER_REPORT.md)

## 1. Executive Summary

Workflow OS should add one closed trusted-host profile for exactly:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

This is a meaningful runtime-depth milestone because it converts a required
engineering command from out-of-kernel disclosure into a governed local check
with immutable input binding, explicit build/cache write posture, bounded
results, durable citations, and honest replay.

Clippy is not merely a read-only formatter with different output. Cargo may
compile and execute build scripts and procedural macros, invoke additional
toolchain components, and write dependency, build, incremental, and temporary
state. The first implementation must therefore prove the closed profile and
handler through an injected process runner only. A real-process smoke requires
a separate accepted security review and explicit trusted-host risk posture.

This plan does not authorize arbitrary Cargo commands, automatic local checks,
default registration, public CLI or schema exposure, provider writes, hosted
execution, or another check profile.

## 2. Selection Decision

Select one existing command vocabulary entry:

```text
profile: cargo_clippy_workspace
command kind: CargoClippyWorkspace
skill: local/check-cargo-clippy@v0
logical command: cargo clippy --workspace --all-targets -- -D warnings
```

Add `CargoClippyWorkspace` to the closed explicit profile identity and provide
only its matching resolver. Do not accept caller-supplied subcommands,
arguments, package filters, features, targets, output paths, environment
overrides, or network posture.

The command identity is already modeled. Existing command vocabulary does not
authorize execution by itself.

## 3. Goals

- Add one closed `CargoClippyWorkspace` profile.
- Bind the exact command without invoking a shell.
- Bind the repository, immutable run bundle, source/configuration inventory,
  Rust toolchain, dependency inputs, writable build/cache/temp roots, sanitized
  environment, timeout, and output bounds before process entry.
- Classify expected writes explicitly without authorizing source writes.
- Keep Cargo dependency resolution offline and fail closed when required local
  inputs are unavailable.
- Produce one payload-free local-check result and existing invocation lifecycle.
- Support existing terminal WorkReport and private artifact citations without
  storing compiler output.
- Preserve restart-safe terminal replay without rerunning clippy.
- Keep the existing project-validation, docs-check, Cargo-format, and
  three-step workflow behavior unchanged.

## 4. Strict Non-Goals

This phase must not add:

- arbitrary Cargo, Rust, shell, package-manager, or user-defined commands;
- `cargo test`, `cargo build`, source-writing format mode, or another profile;
- caller-selected arguments, packages, features, targets, or environment;
- automatic or default local-check registration or execution;
- public CLI, workflow schema, SDK, onboarding, or example exposure;
- network access, dependency download, toolchain installation, or update;
- source, manifest, lockfile, configuration, or generated-source writes;
- command-output evidence or raw stdout/stderr persistence;
- provider mutation, credential loading, external writes, or report delivery;
- OpenShell installation, selection, fork, or live integration;
- hosted/distributed execution, nested harnesses, or enterprise administration;
  or
- production-sandbox or production-readiness claims.

## 5. Trusted-Host Threat Model

The profile is for an explicitly trusted repository on an explicitly trusted
local host. That designation is a disclosed risk posture, not a containment
claim.

Unlike Cargo format, clippy can cause Cargo to execute repository or dependency
build scripts and procedural macros. Those programs may attempt filesystem,
network, process, or environment access outside the command's intended build
outputs. An empty/sanitized environment and Cargo offline mode reduce ambient
authority but do not provide operating-system isolation.

The implementation must therefore state all of the following:

- fixed command identity is not process containment;
- `CARGO_NET_OFFLINE=true` does not prevent build-time code from opening its
  own network connection;
- pre/post commitments detect persistent protected-state mutation but cannot
  prevent or detect a transient write restored before the final commitment;
- removal of secret-bearing environment variables does not prevent reads of
  other host files available to the process identity; and
- a future sandbox-backed provider may strengthen these controls, but the
  incomplete OpenShell integration is not silently treated as present.

The first implementation is consequently injected-runner only. A later opt-in
real smoke must be separately approved and remain limited to the Workflow OS
repository and a pre-provisioned toolchain.

## 6. Closed Contract And Handler

Add the smallest matching API surface:

- `ExplicitLocalCheckProfileId::CargoClippyWorkspace`;
- `ExplicitLocalCheckProfileSelection::cargo_clippy_workspace()`; and
- `CargoClippyWorkspaceLocalHandler` behind the existing authoritative local
  check boundary.

The handler constructor should accept only:

- explicit Cargo, cargo-clippy, clippy-driver, rustc, and rustdoc executable
  paths plus every additional Core-supported toolchain executable that the
  reviewed invocation resolves;
- one canonical toolchain root;
- the exact repository root;
- one explicit Cargo dependency/cache root;
- one explicit target/build-output root;
- one explicit temporary root;
- an injected process runner; and
- Core-owned timeout and output bounds.

The implementation review must reject any fallback to ambient `PATH`, user
home discovery, shell text, optional extra arguments, or inferred writable
directories.

## 7. Immutable Input And Toolchain Binding

Before process entry, the resolved profile must commit:

- canonical profile, command, skill, and immutable run-bundle identity;
- repository root and protected repository inventory;
- workspace root manifest, workspace-member manifests, `Cargo.lock`,
  `rust-toolchain.toml`, `rust-toolchain`, `.cargo/config.toml`,
  `.cargo/config`, and clippy configuration when present;
- source files and build scripts inside the protected repository inventory;
- the exact local dependency package inventory selected from the committed
  lockfile;
- Cargo, cargo-clippy, clippy-driver, rustc, rustdoc, and every supported
  executable in the resolved toolchain chain;
- the explicit dependency/cache, target, and temporary roots and their
  pre-execution posture; and
- the complete sanitized environment and timeout/output policy.

Toolchain and dependency inputs must be canonical, regular files under their
declared roots, content committed, and disjoint from the protected repository
and writable output roots. Repository/toolchain/cache overlap in either
direction fails before process entry.

If the implementation cannot enumerate an executable or dependency input that
materially affects the invocation, it must stop before process entry rather
than record an incomplete execution identity.

## 8. Writable Directory Policy

The profile uses `LocalCheckSideEffectClass::BuildOrCacheWrites` and a validated
side-effect boundary containing:

- `SourceReadOnly`;
- `CacheWriteOnly`;
- `BuildOutputWrite`; and
- `TempWriteOnly` only if the model is deliberately extended and separately
  validated for the explicit temporary root.

The current contract derives cache and build permissions from one undifferenced
directory list. Implementation should add a typed internal directory-role
mapping or equivalent validated representation so the Cargo cache, target, and
temporary roots cannot be substituted for one another. This is an internal
closed-profile boundary, not a public schema change.

Writable roots must be canonical, explicit, outside the protected repository,
outside the toolchain root, and mutually disjoint. The handler must not infer
`target/`, `$HOME/.cargo`, `/tmp`, or another ambient path.

The repository's current `.tools` directory must not become writable merely
because it contains pre-provisioned build dependencies or toolchains.

## 9. Process, Environment, And Network Policy

Construct one executable-plus-arguments request. Never invoke a shell.

Start from an empty environment. Permit only fixed values required by the
reviewed profile, including explicit `CARGO_HOME`, `RUSTUP_HOME`,
`CARGO_TARGET_DIR`, `TMPDIR`, synthesized toolchain-only `PATH`, and
`CARGO_NET_OFFLINE=true` when required. Reject compiler wrappers, rust flags,
target overrides, registry/proxy settings, provider credentials, authorization
headers, and inherited user environment.

The process runner must enforce Core-owned timeout and bounded stdout/stderr
capture. Timeout must terminate the full process group where the supported
runner can prove that behavior. Uncertain descendant-process termination is an
internal failure, not a normal clippy failure.

Network remains contractually disabled. The trusted-host implementation must
not claim operating-system egress denial. A later sandbox-backed execution
provider can strengthen this boundary only after its attestation contract and
live proof are accepted.

## 10. Proportional Governance And Authority

The first real-process use, if later approved, must require visible blocking
approval because it executes build-time code and grants bounded local write
authority. Quiet capture is not appropriate for the initial trusted-host
proof.

Approval context must identify:

- the exact profile and immutable run bundle;
- the trusted repository and toolchain commitments;
- the three writable roots and their roles;
- the lack of OS filesystem/network/process isolation;
- the evidence and report obligations;
- the timeout and replay posture; and
- the fact that approval authorizes this check only, not arbitrary Cargo work.

Existing approval-presentation proof and current-authority reassessment remain
required where the profile enters an approval-bound runtime path. A credential,
installed toolchain, or successful prior run is not reusable authority.

## 11. Execution And Workflow Boundary

The implementation phase should prove one selected profile through existing
private trusted-host application and executor primitives. It must not add a
fourth step to the accepted three-step workflow in the same phase.

Required ordering for an attempted check:

1. validate and commit the immutable run and profile inputs;
2. validate current authority and any required approval presentation;
3. persist dispatch/executor commitments through existing atomic boundaries;
4. snapshot protected repository and read-only toolchain/dependency inputs;
5. invoke the exact process once;
6. terminate or reconcile the complete process group;
7. verify protected inputs and allowed writable-root boundaries;
8. persist the payload-free result and terminal invocation relationship; and
9. derive report and artifact material through existing accepted paths.

No failure after process entry may trigger an automatic rerun. Ambiguous
process or persistence outcomes require reconciliation.

## 12. Result, Evidence, Report, And Artifact Policy

Reuse the existing payload-free local-check result record and stable result
reference. The result may contain only bounded identity, terminal status,
causal event relationships, and commitments already accepted by the local
check model.

Do not persist or cite:

- raw compiler diagnostics;
- stdout/stderr or command transcripts;
- source snippets or file contents;
- absolute local paths;
- environment values;
- dependency or toolchain payloads; or
- credentials or token-like values.

WorkReports and private artifacts may cite the stable local-check result and
disclose passed, failed, timed out, or internal-integrity-failure posture. They
must distinguish a clippy diagnostic failure from a boundary/integrity failure
without copying diagnostic text.

## 13. Failure And Replay Semantics

- Non-zero clippy exit produces a failed local-check result and normal governed
  workflow failure.
- Timeout produces `TimedOut` only after bounded termination is established.
- Pre-entry identity, authority, directory, environment, or input-integrity
  failure produces no process invocation and no fabricated check result.
- Post-entry protected-state drift or output-boundary violation fails the
  operation and cannot be reported as a successful clippy result.
- Persistence ambiguity never reruns clippy automatically.
- Exact terminal replay returns the existing result, report, and artifact
  posture without process entry.
- Changed source, lockfile, profile, toolchain, dependency inventory,
  environment, output root, or authority commitment is a new governed attempt,
  not replay.

All errors use stable codes and must not echo paths, output, source, manifests,
environment values, dependency names when sensitive, or toolchain payloads.

## 14. Test Plan

The implementation phase must add focused tests for:

- exact profile selection and resolver mismatch;
- exact command and argument construction without a shell;
- every bound executable and configuration input affecting identity;
- repository/toolchain/cache/target/temp overlap in both directions;
- missing or changed toolchain and dependency inputs;
- empty-environment construction and forbidden ambient variables;
- explicit offline Cargo posture;
- typed writable-root role substitution;
- source, manifest, lockfile, and toolchain mutation detection;
- permitted cache/build/temp mutation acceptance;
- non-zero, timeout, output-bound, and process-group uncertainty behavior;
- no automatic retry after process entry;
- payload-free events, result records, Debug, errors, reports, and artifacts;
- terminal replay without rerunning the process;
- existing one-step, two-step, and three-step profile compatibility; and
- no public registration, CLI, schema, provider, or default behavior.

An eventual opt-in live smoke must be separately reviewed and must prove the
exact bundled toolchain and Workflow OS repository only. Ordinary CI remains
network-free and does not run the live smoke.

## 15. Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Add the closed profile ID, exact contract, typed internal writable-root
   roles, resolver, handler, and injected-runner tests.
3. Review that implementation before enabling a standard process runner.
4. Plan and review one ignored opt-in live Workflow OS repository smoke,
   including the explicit trusted-host risk disclosure.
5. Only after the smoke review, plan whether to compose clippy into a larger
   private workflow.
6. Keep OpenShell-backed containment, default registration, public exposure,
   and other check families separately governed.

## 16. Planning Risks And Open Questions

- The complete executable chain may include platform compiler/linker tools or
  SDK material outside the Rust toolchain root.
- Build scripts and procedural macros may invoke tools that Core cannot
  enumerate statically.
- Cargo may require writable cache metadata even in offline mode.
- The existing side-effect boundary does not associate directory roles with
  individual paths.
- Process-group termination behavior varies by platform.
- Final-state commitments do not prove absence of transient writes.
- Current OpenShell integration lacks the accepted live attestation surface
  needed to substitute sandbox enforcement silently.

The focused plan review must classify the first two items as either bounded
trusted-host limitations or blockers before implementation. No implementation
may simply omit them from the execution identity.

## 17. Validation For Planning

Run:

```text
npm run check:docs
git diff --check
```

Rust formatting, clippy, and tests are not required for this documentation-only
phase. They become mandatory for implementation and review.

## 18. Final Recommendation

Proceed next to a focused maintainer/security review of this plan.

If the review accepts the explicit trusted-host limitations, implement only the
closed profile and injected-runner proof. Do not add a live process smoke,
workflow composition, arbitrary command execution, or public/default exposure
in the same phase.

## 19. Governed Planning Record

- workflow: `dg/d`
- run: `run-1792801477137949000-2`
- approval: `approval/run-1792801477137949000-2/planning-approved`
- presentation: `presentation/ce8e615bc54886e5`
- presentation content hash:
  `ce8e615bc54886e565529f445ed19813e86725690c6d358ec1adb59f38a5140b`
- approval outcome: granted under delegated-maintainer authority for the exact
  persisted planning scope
- close status: `Completed`
- event summary: 39 events, one approval, zero retries, and zero escalations
- presentation enforcement: `proof_enforced`, with one persisted presentation
  record and the approval-event proof marker present

The kernel governed scope, approval, and phase history. Codex inspected the
repository and prior plans, authored this plan and roadmap update, ran docs and
diff checks, and performed Git operations outside the kernel. No Rust check
handler, process, provider, report artifact, or external write was executed by
the governed planning workflow.
