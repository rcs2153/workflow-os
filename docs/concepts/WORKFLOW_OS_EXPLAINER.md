# Workflow OS Explainer

Workflow OS is a governance kernel for work performed by agents, humans,
deterministic programs, and external tools.

Its job is not to be the intelligence doing the work. Its job is to make the
work governable.

```text
Agent executes. Workflow OS governs.
```

That means an executor can still explore, reason, write code, call a reviewed
tool, or operate inside a sandbox. Workflow OS establishes the durable contract
around material actions: identity, immutable inputs, policy, authority,
required context, checks, approvals, side effects, evidence, event history,
reporting, and lawful continuation.

The project exists because autonomous execution without this contract is hard
to inspect and harder to improve. A prompt transcript may explain what a model
said. It does not reliably prove which workflow version ran, what authority was
current, whether required evidence existed, what side effect occurred, or why
the next action was allowed.

## The Engineering Thesis

Workflow OS separates three things that agent systems often collapse:

1. **Execution**: code edits, tool calls, provider requests, analysis, or human
   work.
2. **Governance**: validation, policy, authority, approvals, evidence and check
   obligations, side-effect boundaries, and reporting requirements.
3. **Orchestration**: deciding when and where an executor gets another turn.

Workflow OS is primarily the second layer. It implements some local execution
and hosted dispatch machinery because governance must be proven against real
state transitions, but it is not trying to replace the executor or become a
universal scheduler.

This boundary matters. If Workflow OS claimed to govern every internal model
decision, it would create brittle orchestration and false confidence. Instead,
it governs the transitions that change durable state, consume authority, expose
data, invoke tools, create side effects, satisfy gates, or close work.

## The Core Objects

### Project

A Workflow OS project contains a manifest and versioned definitions for
workflows, skills, policies, and tests. Rust is the canonical parser and
validator. The TypeScript package helps author compatible specs; it is not a
second runtime model.

### Workflow

A workflow is an authored contract for governed work. It identifies ordered
steps and their skill, policy, approval, retry, escalation, check, and reporting
requirements.

A workflow is not the agent's full reasoning graph. The executor can make many
local decisions inside a step. The workflow captures the boundaries that must
remain stable and inspectable.

### Run

A run is one durable execution of one exact workflow definition. It has stable
identity, state, event history, correlation metadata, and immutable input
commitments.

The run is not allowed to drift to the newest files on disk. Immutable run
bundles preserve the workflow and referenced declarations needed to prove what
was authorized. Approval resume and retry paths reject changed referenced
definitions rather than silently executing new work under old approval.

### Event log and projection

Meaningful transitions append events. A current snapshot is a projection for
efficient reads, not an independent source of truth. Backends must reconcile
snapshot and event history and must reject ambiguous concurrent transitions.

Events answer questions such as:

- when the run was created and validated;
- which step was scheduled;
- which policy decision was recorded;
- whether approval was requested, granted, or denied;
- whether a handler was invoked;
- which side effect was proposed, attempted, completed, failed, denied, or
  skipped;
- why the run completed, failed, escalated, canceled, yielded, or waited.

### Evidence and report

`EvidenceReference` is a bounded citation, not a payload warehouse. It records
stable identity, kind, scope, sensitivity, redaction posture, and a safe target.

`WorkReport` is the governed handoff. It can cite evidence, checks, audit events,
approvals, policy decisions, side effects, and provider outcomes while
disclosing incomplete work, limitations, risks, and operator notes. Reports are
not audit logs and do not replace the event stream; they are a terminal summary
whose claims point back to durable references.

### Side effect

A `SideEffect` records a material effect boundary independently from the skill
or provider that caused it. Proposal, authorization linkage, attempt, outcome,
and report citation are separate facts. This prevents “the tool returned OK”
from becoming the entire governance model for an external mutation.

## A Governed Run, Step By Step

The following is the conceptual local path. Specific APIs may expose only a
subset or add stronger gates.

### 1. Load

The project loader reads the manifest and declared spec directories. It avoids
network access and code execution. Source locations are retained for bounded
diagnostics.

