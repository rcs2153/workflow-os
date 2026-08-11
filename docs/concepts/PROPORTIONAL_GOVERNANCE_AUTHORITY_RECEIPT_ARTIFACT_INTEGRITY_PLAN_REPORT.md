# Proportional-Governance Authority-Receipt Artifact Integrity Plan Report

## 1. Executive Summary

The next durability boundary is now planned. The plan separates trusted
in-memory receipt issuance from durable local receipt records and defines a
later explicit report-artifact referential-integrity helper.

No runtime or persistence implementation was added.

## 2. Scope Completed

- Audited the accepted decision-time receipt, citation, WorkReport,
  executor-propagation, artifact-store, and existing integrity boundaries.
- Defined the durable receipt trust distinction.
- Defined candidate store, create-only, exact-idempotent, and conflict rules.
- Defined strict, fail-closed artifact-integrity semantics.
- Defined privacy, error, ordering, and future test requirements.
- Sequenced the first implementation as model/store contract only.

## 3. Scope Explicitly Not Completed

The phase added no Rust model, store, persistence, artifact write, executor
integration, event, audit projection, schema, SDK, CLI/UI behavior, provider,
OpenShell change, SideEffect execution, hosted behavior, or release change.

## 4. Key Architecture Decision

Persistence must not restore receipt trust. The existing trusted receipt is
serialize-only by design. A future store may accept that trusted value, but a
read must return an explicitly persisted and structurally verified record that
remains local, unsigned, point-in-time evidence only.

Artifact integrity may prove exact reference and immutable run linkage. It may
not claim current authority, issuer authentication, or permission to execute a
later operation.

## 5. Recommended First Implementation

Add only:

- one persisted receipt-record model;
- one transport-neutral receipt-record store contract; and
- one in-memory test implementation.

Review that boundary before local filesystem persistence or artifact integrity
is implemented.

## 6. Validation

Validation completed successfully:

- `npm run check:docs`: passed
- `git diff --check`: passed

## 7. Remaining Limitations

- Receipts remain in-memory only.
- Durable report artifacts cannot resolve receipt citations.
- No authenticated receipt envelope exists.
- No automatic report or artifact path is changed.
- The local filesystem is not an authenticated issuer boundary.

## 8. Recommended Next Phase

Implement the persisted receipt-record core model and transport-neutral store
contract only, then perform a focused maintainer review.

## 9. Governed Planning Record

- Dogfood workflow: `dg/d`
- Run ID: `run-1786425894422721000-2`
- Approval ID: `approval/run-1786425894422721000-2/planning-approved`
- Presentation ID: `presentation/42f9a82a0dc5e540`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: architecture inspection and documentation authoring
