# Reasoning Lineage / Claim Graph

Reasoning Lineage / Claim Graph is a proposed future architecture and product direction for Workflow OS. It is not implemented as a runtime feature, schema, persistence table, CLI command, domain pack, write-capable adapter, or UI.

## 1. Definition

Reasoning Lineage is a future provenance model for Workflow OS. It would represent how conclusions, findings, recommendations, decisions, and work reports are derived from prior reasoning steps, evidence references, actor contributions, corrections, and context bindings.

Reasoning Lineage is not the workflow runtime. The workflow runtime answers how work executes and changes state. Reasoning Lineage would be a provenance layer that may attach to workflow runs, steps, skills, evidence references, decisions, approvals, and work reports.

The concept is inspired by the Magellan idea of a typed, persistent, auditable reasoning graph, but Workflow OS should not import a full Magellan implementation wholesale. Workflow OS must adapt any claim graph concept to its own product boundary: a declarative, local-first, policy-gated, auditable workflow kernel.

## 2. Why It Matters

Workflow state, audit logs, and work reports are necessary but not sufficient.

- Workflow state answers: where is the work in execution?
- Audit events answer: who or what did what, when, under what policy?
- Work reports answer: what was done, what evidence was used, and what is complete or incomplete?
- Reasoning lineage answers: how did the conclusion, recommendation, decision, or report emerge, change, branch, or get corrected?

This distinction matters whenever an operator needs to review not only the final answer, but the path by which that answer became credible.

Examples:

- Legal review: trace a contract recommendation back to clauses, policy references, unresolved ambiguities, reviewer corrections, and approval decisions.
- Finance exceptions: show how evidence, thresholds, missing information, and approver input produced a recommendation.
- Security triage: preserve how findings were decomposed, corrected, escalated, or downgraded as new evidence arrived.
- Customer support resolution: show how the final response emerged from customer context, policy constraints, prior interactions, and human review.
- Procurement review: connect vendor assessment findings to evidence, policy gates, risk decisions, and approval packets.
- Data and analytics interpretation: trace an insight to source assumptions, data quality checks, validation failures, and analyst corrections.
- Software engineering review: connect a recommendation to changed files, tests, CI context, security findings, reviewer decisions, and unresolved risks.
- Executive decision support: preserve the claims, evidence, confidence, disagreements, and decisions behind a recommendation.

## 3. Relationship To Governed Work Pattern

Governed Work Pattern defines the enterprise work structure: context, evidence, policy, approvals, side effects, validation, audit, observability, and reports.

Reasoning Lineage complements Governed Work Pattern by modeling provenance relationships among:

- claims
- findings
- decisions
- evidence references
- corrections
- report sections
- actor contributions
- context bindings

Reasoning Lineage should not replace Governed Work Pattern. Governed Work Pattern defines how governed work is structured and controlled. Reasoning Lineage would explain how conclusions and recommendations within that governed work were derived, revised, and supported.

## 4. Relationship To Existing Kernel

Reasoning Lineage could eventually connect to existing and proposed Workflow OS concepts:

- `workflow_run`: the durable execution instance whose work may produce claims, findings, decisions, or reports.
- `run_event`: the append-only runtime history that may be cited as operational evidence.
- `skill_invocation`: the bounded capability call that may produce claims, findings, summaries, or proposed corrections.
- `policy_decision`: a governance decision that may support, block, or require approval for a claim or action.
- `approval_decision`: a human decision that may approve, deny, or annotate a proposed action or report.
- `adapter_invocation_record`: an external read record that may provide evidence or context.
- `audit_event`: low-level operational history that reasoning lineage may cite but should not replace.
- `evidence_reference`: a future pointer to evidence used by claims, findings, decisions, or report sections.
- `work_report`: a future governed handoff artifact that may include or cite reasoning lineage.
- `work_report_contract`: a future declarative shape for required report sections and evidence links.
- `quality_gate`: a future validation or review gate that may accept, reject, or require correction of claims.
- `incomplete_work_disclosure`: a future structured statement of uncertainty, missing evidence, deferred work, or unresolved questions.

None of these concepts are changed by this document. This concept records a future direction only.

## 5. Candidate Primitives

The following are future candidates only. They are not implemented by this document.

- `reasoning_node`
- `claim`
- `finding`
- `decision_claim`
- `reasoning_edge`
- `reasoning_edge_type`
- `derived_from`
- `follow_up`
- `refine`
- `decompose`
- `temporal_pivot`
- `branch`
- `correction`
- `reference_resolution`
- `context_binding`
- `confidence`
- `actor_attribution`
- `evidence_link`
- `report_section_link`
- `claim_status`
- `superseded_by`
- `corrected_by`

These names should be treated as vocabulary for future design, not as approved schema or runtime surface.

## 6. Edge Taxonomy

The Magellan-inspired edge taxonomy is useful future design input. Candidate relationship types include:

- `follow_up`: a later claim or question continues from an earlier one.
- `refine`: a later claim narrows, clarifies, or improves an earlier one.
- `decompose`: a claim is broken into smaller claims or findings.
- `temporal_pivot`: reasoning shifts because time, period, or sequence context changes.
- `branch`: multiple alternative reasoning paths are preserved.
- `correction`: a later claim corrects a prior claim without erasing it.

Workflow OS may need to adapt this taxonomy for enterprise work.

Potential additions or alternatives to evaluate later:

- `supports`
- `contradicts`
- `supersedes`
- `cites`
- `depends_on`
- `produced_by`
- `approved_by`
- `rejected_by`
- `unresolved_due_to`
- `escalated_from`

This document does not decide the final taxonomy.

## 7. Additive Corrections

Corrections should likely be additive rather than destructive.

A future correction should not overwrite or erase a prior claim, finding, or recommendation. It should preserve the original artifact, add the correction, and make the correction relationship explicit.

Additive corrections matter for:

- auditability: reviewers can see what changed and why.
- compliance: original statements remain available for reconstruction.
- human review: reviewers can compare the original and corrected reasoning.
- trust: the system does not silently rewrite its own history.
- post-incident analysis: teams can understand how a wrong conclusion emerged and how it was corrected.

This does not mean every incorrect claim should remain prominent forever. It means lineage should preserve enough structure to explain correction without pretending the prior state never existed.

## 8. Reference Resolution And Context Binding

Enterprise work often includes ambiguous references:

- "this issue"
- "that clause"
- "the failed check"
- "last quarter"
- "the customer"
- "the prior recommendation"

Future reasoning lineage should preserve how these references were resolved.

A future `reference_resolution` or `context_binding` object could capture:

- original text
- resolved entity or reference
- source of resolution
- confidence
- actor or system actor that resolved it
- timestamp
- evidence reference, where applicable

Preserving context binding helps reviewers understand whether a conclusion depended on the right document, record, customer, time period, workflow run, provider object, or prior recommendation.

## 9. Actor Attribution

Multi-actor provenance matters because governed work often includes contributions from humans, skills, systems, adapters, and reviewers.

Future reasoning lineage may need to distinguish:

- human operator
- reviewer
- approver
- system actor
- skill actor
- adapter or system source
- agent actor
- external system

This should build on existing Workflow OS actor and system actor concepts. It should not introduce a chat-centric actor model or treat transcript participants as the primary product abstraction.

## 10. Confidence And Uncertainty

Confidence should attach to specific claims, findings, recommendations, or decisions where appropriate, not only to entire workflow runs.

Confidence must not become fake precision. A confidence value or label should be treated as metadata that can support review, escalation, quality gates, and work reports. It should not imply mathematical certainty unless the producing skill or validation process can justify that meaning.

Useful confidence and uncertainty metadata might include:

- confidence level or qualitative confidence label
- evidence strength
- unresolved dependencies
- missing evidence
- contradictory evidence
- actor or skill that assigned the confidence
- timestamp
- reason or basis for the confidence

## 11. Layering Hypothesis

Workflow OS should treat Reasoning Lineage as layered.

Core may eventually own:

- generic reasoning lineage primitives
- generic edge types or edge categories
- actor attribution
- evidence links
- correction semantics
- report-section linkage
- confidence and uncertainty metadata
- reference-resolution and context-binding contracts

Skills may produce:

- claims
- findings
- evidence-backed conclusions
- confidence metadata
- proposed corrections
- decompositions
- unresolved questions
- follow-up recommendations

Domain packs may define:

- legal claim and finding types
- security severity finding types
- finance exception finding types
- support resolution finding types
- engineering review finding types
- procurement assessment finding types
- analytics interpretation finding types

The core should stay domain-neutral. Domain-specific names, severity systems, report formats, and review rubrics should live outside core unless they become genuinely universal Workflow OS concepts.

## 12. Product Boundary

Reasoning Lineage does not turn Workflow OS into:

- a chat transcript database
- a knowledge graph product
- a notebook or wiki
- a generic agent memory system
- a vector memory product
- a reasoning UI product
- a replacement for the workflow runtime
- a full Magellan implementation

Workflow OS remains a declarative, local-first, policy-gated, auditable workflow kernel. Reasoning Lineage is a possible provenance substrate for governed work, not the primary execution model.

## 13. Timing

Do not implement Reasoning Lineage immediately.

This concept must not interrupt:

- Phase 2 live-smoke evidence work
- Phase 2 public read-only integration preview readiness
- local-kernel correctness work

Do not build domain packs yet.

Do not build policy-gated writes yet.

Revisit this concept after Governed Work Pattern and before:

- policy-gated writes
- broader domain packs
- generic runtime adapter execution

Any implementation must be scoped by a separate accepted ADR or implementation plan. Accepting this concept would not by itself authorize runtime changes, schemas, persistence tables, CLI commands, writes, domain packs, or UI behavior.

## 14. Possible First Implementation Hypothesis

This section is non-binding and not implemented.

A possible future implementation could:

- add `evidence_reference` first
- add `work_report_contract`
- add an optional `reasoning_lineage` section to work reports
- add generic `claim` or `finding` references produced by skills
- add additive correction links
- link report sections to evidence, audit events, adapter invocation records, validation results, approval decisions, and claims or findings

This hypothesis is not approved scope. It exists to make a future design discussion concrete and reviewable.

## 15. Open Questions

- Should the core concept be called `claim`, `finding`, `reasoning_node`, or something else?
- What edge types are generic enough for core?
- Should corrections be core or report-level only?
- How should reasoning lineage relate to audit events?
- How should reasoning lineage relate to work reports?
- How should reasoning lineage relate to evidence references?
- How much confidence metadata should be allowed or required?
- How should lineage be queried?
- Should lineage be persisted in the same backend as workflow events or a separate store?
- What is the minimum useful version that does not overfit to software engineering or chat?
- How should privacy and redaction work for reasoning lineage?

## Implementation Status

Not implemented. This document records architecture and product direction only.
