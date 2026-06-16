# Self-Governance Dogfood Hardening Test Plan

Status: Planned.

## 1. Executive Summary

The self-governance dogfood workflow is now a reviewed five-step sequential workflow. The next phase should harden the dogfood test surface around the remaining non-blocking gaps from the conversion review.

This phase is test-only. It should add focused tests for the real dogfood project covering cancellation at the planning approval checkpoint, duplicate run-id replay/rehydration behavior, and report-bearing dogfood execution through existing explicit report-bearing executor APIs.

This plan does not authorize runtime behavior changes, real command execution, default handler registration, automatic report generation, report artifacts, CLI report rendering, schemas, typed handoff runtime behavior, reasoning lineage, side-effect modeling, writes, or release posture changes.

## 2. Goals

- Prove the converted dogfood workflow behaves safely in additional lifecycle edges.
- Add cancellation coverage while the dogfood workflow waits at `planning-approved`.
- Add duplicate run-id replay or rehydration coverage for the dogfood workflow.
- Add report-bearing dogfood execution coverage using existing explicit report input paths, if practical without runtime broadening.
- Preserve the kernel-governed, Codex/human-executed boundary.
- Keep tests deterministic, local, isolated, and non-flaky.
- Avoid real command execution from the dogfood workflow.
- Avoid report artifacts, persistence beyond normal test state, and CLI rendering.

## 3. Non-Goals

This plan does not authorize:

- runtime implementation changes;
- dogfood workflow spec changes;
- skill or policy changes;
- real local validation/check command execution;
- default local check handler registration;
- arbitrary shell execution;
- automatic runtime report generation;
- automatic report artifact writing;
- report CLI rendering;
- workflow schema changes;
- examples outside test/docs updates;
- branching execution;
- parallel or DAG execution;
- nested harness runtime behavior;
- Composable Harness Contract runtime behavior;
- typed handoff runtime behavior;
- command-output evidence attachment;
- approval evidence attachment;
- reasoning lineage or claim graph implementation;
- side-effect boundary implementation;
- write behavior;
- hosted or distributed runtime behavior;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 4. Current Dogfood Baseline

The current dogfood project lives at `dogfood/workflow-os-self-governance`.

The workflow `dg/d` has five ordered local steps:

1. `scope-requested`
2. `planning-approved`
3. `implementation-handoff`
4. `validation-disclosure`
5. `review-and-report-posture`

The workflow is Level 2, local, and approval-gated at `planning-approved`. It uses deterministic placeholder local skill behavior under explicit mock local skill registration. Codex or a human still performs repository edits and validation commands outside the kernel.

Existing dogfood CLI tests cover:

- dogfood project validation;
- pause at `planning-approved`;
- pre-approval invocation limited to `scope-requested`;
- approval grant completing all five steps in order;
- approval denial stopping downstream step invocation.

## 5. Approved Test Targets

### Cancellation While Waiting For Planning Approval

Add a dogfood test that:

- starts `dg/d` with isolated state;
- confirms the run is `WaitingForApproval`;
- cancels the run before approval is granted;
- verifies the run becomes `Canceled`;
- verifies downstream steps after `planning-approved` were not invoked;
- verifies no post-cancellation step invocations are appended.

### Duplicate Run-ID Replay Or Rehydration

Add a dogfood test that exercises the existing duplicate run-id behavior against the real dogfood project.

The test should:

- use isolated state;
- run the dogfood workflow with a stable duplicate/replay setup if the CLI or core helper supports explicit run IDs;
- or use the smallest existing core/local-executor path that can load the dogfood project and prove replay does not duplicate already completed steps;
- verify step invocation order remains stable;
- verify prior completed steps are not repeated;
- verify event history stays append-only and non-ambiguous.

If direct CLI duplicate run-id control is not currently available, document the limitation in the implementation report and add the strongest practical core-level dogfood replay test without adding new CLI behavior.

### Report-Bearing Dogfood Execution

Add a dogfood report-bearing test only through existing explicit APIs.

The test should:

- load or construct the real dogfood workflow through existing test helpers;
- provide explicit report identity, contract identity/version, actor, sensitivity, redaction metadata, and bounded notes;
- run the completed dogfood path through existing report-bearing local executor APIs if practical;
- verify the report is returned in memory;
- verify all required report sections are present;
- verify absent validation/check references remain explicit not-available section text;
- verify no report artifact is written;
- verify no CLI report rendering is introduced.

