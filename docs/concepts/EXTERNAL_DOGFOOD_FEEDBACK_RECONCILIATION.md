# External Dogfood Feedback Reconciliation

## 1. Executive Assessment

Recent external tests of Workflow OS identified real product strengths and
several important risks. The feedback is credible, but it spans different
repository versions and therefore mixes open gaps with work that has already
been implemented and reviewed.

The central conclusion is:

```text
The kernel's governance architecture is holding up under real use. The next
work should compose accepted primitives into runtime enforcement and improve
independent proof, not reopen corrected models or add new mutation families.
```

Workflow OS is a credible local constitutional control plane for governed work.
It separates scope approval, execution, validation, closure, and publication;
records durable decisions; and preserves bounded authority. It is still a
preview kernel, not a mature build orchestrator or enterprise control plane.

## 2. Feedback Disposition

| Finding | Disposition | Repository evidence |
| --- | --- | --- |
| Existing `AGENTS.md` guidance can be overwritten during onboarding | Resolved | Existing unmanaged guidance is preserved by default and the managed Workflow OS block is appended or refreshed in place. Explicit `--force` replacement remains observable. |
| First-run recommendations are too generic | Substantially resolved | Bounded metadata detection and review-only recommendations cover package/TypeScript, Rust, Python, Go, GitHub Actions, conventional directories, and common repository documents. Broader workload-specific recommendation depth remains incremental work. |
| First-run output is too dense | Resolved for the current CLI boundary | Default output is concise; `--verbose` retains the detailed posture matrix; preview JSON remains machine-readable. |
| The real posture analysis and mock approval/audit demo are easy to confuse | Resolved | The mock run is labeled as an optional approval/audit demonstration rather than additional repository analysis. |
| `VisibleDisclosure` should not be a separate execution mode | Resolved | Execution disposition and disclosure obligation are independent axes. A local UI may display quiet decisions without changing execution authority. |
| Proportional governance requires too much manual decision-input configuration | Substantially resolved at the pure-model boundary | Typed workload assessment and workflow-declaration derivation infer posture from bounded validated facts and compose it monotonically with explicit workflow, profile, policy, authority, evidence/check, sensitivity, SideEffect, and steward minima. |
| Governance should be reevaluated when relevant workload inputs change | Resolved for the explicit opt-in local path | A versioned payload-free fingerprint covers decision-relevant facts and immutable definition roots. Exact retry and approval resume now re-read the stored immutable bundle, reassess current typed facts, and require exact durable binding equality before rehydration or approval mutation. Default paths and trusted fact freshness remain open. |
| Approval resume can execute changed workflow content | Resolved | Immutable run bundles are stored and explicitly bound to local execution; approval/resume no longer depends on silently reloading mutable workflow definitions for the accepted path. |
| Run specifications should remain frozen during an active run | Resolved for the accepted local binding path | Immutable run bundle core, store, and executor-binding phases are accepted. Broader runtime paths must adopt the same invariant before expansion. |
| The kernel does not independently prove real engineering checks | Resolved for one explicit authoritative local profile; open generally | The accepted authoritative `DocsCheck` path executes the canonical check, verifies same-call provenance and freshness against the immutable run, derives source-bound governance, and rejects mock or caller-asserted success as proof. Automatic/default checks and additional check families remain unsupported. |
| Actor and role enforcement are weaker than workflow semantics imply | Partially resolved for selected runtime paths; enterprise control remains open | Scoped grants, current-authority resolution, required-context consumption, operational windows, and trusted-host entry bind selected actors at time of use. General RBAC, IdP, organizational stewardship, and broad executor adoption remain unimplemented. |
| Artifact capture and machine-readable reporting need strengthening | Resolved for the selected authoritative local path; broad export remains open | Authoritative terminal paths can persist gated WorkReport artifacts, reconcile exact retries, and expose bounded JSON and metadata inspection. Ordinary undeclared runs, report-body export/publication, and broad automatic artifact production remain deferred. |
| Preview CLI edges remain | Specific reported edges resolved; ongoing product hardening | Missing-manifest diagnostics have an exact no-duplicate regression. The accepted authoritative artifact phase reran integration checks under Node 20 and Node 24. Other reproducible preview ergonomics issues remain valid bounded follow-ups. |

