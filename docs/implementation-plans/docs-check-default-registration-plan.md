# DocsCheck Default-Registration Plan

Status: Explicit `DocsCheck` registry helper implemented. `DocsCheckLocalHandler` remains production-shaped, explicit, non-default, and non-CLI. The next default-registration lane is documented in [Local Check Handler Default-Registration Plan](local-check-handler-default-registration-plan.md), and an explicit non-default local check registration profile/helper is implemented there. Local check result citation planning is documented in [Local Check Result Citation Plan](local-check-result-citation-plan.md). This plan does not implement true ambient default registration, CLI exposure, workflow schema fields, automatic check execution, `AllowlistedHandlerOnly`, report artifacts, evidence attachment, command-output evidence, side-effect boundary implementation, source writes, or release posture changes.

## 1. Executive Summary

Workflow OS now has a reviewed explicit `DocsCheckLocalHandler` for the fixed local command `npm run check:docs`.

The next roadmap question is whether that handler should ever be registered by default. Default registration would make a real local command handler available through normal local executor construction, so it carries more authority than explicit test registration.

This plan recommends **not implementing default registration yet**. It instead scoped a narrowly scoped **explicit local registry helper** for `DocsCheckLocalHandler`, still opt-in and non-CLI, so tests and future dogfood paths can assemble the handler without ad hoc registry wiring while default registration, CLI exposure, and workflow schema exposure remain deferred. That helper is now implemented as `LocalSkillRegistry::register_docs_check_handler(...)`.

## 2. Goals

- Decide whether `DocsCheckLocalHandler` should be registered by default now.
- Preserve explicit command authority.
- Preserve existing `LocalExecutor::new(...)` behavior.
- Avoid ambient local command execution.
- Define prerequisites for any future default registration.
- Define the smallest useful next implementation step.
- Keep docs check execution bounded to `npm run check:docs`.
- Preserve redaction, output, event, and workflow semantics.
- Keep report and evidence integration separate.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- default handler registration;
- CLI handler exposure;
- workflow schema fields;
- automatic local check execution;
- `AllowlistedHandlerOnly` enablement;
- broad handler discovery;
- arbitrary shell execution;
- user-supplied command text;
- broader npm or cargo handlers;
- report artifact writing;
- local check evidence attachment;
- command-output evidence;
- local check result work-report citation;
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

- State directory: `/tmp/workflow-os-docs-check-default-registration-plan`
- Run ID: `run-1781505794683044000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781505794683044000-2/d`
- Final status: `Completed`

Inspection confirmed the expected event history through `RunCompleted`.

## 5. Current Baseline

Implemented:

- `LocalCheckCommandKind::DocsCheck`;
- canonical command template `npm run check:docs`;
- `LocalCheckCommandContract::docs_check_model_only()`;
- `DocsCheckLocalHandler`;
- `TestOnlyDocsCheckHandler` compatibility alias;
- injected process-runner constructor;
- bounded `LocalCheckResult` mapping;
- secret-like stdout/stderr rejection;
- redaction-safe debug output;
- executor tests proving explicit registration and absence of default registration.

Implemented after this plan:

- explicit `LocalSkillRegistry::register_docs_check_handler(...)`;
- first-class local check result reference model;

Not implemented:

- default registry entry;
- CLI flag or command for real local checks;
- workflow schema fields for local check handlers;
- `AllowlistedHandlerOnly`;
- real npm smoke test in normal validation;
- cache/write sandbox;
- local check result work-report citation;
- local check result evidence;
- command-output evidence.

## 6. Default Registration Decision

Default registration should remain deferred.

Reasons:

- default registration changes the authority profile of ordinary local executor construction;
- the current side-effect model does not yet distinguish source writes from cache writes in executable handlers;
- npm cache behavior is not production-sandboxed;
- no CLI or schema contract exists for users to understand when local command execution is active;
- `AllowlistedHandlerOnly` remains intentionally invalid;
- report/evidence integration is not yet designed for local check results;
- automatic dogfood use should remain explicit until command execution posture is reviewed again.

Default registration can be reconsidered only after the prerequisites in this plan are met and reviewed.

## 7. Recommended Next Implementation

Implemented next step: **explicit `DocsCheck` local registry helper, non-default and non-CLI**.

Implemented API:

- `LocalSkillRegistry::register_docs_check_handler(&mut self, DocsCheckLocalHandler)`.

The helper:

- requires an already constructed `DocsCheckLocalHandler`;
- registers only the canonical `local/check-docs` skill ID and `v0` version;
- avoids constructing npm paths, cache directories, or runtime configuration;
- avoids default registration;
- avoids CLI exposure;
- avoids workflow schema exposure;
- remains testable without real npm execution;
- preserves existing `LocalExecutor::new(...)` behavior.

The helper should not:

- create a handler from ambient environment;
- search `PATH`;
- invent npm cache locations;
- enable `AllowlistedHandlerOnly`;
- register cargo, TypeScript, contract, or integration handlers;
- write files or artifacts;
- attach evidence or reports.

## 8. Default Registration Prerequisites

Before default registration is implemented, Workflow OS should have:

