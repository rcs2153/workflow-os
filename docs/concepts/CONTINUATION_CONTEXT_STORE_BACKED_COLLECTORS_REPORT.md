# Continuation Context Store-Backed Collectors Report

## Status

Phase B implemented as a Core-private, read-only collector layer over existing
stable local read interfaces. Focused maintainer/security review is required
before bounded Git or operating-document observers may begin.

## Delivered

Core now has private store-backed collectors for the twelve kernel source
classes authorized by the active delivery queue:

- validated declarations;
- immutable run bundle;
- durable run state;
- event history;
- policy;
- approval;
- capability;
- required context;
- evidence;
- check;
- `SideEffect`; and
- `WorkReport`.

The implementation composes the existing `EventLogStore`, `RunSnapshotStore`,
`ApprovalPresentationRecordStore`, `SideEffectRecordStore`,
`WorkReportArtifactStore`, and `ImmutableRunBundleStore` read contracts. It
does not add another persistence abstraction.

Stable observations are implemented for declarations, immutable bundles,
durable state, event history, policies, approvals, checks, side effects, and
report artifacts. Capability, required-context, and evidence collection remain
typed `Unsupported` because their current interfaces do not provide a complete,
non-consuming, revision-fenced read contract. A queryable store is not treated
as authoritative merely because data can be read from it.

## Stable Read Boundary

Every supported collector binds its result to a source-specific revision
commitment. Run snapshots are reconciled with the append-only event cursor, and
immutable declarations and policies are read from the exact immutable run
bundle. Approval observations bind the snapshot and approval-presentation
records. Check observations bind immutable declarations and typed result
events. `SideEffect` and `WorkReport` observations bind their complete local
record sets.

The collector set captures pre-collection anchors and verifies them after
collection. Drift in the immutable bundle, run snapshot, event cursor, or other
source-specific revision invalidates the observation set rather than combining
facts from incompatible moments.

## Security Properties

- The implementation remains crate-private and has no runtime consumer.
- All reads use existing store interfaces and are non-consuming.
- Collection cannot grant or consume approval, capability, policy permission,
  required context, or other execution authority.
- Core continues to own collector kind, source class, authority domain,
  purpose profile, semantic obligations, sensitivity, and budget posture.
- Missing stable read semantics remain typed `Unsupported`; they are not
  relabeled as empty or unavailable.
- Store failures become stable non-leaking failed posture.
- Source identifiers and revision commitments are payload-free.
- Bounded claims are derived only from existing typed records.
- No raw provider payload, credential, environment value, arbitrary repository
  content, conversation history, or hidden reasoning is collected.
- Reads do not append events or create side-effect/report records.

## Focused Coverage

Five focused tests prove:

- every Phase B source class produces its intended supported or explicitly
  unsupported posture;
- event-cursor drift rejects the complete collection; and
- durable-snapshot drift rejects collection against the captured anchor;
- unchanged store revisions produce the same complete anchor after backend
  restart; and
- repeated collection does not consume or mutate event, `SideEffect`, or
  `WorkReport` state.

## Governed Implementation

- workflow: `dg/implement`
- run ID: `run-1793705834490041000-2`
- approval ID:
  `approval/run-1793705834490041000-2/implementation-approved`
- presentation ID: `presentation/8fd196dfc17b6fb9`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- focused store-backed collector tests: 5 passed
- strict `workflow-core` clippy with warnings denied: passed with the existing
  narrow dead-code allowance for private future-facing models
- full workspace tests: passed
- rustdoc with warnings denied: passed
- formatting: passed
- documentation checks: passed
- diff and whitespace checks: passed

## Boundary Preserved

This phase did not add a new store, collect Git or operating documents, capture
accepted intent, compose an aggregate continuation projection, expose CLI,
schema, SDK, report, hook, or child-harness behavior, consume authority, invoke
a runtime continuation consumer, perform provider work, add nested harness
behavior, add Salsa, or expand hosted behavior.

## Out-Of-Kernel Work

The delegated maintainer implemented the private collectors, fixtures, focused
tests, roadmap update, and this report; ran validation; and reviewed diagnostics
outside the kernel. The kernel governed scope and approval but did not edit
files, execute tests, create commits, or mutate GitHub.
