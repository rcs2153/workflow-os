# first-run

`workflow-os first-run` emits a bounded local report-ready context for an existing Workflow OS project.

It is intended as the immediate next step after:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
```

Use `workflow-os first-run --verbose` when you want the full posture matrix in human-readable text.

## Behavior

The command:

- loads and validates the local Workflow OS project;
- detects whether the first-run governance scaffold is present;
- summarizes safe project counts for workflows, skills, policies, and tests in verbose text output and preview JSON;
- detects bounded safe repository metadata, including `package.json` presence, allowlisted package script keys, package-manager lockfile posture, TypeScript markers, Rust/Python/Go manifest and lockfile posture, GitHub workflow count, conventional source/test directories, and common repository-document presence;
- constructs all v1 WorkReport section shapes through validated `WorkReportSection` constructors;
- validates bounded incomplete-work, known-limitation, risk, and handoff-note disclosures through existing WorkReport note constructors;
- prints explicit `not_available`, `skipped`, and `none_skipped_unsupported` posture where evidence, checks, and side effects are unavailable;
- prints a concise operator summary by default;
- prints a bounded governance field posture summary for ownership, escalation, profile, approvals, policy gates, evidence, checks, side effects, audit/observability, and deferred/advisory fields in `--verbose` text output and preview JSON;
- runs a deterministic ownership/escalation metadata check over loaded workflow and skill definitions, emitting warning counts and stable issue codes only in `--verbose` text output and preview JSON;
- runs a deterministic spec-field coverage check over loaded project, workflow, skill, policy, and test surfaces, emitting posture counts and stable item codes only in `--verbose` text output and preview JSON;
- emits structured review-only workflow discovery recommendations with bounded rationale codes, spec-field coverage codes, and ownership/escalation issue codes in `--verbose` text output and preview JSON.
- emits bounded recommendation next-action hints so users and agents can decide what to review, author, and validate next without automatic workflow generation.

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
- print raw package script command bodies or dependency values;
- print raw Rust, Python, Go, or GitHub Actions manifest/workflow contents;
- generate or register workflows automatically;
- enable write-capable adapters, hosted execution, recursive agents, agent swarms, or Level 3/4 autonomy.

## Output

Text output is bounded and operator-facing:

```text
Workflow OS first-run summary
status: ready_for_review
what_happened: validated a bounded governance envelope without starting a run
what_was_not_done: no workflow run, runtime state, artifacts, local checks, or external writes were created
what_matters_now:
  - review the governance findings before treating the repo as configured
  - detected TypeScript/package metadata can guide implementation and validation workflows
  - the mock first-run workflow is optional and demonstrates approval/audit mechanics only
recommended_next_action: review first-run findings and assign ownership/check obligations
recommendation_next_actions:
  - review_only: recommendations are not active workflows until authored and reviewed
  - start_with: first_run.assign_ownership
  - workflow_candidate: first_run.typescript_implementation
  - validation_candidate: first_run.package_validation_obligations
  - safety_candidate: first_run.side_effect_posture
  - closure_candidate: first_run.report_handoff_obligations
optional_approval_audit_demo: workflow-os --mock-all-local-skills run local/first-run-governance
optional_demo_note: mock skill run demonstrates approval and event history; it is not additional repository analysis
detail: run `workflow-os first-run --verbose` for the full posture matrix
```

Verbose text output keeps the full posture matrix for audit-minded operators:

```text
workflow-os first-run --verbose

...
Detailed posture:
first_run_report_ready: true
mode: report_ready_context
validation: passed
scaffold: present
git_repository: present
spec_counts: workflows=1 skills=1 policies=1 tests=1
safe_repo_metadata:
  package_json: present
  package_manager: npm
  package_scripts: build|lint|test
  typescript: present
  typescript_markers: dependency_typescript|tsconfig_json
  cargo_toml: absent
  cargo_lock: absent
  pyproject_toml: absent
  python_lock_files: none
  go_mod: absent
  go_sum: absent
  github_workflows: 1
  source_dirs: source
  test_dirs: test
  readme: present
  license: present
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
workflow_discovery_recommendations: 7
workflow_discovery_recommendation: id=first_run.repo_implementation kind=create_workflow target=project#1 status=review_only summary=repo_implementation_workflow rationale=first_run.report_ready_context|governed_work_pattern.implementation_boundary coverage=spec_field.workflow.steps_enforced_supported_local_paths|spec_field.workflow.policy_requirements_enforced_supported_local_paths|spec_field.workflow.audit_observability_disclosed ownership=none next_action=review_and_author_workflow_spec
workflow_discovery_recommendation: id=first_run.assign_ownership kind=assign_ownership target=project#1 status=needs_human_review summary=assign_workflow_stewardship rationale=ownership_escalation.warnings_present coverage=spec_field.workflow.owner_disclosed|spec_field.skill.identity_validated ownership=authority.owner_context_required|escalation.placeholder_contact|ownership.placeholder_owner next_action=replace_placeholder_owner_and_escalation
recommendation_next_actions:
  - review_only: recommendations are not active workflows until authored and reviewed
  - start_with: first_run.assign_ownership
  - workflow_candidate: first_run.repo_implementation
  - validation_candidate: first_run.evidence_check_requirements
  - safety_candidate: first_run.side_effect_posture
  - closure_candidate: first_run.report_handoff_obligations
