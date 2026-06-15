# Test-Only Local Check Handler Plan

Status: Test-only `WorkflowOsValidateDogfood` handler implemented. No production local check handler is implemented.

## 1. Executive Summary

Workflow OS now has a local validation/check command contract model with canonical command-template binding. The model can represent a fixed allowlisted check command, validate command tokens, reject mismatched executable/argument vectors, bound output capture, classify side effects, and keep execution posture model-only.

The next question is how to introduce the first real local check handler safely.

This plan recommended a **test-only local check handler** for `WorkflowOsValidateDogfood` as the first execution target. That handler is implemented and executes only the canonical `workflow-os --project-dir dogfood/workflow-os-self-governance validate` template, only through explicit handler registration in focused tests.

This plan does not implement production command execution, default runtime wiring, CLI exposure, workflow schema changes, automatic check execution, side-effect boundary implementation, writes, automatic report generation, report artifact writing, examples, recursive agents, agent swarms, hosted runtime behavior, or production self-hosting.

## 2. Goals

- Define the smallest safe first local check handler boundary.
- Preserve the kernel-governed, Codex-executed dogfood posture.
- Execute only one reviewed allowlisted command in test-only scope.
- Avoid arbitrary shell execution.
- Use canonical executable/argument templates from `LocalCheckCommandKind`.
- Use explicit handler registration, not ambient discovery.
- Keep environment, working directory, network, timeout, and output capture constrained.
- Capture bounded redacted check summaries.
- Preserve existing workflow semantics and event-log correctness.
- Avoid raw command output, command transcripts, environment values, provider payloads, parser payloads, spec contents, and secrets.
- Prepare for future evidence and work report integration without implementing either in this phase.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- production local command execution;
- arbitrary shell-command skills;
- user-supplied command text;
- generic build-command execution;
- CLI handler exposure;
- workflow schema changes;
- automatic check execution;
- automatic Codex control through the kernel;
- automatic runtime report generation;
- automatic report artifact writing;
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

Implemented foundation:

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
- canonical command-template binding for all current check command kinds.

Current limitations:

- no real local build/check skill handlers;
- no command execution through `LocalCheckCommandContract`;
- no output redaction implementation for command output;
- no real sandbox;
- no side-effect boundary model;
- no CLI exposure for allowlisted check handlers;
- no workflow schema fields for checks;
- no automatic work-report generation from check results;
- no check evidence attachment.

## 5. Recommended First Handler Target

Recommended first target: `LocalCheckCommandKind::WorkflowOsValidateDogfood`.

Canonical template:

- executable: `workflow-os`
- arguments: `--project-dir dogfood/workflow-os-self-governance validate`

Rationale:

- It validates the dogfood project itself.
- It is closest to the Workflow OS kernel boundary.
- It has bounded, deterministic success/failure behavior compared with broad `cargo` or `npm` commands.
- It should not write repository source files.
- It avoids running arbitrary test code.
- It avoids Node package scripts for the first handler slice.
- It proves handler mechanics before broader toolchain commands are considered.

Deferred first targets:

- `DocsCheck`: useful but invokes Node/npm and needs package-script output policy review.
- `CargoFmtCheck`: may touch toolchain/cache state and needs side-effect policy.
- `CargoClippyWorkspace`: writes build artifacts and can emit large output.
- `CargoTestWorkspace`: writes build artifacts and executes arbitrary tests.
- TypeScript, contract, and integration checks: defer until the first handler boundary is reviewed.

## 6. Handler Scope

The first handler should be test-only.

Allowed:

- focused Rust tests may register the handler explicitly;
- the handler may execute the canonical `WorkflowOsValidateDogfood` template;
- the handler may return a structured local check result object if introduced in the implementation phase;
- the handler may emit normal workflow events already produced by `LocalExecutor` for skill execution;
- the handler may produce bounded redacted output summaries in memory.

Not allowed:

- CLI flags to register real check handlers;
- broad handler discovery;
- production default registration;
- automatic execution after Codex tasks;
- workflow schema fields for check contracts;
- report artifact writing;
- persistence of command output;
- shell strings;
- user-supplied executable or arguments;
- source writes;
- network access;
- provider calls;
- live adapter execution.

