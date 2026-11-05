# Continuity Of Authority Cross-Gate Binding Review

## Verdict

Accepted after correcting three blocking findings.

The private Core boundary now prevents a replacement executor from resolving
fresh authority unless one exact, revision-fenced lifecycle subject binds the
current accepted dispatch posture, verified WorkReport identity/location,
current project-resource restriction, run-bound continuation scope, and lawful-
next operation.

This is an owner-conducted focused maintainer/security review backed by
repository evidence. It is not independent external security assurance and it
does not accept a production replacement-executor runtime.

## Governed Review

- workflow: `dg/review`;
- run: `run-1793892114577543000-2`;
- approval: `approval/run-1793892114577543000-2/review-scope-approved`;
- approval presentation: `presentation/d8df25d4d17aae1e`;
- presentation content hash:
  `d8df25d4d17aae1efcab326b80579121e61b31d1ac2ffe7d88525013f87efbbf`;
- decision: granted under delegated-maintainer authority after the complete
  handoff was presented; and
- state root: `/private/tmp/workflow-os-cross-gate-binding-review-state`.

The phase closed `Completed` with 39 events, one approval, zero retries, and
zero escalations. The event trail contains the approval request and grant,
eight policy decisions, six scheduled and successful skill invocations, and
one terminal completion. Approval presentation proof was enforced and its
event marker is present.

Repository inspection, corrections, and validation occurred outside the
kernel. The kernel coordinated the bounded review phase and approval; it did
not execute or fabricate review evidence. Repo edits, shell commands, focused
tests, static checks, documentation checks, git operations, and the review
report were performed by the delegated maintainer outside the kernel. No live
provider check was required or claimed.

## Blocking Findings And Corrections

### 1. Dispatch Events Were Not Gate 1 Acceptance

The first store-backed candidate derived accepted dispatch commitments from
`AuthorizedExecutionAttemptDispatchAdmitted` events. Gate 1 explicitly proved
that a nonempty event stream is not sufficient: accepted hosted dispatch also
requires the exact durable queue/work-item posture.

The correction removes event-only inference. Cross-gate composition now
requires a dedicated `CurrentAcceptedDispatchSource`. Its implementation must
derive accepted disposition from complete current hosted state, including
queue/work-item evidence, and return its own revision fence. No production
source implementation is added in this phase.

### 2. The Lifecycle Run Was Not Joined To Continuation Scope

The first candidate committed the lifecycle binding and continuation scope in
the same fresh-authority request but did not require their run IDs to be equal.
A caller could therefore combine an aggregate for one run with a valid cross-
gate subject for another run.

The correction requires both original and replacement continuation scopes to
be run-bound and to identify the exact lifecycle run. Another-run substitution
fails with
`continuation_collection.replacement_executor.lifecycle_binding.scope_mismatch`
before authority resolution.

### 3. The Lawful-Next Operation Was Not Part Of The Lifecycle Subject

The first candidate bound run, dispatch, report, and resource facts but did not
bind the exact operation selected by current accepted intent. Same-run facts
could therefore accompany another lawful-next operation.

The correction adds a dedicated current lawful-next operation source, includes
its identity and revision in the lifecycle commitment, and requires equality
with the operation in the fresh-authority request. Substitution fails with
`continuation_collection.replacement_executor.lifecycle_binding.operation_mismatch`
before authority resolution.

## Accepted Security Properties

The corrected private model now establishes:

- no fresh replacement authority without a cross-gate lifecycle binding;
- no cross-run lifecycle binding;
- no different lawful-next operation;
- no dispatch, report, resource, immutable-context, or revision substitution;
- no event-only inference of Gate 1 acceptance;
- no missing or duplicate dispatch evidence;
- canonical order-independent dispatch commitment;
- no authority resolution for pause, stop, blocked, or already-completed work;
- exact continuation delivery and acknowledgment remain non-authorizing; and
- no authority payload is returned by lifecycle composition.

## Residual Boundaries

- There is no production registry or replacement-executor runtime caller.
- No concrete current accepted-dispatch, lawful-next-operation, or project-
  resource source is activated by this increment.
- A future Gate 1 source implementation must compose the exact durable hosted
  disposition, not infer it from events.
- A future accepted-intent source implementation must prove one current lawful-
  next operation under the existing supersession contract.
- Live credentials, provider writes, network transport, schemas, SDKs, CLI,
  hosted API exposure, and release admission are unchanged.
- This review is not independent external security assurance.

## Validation

Passed after review corrections:

- focused continuation collection tests: 60 passed;
- strict `workflow-core` all-target/all-feature Clippy;
- workspace formatting;
- documentation and release-state checks; and
- diff hygiene.

The final validation commands are rerun after this review document and roadmap
update. No opt-in live-provider test is required or claimed for this private
model boundary.

## Next Authorized Phase

Commit and submit this corrected private boundary through exact-head pull-
request validation. After merge, reconcile the product contract, hosted and
integration claims, and release-admission posture without claiming a production
replacement executor or concrete cross-gate source activation.