### 2. Validate

Validation checks schema versions, identifiers, references, policy effects,
approval requirements, retry bounds, state transitions, local-check
declarations, and unsupported features. Unknown semantics fail closed.

This is important: a policy file is not decoration. Declared policy effects
must be understood by validation and the runtime path that claims to enforce
them. Unsupported effects are rejected rather than treated as meaningful prose.

### 3. Freeze authoritative inputs

Before sensitive execution paths, Workflow OS constructs an immutable run
bundle from referenced definitions and canonical declaration records. Content
addressing binds the run to the exact material it was created from.

This closes a classic approval time-of-check/time-of-use gap: editing a
workflow or skill after approval cannot transparently change what resumes.

### 4. Create or rehydrate durable state

The kernel creates a run or loads the existing run by ID. Idempotency records
prevent duplicate invocation from creating duplicate runs or repeating
completed effects. Rehydration verifies that stored state remains consistent
with event history and immutable identity.

### 5. Derive current governance posture

Before a material action, Core can evaluate:

- declared policy and action capability;
- current authority facts and scoped grants;
- required context availability;
- independent check results and assurance;
- approval requirements and presentation proof;
- sensitivity and side-effect posture;
- previous stricter governance decisions.

The proportional-governance model derives one of four postures:

- **quiet capture**: proceed and record evidence without interruption;
- **visible disclosure**: proceed, but surface a bounded disclosure;
- **blocking approval**: do not proceed until a valid approval is recorded;
- **denial**: prohibit the action.

The decision is monotonic: a weaker later assessment cannot silently relax a
stricter prior requirement.

Visible disclosure is represented separately from quiet capture because it is
an auditable delivery obligation, not merely a UI preference. A future UI may
render both in one stream, but the kernel must still know whether disclosure was
required and whether the target surface accepted it.

### 6. Present and decide approval

An approval request can be bound to a durable presentation record containing
the concrete scope, non-goals, touched surfaces, expected validation, and next
action. The decision path can require proof that the exact presentation was
delivered recently enough and matches the approval being decided.

Approval is not authority by itself. It is one input into an authorized path.
Current policy, capability, context, checks, immutable run identity, and
side-effect constraints may still block execution.

### 7. Invoke one explicit execution boundary

The local executor resolves an explicitly registered handler. No handler means
no execution. Fixed local-check profiles bind a reviewed command contract and
side-effect allowance; they do not open a generic shell escape hatch.

External integrations go through adapters or provider interfaces. The kernel
evaluates the preconditions, the provider performs the external operation, and
Core reconciles the provider outcome into durable state. Uncertainty after a
provider call is represented as uncertainty, not blindly retried.

### 8. Record outcome and report

Core appends the resulting events, stores bounded references, reconciles the
run state, and, on explicit report paths, produces a WorkReport. Artifact gates
can require that cited SideEffects, approval proof markers, and authority
receipts exist and match before a report artifact is written.

Report failure does not rewrite historical workflow success or failure. It is a
separate outcome that must be disclosed.

## Authority Is Not A Boolean

Workflow OS deliberately avoids a single `approved = true` concept.

Authority is composed from exact facts:

- actor and system-actor identity;
- project, workflow, run, step, and harness scope;
- capability and resource scope;
- issuance, expiration, and revocation posture;
- immutable bundle and current event cursor;
- policy and approval commitments;
- sensitivity limits;
- required context and check evidence.

The capability model can resolve whether one requested action has an exact
active grant. The current-authority source model can bind a fresh external or
durable authority snapshot. Step-scoped projections expose only capabilities
that remain authorized for that exact step.

These models do not create credentials or magically constrain an unrelated
agent process. Enforcement exists only where an execution path consumes them
before the protected action.

## Continuation: The Kernel, Not The Conversation, Chooses Next

Long-running agent work exposed a second control-plane problem: a model can end
a turn, lose context, or resume from a stale summary even while lawful work
remains.

Workflow OS therefore distinguishes:

- workflow lifecycle state;
- authority state;
- execution-window state;
- executor yield;
- genuine wait conditions;
- resume directives and attempt outcomes.