## 7. Handler API Shape

The implementation should choose the smallest idiomatic shape already compatible with local executor patterns.

Preferred approach:

- add a focused local skill handler used only by tests;
- require an explicit `LocalCheckCommandContract`;
- validate the contract immediately before execution;
- require `command_kind == WorkflowOsValidateDogfood`;
- require the existing canonical executable/argument template;
- require `execution_posture == AllowlistedHandlerOnly` only if that posture is separately enabled by the implementation phase, otherwise use a distinct internal test-only execution gate that does not weaken the model contract.

Important design constraint:

The current model intentionally rejects `AllowlistedHandlerOnly`. The implementation phase must decide whether to:

- introduce a separate handler input that pairs a model-only contract with an explicit test-only execution authorization; or
- allow `AllowlistedHandlerOnly` only for `WorkflowOsValidateDogfood` behind a narrowly validated test-only handler boundary.

The implementation must not broadly authorize `AllowlistedHandlerOnly` for all command kinds.

## 8. Command Authority Rules

The handler must:

- use `std::process::Command` or equivalent only with explicit executable and argument vector;
- never invoke a shell;
- never concatenate command strings;
- never accept caller-supplied command text;
- never accept caller-supplied extra arguments;
- reject any contract not matching `WorkflowOsValidateDogfood`;
- reject any executable/argument mismatch through existing template validation;
- reject unsafe or unsupported execution posture;
- use a repository-root working directory resolved from explicit test context;
- avoid path traversal or ambient current-directory assumptions;
- disable network by environment policy as far as practical for the test slice;
- use bounded timeout behavior.

## 9. Working Directory And Environment

Working directory:

- use the repository root;
- do not infer from process ambient current directory unless the test passes it explicitly;
- reject non-repository paths if a repository-root guard exists or can be implemented narrowly;
- do not run from arbitrary caller-supplied directories.

Environment:

- use sanitized minimal environment;
- remove secret-bearing variables;
- do not pass tokens, provider credentials, authorization headers, private keys, or arbitrary environment values;
- allow only required non-secret variables for locating the local binary/toolchain if needed;
- document any required environment passthrough explicitly in tests.

The first implementation should prefer invoking an already-built local binary path supplied by test context rather than relying on global PATH lookup.

## 10. Output Capture And Redaction

The handler should capture bounded output summaries only.

Rules:

- bound stdout and stderr according to `LocalCheckOutputCapturePolicy`;
- do not persist raw output;
- do not store full command transcripts;
- do not copy raw spec contents;
- do not copy parser payloads;
- do not copy provider payloads;
- do not copy environment values;
- redact secret-like values before output becomes diagnostics, errors, work report text, or future evidence;
- store exit status, duration, check ID, and bounded redacted summary only if a result type is introduced.

Output that exceeds bounds should be truncated safely and marked as truncated without retaining hidden raw tail content.

## 11. Result Model

If the implementation needs a result type, it should be small and local-check-specific.

Candidate fields:

- check command ID;
- command kind;
- result status;
- exit code, if available;
- duration;
- bounded redacted stdout summary;
- bounded redacted stderr summary;
- truncation flags;
- started/completed timestamps;
- stable error code for internal failures;
- citation hooks for future work reports.

The result type must not store:

- raw command output;
- full command transcript;
- raw spec files;
- parser payloads;
- provider payloads;
- environment values;
- tokens or credentials.

Do not attach `EvidenceReference` in the first handler implementation unless separately planned.

## 12. Failure Semantics

Recommended behavior:

- successful validation command maps to `LocalCheckResultStatus::Passed`;
- non-zero validation exit maps to `LocalCheckResultStatus::Failed`;
- timeout maps to `LocalCheckResultStatus::TimedOut`;
- handler construction failure maps to `LocalCheckResultStatus::InternalError` or a structured `WorkflowOsError`;
- unsupported contract maps to `PolicyDenied` or a stable validation error;
- redaction failure maps to `RedactionFailed` and must not expose raw output.

For the first implementation, failed validation should fail only the governed test step that explicitly invokes the handler. It must not change global executor semantics.

