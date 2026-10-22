# Executor Capability-Gated GitHub Read-Only Composition Report

## 1. Executive Summary

Workflow OS now has one crate-private, explicit executor operation that
composes the accepted GitHub repository-metadata read with immutable run state,
policy, current scoped authority, durable source continuity, and one-time
continuation enforcement. The operation uses the existing injected read-only
adapter and returns an in-memory bounded result.

This remains a local, SQLite-backed, no-write vertical slice. It does not alter
ordinary executor behavior or expose a public API, CLI command, schema field,
runtime configuration, ambient credential path, second provider operation, or
provider mutation.

## 2. Scope Completed

- Added a crate-private executor input and closed outcome vocabulary.
- Rehydrated the exact running workflow and immutable bundle from SQLite.
- Derived the selected repository-metadata operation from the immutable step,
  skill, input mapping, adapter requirement, capability declaration, and policy.
- Required an already-granted step approval when the immutable policy requires
  approval.
- Re-evaluated policy inside Core and constructed the adapter precheck there.
- Resolved exact current capability authority and reconciled its durable source
  continuity through the existing SQLite V8 store.
- Bound one governed continuation claim to the exact operation, context,
  authority, approval, policy, and immutable bundle.
- Revalidated the event cursor after claim consumption and before writing the
  invocation-start boundary.
- Appended `SkillInvocationRequested` and `SkillInvocationStarted` before the
  injected provider call.
- Emitted bounded adapter audit and observability telemetry.
- Returned the trusted payload-free capability-use receipt separately from
  adapter telemetry.
- Closed successful and provider-failed one-step runs with truthful existing
  event vocabulary.

## 3. Scope Explicitly Not Completed

No second GitHub read operation, provider write, public/default executor path,
generic adapter framework, workflow schema, CLI, SDK, example, runtime config,
credential discovery, report artifact, hosted identity, OpenShell integration,
nested harness execution, reasoning lineage, or release-posture change was
implemented.

## 4. Executor API Summary

The new operation is crate-private and implemented adjacent to
`LocalExecutor<SqliteStateBackend>`. Its caller supplies an exact required-
context execution binding, contract, current-authority source, expected durable
cursor, bounded repository target, evaluation time, correlation identity,
redaction metadata, and injected `GitHubReadOnlyAdapter`.

The caller does not supply an allowed policy precheck, capability projection,
authority receipt, source continuity posture, or operation binding. Core
derives or validates those values from current durable and immutable state.

## 5. Admission And Ordering Summary

The operation performs this order:

1. validate the required-context binding and contract;
2. rehydrate the running workflow and compare the expected durable cursor;
3. read and validate the exact immutable bundle;
4. resolve the current scheduled step and immutable skill/policies;
5. derive the fixed repository-metadata operation binding;
6. require the existing approval and re-evaluate policy;
7. resolve current authority and reconcile durable source continuity;
8. project and consume the operation-specific continuation claim;
9. revalidate the post-claim event cursor;
10. append write-ahead invocation request/start events;
11. invoke the injected read-only provider once; and
12. emit telemetry, construct the receipt, and close the invocation/run.

Failures through post-claim cursor validation produce zero provider calls.

## 6. Outcome And Event Semantics

The closed result distinguishes:

- blocked before provider entry;
- provider read failure;
- read plus receipt success; and
- read success with receipt failure.

Provider failure appends `SkillInvocationFailed` and `RunFailed`. Full success
appends `SkillInvocationSucceeded` and `RunCompleted`. Receipt failure does not
mislabel the provider as failed or append invocation success; it records the
stable post-read run failure while retaining successful adapter telemetry.

The one-step immutable operation contract excludes `continue` terminal
behavior, so completing the run after full governed success does not claim that
unrelated work completed.

## 7. Evidence, Telemetry, And Receipt Posture

The implementation returns the existing trusted
`CapabilityUseAuthorityReceipt` and emits the existing adapter audit and
observability telemetry. It does not create an `EvidenceReference` for the
receipt because the current evidence taxonomy has no truthful capability-use
receipt kind. Receipt-specific evidence or report citation vocabulary remains
separately scoped.

No provider response body, capability inventory, grant payload, policy payload,
repository identity, credential, header, path, or command output is copied into
workflow events, errors, Debug output, or durable continuity state.

## 8. Concurrency And Replay Summary

The operation combines two independent protections:

- durable source continuity records authoritative source monotonicity; and
- the governed continuation claim permits one consumer for the exact cursor
  and operation commitment.

Focused concurrent consumers produce exactly one provider call. A completed
run cannot replay the operation. A cursor change after claim consumption fails
closed before provider entry. A provider-started invocation without a terminal
outcome remains an ambiguous durable posture and is not automatically retried
by this API.

## 9. Privacy And Error Summary

Errors use stable `executor.github_repository_metadata.*` codes and bounded
messages. Debug output redacts provider results and reports only run posture and
stable error classification. Focused tests scan SQLite main and sidecar files
for a forbidden provider marker and find none.

## 10. Test Coverage

New composition tests prove:

- one exact immutable operation performs one injected provider read;
- ordered write-ahead request/start and success/completion events;
- bounded receipt validation and exactly one audit/observability telemetry row;
- exact terminal replay performs no second provider call;
- immutable target substitution fails before provider entry;
- provider failure remains distinct and non-leaking;
- concurrent exact consumers produce one successful consumer and one provider
  call;
- post-claim cursor mismatch fails closed before provider entry; and
- raw provider and repository markers do not appear in Debug or SQLite files.

Existing current-authority and durable-continuity suites continue to cover
stale, incomplete, revoked, unavailable, prerequisite-bearing, rollback,
generation, watermark, query, and registration substitution failures. Existing
governed-continuation suites continue to cover durable claim and restart
postures.

## 11. Governed Phase Record

- workflow ID: `dg/implement`
- run ID: `run-1792703187522342000-2`
- approval ID: `approval/run-1792703187522342000-2/implementation-approved`
- presentation ID: `presentation/f6235bc86b0d4cf4`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- approval-presentation enforcement: persisted proof enforced
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, no retries, and no escalations
- kernel boundary: governance coordination only; implementation, tests,
  documentation, validation, Git, pull-request, CI, and merge work remained
  outside the kernel

Out-of-kernel work consisted of Rust and documentation edits, focused and
workspace-wide Cargo validation, the documentation check, diff inspection, and
the Git and pull-request operations needed to deliver this phase. No skipped
required checks, provider writes, report artifacts, CLI actions, or simulated
kernel coverage are claimed.

## 12. Validation

- focused executor GitHub repository-read composition tests: passed, 5 tests
- focused `workflow-core` Clippy with all targets and denied warnings: passed
- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed

## 13. Remaining Limitations

- The operation remains crate-private, explicit, local, and SQLite-only.
- No public runtime or workflow schema selects the operation.
- Existing direct adapter APIs remain outside this capability-gated executor
  boundary.
- Receipt construction has no injected failure seam; the closed failure posture
  is preserved but cannot be forced by the focused composition test.
- Existing event and continuation records expose an ambiguous-started posture,
  but this phase adds no automatic recovery or operation journal.
- Process-level concurrency and hosted/multi-worker use remain unsupported.
- The separate draft-pull-request live sandbox proof remains dependency-tracked.

## 14. Recommended Next Phase

Perform the focused maintainer/security review of this executor composition.
Verify operation derivation, admission ordering, post-claim cursor validation,
event truthfulness, concurrent at-most-once behavior, restart ambiguity,
telemetry, receipt posture, privacy, and unchanged default behavior before any
public adoption or second provider operation.
