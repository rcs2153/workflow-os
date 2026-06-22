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
```

`--json` emits preview JSON only. CLI JSON remains experimental through `0.2.0-preview.1`.

The posture summary classifies fields without printing raw owner, maintainer, escalation-contact, file, command, provider, or source-content values. It is a disclosure surface, not RBAC, paging, hosted policy enforcement, workflow auto-generation, command execution, or enterprise admin control.

## Failure Behavior

If no Workflow OS project is present, the command fails with `cli.first_run.manifest_missing` and directs the user to run `workflow-os init-repo-governance`.

If project validation fails, the command fails with `cli.first_run.validation_failed` and directs the user to run `workflow-os validate` for diagnostics. The first-run error itself remains bounded and does not echo raw spec content.
