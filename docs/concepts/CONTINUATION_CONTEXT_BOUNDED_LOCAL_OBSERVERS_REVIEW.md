# Continuation Context Bounded Local Observers Review

## Decision

Accepted after one blocking correction.

Phase C remains a private injected observation contract. It may proceed to the
separately governed durable accepted-intent capture phase. It does not authorize
a production Git or filesystem observer, host registration, aggregate
continuation composition, runtime consumption, or any public surface.

## Blocking Finding: Observer Producer Was Not Bound

The first implementation bound continuation purpose, scope, actor, and observed
values, but it did not bind the injected observer implementation or
configuration across anchor capture and collection. A future host could
therefore substitute its observation method while retaining the same apparent
source values and request identity.

The correction adds one payload-free observer contract commitment to the local
observation request. The request commitment now covers purpose, scope, actor,
and observer contract. Both anchor capture and collection derive the request
from the currently injected observer, so contract substitution fails before an
observation can satisfy the anchored collection.

This commitment is a continuity binding, not producer authentication or
execution authority. A future production host observer still requires its own
reviewed registration, implementation/configuration identity, and assurance
boundary.

## Accepted Properties

- Core owns the exact three-source Phase C batch.
- Git/worktree observations contain bounded typed metadata and payload-free
  commitments only.
- Operating-document identity is a closed enum for exactly `AGENTS.md`,
  `docs/ENGINEERING_STANDARD.md`, and `ROADMAP.md`.
- Core validates request binding, source revision, observer-contract continuity,
  count bounds, path-safety attestations, symlink posture, per-file and aggregate
  size, sensitivity, deterministic ordering, and debug redaction.
- Revision drift rejects collection rather than mixing observations.
- Empty results preserve complete typed negative knowledge.
- Typed handoff remains explicitly `Unsupported` because no stable store
  contract exists.
- The implementation is private and has no runtime consumer.
- No local process, filesystem, Git, provider, state, report, or authority
  mutation occurs.

## Remaining Boundaries

- Path and symlink safety are producer attestations validated as a closed shape;
  this phase does not independently read the filesystem to prove them.
- The source set is a reference contract, not a production local observer.
- The observer contract commitment does not establish producer trust.
- Operating documents emit inventory/reference posture only and cannot grant
  permission or satisfy governed checks, evidence, approval, or capability.
- A host producer, typed-handoff store, accepted-intent store, aggregate
  projection, public API, and runtime consumer remain deferred.

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793708389845728000-2`
- approval ID:
  `approval/run-1793708389845728000-2/review-scope-approved`
- presentation ID: `presentation/6d2f1fc8b95f7f36`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- focused bounded-local-observer tests: 6 passed after correction
- strict `workflow-core` clippy with warnings denied: passed with the existing
  narrow dead-code allowance for private future-facing models
- full workspace tests: passed
- rustdoc with warnings denied: passed
- formatting, documentation, and diff checks: passed

## Out-Of-Kernel Work

The delegated maintainer reviewed the model, trust boundary, revision fencing,
producer continuity, request binding, negative knowledge, redaction, bounds,
tests, roadmap, and implementation report; corrected the producer-substitution
blocker; and authored this review outside the kernel. The kernel governed scope
and approval but did not inspect files, edit code or documentation, execute
tests, create commits, or mutate GitHub.
