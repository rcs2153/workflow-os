# DocsCheck Registry Helper Report

Report date: 2026-06-15

## 1. Executive Summary

Workflow OS now has an explicit `DocsCheck` registry helper.

The helper registers a caller-supplied `DocsCheckLocalHandler` for the canonical `local/check-docs` `v0` skill. It does not construct handlers, resolve npm paths, infer repository roots, create cache directories, read ambient environment, enable default registration, expose CLI behavior, add workflow schema fields, write artifacts, attach evidence, model side effects, or change release posture.

## 2. Governance Run

This implementation phase was governed by the self-governance dogfood workflow before code changes.

- State directory: `/tmp/workflow-os-docs-check-registry-helper-impl`
- Run ID: `run-1781506068912076000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781506068912076000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Completed

- Added `LocalSkillRegistry::register_docs_check_handler(...)`.
- Kept `LocalSkillRegistry::new()` empty/default-safe.
- Updated the executor test helper to use the explicit registry helper.
- Preserved explicit `DocsCheckLocalHandler` construction.
- Preserved non-default runtime behavior.
- Updated planning docs.

## 4. Scope Explicitly Not Completed

- No default handler registration.
- No CLI exposure.
- No workflow schema fields.
- No automatic local check execution.
- No `AllowlistedHandlerOnly` enablement.
- No npm path resolution.
- No cache directory creation.
- No ambient environment reads.
- No report artifact writing.
- No local check evidence attachment.
- No local check work-report citation integration.
- No side-effect boundary implementation.
- No source writes.
- No broader cargo/npm command handlers.
- No release posture change.

## 5. Helper API Summary

New helper:

- `LocalSkillRegistry::register_docs_check_handler(&mut self, DocsCheckLocalHandler) -> Result<(), WorkflowOsError>`

Behavior:

- registers only `local/check-docs`;
- registers only skill version `v0`;
- requires a prebuilt `DocsCheckLocalHandler`;
- returns a structured error only if the built-in skill identity is invalid.

## 6. Runtime Boundary Summary

Runtime behavior remains unchanged.

- `LocalSkillRegistry::new()` remains empty.
- Missing handler behavior remains the existing failed-run path.
- Explicit helper registration uses normal `LocalExecutor` skill invocation events.
- No new event kinds are introduced.
- No post-terminal events are appended.
- No report artifacts are created.

## 7. Privacy And Authority Summary

The helper does not touch command execution details.

It does not:

- resolve npm executable paths;
- infer repository roots;
- create cache directories;
- read environment variables;
- pass credentials;
- run npm;
- store command output;
- create evidence or report citations.

All command authority remains inside explicit `DocsCheckLocalHandler` construction and invocation.

## 8. Test Coverage Summary

Updated tests cover:

- default registry remains empty/default-safe;
- missing `DocsCheck` handler behavior remains unchanged;
- explicit helper registration executes through `LocalExecutor`;
- generated process request still uses `npm run check:docs`;
- no report artifacts are written;
- existing local check, executor, report, evidence, validation, adapter, and runtime tests remain part of validation.

## 9. Commands Run And Results

Validation commands for this phase:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 10. Remaining Known Limitations

- Default registration remains deferred.
- CLI exposure remains deferred.
- Workflow schema fields remain deferred.
- `AllowlistedHandlerOnly` remains disabled.
- Real npm smoke tests remain outside normal validation.
- Production cache/write sandbox remains deferred.
- Local check results are not yet cited in evidence or WorkReports.

## 11. Recommended Next Phase

Recommended next phase: **DocsCheck registry helper review**.

The review should verify the helper does not become default registration, does not construct execution context, does not add CLI/schema/artifact/evidence behavior, and preserves existing local executor semantics.
