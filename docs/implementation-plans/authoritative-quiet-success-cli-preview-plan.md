# Authoritative Quiet-Success CLI Preview Plan

Status: Implemented and accepted with non-blocking follow-ups.

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Authoritative Proportional-Governance Route Dispatcher Plan](authoritative-proportional-governance-route-dispatcher-plan.md)
- [Authoritative Governance Report Consumer Plan](authoritative-governance-report-consumer-plan.md)
- [Authoritative Governance Report Consumer Review](../concepts/AUTHORITATIVE_GOVERNANCE_REPORT_CONSUMER_REVIEW.md)
- [Authoritative Quiet-Success CLI Preview Plan Review](../concepts/AUTHORITATIVE_QUIET_SUCCESS_CLI_PREVIEW_PLAN_REVIEW.md)
- [Terminal Local Report Generation Plan](terminal-local-report-generation-plan.md)

## 1. Executive Summary

Workflow OS now has accepted Core paths that:

- derive one authoritative proportional-governance assessment;
- select quiet proceed, visible proceed, approval required, or denial;
- execute one canonical `DocsCheck` exactly once;
- preserve the selected route and run truth;
- derive a payload-free local-check result reference; and
- generate an in-memory `WorkReport` for terminal outcomes.

The ordinary CLI does not consume those paths. `workflow-os run` still invokes
the existing executor directly, and the authoritative path requires an
explicit `DocsCheckLocalHandler` that is intentionally specific to the Workflow
OS repository. The CLI has no generic, validated, non-ambient local-check
registration source.

External evaluation correctly recommends reducing ceremony for low-risk work.
An explicit CLI preview could make quiet success tangible, but exposing the
current repository-specific handler as a general command would overclaim the
product and create hidden execution configuration.

Therefore this plan defines the desired operator contract and records two
original prerequisites:

1. complete report generation after an approval-required route resumes; and
2. establish an explicit validated CLI check-profile source that does not
   discover or accept arbitrary commands.

Both original prerequisites are implemented and reviewed in isolation. The
focused prerequisite re-review found that they do not yet compose across the
same authority boundary: fresh-run report generation accepts the resolved
explicit project-validation profile, while approval-resume report completion
still accepts only `DocsCheckLocalHandler`. The CLI must not start a run with
one closed profile and then resume its approval through a different handler
contract.

That narrow explicit-profile approval-resume report-completion bridge is now
implemented and reviewed. The CLI preview is also implemented as an additive
`--authoritative-governance` route on `run` and `approve`.

The implementation does not change runtime defaults or add schemas, providers,
OpenShell integration, SideEffect execution, writes, artifacts, or report
persistence.

## 2. Product Decision

The future preview should be an explicit extension of `workflow-os run`, not a
second orchestration command:

```text
workflow-os run <workflow-id> --authoritative-governance
```

The exact flag remains provisional until implementation review.

The flag should mean:

```text
Use one validated, explicitly selected local-check profile.
Let Core derive and select the authoritative governance route.
Render one bounded operator result.
Preserve the durable run and evidence/report posture.
```

It must not mean:

- infer or execute arbitrary repository commands;
- enable every modeled local-check kind;
- approve its own blocking route;
- make proportional governance the default;
- persist or export a WorkReport artifact; or
- authorize provider or filesystem mutation.

## 3. Why The CLI Is Not Ready Today

The current CLI constructs:

- a local state backend;
- an empty or mock-only skill registry;
- a `LocalExecutionRequest`; and
- the existing `LocalExecutor::execute(...)` path.

The accepted authoritative consumer additionally requires:

- a stored immutable run-bundle boundary;
- one explicit `DocsCheckLocalHandler`;
- typed runtime governance facts;
- optional visible-disclosure dependencies;
- report identity and bounded report context; and
- stable local-check reference metadata.

`DocsCheckLocalHandler` validates that it is running from the Workflow OS
repository and executes `npm run check:docs`. Registering it in the product CLI
would not provide a generic existing-repository capability.

Other local-check kinds remain model vocabulary without accepted handlers.
The CLI must not convert model-only command templates into executable authority.

## 4. Goals

- Define one honest, opt-in operator surface for authoritative governance.
- Preserve all four authoritative route variants.
- Give eligible quiet work a concise completion result without an approval
  pause.
- Keep visible disclosure non-blocking and distinct from approval.
- Present approval-required outcomes with the complete existing handoff.
- Render denial as a governed terminal result.
- Include a stable inspect command or run reference.
- Preserve payload-free check evidence and report posture.
- Keep human output concise and retain bounded JSON detail.
- Avoid ambient command discovery, hidden handler registration, and inferred
  execution authority.
- Keep existing `workflow-os run` behavior unchanged without the explicit flag.

## 5. Non-Goals

This plan does not authorize:

- implementation during this planning phase;
- default proportional-governance execution;
- automatic approval or model self-approval;
- arbitrary command flags or shell strings;
- repository script discovery as execution authority;
- automatic local-check registration;
- workflow schema changes;
- runtime configuration files;
- report artifacts or report persistence;
- CLI export commands;
- providers or OpenShell;
- SideEffect execution or provider writes;
- examples or scaffold changes;
- hosted behavior, enterprise administration, or release changes.

