# Self-Governed Build Benchmark

The Self-Governed Build Benchmark is the maintained Workflow OS dogfood loop for building Workflow OS with Workflow OS.

This guide is for Workflow OS maintainers using the repo-local `dg/*` dogfood workflows. It is not the default setup path for downstream repositories. For ordinary project onboarding, use `workflow-os init-repo-governance` and `workflow-os first-run`; those commands create a local governance envelope and review-only recommendations without requiring users to copy `dg/*` workflows.

The operating model is:

```text
Agent executes. Workflow OS governs.
```

That means Codex, Claude Code, or a human performs repository work while the local Workflow OS kernel governs scope, validation, run identity, approval checkpoints, event history, validation/check disclosure, and report posture where those capabilities exist.

This guide is an operating runbook. It does not add runtime behavior, automatic local check execution, CLI report rendering, schema fields, report artifact automation, arbitrary shell execution, recursive agents, agent swarms, write-capable adapters, hosted execution, production self-hosting, or Level 3/4 autonomy.

## When To Use It

Use this benchmark loop for material Workflow OS roadmap work:

- planning phases;
- implementation phases;
- maintainer reviews;
- blocker fixes and blocker-fix reviews;
- docs cleanup phases;
- validation/check handler phases;
- report/artifact/citation phases;
- PR hygiene and conflict-avoidance handoffs.
- runtime-composition phases;
- focused blocker fixes;
- release hygiene phases.
- merged-branch cleanup readiness phases.
- recommendation-only workflow discovery phases.

Do not use it to bypass explicit scope, failed validation, denied policy, missing approvals, failed checks, or maintainer review.

## Current Honest Boundary

Today the benchmark is **kernel-governed and agent/human-executed**.

Implemented:

- repo-local `npm run dogfood:benchmark` development helper;
- repo-local governed phase runner commands, `phase-start` and `phase-close`, for material Workflow OS roadmap work;
- local project validation;
- sequential multi-step dogfood workflow;
- approval pause/resume;
- durable local event history;
- explicit report-bearing APIs in core;
- explicit local report artifact store;
- selected hook, local check, typed handoff, evidence, and WorkReport foundations;
- explicit `DocsCheckLocalHandler` through non-default registration in tested code paths.

Not implemented:

- automatic local check execution;
- default `DocsCheckLocalHandler` registration;
- CLI report rendering;
- CLI report artifact writing;
- workflow-declared hooks;
- workflow schema fields for dogfood benchmark behavior;
- arbitrary shell execution;
- repository writes from inside the kernel;
- recursive agents or agent swarms;
- production self-hosting.

When no explicit handler exists for a validation/check step, run the command outside the kernel and disclose that honestly.

Repo-local helper planning and implementation are documented in [Self-Governed Build Benchmark CLI/Dev-Helper Plan](../implementation-plans/self-governed-build-benchmark-cli-dev-helper-plan.md). The helper wraps existing CLI commands; it is not a stable public product CLI command and does not approve checkpoints automatically.

## Repo-Local Helper

For the normal dogfood loop, use:

```sh
npm run dogfood:benchmark -- commands
npm run dogfood:benchmark -- validate
npm run dogfood:benchmark -- start
npm run dogfood:benchmark -- approve <run-id> <approval-id> --reason reviewed-governance-task
npm run dogfood:benchmark -- inspect <run-id>
```

For material Workflow OS roadmap phases, prefer the governed phase runner:

```sh
npm run dogfood:benchmark -- phase-start --phase implementation \
  --work-summary "Implement the approved bounded phase change." \
  --approved-scope "Update the relevant runner, model, docs, and focused tests only." \
  --strict-non-goals "No hidden approvals, automatic approvals, repo automation, shell execution by the kernel, persistence, artifacts, schemas, side effects, writes, hosted behavior, or release posture changes." \
  --expected-touched-surfaces "Relevant implementation files, focused tests, roadmap/docs, and phase report." \
  --validation-required "Run the focused tests, docs check, and git diff check required by the phase." \
  --why-now "This phase follows the accepted roadmap, review, bug, or implementation plan."
npm run dogfood:benchmark -- phase-close <run-id> --phase implementation
```

