# Scoped Capability GitHub Read-Only Integration Plan Blocker Fix Review

## 1. Executive Verdict

**Blockers fixed; proceed to the scoped capability GitHub read-only runtime
implementation.**

The corrected plan resolves all five findings without broadening the selected
operation or weakening existing adapter, authority, or receipt boundaries.

## 2. Scope Verification

The fix remains documentation-only. It adds no Rust behavior, adapter call,
provider write, live-network requirement, credential handling, CLI, schema,
SDK, automatic grant, hosted identity, nested harness, or release change.

## 3. Policy-Precheck Assessment

The plan now correctly treats capability authority as additive to policy.
Fixture execution accepts only fixture-test allowed provenance, while live
read-only execution accepts only runtime-policy allowed provenance. Missing,
denied, approval-granted, mock, local, and write modes fail before source use.

The helper does not synthesize policy success from a capability grant, and a
grant's independent policy prerequisite remains unsatisfied in this slice.

## 4. Repository Identity Assessment

Owner and repository are separately bounded, validated, and normalized to
ASCII lowercase before source query or adapter construction. One GitHub
repository therefore has one capability-resource identity across case aliases.

The plan also forbids retaining original caller spelling in errors, Debug, or
receipt evidence.

## 5. Source Query Assessment

The plan now requires dedicated crate-private single-query capability-use
query, query-set, and fact-set types. Existing required-context query and fact
types are not reused, and no fake context requirement is created.

Reusing source registration, freshness, completeness, consistency, snapshot,
and commitment primitives remains appropriate and avoids a parallel source
trust system.

## 6. Grant And Projection Assessment

Generic capability resolution remains unchanged. This first integration adds
a stricter post-resolution check that the selected grant itself contains the
exact run and step plus the exact harness posture. That is an integration
constraint, not a hidden change to grant semantics.

Projection remains derived from the fresh authorized resolution and cannot be
supplied by the caller or reused as permission.

## 7. Receipt Assessment

Choosing a separate `CapabilityUseAuthorityReceipt` V1 is the smallest truthful
design. It prevents required-context fields from being fabricated and leaves
the current `AuthorityReceipt` V1 bytes and APIs unchanged.

Trusted construction remains private and proof-based. Deserialization produces
only an explicitly unverified claim. The new receipt remains local, unsigned,
point-in-time, payload-free evidence and cannot authorize later use.

## 8. Failure Semantics Assessment

The explicit outcome domains preserve whether the client was never called,
the read failed, the read and receipt succeeded, or the read succeeded while
receipt construction failed. This prevents a completed provider read from
being mislabeled as an ordinary error and retried blindly.

The required pre-call ordering and zero-client-call tests remain clear.

## 9. Privacy And Test Assessment

The plan prohibits raw provider bodies, repository spelling, source facts,
grant contents, credentials, headers, environment values, paths, and
secret-like strings in errors, Debug, or receipts.

The expanded test plan now covers policy-provenance mismatch, case aliases,
segment bounds, exact selected-grant scope, absence of context fabrication,
existing receipt byte compatibility, unverified receipt deserialization, and
post-read receipt failure.

## 10. Remaining Blockers

None.

## 11. Implementation Guardrails

- Keep all new composition APIs crate-private.
- Use an injected fixture client for required tests; live GitHub is optional.
- Construct the exact existing GitHub adapter action contract and do not call
  the client directly.
- Do not persist the new receipt or expose it through CLI/schema in this phase.
- Do not broaden to another read operation or any provider mutation.
- Treat direct existing adapter APIs as unchanged and not automatically
  capability-gated.

## 12. Recommended Next Phase

Implement the complete scoped capability GitHub repository-metadata read
vertical slice in one governed milestone, including source query/fact types,
same-call resolution, exact projection, adapter composition, separate receipt,
explicit outcomes, focused tests, docs, and an implementation report.

## 13. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1792692068084935000-2`
- approval ID:
  `approval/run-1792692068084935000-2/review-scope-approved`
- presentation ID: `presentation/ab4099ef4add3f32`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced with one persisted
  presentation record and event marker
- validation summary: `npm run check:docs` and `git diff --check` passed
- kernel boundary: governance coordination only; plan inspection, review
  writing, validation, Git, and pull-request work remain out of kernel
- out-of-kernel work: plan inspection, review authorship, documentation
  validation, Git operations, pull-request publication, CI observation, and
  merge handling
