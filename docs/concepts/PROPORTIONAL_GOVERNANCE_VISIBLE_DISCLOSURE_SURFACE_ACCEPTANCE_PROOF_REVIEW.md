# Proportional-Governance Visible-Disclosure Surface-Acceptance Proof Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation creates a truthful durable proof boundary for the selected
visible route. It records surface acceptance before validation and skill
execution, rejects invalid history, projects bounded audit posture, cites the
durable event from terminal reports, and delays CLI success posture until the
event is durable.

## 2. Scope Verification

The phase stayed within the approved selected-path runtime-composition scope.
It did not add new disclosure surfaces, retries, outbox behavior, human
acknowledgement, approval changes, schemas, provider execution, OpenShell
execution, SideEffect execution, writes, hosted expansion, or release changes.

## 3. Event Model Assessment

`GovernanceDisclosureSurfaceAccepted` is the correct event name. It avoids the
stronger and unsupported claims implied by `Delivered`, `Displayed`, or
`Acknowledged`.

The event reuses the validated payload-free receipt and requires:

- `Created` run status;
- an exact prior assessment binding;
- matching run, workflow, immutable bundle, aggregate assessment, and
  correlation identities;
- valid request and acceptance timestamps;
- a unique delivery identity; and
- an idempotency key derived from the full request identity.

Legacy snapshots omit the new vector safely through a Serde default.

## 4. Runtime Ordering Assessment

The selected visible route invokes the explicitly injected surface, validates
its receipt, then appends:

1. `RunCreated`;
2. `GovernanceAssessmentBound`;
3. `GovernanceDisclosureSurfaceAccepted`;
4. `RunValidated`;
5. `RunStarted`; and
6. ordinary skill events.

Focused tests prove the acceptance event precedes skill invocation. Surface
failure produces no run events or skill execution. Event/audit failure remains
an execution-stopping error under the existing append boundary.

## 5. Idempotency And Crash Semantics

Durable duplicate delivery identities fail closed. The event key commits to
delivery ID, assessment fingerprint, correlation identity, surface kind,
surface reference, and request timestamp.

The implementation correctly does not claim exactly-once external delivery. A
crash after surface acceptance and before event persistence can result in a
second invocation on retry. Exactly-once or independently confirmed delivery
would require a separately designed outbox/provider protocol.

## 6. Audit, Privacy, And Redaction

Generic audit projection includes only:

- `surface=injected_local`;
- `human_observation=not_claimed`; and
- `acknowledgement=not_claimed`.

It does not copy the surface reference, delivery identity, assessment
fingerprint, receipt serialization, provider output, command output, paths,
tokens, or secret-like values. The durable event contains bounded reference
metadata required to prove request identity, not raw disclosure content.

## 7. WorkReport Assessment

Terminal selected-path report composition discovers the durable event and
adds its event ID to workflow-event citations. It avoids duplicate caller
citations and does not recreate the receipt or EvidenceReference values.

This is appropriate for the currently selected one-surface route. If a future
route records multiple accepted surfaces, citation policy must be revisited so
all required acceptance events are cited deterministically.

## 8. CLI Assessment

The CLI surface handler no longer prints a success line while the receipt is
transient. Human output reports persisted surface acceptance only after the
route returns with durable state and explicitly states that observation and
acknowledgement are not claimed. JSON output exposes the bounded persisted/not
applicable posture.

## 9. Test Quality

Tests cover:

- event rehydration and legacy snapshot compatibility;
- missing assessment, missing idempotency, duplicate identity, and identity
  mismatch;
- bounded audit projection and non-leakage;
- selected-route ordering before skill invocation;
- durable backend rehydration;
- automatic report citation; and
- human and JSON CLI output.

The complete affected binaries and workspace suite passed.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add an injected event-append failure regression proving that a surface may
  have accepted the request while skills remain unexecuted and no CLI success
  is printed.
- Consider exposing explicit observation and acknowledgement non-claim fields
  in JSON rather than only the aggregate persisted posture.
- If multiple surfaces become supported, cite every required durable
  acceptance event rather than the first matching event.
- Plan an outbox/retry protocol only if stronger external-delivery semantics
  become a real product requirement.

## 12. Validation Review

The reviewed matrix passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- complete affected Core and CLI test binaries;
- `cargo test --workspace`;
- `npm run check:docs`;
- `npm run check:integrations`;
- `npm run check`; and
- `git diff --check`.

## 13. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1786582390652533000-2`
- Approval: `approval/run-1786582390652533000-2/review-scope-approved`
- Presentation: `presentation/d1e87826d8b7491b`
- Approval outcome: granted by delegated maintainer through proof-enforced
  approval
- Kernel boundary: the kernel governed review scope, approval, and event
  history; code inspection and validation remained outside the kernel

## 14. Recommended Next Phase

Accept the phase and continue roadmap runtime composition. Do not broaden this
event into proof of human acknowledgement. Select the next phase from the
current roadmap based on the highest remaining runtime-enforcement gap.
