# Dogfood Real DocsCheck Plan

Status: Implemented. The self-governance dogfood workflow now includes an explicit `local/check-docs` checkpoint, and focused tests prove it can run through a caller-supplied `DocsCheckLocalHandler` using `LocalCheckRegistrationProfile::explicit_docs_check(...)` with an injected runner. Local check side-effect/cache/write boundary planning is documented in [Local Check Side-Effect Boundary Plan](local-check-side-effect-boundary-plan.md). This does not implement default handler registration, CLI exposure, workflow schema fields, automatic check execution, command-output evidence, side-effect boundary modeling, writes, or release posture changes.

## 1. Executive Summary

Workflow OS now has a production-shaped explicit `DocsCheckLocalHandler`, an explicit non-default `DocsCheck` registry helper, and an explicit local check registration profile/helper.

The next question is whether the self-governance dogfood workflow should exercise the real `DocsCheck` handler through explicit registration. This plan recommends a bounded dogfood implementation that proves the kernel can govern a real docs check while preserving the current authority boundary:

- the caller constructs `DocsCheckLocalHandler` explicitly;
- the caller registers it through `LocalCheckRegistrationProfile::explicit_docs_check(...)`;
- the self-governance workflow invokes the existing `local/check-docs` skill;
- results remain bounded local check results, not raw command transcripts.

This plan does not implement runtime behavior. It does not add true default registration, CLI exposure, workflow schema fields, automatic check execution, command-output evidence, persistence, report artifact auto-writing, side-effect modeling, writes, examples, hosted execution, or release posture changes.

## 2. Goals

- Dogfood the real `DocsCheckLocalHandler` through the kernel.
- Preserve explicit local command authority.
- Keep `LocalSkillRegistry::new()` default-safe.
- Use the existing explicit registration profile/helper.
- Exercise the canonical `npm run check:docs` command template only.
- Keep the command non-shell, bounded, and testable.
- Use explicit npm executable, repository root, cache path, and process-runner inputs.
- Preserve existing executor, policy, approval, event, and report behavior.
- Capture only bounded local check result summaries.
- Avoid raw command output, raw docs contents, parser payloads, environment values, and secrets.
- Produce an implementation path that is narrow enough for review.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- true ambient default registration;
- registering `DocsCheckLocalHandler` from `LocalSkillRegistry::new()`;
- automatic local check execution;
- CLI flags or commands for real local checks;
- workflow schema fields for check registration;
- workflow-declared local check handlers;
- runtime config for local check handlers;
- `AllowlistedHandlerOnly` enablement;
- broad handler discovery;
- arbitrary shell execution;
- user-supplied command text;
- cargo, TypeScript, contract, integration, or live-provider check handlers;
- command-output evidence attachment;
- local check evidence attachment;
- raw command transcript storage;
- automatic report artifact writing;
- persistence changes;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- recursive agents;
- agent swarms;
- hosted or distributed runtime behavior;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented:

- local validation/check command contract model;
- canonical command-template binding;
- structured `LocalCheckResult`;
- injectable `LocalCheckProcessRunner`;
- explicit production-shaped `DocsCheckLocalHandler`;
- explicit `LocalSkillRegistry::register_docs_check_handler(...)`;
- explicit `LocalCheckRegistrationProfile::explicit_docs_check(...)`;
- local check result reference model;
- WorkReport local check citation vocabulary;
- terminal report helper support for supplied local check result references;
- multi-step self-governance dogfood workflow;
- report-bearing local execution helper paths.

Implemented in this phase:

- dogfood workflow execution with the real `DocsCheckLocalHandler` through explicit profile registration and injected-runner tests;

Still not implemented:

- true default handler registration;
- CLI exposure for local checks;
- workflow schema fields for local check registration;
- side-effect/cache/write sandbox policy;
- command-output evidence;
- local check evidence attachment;
- report artifact auto-writing from check execution.

## 5. Dogfood Target

The first dogfood target should be the self-governance workflow invoking `local/check-docs` with a real `DocsCheckLocalHandler` supplied by the caller.