```

`--json` emits preview JSON only. CLI JSON remains experimental through `0.2.0-preview.1`. JSON output continues to include the bounded detailed posture fields even when default human text is concise.

The posture summary, ownership/escalation check, spec-field coverage check, and workflow discovery recommendations classify fields without printing raw owner, maintainer, escalation-contact, config, mapping, file, command, provider, parser, or source-content values. Findings use bounded target ordinals such as `workflow#1`, stable issue codes such as `ownership.placeholder_owner`, known schema vocabulary such as `surface=workflow field=triggers`, and review-only recommendation identifiers such as `first_run.repo_implementation`; they do not print raw ownership values or caller-supplied field values. This is a disclosure and recommendation surface, not RBAC, paging, hosted policy enforcement, workflow auto-generation, command execution, background trigger execution, local check execution, provider calls, write-capable adapters, or enterprise admin control.

When safe ecosystem metadata is present, recommendations may become more concrete. For example, a detected TypeScript package can add review-only recommendations such as `first_run.typescript_implementation` and `first_run.package_validation_obligations`; a Rust crate can add `first_run.rust_implementation` and `first_run.rust_validation_obligations`; a Python project can add `first_run.python_implementation`; a Go module can add `first_run.go_implementation`; and GitHub Actions presence can add `first_run.github_actions_ci_evidence`. Those recommendations cite metadata posture only. They do not make package scripts required, execute `npm`, `cargo`, Python, Go, or CI commands, generate workflows, or register local check handlers.

Each recommendation also includes a bounded `next_action` code in verbose text and preview JSON. These codes describe what a maintainer or agent should do next, such as `review_and_author_workflow_spec`, `define_evidence_and_validation_obligations`, or `define_side_effect_posture_before_writes`. They are guidance only. They do not create files, mutate state, approve gates, or execute checks.

Default text output groups the next actions into a short `recommendation_next_actions` list. This list is intentionally decision-oriented:

- `review_only` reminds users that recommendations are not active workflows until explicitly authored and reviewed;
- `start_with` points at stewardship and ownership work when placeholders remain;
- `workflow_candidate` points at the most concrete detected implementation workflow candidate;
- `validation_candidate` points at the most concrete detected validation/evidence candidate;
- `safety_candidate` points at side-effect posture before writes;
- `closure_candidate` points at report and handoff obligations.

Recommendation detail output is implemented as:

```sh
workflow-os first-run --recommendation first_run.repo_implementation
workflow-os --json first-run --recommendation first_run.assign_ownership
```

The detail view explains why one already-computed recommendation exists, the bounded rationale and metadata-signal codes behind it, the next action code, and what must be authored or reviewed before the recommendation becomes active. It does not generate workflow files, register workflows, execute commands, call providers, create runtime state, or mutate repository files.

Recommendation detail also includes an inactive draft proposal summary. This is model/helper-only authoring posture: it lists required authoring decisions, validation expectations, missing fields, and non-goals such as `no_file_written`, `no_workflow_registered`, `no_command_executed`, and `no_runtime_state_created`. It does not write a draft workflow file, activate a workflow, register checks, execute commands, or promote the recommendation.

For a dedicated authoring preview, use:

```sh
workflow-os author workflow --from-recommendation first_run.repo_implementation --dry-run
```

The authoring dry-run output is still inactive and non-mutating. It makes the same required decisions and validation expectations easier to review before any future file-writing or promotion flow exists.

The optional approval/audit demo command is deliberately separate from the recommended next action. `workflow-os first-run` is the real bounded posture analysis. `workflow-os --mock-all-local-skills run local/first-run-governance` is an optional local demo of approval checkpoints, durable event history, and inspectable runtime state using mock local skill behavior.

## Failure Behavior

If no Workflow OS project is present, the command fails with `cli.first_run.manifest_missing` and directs the user to run `workflow-os init-repo-governance`.

If project validation fails, the command fails with `cli.first_run.validation_failed` and directs the user to run `workflow-os validate` for diagnostics. The first-run error itself remains bounded and does not echo raw spec content.
