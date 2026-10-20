# Trusted-Host Workflow Completion Reconciliation Plan

Status: implemented; focused maintainer/security review is next. The accepted
design now makes opening, dispatch, and terminal outcome transactions own the
matching invocation lifecycle events and makes the terminal outcome
transaction own workflow terminalization. Required-output validation selects
success or fixed terminal failure before the terminal transaction is applied.

## 1. Executive Summary

The unpublished trusted-host local application can now admit and execute one
canonical no-write docs-check operation against authoritative SQLite state.
The operation records its authorized attempt outcome and closes its execution
window, but an operational `Terminal` result deliberately leaves the workflow
run `Running`. That separation prevented the foreground host from fabricating
workflow completion, but it is not a complete runtime lifecycle.

The next bounded implementation should add one Core-owned workflow-lifecycle
composition mode to the existing opening, dispatch, and attempt-outcome
transactions. The opening transaction should bind and append the invocation
request. The dispatch transaction should append invocation start before
returning executor authority. The terminal attempt-outcome transaction should,
while the exact in-memory result is still present, atomically commit the
continuity result, invocation terminal event, and workflow terminal event. For
the existing one-step docs-check profile only, all three transactions should
freshly verify the exact run, immutable bundle, step, invocation, window,
attempt, cursor, and executor binding or fail closed without partial lifecycle
events.

The foreground application must continue to receive only its existing bounded
posture vocabulary. It must not receive workflow-transition authority, a
reusable completion token, raw output, or mutable event input.

The private Core and SQLite boundary described by this plan is implemented.
No public surface or broader runtime authority is implied.

## 2. Problem Statement

Current trusted-host execution has two correct but incomplete truths:

1. authorized execution continuity owns whether one attempt was admitted,
   yielded, succeeded, failed, or became ambiguous; and
2. the workflow event stream owns whether the workflow is running, waiting,
   completed, failed, or canceled.

The accepted application slice updates the first truth but intentionally does
not update the second. Its public bounded result collapses both successful and
failed closed windows to `operation.terminal`. A caller therefore cannot and
must not infer `RunCompleted` from application return.

Leaving every successfully executed docs-check run `Running` forever is also
incorrect. It prevents terminal reports, accurate inspection, restart
classification, and later lifecycle composition. The missing boundary is not
another application callback. It is an authoritative Core reconciliation that
joins the two existing state machines without weakening either one.

## 3. Goals

- Give Core sole ownership of workflow terminal reconciliation after the
  accepted trusted-host operation reaches an authoritative terminal posture.
- Keep the first implementation restricted to the existing canonical,
  one-step, no-write docs-check profile.
- Preserve the exact `SkillOutput` in memory until reconciliation so success
  can use the existing bounded output reference without copying command output.
- Freshly verify the durable run, immutable bundle, selected step, invocation,
  executor, operational window, terminal attempt, and event cursor.
- Atomically append the complete workflow terminal event sequence or append
  nothing.
- Make exact replay idempotent and conflicting replay fail closed.
- Keep application output bounded and payload-free.
- Define crash, race, replay, substitution, privacy, and migration tests before
  implementation.

## 4. Non-Goals

- No implementation in this planning phase.
- No public or released CLI, SDK, schema, or workflow-spec field.
- No application-authored workflow events or completion authority.
- No generic workflow-completion API.
- No scheduler, daemon, discovery, polling, detached worker, or background
  process.
- No automatic approval or delegated-authority broadening.
- No second local-check profile, adapter, provider call, provider mutation, or
  SideEffect execution.
- No OpenShell integration, nested harness execution, or agent-team runtime.
- No retry-policy implementation or escalation broadening.
- No WorkReport generation or report-artifact write.
- No PostgreSQL, filesystem, or hosted parity.
- No raw process output, command transcript, provider payload, source content,
  environment value, credential, token, or secret storage.
- No claim that the final validation-to-process-entry race is eliminated.
- No example, release-posture, or product-readiness change.

## 5. Existing Boundaries To Preserve

The implementation must compose, not replace:

- immutable run-bundle publication and exact stored-bundle validation;
- Core-owned admission and explicit-zero current-authority proof;
- one-use opening and resumed capabilities;
- atomic dispatch reservation before executor entry;
- authoritative continuity attempt outcome persistence;
- bounded redispatch and local timer ownership;
- the one-shot, opaque prepared application session;
- workflow event transition validation and deterministic snapshot projection;
- create-only and exact-replay posture; and
- redaction-safe application failures and output.