- explicit side-effect boundary modeling for local command handlers;
- documented npm cache/write sandbox policy;
- reviewed `AllowlistedHandlerOnly` or another execution-authorization posture;
- public CLI behavior plan, if CLI registration is considered;
- workflow schema plan, if spec-declared checks are considered;
- real npm smoke-test posture, likely opt-in or tightly bounded;
- stable local check result identity if results are to be cited;
- work-report citation plan for local check results;
- command-output evidence policy if command output evidence is ever considered;
- threat model update for local command execution authority.

## 9. Registration Options

| Option | Recommendation | Notes |
| --- | --- | --- |
| Keep explicit manual registry wiring only | Acceptable but awkward | Safe, but repeated test setup will drift. |
| Add explicit registry helper | Recommended | Improves ergonomics without ambient authority. |
| Register `DocsCheck` in `LocalSkillRegistry::new()` | Defer | This is true default registration and changes runtime authority. |
| Add CLI flag to register `DocsCheck` | Defer | Needs CLI contract and security documentation. |
| Add workflow schema field for checks | Defer | Needs schema and authorization design. |
| Register all local check handlers | Reject | Too broad and unsafe. |

## 10. Authorization Posture

The next helper should not change authorization posture.

Rules:

- `docs_check_model_only()` remains `ModelOnly`;
- serialized `AllowlistedHandlerOnly` remains invalid;
- execution authority remains in explicit Rust construction/registration;
- forged workflow specs must not gain execution authority;
- missing handlers should continue to fail clearly when not explicitly registered.

Future authorization work should decide whether `AllowlistedHandlerOnly` can become valid for a narrow subset of local check contracts.

## 11. Runtime And Event Boundary

The explicit registry helper should use existing runtime mechanics only.

Allowed:

- explicit registration into a caller-provided `LocalSkillRegistry`;
- normal `LocalExecutor` events when a workflow invokes the registered skill;
- existing failed-run behavior when the handler is absent.

Not allowed:

- new runtime event kinds;
- automatic execution;
- post-terminal events;
- report artifact writes;
- StateBackend writes outside normal executor events;
- observability or audit noise beyond existing executor paths;
- CLI output.

## 12. Environment, Toolchain, And Cache Boundary

The registry helper must not create environment or toolchain policy.

It should not:

- resolve npm executable paths;
- infer repository root;
- create cache directories;
- read ambient environment variables;
- pass through credentials;
- run npm directly.

Those responsibilities stay with explicit `DocsCheckLocalHandler` construction and later production sandbox planning.

## 13. Privacy And Redaction

The helper must not store or copy:

- raw command output;
- docs contents;
- parser payloads;
- provider payloads;
- environment values;
- npm tokens;
- registry credentials;
- authorization headers;
- private keys;
- token-like strings.

Errors should remain stable and non-leaking. Debug output for any helper-owned type, if one is introduced, must not reveal paths, cache directories, command arguments, or environment values.

## 14. Test Plan For Future Implementation

Future implementation should add tests for:

- explicit helper registers only `local/check-docs` `v0`;
- helper requires a supplied `DocsCheckLocalHandler`;
- helper does not construct npm paths or cache directories;
- helper does not read ambient environment;
- `LocalSkillRegistry::new()` remains empty/default-safe;
- missing default handler behavior remains unchanged;
- explicit helper registration executes through `LocalExecutor`;
- generated process request still uses `npm run check:docs`;
- no report artifacts are written;
- no source files are written;
- no CLI output is emitted;
- no `AllowlistedHandlerOnly` behavior changes;
- no schema files change;
- existing local check, executor, report, evidence, validation, adapter, and runtime tests still pass.

## 15. Documentation Requirements For Future Implementation

Docs must say:

- an explicit registry helper is implemented, if implemented;
- default registration is not implemented;
- CLI exposure is not implemented;
- workflow schema fields are not implemented;
- automatic local check execution is not implemented;
- `AllowlistedHandlerOnly` is not enabled;
- report artifacts are not implemented;
- evidence attachment is not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported.

## 16. Risks

- A helper named too broadly could be mistaken for default registration.
- A helper that constructs handlers from ambient paths would become hidden runtime configuration.
- Registering a handler by default would make local command execution ambient.
- Enabling `AllowlistedHandlerOnly` too early would let serialized specs imply execution authority.
- CLI exposure would create public compatibility expectations before cache and side-effect posture are settled.
- Report/evidence integration could accidentally store raw command output if planned too late.

## 17. Open Questions

- Should the explicit helper live on `LocalSkillRegistry` or as a free function?
- Should the helper accept a handler or a fully explicit handler-construction input?
- Should future dogfood workflows use the helper before any CLI exposure exists?
- What exact side-effect model is required before default registration?
- Should `AllowlistedHandlerOnly` wait for the side-effect boundary model?
- Should real npm smoke tests be opt-in or part of normal validation?
- Should local check results get stable IDs before report/evidence integration?

## 18. Final Recommendation

Do not implement default registration yet.

Implemented phase: **explicit DocsCheck registry helper, non-default and non-CLI**.

That phase improves safe wiring ergonomics while preserving all current boundaries: no default registration, no CLI, no schema fields, no automatic execution, no `AllowlistedHandlerOnly`, no artifacts, no evidence, no side-effect model, no source writes, and no release posture change.

Recommended next phase: **DocsCheck registry helper review**.
