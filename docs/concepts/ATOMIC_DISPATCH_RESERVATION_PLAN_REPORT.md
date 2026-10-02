# Atomic Dispatch Reservation Plan Report

## 1. Executive Summary

The bounded atomic dispatch-reservation phase is planned. The plan replaces
the accepted one-shot supervisor's read-only dispatchability check with a
private Core/SQLite transaction that admits exactly one claimant before
executor entry.

This phase produced planning artifacts only. No runtime behavior, schema,
executor, provider, or public contract changed.

## 2. Scope Completed

- Defined the concurrent duplicate-dispatch threat.
- Defined one-winner reservation and exact binding invariants.
- Defined a non-cloneable, non-serializable private capability boundary.
- Defined durable payload-free reservation and projection posture.
- Defined replay and ambiguity behavior that never reissues authority.
- Defined supervisor integration without a repeated loop.
- Defined trusted-time, privacy, migration, reconciliation, and test posture.
- Defined a small implementation sequence and focused review gate.

## 3. Scope Explicitly Not Completed

- No reservation types or runtime helper were implemented.
- No SQLite schema or migration was added.
- No supervisor code changed.
- No scheduler, repeated dispatch, model turn, provider, OpenShell, nested
  harness, automatic approval, CLI, SDK, public config, schema, example,
  hosted behavior, or new mutation family was added.

## 4. Core Decision

Only the first unambiguous successful reservation transaction may receive an
in-memory dispatch capability. Exact replay may receive a receipt but no
capability. Ambiguous commit reconciliation may prove admission but must return
an admitted-without-capability posture. Durable state is evidence, not a
reconstructable execution credential.

## 5. Proposed Persistence Boundary

The plan recommends an additive private SQLite reservation relation keyed by
attempt identity. The exact reservation derives a `dispatch_admitted` posture
while preserving the accepted attempt outcome vocabulary. Result, yield, and
ambiguity persistence will require the reservation binding.

Filesystem and PostgreSQL support remain deferred.

## 6. Security And Privacy Posture

Reservation binds the immutable invocation, selected executor, window,
attempt, actor, authority, governance, cursor, revision, trusted-time epoch,
and expiry posture. Persisted records contain only bounded identities,
commitments, revisions, and timestamps. No prompt, transcript, source,
command output, environment value, credential, or provider payload is in
scope.

## 7. Validation

Required planning validation:

- `npm run check:docs` passed.
- `git diff --check` passed.

## 8. Remaining Limitations

- The design is not implemented or reviewed yet.
- Crash after admission but before executor entry remains fail-closed and
  requires future explicit recovery design.
- The first implementation remains private, local, and SQLite-only.
- No broader execution topology is authorized.

## 9. Recommended Next Phase

Perform a **focused maintainer/security review of the atomic dispatch
reservation plan**. If accepted, implement the private SQLite one-winner slice
before any repeated scheduling or execution-provider broadening.

## 10. Governed Phase Record

- workflow: `dg/d`
- run: `run-1790908801430523000-2`
- approval: `approval/run-1790908801430523000-2/planning-approved`
- presentation: `presentation/603cfe9e60c95b08`
- presentation hash:
  `603cfe9e60c95b088f4d79ad1fc4bacf7878802c6c968fe18b23b052da22a230`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- validation summary: documentation and diff checks passed
- approved boundary: planning and documentation only
- out-of-kernel work: document authoring, validation, git, and future PR work
  are performed by the delegated maintainer; Workflow OS governs the phase but
  does not edit repository files or run shell commands
