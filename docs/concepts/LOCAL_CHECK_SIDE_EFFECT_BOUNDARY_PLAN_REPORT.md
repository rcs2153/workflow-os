# Local Check Side-Effect Boundary Plan Report

Report date: 2026-06-16

## 1. Executive Summary

Created a planning-only local check side-effect boundary document for Workflow OS. The plan defines how local validation/check commands should classify source reads, cache writes, build outputs, temp writes, network posture, environment posture, and source-write denial before live docs checks or broader cargo/npm handlers are considered.

No runtime behavior, code, live command execution, default registration, CLI exposure, schema fields, evidence attachment, persistence, report artifacts, writes, or release posture changes were introduced.

## 2. Scope Completed

- Added [Local Check Side-Effect Boundary Plan](../implementation-plans/local-check-side-effect-boundary-plan.md).
- Classified current and candidate local check command families.
- Documented directory, environment, network, runtime, report, and evidence boundaries.
- Defined future test expectations for a model-only local check side-effect boundary.
- Updated roadmap and local check planning docs to point at the new plan.
- Preserved current dogfood DocsCheck implementation posture as explicit, injected-runner-tested, and non-default.

## 3. Scope Explicitly Not Completed

- No side-effect boundary model code.
- No live npm smoke test.
- No cargo, TypeScript, contract, integration, or provider check handler.
- No default handler registration.
- No CLI exposure.
- No workflow schema fields.
- No automatic check execution.
- No command-output evidence.
- No local check evidence attachment.
- No local check result persistence.
- No report artifact auto-writing.
- No generic side-effect records.
- No writes.
- No release posture change.

## 4. Planning Boundary Summary

The plan distinguishes local check side-effect boundaries from the broader future side-effect boundary ADR. Local checks need an earlier, narrower policy because command execution may touch local caches or build outputs even when source files are not modified.

The plan recommends keeping local checks fail-closed when side effects are unclassified, and treating source-write support as rejected for local check phases.

## 5. Command Classification Summary

- `WorkflowOsValidateDogfood`: source-read-only, no cache/build writes expected.
- `DocsCheck`: source-read-only plus explicit npm cache allowance before live execution.
- `CargoFmtCheck`: deferred until toolchain/cache behavior is declared.
- `CargoClippyWorkspace` and `CargoTestWorkspace`: deferred until build-output policy is accepted.
- TypeScript, contract, and integration checks: deferred.
- Live provider smokes and arbitrary commands: rejected for local check v1.

## 6. Privacy And Redaction Summary

The plan keeps environment and path handling conservative:

- no inherited ambient environment;
- no provider or registry credentials;
- no secret-like environment names or values;
- no raw command output, parser payloads, provider payloads, source snippets, or credentials in errors;
- cache/build/temp paths must be bounded and redaction-safe.

## 7. Test Coverage Summary

This was a planning-only phase, so no Rust tests were added.

The future test plan covers side-effect class representation, fail-closed unclassified checks, source-write rejection, explicit cache/build directory requirements, secret-like path rejection, environment allowlist validation, network-disabled posture, docs-check cache allowance, and redaction-safe errors.

## 8. Commands Run And Results

- `cargo fmt --all --check`
  - Passed during the preceding implementation review in this turn.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed during the preceding implementation review in this turn.
- `cargo test --workspace`
  - Passed during the preceding implementation review in this turn.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 9. Remaining Known Limitations

- The plan does not yet decide whether to extend `LocalCheckSideEffectClass` or add a separate boundary model.
- No source-write detection design is accepted.
- No live docs-check smoke posture is accepted.
- Cargo and npm build/cache policies remain deferred beyond docs-check cache posture.
- The generic side-effect boundary ADR remains separate future work for write-capable adapters.

## 10. Recommended Next Phase

Recommended next phase: **local check side-effect boundary plan review**.

The review should verify that the plan is narrow enough for local checks, does not overclaim runtime safety, and provides a clear model-only implementation path before any live check execution or broader handler family is added.