## 6. Required Prerequisite: Approval-Resume Report Completion

The accepted report consumer returns `DeferredNonTerminal` when the
authoritative route pauses for approval. That is truthful, but an operator
preview would be incomplete if approval resume could not produce the same
validated terminal report posture.

Before CLI exposure, add and review one explicit approval-resume report
completion path that:

- reuses the original authoritative assessment and approval binding;
- revalidates immutable and resolved execution context;
- requires current approval-presentation proof;
- resumes through the accepted executor path;
- generates a report only after terminal completion;
- performs the accepted decision-time canonical check reassessment exactly
  once;
- cites the exact fresh decision-time local-check result that authorized the
  approval decision;
- does not rerun the check again only for reporting or fabricate a result;
- preserves denial and failure truth; and
- keeps report failure separate from workflow status.

This prerequisite remains local and in memory.

The original request-time result explains why approval was requested. It is
not sufficient terminal authorization evidence after an approval wait. The
accepted approval path intentionally performs a fresh check reassessment before
decision mutation; see the
[Authoritative Approval-Resume Report Completion Plan](authoritative-approval-resume-report-completion-plan.md).

The accepted completion helper currently binds that reassessment to
`DocsCheckLocalHandler`. The generic explicit profile introduced afterward can
enter fresh quiet, visible, approval-required, denied, and report routes, but
cannot yet enter the proof-enforced approval-resume report helper. The required
fix is a closed profile-specific bridge that uses the profile's canonical
handler for the decision-time reassessment and terminal report citation.

## 7. Required Prerequisite: Explicit Check Profile Source

The CLI needs one reviewed source of executable local-check authority.

The first acceptable source must:

- select only an already implemented handler;
- use a canonical `LocalCheckCommandContract`;
- bind executable identity and arguments to the accepted contract;
- validate repository and working-directory posture;
- use sanitized environment handling;
- preserve network and side-effect policy;
- reject missing, ambiguous, or unsupported handlers;
- avoid PATH guessing where executable identity matters;
- never accept an arbitrary command line; and
- be explicit at the invocation boundary.

Possible future sources to evaluate separately:

- one built-in project-validation handler applicable to every Workflow OS
  project;
- one explicit local runtime profile assembled by an embedding caller; or
- reviewed workflow schema fields after a dedicated schema phase.

The Workflow OS-specific `DocsCheckLocalHandler` is not sufficient as the
general product source.

## 8. Candidate CLI Contract

After both prerequisites are accepted, the first preview may extend `run` with:

```text
workflow-os run <workflow-id> --authoritative-governance
```

Candidate optional controls:

- `--run-id <run-id>`;
- existing global `--json`;
- one future bounded check-profile selector, only after its own accepted
  contract exists.

The command must not accept:

- raw executable paths;
- raw argument arrays;
- shell snippets;
- report section prose;
- caller-selected governance route;
- caller-selected check status;
- caller-selected approval outcome; or
- provider credentials.

If no accepted check profile is available, the command must fail before
`RunCreated` with a stable non-leaking next action.

## 9. Invocation And Identity Construction

The CLI should derive ordinary identity from validated project and invocation
state:

- workflow ID from the positional argument;
- run ID from `--run-id` or the existing generator;
- workflow version, schema version, and spec hash from the immutable bundle;
- correlation ID from the existing CLI generator;
- actor as the existing system CLI actor unless a separately validated actor
  surface is approved;
- report ID and local-check result-reference ID from deterministic,
  collision-safe run-scoped construction;
- report contract identity from an explicit built-in preview contract only if
  that contract is separately reviewed.

The CLI must not invent evidence, event, audit, approval, or output references.

## 10. Route Rendering

### Quiet Proceed

Default human output should be one bounded completion block:

```text
status: Completed
governance: proceeded
disclosure: quiet
report: generated_in_memory
inspect: workflow-os inspect <run-id>
```

The output must not dump report sections or check output by default.

### Visible Proceed

Render the bounded accepted disclosure and the same completion fields. Make
clear that no approval was requested.

### Approval Required

Render the existing complete approval request and next action. Do not collapse
the handoff to `WaitingForApproval`.

Aggregate proportional-governance approval and authored workflow or step
approval remain separate gates. When aggregate approval reveals a later
authored gate, the CLI persists and renders a fresh complete presentation for
that exact gate. It does not reuse or imply approval across subjects.

### Denied

Render the stable denial code, terminal status, and inspect command without raw
policy details or payloads.

### Report Failure

Preserve the route and run result. Render the stable report-generation error
code separately and never rewrite a successful or denied workflow outcome.

## 11. JSON Contract

The preview JSON should be additive and explicitly experimental.

Required bounded fields:

- schema version;
- run ID;
- workflow ID;
- terminal or waiting status;
- execution disposition;
- disclosure requirement;
- route kind;
- report posture;
- report ID when generated;
- local-check result-reference ID;
- approval ID when waiting;
- stable error code when report generation failed;
- inspect command or inspectable run reference.

