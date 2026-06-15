# DocsCheck Registry Helper Review

Review date: 2026-06-15

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

`LocalSkillRegistry::register_docs_check_handler(...)` is a narrow explicit wiring helper. It registers a caller-supplied `DocsCheckLocalHandler` for the canonical `local/check-docs` `v0` skill without becoming default registration or constructing execution context.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved explicit-helper scope.

Confirmed in scope:

- explicit `LocalSkillRegistry::register_docs_check_handler(...)`;
- executor tests updated to use the helper;
- roadmap and planning documentation updates;
- implementation report.

No accidental implementation was found for:

- default handler registration;
- CLI exposure;
- workflow schema fields;
- automatic local check execution;
- `AllowlistedHandlerOnly` enablement;
- npm path resolution;
- repository-root inference;
- cache directory creation;
- ambient environment reads;
- report artifact writing;
- evidence attachment;
- work-report citation integration;
- side-effect boundary implementation;
- source writes;
- broader cargo/npm command handlers;
- release posture changes.

## 3. Governance Check

This review was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-docs-check-registry-helper-review`
- Run ID: `run-1781506407756170000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781506407756170000-2/d`
- Final status: `Completed`

Inspection confirmed the expected event history through `RunCompleted`.

## 4. Helper API Assessment

The helper API is appropriately small.

Implemented API:

- `LocalSkillRegistry::register_docs_check_handler(&mut self, DocsCheckLocalHandler) -> Result<(), WorkflowOsError>`

Verified:

- requires a prebuilt `DocsCheckLocalHandler`;
- registers only `local/check-docs`;
- registers only version `v0`;
- returns a structured error only if the built-in skill ID or version becomes invalid;
- does not introduce a new registry type or runtime configuration object.

The helper improves wiring ergonomics without broadening handler discovery.

## 5. Default Registration Assessment

Default registration remains deferred.

Verified:

- `LocalSkillRegistry::new()` remains empty;
- missing `DocsCheck` handler behavior remains the existing failed-run path;
- no default registry entry is added;
- no CLI path creates a registry with `DocsCheckLocalHandler`;
- no workflow schema field implies handler registration.

This preserves existing local executor authority boundaries.

## 6. Runtime And Event Boundary Assessment

The runtime boundary remains clean.

Verified:

- explicit helper registration uses existing `LocalExecutor` and `SkillHandler` mechanics;
- no new runtime event kinds are added;
- no post-terminal events are appended;
- persisted events match returned run events in existing executor coverage;
- no StateBackend writes occur outside normal explicit executor behavior;
- no report artifacts are created;
- no CLI output path is added.

## 7. Command Authority Assessment

The helper does not change command authority.

Verified:

- it does not construct `DocsCheckLocalHandler`;
- it does not resolve npm paths;
- it does not infer repository roots;
- it does not create cache directories;
- it does not read environment variables;
- it does not pass credentials;
- it does not run npm;
- it does not enable `AllowlistedHandlerOnly`;
- it does not register cargo, TypeScript, contract, or integration handlers.

Command authority remains inside explicit handler construction and invocation.

## 8. Privacy And Redaction Assessment

The helper introduces no new data-storage or redaction surface.

Verified:

- no raw command output is stored;
- no docs contents are copied;
- no parser payloads are copied;
- no provider payloads are copied;
- no environment values are copied;
- no npm tokens, registry credentials, authorization headers, private keys, or token-like strings are copied;
- no new debug type is introduced.

Existing `DocsCheckLocalHandler` and `LocalCheckResult` redaction behavior remains the relevant boundary.

## 9. Test Quality Assessment

The tests are sufficient for this narrow helper phase.

Covered:

- `LocalSkillRegistry::new()` remains empty/default-safe through the existing missing-handler test;
- explicit helper registration executes through `LocalExecutor`;
- generated process request still uses `npm run check:docs`;
- no report artifacts are written;
- existing local check, executor, report, evidence, validation, adapter, and runtime suites pass.

Non-blocking test follow-ups:

- add a more direct unit test for the helper once the registry exposes safe inspection or a test-only query method;
- add explicit coverage that the helper does not read ambient environment if a future helper accepts construction inputs;
- keep real npm smoke tests separate and opt-in until cache/write sandboxing is reviewed.

## 10. Documentation Review

Docs were updated honestly.

Confirmed:

- default registration remains documented as deferred;
- the explicit registry helper is documented as implemented;
- CLI exposure remains unimplemented;
- workflow schema fields remain unimplemented;
- automatic local check execution remains unimplemented;
- `AllowlistedHandlerOnly` remains disabled;
- report artifacts remain unimplemented;
- evidence attachment remains unimplemented;
- side-effect boundary modeling remains unimplemented;
- source writes remain unsupported.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add direct helper inspection tests if a safe registry query API becomes necessary.
- Plan local check result identity and citation before WorkReport integration.
- Plan command-output evidence policy before any `EvidenceKind::CommandOutput` usage.
- Plan npm cache/write sandboxing before default registration or real npm smoke tests enter normal validation.
- Plan `AllowlistedHandlerOnly` authorization separately before serialized contracts can imply handler execution.

## 13. Recommended Next Phase

Recommended next phase: **local check result citation planning**.

The registry helper completes safe explicit wiring for `DocsCheck`. The next low-risk roadmap step should decide how local check outcomes become stable references for WorkReports or evidence without storing raw command output and without enabling default registration, CLI behavior, schema fields, side-effect modeling, or writes.

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
