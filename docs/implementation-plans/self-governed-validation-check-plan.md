# Self-Governed Validation/Check Plan

Status: Contract model, canonical command-template binding, one test-only dogfood validation handler, local check handler infrastructure, an explicit production-shaped/non-default `DocsCheckLocalHandler`, an explicit non-default DocsCheck registry helper, a local check result reference model, and a model-only local check side-effect boundary are implemented. `DocsCheck` production-posture planning is documented in [DocsCheck Local Handler Production-Posture Plan](docs-check-production-posture-plan.md), default-registration planning is documented in [DocsCheck Default-Registration Plan](docs-check-default-registration-plan.md), dogfood real DocsCheck execution through explicit profile registration is implemented as documented in [Dogfood Real DocsCheck Plan](dogfood-real-docs-check-plan.md), and local check side-effect/cache/write boundary planning is documented in [Local Check Side-Effect Boundary Plan](local-check-side-effect-boundary-plan.md). Local check result citation planning is documented in [Local Check Result Citation Plan](local-check-result-citation-plan.md), and self-governance multi-step dogfood conversion planning is documented in [Self-Governance Dogfood Multi-Step Conversion Plan](self-governance-dogfood-multi-step-conversion-plan.md). Production/default local validation/check skill handler registration and WorkReport/evidence citation wiring for local check results are not implemented.

## 1. Executive Summary

Workflow OS now has a first self-governance dogfood project at `dogfood/workflow-os-self-governance`. That project proves the local kernel can govern a Workflow OS planning/docs task: validate specs, create a local run, pause for approval, resume after approval, and preserve inspectable event history.

Governed multi-step execution has since been implemented, hardened, and reviewed. The planned next dogfood step is to convert the self-governance project into a small sequential multi-step workflow while keeping validation/check command execution outside the kernel unless separately scoped.

This is kernel-governed, Codex-executed dogfooding. Codex or a human still performs repository edits and validation commands outside the kernel.

The next question is how to introduce real local validation/check skill handlers safely. The goal is not to turn Workflow OS into a shell runner. The goal is to let the kernel govern selected validation and quality gates for Workflow OS itself through explicit contracts, narrow command allowlists, bounded output capture, evidence references, work reports, and conservative failure semantics.

This plan has produced a local validation/check command contract model with canonical command-template binding for each allowed command kind. It does not implement validation/check handlers, command execution, side-effect modeling, writes, automatic report generation, CLI report rendering, schemas, examples, recursive agents, agent swarms, hosted execution, or production self-hosting.

## 2. Goals

- Define a safe roadmap for self-governed validation/check execution.
- Keep Workflow OS local-first, deterministic, and auditable.
- Preserve the kernel-governed, Codex-executed boundary until explicit local check handlers are implemented and reviewed.
- Introduce real validation/check handlers only through explicit allowlisted commands.
- Avoid arbitrary shell execution.
- Avoid ambient filesystem, network, or environment authority.
- Capture bounded check results without copying raw logs or secrets.
- Prepare check results to cite `EvidenceReference`, validation diagnostics, runtime events, and work reports through stable references.
- Preserve existing workflow semantics and local executor behavior.
- Make side effects explicit before any command that can write build artifacts, caches, or local state.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- arbitrary shell-command skills;
- generic build-command execution;
- automatic Codex control through the kernel;
- automatic runtime report generation for every run;
- automatic report artifact writing from executor paths;
- CLI report rendering or export;
- workflow schema changes;
- local check result citation implementation;
- command-output evidence;
- example updates;
- production self-hosting claims;
- hosted or distributed runtime claims;
- recursive agents;
- agent swarms;
- Level 3 or Level 4 autonomy;
- write-capable adapters;
- side-effect boundary implementation;
- new persistence behavior;
- provider calls or live adapter execution;
- DLP or access-control systems;
- release posture changes.

## 4. Current Foundation

Implemented foundation relevant to this plan:

- Local project validation.
- Sequential local executor.
- Explicit local skill handler registry.
- Approval-gated execution.
- Durable local event history.
- Policy decisions and audit events.
- Work report contract and report models.
- In-memory terminal local report generation helper.
- Executor-integrated report-bearing local execution path.
- Explicit local report artifact store.
- First self-governance dogfood project for planning/docs work.

Current limitations:

- Declared `local/*` skills are not automatically executable.
- CLI `--mock-all-local-skills` is deterministic preview tooling only.
- There are no real local build/check skill handlers.
- The local executor is sequential and local only; branching, parallelism, and live command execution remain unsupported.
- There is no side-effect boundary model.
- The command allowlist/template model exists, but it remains non-executing.
- There is no bounded command-output capture model for real check handlers.
- There is no automatic work-report generation or artifact writing from executor paths.
- There is a first-class local check result reference model, but no work-report citation wiring for local check outcomes.

## 5. Dogfood Governance Boundary

The self-governance dogfood project should remain the governance wrapper for Workflow OS build work while this phase is planned.

For planning/docs tasks:

- the dogfood workflow may validate, run, pause for approval, resume, and preserve events;
- Codex or a human still performs repository edits;
- validation commands still run outside the kernel;
- final implementation reports still disclose commands run, risks, and incomplete work.

For future validation/check tasks:

- the kernel may govern the check request and approval boundary;
- real check execution must not be added until command authority, output capture, and side-effect policy are explicitly modeled;
- no check should be described as kernel-executed unless a real handler exists and is tested.

## 6. Candidate Check Inventory

| Candidate check | Current source | First implementation? | Rationale |
| --- | --- | --- | --- |
| `workflow-os validate` for the dogfood project | Existing CLI command | First after command boundary model | It is closest to the kernel and produces bounded diagnostics, but still shells out if invoked from a handler. |
| `npm run check:docs` | Existing docs checker | Explicit production-shaped handler implemented | It is narrowly scoped and docs-only. Default registration, CLI exposure, and automatic execution remain deferred. |
| `cargo fmt --all --check` | Rust formatter | Defer until side-effect policy is explicit | It is a common gate, but toolchain execution and cache behavior should be declared. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Rust lint gate | Defer until side-effect policy is explicit | It may write build artifacts and has potentially large output. |
| `cargo test --workspace` | Rust test gate | Defer until side-effect policy is explicit | It writes build artifacts and can execute arbitrary test code. |
| `npm run check:ts` | TypeScript checks | Defer | It may create caches or broader output and is less central to the first dogfood slice. |
| `npm run check:contracts` | SDK/schema contract checks | Defer | It may build artifacts and belongs after docs/Rust gate boundaries. |
| `npm run check:integrations` | Integration docs/checks | Defer | It depends on broader integration posture. |
| Live GitHub/Jira/CI smoke tests | Opt-in live scripts | Reject for self-governed local check v1 | Live provider access and credentials must remain outside this local dogfood check phase. |
| Arbitrary user-supplied commands | None | Reject | This would turn Workflow OS into an unsafe shell runner. |

## 7. Recommended First Target

The first implementation should not execute `cargo` or `npm` yet.

The first implementation phase added the **local validation/check command contract model only**. A follow-up model-only hardening phase bound each `LocalCheckCommandKind` to a canonical executable and argument vector.

That phase defines:

- a domain-neutral check command ID;
- a fixed allowed command vocabulary;
- canonical executable and argument templates for each command kind;
- expected working directory policy;
- environment policy;
- timeout policy;
- output-size limits;
- redaction policy;
- side-effect classification;
- result status vocabulary;
- report/evidence citation hooks;
- stable error codes.

The first test-only handler boundary is implemented and documented in [Test-Only Local Check Handler Plan](test-only-local-check-handler-plan.md). It executes only `workflow-os validate` for the dogfood project through explicit test registration. The handler review recommended broader handler planning before adding more command kinds. That planning is documented in [Broader Local Check Handler Plan](broader-local-check-handler-plan.md), and the first infrastructure slice is implemented. Production handler registration remains unsupported.

