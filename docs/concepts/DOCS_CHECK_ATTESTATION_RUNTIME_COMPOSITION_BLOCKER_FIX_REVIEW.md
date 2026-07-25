# DocsCheck Attestation Runtime Composition Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to consumer integration planning.

## 2. Scope Verification

The fix stayed within the approved immutable-attribution boundary. It removed
caller-supplied skill identity, resolved the selected step and its canonical
skill record from the validated stored bundle, added focused regressions, and
updated status documentation.

It did not add executor consumption, automatic checks, persistence, events,
evidence, reports, artifacts, schemas, CLI behavior, providers, SideEffects,
writes, hosted behavior, or release changes.

## 3. Authority Boundary Assessment

The caller may select a step ID, but the step must resolve exactly once from the
stored canonical workflow. The caller no longer supplies skill ID or version.
Core resolves exactly one step-scoped manifest reference, resolves the exact
canonical skill record by identity and content hash, and verifies agreement
among the stored workflow step, manifest reference, canonical skill ID, and
canonical skill version.

Handler selection, execution binding, observation, candidate, and accepted
proof therefore use Core-derived skill identity. Self-consistent caller
relabeling is no longer possible through this helper.

## 4. Pre-Execution Ordering Assessment

Workflow/run manifest checks and canonical step/skill resolution occur before
contract use, clock sampling, request construction, or runner invocation.
Missing, duplicate, or inconsistent stored records fail before execution
authority advances.

The existing order remains intact after resolution: immutable binding, request,
start observation, process execution, completion observation, structured
result, candidate, and verifier evaluation.

## 5. Failure And Privacy Assessment

Unresolved or inconsistent workflow, step, skill reference, and skill records
return stable `local_check_attestation.runtime.*` errors with static messages.
Errors do not echo IDs, versions, hashes, paths, command values, output, source
content, or credentials.

No partial result or proof is returned for attribution failure. Debug output
continues to redact bundle, identity, clock, result, and accepted-proof detail.

## 6. Test Quality Assessment

The focused tests prove that an unknown selected step fails before clock or
runner use and that a valid stored step derives its canonical skill ID and
version without caller skill authority. Compile-time construction also no
longer exposes skill ID/version fields on the helper input.

The validated immutable-bundle store already rejects inconsistent manifest and
record sets, while existing verifier and composition tests cover stored-bundle
substitution, workflow/run mismatch, process ordering, no-proof status,
freshness, clock failure, runner failure, and Debug safety.

One non-blocking follow-up is to add a purpose-built malformed step-to-skill
fixture if a safe test-only constructor is introduced later. Production
`StoredImmutableRunBundle` values cannot currently be constructed with that
invalid posture.

## 7. Compatibility Assessment

The helper remains crate-private and opt-in. Existing explicit
`DocsCheckLocalHandler` invocation is unchanged, no handler is registered by
default, and no executor or runtime consumer calls the helper.

## 8. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Handler implementation provenance remains honestly
  `RegisteredUnattested`.
- A future consumer must reevaluate freshness at its own decision boundary.
- Accepted proof remains in memory and is not persisted, evented, cited, or
  reused.
- The dogfood phase-close presentation-record read cap remains open.

## 11. Governed Review

- workflow: `dg/review`
- run: `run-1784921015396241000-2`
- approval: `approval/run-1784921015396241000-2/review-scope-approved`
- presentation: `presentation/97dacb8d0cf0d4f7`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- kernel boundary: governance coordination only; review, repository inspection,
  documentation, and validation ran outside the kernel

## 12. Recommended Next Phase

Plan the first explicit consumer of the accepted in-memory `DocsCheck`
attestation outcome. The plan must define freshness reevaluation, honest
no-proof handling, and failure semantics before adding runtime integration.

Do not add automatic checks, default registration, persistence, events,
evidence, reports, artifacts, schemas, CLI behavior, providers, SideEffects,
writes, hosted behavior, or release changes during planning.
