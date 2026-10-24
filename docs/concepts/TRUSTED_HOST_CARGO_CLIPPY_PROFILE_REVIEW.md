# Trusted-Host Cargo Clippy Profile Implementation Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation preserves the most important scope boundary: the Cargo
clippy handler is crate-private, accepts only an injected process runner, and
has no standard runner, public production resolver, workflow composition,
default registration, or live execution path. The exact command, sanitized
environment, typed writable-root roles, protected-input commitments, bounded
result posture, and non-leaking errors are materially stronger than command
allowlisting alone.

Acceptance is blocked because current-identity validation does not revalidate
the filesystem identity of the repository, toolchain, dependency-cache,
build-output, or temporary roots after construction. In particular, a writable
root can be replaced by a symbolic link or redirected path before runner entry
without changing the stored execution identity. The focused tests also omit
mandatory post-resolution root-substitution and incomplete-inventory proofs.
These gaps must be closed before any standard-process or live-smoke phase.

## 2. Scope Verification

The implementation stayed within the approved injected-runner-only scope. It
added:

- one closed `CargoClippyWorkspace` profile identity and exact canonical
  command contract;
- one crate-private handler and crate-private definition;
- one injected-runner constructor;
- explicit toolchain, dependency, repository, cache, build-output, and
  temporary inputs;
- typed private writable-root roles;
- payload-free result behavior and stable non-leaking errors;
- focused module tests; and
- an implementation report and roadmap update.

It did not add a standard runner, live Cargo process, production resolver,
workflow composition, default registration, public CLI/config/schema/SDK
surface, providers, hosted execution, OpenShell integration, or another
profile.

## 3. Closed Command And Resolver Assessment

The canonical contract fixes exactly:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

It fixes repository-root execution, a 15-minute timeout,
`BuildOrCacheWrites`, bounded non-persistent output, and the eight allowed
environment variables. The handler rejects any contract that is not exactly
equal to the Core-owned canonical contract.

Public API exposes only non-executing profile vocabulary. The generic public
resolver rejects the clippy profile with
`local_check.profile.resolver_mismatch`, and the trusted-host executor rejects
it with `executor.trusted_host.profile_not_executable`. The private handler
does not select `StdLocalCheckProcessRunner` and cannot be constructed outside
its module. This satisfies the mandatory no-live-execution boundary.

## 4. Toolchain And Dependency Identity Assessment

Construction requires canonical regular executable files for Cargo,
cargo-clippy, clippy-driver, rustc, rustdoc, and a non-empty explicit set of
additional toolchain executables. The implementation commits relative paths
and contents and repeats those commitments before and after runner entry.
Dependency inputs similarly must be canonical regular files under the explicit
dependency-cache root and are content committed before and after entry.

This is sufficient for an injected proof only when the supplied fixture
inventory is explicitly treated as complete. It is not proof that a real
Cargo invocation has enumerated every build-script, procedural-macro,
platform-tool, SDK, or dependency input. The implementation report preserves
that limitation, and it remains a blocker to any live process.

Focused tests prove one missing platform-tool inventory and persistent
toolchain/dependency mutation. They do not directly prove empty dependency
inventory rejection, duplicate inventory rejection for both families, or
post-resolution root replacement. Those omissions are part of the blocker-fix
test scope.

## 5. Root And Protected-State Assessment

Construction canonicalizes each root, rejects root-level symbolic links, and
rejects overlap in either lexical direction across repository, toolchain,
cache, build-output, and temporary roots. Typed role ordering prevents cache,
build, and temporary roots from being substituted inside the stored handler.

The protected repository commitment is deterministic, includes source,
manifests, lockfile, build scripts, ignored and untracked material, and rejects
symbolic links below the root. Only `.git`, `.workflow-os`, and
`dogfood/.workflow-os` are excluded. Persistent protected-source mutation is
checked after runner return before a result can become authoritative.

The blocker is the time-of-use root identity. `validate_current_identity()`
checks stored canonical paths for lexical overlap and recomputes file/tree
commitments, but it does not call `symlink_metadata` or canonicalize the roots
again. The execution identity also reuses each writable root's construction-
time `initial_commitment` rather than validating current root identity. A
cache, build-output, or temporary directory replaced by a symbolic link after
handler construction can therefore reach runner entry without a boundary
error. Repository or read-only root replacement can likewise be invisible
when the replacement resolves to equivalent committed contents.

The blocker fix must revalidate every root immediately before and after runner
entry: it must remain an unlinked directory, canonicalize to the exact stored
root, remain mutually disjoint, and retain its assigned role. Writable root
contents may change; root identity may not.

## 6. Environment And Process Assessment

The handler constructs an exact environment containing only `CARGO_HOME`,
`CARGO_NET_OFFLINE`, `CARGO_TARGET_DIR`, `PATH`, `RUSTC`, `RUSTDOC`,
`RUSTUP_HOME`, and `TMPDIR`. `PATH` is synthesized only from committed
toolchain executable parents. No home, proxy, provider credential, compiler
wrapper, registry, target override, or inherited environment value is added.

The shared standard runner clears inherited environment, but this profile does
not select that runner. Its injected tests prove request construction and
normalized results only. They do not prove process-group termination,
filesystem isolation, or network denial. Those remain explicit blockers to a
future live phase rather than defects in this non-live slice.

## 7. Result, Privacy, And Error Assessment

