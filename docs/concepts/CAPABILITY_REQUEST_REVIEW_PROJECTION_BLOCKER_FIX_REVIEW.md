# Capability Request Review Projection Blocker Fix Review

## 1. Executive Verdict

**Blockers fixed; proceed to step-scoped capability projection.**

The two review findings are resolved. Capability requests can no longer attach
a resolution from unrelated identity/scope context, and serialized projections
can no longer pair a resolution posture with illegal reasons. The model remains
explicitly non-authoritative and model-only.

## 2. Scope Verification

The fix stayed within approved scope. It added resolution-context vocabulary,
request equality validation, shared posture/reason validation, focused tests,
and honest documentation. It introduced no grant issuance, runtime authority,
tool/context exposure, connector behavior, invocation, workflow resume,
executor integration, persistence, events, schemas, CLI behavior, provider
writes, hosted behavior, enterprise identity, or release changes.

## 3. Resolution Context Assessment

`CapabilityResolutionContext` is derived inside
`resolve_capability_authority` from the explicit resolver input. It captures the
actor, capability, bounded resource, workflow, run, step, optional harness, and
requested sensitivity. It is private-field, read-only vocabulary with validated
standalone deserialization and redaction-safe Debug output.

Every `CapabilityResolution` carries that context. A request requires exact
equality across all corresponding fields. Stable mismatch errors disclose no
raw context values. The original context-substitution blocker is fixed.

## 4. Posture/Reason Assessment

One canonical helper now defines legal reason families for authorized,
independent-evaluation, and non-authorized postures. Full resolutions reuse it
alongside their stricter availability and selected-grant invariants. Review
projections reuse it before checking deterministic reason/action equality.

Wire tests prove that non-authorized plus active-grant reason and
independent-evaluation plus denial reason fail closed. The original
posture/reason blocker is fixed.

## 5. Authority And Freshness Boundary

The fix establishes identity/scope integrity, not freshness or runtime
authority. Requests remain `not_granted`; projections remain review-only.
Availability, grants, prerequisites, and time can change after request
creation. Future grant issuance, tool projection, and invocation must re-resolve
current authority and must not trust the stored request as permission.

This distinction is explicit in the reports and roadmap.

## 6. Privacy And Serde Assessment

- Context IDs and resource references use existing validated model types.
- Standalone context deserialization rejects unknown sensitivity and invalid
  resource posture.
- Resolution deserialization validates context and full resolution invariants.
- Request deserialization reconstructs the exact context-equality boundary.
- Projection deserialization validates posture/reasons and reasons/actions.
- Debug output redacts context, request, actor, resource, and steward identity.
- Stable errors do not echo raw identifiers or secret-like values.
- No raw payload, source, command, parser, environment, or credential fields
  were introduced.

## 7. Test Quality

The focused suite contains 44 passing tests. New regression coverage includes
actor, resource, run, and sensitivity context substitution plus impossible
projection posture/reason combinations. Existing resolution wire tests now
exercise the context-bearing shape. Existing grant, availability, resolution,
request, projection, privacy, and serde tests remain green.

The full workspace suite, formatting, strict clippy, docs check, and diff check
all pass.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- Bind future persisted request records to an accepted immutable source/run
  commitment.
- Define stale-request and re-review semantics before persistence.
- Preserve mandatory fresh re-resolution before grant issuance or invocation.
- Treat the new context field as preview wire vocabulary until schema and
  compatibility policy are separately approved.

## 10. Recommended Next Phase

Proceed to a pure step-scoped capability projection model/helper phase. It may
derive bounded visible capability identifiers from fresh explicit resolution
inputs. It must not load tools, connect adapters, expose credentials, execute
commands, invoke providers, mutate state, persist projections, add events,
change schemas/CLI, or authorize provider writes.

## 11. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test capability_authority --quiet`: passed,
  44 tests.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: rerun at review close.
- `git diff --check`: rerun at review close.

Governed re-review:

- workflow: `dg/review`
- run ID: `run-1784178131649231000-2`
- approval ID: `approval/run-1784178131649231000-2/review-scope-approved`
- presentation ID: `presentation/2a116934faa9f94b`
- approval outcome: granted under delegated-maintainer authority after complete
  handoff presentation.
