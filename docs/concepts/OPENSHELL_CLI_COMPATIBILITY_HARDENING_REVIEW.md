# OpenShell CLI Compatibility Hardening Review

## 1. Executive Verdict

Needs blocker fixes.

The hardening slice materially improves the disconnected OpenShell v0.0.101
compatibility boundary. Executable digest verification, strict successful
stderr handling, detailed policy coherence, resource-version preservation, and
before/policy/after reconciliation are all appropriate and remain within the
approved no-execution scope.

One retry-safety defect blocks acceptance: failures discovered after a
subprocess has started are represented as `NotStarted` in several paths. A
future caller could therefore retry an operation even though OpenShell may
already have created or changed external sandbox state. The transport remains
disconnected, so this does not expose a current live mutation path, but the
attempt posture must be correct before the compatibility boundary can be used
by `OpenShellNoWriteClient` or a live proof.

## 2. Scope Verification

The phase stayed within its approved compatibility-hardening scope.

Implemented:

- expected executable digest configuration and before/after verification;
- rejection of successful nonempty stderr;
- detailed policy source and revision validation;
- sandbox resource-version preservation;
- drift-detecting before/policy/after reconciliation; and
- focused tests and honest documentation.

Not introduced:

- OpenShell installation, gateway, driver, or live sandbox use;
- `OpenShellNoWriteClient` implementation or provider wiring;
- automatic provider selection, arbitrary commands, credentials, or access
  material;
- provider mutations, workflow schemas, examples, or release changes;
- runtime image, OCSF, denied-egress, artifact, or cleanup evidence
  fabrication; or
- an OpenShell fork.

## 3. Binary Integrity Assessment

The typed expected digest and checks before and after each subprocess are a
useful compatibility-integrity improvement. Configuration and Debug output do
not expose the executable path or digest.

The check remains non-atomic and is not build provenance. It does not prove
who built the executable, bind the binary to the recorded upstream commit, or
prevent replacement after the final check. The implementation report states
these limits accurately.

The blocking issue is attempt posture. `verify_binary_digest` uses
`hash_file`, and both read failure and digest mismatch resolve to errors whose
attempt posture is `NotStarted`. That is correct for the pre-invocation check,
but incorrect when the same helper runs after the subprocess returns. A
post-invocation read failure or digest mismatch must be
`MayHaveStarted`.

## 4. Subprocess And Stderr Assessment

The fixed argv, closed stdin, timeout, bounded streams, and nonzero-exit
handling remain conservative. Successful stderr is now rejected without
copying its content into an error.

The rejection currently uses the shared `protocol_error`, which always marks
the attempt `NotStarted`. Because stderr is inspected only after the process
has completed, this must be `MayHaveStarted`.

The same problem exists when a joined stream exceeds its configured bound:
`join_bounded` returns `protocol_error()` after the child has run, producing an
incorrect `NotStarted` posture. Reader failures and nonzero exits correctly use
`MayHaveStarted`.

## 5. Policy And Reconciliation Assessment

Detailed sandbox state now requires:

- a reviewed `sandbox` or `global` policy source;
- a detailed revision matching the current policy version; and
- a present policy payload.

The reconciliation helper compares complete bounded sandbox states around the
effective-policy read and then checks policy version, revision, and source
coherence. This is an appropriately fail-closed drift detector. It remains a
three-command observation rather than an atomic snapshot or execution
attestation, and the documentation says so.

Protocol and drift failures discovered after one or more subprocesses also use
the shared `NotStarted` protocol error. Before live use, attempt posture must
distinguish input failures before invocation from parse, coherence, and drift
failures after invocation. This is especially important for sandbox creation,
where malformed structured output can follow successful external creation.

## 6. Privacy And Error Assessment

No payload leakage was found. Errors remain stable and do not copy paths,
binary digests, stderr, policy payloads, sandbox identities, credentials, or
provider output. Debug implementations redact sensitive identities and
hashes.

