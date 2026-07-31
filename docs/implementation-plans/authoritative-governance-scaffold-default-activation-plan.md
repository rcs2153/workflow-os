# Authoritative Governance Scaffold Default Activation Plan

Status: Deferred pending Core-owned runtime-fact and disclosure routing.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Authoritative Governance Scaffold Opt-In Plan](authoritative-governance-scaffold-opt-in-plan.md)
- [Authoritative Governance Scaffold External Repository Evaluation](../concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_EXTERNAL_REPOSITORY_EVALUATION.md)
- [Authoritative Governance Scaffold Runtime Contract Blocker Fix Review](../concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_RUNTIME_CONTRACT_BLOCKER_FIX_REVIEW.md)
- [Current Product Contract](../user-guide/current-product-contract.md)

## 1. Executive Summary

Workflow OS has proved one complete, closed authoritative onboarding path:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

The corresponding scaffold includes the exact canonical project-validation
requirement. The path has been exercised in disposable external repositories,
preserves existing agent guidance, executes one fixed source-read-only and
network-disabled validation contract, routes proportional governance in Core,
keeps authored workflow approvals separate, persists a terminal WorkReport
artifact, and produces quiet-success output for eligible completion.

The remaining adoption cost is that users must know to select
`--authoritative-governance`. That makes the evidence-preserving path an expert
feature even though it is now the most credible default posture for a newly
generated Workflow OS governance envelope.

The proposed default remains directionally correct, but implementation is
deferred. Focused code inspection found that the CLI still:

- constructs runtime-fact records for the selected workflow;
- marks non-selected steps' evidence/check posture as `Satisfied`; and
- predicts whether visible disclosure will be required before the authoritative
  local check has produced its same-call result.

The default scaffold currently contains one step, so its exercised happy path
does not exploit the multi-step preclassification. The project declaration is
project-wide, however, and the runtime surface can accept other workflows.
Default activation would therefore promote a route whose complete fact and
disclosure selection is not yet owned by Core.

The prerequisite is defined in the
[Core-Owned Authoritative Runtime-Fact Derivation Plan](core-owned-authoritative-runtime-fact-derivation-plan.md).
After that implementation and review pass, this plan can resume unchanged in
product intent: default the already-supported closed declaration for newly
generated scaffolds, add an explicit opt-out, preserve the positive flag, and
leave existing projects unchanged.

## 1.1 Prerequisite Finding

The following code paths prevent default activation:

- `authoritative_governance_workflow_inputs(...)` in the CLI classifies
  evidence/check facts for steps whose checks were not observed in the
  authoritative same call.
- `authoritative_visible_disclosure_required(...)` independently predicts the
  route from optimistic `Sufficient`, `Satisfied`, and `None` facts so the CLI
  can decide whether to inject visible-delivery dependencies.
- the public authoritative executor request can represent multiple workflow
  steps even though the accepted project-validation composition observes one
  selected step only.

The prerequisite must remove those caller decisions, constrain the closed
profile to its proven one-step boundary, and let Core conditionally consume
visible delivery only after the actual source-bound assessment selects that
route.

## 2. Product Decision

New repository governance scaffolds should default to the least interruptive
currently enforceable posture:

```text
observe_and_report
+ workflow_os_project_validation
+ proportional route selection
+ quiet success when eligible
+ durable evidence and WorkReport artifact posture
```

This is not automatic approval. The selected proportional-governance route may
still require visible disclosure, blocking approval, or denial. Authored
workflow approval gates remain separate and continue to apply.

## 3. Goals

The implementation must:

1. make the closed authoritative declaration and canonical local-check
   requirement the default output of `init-repo-governance`;
2. add `--no-authoritative-governance` for an explicit legacy scaffold;
3. continue accepting `--authoritative-governance` for script compatibility;
4. reject simultaneous positive and negative flags before any write;
5. disclose enabled or disabled posture in normal and dry-run output;
6. preserve existing file safety and `AGENTS.md` merge behavior;
7. leave existing repositories and manifests unchanged;
8. keep the scaffold command itself non-executing;
9. preserve the exact closed command contract and current runtime authority
   derivation; and
10. update user-facing help and current-product documentation honestly.

## 4. Strict Non-Goals

This phase does not authorize:

- inferred repository commands or source inspection;
- additional local-check profiles;
- automatic or delegated approval;
- removal or weakening of authored workflow approval gates;
- new proportional-governance profiles or decision semantics;
- provider execution or OpenShell integration;
- sandbox execution, credentials, or network access;
- SideEffect execution or external writes;
- hosted administration or enterprise stewardship;
- mutation of existing project manifests;
- automatic workflow generation or promotion;
- examples, schemas, SDK changes, or release posture changes; or
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 5. CLI Contract

The command surface becomes:

```text
workflow-os init-repo-governance \
  [--output-dir <path>] \
  [--agent generic|codex|claude] \
  [--authoritative-governance | --no-authoritative-governance] \
  [--force] \
  [--dry-run]
```

Behavior:

- no posture flag: generate the closed authoritative scaffold;
- `--authoritative-governance`: generate the same closed authoritative
  scaffold;
- `--no-authoritative-governance`: generate the legacy undeclared scaffold;
- both flags: fail with a stable usage error before writes.

The positive flag remains supported because existing automation may already
use it. It should not be described as required after this phase.

## 6. Runtime Boundary

Scaffolding must still perform no runtime work. It writes only validated
configuration.

Later runs in a default scaffold:

- derive activation from the validated project declaration;
- bind activation and the canonical check declaration into the immutable run
  bundle;
- execute only the fixed `workflow-os validate` local-check contract;
- resolve current local project authority inside Core;
- select quiet, visible, approval-required, or denied behavior through the
  accepted proportional-governance dispatcher;
- preserve separate authored workflow approval gates; and
- persist a terminal WorkReport artifact only through the existing reviewed
  gates.

The scaffold does not authorize arbitrary repository commands, provider calls,
or external mutation.

## 7. Compatibility And Migration

This is a default change for newly generated scaffolds, not a migration.

- Existing repositories are not inspected for automatic activation.
- Existing `workflow-os.yml` files are not rewritten.
- Re-running scaffolding without `--force` retains current fail-closed file
  protection.
- `--force` retains its current explicit replacement behavior.
- Callers that require the legacy scaffold can select
  `--no-authoritative-governance`.
- The current positive flag remains accepted and idempotent with the new
  default.

## 8. Test Plan

Focused tests must prove:

1. default scaffold contains the exact closed authoritative declaration;
2. default workflow contains the exact canonical project-validation
   requirement;
3. explicit positive selection produces the same governed posture;
4. explicit negative selection omits both declaration and requirement;
5. contradictory flags fail before any file or state write;
6. default and explicit-negative scaffolds validate;
7. default `first-run --verbose` reports supported enforced posture;
8. default scaffold reaches the accepted authoritative runtime path;
9. quiet success remains concise when eligible;
10. proportional and authored approvals remain separate;
11. terminal completion persists exactly one WorkReport artifact;
12. dry-run reports the selected posture and writes nothing;
13. existing agent guidance remains preserved;
14. unknown options still fail before writes; and
15. existing CLI, Core, schema-contract, integration, and docs tests pass.

## 9. Privacy And Safety

The default adds only fixed identifiers and closed policy values. It stores no
source content, command output, provider payload, path, environment value,
credential, or token.

The local check remains source-read-only and network-disabled. Any mismatch in
the project declaration, immutable activation, canonical requirement, current
authority, local-check result, approval context, or artifact gate continues to
fail closed.

## 10. OpenShell Relationship

OpenShell remains a potential optional execution provider, not part of this
phase. Default scaffold activation must not imply sandboxing.

Before OpenShell implementation, Workflow OS still needs a provider-neutral
workload/materialization contract, policy translation and effective-policy
attestation, typed sandbox and image identity, access-material resolution,
provider lifecycle and reconciliation, and a dedicated threat review.

## 11. Implementation Sequence

1. Replace the scaffold command's boolean option with an explicit internal
   activation selection whose default is enabled.
2. Add negative-flag parsing and contradictory-flag rejection.
3. Reuse the existing authoritative manifest and workflow templates unchanged.
4. Update normal, dry-run, and help output.
5. Update and extend focused CLI tests.
6. Update roadmap and current-product documentation.
7. Run full validation and a disposable-repository proof.
8. Perform a focused maintainer review before merge.

## 12. Final Recommendation

Do not implement default activation yet.

First implement and review the Core-owned runtime-fact and disclosure-routing
prerequisite. Then return to this plan without broadening its product scope.

Do not generalize the command contract, infer repository-specific checks, add
providers, or make OpenShell the runtime in this phase. The product value is
that safe low-risk governance becomes the easy path while the kernel retains
authority, evidence, approval, and reporting boundaries.
