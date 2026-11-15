# Agent Harness Hook Integration Plan

Status: Planning accepted; the agent harness hook contract model is implemented. Runtime agent harness hooks are not implemented. The current `workflow-os init-agent-harness` command remains documentation/scaffold-only and does not run workflows, execute checks, approve gates, mutate runtime state, write report artifacts, add schemas, enable writes, host agents, or change release posture.

## 1. Executive Summary

The current agent scaffold is a human/agent orientation layer: useful for declaring conventions, expectations, and structure, but not itself an enforcement layer.

The next maturity layer is governed hooks: deterministic, named checkpoints that the harness invokes before or after important phases of work, instead of relying on the agent to remember and follow instructions.

This plan defines the future hook direction for Workflow OS. The model-only hook contract now exists, but it does not implement hook execution, runtime automation, automatic local checks, workflow schema fields, CLI hook commands, persistence, report artifacts, examples, reasoning lineage, side-effect modeling, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 2. Goals

- Make the agent harness less dependent on prose-only instruction following.
- Define a future hook layer for explicit governed checkpoints.
- Preserve the current mental model: `Agent executes. Workflow OS governs.`
- Keep hooks deterministic, named, bounded, and auditable.
- Reuse existing primitives where possible: workflow/run identity, policy gates, approvals, local check references, evidence references, typed handoffs, WorkReports, and audit records.
- Support a future path where agents invoke governance checkpoints through explicit commands or APIs.
- Avoid turning Workflow OS into a generic agent framework or agent swarm orchestrator.

## 3. Non-Goals

Do not implement in this phase:

- runtime hook execution;
- automatic workflow execution;
- automatic local check execution;
- default local check handler registration;
- command-output evidence;
- workflow schema fields;
- CLI hook commands;
- runtime harness generation;
- nested harness execution;
- recursive agents;
- agent swarms;
- hosted or distributed agent execution;
- side-effect modeling;
- writes;
- approval evidence attachment;
- reasoning lineage;
- persistence changes;
- report artifact auto-writing;
- examples;
- release posture changes.

## 4. Scaffold Versus Hooks

The scaffold and hook layers serve different purposes.

The current scaffold layer:

- creates or updates `AGENTS.md`;
- creates or updates `.workflow-os/agent-harness-prompt.md`;
- teaches the operating model;
- gives the agent a copy/paste prompt;
- remains documentation-only.

The future hook layer should:

- expose deterministic named checkpoints;
- be invoked explicitly by the harness or agent;
- validate inputs and identities;
- return structured results;
- fail closed on missing context or unsafe requests;
- avoid raw payload copying;
- preserve auditability and reportability.

The scaffold is useful but advisory. Hooks are the future integration boundary where governance becomes less brittle.

### Downstream Governance Posture Contract

After explicit repository initialization, cooperative governance and
authoritative observation are default-on; hard enforcement is explicit,
progressively stronger, and claimed only when technically proven.
**Agent executes. Workflow OS governs.**

Installing the binary or importing an SDK must not mutate a repository.
Explicit `init-repo-governance` defaults to the authoritative
`observe_and_report` project-validation scaffold. Preserve existing unmanaged
`AGENTS.md` text and append or update only the managed block by default;
generate `.workflow-os/agent-harness-prompt.md` as orientation. The explicit
`--no-authoritative-governance` legacy path omits that authoritative
scaffold; disclose the reduced guarantee rather than labeling it Observe.
`init-agent-harness` remains documentation-only and does not establish a
runnable project or enforcement boundary. See the
[quickstart](../user-guide/agent-harness-quickstart.md).

| Posture | Boundary and claim |
| --- | --- |
| **Observe** | Default after explicit repo initialization. Cooperative harnesses can discover managed instructions, project validation, declared checkpoints, and available durable governance and report paths. Bypass remains possible; files alone do not prove participation. |
| **Enforce** (future) | Explicit per repository, workflow, or policy. A versioned governed hook or gateway produces proof at consequential checkpoints. Missing, stale, skipped, or mismatched participation is detected and handled under declared policy. Claims cover only the integrated action boundary; direct paths may remain. |
| **Strict** (future) | Explicit deployment opt-in. Remove or mediate ambient reusable credentials and unrestricted consequential tool and egress paths outside agent control. Cells and gateways re-resolve current authority; they cannot mint, widen, renew, or serialize executable authority. Only Strict may claim technical non-bypassability, only at the tested boundary. It is not a default; deployment, recovery, revocation, and operational guarantees must first be implemented and accepted. |

