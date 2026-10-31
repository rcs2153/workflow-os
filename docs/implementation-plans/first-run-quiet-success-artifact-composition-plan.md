# First-Run Quiet-Success Artifact Composition Plan

Status: Accepted after focused maintainer/security review and two required
corrections. Runtime implementation remains separately governed.

Related foundations:

- [First-Run Quiet-Success Artifact Queue Selection Report](../concepts/FIRST_RUN_QUIET_SUCCESS_ARTIFACT_QUEUE_SELECTION_REPORT.md)
- [First-Run Governed WorkReport Artifact Plan](first-run-governed-work-report-artifact-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Decision Axes And Workload Inference Plan](proportional-governance-decision-axis-and-inference-plan.md)

## 1. Decision

Introduce a second exact generated first-run workflow for governed report
artifacts. Do not remove, reinterpret, or auto-approve the existing
`local/first-run-governance` workflow.

The contract split is:

| Contract | Purpose | Default posture |
| --- | --- | --- |
| `local/first-run-governance` | Optional approval/audit demonstration | Explicit human approval |
| `local/first-run-report-artifact` | Explicit bounded real analysis and local artifact | Core-derived proportional governance |

The artifact workflow uses a new exact low-sensitivity
`local/first-run-report-artifact` skill, a new exact generated
observe-and-report policy, the existing local allow policy, and the required
authoritative project-validation check. The existing medium-sensitivity demo
skill remains unchanged. The artifact contract declares no human approval
minimum. That absence does not guarantee quiet success: Core still
composes every current policy, profile, authority, evidence/check, sensitivity,
SideEffect, runtime-escalation, and future steward minimum.

## 2. Why A Separate Contract

The current artifact command executes the same workflow used to demonstrate
approval behavior. Its policy and step both require approval, so two approval
decisions are truthful. Silently skipping them would violate the declared
contract.

A separate versioned contract makes the product distinction inspectable:

- the demo teaches approval and audit mechanics;
- the artifact operation performs bounded useful work;
- the user does not choose the governance route;
- explicit invocation requests work but grants no authority; and
- Core may interrupt when current facts require it.

## 3. Generated Contract

New authoritative scaffolds should add:

- `workflows/first-run-report-artifact.workflow.yml` with ID
  `local/first-run-report-artifact`;
- `skills/first-run-report-artifact.skill.yml` with ID
  `local/first-run-report-artifact` and low approval sensitivity;
- `policies/observe-and-report.policy.yml` with ID
  `default/observe-and-report`; and
- one focused generated test proving no declared human gate, required project
  validation, no external capability, and bounded local-report posture.

The workflow should:

- contain one `first-run-report` step using
  `local/first-run-report-artifact`;
- select governance profile `observe_and_report`;
- reference `default/observe-and-report` and `local/allow`;
- require the closed `workflow_os_project_validation` profile with no reuse;
- declare no external capabilities or provider SideEffects;
- require audit and terminal report posture; and
- avoid schema fields not already supported by the current contract.

The new policy uses the already parsed and enforced `allow_local` effect. It
must not introduce a prose-only or unknown effect. The artifact skill's low
sensitivity is limited to the bounded analyzer and create-only local artifact;
it is not a general classification for repository commands or other skills.

Exact canonical hashes for the workflow, policy, artifact skill, local policy,
and test
must be allowlisted at the trusted handler boundary. Customized or partially
upgraded contracts fail before run creation.

## 4. Scaffold And Compatibility

`init-repo-governance` should create the new missing files additively. It must
preserve unmanaged agent instructions and refuse conflicting existing paths.
It must not require `--force` merely to add absent canonical files.

For already scaffolded repositories:

- ordinary `first-run` remains available;
- the approval/audit demo remains available and unchanged;
- a new artifact start missing the new exact contract fails before state with a
  stable `contract_upgrade_required` diagnostic and additive scaffold command;
- no hidden built-in workflow substitutes for missing project declarations;
- existing customized files are not overwritten; and
- no migration is performed during `first-run` itself.

Existing in-flight artifact runs remain bound to their immutable legacy
workflow. `--run-id` resume, denial, replay, and recovery must route by durable
run identity and presentation records, not by the current scaffold default.
New starts use only `local/first-run-report-artifact`.

## 5. Authoritative Input And Routing

The command continues to compute the accepted bounded analysis commitment
before run creation. The new workflow, policy, test, and all current project
definition hashes join that immutable commitment.

Core derives the route from:

- exact immutable workflow, skill, policy, test, and manifest identity;
- governance profile and every declared policy result;
- current actor and authority posture;
- authoritative project-validation result;
- evidence and check completeness;
- sensitivity and bounded local-write posture;
- external SideEffect absence;
- prior accepted disposition and disclosure obligation; and
- current runtime escalation facts.

The caller supplies facts through reviewed typed boundaries but cannot supply a
route. Unknown safety-relevant facts cannot produce `Proceed`.