`OperationalExecutionWindow` terminal posture remains distinct from workflow
terminal posture. Reconciliation may connect them only after fresh proof of the
exact accepted terminal outcome.

## 6. Ownership Decision

The lifecycle composition must execute inside Workflow OS Core across the
three existing authoritative transaction points: opening, dispatch admission,
and attempt-outcome persistence. Terminal workflow mutation occurs inside the
attempt-outcome transaction before the supervisor returns and before the
prepared session projects the coarse `TrustedHostLocalApplicationOutcome`.

This location is required because Core still owns, in memory:

- the Core-derived invocation and invocation-attempt identities;
- the exact executor result and optional successful `SkillOutput` before
  outcome persistence;
- the exact operational locator;
- the immutable run and step binding;
- the injected executor commitment; and
- the SQLite backend that owns both continuity and workflow events.

The `workflow-local-host` library and foreground binary remain consumers of a
bounded result. They must not receive a run builder, event builder,
terminal-event list, output payload, completion capability, or reconciliation
operation ID. A later transaction after application return is explicitly
forbidden because a successful output would no longer be recoverable.

## 7. Candidate Private Boundary

The implementation should add one private closed lifecycle context,
provisionally `TrustedHostApplicationWorkflowLifecycleContext`, assembled by
admission from the immutable execution plan. It should contain:

- the exact workflow/run/step/skill identity;
- the immutable run-bundle binding;
- the existing invocation idempotency key;
- one Core-generated invocation ID;
- a rule for deriving one Core-owned `SkillAttemptId` from the exact
  continuity attempt ID;
- actor, correlation, invocation, and executor commitments; and
- the fixed one-step terminal mapping selected by the accepted profile.

The lifecycle context may be carried only through Core's private operational
entry, supervisor, and SQLite store calls. It must not be public, serializable,
cloneable authority or a generic callback.

The existing operations should gain narrowly typed private composition inputs:

1. opening receives the validated invocation-request event candidate;
2. dispatch reservation derives and receives the invocation-start event
   candidate after the continuity attempt ID exists; and
3. attempt-outcome persistence receives the exact in-memory executor result
   and derives the invocation-terminal plus run-terminal event candidates.

No input may accept caller-selected workflow status, event kinds, failure
text, output reference, attempt classification, policy result, approval result,
or authority fact. The output reference comes only from the successful exact
executor result and is validated against the immutable skill definition before
the terminal transaction.

## 8. Source-Of-Truth Matrix

| Concern | Authoritative source | Forbidden substitute |
| --- | --- | --- |
| Workflow/run/step identity | immutable bundle plus durable run | application arguments alone |
| Current workflow status | rehydrated event stream | application return string |
| Invocation identity | Core-generated ID committed with opening | restart-generated ID |
| Invocation attempt identity | domain-separated derivation from continuity attempt ID | caller ID |
| Executor identity | committed executor identity and current validation | handler label |
| Attempt outcome | terminal continuity attempt record | coarse `Terminal` posture |
| Success output | in-memory output from the exact admitted attempt | persisted process output |
| Event cursor | freshly rehydrated durable run | cached pre-execution cursor |
| Terminal transition | attempt-outcome SQLite transaction | later host callback |
| Replay result | exact durable terminal event sequence | prior success response |

## 9. Supported First-Slice Outcomes

The first implementation should support only outcomes reachable from the
accepted `LocalSkillAttemptExecutor` and canonical docs-check profile. Before
those outcomes are possible, the lifecycle is recorded as follows:

### Invocation request and start

- Fresh opening atomically appends `SkillInvocationRequested` immediately
  before `OperationalExecutionWindowOpened` in the same transaction.
- The request uses the invocation ID and idempotency key already owned by the
  immutable execution plan.
- Existing-run admission must resolve exactly one matching durable invocation
  request; absence, duplication, or conflicting identity blocks before
  preparation.
- Dispatch reservation atomically appends
  `AuthorizedExecutionAttemptDispatchAdmitted` and then
  `SkillInvocationStarted` before it returns the one-use dispatch capability.
- `SkillAttemptId` is derived by Core through a domain-separated commitment to
  the exact continuity attempt ID. The application cannot select either ID.
- If dispatch commits but execution never starts, restart observes admitted
  but incomplete work and follows the existing ambiguity/recovery posture. It
  does not append a terminal invocation event.

### Successful attempt

Reconciliation may complete the workflow only when all of these are true:

- the redispatch loop stopped as terminal;
- the exact authoritative attempt is `Succeeded`;
- the execution window is closed;
- exactly one in-memory `SkillOutput` is present;
- required output validation succeeds against the immutable skill definition;
- the workflow is the accepted one-step docs-check topology; and
- the current workflow run is still `Running` at the expected cursor.

The attempt-outcome transaction should append, in this order:

1. the existing authorized-execution continuity outcome projection;
2. `SkillInvocationSucceeded` with the validated bounded `output_ref`; and
3. `RunCompleted`.

### Terminal failure

Reconciliation may fail the workflow only when:

- the exact authoritative attempt is `TerminalFailure`;
- no successful `SkillOutput` is present;
- the execution window is closed;
- the topology and current cursor still match; and
- the run is still `Running`.

The attempt-outcome transaction should append, in this order:

1. the existing authorized-execution continuity outcome projection;
2. `SkillInvocationFailed` with a fixed, non-leaking trusted-host failure
   record; and
3. `RunFailed` with the same stable failure classification.

The first slice must not preserve raw handler errors because the accepted
executor deliberately reduces them to terminal failure.

### Unsupported or unsafe outcome

`RetryableFailure`, `Yielded`, `AmbiguousMayHaveStarted`, missing output on
success, output on failure, multiple attempts, non-terminal window posture,
changed cursor, changed bundle, or changed invocation binding must not produce
a workflow terminal event. The helper must return a stable blocked or
recovery-required result and leave the run non-terminal.

Retry and ambiguity semantics require separate planning. They must not be
silently converted into failure or success.

## 10. Atomicity And Replay

SQLite should extend the three existing authoritative operations rather than
add a later reconciliation transaction:

### Opening transaction

The accepted registered-authority opening operation should validate and append
the exact invocation-request event immediately before its existing operational
opening projection. Both events commit or neither does. Exact replay validates
both.

### Dispatch transaction

The accepted dispatch-reservation operation should derive the skill-attempt ID
from its exact continuity attempt, append its existing dispatch-admission
projection, then append invocation start. Both events commit before executor
entry or neither does. A committed dispatch never yields a second capability.

### Attempt-outcome transaction

For the closed application lifecycle mode, the accepted projected
attempt-outcome operation should:

1. load and validate the exact run and expected current cursor;
2. load and validate the immutable bundle, window, attempt, dispatch,
   invocation, executor, and actor bindings;
3. validate the executor result and required outputs while the exact
   `SkillOutput` remains in memory;
4. apply the shared continuity attempt/window transition semantics;
5. build and validate the exact continuity, invocation-terminal, and
   run-terminal event sequence;
6. persist the continuity state and all events in one SQLite transaction; and
7. return only after commit or exact fresh-connection reconciliation.

The existing generic continuity-only mode remains unchanged for all other
callers. A generic callback is not added.

The blocker fix does not presume that no internal schema change is required.
The implementation must first prove whether existing operation request JSON,
event rows, deterministic idempotency keys, and operation reconciliation can
bind the complete lifecycle context. Any needed internal migration requires
explicit implementation scope and review; no public schema exposure is
authorized.

Exact replay of each operation validates its complete associated event
sequence and returns existing truth without appending events. Any different
invocation, attempt, output reference, bundle, cursor, actor, executor, or
event sequence under the same operation fails closed.

If commit outcome is ambiguous, reconciliation reopens SQLite and accepts only
the complete expected operation plus event sequence. Partial or conflicting
state is corrupt and is never repaired piecemeal.

## 11. Event And Audit Posture

The workflow event stream remains the lifecycle source of truth. The first
slice should reuse existing `SkillInvocationSucceeded`,
`SkillInvocationFailed`, `RunCompleted`, and `RunFailed` vocabulary rather
than add a parallel trusted-host completion event.

Durable event commit must not depend on an external audit, log, or telemetry
sink. Existing event-derived audit and observability projections may consume
the committed events afterward. A sink failure must not create uncertainty
about whether workflow completion committed.

The implementation report must disclose whether sink delivery remains
best-effort, replay-derived, or separately retryable. It must not claim atomic
external signal delivery.

## 12. Privacy And Redaction

- Persist only the existing bounded output reference, never command output.
- Use a fixed failure code and message for trusted-host terminal failure.
- Do not expose run, step, window, attempt, path, command, environment, bundle,
  hash, or output data through application errors or Debug output.
- Keep reconciliation input and exact bindings crate-private and Debug-redacted.
- Treat local paths and executable identity as sensitive even though this first
  operation is read-only.
- Do not serialize a completion capability or the in-memory `SkillOutput`.

