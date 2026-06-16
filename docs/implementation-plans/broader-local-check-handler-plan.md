# Broader Local Check Handler Plan

Status: Local check result model, injectable process-runner infrastructure, explicit production-shaped/non-default `DocsCheckLocalHandler`, an explicit non-default DocsCheck registry helper, an explicit non-default local check registration profile/helper, and a local check result reference model are implemented. `DocsCheck` production-posture planning is documented in [DocsCheck Local Handler Production-Posture Plan](docs-check-production-posture-plan.md), default-registration planning is documented in [DocsCheck Default-Registration Plan](docs-check-default-registration-plan.md) and [Local Check Handler Default-Registration Plan](local-check-handler-default-registration-plan.md), local check result citation planning is documented in [Local Check Result Citation Plan](local-check-result-citation-plan.md), and WorkReport local check result citation target planning is documented in [WorkReport Local Check Result Citation Target Plan](work-report-local-check-citation-target-plan.md). No true ambient default command handler registration, evidence attachment, or command-output evidence is authorized by this document.

## 1. Executive Summary

Workflow OS now has a local validation/check command contract model, canonical command-template binding, and a test-only `WorkflowOsValidateDogfood` handler. The test-only handler proved that a local check can run through the existing `SkillHandler` and `LocalExecutor` boundary without shell invocation, default registration, CLI exposure, workflow schema changes, report artifacts, or source writes.

The next question is how to safely broaden local check handling beyond the single dogfood validation command and how to make local check results citeable without storing raw command output.

This plan recommended reusable local check handler infrastructure only:

- a local check result model;
- an injectable process runner abstraction for deterministic tests;
- shared bounded output capture and redaction behavior;
- handler boundary tests for failure, timeout, secret-like output, and environment sanitization.

That infrastructure is implemented and documented in [Local Check Handler Infrastructure Report](../concepts/LOCAL_CHECK_HANDLER_INFRASTRUCTURE_REPORT.md). The infrastructure blocker fix is reviewed in [Local Check Handler Infrastructure Blocker Fix Review](../concepts/LOCAL_CHECK_HANDLER_INFRASTRUCTURE_BLOCKER_FIX_REVIEW.md). The first non-dogfood explicit handler is documented in [DocsCheck Local Handler Plan](docs-check-local-handler-plan.md), [DocsCheck Local Handler Report](../concepts/DOCS_CHECK_LOCAL_HANDLER_REPORT.md), and [DocsCheck Local Handler Review](../concepts/DOCS_CHECK_LOCAL_HANDLER_REVIEW.md). Production-posture decisions and the explicit production-shaped handler are documented separately in [DocsCheck Local Handler Production-Posture Plan](docs-check-production-posture-plan.md). Default-registration planning is documented in [DocsCheck Default-Registration Plan](docs-check-default-registration-plan.md), the explicit registry helper implementation is documented in [DocsCheck Registry Helper Report](../concepts/DOCS_CHECK_REGISTRY_HELPER_REPORT.md), and local check result citation/reference work is documented in [Local Check Result Citation Plan](local-check-result-citation-plan.md). These phases do not add default handler registration, CLI exposure, workflow schema fields, automatic check execution, report artifact writing, WorkReport local check result citation wiring, evidence attachment, command-output evidence, side-effect boundary modeling, writes, recursive agents, agent swarms, hosted execution, or release posture changes.

## 2. Goals

- Preserve the kernel-governed, Codex-executed dogfood posture.
- Keep local check execution explicit, allowlisted, and non-shell.
- Introduce reusable handler infrastructure before adding more command kinds.
- Make handler behavior deterministic and testable without invoking real toolchains in every unit test.
- Represent local check outcomes as validated structured results instead of ad hoc skill-output maps.
- Capture bounded, redaction-safe output summaries only.
- Preserve existing local executor semantics and event ordering.
- Prepare future work report and evidence integration without implementing it.
- Keep production registration and CLI exposure deferred until handler infrastructure is reviewed.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- production local command execution;
- default handler registration;
- broad handler discovery;
- arbitrary shell-command skills;
- user-supplied command text;
- generic build-command execution;
- CLI handler exposure;
- workflow schema changes;
- automatic check execution;
- automatic Codex control through the kernel;
- automatic runtime report generation;
- automatic report artifact writing;
- evidence attachment;
- command-output evidence policy;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- provider calls or live adapter execution;
- recursive agents;
- agent swarms;
- hosted or distributed runtime behavior;
- Level 3 or Level 4 autonomy;
- DLP or access-control systems;
- release posture changes.

