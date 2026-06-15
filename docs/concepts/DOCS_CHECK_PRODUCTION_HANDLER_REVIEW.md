# DocsCheck Production-Shaped Handler Review

Review date: 2026-06-15

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

`DocsCheckLocalHandler` gives the `DocsCheck` path a production-shaped explicit handler name without expanding runtime authority. The handler remains explicit-only, non-default, non-CLI, non-schema, non-artifact, non-evidence, and non-writing.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved production-shaped explicit-handler scope.

Confirmed in scope:

- `DocsCheckLocalHandler`;
- `TestOnlyDocsCheckHandler` compatibility alias;
- exported handler type;
- tests updated to exercise `DocsCheckLocalHandler` directly;
- documentation and phase report updates.

No accidental implementation was found for:

- default handler registration;
- CLI exposure;
- workflow schema fields;
- automatic local check execution;
- `AllowlistedHandlerOnly` enablement;
- real npm smoke tests in normal validation;
- report artifact writing;
- evidence attachment;
- work-report citation integration;
- side-effect boundary implementation;
- source writes;
- broader cargo/npm command families;
- release posture changes.

## 3. Governance Check

This review was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-docs-check-production-handler-review`
- Run ID: `run-1781505581194459000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781505581194459000-2/d`
- Final status: `Completed`

Inspection confirmed the expected event history through `RunCompleted`.

## 4. API And Naming Assessment

The new API shape is appropriate for the phase.

Implemented:

- `DocsCheckLocalHandler`;
- `DocsCheckLocalHandler::new(...)`;
- `DocsCheckLocalHandler::new_with_process_runner(...)`;
- `TestOnlyDocsCheckHandler` as a compatibility alias.

This removes the misleading impression that the handler can only exist in tests while preserving explicit construction and deterministic test injection. The compatibility alias avoids unnecessary churn for any callers that still use the original test-scoped name.

The handler is exported from `workflow-core`, which is acceptable for this phase because the export remains explicit and no default registry, CLI, or schema path exposes it. Public API stabilization should still be reviewed before release-hardening.

## 5. Command Authority Assessment

Command authority remains bounded.

Verified:

- the handler accepts only `LocalCheckCommandKind::DocsCheck`;
- the canonical contract remains `LocalCheckCommandContract::docs_check_model_only()`;
- the command remains fixed to `npm run check:docs`;
- no shell invocation is introduced;
- no caller-supplied command text is accepted;
- no caller-supplied extra arguments are appended;
- unsupported command kinds fail closed with `local_check.handler.unsupported_kind`;
- serialized `AllowlistedHandlerOnly` remains rejected elsewhere in the local check contract model.

The implementation did not turn workflow specs into execution authority.

## 6. Environment, Toolchain, And Cache Assessment

The environment posture remains conservative.

Verified:

- the handler requires an explicit npm executable path;
- the handler validates that the executable path is a file;
- repository root is validated by requiring `package.json` and `scripts/check-docs.mjs`;
- the process environment remains sanitized and minimal;
- optional `NPM_CONFIG_CACHE` is explicitly supplied and validated;
- secret-like cache paths fail closed without leaking values;
- ambient npm tokens, provider credentials, registry credentials, authorization headers, and private keys are not inherited.

No production cache sandbox was added, which is correct for this phase.

## 7. Runtime Boundary Assessment

Runtime behavior remains unchanged.

Verified:

- no default registry entry exists;
- empty registry behavior still returns the existing missing-handler failed run;
- explicit registration executes through the existing `LocalExecutor` and `SkillHandler` mechanics;
- persisted backend events match returned run events;
- no new runtime event kind is added;
- no post-terminal events are appended;
- no report artifacts are created;
- no CLI output path is added.

## 8. Output And Redaction Assessment

Output handling remains safe.

Verified:

- process output still flows through `LocalCheckResult`;
- success, non-zero exit, timeout, and secret-like stdout/stderr behavior are unchanged;
- `DocsCheckLocalHandler` debug output redacts local paths, cache paths, command arguments, and runner details;
- raw stdout/stderr are not persisted;
- command transcripts are not stored;
- docs contents, parser payloads, provider payloads, tokens, credentials, and private keys are not copied into durable model surfaces.

## 9. Test Quality Assessment

The updated tests are sufficient for this narrow phase.

Covered:

- direct construction through `DocsCheckLocalHandler`;
- unsupported command-kind rejection;
- secret-like cache path rejection;
- debug redaction under the production-shaped name;
- injected-runner success, failure, timeout, and secret-like output behavior;
- explicit executor registration;
- no default registration;
- command argument and environment capture;
- event-log preservation;
- absence of work report artifacts;
- existing local check, executor, report, evidence, validation, adapter, and runtime suites.

Remaining gaps are non-blocking because they remain out of scope:

- no real npm smoke test;
- no production cache/write sandbox test;
- no default-registration test beyond confirming absence;
- no local check result evidence or WorkReport citation tests.

## 10. Documentation Review

Docs were updated honestly.

Confirmed:

- `DocsCheckLocalHandler` is documented as implemented;
- `TestOnlyDocsCheckHandler` is documented as a compatibility alias;
- docs continue to state that default registration is not implemented;
- docs continue to state that CLI exposure is not implemented;
- docs continue to state that workflow schema fields are not implemented;
- docs continue to state that automatic execution is not implemented;
- docs continue to state that report artifacts, evidence attachment, side-effect boundary modeling, source writes, and release posture changes are not implemented.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Review public API stabilization for `DocsCheckLocalHandler`, `LocalCheckProcessRunner`, `LocalCheckProcessRequest`, and `LocalCheckProcessOutput` before release hardening.
- Plan whether and how default registration should exist.
- Plan whether `AllowlistedHandlerOnly` can become valid for reviewed local check contracts.
- Define production npm cache/write sandbox policy before treating real npm execution as fully supported runtime behavior.
- Plan local check result citation in WorkReports separately.
- Plan local check evidence separately, especially before any `CommandOutput` evidence usage.

## 13. Recommended Next Phase

Recommended next phase: **DocsCheck default-registration planning**.

The handler is now production-shaped but still explicit-only. The next roadmap decision should be whether `DocsCheck` should ever be registered by default, and if so under what authorization, cache, side-effect, CLI, schema, and report/evidence boundaries.

## 14. Validation

Validation commands run for this review:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
