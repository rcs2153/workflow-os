# Current-Authority Use-Boundary Hardening Review

## 1. Executive Verdict

Phase accepted; proceed to planning one concrete Core-owned read-only
consumer.

## 2. Scope Verification

The phase stayed within direct private test and documentation hardening.

It did not add a public authority API, production consumer, executor
integration, persistence, replay record, provider, OpenShell adapter, sandbox,
SideEffect, write, event, artifact, schema, SDK, CLI, UI, dependency, hosted
behavior, or release change.

## 3. Invalidation Assessment

The new tests exercise the same `use_current_authority` method a later
consumer would depend on.

- Expired and revoked grants block before use.
- A coherent changed harness contract and execution binding cannot reuse
  grants scoped to the prior contract.
- A contract supplied against a different execution binding fails at the
  source-request boundary.
- Unresolved policy, approval, evidence, and check prerequisites remain
  explicit and block before use.
- Every blocked, stale, or invalid path proves zero consumer invocations.

The tests do not rely on a prior returned assessment or caller assertion.

## 4. Fixed-Vector Assessment

The stable bounded vector covers successful use, blocked authority,
stale-source failure, and ambiguous consumer completion.

It pins typed semantics without making private hashes, timestamps, IDs, or
fixture values into compatibility surfaces. This is the correct level of
stability while the API remains crate-private.

## 5. Privacy And Error Assessment

The phase adds no new production storage or output. The exact
binding/contract mismatch code is stable and the regression confirms that its
Debug output does not disclose the contract ID.

The vector contains only bounded enums and reasons. No payload or secret-like
test value becomes report or serialization output.

## 6. Test Quality Assessment

The six new tests protect behavior rather than construction:

- each negative path measures consumer non-invocation;
- coherent and incoherent substitution cases are distinct;
- all independent prerequisites are represented;
- stale source remains distinguishable from blocked authority; and
- ambiguous completion remains distinguishable from known failure.

The repository-wide test and clippy suites pass. No lint suppression or new
test dependency was introduced.

## 7. Documentation Finding

The initial plan update labeled the entire future test list as implemented.
That would have overclaimed direct retry, approval-resume, worker-restart, and
compile-time escape coverage.

The review corrected the plan to separate implemented direct use-boundary
tests from future runtime-boundary tests. The corrected docs check passes.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- Actor, run, step, sensitivity, availability, and reference changes remain
  covered through lower-level source/resolver tests; add direct use-boundary
  cases only when a real consumer makes them independently valuable.
- Keep the generic callback private and test-only.
- Specialize the real consumer as one concrete Core-owned read-only
  operation.
- Do not claim durable replay prevention before authoritative persistence and
  atomic consumption exist.

## 10. Recommended Next Phase

Plan one concrete Core-owned opt-in read-only consumer.

The plan should choose a useful operation that can consume bounded authorized
references without dereferencing provider payloads or executing external
work. OpenShell integration remains later and should be treated as an
optional execution-provider boundary, not as the authority source.

## 11. Validation

- focused registered-source tests: passed, 24 tests;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 12. Governed Review Record

- workflow: `dg/review`;
- run ID: `run-1785178619471166000-2`;
- approval ID:
  `approval/run-1785178619471166000-2/review-scope-approved`;
- approval presentation ID: `presentation/28d284d30d5b01c1`;
- approval presentation content hash:
  `28d284d30d5b01c1ed0922a86d80bf30ebc99bbce2157138af1a7346ae1fbcd1`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- phase status: completed;
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
- approval-presentation enforcement: proof persisted before approval;
- out-of-kernel work: the delegated maintainer inspected the implementation,
  tests, plans, roadmap, and report and corrected one documentation overclaim;
  the kernel governed scope and approval but did not inspect code, execute
  validation, edit files, or mutate git.
