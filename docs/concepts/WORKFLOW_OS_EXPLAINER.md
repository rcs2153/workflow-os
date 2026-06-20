# Workflow OS Explainer

Workflow OS starts from a simple idea:

```text
Agent executes. Workflow OS governs.
```

That sentence is the shortest version of the project. It says that the model, coding assistant, human operator, script, or tool may do the work, but Workflow OS should define the governing boundary around that work: what is allowed, what must be checked, what needs approval, what evidence was considered, what happened, what remains incomplete, and what should be handed off next.

Workflow OS is not trying to be another chat interface. It is not trying to be a generic agent swarm. It is not trying to make every model call autonomous. It is a local-first governed workflow kernel for AI-assisted work.

The bet is that as AI work becomes more common inside companies, the hard part will not be getting a model to take action. The hard part will be knowing which action was authorized, which context was used, which policy gate passed, which approval was required, which evidence supports the output, which side effects were proposed or skipped, and what the next person or workflow can trust.

That is the space Workflow OS is building toward.

## The Problem It Is Solving

Most AI workflows begin as prompts. Then they become prompt templates. Then they become scripts. Then they become a pile of scripts, tool calls, agent instructions, Slack messages, review notes, CI logs, ticket comments, and half-remembered operating norms.

That can work for a demo. It does not scale into governed work.

Once AI work touches real systems, the questions change:

- What was the workflow supposed to do?
- Which version of the workflow ran?
- Which step failed?
- Which approval was required?
- Who approved it?
- What evidence did the agent use?
- Did it copy a secret into a report?
- Did it write to a provider?
- Was that write allowed?
- What did it skip?
- What should the next operator know?

Those questions are not solved by making the model more persuasive. They are solved by turning the work into a governed system.

Workflow OS is the kernel for that governed system.

## The Core Mental Model

Workflow OS separates execution from governance.

Execution is the messy part. A person edits a file. Codex writes code. Claude drafts a plan. A deterministic local skill validates a project. A read-only adapter pulls repository context. A future write-capable adapter may propose or perform a side effect.

Governance is the boundary around that execution. It says:

- this workflow has an identity;
- this run has an identity;
- these steps are allowed;
- this policy gate must pass;
- this approval is mandatory;
- this event happened;
- this evidence can be cited;
- this report must disclose what happened and what did not happen.

The current preview implements the local kernel foundations for that boundary. It can load and validate Workflow OS projects, execute sequential local workflows, persist event-sourced run state, pause and resume approval-gated runs, emit audit and observability records, model evidence references, model work reports, and expose narrow local report APIs.

It also includes read-only adapter contracts and fixture-first GitHub, Jira, and CI/GitHub Actions preview paths. Those adapters are deliberately read-only in the public preview. Write-capable adapters remain future work.

## What A Workflow Is

A Workflow OS workflow is an authored unit of governed work.

It is not just a prompt. It is not just a script. It is a declared process with steps, policies, capabilities, approval requirements, retry and escalation behavior, validation rules, and run identity.

In the local kernel today, a workflow can run as a sequential multi-step process. Each meaningful transition is recorded as an event. If the workflow reaches an approval gate, execution pauses. A human can approve or deny. The run can be inspected later.

This matters because the workflow is no longer a vague instruction to an agent. It becomes something with shape.

The kernel can say:

- this step was scheduled;
- this policy decision was recorded;
- this approval was requested;
- this approval was granted or denied;
- this skill invocation was requested;
- this step completed or failed;
- this run reached a terminal state.

That gives AI-assisted work a durable spine.

## Why Local-First Matters

Workflow OS is local-first on purpose.

The current preview is not a hosted orchestration platform. It is not a production distributed runtime. It does not require a remote service to understand the kernel. It gives developers and maintainers a way to run governed workflows on a laptop, inspect the state, and build confidence in the model before introducing distributed workers, production databases, or hosted control planes.

Local-first also makes dogfooding possible. Workflow OS can be used to govern work on Workflow OS itself. The agent or human still edits the repository, but the kernel can validate the dogfood project, run the governed workflow, pause at approval checkpoints, preserve events, and produce report posture.

That is the loop:

```text
Workflow OS governs the build of Workflow OS.
```

The project is not claiming full self-hosting. It is proving, phase by phase, that the governance model can be applied to its own development.

## What The Kernel Does Today

The public preview includes several important capabilities.

It validates projects before execution. Workflow specs, skill specs, policy specs, and project manifests are checked deterministically. Validation diagnostics preserve source locations. Selected validation diagnostics can attach safe `EvidenceReference` values without copying raw spec contents.

