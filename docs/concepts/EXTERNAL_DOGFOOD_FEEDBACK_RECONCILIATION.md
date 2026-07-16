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
| Governance should be reevaluated when relevant workload inputs change | Resolved in the model, open in runtime composition | A versioned payload-free fingerprint covers decision-relevant facts and immutable definition roots. Automatic invalidation and reassessment at execution boundaries are not yet integrated. |
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
definition roots. The remaining gap is automatic use of that invalidation
boundary during runtime composition, not the fingerprint model itself.

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

1. **Pure capability resolution.** Resolve explicit capability definitions,
   current availability, scoped grants, actor, resource, workflow, run, and
   step identity. Availability alone must never authorize invocation.
2. **Runtime proportional-governance reassessment.** Recompute decisions when a
   bound fingerprint changes and fail closed on stale, unknown, or unsupported
   authority. Begin as an explicit opt-in integration after pure capability
   resolution, not as a global default.
3. **Independent check attestation.** Bind check identity, invocation,
   structured result, provenance, freshness, and immutable run context without
   treating raw command output or mock success as proof.
4. **Actor-bound authority enforcement.** Compose scoped grants, approvals,
   policy, capability availability, and run/step/resource identity before tool
   projection or invocation. Enterprise RBAC and IdP remain later layers.
5. **Default artifact/report composition only after review.** Broaden
   machine-readable artifact and event export only after integrity,
   authorization, and privacy boundaries are accepted for the selected path.

These priorities reduce the gap between documented governance and enforced
runtime behavior. They do not authorize a new provider mutation family.

## 6. Sequencing Decision

The next implementation remains the pure capability resolution helper already
sequenced in the scoped runtime authority plan. It is the smallest dependency
needed before step-scoped tool projection, actor-bound invocation, and runtime
proportional-governance reassessment can be correct.

After that helper is implemented and reviewed, the roadmap should prefer one
explicit composition path that combines capability resolution with the
accepted proportional-governance assessment. Independent check attestation
should proceed before broader provider mutation families or default executor
writes.

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
