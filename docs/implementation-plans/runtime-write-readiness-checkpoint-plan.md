# Runtime Write-Readiness Checkpoint Plan

## 1. Executive Summary

Workflow OS has advanced from model-only write readiness into explicit local
runtime composition for one GitHub pull request comment lane. The current system
can model write preflight, proposed SideEffect records, approval linkage,
attempted/completed/failed lifecycle transitions, provider-call orchestration,
eligible workflow event proof, reconciliation disclosure, strict artifact
event-proof gates, provider lookup/recovery posture, and operator recovery
summaries.

That still does not mean Workflow OS should enable broader or default writes.
This checkpoint defines what must be true before any future phase expands from
explicit, caller-supplied, GitHub PR comment-only helpers toward broader
write-capable adapter behavior.

This plan does not implement code. It does not authorize provider writes,
hidden auth loading, automatic retries, automatic repair, CLI mutation commands,
workflow schema changes, examples, hosted behavior, reasoning lineage,
recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 2. Goals

- Consolidate the current runtime write-readiness boundary.
- Identify the remaining gates before broader write-capable adapter behavior.
- Preserve the invariant that default executor paths do not write providers.
- Keep live provider calls explicit, local, caller-supplied, and narrow.
- Keep durable workflow event proof distinct from provider response and lookup
  observation.
- Keep report artifact writing gated by event proof, approval linkage,
  high-assurance posture when required, and referential integrity.
- Define the next code-bearing phase only after this checkpoint is reviewed.

## 3. Non-Goals

Do not implement or authorize in this checkpoint:

- default provider writes;
- automatic provider calls;
- hidden auth loading from environment, keychain, GitHub CLI, git remotes,
  config files, OAuth, or secret managers;
- automatic retries;
- automatic repair or reconciliation mutation;
- provider lookup without explicit caller input;
- CLI mutation commands;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- broad GitHub write support;
- Jira, CI, or other provider writes;
- enterprise RBAC, IdP, quorum, revocation, or policy administration;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Implemented Boundary

Implemented write-readiness foundations include:

- adapter write preflight model/helper;
- GitHub PR comment request/response model;
- fixture and dry-run validation without provider calls;
- proposed GitHub PR comment SideEffect record composition;
- proposed SideEffect record persistence;
- proposed/attempted/completed/failed SideEffect lifecycle transitions;
- explicit executor lifecycle event append helpers;
- approval-side-effect linkage and high-assurance approval disclosure gates;
- no-provider attempted and outcome orchestration;
- injected provider-call trait and concrete injected-transport provider client;
- provider write reconciliation model/helper;
- explicit executor-integrated provider-write helper;
- completed/failed event append for eligible reconciled provider outcomes;
- provider reconciliation disclosure projection;
- strict provider report artifact event-proof gate helper;
- provider event-proof recovery classification;
- injected provider lookup reconciliation helper;
- operator recovery summary helper and bounded CLI read surface;
- provider-call orchestration gate clarity projection.

All of these remain explicit and opt-in. Default `LocalExecutor::execute(...)`
still does not perform provider writes.

## 5. Remaining Readiness Gates

Before broader write behavior, Workflow OS needs reviewed answers for these
gates:

1. Auth loading boundary: how credentials are supplied without ambient
   authority or leakage.
2. Operator recovery boundary: how humans resolve ambiguous provider/local
   outcomes without fabricating event proof.
3. Retry boundary: whether any retry is ever allowed after provider ambiguity.
4. Artifact boundary: when provider-write results may be written into durable
   report artifacts.
5. Audit boundary: whether provider-write lifecycle outcomes need dedicated
   audit records beyond workflow event projection.
6. CLI boundary: whether any local command may expose provider mutation.
7. Schema boundary: whether workflow specs may declare write requirements.
8. Adapter expansion boundary: what requirements a second provider operation
   must satisfy before implementation.
9. Stewardship boundary: who is allowed to approve write-capable workflow or
   adapter configuration.
10. Test environment boundary: how live sandbox writes are proven without
    risking production targets.

If any gate lacks a reviewed answer for a provider operation, that operation
remains unsupported.

## 6. Default Executor Policy

Default executor behavior must remain write-denied.

Rules:

- `LocalExecutor::execute(...)` must not call providers.
- Report-bearing executor paths must not write providers by default.
- Provider-write helpers must remain explicit APIs with caller-supplied
  provider/auth inputs.
- Provider-write failures after a workflow run exists must not silently rewrite
  workflow pass/fail semantics.
- Ambiguous provider outcomes must block retry and require operator recovery.

Any future change to default executor write posture requires separate planning,
review, and likely an ADR.

