# DocsCheck Local Handler Production-Posture Plan

Status: Explicit production-shaped `DocsCheckLocalHandler` is implemented and remains non-default/non-CLI. Default-registration planning is documented in [DocsCheck Default-Registration Plan](docs-check-default-registration-plan.md), local check result citation planning is documented in [Local Check Result Citation Plan](local-check-result-citation-plan.md), and the model-only local check side-effect/cache/write boundary is documented in [Local Check Side-Effect Boundary Plan](local-check-side-effect-boundary-plan.md). Production/default registration, CLI exposure, workflow schema fields, automatic check execution, report artifacts, local check result citation, evidence attachment, command-output evidence, live side-effect enforcement, source writes, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has a reviewed, explicit `DocsCheck` local handler that can run through the existing `SkillHandler` and `LocalExecutor` boundary when manually registered in tests.

The next question is not how to run more commands. The next question is whether `DocsCheck` can safely move from test-scoped explicit construction toward a production posture.

This plan defined the required decisions before any production/default registration is implemented. The first production-shaped explicit handler is now implemented as `DocsCheckLocalHandler`, but production/default registration, CLI behavior, schema fields, automatic execution, report artifacts, local check result citation, evidence attachment, command-output evidence, side-effect modeling, writes, and release posture changes remain unimplemented.

## 2. Goals

- Decide what "production posture" means for the narrow `DocsCheck` handler.
- Preserve explicit command authority and non-shell execution.
- Keep `DocsCheck` bounded to `npm run check:docs`.
- Define cache, environment, toolchain, and filesystem-write posture.
- Decide whether `LocalCheckExecutionPosture::AllowlistedHandlerOnly` should become valid for reviewed local check contracts.
- Define the minimum production-readiness test expectations.
- Preserve deterministic workflow semantics and event ordering.
- Keep raw command output, docs contents, parser payloads, provider payloads, tokens, and credentials out of durable model surfaces.
- Prepare for later work-report or evidence citation without implementing either.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- production handler registration;
- default handler registration;
- CLI handler exposure;
- workflow schema fields;
- automatic check execution;
- arbitrary shell execution;
- broader npm or cargo command families;
- command-output evidence attachment;
- local check result work-report integration;
- report artifact writing;
- persistence changes;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- live provider access;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 4. Governance Check

