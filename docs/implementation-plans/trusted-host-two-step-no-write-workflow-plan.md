# Trusted-Host Two-Step No-Write Workflow Plan

Status: planning complete; focused maintainer/security review required before
implementation.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Trusted-Host Local Application Admission Plan](trusted-host-local-application-admission-plan.md)
- [Trusted-Host Workflow Completion Reconciliation Plan](trusted-host-workflow-completion-reconciliation-plan.md)
- [Trusted-Host Project Validation Profile Plan](trusted-host-project-validation-profile-plan.md)
- [Trusted-Host Terminal WorkReport Composition Plan](trusted-host-terminal-work-report-composition-plan.md)
- [Trusted-Host Terminal WorkReport Artifact Plan](trusted-host-terminal-work-report-artifact-plan.md)
- [Trusted-Host Terminal WorkReport Artifact Blocker Fix Review](../concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_ARTIFACT_BLOCKER_FIX_REVIEW.md)

## 1. Executive Summary

The unpublished trusted-host application now proves two independent closed,
one-step, no-write workflows. Each path validates immutable inputs, resolves
current authority, runs one exact local check, records a payload-free result,
terminalizes the run, composes a terminal `WorkReport`, and persists one
validated artifact. This is a complete one-step proof, but it does not yet
prove that the trusted-host boundary can advance a governed workflow from one
successful step to the next.

The next runtime-composition slice should add one exact two-step workflow:

```text
workflow-os project validation
  -> docs check
  -> terminal WorkReport
  -> private SQLite report artifact
```

Both steps must reuse the two accepted closed local-check profiles. Core, not
the host caller, must select the current step, derive the matching profile,
append the first step's successful invocation and the next step's scheduling
state atomically, and terminalize only after the second step succeeds. Failure
at either step must fail the workflow according to the immutable workflow
definition. Exact restart replay must neither rerun a completed step nor skip
an incomplete one.

This plan does not implement anything. It does not authorize arbitrary
commands, another check profile, parallelism, branching, providers, writes,
public CLI behavior, workflow schema changes, examples, hosted expansion, or
nested harness execution.

## 2. Why This Is The Next Executable Proof

The accepted artifact phase completed the prior active delivery item and
explicitly called for the smallest remaining runtime-composition proof. The
dedicated draft-pull-request live sandbox remains dependency-tracked until its
least-privilege sandbox credential is available and must not block unrelated
no-write work.

A two-step closed workflow is the smallest proof that adds runtime capability
instead of another model family:

- it uses only accepted profiles and existing no-write handlers;
- it exercises actual sequential workflow progression;
- it proves restart safety at the boundary between steps;
- it tests whether one final report and artifact honestly summarize both
  checks; and
- it does not require provider, network, credential, write, or public product
  expansion.

## 3. Goals

- Admit exactly one immutable two-step trusted-host workflow.
- Execute project validation first and docs check second.
- Keep Core authoritative for runnable-step selection and profile matching.
- Commit first-step success and second-step scheduling in one SQLite
  transaction.
- Preserve one invocation lifecycle and one local-check result per executed
  step.
- Fail the run honestly when either concrete check fails or times out.
- Resume after process loss without rerunning a committed successful step.
- Compose one deterministic terminal report from both durable check results.
- Persist one exact report artifact through the accepted artifact gate and
  SQLite store.
- Preserve the existing one-step paths without changing their public or
  unpublished compatibility posture.

## 4. Non-Goals

This phase must not add:

- a third local-check profile or arbitrary command execution;
- caller-selected step ordering or caller-authored continuation state;
- parallel steps, branches, loops, dynamic fan-out, or nested harnesses;
- provider access, network access, credentials, SideEffect execution, or
  external writes;
- automatic approval, approval bypass, or delegated-authority broadening;
- retries beyond the already accepted closed trusted-host semantics;
- queue polling, background scheduling, a daemon, or worker discovery;
- released `workflow-os` CLI behavior, report rendering, inspection, or
  export;
