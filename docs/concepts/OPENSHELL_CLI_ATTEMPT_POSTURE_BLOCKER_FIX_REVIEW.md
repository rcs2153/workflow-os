# OpenShell CLI Attempt-Posture Blocker Fix Review

## 1. Executive Verdict

Blocker fixed with non-blocking follow-ups.

The disconnected OpenShell v0.0.101 CLI compatibility transport now preserves
`NotStarted` only where Workflow OS proves that no runner activity occurred.
Failures discovered after subprocess activity use `MayHaveStarted`, preventing
a future caller from treating uncertain external state as a safe automatic
retry. The fix is narrow, non-leaking, and does not authorize live OpenShell
provider wiring.

## 2. Scope Verification

The phase stayed within the approved blocker-fix scope. It changed bounded
attempt-posture classification, moved static validation before runner use,
added focused regression coverage, and updated honest documentation.

It did not install or run OpenShell, implement `OpenShellNoWriteClient`, wire a
live execution provider, add credentials, enable writes, select OpenShell
automatically, change workflow schemas or examples, fork OpenShell, or change
release posture.

## 3. Original Blocker Restatement

The compatibility hardening reused error helpers that always returned
`NotStarted`. That was correct before invocation, but unsafe after a subprocess
had run. Post-invocation executable changes, successful stderr, bounded-output
failure, malformed structured output, policy incoherence, or reconciliation
drift could therefore be mislabeled as safe to retry even though external
sandbox state might exist or have changed.

## 4. Fix Approach Assessment

The implementation passes an explicit `HostedExecutionAttemptPosture` through
the protocol, executable-digest, and file-hashing boundaries. Call sites choose
the posture according to whether runner activity has occurred. Static sandbox
name and policy-path validation now precede the version subprocess.

This is the smallest idiomatic correction. It preserves existing error
categories and privacy behavior while making retry posture explicit at the
point where the transport has the necessary execution context.

## 5. Attempt-Posture Assessment

The reviewed boundary is correct:

- invalid static input and pre-invocation executable open, read, or digest
  failures are `NotStarted` and invoke no runner;
- post-invocation executable mutation or removal is `MayHaveStarted`;
- successful stderr, stream overflow, malformed output, incomplete or
  incoherent security state, and reconciliation drift are `MayHaveStarted`;
  and
- version-response mismatch is conservatively `MayHaveStarted` after the
  version subprocess runs.

The version probe does not itself create the requested sandbox, so this final
classification may cause conservative reconciliation. It is nevertheless safe:
the transport does not overstate proof of non-execution.

## 6. Retry And Reconciliation Safety

The fix removes the identified unsafe retry signal. A future provider consumer
can distinguish provable pre-invocation rejection from uncertainty following
runner activity and require reconciliation for the latter.

This does not make the CLI transport atomic or attesting. Reconciliation still
uses multiple observations, and the pinned CLI still lacks facts required by
the provider contract. The transport therefore remains disconnected.

## 7. Privacy And Error Assessment

No leakage regression was found. Stable errors and Debug output do not copy
subprocess output, stderr, executable paths or digests, policy contents,
sandbox identities, credentials, or provider values. The fix changes bounded
attempt posture only.

## 8. Regression And Test Quality

The focused 17-test OpenShell CLI set directly covers the blocker paths,
including pre-invocation no-runner proof, post-invocation executable mutation
and removal, successful stderr, exact and overflowing stream bounds, malformed
create output, incomplete security state, reconciliation drift, and version
mismatch. The complete `workflow-hosted` suite contains 35 tests, and the
workspace suite remains green.

Two improvements are non-blocking:

1. Add direct attempt-posture assertions to the detailed-policy and
   effective-policy mismatch tests, rather than relying on adjacent protocol
   and reconciliation regressions.
2. Add an end-to-end runner overflow regression if the transport runner later
   exposes a stable test seam beyond the focused `join_bounded` test.

## 9. Documentation Review

The blocker-fix report and roadmap accurately state that the transport is
disconnected, non-atomic, and non-attesting. They do not overclaim policy-byte
binding, binary provenance, driver-observed image identity, complete structured
observations, cleanup confirmation, or a live sandbox proof.

## 10. Blockers

None for acceptance of the attempt-posture blocker fix.

The unresolved upstream attestation facts remain blockers to live provider
wiring, not defects in this bounded fix.

## 11. Non-Blocking Follow-Ups

- Strengthen the two policy-mismatch posture assertions described above.
- Clarify version-probe versus governed-operation attempt semantics if a future
  consumer needs finer-grained reconciliation decisions.
- Continue to avoid a fork while an upstream/API attestation boundary remains
  feasible.

## 12. Recommended Next Phase

Define the smallest upstream/API attestation contract needed for an optional
OpenShell no-write execution provider.

That phase should establish how Workflow OS obtains exact policy input,
driver-observed immutable image identity, complete structured execution and
denied-egress observations, and machine-readable cleanup proof. It must not
wire a live provider, add credentials or writes, select OpenShell by default,
add schemas/examples, claim production readiness, or begin a fork.

## 13. Validation Evidence

The reviewed implementation passed locally:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test -p workflow-hosted` with 35 tests;
- `cargo test --workspace` with existing opt-in live and
  environment-dependent tests ignored;
- `npm run check:docs`; and
- `git diff --check`.

The merged implementation also passed all seven required GitHub Actions jobs
on pull request 422.

## 14. Governed Review Evidence

- Workflow: `dg/review`.
- Run ID: `run-1786264330998741000-2`.
- Approval ID:
  `approval/run-1786264330998741000-2/review-scope-approved`.
- Approval presentation ID: `presentation/ad8a0de5a8b07bca`.
- Approval presentation hash:
  `ad8a0de5a8b07bcad56b5c3e4524d19af7fff8a83deb88b3117e6226afe5d771`.
- Approval outcome: granted by delegated maintainer.
- Phase status: `Completed`.
- Event summary: 39 events comprising one run creation, validation, start,
  resume, and completion; six scheduled and successful skill invocations;
  eight policy decisions; one approval request; and one approval grant. There
  were no retries or escalations.
- Approved scope: inspect the bounded attempt-posture fix, tests, reports,
  prior review, and roadmap; author one focused maintainer review; and preserve
  the disconnected no-write boundary.
- Validation summary: formatting, warning-denied workspace clippy, the complete
  workspace test suite, documentation checks, and diff checks passed. Existing
  opt-in live and environment-dependent tests remained intentionally ignored.
- Out-of-kernel work: Codex inspected implementation, tests, documentation,
  local validation evidence, and merged CI evidence; authored this review; and
  ran validation. The kernel governed scope and approval but did not edit
  files, run tests, or perform git or pull-request actions. The workspace test
  suite used the clean temporary target after the repository target reproduced
  its known macOS loader stall; no test scope was skipped.
