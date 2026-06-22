# Spec Field Coverage Check Plan

Status: Implemented as a warning-only `workflow-os first-run` disclosure check in [Spec Field Coverage Check Report](../concepts/SPEC_FIELD_COVERAGE_CHECK_REPORT.md). It does not change validation behavior, add schema fields, execute commands, generate workflows, or change runtime semantics.

## 1. Executive Summary

Workflow OS specs and scaffolds now expose rich governance fields before every field has full runtime behavior. That is intentional, but users need a clear map of what the kernel currently enforces, validates, discloses, treats as advisory, or defers.

The next question is how `workflow-os first-run` should report spec-field coverage in a deterministic, bounded, redaction-safe way. The first implementation should be a local warning/reporting check that inventories known project, workflow, skill, policy, and test fields and explains their operational posture without pretending unsupported automation exists.

The first implementation extends `workflow-os first-run`; standalone coverage commands, strict gating, workflow discovery integration, catalog/store behavior, and broader automation remain deferred.

## 2. Goals

- Make rich YAML fields understandable as governed obligations, not decorative metadata.
- Give new users a bounded "what is real today" coverage map during first-run onboarding.
- Classify fields as enforced, validated, disclosed, advisory, or deferred.
- Preserve deterministic validation and existing workflow semantics.
- Avoid raw source contents, raw values, command output, provider payloads, and secrets.
- Prepare future workflow discovery and catalog stewardship to reason about field coverage.
- Keep the first implementation small, local, and warning-only.

## 3. Non-Goals

Do not implement in this phase:

- new CLI commands;
- workflow schema changes;
- stricter validation failures;
- automatic command execution;
- automatic local check execution;
- provider calls;
- write-capable adapters;
- workflow generation or registration;
- RBAC, IdP, enterprise admin controls, or escalation notifications;
- hosted or distributed runtime behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Coverage Taxonomy

The check should use the existing scaffold operationalization taxonomy.

| Posture | Meaning |
| --- | --- |
| Enforced | Runtime or validation blocks unsafe or incomplete behavior in a supported path. |
| Validated | Deterministic validation checks shape, references, uniqueness, or consistency. |
| Disclosed | First-run, WorkReport, audit, or inspection output reports the field and its gaps. |
| Advisory | The field guides humans or agents but does not affect validation or runtime behavior yet. |
| Deferred | The field is intentionally reserved for a later accepted phase. |

The check should prefer honest under-claiming. If a field is parsed but not currently validated or enforced, it should be reported as advisory or deferred instead of implied as active governance.

## 5. Candidate Field Inventory

The first implementation should inventory fields by surface, not by raw source file contents.

| Surface | Example fields | Current posture | First check posture |
| --- | --- | --- | --- |
| Project identity | `schema_version`, `project.id`, `project.name` | Validated | Validated |
| Project layout | `layout.workflows`, `layout.skills`, `layout.policies`, `layout.tests` | Parsed and loaded | Validated or disclosed |
| Project config | `config` overlays | Parsed as non-secret metadata | Advisory or deferred |
| Workflow identity | `schema_version`, `id`, `version`, `display_name` | Validated | Validated |
| Workflow metadata | `description`, `owner`, `tags`, lifecycle/status | Parsed; selected warnings | Disclosed with owner/escalation linkage |
| Autonomy | `autonomy_level`, `disabled_by_default` | Level 3/4 safety validated | Validated and disclosed |
| Triggers | manual, file, schedule, external event | Required by validation, not executed by background processor | Validated plus deferred execution |
| State model | `state_model` | Parsed for future use | Advisory |
| Steps | ordered `steps`, `skill_ref`, `terminal_behavior` | Validated; sequential local executor uses supported paths | Enforced for supported local run paths |
| Branches | `branches`, branch targets | Validated for known targets | Validated, not executed as branching runtime |
| Mappings | `input_mapping`, `output_mapping` | Parsed; selected executor use | Validated or advisory by mapping form |
| Policy requirements | workflow and step policy refs | Validated; local executor evaluates supported policy gates | Enforced for supported local run paths, disclosed otherwise |
| Approval posture | workflow requirements, step approval policy, sensitivity | Validated; local executor can pause for approvals | Enforced for supported local run paths, disclosed otherwise |
| Retry/escalation | retry refs, step retry policy, escalation policy | Validated; sequential executor supports selected behavior | Enforced for supported local paths, disclosed otherwise |
| Timeout/cancellation | timeout policy, step timeout, cancellation behavior | Validated in selected cases | Validated or advisory |
| Audit/observability | audit requirements, observability requirements | Declared; local runtime events exist, no production sink | Disclosed |
| Skill identity | `schema_version`, `id`, `version`, `display_name` | Validated | Validated |
| Skill contracts | input/output fields, required fields | Validated | Validated |
| Sensitivity/redaction | sensitive fields, redaction behavior | Validated | Validated |
| Capabilities/adapters | allowed capabilities, adapter requirements | Validated as declarations; read-only adapters exist | Validated and disclosed, writes deferred |
| Failure/evaluation | failure modes, evaluation criteria | Validated for selected requirements | Validated or advisory |
| Policy specs | policy id/name/rules/effects | Parsed and validated for refs/effects | Validated |
| Test specs | test id/name/target/assertions | Loaded and reference-shaped; no test execution | Validated or deferred |

