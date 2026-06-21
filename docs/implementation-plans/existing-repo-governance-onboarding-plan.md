# Existing Repo Governance Onboarding Plan

Status: In progress. The first in-repo governance scaffold command, `workflow-os init-repo-governance`, is implemented and accepted in [Existing Repo Governance Scaffold Review](../concepts/EXISTING_REPO_GOVERNANCE_SCAFFOLD_REVIEW.md). The follow-on first-run governed ledger/report mode is planned in [First-Run Governed Ledger/Report Plan](first-run-governed-ledger-report-plan.md). Sidecar external-repo mode, capability-aware blocked-vs-failed classification, patch artifact modeling, automatic workflow recommendations, and write-capable adapters remain future work.

## 1. Executive Summary

Recent external testing showed a product gap: Workflow OS can govern projects that are already Workflow OS projects, but a serious user pointing the CLI at an existing repository gets the technically correct but incomplete answer: `workflow-os.yml` is missing.

That means the kernel is further along than the adoption path. We have strong internal dogfood workflows for building Workflow OS, but we have not yet made the path obvious for a user who wants:

```text
Use Workflow OS to govern agent work in this existing repo.
```

This is now a P0 adoption fix. The next phases should create a safe setup path for existing repositories while preserving the current runtime boundary: the agent or human executes repository work; Workflow OS governs scope, approvals, evidence, side-effect disclosure, validation/check posture, and final reporting.

The plan is not only "create `workflow-os.yml`." The first-run experience should apply the Governed Work Pattern immediately, even before custom workflows exist. Workflow OS should connect, map, document, collect or cite evidence, disclose gaps, produce a WorkReport, and recommend the first workflows/checkpoints to formalize.

This plan does not implement arbitrary command execution, write-capable adapters, hosted behavior, recursive agents, agent swarms, runtime hook broadening, schemas, examples that overclaim automation, or Level 3/4 autonomy.

## 2. What We Learned

The scratch Express experiment was useful product-discovery evidence, but it did not test the real Workflow OS kernel. The corrected follow-up did test the real kernel and found:

- Real Workflow OS can validate and run existing Workflow OS projects.
- Approval pause/resume, local durable state, ordered event history, fixture-backed read-only adapters, mock local skills, and dogfood workflows work within the current preview contract.
- Pointing Workflow OS directly at a normal OSS repository fails because the repo is not a Workflow OS project.
- Dogfood workflows are useful reference patterns, but they are not user starter assets.
- A user needs a bridge from "my existing repo" to "a governed Workflow OS project for this repo."

The most important product correction is:

```text
Dogfood workflows prove Workflow OS can govern its own build.
They do not solve onboarding for a user's existing repo.
```

The second correction is that Workflow OS should be useful before a user has authored mature workflows:

```text
Before workflows are mature, Workflow OS observes, structures, evidences, and reports.
As patterns repeat, Workflow OS recommends and hardens workflows.
Once workflows exist, Workflow OS governs execution against them.
```

## 3. Goals

- Make the existing-repo setup path obvious and hard to misread.
- Separate dogfood workflows from portable examples and starter templates.
- Provide a safe scaffold for governing work in an existing repository.
- Apply the Governed Work Pattern as a default first-run posture before custom workflows exist.
- Produce an immediate evidence-aware WorkReport or report-ready context where the current implementation boundary allows it.
- Recommend candidate workflows, checkpoints, evidence requirements, and approval gates based on first-run observations.
- Support both in-repo and sidecar governance shapes where appropriate.
- Keep agent execution fast while Workflow OS governs steps, approvals, evidence, side effects, checks, and reports.
- Make missing-manifest diagnostics actionable for new users.
- Define how agent-executed evidence, command outputs, and patches should be referenced without claiming the kernel executed them.
- Prepare for future capability-aware execution without enabling arbitrary shell execution now.

## 4. Non-Goals

Do not implement in this lane:

- automatic arbitrary shell command execution;
- write-capable adapters;
- GitHub branch creation, commits, PR creation, comments, labels, merges, or closes;
- Jira issue creation, updates, comments, status transitions, labels, or links;
- CI reruns, workflow dispatch, cancellation, or artifact mutation;
- hosted or distributed runtime;
- production backend;
- recursive agents or agent swarms;
- workflow auto-generation from observed work;
- automatic workflow registration or promotion;
- automatic report artifact writing from every run;
- CLI report rendering beyond separately scoped behavior;
- workflow schema changes unless separately planned;
- Level 3/4 autonomy.

