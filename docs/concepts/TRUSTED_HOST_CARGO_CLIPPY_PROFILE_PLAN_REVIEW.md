# Trusted-Host Cargo Clippy Profile Plan Review

## 1. Executive Verdict

**Plan accepted with mandatory implementation constraints; proceed to the
injected-runner-only closed Cargo clippy profile implementation.**

The selected profile is the right next runtime-depth step. It binds one exact
engineering command while forcing build/cache/temp writes, toolchain and
dependency identity, build-time code execution, authority, replay, and report
truth into the governed boundary. The plan is appropriately explicit that a
fixed command and trusted host are not operating-system containment.

Implementation may proceed only as a non-live proof. It must not add a
standard-process constructor or expose a resolver that external callers can
pair with an arbitrary process-running implementation. Build scripts,
procedural macros, platform tools, transient restored writes, host-file reads,
and non-enforced egress remain blockers for a later real-process phase.

## 2. Scope Verification

The plan stays within one closed Cargo clippy profile and an injected-runner
proof. It does not authorize a live clippy process, Cargo test or build, generic
Cargo or shell execution, caller-selected arguments or environment, default
registration, automatic checks, public CLI/schema/SDK exposure, provider
writes, hosted execution, OpenShell integration, workflow composition, or
another check family.

The profile remains a private trusted-host runtime primitive. Its existence
does not make the command safe for untrusted repositories and does not
authorize source, manifest, lockfile, configuration, or generated-source
writes.

## 3. Exact Profile And Command Assessment

The exact command is sufficiently closed:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

The plan rejects package, feature, target, argument, environment, network, and
output-path overrides. The profile has one identity, command kind, skill
identity, and fixed argument vector. No shell is involved.

This is materially different from arbitrary Cargo execution. The
implementation must preserve that distinction in constructors, matching,
Debug output, errors, serialization, and tests.

## 4. Injected Runner And API Boundary Assessment

`LocalCheckProcessRunner` is a public exported trait. The standard host runner
is private, but a public clippy resolver accepting `Arc<dyn
LocalCheckProcessRunner>` would still let an external caller supply an
implementation that starts a real process. Calling that API
"injected-runner-only" would therefore be an authority claim the type boundary
does not enforce.

The accepted implementation must use one of these equivalent narrow shapes:

- a crate-private clippy resolver and handler constructor exercised by module
  tests; or
- an explicitly test-only constructor that cannot be reached from a normal
  production profile-resolution path.

It must not add a clippy equivalent of `resolve_cargo_fmt_check(...)`, must not
select `StdLocalCheckProcessRunner`, and must not export a production clippy
handler capable of process entry. Public profile vocabulary may be added only
if it remains non-executable without a later separately reviewed authority
boundary.

This is a mandatory implementation acceptance condition, not a non-blocking
follow-up.

## 5. Build-Time Code And Security Assessment

Clippy may compile and execute repository and dependency build scripts and
procedural macros. Those programs can attempt filesystem, network, process,
and host-file access beyond Cargo's intended output roots. Platform linkers,
SDK tools, compiler wrappers, and build-script-spawned tools may also affect
execution without being statically enumerable.

For the injected proof, these risks can be represented through explicit fake
inventories and fail-closed tests without executing code. They are not
blockers for that proof.

They are blockers for a later real-process phase unless that phase either:

- proves a complete executable and dependency identity for the selected
  repository and toolchain; or
- introduces separately accepted containment and attestation strong enough to
  replace complete static enumeration.

The implementation report must not imply that offline Cargo mode, an empty
environment, fixed arguments, or final-state commitments mitigate these risks
as OS controls.

## 6. Toolchain And Dependency Identity Assessment

The plan correctly binds Cargo, cargo-clippy, clippy-driver, rustc, rustdoc,
supported toolchain executables, repository configuration, lockfile-selected
dependency inputs, and immutable source/build-script inventory. Canonical
regular-file containment, content commitments, and bidirectional root-overlap
rejection are appropriate.

For the injected proof, every identity input must be supplied explicitly by
the fixture and reproduced deterministically. Missing or unenumerated material
must fail before the runner is invoked. The implementation may not silently
fall back to ambient `PATH`, user-home configuration, rustup discovery,
dependency download, toolchain installation, or inferred roots.

## 7. Writable Directory And Integrity Assessment

`BuildOrCacheWrites` is appropriate only with typed per-root roles. An
undifferentiated writable-directory list is insufficient because Cargo home,
target output, and temporary state have different authority and replay
meaning. The internal model must distinguish at least cache, build output, and
temporary roots and reject role substitution.

All writable roots must be explicit, canonical, mutually disjoint, outside
the protected repository, and outside read-only toolchain/dependency roots.
The repository `.tools` directory must not become writable by implication.

