# OpenShell CLI Attempt-Posture Blocker Fix Report

## 1. Executive Summary

The disconnected OpenShell v0.0.101 CLI compatibility transport now reports
retry posture according to whether Workflow OS can prove that the governed
operation did not start. Static input and pre-invocation executable-integrity
failures remain `NotStarted`. Failures discovered after subprocess activity
are now `MayHaveStarted`, so a future caller cannot mistake uncertain external
sandbox state for a safe automatic retry.

This is a compatibility-boundary blocker fix only. It does not install or run
OpenShell, implement `OpenShellNoWriteClient`, wire a live execution provider,
add credentials, enable writes, or claim runtime attestation.

## 2. Blocker Fixed

The hardening review found that the shared protocol and executable-digest
error helpers always returned `NotStarted`, including after a subprocess had
run. That classification was unsafe for:

- post-invocation executable read or digest failure;
- successful subprocess output accompanied by stderr;
- bounded-output overflow;
- malformed or incoherent structured output; and
- observable drift during multi-command reconciliation.

Those paths now return `MayHaveStarted`. `NotStarted` is retained only when
the transport rejects static inputs or the executable fails integrity checks
before the runner is called.

## 3. Implementation Approach

The transport now passes an explicit `HostedExecutionAttemptPosture` through
its bounded protocol, digest-verification, and file-hashing helpers. Call sites
select the posture from their actual position relative to subprocess activity.

Static sandbox-name and policy-path validation now occurs before the version
subprocess. This preserves a provable no-attempt boundary and avoids invoking
OpenShell for invalid local input. Version-response failures remain
conservatively `MayHaveStarted` because the compatibility subprocess ran even
though the requested sandbox operation did not follow.

No raw subprocess output, path, digest, policy payload, sandbox identity, or
provider value was added to errors or Debug output.

## 4. Attempt-Posture Boundary

`NotStarted` now means Workflow OS proved that the governed operation did not
reach the command runner. It applies to:

- invalid static sandbox names;
- non-absolute policy paths; and
- executable open, read, or digest mismatch before invocation.

`MayHaveStarted` now applies to:

- executable open, read, or digest mismatch after invocation;
- successful subprocess stderr;
- stdout or stderr bound failure;
- malformed, incomplete, or incoherent structured responses;
- unsupported or mismatched detailed policy state;
- sandbox or policy reconciliation drift; and
- version-response mismatch after the version subprocess runs.

This posture does not prove that OpenShell created a sandbox. It prevents a
future caller from treating uncertainty as proof of non-execution.

## 5. Privacy And Error Posture

Errors retain stable categories and bounded attempt posture without carrying
raw values. Secret-like or private subprocess output, executable paths,
digests, policy contents, sandbox identifiers, and provider values remain
absent from errors and Debug formatting.

The fix does not make CLI responses authoritative attestation. It only makes
the disconnected compatibility transport's retry signal conservative.

## 6. Tests Added And Strengthened

Focused regressions prove:

- pre-invocation digest mismatch returns `NotStarted` and calls no runner;
- static invalid input returns `NotStarted` and calls no runner;
- post-invocation executable mutation returns `MayHaveStarted`;
- post-invocation executable removal returns `MayHaveStarted`;
- successful stderr returns `MayHaveStarted` without leaking its marker;
- an exact stream bound succeeds and overflow returns `MayHaveStarted`;
- malformed create output returns `MayHaveStarted`;
- incomplete security state returns `MayHaveStarted`; and
- reconciliation drift and version mismatch return `MayHaveStarted`.

The focused OpenShell CLI set now contains 17 tests. The complete
`workflow-hosted` crate contains 35 tests.

## 7. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-hosted`: passed; 35 tests.
- `cargo test --workspace`: passed; existing explicitly opt-in live and
  environment-dependent tests remained ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

The Rust validation used a clean temporary Cargo target because the existing
repository target cache stalled in the macOS loader before the Rust test
harness. The clean target completed successfully; no product code or test was
skipped because of that local tooling condition.

## 8. Scope Explicitly Not Completed

This fix did not:

- install OpenShell, start a gateway, select a driver, or run a sandbox;
- implement or wire `OpenShellNoWriteClient`;
- add arbitrary commands, credentials, access material, or provider writes;
- add automatic provider selection, workflow schemas, SDK fields, examples,
  or release changes;
- bind exact policy bytes or establish binary build provenance;
- obtain driver-observed immutable image identity;
- obtain complete OCSF, denied-egress, artifact, or cleanup observations; or
- fork OpenShell.

## 9. Remaining Known Limitations

- Executable digest checks remain non-atomic and do not establish provenance.
- Reconciliation remains a multi-command observation rather than an atomic
  execution attestation.
- Exact policy-byte binding remains unresolved.
- The pinned CLI response surface does not provide the complete runtime image,
  observation, and cleanup facts required by the provider contract.
- No live OpenShell smoke proof exists.

## 10. Recommended Next Phase

Perform a focused maintainer review of the attempt-posture blocker fix.

Do not wire a live provider, add credentials or writes, select OpenShell by
default, add schemas/examples, or begin a fork during that review. If the fix
is accepted, continue with the smallest upstream/API attestation contract that
can supply exact policy input, driver-observed image, complete structured
observations, and machine-readable cleanup proof.

## 11. Governed Phase Evidence

- Workflow: `dg/blocker`.
- Run ID: `run-1786262272611514000-2`.
- Approval ID:
  `approval/run-1786262272611514000-2/fix-approved`.
- Approval presentation ID: `presentation/661663a32a03236b`.
- Approval presentation hash:
  `661663a32a03236b29b715a62ade54268bf20609ceded484ba53fa50c9af52f1`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: bounded attempt-posture correction, focused regression
  tests, honest documentation, and no live execution.
- Phase status: `Completed`.
- Event summary: 39 events comprising one run creation, validation, start,
  resume, and completion; six scheduled and successful skill invocations;
  eight policy decisions; one approval request; and one approval grant. There
  were no retries or escalations.
- Validation summary: formatting, warning-denied clippy, the 35-test
  `workflow-hosted` suite, the complete workspace suite, documentation checks,
  and diff checks passed. Existing opt-in live and environment-dependent tests
  remained intentionally ignored.
- Out-of-kernel work: Codex inspected and edited code/docs and ran validation.
  The kernel governed scope and approval but did not invoke OpenShell, edit
  files, execute tests, or perform git or pull-request actions. Rust validation
  used a clean temporary target after the existing local target cache stalled
  before test-harness startup.

## 12. Review Status

The focused maintainer review is complete in
[OpenShell CLI Attempt-Posture Blocker Fix Review](OPENSHELL_CLI_ATTEMPT_POSTURE_BLOCKER_FIX_REVIEW.md).
It accepts the blocker fix with non-blocking test-strengthening follow-ups and
recommends defining the smallest upstream/API attestation contract before any
live provider wiring.
