# Workflow OS Project Charter

Workflow OS is intended to become an enterprise-grade open-source framework for defining, validating, testing, executing, governing, observing, and scaling AI-driven workflows.

The project exists to make AI-driven enterprise work declarative, testable, governed, auditable, and portable in the same way that mature data teams expect analytics transformations to be declarative, testable, governed, and portable.

The current v0 release posture has two preview layers: `0.1.0-preview.1` established the **public local kernel preview**, and `0.2.0-preview.1` adds a narrow **public read-only integration preview** for GitHub, Jira, and GitHub Actions / CI. Both previews are for evaluation and contribution. They are not production deployment platforms, hosted services, production distributed runtimes, UI products, adapter-complete frameworks, write-capable integration systems, or Level 3/4 autonomy systems.

Phase 2 read-only adapter work for GitHub, Jira, and CI/GitHub Actions is approved only as a read-only public preview posture. It remains fixture-first in normal CI and opt-in for live providers. It does not imply write-capable adapters, production integration readiness, generic live adapter execution, OAuth, webhooks, hosted operation, distributed workers, production backends, or Level 3/4 autonomy.

The [Governed Work Pattern](concepts/governed-work-pattern.md) captures a proposed long-term product direction: Workflow OS should support repeatable enterprise work that binds required context, evidence, policy gates, approvals, side-effect boundaries, validation, audit, observability, and structured reporting. This direction is not implemented as a runtime feature yet, and it does not change the v0 or Phase 2 release boundaries.

A future [Reasoning Lineage / Claim Graph](concepts/reasoning-lineage.md) concept may extend Governed Work Pattern by modeling claims, findings, corrections, confidence, actor attribution, reference resolution, and evidence relationships as provenance artifacts. That work should be considered after Governed Work Pattern, before policy-gated writes or broader domain packs, and as a provenance substrate rather than a replacement for the Workflow OS runtime.

## What Workflow OS Is

Workflow OS Core is a local-first, declarative kernel for governed AI workflows.

Core provides the foundation for:

- Defining workflow specs.
- Validating workflow specs.
- Testing workflow behavior.
- Executing narrow local workflow runs.
- Enforcing policy before side effects.
- Managing durable workflow state through interfaces.
- Recording auditable runtime events.
- Invoking explicitly registered local skills.
- Defining adapter contracts for external systems.

The intended long-term core product is the kernel, contract model, validator, runtime, policy layer, audit layer, observability model, CLI, schemas, and compatibility surface required to make governed AI workflow execution trustworthy. v0 implements the local kernel preview for those contracts, not the full production system.

Over time, Workflow OS should make governed work patterns explicit across enterprise domains without turning Core into a chat agent framework, SaaS workflow builder, or domain-specific automation product.

## What Workflow OS Is Not

Workflow OS Core is not:

- A replacement for Temporal.
- A replacement for Airflow.
- A replacement for GitHub Actions.
- A replacement for Zapier.
- A generic business process management engine.
- A chat agent framework.
- A one-off GitHub, Jira, or CI automation tool.
- An enterprise SaaS application.
- A UI product in v0.
- A production distributed runtime in v0.
- An adapter-complete framework in v0.
- A Level 3/4 autonomy system in v0.

Workflow OS may eventually integrate with orchestration systems, ticketing systems, CI systems, repositories, approval tools, observability platforms, and enterprise data systems. Phase 2 read-only adapters are early contract proofs for that direction. Those integrations must remain adapters around a correct core, not the definition of the product itself.

## Why The dbt Core Analogy Matters

The dbt Core analogy matters because Workflow OS should do for AI-driven enterprise workflows what dbt Core did for analytics engineering:

- Move critical work into declarative, reviewable files.
- Make validation and testing part of the normal development loop.
- Treat execution as a reproducible operation over explicit project state.
- Support local development before production deployment.
- Encourage modularity, version control, documentation, and governance.
- Build an ecosystem around a stable open-source core.

The analogy does not mean Workflow OS is a data transformation tool. It means the project should preserve the same kind of disciplined core: local-first, file-driven, composable, testable, documented, and suitable for serious production adoption.

## Why Stateless Workers Over Durable State

Workflow OS uses stateless workers over durable state because enterprise workflow execution must survive real operational conditions.

