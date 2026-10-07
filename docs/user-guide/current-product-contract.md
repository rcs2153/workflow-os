# Current Product Contract

This document is the concise truth about the current Workflow OS repository.
It describes implemented behavior on `main`, not every future concept in the
roadmap and not only the last packaged preview release.

## Product In One Sentence

Workflow OS is a governance kernel that binds autonomous or human execution to
durable identity, policy, authority, evidence, side-effect, approval, event, and
reporting contracts.

```text
Agent executes. Workflow OS governs.
```

The executor may be a coding agent, person, deterministic handler, adapter, or
sandbox. Workflow OS does not replace it. The kernel decides whether a material
action is valid and authorized, records the decision and outcome, and derives
what may happen next from durable state.

## Maturity

- **Release identity:** `0.2.0-preview.1`.
- **Repository maturity:** active preview with substantial post-release kernel
  work on `main`.
- **Strongest supported posture:** local governed execution and evaluation.
- **Hosted posture:** reviewed single-tenant, no-write alpha; not production or
  multi-tenant.
- **External writes:** narrow, explicit GitHub sandbox proof paths only; not a
  general provider-write product.
- **Primary audience today:** maintainers, infrastructure engineers, security
  engineers, and teams evaluating governed agent work.

## Implemented Product Surfaces

### Project and onboarding

- Load and validate versioned Workflow OS manifests and YAML definitions.
- Scaffold governance into an existing repository without replacing unmanaged
  `AGENTS.md` content by default.
- Inspect allowlisted repository metadata through `first-run` without reading
  arbitrary source files or running repository commands.
- Explain workflow recommendations and author inactive drafts.
- Preflight, steward-review, promote, catalog, archive, and propose catalog
  repair through explicit CLI commands.

### Local execution

- Execute deterministic sequential multi-step workflows.
- Pause and resume approval-gated steps.
- Apply bounded retry, escalation, cancellation, denial, and terminal-state
  semantics.
- Persist ordered events and rehydrate runs across process restarts.
- Refuse missing handlers instead of pretending declared skills executed.
- Register fixed reviewed local-check profiles explicitly; undeclared arbitrary
  command execution remains unavailable.

### Run integrity and continuation

- Bind runs to immutable workflow, skill, policy, handler, schema, and content
  commitments.
- Reassess current facts and proof-bound approval context before sensitive
  resume paths.
- Persist approval presentation evidence and reject missing, stale, ambiguous,
  or mismatched proof where required.
- Project a bounded read-only `next-action` from current durable state.
- Model execution windows, executor yields, typed waits, resume directives,
  attempts, and outcomes.
- Provide SQLite atomic continuity operations and reviewed trusted-host private
  composition paths. These do not turn the public CLI into an autonomous
  scheduler.

### Governance and reporting

- Evaluate declared policy effects before governed actions.
- Select proportional-governance posture from bounded inputs: quiet capture,
  visible disclosure, blocking approval, or denial.
- Represent current authority, scoped capability grants, required context,
  independent checks, typed handoffs, and hook checkpoints.
- Record EvidenceReference, SideEffect, approval, audit, and observability
  vocabulary without copying raw provider payloads by default.
- Build WorkReports that cite stable references and disclose work performed,
  decisions, checks, approvals, side effects, incomplete work, limitations,
  risks, and handoff notes.
- Persist explicit local report artifacts through gated opt-in paths with
  referential-integrity checks.

### State and hosting

- Local filesystem state for ordinary local evaluation.
- Explicit migration into verified inactive SQLite staging and separate
  activation.
- PostgreSQL state with shared-state conformance, fenced hosted work claims,
  outcome reconciliation, and recovery coverage.
- A single-tenant hosted alpha API/worker that authenticates one trust domain,
  dispatches no-write work, and persists terminal reporting.
- Optional OpenShell no-write provider contracts and pinned CLI inspection
  compatibility. The current CLI compatibility boundary does not satisfy the
  full execution-provider evidence contract.

### Integrations

- Fixture-first and opt-in live read-only GitHub, Jira, and GitHub Actions
  adapters.
- One explicit GitHub pull-request comment provider-write sandbox path.
- One explicit managed draft pull-request helper for an already-pushed,
  same-repository branch.
- Provider outcomes are reconciled through Core-owned gates and records; these
  paths are not enabled automatically by project installation.

## CLI Contract

The principal public commands are:

```text
workflow-os validate
workflow-os init-repo-governance
workflow-os init-agent-harness
workflow-os first-run
workflow-os author workflow ...
workflow-os run <workflow-id>
workflow-os approve <run-id> <approval-id>
workflow-os status <run-id>
workflow-os next-action <run-id>
workflow-os inspect <run-id>
workflow-os doctor state
workflow-os state migrate-sqlite ...
workflow-os state activate-sqlite ...
```

Advanced Core compositions, hosted operations, and provider-write proof paths
are not all exposed as stable CLI commands.

## Demonstration And Evidence Boundaries

- `--mock-all-local-skills` demonstrates kernel sequencing. Mock success is not
  evidence that external or repository work occurred.
- The scaffolded `local/first-run-governance` workflow is an approval/audit
  demonstration until a real reviewed handler is provided.
- `first-run` is real bounded posture analysis, but its recommendations are not
  active workflows and do not claim source-code understanding.
- Repository `dg/*` workflows govern Workflow OS development. They are examples
  of a team-specific workflow catalog, not community defaults.
- Model types and helper APIs do not imply automatic runtime enforcement unless
  the specific executor or hosted path is documented and tested.

## Important Non-Capabilities

Workflow OS does not currently provide:

- production multi-tenant hosting, high availability, or enterprise identity;
- a general agent runtime, recursive agents, or agent swarms;
- broad GitHub, Jira, CI, cloud, or SaaS mutation support;
- automatic execution of arbitrary repository commands;
- production nested harness execution;
- enterprise RBAC, IdP integration, quorum approval, or central policy admin;
- a complete UI for live disclosures, approvals, workflow stewardship, or
  reporting;
- automatic Reasoning Lineage / Claim Graph capture;
- a guarantee that caller-supplied evidence, checks, or authority claims are
  independently verified;
- a production scheduler that continuously redispatches agents from continuity
  directives.

Unknown or unsupported behavior should fail closed. A roadmap model is not a
product capability until it is wired into an explicit, tested execution path.

## Safe Evaluation Loop

From an existing repository:

```sh
workflow-os init-repo-governance --agent codex --dry-run
workflow-os init-repo-governance --agent codex
workflow-os validate
workflow-os first-run
workflow-os first-run --recommendation <id>
workflow-os author workflow --from-recommendation <id> --dry-run
```

To demonstrate approval and durable event history:

```sh
workflow-os --mock-all-local-skills run local/first-run-governance
workflow-os --mock-all-local-skills approve <run-id> <approval-id> \
  --actor user/evaluator --reason reviewed-first-run
workflow-os inspect <run-id>
workflow-os doctor state
```

The safe interpretation is:

1. scaffolding establishes a governance envelope;
2. first-run maps bounded posture and gaps;
3. recommendation authoring creates reviewable candidates;
4. the mock run proves kernel state transitions, not repository automation;
5. real execution requires a separately reviewed handler or provider boundary.

## Trust Contract

Trust Workflow OS only for the behavior enforced by the exact path you are
using. The kernel's value is not that it claims total control over an agent. Its
value is that material governance facts become typed, durable, fail-closed, and
inspectable.

For the engineering explanation, continue to
[Workflow OS Explainer](../concepts/WORKFLOW_OS_EXPLAINER.md).
