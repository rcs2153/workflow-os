# first-run

`workflow-os first-run` emits a bounded local report-ready context for an existing Workflow OS project.

It is intended as the immediate next step after:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
```

## Behavior

The command:

- loads and validates the local Workflow OS project;
- detects whether the first-run governance scaffold is present;
- summarizes safe project counts for workflows, skills, policies, and tests;
- constructs all v1 WorkReport section shapes through validated `WorkReportSection` constructors;
- validates bounded incomplete-work, known-limitation, risk, and handoff-note disclosures through existing WorkReport note constructors;
- prints explicit `not_available`, `skipped`, and `none_skipped_unsupported` posture where evidence, checks, and side effects are unavailable;
- prints a bounded governance field posture summary for ownership, escalation, profile, approvals, policy gates, evidence, checks, side effects, audit/observability, and deferred/advisory fields;
- runs a deterministic ownership/escalation metadata check over loaded workflow and skill definitions, emitting warning counts and stable issue codes only;
- runs a deterministic spec-field coverage check over loaded project, workflow, skill, policy, and test surfaces, emitting posture counts and stable item codes only;
- recommends review-only workflow candidates.

The command does not fabricate a terminal `WorkReport`, because no workflow run has occurred. It emits a report-ready context instead.

## Boundaries

`first-run` does not:

- run workflows;
- approve checkpoints;
- execute repository commands;
- execute local check handlers;
- register real local skill handlers;
- create runtime state;
- append workflow events;
- create WorkReport artifacts;
- persist reports;
- call providers;
- read raw repository source contents;
- copy command output, parser payloads, provider payloads, environment values, credentials, or token-like values;
- generate or register workflows automatically;
- enable write-capable adapters, hosted execution, recursive agents, agent swarms, or Level 3/4 autonomy.

## Output

Text output is bounded and operator-facing:

```text
first_run_report_ready: true
mode: report_ready_context
validation: passed
scaffold: present
git_repository: present
spec_counts: workflows=1 skills=1 policies=1 tests=1
sections: 11
...
evidence: not_available
checks: skipped
side_effects: none_skipped_unsupported
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
deferred_fields:
  - triggers_declared_not_background_executed
  - state_model_advisory
  - tests_declared_not_automatically_executed
  - workflow_recommendations_review_only
ownership_escalation_check: warnings
ownership_escalation_findings: 7
ownership_missing_owner: 0
ownership_placeholder_owner: 2
escalation_missing_contact: 0
escalation_placeholder_contact: 2
lifecycle_warnings: 2
authority_context_warnings: 1
ownership_escalation_finding: target=workflow#1 code=ownership.placeholder_owner severity=warning
spec_field_coverage_check: warnings
spec_field_coverage_enforced: 4
spec_field_coverage_validated: 15
spec_field_coverage_disclosed: 3
spec_field_coverage_advisory: 3
spec_field_coverage_deferred: 1
spec_field_coverage_item: surface=workflow field=triggers posture=validated_deferred_execution code=spec_field.triggers.not_background_executed
spec_field_coverage_item: surface=workflow field=state_model posture=advisory code=spec_field.workflow.state_model_advisory
spec_field_coverage_item: surface=skill field=capabilities_adapters posture=validated_writes_deferred code=spec_field.skill.capabilities_adapters_writes_deferred
spec_field_coverage_item: surface=test field=assertions posture=validated_deferred_execution code=spec_field.tests.not_automatically_executed
```

`--json` emits preview JSON only. CLI JSON remains experimental through `0.2.0-preview.1`.

The posture summary, ownership/escalation check, and spec-field coverage check classify fields without printing raw owner, maintainer, escalation-contact, config, mapping, file, command, provider, parser, or source-content values. Findings use bounded target ordinals such as `workflow#1`, stable issue codes such as `ownership.placeholder_owner`, and known schema vocabulary such as `surface=workflow field=triggers`; they do not print raw ownership values or caller-supplied field values. This is a disclosure surface, not RBAC, paging, hosted policy enforcement, workflow auto-generation, command execution, background trigger execution, local check execution, provider calls, write-capable adapters, or enterprise admin control.

## Failure Behavior

If no Workflow OS project is present, the command fails with `cli.first_run.manifest_missing` and directs the user to run `workflow-os init-repo-governance`.

If project validation fails, the command fails with `cli.first_run.validation_failed` and directs the user to run `workflow-os validate` for diagnostics. The first-run error itself remains bounded and does not echo raw spec content.
