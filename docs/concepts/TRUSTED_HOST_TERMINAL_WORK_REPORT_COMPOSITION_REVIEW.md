# Trusted-Host Terminal WorkReport Composition Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation stays within the approved private in-memory composition
scope and correctly establishes a payload-free durable result-binding event,
atomic lifecycle persistence, terminal rehydration, deterministic report
identity, and exact replay without process re-execution. One fail-closed
integrity gap remains before the phase can be accepted.

## 2. Scope Verification

The phase stayed within scope. It did not add:

- report persistence or artifacts;
- released CLI report contents or export;
- additional check profiles or arbitrary commands;
- scheduling or automatic activation;
- provider, network, credential, SideEffect, or write behavior;
- automatic approval or policy broadening;
- hosted expansion, OpenShell, nested harnesses, or reasoning lineage;
- workflow schema, examples, or release-posture changes.

The only host-visible addition is fixed generated/failed posture in the
unpublished trusted-host binary.

## 3. Durable Result Event Assessment

`LocalCheckResultRecorded` is appropriately narrow and payload-free. It binds
the result reference to command, profile, step, invocation, attempt, immutable
bundle, and invocation-terminal event identity. It accepts only concrete
`Passed`, `Failed`, and `TimedOut` states and revalidates that constraint during
deserialization.

The event excludes stdout, stderr, summaries, paths, environment values,
source contents, parser payloads, and provider payloads. Its Debug output
redacts bound identity.

## 4. Atomicity Assessment

The SQLite lifecycle path includes the result event in the same transaction as
the continuity projection, invocation terminal event, and run terminal event.
The operation request commitment includes the result event, preventing a
conflicting exact replay from substituting a different binding.

The committed event order is result binding, invocation terminal, then run
terminal. Existing foreground tests observe that order and prove replay does
not append events.

## 5. Composition Assessment

The private Core composition boundary correctly:

- rehydrates the exact requested run;
- requires terminal posture and an immutable bundle;
- rejects bundle ID/version substitution;
- requires exactly one result-binding event;
- checks profile, command, step, and bundle identity;
- requires one referenced invocation-terminal event;
- derives a deterministic report ID and fixed private contract identity; and
- constructs the report through existing validated WorkReport APIs.

The host cannot supply report IDs, contract IDs, summaries, citations,
limitations, handoff text, timestamps, or generator identity.

## 6. Blocker Finding

### B1: Result status is not bound to terminal event semantics and strict order

`trusted_host_terminal_report_bindings` accepts either invocation success or
failure for every concrete result status, and accepts completed, failed, or
canceled run terminal events without checking compatibility. It also requires
only that the run terminal event follows the result event; it does not require
the referenced invocation-terminal event to fall strictly between them.

Therefore a malformed or tampered durable history can pair:

- `Passed` with `SkillInvocationFailed` or `RunFailed`;
- `Failed`/`TimedOut` with `SkillInvocationSucceeded` or `RunCompleted`;
- any concrete check result with `RunCanceled`; or
- a referenced invocation-terminal event ordered before the result event.

The current Core-owned writer produces the correct shape, but report
composition is a trust boundary over durable state and the accepted plan
explicitly requires status-conflicting and binding-conflicting histories to
fail closed. Accepting those combinations would let a report cite contradictory
check and workflow truth.

Required fix:

- require `Passed` + invocation success + `RunCompleted`;
- require `Failed` or `TimedOut` + invocation failure + `RunFailed`;
- reject `RunCanceled` when a concrete local-check result event exists;
- require strict result-event < invocation-terminal-event < run-terminal-event
  ordering; and
- add focused tampered-history tests for each mismatch with stable non-leaking
  errors.

## 7. Workflow Semantics Assessment

Normal generated histories preserve workflow truth: passing checks complete,
failed checks fail, and report composition occurs only after terminal commit.
Report failure does not rewrite run state or append replacement terminal events.
Replay does not rerun the process.

The blocker concerns validation of malformed durable history, not the normal
writer path.

## 8. Privacy And Redaction Assessment

No raw check output, command transcript, project path, source/spec content,
parser payload, environment value, credential, token, approval reason, or
provider payload is copied into the result event or report. Fixed host output
does not reveal report contents or error detail. Stable validation errors are
non-leaking.

## 9. Test Quality Assessment

Strong existing coverage includes:

- passing project-validation and docs-check execution;
- failed concrete check execution;
- payload-free result event presence and immutable bundle binding;
- event ordering on normal successful history;
- exact terminal replay without event growth;
- invalid non-concrete result deserialization;
- bounded host failure output;
- 227 WorkReport tests;
- 13 foreground host tests; and
- the complete workspace suite.

Missing blocker coverage:

- passed-result/invocation-failure mismatch;
- non-passing-result/invocation-success mismatch;
- result/run-terminal mismatch;
- concrete result paired with cancellation; and
- invocation-terminal ordering before the result event.

## 10. Documentation Assessment

The plan and implementation report accurately preserve the in-memory-only,
private, no-write boundary. The implementation report should remain as the
record of work performed; this review and the roadmap now record that final
acceptance is blocked pending the narrow integrity fix.

## 11. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live-provider tests remain ignored
  as designed.
- `cargo test -p workflow-core --test work_report`: passed, 227 tests.
- `cargo test -p workflow-local-host --test foreground_admission`: passed, 13
  tests.
- `npm run check:docs`: passed before this review document and rerun at review
  close.
- `git diff --check`: passed before this review document and rerun at review
  close.

## 12. Governed Review Record

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1792665121042229000-2`
- Approval ID: `approval/run-1792665121042229000-2/review-scope-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/59fd2872b88f78b0`
- Review status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Validation summary: full implementation validation remained green; docs and
  diff checks passed after the review findings were recorded

The agent performed repository inspection, validation, and documentation edits
outside the kernel under the approved review scope. The kernel coordinated the
review gate and durable phase record; it did not inspect code, execute checks,
write files, commit changes, or perform provider activity.

## 13. Recommended Next Phase

Implement the narrow trusted-host terminal WorkReport composition blocker fix,
then perform focused blocker-fix review.

Do not add report artifacts, persistence, released CLI rendering, another
profile, arbitrary commands, scheduling, providers, writes, hosted expansion,
or nested harness execution.
