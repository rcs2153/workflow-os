# Hacker News Launch Admission Phase 1 Report

Status: implementation complete; exact-head Linux rehearsal required before merge

## Decision

Phase 1 defines and implements the provisional native-archive contract without
publishing artifacts or changing Workflow OS runtime semantics.

The selected targets are:

- `aarch64-apple-darwin`; and
- `x86_64-unknown-linux-musl`.

Archives must be packaged and version-checked on a matching native host. The
packager refuses unsupported targets, host/target mismatches, non-exact source
commits, source commits that differ from checkout `HEAD`, binary target
mismatches, and version mismatches.

## Implemented Scope

- deterministic `.tar.gz` archive and SHA-256 generation;
- exact release version synchronization across every Rust crate, the root
  package, and the TypeScript SDK;
- exact source-commit binding and clean/dirty checkout disclosure;
- native binary target and executable version verification;
- bounded archive layout with license, README, manifest, installer,
  uninstaller, and CLI binary;
- install to `$HOME/.local/bin` or `WORKFLOW_OS_INSTALL_DIR`;
- fail-closed overwrite behavior unless `--force` is explicit;
- uninstall of the binary only, preserving local Workflow OS state;
- focused deterministic archive and install/uninstall tests; and
- a non-publishing native Linux CI rehearsal that does not upload artifacts.

The public packaging command requires `--provisional`. This Phase 1 tool cannot
silently create or label final release candidates.

## Local macOS Rehearsal

The Apple Silicon host built and packaged the release CLI from source commit:

```text
5ffb6b7ee0bb11047264bf806fbb697286af4fc9
```

The provisional archive was:

```text
workflow-os-v0.3.0-preview.1-aarch64-apple-darwin.tar.gz
sha256: 7bb117a11773853456fcc032efb4df9231e055a802316a34f0a978c4237afe7b
```

The manifest disclosed `working_tree_clean: false` and `provisional: true`.
That is acceptable rehearsal evidence and explicitly disqualifies this archive
from promotion.

Local validation proved:

- the checksum file verifies;
- the archive has the exact documented layout;
- the installed executable reports `workflow-os 0.3.0-preview.1`;
- a second install refuses overwrite without `--force`;
- uninstall removes the executable; and
- uninstall preserves existing `.workflow-os` state.

## Linux Rehearsal Boundary

The macOS host installed Rust's official Linux musl standard library and
attempted a real cross-build. The build failed closed because native C
dependencies require `x86_64-linux-musl-gcc`, which is not installed on this
host. No Linux archive was fabricated from that attempt.

The CI job `Release Packaging Rehearsal` checks out the exact PR head, installs
the native musl toolchain on Linux, builds the x86-64 binary, packages it,
verifies its checksum, installs it, checks its version, proves overwrite
refusal, and uninstalls it. It uploads nothing. Terminal-green exact-head CI is
required before this phase may merge.

## Validation

Completed locally:

- `npm run test:release-package`: 4 passed;
- deterministic repeat archive equality;
- native macOS release build and version check;
- macOS checksum and archive-layout check;
- real archive install, overwrite refusal, uninstall, and state preservation;
- `npm run check:docs`; and
- `git diff --check`.

The full existing CI matrix and the new native Linux rehearsal remain exact-head
merge gates.

## Deferred Scope

Phase 1 does not authorize:

- artifact publication, tags, GitHub Releases, or announcements;
- final clean-room candidates;
- Phase 2 golden-path composition;
- additional platforms or package managers;
- code signing or notarization;
- provider mutation or a generic command runner; or
- Salsa, OpenShell, nested harness, hosting, or enterprise features.

Phase 2 changes the selected user path. Final macOS and Linux archives must be
rebuilt from the exact clean post-composition commit before Phase 3.

## Governed Phase

- workflow: `dg/implement`
- run: `run-1793744836989109000-2`
- approval: `approval/run-1793744836989109000-2/implementation-approved`
- presentation proof: `presentation/71b42fae848a8bd0`
- approval outcome: granted under delegated-maintainer authority

Repository edits, builds, tests, archive creation, install/uninstall rehearsal,
and git/PR actions were executed outside the kernel and are disclosed here.
The kernel governed scope and approval; it did not execute shell commands or
publish artifacts.