- workflow spec or schema changes;
- onboarding scaffold or example changes;
- hosted or PostgreSQL parity claims;
- a new report, artifact, store, event, or current-authority model; or
- release-posture changes.

## 5. Current Boundary

### 5.1 Admission is deliberately one-step

`admit_trusted_host_local_application_operation(...)` rejects a plan or stored
workflow unless it has exactly one step. The request also accepts one
caller-supplied `selected_step_id` and one resolved profile. Those constraints
were correct for the first proof but cannot establish sequential progression.

### 5.2 Lifecycle success always terminalizes the run

`TrustedHostWorkflowLifecycleContext::terminal_events(...)` currently emits
`SkillInvocationSucceeded` followed by `RunCompleted` for success. A two-step
workflow instead needs the first successful invocation to make the second
immutable step runnable without emitting `RunCompleted`.

### 5.3 The host resolves one profile per process entry

`workflow-local-host` currently resolves either `docs-check` or
`workflow-os-project-validation` from explicit arguments. The new path needs a
closed registry containing both accepted profiles, not an arbitrary profile
map or caller-supplied command.

### 5.4 Terminal reporting already supports multiple references

The `WorkReport` and artifact models can represent multiple validation
citations. The trusted-host composer is currently specialized to one concrete
result and must be generalized only enough to require and cite the exact two
durable results in workflow order.

## 6. Selected Workflow Shape

The only new accepted topology is:

| Order | Step | Skill | Closed profile |
| --- | --- | --- | --- |
| 1 | `project-validation` | `local/workflow-os-validate@v0` | `workflow_os_project_validation` |
| 2 | `docs-check` | `local/check-docs@v0` | `docs_check` |

Both steps must use the existing local/read-only policy requirements, fail the
workflow on terminal failure, require no provider capability, and declare an
explicit-zero current-authority and required-context contract.

The implementation may use one dogfood-only workflow or a test fixture. It
must not add the shape to onboarding scaffolds, examples, released schemas, or
default execution.

## 7. Private Host Input

Add one explicit unpublished application mode for the two-step shape. The
exact flag name should follow current parser conventions, but it must be a
closed enum value rather than an arbitrary workflow mode.

The caller may provide:

- governance project root;
- repository root;
- SQLite database path;
- workflow ID and optional idempotent run ID;
- immutable bundle ID;
- actor and correlation ID;
- observed and expiry timestamps;
- explicit `workflow-os` executable path;
- explicit `npm` executable path; and
- optional bounded npm cache directory.

The caller must not provide a step ID, current-step index, next-step claim,
profile ID, command arguments, result, output reference, event cursor,
approval state, capability, or continuation disposition for this mode.

Existing one-step argument shapes remain unchanged. Mixed one-step and
two-step argument families fail before backend mutation.

## 8. Closed Profile Registry

The host resolves both accepted profiles once, before admission, and retains
them for the process lifetime. Core receives a closed registry whose complete
key set is exactly:

- `local/workflow-os-validate@v0`; and
- `local/check-docs@v0`.

The registry must:

- reject missing, duplicate, unknown, or identity-mismatched entries;
- preserve each profile's existing executable, root, timeout, environment,
  network, and output contract;
- expose no generic command constructor;
- keep profile internals redacted from `Debug` and errors; and
- derive the selected profile from the immutable current step inside Core.

## 9. Core-Owned Sequential Transition

Core must derive progression from the immutable workflow and durable event
history in one transaction.

For the first successful step, the transaction must append:

1. the payload-free `LocalCheckResultRecorded` event;
2. `SkillInvocationSucceeded` for the exact first invocation; and
3. `StepScheduled` for the exact second immutable step.

It must not append `RunCompleted`.

For a concrete first-step failure or timeout, the transaction appends the
result, failed invocation, and `RunFailed`; no second step is scheduled.