`phase-start` validates the dogfood project, starts the mapped `dg/*` workflow, prints the real `run_id`, `approval_id`, status, and next action, prints an approval command for a human or maintainer to run explicitly, emits a structured `approval_handoff` block that agents must relay before asking for approval, and emits a `copy_safe_approval_request` block for the final user-facing approval request. For live material phase starts, bounded work-context fields are required before the runner will present approval as ready. It does not approve automatically.

`phase-close` reads status and inspect output for the run, summarizes event counts, approval/retry/escalation counts, terminal posture, and the phase-report fields that must be disclosed. It does not generate or persist a WorkReport artifact.

Fixed P0 bug: [Governed Phase Approval Handoff Context Bug](../concepts/GOVERNED_PHASE_APPROVAL_HANDOFF_CONTEXT_BUG.md) tracks the approval-context loss found during dogfooding. The runner now emits a required structured `approval_handoff` block.

Fixed P0 follow-up: [Governed Phase Approval Handoff Preservation Bug](../concepts/GOVERNED_PHASE_APPROVAL_HANDOFF_PRESERVATION_BUG.md) tracks the agent-relay gap found after the runner-side fix. Agents must preserve and present the complete emitted block before asking a maintainer to approve. The approval request, including any final response that waits for approval, must not collapse the block into vague prose such as "waiting for approval."

Fixed P0 follow-up: [Governed Phase Approval Work Summary Bug](../concepts/GOVERNED_PHASE_APPROVAL_WORK_SUMMARY_BUG.md) tracks the remaining gap where a preserved handoff could still fail to explain the concrete work being approved. The runner now supports required bounded work-context fields for material live phase starts. Agents must not ask for approval from a handoff that does not explain the proposed work, expected scope, strict non-goals, likely touched surfaces, validation expectations, and why the phase is next.

Fixed P0 follow-up: [Governed Phase Approval Final Request Preservation Bug](../concepts/GOVERNED_PHASE_APPROVAL_FINAL_REQUEST_PRESERVATION_BUG.md) tracks the repeated failure where an agent relayed the full handoff in commentary but ended with a vague final approval request. The runner now emits `copy_safe_approval_request_required: true` and a copy-safe approval request block. If the turn ends waiting for approval, the final response must preserve that copy-safe request; earlier commentary is not sufficient.

P0 hardening status: [Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md) tracks the proof boundary. The helper emits the approval content, repo instructions require agents to present it, and the dogfood runner now persists a typed, redaction-safe approval-presentation record during material `phase-start` runs before approval. Dogfood runner proof persistence is documented in [Dogfood Runner Approval-Presentation Persistence Plan](../implementation-plans/dogfood-runner-approval-presentation-persistence-plan.md). Future enforcement should make dogfood approvals call the opt-in presentation-enforcement path by default and fail closed when proof is missing or stale.


Useful options:

```sh
npm run dogfood:benchmark -- start --state-dir /tmp/workflow-os-self-governance-state
npm run dogfood:benchmark -- start --run-id run/dogfood-phase
npm run dogfood:benchmark -- validate --dry-run --no-build
npm run dogfood:benchmark -- prompt
```

The helper keeps approval explicit. It may extract IDs from the local kernel output so humans can see the required checkpoint, but it does not approve immediately after start, register real check handlers, run arbitrary commands, write report artifacts, or render WorkReports.

Phase runner mappings:

| Phase | Workflow |
| --- | --- |
| `planning` | `dg/d` |
| `docs` | `dg/d` |
| `implementation` | `dg/implement` |
| `review` | `dg/review` |
| `blocker` | `dg/blocker` |
| `pr` | `dg/pr` |
| `release` | `dg/release` |
| `runtime-composition` | `dg/runtime-composition` |
| `branch-cleanup` | `dg/branch-cleanup` |
| `workflow-discovery` | `dg/workflow-discovery` |
| `spec-field-operationalization` | `dg/spec-field-operationalization` |