## 6. First Implementation Target Recommendation

The first implementation should extend `workflow-os first-run` rather than add a separate command.

Reasons:

- first-run is already the onboarding ledger/report posture surface;
- it already loads and validates the project;
- it already emits bounded field posture and ownership/escalation warnings;
- users see coverage immediately after scaffold setup;
- the behavior can remain report-only and warning-only.

The implementation should emit a `spec_field_coverage_check` block in text output and preview JSON. It should not fail validation, execute checks, execute workflows, or mutate project files.

## 7. Output Model

The output should be bounded and stable.

Recommended text shape:

```text
spec_field_coverage_check: warnings
spec_field_coverage_enforced: 4
spec_field_coverage_validated: 18
spec_field_coverage_disclosed: 7
spec_field_coverage_advisory: 5
spec_field_coverage_deferred: 6
spec_field_coverage_item: surface=workflow field=triggers posture=validated_deferred_execution code=spec_field.triggers.not_background_executed
```

Rules:

- print field names and posture codes only;
- use bounded surface names such as `project`, `workflow`, `skill`, `policy`, and `test`;
- do not print raw field values;
- do not print file paths by default;
- do not print command output, source snippets, provider payloads, parser payloads, environment values, or secret-like strings;
- preserve preview JSON as experimental.

## 8. Runtime And Validation Boundary

The coverage check is a disclosure/check surface, not validation semantics.

It must not:

- change whether `workflow-os validate` passes or fails;
- change whether local executor runs pass, fail, pause, retry, or cancel;
- treat advisory fields as enforced;
- treat owner strings as authorization;
- treat escalation contacts as notification routing;
- treat declared checks as executable commands;
- treat adapter requirements as permission to call providers.

If the check itself fails due to an internal construction error, the implementation should fail the `first-run` command with a stable non-leaking internal error rather than emit partial misleading coverage.

## 9. Privacy And Redaction

The check should be safe for public repository output and CI logs.

It may disclose:

- known schema field names;
- surface names;
- posture categories;
- stable issue codes;
- bounded counts.

It must not disclose:

- raw owner, maintainer, or escalation-contact values;
- raw config values;
- raw mapping literals;
- raw examples;
- raw spec contents;
- raw parser payloads;
- raw command output;
- provider payloads;
- environment variables;
- credentials, tokens, private keys, or authorization headers.

Field names that are known schema vocabulary are acceptable. Caller-supplied field values are not.

## 10. Test Plan

Future implementation should add focused tests for:

- first-run output includes `spec_field_coverage_check`;
- enforced, validated, disclosed, advisory, and deferred counts are present;
- workflow trigger fields are reported as validated but background execution is deferred;
- state model fields are reported as advisory;
- test specs are reported without executing tests;
- side-effect and adapter write posture is deferred or unsupported;
- owner and escalation values are not printed;
- mapping literals and config values are not printed;
- secret-like values in specs still fail through existing loader/validation paths;
- preview JSON contains bounded counts and codes only;
- coverage check does not change `workflow-os validate` pass/fail behavior;
- existing first-run, scaffold, validation, runtime, report, and docs tests still pass.

## 11. Proposed Implementation Sequence

1. Add a small internal coverage taxonomy helper for known spec surfaces.
2. Add first-run text and preview JSON output for bounded counts and item codes.
3. Add tests for representative project, workflow, skill, policy, and test fields.
4. Verify non-leakage for owner, escalation, mapping literal, config value, and secret-like payload cases.
5. Run maintainer review before expanding coverage or making any posture blocking.

## 12. Deferred Work

Explicitly defer:

- a standalone `workflow-os coverage` command;
- strict profile gating;
- validation failures based on coverage posture;
- automatic local check execution;
- workflow discovery integration;
- catalog/store integration;
- workflow schema changes;
- enterprise stewardship/RBAC/admin controls;
- notification or escalation routing;
- write-capable adapters;
- hosted runtime behavior.

## 13. Final Recommendation

The next implementation prompt should be:

```text
First-run spec field coverage check.
```

It should be warning-only, local, deterministic, bounded, and redaction-safe. It should make the current governance coverage visible without claiming unsupported automation.

Do not build schema changes, strict validation, command execution, workflow generation, enterprise admin controls, provider writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy in this phase.
