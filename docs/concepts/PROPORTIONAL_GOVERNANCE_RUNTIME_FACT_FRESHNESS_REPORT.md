# Proportional-Governance Runtime-Fact Freshness Report

## 1. Executive Summary

Workflow OS now has a Core-owned boundary for obtaining current typed runtime
facts from one explicitly registered injected source and assessing them against
an exact stored immutable run bundle in the same call. The boundary validates
identity, bundle binding, exact step coverage, and bounded freshness before it
returns a payload-free accepted snapshot with the existing governance
assessment.

## 2. Scope Completed

- Added bounded source, source-contract-version, and snapshot identities.
- Added a payload-free source registration with a Core freshness limit.
- Added a read-only exact-bundle source request.
- Added a structurally bounded untrusted observation model.
- Added an injected source interface.
- Added same-call identity, bundle, freshness, and coverage validation.
- Reused the accepted immutable-bundle governance assessment.
- Added deterministic fact-set and accepted-snapshot commitments.
- Added redaction-safe Debug and stable non-leaking source failure mapping.

## 3. Scope Explicitly Not Completed

No executor integration, default activation, persistence, replay authority,
automatic checks, multi-step expansion, schema, CLI, UI, provider call,
OpenShell integration, SideEffect execution, write capability, hosted source
registry, or cryptographic attestation was added.

## 4. Trust And Freshness Boundary

The embedding caller explicitly chooses the source registration and injected
implementation. This is a local trust boundary, not identity authentication.
Core requires exact registration identity and version, exact immutable-bundle
binding, a non-future observation, and age within the stricter source/Core
limit. The source is called once per assessment.

Accepted snapshots intentionally cannot be deserialized into trusted authority.
They commit the source registration, exact bundle, observation/evaluation time,
effective age limit, canonical fact set, and aggregate assessment.

## 5. Validation Boundary

The helper rejects:

- invalid or secret-like identifiers;
- invalid freshness bounds;
- source failure;
- source identity/version mismatch;
- immutable-bundle mismatch;
- future-dated or stale observations;
- empty, excessive, missing, duplicate, or unexpected step facts; and
- commitment serialization failure.

## 6. Privacy And Redaction

Public errors use stable bounded messages. Source errors are replaced rather
than forwarded. Debug output redacts source, snapshot, bundle, timestamp, and
commitment values. The model stores no raw provider payload, command output,
parser payload, spec content, environment value, credential, or token.

## 7. Test Coverage

Focused tests cover fresh success, stricter age selection, stale and
future-dated rejection, source and bundle mismatch, exact step coverage,
canonical ordering, commitment invalidation, source error wrapping, Debug and
serialization non-leakage, and identifier/freshness validation.

## 8. Commands And Results

- Focused runtime-fact source tests: passed.
- Focused clippy gate: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed using the same pinned toolchain with
  `CARGO_TARGET_DIR=/private/tmp/workflow-os-validation-target` to avoid the
  local repository target-directory process-start delay.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Known Limitations

- Registration is an explicit caller trust decision, not signed attestation.
- The accepted snapshot is not persisted or consumed by an executor.
- Retry/resume does not yet obtain facts through this source boundary.
- No remote source protocol, source lifecycle, revocation, or hosted registry
  exists.
- Assessment disposition enforcement remains separately bounded.

## 10. Recommended Next Phase

Proceed to focused planning for one explicit opt-in executor consumer of the
same-call source boundary. Preserve quiet success as a future product outcome,
but do not broaden defaults until fresh facts and durable reassessment bindings
are composed end to end.

## 11. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786275196240079000-2`
- Approval ID: `approval/run-1786275196240079000-2/composition-approved`
- Presentation ID: `presentation/07abe41c30e53cef`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Validation summary: focused tests and clippy, full formatting, workspace
  clippy, workspace tests, docs checks, and diff checks passed
- Out-of-kernel work: Rust implementation and tests, documentation updates,
  validation commands, temporary validation target selection, diff inspection,
  report drafting, and git/PR work after phase close