The required fix must preserve that posture. It should change only the bounded
attempt classification, not add raw error context.

## 7. Test Quality Assessment

The focused tests cover digest mismatch before invocation, executable mutation,
successful stderr, detailed policy mismatch, unsupported policy source,
reconciled success, and resource-version drift. Existing strict parsing,
version, argv, stale-policy, and Debug tests continue to provide useful
coverage.

Missing blocker regressions:

1. post-invocation executable read failure reports `MayHaveStarted`;
2. post-invocation executable digest mismatch reports `MayHaveStarted`;
3. successful nonempty stderr reports `MayHaveStarted`;
4. output-bound failure reports `MayHaveStarted`;
5. malformed create output after a successful runner call reports
   `MayHaveStarted`; and
6. reconciliation drift after subprocess activity reports
   `MayHaveStarted`.

The tests should also retain the pre-invocation digest-mismatch assertion that
no runner call occurs and the posture is `NotStarted`.

## 8. Documentation Assessment

The implementation report and roadmap accurately describe the transport as
disconnected, non-atomic, and non-attesting. They correctly retain exact policy
input binding, driver-observed image identity, structured observations,
machine-readable cleanup, and live smoke proof as blockers.

The report's statement that post-run digest and stderr failures fail closed is
true, but incomplete with respect to retry posture. This review records the
fix-forward requirement without erasing the original phase evidence.

## 9. Blockers

1. Replace the single-context protocol/digest error mapping with explicit
   pre-invocation and post-invocation attempt posture.
2. Preserve `NotStarted` only where Workflow OS can prove the governed
   operation did not start.
3. Use `MayHaveStarted` for failures observed after process spawn, including
   post-run digest verification, stderr rejection, stream overflow, structured
   response parsing, and reconciliation drift where applicable.
4. Add focused non-leakage and attempt-posture regression tests for those
   paths.

## 10. Non-Blocking Follow-Ups

- Bind the exact policy bytes consumed by OpenShell.
- Establish reviewed binary distribution and provenance beyond a local digest.
- Obtain driver-observed immutable image identity.
- Obtain complete structured execution and denied-egress observations.
- Obtain machine-readable cleanup confirmation.
- Add a live sandbox smoke proof only after the preceding contracts exist.

## 11. Recommended Next Phase

Implement an OpenShell CLI attempt-posture blocker fix.

The phase should change only error classification and focused tests in the
disconnected compatibility transport. It must not install OpenShell, run a
live sandbox, implement `OpenShellNoWriteClient`, add credentials, enable
provider writes, select OpenShell automatically, add schemas/examples, fork
OpenShell, or change release posture.

## 12. Validation Evidence

The reviewed implementation passed:

- `cargo check -p workflow-hosted`;
- `cargo test -p workflow-hosted openshell_cli` with 13 focused tests;
- `cargo fmt --all --check`;
- `cargo clippy -p workflow-hosted --all-targets -- -D warnings`;
- `cargo test -p workflow-hosted` with 31 tests;
- `cargo test --workspace`;
- `npm run check:docs`; and
- all seven required GitHub Actions jobs on the merged implementation.

The review documentation was additionally checked with `npm run check:docs`
and `git diff --check`.

## 13. Governed Review Evidence

- Workflow: `dg/review`.
- Run ID: `run-1786261649862048000-2`.
- Approval ID:
  `approval/run-1786261649862048000-2/review-scope-approved`.
- Approval presentation ID: `presentation/f51b9e94b0e1646d`.
- Approval presentation hash:
  `f51b9e94b0e1646d47b855176c2f9f1463ce5c179b33eeb626e2296694cc444a`.
- Approval outcome: granted by delegated maintainer.
- Out-of-kernel work: Codex inspected implementation, tests, documentation,
  and validation evidence and authored this review. The kernel governed scope
  and approval but did not edit files, run tests, or perform git actions.