## 3. Proportional Governance Product Decision

The external proportional-governance critique is correct in principle and is
already reflected in the accepted design.

`Visible` is a disclosure obligation, not an execution category. The kernel
decides independently:

- whether execution may proceed, requires approval, or is denied;
- whether operator-visible disclosure is required;
- which evidence, checks, audit, SideEffect, limitation, and report records
  must be retained.

An operator preference or local UI may display quiet decisions live. That does
not make the work stricter. Conversely, hiding a policy-required disclosure
does not satisfy the obligation.

Configuration should express deterministic constraints and explicit minima,
not require users to hand-author every decision input. The onboarding target
remains deriving most ordinary posture from safe repository and workflow
metadata, then asking only about unresolved authority, sensitivity, evidence,
checks, approvals, and mutation posture. Inference may escalate but may never
weaken an explicit policy, workflow, profile, authority, or steward minimum.

The build-system invalidation analogy is also accepted. A governance decision
is bound to a versioned fingerprint over its relevant validated facts and
definition roots. The explicit opt-in local executor now uses that invalidation
boundary on exact retry and approval resume. Remaining work is trusted fact
freshness and carefully reviewed adoption, not a new fingerprint model.

## 4. Onboarding Product Decision

The useful first-run product loop is now:

1. validate the repository and identify the missing governance envelope;
2. preserve existing agent instructions while adding the managed boundary;
3. inspect safe bounded repository metadata;
4. produce concise governance posture and concrete review-only
   recommendations;
5. expose detail through verbose and machine-readable views;
6. keep the optional mock approval/audit demonstration distinct from real
   repository posture analysis.

The next onboarding improvements should deepen structured recommendations and
their validation obligations. They must not read arbitrary source contents,
execute detected commands automatically, fabricate evidence, or silently
activate generated workflows.

## 5. Remaining Runtime Priorities

The following work remains load-bearing:

1. **Production caller composition for accepted trusted-host boundaries.**
   Connect the reviewed private preparation/session boundary to the smallest
   unpublished local-host caller without moving authority construction,
   current-state revalidation, or identity ownership into application code.
2. **Broader actor-bound authority adoption.** Compose scoped grants,
   approvals, policy, capability availability, and run/step/resource identity
   in each additional concrete consumer before tool projection or invocation.
   Enterprise RBAC and IdP remain later layers.
3. **Broader check-proof and artifact coverage.** Extend the accepted
   authoritative `DocsCheck` and terminal artifact pattern only through
   separately reviewed profiles. Do not treat mock success, arbitrary handler
   output, or artifact presence as proof.
4. **Integrity-safe reporting and export.** Add report-body inspection,
   export, or broader event streaming only after authorization, sensitivity,
   retention, and privacy boundaries are explicit.
5. **Incremental onboarding depth.** Continue deriving concrete review-only
   workflow and validation recommendations from safe metadata, while keeping
   unresolved authority, sensitivity, approval, and mutation decisions explicit
   and reviewable.

These priorities reduce the gap between documented governance and enforced
runtime behavior. They do not authorize a new provider mutation family.

## 6. Sequencing Decision

Capability grant, current-authority resolution, required-context consumption,
immutable run binding, source-bound proportional-governance routing, one
authoritative local-check consumer, and authoritative terminal artifact
persistence are implemented for their accepted boundaries. The next phase is
not another independent-check model. It is the narrow local-host caller
composition planning authorized by the accepted trusted-host preparation
review.

No broader provider mutation family or default executor write should precede
concrete actor-bound time-of-use adoption and proof-bearing execution at the
selected consumer boundary.

## 7. Explicit Non-Goals

This reconciliation does not authorize:

- reopening the accepted two-axis proportional-governance model;
- replacing deterministic assessment with model judgment;
- arbitrary source inspection or command execution during onboarding;
- automatic workflow activation or silent workflow mutation;
- UI, hosted administration, RBAC, IdP, or enterprise identity work;
- default provider writes or additional mutation families;
- treating mock handlers as execution evidence;
- reasoning-lineage implementation;
- release-posture changes.

## 8. Governed Review Evidence