## 5. Default Governed Work Pattern Posture

Workflow OS already carries strong opinions through the Governed Work Pattern. The onboarding path should make those opinions first-class instead of leaving them scattered across docs.

Out of the box, Workflow OS should expect serious agent-assisted work to produce or disclose:

- bounded goal and scope;
- repository/task context;
- evidence considered or missing;
- validation/check expectations and skipped-check disclosure;
- policy and approval posture where relevant;
- side-effect declarations, including none/skipped/unsupported;
- risks and known limitations;
- incomplete or deferred work;
- final WorkReport closure;
- candidate workflow/checkpoint recommendations when repeated patterns appear.

This default posture is not a substitute for authored workflows. It is the first-run expression of the Governed Work Pattern.

The kernel should not fabricate evidence, silently execute arbitrary commands, or auto-promote workflow recommendations. It should provide an opinionated local ledger/report path that says what happened, what was proven, what was skipped, and what should become governed workflow structure next.

## 6. Required Conceptual Separation

The repository should maintain three distinct categories:

```text
dogfood/   = how Workflow OS governs building Workflow OS
examples/  = portable learning examples for evaluators
scaffolds/ = starter setup paths for a user's own repo
```

Dogfood workflows should be described as:

- real internal dogfood evidence;
- reference governance patterns;
- implementation benchmarks;
- not community defaults;
- not plug-and-play workflow packs.

Portable onboarding should not ask users to copy `dg/*` workflows into their own projects.

## 7. Target User Scenarios

### Existing App Repo

A user is inside an existing application repository and wants Codex or Claude Code to work under Workflow OS governance.

Expected path:

1. Build or install the local CLI.
2. Run a scaffold command or follow a documented manual setup.
3. Generate a minimal Workflow OS project contract for the current repo.
4. Generate agent instructions for using Workflow OS as the governing layer.
5. Validate the generated project.
6. Start a first-run governed observation/report path.
7. Let the agent execute repository work under the approved boundaries.
8. Inspect the run and produce a first WorkReport or report-ready context.
9. Review recommended workflow/checkpoint/evidence improvements.

### External OSS Experiment

A user wants to test an agent-driven change against an external OSS repo without committing upstream or mutating the Workflow OS repo.

Expected path:

1. Create a sidecar Workflow OS project.
2. Reference the external target repo path or pinned snapshot.
3. Declare forbidden write roots.
4. Declare expected patch artifact posture.
5. Declare evidence requirements for tests, lint, and command summaries.
6. Keep command execution agent/human-executed unless reviewed handlers are explicitly supplied.
7. Produce a final report with patch and evidence references.
8. Recommend whether this repeated experiment shape should become a reusable sidecar workflow.

### Team Repo Governance

A team wants a repeatable governance envelope for product work, engineering work, release hygiene, or review tasks.

Expected path:

1. Start from a starter scaffold, not Workflow OS's `dg/*` dogfood workflows.
2. Review generated policies, approval gates, evidence requirements, and report requirements.
3. Customize ownership, authority, side-effect posture, checks, and handoffs.
4. Store authored contracts in git.
5. Keep runtime state in Workflow OS state storage.
6. Use reports and repeated-run observations to recommend workflow changes instead of relying on manual YAML authoring forever.

## 8. First Implementation Scope

Status: Implemented as `workflow-os init-repo-governance`.

The smallest useful implementation should add a first-party existing-repo scaffold path.

Candidate CLI shape:

```sh
workflow-os init-repo-governance
```

or:

```sh
workflow-os init-existing-repo
```

The exact command name should be chosen during implementation based on existing CLI naming conventions.

The scaffold should create or update:

- `workflow-os.yml`;
- minimal `workflows/` definition for a governed agent task;
- minimal policy/skill declarations required by the project shape;
- `.workflow-os/agent-harness-prompt.md`;
- `AGENTS.md` section or merge-safe instructions where appropriate;
- a README or setup note explaining current boundaries.

It should be explicit that generated local skills are governed placeholders unless a real handler is implemented, registered, and reviewed.

