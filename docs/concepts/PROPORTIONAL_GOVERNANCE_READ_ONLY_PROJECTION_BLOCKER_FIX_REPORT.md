# Proportional Governance Read-Only Projection Blocker Fix Report

## 1. Executive Summary

The projection deserialization error-redaction blocker is fixed. Projection
input now passes through private safe wire enums that return fixed field-specific
errors for unknown or malformed values without echoing caller input.

The public projection model, accepted selector, valid serialization shape, and
all runtime behavior remain unchanged.

## 2. Original Blocker

The projection had custom consistency validation, but its private wire fields
used public enums with derived `Deserialize`. Serde could reject an unknown enum
or reason value before reaching the fixed projection error and include the
caller-supplied value in its generated unknown-variant message.

That violated the accepted non-leaking deserialization boundary.

## 3. Fix Approach

A private `projection_wire_enum` macro now creates projection-only wrappers with
custom visitors. Each visitor:

- accepts only the existing serialized enum vocabulary;
- maps valid values to the unchanged public enum;
- returns a fixed field-specific error for unknown strings;
- returns the same fixed error for malformed primitive, sequence, map, or null
  values;
- never includes the supplied value in the error.

Reasons deserialize into a temporary vector and then a `BTreeSet`. Duplicate
reasons are rejected before projection construction.

## 4. Public Contract Preservation

- Public enum variants and derives are unchanged.
- Valid projection JSON field names and values are unchanged.
- The selector and accepted decision model are unchanged.
- Projection construction and accessors are unchanged.
- No new runtime consumer exists.

## 5. Privacy And Redaction

Unknown mode, risk-class, action, decision-posture, persistence-posture, and
reason values now fail with bounded fixed errors. A malformed numeric projection
mode also fails without echoing the numeric value.

Valid `Debug` and serialization remain payload-free.

## 6. Tests Added

Focused regression tests cover:

- unknown mode non-leakage;
- unknown risk-class non-leakage;
- unknown action-requirement non-leakage;
- unknown decision-posture non-leakage;
- unknown persistence-posture non-leakage;
- malformed numeric mode non-leakage;
- unknown reason non-leakage;
- existing projection and selector behavior.

## 7. Governed Phase Evidence

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783821573368097000-2`.
- Approval ID: `approval/run-1783821573368097000-2/fix-approved`.
- Presentation ID: `presentation/eac468aa69732427`.
- Approval outcome: granted through the proof-enforced approval path.
- Phase status: completed.
- Event summary: 39 events, one approval request, one proof-marked approval
  grant, eight policy decisions, six scheduled steps, six successful mock skill
  invocations, no retries, and no escalations.

## 8. Validation

- `cargo test -p workflow-core --test proportional_governance`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

The first focused compile exposed an internal wrapper-ordering derive mismatch.
The implementation was simplified to a vector wire plus explicit duplicate
rejection; no public trait or contract was widened.

## 9. Out-Of-Kernel Work

Codex edited the Rust deserialization boundary, tests, roadmap/status docs, and
this report and ran the focused commands. The kernel governed scope and approval
but did not edit files, run tests, create a WorkReport, or persist an artifact.

No git, pull request, provider, or external-write action occurred in this phase
before final validation.

## 10. Remaining Limitations

- Focused blocker-fix review subsequently accepted the fix in
  [Proportional Governance Read-Only Projection Blocker Fix Review](PROPORTIONAL_GOVERNANCE_READ_ONLY_PROJECTION_BLOCKER_FIX_REVIEW.md).
- The projection remains in memory, assessed but not enforced, and not
  persisted.
- Immutable run-bundle hardening remains future work.

## 11. Recommended Next Phase

Historical recommendation at phase close: run the full suite and focused
re-review. Both are complete. The current next phase is immutable run-bundle
hardening planning before additional provider mutation expansion.
