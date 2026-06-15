# DocsCheck Production-Shaped Handler Report

Report date: 2026-06-15

## 1. Executive Summary

Workflow OS now has an explicit production-shaped `DocsCheckLocalHandler`.

The handler keeps the same conservative boundary as the reviewed test-scoped handler: explicit construction only, no default registration, no CLI exposure, no workflow schema fields, no automatic check execution, no report artifacts, no evidence attachment, no side-effect boundary implementation, no source writes, and no release posture change.

## 2. Governance Run

This implementation phase was governed by the self-governance dogfood workflow before code changes.

- State directory: `/tmp/workflow-os-docs-check-production-handler-impl`
- Run ID: `run-1781505130620275000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781505130620275000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Completed

- Added `DocsCheckLocalHandler`.
- Kept `TestOnlyDocsCheckHandler` as a compatibility alias.
- Updated tests to exercise `DocsCheckLocalHandler` directly.
- Preserved explicit process-runner injection for deterministic tests.
- Preserved the existing canonical `DocsCheck` command contract.
- Preserved non-default executor registration behavior.
- Updated roadmap and planning docs.

## 4. Scope Explicitly Not Completed

- No default handler registration.
- No CLI exposure.
- No workflow schema fields.
- No automatic local check execution.
- No `AllowlistedHandlerOnly` enablement.
- No real npm smoke test in normal validation.
- No report artifact writing.
- No local check evidence attachment.
- No local check work-report citation integration.
- No side-effect boundary implementation.
- No source writes.
- No broader cargo/npm command families.
- No release posture change.

## 5. Handler API Summary

New production-shaped handler:

- `DocsCheckLocalHandler`

Constructors:

- `new(...)`
- `new_with_process_runner(...)`

Compatibility alias:

- `TestOnlyDocsCheckHandler`

The handler remains explicit-only. It is not registered by default and is not exposed through CLI or workflow schemas.

## 6. Command Authority Summary

The handler still accepts only `LocalCheckCommandKind::DocsCheck`.

The command remains fixed to:

- executable: `npm`
- arguments: `run`, `check:docs`

The handler does not invoke a shell, accept caller-supplied command text, append caller-supplied arguments, or broaden into other npm/cargo commands.

## 7. Environment And Cache Summary

The handler continues to use a sanitized minimal environment.

Allowed values:

- fixed minimal `PATH`;
- optional validated `NPM_CONFIG_CACHE`.

Forbidden values remain excluded:

- inherited ambient environment;
- npm tokens;
- provider credentials;
- registry credentials;
- authorization headers;
- private keys;
- secret-like environment names or values.

Cache posture remains explicit and limited. A broader production cache/write sandbox remains deferred.

## 8. Runtime Boundary Summary

Runtime behavior remains unchanged:

- no default registration;
- no automatic execution;
- no CLI behavior;
- no workflow schema fields;
- no post-terminal events;
- no report artifacts;
- no runtime state mutation beyond normal explicit executor runs in tests.

Explicit tests may register the handler with `LocalExecutor` to prove existing event and output behavior.

## 9. Redaction And Privacy Summary

The handler continues to route process output through `LocalCheckResult`.

Rules preserved:

- bounded stdout/stderr summaries only;
- secret-like stdout/stderr fail closed;
- handler debug output redacts local paths, cache paths, and process runner details;
- raw command output is not persisted;
- command transcripts, docs contents, parser payloads, provider payloads, tokens, and credentials are not copied into durable model surfaces.

## 10. Test Coverage Summary

Updated tests cover:

- production-shaped handler construction through `DocsCheckLocalHandler`;
- unsupported command-kind rejection;
- secret-like cache path rejection;
- debug redaction;
- injected-runner success, failure, timeout, and secret-like output behavior;
- explicit executor registration;
- non-default registration behavior;
- absence of report artifacts.

Existing local check, executor, report, evidence, validation, adapter, and runtime tests remain part of the validation gate.

## 11. Commands Run And Results

Validation commands for this phase:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 12. Remaining Known Limitations

- `DocsCheckLocalHandler` remains explicit-only.
- No default handler registry exists.
- No CLI exposure exists.
- No workflow schema field exists.
- No `AllowlistedHandlerOnly` execution posture is enabled.
- No production sandbox or side-effect boundary exists.
- Real npm execution remains outside normal tests.
- Local check results are not yet cited in evidence or work reports.

## 13. Recommended Next Phase

Recommended next phase: **DocsCheck production-shaped handler review**.

The review should verify that the handler naming change did not accidentally expand authority, registration, CLI behavior, schema exposure, artifact writing, evidence attachment, source writes, or release posture.
