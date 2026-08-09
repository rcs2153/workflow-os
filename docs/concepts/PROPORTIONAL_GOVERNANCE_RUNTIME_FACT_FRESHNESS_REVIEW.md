# Proportional-Governance Runtime-Fact Freshness Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation closes the model/helper portion of the trusted fact
freshness gap without claiming executor enforcement or authenticated
attestation. Current facts can now be obtained from one explicitly registered
source, validated against the exact immutable bundle and a deterministic age
limit, and assessed in the same call.

## 2. Scope Verification

The phase stayed within the approved Core model/helper scope. It did not alter
executor defaults, invoke checks or providers, persist snapshots, expose schema
or CLI behavior, integrate OpenShell, execute SideEffects, add writes, or change
release posture.

## 3. Source Boundary Assessment

The injected trait is narrow and testable. The request is read-only and bound to
the exact stored immutable run bundle and Core-selected evaluation time. The
source is called once. Source output remains untrusted until the same-call
helper validates it.

Registration is correctly documented as an embedding trust decision rather
than authentication. This distinction must remain explicit before any remote,
plugin-loaded, or hosted source is accepted.

## 4. Freshness And Integrity Assessment

Core validates:

- registered source identity and contract version;
- exact immutable run bundle binding;
- non-future observation time;
- the stricter source/Core maximum age;
- exact one-per-step fact coverage;
- deterministic workflow-step ordering; and
- payload-free fact and snapshot commitments.

Changed facts change both the fact-set and snapshot commitments. Source ordering
does not change the canonical commitment.

## 5. Error And Privacy Assessment

Source-provided errors are replaced with a stable Core-owned failure, preventing
arbitrary source values from crossing the boundary. Identifier validation
rejects secret-like text. Debug implementations redact IDs, hashes, bundle
bindings, timestamps, and commitments. Accepted snapshots are serialize-only
records and cannot be deserialized into authority.

## 6. Test Quality Assessment

Focused tests cover the primary success path and the meaningful trust-boundary
failures: stale and future observations, source and bundle mismatch, incomplete
coverage, canonical ordering, changed commitments, source error leakage,
serialization posture, and invalid identifiers and age bounds.

The tests correctly use an injected fake source and identify that this proves
Core validation, not production source authenticity.

## 7. Compatibility Assessment

The API is additive. Existing proportional-governance assessment, retry/resume,
executor, approval, evidence, report, SideEffect, and adapter paths remain
unchanged. No public workflow schema changes were introduced.

## 8. Blockers

None for this model/helper phase.

## 9. Non-Blocking Follow-Ups

- Define the exact durable snapshot commitment contract before executor retry
  and approval-resume consumption.
- Add persisted-corruption and replay tests when persistence is introduced.
- Define authenticated source identity only if remote or dynamically loaded
  sources become necessary.
- Do not confuse a serialized accepted snapshot with reusable authorization.

## 10. Product Feedback Reconciliation

The fresh-pull user review is aligned with this phase. Workflow OS now explains
its local governance posture well; the next product problem is reducing
low-risk ceremony while preserving evidence. This phase supplies a prerequisite
for trustworthy quiet success: runtime decisions can eventually use current,
source-bound facts instead of detached caller assertions. It does not itself
activate quiet success or resolve Node-version integration-check UX.

## 11. Recommended Next Phase

Plan one explicit opt-in executor consumer of fresh source-bound facts. Keep the
consumer local, same-call, and additive; preserve workflow pass/fail semantics;
and do not broaden proportional-governance defaults until durable retry/resume
binding is reviewed.

## 12. Validation Reviewed

- Focused runtime-fact source tests: passed.
- Focused clippy gate: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed using the same pinned toolchain with a
  temporary target directory to avoid local process-start delay.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Governed Review Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786275196240079000-2`
- Approval ID: `approval/run-1786275196240079000-2/composition-approved`
- Presentation ID: `presentation/07abe41c30e53cef`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: source and test implementation, documentation, full
  validation, diff review, and git/PR work after phase close
