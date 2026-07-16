# Capability Request Review Projection Review

## 1. Executive Verdict

**Needs blocker fixes.**

The model-only scope is clean, the explicit non-authority boundary is strong,
and the deterministic review-action mapping is useful. Two semantic integrity
gaps must be fixed before the request or projection can become a trustworthy
review input.

## 2. Scope Verification

The phase stayed within the approved model/helper boundary. It added no grant
issuance, automatic approval, connector activation, tool/context exposure,
runtime invocation, workflow resume, executor integration, persistence, events,
schemas, CLI behavior, provider writes, hosted administration, enterprise
identity, or release posture changes.

## 3. Model Assessment

The new request vocabulary is domain-neutral and appropriately bounded:

- request identity and typed purpose are explicit;
- actor, workflow, run, step, optional harness, resource, sensitivity, steward,
  lifecycle, and redaction context are represented;
- authority posture has only `not_granted`;
- already-authorized resolutions are rejected;
- review actions cover availability, connector, unsupported capability, grant,
  lifecycle, scope, policy, approval, evidence, and check posture.

The request stores references and bounded metadata rather than credentials,
payloads, command output, source contents, or provider responses.

## 4. Review Projection Assessment

`project_capability_request_for_review` is pure and non-mutating. It projects
stable resolution reasons into sorted, deduplicated review actions. The
serialized projection now retains its source reasons and rejects substituted
actions whose deterministic mapping does not match.

The projection cannot grant authority, connect a capability, expose a tool,
resume a run, or invoke a provider. This is the correct product boundary.

## 5. Blocker Findings

### Blocker 1: request context is not bound to resolution context

`CapabilityRequest::new` accepts a valid `CapabilityResolution` alongside
independent request fields. The resolution currently carries posture,
availability, selected grant, reasons, and evaluation time, but not the actor,
capability, resource, workflow, run, step, harness, or requested sensitivity
against which it was resolved.

Therefore a caller can attach a valid resolution from request A to the identity
and scope of request B. The resulting request remains non-authoritative, but its
review explanation may be false. That is unacceptable for a review-facing
governance record.

Required fix: bind every resolution to a validated resolution-context value
derived from the explicit resolver input, then require the request identity and
scope to match that context. This binding must not be described as a freshness
or time-of-use guarantee; future grant issuance and invocation must still
re-resolve current authority.

### Blocker 2: projection posture does not validate reason legality

The projection validates that actions match reasons, but it does not validate
that the reasons are valid for the declared resolution posture. A crafted wire
projection can declare `not_authorized` with `active_grant_matched`, map it to
the expected action, and pass validation even though a full
`CapabilityResolution` would reject that combination.

Required fix: reuse one canonical resolution posture/reason invariant at both
the resolution and projection boundaries, or validate a complete source
resolution envelope rather than duplicating a weaker subset.

## 6. Validation Assessment

Existing validation correctly rejects:

- unknown requested sensitivity;
- already-authorized requests;
- future resolution timestamps;
- invalid request expiry;
- unsafe identifiers and redaction metadata;
- empty, duplicate, unordered, or action-inconsistent projections.

Stable error codes do not echo raw identifiers or secret-like values. The two
blockers concern semantic binding, not basic bounds or leakage.

## 7. Privacy And Serde Assessment

Debug output redacts request, actor, resource, workflow, run, step, harness, and
steward identities. Custom deserialization reconstructs validated request and
projection boundaries. Serialization contains only model fields and no raw
provider/spec/command/parser payloads.

Serde remains fail-open on the two semantic combinations described above, so
wire compatibility is not accepted until the blocker fix adds tests for both.

## 8. Test Quality

The 42 focused tests cover the normal request/projection taxonomy, deterministic
ordering, authorized-request rejection, lifecycle/sensitivity validation,
serde round trips, substituted actions, and redaction safety. The workspace
suite, strict clippy, formatting, and docs checks pass.

Missing blocker tests:

- a resolution for one actor/resource/run cannot be attached to another;
- request sensitivity must match resolution context;
- `not_authorized` plus `active_grant_matched` projection wire fails;
- independent-evaluation posture rejects availability/grant-denial reasons;
- projection posture/reason validation stays aligned with full resolution
  validation.

## 9. Documentation Review

The implementation report is honest about the model-only boundary and the need
for fresh runtime re-resolution. It must be updated after the blocker fix to
describe the new identity/scope binding and preserve the distinction between
context integrity and freshness.

## 10. Blockers

1. Bind capability resolution identity/scope context to capability requests.
2. Enforce canonical posture/reason invariants in review projections.

## 11. Non-Blocking Follow-Ups

- Bind future persisted requests to immutable run bundles or another accepted
  source commitment.
- Define stale-request and re-review behavior before persistence.
- Re-resolve current authority before any future grant issuance, tool
  projection, or invocation.
- Decide whether request expiry should later produce an explicit expired review
  posture rather than only a deadline.

## 12. Recommended Next Phase

Run a focused capability request/projection semantic-binding blocker fix. Do
not begin tool/context projection, authority receipts, runtime wiring,
persistence, events, schemas, CLI behavior, connectors, or provider writes
until the fix is implemented and re-reviewed.

## 14. Fix-Forward Note

Both blockers were fixed and accepted in
[Capability Request Review Projection Blocker Fix Review](CAPABILITY_REQUEST_REVIEW_PROJECTION_BLOCKER_FIX_REVIEW.md).
This note preserves the original findings; it does not erase them.

## 13. Validation Run

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test capability_authority --quiet`: passed,
  42 tests.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed before this review document; rerun at phase
  close.
- `git diff --check`: passed before this review document; rerun at phase close.

Governed review:

- workflow: `dg/review`
- run ID: `run-1784175839245285000-2`
- approval ID: `approval/run-1784175839245285000-2/review-scope-approved`
- presentation ID: `presentation/bbb9ec79264c40e8`
- approval outcome: granted under delegated-maintainer authority after complete
  handoff presentation.
