# Native Archive Contract

Status: reviewed implementation contract; final candidate production remains
separately governed

Workflow OS native archives are a local-first distribution shape for the
`workflow-os` CLI. This contract does not authorize publication and does not
claim production readiness.

## Supported Native Targets

The Hacker News launch-admission rehearsal selects:

- `aarch64-apple-darwin` for Apple Silicon macOS; and
- `x86_64-unknown-linux-musl` for portable x86-64 Linux.

Each archive must be built and packaged on a matching native host. The
packager fails closed when the host platform or architecture does not match
the requested target. Cross-compilation alone is not accepted as executable
version evidence.

Windows, Intel macOS, Linux arm64, package-manager formulae, installers, and
container images are unsupported by this contract.

## Archive Identity And Layout

Archive names use:

```text
workflow-os-v<VERSION>-<TARGET>.tar.gz
```

The archive contains one directory with the same basename and exactly these
entries:

```text
LICENSE
PACKAGE-MANIFEST.json
README.md
install.sh
uninstall.sh
workflow-os
```

`PACKAGE-MANIFEST.json` uses `workflowos.dev/release-package/v1` and records
the version, target, exact source commit, explicit package posture, clean-tree
fact, binary SHA-256, install defaults, and state posture. The archive has a
sibling `.sha256` file using the common
`<digest>  <filename>` format.

The packager produces deterministic tar headers, ordering, timestamps,
ownership fields, and gzip output. Repeating the same package operation over
identical inputs must produce the same checksum.

## Version And Target Verification

Before writing an archive, the packager must:

1. confirm all Rust crate, root package, and TypeScript SDK versions match;
2. confirm `file` identifies the binary as the requested native target,
   including static linkage for the Linux musl target;
3. execute `workflow-os --version` on the native host; and
4. require the exact output `workflow-os <VERSION>`.

An exact 40-character source commit is mandatory. All active Rust and npm
manifests and their lockfiles must agree on the version.

The packager requires exactly one explicit posture:

```text
--provisional
--final-candidate
```

Missing or conflicting posture fails before binary reads or output creation.
`provisional_rehearsal` may use explicitly disclosed verification bypasses for
contract tests and non-publishing CI. `final_candidate` categorically rejects
every binary or repository verification bypass through the exported API. It
requires checkout `HEAD` to equal the requested source commit, a clean tracked
and untracked worktree, a matching native host and binary, and exact
`workflow-os <VERSION>` output.

A provisional archive can never be promoted into a final candidate merely by
renaming it because the manifest posture is content-bound into the archive and
checksum. Final candidates must be rebuilt under final admission.

## Output Safety

All admission and archive construction completes before the output directory
is created. Existing archive or checksum paths fail closed. The packager writes
private temporary files and links them into their final names without replacing
existing files; an incomplete pair is removed on packager failure. Identical
inputs in distinct output directories produce identical archives and checksums.

## Install Contract

`install.sh` installs only the `workflow-os` executable.

- Default destination: `$HOME/.local/bin/workflow-os`
- Override directory: `WORKFLOW_OS_INSTALL_DIR`
- Existing binaries: fail closed unless the user passes `--force`
- Installed mode: executable (`0755`)

The script does not edit shell profiles, modify `PATH`, initialize a project,
create runtime state, install services, or make network requests.

## Uninstall And State Contract

`uninstall.sh` removes only the installed executable at the selected install
directory. A missing executable is an idempotent success.

Uninstall never removes repository `.workflow-os` directories, local runtime
state, reports, evidence, logs, user configuration, or project files. Users
must review and remove those separately when appropriate.

## Provisional Rehearsal And Final-Candidate Boundaries

Phase 1 archives are provisional and must not be published. The macOS arm64
archive is rehearsed locally. The Linux x86-64 archive is rehearsed in the
non-publishing CI job on a native Linux runner. CI verifies checksum, archive
creation, install, version, overwrite refusal, and uninstall behavior without
uploading the archive.

Final-candidate support in the packager is an admission mechanism, not
authority to produce or publish a real release candidate. Candidate production
requires a separate governed release phase after the implementation is merged,
from one frozen exact clean commit. Tagging, GitHub Release creation, asset or
registry upload, and announcement remain separately authorized external
mutations.
