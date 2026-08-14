# Project-Scoped Approval Routing Core Model Review

## 1. Executive Verdict

Needs blocker fixes.

Ordinary maintainer routing is appropriately bounded and preserves the central
metadata-versus-authority invariant. Escalation-contact routing is not yet safe:
the caller can select it for any pending approval without proving that a runtime
escalation occurred.

## 2. Scope Verification

The phase remained within the approved Core model-only scope. It added a pure
resolver, model vocabulary, private/read-only route records, exports, focused
tests, and documentation.

It did not add persistence, a hosted inbox, notification delivery, workflow
schema changes, dynamic identity, RBAC, IdP integration, approval automation,
provider writes, runtime execution behavior, CLI behavior, or release changes.

## 3. Findings

### Blocker: escalation-contact routing lacks an escalation subject

`ProjectApprovalRouteInput` accepts a caller-selected
`ProjectApprovalRoutingReason`, and the resolver maps
`WorkflowEscalationContact` directly to `ownership.escalation_contact`. The
input contains no `EscalationRecord`, escalation event reference, or equivalent
bound proof that the approval is an actual runtime escalation. The focused test
therefore routes the same ordinary pending approval to the escalation contact
merely by changing the enum.

This violates the accepted plan rule that the escalation contact is selected
only for an actual runtime escalation route. It also gives future callers a
simple way to bypass ordinary maintainer routing while still returning a
truthy `Routed` result.

Action required: either add and validate an exact run-bound escalation subject
for `WorkflowEscalationContact`, or remove runnable escalation-contact
resolution from this slice and retain it as future vocabulary. Add a negative
test proving an ordinary approval cannot be routed to the escalation contact.

Relevant implementation: `crates/workflow-core/src/project_approval_routing.rs`
lines 313-347. Relevant test gap:
`crates/workflow-core/tests/project_approval_routing.rs` lines 208-245.

## 4. Model Assessment

The model is otherwise minimal and domain-neutral:

- `ProjectApprovalRouteId` is bounded and content-derived;
- route reason, status, and notification posture use closed enums;
- route fields are private with read-only accessors;
- routed and unresolved outcomes are explicit;
- route identity binds scope, run, approval, workflow, reason, status,
  recipient, and notification posture;
- the model stores stable references rather than payloads.

The timestamp is intentionally not part of the content identity, so repeated
resolution of decision-equivalent inputs remains deterministic.

## 5. Authority Boundary Assessment

Ordinary maintainer routing correctly implements:

```text
routing candidate = immutable workflow metadata
decision authority = deployment-owned exact-project capability
effective recipient = candidate intersection authority
```

The resolver does not treat `owning_team` as an actor, does not create grants,
does not fall back across projects or organizations, and returns unresolved
posture when authority is absent. Duplicate matching principal state fails
closed as ambiguous.

The blocker concerns routing-subject truth, not capability enforcement.

## 6. Validation And Failure Assessment

The resolver validates:

- project scope;
- approval subject shape;
- pending approval posture;
- bounded authority-view size;
- exact active run binding;
- exact organization and project capability;
- unique matching authority;
- route status, recipient, and notification consistency.

Errors use stable codes and do not include actor, project, approval, workflow,
path, reason, payload, or credential values. Invalid serialized route state
fails closed through generic serde errors.

## 7. Privacy And Redaction Assessment

No approval reasons, source contents, workflow payloads, evidence payloads,
command output, provider payloads, paths, credentials, or contact details are
stored. `Debug` redacts route, scope, run, approval, workflow, and recipient
identities. Serialization exposes only the stable references required by the
future project-scoped inbox boundary.

No privacy blocker was found.

## 8. Serde And Determinism Assessment

Valid routes round-trip through serde. Reconstruction recomputes the route
identity and rejects tampering. Repeated decision-equivalent inputs produce the
same route ID. The serialization shape is suitable for later create-only
persistence but does not itself authorize persistence or schema exposure.

## 9. Test Quality Assessment

Strong coverage exists for:

- exact-project maintainer routing;
- metadata not granting authority;
- ignored owning-team text;
- missing metadata;
- wrong-project and wrong-organization grants;
- ambiguous duplicate authority;
- inactive run binding;
- decided approvals;
- deterministic identity;
- serde tampering;
- redaction-safe `Debug`.

Missing blocker coverage:

- ordinary approval rejection when escalation-contact routing is selected;
- exact run-bound escalation proof when escalation routing is enabled.

## 10. Documentation Review

The plan, roadmap, and implementation report now state that the Core model is
implemented, the focused review found one blocker, and persistence, hosted
inbox behavior, notification delivery, dynamic identity, schemas, provider
writes, and runtime changes remain unimplemented.

## 11. Blockers

1. Bind escalation-contact routing to actual run-bound escalation proof, or
   remove it from executable resolution until that proof contract exists.

## 12. Non-Blocking Follow-Ups

- Consider replacing the caller-supplied immutable `OwnershipMetadata` input
  with a later adapter that derives it from the immutable run bundle. This is a
  hosted integration responsibility and does not block the pure Core resolver.
- Consider whether unresolved-route health should later be visible only through
  project audit posture or through a separately authorized operator surface.

## 13. Recommended Next Phase

Project-scoped approval-routing escalation-subject blocker fix.

Keep the fix narrow: require an exact run-bound escalation subject for
escalation-contact routing, or make that reason non-runnable. Re-run focused and
workspace tests, then perform blocker-fix review before route persistence or the
hosted inbox.

## 13.1 Fix-Forward Note

The blocker fix now requires a supplied `EscalationRecord` for
`WorkflowEscalationContact`, validates that its run matches the pending approval
and that its contact matches immutable escalation metadata, includes the stable
escalation reference in route identity, and rejects escalation proof on ordinary
maintainer routes. The original blocker finding remains part of the review
record. Focused blocker-fix review is still required before persistence or
hosted integration.

## 14. Validation

Completed against the reviewed implementation:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `cargo test -p workflow-core --test project_approval_routing`: passed, 8 tests;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 15. Governed Phase Record

- dogfood workflow: `dg/review`;
- run ID: `run-1786675380847601000-2`;
- approval ID: `approval/run-1786675380847601000-2/review-scope-approved`;
- approval outcome: granted with persisted presentation proof
  `presentation/69e471eb77201c4a`;
- terminal status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: approval event marker present and matched to the persisted
  presentation record.

Repository inspection, review analysis, documentation edits, validation, and
future git or pull-request operations are executor work performed outside the
kernel. The kernel governed the approved review scope and recorded the
approval; it did not perform those actions.
