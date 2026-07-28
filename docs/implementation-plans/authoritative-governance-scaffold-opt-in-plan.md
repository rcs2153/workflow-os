# Authoritative Governance Scaffold Opt-In Plan

Status: Implemented.

Implementation evidence is captured in the
[Authoritative Governance Scaffold Opt-In Report](../concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_OPT_IN_REPORT.md).
Phase review is captured in the
[Authoritative Governance Scaffold Opt-In Review](../concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_OPT_IN_REVIEW.md).

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Profile-Controlled Authoritative Governance Activation Plan](profile-controlled-authoritative-governance-activation-plan.md)
- [Authoritative WorkReport Artifact Persistence Plan](authoritative-work-report-artifact-persistence-plan.md)
- [Existing Repo Governance Onboarding Plan](existing-repo-governance-onboarding-plan.md)
- [Current Product Contract](../user-guide/current-product-contract.md)

## 1. Executive Summary

Workflow OS already supports one closed project declaration that activates the
authoritative proportional-governance path:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

The declaration is validated, immutable-run-bound, and executable, but a new
repository user must currently edit `workflow-os.yml` by hand after running
`workflow-os init-repo-governance`. This creates avoidable friction at the
point where quiet, evidence-preserving governance should become easy to adopt.

This phase adds one explicit scaffold option:

```sh
workflow-os init-repo-governance --authoritative-governance
```

The option writes the already-supported declaration. It does not introduce a
new profile, infer activation, run a workflow, execute a check, or create
runtime state during scaffolding. Without the option, existing scaffold output
and later ordinary execution remain unchanged.

## 2. Goals

The implementation must:

1. add one explicit `init-repo-governance` opt-in flag;
2. write the closed `observe_and_report` and
   `workflow_os_project_validation` declaration only when selected;
3. preserve the existing manifest shape when the option is absent;
4. disclose enabled posture in normal and dry-run output while preserving
   existing output when absent;
5. produce a valid project when enabled;
6. make `first-run --verbose` report the declaration as supported and
   enforced;
7. preserve scaffold-only behavior and existing file-safety rules; and
8. document what later authoritative runs will do.

## 3. Strict Non-Goals

This phase does not add:

- automatic or inferred authoritative activation;
- a new proportional-governance decision or route;
- arbitrary commands or additional local-check profiles;
- automatic approval or model self-approval;
- provider execution, OpenShell integration, network access, or credentials;
- SideEffect execution or external writes;
- hosted behavior, enterprise stewardship, or schema families;
- workflow generation or promotion;
- recursive agents, agent swarms, or Level 3/4 autonomy; or
- release posture changes.

## 4. CLI Contract

The supported command becomes:

```text
workflow-os init-repo-governance [--output-dir <path>] [--agent generic|codex|claude] [--authoritative-governance] [--force] [--dry-run]
```

When selected, human output includes:

```text
authoritative_execution: enabled
authoritative_execution_profile: observe_and_report
authoritative_execution_local_check_profile: workflow_os_project_validation
```

When absent, existing output remains unchanged. Dry-run output uses the enabled
posture labels only when selected and writes nothing.

## 5. Runtime Consequence

The scaffold command itself remains non-executing. The declaration it writes
changes later project behavior:

- `run` and matching `approve` use the accepted authoritative path without a
  repeated compatibility flag;
- Core derives quiet, visible, approval-required, or denied posture from the
  current bounded inputs;
- the fixed Workflow OS project-validation check is the only local-check
  authority selected by this declaration;
- immutable run and approval-resume integrity remain enforced; and
- accepted terminal authoritative paths may persist the validated WorkReport
  artifact through the existing artifact gates.

The declaration does not authorize repository commands, provider calls, or
external writes.

## 6. Tests

Focused coverage must prove:

- the default scaffold omits `authoritative_execution`;
- the opted-in scaffold writes the exact closed declaration;
- both default and opted-in projects validate;
- first-run reports the opted-in declaration as enforced;
- dry-run discloses the intended posture without writing files or state; and
- existing scaffold, preservation, workflow demo, validation, runtime, and
  documentation tests continue to pass.

## 7. Final Recommendation

Ship this as an explicit experimental onboarding option. Do not make it the
default until external evaluation shows that its fixed validation authority,
quiet-success behavior, approval escalation, and artifact posture are
understood and useful across normal repositories.