The target is appropriate because:

- `DocsCheck` is narrow and project-owned;
- it already maps to a canonical command template: `npm run check:docs`;
- it has a production-shaped explicit handler;
- it exercises a real local command without broadening to cargo, tests, integration checks, or live providers;
- docs check output can be represented as bounded local check result summaries;
- the command can run under explicit npm executable, repository root, and npm cache path inputs.

The target remains risky enough to require explicit setup because npm may touch cache state. That is why this phase must not make it default, schema-driven, CLI-driven, or automatic.

## 6. Handler And Registration Boundary

Future implementation should use:

- `LocalCheckCommandContract::docs_check_model_only()`;
- explicit `DocsCheckLocalHandler::new(...)` or `new_with_process_runner(...)`;
- explicit npm executable path;
- explicit repository root;
- explicit npm cache directory, preferably outside repository source;
- `LocalCheckRegistrationProfile::explicit_docs_check(handler)`;
- `LocalSkillRegistry::register_local_check_profile(...)`;
- existing `LocalExecutor` or report-bearing local executor API.

The implementation must not:

- search `PATH`;
- infer npm executable location;
- infer repository root from process cwd;
- create cache policy implicitly;
- read ambient environment variables;
- pass through credentials;
- construct shell commands;
- register broader command families;
- make the handler available from default registry construction.

## 7. Workflow Integration Boundary

The future dogfood implementation should use a workflow that already declares or can safely declare a `local/check-docs` step.

Allowed:

- a test-only or dogfood-only explicit Rust setup that registers the handler before running the self-governance workflow;
- normal local executor skill invocation;
- normal policy and approval behavior;
- bounded local check result mapping;
- report-bearing result paths when already explicitly requested by the caller.

Not allowed:

- automatic local check registration from the workflow spec;
- workflow schema fields that declare real handler activation;
- CLI flags that turn on real local checks;
- post-terminal event appends outside existing executor behavior;
- report artifact writes;
- out-of-band state writes;
- observability or audit events outside existing executor paths.

## 8. Execution And Side-Effect Posture

The future implementation must document and test that `DocsCheck` is classified as `NoSourceWrites`, not no-writes.

Rules:

- repository source files must not be modified;
- npm cache writes may occur only in an explicitly supplied cache directory;
- no network access should be required by the docs check;
- no credentials should be supplied;
- no environment pass-through should occur;
- no cleanup of user files should be attempted;
- any cache behavior must be disclosed in the implementation report.

This plan does not implement the general side-effect boundary model. If future work needs broader cache/write classification, that must be handled by a separate side-effect boundary phase. The first planning boundary for local checks is [Local Check Side-Effect Boundary Plan](local-check-side-effect-boundary-plan.md).

## 9. Result, Evidence, And Report Boundary

The dogfood run should produce a bounded `LocalCheckResult` through existing handler behavior.

Allowed:

- bounded stdout/stderr summaries inside `LocalCheckResult`;
- local check status and stable error code fields;
- local check result references if the implementation creates them through existing models;
- WorkReport citations to supplied local check result references when report-bearing execution is explicitly requested.

Not allowed:

- raw command output evidence;
- raw command transcripts;
- copying raw docs contents into reports;
- local check evidence attachment;
- automatic report artifact writing;
- fabricated missing citations;
- report text that claims evidence was attached when only bounded check output exists.

## 10. Privacy And Redaction

The implementation must not store or copy:

- raw command output;
- raw docs contents;
- parser payloads;
- provider payloads;
- environment values;
- npm tokens;
- registry credentials;
- authorization headers;
- private keys;
- token-like strings;
- unbounded local paths;
- command arguments beyond canonical planned metadata where already documented.

Errors must use stable codes and avoid raw paths, snippets, output, environment names, and secret-like values. Debug output for any new setup/helper type must redact executable paths, repository paths, cache paths, command arguments, runner details, and user-provided values.

## 11. Error Handling

Future implementation should fail closed when:

- the npm executable path is invalid;
- the repository root is invalid;
- the cache path is invalid under existing handler rules;
- the docs check handler receives the wrong command kind;
- the process runner fails;
- process output contains secret-like values;
- local check result construction fails;
- a non-canonical argument template is attempted.

Failures must not become misleading project diagnostics. They should remain structured `WorkflowOsError` values with stable non-leaking codes.

## 12. Test Plan

Future implementation should add focused tests for:

1. self-governance dogfood can invoke real `DocsCheck` through explicit profile registration with an injected runner;
2. the default registry still fails closed for `local/check-docs`;
3. `LocalSkillRegistry::new()` remains default-safe;
4. the explicit profile registers only `local/check-docs` `v0`;
5. the generated process request uses canonical `npm run check:docs` arguments;
6. the process request uses explicit npm executable and repository root inputs;
7. the process request includes only sanitized minimal environment values;
8. cache path behavior is explicit and does not write source files in tests;
9. successful docs check output becomes a bounded `LocalCheckResult`;
10. failed docs check output becomes a bounded failure result or structured non-leaking error according to existing handler behavior;
11. secret-like stdout/stderr fails closed without leakage;
12. handler construction errors do not leak local paths or cache names;
13. no post-terminal events are appended beyond existing executor behavior;
14. no report artifacts are written automatically;
15. no CLI output is introduced;
16. existing local check, local executor, report, evidence, validation, adapter telemetry, dogfood, and runtime tests still pass.

If a real npm smoke test is added, it must be explicitly opt-in or tightly scoped to the local development environment and must not become a CI requirement without a separate review.

## 13. Proposed Implementation Sequence

Recommended small phases:

1. Add a dogfood-only test/helper that assembles an explicit `DocsCheckLocalHandler` with injected runner and registration profile.
2. Exercise the self-governance workflow path that invokes `local/check-docs`.
3. Add tests for default-safe behavior, canonical process request construction, bounded result mapping, and no artifact writes.
4. Review the dogfood real DocsCheck phase.
5. Only after review, consider whether a tightly bounded real npm smoke test belongs in local validation.
6. Keep true default registration, CLI exposure, workflow schema fields, command-output evidence, side-effect boundary implementation, and writes deferred.

## 14. Open Questions

- Should the first implementation use only injected-runner tests, or include an opt-in real npm smoke test?
- Should the dogfood workflow add a dedicated docs-check step, or reuse an existing validation/check step if present?
- Should local check result references be generated by the dogfood test path, or remain supplied manually until report exposure is expanded?
- What exact npm cache directory should local dogfood instructions recommend?
- Should a future CLI setup command print the explicit Rust/API boundary, or should CLI remain entirely out of scope?
- When should `AllowlistedHandlerOnly` become a valid execution posture, if ever?
- What side-effect boundary is sufficient before cargo/clippy/test handlers are considered?

## 15. Documentation Updates For Future Implementation

Future implementation should update:

- [Broader Local Check Handler Plan](broader-local-check-handler-plan.md);
- [Local Check Handler Default-Registration Plan](local-check-handler-default-registration-plan.md);
- [Self-Governed Validation/Check Plan](self-governed-validation-check-plan.md);
- [Roadmap](../../ROADMAP.md);
- an end-of-phase report under `docs/concepts/`.

Docs must continue to state:

- real DocsCheck dogfood is explicit, not automatic;
- true default registration is not implemented;
- CLI exposure is not implemented;
- workflow schema exposure is not implemented;
- command-output evidence is not implemented;
- local check evidence attachment is not implemented;
- report artifacts are not automatically written;
- side-effect boundary and writes remain unsupported.

## 16. Final Recommendation

The recommended next implementation phase is: **dogfood real DocsCheck implementation, explicit-profile and injected-runner first**.

The phase should prove that the self-governance workflow can use the real `DocsCheckLocalHandler` through explicit local registration while preserving the existing safe-by-default posture. It must still not implement ambient default registration, CLI exposure, workflow schema changes, automatic check execution, command-output evidence, side-effect boundary modeling, writes, or release posture changes.