It runs local workflows. The local executor supports sequential multi-step runs with deterministic state transitions. It can pause for approvals, resume after approval, fail closed on policy denial, retry bounded failures, and escalate when configured.

It persists local state. Runs are event-sourced. Events are append-only. Snapshots are projections. The event log remains the source of truth. Local state can be inspected and rehydrated.

It records governance signals. Audit records and observability records are emitted for meaningful runtime behavior. The goal is not to bury governance in prose, but to preserve structured facts.

It models evidence. `EvidenceReference` provides a citation substrate: a safe pointer to evidence, not a payload dump. Evidence can point to a spec file, adapter record, validation result, workflow event, audit event, or future evidence types. The design is intentionally conservative about sensitivity and redaction.

It models work reports. `WorkReportContract` and `WorkReport` define the future handoff artifact: what work was performed, what evidence was considered, what decisions were made, what approvals and policy gates mattered, what validation ran, what side effects were none/skipped/unsupported, what risks remain, and what the operator should know next. In-memory report helpers exist, and local report artifacts can be explicitly stored, but automatic report generation for every run is not implemented.

It models side effects before enabling writes. The SideEffect core model exists, WorkReport can cite SideEffect IDs, SideEffect workflow event vocabulary exists, proposed/denied/skipped events can be explicitly appended in a local executor path, and explicit SideEffect records can be stored locally. Runtime side-effect execution, provider mutation, write-capable adapters, and automatic discovery remain unimplemented.

That boundary is the point. The project is building the vocabulary and safety rails before opening mutation.

## What It Does Not Do Yet

Workflow OS is intentionally honest about what it does not do.

It does not provide production distributed execution. It does not provide a hosted service. It does not provide a UI. It does not provide write-capable GitHub, Jira, CI, or provider adapters. It does not automatically execute arbitrary local commands. It does not silently register local check handlers. It does not run recursive agents or agent swarms. It does not enable Level 3 or Level 4 autonomy by default.

It also does not replace deterministic governance with model self-review.

That last point matters. A model saying "this looks good" is not a policy gate. A model saying "I reviewed my own work" is not an approval. A natural-language summary is not an audit record. A copied log is not a safe evidence reference.

Workflow OS is built around the opposite posture: typed state, deterministic validation, explicit approval, bounded evidence, and reportable handoffs.

## The Governed Work Pattern

The larger architecture is called the Governed Work Pattern.

It describes a disciplined loop for serious AI-assisted work:

1. Read required context.
2. Respect explicit product and policy boundaries.
3. Make scoped changes or recommendations.
4. Run validation and quality gates.
5. Preserve evidence.
6. Require approval for sensitive or irreversible actions.
7. Produce a structured work report.
8. Disclose incomplete, deferred, skipped, or uncertain work.

This pattern applies to software engineering, but it is not limited to software engineering.

A product marketing workflow might read roadmap facts, launch constraints, customer evidence, and product decisions, then produce narrative drafts without constantly asking engineering for status.

A security workflow might triage alerts, cite evidence, classify risk, escalate ambiguous findings, and preserve what was not remediated.

A legal workflow might review contract language against policy, cite source clauses, flag ambiguous obligations, and require approval before anything leaves the company.

A finance workflow might reconcile exception evidence, apply approval thresholds, preserve audit context, and produce a decision packet.

In each case, the value is not that an AI can write words. The value is that the work is governed.

## Harnesses, Agents, And Tools

Workflow OS uses the word "harness" carefully.

A harness is a bounded execution envelope. It is not synonymous with an agent.

A harness may contain:

- a model or coding agent;
- deterministic code;
- a local skill;
- read-only tools;
- future write-capable tools;
- policy checks;
- validation;
- human approval;
- typed handoff requirements.

This distinction matters because the future is not simply "agents managing agents." The more useful direction is governed execution envelopes that can be composed safely.

That future direction is captured as Composable Harness Contracts. A composable harness contract should eventually define the harness name, purpose, allowed inputs, required context, allowed tools, allowed side effects, output schema, evidence requirements, approval policy, timeout and retry policy, failure semantics, and handoff requirements.

The core model exists as vocabulary and validation. Runtime nested harness execution does not exist yet.

That is deliberate. Nested harnesses are powerful, but they are dangerous if introduced before identity, durable state, evidence, policy gates, approval, typed handoffs, scoped authority, validation, and final work reports are stable.

## The Role Of Hooks

The current agent scaffold is useful for orientation. It tells Codex, Claude Code, or another coding agent how to behave in a Workflow OS repository. It makes the operating model easy to adopt:

```text
Agent executes. Workflow OS governs.
```

But instructions are not enforcement.

The next maturity layer is hook infrastructure: deterministic, named checkpoints that the harness invokes before or after important phases of work. Hooks should make governance less dependent on an agent remembering prose instructions.

Workflow OS has already implemented hook contract vocabulary, in-memory hook invocation helper models, hook disclosure models, and selected executor hook checkpoint behavior. Runtime hook execution is still carefully bounded and explicit. Hooks do not silently enable command execution, workflow runs, approvals, local checks, writes, hosted behavior, recursive agents, agent swarms, or higher autonomy.

Hooks are part of the move from "please remember the rules" to "the harness invokes the checkpoint."

## The Work Report

The work report is one of the most important ideas in Workflow OS.

It is not a marketing summary. It is not an audit log. It is not a transcript. It is a governed handoff artifact.

A good work report should answer:

- what work was performed;
- what evidence was considered;
- what decisions were made;
- which policy gates were evaluated;
- which approvals were requested, granted, or denied;
- which validation and quality checks ran;
- which side effects were none, skipped, unsupported, denied, proposed, or eventually completed;
- what remains incomplete or deferred;
- what limitations and risks remain;
- what the next operator should know.

The report should cite stable references rather than copying raw payloads. It should be bounded and redacted. It should be useful to a person, but grounded in structured state.

This is how Workflow OS starts to turn AI work into something that can be handed from one person, harness, team, or workflow to another without losing the plot.

## Why This Is Different From A Task Runner

A normal task runner can run commands.

Workflow OS is trying to answer a different question: under what governance boundary should work proceed?

That means the important pieces are not only execution steps. They are identity, policy, approval, audit, evidence, reportability, and side-effect boundaries.

The kernel cares about questions like:

- Is this workflow version the same one that started the run?
- Did this run mutate after reaching a terminal state?
- Was this approval requested before the gated step?
- Did a duplicate run ID cause duplicated side effects?
- Did report generation failure change the workflow result?
- Did a citation fabricate evidence?
- Did debug output leak a token-like value?
- Did a future write path get modeled before authority was explicit?

These are governance questions. They are why Workflow OS is a kernel, not just a wrapper around commands.

## A Concrete Example

Imagine a team using Workflow OS to govern an AI-assisted software change.

The workflow starts by validating the project and reading the relevant engineering standard. It runs a planning step. The plan requires approval before implementation. After approval, an agent edits the repository. The workflow records that the implementation step happened. Local checks run through explicit handlers where they exist. A review step captures findings. A final work report cites validation, local check results, relevant evidence references, hook disclosures, typed handoffs, and any side-effect records.

The agent still writes the code. The human still approves sensitive checkpoints. The local machine still runs the tools.

Workflow OS governs the shape of the work.

Now imagine the same pattern in a product narrative workflow. A PMM wants launch messaging based on product, engineering, and design facts. The workflow declares required context, reads allowed sources, cites evidence, flags missing roadmap facts, separates claims from assumptions, requires approval before external-facing language, and produces a report that explains what was used and what remains uncertain.

That is the broader point. Workflow OS is not only for code. It is for governed work.

## Roadmap Direction

The roadmap is intentionally phased.

The project first builds the local deterministic kernel: specs, validation, local execution, durable state, policy, approvals, audit, and inspection.

Then it adds governed workflow depth: multi-step execution, evidence references, work reports, hook checkpoints, typed handoffs, side-effect modeling, and local check handlers.

Then it can move toward more serious capabilities: side-effect discovery, write-capable adapters, stronger approval models, composable harness contracts, nested harness execution patterns, and eventually reasoning lineage or claim graph concepts.

The ordering matters. If writes come before side-effect authority, the system becomes unsafe. If nested harnesses come before typed handoffs, context drifts. If model review replaces deterministic gates, governance becomes theater. If reports copy raw payloads, privacy and security degrade.

Workflow OS is trying to build the boring substrate first so the powerful things can be added without pretending.

## The Short Version

Workflow OS is a local-first governed workflow kernel for AI-assisted work.

It helps define the work, validate the work, run bounded local workflows, pause for approvals, preserve event history, cite evidence, model reports, and prepare for safe side-effect handling.

It is not an agent swarm, not a hosted automation platform, not a production distributed runtime, and not a write-capable adapter framework yet.

Its opinion is simple:

AI execution is becoming easy.

Governed execution is the hard part.

Workflow OS is building the governed substrate.
