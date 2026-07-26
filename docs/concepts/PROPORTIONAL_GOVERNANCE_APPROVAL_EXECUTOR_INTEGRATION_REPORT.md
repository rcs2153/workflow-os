# Proportional Governance Approval Executor Integration Report

## 1. Executive Summary

Workflow OS now has one explicit local executor path for a complete,
source-bound proportional-governance result whose route is
`RequireApproval + Visible`.

Core constructs the aggregate approval subject from the same-call
authoritative `DocsCheck` assessment, persists the exact assessment, and
pauses before scheduling any workflow step. Grant and denial reuse the
existing durable approval lifecycle. Both require fresh exact reassessment
and persisted approval-presentation proof before decision events.

This is additive and opt-in. Existing executor methods and step-scoped
approval behavior remain unchanged.

## 2. Scope Completed

- Extended `ApprovalRequest` with one backward-compatible aggregate
  governance subject.
- Required exactly one truthful subject: complete step/skill identity or one
  aggregate `GovernanceApprovalBinding`.
- Added an explicit fresh-run authoritative approval request and result.
- Constructed the aggregate binding and deterministic approval identity inside
  Core from the same-call assessment.
- Persisted the exact immutable bundle and governance assessment before pause.
- Reused existing approval request events, projections, presentation proof,
  decisions, and resume state machine.
- Reassessed the authoritative local check and exact aggregate binding before
  grant or denial mutation.
- Resumed aggregate grants from the workflow start without treating the grant
  as a later step approval.
- Explicitly rejected aggregate approval as SideEffect authority.
- Preserved CLI handling for existing step approvals while allowing aggregate
  requests to have no current step.

## 3. Scope Explicitly Not Completed

The phase did not add:

- automatic or model-selected approval;
- a second approval system or synthetic workflow step;
- the authoritative `Denied + Visible` runtime route;
- retry or existing-run support for the authoritative consumer;
- CLI or workflow-schema exposure for proportional approval;
- approval-presentation UI;
- providers, OpenShell, sandbox execution, or credentials;
- SideEffect execution or new provider mutation families;
- report artifacts or automatic report generation;
- hosted behavior, reasoning lineage, or release changes.

## 4. API And Model Summary

The implementation adds:

- `LocalExecutionWithAuthoritativeDocsCheckApprovalGovernanceRequest`;
- `LocalExecutionWithAuthoritativeDocsCheckApprovalGovernanceResult`;
- `LocalGovernanceAssessmentApprovalPresentationDecisionRequest`;
- `execute_with_authoritative_docs_check_approval_governance(...)`; and
- `decide_approval_with_governance_reassessment_and_presentation(...)`.

`ApprovalRequest` retains its existing serialized step fields as optional
fields. A valid request contains either all three step subject fields or one
aggregate governance binding. Mixed, incomplete, missing, identity-mismatched,
or aggregate-plus-idempotency subjects fail closed with stable
`approval_request.subject.*` codes.

## 5. Runtime Ordering

The explicit path orders work as follows:

1. Require a fresh run and empty durable event state.
2. Prepare and validate the execution plan.
3. Build and create-only claim the immutable run bundle.
4. Execute the canonical `DocsCheck`.
5. Derive the complete source-bound aggregate assessment.
6. Require exact `RequireApproval + Visible`.
7. Persist the exact governance assessment binding.
8. Construct the aggregate approval binding in Core.
9. Append ordinary run-start events.
10. Append the existing approval request and pause before `StepScheduled`.

Decision ordering is:

1. Load the durable run and pending approval.
2. Re-run the authoritative check against the stored immutable bundle.
3. Require exact equality with the stored assessment and approval subject.
4. Resolve and validate fresh matching presentation proof.
5. Attach the existing proof marker.
6. Apply the existing grant or denial lifecycle.

No decision, resume, step, or skill event is appended when reassessment or
presentation validation fails.

## 6. Authority Boundary

An aggregate grant authorizes only resumption through the approved
proportional-governance boundary.

It does not:

- authorize a SideEffect;
- supply step or skill identity;
- carry a step idempotency key;
- satisfy a later workflow-declared step approval;
- authorize provider execution; or
- replace policy, capability, immutable-input, or check gates.

This keeps aggregate workflow posture separate from action-specific mutation
authority.

## 7. Failure And Privacy Behavior

The route fails closed when:

- the aggregate assessment is incomplete or not exactly
  `RequireApproval + Visible`;
- the source binding is absent;
- the approval subject is malformed or mismatched;
- current facts no longer reproduce the durable assessment;
- approval-presentation proof is missing, stale, ambiguous, or mismatched; or
- existing approval-resume validation fails.

Errors use stable codes and omit raw checks, source contents, paths, commands,
provider payloads, credentials, tokens, approval prose, and caller-supplied
secret-like values.

## 8. Test Coverage

Focused tests prove:

- the authoritative route pauses before any step or skill invocation;
- the aggregate request contains no synthetic step, skill, or idempotency key;
- aggregate and legacy step approval wire shapes round-trip;
- mixed, missing, and aggregate-idempotency subjects fail closed without
  leaking test values;
- missing presentation proof mutates no events;
- changed reassessment facts mutate no events;
- matching reassessment plus proof grants and completes the workflow;
- proof-enforced denial fails the run without invoking skills;
- the exact assessment is persisted; and
- existing approval, SideEffect, WorkReport, adapter, runtime, and CLI suites
  remain compatible.

## 9. External Feedback Reconciliation

Fresh-repo evaluation describes the kernel as coherent and honest while
identifying ceremony as the next product constraint. This phase advances the
same proportional-governance thesis: low-risk work can remain quiet, while an
authoritatively higher-risk assessment now reaches a real existing approval
boundary instead of a model-only result.

Repo-specific onboarding recommendations, concise first-run presentation, the
Node 20 integration-check posture, and the duplicate pre-scaffold diagnostic
remain separate product-polish lanes. They do not weaken this runtime
authority boundary.

## 10. Validation

The following passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- focused aggregate approval, denial, serde, and event tests; and
- the pre-documentation full `workflow-core` suite.

`npm run check:docs` and `git diff --check` are run after final documentation
and recorded at phase close.

## 11. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1785045324395763000-2`
- approval:
  `approval/run-1785045324395763000-2/composition-approved`
- presentation: `presentation/57b10e053dc171a9`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation event marker: present
- out-of-kernel work: source inspection, Rust implementation, focused and
  workspace validation, documentation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run checks, create a WorkReport artifact, or perform git actions

## 12. Remaining Limitations And Recommendation

The path is fresh-run-only, local, `DocsCheck`-only, and explicit. Approval
presentation remains an existing local proof boundary rather than a new UI.
The aggregate request has no standalone schema exposure.

Proceed next to a focused maintainer review. If accepted, implement the
bounded authoritative `Denied + Visible` route before any optional execution
provider, additional provider mutation, or broader approval default.