Pre/post commitments can detect persistent protected-state drift. They cannot
prevent writes, observe restored transient writes, or stop reads of accessible
host files. Tests and docs must retain that limitation.

## 8. Environment, Network, Timeout, And Process Assessment

The empty-environment posture, explicit Cargo/Rustup/target/temp roots,
synthesized toolchain-only `PATH`, Cargo offline setting, bounded output, and
Core-owned timeout are appropriate contract inputs.

The injected proof must verify request construction and normalized outcomes;
it must not claim that network is denied, descendants are terminated, or the
filesystem is isolated. Those claims require a real runner and separate
security proof. A future timeout may be classified `TimedOut` only after the
selected process boundary proves complete process-group termination.

## 9. Authority And Proportional Governance Assessment

The first live use correctly requires visible blocking approval. Approval must
bind the exact immutable run/profile identity, trusted repository and
toolchain commitments, typed writable roots, timeout/replay posture, evidence
obligations, and absence of OS containment.

The injected implementation itself must not create a reusable grant or imply
future live authority. Installed tools, cached dependencies, a previous pass,
or a prior approval cannot authorize another run.

## 10. Result, Report, Artifact, And Replay Assessment

The existing payload-free local-check result and stable result reference are
the correct output boundary. Reports and private artifacts may cite only the
stable result and distinguish clippy failure, timeout, and internal integrity
failure without compiler text.

Raw diagnostics, stdout/stderr, command transcripts, source snippets, paths,
environment values, dependency or toolchain payloads, credentials, and tokens
must not enter events, results, reports, artifacts, Debug output, errors, or
serialization.

Terminal replay must return existing durable posture without invoking the
runner. Any changed source, lockfile, profile, toolchain, dependency inventory,
environment, writable root, or authority commitment is a new attempt.

## 11. Test Plan Assessment

The plan's test matrix is sufficient with these required additions:

- prove no standard-process clippy constructor or production resolver exists;
- prove the injected runner is not called after any identity, overlap,
  environment, authority, or integrity validation failure;
- prove writable-root role substitution fails before runner entry;
- prove build-script, procedural-macro, and platform-tool inputs that cannot be
  enumerated fail closed in the modeled boundary; and
- prove the profile remains absent from default registration, workflows, CLI,
  schemas, SDKs, and examples.

Focused tests must identify the production boundary that remains unverified.
Complete engineering-standard validation is required before implementation
acceptance even though no live process is authorized.

## 12. Blockers

There is no blocker to the injected-runner-only implementation if the runner
and resolver boundary in section 4 is enforced.

The following remain blockers to any live-process phase:

- incomplete enumeration of build-script, procedural-macro, platform-tool, or
  SDK execution inputs;
- no OS-enforced filesystem or network containment;
- no proof of complete process-group termination;
- inability of final-state commitments to detect restored transient writes;
  and
- no accepted live sandbox attestation replacing those controls.

## 13. Non-Blocking Follow-Ups

- Keep public profile vocabulary separate from executable authority.
- Consider a sealed internal injected-runner harness if future closed profiles
  otherwise repeat the same test-only boundary.
- Defer OpenShell-backed containment until its execution and attestation
  contract is independently accepted.
- Do not compose clippy into the private workflow before the implementation
  review passes.

## 14. Documentation And Honesty Assessment

The plan accurately states that the trusted-host designation is a disclosed
risk posture, not containment. It preserves the distinction between disabled
network policy and enforced egress denial, and between persistent-mutation
detection and write prevention. Its live-process limitations must remain
visible in the implementation report and later review.

## 15. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 16. Governed Review Record

- Workflow ID: `dg/review`
- Run ID: `run-1792802322782810000-2`
- Approval ID:
  `approval/run-1792802322782810000-2/review-scope-approved`
- Presentation ID: `presentation/15a426a04e786961`
- Presentation content hash:
  `15a426a04e7869617de503b850a30cc3e920f9ee418c4e0efdaba9e8130bef2d`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted review-only scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, and zero escalations
- Approval presentation enforcement: `proof_enforced`, with one persisted
  presentation record and the approval-event proof marker present
- Out-of-kernel work: the delegated maintainer inspected the plan, engineering
  standard, runner/export boundary, accepted profile patterns, and test
  surfaces; wrote this review and roadmap verdict; and ran validation outside
  the kernel. The kernel governed scope and approval but did not perform those
  actions.

## 17. Recommended Next Phase

Implement only the injected-runner closed Cargo clippy profile under the
mandatory runner/API constraint in section 4. Add the profile vocabulary,
exact contract, typed writable-root roles, immutable identity validation,
payload-free result behavior, replay posture, and focused tests.

Do not add a standard process runner, live smoke, workflow composition,
default registration, public CLI/schema/SDK exposure, provider access,
hosted execution, OpenShell integration, or another profile.