All material Workflow OS implementation, review, blocker, PR hygiene, release, and workflow-discovery phases should begin with `phase-start` unless explicitly exempted. The exemption must be disclosed in the phase report.

## PR Hygiene Loop

Use the `dg/pr` dogfood workflow when a phase is ready to become a branch or PR, or when a branch has drifted from `main`. It governs the conflict-prevention loop without running git for you.

The governed checkpoints require disclosure that:

- `main` was fetched and integrated before PR work;
- hot files were scoped for conflict risk;
- approval was granted before PR staging;
- repository edits and git operations were performed outside the kernel;
- validation commands and skipped checks were disclosed;
- merge or rebase results and conflict resolutions were disclosed;
- branch, commit, PR URL, mergeability, and validation status were reported.

Start it with the generic CLI:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-pr-hygiene-state \
  --mock-all-local-skills \
  run dg/pr
```

This workflow does not run `git`, call GitHub, open PRs, resolve conflicts, persist reports, or change repository state. It makes the handoff auditable; Codex or a human still executes the repository operations.

## Dogfood Workflow Suite

Use a narrower workflow when the work shape is known:

| Workflow | Use for | What it governs |
| --- | --- | --- |
| `dg/d` | Planning/docs benchmark work and older self-governed phases | Scope, approval, implementation handoff, validation disclosure, explicit docs check, report posture |
| `dg/implement` | Accepted implementation phases | Scope confirmation, required context, implementation approval, edit handoff, validation disclosure, implementation report |
| `dg/review` | Maintainer reviews and blocker-fix reviews | Review context, review approval, scope verification, validation assessment, findings classification, review verdict |
| `dg/pr` | PR preparation and conflict avoidance | Main sync, hot-file risk, validation disclosure, conflict-resolution disclosure, PR readiness |
| `dg/runtime-composition` | Runtime-composition phases | Primitive inventory, explicit opt-in integration path, approval, validation disclosure, composition report |
| `dg/blocker` | Focused blocker fixes | Original blocker restatement, minimal fix boundary, approval, regression validation, fix report |
| `dg/release` | Release hygiene and public-preview readiness | Release scope, public docs checks, approval, validation disclosure, publication handoff, readiness report |
| `dg/branch-cleanup` | Merged branch cleanup readiness | Repo-state readiness, main-sync disclosure, merged branch inventory, delete-candidate review, cleanup approval, post-cleanup validation, cleanup report |
| `dg/workflow-discovery` | Recommendation-only workflow discovery | Signal-source inventory, repeated-pattern analysis, overlap/conflict review, recommendation drafting, stewardship approval, discovery report |

The suite is meant to reduce the gap between “Workflow OS governs its own build” and the actual day-to-day build loop. The workflows are not automation owners. They do not edit files, run arbitrary commands, call GitHub, push branches, create PRs, delete branches, generate workflow files, register workflows, persist reports, or bypass human approval.

The `dg/*` suite is specific to Workflow OS's own repository work. It is not a default workflow pack for downstream users. Treat these workflows as reference patterns for kernel-governed work. Portable examples live under `examples/`; user and team workflows should live in their own Workflow OS projects and, later, a governed workflow catalog/store.

## Branch Cleanup Loop

Use `dg/branch-cleanup` before deleting merged local or remote branches. It governs branch cleanup as a repository hygiene workflow rather than leaving deletion decisions to an ad hoc agent step.

The governed checkpoints require disclosure that:

- the current branch and working tree were checked;
- `main` was fetched and local main sync was disclosed;
- local and remote branches merged to `main` were inventoried;
- deletion candidates and non-candidates were presented before deletion;
- approval was granted before any local or remote branch deletion;
- Codex or a human performed approved cleanup outside the kernel;
- post-cleanup branch state and skipped deletions were disclosed;
- cleanup actions, skips, validation, and next steps were reported.

Start it with:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-branch-cleanup-state \
  --mock-all-local-skills \
  run dg/branch-cleanup
```

This workflow does not run `git`, delete local branches, delete remote branches, prune remotes, inspect GitHub, force-push, persist reports, or change repository state. It makes cleanup reviewable; Codex or a human still executes approved repository operations.

## Workflow Discovery Loop

Use `dg/workflow-discovery` when repeated work friction suggests that Workflow OS should recommend a new governed workflow or a change to an existing workflow.

Good signals include:

- repeated manual handoffs;
- repeated branch, PR, validation, or report hygiene issues;
- recurring review findings;
- missing approval or evidence gates;
- repeated natural-language handoffs that should become typed workflow boundaries;
- overlapping workflows that appear to govern the same authority, resource, report, handoff, or side-effect boundary.

The governed checkpoints require disclosure that:

- the discovery window and source set were scoped;
- recent reports, roadmap updates, dogfood runs, and manual workarounds were inventoried;
- repeated work patterns and missing gates were identified;
- overlap, conflict, duplicate authority, and lifecycle risks were reviewed;
- recommendations were drafted with rationale and explicit non-goals;
- stewardship approval was granted before recommendations became roadmap or implementation handoff;
- accepted, rejected, and deferred recommendations were disclosed;
- discovery signals, recommendations, conflicts, deferrals, and next steps were reported.

Start it with:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-workflow-discovery-state \
  --mock-all-local-skills \
  run dg/workflow-discovery
```

This workflow does not generate workflow files, register workflows, mutate specs, modify roadmap state, execute commands, inspect GitHub, approve itself, or resolve workflow conflicts automatically. It makes recommendations reviewable; Codex or a human still performs accepted follow-up implementation.

## Benchmark Loop

Use this loop for each governed phase:

1. Read [Engineering Standard](../ENGINEERING_STANDARD.md), [Roadmap](../../ROADMAP.md), the relevant plan, report, and review docs.
2. Validate the dogfood project.
3. Start or resume the governed dogfood workflow, preferably with `npm run dogfood:benchmark -- phase-start --phase <phase>`.
4. Treat approval checkpoints as mandatory.
5. Execute only the approved scope.
6. Run implemented explicit local check handlers only when explicitly registered and reviewed.
7. Run required validation commands outside the kernel when no handler exists.
8. Preserve check outcomes as bounded summaries or stable references where implemented.
9. Produce the structured implementation or review report required by the phase.
10. Inspect and disclose run status, approval/checkpoint context, commands run, failures, limitations, and next phase, preferably with `npm run dogfood:benchmark -- phase-close <run-id> --phase <phase>`.
11. Do not advance the roadmap based on model self-review alone.

Every material phase report should include:

- dogfood workflow ID;
- run ID;
- approval ID where one was requested;
- approval outcome;
- event summary;
- validation summary;
- commands and checks run outside the kernel;
- skipped checks with reason;
- report posture;
- work performed outside the kernel.

## Run The Dogfood Workflow

From the repository root, build the CLI:

```sh
cargo build -p workflow-cli --bin workflow-os
```

Validate the dogfood project:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  validate
```

Start a governed dogfood run:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  run dg/d
```

The run should execute the scope checkpoint, pause at the planning approval checkpoint, and print a `run_id` plus `approval_id`.

Approve only after the phase scope is understood:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/dogfood-reviewer \
  --reason reviewed-governance-task
```

Inspect the governed trail:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  inspect <run-id>
```

The `--mock-all-local-skills` path is a deterministic preview mechanism. It is not proof that the kernel executed real repository checks or edits.

## Agent Prompt

Paste this into Codex, Claude Code, or another coding agent after starting or identifying the governed phase:

```text
Use Workflow OS as the governing layer for this Workflow OS phase.

Before editing:
1. Read docs/ENGINEERING_STANDARD.md, ROADMAP.md, and the relevant plan/report/review docs.
2. Validate the dogfood project or relevant Workflow OS project.
3. Start or resume the governed dogfood workflow when this phase requires dogfooding.
4. Treat approval checkpoints as mandatory.

While working:
1. Stay inside the approved phase scope.
2. Do not invent run IDs, approvals, evidence references, audit events, local check results, WorkReports, validation results, or command output.
3. Run implemented explicit local check handlers only when they are explicitly registered and reviewed.
4. Run required validation commands outside the kernel when no handler exists, and disclose that boundary.
5. Do not claim automatic local checks, write-capable adapters, recursive agents, hosted execution, production self-hosting, or Level 3/4 autonomy.

Before finishing:
1. Report completed scope, explicitly deferred scope, validation results, commands run, and next recommended phase.
2. Include the governed run status and approval/checkpoint context when a dogfood run was used.
3. Distinguish kernel-governed behavior from agent/human-executed work.
```

## Phase Checklist

Before work:

- Relevant plan/review docs read.
- Dogfood project or relevant project validated.
- Governed run started or resumed when required.
- Approval checkpoint respected.
- Scope written in phase terms.

During work:

- No scope expansion without approval.
- No fabricated kernel state.
- No raw command output copied into reports.
- No unsupported runtime capability claimed.
- Manual validation clearly distinguished from kernel-executed checks.

Before handoff:

- Validation commands run or explicitly not run with reason.
- Structured report created or updated.
- Run status and approval/checkpoint context disclosed.
- Deferred work and limitations stated.
- Next phase recommended.

## Benchmark Matrix

| Kernel primitive | Current benchmark use | Boundary |
| --- | --- | --- |
| Project validation | Required before governed work | CLI/core validation only |
| Run identity | Required for governed phases | Local run state |
| Event history | Inspectable after run | Not a report replacement |
| Multi-step execution | Dogfood workflow uses sequential steps | No branching/nesting |
| Approvals | Mandatory checkpoint | No multi-party approval yet |
| Local checks | DocsCheck explicit handler exists | Not default or CLI-enabled |
| WorkReports | Core/report helper APIs exist | No CLI rendering |
| Report artifacts | Explicit store exists | No automatic writes |
| Hooks | Selected explicit paths exist | No workflow-declared hooks |
| Typed handoffs | Model exists | Dogfood integration deferred |
| EvidenceReference | Selected attachment paths exist | No broad automatic evidence |
| Side-effect boundary | Planning/model foundations exist | No writes |
| Reasoning lineage | Future architecture | Not benchmark proof |

## Failure Handling

Stop and create a blocker-fix or planning phase when:

- validation fails;
- approval is missing or denied;
- a required explicit handler is unavailable;
- required validation/check commands fail;
- report generation fails where explicitly requested;
- report artifact writing fails where explicitly requested;
- a reference is missing but claimed;
- scope expands beyond approval;
- docs claim unsupported runtime behavior;
- the work requires writes, side effects, live adapters, hosted behavior, or higher autonomy before those boundaries are accepted.

Do not quietly continue past a failed governance checkpoint.

## Metrics To Track

Useful benchmark metrics:

- governed phases with run IDs;
- approval pass/deny behavior;
- validation/check commands run;
- explicit handler coverage vs manual checks;
- report-bearing result usage;
- report artifact usage when explicitly requested;
- blocker fixes found by dogfooding;
- roadmap phases advanced through accepted review;
- unsupported claims caught before merge;
- scope expansions prevented or redirected.

These metrics should teach whether the kernel improves development governance. They are not production-readiness claims.

## Related Docs

- [Self-Governed Build Benchmark Plan](../implementation-plans/self-governed-build-benchmark-plan.md)
- [Self-Governed Build Benchmark Plan Review](../concepts/SELF_GOVERNED_BUILD_BENCHMARK_PLAN_REVIEW.md)
- [Workflow OS Self-Governance Dogfood Project](../../dogfood/workflow-os-self-governance/README.md)
- [Agent Harness Quickstart](agent-harness-quickstart.md)
- [Root Agent Instructions](../../AGENTS.md)
