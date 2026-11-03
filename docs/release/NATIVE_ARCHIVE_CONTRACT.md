# Native Archive Contract

Status: provisional launch-admission packaging contract

Workflow OS native archives are a local-first distribution shape for the
`workflow-os` CLI. This contract does not authorize publication and does not
claim production readiness.

## Supported Rehearsal Targets

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

`PACKAGE-MANIFEST.json` records the release-package schema, version, target,
exact source commit, provisional posture, binary SHA-256, install defaults,
and state posture. The archive has a sibling `.sha256` file using the common
`<digest>  <filename>` format.

The packager produces deterministic tar headers, ordering, timestamps,
ownership fields, and gzip output. Repeating the same package operation over
identical inputs must produce the same checksum.

## Version And Target Verification

Before writing an archive, the packager must:

1. confirm all Rust crate, root package, and TypeScript SDK versions match;
2. confirm `file` identifies the binary as the requested native target;
3. execute `workflow-os --version` on the native host; and
4. require the exact output `workflow-os <VERSION>`.

An exact 40-character source commit is mandatory and must equal checkout
`HEAD`. The manifest records whether the working tree was clean. A package
produced from a dirty checkout may be used only as a disclosed provisional
rehearsal and must never be promoted as a final candidate.

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

## Phase 1 Rehearsal Posture

Phase 1 archives are provisional and must not be published. The macOS arm64
archive is rehearsed locally. The Linux x86-64 archive is rehearsed in the
non-publishing CI job on a native Linux runner. CI verifies checksum, archive
creation, install, version, overwrite refusal, and uninstall behavior without
uploading the archive.

Phase 2 changes the selected golden path. Final native candidates therefore
must be rebuilt from the exact post-composition commit and revalidated before
clean-room evaluation.
