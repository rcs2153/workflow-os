# OpenShell Pinned CLI Compatibility Transport Report

## 1. Executive Summary

Workflow OS now has a bounded compatibility transport for OpenShell v0.0.101,
pinned to upstream commit
`8ddd98c3dff62619a3963f99ba1e055b67650e72`. It validates the exact CLI
version, constructs fixed no-provider sandbox arguments, enforces subprocess
time/output bounds, and strictly parses reviewed machine-readable sandbox and
effective-policy response shapes.

The compatibility spike also found a security-relevant upstream gap. The
pinned CLI does not expose the driver-observed immutable image identity,
complete OCSF observations, or machine-readable cleanup confirmation required
by `OpenShellNoWriteClient`. The transport is therefore not wired into live
execution. Workflow OS fails closed rather than presenting requested image or
human output as runtime attestation.

## 2. Scope Completed

- Pinned OpenShell CLI version and upstream commit constants.
- Explicit absolute binary, workspace, and digest-pinned image configuration.
- Fixed argv invocation without a shell or caller-selected command.
- Manual policy approval and disabled provider auto-creation arguments.
- Bounded subprocess timeout, stdout, and stderr handling.
- Strict sandbox create/get JSON parsing.
- Strict full effective-policy JSON parsing and canonical policy hashing.
- Non-leaking Debug and structured transport/protocol failures.
- Compatibility fixtures and focused regression tests.

## 3. Scope Explicitly Not Completed

- No OpenShell installation, gateway, compute driver, or sandbox launch.
- No `OpenShellNoWriteClient` implementation or provider selection.
- No arbitrary command, provider, environment, credential, or access material.
- No filesystem/network/process execution proof.
- No OCSF collection, artifact collection, or cleanup execution.
- No provider mutation, schema, broad CLI, automatic default, or fork.
- No live smoke proof or production-readiness claim.

## 4. Compatibility Boundary

The transport consumes only version-pinned structured output from:

- `sandbox create --output json`;
- `sandbox get --output json`; and
- `policy get <sandbox> --full --output json`, whose v0.0.101 response is the
  effective policy by default.

Unknown fields fail closed because a security-relevant response-shape change
must be reviewed before adoption. Human CLI text is not parsed. Nonzero exits,
timeouts, oversized output, malformed UTF-8/JSON, stale effective-policy
versions, missing policy bodies, and identity mismatches return bounded errors
without copying subprocess output.

## 5. Attestation Gap

OpenShell's Docker and Podman drivers inspect the selected image and launch by
an immutable image ID, but v0.0.101 structured CLI sandbox output omits both the
requested image and driver-observed immutable identity. Its effective-policy
surface is useful and machine-readable, but no equivalent complete CLI surface
exists for OCSF observation reduction or cleanup confirmation.

The existing Workflow OS provider requires all of those facts. The CLI
transport cannot honestly synthesize them, so it remains a compatibility
building block rather than an execution client.

## 6. Privacy And Error Posture

Configuration paths, workspace, image references, sandbox identities, policy
hashes, and subprocess output are redacted or omitted from Debug and errors.
The subprocess receives no stdin and is invoked directly. Output is bounded in
memory and never enters Core events, evidence summaries, or reports.

## 7. Test Coverage

Focused tests cover:

- all reviewed machine-readable response fixtures;
- exact version compatibility;
- fixed no-provider/manual-approval argv;
- immutable image and absolute binary validation;
- unknown response fields failing closed;
- stale effective policy failing closed; and
- Debug/error non-leakage.

## 8. Commands Run And Results

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p workflow-hosted openshell_cli`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

All commands passed. The focused compatibility suite includes seven transport
tests. Workspace tests retained only the repository's existing explicitly
ignored live-provider and environment-dependent cases.

## 9. Remaining Limitations

- A complete provider client still needs trustworthy effective runtime-image,
  observation, and cleanup surfaces.
- OpenShell remains alpha and every consumed response shape must remain pinned.
- No local OpenShell installation or reviewed compute driver exists on this
  machine for a smoke proof.
- Requested configuration is not execution evidence.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of this compatibility boundary and
the discovered attestation gap. Then pursue an upstream/API-compatible way to
obtain complete runtime image, OCSF, and cleanup facts before implementing
`OpenShellNoWriteClient` or running a live smoke proof.

Do not fork OpenShell, weaken the provider contract, add credentials, accept
arbitrary commands, or enable automatic provider selection.

## 11. Governed Phase Evidence

- Workflow: `dg/runtime-composition`.
- Run ID: `run-1786243334569626000-2`.
- Approval ID:
  `approval/run-1786243334569626000-2/composition-approved`.
- Approval presentation ID: `presentation/ed5b91aa354029bb`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: pinned CLI transport, strict fixtures, bounded subprocess
  behavior, tests, and honest documentation without installation/live smoke.
- Out-of-kernel work: Codex inspected pinned upstream source, edited code/docs,
  and ran validation. The kernel governed scope and approval but did not install
  OpenShell, invoke a sandbox, edit files, execute tests, or perform git/PR
  actions.