Do not serialize:

- `WorkflowRun` internals wholesale;
- raw WorkReport section text by default;
- stdout or stderr;
- command transcripts;
- policy payloads;
- approval presentation text;
- paths;
- environment values; or
- provider data.

## 12. Workflow Semantics

- The authoritative dispatcher remains the only route selector.
- The canonical check runs at most once for a fresh invocation.
- Existing `run` semantics remain unchanged without the explicit flag.
- Quiet success means no unnecessary interruption, not missing governance.
- Visible disclosure does not become approval.
- Approval remains blocking and proof-enforced.
- Denial remains terminal and cannot be downgraded.
- Report rendering failure cannot erase a durable run.
- The CLI does not append events solely for rendering.

## 13. Privacy And Security

- Use validated Core constructors for all request and report inputs.
- Keep local-check summaries and process output out of CLI output.
- Keep `Debug` output bounded and redacted.
- Reject secret-like report and reference metadata.
- Do not print executable paths, cache paths, source paths, or environment
  details.
- Do not treat local process containment as business authorization.
- Do not claim sandboxing until an execution substrate is separately accepted.

## 14. Test Plan

The implementation tests:

1. existing `run` behavior is unchanged without the flag;
2. missing accepted check profile fails before run creation;
3. quiet proceed emits concise human output and a generated report posture;
4. visible proceed emits disclosure without approval language;
5. approval-required emits the complete approval handoff;
6. denied emits terminal governed denial;
7. one invocation executes the canonical check once;
8. report citation is bound to the actual same-call result;
9. approval resume can produce the terminal report without rerunning the
   original check;
10. report failure preserves the run and route;
11. JSON output is bounded and stable;
12. human output includes an inspect command;
13. stdout, stderr, paths, environment values, and secret-like values do not
   leak;
14. no report artifact or extra filesystem output is created;
15. no provider or SideEffect execution occurs;
16. CLI parser rejects arbitrary command and route-selection inputs;
17. existing CLI tests pass;
18. existing proportional-governance, local-check, report, approval, and
   executor tests pass; and
19. `cargo test --workspace` passes.

## 15. Implementation Sequence

1. Plan and implement approval-resume report completion.
2. Review the approval-resume report path.
3. Plan the first generic, explicit local-check profile source.
4. Implement and review that check-profile source without CLI behavior.
5. Re-review this CLI plan against both accepted prerequisites.
6. Implement and review the closed explicit-profile approval-resume report
   completion bridge identified by the prerequisite re-review.
7. Add the explicit CLI flag and bounded route renderer.
8. Add focused CLI and privacy tests.
9. Run full validation.
10. Perform phase-level review before considering defaults.

Do not combine the prerequisite implementations and CLI exposure into one
change.

## 16. Relationship To User Feedback

Fresh-pull evaluation says the kernel now explains itself well and that the
next product problem is unnecessary ceremony for low-risk work.

This plan agrees. It also preserves the evaluator's trust signal: Workflow OS
must continue distinguishing real execution, mock behavior, advisory posture,
and unsupported capabilities.

The earlier Node 24 integration-check failure and duplicate missing-manifest
diagnostic are already fixed on current `main`. They remain regression
concerns, not blockers for this lane.

## 17. Relationship To OpenShell

OpenShell may later implement the execution-substrate side of a provider-neutral
check profile. It is not required for this CLI preview plan and is not
authorized here.

Before any OpenShell integration, Workflow OS must separately define:

- provider-neutral sandbox request and result contracts;
- policy translation and effective-policy identity;
- sandbox/session and image identity;
- denial, log, and artifact reference durability;
- credential-provider references;
- degraded-isolation behavior; and
- responsibility for lifecycle and security updates.

Do not fork OpenShell for this phase.

## 18. Open Questions

- The first generic executable check profile is now the accepted
  `workflow_os_project_validation` profile.
- Should project validation become a first-class local-check result rather than
  a shell-invoked command?
- Which report contract identity should the CLI preview use?
- Approval resume should remain on the existing `approve` command when CLI
  integration is implemented, but it first needs a closed explicit-profile
  Core bridge so the decision-time check cannot change handler authority.
- How should a generated in-memory report be inspected without introducing
  report persistence?
- Which output fields belong in default human text versus `--json`?
- When can the explicit flag become a profile-controlled default?

## 19. Final Recommendation

Proceed to phase-level implementation review.

The additive `run --authoritative-governance` and
`approve --authoritative-governance` preview is implemented with concise
route-aware output. Ordinary command behavior remains unchanged. Providers,
OpenShell, artifacts, report persistence, SideEffect execution, writes,
schemas, hosted behavior, and release changes remain out of scope.

## 20. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785093370914347000-2`
- approval: `approval/run-1785093370914347000-2/planning-approved`
- presentation: `presentation/f9cb10214ff0b9d5`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- validation: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: architecture inspection, plan authoring, roadmap edit,
  and documentation validation
- missing coverage: the kernel coordinates governance only; it does not inspect
  code, edit files, execute validation, or perform git and PR actions
