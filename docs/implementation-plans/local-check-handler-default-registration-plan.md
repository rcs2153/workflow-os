# Local Check Handler Default-Registration Plan

Status: Implemented. An explicit non-default local check registration profile/helper is implemented. It supports `None` and caller-supplied `DocsCheck` registration only, keeps `LocalSkillRegistry::new()` default-safe, and does not add CLI exposure, workflow schema fields, automatic check execution, true ambient default registration, command-output evidence, side-effect modeling, writes, or release posture changes.

## 1. Executive Summary

Workflow OS has a production-shaped `DocsCheckLocalHandler`, an explicit non-default registry helper, local check result references, WorkReport local check citation vocabulary, and terminal report helper integration for supplied local check references.

The self-governance dogfood workflow is now multi-step, cleaned up, and hardened around cancellation, duplicate run-id rehydration, and explicit report-bearing execution.

The next question is whether any local check handler should become available by default. This plan recommends **not enabling ambient default registration yet**. Instead, it implements an explicit, opt-in registration profile/helper boundary that can be tested without executing real commands by default.

This plan does not implement default registration, CLI exposure, workflow schema fields, automatic check execution, command-output evidence, side-effect modeling, writes, or release posture changes.

## 2. Goals

- Decide the next safe step toward default local check handler registration.
- Preserve explicit local command authority.
- Keep `LocalSkillRegistry::new()` default-safe.
- Avoid ambient real command execution.
- Keep `DocsCheckLocalHandler` as the only candidate for any future default-registration lane.
- Define a narrow opt-in registration boundary that can be tested without CLI or schema changes.
- Preserve existing local executor semantics and event behavior.
- Preserve redaction-safe local check result behavior.
- Keep report and evidence integration reference-oriented and separate.
- Identify prerequisites before any true default registration is implemented.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- ambient default registration in `LocalSkillRegistry::new()`;
- automatic local check execution;
- CLI flags or commands for local check execution;
- workflow schema fields for local check registration;
- `AllowlistedHandlerOnly` enablement;
- broad handler discovery;
- arbitrary shell execution;
- user-supplied command text;
- cargo, npm, TypeScript, contract, integration, or live-provider handler registration beyond `DocsCheck`;
- command-output evidence attachment;
- local check evidence attachment;
- automatic work-report generation;
- automatic report artifact writing;
- report CLI rendering;
- persistence changes;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- hosted or distributed runtime behavior;
- recursive agents;
- agent swarms;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented:

- local check command contract model;
- canonical command-template binding;
- test-only `WorkflowOsValidateDogfood` handler;
- local check result model;
- injectable local check process-runner boundary;
- production-shaped explicit `DocsCheckLocalHandler`;
- explicit non-default `LocalSkillRegistry::register_docs_check_handler(...)`;
- local check result reference model;
- WorkReport local check result citation target vocabulary;
- terminal report helper integration for supplied local check result references;
- dogfood multi-step workflow hardening tests.

Not implemented:

- true default registration;
- automatic local check execution;
- CLI check execution;
- workflow schema declaration for local checks;
- default executable/toolchain discovery;
- side-effect/cache/write sandbox;
- command-output evidence;
- local check evidence attachment;
- dogfood workflow use of real `DocsCheck`;
- report artifact generation from local check results.

## 5. Default Registration Decision

True ambient default registration should remain deferred.

Do not register `DocsCheckLocalHandler` automatically from `LocalSkillRegistry::new()`.

Reasons:

- default registration changes the authority profile of ordinary local executor construction;
- `DocsCheckLocalHandler` still requires explicit executable, working-directory, cache, and process-runner decisions;
- npm execution may write cache/build artifacts even when source files are unchanged;
- the side-effect boundary model is not implemented;
- CLI and workflow schema behavior for local check execution is not designed;
- command-output evidence policy remains planning-only;
- report artifact behavior remains separate and explicit;
- dogfood currently proves governance checkpoints, not real command execution.

## 6. Recommended Next Implementation Boundary

Implemented boundary: **explicit local check registration profile model/helper, non-default and non-CLI**.

The implementation adds the smallest explicit helper that represents a registration posture without enabling ambient execution.

Candidate shape:

- `LocalCheckRegistrationProfile`
- `LocalCheckRegistrationMode`
- `LocalCheckRegistrationOptions`
- `LocalSkillRegistry::register_local_check_profile(...)`

The implementation may choose different names if repository conventions suggest a better fit.

The helper:

- requires explicit caller input;
- supports only `DocsCheck` in the first implementation;
- requires a prebuilt `DocsCheckLocalHandler`;
- avoids reading ambient environment;
- avoids searching `PATH`;
- avoids constructing npm paths or cache directories implicitly;
- avoids registering dogfood validation, cargo, TypeScript, contract, integration, or provider checks;
- keeps `LocalSkillRegistry::new()` empty/default-safe;
- exposes inspection of what would be registered without executing commands;
- remains testable with injected runners and without real npm execution.

## 7. Registration Modes

Recommended representable modes:

- `None`
  - Register no local check handlers.
  - This should remain equivalent to current default behavior.

- `ExplicitDocsCheck`
  - Register a caller-supplied `DocsCheckLocalHandler`.
  - This is a named version of today’s explicit helper posture.

Rejected for now:

- `All`
- `FromWorkflowSpec`
- `FromEnvironment`
- `FromPath`
- `AllowlistedHandlerOnly`
- `CargoWorkspace`
- `NpmScripts`
- `ArbitraryCommand`

These modes either require schema/CLI/security work or would broaden local command authority too early.

## 8. Authorization And Governance Rules

The registration profile must preserve these rules:

- execution authority is granted only by explicit Rust construction/registration;
- serialized workflow specs cannot activate local command execution;
- local check contracts remain model-only unless an explicit handler is registered;
- `AllowlistedHandlerOnly` remains invalid unless separately designed;
- missing handlers keep failing closed with stable non-leaking errors;
- registered handlers still execute only canonical command templates;
- policy decisions still occur before skill invocation;
- approval behavior remains unchanged.

## 9. Runtime And Event Boundary

Allowed:

- explicit helper registration into a caller-provided `LocalSkillRegistry`;
- normal executor event flow if a workflow invokes a registered handler;
- existing local check result mapping;
- injected-runner tests.

Not allowed:

- new runtime event kinds;
- automatic check execution;
- post-terminal events;
- report artifact writes;
- StateBackend writes outside normal executor events;
- observability or audit noise beyond existing executor behavior;
- CLI output;
- workflow schema changes.

## 10. Environment, Toolchain, And Cache Boundary

The default-registration profile must not invent toolchain policy.

It must not:

- resolve npm executable paths;
- infer repository root;
- create cache directories;
- read ambient environment variables;
- pass through credentials;
- invoke shell commands;
- call external systems.

Any future true default registration must first define:

- executable provenance;
- working-directory policy;
- cache/write sandbox policy;
- network posture;
- timeout posture;
- output capture posture;
- failure behavior;
- documentation and operator expectations.

## 11. Privacy And Redaction

The registration profile must not store or copy:

- raw command output;
- docs contents;
- parser payloads;
- provider payloads;
- environment values;
- npm tokens;
- registry credentials;
- authorization headers;
- private keys;
- token-like strings;
- unbounded paths or command arguments.

Debug output must not reveal executable paths, cache paths, command arguments, environment names, or user-provided payloads. Errors must use stable codes and avoid leaking local paths or secret-like values.

## 12. Test Coverage

Implemented tests cover:

- `LocalSkillRegistry::new()` remains default-safe and empty of local check handlers;
- `None` registration mode registers no handlers;
- `ExplicitDocsCheck` registers only `local/check-docs` `v0`;
- helper requires explicit caller input;
- helper does not construct npm paths or cache directories;
- helper does not read ambient environment;
- explicit profile registration can execute through `LocalExecutor` with injected runner;
- generated process request still uses canonical `npm run check:docs`;
- missing default handler behavior remains unchanged;
- no report artifacts are written;
- no source files are written;
- no CLI output is emitted;
- no `AllowlistedHandlerOnly` behavior changes;
- no schema files change;
- Debug/errors are redaction-safe;
- existing local check, executor, report, evidence, validation, adapter, dogfood, and runtime tests still pass.

Duplicate registration semantics remain inherited from existing registry behavior and should be reviewed separately before any true default-registration posture is introduced.

## 13. Documentation Updates

Updated:

- `docs/implementation-plans/local-check-handler-default-registration-plan.md`
- `docs/implementation-plans/docs-check-default-registration-plan.md`
- `docs/implementation-plans/broader-local-check-handler-plan.md`
- `ROADMAP.md`

Docs must say:

- an explicit local check registration profile/helper is implemented, if implemented;
- true ambient default registration is not implemented;
- `LocalSkillRegistry::new()` remains default-safe;
- CLI exposure is not implemented;
- workflow schema exposure is not implemented;
- automatic local check execution is not implemented;
- command-output evidence is not implemented;
- side-effect boundary and writes remain unsupported.

## 14. End-Of-Phase Report

Created:

- `docs/concepts/LOCAL_CHECK_HANDLER_DEFAULT_REGISTRATION_REPORT.md`

The report must include:

1. executive summary;
2. scope completed;
3. scope explicitly not completed;
4. registration model/helper summary;
5. default-safe behavior summary;
6. authorization boundary summary;
7. runtime/event boundary summary;
8. privacy/redaction summary;
9. test coverage summary;
10. commands run and results;
11. remaining known limitations;
12. recommended next phase.

Recommended next phase should be one of:

- local check handler default-registration review;
- dogfood real DocsCheck planning;
- local check side-effect boundary planning;
- command-output evidence planning;
- blocker fix;
- defer.

## 15. Validation

Run:

- targeted local check/default registration tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

## 16. Dogfood Governance For This Planning Phase

This planning phase was governed by the self-governance dogfood workflow before documentation edits:

- `workflow-os --project-dir dogfood/workflow-os-self-governance validate` passed with expected experimental lifecycle warnings;
- `workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-default-registration-plan-state-20260616 --mock-all-local-skills run dg/d` paused at `planning-approved`;
- approval was granted with bounded reason `local-check-default-registration-planning`;
- `inspect` showed the run completed with five checkpoint invocations and 34 durable events.

## 17. Final Recommendation

The recommended next phase is: local check handler default-registration review.

The implemented helper makes the registration posture explicit and testable while preserving the current safety boundary. The review should verify that it did not add true default registration, CLI exposure, workflow schema changes, automatic check execution, command-output evidence, side-effect boundary implementation, writes, or release posture changes.
