# Self-Governed Build Benchmark Plan

Status: Planning complete. This plan defines how Workflow OS should use its own local kernel as the governing body for building Workflow OS itself. It synthesizes parallel review of the dogfood project, roadmap, agent harness docs, runtime/executor capabilities, local check infrastructure, hook infrastructure, and report foundations. It does not implement new runtime behavior, CLI behavior, schemas, automatic local checks, report artifact automation, command execution, write-capable adapters, reasoning lineage, side-effect boundary enforcement, recursive agents, agent swarms, hosted execution, or release posture changes.

## 1. Executive Summary

Workflow OS is ready to dogfood its own kernel as the default development governance loop.

The correct framing is:

```text
Agent executes. Workflow OS governs.
```

That means Codex, Claude Code, or a human performs repository work while the local Workflow OS kernel governs scope, validation, run identity, approvals, event history, local check references, hook checkpoints, report posture, and final handoff artifacts where those capabilities exist.

This plan defines the **Self-Governed Build Benchmark**: a maintained benchmark protocol for proving Workflow OS by using Workflow OS to build Workflow OS. It should become the canonical example of governed agent work, but it must stay honest. The kernel does not autonomously edit code, run arbitrary shell commands, spawn recursive agents, fabricate evidence, or replace maintainer review.

## 2. Goals

- Make kernel dogfooding the default operating mode for Workflow OS roadmap phases.
- Turn the existing dogfood project into a benchmark protocol, not just a demo.
- Exercise as many implemented kernel primitives as safely possible while building the kernel.
- Preserve the kernel-governed, agent-executed boundary.
- Use durable local run state, approvals, event history, local checks, hooks, typed handoffs, report citations, and report artifacts as they become available.
- Create a repeatable loop for planning, implementation, review, blocker fixes, docs cleanup, validation/check work, and release hygiene.
- Define metrics that show whether the kernel is improving the development process.
- Keep missing capabilities explicit instead of papering them over with agent memory or prose.
- Avoid overclaiming autonomy, production self-hosting, write support, hosted execution, or recursive agent orchestration.

## 3. Non-Goals

Do not implement in this phase:

- new runtime behavior;
- automatic kernel control of Codex or other agents;
- automatic runtime report generation for every run;
- runtime result exposure changes;
- CLI report rendering;
- CLI report artifact writing;
- automatic local check execution;
- default local check handler registration;
- arbitrary shell execution;
- real cargo/npm check broadening beyond already implemented explicit handlers;
- workflow schema changes;
- workflow-declared hooks;
- runtime hook configuration;
- hook warning/skipped continuation;
- command-output evidence attachment;
- evidence attachment broadening;
- approval evidence attachment;
- reasoning lineage or claim graph;
- side-effect boundary enforcement;
- write-capable adapters;
- repository writes from inside the kernel;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- production self-hosting claims;
- Level 3 or Level 4 autonomy claims;
- release posture changes.

## 4. Current Foundation

Implemented foundation:

- local project validation;
- sequential local multi-step execution;
- durable event-sourced local run state;
- approval pause/resume;
- policy decisions and audit/observability records;
- report-bearing executor APIs;
- work report contracts, reports, citations, and explicit artifact store;
- typed handoff model and report citation vocabulary;
- agent harness onboarding and scaffold;
- hook contract, runtime helper, selected executor checkpoints, hook event vocabulary, generic audit projection, `BeforeSkillInvocation` event append, and failed-closed behavior;
- local check command contracts, result model, references, side-effect boundary model, explicit `DocsCheckLocalHandler`, and explicit non-default registration profile;
- self-governance dogfood project at `dogfood/workflow-os-self-governance`.

Current honest boundary:

- the dogfood lane is kernel-governed and Codex/human-executed;
- README examples still mostly use `--mock-all-local-skills`;
- real `DocsCheck` execution exists through explicit handler registration and tests, not ambient CLI use;
- report-bearing execution exists in core APIs, but normal CLI `run` is not a report-rendering command;
- hooks are explicit and narrow, not workflow-declared ambient enforcement;
- WorkReports are governed handoff artifacts, not audit logs or reasoning lineage graphs.

## 5. Opinionated Recommendation

Workflow OS should now treat self-governance as a P0 product benchmark.

The benchmark should prove this loop:

```text
bounded roadmap phase
-> kernel validation
-> governed run identity
-> scope checkpoint
-> approval checkpoint
-> agent/human execution
-> explicit validation/check checkpoint
-> report-bearing result
-> review/blocker decision
-> next phase
```

The benchmark should not say "the kernel builds itself" without qualification. The stronger, safer claim is:

```text
Workflow OS governs its own development loop while agents and maintainers execute the work.
```

That is both more accurate and more compelling.

## 6. Benchmark Operating Protocol

Every material Workflow OS phase should follow this protocol unless the task is explicitly outside dogfood scope.

1. Read the engineering standard and current roadmap state.
2. Validate the dogfood project or the relevant Workflow OS project.
3. Start or resume a governed dogfood run for the phase.
4. Treat approval checkpoints as mandatory.
5. Execute only the approved scope.
6. Use explicit local check handlers only when they are implemented, registered, and reviewed.
7. Run required validation commands outside the kernel when no handler exists.
8. Preserve check outcomes as bounded summaries or stable references where implemented.
9. Produce the structured implementation/review report required by the phase.
10. Inspect and disclose the governed run state, approval/checkpoint context, commands run, failures, limitations, and next phase.
11. Do not advance the roadmap based on model self-review alone.

## 7. Eligible Phase Types

The benchmark should cover these phase types:

- planning phase;
- implementation phase;
- maintainer review phase;
- blocker fix phase;
- blocker fix review phase;
- docs cleanup phase;
- validation/check handler phase;
- report/artifact/citation phase;
- release hygiene phase.

Each phase type should have a known governance posture:

| Phase type | Kernel posture | Agent/human posture |
| --- | --- | --- |
| Planning | validate, run, approve scope, preserve report | write plan docs |
| Implementation | validate, approve, govern checkpoints, record checks | edit code/docs within scope |
| Review | govern review scope and evidence inspected | inspect implementation and write review |
| Blocker fix | govern narrow fix scope | make minimal fix |
| Docs cleanup | govern docs-only scope | update docs honestly |
| Validation/check | govern explicit check request | run supported handlers or manual commands |
| Release hygiene | govern readiness checklist | run approved release checks |

## 8. Benchmark Workflow Shape

The current dogfood workflow already has useful checkpoints:

- `scope-requested`
- `planning-approved`
- `implementation-handoff`
- `validation-disclosure`
- `docs-check`
- `review-and-report-posture`

Future dogfood work should either expand this workflow carefully or add narrowly scoped dogfood workflows for specific phase types.

Recommended next workflow shape:

- intake/scope checkpoint;
- planning approval checkpoint;
- typed implementation handoff checkpoint;
- before-work hook checkpoint when supported;
- validation/check checkpoint;
- local check result reference checkpoint where supported;
- review checkpoint;
- final report checkpoint;
- report artifact checkpoint where explicitly requested.

This should remain sequential until branching or nested harness execution is separately planned and reviewed.

## 9. Benchmark Matrix

The benchmark should maintain a matrix mapping dogfood behavior to kernel primitives.

| Kernel primitive | Current dogfood status | Target benchmark status |
| --- | --- | --- |
| Project validation | Implemented through CLI/core | Required before phase start |
| Run identity | Implemented | Required for every governed phase |
| Event history | Implemented | Inspected or cited in reports |
| Multi-step execution | Implemented | Used for all material phases |
| Approvals | Implemented | Mandatory at phase approval checkpoints |
| Policy gates | Implemented | Preserved before meaningful runtime actions |
| Local checks | DocsCheck explicit handler implemented | Expand only by scoped handler phases |
| Check references | Model implemented | Propagate into reports where supported |
| WorkReports | Implemented core/helper paths | Use explicit report-bearing APIs in benchmark tests |
| Report artifacts | Explicit store implemented | Opt-in only; no automatic writes |
| Hooks | Narrow explicit paths implemented | Use deterministic checkpoints only after scoped plans |
| Hook disclosures | Planned | Implement before warning/skipped continuation |
| Typed handoffs | Model implemented | Integrate planning -> implementation -> review |
| EvidenceReference | Implemented selected paths | Broaden only through accepted attachment phases |
| Side-effect boundary | Model/planning partial | Required before broader commands/writes |
| Reasoning lineage | Future architecture | Do not use as current benchmark proof |

## 10. Agent Responsibilities

The agent should:

- use Workflow OS as the governing layer;
- read required docs before edits;
- validate relevant specs;
- start or resume governed runs when the phase requires dogfooding;
- pause for approval rather than bypass it;
- stay within approved scope;
- run validation commands required by the phase;
- report commands, outcomes, deferred scope, and next phase;
- distinguish implemented behavior from planned behavior;
- never invent run IDs, approvals, evidence, audit events, check results, reports, or command output.

## 11. Maintainer Responsibilities

The maintainer should:

- decide which phases require dogfood governance;
- approve or deny governance checkpoints;
- reject scope expansion without a new phase or approval;
- review implementation and reports before roadmap advancement;
- keep dogfood docs honest;
- require blocker fixes when benchmark evidence exposes a real gap;
- prevent benchmark language from becoming production self-hosting or autonomous-agent overclaim.

## 12. Validation And Check Expectations

The benchmark should prefer implemented explicit handlers where safe:

- `DocsCheckLocalHandler` for docs checks through explicit non-default registration;
- future local handlers only after side-effect/cache/write posture is planned and reviewed.

When no handler exists, validation remains manual/outside-kernel and must be disclosed in the phase report.

Do not treat manually run commands as kernel-executed checks.

Do not store raw command transcripts as evidence or report text.

## 13. Report And Evidence Expectations

Every benchmarked phase should produce a structured report.

Where implemented, reports should cite:

- run identity;
- approval checkpoint;
- workflow or audit events;
- validation diagnostics;
- local check result references;
- hook invocation IDs;
- typed handoff IDs;
- explicit report artifact IDs.

Reports must not:

- copy raw command output;
- copy raw spec contents;
- copy provider payloads;
- fabricate missing evidence;
- claim report completeness when required references are unavailable;
- replace audit logs or future reasoning lineage.

## 14. Hooks And Instructions

`AGENTS.md` and the agent harness prompt are orientation. They are not enforcement.

The benchmark should continue using those files, but the next maturity layer should be deterministic hook checkpoints invoked by the harness/kernel where implemented.

Future hook-backed dogfood should:

- use explicit checkpoint kinds;
- preserve policy-before-side-effect ordering;
- fail closed on unsupported statuses;
- require bounded disclosures before warning/skipped continuation;
- avoid raw agent memory as the source of governance truth.

## 15. Metrics

The benchmark should track:

- phase count governed through dogfood;
- percentage of phases with a governed run ID;
- approval checkpoint pass/deny behavior;
- validation/check commands run;
- implemented check-handler coverage vs manual checks;
- report-bearing result usage;
- report artifact usage when explicitly requested;
- number of blocker fixes found through dogfood;
- number of roadmap phases advanced through accepted review;
- unsupported behavior claims caught before merge;
- scope expansions prevented or redirected.

These are product-learning metrics, not vanity metrics.

## 16. Failure Modes

The benchmark should fail closed or stop for maintainer decision when:

- dogfood project validation fails;
- a required approval is missing or denied;
- an explicit local check handler is unavailable;
- validation/check commands fail;
- report generation fails;
- report artifact writing fails where explicitly requested;
- evidence/check/report references are missing but claimed;
- an agent attempts to widen scope;
- docs claim unsupported runtime behavior;
- a phase would require writes, side effects, hosted behavior, or live adapters before those boundaries are accepted.

Failures should create a blocker-fix or planning phase, not a quiet bypass.

## 17. Privacy And Redaction

The benchmark must not store or copy:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded agent notes;
- unbounded model self-review.

Errors and reports must use stable codes, bounded summaries, and redaction-safe references.

## 18. Proposed Implementation Sequence

1. Add this self-governed build benchmark plan.
2. Review the plan.
3. Update the dogfood README into a benchmark runbook.
4. Add a maintainer-facing benchmark guide or checklist.
5. Add tests that exercise benchmark behavior through existing explicit APIs.
6. Integrate typed handoff references into dogfood report inputs.
7. Integrate local check result references into dogfood reports where already supported.
8. Plan an explicit dogfood report-bearing CLI/dev helper if needed.
9. Plan opt-in report artifact writing for benchmark runs if needed.
10. Continue broadening local check handlers only through side-effect-aware, explicit phases.

## 19. Parallel Review Inputs

This plan incorporates three parallel review lanes:

- dogfood/roadmap review: current dogfood is credible but lacks one operating protocol;
- runtime/kernel review: core APIs exist for reports, checks, hooks, state, approvals, and artifacts, but CLI/default paths remain intentionally conservative;
- user-facing docs review: onboarding states the right mental model, but the benchmark loop needs to become a first-class maintained guide.

All three reviews agreed that the next step is not more autonomy. It is tighter deterministic handoff between existing governance primitives.

## 20. Final Recommendation

Recommended next phase: **self-governed build benchmark plan review**.

After review, implement the benchmark as docs/tests around the existing dogfood project before adding new runtime power. The project should continue to avoid recursive agents, agent swarms, automatic command execution, repository writes from inside the kernel, write-capable adapters, production self-hosting claims, and Level 3/4 autonomy claims.
