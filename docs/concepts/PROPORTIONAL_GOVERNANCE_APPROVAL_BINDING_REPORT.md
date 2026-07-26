# Proportional Governance Approval Binding Report

## 1. Executive Summary

Workflow OS now has a narrow, payload-free binding for an aggregate,
source-bound proportional-governance approval subject.

The existing `ApprovalRequest` is unambiguously scoped to one workflow step
and skill. Reusing it unchanged for a pre-execution aggregate decision would
require a synthetic or unrelated step. The new
`GovernanceApprovalBinding` instead commits one bounded binding identity to
the exact complete, source-bound `RequireApproval + Visible`
`GovernanceAssessmentBinding`.

This is a model prerequisite only. It does not create a second approval
system and does not request, present, grant, deny, persist, resume, or route an
approval.

The model is not standalone runtime proof that an authoritative check ran.
The future executor route must construct it from the same-call authoritative
assessment and match the exact durable assessment before requesting approval.

## 2. Scope Completed

- Added `GovernanceApprovalBindingVersion`.
- Added bounded, secret-like-safe `GovernanceApprovalBindingId`.
- Added `GovernanceApprovalBinding`.
- Required exact ownership of the source-bound aggregate assessment shape.
- Required complete facts and an authoritative source binding.
- Required the only valid route to be `RequireApproval + Visible`.
- Added fail-closed serde, unknown-field rejection, and redaction-safe Debug.
- Exported the model from `workflow-core`.
- Added focused model and privacy tests.

## 3. Scope Explicitly Not Completed

The phase did not add:

- executor approval routing;
- synthetic workflow steps or skill identities;
- approval requests, presentation records, decisions, persistence, events, or
  resume behavior;
- automatic or delegated approver installation;
- CLI, schema, workflow-spec, or runtime-config changes;
- provider execution, OpenShell integration, SideEffect execution, or writes;
- hosted behavior, reasoning lineage, or release changes.

## 4. Existing Model Assessment

`ApprovalRequest` requires:

- `step_id`;
- `skill_id`;
- `skill_version`; and
- step execution-context and idempotency data.

Those fields truthfully describe a step gate. They do not truthfully describe
an aggregate decision made before any workflow step executes. Attaching the
aggregate gate to the first step would falsely imply that the approval
authorizes only that step and would weaken later context validation.

The new binding does not replace `ApprovalRequest`. It supplies the missing
aggregate subject commitment that a separately scoped integration can attach
to the existing approval lifecycle.

Deserializing or constructing a structurally valid binding does not grant
execution or approval authority. The future executor must not trust a
caller-supplied binding as proof of same-call assessment.

## 5. Model Boundary

`GovernanceApprovalBinding` stores:

- a contract version;
- a bounded approval-binding ID; and
- the exact `GovernanceAssessmentBinding`.

The assessment already commits:

- workflow and run identity;
- immutable run-bundle identity and integrity root;
- aggregate assessment fingerprint and algorithm;
- ordered step count;
- execution and disclosure posture;
- assessment completeness; and
- authoritative fact-source commitment.

The model stores no workflow definition, skill payload, command, check output,
source content, path, provider payload, credential, token, or rendered
approval prose.

## 6. Validation And Failure Behavior

Construction and deserialization fail closed unless:

- the binding ID is non-empty, bounded, uses supported characters, and is not
  secret-like;
- the assessment is complete;
- the assessment contains an authoritative source binding; and
- the aggregate route is exactly `RequireApproval + Visible`.

Stable errors use the
`governance.proportional_approval_binding.*` namespace and do not include
caller-supplied identifiers or assessment identities.

Unknown serialized fields are rejected. Invalid serialized routes cannot be
silently accepted.

## 7. Privacy And Redaction

`Debug` redacts the approval-binding ID. The nested assessment already redacts
workflow ID, run ID, aggregate fingerprint, and source fingerprint.

Serialization contains only bounded identities and payload-free integrity
commitments required for future exact matching. It does not contain raw
execution or evidence payloads.

## 8. Test Coverage

Focused tests prove:

- a valid source-bound aggregate approval assessment is accepted;
- exact workflow/run and aggregate assessment identity remain committed;
- proceed, denied, incomplete, and unbound assessments are rejected;
- identifiers are bounded and secret-like-safe;
- valid bindings round-trip through serde;
- invalid serialized routes fail closed;
- unknown serialized fields fail closed; and
- Debug does not expose approval, workflow, or run identity.

## 9. Validation

Required phase validation:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Results are recorded at governed phase close.

All required commands passed. The focused approval-binding test target also
passed with 6 tests.

## 10. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1785042181409464000-2`
- approval:
  `approval/run-1785042181409464000-2/implementation-approved`
- presentation: `presentation/ef1aa1bdde6ff428`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation event marker: present
- out-of-kernel work: source inspection, Rust implementation, focused tests,
  documentation, validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, create a WorkReport artifact, or perform git actions

## 11. Remaining Limitations And Recommendation

The binding is not yet attached to a durable approval request or enforced by
the executor. Existing step approvals remain unchanged.

Proceed next to a focused maintainer review of this model. After acceptance,
implement one explicit approval-required executor route that reuses the
existing approval request, presentation-proof, decision, resolved-context, and
state-machine semantics while binding the aggregate subject without creating
a synthetic step.