## 4. Current Foundation

Implemented:

- self-governance dogfood project;
- local project validation;
- local executor with explicit skill handler registry;
- approval-gated execution;
- durable local event history;
- policy decisions and audit events;
- work report contract and report models;
- in-memory terminal local report helper;
- executor-integrated report-bearing local execution path;
- explicit local report artifact store;
- local check command contract model;
- canonical command-template binding for current local check command kinds;
- test-only `WorkflowOsValidateDogfood` handler;
- test-only handler review.

Remaining limitations:

- no production local check handlers;
- no default handler registration;
- no broader command handlers beyond the test-only dogfood validation handler;
- no real sandbox;
- no side-effect boundary model;
- no CLI exposure for allowlisted check handlers;
- no workflow schema fields for checks;
- no automatic work-report generation from check results;
- no local check evidence attachment.

## 5. Review Findings Driving This Plan

The test-only handler review found no blockers and recommended broader local check handler planning.

Non-blocking follow-ups from that review:

- Add an injectable process-runner boundary before expanding beyond `WorkflowOsValidateDogfood`.
- Add deterministic tests for failed validation, timeout behavior, secret-like stdout/stderr rejection, and environment sanitization.
- Tighten documentation wording around real handlers versus test-only handlers.
- Decide whether a local check result type should replace skill-output map fields before report/evidence integration.
- Keep additional command kinds deferred until side-effect policy, output policy, and sandbox posture are reviewed.

This plan treats those follow-ups as prerequisites for broadening command coverage.

## 6. Candidate Command Families

| Command family | Current model kind | Recommended near-term status | Rationale |
| --- | --- | --- | --- |
| Dogfood validation | `WorkflowOsValidateDogfood` | Keep as current test-only proof path | Already implemented for explicit tests. Good regression target for shared infrastructure. |
| Docs check | `DocsCheck` | Explicit production-shaped handler implemented; default registration deferred | Narrow project-owned command. Implementation uses explicit construction and injected-runner tests; default registration and CLI remain deferred. |
| Cargo fmt check | `CargoFmtCheck` | Defer until cache/toolchain side-effect policy is explicit | Usually no source writes with `--check`, but toolchain/cache behavior should be declared. |
| Cargo clippy | `CargoClippyWorkspace` | Defer until build/cache side-effect policy is explicit | Writes build artifacts, has large output, and can be slow. |
| Cargo test | `CargoTestWorkspace` | Defer until test side-effect policy is explicit | Executes arbitrary tests and writes build artifacts. |
| TypeScript check | `TypeScriptCheck` | Defer | Depends on Node/npm toolchain and cache posture. |
| Contract check | `ContractCheck` | Defer | Should follow docs/check infrastructure and schema compatibility planning. |
| Integration check | `IntegrationCheck` | Defer | Broader integration posture and output rules needed. |
| Live provider smoke tests | None | Reject for local check v1 | Requires credentials/network and belongs outside local check handler expansion. |
| Arbitrary user commands | None | Reject | Would turn Workflow OS into a shell runner. |

## 7. Recommended Next Implementation Target

Implemented infrastructure phase: **local check handler infrastructure only**.

That phase should:

1. Add a structured `LocalCheckResult` model.
2. Add an injectable process runner abstraction used by the test-only handler.
3. Move bounded output capture/redaction into shared local check helpers.
4. Add deterministic tests for failed exit, timeout, secret-like output, and environment construction.
5. Keep `WorkflowOsValidateDogfood` as the only executable command kind.
6. Preserve explicit test-only registration.

The phase did not add `DocsCheck`, `CargoFmtCheck`, `CargoClippyWorkspace`, `CargoTestWorkspace`, TypeScript, contract, or integration command handlers.

## 8. Local Check Result Model

The implementation introduced a small validated result model without widening runtime behavior.

Candidate type:

- `LocalCheckResult`

Candidate fields:

- check command ID;
- command kind;
- result status;
- exit code if available;
- duration in milliseconds;
- bounded stdout summary;
- bounded stderr summary;
- stdout truncated flag;
- stderr truncated flag;
- stable error code for internal failures if available.

Rules:

- Validate all fields at construction.
- Keep output summaries bounded.
- Reject secret-like summaries before storage.
- Do not store full raw output.
- Do not store full command transcript.
- Do not store environment values.
- Do not store provider payloads, parser payloads, raw spec contents, tokens, credentials, or authorization headers.
- Implement redaction-safe `Debug`.
- Support serde only if the repository pattern requires it; if serde is added, invalid serialized results must fail closed without leaking values.

