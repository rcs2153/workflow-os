# Continuation Context Accepted-Intent Capture Report

## Status

Phase D is implemented and focused-review accepted as a Core-private
accepted-intent capture and collection contract. Aggregate continuation-context
composition may proceed only as a separate governed phase.

## Delivered

Core now has a bounded contract for preserving the accepted semantic effect of
one attributable user or delegated-maintainer directive before conversational
context can disappear.

Each `AcceptedIntentRecord` binds:

- a stable record identity;
- the attributable actor;
- a payload-free directive commitment;
- exact project, workflow, run, and phase scope;
- objective, amendment, priority, pause, stop, or resume kind;
- bounded semantic meaning; and
- exact accepted-intent record identities superseded by this record; and
- current or superseded posture.

The record is orientation input only. It does not represent approval, policy
satisfaction, capability, required context, evidence, a check result, or
execution authority.

## Capture Boundary

The private capture request binds one record, an exact expected store revision,
and explicit record identities to supersede. Capture:

- requires a new record to be current;
- canonicalizes and rejects duplicate supersession identities;
- rejects stale expected revisions;
- replays an exact existing record without a second write;
- rejects record-identity rebinding;
- rejects one directive commitment being rebound to another record;
- requires every superseded record to exist and remain current;
- rejects supersession across objective, amendment, priority, or control-state
  families;
- requires exact supersession of an existing singleton objective, priority, or
  pause/stop/resume posture;
- commits supersession and the replacement record atomically; and
- rereads and verifies the exact committed revision and record.

Focused review corrected the initial replay model. Supersession lineage is now
part of the durable record rather than ephemeral write-request context, and an
exact replay may present the original pre-write expected revision only when the
entire already-stored record, including lineage, is identical.

The store interface is compare-and-replace rather than a second event ledger or
permission system. A conforming backend must atomically reject a stale expected
revision without changing state.

## Collection Boundary

The Phase D collector reads one exact-scope snapshot and binds its revision into
the complete continuation collection anchor. A changed store revision after
anchor capture fails closed.

Observed accepted intent travels as typed `AcceptedIntentRecord` values rather
than as a caller-authored generic source payload. Core later derives the
accepted-intent source identity, authority domain, semantic claim, sensitivity,
and canonical record revision from those records. Other collectors cannot
substitute accepted-intent records.

Empty snapshots carry the exact Core-owned typed negative-knowledge accounting
required by the purpose and scope profile. Collection performs no writes and
consumes no authority.

## Persistence Truth

This phase defines the durable store contract and an in-memory conformance
fixture. It does not ship a production filesystem or PostgreSQL implementation,
register the store with a host, ingest chat transcripts, or claim that accepted
intent survives process restart in a released runtime.

A production backend and host capture path require separate implementation and
review. The host must provide attributable directive identity and a bounded
semantic effect; Core must not infer acceptance from raw conversation.

## Focused Coverage

Six Phase D tests prove:

- attributable capture and exact replay;
- atomic singleton supersession;
- cross-family supersession rejection;
- stale revision rejection;
- directive rebinding rejection;
- revision-fenced non-authorizing collection; and
- debug redaction of actor, meaning, and commitments.

The existing collector conformance suite additionally proves that accepted
intent survives aggregate observation conversion without generic source
substitution.

## Governed Implementation

- workflow: `dg/implement`
- run ID: `run-1793709908239188000-2`
- approval ID:
  `approval/run-1793709908239188000-2/implementation-approved`
- presentation ID: `presentation/f99f3785db0d9917`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- focused continuation-context tests: 73 passed
- strict `workflow-core` clippy with warnings denied: passed with the existing
  narrow dead-code allowance for private future-facing models
- full workspace tests: passed; `workflow-core` ran 712 unit tests and all
  workspace integration and doc tests were green
- rustdoc with warnings denied: passed
- formatting: passed
- documentation and diff checks: passed
- focused maintainer/security review: accepted after exact replay, durable
  supersession-lineage, and capture-result debug-redaction corrections

## Boundary Preserved

This phase did not persist raw conversation or hidden reasoning, create a
production store, add host ingestion, expose CLI/schema/SDK behavior, consume
approval or capability authority, invoke a runtime consumer, perform provider
work, add nested harness behavior, add Salsa, or expand hosted behavior.

It also does not yet prove whether an intent is already fully represented by an
immutable run, typed handoff, accepted plan, or approval presentation. The
future aggregate composition boundary must prefer those existing authoritative
sources and must not duplicate their meaning into accepted-intent storage.

## Out-Of-Kernel Work

The delegated maintainer implemented the private model, store contract,
collector, conformance fixture, focused tests, roadmap update, and this report;
ran validation; and reviewed diagnostics outside the kernel. The kernel
governed scope and approval but did not edit files, execute tests, create
commits, or mutate GitHub.
