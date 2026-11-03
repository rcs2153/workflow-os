# Continuation Context Trusted Local Collectors Plan

## Status

Planned P0 increment. No collector, persistence, public contract, or runtime
consumer is implemented by this document.

## 1. Decision

Implement trusted local continuation-context collection as a Core-owned,
closed, read-only composition boundary.

Collectors will not construct authoritative context decisions, choose required
sources, select semantic obligations, resolve conflicts, or authorize work.
They will return bounded typed observations from existing authoritative stores
and explicitly injected local observers. Core will own:

- which collector kinds exist;
- the exact source class and authority domain for each kind;
- which collectors are required for each purpose and scope;
- result validation and source-accounting semantics;
- anchor capture and post-collection stability verification;
- conversion into coverage, claim accounting, and source definitions;
- deterministic ordering and canonical commitments; and
- the final non-authorizing continuation-context decision.

This preserves the accepted rule that the kernel, not a caller or agent,
defines what must be understood before material continuation.

## 2. Problem

The accepted continuation-context model receives explicit validated inputs. It
does not yet establish how those inputs are obtained from current local truth.
A host could currently omit a source class, relabel a failure as empty, attach
the wrong authority domain, inject an untrusted semantic candidate, or combine
facts observed at incompatible run cursors.

That is safe while the model remains private and test-only. It is not a valid
foundation for post-compaction rehydration, replacement-executor continuity,
or a claim that current kernel state outranks model memory.

## 3. Security Invariants

1. Collector registration is closed and Core-owned. A caller cannot register a
   new authoritative collector or substitute one collector for another.
2. Collector kind determines source class, authority domain, allowed claim
   kinds, sensitivity ceiling, and accepted result shapes.
3. Coverage expectation and semantic obligation remain derived from purpose
   and validated scope. Collectors cannot weaken either profile.
4. `Empty` is a successful typed observation, not absence of a result. It is
   valid only for collector kinds whose contract can prove complete negative
   knowledge for the exact bound scope and revision.
5. `Unsupported`, `Denied`, `Failed`, `Stale`, and `OmittedOptional` remain
   distinct. None may be converted to `Empty`.
6. Every observation binds the exact project, workflow, run when applicable,
   purpose, actor, source revision, freshness basis, and collection anchor.
7. Run-bound collection captures an immutable bundle binding and event cursor
   before collection. Every independently mutable authoritative source also
   supplies a source-specific revision fence. Core rereads all required fences
   after collection; any change invalidates the complete set before derivation.
8. Pre-run collection binds the validated project-definition commitment and
   Git/worktree observation revision. Drift requires a fresh collection.
9. Collection never grants approval, capability, policy permission, or
   execution authority. Serialized observations and decisions remain
   non-authorizing.
10. No collector crawls arbitrary repository content, follows symlinks outside
    the project root, reads environment values, loads credentials, copies raw
    provider payloads, or captures conversation history or hidden reasoning.
11. Sensitive source existence, locators, revisions, and exact counts remain
    private unless a separately reviewed redacted projection permits them.
12. A collector failure cannot partially update durable governance state. This
    increment is read-only and produces one ephemeral canonical result.

## 4. Closed Collector Contract

The first model should define a private closed `ContinuationCollectorKind`.
Each variant maps in Core to exactly one existing
`ContinuationContextSourceClass` and
`ContinuationContextAuthorityDomain` pair.

The collector receives a Core-built `ContinuationCollectionRequest` containing
only validated bindings:

- purpose;
- exact project identity;
- workflow and run identity when run-bound;
- immutable run-bundle binding when run-bound;
- current event cursor anchor when run-bound;
- actor and capability-filtered access posture;
- maximum sensitivity;
- materialization budget partition; and
- Core trusted-time observation.

The collector returns one `ContinuationCollectionObservation` with:

- collector kind;
- exact request binding commitment;
- source revision and freshness basis;
- a source-specific pre-collection revision fence and the exact operation used
  to verify it after collection;
- typed access, integrity, and sensitivity posture;
- one closed result posture;
- bounded source references;
- bounded typed semantic candidates only where that collector contract permits
  them; and
- bounded typed empty-accounting entries only where complete negative
  knowledge is provable.

The first closed result postures are:

- `Observed`;
- `Empty`;
- `Unavailable`;
- `Denied`;
- `Failed`;
- `Unsupported`;
- `Stale`; and
- `OmittedOptional`.

No free-form result status, source class, authority domain, or claim kind is
accepted from a collector implementation.

An authoritative collector is not supported merely because its store can be
queried. It is supported only when the existing interface can produce a stable
source revision or an equivalent exact snapshot proof that can be verified
after collection. A best-effort sequence of unrelated reads cannot produce a
ready aggregate decision. Missing revision support remains typed
`Unsupported` until a separately reviewed read contract exists.

## 5. Source Ownership Matrix

