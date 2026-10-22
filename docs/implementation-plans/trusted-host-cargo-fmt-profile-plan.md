# Trusted-Host Cargo Format Check Profile Plan

Status: implementation review found blockers; a narrow blocker fix is next.
The closed no-write trusted-host Cargo format profile is implemented with exact
three-executable binding, protected repository final-state commitments,
one-step terminal report/artifact composition, and restart-safe replay. Before
acceptance it must commit the complete bounded toolchain input and exclusion
policy boundary and complete the required focused regression matrix. It
remains unpublished and does not authorize broader Cargo commands or topology.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Local Check Side-Effect Boundary Plan](local-check-side-effect-boundary-plan.md)
- [Trusted-Host Project Validation Profile Plan](trusted-host-project-validation-profile-plan.md)
- [Trusted-Host Two-Step No-Write Workflow Plan](trusted-host-two-step-no-write-workflow-plan.md)
- [Trusted-Host Two-Step No-Write Workflow Review](../concepts/TRUSTED_HOST_TWO_STEP_NO_WRITE_WORKFLOW_REVIEW.md)

## 1. Executive Summary

The trusted-host application now proves two closed local-check profiles and one
exact two-step workflow. The next runtime-composition step should add a third
closed profile for the already modeled `cargo fmt --all --check` command.

This is the smallest useful expansion toward independent engineering-check
verification. It exercises a real Rust toolchain while remaining narrower than
clippy or tests: it does not compile code, execute repository tests, require
network access, or intentionally write source files. The kernel must still
treat toolchain and filesystem behavior as governed execution, not assume that
the command is harmless because its conventional intent is read-only.

The implementation should add one exact Cargo-format profile and one
independent one-step trusted-host proof. It should not add that profile to the
accepted two-step workflow in the same phase. Workflow broadening follows only
after the profile itself is reviewed.

## 2. Selection Decision

Select:

```text
profile: cargo_fmt_check
command kind: cargo_fmt_check
skill: local/check-cargo-fmt@v0
logical command: cargo fmt --all --check
```

Reuse `LocalCheckCommandKind::CargoFmtCheck` and its canonical fixed argument
template. Add only the closed profile, handler, immutable declaration, and
trusted-host wiring needed for this exact command.

Do not select `CargoClippyWorkspace` or `CargoTestWorkspace` yet. Both write
build outputs, clippy compiles workspace targets, and tests execute repository
code. They require a separately reviewed build-output and test-execution
policy.

## 3. Goals

- Add one closed `CargoFmtCheck` explicit local-check profile.
- Execute only `cargo fmt --all --check` through an executable-plus-arguments
  process request, never a shell.
- Bind the supplied Cargo executable, resolved rustfmt component posture,
  repository root, fixed arguments, sanitized environment, timeout, and
  side-effect boundary before process entry.
- Keep network disabled and reject ambient registry or provider authority.
- Prove protected source-tree final-state equivalence across the check boundary
  and disclose that this does not prevent or observe a transient write that is
  restored before the post-execution commitment.
- Record one bounded payload-free local-check result and normal invocation
  lifecycle.
- Compose the existing terminal report and private artifact for the one-step
  proof.
- Preserve exact replay without rerunning the formatter.
- Keep project validation, docs check, and the accepted two-step workflow
  unchanged.

## 4. Strict Non-Goals

This phase must not add:

- arbitrary Cargo subcommands, arguments, packages, feature flags, or targets;
- cargo clippy, cargo test, cargo build, TypeScript, integration, or provider
  profiles;
- rustfmt source-writing mode;
- dependency installation, toolchain installation, update checks, or network
  access;
- inherited user environment or registry configuration;
- source, lockfile, manifest, generated-code, or project-metadata writes;
- caller-selected cache, build, or temporary write locations;
- another multi-step topology or modification of the accepted two-step shape;
- providers, external writes, automatic approval, or delegated-authority
  broadening;
- released CLI behavior, schemas, SDKs, onboarding changes, or examples;
- hosted parity, parallelism, branching, loops, or nested harnesses; or
- release-posture changes.

## 5. Why Cargo Format Is The Third Profile

The current proof should broaden independent verification before broadening
topology again. Cargo format is the narrowest missing engineering check:

- its command kind and canonical argument vector already exist;
- `--check` reports drift without intentionally rewriting source;
- it does not execute project tests or provider code;
- it does not require dependency resolution or network access;
- it introduces an explicit Rust toolchain boundary that later clippy and test
  profiles will also need; and
- a failed format check is a concrete workflow failure with no need to retain
  raw formatter output.

This does not establish that all Cargo commands are safe or that format success
is sufficient engineering evidence.

## 6. Closed Profile And Handler

Extend the closed profile vocabulary with:

- `ExplicitLocalCheckProfileId::CargoFmtCheck`;
- `ExplicitLocalCheckProfileSelection::cargo_fmt_check()`; and
- a dedicated `CargoFmtCheckLocalHandler` implementing the existing
  authoritative local-check handler boundary.

The handler constructor should accept only:

- explicit Cargo, cargo-fmt, and rustfmt executable paths, or an equivalently
  complete resolved formatter chain whose final backing executables are known;
- the exact repository root;
- an explicit read-only toolchain-state root and exact toolchain selector
  required to invoke the pinned rustfmt component;
- an injected process runner for deterministic tests; and
- fixed Core-owned timeout and output bounds.

It must not accept command text, extra arguments, package filters, features,
environment overrides, a network posture, or output paths.

## 7. Toolchain Identity And Availability

The trusted-host boundary must not search ambient `PATH` or install a missing
component. Before process entry it should prove:

- every executable in the resolved Cargo-to-rustfmt chain is a regular file
  contained by the explicit toolchain root;
- the content hash of Cargo, cargo-fmt, rustfmt, and any executable proxy or
  backing binary actually used is included in the resolved execution
  fingerprint;
- the selected invocation resolves only the committed rustfmt component from
  the explicitly supplied local toolchain context;
- the toolchain selector and any repository `rust-toolchain.toml`,
  `rust-toolchain`, `rustfmt.toml`, `.rustfmt.toml`, `.cargo/config.toml`, or
  `.cargo/config` inputs that affect resolution or formatting are present in
  the immutable run/source commitment;
- no install or update action is attempted; and
- replacement of the executable or bound toolchain material fails before
  execution.

Cargo executable hashing alone is not sufficient. The implementation must bind
the complete resolved formatter chain, including the final rustfmt executable.
Rustup proxies are acceptable only when Core resolves and commits the exact
backing toolchain material before process entry. If that cannot be done without
ambient lookup or mutation, the implementation must stop before process entry.

## 8. Environment And Network Boundary

Build the process environment from an empty map. Permit only fixed,
non-sensitive entries required for the explicit local toolchain, such as a
bounded `CARGO_HOME` or `RUSTUP_HOME` when the selected executable requires
them. Do not inherit:

- provider or registry access values;
- proxy settings;
- arbitrary `PATH` entries;
- compiler wrappers;
- Rust flags;
- target-directory overrides; or
- user Cargo configuration overrides.

Any required `PATH` must be synthesized only from directories containing the
already committed executable chain. The implementation must not consult user
home Cargo/rustup configuration. Repository-local configuration that affects
the invocation must be inside the protected inventory and immutable run
commitment; unsupported configuration fails before process entry.

Network posture remains disabled. A missing local toolchain or rustfmt
component fails closed with a stable non-leaking error; the handler must not
download it.

## 9. Filesystem And Side-Effect Boundary

The profile derives a validated `LocalCheckSideEffectBoundary` with:

- source read access to the exact repository root;
- source writes forbidden;
- network access forbidden;
- no declared build-output write requirement; and
- one explicit read-only toolchain-state input outside the repository root.

The existing model is sufficient only if the selected pre-provisioned
toolchain can run without any declared cache, build, temp, or toolchain-state
write. If a real invocation requires such a write, implementation must stop and
return to a separately reviewed narrow side-effect-model extension. It must not
reclassify the profile opportunistically.

Because process isolation is not yet implemented, the first trusted-host proof
must compare protected final state rather than claim sandbox prevention.
Capture a bounded metadata commitment over protected repository files and the
explicit read-only toolchain inputs immediately before and after execution.
Any observed addition, removal, type change, content change, or committed
toolchain/config change fails the operation and must not be reported as a
successful format check.

This comparison detects persistent mutations only. It does not prove that the
process never wrote and restored a file between commitments. Documentation,
events, reports, and review must preserve that limitation until process-level
filesystem isolation or equivalent write denial exists.

The commitment must exclude accepted runtime state and explicitly bound local
toolchain/cache roots outside the protected source inventory. It must not copy
file contents into events, results, reports, or errors. The implementation
review must reject a source-mutation check that is based only on git status,
because ignored and untracked protected files are still part of the boundary.
Inventory traversal must reject symlinks or other path resolution that escapes
the canonical repository or toolchain root. Exclusions must be exact,
Core-owned, and included in the resolved execution fingerprint; callers cannot
add exclusions.

## 10. Workflow And Runtime Shape

The first implementation proves one immutable one-step workflow whose only
step references `local/check-cargo-fmt@v0` and the canonical Cargo-format
declaration.

Core must preserve the existing trusted-host sequence:

1. validate project and immutable workflow;
2. create or rehydrate the exact immutable run bundle;
3. resolve current authority and required context for the closed no-provider
   profile;
4. open the operational execution window;
5. reserve and dispatch one opaque operation;
6. execute the exact handler once;
7. record one payload-free local-check result and invocation outcome;
8. terminalize the run;
9. compose the terminal WorkReport; and
10. persist the private report artifact through the accepted gate.

