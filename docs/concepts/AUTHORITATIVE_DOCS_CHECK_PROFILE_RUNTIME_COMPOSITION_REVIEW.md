# Authoritative Docs-Check Profile Runtime Composition Review

## 1. Executive Verdict

**Phase accepted; proceed to delivery and then reassess the next complete
runtime vertical slice.**

The implementation adds one real engineering-check path without opening an
arbitrary command surface. It reuses the accepted immutable declaration,
same-call attestation, proportional-governance, and executor boundaries.

## 2. Scope Verification

The phase stayed within its corrected approval. It adds the closed
`docs_check` profile value, explicit resolution, a closed handler variant, and
the existing explicit-facts route as public API. It does not add default
registration, project activation, CLI behavior, provider calls, writes,
OpenShell behavior, persistence changes, new workflow fields, or release
changes.

The initial governed run was correctly closed without edits when review showed
that its schema non-goal conflicted with the required serde-visible profile
value. The corrected run authorized that exact additive contract change.

## 3. Model And Compatibility Assessment

The profile enum remains closed with two variants. Existing
`workflow_os_project_validation` serialization and behavior are unchanged.
`docs_check` is additive vocabulary. The project configuration constructor
continues to accept only `observe_and_report + workflow_os_project_validation`,
so the new value does not silently activate a new YAML runtime path.

The resolved profile uses a private enum rather than a trait object supplied by
callers. Public selection therefore cannot inject a handler implementation or
command string.

## 4. Resolution And Registration Assessment

Docs resolution requires explicit npm, repository-root, and optional cache
paths. The canonical handler validates the fixed contract and prerequisites.
Resolution performs no process execution. Mismatched resolver selection fails
with `local_check.profile.resolver_mismatch` and does not echo paths or command
text.

Registration preserves collision checks and does not alter the empty default
registry. The resolved profile yields only its canonical skill ID/version and
boxed closed handler.

## 5. Runtime Assessment

The public explicit-facts route accepts no executable, arguments, or shell
input. It binds the resolved profile's canonical command contract into the
immutable run bundle, matches the exact selected declaration and fingerprint,
runs the check once, converts the accepted attestation into the selected step's
evidence/check fact, and routes the complete aggregate through existing quiet,
visible, approval, or denial semantics.

The end-to-end test proves a two-step workflow completes through ordinary
executor state and event behavior after one authoritative docs check. This is
runtime proof, not construction-only coverage.

## 6. Privacy And Security Assessment

Existing sanitized environment, disabled network, no-source-write posture,
bounded output, timeout, redaction, and non-leaking errors remain in force.
`Debug` for the public request redacts execution identity and selected step and
reports only bounded posture/count metadata.

## 7. Tests And Validation

Coverage proves:

- profile resolution is non-executing;
- explicit execution uses the canonical docs command;
- resolver mismatch fails closed without leakage;
- authoritative routing runs the check once;
- the result supplies the selected evidence/check fact; and
- ordinary two-step execution completes.

All required implementation validation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- Add a direct wire-format assertion if profile IDs are emitted in a generated
  schema or SDK surface.
- Keep `DocsCheck` explicit-facts-only until a separate phase defines a
  trustworthy project activation and current-fact source.
- Prefer a repository-neutral check profile for the next generalization rather
  than accumulating Workflow OS-specific commands.

## 10. Governed Review Evidence

- workflow: `dg/review`;
- run: `run-1786607619687680000-2`;
- approval: `approval/run-1786607619687680000-2/review-scope-approved`;
- presentation: `presentation/bec838d600ade8b1`;
- outcome: granted under delegated maintainer authority.

The review changes only this review record. Codex performed inspection and docs
validation outside the kernel; the kernel governed scope, approval, and event
history.

## 11. Recommended Next Phase

Deliver this accepted vertical slice. Then reassess the roadmap using runtime
value as the criterion: either define one repository-neutral authoritative
engineering-check profile or begin the collaborative team beta foundation.
Do not add another provider mutation merely because the model vocabulary exists.
