# Self-Governed Build Benchmark Behavior Test Report

## 1. Executive Summary

The self-governed build benchmark behavior test phase is complete. A focused workflow-core test now proves the benchmark loop through existing explicit APIs without adding runtime behavior, CLI behavior, automatic checks, schemas, writes, hosted behavior, recursive agents, or release posture changes.

The tested posture remains:

```text
Agent executes. Workflow OS governs.
```

## 2. Scope Completed

- Added focused behavior tests in `crates/workflow-core/tests/local_executor.rs`.
- One test validates the dogfood project before execution.
- One test starts a governed dogfood run.
- One test verifies the run pauses at the planning approval checkpoint.
- The tests grant approval through the existing approval API.
- The tests complete the existing sequential dogfood workflow.
- One test executes the explicit `DocsCheckLocalHandler` path through non-default profile registration and an injected runner.
- One test rehydrates the completed run through `execute_with_report(...)`.
- One test supplies stable report references for local check result, typed handoff, and hook invocation citations.
- One test verifies all required v1 WorkReport sections are present.
- The tests verify no report artifacts are written automatically.
- The tests verify event history is preserved and no duplicate step execution occurs during report-bearing rehydration.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- runtime behavior changes;
- automatic kernel control of agents;
- automatic runtime report generation;
- runtime result exposure changes;
- CLI report rendering;
- CLI report artifact writing;
- automatic local check execution;
- default local check handler registration;
- arbitrary shell execution;
- workflow schema changes;
- workflow-declared hooks;
- runtime hook configuration;
- warning/skipped hook continuation;
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

## 4. Behavior Covered

The new behavior tests cover the benchmark loop:

```text
dogfood validation
-> governed run start
-> planning approval pause
-> approval grant
-> explicit docs-check handler execution
-> completed sequential dogfood workflow
-> report-bearing rehydration
-> stable citation assertions
-> no automatic artifact writes
```

This exercises existing local kernel primitives without adding smoother CLI paths or ambient automation.

## 5. Validation Boundary Summary

The first test validates the dogfood project with the existing project loader/validator before execution. It does not shell out to the CLI for validation and does not claim manual commands are kernel-executed checks.

The docs-check step uses the already implemented explicit handler with an injected process runner. It does not register a default handler, add CLI flags, or execute arbitrary shell commands.

## 6. Report And Citation Summary

The second test uses existing `execute_with_report(...)` APIs and supplies stable references explicitly.

It verifies report citations for:

- local check result reference;
- typed handoff ID;
- agent harness hook invocation ID.

The test does not create `EvidenceReference` values, local check result references, typed handoffs, or hook invocations implicitly.

## 7. Privacy And Redaction Summary

The tests use bounded, non-secret values. They do not copy raw command output, provider payloads, raw spec contents, parser payloads, environment values, credentials, token-like values, or unbounded model self-review into reports.

The injected docs-check output is bounded test data and is not attached as command-output evidence.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test local_executor self_governed_build_benchmark`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Known Limitations

- Benchmark behavior coverage is still focused on workflow-core APIs, not CLI behavior.
- CLI report-bearing dogfood execution remains unimplemented.
- Explicit dogfood DocsCheck registration remains API/test-oriented rather than normal CLI workflow.
- Local check result references are supplied explicitly to reports; automatic propagation remains deferred.
- Typed handoff integration into dogfood remains supplied-reference only.
- Hook disclosures remain planned but unimplemented.
- Reasoning lineage remains future architecture.

## 10. Recommended Next Phase

Recommended next phase: **self-governed build benchmark behavior test review**.

That review should verify the test coverage proves the runbook loop through existing explicit APIs without overclaiming CLI behavior, automatic checks, report artifact writes, recursive agents, writes, hosted behavior, or production self-hosting.
