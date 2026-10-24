# Trusted-Host Cargo Clippy Profile Blocker-Fix Report

## 1. Executive Summary

The focused blocker fix closes the root time-of-use gap found by the accepted
Cargo clippy profile implementation review. The crate-private handler now
revalidates every typed root immediately before and after injected-runner
entry. The phase remains non-live and does not create production authority to
run Cargo.

## 2. Blockers Fixed

- Repository, toolchain, dependency-cache, build-output, and temporary roots
  must remain unlinked directories and canonicalize to their stored paths.
- Writable-root paths must still match their stored cache, build-output, and
  temporary roles.
- Current canonical roots must remain mutually disjoint.
- Empty and duplicate dependency/toolchain inventories fail closed.
- Direct trusted-host executor authority rejects the clippy profile.
- Standard registration profiles expose no clippy handler.
- The implementation report now describes current root revalidation exactly.

## 3. Implementation Approach

The handler stores the accepted build-output and temporary roots alongside the
existing repository, toolchain, and dependency-cache roots. Its existing
`validate_current_identity` boundary now calls one private root validator for
all five roots before recomputing toolchain, dependency, source, and execution
commitments.

The validator uses `symlink_metadata`, requires a directory rather than a
symbolic link, canonicalizes the current path, and compares it with the stored
canonical path. This validation runs while building the process request,
immediately before the injected runner, and immediately after the runner
returns. Writable root contents remain allowed to change; root identity does
not.

## 4. Validation Boundary

Root replacement fails with the stable non-leaking code
`local_check.profile.clippy.root_identity_changed`. Typed role substitution
continues to fail with
`local_check.profile.clippy.writable_root_roles_invalid`, and root nesting or
overlap continues to fail with `local_check.profile.clippy.root_overlap`.

The fix does not claim inode pinning, operating-system containment, egress
denial, process-group control, transient-write observation, or complete
real-Cargo input enumeration. Those remain blockers to a future live process.

## 5. Test Coverage

Focused tests now prove:

- all five roots reject symbolic-link replacement before runner entry;
- all five roots reject replacement performed by the injected runner;
- build-output containing temp and temp containing build-output both fail;
- empty dependency inventory, duplicate dependency inventory, and duplicate
  additional-tool inventory fail closed;
- the trusted-host executor maps clippy to
  `executor.trusted_host.profile_not_executable`;
- registration profiles never include `CargoClippyWorkspace`; and
- prior exact-command, environment, mutation, privacy, and result tests remain
  green.

## 6. Privacy And Security

The new error contains no path, root, file, command output, environment value,
or credential. Debug and result serialization remain payload-free. No raw
source, dependency, or runner output is added to events, evidence, reports, or
artifacts.

## 7. Scope Explicitly Not Completed

This fix does not add live Cargo execution, a standard runner, a public
production resolver, workflow composition, default registration, CLI/config/
schema/SDK exposure, provider access, hosted execution, OpenShell integration,
another profile, or write authority beyond the existing modeled local-check
boundary.

## 8. Validation

- `cargo fmt --all --check`: passed.
- focused Cargo clippy profile tests: passed, 12 tests.
- direct trusted-host executor rejection test: passed, 1 test.
- registration exclusion regression: passed, 1 test.
- `cargo test -p workflow-core --test local_check`: passed, 83 tests with 2
  existing opt-in live smokes ignored.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Governed Fix Record

- Workflow ID: `dg/blocker`
- Run ID: `run-1792806381333458000-2`
- Approval ID: `approval/run-1792806381333458000-2/fix-approved`
- Presentation ID: `presentation/5808cc12cc6c5c09`
- Presentation content hash:
  `5808cc12cc6c5c09ebbcfda90a35aa901f2d4cec811d5d57a775a3afc471fa62`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted blocker-fix scope
- Phase status: `Completed`
- Event summary: 39 events; 1 approval; 0 retries; 0 escalations. Approval-
  presentation proof was enforced and the persisted proof marker is present.
- Out-of-kernel work: implementation, tests, documentation, validation, git,
  and PR operations are executed by the delegated maintainer. The kernel
  governs scope and approval but does not perform those actions.

## 10. Remaining Known Limitations

The accepted implementation review's live-execution limitations remain:
complete build-script, procedural-macro, platform-tool, SDK, and dependency
enumeration; process-group termination; OS filesystem/network containment;
transient-write observation; and independent sandbox attestation.

## 11. Recommended Next Phase

Perform one focused **trusted-host Cargo clippy profile blocker-fix review**.
Do not plan or authorize a live smoke unless that review accepts the fix and a
separate live-execution security plan closes the remaining limitations.