For the second successful step, the transaction appends the result,
`SkillInvocationSucceeded`, and `RunCompleted`. A concrete second-step failure
or timeout appends the result, failed invocation, and `RunFailed`.

The exact implementation should reuse the existing executor's progression
rules and event validation rather than invent a second workflow semantics
engine. If those rules cannot be reused without a new generic abstraction,
implementation must stop for review rather than copy them loosely.

## 10. Admission And Runnable-Step Selection

The multi-step admission boundary must:

1. load and validate the project;
2. create or rehydrate the exact immutable run bundle;
3. require the exact two-step topology and order;
4. rehydrate the complete current run and event cursor;
5. reject contradictory, duplicate, skipped, or out-of-order step history;
6. derive the only currently runnable step;
7. derive its profile from the closed registry;
8. resolve current authority and required context for that exact step;
9. evaluate current policy and approval posture;
10. prepare one opaque operation for that step; and
11. return only existing bounded prepared, waiting, blocked, denied, or
    terminal-replay posture.

No caller-provided next-step value may participate in the decision.

## 11. Process Loop Boundary

The unpublished process may execute at most two prepared operations for the
explicit run. After one operation returns a committed intermediate-success
posture, it must call the Core admission boundary again with the same run and
immutable bundle identities. That second call freshly rehydrates state and
authority before preparing step two.

The process must not retain a reusable continuation capability across steps or
construct step-two inputs from memory. It must stop on waiting, blocked,
denied, canceled, unsupported, or failure posture. This is a bounded in-process
loop, not queue polling or a general scheduler.

## 12. Atomicity And Crash Recovery

The following invariants are required:

- result, invocation terminal, and next-step scheduling or run terminalization
  commit atomically for each step;
- crash before a step transaction commits permits that exact step to be
  retried under existing one-use execution rules;
- crash after first-step commit rehydrates step two as the only runnable step;
- crash after second-step terminal commit enters terminal replay and performs
  no process execution;
- replay never appends a duplicate result, invocation terminal, step schedule,
  or run terminal event;
- a first-step result cannot be rebound to step two; and
- an artifact cannot be written until the run is terminal.

## 13. Report And Artifact Semantics

The two-step path requires a private v3 report commitment. The existing v2
contract, domain separator, material ordering, report IDs, and artifacts must
remain byte-for-byte stable for both accepted one-step paths. Generalizing v2
in place would make exact replay derive a different artifact for an already
terminal one-step run and could convert a valid stored artifact into a false
conflict.

The v3 material must bind, in order:

- the exact immutable two-step workflow and bundle identities;
- each executed step ID and profile identity;
- each concrete local-check result event;
- each matching invocation terminal event; and
- the run terminal event.

The final private v3 report must cite exactly the available concrete
validation results in workflow order. It must disclose:

- project validation passed or failed;
- docs check passed, failed, timed out, or did not run because the workflow
  terminated earlier;
- no provider activity or SideEffect execution occurred; and
- no released inspection or export surface exists.

For a first-step terminal failure, the v3 report cites only the concrete first
result and explicitly records that the second check was not attempted. It must
not fabricate a missing result citation.

The two-step v3 domain separator and deterministic report identity must be
distinct from v2. No migration or v2 rewrite is authorized. Exact replay of an
existing one-step run must continue to derive the same v2 report and artifact;
exact replay of a two-step run must derive the same v3 report and artifact.

Artifact construction, gates, create-only storage, duplicate reconciliation,
and nonzero host failure semantics remain exactly as accepted. The report
composer must rehydrate one immutable terminal history once and use that same
validated material for report construction and artifact gates, closing the
accepted non-blocking double-read follow-up where practical.

## 14. Failure Semantics

- Invalid topology or registry mismatch fails before process execution.
- Out-of-order, skipped, duplicated, or contradictory history fails closed.
- Missing handler/profile resolution fails before opening execution.
- A concrete check failure is workflow truth, not an internal host error.
- Internal execution failure without a concrete result does not fabricate a
  result or terminal report.