Workers may restart, crash, scale horizontally, be rescheduled, or be replaced. Workflow correctness must not depend on a single long-lived process remembering what happened. Durable state and append-only meaningful events provide the source of truth; workers should read state, attempt valid transitions, perform policy-approved work, and record outcomes.

This design supports:

- Restart safety.
- Horizontal scaling.
- Replay and reconciliation.
- Auditable state transitions.
- Duplicate event deduplication.
- Idempotent skill invocation.
- Separation between core workflow correctness and deployment topology.
- Portability across local execution, CI, and future production runtimes.

Stateless workers also prevent adapters from becoming hidden state owners. External systems may provide signals and receive approved side effects, but the workflow run state belongs to the core state model.

## Why v0 Focuses On The Local-First Kernel

v0 must focus on the local-first kernel before real integrations because the kernel defines the product's trust boundary.

The earliest version must prove:

- Workflow specs can be declared and validated.
- Schema versions and content hashes are tracked.
- Runs reference immutable workflow definitions.
- State transitions are explicit and auditable.
- Policy decisions happen before side effects.
- Skills can be invoked through controlled interfaces.
- Runtime behavior can be tested locally.
- Failures are visible, explainable, and safe.

Real integrations are valuable only after these guarantees exist. Without the kernel, integrations would create a collection of automation scripts instead of a governed workflow framework.

## Why GitHub, Jira, And CI Writes Are Deferred

Write-capable GitHub, Jira, and CI adapters are deferred until the kernel is correct because those integrations introduce real external writes, permission boundaries, identity questions, rate limits, retries, secrets, audit requirements, and user trust concerns.

Adding write-capable adapters too early would bias the core toward a few systems and make Workflow OS look like a repository or ticket automation tool. The project must first define generic workflow state, policy, audit, idempotency, validation, and execution semantics.

Phase 2 read-only adapters are intentionally narrower: they retrieve external facts through the generic adapter contract, use fixture tests by default, require opt-in live tests, and do not mutate provider state. They are public preview integrations only, not production integrations.

Once the kernel is correct, GitHub, Jira, CI, and other enterprise integrations can be expanded as adapters that obey the same rules:

- They do not mutate core state directly.
- They are capability-gated.
- They are policy-gated.
- They are auditable.
- They are idempotent.
- They fail closed on unsafe or ambiguous actions.

## Why Level 1 And Level 2 Autonomy Are Default

Level 1 and Level 2 autonomy are default because enterprise AI workflows must earn trust before taking sensitive action.

Early and default workflow behavior should emphasize:

- Drafting.
- Classification.
- Recommendation.
- Validation.
- Summarization.
- Plan generation.
- Human-reviewed execution.
- Low-risk local execution.

Level 3 and Level 4 autonomy can create or mutate important external state. That requires explicit policy enablement, capability gating, auditability, idempotency, and human approval for sensitive or ambiguous actions. Higher autonomy must be a deliberate governance decision, not an accidental default.

## Why The Core Must Be Generic Across Enterprise Domains

Workflow OS Core must be generic across enterprise domains because governed AI workflow execution is not unique to software engineering.

The same core concepts apply across many domains:

- Workflow specs.
- Inputs and outputs.
- Skills.
- Adapters.
- Policy.
- Approvals.
- Durable state.
- Event logs.
- Audit records.
- Observability.
- Idempotent side effects.
- Human escalation.

If Core is shaped around GitHub, Jira, CI, sales, support, finance, legal, or any single enterprise domain, it will not become a durable foundation. Domain-specific behavior belongs in skills, adapters, policies, templates, and examples. Core must provide the stable substrate that lets those domain layers exist without fragmenting the runtime model.

## Project Direction

Workflow OS should grow from the center outward:

1. Establish engineering standards and product boundary.
2. Define the canonical workflow spec and schema versioning model.
3. Build the Rust core domain model and validator.
4. Build the local-first CLI.
5. Build the runtime state and event model.
6. Add policy, audit, and observability primitives.
7. Add controlled skill invocation.
8. Add adapters only after core invariants are enforced.
9. Expand SDK ergonomics without creating incompatible parallel models.
10. Prepare the repository for open-source readiness.
11. Revisit Governed Work Pattern before policy-gated writes or domain packs.
12. Evaluate Reasoning Lineage / Claim Graph as a provenance substrate before broader governed work reporting.

This order matters. A correct kernel makes integrations powerful. Integrations without a correct kernel create unmanaged automation.