## 13. Failure Semantics

- Stale workflow cursor: fail closed with no event append.
- Missing or changed immutable bundle: fail closed with no event append.
- Non-terminal or mismatched attempt/window: return blocked or
  recovery-required, never workflow completion.
- Missing successful output: fail closed; do not fabricate an output ref.
- Invalid required outputs: record terminal failure only if that validation and
  failure mapping are part of the same accepted transaction design; otherwise
  block the first implementation and fix the plan.
- Ambiguous SQLite commit: reconcile the complete expected sequence on a fresh
  connection.
- Exact replay: return the existing terminal run without new events.
- Conflicting replay: fail closed without mutation.
- Failure before the terminal transaction: continuity and workflow remain
  pre-terminal; the executor result is not claimed committed.
- Failure after an ambiguous terminal commit: reopen and reconcile the complete
  operation and event sequence; never rerun the skill merely because the
  caller did not observe success.
- Committed dispatch with no terminal outcome: preserve the existing
  admitted/ambiguous recovery posture and never fabricate completion.

There is no lawful state in the corrected first-slice design where a terminal
successful continuity attempt is committed but its required workflow result
exists only in lost memory.

## 14. Test Plan

The implementation must include direct tests for:

1. opening commits invocation request and operational opening atomically in
   exact order;
2. dispatch commits admission and invocation start atomically before executor
   entry;
3. Core-derived invocation and attempt identities survive exact restart and
   cannot be substituted;
4. successful docs-check commits continuity success, invocation success, and
   run completion atomically;
5. terminal handler failure commits continuity failure, invocation failure,
   and run failure atomically;
6. the workflow is terminal when the foreground call returns terminal;
7. exact replay of opening, dispatch, and outcome appends no events;
8. concurrent terminal outcome calls have one winner and one exact replay;
9. injected failure at every transaction phase leaves either the complete
   transaction or no transaction;
10. restart after committed success needs no in-memory output and observes the
    complete terminal event sequence;
11. ambiguous commit reconciliation accepts only the complete exact sequence;
12. a partial or orphan invocation lifecycle is rejected as corrupt;
13. stale workflow cursor rejects before mutation;
14. changed actor, bundle, step, invocation, executor, window, attempt, or
    output reference rejects before mutation;
15. success without output and failure with output reject;
16. retryable, yielded, and ambiguous attempt states do not terminalize the
    workflow;
17. output validation remains bound to the immutable skill definition and
    cannot disagree with committed continuity state;
18. no raw command output, path, environment value, payload, or secret appears
    in state, Debug, errors, stdout, or stderr;
19. the foreground application still exposes only bounded posture strings;
20. approval wait, policy denial, blocked state, cancellation, and terminal
    replay behavior remain unchanged;
21. the accepted executable-identity negative proofs remain green; and
22. workspace, documentation, and integration validation pass.

## 15. Proposed Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Fix planning blockers, if any, without writing runtime code.
3. Add one private Core lifecycle context and extend the existing opening,
   dispatch, and outcome operations with a closed application-lifecycle mode.
4. Compose terminal workflow mutation inside outcome persistence before the
   supervisor returns and before bounded application projection.
5. Add focused success, failure, replay, race, crash, substitution, and privacy
   tests.
6. Run full repository validation and produce an implementation report.
7. Perform focused implementation/security review before any additional
   profile, scheduler, public surface, provider, or sandbox work.

## 16. Open Questions For Review

- Can existing operation request JSON, workflow event rows, and idempotency
  fields bind each complete atomic lifecycle sequence without a migration?
- Should required-output validation failure map to the fixed trusted-host
  terminal failure in the first slice, or block reconciliation until the
  executor preserves a typed failure class?
- Which existing Core event-construction helper can create request, start, and
  terminal candidates without exposing a generic event append API?
- How should committed events trigger existing audit and observability sinks
  without coupling sink availability to durable workflow truth?
- Is a fresh trusted-time observation required for reconciliation, or is the
  already committed terminal attempt plus current workflow cursor sufficient?
- What exact bounded result should distinguish successful reconciliation,
  exact replay, blocked recovery, and durable corruption before application
  projection?

## 17. Final Recommendation

Proceed next to a focused maintainer and security re-review of the corrected
plan.

If accepted, implement only the private Core-plus-SQLite reconciliation for the
existing one-step docs-check profile. Do not broaden trusted-host profiles,
public configuration, scheduling, providers, OpenShell, nested harnesses,
automatic approval, schemas, examples, or release posture.
