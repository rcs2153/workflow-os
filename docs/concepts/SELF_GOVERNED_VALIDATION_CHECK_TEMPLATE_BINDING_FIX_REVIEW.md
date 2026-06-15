# Self-Governed Validation/Check Template Binding Fix Review

Review date: 2026-06-14

## 1. Executive Verdict

Template binding fix accepted; proceed to test-only local check handler planning.

The fix closes the pre-execution blocker identified in the contract model review. Each `LocalCheckCommandKind` is now bound to a canonical executable and argument template, mismatches fail closed, and command execution remains unauthorized.

## 2. Governance Run

This review was governed by the self-governance dogfood workflow before the review document was written.

- State directory: `/tmp/workflow-os-self-governance-state.fWAPQy`
- Run ID: `run-1781497828582818000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781497828582818000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Verification

The phase stayed within the approved model-only blocker-fix scope.

Implemented:

- canonical executable/argument templates for each `LocalCheckCommandKind`;
- validation that rejects mismatched executable/argument definitions;
- stable non-leaking error code `local_check.command_template.mismatch`;
- focused tests for canonical templates and mismatch rejection;
- documentation updates and an end-of-fix report.

No accidental implementation found for:

- local command execution;
- real local build/check skill handlers;
- arbitrary shell execution;
- executor integration;
- CLI exposure;
- workflow schema changes;
- example updates;
- automatic check execution;
- automatic report generation;
- automatic report artifact writing;
- side-effect boundary implementation;
- writes;
- provider calls or live adapter execution;
- recursive agents or agent swarms;
- hosted or distributed runtime behavior;
- production self-hosting;
- release posture changes.

## 4. Original Blocker Restatement

The contract model review found that `LocalCheckCommandKind` was not yet bound to a canonical executable/argument template.

That meant a model-only `DocsCheck` contract could carry a different safe-looking executable and argument vector. This was acceptable while all execution postures failed closed, but it had to be fixed before any `AllowlistedHandlerOnly` or real handler phase could be considered.

## 5. Fix Approach Assessment

The implementation adds an internal `LocalCheckCommandTemplate` returned by `LocalCheckCommandKind::template()`.

The approach is minimal and idiomatic for the current model:

- templates are static data;
- callers cannot mutate the template table;
- validation compares executable and argument vector exactly;
- validation remains inside `LocalCheckCommandContract::validate()`;
- the public API shape is unchanged;
- no execution, handler, CLI, schema, or runtime behavior is introduced.

This is compatible with future handler planning because handlers can later rely on the validated contract kind and template match before deciding whether a separately reviewed execution posture is allowed.

## 6. Template Coverage Assessment

Canonical templates exist for all planned command kinds:

- `WorkflowOsValidateDogfood`;
- `DocsCheck`;
- `CargoFmtCheck`;
- `CargoClippyWorkspace`;
- `CargoTestWorkspace`;
- `TypeScriptCheck`;
- `ContractCheck`;
- `IntegrationCheck`.

The test `all_planned_command_kinds_bind_to_canonical_templates` covers every current command kind and validates the expected executable/argument vector for each one.

The templates match the documented inventory in the self-governed validation/check plan. This does not mean all templates are ready to execute; `cargo`, `npm`, and broader integration checks still require side-effect, output-capture, and handler review before execution.

## 7. Validation Boundary Assessment

Validation now checks:

- model-only execution posture;
- executable token safety;
- argument token safety;
- canonical executable/argument template match;
- environment variable bounds and shape;
- output directory safety;
- timeout bounds;
- output capture bounds;
- raw output persistence rejection;
- side-effect classification;
- duplicate citation kind rejection.

Mismatched executable and mismatched argument vectors fail with `local_check.command_template.mismatch`. The error is stable and does not echo caller-supplied command text.

The ordering is appropriate: token safety validation still catches shell metacharacters, whitespace, oversized tokens, and secret-like tokens before template mismatch validation.

## 8. Execution Posture Assessment

Command execution remains deferred.

`LocalCheckCommandContract::validate()` still rejects every posture except `LocalCheckExecutionPosture::ModelOnly` with `local_check.execution.deferred`.

No `std::process::Command`, handler registration, executor path, CLI command, state backend behavior, workflow event, audit event, or report artifact behavior was added.

## 9. Privacy And Redaction Assessment

The fix does not introduce new payload storage.

Positive findings:

- template mismatch errors do not include raw executable or argument text;
- Debug output still redacts command IDs, executable text, arguments, environment names, and output directories;
- invalid serialized payloads still fail closed through validated deserialization;
- the model does not store raw command output, command transcripts, provider payloads, spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values.

No new leakage path was found.

## 10. Test Quality Assessment

The tests are sufficient for this model-only fix.

Covered:

- all planned command kinds are representable;
- all planned command kinds bind to canonical templates;
- mismatched executable fails closed without leaking the value;
- mismatched arguments fail closed without leaking the value;
- premature `AllowlistedHandlerOnly` remains rejected;
- whitespace tokens are rejected without leakage;
- too many arguments are rejected;
- too many environment variables are rejected;
- timeout upper bound is rejected;
- existing local-check validation, serde, and Debug non-leakage tests still pass.

Non-blocking gaps before real execution planning:

- output-directory path validation remains broad and should be tightened before handlers can write caches or artifacts;
- command templates do not yet declare permitted output directories per command kind;
- templates do not yet declare handler-specific environment allowlists;
- there is no test proving a future execution-capable posture can only validate after separate review because such a posture is intentionally still rejected.

These are not blockers for accepting the template binding fix.

## 11. Documentation Review

Docs correctly state:

- the contract model exists;
- canonical command-template binding is implemented;
- real local validation/check skill handlers are not implemented;
- command execution remains unsupported;
- CLI exposure is not implemented;
- workflow schema changes are not implemented;
- automatic check execution is not implemented;
- side-effect boundary implementation is not implemented;
- writes remain unsupported;
- recursive agents, agent swarms, hosted execution, and production self-hosting are not claimed.

One minor documentation follow-up: the runtime integration options table still says a test-only handler is preferred after contract model review. The intent is clear from the surrounding updated implementation sequence, but this can be tightened to say after template binding review.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Tighten the runtime integration options wording to say the test-only handler comes after template binding review.
- Define permitted output directory semantics per command kind before any handler execution.
- Define handler-specific environment allowlists before any handler execution.
- Decide whether the first real check should be `WorkflowOsValidateDogfood` or `DocsCheck`.
- Decide whether early dogfood checks require approval every time.
- Decide whether local check results first become work report text, validation/test evidence, or command-output evidence under a separately reviewed policy.

## 14. Recommended Next Phase

Recommended next phase: **test-only local check handler planning**.

The plan should choose one low-risk allowlisted command, likely `WorkflowOsValidateDogfood` or `DocsCheck`, and define the execution boundary before implementation:

- no arbitrary shell;
- no user-supplied command text;
- fixed executable/argument vector from the validated template;
- sanitized environment;
- repository-root working directory policy;
- network disabled;
- bounded timeout;
- bounded stdout/stderr capture;
- redaction before diagnostics, evidence, or report text;
- no raw output persistence;
- no CLI exposure;
- no workflow schema changes;
- no automatic check execution;
- no source writes.

Still do not build:

- real command execution in this review phase;
- broad local skill handler discovery;
- CLI handler exposure;
- workflow schema fields;
- automatic check execution;
- side-effect boundary implementation;
- writes;
- automatic report generation;
- automatic artifact writing;
- recursive agents or agent swarms;
- production self-hosting behavior.

## 15. Validation

Review validation commands:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Results are recorded in the final review report.