## 8. Command Authority Rules

Future local check handlers must follow these rules:

- Accept a typed check command, not an arbitrary shell string.
- Execute only commands from a repository-owned allowlist.
- Avoid shell interpolation, pipes, redirection, command substitution, glob expansion, and arbitrary environment variables.
- Use explicit executable plus argument vectors.
- Use a fixed working directory policy rooted in the repository.
- Use a fixed environment policy with secret-bearing variables removed unless explicitly allowed.
- Apply bounded timeouts.
- Apply bounded stdout/stderr capture.
- Redact output before storing summaries or diagnostics.
- Return structured errors with stable codes.
- Never include raw command output, paths with secrets, environment values, tokens, or provider payloads in errors.

The first real handler should not accept user-supplied command text.

## 9. Side-Effect Policy

Validation/check commands are not automatically read-only.

Examples:

- `cargo test` and `cargo clippy` can write build artifacts under `target/`.
- `npm` commands may write caches or build outputs depending on scripts.
- test code can perform filesystem or network behavior unless constrained by tests and environment.

Before real check execution is broadened, each allowlisted check must declare:

- whether it is intended to be read-only;
- expected local writes, if any;
- permitted output directories;
- cache behavior;
- whether network access is forbidden;
- whether credentials are forbidden;
- cleanup expectations;
- whether failure leaves partial artifacts;
- whether the check may run in normal CI without special privileges.

For the first real handler, prefer commands that can run without network access and without modifying repository source files.

The local-check-specific planning boundary for source writes, cache writes, build outputs, temp writes, and network posture is documented in [Local Check Side-Effect Boundary Plan](local-check-side-effect-boundary-plan.md). That plan is narrower than the future generic side-effect boundary ADR for write-capable adapters.

## 10. Output Capture And Redaction

Future handlers should treat command output as sensitive by default.

Rules:

- Capture bounded stdout and stderr summaries only.
- Do not store full raw logs.
- Do not store command transcripts by default.
- Do not copy provider payloads, spec contents, parser payloads, environment variables, tokens, credentials, private keys, or authorization headers.
- Apply redaction before constructing diagnostics, evidence references, or work report text.
- Store exit code, duration, check ID, and bounded redacted summary.
- Use `EvidenceKind::CommandOutput` only after a dedicated command-output evidence policy is reviewed.
- Prefer `EvidenceKind::TestResult` or `EvidenceKind::ValidationResult` where a structured local result exists.

Raw logs may remain visible to the process operator during command execution, but they should not become workflow state, report text, or artifacts by default.

## 11. Evidence And Work Report Integration

Future check handlers should integrate with the existing evidence/report foundation without copying payloads.

Recommended behavior:

- attach or cite stable check-result references, not raw output;
- cite validation diagnostics by stable references where available;
- cite workflow events and audit events for check scheduling and completion;
- cite report artifacts only when explicitly written through the artifact store;
- include validation/check outcomes in the `validation and quality checks` report section;
- disclose skipped, unavailable, or failed checks explicitly;
- include known limitations when a check is mocked, skipped, partial, or outside the kernel.

Do not create fake evidence for missing check results.

## 12. Failure Semantics

The first real validation/check handler must fail closed and preserve workflow semantics.

Recommended status vocabulary:

- `passed`;
- `failed`;
- `timed_out`;
- `skipped`;
- `not_available`;
- `internal_error`;
- `policy_denied`;
- `redaction_failed`.

Rules:

- A failed check should fail the governed step only when the workflow declares that check as required.
- A handler construction, redaction, timeout, or policy error must return a structured non-leaking error.
- Failure must not append misleading user diagnostics.
- Partial output capture must not become successful evidence.
- Retry behavior must remain explicit and bounded.
- Approval gates should be required before sensitive or expensive checks until the policy model is reviewed.

