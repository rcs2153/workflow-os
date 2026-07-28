# Authoritative Governance Scaffold External-Repository Evaluation

## 1. Executive Verdict

**Runtime blocker found and fixed; external proof now passes.**

The explicit scaffold option preserves default compatibility, existing agent
guidance, validation behavior, safe repository metadata discovery, and
review-only recommendations. The initial opted-in generated project could not
start its generated workflow through the declared authoritative path.

The scaffold writes the project-level
`workflow_os_project_validation` profile selection but does not add the exact
workflow-step local-check declaration required by the authoritative CLI
consumer. The resulting project:

- validates successfully;
- reports `authoritative_execution: declared_supported_enforced`; and
- then fails before run creation with
  `cli.authoritative_governance.check_profile_missing`.

This was a setup-to-runtime contract blocker. The focused fix now adds the
canonical closed requirement only to explicitly authoritative scaffolds.
Default generated workflow content remains unchanged. The same disposable
evaluation now reaches both explicit approval gates, completes, and persists
one bounded WorkReport artifact.

## 2. Evaluation Scope

The evaluation created two identical disposable git repositories under
`/private/tmp` with:

- a bounded `package.json` containing `build` and `test` script keys;
- a `tsconfig.json` marker; and
- pre-existing repository-specific `AGENTS.md` guidance.

One repository used default scaffolding. The other used:

```sh
workflow-os init-repo-governance \
  --agent codex \
  --authoritative-governance
```

No source contents, credentials, provider configuration, network access,
external writes, or production repository state were used.

## 3. Successful Findings

Both scaffold paths:

- preserved the pre-existing unmanaged `AGENTS.md` guidance;
- appended the managed Workflow OS block;
- generated the same bounded governance envelope;
- passed `workflow-os validate`;
- detected the bounded TypeScript/package metadata;
- kept recommendations review-only;
- created no run, runtime state, artifact, local check, or external write
  during `first-run`; and
- described the mock workflow as an optional approval/audit demonstration.

The default scaffold remained undeclared and reported:

```text
authoritative_execution: not_declared
```

The explicit scaffold alone added the closed declaration and reported:

```text
authoritative_execution: declared_supported_enforced
authoritative_execution_profile: observe_and_report
authoritative_execution_local_check_profile: workflow_os_project_validation
```

The review-only proportional-governance assessment remained the same in both
repositories. It selected approval plus visible disclosure because authority
and evidence/check facts were incomplete. The evaluation did not manufacture a
quiet-success claim.

## 4. Blocker

The generated workflow contains no `local_check_requirements` entry. The
authoritative run path requires exactly one entry with:

```yaml
local_check_requirements:
  - id: project-validation
    command_id: local-check/workflow-os-validate
    requirement_level: required
    minimum_assurance: kernel_observed_local_process
    accepted_statuses: [passed]
    freshness:
      mode: no_reuse
    exact_immutable_run_binding_required: true
    truncation_allowed: false
    network_maximum: disabled
    side_effect_maximum: no_source_writes
```

Before the fix, the attempted opted-in run failed before creating runtime
state:

```text
error[cli.authoritative_governance.check_profile_missing]:
authoritative governance requires one workflow-os project-validation check declaration
```

The failure itself was safely fail-closed. The blocker was that the scaffold
advertised an enforced and runnable posture without generating the required
workflow contract.

## 5. Fix And Re-Evaluation

The narrow fix:

- adds the exact closed project-validation requirement only to workflows
  generated with `--authoritative-governance`;
- keeps default generated workflow content unchanged;
- keeps dry-run non-writing;
- keeps unknown-option failure before writes;
- proves the opted-in generated project validates;
- proves its generated workflow reaches the existing authoritative route;
- proves the fixed check runs before any mock skill invocation;
- preserves separate proportional-governance and workflow-step approvals;
- proves terminal completion and exactly one WorkReport artifact; and
- keeps provider execution, OpenShell, arbitrary commands, SideEffect execution,
  external writes, schemas, examples, hosted behavior, and release posture out
  of scope.

The repeated disposable run produced:

```text
status: Completed
report: generated_in_memory
artifact: persisted
approvals: 2
work_report_artifacts: 1
```

Its ordered event trail showed `GovernanceAssessmentBound`, both approval
request/decision pairs, one skill invocation, and `RunCompleted`.

## 6. Commands Run

- `workflow-os init-repo-governance --agent codex`
- `workflow-os init-repo-governance --agent codex --authoritative-governance`
- `workflow-os validate` in both disposable repositories
- `workflow-os first-run --verbose` in both repositories
- `workflow-os --mock-all-local-skills run local/first-run-governance` in the
  opted-in repository

Before the fix, the authoritative run failed with the blocker above. After the
fix, the complete command sequence passed through terminal artifact
persistence.

## 7. Governed Evaluation Record

- workflow id: `dg/review`
- run id: `run-1785218225110625000-2`
- approval id:
  `approval/run-1785218225110625000-2/review-scope-approved`
- presentation id: `presentation/9cc7be39554ab529`
- approval outcome: granted by delegated maintainer
- approval proof: persisted
- out-of-kernel work: disposable repository creation and CLI evaluation were
  performed by Codex under the kernel-governed review scope

## 8. Recommended Next Phase

Perform a focused blocker-fix review. Keep default activation unchanged until
maintainers decide whether this explicit opt-in has enough external evidence
for broader onboarding treatment. Do not begin OpenShell integration as part
of that review.
