# Trusted-Host Cargo Format Check Profile Plan Review

## 1. Executive Verdict

**Plan accepted after planning blocker corrections; proceed to the one-step
trusted-host Cargo format profile implementation.**

The selected profile is the right next no-provider runtime-composition step.
It exercises a real Rust formatter through one closed command contract while
keeping clippy, tests, arbitrary commands, providers, writes, public surfaces,
and topology expansion out of scope. Review found four security and honesty
gaps in the original plan. The plan now resolves them before implementation.

## 2. Scope Verification

The plan stays within a single unpublished trusted-host profile and an exact
one-step proof. It does not authorize another provider mutation, generic shell
or command execution, source-writing format mode, dependency or toolchain
installation, network access, clippy, tests, another multi-step topology,
public CLI/schema/SDK behavior, hosted execution, nested harnesses, or release
posture changes.

## 3. Profile Selection Assessment

`cargo fmt --all --check` is appropriately narrower than clippy or tests. Its
command kind and argument vector already exist, it does not intentionally
rewrite source, and it does not execute repository tests. The plan correctly
keeps it independent from the accepted two-step workflow until the new profile
has its own implementation and security review.

This selection does not make arbitrary Cargo commands safe and does not claim
that format success is sufficient engineering evidence.

## 4. Toolchain Identity Assessment

The original plan left Cargo-only hashing as an open possibility. That was a
blocker: `cargo fmt` dispatches additional formatter/toolchain components, so a
Cargo hash alone cannot identify the code that formats the repository.

The corrected plan requires the complete resolved Cargo, cargo-fmt, and
rustfmt executable chain, including final backing binaries behind any proxy,
to be contained, hashed, and included in the execution fingerprint. It also
binds toolchain selectors and repository Cargo/rustfmt configuration inputs.
Ambient `PATH`, user-home configuration, installation, update, and network
fallback remain forbidden.

## 5. Filesystem And Side-Effect Assessment

The existing side-effect model is adequate only for a pre-provisioned
toolchain that needs no declared cache, build, temp, or toolchain-state write.
The implementation must stop before execution if the selected real toolchain
cannot satisfy that posture. It may not silently broaden the model.

The original phrase "prove source-tree non-mutation" overstated what a
before/after commitment can establish without process isolation. The corrected
plan promises only protected final-state equivalence and detection of
persistent mutation. A write restored before the post-execution commitment is
not observable and must remain an explicit limitation.

The protected inventory includes ignored and untracked repository entries,
binds exact Core-owned exclusions, commits explicit read-only toolchain inputs,
and rejects symlink escape. Git status alone is insufficient.

## 6. Result And Failure Assessment

A zero process exit is not enough for success. Formatter failure, timeout,
process failure, persistent source/toolchain mutation, or integrity ambiguity
fails closed. A commitment mismatch is a distinct trusted-host integrity error
before result acceptance, not a failed formatter result. It therefore cannot
produce a successful local-check result, WorkReport, or artifact.

Errors remain stable and may not copy executable paths, configuration,
environment values, source names or contents, process output, credentials, or
toolchain details.

## 7. Runtime And Replay Assessment

The planned one-step path reuses immutable admission, operational execution
windows, opaque operation reservation, authoritative local-check results,
terminal reports, private artifacts, and exact replay. The host receives no
caller-selected arguments, environment, output locations, profile, or
topology. Restart after terminalization must execute no additional process and
append no events.

## 8. Test Plan Assessment

The corrected test plan covers:

- exact profile, skill, command, and fixed arguments;
- complete formatter-chain and configuration-input identity;
- missing toolchain/component behavior without install or network use;
- empty environment plus bounded synthesized toolchain context;
- repository and toolchain final-state commitments;
- additions, removals, content/type changes, ignored/untracked mutation, and
  symlink escape;
- integrity failure before local-check result acceptance;
- success, failure, timeout, lifecycle, report, and artifact behavior;
- exact replay without process or event duplication; and
- non-leaking Debug, errors, events, reports, and serialization.

Focused trusted-host tests and complete engineering-standard validation remain
required before implementation acceptance.

## 9. Planning Blockers Found And Corrected

### B1. Cargo hashing did not bind the invoked formatter

The plan now requires commitment of the complete resolved formatter chain and
configuration inputs.

### B2. Source non-mutation was overstated

The plan now distinguishes final-state equivalence and persistent-mutation
detection from process-level write prevention.

### B3. Ambient configuration and path resolution were underspecified

The plan now forbids user-home configuration and ambient `PATH`, binds
repository configuration, and requires symlink/root containment.

### B4. Toolchain-state side effects were unresolved

The plan now allows only an explicit read-only pre-provisioned toolchain. A
required write blocks implementation and requires separate model review.

## 10. Remaining Limitations

- There is no process sandbox or OS-enforced source-write denial.
- Transient writes restored before the post-execution commitment are not
  detectable.
- The profile remains unpublished and local to the trusted-host application.
- Toolchain availability is pre-provisioned; Workflow OS will not install or
  update it.
- Clippy, tests, arbitrary Cargo commands, providers, and external mutations
  remain unsupported by this phase.

## 11. Non-Blocking Follow-Ups

- Consider process-level filesystem containment only through a separate
  runtime/sandbox phase; do not imply it from metadata commitments.
- Keep the formatter profile out of the accepted two-step workflow until its
  implementation review passes.
- Revisit a narrow toolchain/cache write vocabulary only if the real pinned
  toolchain cannot operate read-only.

## 12. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Governed Review Record

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1792672479290624000-2`
- Approval ID:
  `approval/run-1792672479290624000-2/review-scope-approved`
- Presentation ID: `presentation/c693c42fefd90179`
- Approval outcome: granted under delegated-maintainer authority after review
  of the persisted complete handoff
- Phase status: `Completed`
- Out-of-kernel work: the delegated maintainer inspected plans, runtime models,
  and tests; corrected planning blockers; wrote this review; and ran validation
  outside the kernel under the approved scope.

## 14. Recommended Next Phase

Implement the one-step trusted-host Cargo format-check profile exactly as
corrected. Do not add clippy, tests, generic command execution, provider
access, source writes, public surfaces, hosted expansion, or another workflow
topology.
