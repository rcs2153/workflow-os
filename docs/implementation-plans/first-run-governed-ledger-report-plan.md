# First-Run Governed Ledger/Report Plan

Status: Implemented. This plan follows the accepted `workflow-os init-repo-governance` scaffold review. The first implementation adds `workflow-os first-run`, a local, explicit first-run governed ledger/report posture command for a newly scaffolded or otherwise valid Workflow OS project. The implementation emits a validated report-ready context rather than fabricating a terminal `WorkReport`, because no workflow run has occurred.

## 1. Executive Summary

`workflow-os init-repo-governance` now gives a normal existing repository a valid local Workflow OS project envelope. That solves the missing-manifest setup problem, but it does not yet deliver the immediate ledger/report value that makes Workflow OS useful before a user has authored mature workflows.

The implemented first-run governed ledger/report mode produces a bounded report-ready context from explicit local inputs and safe project observations. It applies the Governed Work Pattern out of the box: goal, context, evidence posture, skipped checks, approval posture, side-effect posture, risks, incomplete work, report section closure, and review-only workflow recommendations.

This implementation does not authorize arbitrary shell execution, real local handler registration, provider calls, provider writes, automatic workflow generation, automatic workflow registration, report artifact writing, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

## 2. Goals

- Make Workflow OS useful immediately after `workflow-os init-repo-governance`.
- Produce an evidence-aware first-run `WorkReport` or report-ready context through validated model constructors.
- Preserve the boundary that agents or humans execute unsupported repository work while Workflow OS governs.
- Use local deterministic observations only.
- Disclose missing evidence, skipped checks, unavailable references, and unsupported capabilities.
- Recommend candidate workflows, checkpoints, evidence requirements, approval gates, side-effect declarations, and report sections.
- Keep recommendations review-only.
- Avoid raw repo content, raw command output, provider payloads, environment values, credentials, and token-like values.
- Preserve existing workflow pass/fail semantics.
- Keep the implementation local, explicit, and opt-in.

## 3. Non-Goals

Do not implement in this phase:

- automatic arbitrary shell command execution;
- automatic local check execution;
- command-output evidence attachment;
- real local skill handler registration;
- live adapter calls;
- GitHub, Jira, CI, or provider writes;
- branch, commit, PR, issue, comment, label, merge, rerun, dispatch, or cancellation behavior;
- automatic workflow generation;
- automatic workflow registration or promotion;
- report artifact writing by default;
- general CLI report rendering;
- workflow schema changes;
- sidecar external-repo governance;
- patch artifact modeling;
- capability-aware blocked-vs-failed execution classification;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Foundation

The following foundations are already implemented and should be reused:

- `workflow-os init-repo-governance` scaffold.
- Local project validation.
- Sequential local executor.
- Approval pause/resume.
- Durable local runtime state and event history.
- `WorkReportContract` and `WorkReport` models.
- In-memory terminal local report generation helper.
- In-memory runtime result exposure helper.
- Explicit `LocalExecutor::execute_with_report(...)`.
- Explicit report artifact store and gated artifact-writing helpers.
- EvidenceReference core model and selected attachment paths.
- SideEffect, hook, typed handoff, local check, and adapter telemetry citation vocabularies.

The first-run mode should compose existing primitives. It should not introduce a new primitive family unless implementation finds a small missing input type is necessary.

## 5. Recommended User Experience

