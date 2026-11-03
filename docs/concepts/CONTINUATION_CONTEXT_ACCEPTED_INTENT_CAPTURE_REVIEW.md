# Continuation Context Accepted-Intent Capture Review

## Decision

Accepted after blocking replay and provenance corrections.

Phase D may proceed to a separately governed aggregate non-consuming
composition phase. It does not authorize a production accepted-intent backend,
host directive ingestion, raw conversation storage, public CLI or schema
surface, runtime continuation consumer, or execution authority.

## Blocking Finding: Replay Was Not Bound To The Original Capture

The first implementation checked the caller's expected revision before checking
whether the exact record was already committed. A retry using the original
pre-write request therefore failed as stale, while a caller that supplied the
post-write revision could receive replay posture.

That inverted the idempotency boundary. Restart-safe replay must recognize the
exact already-committed operation; it must not require callers to discover and
substitute a newer expected revision.

The correction checks the complete stored record before stale-revision
rejection. An identical record replays against the current committed snapshot.
A different record under the same identity remains a conflict.

## Blocking Finding: Supersession Lineage Was Ephemeral

The first implementation carried `supersedes` only in the capture request. Once
the replacement was stored, Core could not prove whether a later replay supplied
the same supersession set. A caller could omit lineage while presenting the same
replacement record.

The correction makes sorted supersession identities part of
`AcceptedIntentRecord`. They now participate in canonical snapshot and record
commitments, exact equality, debug shape, replay identity, and store validation.
Snapshots reject self-reference, dangling lineage, cross-family lineage, and a
lineage target that is not durably superseded.

## Additional Hardening

`AcceptedIntentCaptureResult` originally derived `Debug`, which could expose a
record identity and revision commitment. It now has a custom redacted debug
implementation, covered by a focused test.

## Accepted Properties

- Accepted intent is attributable to an actor and exact scope.
- The directive is represented by a payload-free provenance commitment.
- Semantic meaning is bounded by the continuation text contract.
- Raw conversation and hidden reasoning are not store fields.
- Store mutation is optimistic-revision fenced and compare-and-replace atomic.
- Exact replay is restart-safe and does not perform another write.
- Identity, directive, scope, actor, meaning, kind, or lineage substitution
  fails closed.
- Singleton objective, priority, and pause/stop/resume replacement requires
  exact same-family supersession.
- The collector is revision fenced and read-only.
- Accepted records cannot be injected through another collector kind or a
  generic source payload.
- Debug output redacts actor, meaning, record identity, and commitments.
- The record and collector remain non-authorizing.

## Remaining Boundaries

- The store is a private contract with an in-memory conformance fixture, not a
  production durable backend.
- No trusted host capture path exists.
- Core cannot yet prove that a proposed intent is already represented by an
  immutable run, typed handoff, accepted plan, or approval presentation.
  Aggregate composition must prefer those sources instead of duplicating them.
- The capture boundary does not authenticate a host identity or infer acceptance
  from conversation.
- No runtime consumer, provider operation, authority use, report write, schema,
  CLI, SDK, or hosted behavior is added.

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793710581116761000-2`
- approval ID: `approval/run-1793710581116761000-2/review-scope-approved`
- presentation ID: `presentation/9294a94af9810849`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- focused continuation-context tests: 73 passed after correction
- strict `workflow-core` clippy with warnings denied: passed with the existing
  narrow dead-code allowance for private future-facing models
- full workspace tests: passed after review correction; `workflow-core` ran 712
  unit tests and all workspace integration and doc tests were green
- warnings-denied rustdoc: passed after review correction
- formatting: passed
- documentation and diff checks: passed after review correction

## Out-Of-Kernel Work

The delegated maintainer reviewed capture identity, attribution, scope,
provenance, replay, supersession lineage, CAS behavior, snapshot validation,
collector drift, redaction, non-authority, tests, roadmap, and implementation
report; corrected the replay and lineage blockers; and authored this review
outside the kernel. The kernel governed scope and approval but did not inspect
files, edit code or documentation, execute tests, create commits, or mutate
GitHub.
