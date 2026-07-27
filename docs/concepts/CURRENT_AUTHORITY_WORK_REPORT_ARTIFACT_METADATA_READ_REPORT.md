# Current-Authority WorkReport Artifact Metadata Read Report

## 1. Executive Summary

Workflow OS now has its first concrete Core-owned consumer behind the private
same-call current-authority boundary.

The implementation reads bounded metadata for one exact `WorkReport` artifact
from an explicit caller-supplied `WorkReportArtifactStore`. It validates the
exact required target and access level, freshly resolves registered current
authority, and reaches the store only when authority and required context are
ready. It reads at most one exact `(run_id, report_id)` record and returns only
report ID, run ID, terminal status, and sensitivity.

The operation remains private. It does not return the contained report, expose
a generic authority callback, integrate with the executor, change
persistence, invoke providers or sandboxes, emit events, or enable writes.

## 2. Scope Completed

- Added a private exact-target metadata-read input.
- Added a private payload-free metadata view.
- Added bounded `Found`, `NotFound`, `Blocked`, `SourceFailure`, and
  `StoreFailure` outcomes.
- Reused the existing private `FnOnce` current-authority use boundary.
- Required the exact target to be declared as required
  `BoundedMetadata`.
- Accepted an explicit `WorkReportArtifactStore` with no hidden state lookup.
- Read one exact artifact only after fresh authority resolution succeeds.
- Revalidated returned artifact identity and model integrity.
- Enforced both contract and execution sensitivity ceilings.
- Reconciled the captured store result with the bounded use outcome and failed
  closed on impossible disagreement.
- Added an instrumented store fixture and focused security/privacy tests.
- Updated the accepted plan and roadmap status.

## 3. Scope Explicitly Not Completed

The phase did not add or change:

- public current-authority APIs;
- generic consumer traits or reusable authority handles;
- report body, section, citation, note, risk, or disclosure access;
- list, search, or arbitrary governed-context dereference;
- executor, runtime-default, local-check, or skill integration;
- production authority sources or durable replay prevention;
- provider, OpenShell, sandbox, SideEffect, or write behavior;
- events, audit projection, new persistence, schemas, SDKs, CLI, examples,
  dependencies, hosted behavior, or release posture.

## 4. Concrete Read Boundary

The private sequence is:

1. validate the execution binding and exact immutable contract;
2. require the requested `WorkReportId` target at required
   `BoundedMetadata`;
3. invoke the existing private same-call current-authority use boundary;
4. rerun registered-source selection, capability resolution, projection, and
   required-context consumption;
5. stop without touching the store unless the result is ready;
6. verify that the exact target satisfaction remains present inside the
   borrowed `FnOnce` capability;
7. read exactly one artifact using the bound run ID and requested report ID;
8. validate the artifact, identity, and sensitivity;
9. project only four bounded metadata fields; and
10. reconcile the captured read with the bounded consumer posture.

The borrowed authority capability never reaches the store or caller.

## 5. Outcome And Error Posture

- `Found` means one validated exact artifact produced bounded metadata.
- `NotFound` means authority was ready and one exact read returned no record.
- `Blocked` means authority or required context was not ready and the store was
  untouched.
- `SourceFailure` means the registered source could not produce a fresh,
  complete, coherent snapshot and the store was untouched.
- `StoreFailure` means authority was ready but the explicit read failed, the
  artifact identity was inconsistent, or the stored sensitivity exceeded the
  authorized ceiling.

Underlying store errors are discarded. Stable validation errors contain no
caller IDs, report content, paths, provider output, or secret-like values.
Impossible capability/result disagreement fails closed as an internal
inconsistency rather than being mislabeled as a store failure.

## 6. Privacy And Redaction

The metadata view contains only:

- `WorkReportId`;
- `WorkflowRunId`;
- terminal `WorkReportStatus`; and
- `WorkReportSensitivity`.

Its Debug implementation redacts both identifiers. It is private,
non-serializable, and does not retain the artifact or report body. The outcome
Debug surface shows only bounded enum posture and redacted metadata.

No report sections, citations, summaries, notes, limitations, risks,
incomplete-work disclosures, workflow hashes, paths, redaction details,
provider payloads, commands, logs, environment values, credentials, tokens, or
private keys are copied.

## 7. Test Coverage

The registered-source unit set now contains 34 tests. New coverage proves:

- ready authority reads one exact artifact once;
- only the four approved metadata fields are exposed;
- Debug does not expose IDs or report text;
- absent artifacts return `NotFound` after one read;
- store errors become bounded non-leaking `StoreFailure`;
- reference-only, optional, and undeclared targets fail before store access;
- revoked target authority and unresolved approval prerequisites block before
  store access;
- stale source and changed run binding block before store access;
- declared reference sensitivity mismatch blocks before store access;
- returned artifact identity or actual sensitivity mismatch fails safely after
  one bounded read;
- repeated calls independently rerun authority and read once each; and
- no write or list operation is performed.

Existing current-authority, required-context, capability, WorkReport, store,
runtime, provider, and workspace tests remain part of the full validation
boundary.

## 8. Validation

The complete repository gate passed:

- focused registered-source tests: passed, 34 tests;
- focused `workflow-core` Clippy with warnings denied: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 9. Remaining Limitations

- The source and consumer remain private and in-memory.
- The operation proves same-call gating only.
- No durable use identity, lock, lease, or atomic consumption exists.
- Authority-source and artifact-store reads are not transactionally coupled.
- Cross-process replay prevention and restart semantics remain unproved.
- The stored artifact is assumed immutable under the existing store contract.
- No public or executor consumer can use this operation yet.

## 10. Recommended Next Phase

Focused maintainer review accepts the private metadata-read implementation in
the
[Current-Authority WorkReport Artifact Metadata Read Review](CURRENT_AUTHORITY_WORK_REPORT_ARTIFACT_METADATA_READ_REVIEW.md).

Do not broaden this operation into public or executor behavior. Return to the
active proportional-governance and quiet-success roadmap lane while providers,
OpenShell, sandbox execution, SideEffects, writes, and new persistence remain
deferred.

## 11. Governed Phase Record

- workflow: `dg/implement`;
- run ID: `run-1785180199858950000-2`;
- approval ID:
  `approval/run-1785180199858950000-2/implementation-approved`;
- approval presentation ID: `presentation/cc371e4df1524027`;
- approval presentation content hash:
  `cc371e4df1524027c656acab610255f48ad88eb5e69da33e3380ead5e54bf2f1`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- phase status: completed;
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
- approval-presentation enforcement: proof persisted before approval;
- out-of-kernel work: the delegated maintainer edited implementation, tests,
  and documentation and ran validation; the kernel governed scope and
  approval but did not edit files, execute checks, mutate git, or access the
  artifact store.
