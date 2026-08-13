# Proportional-Governance Visible-Disclosure Surface-Acceptance Proof Plan

Status: Implemented and accepted with non-blocking follow-ups.

## 1. Executive Summary

The selected local project-validation runtime can derive `Proceed + Visible`,
send one payload-free disclosure request to an injected local surface, and
receive a validated surface-acceptance receipt. Before this phase, that receipt
was returned transiently and did not become part of durable workflow history.

This phase adds one payload-free workflow event proving the narrow fact that
the configured surface accepted the exact disclosure request. It does not
claim external delivery, display, human observation, understanding, or
acknowledgement.

## 2. Goals

- Record surface acceptance durably before validation and skill execution.
- Bind the event to the exact durable assessment, run, workflow, correlation,
  surface, delivery identity, and timestamps.
- Require an idempotency key and reject duplicate delivery identities.
- Project only bounded posture into generic audit records.
- Cite the durable event from terminal WorkReports produced by the selected
  report path.
- Print CLI success posture only after durable event persistence.
- Preserve quiet, approval, denial, and ordinary executor behavior.

## 3. Non-Goals

This phase does not add:

- proof that a human saw, understood, or acknowledged the disclosure;
- exactly-once external delivery semantics;
- retries, a disclosure outbox, notifications, a UI provider, or hosted
  disclosure delivery;
- automatic approval or changes to approval presentation;
- workflow schema, SDK, or example changes;
- provider mutation, OpenShell execution, credentials, SideEffect execution,
  or writes;
- a new governance mode or proportional-governance decision axis; or
- release-posture changes.

## 4. Durable Event Contract

Add `GovernanceDisclosureSurfaceAccepted` to the workflow event vocabulary.
Its payload is the already validated, payload-free
`GovernanceDisclosureDeliveryReceipt`.

The runtime accepts the event only when:

- the run is still `Created`;
- the matching governance assessment is already bound;
- run, workflow, immutable bundle, aggregate assessment, and correlation
  identities match exactly;
- the receipt is internally valid and accepted after its request time;
- the event timestamp is not earlier than surface acceptance;
- the delivery identity has not already been recorded; and
- an idempotency key is present.

## 5. Ordering

The selected visible route orders work as follows:

1. derive the complete authoritative assessment;
2. invoke the explicitly injected disclosure surface;
3. construct and validate the surface-acceptance receipt;
4. persist the immutable assessment binding;
5. append `RunCreated`;
6. append `GovernanceAssessmentBound`;
7. append `GovernanceDisclosureSurfaceAccepted`;
8. append `RunValidated` and `RunStarted`; and
9. begin ordinary step execution.

If the surface rejects the request, no run events or skills are produced.
If event persistence fails after the surface accepted the request, execution
does not proceed. A crash between surface acceptance and event persistence can
cause a later retry to invoke the surface again. This phase therefore provides
at-least-once surface invocation and exactly-once durable recording per
delivery identity, not exactly-once external delivery.

## 6. Audit And Reporting

Generic audit projection records only:

- the injected-local surface kind;
- `human_observation=not_claimed`; and
- `acknowledgement=not_claimed`.

It does not copy surface references, delivery identities, assessment
fingerprints, disclosure payloads, command output, provider output, or secret
material.

Terminal WorkReports from the selected project-validation report composition
cite the durable workflow event ID automatically. They do not recreate a
delivery receipt or claim observation.

## 7. CLI Posture

The injected CLI surface no longer prints success from inside `deliver()`.
Human and JSON output report `disclosure_surface_acceptance=persisted` only
after the governed route returns with the durable event present. Human output
also states that observation and acknowledgement are not claimed.

## 8. Tests

Focused tests must prove:

- valid event ordering and snapshot rehydration;
- legacy snapshots remain readable;
- missing assessment, missing idempotency, duplicate identity, and mismatched
  identity fail closed;
- audit output is bounded and payload-free;
- the selected visible route invokes the surface once in the successful path,
  records one durable event, and executes skills only after that event;
- terminal reports cite the durable event ID; and
- CLI output reports persistence and explicit non-claims without the old
  premature success line.

## 9. Validation

Run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused runtime, audit, executor, report, and CLI tests;
- `cargo test --workspace`;
- `npm run check:docs`;
- `npm run check:integrations`;
- `npm run check`; and
- `git diff --check`.

## 10. Recommendation

After implementation, run a focused maintainer review before broadening
disclosure surfaces, adding retry/outbox semantics, or using the event as a
stronger observation claim.
