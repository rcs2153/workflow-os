# Trusted-Host Project-Validation Profile Plan

Status: planning blockers corrected; focused maintainer/security re-review is
next.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Generic Explicit Local-Check Profile Source Plan](generic-explicit-local-check-profile-source-plan.md)
- [Trusted-Host Local Application Admission Plan](trusted-host-local-application-admission-plan.md)
- [Trusted-Host Workflow Completion Reconciliation Plan](trusted-host-workflow-completion-reconciliation-plan.md)
- [Trusted-Host Local Application Admission Implementation Report](../concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_IMPLEMENTATION_REPORT.md)

## 1. Executive Summary

The accepted trusted-host local application proves one closed, one-step,
no-write `DocsCheck` profile through immutable admission, current-authority
resolution, opaque one-shot execution, atomic invocation lifecycle, workflow
terminalization, and exact replay. The next runnable roadmap item is one
additional closed profile, not a generic local-command runtime.

This plan selects the already implemented
`WorkflowOsProjectValidation` profile. The unpublished
`workflow-local-host` process should be able to validate the exact Workflow OS
project whose immutable workflow authorizes the step, using one explicitly
supplied and content-bound Workflow OS executable. Core must continue to own
profile resolution, authority and required-context proof, immutable execution
binding, process admission, lifecycle events, and terminal truth.

The implementation remains local, synchronous, SQLite-backed, no-write, and
unpublished. It does not add a released CLI path, arbitrary command execution,
automatic approval, provider access, OpenShell, hosted behavior, or another
profile family.

## 2. Selection Decision

Select:

```text
profile: workflow_os_project_validation
command kind: workflow_os_project_validation
skill: local/workflow-os-validate@v0
logical command: workflow-os validate
```

Do not add a new profile identifier. Reuse the reviewed
`ExplicitLocalCheckProfileSelection::workflow_os_project_validation()` and its
canonical `WorkflowOsProjectValidationLocalHandler`.

## 3. Why This Profile Is Next

Project validation is the smallest profile that meaningfully generalizes the
trusted-host proof:

- it applies to every valid Workflow OS project rather than only this
  repository's documentation toolchain;
- it already has a closed command contract, handler, immutable declaration,
  same-call local-check result, and proportional-governance composition;
- it requires no network, provider, credential, source write, or arbitrary
  shell input;
- its executable and working directory can be bound before process entry; and
- it exercises a second real command identity without creating a command
  registry or scheduler.

Cargo, npm, TypeScript, integration, provider, and repository-discovered check
profiles remain out of scope.

## 4. Existing Boundaries To Preserve

Implementation must compose the existing accepted boundaries:

- `ExplicitLocalCheckProfileId::WorkflowOsProjectValidation`;
- canonical `workflow_os_project_validation_model_only()` contract;
- `WorkflowOsProjectValidationLocalHandler` executable-content binding;
- immutable run-bundle publication and exact stored-bundle replay;
- one-step trusted-host admission and opaque prepared-session handoff;
- registered current-authority source resolution;
- operational-window opening, dispatch reservation, and continuity;
- atomic invocation request/start/terminal events and run terminalization; and
- fixed payload-free host outcomes and errors.

The implementation must not duplicate project validation in the host crate or
construct workflow events from application arguments.

## 5. Closed Profile Interface

The unpublished host adds one closed profile selector:

```text
--profile docs-check
--profile workflow-os-project-validation
```

The selector is an exhaustive enum-like match. Unknown values fail before
backend opening, project loading, run creation, or process execution.

Selection follows this exact compatibility rule:

- `--profile workflow-os-project-validation` is mandatory for the new path;
- `--profile docs-check` explicitly selects the existing path; and
- omitting `--profile` is a temporary, tested compatibility alias for the
  already accepted docs-check path only.

Project-validation arguments without its explicit selector, mixed
profile-specific argument families, and unknown selectors fail before backend
opening or state creation. The host must not infer authority from ambient
files, executable names, or argument coincidence. The omission alias is an
unpublished compatibility measure, not a released default-activation promise.

For `workflow-os-project-validation`, the host accepts one explicit
`--workflow-os-executable` path. It must not accept command arguments, shell
text, environment overrides, timeout overrides, output bounds, or alternate
contract fields. Existing docs-check arguments remain compatible.

The selected project-validation working directory must resolve to the exact
governance project root used by the execution request. A caller cannot govern
one project while validating another. Canonical path equality must be checked
without exposing either path in errors or Debug output.

## 6. Command And Execution Identity

The existing canonical profile identity remains authoritative:

```text
profile_id: workflow_os_project_validation
skill_id: local/workflow-os-validate
skill_version: v0
command_id: local-check/workflow-os-validate
arguments: [validate]
network: disabled
source writes: forbidden
environment: sanitized minimal
```