Exact restart replay must not rerun Cargo or append duplicate events.

## 11. Result, Evidence, And Reporting

The existing `LocalCheckResult` remains authoritative check truth. Only a
passed result may produce skill success. Failed, timed-out, persistently
source-mutating, or internal handler outcomes fail the invocation with stable
non-leaking codes.

A final-state commitment mismatch is a distinct trusted-host integrity error,
not a formatter result. A process exit of zero cannot become a successful
`LocalCheckResult` or report when the integrity comparison fails.

Do not persist stdout, stderr, command transcripts, source paths, Cargo config,
or toolchain paths. The WorkReport should cite the stable local-check result
reference and disclose the Cargo-format posture without copying output.

This phase does not create `EvidenceReference` values implicitly and does not
claim independent third-party attestation.

## 12. Failure Semantics

Fail before process entry for:

- unknown or mixed profile arguments;
- missing or substituted Cargo executable;
- unavailable local rustfmt component;
- repository-root mismatch;
- unclassified side-effect posture;
- non-empty authority or context inventories; or
- malformed immutable declaration.

Fail after process entry for:

- non-zero exit;
- timeout;
- process failure;
- rejected or secret-like bounded output; or
- protected source-tree mutation.

Errors must use stable codes and never include raw paths, environment values,
source names, command output, auth material, or toolchain configuration.

## 13. Test Plan

Focused tests should prove:

- canonical profile and handler identity;
- exact fixed command arguments;
- deterministic injected-runner success, failure, and timeout;
- explicit executable and repository-root binding;
- complete Cargo/cargo-fmt/rustfmt chain and configuration-input binding;
- missing component fails without installation or network use;
- environment starts empty and rejects ambient or sensitive entries;
- source-tree addition, deletion, content mutation, and type change fail closed;
- ignored and untracked protected-file mutation is detected;
- symlink escape and caller-selected exclusion attempts fail before execution;
- persistent toolchain-state mutation fails closed;
- runtime-state and explicit external toolchain roots are not misclassified as
  source mutation;
- success records one result and one invocation lifecycle;
- failure schedules no later work and produces no success report;
- exact replay performs zero additional process invocations and appends no
  events;
- report and artifact cite only bounded stable references;
- Debug, errors, events, reports, and serialization do not leak paths or
  command output; and
- existing project-validation, docs-check, two-step, immutable-bundle,
  authority, report, artifact, SQLite, and runtime tests remain unchanged.

Before phase acceptance, run the focused trusted-host suite and the complete
workspace validation required by the engineering standard.

## 14. Implementation Sequence

1. Perform focused maintainer/security review of this plan.
2. Add the closed profile and canonical handler with injected-runner tests.
3. Add explicit toolchain identity and no-install availability checks.
4. Add protected source-tree pre/post commitment and mutation tests.
5. Wire one one-step trusted-host path through existing admission, lifecycle,
   report, and artifact boundaries.
6. Run focused and workspace validation.
7. Perform focused implementation/security review.
8. Only after acceptance, decide whether to add the profile to a larger
   sequential workflow.

## 15. Review Decisions

- Bind the complete resolved Cargo/cargo-fmt/rustfmt executable chain and every
  repository configuration input that can affect formatting. Cargo hashing
  alone is insufficient.
- Use the existing source-read-only side-effect model only when a
  pre-provisioned toolchain needs no declared writes. Any required write is a
  blocker requiring a separately reviewed model extension.
- Inventory all repository entries, including ignored and untracked files,
  with exact Core-owned exclusions for accepted runtime state. Reject symlink
  escape and include the inventory/exclusion policy in the execution
  fingerprint.
- Commit the explicit read-only toolchain inputs before and after execution as
  well as protected repository state.
- Treat commitment mismatch as a distinct trusted-host integrity failure before
  accepting a local-check result.
- Describe this mechanism as persistent-mutation detection and final-state
  equivalence, never process-level write prevention.

## 16. Final Recommendation

Perform focused maintainer/security review of the implemented one-step
`CargoFmtCheck` trusted-host profile. Do not add the profile to a larger
sequential workflow until that review accepts its toolchain identity,
final-state integrity, result/report behavior, replay, and explicit transient-
write limitation.

## 17. Governed Planning Record

- workflow: `dg/d`
- run ID: `run-1792671637228194000-2`
- approval ID: `approval/run-1792671637228194000-2/planning-approved`
- presentation ID: `presentation/5f14afe001dff87c`
- approval outcome: granted under delegated-maintainer authority after review
  of the persisted complete handoff
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof marker
- validation: `npm run check:docs` passed; `git diff --check` passed
- out-of-kernel work: source and accepted-plan inspection plus documentation
  edits and validation were performed by the delegated maintainer; the kernel
  governed scope and approval but did not edit files, run checks, or mutate git
