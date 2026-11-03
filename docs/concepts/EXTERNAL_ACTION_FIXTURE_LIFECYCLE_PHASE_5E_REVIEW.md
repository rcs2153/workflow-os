# External Action Fixture Lifecycle Phase 5E Review

Status: accepted after focused maintainer/security review and blocker
corrections.

## Decision

Accept Phase 5E. The local Core-private create-draft to mark-ready to
squash-merge conformance proof is closed, replay-safe, and bounded by fresh
current authority at every durable mutation. This decision does not authorize
live GitHub behavior, credentials, network access, public connector APIs, CLI
mutation, branch deletion, scheduling, broader provider mutations, nested
harnesses, Salsa, release, or publication.

This is owner-conducted repository assurance, not independent external
security review.

## Findings And Corrections

### 1. Draft creation did not validate the exact provider resource

The initial aggregate path fenced provider revision but did not validate the
projected repository, base, and reviewed head before draft mutation.

Corrected:

- draft creation applies exact provider-resource validation before lifecycle or
  connector mutation; and
- changed repository state produces zero provider mutations.

### 2. Aggregate scope and per-transition evidence were insufficiently closed

The initial composition did not prove that the projection contained only the
three accepted transitions or that every transition resolved its own current
authority and source observation.

Corrected:

- the projection must contain exactly create draft, mark ready, and squash
  merge;
- additional or missing transitions reject before source observation;
- create, ready, and merge use separate authority sources, fact sources, and
  evaluation times; and
- every transition outcome carries its distinct payload-free source-anchor
  commitment.

### 3. Aggregate concurrency and replay required stronger proof

Sequential tests did not establish that competing aggregate callers could not
duplicate one or more provider mutations. Terminal replay also depended on
provider receipt lookup without an explicit unavailable-path test.

Corrected:

- synchronized aggregate callers produce one successor and at most one
  mutation for each transition;
- exact replay remains valid after the competing calls;
- unavailable terminal receipt lookup fails closed without redispatch; and
- the implementation report discloses receipt lookup as an availability
  dependency.

### 4. Create-time connector refusal was not a typed resumable wait

Ready and merge had typed wait behavior, but refusal before draft dispatch did
not initially prove equivalent continuation semantics.

Corrected:

- create refusal registers a typed durable capability wait;
- it derives no successor and performs no provider mutation;
- lawful re-entry resumes the exact lifecycle without a second approval; and
- later transitions are not evaluated while create remains waiting.

### 5. Durable wait writes occurred outside fresh current authority

The first correction still initialized, registered, or resolved durable wait
state before the same-call current-authority boundary. Revoked or stale
authority could therefore have written transition state even though connector
mutation remained blocked.

Corrected:

- lifecycle initialization, wait registration, and wait resolution all run
  inside a fresh exact-grant current-authority use;
- draft static validation occurs before lifecycle initialization;
- rejected create, ready, and merge authority writes no lifecycle state for
  the rejected transition; and
- rejected wait resolution preserves the exact waiting cursor and performs no
  provider mutation.

## Accepted Boundaries

- The aggregate admits exactly three reviewed GitHub pull-request transitions.
- Each transition freshly resolves exact current authority and evidence.
- Every durable lifecycle mutation is authority-bound.
- Provider resource and revision drift fail before mutation.
- Typed waits preserve exact transition state and resume without workflow
  restart.
- Completion inspects the exact successor and all three successful lifecycle
  records.
- Exact replay never duplicates provider mutation.
- Receipt lookup unavailability fails closed and leaves durable terminal state
  intact.
- The fixture remains deterministic, credential-free, network-free, and
  non-production.

## Validation

Passed after correction:

- all 57 focused external-action tests;
- all 773 all-feature `workflow-core` library tests;
- strict all-target, all-feature `workflow-core` Clippy;
- `cargo fmt --all -- --check`;
- `npm run check:docs`;
- `git diff --check`; and
- the 11 focused Phase 5E tests.

## Residual Risks

- Successful terminal replay depends on bounded provider receipt lookup.
- Reserve and wait registration remain separate local event-store appends; a
  crash between them is an availability-hardening concern, not an authority or
  duplicate-mutation bypass.
- The in-memory authority and fixture provider prove the contract shape but do
  not provide live platform security or operational assurance.
- Independent external security review remains outstanding.

## Next Authorized Increment

Phase 5E acceptance closes the local external-action fixture lifecycle lane. It
does not authorize a live connector. The next increment must be selected by the
roadmap's authoritative Active Delivery Queue and must not be inferred from
this review record.