The resolved execution fingerprint must continue to bind:

- executable path and executable content hash;
- fixed argument sequence;
- exact governed project working directory;
- sanitized environment;
- timeout; and
- canonical command contract.

Executable replacement or governed-root substitution after resolution fails
before process entry.

## 7. Workflow Shape

Only one immutable one-step workflow is accepted. Its selected step must:

- match the caller-selected step ID;
- reference `local/workflow-os-validate@v0`;
- declare the canonical project-validation command contract;
- have no adapter;
- declare no SideEffect event or lifecycle event;
- have no before-skill invocation hook in this slice; and
- use existing supported terminal behavior.

The implementation may use a test fixture or a dogfood-only workflow to prove
the shape. It must not register a default workflow, modify onboarding
scaffolds, or expose the profile through the released CLI.

## 8. Required Context And Current Authority

The existing explicit-zero authorization is intentionally docs-check-specific.
Implementation must not simply relax that condition to accept every resolved
profile.

Add one exhaustive Core-owned profile eligibility decision that proves the
project-validation profile requires, for this exact operation:

- zero capability grants;
- zero capability-availability facts;
- zero governed-context references; and
- no provider or mutation authority.

The governed project root and executable are execution-bound inputs, not
governed-context references. Their exact private commitments must be included
in the execution binding and current-authority source configuration.

The registered source must use a distinct stable source identity and harness
contract identity for project validation. It must not reuse docs-check labels.
Empty inventories mean that the exact closed operation requires no current
authority facts; they are not a synthetic grant and cannot satisfy policy,
approval, evidence, check, dispatch, or operational-window requirements.

Any unknown, non-empty, stale, conflicting, or profile-mismatched requirement
fails before operational preparation.

## 9. Policy And Approval

Existing policy and authored approval semantics remain authoritative:

- policy denial returns bounded denied posture before process execution;
- authored approval pauses before operational preparation;
- admission does not grant or synthesize approval;
- a waiting run can re-enter only after a separately governed decision changes
  durable state; and
- exact replay must not duplicate approval, invocation, or terminal events.

This phase does not add proportional-governance automatic approval or delegated
self-approval.

## 10. Check And Evidence Posture

The actual same-call `LocalCheckResult` is the check truth. The existing
general result model remains unchanged because proportional-governance and
other authoritative APIs legitimately consume non-passing results as data.

Trusted-host execution must introduce one private Core-owned closed-profile
adapter between resolved handler invocation and skill-attempt completion. The
adapter:

1. invokes the exact resolved handler once and obtains one
   `LocalCheckResult`;
2. converts only `LocalCheckStatus::Passed` into the existing bounded
   `SkillOutput`;
3. converts `Failed`, `TimedOut`, and internal handler failure into stable,
   non-leaking execution failure before lifecycle terminalization;
4. never reruns the process;
5. never persists stdout, stderr, summaries, or command transcripts in failure
   events or errors; and
6. applies identically to both the docs-check and project-validation
   trusted-host paths.

The implementation must otherwise preserve:

- fixed command-contract identity;
- bounded success/failure status;
- bounded output reference derived by the existing handler path;
- no raw stdout or stderr persistence;
- no command transcript evidence; and
- no conversion of failed or missing output into success.

The durable invocation and workflow events are audit state, not a substitute
for a `WorkReport` or an `EvidenceReference`. The implementation must not claim
that project source, parser payloads, or command output were captured as
evidence.

## 11. Workflow Lifecycle And Replay

Reuse the accepted lifecycle composition unchanged:

1. admission validates and binds the immutable run;
2. operational opening atomically appends invocation request;
3. dispatch atomically appends admission and invocation start;
4. the exact process executes once;
5. terminal outcome atomically commits continuity, invocation terminal event,
   and run terminal event; and
6. exact terminal replay returns terminal posture without another process or
   event.

Project-validation success may complete only the accepted one-step workflow.
Validation failure must produce the existing bounded terminal-failure path.
Retryable, yielded, ambiguous, stale, mismatched, or incomplete state must not
be silently converted into completion.

## 12. Terminal Report Posture

The current trusted-host local application does not generate or persist a
`WorkReport` or report artifact. Adding the second profile must not obscure
that limitation.

For this phase:

- terminal workflow state and bounded output references are required;
- `WorkReport` generation remains absent;
- host output must not imply that a final report exists; and
- phase documentation must explicitly disclose the missing report artifact.

Before this unpublished path becomes a released operator surface or grows
beyond closed proofs, plan a separate terminal-report composition phase that
uses the exact durable run and same-call check reference without rerunning the
command or copying raw output.

