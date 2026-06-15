# DocsCheck Local Handler Review

Review date: 2026-06-15

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The explicit, test-scoped `DocsCheck` local handler satisfies the approved phase boundary. It is non-default, non-shell, explicitly constructed, redaction-safe, and exercised through deterministic injected-runner tests plus executor integration tests.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved implementation scope.

Confirmed in scope:

- `DocsCheck` model-only local check contract;
- explicit `TestOnlyDocsCheckHandler`;
- injected process-runner path for deterministic tests;
- bounded, validated local check result mapping;
- executor tests for non-default and explicit-registration behavior;
- documentation and phase report updates.

No accidental implementation was found for:

- production handler registration;
- default handler registration;
- CLI behavior;
- workflow schema fields;
- automatic local check execution;
- report artifact writing;
- persistence;
- evidence attachment;
- command-output evidence;
- side-effect boundary implementation;
- source writes;
- broader cargo/npm command families;
- live provider access;
- release posture changes.

## 3. Governance Check

This review was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-docs-check-handler-review`
- Run ID: `run-1781504138246743000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781504138246743000-2/d`
- Final status: `Completed`

Inspection confirmed the expected event history through `RunCompleted`.

## 4. Handler API Assessment

The handler API is appropriately narrow for this phase.

Implemented surface:

- `LocalCheckCommandContract::docs_check_model_only()`;
- `TestOnlyDocsCheckHandler::new(...)`;
- `TestOnlyDocsCheckHandler::new_with_process_runner(...)`;
- `LocalCheckCommandContract::allowed_environment_variables()`;
- actual local check kind reporting through `LocalCheckResult::to_skill_output()`.

The handler validates the supplied contract, accepts only `LocalCheckCommandKind::DocsCheck`, requires repository-root working directory policy, sanitized minimal environment, disabled network policy, and `NoSourceWrites` side-effect classification.

The injected-runner constructor is the right test boundary. The standard-runner constructor exists but remains explicitly constructed and is not registered by default.

## 5. Command Authority Assessment

Command authority remains bounded.

Verified:

- the built-in `DocsCheck` contract binds to executable `npm`;
- the fixed argument vector is `run`, `check:docs`;
- no shell invocation is introduced;
- no user-supplied command text is accepted;
- no caller-supplied extra arguments are appended;
- the handler rejects unsupported local check kinds;
- the process request is built from validated contract fields.

This preserves the intended allowlist model while still proving the first non-dogfood local check path.

## 6. Environment And Cache Assessment

The environment posture is conservative.

Verified:

- the handler starts from the existing sanitized minimal environment;
- `PATH` is fixed to a minimal local value;
- optional `NPM_CONFIG_CACHE` is explicitly supplied;
- environment values are validated before handler storage and before invocation;
- secret-like cache paths fail closed without leaking values;
- `NPM_TOKEN`, provider credentials, authorization headers, private keys, and ambient environment inheritance are not introduced.

The implementation does not create or manage a production npm cache. That remains a future production-posture decision.

## 7. Runtime And Event Boundary Assessment

The runtime boundary remains clean.

Verified:

- `DocsCheck` is not registered by default;
- an empty local skill registry yields the existing failed-run path for a missing handler;
- explicit registration executes through normal `LocalExecutor` skill events;
- no new runtime event kind is added;
- no post-terminal events are appended;
- persisted backend events match the returned run events;
- no work report artifacts are created;
- no CLI output path is added.

The generalized local check skill output preserves existing executor semantics while making the reported check kind accurate for `DocsCheck`.

## 8. Output And Redaction Assessment

Output handling remains redaction-safe.

Verified:

- process output is converted through `LocalCheckResult`;
- success maps to `passed`;
- non-zero exit maps to `failed`;
- timeout maps to `timed_out` with `local_check.handler.timed_out`;
- stdout/stderr summaries remain bounded by the local check result policy;
- secret-like stdout or stderr fails closed with `local_check.output.secret_like`;
- `TestOnlyDocsCheckHandler` debug output redacts local paths, cache paths, process runner details, and command arguments;
- raw command transcripts are not persisted;
- no docs content, parser payload, provider payload, or command output evidence is attached.

## 9. Test Quality Assessment

The tests are focused and meaningful.

Covered:

- `DocsCheck` built-in contract shape;
- model-only and non-executing posture;
- canonical executable and arguments;
- allowed `NPM_CONFIG_CACHE` declaration;
- unsupported command-kind rejection;
- secret-like cache path rejection;
- handler debug redaction;
- injected-runner success, failure, and timeout mappings;
- secret-like stdout/stderr rejection;
- non-default executor registration behavior;
- explicit executor registration behavior;
- output reference shape;
- process request argument and environment capture;
- event-log preservation;
- absence of work report artifacts.

Remaining test gaps are non-blocking because they are outside the approved phase:

- no real npm smoke test;
- no production sandbox/cache write test;
- no production default-registration test;
- no evidence or work-report citation test for local check results.

## 10. Documentation Review

Docs were updated honestly.

Confirmed:

- the plan status says explicit test-scoped `DocsCheck` handler is implemented;
- the phase report records the implementation and validation results;
- docs continue to state that production handler registration is not implemented;
- docs continue to state that default registration, CLI exposure, schema fields, automatic execution, report artifacts, evidence attachment, side-effect modeling, source writes, and release posture changes are not implemented.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Reassess whether `TestOnlyDocsCheckHandler` should remain publicly exported before any production API stabilization.
- Plan production/default registration posture separately before enabling `DocsCheck` outside explicit construction.
- Decide whether `AllowlistedHandlerOnly` should become valid for reviewed local check contracts.
- Define production npm cache/write sandbox policy before real npm execution is treated as supported runtime behavior.
- Plan local check result citation or WorkReport integration separately, without storing raw command output.

## 13. Recommended Next Phase

Recommended next phase: **DocsCheck local handler production-posture planning**.

The implementation is safe as an explicit, test-scoped handler. The next decision should be whether and how `DocsCheck` can move toward a production posture: default registration, allowlisted execution posture, cache/write sandboxing, and eventual result citation all need explicit planning before more code expands this path.

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
