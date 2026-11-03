# External Action Fixture Lifecycle Phase 5E Report

Status: implemented and accepted after focused maintainer/security review and
blocker correction.

## Scope

This phase composes the accepted local fixture boundaries into one
Core-private create-draft to mark-ready to squash-merge conformance entrypoint.
It remains deterministic, credential-free, network-free, fixture-backed, and
non-production.

## Implementation

### Closed lifecycle admission

The conformance entrypoint accepts only an immutable authorization projection
whose transition set contains exactly:

- `GitHubPullRequestCreateDraft`;
- `GitHubPullRequestMarkReadyAfterTerminalGreen`; and
- `GitHubPullRequestSquashMergeUnchangedHead`.

Missing or additional transitions reject before source observation or provider
entry. The aggregate cannot silently widen into ref updates or branch
deletion.

### Per-transition current authority and evidence

Create, ready, and merge each receive a distinct transition context containing
their current-authority source, fixture fact source, and evaluation time. The
entrypoint invokes the existing same-call authority and connector-admission
boundary independently for every transition. A revoked later authority or
wrong later evidence therefore stops after only the earlier lawful provider
operations.

Each transition output carries the payload-free source-anchor commitment that
bound complete source coverage to the provider revision admitted for that
transition. The complete proof demonstrates that create, ready, and merge use
distinct transition-bound anchors rather than reusing one aggregate snapshot.

### Pre-mutation resource integrity

Focused review of the initial implementation found that draft creation fenced
provider revision but did not validate the exact projected repository, base,
and reviewed head before connector admission. The draft path now applies the
same exact provider-resource validation required by ready and merge. A changed
head before create produces zero provider mutations.

### Typed waits and lawful re-entry

Create, ready, and merge waits remain first-class conformance outcomes.
Connector refusal before draft creation registers a durable capability wait
without deriving a successor or entering the provider. Pending checks and
later connector refusal likewise remain typed outcomes instead of generic
non-terminal errors. The outcome identifies the waiting transition and reason,
preserves the durable wait record and exact successor when one exists, and does
not evaluate later transition sources. Re-entry reruns fresh authority and
evidence composition and resumes the same lifecycle without manual state
repair or a second authorization.

Every durable wait lifecycle write, including initialization, registration,
and resolution, executes inside a fresh exact-grant current-authority use. A
revoked or otherwise rejected authority therefore cannot create transition
state, register a wait, or advance a waiting cursor. Read-only inspection of an
already waiting transition remains non-mutating.

### Durable completion and replay

Successful completion rereads and validates the exact Core-owned successor,
requires terminal ready and exact merge outcomes, and inspects all three
durable lifecycle records. The payload-free inspection binds transition kind,
terminal status, event count, operation-definition commitment, and final event
commitment.

Exact replay re-evaluates current transition contexts, resolves the existing
provider receipts, returns the same lifecycle records, and does not redispatch
any provider mutation. Receipt lookup remains an explicit availability
dependency: if provider lookup is unavailable, replay fails closed while the
durable terminal lifecycle and exact successor remain intact and no mutation
is redispatched.

## Focused Tests

The Phase 5E tests prove:

1. one complete three-transition lifecycle performs exactly three fixture
   provider mutations;
2. exact replay performs no duplicate mutation and returns identical durable
   lifecycle inspection;
3. each transition observes its own source and carries a distinct source-anchor
   commitment;
4. wrong ready evidence stops after draft creation and never observes merge
   evidence;
5. a changed provider resource before draft creation causes zero mutation;
6. an extra projected transition rejects before source observation or provider
   entry;
7. pending checks return typed ready-wait posture and later resume to terminal
   completion without workflow restart; and
8. a revoked ready authority is freshly resolved and stops after only the
   lawful draft mutation;
9. create-time connector refusal creates a typed durable wait and resumes to
   completion without a second approval;
10. competing aggregate callers produce one successor and at most one mutation
    per transition, after which exact replay remains valid; and
11. unavailable terminal receipt lookup fails closed without redispatch;
12. rejected create, ready, and merge wait authority writes no lifecycle state
    for the rejected transition; and
13. rejected wait-resolution authority preserves the exact waiting cursor and
    performs no provider mutation.

The accepted Phase 5A through 5D tests continue to prove source coverage,
revision fencing, successor persistence and repair, wait proof, exact ready and
merge replay, ambiguity reconciliation, and authoritative negative lookup.

## Validation

Passed before focused review:

- all 57 focused external-action tests;
- all 773 all-feature `workflow-core` library tests;
- strict all-target, all-feature `workflow-core` Clippy;
- `cargo fmt --all -- --check`; and
- the 11 focused Phase 5E tests.

Documentation validation and `git diff --check` passed before final review.

## Governed Phase Record

Implementation:

- dogfood workflow: `dg/implement`;
- run: `run-1793739364078388000-2`;
- approval: `approval/run-1793739364078388000-2/implementation-approved`;
- approval presentation: `presentation/3ab7fa64a66d9b80`;
- approval outcome: granted under standing delegated-maintainer authority with
  persisted presentation proof; and
- retries and escalations: none after the accepted governed start.

Two rejected pre-run attempts created no governed run: one contained a
secret-like term and one exceeded the bounded one-line scope contract. They
were corrected through the runner rather than by modifying state.

Repository edits, shell validation, Git operations, and later PR actions are
performed by the delegated maintainer outside the kernel. The runner governs
the approved scope and records its decision; it does not silently perform
those actions. No missing handler, check, report, or provider coverage was
simulated.

## Limits

This phase does not add live GitHub behavior, credentials, network access,
public connector APIs, schema or SDK exposure, CLI mutation, branch deletion,
background polling, scheduling, broader provider mutation, nested harnesses,
Salsa, release, or publication.

The provider fixture is authoritative only inside this local conformance proof.
Terminal replay intentionally depends on bounded provider receipt lookup; an
unavailable lookup blocks replay without changing durable terminal state or
permitting redispatch.
The owner-conducted focused review is repository assurance, not independent
external security review.
