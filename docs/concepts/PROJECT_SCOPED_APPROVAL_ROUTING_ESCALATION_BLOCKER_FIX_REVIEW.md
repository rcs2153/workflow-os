# Project-Scoped Approval Routing Escalation Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to project-scoped route persistence planning.

The resolver no longer permits an ordinary pending approval to select the
workflow escalation contact by changing only a caller-supplied routing enum.
Escalation-contact routing now requires an explicit `EscalationRecord` bound to
the exact approval run and immutable escalation contact, and the route commits
to the stable escalation reference.

## 2. Scope Verification

The fix stayed within the approved Core-only blocker scope. It added one
explicit escalation subject to resolver input, one optional stable escalation
reference to the route, validation, deterministic identity binding, focused
tests, and bounded documentation updates.

It did not add route persistence, event-history discovery, a hosted inbox,
notification delivery, workflow schema changes, dynamic identity, RBAC, IdP
integration, approval automation, provider writes, runtime execution behavior,
CLI behavior, or release changes.

## 3. Original Blocker Restatement

The initial route resolver accepted `WorkflowEscalationContact` as a
caller-selected routing reason without requiring an escalation subject. The
same ordinary pending approval could therefore be routed to the escalation
contact merely by changing the enum, even though no runtime escalation was
represented.

That contradicted the accepted rule that escalation contacts are candidates
only for actual runtime escalation routes.

## 4. Fix Approach Assessment

The selected approach is narrow and compatible with the model-only boundary:

- `ProjectApprovalRouteInput` accepts an optional borrowed `EscalationRecord`;
- ordinary maintainer routing rejects an escalation subject;
- escalation-contact routing rejects a missing subject;
- the subject run must equal the pending approval run;
- the subject contact must equal immutable workflow escalation metadata;
- the route stores only the stable escalation identifier;
- the identifier participates in the content-derived route identity;
- deserialization revalidates subject posture and route identity.

This removes the enum-only bypass without copying escalation failure messages,
reasons, suggested actions, attempts, or skill metadata into the route.

## 5. Validation Boundary Assessment

The resolver now fails closed with stable codes for:

- missing escalation subjects;
- unexpected escalation subjects on ordinary routes;
- run or immutable-contact mismatch;
- malformed escalation references;
- invalid serialized escalation-route posture.

Errors do not include escalation, run, approval, actor, project, path, reason,
or payload values. Existing pending-approval, active run binding, exact-project
authority, bounded authority-view, and unique-principal validation remains
unchanged.

## 6. Determinism, Serde, And Privacy Assessment

The escalation reference changes route identity and is excluded from `Debug`.
Valid escalation routes round-trip through serde; removing the escalation
reference or changing a decision-relevant field fails closed with the generic
route deserialization error.

The route remains payload-free. It does not store escalation messages, command
output, provider payloads, source content, credentials, paths, or approval
reasons.

## 7. Authority And Source-Of-Truth Assessment

The fix preserves the routing invariant:

```text
routing candidate = immutable workflow metadata plus exact routing subject
decision authority = deployment-owned exact-project capability
effective recipient = candidate intersection authority
```

An escalation record does not grant authority. The selected contact must still
hold exact-project `ApprovalDecide` authority.

The pure resolver intentionally accepts explicit trusted inputs. Before a
hosted inbox may rely on the result, the persistence/composition phase must
source ownership from the immutable run definition and escalation proof from
durable run state rather than accepting caller-authored records as durable
provenance. This is the next integration boundary, not a blocker in the pure
Core model.

## 8. Regression Assessment

Existing maintainer routing remains unchanged. Missing metadata remains
explicit, named actors without exact-project authority remain unresolved,
wrong-project and wrong-organization grants do not route, duplicate authority
state fails closed, and route identity remains deterministic.

The full workspace suite confirms no regression in approval presentation,
runtime escalation, proportional governance, hosted project boundaries,
provider writes, state backends, WorkReport, EvidenceReference, adapters, or
CLI behavior.

## 9. Test Quality Assessment

Focused coverage now proves:

- ordinary approval escalation selection fails without a subject;
- ordinary routing rejects unexpected escalation proof;
- wrong-run escalation proof fails closed;
- immutable-contact mismatch fails closed;
- exact run/contact proof routes only with exact-project authority;
- the stable escalation reference participates in serde posture;
- missing serialized proof fails closed;
- `Debug` redacts the escalation reference;
- original scope, authority, ambiguity, determinism, tamper, and privacy tests
  remain green.

No blocker-level test gap remains for this model slice.

## 10. Documentation Review

The roadmap, plan, initial review, and blocker-fix report preserve the original
finding and state the fix accurately. They continue to disclose that route
persistence, durable source authentication, hosted inbox behavior, notification
delivery, dynamic identity, schemas, provider writes, and runtime changes are
not implemented.

## 11. Blockers

None for acceptance of the Core route model.

## 12. Non-Blocking Follow-Ups

- The persistence/composition boundary must resolve ownership and escalation
  subjects from immutable/durable run state before writing a route.
- Route persistence must be create-only, exactly idempotent for matching
  content, and fail closed for conflicts.
- The future hosted inbox must filter by exact project, exact recipient, and
  current `ApprovalDecide` authority without treating a route as authority.

## 13. Recommended Next Phase

Project-scoped approval route persistence planning, followed by one narrow
create-only implementation slice. The plan should include durable source
authentication, duplicate reconciliation, conflict behavior, restart tests,
and the boundary between routing truth and decision authority. Hosted inbox
consumption should remain a later reviewed phase.

## 14. Validation

Completed against the fixed implementation:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test -p workflow-core --test project_approval_routing`: passed, 10 tests;
- `cargo test --workspace`: passed; explicitly opt-in live tests remained
  ignored as designed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 15. Governed Phase Record

- dogfood workflow: `dg/review`;
- run ID: `run-1786680506099773000-2`;
- approval ID:
  `approval/run-1786680506099773000-2/review-scope-approved`;
- approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof `presentation/2b38876a1a92b793`;
- terminal status: `Completed`;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: approval event marker present and matched to the persisted
  presentation record.

Repository inspection, review analysis, documentation edits, validation, and
future git or pull-request operations are executor work performed outside the
kernel. The kernel governed the approved review scope and recorded the
approval; it did not perform those actions.
