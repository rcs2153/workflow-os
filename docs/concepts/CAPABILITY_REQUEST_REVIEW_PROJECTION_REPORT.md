# Capability Request Review Projection Report

## 1. Executive Summary

Workflow OS now has a bounded capability-request model and a pure review-only
projection helper. A request represents missing or incomplete authority; it is
always `not_granted`. The projection converts validated capability-resolution
reasons into deterministic review actions without granting authority,
activating a connector, exposing a tool, resuming a run, or invoking a provider.

This remains a model/helper phase. No runtime, persistence, event, schema, CLI,
connector, tool, provider-write, hosted, or enterprise identity behavior was
added.

## 2. Scope Completed

- Added validated capability request identity and typed purpose vocabulary.
- Added bounded request definitions carrying actor, workflow, run, step,
  harness, resource, sensitivity, lifecycle, steward, and redaction context.
- Added explicit `CapabilityRequestAuthorityPosture::NotGranted`.
- Added a pure `project_capability_request_for_review` helper.
- Added deterministic review actions for availability, connector,
  unsupported-capability, grant, lifecycle, scope, policy, approval, evidence,
  and check obligations.
- Bound serialized review actions to ordered source resolution reasons.
- Bound every resolution to the actor, capability, resource, workflow, run,
  step, harness, and sensitivity context used by the resolver.
- Required request identity and scope to match that resolution context exactly.
- Shared canonical posture/reason validation across resolution and projection
  wire boundaries.
- Added redaction-safe Debug and validated serde boundaries.

## 3. Scope Explicitly Not Completed

- No capability grant issuance from a request.
- No automatic approval or delegated authority.
- No connector installation, connection, or activation.
- No tool or context visibility projection.
- No tool, command, adapter, or provider invocation.
- No workflow resume or executor integration.
- No persistence, events, audit projection, schemas, SDKs, or CLI behavior.
- No provider writes, hosted administration, enterprise identity, or release
  posture changes.

## 4. Model And Helper Summary

`CapabilityRequest` accepts a validated non-authorizing
`CapabilityResolution`, a typed request purpose, bounded identities and scope,
known sensitivity, timestamps, optional review steward, and validated redaction
metadata. It rejects authorized resolutions, unknown sensitivity, invalid
lifecycles, future resolution timestamps, and unsafe identifiers or metadata.

`CapabilityRequestReviewProjection` carries the request ID, explicit
non-authority posture, source resolution posture and ordered reasons,
deterministic review actions, optional steward, deadline, and sensitivity. Its
validated wire form recomputes expected actions from the reasons and rejects a
projection whose actions were substituted or reordered.

## 5. Authority Boundary

A capability request is not a grant. A review projection is not an approval,
policy decision, connector action, or runtime authorization. Neither type can
create authority or make a capability executable.

The request carries a validated, identity-bound resolution snapshot. That
snapshot is useful for review posture but is not a freshness guarantee. Any future grant
issuance, tool projection, or runtime invocation must re-resolve authority from
current explicit availability, grant, actor, resource, scope, sensitivity,
prerequisite, and evaluation-time inputs. It must not trust this request or
projection as an authority source.

## 6. Validation And Privacy

- Stable validation codes describe failure classes without echoing raw values.
- IDs, resource references, and redaction metadata are bounded and reject
  secret-like strings.
- Debug output redacts request, actor, resource, and steward identities.
- Custom deserialization reconstructs validated request and projection values.
- Review projections fail closed on unknown sensitivity, authorized posture,
  duplicate or unordered reasons/actions, posture/reason mismatch, and
  reason/action mismatch.
- The model stores no credentials, provider payloads, command output, source
  contents, parser payloads, environment values, or unrestricted metadata maps.

## 7. Test Coverage

Focused tests cover:

- missing-grant request and review projection;
- rejection of already-authorized work;
- independent policy, approval, evidence, and check obligations;
- missing, unknown, disconnected, and unsupported availability;
- revoked, expired, and sensitivity-insufficient grants;
- lifecycle and sensitivity validation;
- deterministic ordering;
- request/projection serde round trips;
- invalid wire forms, context substitution, impossible posture/reason pairs,
  and substituted review actions;
- redaction-safe errors, Debug, and serialization boundaries.

## 8. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-core --test capability_authority --quiet`: passed,
  44 tests after the blocker fix.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1784173483395832000-2`
- approval ID:
  `approval/run-1784173483395832000-2/implementation-approved`
- presentation ID: `presentation/c1698a4deedb4cb6`
- approval outcome: granted under delegated-maintainer authority after the full
  approval handoff was presented.

Two earlier phase-start attempts failed closed before a run was created: one
exceeded the bounded scope length and one used wording rejected by the
secret-like-value detector. The governed phase was restarted with bounded,
non-sensitive context rather than bypassing either validation.

## 10. Remaining Limitations

- Resolution snapshots are identity/scope bound but not immutable-run-bundle
  committed.
- No durable request or projection store exists.
- No steward review workflow consumes the projection.
- No grant issuance path exists.
- No runtime re-resolution or time-of-use enforcement exists.
- No tool/context projection or authority receipt exists.

## 11. Recommended Next Phase

Perform a focused capability request and review-projection maintainer review.
Review the non-authority invariant, reason/action integrity, serde safety,
freshness limitation, privacy posture, and regression coverage before any tool
or context projection work begins.
