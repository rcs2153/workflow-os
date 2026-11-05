# Continuity Of Authority Cross-Gate Review

## Verdict

Blocked at the integration boundary.

Continuity-of-authority Gates 1, 2, and 3 remain closed for their documented
individual boundaries. This review found no regression in hosted dispatch
truth, WorkReport artifact identity validation, or GitHub repository-resource
authorization continuity.

The repository does not yet prove the stronger cross-gate invariant required
by the delivery plan: a context-free replacement executor cannot obtain fresh
authority while inheriting dispatch success, completed-work/report identity,
or project-resource authority from different lifecycle bindings.

## Governed Review

- workflow: `dg/review`;
- run: `run-1793890863234808000-2`;
- approval: `approval/run-1793890863234808000-2/review-scope-approved`;
- approval presentation: `presentation/fb8b160934c9e8a4`;
- presentation content hash:
  `fb8b160934c9e8a4af3ca63198a769a3819b613aba47286c04403ad2c25c90a4`;
- decision: granted under delegated-maintainer authority after the complete
  handoff was presented; and
- state root: `/private/tmp/workflow-os-cross-gate-review-state`.

Repository inspection and test execution occurred outside the kernel. The
kernel coordinated the bounded review phase and approval; it did not execute
or fabricate the review evidence.

Phase close reported `Completed` with 39 events, one approval, zero retries,
and zero escalations. The event trail contains the matching persisted approval-
presentation proof marker.

## Evidence Reviewed

The review inspected:

- Gate 1 hosted retry reconciliation and accepted-dispatch evidence;
- Gate 2 `WorkReportArtifactLocationExpectation` validation across durable and
  injected stores;
- Gate 3 resolved execution-context and current project-resource
  reassessment;
- continuation-context store-backed WorkReport collection;
- exact projection delivery and acknowledgment;
- replacement-executor orientation and fresh-authority request construction;
  and
- the existing private replacement-executor regression matrix.

Focused continuation collection validation passed 53 tests. The replacement
matrix correctly rejects aggregate substitution, operation substitution,
recipient reuse, repeated completed work, stale sources, and mismatched
delivery acknowledgments.

## Blocking Finding

The current replacement-executor proof is intentionally private,
non-authorizing model behavior. Its lawful-next and completed-work identities
come from fixture collectors. Its fresh-authority request binds the aggregate
scope, aggregate proof, decision, recipient, and one operation identity, then
delegates resolution to an injected resolver that returns no authority payload.

That proof does not currently bind or require all of the following as one
immutable lifecycle subject:

- the Gate 1 accepted hosted disposition and exact dispatch identity;
- the Gate 2 exact WorkReport artifact location, report identity, workflow/run
  identity, and retained integrity expectation; and
- the Gate 3 current project-resource restriction and resolved execution-
  context commitment.

The store-backed WorkReport collector validates artifacts using the strongest
run identity available, but projects only that a report artifact exists. It
does not project the exact artifact-location expectation into the replacement
fresh-authority request. The replacement proof has no production registry or
runtime caller, and no integrated regression substitutes one gate's lifecycle
binding while holding the other two constant.

Passing the three gate suites independently therefore cannot establish the
cross-gate claim. Treating adjacency as composition would allow a future
runtime caller to accidentally resolve fresh authority over a mixed lifecycle.

## Required Correction

Before product-contract or release-admission reconciliation, implement one
Core-owned, payload-free cross-gate lifecycle binding for replacement-executor
fresh-authority resolution. The bounded increment must:

1. derive the subject from authoritative current stores rather than fixture
   prose or caller-supplied success claims;
2. bind the exact run and immutable execution context;
3. bind the accepted hosted disposition and dispatch identity when hosted
   continuation is claimed;
4. bind exact completed WorkReport identity and location expectation when the
   report supplies completed-work truth;
5. bind current project-resource restriction and resolved execution context
   when provider work remains lawful;
6. reject any substituted, absent, stale, or cross-run component before fresh
   authority resolution;
7. keep orientation, delivery receipts, and the lifecycle-binding candidate
   non-authorizing; and
8. add an integrated regression matrix with each single-component
   substitution plus unchanged control, replay, pause, stop, and already-
   completed cases.

This correction must not add provider writes, network transport, new provider
families, schemas, SDK exposure, hosted administration, or a public continuation
API.

## Validation

Passed during this review:

- `cargo test -p workflow-core --all-features continuation_context_collection`
  (53 matching unit tests passed; integration targets contained no matching
  tests); and
- source and documentation inspection of all three accepted gate boundaries.

Documentation and diff checks are required after this report and roadmap status
are written.

## Next Authorized Phase

Implement the narrow cross-gate lifecycle-binding model and integrated
regressions described above. Do not begin product-contract, hosted,
integration, release-admission, broader provider mutation, or comparative
framework benchmark work until the correction passes focused review.