The first implementation should tee up the first-run governed work path. If full immediate report generation is not yet feasible in the CLI surface, the scaffold should at least generate the minimal workflow/report posture needed for a follow-up command to produce:

- mapped repository/task context;
- explicit unsupported capability disclosures;
- validation/check recommendations;
- side-effect posture;
- first WorkReport closure;
- candidate workflow/checkpoint recommendations.

## 9. First-Run Ledger Mode

Status: Planned in [First-Run Governed Ledger/Report Plan](first-run-governed-ledger-report-plan.md), not implemented.

The P0 product experience should include a local, explicit first-run mode for repositories that are not mature Workflow OS projects yet.

Candidate CLI shape:

```sh
workflow-os observe
```

or:

```sh
workflow-os first-run
```

The exact command name should be chosen during implementation. It may also be a follow-up to the scaffold command rather than a separate command if that better matches CLI conventions.

First-run ledger mode should:

- require explicit user invocation;
- operate locally;
- use the generated governance envelope;
- map basic repository/project context without requiring network access;
- record bounded observations and disclosures;
- produce a WorkReport or report-ready context through existing validated constructors;
- recommend workflow candidates without auto-registering them;
- preserve the boundary that agents/humans execute unsupported commands.

First-run ledger mode should not:

- execute arbitrary shell commands by default;
- mutate external systems;
- create commits, branches, PRs, issues, comments, or CI reruns;
- fabricate evidence references;
- auto-promote workflow recommendations;
- claim production audit/compliance coverage.

## 10. Sidecar Versus In-Repo Mode

The first phase should choose the smaller implementation path, but the plan should preserve both concepts:

- **In-repo mode**: add Workflow OS governance files to the repository being governed.
- **Sidecar mode**: create a separate Workflow OS project that governs an external target repo by path/reference.

Recommended first implementation: in-repo mode for simplicity, with sidecar mode planned immediately after for external OSS experiments.

Why:

- in-repo mode matches the common user setup;
- it makes `workflow-os validate` work immediately;
- it avoids needing target-repo path/reference schema changes up front;
- it keeps generated files reviewable in git.

Sidecar mode remains important for sandboxed OSS experiments and should be next after the in-repo scaffold is reviewed.

## 11. Missing Manifest Recovery

When `workflow-os validate` or `workflow-os doctor` sees `loader.manifest_missing`, the CLI should eventually provide an actionable next step.

Future diagnostic guidance should say, in bounded form:

```text
No workflow-os.yml was found.
This directory is not a Workflow OS project yet.
Run `workflow-os init-repo-governance` to scaffold local governance files, or pass --project-dir to an existing Workflow OS project.
```

This must not auto-create files during validation.

## 12. Agent-Executed Evidence Pattern

Current Workflow OS should remain honest: Codex or a human may execute commands outside the kernel.

The onboarding scaffold should explain how to report and reference:

- commands run;
- exit codes;
- test/lint summaries;
- patch files;
- validation outputs;
- capability denials;
- skipped checks;
- forbidden write-root assertions;
- final report notes.

The first implementation may use bounded report text and existing explicit citation/reference fields. It should not create fake evidence references, claim kernel execution, or copy raw logs by default.

## 13. First-Run Workflow Recommendations

The first-run WorkReport should carry opinions about what should become workflow structure next.

Recommended output should include bounded, review-only suggestions such as:

- candidate workflow names and purposes;
- recommended approval gates;
- recommended evidence requirements;
- recommended validation/check obligations;
- recommended side-effect declarations;
- recommended WorkReport sections;
- risks if the work remains ungoverned;
- conflicts with existing workflows, if detectable.

These recommendations must remain review-only. They must not generate active workflow files, register workflows, mutate roadmap state, or approve themselves unless a separately reviewed workflow-catalog implementation exists.

## 14. Patch Artifact Posture

Patch generation is a common governed-work artifact for existing repos and OSS experiments.

Future implementation should support a first-class "proposed patch" posture, but the first onboarding phase can document this as an agent-produced artifact:

- patch is generated by the agent or human;
- patch is scoped to approved files;
- dependency folders, lockfile churn, build artifacts, and VCS metadata are excluded unless approved;
- patch is cited or summarized in the final report;
- patch generation does not imply commits, branches, PRs, or provider writes.