- Report generation or artifact persistence failure returns nonzero host
  posture without changing terminal workflow truth.
- Errors expose fixed codes only and never copy paths, commands, output,
  source, policy text, credentials, or caller-provided prose.

## 15. Privacy And Redaction

The new path must not persist or print:

- stdout, stderr, command transcripts, or check summaries;
- source, spec, parser, provider, or response payloads;
- project, repository, executable, cache, database, or artifact paths;
- environment values, credentials, authorization headers, keys, or tokens;
- approval reasons or policy source text; or
- arbitrary caller-authored report prose.

Existing payload-free result references, event commitments, report citations,
redaction metadata, and artifact sensitivity remain authoritative.

## 16. Required Tests

Focused implementation tests must prove:

1. the exact two-step workflow completes in order and persists one artifact;
2. first-step success schedules step two without completing the run;
3. first-step failure records one result, fails the run, and never invokes
   step two;
4. second-step failure records both results and fails the run honestly;
5. exact restart after first-step commit invokes only step two;
6. exact terminal replay invokes neither step and reconciles one artifact;
7. crash before first-step commit does not produce partial progression;
8. duplicate first- or second-step terminal input is idempotent;
9. skipped, reordered, duplicated, or substituted step history fails closed;
10. caller-supplied step/profile fields are unavailable in the new mode;
11. missing or extra registry entries fail before state mutation;
12. each step uses its exact accepted executable/root/timeout contract;
13. report citations bind both durable results in workflow order;
14. first-step failure reports the second check as not attempted without a
    fabricated citation;
15. two-step reports use the distinct v3 commitment and deterministic identity;
16. exact one-step replay preserves existing v2 report and artifact identity;
17. artifact failure leaves terminal run and event history unchanged;
18. existing one-step docs-check and project-validation behavior is unchanged;
19. output, `Debug`, serialization, and errors remain non-leaking; and
20. full workspace, docs, hosted, provider, and state regressions pass.

## 17. Candidate Implementation Sequence

1. Add the closed two-profile registry and exact topology validator.
2. Add Core-owned current-step derivation for the selected shape.
3. Generalize the trusted-host lifecycle success transition to distinguish an
   intermediate successful step from terminal success.
4. Commit the intermediate result, invocation success, and next-step schedule
   atomically in SQLite.
5. Add the bounded two-operation unpublished process loop.
6. Add a distinct private v3 two-step report composition path while preserving
   the existing v2 one-step material and identity unchanged.
7. Reuse the accepted artifact path without changing its store or gates.
8. Add focused crash, replay, order, privacy, and one-step regression tests.
9. Perform focused maintainer/security review before any public adoption.

## 18. Open Questions For Implementation Review

- Can the generic executor's progression helper be reused directly without
  widening its public surface?
- Should intermediate success have a private typed outcome or be represented
  only by fresh re-admission posture?
- Can report composition and artifact gate input share one validated terminal
  history object in this slice?
- Is an injected crash seam needed to prove the first-step atomic boundary, or
  can existing SQLite transaction tests prove it without production code?

None of these questions authorizes scope expansion.

## 19. Acceptance Criteria

- The plan selects exactly one two-step no-write workflow.
- Existing profile identities and handlers are reused.
- Core, not the caller, derives step order and matching profile.
- First-step success does not terminalize the run.
- Per-step progression is atomic, restart-safe, and idempotent.
- Final report and artifact bind the complete terminal history honestly.
- Existing one-step behavior remains compatible.
- No providers, writes, public CLI, schemas, examples, hosted expansion,
  arbitrary commands, parallelism, or nested harness behavior is authorized.

## 20. Recommended Next Phase

Perform focused maintainer/security review of this plan. If accepted, implement
the two-step no-write trusted-host vertical slice exactly as reviewed, starting
with Core-owned topology/current-step derivation and atomic intermediate-step
progression before changing the host process loop or report composer.
