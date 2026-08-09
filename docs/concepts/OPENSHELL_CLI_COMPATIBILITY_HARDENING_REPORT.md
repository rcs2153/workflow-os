# OpenShell CLI Compatibility Hardening Report

## 1. Executive Summary

The disconnected OpenShell v0.0.101 CLI compatibility transport now fails
closed on executable digest mismatch, successful stderr, incoherent detailed
policy fields, and observable sandbox/policy drift. The phase strengthens a
local parsing and subprocess boundary only. It does not implement
`OpenShellNoWriteClient`, invoke a live sandbox, or claim runtime attestation.

## 2. Scope Completed

- Required an expected SHA-256 executable digest in transport configuration.
- Verified executable contents before and after each reviewed subprocess call.
- Rejected nonempty stderr even when the subprocess exited successfully.
- Required detailed policy revision to match the current loaded policy version.
- Restricted detailed policy source to the reviewed `sandbox` or `global`
  vocabulary.
- Preserved sandbox resource version in the parsed compatibility model.
- Added a before/policy/after reconciliation helper that rejects observable
  sandbox, lifecycle, policy-version, or policy-source drift.
- Preserved configured stream bounds through reader-thread join validation.

## 3. Scope Explicitly Not Completed

This phase did not:

- install OpenShell, start a gateway, select a compute driver, or run a sandbox;
- implement or wire `OpenShellNoWriteClient`;
- accept arbitrary commands, credentials, access material, or provider writes;
- create automatic provider selection, workflow schemas, SDK fields, examples,
  or release changes;
- fork OpenShell or claim production readiness;
- bind a policy path atomically to the exact bytes OpenShell consumes; or
- fabricate runtime image, OCSF observation, denied-egress, artifact, or
  cleanup evidence.

## 4. Binary Integrity Posture

`OpenShellCliTransportConfig` now accepts a typed expected executable digest.
Every subprocess operation hashes the configured absolute path before and after
the runner returns and rejects a mismatch with a stable protocol error. Debug
output redacts the path and digest.

This is a compatibility integrity check, not full provenance. It does not prove
who built the executable, bind it to the recorded upstream commit, verify a
signature, or exclude transient replacement between the two checks. A live
integration still needs an installer-owned immutable location and reviewed
digest/signature distribution posture.

## 5. Stderr And Subprocess Posture

The transport no longer ignores successful stderr. Any nonempty stderr causes a
bounded protocol failure without copying warning text into errors. Fixed argv,
closed stdin, timeout, and stream bounds remain unchanged. The configured
stream limit is now used consistently when joined reader output is validated.

## 6. Reconciliation Semantics

`inspect_reconciled_sandbox` performs:

1. one detailed sandbox observation;
2. one full effective-policy observation; and
3. a second detailed sandbox observation.

It accepts the result only when both detailed observations are identical and
their current policy version, detailed revision, policy source, and effective
policy version agree. The returned `OpenShellCliReconciledSnapshot` exposes
only bounded identity/revision/hash accessors and redacts sensitive identity and
hash values in Debug.

The three subprocess calls are not atomic. The result detects visible drift but
must not be used as execution attestation or represented as a single upstream
snapshot.

## 7. Upstream Attestation Gaps

Live provider wiring remains blocked on:

1. exact binding of sandbox policy input to reviewed bytes;
2. driver-observed immutable runtime-image identity;
3. complete machine-readable execution and denied-egress observations;
4. machine-readable cleanup confirmation; and
5. an observation boundary trustworthy enough for exact receipt binding.

Requested configuration, labels, annotations, human output, or independently
valid CLI responses are not substitutes for these facts.

## 8. Tests Added

Focused tests prove:

- digest mismatch prevents subprocess invocation without leaking configured
  values;
- executable mutation during invocation fails closed;
- successful stderr fails closed without leaking its content;
- mismatched detailed policy revision and unsupported source fail closed;
- stable reconciled observations produce a bounded snapshot; and
- sandbox resource-version drift rejects reconciliation.

Existing version, fixed-argv, strict-fixture, stale-policy, and privacy tests
continue to pass.

## 9. Validation Commands And Results

- `cargo check -p workflow-hosted`: passed.
- `cargo test -p workflow-hosted openshell_cli`: passed; 13 focused tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-hosted --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-hosted`: passed; 31 tests.
- `cargo test --workspace`: passed; existing explicitly opt-in live and
  environment-dependent tests remained ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- Binary digest checks are non-atomic and do not establish build provenance.
- Reconciliation spans three subprocesses and is not atomic attestation.
- Exact policy-file byte binding remains unresolved.
- The pinned OpenShell release still lacks required structured image,
  observation, and cleanup facts.
- No live OpenShell environment or smoke proof exists.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of this hardening slice. If
accepted, resolve or prototype the upstream attestation and exact policy-input
binding contract before any live sandbox integration.

Fix-forward note: the focused review found that several failures observed
after subprocess start are classified as `NotStarted`. The hardening phase
therefore requires a bounded attempt-posture blocker fix before acceptance or
live integration. See
[OpenShell CLI Compatibility Hardening Review](OPENSHELL_CLI_COMPATIBILITY_HARDENING_REVIEW.md).

Do not implement `OpenShellNoWriteClient`, run a live sandbox, add credentials,
enable provider writes, select OpenShell automatically, or fork OpenShell in
the review phase.

## 12. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run ID: `run-1786257898819019000-2`.
- Approval ID:
  `approval/run-1786257898819019000-2/implementation-approved`.
- Approval presentation ID: `presentation/f3844eebf1a52c31`.
- Approval presentation hash:
  `f3844eebf1a52c315435a0bfc51d8cda13af0e6e31eb54b9589a88341e7408bf`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: locally provable OpenShell CLI compatibility hardening,
  focused tests, honest documentation, and no live execution.
- Out-of-kernel work: Codex inspected and edited code/docs and ran validation.
  The kernel governed scope and approval but did not invoke OpenShell, edit
  files, execute tests, or perform git/pull-request actions.
