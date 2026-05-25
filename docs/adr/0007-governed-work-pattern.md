# ADR 0007: Governed Work Pattern

## Status

Proposed

## Status Change Criteria

This ADR should not move from `Proposed` to `Accepted` until:

- Phase 2 live-smoke/public-preview readiness is resolved or explicitly paused.
- Maintainers agree whether `evidence_reference` belongs in core.
- Maintainers agree whether `work_report_contract` belongs in core.
- The relationship between work reports and audit events is defined.
- The relationship to future Reasoning Lineage or Claim Graph work is clarified.
- A minimum viable implementation plan is reviewed and scoped.
- The accepted decision still preserves the Workflow OS product boundary.
- Acceptance does not by itself authorize writes, domain packs, schemas, generic runtime adapter execution, or new CLI behavior.

## Context

Workflow OS has completed the `0.1.0-preview.1` local kernel preview readiness path. Phase 2 read-only integration work is ready for internal fixture-backed evaluation, but not public read-only preview because maintainer-owned live provider smoke evidence is still missing.

The project should not move into write-capable adapters, domain packs, or new runtime primitives before capturing the broader product direction that has emerged from its own development process.

Phase 2 read-only adapters introduce real external context into Workflow OS. That changes the architectural pressure. The next pressure is not simply more integration breadth; it is how the kernel should reason about evidence, decision traceability, quality gates, reporting, and side-effect governance before any broader or write-capable integration work.

Workflow OS tasks have consistently followed a disciplined pattern:

- read required context before acting
- respect explicit product boundaries
- make scoped changes
- run validation and quality gates
- preserve evidence
- require approval for sensitive or irreversible actions
- produce structured implementation reports
- disclose incomplete or deferred work

That pattern is useful beyond engineering. It can describe governed AI-assisted work in legal, finance, HR, security, procurement, customer support, operations, data/analytics, and software engineering.

## Decision

Workflow OS will recognize **Governed Work Pattern** as a proposed architecture and product direction and as a future design guide.

Governed Work Pattern is a reusable structure for AI-assisted enterprise work that binds context, evidence, policy, approvals, side-effect boundaries, validation, audit, observability, and structured reporting into repeatable workflows.

This ADR accepts the concept as direction, not implementation. It does not schedule implementation, change runtime scope, add schemas, add CLI behavior, add domain packs, or enable writes. The detailed concept is documented in [docs/concepts/governed-work-pattern.md](../concepts/governed-work-pattern.md).

The direction is:

- Core should own generic governance primitives such as durable workflow state, immutable run identity, event logs, policy decisions, approvals, auditability, observability, evidence references, report contracts, and side-effect boundaries.
- Skills should remain bounded capabilities such as summarizing, classifying, comparing against policy, validating evidence, drafting recommendations, generating reports, and preparing approval packets.
- Domain packs, if introduced later, should provide opinionated templates for specific enterprise work patterns without changing the core product boundary.

Core concepts introduced under this direction should remain domain-neutral. Engineering-specific concepts such as pull requests, Jira tickets, CI runs, branches, commits, code reviews, and merges belong in adapters, skills, examples, or domain packs, not core.

The concept distinguishes:

- `required_context`: what must be loaded or read before work starts.
- `evidence_reference`: what was actually used to support a conclusion, decision, validation result, approval, or report.
- Audit records: low-level operational history for reconstruction, compliance review, troubleshooting, and accountability.
- Work reports: high-level governed handoff artifacts that explain what was done, why it is ready or blocked, what evidence was used, what decisions were made, what validation occurred, what approvals occurred, what side effects were attempted or avoided, and what remains incomplete.

Work reports may cite audit records, workflow events, adapter invocation records, validation results, approval decisions, and evidence references. They should not be reduced to audit logs.

Future side-effect modeling should use explicit domain-neutral states, such as:

- proposed
- approved
- attempted
- completed
- denied
- skipped
- failed
- potentially rolled back in a future version where a specific adapter contract honestly supports rollback or compensation

Rollback remains a future candidate only. This ADR does not imply rollback exists.

As a non-binding future hypothesis, an initial implementation could include:

- `evidence_reference` as a generic reference object.
- `work_report_contract` as a declarative report schema.
- a terminal work report artifact.
- links from report sections to audit events, adapter invocation records, validation results, approval decisions, and evidence references.
- domain-specific report templates outside core.

This hypothesis is not an approved design and must not be treated as implementation scope.

## Consequences

Positive consequences:

- Workflow OS gains language for governed enterprise work that generalizes beyond engineering.
- Future prompts can distinguish core governance primitives from skills and domain templates.
- The project can evolve implementation reports into structured work reports without pretending that feature already exists.
- The concept strengthens the product boundary: Workflow OS remains a governed workflow kernel, not a chat agent framework or coding-agent wrapper.
- The project has a guardrail against adding more integrations before it can explain evidence, decisions, reports, and side-effect states.
- Future write-capable adapter work has clearer prerequisites around evidence, approvals, reporting, and side-effect governance.

Tradeoffs:

- The concept introduces future modeling pressure around `evidence_reference`, `work_report`, and `work_report_contract`.
- Core ownership must be decided carefully so the kernel does not become a generic BPM engine.
- Domain packs must remain deferred until the kernel can support them without overfitting to one domain.
- Reports and audit records must remain distinct enough that operator handoff artifacts do not become noisy audit dumps.

## Alternatives Considered

1. Treat governed work as a skill-only pattern.
   This was rejected because evidence, policy, approvals, audit, observability, side-effect boundaries, and report contracts affect runtime trust and likely need core representation.

2. Treat governed work as domain-pack behavior only.
   This was rejected because the pattern is cross-domain and should not be hidden inside engineering, legal, finance, or support templates.

3. Implement work reports, evidence references, and quality gates immediately.
   This was rejected because Phase 2 live-smoke evidence and public-preview readiness remain unresolved, and the repository standard requires scoped changes.

4. Avoid naming the pattern.
   This was rejected because unnamed patterns tend to drift into ad hoc behavior. A named concept gives maintainers a stable review anchor.

## Non-Goals

This ADR does not:

- implement runtime features
- add schemas
- add domain packs
- add write-capable adapters
- add CLI commands
- add a chat-agent framework
- add a BPM engine
- add a SaaS workflow builder
- claim governed work reports are implemented
- change Phase 2 public-preview readiness

## Implementation Timing

Do not implement this immediately.

Revisit Governed Work Pattern before:

- policy-gated writes
- generic runtime adapter execution
- broader domain packs
- public claims about governed work reports

Phase 2 live-smoke evidence and public read-only preview readiness should not be interrupted by this concept. This ADR should be revisited only after that work is resolved or explicitly paused.

The sequencing should be:

1. Capture Governed Work Pattern as proposed architecture and product direction.
2. Resolve Phase 2 live-smoke/public-preview readiness.
3. Consider Reasoning Lineage or Claim Graph as a follow-on concept for claim, assumption, evidence, and decision relationships.
4. Revisit Governed Work Pattern before policy-gated writes, generic runtime adapter execution, or broader domain packs.

## Explicit Implementation Statement

No runtime feature is implemented by this ADR. Governed Work Pattern is not implemented.
