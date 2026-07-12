# Proportional Governance Read-Only Projection Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to immutable run-bundle hardening planning.**

The projection-specific safe wire boundary prevents unknown caller-supplied enum
and reason values from entering deserialization errors. The valid model and
serialization contract remain unchanged.

## 2. Scope Verification

The fix stayed within the approved deserialization and test boundary. It did not
redesign the selector, change public enum vocabulary, integrate the executor,
change approval defaults, add CLI behavior, persist decisions, append events,
create reports, alter schemas, call providers, write externally, add hosted or
RBAC behavior, or change release posture.

## 3. Fix Assessment

Private projection-only wrappers now parse each accepted serialized value with a
custom visitor. Unknown strings and malformed primitive/container values return
fixed errors without formatting the supplied value.

The wrappers map to the unchanged public enums only after a valid match. The
projection then applies its existing consistency checks for mode/risk,
profile-minimum reason, action mapping, and fixed posture.

Reasons use a temporary vector before conversion to the model `BTreeSet`, and
duplicate reasons fail consistency validation.

## 4. Error Non-Leakage Assessment

Focused tests prove non-leakage for:

- mode;
- risk class;
- action requirement;
- decision posture;
- persistence posture;
- reason values;
- malformed numeric mode input.

The tests use secret-like marker strings and verify those markers do not appear
in returned errors. Valid `Debug` and serialization remain enum-only and
payload-free.

## 5. Contract Preservation

- Valid JSON field names are unchanged.
- Valid serialized enum values are unchanged.
- Public enum derives and variants are unchanged.
- The projection helper and accessors are unchanged.
- The core selector and decision model are unchanged.
- No runtime consumer was added.

The safe visitors use self-describing deserialization, which matches current
JSON/YAML-oriented repository usage. No binary serialization contract is
claimed.

## 6. Test Quality

The unknown-value regression matrix covers every field implicated by the
original blocker. Existing mapping, source-preservation, serde round-trip,
inconsistent-state, privacy, selector, and workspace tests remain green.

A direct duplicate-reason regression would improve future maintenance but is
non-blocking because duplicate rejection is explicit and the current public
projection constructor cannot create duplicates.

## 7. Privacy And Redaction

The original invalid-input leakage path is closed. No raw prompts, policies,
paths, payloads, actor data, approval reasons, environment values, credentials,
or token-like values are stored by the projection or included in its fixed
errors.

## 8. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test proportional_governance`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add a direct duplicate-reason deserialization regression before any public
  schema or CLI exposure.
- Reassess format compatibility if a non-self-describing serialization format
  becomes a supported contract.

## 11. Recommended Next Phase

Plan immutable run-bundle hardening. Define the exact resolved workflow,
policy, skill, governance, and configuration inputs needed for later inspection
and safe replay before expanding provider mutations.

Do not activate quiet success, integrate the executor, persist proportional
decisions, or add another mutation family as part of that planning phase.

## 12. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783869798343387000-2`.
- Approval ID:
  `approval/run-1783869798343387000-2/review-scope-approved`.
- Presentation ID: `presentation/5aedc7ef3317fe7c`.
- Approval outcome: granted through the proof-enforced approval path.
- Final status: completed.
- Event summary: 39 events, one approval request, one proof-marked approval
  grant, eight policy decisions, six scheduled steps, six successful mock skill
  invocations, no retries, and no escalations.
- Out-of-kernel work: Codex inspected the safe wire parser, tests, validation
  evidence, and docs and authored this review. The kernel governed scope and
  approval but did not perform the review or edit files.
- Report posture: this review is the blocker-fix review record. No runtime
  WorkReport or report artifact was generated or persisted.