The existing `SkillOutput` may continue carrying summarized values in tests, but it should be derived from `LocalCheckResult` rather than duplicating validation rules.

## 9. Process Runner Boundary

The implementation introduced an injectable process-runner boundary so behavior tests can avoid baking in direct process execution.

Candidate trait or internal abstraction:

- `LocalCheckProcessRunner`
- `LocalCheckProcessRequest`
- `LocalCheckProcessOutput`

The abstraction should allow tests to deterministically simulate:

- successful exit;
- non-zero exit;
- timeout;
- process-spawn failure;
- secret-like stdout;
- secret-like stderr;
- oversized stdout/stderr;
- missing exit code;
- environment construction.

Rules:

- The production runner, if retained, must still use executable plus argument vector.
- No shell invocation.
- No command string concatenation.
- No caller-supplied extra arguments.
- Explicit working directory.
- Sanitized environment.
- Bounded timeout.
- Bounded output capture.
- Stable non-leaking errors.

The abstraction should remain internal unless a public API is clearly justified by tests or handler construction.

## 10. Handler Registry And Authorization Posture

The next implementation must keep handler registration explicit.

Allowed:

- tests may construct and register the handler explicitly;
- the test-only dogfood handler may use the shared process runner/result model;
- the contract may remain `ModelOnly` while execution authority lives in the explicit test-only handler type.

Not allowed:

- default registration;
- CLI flags that enable real handlers;
- broad handler discovery;
- workflow schema fields;
- ambient replacement of `--mock-all-local-skills`;
- automatic execution after Codex tasks;
- production registration for `DocsCheck`, cargo, npm, or integration checks.

Before production registration is considered, a separate phase must decide whether `AllowlistedHandlerOnly` should become valid for a narrow subset of command kinds or whether execution authorization should stay outside serialized contracts.

## 11. Command Authority Rules

All future handler work must preserve these rules:

- typed command kind, never arbitrary shell text;
- canonical executable and argument template;
- no shell invocation;
- no pipes, redirection, glob expansion, command substitution, or command concatenation;
- no caller-supplied extra arguments;
- explicit repository-root or dogfood-root working directory policy;
- sanitized minimal environment by default;
- no provider credentials;
- disabled network policy unless a later phase explicitly designs otherwise;
- bounded timeout;
- bounded output summaries;
- stable non-leaking errors;
- no source writes unless separately authorized through a side-effect boundary.

## 12. Environment Policy

Environment should stay minimal and explicit.

Implemented infrastructure behavior:

- build a local check environment map through one helper;
- start from an empty environment;
- add only non-secret required variables;
- prefer explicit executable paths passed by tests for dogfood validation;
- never pass through tokens, credentials, authorization headers, provider keys, private keys, secret-like variable names, or unbounded caller environment.

The first infrastructure phase should test environment construction without relying on platform-specific process inspection where possible.

## 13. Output Capture And Redaction Policy

Output remains sensitive by default.

Rules:

- Capture bounded stdout and stderr summaries only.
- Mark truncation explicitly.
- Reject or redact secret-like output before constructing `LocalCheckResult`.
- Do not persist raw output.
- Do not store command transcripts.
- Do not copy raw spec contents, parser payloads, provider payloads, environment values, tokens, credentials, CI logs, or private keys.
- Use stable error code `local_check.output.secret_like` or a more specific stable code if introduced.
- Do not attach `EvidenceReference` for command output until command-output evidence policy is reviewed.

## 14. Failure Semantics

Recommended status mapping:

- process success maps to `LocalCheckResultStatus::Passed`;
- non-zero exit maps to `LocalCheckResultStatus::Failed`;
- timeout maps to `LocalCheckResultStatus::TimedOut`;
- process spawn/wait failure maps to `LocalCheckResultStatus::InternalError` or a structured internal error;
- unsupported contract maps to a validation error;
- secret-like output maps to `LocalCheckResultStatus::RedactionFailed` or a structured redaction error.

For the next infrastructure phase, result construction failure should fail the explicitly invoked skill step. It must not alter global executor semantics, append post-terminal events, or become automatic workflow behavior.

## 15. Runtime, Event, And Report Boundary

The next implementation must stay inside existing local executor skill mechanics.

Allowed:

- existing skill invocation events produced by an explicitly run workflow;
- bounded check result summary in `SkillOutput`;
- tests verifying returned run events match persisted backend events.