After scaffold:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
```

Candidate command name: `workflow-os first-run`.

Alternative command names:

- `workflow-os report first-run`
- `workflow-os observe`
- `workflow-os init-repo-governance --first-run`

Recommendation: use `workflow-os first-run` for the first implementation if it fits CLI conventions. It is short, explicit, and avoids implying continuous monitoring.

The command should:

1. Load and validate the local Workflow OS project.
2. Confirm the project contains or is compatible with the first-run governance scaffold.
3. Build a bounded first-run context from safe local inputs.
4. Construct a `WorkReport` or report-ready context through existing constructors.
5. Print bounded operator-facing output that summarizes the report posture.
6. Return a stable exit code and stable non-leaking errors.

The command should not run the generated workflow automatically unless separately scoped. Running the approval-gated mock workflow remains an explicit user action:

```sh
workflow-os --mock-all-local-skills run local/first-run-governance
```

## 6. Input Boundary

Allowed first-run inputs:

- project manifest identity;
- workflow IDs and versions from the local project;
- skill IDs and versions from the local project;
- policy IDs from the local project;
- declared schema version;
- local project validation diagnostics;
- safe repository metadata derived without shell execution;
- generated scaffold presence;
- explicitly supplied actor/system actor;
- explicitly supplied correlation ID if available;
- explicitly supplied bounded handoff notes;
- explicitly supplied bounded limitations, risks, and incomplete-work disclosures.

Safe repository metadata may include:

- current working directory basename or a user-supplied repository label;
- whether common directories exist, by name only;
- whether Workflow OS scaffold files exist;
- counts of workflow, skill, policy, and test specs;
- whether a `.git` directory exists, as a boolean only.

Disallowed inputs:

- raw source file contents;
- raw command output;
- raw test output;
- raw parser payloads;
- raw provider payloads;
- raw CI logs;
- raw Jira/GitHub bodies or comments;
- raw Git diffs;
- environment variable values;
- credentials, tokens, authorization headers, private keys, and secret-like values;
- absolute private paths unless explicitly redacted or avoided.

## 7. Report/Ledger Semantics

The first-run mode should behave like a local governed ledger entry plus WorkReport posture, not like a production audit service.

It should record or disclose:

- what project was loaded;
- what scaffold files were found;
- what validation was run or skipped;
- what evidence was available;
- what evidence was missing;
- what checks were recommended but not executed;
- what approvals exist in the generated workflow;
- what side effects are unsupported or skipped;
- what work remains incomplete;
- what risks remain;
- what workflow candidates are recommended next.

It must not claim:

- the kernel executed repository commands;
- tests passed unless validation evidence exists;
- provider state was inspected unless an adapter call actually occurred;
- external writes are supported;
- workflow recommendations are active workflows;
- report output is a production compliance artifact.

## 8. WorkReport Population Policy

The first-run report should include all v1 report sections.

| Section | First-run source | Required behavior |
| --- | --- | --- |
| Work performed | Scaffold/project validation posture and first-run mode invocation. | Say this is first-run governance mapping, not repository implementation work. |
| Evidence considered | Existing safe project files and validation diagnostics/references where available. | Cite references when stable; otherwise disclose no stable evidence references. |
| Decisions made | No autonomous decisions by default. | State that workflow recommendations are review-only. |
| Policy gates evaluated | Generated/default policy IDs and validation posture. | Do not imply policy-enforced execution unless a run occurred. |
| Approvals | Generated workflow approval requirements. | State whether approval-gated workflow exists; do not fabricate approval decisions. |
| Validation and quality checks | `workflow-os validate` result or diagnostics. | Include validation result status; do not copy raw spec contents. |
| Side effects | Current local/no-write posture. | State none/skipped/unsupported. |
| Incomplete or deferred work | Missing evidence, skipped checks, no real handlers, no artifacts, no provider reads. | Make gaps explicit. |
| Known limitations | Local-only scaffold/report posture. | Mention no command execution, no writes, no hosted behavior. |
| Risks | Misreading mock workflow as real execution, missing evidence, skipped checks. | Keep bounded and non-alarmist. |
| Operator handoff notes | Next steps for user/agent. | Include validate, optional mock run, and recommended workflow review. |

## 9. Evidence And Citation Policy

Allowed citations:

- project manifest reference;
- workflow spec reference by path or stable local reference if safe;
- validation diagnostic/reference ID if available;
- EvidenceReference ID only if supplied or already constructed by a reviewed path;
- workflow event ID only if a run is explicitly supplied or executed in a separately scoped path;
- local check result ID only if supplied by a reviewed local check path;
- side-effect ID, hook ID, typed handoff ID, adapter telemetry reference only if explicitly supplied by existing reviewed paths.

Rules:

- Do not recreate `EvidenceReference` values implicitly.
- Do not fabricate IDs.
- Do not cite raw repo content.
- Do not cite command output.
- Missing optional references should become explicit section text such as `not available`, not fake missing-citation records.
- Required citation slots should be deferred until contract-driven citation enforcement is designed.

## 10. Workflow Recommendation Policy

The first-run report should carry opinions about what should be formalized next.

Recommendations may include:

- candidate workflow names;
- purpose of each candidate workflow;
- recommended approval gates;
- recommended evidence requirements;
- recommended validation/check obligations;
- recommended side-effect posture;
- recommended report sections;
- risks of leaving the work ad hoc;
- conflicts with existing local workflows if trivially detectable by ID/purpose.

Recommendations must:

- be review-only;
- be bounded and non-secret;
- not create files;
- not register workflows;
- not mutate `workflow-os.yml`;
- not alter roadmap state;
- not approve themselves.

## 11. CLI Output Posture

The first implementation may print bounded first-run operator output because the command's purpose is onboarding. This is not a general WorkReport renderer.

Allowed output:

- stable success/failure status;
- report ID if a validated report is created;
- validation status;
- generated section headings or bounded summaries;
- recommended next commands;
- candidate workflow recommendations.

Disallowed output:

- raw spec contents;
- raw command output;
- raw parser payloads;
- raw provider payloads;
- raw diffs;
- environment values;
- secrets or token-like values;
- verbose internal Debug output.

JSON output may remain deferred unless the existing CLI JSON pattern makes it small and safe. If JSON is added, it must be preview-marked and tested for redaction.

## 12. Persistence And Artifact Posture

Default first-run mode should not write report artifacts.

Allowed:

- in-memory `WorkReport` construction;
- bounded CLI output;
- optional future explicit `--write-artifact` flag only after a separate artifact plan/review.

Disallowed:

- automatic local report artifact writing;
- state backend report writes;
- hidden local caches;
- provider writes;
- generated workflow files from recommendations;
- persistent catalog mutations.

## 13. Error Handling

Errors must be stable and non-leaking.

Recommended stable codes:

- `cli.first_run.manifest_missing`
- `cli.first_run.project_invalid`
- `cli.first_run.validation_failed`
- `cli.first_run.report_generation_failed`
- `cli.first_run.unsupported_project_shape`
- `cli.first_run.secret_like_input`

Errors must not include:

- raw file contents;
- raw paths beyond safe repo-relative names;
- raw diagnostics that include secret-like values;
- command output;
- provider payloads;
- environment values;
- tokens or credentials.

Report generation failure should not mutate runtime state or imply workflow failure unless a workflow was explicitly run and failed independently.

## 14. Relationship To Existing Scaffold

`workflow-os init-repo-governance` should remain scaffold-only.

The first-run mode should consume the scaffold as one supported project shape, but it should not be hardcoded so tightly that future sidecar or custom projects cannot use it.

If the scaffold is missing, the command should provide an actionable next step:

```text
No Workflow OS project was found. Run `workflow-os init-repo-governance` first.
```

If a Workflow OS project exists but is not the generated scaffold, the command may still produce a conservative report if it can validate the project safely. If it cannot, it should fail with a stable unsupported-project-shape error.

## 15. Test Plan

Future implementation tests should cover:

- first-run command/helper works after `init-repo-governance`;
- generated report uses existing `WorkReport` constructors;
- report contains all v1 sections;
- validation status is represented without raw spec content;
- missing evidence is disclosed explicitly;
- skipped checks are disclosed explicitly;
- unsupported command execution is disclosed explicitly;
- side effects section says none/skipped/unsupported;
- workflow recommendations are review-only and do not create files;
- no runtime state is written by first-run mode unless an explicit run is separately scoped;
- no report artifact is written by default;
- no arbitrary command execution occurs;
- no provider calls occur;
- no raw source, parser, command, or provider payload is copied;
- secret-like supplied notes/risks/limitations are rejected without leakage;
- errors use stable codes and avoid raw values;
- existing `init-repo-governance` tests still pass;
- existing WorkReport, executor, validation, EvidenceReference, SideEffect, hook, typed handoff, local check, and adapter tests still pass;
- `npm run check:docs` passes.

## 16. Documentation Updates Required

The implementation updates:

- `README.md`;
- `ROADMAP.md`;
- `docs/cli/README.md`;
- `docs/cli/overview.md`;
- `docs/cli/first-run.md`;
- `docs/cli/init-repo-governance.md`;
- `docs/user-guide/agent-harness-quickstart.md`;
- `docs/implementation-plans/existing-repo-governance-onboarding-plan.md`;
- `docs/concepts/governed-work-pattern.md`.

Docs must say:

- first-run governed ledger/report mode is implemented as a report-ready context command;
- first-run mode is explicit and local;
- it does not execute arbitrary commands;
- it does not run providers or writes;
- it does not generate active workflows;
- recommendations are review-only;
- report artifacts are not written by default;
- hosted/distributed behavior, recursive agents, agent swarms, and Level 3/4 autonomy remain unsupported.

## 17. Proposed Implementation Sequence

1. Add a first-run report input/context helper that accepts validated project context and safe observations.
2. Construct a `WorkReport` or report-ready context through existing report constructors.
3. Add CLI command `workflow-os first-run` as a bounded onboarding output path if the helper is accepted.
4. Wire the command to validate the project and call the helper.
5. Add focused tests for scaffold-to-first-run flow, no state writes, no artifacts, no command execution, and redaction.
6. Update docs and create an implementation report.
7. Run maintainer review before sidecar, patch artifact, capability-aware classification, or automatic workflow recommendation implementation.

## 18. Open Questions

- Should first-run mode create a full `WorkReport` immediately, or a report-ready context first?
- Should the command require the generated `local/first-run-governance` workflow, or support any valid local project?
- Should the command run `workflow-os validate` internally or require users to run validation first?
- Should CLI output be human text only, or offer preview JSON?
- Should report IDs be generated by the command or supplied by advanced callers?
- Should first-run output include repo-relative file names or avoid paths entirely by default?
- When should optional report artifact writing be introduced?
- How much workflow recommendation detail is useful before workflow catalog governance exists?

## 19. Final Recommendation

First-run governed ledger/report mode implementation is complete for the bounded report-ready context slice.

The implementation exposes `workflow-os first-run` as the explicit onboarding command. It produces a validated report-ready context from the generated governance envelope, discloses missing evidence and skipped checks, and recommends review-only workflow candidates.

It must still not add arbitrary command execution, live handlers, provider calls, writes, report artifacts by default, schema changes, automatic workflow generation, workflow registration, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.
