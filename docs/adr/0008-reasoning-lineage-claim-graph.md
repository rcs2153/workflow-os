# ADR 0008: Reasoning Lineage / Claim Graph

## Status

Proposed

## Status Change Criteria

This ADR should not move from `Proposed` to `Accepted` until:

- Governed Work Pattern is accepted or explicitly scoped.
- Maintainers decide whether `evidence_reference` belongs in core.
- Maintainers decide whether `work_report_contract` belongs in core.
- Maintainers decide whether reasoning lineage belongs in core, work reports, domain packs, or some combination of those layers.
- A minimum viable implementation plan is reviewed and scoped.
- Privacy and redaction implications are reviewed.
- The relationship to audit events and work reports is defined.
- The accepted decision still preserves the Workflow OS product boundary.
- Acceptance does not by itself authorize writes, domain packs, schemas, persistence tables, generic runtime adapter execution, new runtime behavior, or CLI behavior.

## Context

Workflow OS has completed the `0.1.0-preview.1` local kernel preview readiness path. Phase 2 read-only integration work has since advanced to the `0.2.0-preview.1` public read-only integration preview posture, with narrow maintainer-owned live smoke evidence and fixture-first normal CI.

ADR 0007 accepts Governed Work Pattern as architecture and product direction. Governed Work Pattern describes how Workflow OS should structure governed enterprise work around context, evidence, policy, approvals, side-effect boundaries, validation, audit, observability, and reports. It remains direction only; evidence references, work report contracts, terminal work report artifacts, and side-effect boundary modeling still require separate scoped ADRs or implementation plans.

Reasoning Lineage / Claim Graph is a follow-on concept. It asks a different question: how did a conclusion, recommendation, decision, or report emerge from prior claims, evidence, actor contributions, corrections, context bindings, and reasoning steps?

The concept is inspired by the Magellan idea of a typed, persistent, auditable reasoning graph. Workflow OS should treat that inspiration as design input, not as an implementation to import wholesale.

The project should not move into policy-gated writes, broader domain packs, generic runtime adapter execution, or new runtime primitives before deciding whether a reasoning/provenance substrate belongs in the architecture.

## Decision

Workflow OS will recognize **Reasoning Lineage / Claim Graph** as proposed architecture and product direction and as a future design guide.

Reasoning Lineage is a future provenance model that could represent how conclusions, findings, recommendations, decisions, and work reports are derived from prior reasoning steps, evidence references, actor contributions, corrections, and context bindings.

This ADR accepts the concept as direction, not implementation. It does not schedule implementation, change runtime scope, add schemas, add persistence tables, add CLI behavior, add domain packs, add UI, or enable writes. The detailed concept is documented in [docs/concepts/reasoning-lineage.md](../concepts/reasoning-lineage.md).

The direction is:

- Reasoning Lineage should be treated as a provenance layer, not as the primary workflow runtime.
- Reasoning Lineage may eventually attach to workflow runs, steps, skills, evidence references, policy decisions, approval decisions, adapter invocation records, audit events, and work reports.
- Candidate concepts such as `claim`, `finding`, `reasoning_node`, `reasoning_edge`, `correction`, `reference_resolution`, `context_binding`, `confidence`, and `actor_attribution` should be evaluated before implementation.
- Corrections should likely be additive rather than destructive.
- Edge taxonomy should be evaluated from both Magellan-inspired relationships and Workflow OS enterprise governance needs.
- Core concepts should remain domain-neutral. Engineering-specific concepts such as pull requests, Jira tickets, CI runs, branches, commits, code reviews, and merges belong in adapters, skills, examples, or domain packs, not core.

## Consequences

Positive consequences:

- Workflow OS gains language for reasoning provenance without confusing it with workflow execution.
- Future reasoning-lineage work can let work reports cite claims, findings, evidence, corrections, and audit events without becoming raw audit dumps.
- The project can evaluate evidence, confidence, actor attribution, and context binding before write-capable integrations increase risk.
- The concept strengthens the product boundary: Workflow OS remains a governed workflow kernel, not a chat transcript database, generic memory system, or knowledge graph product.
- Future domain packs can define domain-specific claim or finding types without forcing those types into core.

Tradeoffs:

- The concept introduces future modeling pressure around reasoning nodes, edge taxonomies, query patterns, storage, privacy, and redaction.
- Core ownership must be decided carefully so the kernel does not become a generic knowledge graph or BPM engine.
- Reasoning lineage could become noisy if it captures every intermediate thought rather than durable, reviewable provenance artifacts.
- Privacy and redaction requirements may be stronger than ordinary workflow events because claims and findings can summarize sensitive provider data.
- The relationship between audit events, work reports, and reasoning lineage must be designed carefully to avoid duplicate or contradictory sources of truth.

## Alternatives Considered

1. Treat reasoning lineage as audit logs only.
   This was rejected because audit logs answer who or what did what, when, and under which policy. They do not explain how a conclusion emerged, branched, changed, or was corrected.

2. Treat reasoning lineage as work report text only.
   This was rejected because prose reports are useful handoff artifacts, but they are not enough for structured provenance, additive corrections, edge relationships, or queryable evidence links.

3. Implement a full Magellan-style graph immediately.
   This was rejected because Workflow OS must preserve its own product boundary and the repository standard requires scoped changes. A full graph implementation would be premature before evidence references, work reports, privacy posture, and storage boundaries are decided.

4. Make the first version domain-specific to engineering review.
   This was rejected because Workflow OS must remain generic across legal, finance, HR, security, procurement, support, operations, data/analytics, and software engineering.

5. Defer naming the concept.
   This was rejected because unnamed provenance concerns tend to leak into audit logs, reports, skills, and domain examples inconsistently. A named concept gives maintainers a review anchor.

## Non-Goals

This ADR does not:

- implement runtime features
- add schemas
- add persistence tables
- add CLI commands
- add domain packs
- add write-capable adapters
- add UI behavior
- add a chat transcript database
- add a knowledge graph product
- add a notebook or wiki
- add a generic agent memory system
- add a vector memory product
- add a reasoning UI product
- replace the workflow runtime
- import a full Magellan implementation
- change Phase 2 public-preview readiness

## Implementation Timing

Do not implement this immediately.

This concept must not interrupt:

- Phase 2 live-smoke evidence work
- Phase 2 public read-only integration preview readiness
- local-kernel correctness work

Revisit Reasoning Lineage / Claim Graph after Governed Work Pattern and before:

- policy-gated writes
- generic runtime adapter execution
- broader domain packs
- public claims about governed work reports with reasoning provenance

Any implementation must be scoped by a separate accepted ADR or implementation plan. That future plan must define privacy and redaction behavior, storage boundaries, relationship to audit events, relationship to work reports, and the minimum useful domain-neutral model.

The sequencing should be:

1. Capture and accept Governed Work Pattern as architecture and product direction.
2. Capture Reasoning Lineage / Claim Graph as a follow-on proposed provenance direction.
3. Resolve Phase 2 live-smoke/public-preview readiness.
4. Decide whether `evidence_reference` and `work_report_contract` belong in core.
5. Revisit Reasoning Lineage / Claim Graph before policy-gated writes, generic runtime adapter execution, or broader domain packs.

## Explicit Implementation Statement

No runtime feature is implemented by this ADR. No schema, persistence table, CLI behavior, domain pack, UI, or write behavior is added. Reasoning Lineage / Claim Graph is not implemented.
