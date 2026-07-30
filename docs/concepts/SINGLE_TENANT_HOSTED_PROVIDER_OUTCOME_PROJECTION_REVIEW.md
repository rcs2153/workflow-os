# Single-Tenant Hosted Provider Outcome Projection Review

Review date: 2026-07-29

## 1. Executive Verdict

**Provider-failure and reconciliation projection accepted; proceed to the
hosted deployment/recovery proof.**

The implementation closes the remaining authoritative outcome gap for the
current no-write hosted provider. Known pre-start rejection becomes atomic
workflow failure. Provider uncertainty becomes atomic reconciliation
escalation. Neither path leaves a falsely running workflow, performs a blind
retry, or fabricates provider evidence.

## 2. Scope Verification

The phase stayed within the approved hosted provider-outcome boundary. It did
not add provider writes, another provider, OpenShell, credentials,
access-material resolution, multi-tenancy, enterprise identity, hosted UI,
schema changes, automatic retries, or production claims.

## 3. Outcome Model Assessment

`HostedUnreceiptedOutcome` distinguishes the only two authoritative states
available without a valid receipt:

- `RejectedBeforeStart`, where failure is known and no invocation attempt may
  exist;
- `ReconciliationRequired`, where provider execution may have started and an
  invoking attempt must exist.

`HostedUnreceiptedResultProjection` produces fixed, payload-free workflow
events from those states. Exactly bound ambiguous receipts follow the same
escalation semantics while retaining their exact provider receipt.

The model is narrow and domain-neutral enough for another future provider
without exposing provider-native errors or payloads.

## 4. Workflow Semantics Assessment

Pre-start rejection appends `SkillInvocationFailed` followed by `RunFailed`.
The projected run is terminal `Failed`.

Provider uncertainty appends `EscalationTriggered`. The projected run is
`Escalated`, which preserves the need for operator reconciliation instead of
claiming terminal failure or success.

The implementation does not change successful, failed-with-receipt, or
canceled receipt semantics. It changes only the prior incorrect treatment of
an ambiguous receipt as ordinary run failure.

The worker maps each receipt status to the corresponding durable work-item
status. In particular, an ambiguous receipt cannot be mislabeled completed
while its run is escalated.

## 5. PostgreSQL Atomicity Assessment

The new transaction validates binding before storage and then atomically
commits:

- exact Core-generated events;
- rehydrated run snapshot;
- failed or ambiguous work item;
- reconciliation-required attempt when applicable;
- fenced lease release.

The pre-start path rejects any existing attempt. The reconciliation path
requires the exact invoking attempt revision. Exact replay validates work
item, events, rehydrated run, and attempt posture. Conflicting replay fails
closed.

This corrects the prior worker behavior where attempt reconciliation and work
item ambiguity were separate durable operations and the run stayed `Running`.

## 6. Worker Integration Assessment

The worker now checks attempt posture before changing durable state. A
provider validation error proved `NotStarted` uses the atomic failure
projection. An invocation error after the durable invoking posture uses the
atomic reconciliation projection and returns a bounded
`ReconciliationRequired` worker outcome.

Provider error details are deliberately discarded at the governance boundary;
fixed platform-owned reconciliation records avoid leakage and do not pretend
to classify an uncertain remote outcome more precisely than the evidence
allows.

## 7. Privacy And Redaction Assessment

The projection stores no raw provider error, command output, source content,
path, credential, token, environment value, or access material. Event
identities and idempotency keys are derived with bounded hashing. Debug output
does not expose work-item, invocation, or provider identity.

Errors use stable platform-owned codes and summaries.

## 8. Test Quality Assessment

Focused model tests cover:

- pre-start rejection event ordering and final status;
- uncertainty escalation and final status;
- ambiguous receipt escalation;
- exact receipt-status to durable work-item status mapping;
- projection Debug non-leakage.

Live PostgreSQL conformance covers:

- no-attempt pre-start failure;
- attempt-aware reconciliation escalation;
- exact replay for both outcomes;
- durable run, work-item, and attempt postures;
- compatibility with completed receipt projection.

The live transaction remains CI-dependent when no local PostgreSQL test URL is
configured. Mandatory CI is required before merge.

## 9. Documentation Assessment

The roadmap, hosted plan, runtime guide, threat model, report, and this review
now say:

- provider-failure/reconciliation projection is implemented;
- uncertainty escalates and blocks blind retry;
- no provider write or OpenShell integration was added;
- access material, production authority, and complete deployment/recovery
  proof remain open;
- the hosted alpha is not production ready.

## 10. Blockers

No blocker remains inside this phase.

The complete hosted alpha remains blocked by:

1. deployment/recovery proof;
2. production-suitable authentication and authority;
3. access-material isolation and time-of-use resolution;
4. terminal report proof in the deployed topology.

## 11. Non-Blocking Follow-Ups

- Add fault injection for process loss between provider return and transaction
  commit.
- Define an operator resolution model for an ambiguous receipt separately.
- Separate API and worker database privileges.
- Move bounded event pagination into the database query.

## 12. Recommended Next Phase

Proceed to **single-tenant hosted deployment and recovery proof**.

The fresh-pull user review reinforces that proportional governance and quiet
success are the next product-pressure lane, but it does not supersede this
load-bearing hosted integrity sequence. OpenShell remains a future optional
execution provider after the hosted and scoped-authority boundaries are
accepted; a fork is not justified.
