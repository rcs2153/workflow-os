# External Action Connector Same-Call Admission Review

## Verdict

Accepted after blocker correction.

The dark connector-admission slice preserves the intended architecture. A
serialized authorization projection does not become bearer authority. The
fixture connector can run only inside the registered current-authority same-
call boundary, after exact approval-presentation, binding, resource, evidence,
window, revocation, policy, and lifecycle checks. The connector receives one
borrowed, non-serializable capability and no credential or provider payload.

The implementation remains fixture-only and does not claim that every injected
fact is already reloaded from an authoritative production source.

## Findings Corrected

### P0: Known-failure retry reused one local attempt identity

The initial wrapper derived `AuthorizedExecutionAttemptId` only from the stable
operation identity. A known provider failure could therefore reserve another
attempt with the same local attempt ID. That weakened audit distinction between
provider calls and could complicate later reconciliation.

Attempt identity now binds the immutable operation to the exact lifecycle
sequence and head commitment from which reservation begins. The provider
operation/idempotency identity remains stable, while each local retry gets a
distinct attempt identity. Focused coverage proves a known failure can retry to
success with two distinct attempts.

### P0: The admission wrapper incorrectly treated known failure as terminal

The accepted lifecycle permits retry from `OutcomeFailed` and
`ReconciledFailed`. The initial admission wrapper returned `ExistingTerminal`
for those states, silently making that lifecycle branch unreachable.

Known failures now proceed through a new reservation. Success, denial,
revocation, and consumption remain terminal. Ambiguous, attempted, admitted,
reserved, and waiting posture remain recovery-required and cannot redispatch.

### P1: Current-fact naming overstated the fixture boundary

Only capability authority is freshly resolved through the registered current-
authority source in this slice. Approval, policy, execution-window, revocation,
resource, and evidence facts are injected fixture values that Core validates
exactly against the immutable projection.

The input is now named `ExternalActionConnectorFixtureFacts`, and the report
states the limitation directly. A later end-to-end fixture phase must compose
authoritative sources for these facts before any production connector claim.

## Accepted Properties

- exact workflow, run, actor, immutable bundle, operation, resource, and
  projected transition bindings are required;
- the projected capability grant is freshly resolved from the complete current
  grant and availability inventories;
- the selected current grant must be the exact projected grant;
- revoked, expired, missing, unavailable, or scope-mismatched authority never
  reaches the connector;
- approval decisions require exact persisted presentation proof and strict
  non-goal binding;
- evidence predicates must exactly match the canonical projection and remain
  fresh;
- lifecycle reservation, admission, attempted posture, and outcome observation
  surround connector entry;
- terminal replay does not invoke the connector twice;
- ambiguous outcome blocks blind redispatch; and
- the private use capability is borrowed, non-cloneable, non-serializable, and
  payload-free.

## Residual Boundaries

- Fixture facts other than capability authority are trusted inputs to this
  crate-private proof. This is not yet a production source-reload boundary.
- The local lifecycle trust model detects corruption and substitution through
  commitments but is not a signature system against an administrator who can
  rewrite all local state.
- No platform credential, provider permission, network transport, live GitHub
  call, successor pull-request binding, or reconciliation lookup exists here.
- Known failure permits another separately identified attempt; connector-side
  operation idempotency and live reconciliation remain later responsibilities.
- No SideEffect, WorkReport, public telemetry, scheduler, or automatic resume
  path is claimed.
- This owner-conducted focused review is not independent external security
  assurance.

## Validation

- focused all-feature admission tests: 6 passed;
- full all-feature `workflow-core` library suite: 738 passed after review
  correction;
- strict all-target, all-feature `workflow-core` Clippy: passed after review
  correction;
- `cargo fmt --all -- --check`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## Governed Review Record

- workflow: `dg/review`
- run: `run-1793726838376149000-2`
- approval: `approval/run-1793726838376149000-2/review-scope-approved`
- approval presentation: `presentation/2d215d72804fdca6`
- approval outcome: granted by delegated maintainer
- phase status: accepted after blocker correction and final validation
