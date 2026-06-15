# Self-Governed Validation/Check Template Binding Fix Report

Report date: 2026-06-14

## 1. Executive Summary

The local validation/check command contract model now binds every `LocalCheckCommandKind` to a canonical executable and argument template.

This closes the model-level gap identified in the contract model review: a caller can no longer construct a valid `DocsCheck` contract with another safe-looking executable or argument vector. The fix remains model-only and does not execute commands.

## 2. Governance Run

This phase was governed by the self-governance dogfood workflow before implementation.

- State directory: `/tmp/workflow-os-self-governance-state.AI2Km7`
- Run ID: `run-1781496781940809000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781496781940809000-2/d`
- Final status: `Completed`

Inspection confirmed event history through `RunCompleted`.

## 3. Scope Completed

- Added canonical executable and argument templates for every `LocalCheckCommandKind`.
- Added validation that rejects command-kind/template mismatches.
- Added stable non-leaking error code `local_check.command_template.mismatch`.
- Added focused tests for all canonical templates.
- Added focused tests for mismatched executable and mismatched arguments.
- Added additional boundary tests for whitespace tokens, excessive arguments, excessive environment variables, and timeout upper bound.
- Updated planning and review documentation with a fix-forward note.

## 4. Scope Explicitly Not Completed

- No local command execution.
- No real local build/check skill handlers.
- No arbitrary shell execution.
- No CLI exposure.
- No workflow schema changes.
- No example updates.
- No automatic check execution.
- No automatic report generation.
- No automatic report artifact writing.
- No side-effect boundary implementation.
- No writes.
- No provider calls or live adapter execution.
- No recursive agents or agent swarms.
- No hosted or distributed runtime behavior.
- No production self-hosting claim.
- No release posture change.

## 5. Implementation Approach

The implementation adds an internal canonical template for each command kind:

- `WorkflowOsValidateDogfood` -> `workflow-os --project-dir dogfood/workflow-os-self-governance validate`
- `DocsCheck` -> `npm run check:docs`
- `CargoFmtCheck` -> `cargo fmt --all --check`
- `CargoClippyWorkspace` -> `cargo clippy --workspace --all-targets -- -D warnings`
- `CargoTestWorkspace` -> `cargo test --workspace`
- `TypeScriptCheck` -> `npm run check:ts`
- `ContractCheck` -> `npm run check:contracts`
- `IntegrationCheck` -> `npm run check:integrations`

`LocalCheckCommandContract::validate()` now validates the executable token, argument tokens, and then the canonical template match. Mismatches fail closed without echoing caller-supplied command text.

## 6. Validation Boundary Summary

The contract model now validates:

- identifier shape;
- model-only execution posture;
- executable and argument token safety;
- canonical executable/argument template match;
- environment variable bounds and shape;
- output directory declarations;
- timeout bounds;
- output capture bounds;
- no raw output persistence;
- classified side effects;
- duplicate citation-kind rejection.

The fix keeps `LocalCheckExecutionPosture::AllowlistedHandlerOnly` rejected. Template binding prepares the model for a future reviewed handler phase but does not authorize execution.

## 7. Redaction And Privacy Summary

Template mismatch errors use stable codes and do not include raw executable or argument text. Debug output remains redacted for command IDs, executable text, argument text, environment names, and output directories.

The model still does not store raw command output, command transcripts, provider payloads, spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values.

## 8. Test Coverage Summary

Added or expanded focused tests for:

- every planned command kind validating against its canonical template;
- mismatched executable rejection without leaking caller-supplied text;
- mismatched arguments rejection without leaking caller-supplied text;
- whitespace token rejection without leakage;
- excessive argument count rejection;
- excessive environment variable count rejection;
- timeout upper-bound rejection.

Existing local-check model tests continue to cover model-only posture, shell metacharacter rejection, secret-like value rejection, output capture bounds, raw output persistence rejection, unclassified side-effect rejection, serde round trip, invalid serde failure, and Debug non-leakage.

## 9. Commands Run And Results

- Self-governance dogfood run and approval:
  - Passed; final status `Completed`.
- `cargo test -p workflow-core --test local_check`
  - Passed.

Full validation commands for the phase:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Results are recorded in the final implementation report.

## 10. Remaining Known Limitations

- The model does not execute commands.
- The model does not register local skill handlers.
- The model does not expose CLI behavior.
- The model does not write report artifacts.
- The model does not define workflow schema fields.
- The model does not enforce a real sandbox.
- Output capture and redaction remain policy definitions, not handler behavior.
- Side-effect boundary implementation remains future work.
- Command-output evidence attachment remains separately scoped future work.

## 11. Recommended Next Phase

Recommended next phase: **local validation/check command template binding fix review**.

The review should verify that canonical templates are complete, mismatched definitions fail closed, errors remain non-leaking, and command execution is still not authorized. After that review, the project can plan a test-only real local check handler for one low-risk allowlisted command.
