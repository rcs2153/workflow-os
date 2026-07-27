# Governed Context Access Projection Plan Report

## 1. Executive Summary

Planning now defines the next scoped-authority boundary: a pure, step-scoped
projection of authorized context references and bounded enumerated metadata.
The plan preserves the distinction between knowing or citing a reference and
being authorized to read its target.

No model, runtime, dereference, persistence, schema, SDK, CLI, provider,
sandbox, or write behavior is implemented.

## 2. Scope Completed

- Defined context-reference and context-authority source-of-truth boundaries.
- Defined reference-only and bounded-metadata access posture.
- Defined a fixed typed stable-reference taxonomy using existing Core
  identities.
- Defined exact access-level capability identifiers and canonical
  context-reference resource binding.
- Defined composition with current capability-resolution authority.
- Defined deterministic step-scoped projection semantics.
- Required complete evaluated-candidate retention and exact wire
  recomputation of entries and gaps.
- Defined unavailable, unknown, unauthorized, and independently evaluated gap
  posture without fake references.
- Defined freshness and time-of-use re-resolution obligations.
- Defined privacy, redaction, serde, and stable-error requirements.
- Defined relationships to EvidenceReference, typed handoffs, WorkReport,
  proportional governance, SideEffects, Composable Harness Contracts, and
  optional sandbox providers.
- Defined a focused future test plan and implementation sequence.

## 3. Scope Explicitly Not Completed

- No context or evidence target dereference.
- No source, report, event, transcript, prompt, or memory payload access.
- No model or helper implementation.
- No runtime consumption or time-of-use enforcement.
- No tool loading, command execution, connector activation, or provider call.
- No OpenShell integration or sandbox lifecycle.
- No SideEffect execution or writes.
- No persistence, events, audit projection, or authority receipts.
- No schemas, SDKs, CLI behavior, UI, hosted administration, or release
  changes.

## 4. Planning Decision

Context visibility will reuse scoped capability authority rather than create a
parallel permission system. A future projection entry must retain the exact
authorized source resolution used to select it and remain bound to actor,
workflow, run, step, optional harness, evaluation time, resource, sensitivity,
and access level.

An EvidenceReference, WorkReport citation, event, handoff, or known object ID
does not authorize payload access. Availability also does not imply authority.

## 5. First Implementation Boundary

The first implementation should add only:

- typed stable context references for already-supported Core IDs;
- reference-only and bounded-metadata access levels;
- explicit availability and bounded gap posture;
- a pure deterministic step-scoped projection helper;
- validated serde, redaction-safe Debug, stable errors, and focused tests.

`reference_only` maps exactly to `context.reference.view`;
`bounded_metadata` maps exactly to `context.metadata.view`. Both use a
Core-derived `ContextReference` capability resource for the typed target. The
projection retains every evaluated candidate and validates that its serialized
entries and gaps are the exact deterministic derivation.

The helper must not read hidden state, dereference targets, contact stores,
inspect repositories, load tools, invoke providers, mutate runs, or emit events.

## 6. Privacy And Security Posture

The plan forbids raw source, evidence, report, event, transcript, prompt,
provider, command, parser, environment, and credential payloads. Bounded
metadata must be a fixed typed set rather than an unrestricted map. Debug and
errors must not echo stable identities or rejected caller values.

Serialized projections remain sensitive authority snapshots. They are not
self-authenticating receipts and cannot authorize future dereference without
fresh time-of-use resolution.

## 7. Test Coverage Planned

The test plan covers valid projections, all implemented reference kinds,
authority and scope mismatch, availability, independent prerequisites,
sensitivity, access-level narrowing, expiry/revocation, duplicates,
deterministic ordering, serde tampering, non-leakage, forbidden payload
absence, and existing governance regressions.

## 8. Commands And Results

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Governed phase close: completed successfully; see the governance summary
  below.

## 9. Dogfood Governance

- workflow: `dg/d`
- run ID: `run-1785124217091576000-2`
- approval ID:
  `approval/run-1785124217091576000-2/planning-approved`
- presentation ID: `presentation/6d2642265c2d52e9`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was reviewed and presented.
- phase status: completed;
- approval-presentation proof: persisted and enforced;
- event summary: 39 events, one approval, zero retries, and zero escalations.

Planning and documentation edits, documentation validation, and Git operations
were performed by the delegated maintainer outside the kernel. The kernel
governed phase scope, approval, durable event history, and close disclosure; it
did not edit files, run documentation checks, or perform Git operations.

The first phase-start attempt failed closed before creating a run because one
approval-context field exceeded the runner's bounded-line contract. The phase
was restarted with shorter wording rather than bypassing that validation.

## 10. Remaining Limitations

- Required-context contract consumption is not designed yet.
- Projection snapshots have no durable immutable binding or runtime freshness
  lease.
- No audited dereference path or authority receipt exists.
- No sandbox or harness runtime consumes a context projection.

## 11. Recommended Next Phase

Perform a focused maintainer review of this planning boundary. If accepted,
implement the governed context-access core model and pure step-scoped
projection helper with reference-only and bounded-metadata behavior.
