# Spec Field Coverage Check Report

## 1. Executive Summary

The first-run spec field coverage check is implemented as a bounded, warning-only extension to `workflow-os first-run`.

The check inventories known project, workflow, skill, policy, and test spec surfaces and reports whether representative rich fields are currently enforced, validated, disclosed, advisory, or deferred. It makes the current governance posture visible during onboarding without changing validation semantics, running workflows, executing local checks, calling providers, generating workflows, or enabling writes.

## 2. Scope Completed

- Added an internal spec-field coverage taxonomy helper for `workflow-os first-run`.
- Added bounded text output:
  - `spec_field_coverage_check`
  - category counts for enforced, validated, disclosed, advisory, and deferred fields
  - stable `spec_field_coverage_item` rows with surface, field, posture, and code.
- Added preview JSON output for the same bounded coverage counts and item codes.
- Covered project, workflow, skill, policy, and test surfaces.
- Reported workflow triggers as validated while background execution remains deferred.
- Reported workflow state model as advisory.
- Reported test assertions as validated/deferred execution.
- Reported skill capability/adapter posture as validated while writes remain deferred.
- Added focused CLI tests for bounded output and non-leakage.
- Updated roadmap, implementation plans, and first-run CLI documentation.

## 3. Scope Explicitly Not Completed

This phase did not add:

- new CLI commands;
- workflow schema changes;
- stricter validation failures;
- runtime execution changes;
- workflow generation or registration;
- automatic command execution;
- automatic local check execution;
- provider calls;
- write-capable adapters;
- RBAC, IdP integration, enterprise admin controls, or escalation notifications;
- hosted or distributed runtime behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Output Summary

`workflow-os first-run` now emits bounded coverage output such as:

```text
spec_field_coverage_check: warnings
spec_field_coverage_enforced: 4
spec_field_coverage_validated: 15
spec_field_coverage_disclosed: 3
spec_field_coverage_advisory: 3
spec_field_coverage_deferred: 1
spec_field_coverage_item: surface=workflow field=triggers posture=validated_deferred_execution code=spec_field.triggers.not_background_executed
```

The output uses stable schema vocabulary and posture codes only. It does not print raw owner values, escalation values, config values, mapping literals, file contents, command output, parser payloads, provider payloads, environment values, credentials, tokens, or secret-like values.

## 5. Validation Boundary Summary

The check is disclosure-only.

It does not affect:

- `workflow-os validate` pass/fail behavior;
- local executor status behavior;
- approval, retry, cancellation, or policy semantics;
- report artifact behavior;
- workflow state or event history.

Advisory and deferred fields remain advisory/deferred. Owner and escalation fields remain metadata and warnings, not authorization or notification routing.

## 6. Privacy And Redaction Summary

The implementation prints:

- bounded surface names;
- known schema field names;
- category counts;
- posture labels;
- stable issue codes.

It does not print caller-supplied values from config overlays, ownership metadata, escalation contacts, mapping literals, repository files, command output, provider payloads, parser errors, or environment values. Existing secret-like spec validation behavior remains unchanged.

## 7. Test Coverage Summary

Tests cover:

- first-run text output includes the spec-field coverage check;
- enforced, validated, disclosed, advisory, and deferred counts are present;
- workflow triggers are reported as validated/deferred execution;
- workflow state model is reported as advisory;
- skill adapter/capability write posture is reported as deferred;
- test assertions are reported without executing tests;
- preview JSON contains bounded coverage counts and codes;
- owner and escalation values are not printed;
- config values and workflow literal-like values are not printed;
- raw provider/spec/command/parser payload markers are not copied;
- existing first-run failure behavior still does not write state.

## 8. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Known Limitations

- The coverage inventory is representative and static; it is not yet a standalone coverage command.
- Coverage posture is not yet tied into workflow discovery recommendations.
- Catalog/store governance for field coverage is not implemented.
- The check does not enforce strict profiles or block validation.
- Missing citation records remain outside this phase.

## 10. Recommended Next Phase

Recommended next phase: **spec field coverage check review**.

The check changes onboarding output and user interpretation of rich YAML fields, so it should receive a focused maintainer review before expanding coverage, linking it to workflow discovery, or turning any posture into stricter validation or runtime gates.
