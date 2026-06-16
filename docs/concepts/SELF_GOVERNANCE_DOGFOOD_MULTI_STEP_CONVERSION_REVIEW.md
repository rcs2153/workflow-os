# Self-Governance Dogfood Multi-Step Conversion Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The self-governance dogfood workflow has been converted from a single approval-gated placeholder step into a bounded sequential multi-step placeholder workflow. The conversion uses the hardened local multi-step executor, keeps approval scoped to the planning checkpoint, and preserves the kernel-governed, Codex/human-executed boundary.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved dogfood conversion scope.

Implemented scope:

- converted the existing dogfood workflow `dg/d` to five ordered local steps;
- kept the workflow local, Level 2, and approval-gated;
- scoped approval to `planning-approved`;
- kept the existing placeholder skill local/read-only and low-sensitivity;
- updated dogfood README, roadmap, and planning/report docs;
- added CLI integration tests against the real dogfood project with isolated temporary state.

No accidental scope expansion was found for:

- runtime code changes;
- real build-command execution;
- arbitrary shell execution;
- automatic Codex control through the kernel;
- automatic runtime report generation;
- automatic report artifact writing;
- report CLI rendering;
- workflow schema changes;
- examples outside the dogfood project;
- branching execution;
- parallel or DAG execution;
- nested harness runtime behavior;
- side-effect boundary implementation;
- write behavior;
- approval evidence attachment;
- command-output evidence attachment;
- reasoning lineage or claim graph behavior;
- hosted or distributed runtime behavior;
- release posture changes.

## 3. Dogfood Workflow Assessment

The converted workflow remains `dg/d`, preserving continuity for existing dogfood instructions and historical reports.

The new step sequence is clear and bounded:

- `scope-requested`;
- `planning-approved`;
- `implementation-handoff`;
- `validation-disclosure`;
- `review-and-report-posture`.

This is an appropriate first dogfood use of sequential multi-step execution. The steps are authored workflow checkpoints, not nested harness runtime behavior, agent orchestration, or command execution.

The non-final steps use `terminal_behavior: continue`, and the final step preserves the existing terminal shape. No branches or parallel paths were added.

## 4. Approval Boundary Assessment

Approval is correctly scoped to `planning-approved`.

The conversion avoids the previous workflow-level approval requirement, which would have made every step require an explicit `approval_policy` under current validation rules. The placeholder skill now has `approval_sensitivity: low`, so non-approval steps do not accidentally become approval-gated.

The CLI tests verify:

- the run pauses at `planning-approved`;
- the approval ID contains `planning-approved`;
- only `scope-requested` is invoked before approval;
- approval grant resumes and completes all downstream steps;
- approval denial fails the run before downstream steps are requested.

## 5. Governance Boundary Assessment

The dogfood project remains honest about its current authority model.

The kernel governs:

- project validation;
- run identity;
- event history;
- step scheduling;
- policy decisions;
- approval pause/resume;
- placeholder checkpoint execution;
- audit and observability events.

Codex or a human still performs:

- repository edits;
- validation command execution;
- review judgment;
- final implementation reporting.

No docs or specs claim that the kernel runs Rust/npm checks, mutates files, calls providers, controls Codex, or performs production self-hosting.

## 6. Skill And Policy Assessment

The reused `local/d` placeholder skill is acceptable for the first conversion. It keeps the spec small and avoids multiplying placeholder skills without a stronger behavioral distinction.

The skill remains:

- local;
- deterministic through explicit mock registration;
- read-only in declared capability;
- bounded to non-secret literal inputs;
- low approval sensitivity;
- explicit about preserving the kernel-governed, Codex-executed boundary.

The policy remains conservative and local. It supports the scoped planning approval without introducing new policy semantics.

## 7. Test Quality Assessment

The new CLI tests are meaningful runtime tests because they execute the real dogfood project through the CLI and inspect persisted event history.

Strong coverage:

- dogfood project validation;
- pause at the planning approval step;
- pre-approval invocation limited to `scope-requested`;
- approval grant completes all five declared steps in order;
- approval denial stops downstream step invocation;
- temporary isolated state prevents test pollution.

Remaining non-blocking gaps:

- cancellation while waiting on the dogfood planning approval is not directly tested, although the core multi-step cancellation behavior is covered in the local executor suite;
- duplicate run-id rehydration for the dogfood project is not directly tested, although multi-step idempotency is covered in core tests;
- report-bearing dogfood execution is not directly tested, consistent with the report/artifact non-scope of this conversion.

These are not blockers because the conversion scope was a dogfood spec/docs/test conversion, and the runtime-level hardening suite already covers cancellation and replay behavior.

## 8. Documentation Review

Living docs are mostly accurate:

- `README.md` describes the dogfood project as a sequential multi-step governance wrapper.
- `dogfood/workflow-os-self-governance/README.md` explains the new checkpoints and keeps the Codex/human execution boundary explicit.
- `ROADMAP.md` describes the current dogfood slice as sequential multi-step.
- `docs/concepts/SELF_GOVERNANCE_DOGFOOD_MULTI_STEP_CONVERSION_REPORT.md` states scope completed, non-scope, validation, limitations, and recommended next phase.

Non-blocking documentation issue:

- `docs/implementation-plans/self-governance-dogfood-multi-step-conversion-plan.md` now has implemented status but still contains some historical “Current Dogfood State” wording that describes the previous single-step workflow. This is not dangerous because the status and living docs are correct, but it should be cleaned up in a small docs follow-up.

## 9. Privacy And Redaction Assessment

The converted dogfood workflow uses non-secret literal inputs only:

- `workflow-os-planning-docs-task`;
- bounded dogfood boundary labels.

No raw command output, raw spec contents, parser payloads, environment values, provider payloads, credentials, authorization headers, private keys, token-like values, or unbounded natural-language payloads were introduced.

The test approvals use bounded non-secret reasons. The workflow still depends on explicit mock handler registration for placeholder skill execution.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Clean up the implemented conversion plan’s historical “Current Dogfood State” section so it describes the current five-step workflow or clearly labels the prior state.
- Add a dogfood cancellation test if future changes touch approval/cancellation behavior.
- Add a dogfood duplicate run-id replay test if future changes touch idempotency or run rehydration.
- Consider report-bearing dogfood execution tests only after report exposure/artifact posture is separately scoped.
- Keep real local check execution, default handler registration, command-output evidence, and typed handoff integration deferred until separate plans are accepted.

## 12. Recommended Next Phase

Recommended next phase: self-governance dogfood multi-step conversion docs cleanup.

This should be a tiny documentation-only follow-up to align the implemented plan’s historical current-state wording with the now-converted dogfood workflow. After that, the next substantive roadmap phase should be selected deliberately: either dogfood review hardening tests, local check handler default-registration planning, or typed handoff integration planning. Real command execution should remain deferred.

## 13. Validation

- `cargo fmt --all --check` - pass.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass.
- `cargo test --workspace` - pass.
- `npm run check:docs` - pass.