Existing selected `BeforeReport` and `BeforeSkillInvocation` checks do not
establish either future deployment posture. The
[reference enforcement plane](durable-authority-continuation-profile-plan.md#phase-3a-reference-enforcement-plane)
owns the task-cell and gateway proof.

#### Posture Inspection

Have the architecture review choose the smallest stable public projection in
`first-run`, `doctor`, or another bounded inspection surface. Candidate facts
are: instructions installed; harness prompt installed; authoritative project
validation configured; harness acknowledgement observed or unknown; governed
hook configured and invocation observed as separate facts; gateway enforcement
configured; and strict credential and egress boundaries verified separately.
Show the effective posture for the scoped boundary (Observe, Enforce, Strict,
legacy, or unknown), with evidence or an explicit unknown where needed.

Do not infer acknowledgement from file presence, invocation from config,
enforcement from audit events, or strictness from sandbox presence. No new
public field names or commands are committed by this plan.

#### Hook Progression

1. Managed prose orientation.
2. Deterministic, named checkpoint contract.
3. Harness acknowledgement or registration.
4. Versioned, proof-producing hook invocation.
5. Policy-bound behavior for missing, stale, skipped, or mismatched hooks.
6. Consequential-tool mediation through a governed integration.
7. Strict credential and egress controls outside executor control.
8. Recovery, revocation, replacement-executor, and post-effect ambiguity proof.

This is a dependency sequence, not an Active Delivery Queue reorder. Hooks
must not silently enable arbitrary commands, provider calls, approvals,
local checks, writes, recursive agents, or higher autonomy.

#### Cold-Contributor Acceptance

Maintain one repeatable scenario in a real repository with existing contributor
instructions:

1. Explicitly initialize Workflow OS and verify unmanaged guidance survives.
2. Start a fresh supported harness and observe it discovering the managed policy.
3. Present the contributor with the concrete task, scope, non-goals, and
   validation expectations; record the approval decision durably.
4. Complete useful work through ordinary executor tooling; retain what was
   approved and what occurred in the run and report.
5. Have a fresh reviewer reconstruct the approval and outcome, and classify
   the tested boundary as cooperative Observe, integrated Enforce, or Strict.

A real external contribution provides bounded field evidence that a cold
contributor followed repository instructions, reviewed a scoped approval, and
completed useful work. It does not establish non-bypassability or satisfy the
full scenario. Do not identify or quote the contributor or publish screenshots
without consent.

#### Test And Claim Gates

- Verify unmanaged guidance preservation, idempotent managed-block updates,
  explicit initialization before setup mutation, and legacy opt-out disclosure.
- Verify that ignored prose is not reported as enforcement and that configured
  but never invoked hooks remain distinguishable from invocation proof.
- Reject stale, wrong-run, or wrong-scope proof for required checkpoints.
- Prevent Strict classification if direct credential or egress bypass exists.
- Fail closed on revocation, expiry, or inability to resolve current authority
  while disconnected; authenticate a replacement and re-resolve its authority.
- Preserve uncertainty after an effect but before acknowledgement without
  enabling blind retry. Use the existing reference enforcement-plane recovery
  tests rather than defining a separate execution mechanism.

## 5. Candidate Hook Checkpoints

Initial candidate hook names:

- `before_plan`
- `after_plan`
- `before_implementation`
- `after_implementation`
- `before_validation`
- `after_validation`
- `before_review`
- `after_review`
- `before_report`
- `after_report`

These names are illustrative. The first implementation should choose a very small set, likely one planning/checkpoint hook and one validation/reporting hook, after review.

Each hook should be explicit about:

- workflow ID;
- run ID;
- step ID or phase ID if available;
- actor or system actor;
- correlation ID;
- required context references;
- supplied evidence or local check result references;
- approval state if relevant;
- typed handoff IDs if relevant;
- bounded notes, risks, limitations, or disclosures.

## 6. Hook Contract Shape

A future hook contract should define:

- hook name;
- purpose;
- when it may be invoked;
- required inputs;
- optional inputs;
- allowed outputs;
- required policy checks;
- approval requirements;
- evidence requirements;
- local check requirements;
- failure semantics;
- redaction policy;
- sensitivity;
- report or handoff obligations.

Hook outputs should be structured. Natural-language summaries may be allowed as bounded annotations, but they should not become the source of truth for governance state.

## 7. Authority Boundary

Hooks must not grant ambient authority to the agent.

Future hook invocation must not silently authorize:

- workflow state mutation outside runtime APIs;
- approval decisions;
- local command execution;
- external provider calls;
- report artifact writes;
- filesystem writes beyond explicitly scoped documentation/scaffold behavior;
- side effects;
- Level 3/4 autonomy.

Any hook that can lead to side effects must wait until side-effect boundary modeling, high-assurance approval controls where needed, and write-capable adapter policy are separately accepted.

## 8. Relationship To Existing Primitives

Agent harness hooks should build on current primitives rather than invent parallel governance:

- Workflow/run identity anchors hook calls.
- Durable runtime state remains the source of truth.
- Policy gates decide whether a checkpoint may proceed.
- Approvals remain event-sourced runtime gates.
- Local check results and references represent deterministic checks.
- EvidenceReference supplies citation pointers, not raw payload copies.
- Typed handoffs carry structured transfer context.
- WorkReports summarize terminal governed work.
- Audit records preserve who/what invoked checkpoints and why.

## 9. Error Handling

Future hook errors should:

- use stable error codes;
- avoid leaking raw prompts, raw command output, raw provider payloads, environment values, paths, tokens, credentials, or secret-like values;
- fail closed when context, identity, approval, policy, evidence, or check requirements are missing;
- distinguish hook failure from workflow failure unless a scoped runtime design says otherwise;
- never fabricate approvals, evidence, validation results, local check results, or reports.

## 10. Privacy And Redaction

Hook inputs and outputs must be bounded and redaction-aware.

Hooks must not copy:

- raw spec contents;
- raw command transcripts;
- raw provider payloads;
- raw CI logs;
- raw Jira or GitHub bodies;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

File paths should be treated conservatively. If paths are used as references, they should remain bounded and should not be expanded into raw contents or unbounded summaries.

## 11. First Implementation Recommendation

Do not implement runtime hooks yet.

The first implementation after review is complete: **agent harness hook contract model, model-only**.

That phase added the smallest Rust model necessary to represent a hook contract and validate:

- hook identity;
- hook kind/name;
- required inputs;
- required outputs;
- redaction policy;
- sensitivity;
- failure semantics;
- no side-effect authorization.

It does not execute hooks, add CLI behavior, add schema fields, run checks, write artifacts, mutate runtime state, or introduce hosted behavior.

## 12. Test Plan For Future Implementation

Future model-only tests should cover:

- valid minimal hook contract;
- invalid hook ID/name rejection;
- required input requirement validation;
- required output requirement validation;
- duplicate hook requirements rejected;
- redaction policy validation;
- sensitivity validation;
- failure semantics validation;
- side-effect authorization rejected;
- serde round trip for valid contracts;
- invalid serialized contract fails closed;
- Debug output does not leak secret-like values;
- serialization does not leak forbidden raw payload fields;
- no runtime execution, CLI behavior, schema changes, persistence, report artifacts, writes, or hosted behavior introduced.

Future runtime hook tests should not be planned until the model-only hook contract has been reviewed.

## 13. Documentation Updates For Future Implementation

The model-only hook contract implementation updated:

- `ROADMAP.md`;
- `docs/concepts/governed-work-pattern.md`;
- `docs/user-guide/agent-harness-quickstart.md`;
- `docs/implementation-plans/agent-harness-onboarding-plan.md`;
- `docs/implementation-plans/agent-harness-cli-scaffold-plan.md`.

Docs must continue to say:

- scaffold files are orientation, not enforcement;
- hook contracts are model-only until runtime invocation is separately implemented;
- automatic workflow execution is not implemented;
- automatic local check execution is not implemented;
- writes remain unsupported;
- recursive agents and agent swarms remain non-goals.

## 14. Open Questions

- Should hooks be workflow-level, harness-level, step-level, or phase-level?
- Should hooks be declared in future workflow specs or remain local runtime/helper configuration first?
- What is the smallest useful first hook: planning checkpoint, validation checkpoint, or final report checkpoint?
- Should a hook invocation append audit events, or should that wait for runtime hook execution planning?
- Should hook failures block the workflow, warn only, or return structured checkpoint status?
- How should hooks cite local check results without creating command-output evidence?
- How should hooks interact with high-assurance approval controls?
- How should hook contract vocabulary relate to Composable Harness Contracts?

## 15. Final Recommendation

Proceed next to **agent harness hook contract model review**.

After review, runtime invocation planning is documented in [Agent Harness Hook Runtime Invocation Plan](agent-harness-hook-runtime-invocation-plan.md). Runtime hook invocation, CLI hook commands, schema fields, automatic local checks, persistence, report artifacts, examples, side-effect modeling, writes, hosted behavior, recursive agents, agent swarms, and release posture changes must remain out of scope until separately planned and accepted.
