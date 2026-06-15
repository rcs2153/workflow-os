# Self-Governed Validation/Check Contract Model Review

Review date: 2026-06-14

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The local validation/check command contract model is narrow, deterministic, redaction-aware, and non-executing. It is acceptable as a model-only foundation. It must not be used to execute commands until the follow-ups below are addressed and a separate handler plan is reviewed.

## 2. Governance Run

This review was governed by the self-governance dogfood workflow before the review document was written.

- State directory: `/tmp/workflow-os-self-governance-state.wwg9FL`
- Run ID: `run-1781496364358379000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781496364358379000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Verification

The phase stayed within the approved model-only scope.

Implemented:

- local validation/check command contract model;
- command ID and command kind vocabulary;
- model-only execution posture;
- working-directory, environment, network, side-effect, output-capture, redaction, and result-status vocabulary;
- validation and serde behavior;
- focused tests and documentation updates.

No accidental implementation found for:

- local command execution;
- real build/check skill handlers;
- arbitrary shell execution;
- CLI exposure;
- workflow schema changes;
- example updates;
- automatic runtime report generation;
- automatic report artifact writing;
- side-effect boundary implementation;
- writes;
- provider calls or live adapter execution;
- recursive agents or agent swarms;
- hosted or distributed runtime behavior;
- production self-hosting;
- release posture changes.

## 4. Model Assessment

The model is appropriately small for the phase.

Positive findings:

- `LocalCheckCommandContract` is private-field and constructor-validated.
- `LocalCheckCommandId` uses validated typed IDs.
- `LocalCheckCommandKind` captures the planned check vocabulary without adding execution.
- `LocalCheckExecutionPosture::ModelOnly` is enforced by validation.
- The output capture policy rejects raw output persistence.
- The side-effect class requires classification before future execution is considered.
- `LocalCheckResultStatus` is vocabulary only and does not imply runtime status changes.
- The model is exported consistently from `workflow-core`.

The model does not introduce a runtime integration point, executor mutation, event emission, state backend behavior, CLI surface, or schema contract.

## 5. Command Authority Assessment

The command authority boundary is safe for model-only use.

The model rejects:

- shell metacharacters;
- whitespace inside command tokens;
- empty executable or argument tokens;
- too many arguments;
- oversized command tokens;
- secret-like executable, argument, environment, and output-directory values.

The implementation does not invoke commands and does not use `std::process::Command`.

Non-blocking but required before execution: `LocalCheckCommandKind` should be bound to a canonical executable/argv template before any real handler runs. Today a valid `DocsCheck` contract can carry another safe-looking executable and argument vector as long as the execution posture remains `ModelOnly`. That is acceptable while execution is rejected, but it must be tightened before `AllowlistedHandlerOnly` or any handler execution is accepted.

## 6. Execution Posture Assessment

The implementation correctly keeps command execution deferred.

`LocalCheckCommandContract::validate()` rejects any posture other than `ModelOnly` with stable code `local_check.execution.deferred`. This is the right fail-closed behavior for this phase.

No handler registration, CLI command, executor path, or state backend write uses the contract.

## 7. Side-Effect Assessment

The side-effect model is sufficient as vocabulary for this phase.

The contract can represent:

- no source writes;
- build/cache writes;
- unclassified side effects.

Validation rejects `Unclassified`, which prevents a future execution path from treating ambiguous commands as ready. This is conservative.

Remaining design before execution:

- define permitted output directory semantics per command kind;
- define whether `target/`, npm caches, and toolchain caches are allowed;
- decide whether cleanup is part of the handler contract;
- decide whether build/cache writes require a broader side-effect boundary model.

## 8. Output And Redaction Assessment

The output boundary is safe for the model-only phase.

Positive findings:

- stdout and stderr capture bounds must be non-zero;
- bounds are capped at 64 KiB each;
- raw output persistence is rejected;
- redaction policy is bounded summary only;
- Debug output hides executable text, argument text, environment names, output directories, and command IDs;
- invalid serialized secret-like values fail closed without echoing the raw value.

Before real handlers, output capture needs a concrete redaction implementation and tests against realistic command output. The current model only defines the policy.

## 9. Serde And Compatibility Assessment

Serde behavior is appropriate for an internal model foundation.

Valid contracts serialize and deserialize through validated constructors. Invalid serialized contracts fail closed through `Deserialize`.

No public schema was added. Field names are sensible for future schema planning, but schema exposure should remain deferred until execution semantics and compatibility expectations are reviewed.

## 10. Relationship To Work Reports And Evidence

The model aligns with the existing evidence/report foundation without overreaching.

It references `WorkReportCitationKind` as future citation hooks and does not create `EvidenceReference` values. It does not attach command output evidence and does not store raw command output.

Before real handlers, the project still needs a scoped decision on whether local check results should produce `EvidenceKind::TestResult`, `EvidenceKind::ValidationResult`, or a command-output evidence reference. Command-output evidence should require a separate reviewed attachment boundary.

## 11. Test Quality Assessment

The tests cover the model-only risk surface well.

Covered:

- valid model-only contract;
- built-in dogfood validation contract;
- command kind vocabulary;
- result status vocabulary;
- premature execution posture rejection;
- shell metacharacter rejection;
- secret-like argument and environment rejection;
- raw output persistence rejection;
- output bounds;
- unclassified side-effect rejection;
- duplicate citation kind rejection;
- serde round trip;
- invalid serialized payload failure without leaking secret-like values;
- Debug non-leakage.

Missing but non-blocking before execution:

- canonical command-kind-to-template mismatch rejection;
- output-directory path edge cases such as nested safe paths versus sibling traversal variants;
- explicit test that whitespace in tokens is rejected after the final hardening;
- duplicate or excessive environment-variable edge cases;
- too many argument edge case;
- timeout too-large edge case.

These are not blockers for model-only acceptance, but the canonical template mismatch must be fixed before any execution-capable handler phase.

## 12. Documentation Review

Docs correctly state:

- the contract model is implemented;
- real local validation/check skill handlers are not implemented;
- the kernel does not run real build/check commands;
- command execution remains deferred;
- no CLI exposure exists;
- no side-effect boundary implementation exists;
- no automatic report generation or artifact writing is added;
- no recursive agent, agent swarm, or production self-hosting claim is made.

The phase report honestly discloses the non-execution boundary and remaining limitations.

## 13. Blockers

No blockers for accepting the model-only phase.

Blocking before any real command execution:

- bind each `LocalCheckCommandKind` to a canonical executable/argv template, or otherwise reject mismatched executable/argument definitions before `AllowlistedHandlerOnly` can validate.

## 14. Non-Blocking Follow-Ups

- Add tests for command-kind/template mismatch once canonical templates are introduced.
- Add tests for whitespace-token rejection.
- Add tests for too many arguments, too many environment variables, and timeout upper bound.
- Add more precise permitted-output-directory validation semantics.
- Decide whether `BuildOrCacheWrites` can proceed before the broader side-effect boundary exists.
- Decide whether local check result evidence uses validation/test result evidence before command-output evidence is considered.

## 15. Recommended Next Phase

Recommended next phase: **local validation/check command template binding fix**.

This should remain model-only. It should bind each `LocalCheckCommandKind` to a canonical executable and argument template and reject mismatched executable/argument pairs. After that fix is reviewed, the project can plan a test-only real local check handler for one low-risk allowlisted command.

Still do not build:

- real command execution;
- CLI handler exposure;
- workflow schema fields;
- automatic check execution;
- side-effect boundary implementation;
- writes;
- automatic report generation;
- automatic artifact writing;
- recursive agents or agent swarms;
- production self-hosting behavior.

## 16. Validation

Review validation commands:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Results are recorded in the final review report.