Pass, non-zero failure, and timeout outputs become the existing bounded
`LocalCheckResult` statuses. The clippy tests use empty output and prove that
serialized results and handler `Debug` do not reveal fixture paths. Handler
errors use stable static codes and messages and do not echo paths, source,
dependency names, command output, environment values, or credentials.

No raw output, provider payload, source content, manifest content, environment
value, or credential is persisted by the new profile. The implementation does
not claim that final-state commitments detect transient writes restored before
the post-entry check.

## 8. Runtime And Replay Assessment

There is no runtime path for this profile. Public resolution and trusted-host
admission fail closed, so the new handler cannot create a live invocation,
event trail, report artifact, or terminal replay. Existing executor and
immutable-bundle matches only preserve exhaustive closed vocabulary.

This is the correct phase boundary. Runtime composition, terminal replay, and
report/artifact integration must not be inferred from model compatibility.

## 9. Test Quality Assessment

Strong focused coverage proves:

- exact profile, command, arguments, side-effect kinds, and skill identity;
- injected-runner-only request construction;
- exact offline environment and absence of selected ambient variables;
- payload-free pass, failure, and timeout results;
- missing additional-tool identity before runner entry;
- construction-time overlap and stored role substitution rejection;
- persistent source, manifest, lockfile, build-script, toolchain,
  platform-tool, and dependency drift rejection;
- permitted cache, build-output, and temporary content mutation;
- redacted handler `Debug` and serialized result output; and
- rejection by the public generic resolver.

Missing mandatory proofs are:

- post-construction symbolic-link or canonical-root substitution for every
  root role;
- nested overlap in both directions rather than only identical roots;
- empty and duplicate dependency and toolchain inventories;
- current root-role identity after runner return;
- direct executor rejection for the non-executable profile; and
- explicit absence from standard/default registration.

Generic local-check tests cover output bounds and secret-like output, but the
review does not treat them as process-group or live-clippy evidence.

## 10. Documentation Assessment

The plan and implementation report accurately state that the handler is
non-live and that fixed command identity, offline Cargo, and final-state
commitments are not operating-system containment. The report correctly keeps
process-group termination, transient writes, host-file reads, complete
platform-tool enumeration, egress enforcement, and sandbox attestation as
known limitations.

The report's statement that root validation is repeated after the injected
call is too broad. Stored role/order and lexical overlap are repeated, but
current filesystem root identity is not. The blocker fix must correct the code
and then tighten that wording rather than erasing this finding.

## 11. Blockers

1. Revalidate repository, toolchain, dependency-cache, build-output, and
   temporary root identity immediately before and after runner entry. Each must
   remain an unlinked directory, canonicalize to its stored root, remain
   mutually disjoint, and preserve its typed role without exposing paths in
   failures.
2. Add focused regressions for post-resolution symbolic-link/canonical-root
   substitution, nested overlap in both directions, empty and duplicate input
   inventories, post-entry root substitution, executor rejection, and absence
   from standard/default registration.
3. Correct the implementation report's repeated-root-validation claim and
   document that inventory completeness remains modeled fixture evidence only,
   not authorization for a real Cargo process.

## 12. Non-Blocking Follow-Ups

- Keep complete build-script, procedural-macro, platform-tool, SDK, and
  dependency enumeration as a blocker to live execution.
- Keep process-group termination, OS filesystem/network containment, transient
  write observation, and independent sandbox attestation outside this blocker
  fix and mandatory before a live smoke.
- Preserve the distinction between an offline Cargo setting and enforced
  egress denial.
- Do not compose the profile into a workflow or add a production resolver
  until a separate live-execution security plan is accepted.

## 13. Recommended Next Phase

Perform one narrow **trusted-host Cargo clippy profile blocker fix** covering
the three blockers above, followed by a focused blocker-fix review.

Do not add live Cargo execution, a standard runner, public production resolver,
workflow composition, default registration, CLI/schema/SDK exposure,
providers, hosted execution, OpenShell integration, or another profile.

## 14. Validation

- `cargo fmt --all --check`: passed.
- focused Cargo clippy profile tests: passed, 8 tests.
- `cargo test -p workflow-core --test local_check`: passed, 82 tests with 2
  explicitly opt-in smoke tests ignored.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 15. Governed Review Record

- Workflow ID: `dg/review`
- Run ID: `run-1792805094200590000-2`
- Approval ID:
  `approval/run-1792805094200590000-2/review-scope-approved`
- Presentation ID: `presentation/974a01d4acc50344`
- Presentation content hash:
  `974a01d4acc503449b6330be7998269888a7163f42ceb759b1b6dd533da2cadc`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted review-only scope
- Phase status: `Completed`
- Event summary: 39 events; 1 approval; 0 retries; 0 escalations. The trail
  contains 1 `RunCreated`, 1 `RunValidated`, 1 `RunStarted`, 6
  `StepScheduled`, 8 `PolicyDecisionRecorded`, 1 `ApprovalRequested`, 1
  `ApprovalGranted`, 1 `RunResumed`, 6 `SkillInvocationRequested`, 6
  `SkillInvocationStarted`, 6 `SkillInvocationSucceeded`, and 1
  `RunCompleted` event. Approval-presentation proof was enforced and the
  persisted presentation marker is present.
- Out-of-kernel work: code, test, plan, report, and runtime inspection; review
  writing and validation were performed by the delegated maintainer. Git and
  PR operations remain outside the kernel and will be disclosed when complete.