## 13. Runtime And Event Boundary

The first implementation should run through existing local executor skill handler mechanics in tests.

Allowed event behavior:

- normal local executor events for skill invocation may be emitted during a test workflow run;
- existing event ordering and idempotency semantics must remain unchanged.

Not allowed:

- new runtime event types;
- post-terminal event append behavior;
- automatic check events outside an explicitly run workflow;
- state backend writes beyond the normal test workflow run event log;
- report artifact writes;
- automatic work report generation.

## 14. Evidence And Work Report Posture

The first handler implementation should not create `EvidenceReference` values by default.

Recommended v1 posture:

- return or record bounded local check result summaries in the test context;
- cite workflow/audit events later through existing report mechanisms only when separately scoped;
- disclose mocked, skipped, unavailable, or failed checks in report text only when a report is explicitly generated;
- defer command-output evidence until a dedicated command-output evidence policy is reviewed.

Do not create fake evidence for missing results.

## 15. Security And Privacy Requirements

The implementation prompt must require:

- no shell invocation;
- no arbitrary command text;
- no user-supplied extra arguments;
- sanitized environment;
- no provider credentials;
- no network dependency;
- bounded timeout;
- bounded output capture;
- redaction before storage or reporting;
- non-leaking errors;
- no source writes;
- no raw output persistence;
- no CLI exposure;
- no schema exposure;
- no release posture changes.

The implementation must be reviewed before adding additional command kinds.

## 16. Test Plan

Future implementation tests must cover:

- test-only handler registration is explicit;
- handler is not registered by default;
- handler rejects unsupported command kinds;
- handler rejects template mismatches;
- handler rejects unsafe execution posture;
- handler invokes no shell;
- handler executes only canonical `WorkflowOsValidateDogfood`;
- handler runs from repository root;
- handler uses sanitized environment;
- handler requires no provider credentials;
- handler does not write source files;
- handler does not create report artifacts;
- handler does not expose CLI output;
- successful dogfood validation maps to passed result;
- failed validation maps to failed result using a controlled invalid fixture if feasible;
- timeout behavior is bounded or unit-tested through an injectable process runner;
- stdout/stderr summaries are bounded;
- raw output is not persisted;
- secret-like output is redacted or rejected without leakage;
- errors use stable codes and do not leak command text, output, paths, or secrets;
- existing local executor tests still pass;
- dogfood workflow still validates and completes through approval;
- docs checks pass.

## 17. Proposed Implementation Sequence

1. Add a test-only local check handler abstraction around a fixed process-runner trait or helper.
2. Register the handler only in focused tests.
3. Support only `WorkflowOsValidateDogfood`.
4. Add bounded output capture and redaction.
5. Add local check result status mapping.
6. Verify no shell invocation, no source writes, no provider credentials, no report artifacts, and no CLI exposure.
7. Review the handler phase.
8. Only after review, consider wiring the dogfood workflow to use the real handler instead of mock skill execution.

## 18. Open Questions

- Should the implementation introduce a small local-check result type before handler execution?
- Should process execution be behind a test-injectable runner to make timeout and output cases deterministic?
- Should the handler execute the current compiled binary path or invoke `workflow-os` through PATH?
- How should sanitized environment still locate the binary and required dynamic libraries on each platform?
- Should `AllowlistedHandlerOnly` become valid only for `WorkflowOsValidateDogfood`, or should execution authorization live outside the serialized contract?
- What is the exact redaction policy for validation output?
- Should the first handler produce work report section text or only a local check result?
- Should check results eventually use `EvidenceKind::ValidationResult`, `EvidenceKind::TestResult`, or a separate command-output evidence policy?

## 19. Final Recommendation

The next implementation phase should be: **test-only `WorkflowOsValidateDogfood` local check handler**.

It should remain test-only, explicit, and narrow. It should execute only the canonical dogfood validation command through a non-shell process invocation, capture bounded redacted output, and return a structured result without adding CLI exposure, workflow schema changes, automatic check execution, report artifacts, evidence attachment, side-effect boundary implementation, writes, recursive agents, agent swarms, hosted execution, or production self-hosting.
