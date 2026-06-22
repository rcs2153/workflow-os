# Spec Field Coverage Check Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The first-run spec field coverage check is appropriately bounded, warning-only, and redaction-safe. It improves onboarding by making rich YAML fields legible as enforced, validated, disclosed, advisory, or deferred without changing validation behavior, runtime execution, schema shape, provider behavior, or write posture.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- `workflow-os first-run` text output for spec-field coverage status, category counts, and bounded item codes;
- preview JSON output for the same bounded coverage information;
- static coverage taxonomy for project, workflow, skill, policy, and test surfaces;
- tests for representative output and non-leakage;
- docs, roadmap updates, and an end-of-phase report.

No accidental implementation was found for:

- new CLI commands;
- workflow schema changes;
- stricter validation failures;
- workflow execution changes;
- automatic command execution;
- automatic local check execution;
- provider calls;
- workflow generation or registration;
- write-capable adapters;
- RBAC, IdP, enterprise admin controls, or escalation notifications;
- hosted or distributed runtime behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 3. Output Model Assessment

The output model is suitable for first-run onboarding.

Text output includes:

- `spec_field_coverage_check`;
- enforced, validated, disclosed, advisory, and deferred counts;
- item rows containing only surface, field, posture, and code.

Preview JSON includes the same bounded status, counts, categories, posture strings, and codes. The JSON remains part of the existing experimental CLI JSON posture.

The implementation does not print raw file paths, raw field values, command output, provider payloads, parser payloads, environment values, or secret-like strings. The item vocabulary is stable schema vocabulary rather than caller-supplied data.

## 4. Taxonomy Assessment

The taxonomy matches the accepted plan:

- enforced;
- validated;
- disclosed;
- advisory;
- deferred.

Representative postures are conservative:

- triggers are validated while background execution remains deferred;
- state model is advisory;
- test assertions are validated/deferred execution;
- capability/adapter posture discloses that writes remain deferred;
- owner metadata is disclosed without treating ownership as authorization;
- audit/observability is disclosed without claiming production sinks.

The check is best understood as schema-vocabulary coverage, not per-instance field usage coverage. That distinction should remain explicit as the surface expands.

## 5. Runtime And Validation Assessment

The phase preserves runtime and validation semantics.

Verified:

- `workflow-os first-run` still loads and validates the project before emitting the report-ready context;
- `workflow-os validate` behavior is not changed;
- first-run remains report-ready context output, not a terminal WorkReport;
- no runtime state is created;
- no workflow events are appended;
- no local checks, commands, providers, adapters, or workflows are executed;
- no report artifacts are written.

There is no evidence that advisory fields are treated as enforced, owner strings are treated as authorization, escalation contacts are treated as routing, declared checks are treated as executable commands, or adapter requirements are treated as provider permissions.

## 6. Privacy And Redaction Assessment

The privacy posture is acceptable.

The check may disclose:

- bounded surface names;
- known schema field names;
- category counts;
- posture labels;
- stable issue codes.

The implementation and tests protect against leakage of:

- owner and escalation values;
- config values;
- workflow text markers used as raw-value probes;
- raw provider/spec/command/parser payload markers;
- secret-like invalid project content;
- run IDs and approval IDs in first-run output.

Existing secret-like spec validation remains unchanged and continues to fail closed without echoing the secret-like value.

## 7. Test Quality Assessment

Test coverage is adequate for this phase.

Covered:

- first-run output includes `spec_field_coverage_check`;
- all five coverage count buckets are present;
- workflow triggers are reported as validated/deferred execution;
- workflow state model is reported as advisory;
- skill capability/adapter write posture is reported as deferred;
- test assertions are reported as validated/deferred execution;
- preview JSON contains bounded coverage item codes;
- owner and escalation values are not printed;
- config values are not printed;
- raw provider/spec/command/parser payload markers are not printed;
- first-run does not create runtime state or report artifacts;
- existing workspace tests pass.

Shallow or missing coverage:

- The raw mapping-literal test uses a workflow text marker rather than a true `input_mapping` literal value. This is acceptable for this phase because the implementation never reads or prints caller-supplied mapping values, but a future focused test should mutate an actual mapping literal in a valid workflow fixture.
- JSON tests use substring assertions rather than parsing the JSON object and asserting shape. Existing CLI tests use this style, so this is not a blocker, but parsed JSON assertions would make this surface harder to regress.
- The static coverage map is not tested against multiple project shapes, such as a project without tests or without policies. That can be added before expanding coverage into workflow discovery or catalog behavior.

## 8. Documentation Review

Documentation is accurate after a small stale non-goal correction in the plan.

Docs now state:

- the first-run spec-field coverage check is implemented;
- the check is warning-only;
- it reports enforced, validated, disclosed, advisory, and deferred postures;
- validation behavior is unchanged;
- runtime semantics are unchanged;
- standalone coverage commands remain deferred;
- strict gating remains deferred;
- workflow discovery integration remains deferred;
- catalog/store behavior remains deferred;
- schema changes, command execution, workflow generation, provider calls, writes, hosted behavior, recursive agents, agent swarms, and Level 3/4 autonomy remain unsupported.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add a focused first-run test that mutates a real `input_mapping` literal and proves it is not printed.
- Parse preview JSON in CLI tests and assert object shape, category counts, and representative item codes structurally.
- Document explicitly that the current coverage check is schema-vocabulary coverage, not per-instance usage coverage.
- Add project-shape coverage for projects without policy specs, test specs, or skill specs before using coverage results in workflow discovery.
- Consider moving the static taxonomy into a small dedicated module if the coverage inventory grows beyond first-run.

## 11. Recommended Next Phase

Recommended next phase: **workflow discovery integration planning**.

The scaffold field operationalization lane has now produced three onboarding surfaces: field posture output, ownership/escalation warnings, and spec-field coverage. The next useful step is to plan how workflow discovery recommendations can consume these bounded postures without auto-generating workflows or pretending advisory/deferred fields are enforced.

Do not jump from this check directly to strict validation, schema changes, automatic workflow registration, command execution, provider calls, write-capable adapters, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

## 12. Validation

Commands run:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