## 13. Runtime And CLI Integration Options

Potential future integration options:

| Option | Assessment |
| --- | --- |
| Register a real local check `SkillHandler` in tests only | Implemented for `WorkflowOsValidateDogfood` through explicit test registration. It proves the handler boundary without broad CLI surface. |
| Add CLI flag to register allowlisted local check handlers | Defer. CLI exposure raises compatibility and security expectations. |
| Replace `--mock-all-local-skills` with real handler discovery | Reject for now. It would make local skill execution too ambient. |
| Add workflow schema fields for check commands | Defer until the model and handler boundary are reviewed. |
| Execute checks automatically after every Codex task | Reject. Workflow OS should govern explicit runs, not ambient desktop activity. |
| Run checks through live adapters | Reject for this phase. Validation/check dogfooding should stay local. |

## 14. Security And Privacy Review Requirements

Any implementation prompt for real local checks must require review of:

- command allowlist completeness;
- argument injection prevention;
- working-directory policy;
- environment sanitization;
- network posture;
- filesystem write posture;
- timeout behavior;
- output bounds;
- redaction behavior;
- error message non-leakage;
- audit records;
- evidence references;
- work report citations;
- no raw log persistence;
- no secret leakage through `Debug`, serialization, diagnostics, or report text.

## 15. Test Plan For Future Implementation

Future tests should cover:

- valid allowlisted check command construction;
- rejection of unknown commands;
- rejection of shell metacharacters or arbitrary command text;
- fixed argument-vector execution;
- environment sanitization;
- working-directory bounds;
- timeout behavior;
- bounded output capture;
- stdout/stderr redaction;
- non-leaking errors;
- passed check result;
- failed check result;
- timed-out check result;
- skipped or not-available check result;
- side-effect classification for each allowlisted command;
- no repository source file mutation for read-only declared checks;
- no network access for local checks;
- evidence/report citations use stable references only;
- no raw command output copied into work reports;
- existing local executor tests still pass;
- dogfood workflow still validates and completes through approval;
- docs checks pass.

## 16. Proposed Implementation Sequence

1. Add a local validation/check command contract model only.
2. Review the contract model and side-effect classification.
3. Bind each command kind to a canonical executable and argument template.
4. Review the template binding fix.
5. Plan the first test-only local check handler boundary.
6. Add a test-only real local check handler for one low-risk allowlisted command.
7. Review the test-only local check handler.
8. Add bounded output capture and redaction expansion tests as needed.
9. Review before exposing any handler through CLI.
10. Add explicit dogfood workflow wiring only after handler review.
11. Review before adding report artifacts, schema fields, examples, or broader check families.

## 17. Open Questions

- Should the first real check be `workflow-os validate` for the dogfood project or `npm run check:docs`?
- Should `cargo fmt --all --check` be considered read-only if it writes no source files but may touch toolchain state?
- How should the kernel classify build-cache writes under `target/`?
- Should check command execution require approval every time during early dogfooding?
- Should check output summaries become `EvidenceReference` values immediately or only work report section text first?
- Should command-output evidence require a new reviewed attachment boundary before any handler emits it?
- Should the handler run inside a temporary workspace or the repository root?
- Should cleanup be part of the handler contract or outside the kernel?
- How should check results relate to future Composable Harness Contracts?
- What is the smallest useful validation/check dogfood workflow after the planning/docs workflow?

## 18. Final Recommendation

The next phase should be: **test-only `WorkflowOsValidateDogfood` local check handler review**.

It should verify the handler remains test-only, explicit, non-shell, bounded, redaction-safe, and does not add CLI exposure, schema changes, automatic check execution, report artifacts, side-effect boundary implementation, writes, or production self-hosting.

Still not built:

- real local check execution;
- arbitrary shell execution;
- CLI handler exposure;
- workflow schema changes;
- automatic report generation;
- automatic artifact writing;
- examples;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- production self-hosting behavior.