- Workflow: `dg/review`.
- Run ID: `run-1784166055179068000-2`.
- Approval ID:
  `approval/run-1784166055179068000-2/review-scope-approved`.
- Approval presentation: `presentation/7ee9d9dcbe041ba3`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Event summary: 39 ordered events, one approval, no retry or escalation.
- Out-of-kernel work: Codex inspected repository documentation, accepted phase
  reports, source, tests, git history, and roadmap state, then authored this
  reconciliation. The kernel coordinated governance only.
- Report posture: no runtime WorkReport artifact was generated or persisted.

## 9. Current-Main Reconciliation Update

The reconciliation was rechecked after merge of the explicit retry/resume
reassessment path. The external feedback remains accurate about the open check,
authority, artifact, and preview-UX boundaries, but its proportional-governance
and immutable-run concerns now describe accepted implementation rather than
wholly open architecture.

The product decision remains hybrid rather than inference-only: deterministic
derivation should configure most ordinary posture from safe validated metadata,
definitions, and runtime facts, while explicit workflow, profile, policy,
approval, authority, evidence/check, SideEffect, and steward minima remain
authoritative and may only be strengthened by inference.

Current governed review:

- Workflow: `dg/review`.
- Run ID: `run-1784507893478496000-2`.
- Approval ID:
  `approval/run-1784507893478496000-2/review-scope-approved`.
- Presentation ID: `presentation/7687b90b0c9fc4d1`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Out-of-kernel work: current-main documentation, accepted reports, roadmap,
  and implementation evidence were inspected; only reconciliation and roadmap
  priority wording were changed.

## 10. Current Kernel User Review Reconciliation

A later user review describes Workflow OS as a useful constitutional control
plane that separates scope, execution, validation, closure, and publication.
That is the intended product identity and remains a stronger description than
"build orchestrator." The review also identifies check proof, immutable run
inputs, actor enforcement, artifact capture, reporting, and preview tooling as
areas to strengthen.

The review is directionally sound but mixes current and historical posture:

- The approval/resume resolved-context TOCTOU finding was valid at its pinned
  older commit. Current main binds approval to a payload-free resolved-context
  commitment and rejects changed context before grant-side mutation.
- Active-run specification freezing is implemented for the accepted immutable-
  bundle paths and enforced by selected authoritative consumers. It is not yet
  a universal wrapper around every external action.
- Current main independently executes and verifies one canonical local
  `DocsCheck` in the same authoritative call. This is real execution evidence,
  unlike `--mock-all-local-skills`; it is intentionally not automatic or a
  general build-command runner.
- Actor-bound current-authority, required-context, operational-window, and
  trusted-host boundaries now exist for selected paths. General RBAC, IdP,
  groups, and enterprise administration remain absent.
- Authoritative terminal WorkReport artifacts and bounded machine-readable
  output are implemented for the selected local profile. Broad export,
  publication, shared artifact governance, and ordinary-run defaults remain
  deferred.
- The reported duplicate missing-manifest diagnostic has a regression proving
  one rendering. The accepted artifact phase also records passing Node 20 and
  Node 24 integration checks.

The downstream "arbitrary binary override" report is not attributable to a
Workflow OS kernel path from the supplied evidence. It should be treated as a
serious integration warning, but not recorded as a kernel defect until the
wrapper, command, or handler boundary can be reproduced against current main.
Workflow OS should continue to require canonical handler and invocation
bindings so a downstream wrapper cannot silently substitute execution.

This review does not justify a new primitive family or a detour into general
orchestration. It validates the current sequence: finish the trusted-host
application boundary, then broaden proof-bearing consumers deliberately. The
product should continue to optimize for low-friction governed execution, not
for becoming an ambient shell runner.

Current governed review:

- Workflow: `dg/review`.
- Run ID: `run-1791386827785120000-2`.
- Approval ID:
  `approval/run-1791386827785120000-2/review-scope-approved`.
- Presentation ID: `presentation/0d6d716b419d0ac0`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Event summary: 39 ordered events, one approval, no retry or escalation;
  approval-presentation proof was enforced.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: current-main source, tests, roadmap, implementation
  plans, reports, and reviews were inspected; this reconciliation and roadmap
  wording were updated.
- Report posture: no runtime WorkReport artifact was generated or persisted.
