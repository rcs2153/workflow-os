# Local Check Side-Effect Boundary Plan Review

Review date: 2026-06-16

## 1. Executive Verdict

Plan accepted; proceed to local check side-effect boundary model implementation.

The plan is appropriately narrow and fills the immediate gap between the explicit injected-runner `DocsCheckLocalHandler` dogfood path and any future live npm smoke or broader cargo/npm local check handlers. It correctly treats local validation/check commands as potentially side-effecting even when they are intended to be source-read-only, and it keeps runtime execution, default registration, CLI exposure, schema fields, evidence attachment, persistence, report artifacts, writes, and release posture changes out of scope.

## 2. Scope Verification

The plan stayed within planning-only scope.

It did not authorize or implement:

- live local command execution;
- live npm smoke tests;
- cargo, TypeScript, contract, integration, or provider check handlers;
- true default handler registration;
- CLI handler exposure;
- workflow schema fields;
- automatic check execution;
- command-output evidence;
- local check evidence attachment;
- automatic report artifact writing;
- local check result persistence;
- generic side-effect records;
- write-capable adapters;
- source writes;
- provider calls;
- recursive agents;
- agent swarms;
- hosted or distributed runtime behavior;
- release posture changes.

## 3. Planning Boundary Assessment

The plan defines a conservative local-check-specific boundary rather than trying to solve the full generic side-effect record problem.

That separation is important. The existing [Side-Effect Boundary ADR Plan](../implementation-plans/side-effect-boundary-adr-plan.md) is aimed at future write-capable adapters and mutation lifecycle records. The new local check plan is narrower: it focuses on local command families, source-tree protection, cache/build/temp directory policy, disabled network posture, sanitized environment handling, and non-leaking error behavior before local checks are broadened.

This is the right next planning boundary because the current code already has coarse local-check concepts such as `LocalCheckSideEffectClass`, `LocalCheckNetworkPolicy`, `LocalCheckEnvironmentPolicy`, and permitted output directories, but the roadmap did not yet define how cache writes, build outputs, temp writes, or source-write denial should be treated before live command expansion.

## 4. Taxonomy Assessment

The proposed taxonomy is useful and appropriately local:

- `source_read_only`;
- `cache_write_only`;
- `build_output_write`;
- `temp_write_only`;
- `source_write`;
- `network_access`;
- `unclassified`.

It avoids the risky shortcut of treating `NoSourceWrites` as equivalent to no side effects. That distinction matters for `npm`, cargo, test runners, and future toolchain-backed checks.

The plan also correctly keeps `source_write` rejected for local check phases and keeps `unclassified` fail-closed.

## 5. Command Family Assessment

The command classifications are appropriate:

- `WorkflowOsValidateDogfood` remains acceptable as a source-read-only proof path.
- `DocsCheck` is treated as source-read-only plus explicit cache posture before live execution.
- `CargoFmtCheck` remains deferred pending toolchain/cache declaration.
- `CargoClippyWorkspace` and `CargoTestWorkspace` remain deferred pending build-output policy.
- TypeScript, contract, and integration checks remain deferred.
- Live provider smokes and arbitrary user commands remain rejected for local check v1.

This matches the existing roadmap direction and prevents the dogfood docs-check milestone from becoming a general shell-runner wedge.

## 6. Directory Policy Assessment

The directory policy is clear enough to drive implementation planning.

The plan distinguishes:

- repository root;
- protected source paths;
- build output directories;
- package/tool caches;
- temp directories;
- report artifact directories;
- state backend directories.

It also states that cache/build/temp paths must be explicit, bounded, redaction-safe, and not inferred from ambient environment. That is the right boundary before any live local check execution is added.

The implementation phase should decide whether repo-local ignored cache/build directories are allowed or whether cache directories must be outside the repository by default.

## 7. Environment And Network Assessment

The environment and network posture is safe:

- start from an empty environment;
- add only explicitly allowed non-secret variables;
- reject secret-like variable names and values;
- forbid provider tokens, registry credentials, authorization headers, private keys, and broad user environment;
- keep network disabled for v1 local checks;
- treat any network requirement as a separate planning and review boundary.

This aligns with the existing `DocsCheckLocalHandler` posture and the repository threat model.

## 8. Runtime, Event, Audit, Report, And Evidence Assessment

The plan preserves existing runtime boundaries.

It allows only existing workflow events for explicitly invoked workflow steps, bounded `LocalCheckResult` values, stable local check result references, and WorkReport citations to supplied local check result references.

It does not authorize:

- new event kinds;
- post-terminal event appends;
- automatic check execution;
- automatic report generation;
- automatic artifact writing;
- audit events outside existing executor paths;
- CLI output;
- persistence changes;
- command-output evidence;
- local check evidence attachment;
- implicit `EvidenceReference` creation.

This keeps local check side-effect planning aligned with the current reference-first report and evidence posture.

## 9. Failure Semantics Assessment

The plan's failure semantics are conservative and actionable:

- unclassified side effects fail closed;
- missing explicit cache/build/temp directory fails closed;
- directory policy violations fail closed;
- network-required checks fail closed while network is disabled;
- secret-like environment or path values fail closed;
- future source-write detection, if implemented, fails the check with stable non-leaking errors.

The error guidance correctly forbids raw paths, environment values, command output, source snippets, parser payloads, provider payloads, tokens, and credentials.

## 10. Test Plan Assessment

The future test plan covers the important model and validation surfaces:

- side-effect class representation;
- unclassified checks failing closed;
- source-write rejection;
- cache/build directory requirements;
- secret-like path rejection;
- environment allowlist validation;
- network-disabled posture;
- docs-check cache allowance without default registration;
- cargo clippy/test deferral;
- stable non-leaking errors;
- redaction-safe Debug/serialization if serializable model types are added;
- regression coverage for existing local check, executor, report, evidence, diagnostic, adapter telemetry, and runtime tests.

No additional planning blocker was found.

## 11. Documentation Review

Documentation is aligned.

The roadmap and related plans now state that local check side-effect/cache/write boundary planning exists and that live npm smoke, broader cargo/npm handlers, default registration, CLI exposure, schema fields, automatic execution, command-output evidence, local check evidence attachment, persistence, report artifacts, writes, and release posture changes remain unimplemented.

The plan also avoids overclaiming the current `DocsCheckLocalHandler`: it remains explicit, non-default, and proven through injected-runner tests rather than live npm execution.

## 12. Planning Blockers

No planning blockers.

## 13. Non-Blocking Follow-Ups

- In the model implementation phase, decide whether to extend `LocalCheckSideEffectClass` or introduce a separate `LocalCheckSideEffectBoundary` type.
- Decide whether cache directories must be outside the repository by default or whether explicitly declared ignored repo-local caches are acceptable.
- Decide whether build output directories such as `target/` require Git-ignore validation before a handler can declare them.
- Keep source-write detection out of the first model-only phase unless it can be implemented without touching real user files.
- Keep live docs-check smoke deferred until after the side-effect boundary model is implemented and reviewed.

## 14. Recommended Next Phase

Recommended next phase: **local check side-effect boundary model implementation, model-only**.

That phase should add or refine local check side-effect boundary vocabulary and validation without running live commands, adding default registration, exposing CLI behavior, adding schema fields, attaching evidence, persisting local check results, writing report artifacts, adding generic side-effect records, or authorizing writes.

## 15. Validation

Validation commands for this review:

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.