An executor turn ending is not workflow completion. If work can continue, the
kernel may project a resume disposition. If work genuinely cannot continue, it
must persist an exact typed wait or blocked condition.

The local CLI exposes a read-only `next-action` preview. Deeper continuity work
uses atomic SQLite operations for opening execution windows, registering yields
and waits, satisfying typed wake conditions, consuming one-winner directives,
starting attempts, and reconciling outcomes. The trusted-host caller and
supervisor work remains private and narrowly reviewed while the boundary
hardens. Workflow OS does not yet claim to be a production scheduler that can
create model turns by itself.

## State Backends

The repository contains several state postures with different claims:

- **Local filesystem** supports the ordinary local preview and inspectable
  event history. It does not claim every atomic continuity capability.
- **SQLite** is used for explicit migration/activation and the atomic local
  continuity implementation.
- **PostgreSQL** implements the shared-state milestone and supports the hosted
  alpha's fenced work claims, receipts, terminal projections, and recovery.

Backend capability is explicit. A backend cannot advertise an atomic operation
merely because it can store the same fields.

## Hosted And Sandbox Execution

The hosted crate is a single-tenant alpha, not Workflow OS Cloud. It provides an
authenticated API and stateless worker around PostgreSQL, one project trust
domain, Core-owned dispatch, provider outcome reconciliation, and terminal
report persistence.

The optional OpenShell integration follows the same separation of concerns:

- Workflow OS decides the governed request and required evidence;
- OpenShell supplies a sandbox and enforces runtime filesystem, process, and
  network policy;
- Workflow OS records bounded sandbox identity, policy, outcome, log, denied
  action, telemetry, and artifact references.

The current OpenShell provider is no-write and injected. The pinned CLI
compatibility layer can inspect lifecycle and effective policy but does not yet
provide all observations required by the full provider contract.

## Why YAML Exists, And Why YAML Is Not The Product

YAML is the portable authoring format for project and workflow contracts. It is
useful because definitions can be versioned, reviewed, validated, and hashed.

But users should not have to design every workflow manually. The current
onboarding path can inspect safe repository metadata, disclose governance gaps,
recommend workflow candidates, and author inactive drafts. Promotion remains
explicit: recommendation, draft, preflight, steward review, and catalog
promotion are distinct steps.

The long-term direction is governed workflow discovery and evolution from
observed work records, not uncontrolled workflow synthesis. Suggested changes
must not become active authority merely because an agent generated them.

## What The Current Product Proves

The repository proves that these concepts can be composed in a real kernel:

- a run can be tied to immutable definitions;
- approval can be paused, persisted, proven as presented, and resumed safely;
- policy and independent checks can fail closed before execution;
- local and hosted work can leave ordered durable history;
- external write attempts can be modeled and reconciled without granting broad
  provider authority;
- evidence and reports can cite bounded references instead of copying secrets;
- the lawful next action can be derived from durable state rather than model
  memory.

That is more than a schema library, but less than a turnkey enterprise agent
platform.

## What Remains

The largest unfinished product work is not another vocabulary layer. It is
broadening runtime composition without weakening the invariants:

- complete trusted-host continuation and redispatch;
- independent execution evidence and attestations;
- production-quality handler and sandbox integration;
- broader provider mutations only after authority and outcome reconciliation
  are proven;
- nested harness execution through scoped contracts and typed handoffs;
- enterprise identity, stewardship, policy administration, and multi-tenancy;
- an operator experience for quiet evidence, visible disclosures, approvals,
  workflow evolution, and reports;
- Reasoning Lineage after evidence, report, and harness boundaries are stable.

## The Practical Test

For any claimed Workflow OS capability, ask four questions:

1. Which exact path enforces it?
2. Which durable record proves it happened?
3. What fails closed when the proof is missing, stale, or ambiguous?
4. Is this available through a real handler/provider path, or only represented
   as a model or helper?

If those questions do not have concrete answers, the capability is roadmap
language, not runtime truth.

That standard is the point of Workflow OS.