For this local-only operation, request authority comes from one CLI-private,
one-use `ExplicitLocalArtifactRequest` value constructed only by the explicit
`--write-report-artifact` invocation after project-root and output-root
validation. Its commitment binds actor, project, workflow, run, immutable
analysis, artifact-store root, and operation version into the immutable run
context before state creation. Core consumes it for this exact request only.

This value is not approval, a policy result, a reusable capability, an
authority receipt, a credential, or permission for another local write. It is
never accepted from YAML or JSON, never serialized as bearer authority, and
cannot satisfy a steward or policy approval minimum. If the trusted local CLI
request boundary cannot be represented without caller-authored reusable
authority, implementation must stop for a separately reviewed authority plan.

Routing behavior is closed:

- `Proceed + Quiet`: execute once and persist the report artifact;
- `Proceed + Visible`: execute once, persist, and emit required bounded
  disclosure;
- `RequireApproval`: persist a complete proof-backed handoff and stop before
  handler invocation;
- `Deny`: record the governed denial and perform no handler or artifact write.

Any reassessment may preserve or strengthen posture. It may not downgrade a
prior `RequireApproval`, `Deny`, or `Visible` minimum for the active action.

## 6. Local Write And Disclosure

The artifact is a real create-only filesystem mutation. It is not an external
provider SideEffect and must not be labeled as one. The report and CLI disclose
local durable run state and artifact persistence separately from external
SideEffects.

`Quiet` means immediate disclosure is not required by governance. The CLI may
still render a concise completion because it is an operator-selected surface.
Rendering a quiet result does not change the disclosure decision. `Visible`
requires bounded disclosure even in non-interactive JSON or future hosted
surfaces.

## 7. Approval And Artifact Gates

Quiet and visible proceed routes must not create:

- `ApprovalRequested` or `ApprovalGranted` events;
- approval-presentation records;
- approval proof markers; or
- authority receipts that imply an approval occurred.

Artifact persistence must derive its high-assurance and proof-marker
requirements from the exact new workflow and current route. Existing artifact
integrity, referential, check-citation, terminal-status, create-only, and replay
gates remain mandatory.

An escalated approval route uses the existing presentation-proof and fresh
reassessment path. A grant must reassess the same immutable contract and
current facts before execution. Denial remains source-free and artifact-free.

## 8. Replay And Recovery

Exact terminal replay returns the existing equal artifact without analysis,
check, or skill re-execution. Missing-artifact recovery must reproduce the
immutable bounded analysis, rerun only the canonical project-validation check,
cite that fresh result, and never rerun the completed skill.

Recovery must preserve the route that authorized the completed run. It may not
invent retroactive approval proof or recalculate a quieter route. Changed
definitions, metadata, governance inputs, or artifact content fail closed.

## 9. Output Contract

Human and JSON output should identify:

- workflow and run identity;
- execution disposition and disclosure obligation;
- whether approval was required;
- check-result reference;
- report and artifact identity;
- persisted, replayed, recovered, denied, or blocked posture; and
- inspect or upgrade next action.

Output must not include report bodies, source contents, command output,
credentials, policy payloads, or raw evidence.

## 10. Acceptance Matrix

Focused tests must prove:

1. new scaffold creates both contracts without changing the demo;
2. additive upgrade preserves unmanaged content and refuses conflicts;
3. incomplete upgrade fails before run creation;
4. exact complete low-risk facts produce `Proceed` without approval records;
5. visible proceed emits bounded disclosure without approval;
6. every explicit approval minimum produces a proof-backed wait;
7. unknown authority, failed checks, policy denial, sensitivity escalation, and
   substituted definitions cannot produce quiet execution;
8. request-authority substitution, replay across a run or project, changed
   output root, and missing explicit invocation fail before execution;
9. approval grant performs fresh reassessment;
10. denial invokes no handler and writes no artifact;
11. local writes and external SideEffect absence are both truthful;
12. exact replay and missing-artifact recovery preserve route truth;
13. legacy pending and completed artifact runs remain resumable/replayable;
14. ordinary first-run remains non-mutating; and
15. the mock approval/audit demo remains approval-gated.

A disposable external repository must prove scaffold, additive upgrade,
quiet/visible success as applicable, approval escalation, denial, persistence,
replay, recovery, and healthy state.

## 11. Non-Goals

This plan does not authorize caller-selected routes, automatic approval, model
self-approval, arbitrary commands, source inspection, providers, network,
credentials, external SideEffects, OpenShell, broader profiles, public schema
or SDK changes, hosted administration, enterprise identity, nested harnesses,
or release changes.

## 12. Implementation Sequence

1. add exact additive scaffold contracts and validation fixtures;
2. route new starts to the new workflow while preserving legacy durable runs;
3. compose authoritative current facts and closed route handling;
4. derive artifact gates without fabricated approval proof;
5. add replay, recovery, output, and migration diagnostics;
6. run the closed test matrix and disposable-repository proof; and
7. perform focused maintainer/security review before acceptance.

No step may broaden behavior to arbitrary workflows or profiles.

## 13. Review Gate

Focused review must reject implementation if the plan cannot prove exact
contract separation, enforced policy semantics, complete current facts,
monotonic escalation, legacy-run compatibility, or truthful artifact gates.
