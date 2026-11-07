# Current Product Contract

This document is the concise truth about the current Workflow OS repository.
It describes implemented behavior on `main`, not every future concept in the
roadmap and not only the last packaged preview release.

## Product In One Sentence

Workflow OS is an open-source continuity-of-authority kernel for agentic work.
It provides shared governance across people, agents, processes, and execution
frameworks. It converts ephemeral conversation and execution context into a
durable, policy-governed run whose authority, execution state, evidence, and
next permissible action can be recovered by another authorized executor.

```text
Agent executes. Workflow OS governs.
```

The executor may be a coding agent, person, deterministic handler, adapter, or
sandbox. Workflow OS does not replace it. The kernel decides whether a material
action is valid and authorized, records the decision and outcome, and derives
what may happen next from durable state.

The kernel keeps three operational layers connected:

1. **Procedure authority:** who may define, revise, activate, or retire the
   workflow and its policy and check requirements.
2. **Invocation authority:** who authorized this exact run, action, resource,
   execution window, and replacement executor.
3. **Execution truth:** what was accepted, attempted, completed, failed,
   blocked, denied, or externally observed, and what reconciliation remains.

Continuity of authority is the property that preserves those bindings across
retries, interruptions, compaction, restore, and executor replacement. It
determines the next action a current authorized executor may lawfully take
without reconstructing permission from conversation history. Evidence and
reports verify and expose these bindings. They do not replace them, and
Workflow OS is not primarily an audit logger or report generator.

## Core Product Value: Continuity Of Authority

Workflow OS's strongest differentiated value is continuity of authority across
people, agents, processes, retries, recovery, and interruptions. The intended
contract is that the same governed record establishes what was authorized, what
executed, which evidence and reports belong to that exact work, and what remains
permitted. A replacement executor should not reconstruct authority from chat
history.

Current `main` does not yet satisfy that contract across every preview surface.
The hosted dispatch-truth, WorkReport identity-binding, and GitHub repository-
metadata project-resource continuity P0 gates are closed for their documented
boundaries after focused review, exact-head CI, and independent reruns. A
private Core cross-gate model now binds those exact lifecycle facts before
replacement fresh-authority resolution and rejects mixed or missing bindings.
It is not a production replacement-executor caller or a generalized continuity
claim. See the [Continuity Of Authority P0 Release Gates
Plan](../implementation-plans/continuity-of-authority-p0-release-gates-plan.md)
and [Product And Release Reconciliation
Plan](../implementation-plans/continuity-of-authority-product-release-reconciliation-plan.md).

The product must therefore be evaluated first on authority integrity,
execution admission, identity binding, interruption continuity, adversarial
recovery, and the operational cost of safe recovery. Audit completeness and
report quality remain necessary evidence for those contracts, not substitutes
for them. See the [V1 Readiness Criteria](../release/V1_READINESS_CRITERIA.md).

An evaluator-authored Codex-to-LangGraph integration has demonstrated the
intended cross-harness shape: one harness started governed work, another
resumed it without transcript authority, denial and authority drift prevented
the bounded effect, and a fresh reviewer reconstructed the decision from the
shared kernel record. The integrations and identities were evaluator-owned and
the ambiguous target was simulated. Current `main` therefore does not claim
packaged plug-and-play interoperability, authenticated cross-harness
delegation, or a production execution-framework integration.

## Maturity

- **Release preparation identity:** `0.4.0-preview.1`.
- **Repository maturity:** active governed-execution and continuity preview.
- **Strongest supported posture:** local governed execution and evaluation.
- **Hosted posture:** reviewed single-tenant, no-write alpha; not production or
  multi-tenant.
- **External writes:** narrow, explicit GitHub sandbox proof paths only; not a
  general provider-write product.
- **Primary audience today:** maintainers, infrastructure engineers, security
  engineers, and teams evaluating governed agent work.
- **Release blockers:** all three continuity gates and the private cross-gate
  review, validator-assurance minimum and wire hardening, exact-source
  re-candidating, and independent release/security assurance are closed for
  their documented boundaries. The separate publication decision, the
  assurance review's disclosed P2 follow-ups, and the repository's other active
  P0 lanes still block publication, v1, and broader product claims.

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

Current evidence assurance is path-specific. A closed local-check profile can
bind the exact selected executable and invocation without proving that an
operator-selected executable came from a trusted producer. Report artifacts
retain and enforce a complete body commitment plus exact external storage-key,
internal run/report, and available immutable workflow identity across the
accepted filesystem, SQLite, PostgreSQL, transaction, inspect, recovery,
reconciliation, and replay paths. A privileged local storage owner remains
inside the trust boundary, and the product is not a sole enterprise audit
authority or storage-owner tamper-resistant ledger.

### State and hosting

- Local filesystem state for ordinary local evaluation.
- Explicit migration into verified inactive SQLite staging and separate
  activation.
- PostgreSQL state with shared-state conformance, fenced hosted work claims,
  outcome reconciliation, and recovery coverage.
- A single-tenant hosted alpha API/worker that authenticates one trust domain,
  dispatches no-write work, and persists terminal reporting.
- Hosted submission/retry now reconciles the exact immutable run before
  acknowledgement. Approval-waiting and terminal exact runs can be returned
  directly; a `Running` run requires one matching queued work item plus pending
  invocation evidence, or an exact retry must atomically repair that dispatch.
  Changed intent and unsupported durable postures fail closed. The Accepted
  Means Dispatched release gate passed exact-head and post-merge CI plus an
  independent fault-injection rerun. That closes dispatch-truth Gate 1 only; it
  does not make the hosted alpha a production or reliable unattended execution
  product.
- Persisted WorkReport reads have an accepted bounded identity-binding
  implementation: physical run/report keys and available immutable workflow
  identity are checked before retained body commitment and before projection,
  recovery, reconciliation, or replay. Legacy body-commitment posture does not
  excuse a wrong storage identity. This remains local/backend integrity, not
  storage-owner tamper resistance. Exact-head CI, maintainer-operated
  post-merge evidence, and the independent backend-misplacement rerun pass;
  Gate 2 is closed.
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
- The GitHub repository-metadata fixture and opt-in live-read composition have
  a bounded accepted Gate 3 implementation: current project/provider
  restrictions and relevant definition
  context are committed at run creation, revalidated before approval acceptance,
  and revalidated again before atomic fixture/live provider admission. Focused
  review, drift tests, full local validation, exact-head CI, and an independent
  parent-versus-fix rerun passed. This does not generalize to undeclared
  provider families, provider writes, or credential-backed live acceptance.

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
- a guarantee that an operator-selected executable is an official or attested
  validator merely because its bytes and invocation were content-bound;
- cryptographic tamper evidence for local event or WorkReport storage, or proof
  against an owner who can rewrite both records and local integrity anchors;
- a production scheduler that continuously redispatches agents from continuity
  directives;
- a production replacement executor wired to the private cross-gate lifecycle
  binding and concrete current-state sources;
- continuity guarantees for hosted, storage, provider, or replay paths outside
  the explicitly reviewed Gate 1-3 boundaries; or
- project-resource continuity generalized to undeclared provider families,
  provider writes, or credential-backed live acceptance.

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

`Durable` means the selected backend can preserve and replay validated records;
it does not by itself mean independently attested or tamper-proof. Inspection
must be interpreted with the producer and storage assurance of the active path.

For the engineering explanation, continue to
[Workflow OS Explainer](../concepts/WORKFLOW_OS_EXPLAINER.md).
