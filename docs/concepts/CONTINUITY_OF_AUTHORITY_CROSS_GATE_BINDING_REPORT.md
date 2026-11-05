# Continuity Of Authority Cross-Gate Binding Report

## Status

Implemented as a private Core model and store-backed composition boundary.
Focused maintainer/security review is accepted after correcting event-only
dispatch inference, cross-run scope composition, and unbound lawful-next
operation identity.

## Governed Implementation

- workflow: `dg/implement`;
- run: `run-1793891412896723000-2`;
- approval: `approval/run-1793891412896723000-2/implementation-approved`;
- approval presentation: `presentation/625fc18b6bb845c6`;
- presentation content hash:
  `625fc18b6bb845c6c4667beb58ee565f08f59899155b5581fa5d85cf7ab9951e`;
- decision: granted under delegated-maintainer authority after the complete
  handoff was presented; and
- state root: `/private/tmp/workflow-os-cross-gate-implementation-state`.

Repository edits and validation ran outside the kernel. The kernel coordinated
the bounded phase and approval; it did not execute or fabricate implementation
or test evidence.

Phase close reported `Completed` with 39 events, one approval, zero retries,
and zero escalations. The event trail contains the matching persisted approval-
presentation proof marker.

## Implemented Boundary

Core now defines one payload-free, non-authorizing cross-gate lifecycle binding
for replacement-executor fresh-authority resolution. It commits:

- one exact run identity;
- one immutable resolved execution-context commitment;
- the canonical set of accepted dispatch projection commitments;
- one exact WorkReport artifact-location and retained body commitment;
- one current project-resource restriction commitment; and
- one revision fence over the complete observation.

The store-backed source reconstructs current facts through existing read-only
contracts. It:

1. reads the durable run snapshot;
2. reads the exact immutable run bundle and requires its run binding and
   workflow identity to match the snapshot;
3. requires a dedicated current accepted-dispatch source whose implementation
   must prove the complete Gate 1 durable posture, including queue/work-item
   evidence rather than events alone;
4. requires a dedicated current lawful-next-operation source;
5. reads the exact WorkReport through
   `WorkReportArtifactLocationExpectation`, requires an envelope-anchored body
   commitment, and commits the report/run/workflow/location identity;
6. obtains current project-resource authority through a dedicated read-only
   source contract that must reconstruct current facts rather than return
   approval-time cache state; and
7. rereads the complete source revision after composition so drift fails
   closed.

The binding is private to continuation collection. It exposes neither payloads
nor authority.

## Replacement-Executor Enforcement

The fresh-authority request now commits the exact cross-gate lifecycle binding.
The cross-gate replacement proof requires original and replacement executors to
receive the same lifecycle binding in addition to the same continuation
understanding. It also requires the lifecycle run to equal the run-bound
continuation scope and the lifecycle lawful-next operation to equal the exact
operation in the fresh-authority request.

The older private resume proof can no longer resolve fresh authority without
that binding. It fails with
`continuation_collection.replacement_executor.lifecycle_binding.missing`
before invoking the authority resolver. This prevents a future internal caller
from accidentally treating the older conformance-only path as authorization.

Pause, stop, blocked orientation, and already-completed work remain
non-authorizing.

## Regression Matrix

Focused tests prove:

- unchanged exact dispatch/report/resource binding reaches the separately
  injected authority resolver once;
- dispatch substitution fails before authority resolution;
- report artifact-location substitution fails before authority resolution;
- project-resource substitution fails before authority resolution;
- another run scope fails before authority resolution;
- another lawful-next operation fails before authority resolution;
- run and immutable-context mismatch fail closed;
- missing or duplicate dispatch evidence fails closed;
- dispatch ordering is canonical and commitment-stable;
- source revision drift fails closed;
- context-free resume without a cross-gate binding fails closed;
- pause and stop never resolve authority; and
- a lawful-next identity already recorded as completed cannot be revived.

## Validation

Passed:

- `cargo test -p workflow-core --lib continuation_context_collection`:
  60 tests after review corrections;
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings`;
- workspace formatting during implementation; and
- the earlier review-phase documentation and diff checks.

The accepted focused review is recorded in [Continuity Of Authority Cross-Gate
Binding Review](CONTINUITY_OF_AUTHORITY_CROSS_GATE_BINDING_REVIEW.md).

## Explicit Non-Claims

This increment does not add:

- a production replacement-executor or continuation runtime caller;
- provider writes or new provider families;
- network transport, credential resolution, or live-provider evidence;
- schemas, SDK, CLI, or hosted API exposure;
- hosted administration or enterprise identity;
- a public authority token or payload; or
- release admission or a broader unattended-autonomy claim.

## Next Required Phase

Submit the reviewed boundary through exact-head pull-request validation. After
merge, product-contract, hosted, integration, and release-admission
reconciliation may proceed without claiming a production replacement executor
or concrete cross-gate source activation.