If report-bearing dogfood execution is awkward through current CLI helpers, prefer a focused core/local-executor test over adding CLI behavior.

## 6. Test Design Rules

- Use isolated temporary state directories.
- Use deterministic mock local skill handling only.
- Inspect durable event history, not just return values.
- Assert step IDs, run status, approval IDs, and event ordering where relevant.
- Avoid sleeps, wall-clock assumptions, network calls, shell calls, and shared state.
- Do not rely on command output beyond bounded CLI status text already used in tests.
- Do not create filesystem artifacts other than normal isolated runtime state.
- Do not mutate repository files from tests.

## 7. Privacy And Redaction

Tests must not introduce:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw GitHub or Jira bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded natural-language payloads.

Approval reasons, report notes, limitations, risks, and handoff notes must remain bounded and non-secret. Error assertions should use stable codes or bounded status fields rather than raw payload matching.

## 8. Documentation Updates

Update only if needed:

- `docs/implementation-plans/self-governance-dogfood-hardening-test-plan.md`
- `ROADMAP.md`
- `dogfood/workflow-os-self-governance/tests/README.md`
- `docs/concepts/SELF_GOVERNANCE_DOGFOOD_HARDENING_TEST_REPORT.md`

Docs must say:

- dogfood hardening tests are implemented, if the implementation phase completes;
- the dogfood workflow remains kernel-governed and Codex/human-executed;
- real local check execution is not implemented;
- default handler registration is not implemented;
- automatic report generation and report artifacts are not implemented;
- CLI report rendering is not implemented;
- typed handoff runtime behavior is not implemented;
- command-output evidence is not attached;
- reasoning lineage is not implemented;
- side-effect boundary and writes remain unsupported.

## 9. End-Of-Phase Report

Create:

- `docs/concepts/SELF_GOVERNANCE_DOGFOOD_HARDENING_TEST_REPORT.md`

The report must include:

1. executive summary;
2. scope completed;
3. scope explicitly not completed;
4. cancellation test summary;
5. duplicate run-id/replay test summary;
6. report-bearing dogfood execution test summary;
7. dogfood governance boundary summary;
8. privacy/redaction summary;
9. commands run and results;
10. remaining known limitations;
11. recommended next phase.

Recommended next phase should be one of:

- self-governance dogfood hardening test review;
- local check handler default-registration planning;
- typed handoff integration planning;
- report-bearing dogfood execution planning;
- blocker fix;
- defer.

## 10. Validation

Run:

- targeted dogfood CLI/core tests added by the phase;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

Use the repository bundled toolchain paths if the desktop shell does not expose system Cargo or Node on `PATH`.

## 11. Acceptance Criteria

- Dogfood cancellation while waiting for planning approval is covered.
- Dogfood duplicate run-id replay or the strongest currently practical rehydration path is covered.
- Report-bearing dogfood execution is covered through existing explicit APIs, or a clear limitation is documented without adding new runtime behavior.
- Tests inspect state and event history where relevant.
- No runtime code, specs, skills, policies, CLI behavior, schemas, artifacts, real command execution, default handler registration, reasoning lineage, side effects, writes, or release posture changes are introduced.
- Validation passes.

## 12. Dogfood Governance For This Planning Phase

This planning phase was governed by the self-governance dogfood workflow before documentation edits:

- `workflow-os --project-dir dogfood/workflow-os-self-governance validate` passed with expected experimental lifecycle warnings;
- `workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-dogfood-hardening-plan-state-20260615 --mock-all-local-skills run dg/d` paused at `planning-approved`;
- approval was granted with bounded reason `dogfood-hardening-test-planning`;
- `inspect` showed the run completed with five checkpoint invocations and 34 durable events.

## 13. Final Recommendation

The next implementation phase should be: self-governance dogfood hardening tests.

It should remain a test-only phase focused on cancellation, duplicate run-id replay/rehydration, and report-bearing dogfood execution through existing explicit APIs. It must not add real local check execution, default handler registration, automatic reports, report artifacts, CLI rendering, schemas, typed handoff runtime behavior, command-output evidence, reasoning lineage, side effects, writes, or release posture changes.
