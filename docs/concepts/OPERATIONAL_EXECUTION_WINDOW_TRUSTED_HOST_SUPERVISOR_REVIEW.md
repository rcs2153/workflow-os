# Operational Execution Window Trusted-Host Supervisor Review

## 1. Executive Verdict

**Needs blocker fixes.**

The phase is narrow, private, and directionally correct. It proves a useful
yield/resume/success path without fabricating workflow completion. Three
security-critical contracts from the accepted plan are not yet enforced:
attempt authority is reusable at executor entry, the authorized operation does
not bind the complete skill invocation or injected executor, and ambiguous
result persistence is returned without the required fresh-connection
reconciliation.

Do not add repeated scheduling, public configuration, provider execution, or
OpenShell until these blockers are fixed and re-reviewed.

## 2. Scope Verification

The implementation stayed within the approved private one-shot scope. It did
not add:

- a scheduler, daemon, queue, polling loop, or repeated dispatch;
- model-turn creation or automatic conversational resume;
- automatic approval or evidence/check bypass;
- provider execution, provider mutation, live adapters, or OpenShell;
- nested harness execution, recursive agents, or agent swarms;
- public runtime configuration, CLI, SDK, schemas, UI, or examples;
- filesystem or PostgreSQL operational-opening support;
- prompts, transcripts, source, command output, or provider payloads; or
- hosted execution, Reasoning Lineage, or release changes.

The implementation remains crate-private, local, explicit, and SQLite-only.

## 3. One-Use Attempt Authority Assessment

The accepted plan requires a private one-use attempt capability that cannot be
reused for another dispatch. The current supervisor accepts both opening and
resumed capabilities by shared reference. `normalize_capability` reconstructs
an owned `AttemptUseCapability`, invokes the executor, and only then asks the
continuity store to record the outcome or yield.

This ordering lets a caller submit the same borrowed capability to the
supervisor more than once. The second call can enter the executor before the
store rejects the already-closed attempt during result persistence. Durable
state remains fail-closed, but external work may already have executed twice.
That violates the one-use dispatch boundary and is a blocker.

The blocker fix must consume dispatch authority before or at executor entry.
At minimum, the private type boundary must take the capability by value and
make reuse impossible for a conforming host. The fix must also prove that a
duplicate or stale supervisor call cannot enter the injected executor, not
merely that duplicate result persistence eventually fails.

## 4. Invocation And Executor Binding Assessment

The supervisor validates workflow ID, run ID, step ID, and a commitment whose
only dynamic fields are those same three identities plus the fixed
`invoke_current_step_skill` operation name.

`SkillInput` also carries workflow version, schema version, spec hash, skill
ID, skill version, correlation ID, and invocation values. Those fields are
caller supplied and are not compared with the immutable run bundle or included
in the operation commitment. The injected `TrustedHostAttemptExecutor` is also
not bound to an authorized skill or handler identity.

Consequently, a caller can preserve workflow/run/step while substituting a
different skill identity, version, spec hash, invocation values, correlation
identity, or executor implementation. The existing substitution test changes
only the step ID and therefore does not protect this boundary.

This contradicts the report's claim that the exact immutable invocation
identity is validated. It is a blocker. The fix should have Core derive the
bounded invocation from the immutable run bundle and current step, or bind a
canonical complete invocation commitment plus the selected executor/handler
identity before authority is issued. Caller-authored `SkillInput` must not be
treated as proof of authorized work.

## 5. Persistence And Reconciliation Assessment

Success, retryable failure, terminal failure, yield, and
ambiguous-may-have-started results are mapped to the accepted projected
continuity APIs. Stable security rejection errors are bounded and do not echo
caller values.

However, each persistence helper propagates a store error directly. The
supervisor does not construct a reconciliation request, open a fresh SQLite
connection, or call `reconcile_projected_operation` when commit
acknowledgement is ambiguous. It therefore cannot always return the current
Core disposition "after durable outcome reconciliation," as the report claims.

This is a blocker because the injected executor may already have performed
work. The fix must reconcile the exact operation commitment and receipt before
the host may decide whether another action is legal. Confirmed absence,
durably committed success, committed security rejection, and corrupt partial
projection must remain distinct and tested.

## 6. Opening-Attempt Origin Bridge Assessment

The opening-created first attempt cannot truthfully reference a prior
`consume_directive` operation. The implementation preserves the accepted
closed five-operation continuity contract and adds a closed durable origin
discriminator: `operational_opening` or `consume_directive`.

Opening-origin attempts resolve to the exact opening operation, attempt, and
window. Resume-origin attempts continue to resolve to a successful directive
consume. The V4 migration only rebuilds the attempt table when pre-opening
window and attempt state is empty, so it does not assign invented opening
authority to historical attempts.

This bridge is acceptable. Its final acceptance remains contingent on the
blocker-fix regression suite keeping both origin families fail-closed under
cross-link, missing-origin, and restart cases.

## 7. Yield, Resume, And Workflow Semantics Assessment

The focused vertical slice correctly demonstrates:

- an atomic opening and first started attempt;
- a turn-boundary yield;
- `ResumeNow` rather than a fabricated approval wait;
- one directive-bound resumed attempt;
- invocation through the existing local `SkillHandler` adapter;
- durable success outcome; and
- a terminal execution-window disposition while the workflow remains
  `Running`.

No `RunCompleted` event is appended by supervisor success. The supervisor does
not mutate project specs or reinterpret workflow pass/fail semantics. This
part of the phase is accepted.

## 8. Result Mapping Assessment

The result vocabulary is appropriately closed: success, retryable failure,
terminal failure, yield, and ambiguous may have started. The existing
`SkillHandler` adapter conservatively maps every handler error to terminal
failure. That mapping is disclosed and may remain narrow for this private
slice, but it needs explicit regression coverage.

Current focused tests execute only yield and success paths. They do not prove:

- retryable failure persistence;
- terminal failure persistence;
- ambiguous-may-have-started recovery;
- missing yield-generation rejection;
- all result-persistence commit-fault and reconciliation postures; or
- rejection of duplicate dispatch before executor entry.

These missing tests overlap the blockers above. The blocker-fix phase must add
the complete closed result matrix rather than relying on lower-level store
conformance alone.

## 9. Privacy And Error Assessment

The private execution context and result use custom redacted `Debug`
implementations. Stable errors identify the failed boundary without echoing
IDs, paths, payloads, metadata, or secret-like values. Persisted origin
metadata contains bounded identifiers and a closed discriminator only.

No raw prompt, transcript, source, spec body, command output, environment
value, credential, authorization header, private key, provider payload, or
reconstructable authority is introduced. No privacy blocker was found.

## 10. Compatibility And Validation Assessment

The implementation does not alter existing executor entry points or public
contracts. The accepted five-operation continuity contract remains closed.
SQLite V4 schema integrity is bound to a new manifest digest, and existing
opening and continuity tests pass.

Recorded validation passed:

- `cargo fmt --all --check`;
- `cargo clippy -p workflow-core --all-targets -- -D warnings`;
- `cargo test -p workflow-core --lib` with 338 passing tests;
- focused supervisor, opening, and continuity-store suites;
- `npm run check:docs`; and
- `git diff --check`.

The workspace suite was started and completed representative CLI, read-only
example, vertical-slice, Workflow Core, and adapter binaries without failure.
It was stopped because this host imposed an approximately four-minute startup
delay per integration binary across 71 binaries. That limitation is disclosed
correctly and must not be described as a full workspace pass. CI should run the
remaining workspace binaries before merge.

## 11. Blockers

1. Make attempt dispatch authority genuinely one-use at executor entry and
   prove duplicate/stale calls cannot invoke the executor.
2. Bind the complete immutable skill invocation and selected executor/handler
   identity; reject every caller substitution before executor entry.
3. Reconcile ambiguous projected result persistence on a fresh connection
   before returning a disposition or allowing another action.
4. Add the complete success, retryable failure, terminal failure, yield,
   ambiguity, duplicate-dispatch, substitution, and commit-fault regression
   matrix.
5. Correct the implementation report's overclaims about exact invocation
   identity and automatic durable reconciliation until the fixes are proven.

## 12. Non-Blocking Follow-Ups

- Preserve the conservative handler-error-to-terminal-failure mapping until a
  separately reviewed typed retry contract exists.
- Keep the supervisor crate-private and SQLite-only through blocker repair.
- Consider extracting a reusable supervisor conformance harness after the
  closed result matrix is accepted.
- Keep scheduler, model-turn, provider, OpenShell, and nested-harness work
  deferred.

## 13. Recommended Next Phase

Perform the **one-shot trusted-host supervisor blocker fix**.

The fix should remain private and bounded to one dispatch. After focused
re-review accepts one-use dispatch, complete immutable invocation binding,
fresh-connection result reconciliation, and the full result matrix, return to
roadmap sequencing. Do not begin repeated scheduling or broaden execution
providers first.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1790859810229496000-2`
- approval:
  `approval/run-1790859810229496000-2/review-scope-approved`
- presentation: `presentation/b1883c17a94c5707`
- presentation hash:
  `b1883c17a94c57077ae8dae765a8bf4142a41c001699346e07e9160560d47917`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- approval presentation enforcement: proof enforced with one durable
  presentation record and an event-trail marker
- review validation: `cargo fmt --all --check`, focused one-shot supervisor
  tests (2 passed), `npm run check:docs`, and `git diff --check` passed
- skipped validation: the full workspace suite was not rerun during this
  review; the implementation phase's partial-workspace limitation remains and
  CI must complete the full matrix before merge
- out-of-kernel work: source inspection, security review, review authoring,
  validation, and git actions were performed by the delegated maintainer; the
  kernel governed the phase but did not inspect code or edit repository files

## 15. Fix-Forward Status

The bounded blocker fix is implemented and awaiting focused re-review. It
consumes dispatch authority by value, validates current attempt state before
executor entry, binds every `SkillInput` field and an explicit executor
identity into authority issued at opening, reconciles ambiguous projected
writes through the existing fresh-connection API, and adds the missing closed
result, replay, substitution, yield, and persistence-fault tests. The original
findings above remain the review record.
