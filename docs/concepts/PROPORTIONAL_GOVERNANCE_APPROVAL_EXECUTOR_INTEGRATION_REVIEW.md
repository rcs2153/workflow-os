# Proportional Governance Approval Executor Integration Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to the bounded
authoritative denial route.**

The implementation truthfully composes a complete, source-bound
`RequireApproval + Visible` assessment into the existing approval lifecycle.
It pauses before workflow work, requires fresh reassessment and persisted
presentation proof before decision mutation, and does not broaden aggregate
approval into step or SideEffect authority.

## 2. Scope Verification

The phase stayed within the approved additive executor scope.

It added no:

- second approval system, synthetic workflow step, or automatic approver;
- authoritative `Denied + Visible` route;
- retry or existing-run broadening;
- CLI or workflow-schema exposure;
- provider, OpenShell, SideEffect, or write execution;
- hosted behavior, reasoning lineage, or release change.

Existing executor methods and step-scoped approval defaults remain unchanged.

## 3. Approval Subject Assessment

The aggregate `GovernanceApprovalBinding` is carried as a distinct
`ApprovalRequest` subject rather than being misrepresented as the first
workflow step.

Subject validation requires exactly one of:

- complete step, skill, and skill-version identity; or
- one aggregate governance binding with matching run and workflow identity.

Mixed, incomplete, missing, identity-mismatched, and aggregate-plus-step
idempotency shapes fail closed with stable `approval_request.subject.*`
codes. Existing step approval JSON retains the established field names and
round-trips through the custom deserializer.

## 4. Construction And Persistence Boundary

Core constructs the aggregate subject from the same-call authoritative
`DocsCheck` assessment. The caller cannot supply an already-authorizing
aggregate binding to the executor route.

Before the approval request is appended, the route:

1. creates and validates the immutable run bundle;
2. executes the canonical local check;
3. derives the exact source-bound assessment;
4. requires complete `RequireApproval + Visible`;
5. persists the exact assessment binding; and
6. deterministically derives the approval-binding identity.

The approval request is then appended through the existing event and
projection paths.

## 5. Runtime Ordering Assessment

The fresh route appends ordinary run-start state and the aggregate approval
request before any `StepScheduled` or skill-invocation event. The run pauses
in `WaitingForApproval`.

On grant or denial, the implementation performs all of the following before
decision mutation:

- reloads the durable run and pending request;
- reloads the immutable bundle;
- reruns the authoritative local check;
- requires exact equality with the durable assessment and approval subject;
- resolves the durable presentation record;
- validates presentation identity, content, decision, and freshness; and
- derives the existing proof marker.

Changed facts or missing proof append no decision, resume, step, or skill
events.

## 6. Grant And Denial Assessment

A valid grant reuses the existing approval event and resume state machine. An
aggregate grant resumes from the beginning of the immutable workflow and does
not mark any workflow step as already approved. Later workflow-declared
step approvals therefore remain active.

A valid denial reuses the existing denial lifecycle, fails the run, and
invokes no skill.

This phase does not yet consume an authoritative aggregate
`Denied + Visible` assessment directly. That remains the next bounded route.

## 7. Authority Separation

The aggregate approval carries no step, skill, skill version, or step
idempotency key.

It does not:

- authorize a SideEffect;
- satisfy a later step approval;
- authorize provider or sandbox execution;
- replace capability or policy enforcement; or
- weaken immutable-input or resolved-context checks.

SideEffect approval linkage explicitly rejects an aggregate governance
subject.

## 8. Privacy And Error Assessment

The new route stores commitments and bounded local-check results rather than
raw source, command output, provider payloads, credentials, or approval
prose.

Debug implementations redact request and bundle identity surfaces. Validation,
reassessment, proof, and subject errors use stable codes and do not echo
caller-supplied secret-like values. Invalid serialized subjects fail closed.

## 9. Compatibility Assessment

The workspace suite confirms compatibility across:

- existing step-scoped approvals and approval presentation;
- runtime event rehydration and audit projection;
- CLI approval persistence and current-step projection;
- WorkReport and high-assurance approval behavior;
- provider-write readiness helpers; and
- SideEffect approval linkage.

No existing executor method changed its default behavior.

## 10. Test Assessment

Focused coverage proves:

- pause before any step or skill invocation;
- exact aggregate request construction;
- aggregate and legacy step subject serde compatibility;
- fail-closed malformed subjects without value leakage;
- no mutation when presentation proof is missing;
- no mutation when reassessment changes;
- proof-enforced grant and completion;
- proof-enforced denial without skill invocation; and
- explicit SideEffect authority rejection.

The tests inspect state and event ordering, not only return values.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Validate `ApprovalRequest::validate_subject()` at generic
  `ApprovalStore::save_approval_request` implementations so a future direct
  store caller cannot persist an invalid in-memory subject. Current executor
  construction and deserialization already fail closed.
- Add one combined regression workflow containing both the aggregate
  pre-execution gate and a later step-scoped approval. The implementation
  deliberately resets step approval state on aggregate resume, and the
  existing suites preserve step approval behavior, but a direct combined test
  would make that separation unmistakable.

These follow-ups do not block the explicit route because Core constructs and
validates the aggregate request before persistence, decision reassessment is
exact, and no default or schema-facing path accepts caller-created aggregate
authority.

## 13. Validation

Passed:

- focused aggregate approval, reassessment, denial, serde, and event tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 14. Recommended Next Phase

Implement only the bounded authoritative `Denied + Visible` route.

It should consume a complete source-bound denial assessment, record bounded
denial provenance, and stop before skill execution. It must not add approval,
provider, OpenShell, SideEffect execution, writes, CLI/UI behavior, schemas,
hosted behavior, or new mutation families.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1785050274107650000-2`
- approval:
  `approval/run-1785050274107650000-2/review-scope-approved`
- presentation: `presentation/d5d72ce4416e54ad`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation event marker: present
- out-of-kernel work: code and test inspection, review authoring, validation,
  and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not inspect
  code, edit files, execute checks, create a WorkReport artifact, or perform
  git actions
