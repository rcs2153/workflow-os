# Authoritative Runtime Flag Retirement Plan

## 1. Executive Summary

Workflow OS has one validated project declaration for the closed authoritative
execution preview. Core binds that declaration into immutable run input and
derives current authority during fresh execution and approval reassessment.

The standalone `run --authoritative-governance` and
`approve --authoritative-governance` compatibility paths bypass that source by
letting a caller classify the command as authoritative. This phase retires
those two runtime switches. The identically named
`init-repo-governance --authoritative-governance` option remains because it
writes the validated project declaration; it does not grant runtime authority.

## 2. Goals

- Make the validated project declaration the only local activation source for
  the closed authoritative execution preview.
- Make immutable run activation the only authoritative approval-resume route.
- Reject retired runtime flag use before run or approval state mutation.
- Preserve ordinary undeclared execution.
- Preserve the scaffold opt-in that authors the supported declaration.
- Keep approval presentation, current-authority resolution, checks, reports,
  artifacts, and audit behavior unchanged after valid activation.

## 3. Non-Goals

This phase does not add:

- automatic approval or inferred authority;
- actor RBAC, enterprise identity, or delegated capability grants;
- OpenShell or another sandbox runtime;
- command, tool, credential, provider, or network access;
- SideEffect execution or external writes;
- schemas or additional local-check profiles;
- hosted behavior; or
- release posture changes.

## 4. Runtime Contract

`workflow-os run <workflow-id>` enters the authoritative path only when the
current validated project declares the exact supported activation:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

`workflow-os approve <run-id> <approval-id>` enters that path only when the
durable immutable run bundle contains the matching activation. Approval does
not create authority.

Passing `--authoritative-governance` to either command fails with stable code
`cli.authoritative_governance.runtime_flag_retired` before state is created or
changed.

## 5. Scaffold Boundary

`workflow-os init-repo-governance --authoritative-governance` remains
supported. It writes the closed declaration and associated validation
requirements. This is configuration authoring, not a per-command authority
assertion.

## 6. Compatibility And Migration

This is an intentional breaking change to an experimental preview CLI option.
Users of the former runtime flag should:

1. add the supported project declaration directly or regenerate the scaffold
   with the scaffold opt-in;
2. run project validation; and
3. invoke `run` and `approve` without the runtime flag.

Ordinary projects without the declaration retain ordinary execution.

## 7. Failure And Privacy Semantics

Retired flag use fails before project execution, run creation, approval
mutation, local checks, skill invocation, report generation, or artifact
writing. The error uses a stable code and bounded migration text. It does not
include run IDs, approval IDs, paths, commands, credentials, payloads, or
secret-like values.

## 8. Test Plan

- project-declared quiet execution remains successful without the flag;
- visible, approval-required, denied, verbose, JSON, artifact, and retry paths
  remain available through the declaration;
- approval resume derives activation from immutable run input;
- incomplete authoritative check posture still fails before run creation;
- `run --authoritative-governance` fails with the stable retirement code and
  creates no state;
- `approve ... --authoritative-governance` fails with the same stable code and
  changes no state;
- approval handoff output no longer recommends the retired flag;
- help and current CLI documentation no longer advertise the runtime flag; and
- scaffold opt-in behavior remains covered.

## 9. Documentation

Update current CLI and product-contract documentation, the roadmap, and this
phase report/review. Historical reports remain unchanged as records of the
behavior they reviewed.

## 10. Recommended Follow-Up

Resume the scoped runtime authority and capability projection sequence,
beginning with the next narrow current-authority receipt or required-context
phase selected from the roadmap.

Do not broaden provider mutation families or integrate OpenShell as an
authority source.
