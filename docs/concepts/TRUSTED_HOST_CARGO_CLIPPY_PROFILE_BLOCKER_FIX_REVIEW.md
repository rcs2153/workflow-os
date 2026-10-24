# Trusted-Host Cargo Clippy Profile Blocker-Fix Review

## 1. Executive Verdict

**Blocker fix accepted; proceed to trusted-host Cargo clippy live-execution
security planning.**

The merged fix closes the implementation review's injected-runner blockers.
All five roots are revalidated as unlinked directories at their stored
canonical paths immediately before and after runner entry. Current roots must
remain mutually disjoint, writable-root roles remain fixed, protected inputs
are recommitted, and the missing inventory, executor-denial, and registration
regressions are present.

This verdict accepts only the crate-private injected-runner proof. It does not
authorize a standard runner, live Cargo, a smoke test, workflow composition,
default registration, public configuration, provider access, hosted execution,
OpenShell integration, another command profile, writes, or release changes.

## 2. Scope Verification

The fix stayed within the approved blocker scope. It changed the private Cargo
clippy handler, focused Core tests, one executor authority mapping helper, and
phase documentation. The executor helper continues to reject the clippy
profile rather than granting it authority, and no registration profile exposes
the handler.

No live process path, standard process runner, public resolver, workflow step,
CLI/config/schema/SDK surface, provider integration, hosted behavior, or new
profile was added.

## 3. Root Identity Assessment

Construction stores canonical repository, toolchain, dependency-cache,
build-output, and temporary roots. `validate_current_identity()` now calls one
private validator for every root while building the process request,
immediately before the injected runner, and immediately after it returns.

The validator:

- reads the root with `symlink_metadata`;
- rejects a symbolic link or non-directory;
- canonicalizes the current path;
- requires equality with the stored canonical path; and
- returns only a stable non-leaking error on failure.

The current canonical roots are then checked again for overlap in both lexical
directions. This closes the reviewed symbolic-link and canonical-redirection
gap for the injected proof.

The accepted identity is a canonical-path and committed-input identity, not an
inode-pinned directory handle. Replacing a regular directory at the same path
is not independently detected when the relevant committed contents are
identical; build-output and temporary contents are intentionally mutable. That
limitation is acceptable only because this handler has no real-process entry
path. A future live-execution plan must address operating-system containment,
handle/object identity where required, and time-of-check/time-of-use behavior
rather than treating canonical-path equality as a sandbox.

## 4. Typed Root And Overlap Assessment

The stored writable-root vector remains closed and ordered as cache,
build-output, and temporary. Validation rejects missing, additional, reordered,
or substituted roles before constructing the environment or entering the
runner. The stored role paths must also equal the separately stored canonical
cache, build-output, and temporary roots.

Construction and current-identity validation reject identical or nested roots
in either direction across all five root categories. Focused tests directly
cover build-output containing temporary and temporary containing build-output.

## 5. Protected Input And Inventory Assessment

Toolchain executables, additional platform tools, dependency inputs, and the
protected repository tree are recommitted after current-root validation before
and after runner entry. Persistent source, manifest, lockfile, build-script,
toolchain, platform-tool, and dependency drift therefore fails before an
authoritative result is returned.

The fix rejects an empty dependency inventory, a duplicate dependency input,
and a duplicate additional-tool input. It also preserves the earlier
requirement for at least one additional toolchain executable.

These tests prove only that a supplied fixture inventory is closed and
well-formed. They do not prove that every build script, procedural macro, SDK,
platform tool, or dependency required by a real Cargo invocation has been
enumerated.

## 6. Executor And Registration Assessment

The trusted-host authority mapping returns the stable security error
`executor.trusted_host.profile_not_executable` for
`CargoClippyWorkspace`. The focused feature-gated unit regression exercises
that exact branch.

The available registration profiles remain `None` and explicit docs check.
The registration regression verifies that neither profile advertises the
Cargo clippy command kind. No ordinary runtime path can construct or select
the crate-private handler.

## 7. Error, Privacy, And Result Assessment

Root replacement returns
`local_check.profile.clippy.root_identity_changed`. Role and overlap failures
retain their stable codes. These errors contain no path, root name, source
content, dependency material, command output, environment value, or credential.

