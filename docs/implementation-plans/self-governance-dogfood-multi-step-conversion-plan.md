# Self-Governance Dogfood Multi-Step Conversion Plan

Status: Implemented. Governed multi-step local execution is implemented, hardened, and reviewed. The self-governance dogfood workflow now uses a small sequential multi-step placeholder workflow while remaining kernel-governed and Codex/human-executed.

## 1. Executive Summary

Workflow OS now has a hardened sequential local multi-step executor slice. The next dogfood step is to convert the self-governance dogfood project from one approval-gated governance step into a small sequential governed workflow.

The conversion should prove that Workflow OS can govern its own build process through multiple deterministic checkpoints without pretending the kernel performs the actual implementation work. The boundary remains kernel-governed and Codex- or human-executed.

This plan has now been implemented as a dogfood spec/docs/test conversion. It does not add runtime code, real build-command execution, automatic Codex control, CLI behavior, workflow schema changes, examples outside the dogfood project, branching, parallelism, nested harness execution, writes, side-effect modeling, reasoning lineage, or release posture changes.

## 2. Goals

- Convert the dogfood workflow into a small sequential multi-step workflow.
- Exercise the hardened multi-step kernel against Workflow OS's own governed work.
- Preserve the existing kernel-governed, Codex-executed boundary.
- Add clear step boundaries for planning, approval, implementation handoff, validation/check disclosure, review, and final report posture.
- Keep each step local, deterministic, bounded, and auditable.
- Preserve explicit approval where human review is required.
- Preserve current validation and local executor behavior.
- Avoid command execution from the kernel unless a separately approved local check handler exists for that command.
- Keep report and citation behavior reference-oriented and honest.

## 3. Non-Goals

This plan does not authorize:

- implementation beyond the scoped dogfood conversion;
- runtime code changes;
- automatic runtime report generation;
- automatic report artifact writing;
- report CLI rendering;
- workflow schema changes;
- example updates outside the dogfood project;
- branching execution;
- parallel or DAG execution;
- nested harness runtime behavior;
- Composable Harness Contract runtime behavior;
- real build-command execution from the kernel;
- arbitrary shell execution;
- automatic Codex control through the kernel;
- recursive agents, agent swarms, or agent-to-agent orchestration;
- side-effect boundary implementation;
- write behavior;
- approval evidence attachment;
- command-output evidence attachment;
- reasoning lineage or claim graph implementation;
- hosted or distributed runtime behavior;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 4. Current Dogfood State

The current dogfood project lives at `dogfood/workflow-os-self-governance`.

Current behavior:

- The project validates through the normal Workflow OS loader and validator.
- The workflow `dg/d` has one local step, `d`.
- The step uses the placeholder skill `local/d`.
- The step is approval-gated through `approval/d`.
- The workflow is Level 2.
- CLI runs require explicit `--mock-all-local-skills` to use deterministic mock handler behavior.
- Codex or a human still performs repository edits and validation commands outside the kernel.

Current limitations:

- The dogfood workflow does not yet exercise multi-step execution.
- It does not separate planning, implementation handoff, validation disclosure, review, and final report posture into distinct governed steps.
- It does not execute real local validation/check handlers by default.
- It does not generate or persist report artifacts automatically.

## 5. Recommended First Conversion Boundary

The first conversion should remain small and sequential.

Recommended step set:

1. `scope-requested`
   - Purpose: record the non-secret task kind and current dogfood authority boundary.
   - Approval: no separate approval if the following planning approval remains required.
   - Handler: deterministic local placeholder or existing mock local handler.

2. `planning-approved`
   - Purpose: require human approval before Codex or a human proceeds with implementation work.
   - Approval: required.
   - Handler: deterministic local placeholder.

3. `implementation-handoff`
   - Purpose: record that implementation is performed outside the kernel by Codex or a human.
   - Approval: optional for first conversion; do not add extra approval unless useful.
   - Handler: deterministic local placeholder.

4. `validation-disclosure`
   - Purpose: record that validation/check commands remain outside the kernel unless a real handler is separately scoped.
   - Approval: no.
   - Handler: deterministic local placeholder.

5. `review-and-report-posture`
   - Purpose: record expected final review/report obligations and known limitations.
   - Approval: optional for first conversion.
   - Handler: deterministic local placeholder.

This step set is intentionally not a harness runtime. It is an authored sequential workflow that demonstrates multiple governed checkpoints while preserving the current execution boundary.

## 6. Skill Model Recommendation

Use the smallest skill model that keeps the dogfood boundary honest.

Recommended first implementation:

- keep `local/d` only if reusing one placeholder skill across multiple steps is simpler;
- or add narrowly named local placeholder skills such as:
  - `local/dogfood-scope`
  - `local/dogfood-planning-approval`
  - `local/dogfood-implementation-handoff`
  - `local/dogfood-validation-disclosure`
  - `local/dogfood-review-posture`

Each skill must:

- accept only non-secret literal context;
- declare `local.read` only;
- avoid filesystem, network, shell, provider, or write capabilities;
- preserve the kernel-governed, Codex-executed statement;
- use bounded outputs such as `summary`;
- avoid raw command output, raw spec contents, environment values, paths with secrets, tokens, credentials, or provider payloads.

Do not create real command-running skills in this phase.

## 7. Approval Policy Recommendation

Keep the first multi-step dogfood conversion Level 2 and approval-gated.

Recommended behavior:

- maintain at least one human approval before implementation proceeds;
- bind the approval to the planning/approval step rather than all steps;
- keep approval reason text bounded and non-secret;
- preserve existing approval audit events;
- ensure denial fails the run before downstream steps execute;
- ensure approval grant resumes without re-running prior steps.

Do not add approval evidence attachment in this phase.

## 8. Terminal Behavior Recommendation

Use sequential terminal behavior conservatively:

- every non-final successful step should use `terminal_behavior: continue`;
- the final step should use the existing terminal completion/failure behavior that preserves current validation rules;
- do not use branches;
- do not create conditional paths;
- do not rely on unsupported terminal behavior variants.

The conversion should prove ordered progression, not topology complexity.

## 9. Validation And Check Boundary

The dogfood workflow may include a validation disclosure step, but it must not claim to run checks unless a real handler is explicitly registered and scoped.

Allowed in the first conversion:

- text stating that Codex or a human must run validation commands outside the kernel;
- citations or report notes supplied by later explicit report inputs, if already supported;
- optional use of existing test-only local check handlers only in tests, not as a production dogfood default.

Not allowed:

- `cargo`, `npm`, or shell execution from the dogfood workflow;
- automatic validation after implementation;
- default handler registration for local checks;
- command-output evidence;
- raw logs in workflow state or report text.

## 10. Report And Citation Posture

The first conversion should remain compatible with existing report-bearing executor paths but must not make report generation automatic.

Recommended posture:

- the workflow should be able to run through `execute_with_report(...)` in tests where explicit report inputs are supplied;
- report sections should remain bounded and citation-oriented;
- absent validation/check references should remain explicit not-available section text;
- no report artifacts should be written automatically;
- no CLI report rendering should be added;
- final implementation reports remain human-authored Codex output until runtime report exposure is separately expanded.

## 11. Privacy And Redaction

The dogfood workflow must treat project paths, task text, approval reasons, report notes, and validation summaries as potentially sensitive.

Rules:

- no secrets in YAML specs;
- no raw command output;
- no raw spec contents;
- no parser payloads;
- no environment variable values;
- no provider payloads;
- no credentials, authorization headers, private keys, or token-like values;
- no unbounded natural-language payloads;
- errors must use stable codes and avoid leaking payload values.

## 12. Test Plan For Future Implementation

Implementation tests should cover:

- dogfood project validates after conversion;
- multi-step dogfood run schedules every declared step in order;
- planning approval step pauses before downstream steps;
- approval grant resumes without re-running prior steps;
- approval denial stops before downstream steps;
- cancellation while waiting for approval stops before downstream steps;
- final successful dogfood run completes;
- duplicate run ID rehydrates without repeating steps;
- report-bearing execution can return an in-memory report for a completed dogfood run when explicit report inputs are supplied;
- no report artifacts are written automatically;
- no local check command is executed by default;
- no filesystem writes are introduced beyond normal runtime state;
- no CLI behavior changes are required;
- documentation remains honest about Codex/human execution.

Validation commands for the implementation phase:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

## 13. Proposed Implementation Sequence

Recommended small future phases:

1. Convert dogfood workflow specs to a sequential multi-step placeholder workflow.
2. Update dogfood README with run/approve/inspect instructions for the new approval step.
3. Add focused CLI or core tests that validate and run the dogfood workflow through the local executor.
4. Review the conversion before introducing real local check execution.
5. Only after review, consider whether the dogfood workflow should reference explicit local check result citations through existing report inputs.
6. Defer real command execution and default check handler registration until separately approved.

## 14. Open Questions

- Should the first converted dogfood workflow use one reused placeholder skill or multiple narrowly named placeholder skills?
- Should only the planning step require approval, or should the final review/report posture step require a second approval?
- Should dogfood tests use CLI fixtures, core executor fixtures, or both?
- Should the workflow ID remain `dg/d` for continuity, or should it become a clearer ID such as `dg/self-governance`?
- Should report-bearing dogfood execution be tested now or deferred until after the conversion review?
- What is the smallest useful final report posture step that does not imply automatic report generation?

## 15. Final Recommendation

The next implementation phase should be: self-governance dogfood multi-step conversion.

That phase should convert the dogfood project to a small sequential local workflow with explicit planning, approval, implementation handoff, validation disclosure, and review/report posture steps. It must preserve the kernel-governed, Codex-executed boundary and must not add command execution, automatic reports, artifacts, CLI behavior, schemas, examples, writes, branching, parallelism, nested harness runtime behavior, or reasoning lineage.