| Collector kind | Authoritative local source | Source class | First implementation posture |
| --- | --- | --- | --- |
| Validated declarations | Already validated project definitions and catalog commitment | Validated declaration | Implement first |
| Immutable run bundle | Existing immutable-run store and exact run binding | Immutable run bundle | Implement first |
| Durable run state | Existing run snapshot rehydrated from the selected backend | Durable run state | Implement first |
| Event history | Existing append-only event store through the captured cursor | Event history | Implement first |
| Policy | Exact policies bound into the validated project or immutable run | Policy | Implement first |
| Approval | Existing approval records and presentation proof bound to the run | Approval | Implement first |
| Capability | Existing scoped capability and current-authority records | Capability | Implement first |
| Required context | Existing required-context declarations, consumption, and resolution records | Required context | Implement first |
| Evidence | Existing evidence references and validation posture | Evidence | Implement first |
| Check | Existing check declarations and authoritative result references | Check | Implement first |
| SideEffect | Existing SideEffect records and linkage posture | SideEffect | Implement first |
| WorkReport | Existing terminal or current report references and artifact integrity posture | WorkReport | Implement first |
| Accepted intent | Future typed accepted-intent store | Accepted intent | Explicitly unsupported until the separate capture boundary exists |
| Typed handoff | Existing exact typed handoff records when present | Typed handoff | Implement after the first store-backed batch |
| Git observation | Explicit injected read-only Git/worktree observer | Git observation | Implement after store-backed collectors |
| Operating document | Exact allowlisted project-root documents | Operating document | Implement after store-backed collectors |
| Provider observation | Existing bounded provider observation records | Provider observation | Keep unsupported until one accepted provider observation path is selected |
| Agent advisory | Explicit advisory input from the current host | Agent advisory | Never trusted; optional and advisory-only |

`Implement first` means the implementation phase defines and tests private
collectors against existing interfaces. It does not make the aggregate boundary
public or operational.

## 6. Structured Claims And Accounting

Collectors must not infer governed meaning from arbitrary prose.

Structured kernel records may emit only claim kinds explicitly permitted by
their collector contract. For example:

- validated declarations and immutable run inputs may emit objective, scope,
  non-goal, check, SideEffect, validation, and reporting commitments already
  represented by validated structured fields;
- durable run state and events may emit completed, in-progress, remaining,
  blocked, waiting, and terminal posture derived from typed state and events;
- approval, capability, check, evidence, SideEffect, and WorkReport collectors
  may emit only their existing typed posture and references; and
- Git and operating-document collectors initially emit inventory and bounded
  reference posture, not authoritative natural-language claims.

An `Empty` observation may produce typed required-accounting entries only for
claim kinds owned by that collector contract and only after complete source
enumeration for the exact bound revision. An empty query result is not enough
when the store cannot prove completeness.

Operating documents remain guidance and constraints. They cannot grant
approval, capability, policy permission, evidence, check success, or execution
authority. The first allowlist is exact and intentionally small:

- repository-root `AGENTS.md` when present;
- `docs/ENGINEERING_STANDARD.md` when present; and
- `ROADMAP.md` when present.

No glob, recursive discovery, linked-plan inference, or arbitrary document
selection is allowed in the first slice. Each path is canonicalized beneath the
project root, symlinks are rejected, size is bounded, and the initial
observation records a content commitment and bounded metadata rather than
materializing full prose into claims.

## 7. Stable Collection Boundary

Collection must not combine facts from incompatible moments.

### Run-bound collection

1. Rehydrate the exact run and immutable bundle.
2. Capture the event cursor and the source-specific revision fence for every
   independently mutable authoritative store selected by the profile.
3. Derive the Core-owned collector and semantic profiles.
4. Invoke every expected collector, including those expected to return empty,
   unsupported, or denied posture.
5. Reread the immutable bundle binding, event cursor, run terminal posture, and
   every selected source-specific revision fence.
6. Reject the entire observation set as stale if any anchor changed.
7. Validate every observation against the closed collector contract.
8. Convert the complete set into coverage, source definitions, and claim
   accounting.
9. Call the accepted pure continuation-context derivation.

### Pre-run collection

1. Validate the project and capture the canonical definition commitment.
2. Capture the injected Git/worktree observation revision.
3. Derive the Core-owned profiles and invoke the expected collectors.
4. Revalidate the project commitment and Git/worktree revision.
5. Reject drift and rebuild rather than mixing observations.

This is optimistic read consistency, not a cross-store transaction. The later
same-call material consumer must always perform a fresh collection and may not
reuse a preview result as authority.

### Observation versus consumption

Collection is read-only. It may observe that required context, a capability,
or another one-use resource exists and is current, but it cannot consume,
reserve, claim, or burn that resource. Preview and composition therefore use
non-consuming observation interfaces only.

At a later material boundary, the existing same-call consumer must freshly
recollect and revalidate the context, then perform the authoritative one-use
consumption through the existing atomic boundary. The collector output cannot
stand in for that consumption and cannot be replayed as permission.

If an existing store exposes only a consuming read, that collector remains
`Unsupported`; this plan does not authorize adding a side effect to collection.

## 8. Determinism And Commitments

Canonical output must not depend on collector registration order, filesystem
iteration order, map order, or event retrieval order.

Core sorts observations by closed collector kind and source identity, validates
duplicate and missing results, and commits to:

- request binding;
- collector-contract version;
- collector and semantic profile commitments;
- pre- and post-collection anchors;
- every result posture and revision;
- every omission and failure posture;
- selected source references and claim accounting; and
- the resulting inventory, bundle, understanding, and decision commitments.

Debug, error, event, and report surfaces remain payload-free. Canonicalization
tests must prove discovery-order independence and relevant-input sensitivity.

## 9. Implementation Sequence

### Phase A: private collector model and reference composition

- Add closed collector kinds, request, result posture, observation, anchor, and
  composition types in Core.
- Add an in-memory registry containing exactly one collector per closed kind.
- Implement contract validation and pure conversion into the accepted decision
  input.
- Use fixture collectors only in conformance tests.
- Keep every new type crate-private unless a test requires narrower exposure.

### Phase B: store-backed kernel collectors

- Implement validated-declaration, immutable-run, durable-state, event-history,
  policy, approval, capability, required-context, evidence, check, SideEffect,
  and WorkReport collectors against existing read interfaces.
- Add no new persistence tables or duplicate stores.
- Require a source-specific revision fence or exact snapshot proof for each
  independently mutable store. Represent unsupported backends, unavailable
  interfaces, and stores without stable read contracts explicitly.
- Use non-consuming observation interfaces. Do not spend required context,
  capabilities, approval authority, or idempotency state during collection.

### Phase C: bounded local observers

- Add one injected Git/worktree observer returning typed bounded metadata.
- Add exact allowlisted operating-document observations with canonical path,
  symlink, size, and sensitivity enforcement.
- Add typed-handoff collection only from the existing handoff store.

### Phase D: accepted-intent boundary

- Implement the separately reviewed accepted-intent capture and persistence
  boundary.
- Replace the accepted-intent collector's `Unsupported` posture only after that
  store is accepted.

### Phase E: aggregate composition

- Compose all observations through existing governed-context access and
  non-consuming required-context observation.
- Add anchor reread and full-set rejection on drift.
- Produce one private non-authorizing decision.

One separately reviewed same-call consumer may later recollect, validate, and
consume required context through the existing atomic execution boundary. Phase
E itself does not consume it.

Each phase requires focused maintainer/security review before the next phase.
No preview or runtime consumer is authorized until Phase E is accepted.

## 10. Conformance Tests

The first implementation must prove:

- every Core-expected collector kind is represented exactly once;
- a caller cannot omit, duplicate, substitute, or relabel a collector;
- every kind maps to exactly one source class and authority domain;
- result posture cannot contradict the Core-owned expectation profile;
- `Empty` is accepted only with complete typed accounting;
- unsupported, denied, failed, stale, and omitted remain distinguishable;
- collector order does not change canonical outputs;
- a changed run cursor or bundle binding invalidates the entire set;
- a changed approval, capability, evidence, check, SideEffect, WorkReport, or
  other independently mutable source fence invalidates the entire set;
- a store without a stable revision or exact snapshot proof remains
  unsupported and cannot contribute to readiness;
- pre-run declaration or Git drift invalidates the entire set;
- immutable-run facts outrank current declarations for active runs;
- advisory input cannot satisfy a required claim;
- operating documents cannot emit approval, capability, evidence, check, or
  authority claims;
- arbitrary paths, traversal, symlinks, and over-budget documents fail closed;
- sensitive or unauthorized source details are not emitted in public or debug
  surfaces;
- collector panic or partial failure cannot produce a ready decision;
- no durable state or event is written by collection; and
- required context, capabilities, approval authority, and idempotency state are
  not consumed by collection; and
- the same complete source revisions produce byte-stable commitments after
  process restart.

## 11. Acceptance Criteria

- Core owns the complete collector registry and source mapping.
- Every expected source class has one explicit result posture.
- No caller can downgrade collector coverage or semantic completeness.
- The aggregate result is bound to stable pre- and post-collection anchors.
- Existing stores remain the sole owners of their facts.
- No parallel database, event ledger, policy engine, or permission model exists.
- Required gaps and all failure postures fail closed before any later consumer.
- Optional omission remains inspectable without fabricating knowledge.
- Collection output remains private, bounded, deterministic, redaction-safe,
  and non-authorizing.
- Focused review accepts the collector model before store-backed implementation.

## 12. Explicit Non-Goals

This plan does not authorize:

- collector implementation in the planning phase;
- accepted-intent persistence;
- raw conversation or hidden-reasoning capture;
- arbitrary repository indexing or semantic search;
- public CLI, schema, SDK, report, hook, or child-harness exposure;
- execution, approval, capability, or provider authority;
- local command execution by Core;
- provider reads or writes beyond already accepted observation stores;
- compaction host integration or delivery receipts;
- nested harnesses, agent teams, memory infrastructure, or hosted control
  planes;
- Salsa or another incremental computation dependency; or
- a claim of compaction-safe autonomous continuation.

## 13. Next Review

Perform a focused architecture/security review of this plan. The review must
challenge collector trust, source completeness, stable-anchor semantics,
negative knowledge, structured-claim ownership, operating-document boundaries,
sensitivity leakage, and whether any proposed collector duplicates an existing
source of truth. Only an accepted review may authorize Phase A.