## 7. Auth Loading Policy

Auth remains the sharpest unresolved boundary.

Near-term recommendation:

- continue to require caller-supplied auth/provider inputs;
- do not read environment variables, keychains, GitHub CLI state, git remotes,
  or config files inside core helpers;
- treat credential possession as capability to call a provider, not authority
  to perform the write;
- keep auth values out of Debug, serialization, errors, events, audit records,
  SideEffect records, WorkReports, and artifacts.

Future auth loading work should be its own reviewed phase and should start with
an explicit local auth-source model before any provider mutation command exists.

## 8. Operator Recovery Policy

Provider lookup and recovery summaries are implemented as explicit local
helpers, but recovery remains advisory.

Rules:

- remote provider observation must not fabricate local workflow event proof;
- missing event proof must keep strict artifact gates closed;
- operator recovery summaries may recommend next actions but must not mutate
  SideEffect records, workflow events, or report artifacts;
- any future recovery mutation must require fresh proof, explicit approval,
  and a separate reviewed apply path.

## 9. Artifact And Report Policy

Report artifacts must remain stricter than in-memory reports.

Rules:

- in-memory WorkReport disclosure may distinguish provider/local/reconciliation
  posture;
- report artifact writing must require durable event proof when configured;
- provider lookup observation may inform recovery posture but must not satisfy
  event-proof gates by itself;
- missing or mismatched event proof must block artifact writes in strict paths;
- artifact write failures must not trigger provider retries.

## 10. CLI Policy

No provider mutation CLI should be added until all of these are reviewed:

- explicit auth-source model;
- sandbox target policy;
- approval/high-assurance posture;
- SideEffect proposal and lifecycle state;
- durable event proof requirements;
- operator recovery and retry policy;
- redaction policy for terminal output;
- release posture for dangerous preview commands.

The existing recovery-summary CLI is read-only and remains outside provider
mutation behavior.

## 11. Adapter Expansion Policy

The first write lane is GitHub PR comments only. A second provider operation
must not copy the implementation mechanically.

Every new write operation must define:

- capability name and blast radius;
- target identity;
- policy effects;
- approval and high-assurance requirements;
- SideEffect lifecycle mapping;
- idempotency posture;
- provider response classification;
- event proof requirements;
- report/artifact disclosure posture;
- recovery strategy;
- sandbox/live test plan.

Avoid next:

- branch creation;
- pull request creation;
- merge operations;
- Jira transitions;
- CI reruns or workflow dispatch;
- repository file writes;
- destructive provider actions.

## 12. Recommended Next Code Phase

Recommended next implementation phase:

**provider-write sandbox readiness helper, no provider mutation**

The helper should produce a bounded readiness decision for a proposed live
sandbox write. It should accept explicit provider target metadata, approval
posture, SideEffect posture, event-proof posture, auth-source posture, and
artifact/recovery policy. It should return a stable allow/deny/defer decision
without calling providers, loading auth, writing records, appending events,
writing artifacts, exposing CLI behavior, adding schemas, or changing default
executor behavior.

Reason: the system needs one consolidated pre-live checkpoint before any live
sandbox provider call is exercised intentionally.

## 13. Test Plan For Future Implementation

Future sandbox readiness helper tests should cover:

- all gates satisfied returns allowed-for-sandbox;
- missing explicit auth posture returns denied;
- missing approval linkage returns denied when required;
- missing attempted SideEffect state returns denied;
- missing event-proof posture returns denied for artifact-required paths;
- ambiguous provider/local posture returns deferred and retry-blocked;
- production-looking target returns denied;
- unsupported provider operation returns denied;
- Debug and serialization do not leak tokens, URLs, paths, comment bodies,
  provider payloads, side-effect IDs, idempotency keys, or secret-like strings.

## 14. Open Questions

- Should auth-source modeling live in core or adapter-specific modules first?
- What is the smallest acceptable sandbox target definition for GitHub PR
  comments?
- Should sandbox provider writes require high-assurance approval even when the
  target is disposable?
- How should operator recovery decisions be persisted without becoming repair
  automation?
- When should a provider lookup observation become evidence, and when must it
  remain only advisory?
- Should provider-write artifact gates become workflow-declared before CLI
  mutation exists?

## 15. Final Recommendation

Proceed to runtime write-readiness checkpoint plan review.

If accepted, implement the provider-write sandbox readiness helper as a pure
model/helper phase. Do not implement live provider mutation, hidden auth
loading, automatic retry, repair apply, CLI mutation, schemas, examples,
hosted behavior, reasoning lineage, broad write adapters, or release posture
changes.
