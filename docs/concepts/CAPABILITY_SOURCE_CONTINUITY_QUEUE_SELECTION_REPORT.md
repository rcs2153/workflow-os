# Capability Source Continuity Queue Selection Report

## 1. Executive Summary

The next executable roadmap phase is one private, local, Core-owned durable
source-continuity boundary for the accepted capability-gated GitHub repository-
metadata read.

The accepted read-only integration validates an exact-query current-authority
snapshot and compares a supplied prior generation and watermark before provider
invocation. Its focused review correctly limited acceptance to the crate-private
slice because a caller can still select `FirstObservation` when no prior state
is supplied. That is truthful for an isolated helper, but it is not sufficient
for executor-facing or public adoption.

The next milestone must remove that caller choice. Core must derive first or
prior observation from trusted durable state, advance continuity monotonically,
and compose the result into the existing read-only operation before the
injected adapter client can run.

## 2. Queue Decision

The Active Delivery Queue now contains:

1. the dependency-tracked draft-pull-request live sandbox proof; and
2. Core-owned durable capability-source continuity for the accepted GitHub
   repository-metadata read.

The second item is executable without live provider access. The first remains
blocked until the dedicated sandbox access prerequisite is available and must
not stall unrelated no-write hardening.

## 3. Why This Phase Is Next

The previous capability milestone is implemented, blocker-fixed, and accepted.
Its review records one explicit adoption prerequisite: prior source continuity
must come from Core-owned durable state, and an untrusted caller must not be able
to downgrade an established source to `FirstObservation`.

Selecting that prerequisite next has three advantages:

- it closes a concrete accepted integrity limitation rather than introducing a
  new primitive family;
- it advances an existing runtime path without adding another provider
  operation or mutation; and
- it turns rollback detection from caller-assisted validation into a property
  owned by the governance kernel.

Recent external feedback does not justify reopening already-closed foundations.
Immutable run inputs, approval/resume context integrity, independent local-check
proof, proportional-governance routing, quiet-success output, existing-agent-
instruction preservation, and first-use CLI defects all have accepted runtime
or product corrections on current `main`. Actor and enterprise RBAC broadening
remains important, but it is a larger authority program and is not a substitute
for fixing the exact continuity limitation in the selected read-only slice.

## 4. Required Implementation Boundary

The milestone should define, review, and implement one private local boundary
that:

1. keys continuity to the exact registered current-authority source and exact
   repository-read query identity;
2. reconstructs the latest trusted generation and watermark from Core-owned
   durable state;
3. derives first observation only when no trusted record exists;
4. rejects lower generations and changed watermarks at the same generation;
5. accepts a higher generation only through one atomic monotonic transition;
6. reconciles exact replay without weakening the prior observation;
7. rejects conflicting concurrent transitions deterministically;
8. supplies continuity to the existing exact-query snapshot validation rather
   than accepting caller-authored continuity;
9. fails before the injected GitHub adapter client is invoked on any continuity
   error; and
10. records only payload-free commitments and bounded source metadata.

The implementation contract and focused security review belong at the start of
the same governed vertical-slice milestone. A demonstrated authority,
concurrency, migration, recovery, or privacy blocker must be corrected before
implementation continues, but the work should not fragment into an open-ended
documentation chain.

## 5. Storage And Atomicity Posture

The first implementation should use an existing local durable-state abstraction
that can prove create-only first observation, monotonic compare-and-advance,
exact replay, conflict rejection, and restart behavior. It must not create a
parallel authority database or treat a process-local cache as trusted history.

The continuity update and operation admission must have an explicit ordering.
If a crash or storage ambiguity can leave it unclear whether continuity
advanced, the subsequent call must reconcile from durable state and fail closed
when exact outcome cannot be proven. Provider execution must never be used to
discover whether continuity was accepted.

## 6. Privacy And Error Posture

Durable continuity may retain source registration identity, exact query
commitment, generation, watermark commitment, snapshot commitment, bounded
timestamps, and version metadata. It must not retain raw grants, availability
records, repository contents, GitHub payloads, credentials, authorization
headers, environment values, command output, or secret-like caller strings.

Debug, serialization, storage, recovery, and validation errors must use stable
non-leaking codes. They must not echo repository input, source records,
watermark values, provider payloads, or local paths.

## 7. Required Test Matrix

The implementation milestone must prove:

- create-only first observation;
- exact restart reconstruction;
- matching prior observation;
- higher-generation monotonic advancement;
- lower-generation rollback rejection;
- same-generation watermark substitution rejection;
- missing current generation after prior observation rejection;
- exact replay without duplicate advancement;
- one-winner concurrent advancement and deterministic loser posture;
- conflicting concurrent advancement rejection;
- exact source, query, repository, and execution-binding substitution
  rejection;
- no adapter client call on every continuity failure;
- successful read and receipt behavior remains unchanged after accepted
  continuity; and
- storage, Debug, serialization, and errors do not leak forbidden data.

Focused tests should use injected clients and local durable fixtures. Live
GitHub access is not acceptance evidence for this phase.

## 8. Strict Non-Goals

This selection does not authorize:

- another GitHub operation or any provider mutation;
- public or executor-facing capability APIs;
- automatic gating of existing direct adapter APIs;
- workflow schema, SDK, CLI, or runtime configuration;
- automatic capability grants or inferred authority;
- reusable authority from a continuity record, snapshot, or receipt;
- hosted identity, enterprise RBAC, or multi-tenant state;
- OpenShell integration or nested harness execution;
- raw authority-fact persistence; or
- release posture changes.

## 9. Feedback Reconciliation

The latest user evaluations reinforce the project's operating direction:

- the kernel's strongest value is the inspectable separation of scope,
  execution, validation, closure, and publication;
- low-risk work should complete quietly while preserving durable evidence;
- run inputs and approval context must remain immutable and freshly verified;
- real check evidence must be distinguished from mocks; and
- authority must be enforced by kernel-owned facts rather than conversational
  memory or caller assertion.

Current `main` already contains accepted work for the first four points. The
selected continuity phase advances the fifth on a real adapter path. The user
feedback therefore changes emphasis, not architecture: prefer enforceable
runtime composition over additional broad concept families.

## 10. Governed Phase Record

- workflow ID: `dg/workflow-discovery`
- run ID: `run-1792697905951489000-2`
- approval ID:
  `approval/run-1792697905951489000-2/stewardship-approved`
- presentation ID: `presentation/a4d38f0303a199cb`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 49 events, 1 approval, 0 retries, and 0 escalations; the
  ordered trail includes the approval request and grant, ten policy decisions,
  eight scheduled and successful discovery steps, and terminal completion
- approval-presentation enforcement: proof persisted and used by the explicit
  approval command
- validation summary: `npm run check:docs` and `git diff --check` passed
- kernel boundary: governance coordination only; repository inspection,
  selection analysis, document editing, validation, Git, pull-request, CI, and
  merge work remain outside the kernel

## 11. Validation

This planning phase requires:

- `npm run check:docs`; and
- `git diff --check`.

## 12. Recommended Next Phase

Execute the Core-owned durable capability-source continuity vertical slice.
Start with the focused implementation contract and security review, then
implement the accepted boundary in the same governed milestone. Stop for a
demonstrated authority, atomicity, recovery, or privacy blocker. Do not broaden
to another provider operation or mutation.