This planning phase was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-docs-check-production-posture-plan-2`
- Run ID: `run-1781504474525998000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781504474525998000-2/d`
- Final status: `Completed`

Inspection confirmed the expected event history through `RunCompleted`.

## 5. Current Implementation Baseline

Implemented:

- `LocalCheckCommandKind::DocsCheck`;
- canonical command template: `npm run check:docs`;
- `LocalCheckCommandContract::docs_check_model_only()`;
- `DocsCheckLocalHandler`;
- `TestOnlyDocsCheckHandler` compatibility alias;
- injected process-runner constructor;
- bounded `LocalCheckResult` mapping;
- secret-like stdout/stderr rejection;
- redaction-safe handler debug output;
- executor tests proving non-default registration and explicit registration behavior.

Still not implemented:

- default handler registry entry;
- CLI flag or command to enable the handler;
- workflow schema field for local checks;
- real npm smoke test in normal validation;
- npm cache/write sandbox;
- local check result evidence or report citation.

## 6. Production-Posture Options

| Option | Assessment |
| --- | --- |
| Keep `DocsCheck` test-scoped only | Safest. No new runtime authority. Does not advance kernel-dogfood checks beyond proof path. |
| Add explicit production handler type, still non-default | Recommended first implementation if this plan is accepted. It removes the `TestOnly` name while keeping construction explicit. |
| Add default local registry registration | Defer. This makes local command execution ambient and needs stronger sandbox/cache posture. |
| Add CLI flag to register `DocsCheck` | Defer. CLI exposure creates public compatibility and security expectations. |
| Add workflow schema field for check handlers | Defer. Schema exposure should follow production registration and contract enforcement design. |
| Replace `--mock-all-local-skills` with real checks | Reject for now. This would change dogfood and CLI semantics too early. |

Recommended posture: create a production-shaped but explicitly constructed `DocsCheckLocalHandler` only after a review accepts this plan. It should remain non-default and should not be reachable from CLI or workflow schema fields.

## 7. Handler Naming And Export Posture

The current `TestOnlyDocsCheckHandler` proves behavior but should not be treated as stable production API.

Implemented first step:

- introduced `DocsCheckLocalHandler`;
- retained `TestOnlyDocsCheckHandler` as a compatibility alias;
- document the handler as local, explicit, and experimental;
- avoid a broad public API stabilization claim;
- keep process-runner injection available for tests.

Before public release hardening, review whether local check process-runner types should remain exported or become narrower internal/test utilities.

## 8. Execution Posture Decision

`LocalCheckExecutionPosture::AllowlistedHandlerOnly` currently remains rejected by serialized contract validation.

Recommended near-term behavior:

- keep `docs_check_model_only()` unchanged for the next implementation;
- keep execution authority in explicit handler construction, not serialized workflow data;
- do not make `AllowlistedHandlerOnly` valid until a separate authorization review defines exactly which command kinds, environments, and side-effect classes it permits.

If `AllowlistedHandlerOnly` is later enabled, it must be narrow, deny-by-default, and tested against forged serialized contracts.

## 9. Command Authority Rules

Any production-shaped `DocsCheck` handler must:

- accept only `LocalCheckCommandKind::DocsCheck`;
- execute only `npm run check:docs`;
- use executable plus fixed argument vector;
- never invoke a shell;
- reject caller-supplied extra arguments;
- reject mismatched executable or arguments through the contract model;
- use repository root as working directory;
- use sanitized minimal environment;
- use disabled network policy;
- use bounded timeout and output capture;
- return stable non-leaking errors.

It must not support:

- pipes;
- redirection;
- glob expansion;
- command substitution;
- arbitrary package scripts;
- live provider credentials;
- user-supplied command text.

## 10. Toolchain And Path Policy

Production posture requires a clear npm executable policy.

Recommended first implementation:

- require an explicit npm executable path supplied by construction;
- validate that the path exists and is a file;
- do not search arbitrary user `PATH` inside the handler;
- keep tests on injected runners by default;
- optionally add one opt-in real npm smoke test only after sandbox/cache policy is reviewed.

Do not install dependencies, run `npm ci`, mutate lockfiles, or resolve missing toolchains from the handler.

## 11. Environment And Cache Policy

The handler should continue to start from a minimal environment.

Allowed:

- fixed minimal `PATH`;
- explicit `NPM_CONFIG_CACHE`, if supplied and validated.

Forbidden:

- inherited ambient environment;
- `NPM_TOKEN`;
- provider tokens;
- registry credentials;
- authorization headers;
- private keys;
- secret-like environment variable names or values.

Recommended cache posture:

- require an explicit cache directory for any real npm execution;
- treat cache writes as cache-only behavior, not source writes;
- do not create report artifacts in the cache;
- do not treat cache writes as approval for broader filesystem side effects;
- document that production cache sandboxing remains limited until the side-effect boundary is modeled.

The local-check-specific side-effect/cache/write boundary is now planned in [Local Check Side-Effect Boundary Plan](local-check-side-effect-boundary-plan.md). That plan must be reviewed before any live docs-check smoke or broader cargo/npm handler expansion.

## 12. Side-Effect And Filesystem Policy

`DocsCheck` is intended to be source-read-only, but npm may touch cache directories.

Allowed for a production-shaped handler:

- read repository docs and scripts required by `npm run check:docs`;
- write only to an explicitly supplied npm cache directory, if real npm execution is used;
- write normal workflow events only through existing executor behavior when explicitly registered.

Not allowed:

- source file writes;
- lockfile updates;
- package metadata mutation;
- dependency installation;
- report artifact writing;
- state backend writes beyond normal executor events;
- arbitrary filesystem writes;
- side-effect modeling shortcuts.

## 13. Output And Redaction Policy

The existing output boundary should remain the storage boundary.

Rules:

- use `LocalCheckResult` for bounded stdout/stderr summaries;
- reject secret-like stdout/stderr;
- keep debug output redaction-safe;
- do not persist raw command output;
- do not store command transcripts;
- do not copy raw docs contents;
- do not copy parser payloads;
- do not create command-output evidence in this phase;
- keep errors stable and non-leaking.

Recommended timeout remains 120 seconds unless a future measured real-run phase justifies a different bound.

## 14. Registration And Runtime Boundary

Production posture should advance in small steps.

Recommended next implementation boundary:

- explicit construction only;
- no default registry entry;
- no CLI flag;
- no workflow schema fields;
- no automatic check execution;
- no replacement of mock local skills;
- no post-terminal events;
- no report artifacts.

Tests may explicitly register the handler with `LocalExecutor` to prove behavior, just as the current test-scoped handler does.

## 15. Evidence And Work Report Boundary

Local check result citation is useful but must remain separate from production handler registration.

Deferred:

- `EvidenceReference` creation for local check outputs;
- `EvidenceKind::CommandOutput` policy;
- work-report citation of local check results;
- durable check-result store;
- CLI rendering of check results.

Future integration should cite stable local check result references and workflow events, not raw stdout/stderr.

## 16. Test Plan For Future Implementation

Future implementation should add focused tests for:

- explicit production-shaped handler construction;
- rejection of non-`DocsCheck` command kinds;
- canonical argument vector;
- explicit npm executable path requirement;
- invalid executable path failure without leaking paths;
- repository-root validation;
- sanitized environment;
- optional cache directory validation;
- secret-like cache path failure without leaking values;
- no inherited token or provider environment;
- success, non-zero exit, timeout, and runner failure mapping;
- secret-like stdout/stderr rejection;
- redaction-safe `Debug`;
- explicit executor registration;
- no default registration;
- no report artifacts;
- no source writes in tests;
- existing local check, executor, report, evidence, validation, adapter, and runtime tests still pass.

If a real npm smoke test is added later, it should be opt-in or tightly bounded and must use the repository-local toolchain/cache policy.

## 17. Documentation Requirements For Future Implementation

Docs must say:

- whether `DocsCheck` remains explicit-only or gains a production-shaped handler;
- whether `TestOnlyDocsCheckHandler` is retained, renamed, or superseded;
- default registration remains unimplemented unless separately approved;
- CLI exposure remains unimplemented unless separately approved;
- workflow schema fields remain unimplemented;
- automatic local check execution remains unimplemented;
- report artifacts remain unimplemented;
- evidence attachment remains unimplemented;
- side-effect boundary modeling remains unimplemented;
- source writes remain unsupported.

## 18. Risks

- Moving from test-only naming to production-shaped naming can be mistaken for default runtime support.
- Real npm execution can inherit unexpected local toolchain behavior if executable and environment are not explicit.
- Cache writes can be confused with a general side-effect boundary.
- CLI exposure could create premature compatibility expectations.
- Work-report or evidence integration could accidentally store raw command output if not separately planned.
- Enabling `AllowlistedHandlerOnly` too early could turn serialized specs into execution authority.

## 19. Open Questions

- Should the next implementation add `DocsCheckLocalHandler` or rename the existing handler?
- Should `TestOnlyDocsCheckHandler` remain exported after a production-shaped handler exists?
- Should real npm smoke tests be opt-in or part of normal workspace tests?
- Should cache directories be required for real npm execution?
- Should `AllowlistedHandlerOnly` remain invalid until after a side-effect boundary model exists?
- When should local check results become work-report citations?
- Should local check results receive stable IDs before evidence/report integration?
- What minimal sandbox posture is sufficient before default registration?

## 20. Final Recommendation

Implemented phase: **explicit production-shaped `DocsCheckLocalHandler`, non-default and non-CLI**.

That phase keeps execution authority explicit, keeps `docs_check_model_only()` unchanged, avoids `AllowlistedHandlerOnly`, avoids default registration, avoids CLI/schema exposure, avoids artifacts and evidence, and preserves existing runtime semantics.

Recommended next phase after the production-shaped handler review: **explicit DocsCheck registry helper, non-default and non-CLI**, as planned in [DocsCheck Default-Registration Plan](docs-check-default-registration-plan.md).

What must still not be built:

- default handler registration;
- CLI handler exposure;
- workflow schema fields;
- automatic local check execution;
- local check result evidence;
- report artifact writing;
- side-effect boundary implementation;
- source writes;
- release posture changes.