## 15. Capability-Aware Follow-Up

The Express scenario exposed that capability denial is different from code failure.

This plan does not implement capability-aware execution, but it should tee up a follow-up model:

- local bind/socket capability;
- network egress capability;
- filesystem write capability;
- toolchain install capability;
- provider access capability;
- forbidden write roots;
- blocked-vs-failed classification.

That work should connect to existing side-effect, local-check, hook, and report primitives rather than creating a separate execution framework.

## 16. Documentation Updates Required

Update:

- `README.md`;
- `docs/user-guide/README.md`;
- `docs/user-guide/agent-harness-quickstart.md`;
- `dogfood/workflow-os-self-governance/README.md`;
- `ROADMAP.md`;
- relevant CLI docs after the scaffold command exists.

Docs must state:

- dogfood workflows are internal reference patterns, not plug-and-play user assets;
- existing repositories need a Workflow OS project scaffold before `validate` or `run`;
- the first-run path should apply the Governed Work Pattern before custom workflows exist;
- Workflow OS should produce an evidence-aware report or report-ready context as soon as feasible;
- default workflow recommendations are review-only and must not auto-register active workflows;
- Workflow OS governs repository work while agents/humans execute unsupported commands;
- arbitrary command execution is not enabled by the scaffold;
- writes, hosted behavior, recursive agents, agent swarms, and Level 3/4 autonomy remain unsupported.

## 17. Test Plan For Future Implementation

Future implementation should test:

- scaffold creates `workflow-os.yml` in an empty temporary repo;
- scaffold is idempotent;
- scaffold refuses to overwrite user-authored files without explicit approval or flag;
- generated project validates;
- generated workflow can start with `--mock-all-local-skills`;
- generated run pauses for approval when expected;
- generated run inspect output clearly shows mock skill boundaries;
- first-run mode produces a WorkReport or report-ready context through validated constructors;
- first-run mode includes bounded default Governed Work Pattern sections;
- first-run mode recommends candidate workflows/checkpoints without auto-registering them;
- first-run report discloses missing evidence, skipped checks, and unsupported capabilities;
- generated agent prompt includes "Agent executes. Workflow OS governs.";
- generated docs state dogfood workflows are not plug-and-play assets;
- missing manifest diagnostics remain stable and non-leaking;
- no arbitrary command execution is enabled;
- no files are written outside the target repo;
- no network or provider writes are performed;
- existing examples, dogfood workflows, and CLI tests still pass.

## 18. Proposed Phase Sequence

1. Existing repo onboarding planning and docs alignment.
2. CLI scaffold implementation for in-repo governance setup.
3. First-run governed ledger/report mode planning.
4. First-run governed ledger/report mode implementation.
5. Maintainer review of scaffold and first-run report behavior.
6. Sidecar external OSS experiment planning.
7. Sidecar external OSS experiment scaffold implementation.
8. Agent-executed evidence attachment/reference planning.
9. Capability-aware blocked-vs-failed classification planning.
10. Patch artifact model/planning only after scaffold and first-run paths are reviewed.

## 19. Acceptance Criteria For This P0 Lane

- A user can start from an existing repo and get to a valid Workflow OS project without hand-authoring all YAML.
- The setup path makes agent usage the default mental model.
- The first-run path applies the Governed Work Pattern before custom workflows exist.
- The first-run path produces an evidence-aware WorkReport or report-ready context.
- The first-run report carries review-only opinions about recommended workflows, gates, checks, evidence, approvals, side effects, and risks.
- Dogfood workflows are clearly separated from user starter workflows.
- The scaffold does not overclaim automation.
- Missing manifest behavior becomes actionable.
- The generated project can be validated and run through the current local preview boundaries.
- Unsupported command execution, writes, hosted behavior, and high-autonomy behavior remain explicitly unsupported.

## 20. Recommended Next Phase

Proceed to **first-run governed ledger/report mode implementation**.

The implemented scaffold adds the smallest reviewed CLI path that helps a user govern their current repository with Workflow OS. The follow-on plan defines the next narrow code lane: an explicit local first-run mode that produces a validated WorkReport or report-ready context, discloses missing evidence and skipped checks, and recommends review-only workflow candidates without executing arbitrary commands, calling providers, writing artifacts by default, or registering workflows automatically.
