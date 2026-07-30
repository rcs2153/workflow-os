# Current-Authority Proportional-Governance Runtime Composition Review

## 1. Executive Verdict

Phase accepted; proceed to production local current-authority source boundary
planning for the closed project-validation profile.

The implementation closes a real runtime-composition gap without exporting
authority, changing CLI defaults, or broadening execution capabilities.

## 2. Scope Verification

The phase stayed within its private Core composition scope. It did not add
automatic approval, ambient authority, public authority objects, CLI behavior,
schemas, runtime configuration, providers, OpenShell, sandbox execution,
SideEffects, writes, persistence, hosted behavior, or release changes.

## 3. Trust-Boundary Assessment

The bridge does not trust caller-classified authority. It requires the authority
fact to be absent and inserts sufficient authority only while a freshly ready
registered-source assessment is being consumed.

Actor, workflow, run, step, harness contract ID/version, and contract content
hash must match the immutable execution binding. The first route is
intentionally restricted to one fact for one selected step.

## 4. Same-Call Use Assessment

Every use reruns registered-source selection, capability resolution,
step-scoped context projection, and required-context consumption. A borrowed
non-reusable callback is invoked only for ready authority. Blocked and
source-failure paths cannot call the executor route.

The injected fact exists only in the cloned request passed immediately to the
existing authoritative dispatcher. No reusable authorization handle escapes.

## 5. Failure And Privacy Assessment

The bridge fails closed on preclassified authority, mismatched identity,
invalid fact shape, blocked authority, stale/incomplete source, and impossible
callback reconciliation.

Stable errors carry no governed IDs, paths, commands, payloads, provider
outputs, credentials, or secret-like values. Underlying executor errors remain
unchanged when the consumer itself fails.

## 6. Compatibility Assessment

Existing executor methods and public exports are unchanged. The bridge remains
private and does not alter current CLI behavior. The hardcoded CLI authority
fact remains an explicit compatibility limitation, not a claimed enforcement
path.

This is compatible with future quiet-success behavior because lower-friction
routing can consume validated current authority without making authority a
user-controlled classifier.

## 7. Test Quality Assessment

Focused tests cover ready one-call consumption, revoked-authority blocking,
stale-source failure, caller-preclassification rejection, zero callback
invocation on denied paths, and error non-leakage. Existing current-authority
tests continue to cover contract substitution, grant lifecycle, prerequisites,
freshness, source completeness, replay posture, and exact metadata access.

## 8. Documentation Assessment

The plan, report, and roadmap state the private boundary and the unresolved CLI
shortcut honestly. They do not imply production source configuration, sandbox
execution, OpenShell integration, provider mutation, durable replay prevention,
or public authority consumption.

## 9. Blockers

None for the private bridge.

## 10. Non-Blocking Follow-Ups

- Add direct full-route integration coverage when the first production source
  and CLI consumer are introduced.
- Preserve one concrete Core-owned operation; do not export the generic
  same-call callback.
- Decide deterministic source freshness/configuration before replacing the CLI
  shortcut.
- Keep operator presentation independent from execution disposition.

## 11. Recommended Next Phase

Plan the first production local current-authority source and configuration
boundary for the closed project-validation profile. After review, replace the
CLI hardcoded authority fact with the source-bound route.

OpenShell remains a potential optional execution provider after authority and
proportional-governance composition; it must not become the authority source.

## 12. Governed Review Record

This review is part of the approved `dg/runtime-composition` phase:

- run ID: `run-1785403735124792000-2`;
- approval ID:
  `approval/run-1785403735124792000-2/composition-approved`;
- approval presentation ID: `presentation/dffdf3e53f66e336`;
- approval outcome: granted under delegated-maintainer authority; and
- approval-presentation enforcement: proof persisted before approval.

The implementation review does not authorize additional runtime surfaces.

## 13. Validation

- focused registered current-authority tests: passed, 37 tests;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- the canonical complete workspace-test result is required from GitHub Rust CI
  before merge because the local full command encountered extreme per-binary
  macOS launch latency after all reached suites had remained green.