Not allowed:

- new runtime event kinds;
- post-terminal event appends;
- automatic check events outside explicit workflow runs;
- automatic report generation;
- automatic report artifact writing;
- evidence attachment;
- CLI report rendering;
- workflow schema changes.

Future work report integration should cite stable check-result references only after a result reference model is reviewed.

## 16. Side-Effect Posture

The next implementation should not broaden side effects.

Rules:

- `WorkflowOsValidateDogfood` remains classified as `NoSourceWrites`.
- Cargo/npm commands remain deferred until build/cache write behavior is explicitly modeled.
- No source writes are authorized.
- No local state writes are authorized beyond normal explicit workflow event history in tests.
- No report artifacts are authorized.
- No cleanup semantics are introduced.

Before cargo or npm checks are implemented, each command must declare permitted output/cache directories and expected write behavior.

## 17. Test Plan For Next Implementation

Future infrastructure implementation tests should cover:

- `LocalCheckResult` accepts a valid passed result.
- `LocalCheckResult` accepts a valid failed result.
- `LocalCheckResult` rejects unbounded stdout summary.
- `LocalCheckResult` rejects unbounded stderr summary.
- `LocalCheckResult` rejects secret-like stdout summary without leaking it.
- `LocalCheckResult` rejects secret-like stderr summary without leaking it.
- `LocalCheckResult` Debug output does not leak summaries.
- serde round trip for valid result if serde is implemented.
- invalid serialized result fails closed without leaking values if serde is implemented.
- injectable process runner maps zero exit to passed.
- injectable process runner maps non-zero exit to failed.
- injectable process runner maps timeout to timed out or a stable timeout error.
- injectable process runner maps spawn failure to a stable non-leaking error.
- environment construction starts from minimal explicit values.
- secret-like environment variable names are rejected.
- test-only dogfood handler still requires explicit registration.
- test-only dogfood handler still rejects unsupported command kinds.
- test-only dogfood handler still invokes no shell.
- test-only dogfood handler still writes no report artifacts.
- local executor event ordering remains unchanged.
- existing local check, local executor, work report, evidence, diagnostic, adapter telemetry, and runtime tests still pass.

## 18. Documentation Updates For Next Implementation

The next implementation should update:

- this plan;
- [Self-Governed Validation/Check Plan](self-governed-validation-check-plan.md);
- [Test-Only Local Check Handler Plan](test-only-local-check-handler-plan.md);
- [Roadmap](../../ROADMAP.md), if roadmap status changes;
- an end-of-phase report under `docs/concepts/`.

Docs must continue to state:

- production local check handlers are not implemented;
- CLI exposure is not implemented;
- workflow schema fields are not implemented;
- automatic check execution is not implemented;
- report artifacts are not automatically written;
- evidence attachment is not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported;
- recursive agents, agent swarms, hosted execution, and production self-hosting are not implemented.

## 19. Open Questions

- Should `LocalCheckResult` be public, or internal until report/evidence integration is scoped?
- Should `LocalCheckResult` serialize now, or wait until a persistence/evidence/report phase?
- Should process runner injection be trait-based or a small function-pointer/test harness boundary?
- Should timeout return a `LocalCheckResult` with `TimedOut`, or return a `WorkflowOsError` and let the handler decide how to expose it?
- Should redaction failure return a result with `RedactionFailed`, or fail closed with `WorkflowOsError`?
- How much environment construction should be platform-specific?
- Should `AllowlistedHandlerOnly` remain rejected in serialized contracts while test-only execution authority lives outside the model?
- What is the first non-dogfood command kind after infrastructure review: `DocsCheck` or `CargoFmtCheck`?
- What side-effect policy is sufficient before cargo/npm command handlers?
- When should command-output evidence be planned?

## 20. Final Recommendation

The next phase should be: **DocsCheck local handler plan review**.

The review should verify the proposed `DocsCheck` command authority, Node/npm environment policy, cache/write posture, output capture/redaction policy, registration posture, runtime/event boundary, and test plan before Workflow OS considers implementing the first non-dogfood local check handler.

Still not to be built:

- production handler registration;
- CLI handler exposure;
- workflow schema fields;
- automatic check execution;
- report artifact writing;
- evidence attachment;
- command-output evidence;
- side-effect boundary implementation;
- cargo/npm command handlers;
- source writes;
- recursive agents;
- agent swarms;
- hosted execution;
- release posture changes.
