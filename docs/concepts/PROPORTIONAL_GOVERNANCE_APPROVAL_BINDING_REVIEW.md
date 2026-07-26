# Proportional Governance Approval Binding Review

## 1. Executive Verdict

**Phase accepted; proceed to approval-required executor integration.**

The model is a truthful, narrow prerequisite. It supplies an aggregate
approval subject without introducing a second approval lifecycle or
manufacturing a workflow step.

## 2. Scope Verification

The phase stayed within model-only scope.

It added no:

- executor routing or automatic approval;
- synthetic workflow step or skill;
- approval request, decision, presentation, persistence, event, or resume
  behavior;
- CLI, schema, workflow-spec, or runtime-config behavior;
- provider, OpenShell, SideEffect, or write execution;
- hosted behavior, reasoning lineage, or release change.

## 3. Existing Approval Model Assessment

The existing `ApprovalRequest` cannot represent the aggregate pre-execution
gate without ambiguity. It requires a concrete step, skill, skill version,
resolved step execution context, and optional skill idempotency key.

Using the first workflow step would misstate the approval subject and could
make later context validation appear narrower than the authority granted.
Splitting the aggregate subject into a dedicated binding is therefore
justified.

## 4. Model Assessment

`GovernanceApprovalBinding` is appropriately small:

- versioned contract;
- bounded approval-binding identity; and
- exact `GovernanceAssessmentBinding`.

It does not duplicate requester, approver, decision, expiration,
presentation, or lifecycle fields already owned by the approval system.

The aggregate assessment already commits workflow/run identity, immutable
bundle integrity, aggregate fingerprint, step count, route, completeness, and
source commitment.

## 5. Validation Assessment

Validation correctly requires:

- a bounded supported identifier;
- complete assessment facts;
- source-binding presence; and
- exactly `RequireApproval + Visible`.

Proceed, denied, incomplete, and unbound shapes fail closed with stable
non-leaking codes. Unknown serialized fields and invalid serialized routes are
rejected.

## 6. Authority Boundary

A source-bound serialized assessment is not standalone proof that the
authoritative check ran in the current call. The implementation and report now
state this explicitly.

The next executor phase must:

1. derive the assessment through the existing same-call authoritative path;
2. construct the aggregate approval binding inside Core;
3. persist and reread the exact assessment binding;
4. bind the approval request and presentation proof to that exact subject; and
5. revalidate the durable subject before grant-side mutation.

Trusting a caller-supplied binding as runtime authority would be a blocker.

## 7. Privacy And Serde Assessment

The model stores no raw source, command, process, check, provider, credential,
token, or rendered approval payload.

Debug redacts the approval-binding ID and inherits redaction for assessment
identity and fingerprints. Error messages do not echo caller values. Valid
bindings round-trip; invalid and unknown wire fields fail closed.

## 8. Compatibility Assessment

Existing `ApprovalRequest`, approval presentation, decision, persistence,
events, and executor behavior are unchanged.

The new model is additive and public. It creates no runtime authority by
itself. A future integration can carry the binding through the existing
approval lifecycle while preserving current step approvals.

## 9. Test Assessment

Focused coverage proves:

- valid aggregate subject construction;
- exact assessment identity retention;
- route, completeness, and source rejection;
- bounded and secret-like-safe IDs;
- serde round trip and fail-closed invalid wire behavior;
- unknown-field rejection; and
- Debug non-leakage.

The full workspace suite passed, preserving existing approval, presentation,
runtime, provider, SideEffect, evidence, report, and adapter behavior.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Derive the eventual approval ID deterministically inside Core rather than
  accepting arbitrary caller authority.
- Decide the smallest backward-compatible way for the existing approval
  lifecycle to carry an aggregate subject without making step identity
  optional for ordinary step approvals.
- Keep the binding non-authorizing outside the same-call executor path.

## 12. Validation

Passed:

- focused approval-binding tests: 6;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 13. Recommended Next Phase

Implement one explicit approval-required executor route.

The route should reuse the accepted fresh-run immutable-bundle and
authoritative local-check composition, create the aggregate binding inside
Core, reuse existing approval presentation and decision enforcement, and
pause before workflow skill execution. It must not create synthetic steps,
automatic approvers, providers, SideEffects, writes, schemas, hosted behavior,
or release changes.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785044509685901000-2`
- approval:
  `approval/run-1785044509685901000-2/review-scope-approved`
- presentation: `presentation/50c60c28502bc48e`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation event marker: present
- out-of-kernel work: code and test inspection, review authoring, validation,
  and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not inspect
  code, edit files, execute checks, create a WorkReport artifact, or perform
  git actions
