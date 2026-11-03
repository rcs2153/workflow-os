# External Action Transition Lifecycle Review

## Verdict

Accepted after blocker correction.

The lifecycle slice preserves the architecture boundary: immutable projected
scope remains separate from mutable use state; persisted lifecycle data is not
bearer authority; no connector executes; no credential is loaded; and no
provider result is fabricated. The local store now provides a credible durable
foundation for later same-call admission and fixture connector work.

## Findings Corrected

### P0: Wait resolution did not persist the exact resolved condition identity

The initial event shape recorded a wait-resolution commitment and status but
not the exact `AuthorizedExecutionWaitConditionId`. Append-time code checked the
condition ID, but durable rehydration could not prove which active condition had
been resolved.

The event now carries the exact condition ID, binds it into the event
commitment, and requires it to match the active wait during rehydration. A
satisfied wait must restore the exact suspended status; expired or superseded
waits must close as denied.

### P0: Terminal transitions could retain an active wait in memory

The initial rehydration reducer cleared wait state only for `WaitResolved`.
Denial or revocation while waiting produced a terminal status while retaining
the old active wait in the derived snapshot.

Terminal denial, revocation, and consumption now clear active and suspended
wait posture. Durable wait history remains inspectable, but it cannot remain
actionable after terminal closure.

### P1: Interrupted temporary-file cleanup could poison event discovery

The create-only writer links a synchronized temporary file into its final
address. A cleanup failure after the durable link could leave the temporary file
inside the event directory. The reader initially treated every directory entry
as an event and would therefore reject the otherwise valid history.

Readers now ignore only the store's bounded `.tmp-` namespace, while all other
unexpected entries still fail closed. Cleanup is best effort after the durable
link. Retrying the exact append reconciles against the committed event.

### P1: Operation deserialization did not re-enter validation

The initial operation used derived `Deserialize`. Store reads called
`validate()`, but direct deserialization of the public model could produce an
unvalidated value.

Deserialization now denies unknown fields, reconstructs the operation, and
revalidates its deterministic commitment and identity before returning it.

### P1: Atomic reservation was implemented but not directly tested

The store uses the shared local-state writer guard and optimistic cursor, but
the initial focused suite did not prove a competing reservation race.

A concurrent test now launches two different reservations against the same
cursor and proves exactly one succeeds, one fails closed, and only one reserve
event exists.

## Accepted Properties

- operation identity is derived from exact projection and transition
  commitments;
- operation definitions and events use create-only persistence;
- every event is sequence- and commitment-chained;
- exact retry cannot duplicate the immediately preceding event;
- a different command against stale state fails closed;
- known failure may retry under a new attempt;
- ambiguity blocks redispatch until explicit reconciliation;
- current wait binding composes the existing authorized-execution continuity
  vocabulary;
- denial, consumption, and revocation are terminal; and
- no lifecycle record authorizes a connector call.

## Residual Boundaries

- The local filesystem store assumes the existing Workflow OS local-state
  trust boundary. Commitments detect corruption and substitution; they are not
  cryptographic signatures against an administrator who can rewrite all state.
- The lifecycle stores payload-free outcome commitments. A later fixture
  connector phase must bind those commitments to exact bounded observations.
- No trusted clock is introduced here. Expiry and current-authority facts must
  be reassessed in the future same-call Core admission boundary.
- Wait persistence is implemented, but scheduler observation and automatic
  resumption remain explicitly unimplemented.
- Successor PR identity persistence remains a later requirement before the
  full multi-transition fixture proof.

## Validation

- focused lifecycle tests: 6 passed;
- strict all-target, all-feature `workflow-core` Clippy: passed;
- serial all-feature `workflow-core` library suite: 637 passed;
- docs check: passed;
- format and diff checks: passed.

## Governed Review Record

- workflow: `dg/review`
- run: `run-1793682992716377000-2`
- approval: `approval/run-1793682992716377000-2/review-scope-approved`
- approval presentation: `presentation/c8f8ab9acdb3c9c1`
- approval outcome: granted by delegated maintainer
- phase status: completed after blocker correction and validation
