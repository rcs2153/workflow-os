# Workflow Discovery Field Coverage Integration Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation successfully connects existing `workflow-os first-run` posture signals to bounded, review-only workflow discovery recommendation records. It improves the first-run guidance surface without creating workflows, registering workflows, writing catalog state, executing commands, calling providers, or changing validation/runtime semantics.

## 2. Scope Verification

The phase stayed within the approved first-run recommendation scope.

Implemented:

- internal first-run recommendation taxonomy;
- structured recommendation records;
- text output under `workflow_discovery_recommendations`;
- preview JSON output under `workflow_discovery_recommendations`;
- recommendation derivation from governance posture, ownership/escalation warnings, and spec-field coverage codes;
- focused CLI tests;
- documentation and phase report.

No accidental implementation was found for:

- workflow generation;
- workflow registration;
- workflow catalog or store persistence;
- workflow proposal persistence;
- conflict-resolution engine;
- runtime workflow discovery execution;
- local check execution;
- command execution;
- provider calls;
- write-capable adapters;
- schema changes;
- CLI workflow-discovery command;
- RBAC, IdP integration, paging, or enterprise notification;
- approval routing;
- hosted/distributed runtime behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

A stale planning non-goal was corrected during review so the plan no longer says the already-implemented bounded first-run integration is still out of scope.

## 3. Recommendation Model Assessment

The model is intentionally small and appropriate for this phase.

Implemented recommendation fields include:

- recommendation id;
- recommendation kind;
- target surface and ordinal;
- status;
- bounded summary slug;
- rationale codes;
- related spec-field coverage codes;
- related ownership/escalation issue codes.

The implemented kinds cover the initial low-risk slice:

- `create_workflow`;
- `assign_ownership`;
- `add_evidence_check_requirements`;
- `add_side_effect_posture`;
- `add_report_handoff_obligations`.

The implementation does not yet model change/split/merge/retire recommendations or conflict hints. That is acceptable for the first implementation and should wait for catalog/store planning.

## 4. Signal Integration Assessment

The integration correctly consumes only existing bounded first-run signals:

- `GovernanceFieldPosture`;
- `OwnershipEscalationCheck`;
- `SpecFieldCoverageCheck`.

The recommendations cite posture and stable codes rather than raw values. Ownership/escalation recommendations deduplicate issue codes deterministically with a `BTreeSet`, which gives stable output and avoids repeating per-target raw context.

The implementation avoids treating advisory/deferred fields as proof of active automation. For example, side-effect posture remains a disclosure recommendation and does not imply writes or provider calls.

## 5. First-Run Output Assessment

Text output is deterministic and review-oriented.

The new text block reports:

- recommendation count;
- id;
- kind;
- target;
- status;
- summary slug;
- rationale code list;
- coverage code list;
- ownership issue code list.

The legacy prose recommendations remain present, which preserves operator readability while giving automation a structured surface.

JSON output mirrors the bounded shape and remains preview JSON. No schema contract was added.

## 6. Privacy And Redaction Assessment

The implementation is redaction-safe for this phase.

The recommendation output uses:

- static ids;
- static summary slugs;
- static rationale codes;
- stable coverage item codes;
- stable ownership/escalation issue codes;
- bounded target ordinals.

No raw values are copied into recommendation output:

- no raw spec contents;
- no raw config values;
- no raw mapping literals;
- no raw owner, maintainer, or escalation-contact values;
- no source snippets;
- no command output;
- no provider payloads;
- no parser payloads;
- no environment values;
- no credentials, tokens, private keys, or authorization headers.

The implementation does not inspect raw repository source content and does not call providers or execute commands.

## 7. Runtime And State Assessment

The phase did not change runtime behavior.

Verified:

- `first-run` remains a validation/report-ready context command;
- no workflow run is created;
- no runtime state is created;
- no workflow events are appended;
- no report artifacts are written;
- no catalog records are written;
- no commands or checks are executed;
- no provider calls are made.

Existing executor, adapter, side-effect, hook, WorkReport, and validation tests passed unchanged.

## 8. Test Quality Assessment

Tests cover the main implementation obligations:

- structured text output exists;
- JSON output exists;
- create-workflow recommendation is present;
- ownership recommendation is present when scaffold ownership/escalation warnings exist;
- related coverage codes are present;
- related ownership/escalation issue codes are present;
- ordering is deterministic for deduplicated ownership issue codes;
- legacy prose recommendations remain present;
- raw owner values, run IDs, approval IDs, raw config, raw mapping markers, provider markers, command markers, parser markers, and source-content markers are not printed;
- `first-run` does not create runtime state.

Shallow or missing coverage:

- no focused test proves that configured ownership suppresses the `assign_ownership` recommendation;
- no focused test parses JSON structurally rather than substring-checking;
- no focused test proves advisory/deferred fields are never labeled as enforced in recommendation output;
- no focused test mutates actual mapping/config values beyond current marker substitutions;
- no focused test covers smaller projects with absent workflow/skill/test surfaces and a reduced recommendation count.

These are non-blocking because the current implementation is static, bounded, and covered by the full workspace suite.

## 9. Documentation Review

Documentation is honest and aligned after the small stale-plan correction.

Docs state:

- first-run workflow discovery recommendations are implemented as bounded output;
- recommendations are review-only;
- recommendations cite posture, ownership/escalation, and spec-field coverage codes;
- workflow generation is not implemented;
- workflow registration is not implemented;
- catalog/storage is not implemented;
- schema changes are not implemented;
- command execution/local check execution is not implemented;
- provider calls and writes are not implemented;
- hosted behavior is not implemented;
- recursive agents, agent swarms, and Level 3/4 autonomy are not implemented.

The first-run CLI documentation now includes representative structured recommendation output.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add a configured-owner test proving ownership warnings disappear and `assign_ownership` is not emitted.
- Parse preview JSON structurally in tests instead of using substring assertions.
- Add a small-project test covering reduced recommendation output when workflow/skill/test surfaces are absent.
- Add explicit tests that advisory/deferred coverage codes remain advisory/deferred and are not described as enforced.
- Plan workflow catalog/store support before adding conflict hints, workflow proposal state, or workflow promotion/retirement records.
- Later classify recommendations as portable user-facing, dogfood-specific, or enterprise-stewardship oriented.

## 12. Recommended Next Phase

Workflow discovery field coverage catalog/store planning.

The first-run recommendation surface now exists and is bounded. The next useful design question is how recommendations should be persisted, reviewed, promoted, rejected, related, or retired without turning first-run output into automatic workflow generation. That planning should remain local-first and should not introduce schema changes, provider calls, writes, hosted behavior, or enterprise RBAC yet.

## 13. Validation

Review validation commands run:

- `cargo fmt --all --check`: passed using the repository bundled Rust toolchain.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed using the repository bundled Rust toolchain.
- `cargo test --workspace`: passed using the repository bundled Rust toolchain.
- `npm run check:docs`: passed using the repository bundled Node.js runtime.
- `git diff --check`: passed.
