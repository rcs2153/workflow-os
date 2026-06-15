# DocsCheck Local Handler Report

Report date: 2026-06-15

## 1. Executive Summary

Workflow OS now has an explicit test-scoped `DocsCheck` local handler.

The handler uses the existing local check result and process-runner infrastructure, accepts only the canonical `DocsCheck` contract, builds a sanitized process request, and derives skill output from validated `LocalCheckResult` values.

No production handler registration, default registry entry, CLI behavior, workflow schema field, automatic check execution, report artifact writing, evidence attachment, side-effect boundary implementation, source writes, or release posture change was introduced.

## 2. Governance Run

This implementation phase was governed by the self-governance dogfood workflow before code changes.

- State directory: `/tmp/workflow-os-docs-check-handler-impl`
- Run ID: `run-1781503375161141000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781503375161141000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Completed

- Added `LocalCheckCommandContract::docs_check_model_only()`.
- Added `TestOnlyDocsCheckHandler`.
- Added injected-runner constructor for deterministic tests.
- Added explicit optional `NPM_CONFIG_CACHE` process environment support.
- Reused `LocalCheckResult` for status, output summaries, truncation flags, and output references.
- Generalized local check skill output to report the actual command kind.
- Exported the handler from `workflow-core`.
- Added focused local check tests.
- Added executor tests proving non-default registration and explicit registration behavior.
- Updated roadmap and planning docs.

## 4. Scope Explicitly Not Completed

- No production `DocsCheck` handler registration.
- No default handler registration.
- No CLI exposure.
- No workflow schema fields.
- No automatic check execution.
- No report artifact writing.
- No local check evidence attachment.
- No command-output evidence policy.
- No side-effect boundary implementation.
- No source writes.
- No broader cargo/npm command families.
- No live provider access.
- No release posture change.

## 5. Handler API Summary

New handler:

- `TestOnlyDocsCheckHandler`

Construction:

- `new(...)` uses the standard process runner but remains explicitly constructed.
- `new_with_process_runner(...)` accepts an injected `LocalCheckProcessRunner` for deterministic tests.

Required inputs:

- canonical `DocsCheck` local check contract;
- explicit npm executable path;
- repository root;
- optional npm cache directory;
- injected or standard process runner.

The handler is never registered by default.

## 6. Command And Environment Summary

The handler accepts only `LocalCheckCommandKind::DocsCheck`.

It builds a `LocalCheckProcessRequest` using:

- explicit npm executable path;
- fixed arguments: `run`, `check:docs`;
- repository root working directory;
- sanitized environment containing `PATH`;
- optional validated `NPM_CONFIG_CACHE`;
- bounded timeout from the contract.

Secret-like environment keys or values fail closed without leaking values.

## 7. Output And Failure Summary

The handler maps process output through `LocalCheckResult`.

Behavior:

- zero exit maps to `passed`;
- non-zero exit maps to `failed`;
- timeout maps to `timed_out` with stable error code `local_check.handler.timed_out`;
- secret-like stdout/stderr fails closed with stable code `local_check.output.secret_like`;
- raw stdout/stderr are not persisted;
- debug output remains redaction-safe.

## 8. Runtime Boundary Summary

The handler runs only when explicitly registered in tests or narrow internal construction.

Confirmed boundaries:

- no default registry entry;
- no CLI surface;
- no workflow schema surface;
- normal executor skill events only when explicitly registered;
- no post-terminal events;
- no report artifacts;
- no automatic check execution.

## 9. Test Coverage Summary

Added or updated tests for:

- `DocsCheck` built-in model-only contract;
- unsupported handler command kind rejection;
- secret-like npm cache path rejection;
- handler debug redaction;
- injected runner success mapping;
- injected runner non-zero exit mapping;
- injected runner timeout mapping;
- secret-like stdout/stderr rejection;
- non-default executor behavior;
- explicit executor registration behavior;
- no report artifacts on explicit execution.

Focused tests passed:

- `cargo test -p workflow-core --test local_check`
- docs-check filtered local executor tests.

## 10. Commands Run And Results

Validation commands for this phase:

- `cargo test -p workflow-core --test local_check`
  - Passed.
- Filtered docs-check executor tests
  - Passed.
- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 11. Remaining Known Limitations

- `DocsCheck` remains test-scoped and explicitly constructed.
- No production handler registration exists.
- No default registry exists.
- No production sandbox exists.
- Normal tests use injected runners rather than real npm execution.
- Npm cache/write behavior remains explicitly constrained and not a production side-effect model.
- `LocalCheckResult` is not attached to evidence or work reports.
- Broader cargo/npm check families remain deferred.

## 12. Recommended Next Phase

Recommended next phase: **DocsCheck local handler implementation review**.

The review should verify the handler remains explicit, non-default, non-shell, redaction-safe, non-writing, and free of CLI, schema, artifact, evidence, side-effect, or release-posture expansion.
