# Trusted-Host Local Cargo Execution-Provider Suitability Re-evaluation Review

## 1. Executive Verdict

**Decision accepted; proceed to an OpenShell `v0.1.5` execution-evidence
sufficiency matrix.**

The re-evaluation is accurate, appropriately conservative, and materially
updates the old readiness basis without treating upstream progress as runtime
proof. Stable OpenShell `v0.1.5` closes or narrows enough former API gaps to
justify a new exact evidence matrix. It does not yet justify provider wiring,
Cargo execution, a compatibility transport, or a live smoke.

## 2. Scope Verification

The phase stayed within its decision-only boundary. It inspected official
public upstream source and documentation, authored one bounded decision
document, and advanced the roadmap queue. It added no Rust, provider, runner,
Cargo process, OpenShell installation, sandbox, smoke, admission, persistence,
event, audit, schema, SDK, CLI, example, SideEffect, write, authority, fork, or
release behavior.

## 3. Source Boundary Assessment

The decision records exact official refs:

- stable tag `v0.1.5` at
  `bdb956f2cd65a635f1294fccf1d138d263bf650f`; and
- upstream `main` at
  `c4688aa8715ea75d6c39fe468361dbbc16c691fa` on 2026-10-24.

Focused comparison confirms that the public gateway and compute-driver
protobuf files relevant to this decision are unchanged between those refs.
The inspected `main` changes add observability guidance but do not convert
best-effort OCSF collection into complete operation-bound evidence. Selecting
stable `v0.1.5`, rather than unreleased `main`, as the next compatibility
assessment target is correct.

The decision clearly distinguishes public source-contract inspection from
release-asset identity, installed binary identity, selected-driver behavior,
platform enforcement, and live sandbox proof.

## 4. Native Boundary Assessment

The continued rejection of a direct native Cargo process is correct. The
accepted native observer proves process-free host-input observations only. It
does not own filesystem and network containment, process-tree lifecycle,
transient-write observation, effective-control attestation, or restart
reconciliation.

The current composition's exact `DeniedBeforeStart` result with
`ObservationUnavailable` remains the honest posture. Provider adjacency or a
direct host runner would not close the missing custody and evidence boundary.

## 5. OpenShell Advance Assessment

The decision accurately identifies meaningful stable additions:

- durable request IDs for create and exec admission;
- optimistic resource-version checks on selected mutations;
- driver-observed resolved workload identity containing an immutable
  image/rootfs/config digest;
- generation-bound driver fence evidence;
- typed resource and extension capability surfaces;
- durable desired-state operation records;
- provisioning state with pending driver-operation identity;
- typed deletion outcomes and targeted sandbox identity; and
- session- and revision-bound endpoint status reports.

These additions deserve a new matrix because several `v0.0.101` classifications
are now stale. The decision does not overstate them. In particular, schema
presence is not implementation conformance, non-empty evidence, cross-driver
portability, or operation completeness.

## 6. Remaining Blocker Assessment

### 6.1 Exec Reconciliation

The blocker is correctly retained. Official migration guidance says an
admitted exec retry cannot replay output, exit code, or the stream. Durable
launch admission prevents duplicate execution but does not let Workflow OS
recover the exact terminal outcome after stream loss. The result must remain
uncertain or reconciliation-required, never inferred success.

### 6.2 Driver Owner Loss

The blocker is correctly retained. Official lifecycle guidance states that a
gateway crash before recording a driver response can leave the pending
operation unresolved indefinitely, with cleanup, retry, and deletion blocked.
The retained operation ID protects against stale callbacks but is not a
resolution API.

### 6.3 Observation Completeness

The blocker is correctly retained. OCSF JSON records are structured and useful,
but the official documentation explicitly calls collection best-effort. The
bounded console queue can drop lines, file retention is finite, and no
operation-bound manifest proves watermarks, flush state, dropped-event posture,
integrity, and complete interval coverage.

Endpoint reports provide stronger policy-scoped network evidence, but they do
not cover every process, filesystem, protected-write, resource, and terminal
fact required for one Cargo invocation.

### 6.4 Cleanup