## 13. Privacy And Error Boundary

Application-visible output remains fixed and payload-free. Debug, Display,
errors, events, and host output must not expose:

- executable or project paths;
- executable hashes or private execution fingerprints;
- environment values;
- command output or source contents;
- workflow input payloads;
- approval reasons or policy text;
- credentials, tokens, or authorization headers; or
- internal authority-source details.

Profile mismatch, root substitution, executable drift, unsupported topology,
authority incompleteness, and lifecycle conflict use stable bounded errors.

## 14. Implementation Sequence

Implement as one bounded vertical slice:

1. add the closed host profile selector and profile-specific required
   arguments;
2. resolve project validation through the existing profile source;
3. prove governed-root equality and exact immutable skill/contract identity;
4. add a separate Core-owned explicit-zero eligibility and source identity for
   project validation;
5. generalize trusted-host admission messages and identity construction only
   where exhaustive profile matching requires it;
6. add the private trusted-host closed-profile result adapter and use it for
   both accepted profiles;
7. invoke the unchanged opaque prepared-session and lifecycle path;
8. add actual foreground-binary tests for success, failure, timeout, waiting,
   replay, selector compatibility, and substitution; and
9. update roadmap and implementation report.

Do not refactor the trusted-host application into a generic command host.

## 15. Test Plan

Focused tests must prove:

- actual foreground binary executes one valid project-validation run;
- the supplied executable receives exactly `validate` in the exact governed
  project root;
- successful validation completes the workflow and persists the expected
  ordered lifecycle;
- validation failure fails the workflow without raw output leakage;
- timeout fails the workflow without raw output leakage;
- nonzero docs-check exit also produces `SkillInvocationFailed` and
  `RunFailed`, never success or completion;
- failed and timed-out paths execute once, emit fixed bounded failure data,
  and exact replay executes no process and appends no event;
- exact terminal replay executes no process and appends no event;
- authored approval waits without process execution;
- policy denial executes nothing;
- explicit profile selection accepts only the two closed selectors;
- omitted profile remains a tested compatibility alias for docs-check only;
- project-validation arguments without its explicit selector, mixed argument
  families, unknown profiles, and mismatched profile resolvers fail before
  state creation;
- governed-root substitution fails before process execution;
- executable replacement after resolution fails closed;
- immutable skill, version, command declaration, selected step, actor, bundle,
  and correlation substitution fail closed;
- project-validation receives only its own source and contract identities;
- non-empty or unknown current-authority requirements cannot enter the
  explicit-zero path;
- passing docs-check behavior and existing no-profile argument compatibility
  remain unchanged, while non-passing docs-check truth is corrected;
- output and errors do not leak paths, command output, source contents,
  environment values, or secret-like markers;
- no `WorkReport`, report artifact, provider call, source write, or released
  CLI behavior is created; and
- workspace tests remain green.

## 16. Validation

The implementation phase must run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused `workflow-local-host` and `workflow-core` tests;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Repository-required CI remains acceptance evidence for the final commit.

## 17. Strict Non-Goals

Do not add:

- released `workflow-os` CLI behavior or default activation;
- arbitrary executable, argument, command, environment, or handler input;
- command discovery from repository metadata;
- additional check families;
- automatic or delegated approval;
- scheduler, daemon, detached worker, or general job discovery;
- provider access, provider mutation, network access, or credentials;
- source writes, SideEffect execution, or report artifacts;
- OpenShell, containers, nested harnesses, or agent teams;
- PostgreSQL or hosted parity;
- schemas, SDK changes, public examples, or release posture changes; or
- a claim that the final validation-to-process-entry race is eliminated.

## 18. Acceptance Criteria

The implementation is acceptable only when:

- the unpublished host supports exactly the existing docs-check profile and
  the selected project-validation profile;
- only passing local checks can become successful trusted-host skill attempts;
- failed and timed-out checks terminalize the invocation and run as failure
  without rerun or raw-output persistence;
- profile selection follows the exact explicit-selector and temporary
  docs-check compatibility-alias rule;
- project validation operates on the exact governed project;
- Core proves profile-specific explicit-zero authority and required context;
- exact process, immutable, actor, and lifecycle bindings fail closed on
  substitution;
- actual binary tests prove success, failure, wait, denial, and replay;
- docs-check behavior remains unchanged;
- no raw payload or path leaks;
- no WorkReport or provider/write behavior is claimed; and
- all local and required CI validation passes.

## 19. Recommended Next Phase

Perform a focused maintainer/security re-review of the corrected plan. If
accepted, implement the trusted-host project-validation profile and shared
trusted-host result adapter as one complete foreground-binary vertical slice.
Do not begin another profile or broader trusted-host scheduler work first.
