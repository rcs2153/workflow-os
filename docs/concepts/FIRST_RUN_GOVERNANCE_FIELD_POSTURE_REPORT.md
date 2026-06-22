# First-Run Governance Field Posture Report

## 1. Executive Summary

`workflow-os first-run` now emits a bounded governance field posture summary.

The command already produced a report-ready context after loading and validating a local Workflow OS project. This phase makes the rich scaffold/spec fields visible to users by disclosing ownership, escalation, governance profile, approvals, policy gates, evidence, checks, side effects, audit/observability, and deferred/advisory field posture.

The implementation does not change workflow execution, approval behavior, schemas, local check execution, provider behavior, report artifact writing, RBAC, enterprise admin controls, or write support.

## 2. Scope Completed

- Added a first-run governance field posture helper inside the CLI.
- Classified ownership and escalation as configured, placeholder, or missing without printing raw values.
- Disclosed the initial governance profile as `observe_and_report`.
- Disclosed profile posture as `disclosed_not_enforced`.
- Disclosed approval posture.
- Disclosed policy gate posture.
- Disclosed evidence/check/side-effect posture.
- Disclosed audit/observability declaration posture.
- Disclosed deferred/advisory field posture for triggers, state model, tests, and workflow recommendations.
- Added bounded text output.
- Added bounded preview JSON output.
- Added focused CLI tests for placeholder posture, configured owner posture, JSON posture, redaction, and no runtime state creation.
- Updated CLI docs.

## 3. Scope Explicitly Not Completed

This phase does not implement:

- governance profile selection;
- executor behavior changes;
- approval bypass or approval automation;
- RBAC, IdP, admin controls, or enterprise policy stewardship;
- escalation notifications, paging, or directory lookup;
- workflow schema changes;
- automatic command execution;
- automatic local check execution;
- workflow generation, registration, or promotion;
- provider calls or provider writes;
- report artifact writing;
- hosted behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. User Experience Summary

After:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
```

users now see posture lines such as:

```text
governance_profile: observe_and_report
profile_posture: disclosed_not_enforced
ownership: placeholder
escalation: placeholder
approvals: configured
policy_gates: declared_not_evaluated
field_evidence: not_available
field_checks: skipped
field_side_effects: none_skipped_unsupported
audit_observability: declared_runtime_after_run
```

This makes the scaffold explain itself: what is configured, what is placeholder, what is declared but not evaluated, and what is explicitly deferred.

## 5. Validation Boundary Summary

The helper reads only already-loaded and validated project definitions. It does not inspect raw repository contents, execute commands, append runtime events, or create state.

The posture summary is disclosure-only. It does not treat owner strings as RBAC, does not notify escalation contacts, does not change approval behavior, and does not imply enterprise enforcement.

## 6. Redaction/Privacy Summary

The posture summary prints classifications rather than raw ownership or escalation values.

It does not print:

- owner names;
- maintainer IDs;
- escalation contact IDs;
- raw source contents;
- raw command output;
- provider payloads;
- parser payloads;
- environment values;
- credentials;
- tokens;
- private keys.

## 7. Test Coverage Summary

Focused tests cover:

- first-run text output includes profile and field posture;
- generated scaffold ownership and escalation are classified as placeholder;
- configured owner/escalation values are classified without printing raw values;
- JSON output includes bounded profile and field posture;
- JSON output does not print scaffold placeholder owner values;
- first-run still creates no runtime state;
- raw repository payload markers are not copied.

## 8. Commands Run And Results

- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance validate` - passed with expected experimental lifecycle warnings.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-spec-field-operationalization-state --mock-all-local-skills run dg/spec-field-operationalization` - paused for approval as expected.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-spec-field-operationalization-state --mock-all-local-skills approve run-1782088937354580000-2 approval/run-1782088937354580000-2/implementation-scope-approved --actor user/maintainer --reason user-approved-first-run-field-posture-implementation` - completed the governed run.

Additional validation:

- `cargo fmt --all` - applied formatting.
- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-cli --test cli first_run` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- Governance profile is disclosed as `observe_and_report`; profile selection is not implemented.
- Enterprise stewardship/admin controls are not implemented.
- Owner/escalation checks are disclosure-only, not validation failures.
- Field posture is bounded and high-level; full spec-field coverage remains future work.
- Workflow discovery does not yet use field posture to recommend owner/steward/escalation changes.

## 10. Recommended Next Phase

Proceed to **first-run governance field posture review**.

After review, the next implementation should be an ownership and escalation check that detects missing or placeholder ownership and escalation metadata across workflows and skills.
