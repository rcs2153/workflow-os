# Hosted Partial-Run Creation Reproduction And Transition Matrix

## Verdict

The early hosted creation recovery defect is independently reproduced on exact
`main` commit `139d5d4b89f06d1df07aa4f8da20c6a6efb339f9`.

Three valid immutable run prefixes survive injected PostgreSQL event-write
failures, but two subsequent exact retries cannot advance any of them to a
lawful wait, queued work item, or terminal report. Every reproduced case leaves
zero hosted work items and zero WorkReports.

This is a P0 continuity defect in the Core-owned hosted creation state machine.
It is distinct from the closed queue-insert recovery path: these failures occur
before the run has a complete scheduled prefix.

## Reproduction Environment

- source commit: `139d5d4b89f06d1df07aa4f8da20c6a6efb339f9`
- source subject: `Prioritize hosted recovery and cross-harness proof (#780)`
- disposable checkout: `/private/tmp/workflow-os-hosted-repro`
- PostgreSQL: `18.6`, built from the official source archive
- archive SHA-256:
  `555610c24d53e4316da5b7d3fc25c279d96856d5e0e23ee308c328c5fa881d9f`
- database: isolated local cluster and database under `/private/tmp`
- ordinary exact-main PostgreSQL shared-state test before fault injection:
  passed
- modified production files: none
- temporary probe: one test-only helper in the disposable checkout

The probe installed a PostgreSQL `BEFORE INSERT` trigger on
`workflow_os.events`, bound the trigger to one exact run ID and event kind,
executed the public `execute_with_hosted_no_write_dispatch` path, removed the
fault, and issued the same immutable request twice. It then inspected durable
events, the rehydrated status, hosted work-item rows, and WorkReport rows.

Command shape:

```text
WORKFLOW_OS_TEST_POSTGRES_URL=<isolated-local-url> \
WORKFLOW_OS_REQUIRE_POSTGRES_TESTS=1 \
cargo test -p workflow-core --test postgres_state_backend \
  postgresql_backend_proves_shared_state_milestone -- --nocapture
```

## Observed Transition Matrix

| Blocked append | Durable events | Rehydrated status | Exact retry 1 | Exact retry 2 | Work items | Reports |
| --- | --- | --- | --- | --- | --- | ---: |
| `RunValidated` | `RunCreated` | `Created` | `executor.hosted_no_write.retry_posture.invalid` | same | 0 | 0 |
| `RunStarted` | `RunCreated`, `RunValidated` | `Validated` | `executor.hosted_no_write.retry_posture.invalid` | same | 0 | 0 |
| `StepScheduled` | `RunCreated`, `RunValidated`, `RunStarted` | `Running` | `hosted.dispatch.scheduled_invocation.invalid` | same | 0 | 0 |

The injected append returns `postgres_state.backend.unavailable` in all three
cases. The failure is therefore exposed to the caller, but removing the fault
does not make the exact request recoverable.

## Source-Of-Truth Analysis

The hosted create path persists the immutable run bundle and then calls four
separate event appends:

1. `RunCreated`;
2. `RunValidated`;
3. `RunStarted`; and
4. `StepScheduled`.

Each append owns a separate PostgreSQL transaction. An interruption can
therefore leave any event prefix durable.

On exact retry, `reconcile_existing_hosted_no_write_run` rehydrates the run and
accepts only:

- `WaitingForApproval`;
- a terminal status; or
- `Running`.

`Created` and `Validated` are rejected without attempting a bounded repair.
For `Running`, the reconciler sets `step_scheduled = true` without proving that
`StepScheduled` exists, then enters dispatch. The dispatch transaction rejects
the missing scheduled invocation. The current implementation therefore
conflates a projected `Running` status with a complete dispatchable prefix.

The durable event stream, not the snapshot status alone, is the authority for
which transition has actually occurred.

## Exact Prefix Classification

| Durable prefix | Current classification | Required classification |
| --- | --- | --- |
| no events | no run | create once from exact immutable request |
| `RunCreated` | valid but stranded | repair exact `RunValidated`, `RunStarted`, and `StepScheduled` transitions in order |
| through `RunValidated` | valid but stranded | repair exact `RunStarted` and `StepScheduled` transitions in order |
| through `RunStarted` | valid but stranded | repair exact `StepScheduled` transition |
| through `StepScheduled` | dispatchable or approval-bound prefix | use existing approval/dispatch reconciliation |
| lawful approval wait | resumable wait | return existing exact approval posture |
| durable work item | dispatched | acknowledge without duplication |
| terminal | terminal replay | acknowledge exact terminal posture |
| malformed, reordered, ambiguous, or request-mismatched prefix | unsafe | fail closed with bounded recovery-required diagnostics |

This classification is intentionally narrow. A status value or event count is
not enough. Repairability also requires exact immutable-bundle binding,
resolved execution-context agreement, event semantics and ordering, actor and
project scope, current required facts, and absence of conflicting durable
records.

## Recommended Core Seam

Add one Core-owned hosted creation-prefix reconciliation operation before the
existing approval/dispatch reconciliation branch.

The operation should:

1. lock or otherwise serialize the exact run and immutable request;
2. load and semantically validate the complete durable event prefix;
3. compare the exact immutable bundle and resolved execution inputs;
4. classify the last lawful creation transition;
5. deterministically rederive each missing event from the immutable request;
6. append only missing transitions with stable event and idempotency identity;
7. continue through the existing approval or dispatch seam only after
   `StepScheduled` is durably proven; and
8. return an explicit repaired, existing-wait, existing-dispatch, terminal,
   conflict, or recovery-required outcome.

The seam must not set `step_scheduled = true` from `Running` status alone. It
must not synthesize history from a snapshot, accept caller-authored repair
events, or add a second queue mechanism.

The focused implementation review must decide whether the missing creation
events can be appended in one fenced PostgreSQL transaction without changing
the event-store contract. If not, the repair may use individually create-only
appends under one recovery lease, but only with deterministic stable event
identity and restart-safe classification after every append. It must not claim
atomicity that the persistence contract does not provide.

## Permanent Test Requirements

The implementation phase must convert this temporary probe into permanent
coverage and add:

- fault injection before and after every creation transition;
- exact second retry idempotency;
- changed workflow, actor, project, policy, resource, and execution-context
  rejection;
- concurrent exact-retry winner election;
- process restart between interruption and retry;
- PostgreSQL backup and restore of at least one early prefix;
- API response and operator-inspection assertions;
- zero duplicate events, work items, invocations, approvals, SideEffects,
  receipts, and reports; and
- the existing 25-job contention case.

## Governed Phase

- workflow: `dg/review`
- run: `run-1794010250799245000-2`
- approval: `approval/run-1794010250799245000-2/review-scope-approved`
- presentation: `presentation/d231dd82fa756fdc`
- approval outcome: granted under delegated-maintainer authority
- scope: reproduction, source analysis, and transition matrix only

## Non-Goals

This reproduction does not:

- modify production code or persisted schema;
- implement recovery;
- authorize provider-mutation broadening;
- add hosted production claims;
- add nested harnesses, agent teams, or recursive execution;
- authorize release or publication; or
- resume Governed Obligation Plan composition.

## Next Phase

Run focused maintainer/security review of this reproduction, exact-prefix
classification, and proposed Core seam. Production implementation remains
unauthorized until that review accepts the recovery boundary and permanent
test obligations.
