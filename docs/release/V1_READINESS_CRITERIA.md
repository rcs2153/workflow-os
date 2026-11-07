# Workflow OS V1 Readiness Criteria

Status: normative future release criteria; current preview does not satisfy
this contract.

## Product Definition

Workflow OS v1 must be a continuity-of-authority kernel for agentic work.

It must convert an ephemeral conversation or execution context into a durable,
policy-governed run whose intent, authority, execution state, evidence, and
next permissible action can be recovered by another authorized person,
process, or agent.

Evidence, audit records, SideEffects, and WorkReports are required proof and
inspection surfaces. They are not sufficient for v1 if the kernel cannot
preserve lawful authority and continuation.

## Required Contracts

### 1. Authority Integrity

For every supported v1 action path:

- authority is bound to exact actor, scope, resource, action, policy, immutable
  work identity, required facts, execution window, expiry, and revocation;
- a changed target, payload, workflow, policy, evidence obligation, or current
  authority fact cannot reuse the old grant;
- approval is not inferred from chat, agent memory, a checkpoint, or a report;
  and
- delegated authority remains narrower than the delegating authority.

### 2. Execution Admission

- an accepted request reaches one explicit approval wait, executable admission,
  resumable wait, denial, or terminal posture;
- exact retry repairs or acknowledges every valid durable prefix;
- duplicate work is prevented at the declared idempotency and provider-
  observation boundary;
- ambiguous external outcomes remain explicit and reconcilable; and
- no accepted request is silently stranded in a non-terminal state.

### 3. Identity Binding

- approvals, events, checks, EvidenceReferences, SideEffects, receipts,
  artifacts, and WorkReports bind to the exact run and operation;
- cross-run, cross-step, cross-resource, and cross-version substitution fail
  before projection or continuation;
- producer, host, and storage assurance remain explicit; and
- a privileged storage owner is not described as independently attested unless
  a separate accepted mechanism proves it.

### 4. Interruption Continuity

- a replacement supported executor can recover the current lawful next action
  from kernel-owned state;
- recovery works across process restart and at least one documented
  cross-harness handoff;
- a checkpoint or summary cannot authorize action by itself;
- executor yield does not falsely complete a non-terminal run; and
- a genuine wait remains durable and resumes without reconstructing authority
  from conversation history.

### 5. Adversarial Recovery

The supported state backends and execution paths must pass reviewed tests for:

- crash and restart at every durable transition;
- concurrent exact retries;
- backup and restore;
- stale, misplaced, contradictory, corrupted, and partially written state;
- changed current authority and revoked capability;
- report/evidence substitution; and
- external outcome ambiguity.

Failures must be bounded, payload-safe, inspectable, and recoverable only
through a documented lawful path.

### 6. Operational Cost

V1 evidence must measure the practical cost of safe recovery:

- elapsed time and latency;
- human interruptions and approval interactions;
- model turns and evidence lookups;
- gross, cached, and uncached token accounting where available;
- custom integration code and operator steps; and
- authoritative billed cost only when supplied by the provider.

Efficiency claims are secondary to correctness. A lower-cost path that violates
authority, identity, admission, or recovery is rejected.

## Cross-Cutting V1 Gates

Before a v1 declaration:

- the Current Product Contract and public positioning must match exact `main`;
- each supported path must identify what Core proves, what a host asserts, what
  a provider reports, and what remains unverified;
- clean-room installation and the named top-level validation commands must
  succeed from a fresh checkout;
- exact-head CI and required credential-backed tests must be terminal green;
- release source, binaries, archives, checksums, manifests, and tags must agree;
- at least one independent security/release review must cover authority,
  admission, identity, recovery, and provenance; and
- known limitations must not contradict the v1 claim.

V1 does not inherently require a hosted SaaS, visual workflow builder,
multi-tenant control plane, general agent runtime, recursive agents, or broad
provider-write catalog. It requires the declared supported boundaries to be
truthful, recoverable, and continuity-preserving.

## Release Decision

A v1 release decision must map evidence to every criterion above and identify
unsupported paths explicitly. Documentation volume, model completeness,
passing mock demos, or a successful happy path cannot substitute for these
contracts.