Handler Debug output continues to redact toolchain, repository, dependency,
execution-identity, and runner details. Result construction remains bounded
and payload-free; raw stdout and stderr are not stored in the result.

## 8. Test Quality Assessment

The reviewed additions cover:

- symbolic-link replacement of every root before runner entry;
- symbolic-link replacement of every root by the injected runner;
- nested writable-root overlap in both directions;
- empty and duplicate dependency inventory;
- duplicate additional-tool inventory;
- direct executor denial; and
- explicit absence from available registration profiles.

Existing focused coverage continues to prove the exact command and environment,
bounded pass/failure/timeout results, persistent protected-input drift,
permitted writable-root content changes, non-leaking errors, redacted Debug,
and rejection by the public generic resolver.

No test executes Cargo or proves process-group termination, egress denial,
filesystem containment, transient-write observation, or complete real-Cargo
input enumeration. The test names and reports preserve that distinction.

## 9. Documentation Assessment

The implementation report now states exactly that current root validation uses
`symlink_metadata`, directory checks, canonicalization, and stored-path
comparison. It explicitly says that inode pinning and operating-system
containment are not implemented. It also preserves complete-inventory,
process-control, transient-write, egress, and sandbox-attestation limitations.

The original review remains unchanged apart from its fix-forward note, so the
historical blocker finding is not erased.

## 10. Remaining Limitations

- There is no standard runner or live Cargo execution path.
- Canonical-path validation is not inode pinning or a protected directory
  handle.
- There is no operating-system filesystem, process, or network sandbox.
- Offline Cargo configuration is not independently enforced egress denial.
- Persistent final-state checks cannot observe a transient restored write.
- Real Cargo's complete toolchain and dependency input set is not enumerated or
  independently attested.
- No workflow, default registration, CLI, schema, SDK, provider, hosted, or
  OpenShell surface exposes this profile.

## 11. Blockers

None for the injected-runner-only profile.

The limitations above remain blockers to any standard-process implementation,
live smoke, workflow composition, registration, or production-readiness claim.

## 12. Non-Blocking Follow-Ups

- Use precise language: the accepted root identity is canonical-path identity
  plus committed protected inputs, not an inode-pinned or sandboxed object.
- Keep complete real-Cargo input discovery and attestation in the future
  security plan.
- Require explicit process termination, filesystem containment, egress, and
  transient-write postures before a live process is authorized.
- Keep OpenShell or another sandbox substrate in its separately governed lane;
  this review does not select one.

## 13. Validation

- focused Cargo clippy profile tests: passed, 12 tests.
- direct trusted-host executor denial regression: passed, 1 test.
- registration exclusion regression: passed, 1 test.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

The full validation matrix ran against the exact merged blocker-fix revision.
Documentation-only review edits were then checked by the docs and diff checks
at phase close.

## 14. Governed Review Record

- Workflow ID: `dg/review`
- Run ID: `run-1792807924327258000-2`
- Approval ID:
  `approval/run-1792807924327258000-2/review-scope-approved`
- Presentation ID: `presentation/949d329a1ab325a8`
- Presentation content hash:
  `949d329a1ab325a867d45129b198591c4ab5f68683cef9506d7f393342b96852`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted review-only scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, and zero escalations
- Approval-presentation enforcement: proof enforced with the persisted
  presentation record and approval-event proof marker present
- Out-of-kernel work: source and test inspection, review writing, validation,
  git, and PR operations were performed by the delegated maintainer. The
  kernel governed scope and approval but did not perform those actions.

## 15. Recommended Next Phase

Plan the trusted-host Cargo clippy live-execution security boundary. The plan
must decide how complete build-time inputs are discovered and attested, how
root object identity and time-of-check/time-of-use behavior are controlled,
how process groups terminate, how filesystem and network policy are enforced,
how transient writes are handled, and what independent sandbox evidence is
required.

The planning phase must not implement a standard runner, live smoke, workflow
composition, default registration, public surfaces, provider access, hosted
execution, OpenShell integration, another command profile, writes, or release
changes.
