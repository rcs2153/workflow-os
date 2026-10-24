# OpenShell v0.1.5 Upstream Gap Refresh Report

## 1. Executive Summary

The accepted OpenShell upstream attestation proposal and submitted Discussion
#2661 now have a source-current stable `v0.1.5` gap record and one bounded
follow-up comment draft.

The refresh credits upstream progress before identifying remaining selected-
boundary gaps. It does not post externally, implement a provider, install or
execute OpenShell, run Cargo, or authorize a fork.

## 2. Scope Completed

- Refreshed the proposal status and problem statement for stable `v0.1.5`.
- Added an exact release, driver, platform, and future-workload boundary.
- Mapped original proposal needs to stable progress and remaining facts.
- Recorded exec stream-loss and driver-owner-loss reconciliation gaps.
- Recorded the Docker production workload/fence projection gap.
- Updated the incremental upstream sequence and acceptance package.
- Preserved the original submitted Discussion #2661 body unchanged.
- Added one separate copy-ready stable follow-up comment draft.
- Added focused review requirements and a separate submission boundary.
- Updated the roadmap queue honestly.

## 3. Scope Explicitly Not Completed

This phase did not:

- post a discussion comment, issue, pull request, or other upstream mutation;
- install, build, start, or execute OpenShell;
- invoke Cargo or add a native process runner;
- add a compatibility transport, live smoke, or provider wiring;
- add access material, admission, persistence, events, schemas, SDKs, CLI
  behavior, examples, SideEffects, writes, or release changes;
- accept the follow-up draft without focused review; or
- fork OpenShell.

## 4. Stable Delta Summary

Stable `v0.1.5` materially advances the original proposal through:

- bounded durable request identity for create and exec;
- selected resource-version preconditions;
- durable typed desired-state operations;
- pending driver-operation identity and stale-callback fencing;
- revision- and session-bound endpoint status;
- typed deletion outcomes and immutable target identity; and
- public resolved-workload and fence-evidence vocabulary.

The refresh does not equate those additions with complete attestation.

## 5. Remaining Gap Summary

The selected Docker boundary still lacks:

1. production projection of non-empty resolved workload identity and fence
   evidence;
2. durable terminal exec lookup after stream loss;
3. explicit resolution of driver-operation ownership lost with a gateway;
4. an operation-bound complete observation manifest;
5. complete applied-control and degradation capability negotiation; and
6. an exact terminal cleanup receipt for one generation.

These facts keep compatibility transport, live smoke, and provider wiring
blocked.

## 6. Discussion Preservation

The original submitted title and body remain intact in the discussion-draft
document as historical evidence. The refresh adds a separate follow-up comment
section rather than rewriting the content that was submitted and committed in
the submission report.

Current public-thread inspection found two comments from an external
researcher and no recorded OpenShell maintainer response. External comments do
not constitute upstream acceptance.

## 7. Follow-Up Draft Boundary

The new draft:

- thanks and credits upstream progress first;
- names the exact stable source and selected Docker boundary;
- distinguishes schema presence from Docker production projection;
- asks about concrete existing or planned paths;
- proposes general OpenShell increments rather than Workflow OS-specific
  types; and
- retains the optional-provider and no-fork posture.

The draft is not approved for posting by this phase. Focused review and a
separately governed external-action gate remain mandatory.

## 8. Privacy And Security Summary

The refresh contains only public source references, typed lifecycle facts,
bounded classifications, and architectural questions. It includes no token,
credential, access material, private payload, host path, raw command output,
raw OCSF payload, environment value, or unpublished vulnerability.

Future Workflow OS projection remains payload-minimizing: stable IDs,
commitments, typed postures, bounded counts, timestamps, sanitized reasons,
and governed evidence references only.

## 9. Validation Summary

Required at phase close:

- `npm run check:docs` under the repository-pinned Node 20 runtime;
- `git diff --check`;
- changed-surface inspection; and
- governed phase-close event/proof inspection.

Rust formatting, linting, and tests are intentionally not required because
this phase changes documentation only and no Rust or runtime behavior.

## 10. Governed Phase Record

- Workflow ID: `dg/d`.
- Run ID: `run-1792851903832240000-2`.
- Approval ID:
  `approval/run-1792851903832240000-2/planning-approved`.
- Approval presentation ID: `presentation/2b18709db1615f45`.
- Approval presentation content hash:
  `2b18709db1615f45bc7e658eb850218df79716e226ce6949166d995294d1e3a6`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact documentation-refresh scope.
- Out-of-kernel work: the delegated maintainer read official public source and
  the current public discussion, edited documentation, and ran validation.
  The kernel governed scope and approval but did not inspect upstream source,
  edit files, post externally, or run validation.

Phase-close result:

- phase status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0;
- event kinds: 1 `RunCreated`, 1 `RunValidated`, 1 `RunStarted`, 6
  `StepScheduled`, 8 `PolicyDecisionRecorded`, 1 `ApprovalRequested`, 1
  `ApprovalGranted`, 1 `RunResumed`, 6 `SkillInvocationRequested`, 6
  `SkillInvocationStarted`, 6 `SkillInvocationSucceeded`, and 1
  `RunCompleted`; and
- approval-presentation enforcement: proof-enforced with one matching
  presentation record and event marker.

Validation result:

- `npm run check:docs`: passed under the repository-pinned Node 20 runtime;
- `git diff --check`: passed;
- changed-surface inspection: only the approved documentation files changed;
  and
- Rust formatting, linting, and tests: not run because this phase changed
  documentation only and no Rust or runtime behavior.

## 11. Remaining Known Limitations

- The follow-up draft has not yet passed focused review.
- No OpenShell maintainer response has been recorded.
- Stable `v0.1.5` remains insufficient for the selected provider boundary.
- A later stable source reassessment remains necessary after upstream changes.
- A live selected-platform proof remains necessary after source contracts are
  sufficient.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the refreshed `v0.1.5` gap
record and follow-up comment draft.

The review must verify source fidelity, credit and tone, selected-driver
claims, privacy, no-fork posture, non-duplication, and external-action
boundaries. It must not post upstream, execute OpenShell or Cargo, add a
transport or provider, or change runtime behavior.