The decision correctly recognizes substantial progress without declaring the
gap closed. A typed accepted/completed/absent outcome and immutable targeted
sandbox ID improve reconciliation. The stable contract still distinguishes
gateway-record deletion from downstream garbage collection and requires
observation until disappearance for asynchronous cleanup. It does not expose
one complete immutable receipt for exact generation reclamation, descendant
termination, credential purge, completion, and degradation.

### 6.5 Effective Controls And Live Proof

The decision correctly requires the next matrix to select a concrete
gateway/driver/platform combination. Every mandatory control fact must be
available, non-empty, authoritative, exactly bound, and fail closed. A later
live smoke remains necessary even if source analysis finds no API blocker.

## 7. Provider Decision Assessment

The candidate classification is sound:

- reject a direct native Cargo process;
- reject observer-plus-direct-process composition;
- supersede `v0.0.101` as the next assessment target without rewriting its
  historical verdict;
- advance stable `v0.1.5` to a refreshed matrix only;
- treat current `main` as directional evidence, not a compatibility pin; and
- reject a Workflow OS-specific OpenShell fork.

This sequencing preserves Workflow OS as the governance and evidence layer
while leaving execution containment with an optional specialized provider.

## 8. Fork Assessment

The no-fork decision remains correct. The unresolved needs are general sandbox
lifecycle, reconciliation, observation, and cleanup primitives. A fork would
make Workflow OS responsible for a broad security runtime surface and would
fragment upstream maintenance before incompatibility has been demonstrated.

## 9. Privacy And Error Assessment

The proposed future projection remains payload-minimizing. Core may receive
stable IDs, commitments, revisions, typed postures, bounded counts,
timestamps, sanitized reasons, and governed evidence references. Raw command
I/O, OCSF records, source contents, policy bodies, environment values,
credentials, paths, endpoints, and upstream errors remain outside Core.

The decision also preserves the crucial semantic rule: missing, stale,
degraded, best-effort, and unreconciled observations cannot be relabeled as
successful enforcement.

## 10. Blockers

None for accepting this decision phase.

Provider wiring remains blocked by the facts identified in the decision. Those
are implementation-entry blockers, not defects in this decision document.

## 11. Non-Blocking Follow-Ups

- The new matrix should name one exact driver and platform rather than relying
  on cross-driver schema intent.
- It should identify which `v0.1.5` fields are mandatory and reject empty or
  unsupported values.
- It should distinguish request replay from terminal-outcome replay.
- It should define a first-class reconciliation result for stream loss and
  pending-driver owner loss.
- It should retain the historical `v0.0.101` matrix as an evidence record
  rather than editing it into the new verdict.
- It should record the absence of an upstream maintainer response on Discussion
  #2661 without treating third-party comments as acceptance.

## 12. Recommended Next Phase

Create and review the **OpenShell `v0.1.5` execution-evidence sufficiency
matrix**.

That phase is source and contract analysis only. It must not implement Rust,
install or execute OpenShell, invoke Cargo, add a compatibility transport, run
a live smoke, wire a provider, change admission, or broaden authority. If the
matrix closes every mandatory API fact, the following phase may plan one
bounded compatibility client and no-write live proof. If any mandatory fact is
unavailable, provider wiring remains blocked.

## 13. Validation

- official `v0.1.5` and current-main refs: resolved to exact commits;
- focused stable/current protobuf comparison: passed;
- focused stable source and official documentation inspection: completed;
- `npm run check:docs`: passed under the repository-pinned Node 20 runtime;
- `git diff --check`: passed; and
- Rust formatting, linting, and tests: not run because this review changes
  documentation only and no Rust or runtime behavior.

## 14. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792849922869056000-2`.
- Approval ID:
  `approval/run-1792849922869056000-2/review-scope-approved`.
- Approval presentation ID: `presentation/675f90adfe081d00`.
- Approval presentation content hash:
  `675f90adfe081d00fbc511029f1a543447b3f65d8eb3736a86fa2bdbae9203b9`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Out-of-kernel work: the delegated maintainer compared official upstream
  refs, inspected stable API and documentation surfaces, reviewed the decision
  and roadmap, authored this review, and ran validation. The kernel governed
  scope and approval but did not inspect files, edit documentation, or run
  validation.

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
