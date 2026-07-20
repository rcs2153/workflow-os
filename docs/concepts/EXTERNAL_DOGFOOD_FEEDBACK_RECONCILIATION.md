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
| The kernel does not independently prove real engineering checks | Open | Local check models and selected handler/report plumbing exist, but general independent check attestation, freshness, provenance, and default execution remain incomplete. Mock skill success is not execution evidence. |
| Actor and role enforcement are weaker than workflow semantics imply | Open | High-assurance approval and scoped capability vocabulary exist, but general actor-bound runtime authority, RBAC, IdP, and enterprise stewardship are not implemented. |
| Artifact capture and machine-readable reporting need strengthening | Partially resolved | WorkReport, report artifacts, evidence citations, SideEffect linkage, and selected machine-readable projections exist. Default runtime artifact production and broad export/tailing remain deferred. |
| Preview CLI edges remain | Ongoing product hardening | Real-repository tests continue to identify bounded CLI ergonomics issues. These should be fixed when reproducible without displacing runtime correctness work. |

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

1. **Independent check attestation.** Bind check identity, invocation,
   structured result, provenance, freshness, and immutable run context without
   treating raw command output or mock success as proof.
2. **Actor-bound authority enforcement.** Compose scoped grants, approvals,
   policy, capability availability, and run/step/resource identity before tool
   projection or invocation. Enterprise RBAC and IdP remain later layers.
3. **Default artifact/report composition only after review.** Broaden
   machine-readable artifact and event export only after integrity,
   authorization, and privacy boundaries are accepted for the selected path.
4. **Incremental onboarding depth.** Continue deriving concrete review-only
   workflow and validation recommendations from safe metadata, while keeping
   unresolved authority, sensitivity, approval, and mutation decisions explicit
   and reviewable.

These priorities reduce the gap between documented governance and enforced
runtime behavior. They do not authorize a new provider mutation family.

## 6. Sequencing Decision

Capability grant, availability, resolution, request review, pure step-scoped
projection, immutable run binding, and opt-in proportional-governance
reassessment are now implemented for their accepted boundaries. The next
cross-cutting phase should plan independent check attestation, then implement
the smallest model and explicit local proof path needed to distinguish real
engineering checks from mock or caller-asserted success.

Governed context-access projection remains the next phase inside the scoped
authority lane, but it should not displace the more immediate check-proof gap.
No broader provider mutation family or default executor write should precede
independent check attestation and actor-bound time-of-use enforcement.

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
