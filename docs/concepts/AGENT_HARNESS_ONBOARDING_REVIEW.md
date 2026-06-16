# Agent Harness Onboarding Review

Review date: 2026-06-16

## 1. Executive Verdict

Phase accepted; proceed to agent harness CLI scaffold planning.

The onboarding phase makes the intended kernel-governed agent loop visible without adding runtime automation or overclaiming current capabilities. The documentation consistently frames the product behavior as:

```text
Agent executes. Workflow OS governs.
```

## 2. Scope Verification

The phase stayed within documentation/onboarding scope.

Completed scope:

- root `AGENTS.md` instructions;
- `docs/user-guide/agent-harness-quickstart.md`;
- `docs/implementation-plans/agent-harness-onboarding-plan.md`;
- `docs/concepts/AGENT_HARNESS_ONBOARDING_REPORT.md`;
- README onboarding section;
- user-guide index update;
- dogfood README update;
- roadmap placement;
- Governed Work Pattern relationship note.

No accidental scope expansion was found:

- no runtime harness generation;
- no CLI `init-agent-harness`;
- no workflow schema fields;
- no automatic local check execution;
- no default local command handler registration;
- no report CLI rendering;
- no automatic work-report generation;
- no command-output evidence;
- no approval evidence attachment;
- no reasoning lineage;
- no side-effect boundary modeling;
- no write behavior;
- no hosted or distributed runtime behavior;
- no recursive agents or agent swarms;
- no Level 3 or Level 4 autonomy claims;
- no release posture change.

## 3. Onboarding Assessment

The new onboarding path addresses the user-feedback gap directly. It explains that users should not treat manual YAML authoring as the whole experience and instead should connect a coding agent to the local kernel as the governing layer.

The quickstart gives a concrete path:

1. build the local CLI;
2. validate the dogfood project;
3. start a governed dogfood run;
4. paste the canonical setup prompt into a coding agent;
5. approve kernel checkpoints;
6. let the agent execute bounded repository work;
7. inspect the run and review the final report.

This is a good first-step adoption layer because it is useful immediately and does not require new runtime capabilities.

## 4. Prompt And Agent Instruction Assessment

The canonical prompt is appropriately bounded.

It instructs agents to:

- read required engineering and roadmap docs;
- validate before work;
- start or resume governed workflows where required;
- treat approvals as mandatory checkpoints;
- stay inside approved scope;
- avoid inventing workflow state, approvals, evidence, audit events, reports, validation results, or command outputs;
- run phase-required validation;
- report completed scope, deferred scope, validation, and next phase.

`AGENTS.md` is conservative and repository-local. It guides coding agents toward the Workflow OS governance loop without claiming that Workflow OS executes agents directly.

## 5. Product Boundary Assessment

The docs preserve the Workflow OS product boundary.

They state that Workflow OS is the governance layer and that the coding agent performs repository work. They do not present Workflow OS as a chat-agent framework, recursive agent runtime, agent swarm, hosted orchestrator, production self-hosting system, or Level 3/4 autonomy runtime.

The docs correctly distinguish agent harness onboarding from Composable Harness Contracts and nested harness runtime behavior.

## 6. Safety And Governance Assessment

The onboarding path preserves safety boundaries.

The docs explicitly prohibit:

- bypassing validation, policy, approvals, failed checks, or scope limits;
- mutating workflow state by hand;
- fabricating run IDs, approval IDs, evidence references, audit events, reports, validation results, or command outputs;
- replacing deterministic validation with model self-review;
- claiming automatic build/check execution without explicit handler support.

This is the right posture for making the experience feel simple without making authority ambient.

## 7. Future Automation Assessment

The future `workflow-os init-agent-harness` idea is positioned correctly as unimplemented.

The docs describe it as a possible scaffold command that would generate agent instructions and starter prompt material. They also state that it must not silently enable local command execution, writes, hosted behavior, schema changes, or higher autonomy.

This keeps the next “magic setup” direction visible while preserving the current implementation truth.

## 8. Documentation Review

Documentation updates are coherent across:

- `README.md`;
- `docs/user-guide/README.md`;
- `docs/user-guide/agent-harness-quickstart.md`;
- `dogfood/workflow-os-self-governance/README.md`;
- `docs/concepts/governed-work-pattern.md`;
- `ROADMAP.md`;
- `AGENTS.md`.

The new docs consistently avoid forbidden framing such as recursive agents, agent swarms, self-governing agents, or fully autonomous software factory positioning.

## 9. Blockers

No blockers.

## 10. Non-Blocking Follow-Ups

- Consider whether `AGENTS.md` should eventually be generated by a CLI scaffold command rather than manually maintained.
- Consider separate Codex and Claude Code prompt variants only if user testing shows the shared prompt is insufficient.
- Consider a future CLI post-validate hint that points users to the agent harness quickstart, but do not add CLI output without a scoped plan.
- Keep future scaffold work docs-first and non-executing until side-effect and local command authority boundaries are reviewed.

## 11. Recommended Next Phase

Recommended next phase: agent harness CLI scaffold planning.

The next planning phase should define a documentation/scaffold-only command such as `workflow-os init-agent-harness` that can generate or update `AGENTS.md` and a session prompt. It must not add runtime automation, automatic local check execution, workflow schema fields, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 12. Validation

Validation commands for this review:

- `npm run check:docs`
  - Passed.
